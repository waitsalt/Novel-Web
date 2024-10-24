use crate::{
    error,
    model::book::{Book, Chapter, SearchBook},
};
use crate::{model::book::BookInfo, setting::SETTING};
use axum::{
    body::Body,
    extract::Path,
    http::header,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::{
    fs::{self, File},
    io::BufReader,
};

pub async fn router() -> Router {
    Router::new()
        .route("/book/search", post(search))
        .route("/book/:book_id", get(book_read))
        .route("/book/:book_id/:chapter_id", get(chapter_read))
        .route("/book/download/:book_id", get(download))
}

async fn search(Json(search_book): Json<SearchBook>) -> Result<Json<Vec<Book>>, error::Error> {
    let book_list = search_book.search_book().await.unwrap();
    Ok(Json(book_list))
}

async fn book_read(Path(book_id): Path<String>) -> Result<Json<BookInfo>, error::Error> {
    let path = &SETTING.library.path;
    let book_path = format!("{path}/{book_id}/info.json");
    let file = File::open(&book_path).unwrap();
    let reader = BufReader::new(file);
    let book_info: BookInfo = serde_json::from_reader(reader).unwrap();
    Ok(Json(book_info))
}

async fn chapter_read(
    Path((book_id, chapter_id)): Path<(String, String)>,
) -> Result<Json<Chapter>, error::Error> {
    let path = &SETTING.library.path;
    let book_path = format!("{path}/{book_id}/{chapter_id}.json");
    let file = File::open(&book_path).unwrap();
    let reader = BufReader::new(file);
    let chapter: Chapter = serde_json::from_reader(reader).unwrap();
    Ok(Json(chapter))
}

async fn download(Path(book_id): Path<String>) -> Result<impl IntoResponse, error::Error> {
    let path = &SETTING.library.path;
    let book_path = format!("{path}/{book_id}/all.txt");
    let file = tokio::fs::File::open(book_path).await.unwrap();
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let headers = [
        (
            header::CONTENT_TYPE,
            "text/plain; charset=utf-8".to_string(),
        ),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"all.txt\""),
        ),
    ];
    Ok((headers, body))
}

async fn check_download_file(file_path: &String) {
    let chapter_num = 
}
