#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod db;
mod models;
mod routes;
mod schema;

fn main() {
  rocket::ignite().launch();
}
