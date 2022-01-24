use chrono::Utc;
use juniper::{FieldError, FieldResult};

pub fn current_time() -> String {
    Utc::now().to_string()
}

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}