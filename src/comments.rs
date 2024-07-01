use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{auth::Auth, models::Comment, utils::now, AppState};

pub async fn list_comments(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
) -> impl IntoResponse {
    Json(Comment::find_by_post_id(&state.pool, post_id).await)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentBody {
    pub post_id: i32,
    pub content: String,
    pub group_id: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentResponse {
    comment_id: i32,
}

pub async fn create_comment(
    State(state): State<AppState>,
    Auth(auth): Auth,
    Json(body): Json<CreateCommentBody>,
) -> impl IntoResponse {
    let comment_id = Comment::insert(
        &state.pool,
        &Comment {
            post_id: body.post_id,
            user_id: auth.sub,
            content: body.content,
            created_at: now(),
            group_id: body.group_id,
            ..Default::default()
        },
    )
    .await as _;
    (
        StatusCode::CREATED,
        Json(CreateCommentResponse { comment_id }),
    )
}
