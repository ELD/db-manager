//! `TestStand` is a library for temporarily creating databases for parallel testing.
//! This crate provides types and macros for easily adding this functionality to existing
//! test code. It's as simple as adding a `#[derive(TestStand)]` to your database type,
//! calling `my_db_type::test_stand()`, and the rest is taken care of by this library.
//!
//! Example:
//! ```rust,ignore
//! use rocket_db_pools::{sqlx, Database};
//! use test_stand::TestStand;
//!
//! #[derive(Database)]
//! #[database("my_db")]
//! #[cfg_attr(test, derive(TestStand))]
//! pub struct MyDatabase(sqlx::PgPool);
//!
//! #[rocket::launch]
//! fn rocket() -> _ {
//!    rocket::build().attach(MyDatabase::test_stand()) // <-- this is the only line needed
//! }
//!
//! #[rocket::async_test]
//! async fn i_can_spin_up_multiple_rockets_and_not_clobber_database() {
//!    todo!("write this test");
//! }
//!
//! #[rocket::async_test]
//! async fn i_can_spin_up_multiple_rockets_and_not_clobber_database_2() {
//!   todo!("write this test");
//! }
//! ```
mod test_stand;
mod test_stand_pool;

pub use crate::test_stand::*;
pub use crate::test_stand_pool::*;
pub use test_stand_derive::TestStand;
