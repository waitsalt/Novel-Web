use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    uid: String,
    name: String,
    premssion: u8,
}
