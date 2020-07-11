#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};

mod user;
use user::{User};

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
  format!("Hi, I'm {} and I am {} years old!", name, age)
}

#[post("/users", data = "<user>")]
fn post_user(user: Json<User>) -> Json<User> {
  user
}

#[put("/users/<id>", data= "<user>")]
fn put_user(id: i32, user: Json<User>) -> Json<User> {
  user
}

#[delete("/users/<id>")]
fn delete_user(id: i32) -> JsonValue {
  json!({
    "id": id,
    "status": "ok",
  })
}

#[get("/users")]
fn get_users() -> JsonValue {
  json!({
    "name": "eduardo",
    "username": "eduardgz",
    "email": "eduardoperaltardgz@gmail.com",
    "hash": "c79845bf39f29ad8b54dfe23a3918cad"
  })
}

fn main() {
  rocket::ignite()
    .mount(
      "/api/v1",
      routes![
        hello,
        post_user,
        put_user,
        delete_user,
        get_users,
      ]
    )
    .launch();
}
