use lib::model::Db;
use rocket::{launch, routes};

use lib::handlers::*;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(Db::fairing())
        .mount("/", routes![chat::index])
        .mount(
            "/chat",
            routes![
                chat::get_all_chat_posts,
                chat::get_chat_post,
                chat::create_chat_post
            ],
        )
}
