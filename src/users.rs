use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{auth::Auth, models::User, AppState};

pub async fn me(State(state): State<AppState>, Auth(claims): Auth) -> impl IntoResponse {
    if let Some(user) = User::find_by_user_id(&state.pool, claims.sub).await {
        (StatusCode::OK, Json(user)).into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    Json(User::find_all(&state.pool).await)
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    if let Some(user) = User::find_by_user_id(&state.pool, user_id).await {
        (StatusCode::OK, Json(user)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
