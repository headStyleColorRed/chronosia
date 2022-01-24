use crate::context::GraphQLContext;
use crate::models::punch::{Punch, CreatePunchClockInInput, CreatePunchClockOutInput};
use crate::models::user::User;
use crate::operations::punches::Punches;
use crate::operations::users::{CreateUserInput, Users};
use juniper::FieldResult;
use juniper::RootNode;

// The root GraphQL query
pub struct Query;

// The root GraphQL mutation
pub struct Mutation;

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}

// Queries
#[juniper::object(Context = GraphQLContext)]
impl Query {
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        Users::all_users(context)
    }
}

// Mutations
#[juniper::object(Context = GraphQLContext)]
impl Mutation {
    pub fn create_user(context: &GraphQLContext, input: CreateUserInput) -> FieldResult<User> {
        Users::create_user(context, input)
    }

    pub fn clock_in(
        context: &GraphQLContext,
        input: CreatePunchClockInInput,
    ) -> FieldResult<Punch> {
        Punches::clock_in(context, input)
    }

    pub fn clock_out(
        context: &GraphQLContext,
        input: CreatePunchClockOutInput,
    ) -> FieldResult<Punch> {
        Punches::clock_out(context, input)
    }
}
