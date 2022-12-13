use anyhow::Context;
use postr_backend::http;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = match dotenvy::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("No DATABASE_URL env var"),
    };

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .context(format!("could not connect to database: {}", &db_url))?;

    sqlx::migrate!().run(&db).await?;

    http::serve(db).await?;

    Ok(())
}
