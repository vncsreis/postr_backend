use crate::http::users::{UserPublic, UserVec};
use crate::http::Result;
use crate::utils::current_time;
use crate::utils::jwt::Claims;
use axum::extract::Path;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

pub fn router() -> Router {
    Router::new()
        .route("/post", get(get_user_posts).post(insert_post))
        .route(
            "/post/:id",
            get(get_post).delete(delete_post).put(update_post),
        )
        .route("/post/:id/edits", get(get_post_history))
        .route("/post/:id/likes", get(get_users_liked))
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub content: String,
    pub user_id: Uuid,
    pub edited: bool,
    pub created_at: chrono::NaiveDateTime,
    pub last_updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
struct PostVersion {
    id: Uuid,
    content: String,
    version: chrono::NaiveDateTime,
    post_id: Uuid,
}

#[derive(Serialize)]
struct PostVersionVec {
    post_versions: Vec<PostVersion>,
}

#[derive(Serialize)]
pub struct PostVec {
    pub posts: Vec<Post>,
}

#[derive(Deserialize)]
struct PostInsert {
    content: String,
}

#[derive(Deserialize)]
struct PostUpdate {
    content: String,
}

async fn _get_posts(Extension(db): Extension<PgPool>) -> Result<Json<PostVec>> {
    let posts = sqlx::query_as!(
        Post,
        r#"
    
    SELECT
        id,
        content,
        user_id,
        edited,
        created_at,
        last_updated_at
    FROM posts
    "#
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(PostVec { posts }))
}
async fn get_post(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Post>> {
    let post = sqlx::query_as!(
        Post,
        r#"
    SELECT
        id,
        content,
        user_id,
        edited,
        created_at,
        last_updated_at
    FROM posts WHERE id = $1 AND user_id = $2 AND deleted = false
    "#,
        id,
        claims.user_id
    )
    .fetch_one(&db)
    .await?;

    Ok(Json::from(post))
}

async fn get_user_posts(claims: Claims, Extension(db): Extension<PgPool>) -> Result<Json<PostVec>> {
    let posts = sqlx::query_as!(
        Post,
        r#"
    SELECT
        id,
        content,
        user_id,
        edited,
        created_at,
        last_updated_at
    FROM posts WHERE user_id = $1 AND deleted = false
    "#,
        claims.user_id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(PostVec { posts }))
}

async fn insert_post(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(req): Json<PostInsert>,
) -> Result<Json<bool>> {
    let inserted = sqlx::query!(
        r#"
    INSERT INTO posts (content, user_id) VALUES ($1, $2)
    "#,
        req.content,
        claims.user_id
    )
    .execute(&db)
    .await?
    .rows_affected()
        > 0;

    Ok(Json::from(inserted))
}

async fn delete_post(
    _claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<bool>> {
    let deleted = sqlx::query!(
        r#"
    UPDATE posts SET deleted = true, last_updated_at = $2 WHERE id = $1
    "#,
        id,
        current_time()
    )
    .execute(&db)
    .await?
    .rows_affected()
        > 0;

    Ok(Json::from(deleted))
}

async fn update_post(
    _claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<PostUpdate>,
) -> Result<Json<Post>> {
    sqlx::query!(
        r#"
        INSERT INTO posts_history (
            content, post_id
        )
        SELECT
            content, id
        FROM posts WHERE id = $1
    "#,
        id
    )
    .execute(&db)
    .await?;

    let new_post = sqlx::query_as!(
        Post,
        r#"
    UPDATE posts SET
        content = $2,
        edited = true,
        last_updated_at = $3
    WHERE id = $1
    RETURNING 
        id,
        content,
        user_id,
        edited,
        created_at,
        last_updated_at
    "#,
        id,
        req.content,
        current_time(),
    )
    .fetch_one(&db)
    .await?;

    Ok(Json::from(new_post))
}

async fn get_post_history(
    _claims: Claims,
    Path(id): Path<Uuid>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<PostVersionVec>> {
    let post_versions = sqlx::query_as!(
        PostVersion,
        r#"
        SELECT * FROM posts_history WHERE post_id = $1 ORDER BY version DESC
    
    "#,
        id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(PostVersionVec { post_versions }))
}

async fn get_users_liked(
    Path(id): Path<Uuid>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<UserVec<UserPublic>>> {
    let users = sqlx::query_as!(
        UserPublic,
        r#"
        SELECT
            u.id,
            u.name,
            u.username
        FROM users u
        INNER JOIN likes l
        ON u.id = l.user_id
        WHERE l.post_id = $1
        "#,
        id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(UserVec { users }))
}
