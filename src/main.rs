// Global crates
#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use std::io;
mod context;
mod db;
mod endpoints;
mod graphql;
mod models;
mod operations;
mod schema;
mod utils;

#[actix_rt::main]
async fn main() -> io::Result<()> {
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
            .configure(endpoints::graphql_endpoints)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
