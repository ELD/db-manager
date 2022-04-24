use crate::db::PrimaryDb;
use crate::model::{NewTodo, Todo};
use crate::schema::todo::dsl::*;
use diesel::RunQueryDsl;
use rocket::serde::json::Json;
use rocket::{get, post};

#[get("/")]
pub(crate) async fn index(primary_db: PrimaryDb) -> Json<Vec<Todo>> {
    primary_db
        .run(|c| {
            todo.load(c)
                .and_then(|todos| Ok(Json(todos)))
                .expect("no tasks found")
        })
        .await
}

#[post("/", data = "<new_todo>")]
pub(crate) async fn create(
    new_todo: Json<NewTodo>,
    primary_db: PrimaryDb,
) -> Result<rocket::http::Status, rocket::http::Status> {
    primary_db
        .run(|c| {
            diesel::insert_into(todo)
                .values(new_todo.into_inner())
                .execute(c)
                .map_err(|_| rocket::http::Status::BadRequest)
                .map(|_| rocket::http::Status::Created)
        })
        .await
}
