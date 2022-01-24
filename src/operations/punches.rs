use crate::context::GraphQLContext;
use crate::diesel::ExpressionMethods;
use crate::models::punch::{
    CreatePunchClockInInput, CreatePunchClockOutInput, CreatePunchInsert, Punch,
};
use crate::models::user::User;
use crate::operations::users::{Users, FindUserQuery};
use crate::schema::punches::dsl::*;
use crate::utils::{current_time, graphql_translate};
use diesel::QueryDsl;
use diesel::{pg::PgConnection, RunQueryDsl};
use juniper::{FieldError, FieldResult};

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

        // Check that user isn't already clocked
        if Users::user_with_id(context, FindUserQuery { id: input.user_id })?.current_punch.is_some() {
            return FieldResult::Err(FieldError::from("This user is already clocked in"))
        }

        // Make clockin query
        let clock_res = diesel::insert_into(punches).values(&punch).get_result(conn);
        // Parse result
        let res: Result<Punch, FieldError> = graphql_translate(clock_res);
        // Retrieve punch id
        match res {
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
        }
    }

    pub fn clock_out(
        context: &GraphQLContext,
        input: CreatePunchClockOutInput,
    ) -> FieldResult<Punch> {
        use crate::schema::users::dsl::{users, current_punch, id as users_id};
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();

        // Check that user isn't already clocked
        let user_punch_id = match Users::user_with_id(context, FindUserQuery { id: input.user_id })?.current_punch {
            Some(punch_id) => punch_id,
            None => -1,
        };
        

        if user_punch_id == -1 {
            return FieldResult::Err(FieldError::from("This user is already clocked out"))
        }

        // Update user's current punch status
        let empty_number: Option<i32> = None;
        if let Err(err) = diesel::update(users.filter(users_id.eq(input.user_id)))
                    .set(current_punch.eq(empty_number))
                    .get_result::<User>(conn) {
            return FieldResult::Err(FieldError::from(err))
        }

        // Make query
        let res = diesel::update(punches.filter(id.eq(user_punch_id)))
            .set((leave.eq(current_time()), status.eq(1)))
            .get_result(conn);
            
        // Parse result
        graphql_translate(res)
    }
}