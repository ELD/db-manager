#[macro_use]
extern crate diesel_migrations;
use dbmanager::rocket_integration::RocketDatabaseManager;
use libexample::model::{NewTodo, Todo};
use libexample::{db::PrimaryDb, routes};
use rocket::local::Client;

embed_migrations!();

fn setup_rocket() -> (rocket::Rocket, PrimaryDb) {
    let rocket = rocket::ignite()
        .attach(RocketDatabaseManager::new("primary_db".to_string()))
        .attach(PrimaryDb::fairing())
        .mount("/", routes());

    let conn = PrimaryDb::get_one(&rocket).expect("unable to fetch");

    embedded_migrations::run(&*conn).unwrap();

    (rocket, conn)
}

#[test]
fn adds_a_new_todo() {
    let (rocket, _) = setup_rocket();
    let client = Client::new(rocket).expect("unable to create client");
    let new_todo = NewTodo {
        description: "a new todo",
        done: false,
    };

    let expected_todo = Todo {
        id: 1,
        description: "a new todo".to_string(),
        done: false,
    };

    let response = client
        .post("/")
        .body(serde_json::to_vec(&new_todo).unwrap())
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::Created);
}

#[test]
fn retrieves_all_todos() {
    use diesel::RunQueryDsl;
    use libexample::schema::todo::dsl::*;

    let (rocket, conn) = setup_rocket();
    let client = Client::new(rocket).expect("unable to create client");
    let expected_todos = expected_todos();
    diesel::insert_into(todo)
        .values(expected_todos.clone())
        .execute(&*conn);

    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), rocket::http::Status::Ok);

    let mut body = response.body().unwrap().into_string().unwrap();
    let returned_todos: Vec<Todo> = serde_json::from_str(&body).unwrap();

    assert_eq!(returned_todos.len(), expected_todos.len());
    returned_todos
        .iter()
        .enumerate()
        .for_each(|(i, returned_todo)| {
            assert_eq!(returned_todo.id, (i + 1) as i32);
            assert_eq!(returned_todo.description, expected_todos[i].description);
            assert_eq!(returned_todo.done, expected_todos[i].done);
        })
}

fn expected_todos<'a>() -> Vec<NewTodo<'a>> {
    vec![
        NewTodo {
            description: "my first todo",
            done: false,
        },
        NewTodo {
            description: "my second todo",
            done: false,
        },
        NewTodo {
            description: "my third todo",
            done: true,
        },
    ]
}
