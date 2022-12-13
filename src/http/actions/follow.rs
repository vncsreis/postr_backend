use crate::http::Result;
use crate::utils::jwt::Claims;
use axum::{extract::Path, routing::get, Extension, Json, Router};
use sqlx::{types::Uuid, PgPool};

pub fn router() -> Router {
    Router::new()
        .route("/follow/:id", get(follow_user))
        .route("/unfollow/:id", get(unfollow_user))
}

async fn follow_user(
    claims: Claims,
    Path(id): Path<Uuid>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<bool>> {
    let followed = sqlx::query!(
        r#"
        INSERT INTO follows (following_id, followed_id) VALUES($1, $2)
    "#,
        claims.user_id,
        id
    )
    .execute(&db)
    .await?
    .rows_affected()
        > 0;

    Ok(Json::from(followed))
}

async fn unfollow_user(
    claims: Claims,
    Path(id): Path<Uuid>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<bool>> {
    let unfollowed = sqlx::query!(
        r#"
        DELETE FROM follows WHERE following_id = $1 AND followed_id = $2
    "#,
        claims.user_id,
        id
    )
    .execute(&db)
    .await?
    .rows_affected()
        > 0;

    Ok(Json::from(unfollowed))
}
