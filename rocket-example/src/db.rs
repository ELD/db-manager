use rocket_contrib::{database, databases::diesel::PgConnection};

#[database("primary_db")]
pub struct PrimaryDb(PgConnection);
