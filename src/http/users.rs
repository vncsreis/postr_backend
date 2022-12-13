use crate::http::Result;
use crate::utils::jwt::Claims;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use pbkdf2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

use super::error::Error;
use super::posts::{Post, PostVec};

pub fn router() -> Router {
    Router::new()
        .route("/user", get(get_users).post(insert_user))
        .route("/user/:id", get(get_user))
        .route("/user/login", post(login_user))
        .route("/user/follows", get(get_following))
        .route("/user/followers", get(get_followers))
        .route("/user/likes", get(get_user_likes))
}

#[derive(Serialize)]
pub struct User {
    id: Uuid,
    email: String,
    username: String,
    name: String,
}

#[derive(Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    id: Uuid,
    token: String,
}

#[derive(Deserialize)]
struct UserCreate {
    username: String,
    email: String,
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct UserLogin {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

#[derive(Serialize)]
pub struct UserVec<T> {
    pub users: Vec<T>,
}

async fn insert_user(
    Extension(db): Extension<PgPool>,
    Json(usr): Json<UserCreate>,
) -> Result<Json<bool>> {
    let salt = SaltString::generate(&mut OsRng);

    let hash_password = Pbkdf2
        .hash_password(usr.password.as_bytes(), &salt)
        .unwrap();

    let created = sqlx::query!(
        r#"
    INSERT INTO users(username, email, hash_password, name) VALUES ($1, $2, $3, $4)
    "#,
        usr.username,
        usr.email,
        hash_password.to_string(),
        usr.name
    )
    .execute(&db)
    .await?
    .rows_affected()
        > 0;

    Ok(Json::from(created))
}

async fn get_users(Extension(db): Extension<PgPool>) -> Result<Json<UserVec<User>>> {
    let users = sqlx::query_as!(
        User,
        r#"
    SELECT id, email, username, name FROM users;
    "#
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(UserVec { users }))
}

async fn get_user(
    _claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
    SELECT id, email, username, name FROM users WHERE id = $1
    "#,
        id
    )
    .fetch_one(&db)
    .await?;

    Ok(Json::from(user))
}

async fn login_user(
    Extension(db): Extension<PgPool>,
    Json(usr): Json<UserLogin>,
) -> Result<Json<LoginResponse>> {
    let info = sqlx::query!(
        r#"
    SELECT hash_password, id FROM users WHERE email = $1 OR username = $2
    "#,
        usr.email,
        usr.username
    )
    .fetch_one(&db)
    .await?;

    let parsed_hash = PasswordHash::new(&info.hash_password).unwrap();

    let claim = Claims::new(info.id);

    if let Ok(_) = Pbkdf2.verify_password(&usr.password.as_bytes(), &parsed_hash) {
        let token = claim.to_jwt();

        return Ok(Json::from(LoginResponse { id: info.id, token }));
    } else {
        return Err(Error::NotFound);
    }
}

async fn get_following(
    claims: Claims,
    Extension(db): Extension<PgPool>,
) -> Result<Json<UserVec<UserPublic>>> {
    let following = sqlx::query_as!(
        UserPublic,
        r#"
        SELECT
            u.id,
            u.name,
            u.username
        FROM users u
        INNER JOIN follows f
        ON u.id = f.followed_id
        WHERE f.following_id = $1
    "#,
        claims.user_id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(UserVec { users: following }))
}

async fn get_followers(
    claims: Claims,
    Extension(db): Extension<PgPool>,
) -> Result<Json<UserVec<UserPublic>>> {
    let followers = sqlx::query_as!(
        UserPublic,
        r#"
        SELECT
            u.id,
            u.name,
            u.username
        FROM users u
        INNER JOIN follows f
        ON u.id = f.following_id
        WHERE f.followed_id = $1
    "#,
        claims.user_id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(UserVec { users: followers }))
}

async fn get_user_likes(claims: Claims, Extension(db): Extension<PgPool>) -> Result<Json<PostVec>> {
    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT 
            id,
            content,
            p.user_id,
            edited,
            created_at,
            last_updated_at
        FROM posts p
        INNER JOIN likes l
        ON l.post_id = p.id
        WHERE l.user_id = $1
    "#,
        claims.user_id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(PostVec { posts }))
}
