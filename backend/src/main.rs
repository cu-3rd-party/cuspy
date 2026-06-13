use clap::Parser;
use cukiller_backend::{ApiContext, build_service, config};
use cukiller_backend::models::profile::ProfileRequestEvent;
use tokio::sync::broadcast;
use log::info;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, BucketConfiguration, Region};
use sqlx::{AnyPool, any::AnyPoolOptions, migrate::Migrator};
use std::path::Path;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, WebAppInfo},
};
use url::Url;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

async fn connect_database(database_url: &str) -> Result<AnyPool, Box<dyn std::error::Error>> {
    Ok(AnyPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?)
}

const MIGRATION_ROOT: &str = "./migrations";
async fn run_migrations(db: &AnyPool) -> Result<(), Box<dyn std::error::Error>> {
    Migrator::new(Path::new(MIGRATION_ROOT))
        .await?
        .run(db)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    sqlx::any::install_default_drivers();

    let config = config::Config::parse();
    let database_url = &config.database_url;
    let db = connect_database(&database_url).await?;

    run_migrations(&db).await?;

    let credentials = Credentials {
        access_key: Some(config.access_key.clone()),
        secret_key: Some(config.secret_key.clone()),
        security_token: None,
        session_token: None,
        expiration: None,
    };

    let region = Region::Custom {
        region: config.region.clone(),
        endpoint: config.endpoint.clone(),
    };

    let bucket =
        Bucket::new(&config.bucket_name, region.clone(), credentials.clone())?.with_path_style();
    match bucket.head_object("").await {
        Ok(_) => Ok::<(), anyhow::Error>(()),
        Err(S3Error::HttpFailWithBody(404, _)) => {
            Bucket::create_with_path_style(
                &config.bucket_name,
                region.clone(),
                credentials.clone(),
                BucketConfiguration::default(),
            )
            .await?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }?;

    let (profile_request_tx, _) = broadcast::channel::<ProfileRequestEvent>(256);

    let state = ApiContext {
        db,
        bucket,
        config: config.clone(), // просто на всякий случай
        admin_secret: config.admin_secret,
        jwt_secret: config.jwt_secret,
        profile_request_tx,
        #[cfg(feature = "telegram-auth")]
        telegram_bot_token: config.telegram_bot_token.clone(),
        #[cfg(feature = "telegram-auth")]
        public_webapp_url: config.public_webapp_url.clone(),
    };
    let service = build_service(state);

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("backend listening on {}", config.bind_address);
    let server = axum::serve(
        listener,
        service.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    );

    #[cfg(feature = "telegram-auth")]
    {
        tokio::select! {
            result = server => {
                result?;
            }
            _ = run_bot(config.telegram_bot_token, config.public_webapp_url) => {}
        }
    }

    #[cfg(not(feature = "telegram-auth"))]
    {
        server.await?;
    }

    Ok(())
}
