#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod routes;
mod schema;

fn rocket() -> rocket::Rocket {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

  let pool = db::init_pool(database_url);
  rocket::ignite()
    .manage(pool)
    .mount(
      "/api/v1/",
      routes![
        routes::get_users,
      ],
    )
}

fn main() {
  rocket().launch();
}
