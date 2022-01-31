use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::Ready;
use futures_util::future::{err, ok};
use std::env;

use hmac::{Hmac, Mac};
use jwt::{VerifyWithKey, SignWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub struct Authorized;

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if is_authorized(req) {
            ok(Authorized)
        } else {
            err(ErrorUnauthorized("Not authorized"))
        }
    }
}

fn is_authorized(req: &HttpRequest) -> bool {
    // Retrieve environment
    let environment = match env::var("ENVIRONMENT") {
        Ok(environment) => environment,
        Err(_) => return false,
    };

    // Allow passing authorization if enviroment is local
    if environment == "local" {
        return true;
    }

    // Retrieve token bearer
    let bearer_token_result = match req.headers().get("authorized") {
        Some(value) => value.to_str(),
        None => return false,
    };

    // Unwrap token
    let bearer_token = match bearer_token_result {
        Ok(token) => token,
        Err(_) => return false,
    };

    //

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("email", "rodrigopple@gmail.com");
    claims.insert("iat", "1643650650");
    claims.insert("exp", "1643737050");
    
    let token_str = claims.sign_with_key(&key).unwrap();
    println!("Token => {}", token_str);
    //

    // Retrieve secret from environment file
    let secret = match env::var("SECRET") {
        Ok(secret) => secret,
        Err(_) => return false,
    };

    // Turn to bites
    let byte_secret = secret.as_bytes(); 

    // Create hash-based message authentication code
    let key: Hmac<Sha256> = match Hmac::new_from_slice(byte_secret) {
        Ok(key) => key,
        Err(err) => {
            dbg!(err);
            return false
        },
    };

    // Validate authorization token
    let claims: Result<BTreeMap<String, String>, jwt::Error> = bearer_token.verify_with_key(&key);

    // Return result
    match claims {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            return false
        },
    }
}
