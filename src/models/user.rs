use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct User {
    user_id: char,
    user_name: char,
    user_email: char,
    user_password: char,
    user_identify: char,
}