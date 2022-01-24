use juniper::GraphQLInputObject;
use crate::schema::punches;

#[derive(Queryable)]
#[derive(juniper::GraphQLObject)]
pub struct Punch {
    pub id: i32,
    pub user_id: i32,
    pub entry: String,
    pub leave: Option<String>,
    pub status: i32,
}

// The GraphQL input object for clock in
#[derive(GraphQLInputObject)]
pub struct CreatePunchClockInInput {
    pub user_id: i32,
}

// The GraphQL input object for clock out
#[derive(GraphQLInputObject)]
pub struct CreatePunchClockOutInput {
    pub punch_id: i32,
}

// The GraphQL input object for inserting a punch
#[derive(GraphQLInputObject, Insertable)]
#[table_name = "punches"]
pub struct CreatePunchInsert {
    pub user_id: i32,
    pub entry: String,
    pub status: i32,
}