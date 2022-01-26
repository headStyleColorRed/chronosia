#[cfg(test)]

pub mod user {
    use graphql_client::GraphQLQuery;

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/tests/schema.graphql",
        query_path = "src/tests/user/operations/create_user_mutation.graphql",
        response_derives = "Debug"
    )]
    pub struct CreateUserMutation;
}
