use rocket::{local::asynchronous::Client, post, routes};
use rocket_db_pools::{sqlx, Connection, Database};
use test_stand::TestStand;

#[derive(Database)]
#[cfg_attr(test, derive(TestStand))]
#[database("test")]
pub struct PrimaryDatabase(sqlx::PgPool);

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PrimaryDatabase::test_stand())
        .attach(PrimaryDatabase::init())
        .mount("/", routes![modify_database])
}

#[post("/")]
async fn modify_database(mut connection: Connection<PrimaryDatabase>) {
    sqlx::query(r#"INSERT INTO "user" ("name") VALUES ($1), ($2), ($3), ($4), ($5)"#)
        .bind("johndoe".to_string())
        .bind("janedoe".to_string())
        .bind("joeschmoe".to_string())
        .bind("janeschmoe".to_string())
        .bind("peterprincip".to_string())
        .execute(&mut *connection)
        .await
        .unwrap();
}

#[rocket::async_test]
async fn i_can_spin_up_multiple_rockets_and_not_clobber_database() {
    let client_one = Client::untracked(rocket()).await.unwrap();
    let client_two = Client::untracked(rocket()).await.unwrap();

    assert_eq!(
        client_one.post("/").dispatch().await.status(),
        rocket::http::Status::Ok
    );
    assert_eq!(
        client_two.post("/").dispatch().await.status(),
        rocket::http::Status::Ok
    );
}
