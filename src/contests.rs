use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::Auth,
    models::{Contest, Post},
    AppState,
};

pub async fn list_contests(State(state): State<AppState>) -> impl IntoResponse {
    let contests = Contest::find_all(&state.pool).await;
    Json(contests)
}

pub async fn get_contest(
    State(state): State<AppState>,
    Path(contest_id): Path<i32>,
) -> impl IntoResponse {
    let contest = Contest::find_by_id(&state.pool, contest_id).await;
    match contest {
        Some(contest) => Json(contest).into_response(),
        None => (StatusCode::NOT_FOUND, Json("Not Found")).into_response(),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContestBody {
    title: String,
    field: i32,
    started_at: i64,
    ended_at: i64,
    prize: String,
    link: String,
    img: Option<String>,
    ratio: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateContestResponse {
    contest_id: i32,
}

pub async fn create_contest(
    State(state): State<AppState>,
    Auth(auth): Auth,
    Json(body): Json<CreateContestBody>,
) -> impl IntoResponse {
    let contest_id = Contest::insert(
        &state.pool,
        &Contest {
            user_id: auth.sub,
            title: body.title,
            prize: body.prize,
            started_at: body.started_at,
            ended_at: body.ended_at,
            link: body.link,
            field: body.field,
            img: body.img,
            ratio: body.ratio,
            ..Contest::default()
        },
    )
    .await as _;

    (
        StatusCode::CREATED,
        Json(CreateContestResponse { contest_id }),
    )
}

pub async fn delete_contests(State(state): State<AppState>) {
    sqlx::query("DELETE FROM posts WHERE contest_id IS NOT NULL")
        .execute(&state.pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM contests")
        .execute(&state.pool)
        .await
        .unwrap();
}

pub async fn list_linked_posts(
    State(state): State<AppState>,
    Path(contest_id): Path<i32>,
) -> impl IntoResponse {
    let posts = Post::find_by_contest_id(&state.pool, contest_id).await;
    Json(posts).into_response()
}
