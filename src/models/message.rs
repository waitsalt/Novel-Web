use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Message {
    code: u8,
    message: String,
    content: serde_json::Value,
}