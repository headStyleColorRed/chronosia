use actix_web::Responder;
use super::bearer::Authorized;

pub async fn health(_: Authorized) -> impl Responder {
    "We are alive"
}