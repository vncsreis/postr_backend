use super::posts::{Post, PostVec};
use super::Result;
use crate::utils::jwt::Claims;
use axum::routing::get;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

pub fn router() -> Router {
    Router::new().route("/feed", get(get_user_feed))
}

async fn get_user_feed(claims: Claims, Extension(db): Extension<PgPool>) -> Result<Json<PostVec>> {
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
        INNER JOIN follows f
        ON f.followed_id = p.user_id
        WHERE f.following_id = $1
        ORDER BY p.created_at DESC
    "#,
        claims.user_id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json::from(PostVec { posts }))
}
