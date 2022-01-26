#[cfg(test)]

pub mod user {
    use graphql_client::GraphQLQuery;


    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/tests/schema.graphql",
        query_path = "src/tests/user/operations/all_users_query.graphql",
        response_derives = "Debug"
    )]
    pub struct AllUsersQuery;
}
