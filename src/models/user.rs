use super::punch::Punch;
use crate::{context::GraphQLContext, operations::punches::Punches};
use crate::schema::users;
use diesel::Queryable;
use juniper::{FieldResult, GraphQLInputObject};
// The core data type undergirding the GraphQL interface
#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub current_punch: Option<i32>,
}

#[juniper::object(Context = GraphQLContext)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn status(&self) -> i32 {
        self.status
    }
    fn current_punch(&self) -> Option<i32> {
        self.current_punch
    }
    pub fn history(context: &GraphQLContext) -> FieldResult<Vec<Punch>> {
        Punches::all_punches_for_user(context, self.id)
    }
}

// The GraphQL input object for creating TODOs
#[derive(GraphQLInputObject, Insertable)]
#[table_name = "users"]
pub struct CreateUserInput {
    pub name: String
}

#[derive(GraphQLInputObject)]
pub struct FindUserQuery {
    pub id: i32
}

#[derive(GraphQLInputObject)]
pub struct DeleteUserQuery {
    pub id: i32
}