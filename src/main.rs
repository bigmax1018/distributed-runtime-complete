mod node;
mod runtime;
mod network;
mod common;

use common::types::Task;
use node::scheduler::Scheduler;
use node::worker::execute_task;
use runtime::state::StateStore;
use network::rpc::{start_server, send};

use uuid::Uuid;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Change this per node for a cluster
    let my_addr = "127.0.0.1:7000";   // this node's TCP port
    let other_node_addr = "127.0.0.1:7001"; // another node

    let scheduler = Arc::new(Scheduler::new());
    let state_store = Arc::new(StateStore::new());

    //spawn workder loop
    let scheduler_ref = scheduler.clone();
    let state_ref = state_store.clone();

    tokio::spawn(async move {
       loop {
           if let Some(task) = scheduler_ref.next().await {
               let state_clone = state_ref.clone();
               tokio::spawn(async move {
                   execute_task(task, &state_clone).await;
               });
           }
       }
    });

    // submit demo task
    let task = Task {
        id: Uuid::new_v4(),
        payload: vec![],
    };

    scheduler.submit(task).await;

    loop {}
}
