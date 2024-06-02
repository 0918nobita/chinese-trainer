use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub enum Status {
    Todo,
    Done,
}

#[derive(Clone)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    status: Status,
}

impl Todo {
    pub fn new(id: u64, text: String) -> Self {
        Self {
            id,
            text,
            status: Status::Todo,
        }
    }

    pub fn status(&self) -> Status {
        self.status.clone()
    }
}

pub trait TodoRepository: Clone + Send + Sync + 'static {
    fn get_todo_list(&self) -> Vec<Todo>;
}

type TodoList = HashMap<u64, Todo>;

#[derive(Clone, Default)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoList>>,
}

impl TodoRepository for TodoRepositoryForMemory {
    fn get_todo_list(&self) -> Vec<Todo> {
        let store = self.store.read().unwrap();
        store.values().cloned().collect()
    }
}
