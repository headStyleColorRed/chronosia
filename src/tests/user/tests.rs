#[cfg(test)]


pub mod user_tests {
    use graphql_client::{GraphQLQuery, Response};

    #[tokio::main]
    #[test]
    async fn get_all_users() {

        use super::super::models::all_users_query::user::{AllUsersQuery, all_users_query, all_users_query::Variables};

        let variables: Variables = Variables;
        let request_body = AllUsersQuery::build_query(variables);
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:8080/graphql").json(&request_body).send().await.unwrap();
        let response_body: Response<all_users_query::ResponseData> = res.json().await.unwrap();

        println!("{:#?}", response_body);
    }


    #[tokio::main]
    #[test]
    async fn create_user() {
        use super::super::models::create_user_mutation::user::{CreateUserMutation, create_user_mutation, create_user_mutation::Variables};
    
        let input = create_user_mutation::CreateUserInput { name: "Rodrigo".to_string() };
        let variables: Variables = Variables { input };
    
        let request_body = CreateUserMutation::build_query(variables);
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:8080/graphql").json(&request_body).send().await.unwrap();
        let response_body: Response<create_user_mutation::ResponseData> = res.json().await.unwrap();
    
        println!("{:#?}", response_body);
    }
}