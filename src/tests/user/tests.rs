#[cfg(test)]


pub mod user_tests {
    use graphql_client::{GraphQLQuery, Response};
    use crate::tests::utils::format_error;
    
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
        
        // Input parameters
        let input = create_user_mutation::CreateUserInput { name: name.to_string() };
        let variables: Variables = Variables { input };
        
        // Request
        let request_body = CreateUserMutation::build_query(variables);
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:8080/graphql").json(&request_body).send().await.unwrap();

        // Response
        let response: Response<create_user_mutation::ResponseData> = res.json().await.unwrap();
        let response_data = response.data.unwrap();
        let data_user = response_data.create_user;

        // Return
        let user =  UserData {
            id: data_user.id,
            name: data_user.name,
            status: data_user.status,
        };

        Ok(user)
    }

    async fn delete_user(id: i64) -> Result<UserData, ()> {
        use super::super::models::delete_user_mutation::user::{DeleteUserMutation, delete_user_mutation, delete_user_mutation::Variables};

        // Input parameters
        let input = delete_user_mutation::DeleteUserQuery{ id: id };
        let variables: Variables = Variables { input };

        // Request
        let request_body = DeleteUserMutation::build_query(variables);
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:8080/graphql").json(&request_body).send().await.unwrap();

        // Response
        let response: Response<delete_user_mutation::ResponseData> = res.json().await.unwrap();
        let response_data = response.data.unwrap();
        let data_user = response_data.delete_user;

         // Return
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
        let new_user_name = "Michael Scott Carbilias Serpentico";

        // Get current users
        let original_users_count: i32 = get_all_users().await.unwrap();

        // Create new user
        let new_user = create_user(new_user_name).await.unwrap();

        // Check that data is correct
        assert!(new_user.name == new_user_name, "{}", format_error(new_user_name, &new_user.name));
        assert!(new_user.id != -1, "{}", format_error("Something", &new_user.id.to_string()));
        assert!(new_user.status == 0, "{}", format_error(&0.to_string(), &new_user.status.to_string()));
        
        // Get current users
        let added_users_count: i32 = get_all_users().await.unwrap();

        // There should be one more user on the DB
        assert!(original_users_count + 1 == added_users_count, "{}", format_error(&(original_users_count + 1).to_string(), &added_users_count.to_string()));

        // Delete user
        let deleted_user = delete_user(new_user.id).await.unwrap();

        // Check that delted username is correct
        assert!(deleted_user.name == new_user_name, "{}", format_error(&deleted_user.name, &new_user.name));

        // Get current users
        let deleted_users_count: i32 = get_all_users().await.unwrap();

        // There should be the same number of users after deletion
        assert!(original_users_count == deleted_users_count, "{}", format_error(&original_users_count.to_string(), &deleted_users_count.to_string()));
    }
}