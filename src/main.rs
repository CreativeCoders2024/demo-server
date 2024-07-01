use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::filter::LevelFilter;

mod auth;
mod comments;
mod contests;
mod models;
mod posts;
mod users;
mod utils;

const DATABASE_URL: &str = "sqlite:main.db";

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();
    init_tables(&pool).await;

    tracing_subscriber::fmt::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let router = Router::new()
        .route("/signup", post(auth::signup))
        .route("/login", post(auth::login))
        .route("/users/@me", get(users::me))
        .route("/users/:user_id", get(users::get_user))
        .route("/contests", get(contests::list_contests))
        .route("/contests/:contest_id", get(contests::get_contest))
        .route("/contests", post(contests::create_contest))
        .route("/contests", delete(contests::delete_contests))
        .route("/posts", get(posts::list_posts))
        .route("/posts/:post_id", get(posts::get_post))
        .route("/posts", post(posts::create_post))
        .route("/posts", delete(posts::delete_posts))
        .route("/comments/:post_id", get(comments::list_comments))
        .route("/comments", post(comments::create_comment))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { pool });

    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("Failed to bind port");
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

async fn init_tables(pool: &SqlitePool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            user_id INTEGER PRIMARY KEY,
            id VARCHAR(10) NOT NULL,
            pw VARCHAR(20) NOT NULL,
            nickname VARCHAR(10) NOT NULL,
            email VARCHAR(100) NOT NULL,
            bio VARCHAR(1000),
            is_manager BOOLEAN NOT NULL,
            is_withdrawn BOOLEAN NOT NULL,
            field INTEGER,
            profile_img BLOB
        );

        CREATE TABLE IF NOT EXISTS contests (
            contest_id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            title VARCHAR(100) NOT NULL,
            img VARCHAR(1000),
            prize VARCHAR(100) NOT NULL,
            started_at DATETIME NOT NULL,
            ended_at DATETIME NOT NULL,
            link VARCHAR(1000) NOT NULL,
            field INTEGER NOT NULL,
            like_count INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );

        CREATE TABLE IF NOT EXISTS posts (
            post_id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            contest_id INTEGER,
            title VARCHAR(100) NOT NULL,
            content VARCHAR(1000),
            max INTEGER,
            ppl INTEGER,
            desired_field INTEGER,
            created_at DATETIME NOT NULL,
            ended_at DATETIME NOT NULL,
            like_count INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(user_id),
            FOREIGN KEY (contest_id) REFERENCES contests(contest_id)
        );

        CREATE TABLE IF NOT EXISTS comments (
            comment_id INTEGER PRIMARY KEY,
            post_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            content VARCHAR NOT NULL,
            created_at DATETIME NOT NULL,
            edited_at DATETIME,
            group_id INTEGER NOT NULL,
            FOREIGN KEY (post_id) REFERENCES posts(post_id),
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create tables");
}
