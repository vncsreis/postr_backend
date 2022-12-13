use axum::Router;

pub mod follow;
pub mod like;

pub fn router() -> Router {
    let likes_router = like::router();
    let follow_router = follow::router();

    Router::new().nest("/actions", likes_router.merge(follow_router))
}
