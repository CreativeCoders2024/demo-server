use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{models::User, AppState};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupBody {
    username: String,
    password: String,
    nickname: String,
    email: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(body): Json<SignupBody>,
) -> impl IntoResponse {
    if User::find_by_username(&state.pool, &body.username)
        .await
        .is_some()
    {
        return (StatusCode::CONFLICT, "User already exists").into_response();
    }

    User::insert(
        &state.pool,
        &User {
            username: body.username,
            password: body.password,
            nickname: body.nickname,
            email: body.email,
            ..Default::default()
        },
    )
    .await;

    StatusCode::CREATED.into_response()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginBody {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginBody>,
) -> impl IntoResponse {
    let Some(user) = User::find_by_username(&state.pool, &body.username).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    if user.password != body.password {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let token = create_jwt(user.id, b"secret").unwrap();
    Json(LoginResponse { token }).into_response()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: i32, // Subject (whom the token refers to)
    // pub exp: usize,
    pub iat: usize,
}

fn create_jwt(user_id: i32, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    // let expiration = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_secs()
    //     + 3600; // Token expires in 1 hour

    let claims = Claims {
        sub: user_id,
        // exp: expiration as usize,
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize,
    };

    let header = Header::default();
    encode(&header, &claims, &EncodingKey::from_secret(secret))
}

pub struct Auth(pub Claims);

#[async_trait::async_trait]
impl FromRequestParts<AppState> for Auth {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &AppState) -> Result<Self, Self::Rejection> {
        let Some(Ok(value)) = parts.headers.get("authorization").map(HeaderValue::to_str) else {
            return Err((StatusCode::UNAUTHORIZED, "Invalid `Authorization` header"));
        };

        let Some(token) = value.split(' ').last() else {
            return Err((StatusCode::UNAUTHORIZED, "Invalid `Authorization` header"));
        };

        let mut validation = Validation::default();
        validation.required_spec_claims.clear();
        validation.validate_exp = false;
        let key = DecodingKey::from_secret(b"secret");
        let Ok(token_data) = decode(token, &key, &validation) else {
            return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
        };

        Ok(Auth(token_data.claims))
    }
}
