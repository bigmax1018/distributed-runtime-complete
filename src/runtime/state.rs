use dashmap::DashMap;
use crate::common::types::{TaskState};
use uuid::Uuid;

pub struct StateStore {
    pub tasks: DashMap<Uuid, TaskState>,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            tasks: DashMap::new(),
        }
    }

    pub fn save(&self, state: TaskState) {
        self.tasks.insert(state.id, state);
    }

    pub fn get(&self, id: &Uuid) -> Option<TaskState> {
        self.tasks.get(id).map(|s| s.clone())
    }

    pub fn remove(&self, id: &Uuid) {
        self.tasks.remove(id);
    }
}