#[cfg(test)]

pub mod punch {
    use graphql_client::GraphQLQuery;

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/tests/schema.graphql",
        query_path = "src/tests/punch/operations/clock_out_mutation.graphql",
        response_derives = "Debug"
    )]
    pub struct ClockOutMutation;
}
