use crate::db::Conn;
use rocket_contrib::json::{ JsonValue };
use crate::models::User;

#[get("/users")]
pub fn get_users(conn: Conn) -> JsonValue {
  let users = User::get_all_users(&conn);
  json!({
    "status": 200,
    "result": users,
  })
}