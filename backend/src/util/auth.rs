use crate::model::user::User;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for TokenUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
