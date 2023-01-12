#[macro_use]
extern crate diesel;
use ::serde::{Deserialize, Serialize};
use rocket::futures::FutureExt;

use diesel::{Identifiable, Insertable, RunQueryDsl};
use rocket::serde::json::Json;
use rocket::*;
use rocket_sync_db_pools::database;

#[database("koodi101")]
pub struct Db(rocket_sync_db_pools::diesel::pg::PgConnection);

table! {
    chat_posts (id) {
        id -> Int4,
        message -> Varchar,
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable, Identifiable, Queryable)]
struct ChatPost {
    id: i32,
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
async fn get_all_blog_posts(connection: Db) -> Json<Vec<ChatPost>> {
    connection
        .run(|c| chat_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<ChatPost> {
    Json(ChatPost {
        id,
        message: "Hello World!".to_string(),
    })
}

#[post("/", data = "<chat_message>")]
async fn create_blog_post(connection: Db, chat_message: Json<ChatPost>) -> Json<ChatPost> {
    connection
        .run(move |c| {
            diesel::insert_into(chat_posts::table)
                .values(&chat_message.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(Db::fairing())
        .mount("/", routes![index])
        .mount(
            "/chat",
            routes![get_all_blog_posts, get_blog_post, create_blog_post],
        )
}
