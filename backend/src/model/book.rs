use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::database::POOL;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub desc: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BookInfo {
    pub id: String,
    pub name: String,
    pub author: String,
    pub desc: String,
    pub chapter: Vec<String>,
    pub collect_chapter: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chapter {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchBook {
    pub kind: String,
    pub keyword: String,
}

impl SearchBook {
    pub async fn search_book(&self) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        let pool = POOL.get().expect("error").clone();
        let books = sqlx::query_as::<_, Book>("select * from public.book where name like $1;")
            .bind(&self.keyword)
            .fetch_all(&pool)
            .await?;
        Ok(books)
    }
}
