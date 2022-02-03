use crate::{context::GraphQLContext, models::punch::RemovePunchesQuery};
use crate::models::user::*;
use crate::schema::users::dsl::*;
use crate::utils::graphql_translate;
use diesel::{pg::PgConnection, RunQueryDsl, QueryDsl};
use juniper::{FieldResult};
use crate::diesel::ExpressionMethods;
use crate::operations::punches::Punches;

// This struct is basically a query manager. All the methods that it
// provides are static, making it a convenient abstraction for interacting
// with the database.
pub struct Users;

// User queries
impl Users {
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Make query
        let res = users.load::<User>(conn);
        // Parse ressult
        graphql_translate(res)
    }

    pub fn user_with_id(context: &GraphQLContext, input: FindUserQuery) -> FieldResult<User> {
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Make query
        let res = users.find(input.id).get_result(conn);
        // Parse ressult
        graphql_translate(res)
    }
}

// User mutations
impl Users {
    pub fn delete_user(context: &GraphQLContext, input: DeleteUserQuery) -> FieldResult<User> {
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Make query
        Punches::remove_user_punches(context, RemovePunchesQuery { user_id: input.id })?;
        let res = diesel::delete(users.filter(id.eq(input.id))).get_result(conn);
        // Parse ressult
        graphql_translate(res)
    }
    
    pub fn create_user(context: &GraphQLContext, input: CreateUserInput) -> FieldResult<User> {
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Make query
        let res = diesel::insert_into(users).values(&input).get_result(conn);
        // Parse ressult
        graphql_translate(res)
    }
}
