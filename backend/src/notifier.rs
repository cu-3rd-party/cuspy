use crate::ApiContext;
use crate::models::auth::AuthUserRecord;
use crate::models::user::User;
#[cfg(feature = "telegram")]
use log::warn;
use log::{error, info};
#[cfg(feature = "telegram")]
use teloxide::{Bot, prelude::Requester, requests::ResponseResult, types::ChatId};
use uuid::Uuid;

#[cfg(feature = "telegram")]
pub fn notify_telegram(telegram_id: i64, bot_token: String, message: String) {
    tokio::spawn(async move {
        send_message(telegram_id, &bot_token, message.clone())
            .await
            .map_err(|e| {
                warn!("telegram send message failed {e}");
            })
    });
}

// функция кидает пользователю
pub async fn notify_user(state: &ApiContext, user_id: Uuid, message: String) {
    let Some(_auth_user_records) = AuthUserRecord::get_by_user_id(&state.db, user_id).await else {
        return;
    };

    #[cfg(feature = "telegram")]
    {
        for telegram_id in _auth_user_records.into_iter().filter_map(|x| x.telegram_id) {
            notify_telegram(
                telegram_id,
                state.telegram_bot_token.clone(),
                message.clone(),
            );
        }
    }

    info!(
        "notification to user_id={}: {}",
        user_id.to_string(),
        message
    );
}

pub async fn notify_admins(state: &ApiContext, message: String) {
    let Ok(admins) = User::list_by_admin(&state.db, true).await else {
        error!("failed to notify admins");
        return;
    };

    let admin_user_ids: Vec<Uuid> = admins.iter().map(|x1| x1.user_id).collect();
    let admins_table = AuthUserRecord::get_by_user_ids(&state.db, &admin_user_ids).await;
    let _admin_auth_users: Vec<&AuthUserRecord> = admins_table
        .iter()
        .map(|(_, auth_user_record)| auth_user_record)
        .collect();

    #[cfg(feature = "telegram")]
    {
        for telegram_id in _admin_auth_users.into_iter().filter_map(|x| x.telegram_id) {
            notify_telegram(
                telegram_id,
                state.telegram_bot_token.clone(),
                message.clone(),
            );
        }
    }

    info!(
        "notification to {} admins: {}",
        admins_table.len(),
        message,
    );
}

#[cfg(feature = "telegram")]
async fn send_message(telegram_id: i64, bot_token: &str, message: String) -> ResponseResult<()> {
    Bot::new(bot_token)
        .send_message(ChatId(telegram_id), message)
        .await?;

    Ok(())
}
