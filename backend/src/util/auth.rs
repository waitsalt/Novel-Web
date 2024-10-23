use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;

use crate::setting::SETTING;
use crate::{
    error,
    model::user::{ClaimsUser, PublicUser},
};

static VALIDATION: Lazy<Validation> = Lazy::new(Validation::default);
static HEADER: Lazy<Header> = Lazy::new(Header::default);

// create token
pub fn create(public_user: PublicUser, secret: &str) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let claims = ClaimsUser::from(public_user);

    jsonwebtoken::encode(&HEADER, &claims, &encoding_key)
}

// decode token,get PublicUser
pub fn decode(token: &str, secret: &str) -> Result<TokenData<ClaimsUser>, Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    jsonwebtoken::decode(token, &decoding_key, &VALIDATION)
}

#[async_trait]
impl<S> FromRequestParts<S> for ClaimsUser
where
    S: Send + Sync,
{
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| crate::error::AuthError::AuthMiss)?;

        let secret = SETTING.auth.secret.as_str();
        let token_data =
            decode(bearer.token(), secret).map_err(|_| crate::error::AuthError::AuthInvalid)?;

        Ok(token_data.claims)
    }
}

// 检查 token 的
pub async fn check_claims_user(claims_user: &ClaimsUser) -> Result<(), error::Error> {
    let local_time = chrono::Local::now().timestamp() as usize;
    if local_time > claims_user.exp {
        return Err(error::Error::AuthError(error::AuthError::AuthTimeout));
    }
    Ok(())
}
