use crate::schema::todo;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

#[derive(Insertable, Serialize, Deserialize, Copy, Clone)]
#[table_name = "todo"]
pub struct NewTodo<'a> {
    pub description: &'a str,
    pub done: bool,
}
