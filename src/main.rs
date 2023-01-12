#[macro_use]
use rocket::*;
use ::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    id: i32,
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
fn get_random_blog_post() -> Json<Vec<ChatMessage>> {
    Json(vec![ChatMessage {
        id: 1,
        message: "My first post".to_string(),
    }])
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<ChatMessage> {
    Json(ChatMessage {
        id,
        message: "Hello World!".to_string(),
    })
}

#[post("/", data = "<chat_message>")]
fn create_blog_post(chat_message: Json<ChatMessage>) -> Json<ChatMessage> {
    println!("{:#?}", chat_message);
    chat_message
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    rocket.mount("/", routes![index]).mount(
        "/chat",
        routes![get_random_blog_post, get_blog_post, create_blog_post],
    )
}
