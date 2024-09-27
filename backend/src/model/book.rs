use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Book {
    uid: u64,
    name: String,
    author: String,
    desc: String,
    tag: String,
}
