table! {
    tiny_link (id) {
        id -> Varchar,
        tiny -> Varchar,
        origin -> Varchar,
        user_id -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(tiny_link, users,);
