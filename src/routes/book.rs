use axum::{
    routing::{delete, get, post, put}, Json, Router
};
use crate::models::book::Book;

pub fn route() -> Router {
    Router::new()
        .route("/books", get(get_all_books))
        .route("/books", post(create_book))
        .route("/books/:id", get(get_book_by_id))
        .route("/books/:id", delete(remove_book_by_id))
        .route("/books/:id", put(update_book_by_id))
}

async fn get_all_books() -> Json<Vec<Book>> {
    todo!()
}

async fn create_book() -> &'static str {
    "create book"
}

async fn get_book_by_id() -> &'static str{
    "get book by id"
}

async fn remove_book_by_id() -> &'static str{
    "remove book by id"
}

async fn update_book_by_id() -> &'static str{
    "update book by id"
}
