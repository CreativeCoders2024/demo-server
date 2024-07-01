use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{auth::Auth, models::Post, utils::now, AppState};

#[derive(Deserialize)]
pub struct SearchQuery {
    contest: Option<i32>,
}

pub async fn list_posts(
    State(state): State<AppState>,
    query: Query<SearchQuery>,
) -> impl IntoResponse {
    if let Some(contest) = query.contest {
        Json(Post::find_by_contest_id(&state.pool, contest).await)
    } else {
        Json(Post::find_all(&state.pool).await)
    }
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
) -> impl IntoResponse {
    Json(Post::find_by_id(&state.pool, post_id).await)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostBody {
    contest_id: Option<i32>,
    title: String,
    content: String,
    max: i32,
    ppl: i32,
    desired_field: i32,
    ended_at: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostResponse {
    post_id: i32,
}

pub async fn create_post(
    State(state): State<AppState>,
    Auth(auth): Auth,
    Json(body): Json<CreatePostBody>,
) -> impl IntoResponse {
    let post_id = Post::insert(
        &state.pool,
        &Post {
            user_id: auth.sub,
            contest_id: body.contest_id,
            title: body.title,
            content: body.content,
            max: body.max,
            ppl: body.ppl,
            desired_field: body.desired_field,
            created_at: now(),
            ended_at: body.ended_at,
            ..Default::default()
        },
    )
    .await as _;
    (StatusCode::CREATED, Json(CreatePostResponse { post_id }))
}

pub async fn delete_posts(State(state): State<AppState>) -> impl IntoResponse {
    sqlx::query("DELETE FROM posts")
        .execute(&state.pool)
        .await
        .unwrap();
}
