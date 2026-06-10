use std::process::Command;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
};
#[cfg(not(feature = "telegram-auth"))]
use cukiller_backend::api::helpers;
use cukiller_backend::{ApiContext, build_app};
use http_body_util::BodyExt;
use serde_json::{Value, json};
#[cfg(not(feature = "telegram-auth"))]
use sqlx::Row;
use sqlx::any::AnyPoolOptions;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;
use url::Url;

pub const ADMIN_SECRET: &str = "test-admin-secret";
pub const JWT_SECRET: &str = "test-jwt-secret";
#[cfg(feature = "telegram-auth")]
pub const TELEGRAM_BOT_TOKEN: &str = "test-bot-token";

pub struct TestContext {
    pub app: Router,
    #[cfg_attr(feature = "telegram-auth", allow(dead_code))]
    pub db: sqlx::PgPool,
    #[cfg_attr(feature = "telegram-auth", allow(dead_code))]
    pub admin_secret: String,
    db_name: String,
    docker_container_name: Option<String>,
    admin_database_url: String,
}

impl TestContext {
    pub async fn new() -> Self {
        sqlx::any::install_default_drivers();

        let mut docker_container_name = None;
        let admin_database_url =
            match std::env::var("TEST_DATABASE_URL").or_else(|_| std::env::var("DATABASE_URL")) {
                Ok(url) => url,
                Err(_) => {
                    let suffix = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("system time")
                        .as_nanos();
                    let container_name = format!("cukiller-backend-test-pg-{suffix}");
                    let output = Command::new("docker")
                        .args([
                            "run",
                            "--rm",
                            "-d",
                            "--name",
                            &container_name,
                            "-e",
                            "POSTGRES_DB=postgres",
                            "-e",
                            "POSTGRES_USER=postgres",
                            "-e",
                            "POSTGRES_PASSWORD=postgres",
                            "-P",
                            "postgres:18beta2",
                        ])
                        .output()
                        .expect("start temporary postgres container");
                    assert!(
                        output.status.success(),
                        "docker run failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );

                    let port_output = Command::new("docker")
                        .args(["port", &container_name, "5432/tcp"])
                        .output()
                        .expect("inspect postgres port");
                    assert!(
                        port_output.status.success(),
                        "docker port failed: {}",
                        String::from_utf8_lossy(&port_output.stderr)
                    );
                    let port_mapping = String::from_utf8(port_output.stdout).expect("port utf8");
                    let host_port = port_mapping
                        .trim()
                        .rsplit(':')
                        .next()
                        .expect("extract host port");

                    docker_container_name = Some(container_name);
                    format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres")
                }
            };

        let mut admin_pool = None;
        for _ in 0..30 {
            match PgPoolOptions::new()
                .max_connections(5)
                .connect(&admin_database_url)
                .await
            {
                Ok(pool) => {
                    admin_pool = Some(pool);
                    break;
                }
                Err(_) => tokio::time::sleep(std::time::Duration::from_secs(1)).await,
            }
        }

        let admin_pool = admin_pool.expect("connect admin database");

        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let db_name = format!("cukiller_backend_test_{suffix}");
        let create_sql = format!(r#"create database "{db_name}""#);
        sqlx::query(&create_sql)
            .execute(&admin_pool)
            .await
            .expect("create test database");

        let mut db_url = sqlx::postgres::PgConnectOptions::from_str(&admin_database_url)
            .expect("parse database url");
        db_url = db_url.database(&db_name);
        let test_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(db_url)
            .await
            .expect("connect test database");
        let mut any_url = Url::parse(&admin_database_url).expect("parse admin database url");
        any_url.set_path(&format!("/{db_name}"));
        let any_test_pool = AnyPoolOptions::new()
            .max_connections(5)
            .connect(any_url.as_ref())
            .await
            .expect("connect any test database");

        sqlx::migrate!("./migrations")
            .run(&test_pool)
            .await
            .expect("run migrations");

        let state = ApiContext {
            db: any_test_pool,
            admin_secret: ADMIN_SECRET.to_string(),
            jwt_secret: JWT_SECRET.to_string(),
            #[cfg(feature = "telegram-auth")]
            telegram_bot_token: Some(TELEGRAM_BOT_TOKEN.to_string()),
            #[cfg(feature = "telegram-auth")]
            public_webapp_url: Some("https://test.example.com".to_string()),
        };

        Self {
            app: build_app(state),
            db: test_pool,
            db_name,
            admin_secret: ADMIN_SECRET.to_string(),
            docker_container_name,
            admin_database_url,
        }
    }

    pub async fn json(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
        bearer: Option<&str>,
        admin_secret: Option<&str>,
        telegram_init_data: Option<&str>,
    ) -> (StatusCode, Value) {
        let mut builder = Request::builder().method(method).uri(path);
        builder = builder.header(header::CONTENT_TYPE, "application/json");

        if let Some(token) = bearer {
            builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
        }

        if let Some(secret) = admin_secret {
            builder = builder.header("x-admin-secret", secret);
        }

        if let Some(init_data) = telegram_init_data {
            builder = builder.header("x-telegram-init-data", init_data);
        }

        let request = builder
            .body(match body {
                Some(value) => Body::from(serde_json::to_vec(&value).expect("serialize body")),
                None => Body::empty(),
            })
            .expect("build request");

        let response = self
            .app
            .clone()
            .oneshot(request)
            .await
            .expect("router response");
        let status = response.status();
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("read body")
            .to_bytes();
        let json = if bytes.is_empty() {
            Value::Null
        } else if matches!(content_type.as_deref(), Some(value) if value.starts_with("application/json"))
        {
            serde_json::from_slice(&bytes).expect("parse json body")
        } else {
            Value::String(String::from_utf8(bytes.to_vec()).expect("utf8 body"))
        };

        (status, json)
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let db_name = self.db_name.clone();
        let admin_database_url = self.admin_database_url.clone();
        let docker_container_name = self.docker_container_name.clone();

        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("drop runtime");
            runtime.block_on(async move {
                let pool = PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&admin_database_url)
                    .await
                    .expect("connect admin database for cleanup");

                let terminate_sql = format!(
                    "select pg_terminate_backend(pid) from pg_stat_activity where datname = '{db_name}' and pid <> pg_backend_pid()"
                );
                sqlx::query(&terminate_sql)
                    .execute(&pool)
                    .await
                    .expect("terminate connections");

                let drop_sql = format!(r#"drop database if exists "{db_name}""#);
                sqlx::query(&drop_sql)
                    .execute(&pool)
                    .await
                    .expect("drop test database");

                if let Some(container_name) = docker_container_name {
                    let output = Command::new("docker")
                        .args(["stop", &container_name])
                        .output()
                        .expect("stop postgres container");
                    assert!(output.status.success(), "docker stop failed: {}", String::from_utf8_lossy(&output.stderr));
                }
            });
        })
        .join()
        .expect("cleanup thread");
    }
}

pub async fn register_user(
    ctx: &TestContext,
    email: &str,
    telegram_id: i64,
    agent_name: &str,
    telegram_init_data: Option<&str>,
) -> (String, Value) {
    let (status, body) = ctx
        .json(
            "POST",
            "/auth/register",
            Some(json!({
                "email": email,
                "password": "password123",
                "telegram_id": telegram_id,
                "rating": 42,
                "agent_name": agent_name,
                "agent_data": { "track": "backend", "city": "Kyiv" }
            })),
            None,
            None,
            telegram_init_data,
        )
        .await;
    assert_eq!(status, StatusCode::CREATED, "register_user body: {body}");
    let token = body["access_token"].as_str().expect("token").to_string();
    (token, body["user"].clone())
}

#[cfg(not(feature = "telegram-auth"))]
pub async fn seed_admin_user(
    ctx: &TestContext,
    email: &str,
    telegram_id: i64,
    agent_name: &str,
) -> String {
    let user_id = uuid::Uuid::now_v7();
    sqlx::query(
        r#"
        insert into "user" (user_id, telegram_id, agent_name, is_admin)
        values ($1, $2, $3, true)
        "#,
    )
    .bind(user_id)
    .bind(telegram_id)
    .bind(agent_name)
    .execute(&ctx.db)
    .await
    .expect("insert admin user");

    sqlx::query(
        r#"
        insert into rating_history (rating_history_id, user_id, rating, change, reason)
        values ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(uuid::Uuid::now_v7())
    .bind(user_id)
    .bind(helpers::DEFAULT_RATING)
    .bind(helpers::DEFAULT_RATING)
    .bind("initial_rating")
    .execute(&ctx.db)
    .await
    .expect("insert admin rating history");

    sqlx::query(
        r#"
        insert into auth_user (auth_user_id, user_id, login_identifier, password_hash)
        values ($1, $2, $3, $4)
        "#,
    )
    .bind(uuid::Uuid::now_v7())
    .bind(user_id)
    .bind(email)
    .bind(Some(
        helpers::hash_password("password123").expect("hash password"),
    ))
    .execute(&ctx.db)
    .await
    .expect("insert admin auth user");

    let (status, body) = ctx
        .json(
            "POST",
            "/auth/login",
            Some(json!({ "email": email, "password": "password123" })),
            None,
            None,
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    body["access_token"]
        .as_str()
        .expect("admin token")
        .to_string()
}

#[cfg(not(feature = "telegram-auth"))]
pub async fn fetch_user_agent_data(ctx: &TestContext, user_id: &str) -> Value {
    sqlx::query(r#"select * from "agent_data" where agent_data_id = (select agent_data_id from "user" where user_id = $1)"#)
        .bind(uuid::Uuid::parse_str(user_id).expect("uuid"))
        .fetch_one(&ctx.db)
        .await
        .expect("fetch user agent data")
        .get::<Value, _>(0)
}

#[cfg(not(feature = "telegram-auth"))]
pub async fn fetch_latest_audit_actor(ctx: &TestContext, matched_path: &str) -> Option<String> {
    sqlx::query(
        r#"
        select actor_user_id
        from audit_log
        where matched_path = $1
        order by created_at desc, audit_log_id desc
        limit 1
        "#,
    )
    .bind(matched_path)
    .fetch_optional(&ctx.db)
    .await
    .expect("fetch audit actor")
    .and_then(|row| {
        row.try_get::<Option<uuid::Uuid>, _>(0)
            .expect("audit actor uuid")
    })
    .map(|value| value.to_string())
}
