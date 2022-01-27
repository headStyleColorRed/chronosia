#[cfg(test)]


pub mod user_tests {
    use futures::{Future, future::{BoxFuture, ok}};
    use graphql_client::{GraphQLQuery, Response};
    struct UserData {
        id: i64,
        name: String,
        status: i64,
    }

    async fn get_all_users() -> Result<i32, ()> {

        use super::super::models::all_users_query::user::{AllUsersQuery, all_users_query, all_users_query::Variables};

        let variables: Variables = Variables;
        let request_body = AllUsersQuery::build_query(variables);
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:8080/graphql").json(&request_body).send().await.unwrap();
        let response: Response<all_users_query::ResponseData> = res.json().await.unwrap();
        let response_data = response.data.unwrap();
        
        Ok(response_data.all_users.len() as i32)
    }


    async fn create_user(name: &str) -> Result<UserData, ()> {
        use super::super::models::create_user_mutation::user::{CreateUserMutation, create_user_mutation, create_user_mutation::Variables};
    
        let input = create_user_mutation::CreateUserInput { name: name.to_string() };
        let variables: Variables = Variables { input };
    
        let request_body = CreateUserMutation::build_query(variables);
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:8080/graphql").json(&request_body).send().await.unwrap();
        let response: Response<create_user_mutation::ResponseData> = res.json().await.unwrap();
        let response_data = response.data.unwrap();
        let data_user = response_data.create_user;

        let user =  UserData {
            id: data_user.id,
            name: data_user.name,
            status: data_user.status,
        };

        Ok(user)
    }

    #[tokio::main]
    #[test]
    async fn user_flow() {
        // Get current users
        let users_count: i32 = get_all_users().await.unwrap();

        // Create new user
        let new_user = create_user("Michael Scott Carbilias Serpentico").await.unwrap();

        println!("Users => {}", users_count);
        println!("Name => {}", new_user.name);
    }
}