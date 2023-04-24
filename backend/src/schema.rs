// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Integer,
        //when_type -> Nullable<Integer>,
        //when_date -> Nullable<TimestamptzSqlite>,
        //deadline -> Nullable<TimestamptzSqlite>,
        parent -> Nullable<Integer>,
        title -> Text,
        body -> Nullable<Text>,
        //item_type -> Integer,
        //item_status -> Integer,
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
