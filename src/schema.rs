use diesel::table;

table! {
    chat_messages (id) {
        id -> Int4,
        message -> Varchar,
    }
}
