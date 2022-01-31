// Global crates
#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
mod context;
mod db;
mod endpoints;
mod graphql;
mod models;
mod operations;
mod rest;
mod schema;
mod tests;
mod utils;
use rest::generic::health;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Instantiate a new connection pool
    println!("Connecting to database");
    let pool = db::get_pool();

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints, (b) the logger middelware to
    // know that is happening and (b) the configuration function that adds the /graphql logic.
    // TODO: - Add cors
    println!("Starting server on port 8080");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(health))
            .configure(endpoints::graphql_endpoints)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}