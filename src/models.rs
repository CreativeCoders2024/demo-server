use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

#[derive(Clone, Debug, Default, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub email: String,
    pub bio: Option<String>,
    pub is_manager: bool,
    pub is_withdrawn: bool,
    pub field: i32,
    pub profile_img: Option<Vec<u8>>,
}

impl User {
    pub async fn insert(pool: &SqlitePool, user: &User) {
        sqlx::query(
            r#"
            INSERT INTO users (id, pw, nickname, email, bio, is_manager, is_withdrawn, field, profile_img)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.nickname)
        .bind(&user.email)
        .bind(&user.bio)
        .bind(user.is_manager)
        .bind(user.is_withdrawn)
        .bind(user.field)
        .bind(&user.profile_img)
        .execute(pool)
        .await
        .unwrap();
    }

    pub async fn find_all(pool: &SqlitePool) -> Vec<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Option<User> {
        dbg!(&username);
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await
            .unwrap()
    }

    pub async fn find_by_id(pool: &SqlitePool, id: i32) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Contest {
    pub contest_id: i32,
    pub user_id: i32,
    pub title: String,
    pub prize: String,
    pub started_at: i64,
    pub ended_at: i64,
    pub link: String,
    pub field: i32,
    pub img: Option<String>,
    pub like_count: i32,
}

impl Contest {
    pub async fn insert(pool: &SqlitePool, contest: &Contest) -> i64 {
        sqlx::query(
            r#"
            INSERT INTO contests (user_id, title, prize, started_at, ended_at, link, field, img, like_count)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(contest.user_id)
        .bind(&contest.title)
        .bind(&contest.prize)
        .bind(contest.started_at)
        .bind(contest.ended_at)
        .bind(&contest.link)
        .bind(contest.field)
        .bind(&contest.img)
        .bind(contest.like_count)
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
    }

    pub async fn find_all(pool: &SqlitePool) -> Vec<Contest> {
        sqlx::query_as::<_, Contest>("SELECT * FROM contests")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn find_by_id(pool: &SqlitePool, contest_id: i32) -> Option<Contest> {
        sqlx::query_as::<_, Contest>("SELECT * FROM contests WHERE contest_id = ?")
            .bind(contest_id)
            .fetch_optional(pool)
            .await
            .unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub post_id: i32,
    pub user_id: i32,
    pub contest_id: Option<i32>,
    pub title: String,
    pub content: String,
    pub max: i32,
    pub ppl: i32,
    pub desired_field: i32,
    pub created_at: i64,
    pub ended_at: i64,
    pub like_count: i32,
}

impl Post {
    pub async fn insert(pool: &SqlitePool, post: &Post) -> i64 {
        sqlx::query(
            r#"
            INSERT INTO posts (user_id, contest_id, title, content, max, ppl, desired_field, created_at, ended_at, like_count)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(post.user_id)
        .bind(post.contest_id)
        .bind(&post.title)
        .bind(&post.content)
        .bind(post.max)
        .bind(post.ppl)
        .bind(post.desired_field)
        .bind(post.created_at)
        .bind(post.ended_at)
        .bind(post.like_count)
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
    }

    pub async fn find_all(pool: &SqlitePool) -> Vec<Post> {
        sqlx::query_as::<_, Post>("SELECT * FROM posts")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn find_by_contest_id(pool: &SqlitePool, contest_id: i32) -> Vec<Post> {
        sqlx::query_as("SELECT * FROM posts WHERE contest_id = ?")
            .bind(contest_id)
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn find_by_id(pool: &SqlitePool, post_id: i32) -> Option<Post> {
        sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE post_id = ?")
            .bind(post_id)
            .fetch_optional(pool)
            .await
            .unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub comment_id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: i64,
    pub edited_at: Option<i64>,
    pub parent: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CommentWithUser {
    pub comment_id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub nickname: String,
    pub content: String,
    pub created_at: i64,
    pub edited_at: Option<i64>,
    pub parent: Option<i32>,
}

impl Comment {
    pub async fn insert(pool: &SqlitePool, comment: &Comment) -> i64 {
        sqlx::query(
            r#"
            INSERT INTO comments (post_id, user_id, content, created_at, edited_at, parent)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(comment.post_id)
        .bind(comment.user_id)
        .bind(&comment.content)
        .bind(comment.created_at)
        .bind(comment.edited_at)
        .bind(comment.parent)
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
    }

    pub async fn find_by_post_id(pool: &SqlitePool, post_id: i32) -> Vec<CommentWithUser> {
        sqlx::query_as(
            r#"
            SELECT comments.comment_id, comments.post_id, comments.user_id, users.nickname, comments.content, comments.created_at, comments.edited_at, comments.parent
            FROM comments
            JOIN users ON comments.user_id = users.user_id
            WHERE comments.post_id = ?
            "#,
        )
        .bind(post_id)
        .fetch_all(pool)
        .await
        .unwrap()
    }
}
