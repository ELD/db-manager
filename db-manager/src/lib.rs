#[macro_use]
extern crate diesel;

pub mod error;
pub mod manager;
#[cfg(feature = "rocket_integration")]
pub mod rocket_integration;
