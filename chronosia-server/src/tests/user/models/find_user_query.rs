#[cfg(test)]

pub mod user {
    use graphql_client::GraphQLQuery;

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/tests/schema.graphql",
        query_path = "src/tests/user/operations/find_user_query.graphql",
        response_derives = "Debug"
    )]
    pub struct FindUserQuery;
}
