use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::{model::user::User, setting::SETTING};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenUser {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub user: TokenUser,
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

impl Claims {
    pub fn new(user: User) -> Self {
        Self {
            exp: (chrono::Local::now() + chrono::Duration::days(10)).timestamp() as usize,
            user: TokenUser::new(user),
        }
    }
}

pub fn create(user: User, secret: &str) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let claims = Claims::new(user);

    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
}

pub fn decode(token: &str, secret: &str) -> Result<TokenData<Claims>, Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    jsonwebtoken::decode(token, &decoding_key, &Validation::default())
}

#[async_trait]
impl<S> FromRequestParts<S> for TokenUser
where
    S: Send + Sync,
{
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| crate::error::AuthError::AuthInvalid)?;

        let secret = SETTING.auth.secret.as_str();
        let token_data =
            decode(bearer.token(), secret).map_err(|_| crate::error::AuthError::AuthInvalid)?;

        Ok(token_data.claims.user)
    }
}
