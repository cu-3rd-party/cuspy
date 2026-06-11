use crate::ApiContext;
use crate::api::models::db_uuid;
use clap::builder::TypedValueParser;
use log::info;
#[cfg(feature = "telegram-auth")]
use log::warn;
#[cfg(feature = "telegram-auth")]
use teloxide::{Bot, prelude::Requester, requests::ResponseResult, types::ChatId};
use uuid::Uuid;

#[cfg(feature = "telegram-auth")]
pub fn notify_telegram(telegram_id: i64, bot_token: String, message: String) {
    tokio::spawn(async move {
        if let Err(error) = send_message(telegram_id, &bot_token, message).await {
            warn!("telegram notification failed: {error}");
        }
    });
}

#[cfg(not(feature = "telegram-auth"))]
pub fn notify_telegram(_telegram_id: i64, _bot_token: String, _message: String) {}

// функция кидает пользователю
pub async fn notify_user(state: &ApiContext, user_id: Uuid, message: impl Into<String>) {
    let telegram_id = sqlx::query_scalar::<_, i64>(
        r#"select telegram_id from "user" where user_id = cast($1 as uuid)"#,
    )
    .bind(db_uuid(user_id))
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten();

    #[cfg(feature = "telegram-auth")]
    if let Some(telegram_id) = telegram_id {
        if let Some(bot_token) = state.telegram_bot_token.clone() {
            notify_telegram(telegram_id, bot_token, message.into());
        }
    }

    #[cfg(not(feature = "telegram-auth"))]
    {
        info!(
            "notification to user_id={}: {}",
            user_id.to_string(),
            message.into()
        );
    }
}

pub async fn notify_admins(state: &ApiContext, message: impl Into<String>) {
    let message = message.into();
    let recipients = sqlx::query_scalar::<_, i64>(
        r#"
        select telegram_id
        from "user"
        where is_admin = true
        "#,
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    #[cfg(feature = "telegram-auth")]
    for telegram_id in recipients {
        if let Some(bot_token) = state.telegram_bot_token.clone() {
            notify_telegram(telegram_id, bot_token, message.clone());
        }
    }

    #[cfg(not(feature = "telegram-auth"))]
    {
        let _ = state;
        let _ = message;
        let _ = recipients;
    }
}

#[cfg(feature = "telegram-auth")]
async fn send_message(telegram_id: i64, bot_token: &str, message: String) -> ResponseResult<()> {
    Bot::new(bot_token)
        .send_message(ChatId(telegram_id), message)
        .await?;

    Ok(())
}
