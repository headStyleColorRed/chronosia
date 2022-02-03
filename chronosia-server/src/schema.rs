table! {
    punches (id) {
        id -> Int4,
        user_id -> Int4,
        entry -> Varchar,
        leave -> Nullable<Varchar>,
        status -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        status -> Int4,
        current_punch -> Nullable<Int4>,
    }
}

joinable!(punches -> users (user_id));

allow_tables_to_appear_in_same_query!(
    punches,
    users,
);
