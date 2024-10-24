use axum::Json;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, prelude::FromRow};

use crate::database::POOL;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicUser {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimsUser {
    pub exp: usize,
    pub user: PublicUser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyUser {
    pub name: String,
    pub password: String,
}

impl PublicUser {
    pub fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            level: user.level,
            email: user.email,
        }
    }
}

impl ClaimsUser {
    pub fn from(public_user: PublicUser) -> Self {
        Self {
            exp: (chrono::Local::now() + chrono::Duration::days(10)).timestamp() as usize,
            user: public_user,
        }
    }
}

impl VerifyUser {
    pub async fn query_user(&self) -> Result<Json<User>, Box<dyn std::error::Error>> {
        let pool = POOL.get().expect("error").clone();
        let user = sqlx::query_as::<_, User>("select * from public.user where name = $1;")
            .bind(&self.name)
            .fetch_one(&pool)
            .await?;
        Ok(Json(user))
    }
}

impl CreateUser {
    pub async fn query_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let pool = POOL.get().expect("error").clone();
        let users = sqlx::query_as::<_, User>("select * from public.user where name = $1;")
            .bind(&self.name)
            .fetch_all(&pool)
            .await?;
        Ok(users)
    }

    pub async fn create_user(&self) -> Result<Option<PgRow>, sqlx::Error> {
        let pool = POOL.get().expect("error").clone();
        let id = nanoid!();
        let res =
            sqlx::query("insert into public.user(id,name,email,password) values ($1,$2,$3,$4)")
                .bind(&id)
                .bind(&self.name)
                .bind(&self.email)
                .bind(&self.password)
                .fetch_optional(&pool)
                .await?;
        Ok(res)
    }
}
