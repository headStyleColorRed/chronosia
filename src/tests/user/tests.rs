#[cfg(test)]
mod tests {
    use super::super::graphql::queries::UsersQuery;
    use crate::utils::format_error;
    use cynic::QueryBuilder;

    #[test]
    fn test_get_all_users_empty() {
        let operation = UsersQuery::build(());

        let response = reqwest::blocking::Client::new()
            .post("http://127.0.0.1:8080/graphql")
            .json(&operation)
            .send()
            .unwrap();

        let result: UsersQuery = operation
            .decode_response(response.json().unwrap())
            .unwrap()
            .data
            .unwrap();

        let expected: usize = 0;

        assert!(
            result.all_users.len() == expected,
            "{}",
            format_error(&expected.to_string(), &result.all_users.len().to_string())
        );
    }
}
