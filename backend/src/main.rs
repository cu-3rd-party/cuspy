use clap::Parser;
use cukiller_backend::{AppState, build_app, config};
use log::info;
use sqlx::{AnyPool, any::AnyPoolOptions, migrate::Migrator};
use std::path::Path;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, WebAppInfo},
};
use url::Url;

async fn handle_bot_message(bot: Bot, message: Message, webapp_url: String) -> ResponseResult<()> {
    if let Some(text) = message.text()
        && text.starts_with("/start")
    {
        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::web_app(
            "Open web app",
            WebAppInfo {
                url: Url::parse(&webapp_url).expect("valid webapp url"),
            },
        )]]);

        bot.send_message(
            message.chat.id,
            "Bot is alive. Use the button below to open the web app inside Telegram.",
        )
        .reply_markup(keyboard)
        .await?;
    }

    Ok(())
}

async fn run_bot(bot_token: String, webapp_url: String) {
    let bot = Bot::new(bot_token);
    let handler = Update::filter_message().endpoint(move |bot: Bot, message: Message| {
        let webapp_url = webapp_url.clone();
        async move { handle_bot_message(bot, message, webapp_url).await }
    });

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn normalize_database_url(database_url: &str) -> (String, bool) {
    if database_url == ":memory:" {
        return ("sqlite::memory:".into(), true);
    }

    if database_url.starts_with("sqlite:") {
        return (database_url.into(), true);
    }

    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        return (database_url.into(), false);
    }

    if database_url.contains("://") {
        return (database_url.into(), false);
    }

    (format!("sqlite://{database_url}"), true)
}

fn ensure_sqlite_path_exists(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let Some(path) = database_url.strip_prefix("sqlite://") else {
        return Ok(());
    };

    if path.is_empty() || path == ":memory:" {
        return Ok(());
    }

    let path = Path::new(path);
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        std::fs::File::create(path)?;
    }

    Ok(())
}

async fn connect_database(
    database_url: &str,
    is_sqlite: bool,
) -> Result<AnyPool, Box<dyn std::error::Error>> {
    if is_sqlite {
        ensure_sqlite_path_exists(database_url)?;
    }

    Ok(AnyPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?)
}

async fn run_migrations(db: &AnyPool, is_sqlite: bool) -> Result<(), Box<dyn std::error::Error>> {
    let migration_root = if is_sqlite {
        "./migrations/sqlite"
    } else {
        "./migrations"
    };

    Migrator::new(Path::new(migration_root)).await?.run(db).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    sqlx::any::install_default_drivers();

    let config = config::Config::parse();
    let (database_url, is_sqlite) = normalize_database_url(&config.database_url);
    let db = connect_database(&database_url, is_sqlite).await?;

    run_migrations(&db, is_sqlite).await?;

    let state = AppState {
        db,
        admin_secret: config.admin_secret,
        jwt_secret: config.jwt_secret,
        #[cfg(feature = "telegram-auth")]
        telegram_bot_token: config.telegram_bot_token.clone(),
        #[cfg(feature = "telegram-auth")]
        public_webapp_url: config.public_webapp_url.clone(),
    };
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("backend listening on {}", config.bind_address);
    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    );

    if let (Some(bot_token), Some(webapp_url)) = (config.telegram_bot_token, config.public_webapp_url)
    {
        tokio::select! {
            result = server => {
                result?;
            }
            _ = run_bot(bot_token, webapp_url) => {}
        }
    } else {
        server.await?;
    }

    Ok(())
}
