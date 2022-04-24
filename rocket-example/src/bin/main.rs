use libexample::db::PrimaryDb;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(PrimaryDb::fairing())
        .mount("/todo", libexample::routes())
        .ignite()
        .await?
        .launch()
        .await
}
