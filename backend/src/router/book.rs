use crate::{model::book::BookInfo, setting::SETTING};
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    error,
    model::book::{Book, Chapter, SearchBook},
};

pub async fn router() -> Router {
    Router::new()
        .route("/book/search", post(search))
        .route("/book/:book_id", get(book_show))
        .route("/book/:book_id/:chapter_id", get(chapter_show))
        .route("/book/:id/download", get(download))
}

async fn search(Json(search_book): Json<SearchBook>) -> Result<Json<Vec<Book>>, error::Error> {
    let book_list = search_book.search_book().await.unwrap();
    Ok(Json(book_list))
}

async fn book_show(Path(book_id): Path<String>) -> Result<Json<Vec<String>>, error::Error> {
    let path = &SETTING.library.path;
    let book_path = format!("{path}/{book_id}/info.json");
    let file = File::open(book_path).unwrap();
    let reader = BufReader::new(file);
    let book_info: BookInfo = serde_json::from_reader(reader).unwrap();
    Ok(Json(book_info.chapter))
}

async fn chapter_show(
    Path((book_id, chapter_id)): Path<(String, String)>,
) -> Result<String, error::Error> {
    let path = &SETTING.library.path;
    let book_path = format!("{path}/{book_id}/{chapter_id}.txt");
    let mut file = File::open(book_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    Ok(content)
}

async fn download(Path(book_id): Path<String>) -> Result<(), error::Error> {
    let path = &SETTING.library.path;
    let book_path = format!("{path}/{book_id}/all.txt");
    let mut file = File::open(book_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    Ok(())
}
