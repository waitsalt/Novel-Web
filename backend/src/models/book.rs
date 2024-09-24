use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Book {
    book_id: char,
    book_name: char,
    book_author: char,
    book_desc: char,
    book_tags: char,
}