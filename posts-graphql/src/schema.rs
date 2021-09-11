table! {
    authors (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        author_id -> Int4,
    }
}

joinable!(posts -> authors (author_id));

allow_tables_to_appear_in_same_query!(
    authors,
    posts,
);
