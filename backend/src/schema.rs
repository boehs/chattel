// @generated automatically by Diesel CLI.


diesel::table! {
    use crate::item::{WhenMapping,ItemTypeMapping,StatusMapping};
    use diesel::sql_types::{Nullable, Date, Text, Integer};
    items (id) {
        id -> Integer,
        when_type -> Nullable<WhenMapping>,
        when_date -> Nullable<Date>,
        deadline -> Nullable<Date>,
        parent -> Nullable<Integer>,
        title -> Text,
        body -> Nullable<Text>,
        item_type -> ItemTypeMapping,
        item_status -> StatusMapping,
    }
}

diesel::table! {
    timeblocks (id) {
        id -> Integer,
        item -> Nullable<Integer>,
        start_time -> Nullable<TimestamptzSqlite>,
        end_time -> Nullable<TimestamptzSqlite>,
    }
}

diesel::joinable!(timeblocks -> items (item));

diesel::allow_tables_to_appear_in_same_query!(
    items,
    timeblocks,
);
