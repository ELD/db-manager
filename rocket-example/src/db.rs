use rocket_sync_db_pools::{database, diesel};

#[database("primary_db")]
pub struct PrimaryDb(diesel::PgConnection);
