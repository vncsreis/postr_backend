use self::error::Error;
use anyhow::Context;
use axum::{Extension, Router};
use http::Method;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub mod actions;
pub mod error;
pub mod feed;
pub mod posts;
pub mod users;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let app = api_router().layer(Extension(db)).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    let api_routes = Router::new()
        .merge(posts::router())
        .merge(users::router())
        .merge(actions::router())
        .merge(feed::router());

    Router::new().nest("/api", api_routes)
}
