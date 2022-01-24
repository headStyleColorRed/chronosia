use actix_web::Responder;

pub async fn health() -> impl Responder {
    "We are alive"
}