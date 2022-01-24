use super::punch::Punch;
use crate::{context::GraphQLContext, operations::punches::Punches};
use diesel::Queryable;
use juniper::FieldResult;
// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
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

    pub fn history(context: &GraphQLContext) -> FieldResult<Vec<Punch>> {
        Punches::all_punches_for_user(context, self.id)
    }
}
