mod schema {
    cynic::use_schema!("src/tests/schema.graphql");
}

#[cynic::schema_for_derives(file = "src/tests/schema.graphql", module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct UsersQuery {
        pub all_users: Vec<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub status: i32,
    }
}
