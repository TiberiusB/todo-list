use crate::tasks::Task;

#[derive(Debug, Clone)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }
}
