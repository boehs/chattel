// @generated automatically by Diesel CLI.

diesel::table! {
    item (id) {
        id -> Integer,
        when -> Nullable<Text>,
        deadline -> Nullable<Text>,
        parent -> Nullable<Integer>,
        title -> Text,
        body -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        status -> Nullable<Text>,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        when_type -> Nullable<Integer>,
        when_date -> Nullable<Date>,
        deadline -> Nullable<Timestamp>,
        parent -> Nullable<Integer>,
        title -> Text,
        body -> Nullable<Text>,
        item_type -> Integer,
        item_status -> Integer,
    }
}

diesel::table! {
    seaql_migrations (version) {
        version -> Text,
        applied_at -> Integer,
    }
}

diesel::table! {
    timeblock (id) {
        id -> Integer,
        item -> Nullable<Integer>,
        start -> Nullable<Text>,
        end -> Nullable<Text>,
    }
}

diesel::table! {
    timeblocks (id) {
        id -> Integer,
        item -> Nullable<Integer>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
    }
}

diesel::joinable!(timeblock -> item (item));
diesel::joinable!(timeblocks -> items (item));

diesel::allow_tables_to_appear_in_same_query!(
    item,
    items,
    seaql_migrations,
    timeblock,
    timeblocks,
);
