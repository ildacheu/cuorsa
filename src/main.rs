mod http;
mod routes;
mod services;
mod structs;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    // Debug
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initialized router!");
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    info!("{}", url);

    let sqlx_connection = PgPoolOptions::new()
        .max_connections(95) // TODO: Set this to a reasonable value
        .connect(&url)
        .await
        .unwrap();

    sqlx::migrate!("./src/migrations")
        .run(&sqlx_connection)
        .await?;

    http::serve(sqlx_connection).await?;

    Ok(())
}
