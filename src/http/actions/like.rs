use crate::http::Result;
use crate::utils::jwt::Claims;
use axum::{extract::Path, routing::get, Extension, Json, Router};
use sqlx::{types::Uuid, PgPool};

pub fn router() -> Router {
    Router::new()
        .route("/like/:id", get(like_post))
        .route("/unlike/:id", get(unlike_post))
}

async fn like_post(
    claims: Claims,
    Path(id): Path<Uuid>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<bool>> {
    let liked = sqlx::query!(
        r#"
        INSERT INTO likes (user_id, post_id) VALUES($1, $2)
    "#,
        claims.user_id,
        id
    )
    .execute(&db)
    .await?
    .rows_affected()
        > 0;

    Ok(Json::from(liked))
}

async fn unlike_post(
    claims: Claims,
    Path(id): Path<Uuid>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<bool>> {
    let unliked = sqlx::query!(
        r#"
        DELETE FROM likes WHERE user_id = $1 AND post_id = $2
    "#,
        claims.user_id,
        id
    )
    .execute(&db)
    .await?
    .rows_affected()
        > 0;

    Ok(Json::from(unliked))
}
