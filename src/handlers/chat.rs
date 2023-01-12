use crate::model::{ChatMessage, Db};
use crate::schema::chat_messages;

use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rocket::{get, post};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
pub async fn get_all_chat_posts(connection: Db) -> Json<Vec<ChatMessage>> {
    connection
        .run(|c| chat_messages::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch chat messages")
}

#[get("/<message_id>")]
pub async fn get_chat_post(connection: Db, message_id: i32) -> Json<ChatMessage> {
    connection
        .run(move |c| chat_messages::table.find(message_id).first(c))
        .await
        .map(Json)
        .expect("Failed to fetch chat message")
}

#[post("/", data = "<chat_message>")]
pub async fn create_chat_post(
    connection: Db,
    chat_message: Json<ChatMessage>,
) -> Json<ChatMessage> {
    connection
        .run(move |c| {
            diesel::insert_into(chat_messages::table)
                .values(&chat_message.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("failed to puload chat message")
}
