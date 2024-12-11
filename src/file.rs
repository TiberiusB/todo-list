use std::fs::{self, OpenOptions};

use serde_json::from_reader;

use crate::{
    error::TodoResult,
    tasks,
    todo_list::{self, TodoList},
};

pub fn loadjson() -> TodoResult<TodoList> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(false)
        .create(true)
        .open("todo.json")?;

    match from_reader(&file) {
        Ok(tasks) => Ok(TodoList { tasks }),
        Err(_) => {
            fs::write("todo.json", Vec::new())?;
            println!("Created new tIodo list");
            Ok(TodoList::new())
        }
    }
}
