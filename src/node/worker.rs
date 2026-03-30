use std::thread::sleep;
use crate::common::types::{Task, TaskState};
use crate::runtime::state::StateStore;
use tokio::time::{ Duration };

pub async fn execute_task(task: Task, state_store: &StateStore) {
    let mut progress = 0;

    while progress < 100 {
        sleep(Duration::from_millis(100));
        progress += 10;
        let state = TaskState {
            id: task.id,
            progress,
            data: vec![],
        };

        state_store.save(state);
    }
    println!("Task {} completed successfully", task.id);
}