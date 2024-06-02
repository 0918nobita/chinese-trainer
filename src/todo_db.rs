use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusVO {
    Todo,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoEntity {
    pub id: u64,
    pub text: String,
    status: StatusVO,
}

impl TodoEntity {
    pub fn new(id: u64, text: String) -> Self {
        Self {
            id,
            text,
            status: StatusVO::Todo,
        }
    }

    pub fn status(&self) -> StatusVO {
        self.status.clone()
    }
}

pub trait TodoRepository: Send + Sync + 'static {
    fn all(&self) -> Vec<TodoEntity>;
    fn find(&self, id: u64) -> Option<TodoEntity>;
    fn create(&self, text: String) -> TodoEntity;
    fn delete(&self, id: u64) -> Option<()>;
}

#[derive(Default)]
struct TodoList {
    inner: HashMap<u64, TodoEntity>,
    current_id: u64,
}

#[derive(Clone, Default)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoList>>,
}

impl TodoRepository for TodoRepositoryForMemory {
    fn all(&self) -> Vec<TodoEntity> {
        let store = self.store.read().unwrap();
        store.inner.values().cloned().collect()
    }

    fn find(&self, id: u64) -> Option<TodoEntity> {
        let store = self.store.read().unwrap();
        store.inner.get(&id).cloned()
    }

    fn create(&self, text: String) -> TodoEntity {
        let mut store = self.store.write().unwrap();
        let id = store.current_id;
        let todo = TodoEntity::new(id, text);
        store.inner.insert(id, todo.clone());
        store.current_id += 1;
        todo
    }

    fn delete(&self, id: u64) -> Option<()> {
        let mut store = self.store.write().unwrap();
        store.inner.remove(&id).map(|_| ())
    }
}
