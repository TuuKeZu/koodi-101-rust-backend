use ::serde::{Deserialize, Serialize};
use diesel::{Identifiable, Insertable};
use rocket_sync_db_pools::database;

use crate::schema::*;

#[database("koodi101")]
pub struct Db(rocket_sync_db_pools::diesel::pg::PgConnection);

#[derive(Serialize, Deserialize, Debug, Insertable, Identifiable, Queryable)]
#[table_name = "chat_messages"]
pub struct ChatMessage {
    pub id: i32,
    pub message: String,
}
