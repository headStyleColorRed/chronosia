#[cfg(test)]

pub mod punch {
    use graphql_client::GraphQLQuery;

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/tests/schema.graphql",
        query_path = "src/tests/punch/operations/clock_in_mutation.graphql",
        response_derives = "Debug"
    )]
    pub struct ClockInMutation;
}
