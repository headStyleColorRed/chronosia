use crate::{db, endpoints, rest::generic::health};
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Instantiate a new connection pool
    println!("Connecting to database");
    let pool = db::get_pool();

    println!("Starting server on port 8080");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(health))
            .configure(endpoints::graphql_endpoints)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use reqwest;

    #[tokio::main]
    #[test]
    async fn test_health() {
        let response = reqwest::get("http://127.0.0.1:8080/")
            .await
            .unwrap()
            .text()
            .await
            .expect("Error querying health route");

            let expected = "We are alives";
        
            assert!(response == expected, "{}", format_error(expected, &response));
    }

    fn format_error(expected: &str, received: &str) -> String {
        format!("\n\n- Expected: {}\n- Received: {}\n\n", expected, received)
    }
}
