#[cfg(test)]
mod tests {
    use reqwest::{self, StatusCode};
    use crate::utils::format_error;

    #[tokio::main]
    #[test]
    async fn test_health() {
        let response = reqwest::get("http://127.0.0.1:8080/")
            .await
            .unwrap()
            .text()
            .await
            .expect("Error querying health route");

            let expected = "We are alive";
        
            assert!(response == expected, "{}", format_error(expected, &response));
    }

    #[tokio::main]
    #[test]
    async fn test_playground() {
        let response = reqwest::get("http://127.0.0.1:8080/graphql")
            .await
            .expect("Error querying playground");

            let expected: StatusCode = reqwest::StatusCode::OK;
        
            assert!(response.status() == expected, "{}", format_error(expected.as_str(), &response.status().as_str()));
    }
}
