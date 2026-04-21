use clap::Parser;
use cukiller_backend::{AppState, build_app, config};
use log::info;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config = config::Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&db).await?;

    let state = AppState {
        db,
        admin_secret: config.admin_secret,
        jwt_secret: config.jwt_secret,
        #[cfg(feature = "telegram-auth")]
        telegram_bot_token: config.telegram_bot_token,
    };
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("backend listening on {}", config.bind_address);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
