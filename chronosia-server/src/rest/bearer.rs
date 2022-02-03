use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::Ready;
use futures_util::future::{err, ok};
use std::env;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, Validation, DecodingKey};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    iat: usize,
    exp: usize,
}

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

    // Retrieve secret from environment file
    let secret = match env::var("SECRET") {
        Ok(secret) => secret,
        Err(_) => return false,
    };

    // Validate jwt
    let claims = decode::<Claims>(&bearer_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

    // Return result
    match claims {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            return false
        },
    }
}