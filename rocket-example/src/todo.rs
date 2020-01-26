use crate::db::PrimaryDb;
use crate::model::{NewTodo, Todo};
use crate::schema::todo::dsl::*;
use diesel::RunQueryDsl;
use rocket::{get, post};
use rocket_contrib::json::Json;

#[get("/")]
pub(crate) fn index(primary_db: PrimaryDb) -> Json<Vec<Todo>> {
    todo.load(&*primary_db)
        .and_then(|todos| Ok(Json(todos)))
        .expect("no tasks found")
}

#[post("/", data = "<new_todo>")]
pub(crate) fn create(
    new_todo: Json<NewTodo>,
    primary_db: PrimaryDb,
) -> Result<rocket::http::Status, rocket::http::Status> {
    diesel::insert_into(todo)
        .values(new_todo.into_inner())
        .execute(&*primary_db)
        .map_err(|_| rocket::http::Status::BadRequest)
        .map(|_| rocket::http::Status::Created)
}
