mod api;
mod config;

use std::time::Instant;

use axum::{
    extract::{MatchedPath, Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::{self, Next},
    response::Response,
};
use clap::Parser;
use log::{error, info};
use serde_json::{Value, json};
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub admin_secret: String,
    pub jwt_secret: String,
    #[cfg(feature = "telegram-auth")]
    pub telegram_bot_token: String,
}

async fn audit_request(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let started_at = Instant::now();
    let request_id = Uuid::now_v7();

    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_owned());

    let method = request.method().to_string();
    let uri = request.uri().to_string();
    let headers = request.headers().clone();
    let user_agent = header_to_string(&headers, header::USER_AGENT);
    let forwarded_for = header_to_string(&headers, header::HeaderName::from_static("x-forwarded-for"));
    let real_ip = header_to_string(&headers, header::HeaderName::from_static("x-real-ip"));
    let referer = header_to_string(&headers, header::REFERER);
    let origin = header_to_string(&headers, header::ORIGIN);
    let remote_addr = request
        .extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|connect_info| connect_info.0.to_string());

    let response = next.run(request).await;
    let status = response.status();
    let elapsed = started_at.elapsed();
    let elapsed_ms = i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX);

    let access_context = json!({
        "forwarded_for": forwarded_for,
        "real_ip": real_ip,
        "remote_addr": remote_addr,
        "user_agent": user_agent,
        "referer": referer,
        "origin": origin
    });

    if let Err(error) = persist_audit_log(
        &state.db,
        request_id,
        method,
        uri,
        matched_path,
        status,
        elapsed_ms,
        access_context,
    )
    .await
    {
        error!("failed to persist audit log: {error}");
    }

    response
}

fn header_to_string(headers: &HeaderMap, name: header::HeaderName) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

async fn persist_audit_log(
    db: &PgPool,
    request_id: Uuid,
    method: String,
    request_uri: String,
    matched_path: Option<String>,
    status: StatusCode,
    duration_ms: i64,
    access_context: Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        insert into audit_log (
            audit_log_id,
            request_id,
            method,
            request_uri,
            matched_path,
            status_code,
            duration_ms,
            access_context
        )
        values ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(Uuid::now_v7())
    .bind(request_id)
    .bind(method)
    .bind(request_uri)
    .bind(matched_path)
    .bind(i32::from(status.as_u16()))
    .bind(duration_ms)
    .bind(access_context)
    .execute(db)
    .await?;

    Ok(())
}

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
    let app = api::router()
        .route("/", axum::routing::get(|| async { "backend up" }))
        .layer(middleware::from_fn_with_state(state.clone(), audit_request))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("backend listening on {}", config.bind_address);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
