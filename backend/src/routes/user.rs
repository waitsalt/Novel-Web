use axum::{
    routing::{delete, get, post, put}, Json, Router
};

use crate::models::user::User;

pub fn route() -> Router {
    Router::new()
        .route("/users", get(get_all_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/:id", delete(remove_user_by_id))
        .route("/users/:id", put(update_user_by_id))
}

async fn get_all_users() -> Json<Vec<User>>{
    todo!()
}

async fn create_user() {
    todo!()
}

async fn get_user_by_id() {
    todo!()
}

async fn remove_user_by_id() {
    todo!()
}

async fn update_user_by_id() {
    todo!()
}
