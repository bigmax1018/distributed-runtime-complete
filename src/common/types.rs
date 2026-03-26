use serde::{ Serialize, Deserialize };
use uuid::Uuid;

pub type NodeId = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskState {
    pub id: Uuid,
    pub progress: u64,
    pub data: Vec<u8>,
}