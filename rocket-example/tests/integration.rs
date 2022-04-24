#[macro_use]
extern crate diesel_migrations;
use std::thread;
use std::time::Duration;

use dbmanager::rocket_integration::RocketDatabaseManager;
use libexample::model::{NewTodo, Todo};
use libexample::{db::PrimaryDb, routes};
use rocket::local::asynchronous::Client;
use rocket::Ignite;

embed_migrations!();

async fn setup_rocket() -> (rocket::Rocket<Ignite>, PrimaryDb) {
    let rocket = rocket::build()
        .attach(RocketDatabaseManager::new("primary_db".to_string()))
        .attach(PrimaryDb::fairing())
        .mount("/", routes())
        .ignite()
        .await
        .unwrap();

    let conn = PrimaryDb::get_one(&rocket).await.expect("unable to fetch");

    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("unable to run migration");

    (rocket, conn)
}

#[rocket::async_test]
async fn adds_a_new_todo() {
    let (rocket, _) = setup_rocket().await;
    let client = Client::untracked(rocket)
        .await
        .expect("unable to create client");
    let new_todo = NewTodo {
        description: "a new todo".to_string(),
        done: false,
    };

    let response = client
        .post("/")
        .body(serde_json::to_vec(&new_todo).unwrap())
        .dispatch()
        .await;

    assert_eq!(response.status(), rocket::http::Status::Created);

    thread::sleep(Duration::from_secs(3600));
}

#[rocket::async_test]
async fn retrieves_all_todos() {
    use diesel::RunQueryDsl;
    use libexample::schema::todo::dsl::*;

    let (rocket, conn) = setup_rocket().await;
    let client = Client::untracked(rocket)
        .await
        .expect("unable to create client");
    conn.run(|c| {
        diesel::insert_into(todo)
            .values(expected_todos())
            .execute(c)
    })
    .await
    .expect("could not insert");

    let response = client.get("/").dispatch().await;

    assert_eq!(response.status(), rocket::http::Status::Ok);

    let body = response.into_string().await.expect("could not ready body");
    let returned_todos: Vec<Todo> = serde_json::from_str(&body).unwrap();

    let expected_todos = expected_todos();
    assert_eq!(returned_todos.len(), expected_todos.len());
    returned_todos
        .iter()
        .enumerate()
        .for_each(|(i, returned_todo)| {
            assert_eq!(returned_todo.id, (i + 1) as i32);
            assert_eq!(returned_todo.description, expected_todos[i].description);
            assert_eq!(returned_todo.done, expected_todos[i].done);
        });

    thread::sleep(Duration::from_secs(3600));
}

fn expected_todos() -> Vec<NewTodo> {
    vec![
        NewTodo {
            description: "my first todo".to_string(),
            done: false,
        },
        NewTodo {
            description: "my second todo".to_string(),
            done: false,
        },
        NewTodo {
            description: "my third todo".to_string(),
            done: true,
        },
    ]
}
