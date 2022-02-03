#[cfg(test)]

pub mod punch_tests {
    use std::panic;
    use super::super::super::user::user_tests::user_tests as UserMethods;
    use graphql_client::{GraphQLQuery, Response};
    use crate::tests::utils::format_error;
    use graphql_client::Error as GraphQLError;

    struct PunchData {
        user_id: i64,
        leave: Option<String>,
        status: i64
    }

    fn unwrap_error(errors: Option<Vec<GraphQLError>>) -> GraphQLError {
        let mock_error = GraphQLError {message: "Unable to decifer error".to_string(), locations: None, path: None, extensions: None};
        
        let error_array = match errors {
            Some(error) => error,
            None => return mock_error,
        };

        error_array[0].clone()
    }

    async fn clock_in(user_id: i64) -> Result<PunchData, GraphQLError> {
        use super::super::models::clock_in_mutation::punch::{ClockInMutation, clock_in_mutation, clock_in_mutation::Variables};

        // Input parameters
        let input = clock_in_mutation::CreatePunchClockInInput { userId: user_id};
        let variables: Variables = Variables { input };

        // Request
        let request_body = ClockInMutation::build_query(variables);
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:7000/graphql").json(&request_body).send().await.unwrap();

        // Response
        let response: Response<clock_in_mutation::ResponseData> = res.json().await.unwrap();
        let response_data = match response.data {
            Some(res) => Some(res),
            None => return Err(unwrap_error(response.errors)),
        }.unwrap();

        let data_user = response_data.clock_in;

         // Return
         let punch = PunchData {
            user_id: data_user.user_id,
            leave: data_user.leave,
            status: data_user.status,
        };

        Ok(punch)
    }

    async fn clock_out(user_id: i64) -> Result<PunchData, GraphQLError> {
        use super::super::models::clock_out_mutation::punch::{ClockOutMutation, clock_out_mutation, clock_out_mutation::Variables};

        // Input parameters
        let input = clock_out_mutation::CreatePunchClockOutInput { userId: user_id};
        let variables: Variables = Variables { input };

        // Request
        let request_body = ClockOutMutation::build_query(variables);
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:7000/graphql").json(&request_body).send().await.unwrap();

        // Response
        let response: Response<clock_out_mutation::ResponseData> = res.json().await.unwrap();
        let response_data = match response.data {
            Some(res) => Some(res),
            None => return Err(unwrap_error(response.errors)),
        }.unwrap();

        let data_user = response_data.clock_out;

         // Return
         let punch = PunchData {
            user_id: data_user.user_id,
            leave: data_user.leave,
            status: data_user.status,
        };

        Ok(punch)
    }

    pub async fn punch_flow() {
        let new_user_name = "Michael Scott Cesar Riddle";

        // Create new user
        let new_user = UserMethods::create_user(new_user_name).await.unwrap();

        // Clock in
        let punch_in = match clock_in(new_user.id).await {
            Ok(punch) => punch,
            Err(err) => panic!("{}", err.message),
        };

        assert!(punch_in.status == 0, "{}", format_error(&0.to_string(), &punch_in.status.to_string()));
        assert!(punch_in.user_id == new_user.id, "{}", format_error(&punch_in.user_id.to_string(), &new_user.id.to_string()));
        assert!(punch_in.leave.is_none(), "{}", format_error("To be null", &punch_in.leave.unwrap()));

        // Clock out
        let punch_out = match clock_out(new_user.id).await {
            Ok(punch) => punch,
            Err(err) => panic!("{}", err.message),
        };

        assert!(punch_out.status == 1, "{}", format_error(&1.to_string(), &punch_in.status.to_string()));
        assert!(punch_out.user_id == new_user.id, "{}", format_error(&punch_in.user_id.to_string(), &new_user.id.to_string()));
        assert!(punch_out.leave.is_some(), "{}", format_error("To be null", &punch_in.leave.unwrap()));

        // // Delete user
        UserMethods::delete_user(new_user.id).await.unwrap();
    }
}