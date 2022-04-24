use crate::schema::todo;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "todo"]
pub struct NewTodo {
    pub description: String,
    pub done: bool,
}
