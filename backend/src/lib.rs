pub mod config;
pub mod db;
pub mod grpc;
pub mod models;
pub mod notifier;
pub mod rest;

mod r#const;
#[cfg(feature = "telegram-auth")]
pub mod telegram;

use std::time::{Duration, Instant};

use crate::config::Config;
use crate::models::user::User;
use crate::models::{db_json, db_optional_uuid, db_uuid};
use crate::rest::extractor::MaybeAuthUser;
use axum::{
    Router,
    extract::{MatchedPath, Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::{self, Next},
    response::Response,
};
use axum_tonic::RestGrpcService;
use http::{HeaderValue, Method};
use log::{error, info, warn};
use rest::docs;
use s3::Bucket;
use serde_json::{Value, json};
use sqlx::AnyPool;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

#[derive(Clone)]
pub struct ApiContext {
    pub db: AnyPool,
    pub bucket: Box<Bucket>,
    pub config: Config,
    pub admin_secret: String,
    pub jwt_secret: String,
    #[cfg(feature = "telegram-auth")]
    pub telegram_bot_token: String,
    #[cfg(feature = "telegram-auth")]
    pub public_webapp_url: String,
}

fn build_cors_layer(state: &ApiContext) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(HeaderValue::from_str(&state.config.cors_origin).expect("cors origin"))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true)
        .max_age(Duration::from_hours(24))
}

pub fn build_rest(state: ApiContext) -> Router {
    let router1 = Router::new()
        .merge(docs::docs_router())
        .nest(
            "/api",
            rest::router()
                .route("/", axum::routing::get(rest::root)),
        )
        .layer(build_cors_layer(&state))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(state.clone(), audit_request))
        .with_state(state);
    info!("{:?}", router1);
    router1
}

pub fn build_grpc(state: ApiContext) -> Router {
    Router::new()
        .nest("/api", grpc::router(state.clone()))
        .layer(build_cors_layer(&state))
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .layer(middleware::from_fn(move |request, next| {
            audit_grpc_request(state.clone(), request, next)
        }))
}

pub fn build_service(state: ApiContext) -> RestGrpcService {
    RestGrpcService::new(build_rest(state.clone()), build_grpc(state.clone()))
}

async fn audit_request(
    State(state): State<ApiContext>,
    MaybeAuthUser(user): MaybeAuthUser,
    request: Request,
    next: Next,
) -> Response {
    audit_request_inner(state, user, request, next).await
}

async fn audit_grpc_request(state: ApiContext, request: Request, next: Next) -> Response {
    let user = request
        .extensions()
        .get::<Option<User>>()
        .cloned()
        .flatten();

    audit_request_inner(state, user, request, next).await
}

async fn audit_request_inner(
    state: ApiContext,
    user: Option<User>,
    request: Request,
    next: Next,
) -> Response {
    let started_at = Instant::now();
    let request_id = Uuid::now_v7();

    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_owned());

    let method = request.method().to_string();
    let uri = request.uri().to_string();
    let headers = request.headers().clone();
    let actor_user_id = user
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

    if status.as_u16() >= 200 && status.as_u16() < 300 {
        info!(
            "{:<7} | {:<50} | {:>3} | {:>2}ms",
            method, uri, status, elapsed_ms
        );
    } else {
        warn!(
            "{:<7} | {:<50} | {:>3} | {:>2}ms",
            method, uri, status, elapsed_ms
        );
    }

    if let Err(error) = persist_audit_log(
        &state.db,
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

async fn persist_audit_log(db: &AnyPool, entry: AuditLogInsert) -> Result<(), sqlx::Error> {
    sqlx::query(r#"
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
        "#)
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
