use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicUser {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenUser {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimsUser {
    pub exp: usize,
    pub user: TokenUser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyUser {
    pub name: String,
    pub password: String,
}

impl TokenUser {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            level: user.level,
            email: user.email,
        }
    }
}

impl ClaimsUser {
    pub fn new(user: User) -> Self {
        Self {
            exp: (chrono::Local::now() + chrono::Duration::days(10)).timestamp() as usize,
            user: TokenUser::new(user),
        }
    }
}
