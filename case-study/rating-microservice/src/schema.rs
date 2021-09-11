table! {
    comments (id) {
        id -> Int4,
        text -> Text,
        post_id -> Int4,
        author_id -> Int4,
        author_username -> Varchar,
    }
}

table! {
    rates (id) {
        id -> Int4,
        value -> Int4,
        post_id -> Int4,
        author_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    comments,
    rates,
);
