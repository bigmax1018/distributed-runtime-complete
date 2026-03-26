use crate::common::types::TaskState;

pub async fn serialize_state(state: &TaskState) -> Vec<u8> {
    bincode::serialize(state).unwrap()
}

pub async fn deserialize_state(data: &Vec<u8>) -> TaskState {
    bincode::deserialize(data).unwrap()
}