pub mod api;
pub mod config;
pub mod notifier;

use std::time::Instant;

use axum::{
    Router,
    extract::{MatchedPath, Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::{self, Next},
    response::Response,
};
use log::error;
use serde_json::{Value, json};
use sqlx::AnyPool;
use uuid::Uuid;
use crate::api::models::{db_json, db_optional_uuid, db_uuid};

#[derive(Clone)]
pub struct AppState {
    pub db: AnyPool,
    pub is_sqlite: bool,
    pub admin_secret: String,
    pub jwt_secret: String,
    #[cfg(feature = "telegram-auth")]
    pub telegram_bot_token: Option<String>,
    #[cfg(feature = "telegram-auth")]
    pub public_webapp_url: Option<String>,
}

impl AppState {
    pub fn db_param<'a>(&self, sqlite_sql: &'a str, postgres_sql: &'a str) -> &'a str {
        if self.is_sqlite {
            sqlite_sql
        } else {
            postgres_sql
        }
    }

    #[cfg(feature = "telegram-auth")]
    pub fn telegram_auth_enabled(&self) -> bool {
        self.telegram_bot_token.is_some()
    }
}

pub fn build_app(state: AppState) -> Router {
    api::router()
        .route("/", axum::routing::get(|| async { "backend up" }))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(state.clone(), audit_request))
        .with_state(state)
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
    let actor_user_id = crate::api::helpers::require_bearer_token(&headers, &state)
        .ok()
        .map(|auth| auth.user_id)
        .filter(|user_id| *user_id != Uuid::nil());
    let user_agent = header_to_string(&headers, header::USER_AGENT);
    let forwarded_for =
        header_to_string(&headers, header::HeaderName::from_static("x-forwarded-for"));
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
        state.is_sqlite,
        AuditLogInsert {
            request_id,
            actor_user_id,
            method,
            request_uri: uri,
            matched_path,
            status,
            duration_ms: elapsed_ms,
            access_context,
        },
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

struct AuditLogInsert {
    request_id: Uuid,
    actor_user_id: Option<Uuid>,
    method: String,
    request_uri: String,
    matched_path: Option<String>,
    status: StatusCode,
    duration_ms: i64,
    access_context: Value,
}

async fn persist_audit_log(db: &AnyPool, is_sqlite: bool, entry: AuditLogInsert) -> Result<(), sqlx::Error> {
    sqlx::query(if is_sqlite {
        r#"
        insert into audit_log (
            audit_log_id,
            request_id,
            actor_user_id,
            method,
            request_uri,
            matched_path,
            status_code,
            duration_ms,
            access_context
        )
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#
    } else {
        r#"
        insert into audit_log (
            audit_log_id,
            request_id,
            actor_user_id,
            method,
            request_uri,
            matched_path,
            status_code,
            duration_ms,
            access_context
        )
        values (cast($1 as uuid), cast($2 as uuid), cast($3 as uuid), $4, $5, $6, $7, $8, cast($9 as jsonb))
        "#
    })
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(entry.request_id))
    .bind(db_optional_uuid(entry.actor_user_id))
    .bind(entry.method)
    .bind(entry.request_uri)
    .bind(entry.matched_path)
    .bind(i32::from(entry.status.as_u16()))
    .bind(entry.duration_ms)
    .bind(db_json(&entry.access_context))
    .execute(db)
    .await?;

    Ok(())
}
