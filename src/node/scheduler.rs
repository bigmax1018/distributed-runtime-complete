use crate::common::types::{Task};
use std::collections::VecDeque;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct Scheduler {
    queue: Arc<Mutex<VecDeque<Task>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn submit(&self, task: Task) {
        self.queue.lock().await.push_back(task);
    }

    pub async fn next(&self) -> Option<Task> {
        self.queue.lock().await.pop_front()
    }
}