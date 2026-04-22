use clap::Parser;
use cukiller_backend::{AppState, build_app, config};
use log::info;
use sqlx::postgres::PgPoolOptions;
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
        telegram_bot_token: config.telegram_bot_token.clone(),
        #[cfg(feature = "telegram-auth")]
        public_webapp_url: config.public_webapp_url.clone(),
    };
    let app = build_app(state);
    let bot_token = config.telegram_bot_token;
    let webapp_url = config.public_webapp_url;

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    info!("backend listening on {}", config.bind_address);
    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    );

    tokio::select! {
        result = server => {
            result?;
        }
        _ = run_bot(bot_token, webapp_url) => {}
    }

    Ok(())
}
