use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: i32, 
  pub username: String,
  pub email: String,
  pub hash: String,
}

impl User {

  pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
    all_users
      .order(users::id.desc())
      .load::<User>(conn)
      .expect("error!")
  }

}