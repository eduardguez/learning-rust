use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: i32, 
  pub username: String,
  pub email: String,
  pub hash: String,
}