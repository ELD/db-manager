#[macro_use]
extern crate diesel;

use rocket::{routes, Route};

pub mod db;
pub mod model;
pub mod schema;
pub mod todo;

pub fn routes() -> Vec<Route> {
    routes![crate::todo::create, crate::todo::index]
}
