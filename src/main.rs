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
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Change this per node for a cluster
    let my_addr = "127.0.0.1:7000";   // this node's TCP port
    let other_node_addr = "127.0.0.1:7001"; // another node

    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    let state_store = Arc::new(StateStore::new());

    let server_handle = tokio::spawn(start_server(my_addr));

    //spawn workder loop
    let scheduler_ref = scheduler.clone();
    let state_ref = state_store.clone();

    tokio::spawn(async move {
       loop {
           let maybe_task = {
               let mut sched = scheduler_ref.lock().await;
               sched.next().await
           };

           if let Some(task) = maybe_task {
               let state_clone = state_ref.clone();
               tokio::spawn(async move {
                   execute_task(task, &state_clone).await;
               });
           }else {
               tokio::time::sleep(std::time::Duration::from_millis(100)).await;
           }
       }
    });

    // submit demo task locally
    let task = Task {
        id: Uuid::new_v4(),
        payload: vec![],
    };
    {
        let mut sched = scheduler.lock().await;
        sched.submit(task.clone()).await;
    }

    let serialize_task = bincode::serialize(&task).unwrap();
    tokio::spawn(async move {
        send(my_addr, &serialize_task).await;
    });
    server_handle.await.unwrap();
}
