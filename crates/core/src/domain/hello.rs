use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Message {
    pub message: String,
}
