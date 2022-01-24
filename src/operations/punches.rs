use crate::context::GraphQLContext;
use crate::diesel::ExpressionMethods;
use crate::models::punch::{
    CreatePunchClockInInput, CreatePunchClockOutInput, CreatePunchInsert, Punch,
};
use crate::models::user::User;
use crate::schema::punches::dsl::*;
use crate::utils::{current_time, graphql_translate};
use diesel::QueryDsl;
use diesel::{pg::PgConnection, Insertable, RunQueryDsl};
use juniper::{FieldError, FieldResult, GraphQLInputObject};

// This struct is basically a query manager. All the methods that it
// provides are static, making it a convenient abstraction for interacting
// with the database.
pub struct Punches;

// Punch queries
impl Punches {
    pub fn all_punches_for_user(context: &GraphQLContext, id_user: i32) -> FieldResult<Vec<Punch>> {
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Make query
        let res = punches.filter(user_id.eq(id_user)).get_results(conn);
        // Parse result
        graphql_translate(res)
    }
}

// Punch mutations
impl Punches {
    pub fn clock_in(
        context: &GraphQLContext,
        input: CreatePunchClockInInput,
    ) -> FieldResult<Punch> {
        use crate::schema::users::dsl::*;
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Create punch
        let punch = CreatePunchInsert {
            user_id: input.user_id,
            entry: current_time(),
            status: 0,
        };

        // Make query
        let clock_res = diesel::insert_into(punches).values(&punch).get_result(conn);
        // Parse result
        let res: Result<Punch, FieldError> = graphql_translate(clock_res);
        // Retrieve punch id
        let result: Result<Punch, FieldError> = match res {
            Ok(created_punch) => {
                let res = diesel::update(users.filter(id.eq(created_punch.user_id)))
                    .set(current_punch.eq(Some(created_punch.id)))
                    .get_result::<User>(conn);
                    match res {
                        Ok(_) => Ok(created_punch),
                        Err(err) => FieldResult::Err(FieldError::from(err)),
                    }
            }
            Err(err) => FieldResult::Err(FieldError::from(err)),
        };

        result
    }

    pub fn clock_out(
        context: &GraphQLContext,
        input: CreatePunchClockOutInput,
    ) -> FieldResult<Punch> {
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Make query
        let res = diesel::update(punches.filter(id.eq(input.punch_id)))
            .set((leave.eq(current_time()), status.eq(1)))
            .get_result(conn);
        // Parse result
        graphql_translate(res)
    }
}
