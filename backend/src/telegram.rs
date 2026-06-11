use http::HeaderValue;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hmac::digest::Digest;
use url::form_urlencoded;
use serde::Deserialize;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TelegramInitData {
    pub user: TelegramUser,
    // если в будущем потребуется больше вычленять, сюда можно
    // добавить еще полей (язык, регион итп). пока тут только нужное
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TelegramUser {
    pub id: i64,
}

impl TelegramInitData {
    pub fn from_header(token: &str, init_data_str: &str) -> Option<Self> {
        // Достаем и проверяем json
        let user_json = Self::extract_user_json(token, init_data_str)?;

        // Парсим и отправляем пользователю обратно
        Some(Self {
            user: serde_json::from_str(&user_json).ok()?,
        })
    }

    fn extract_user_json(token: &str, init_data_str: &str) -> Option<String> {
        let mut hash = None;
        let mut user_json = None;
        let mut data_pairs = Vec::new();

        // парсим строчку хедера в Vec<String> для проверки хэша
        for (key, value) in form_urlencoded::parse(init_data_str.as_bytes()) {
            if key == "hash" {
                hash = Some(value.into_owned());
            } else {
                if key == "user" {
                    user_json = Some(value.clone().into_owned());
                }
                data_pairs.push(format!("{key}={value}"));
            }
        }

        // сверяем хэши
        data_pairs.sort();
        let data_check_string = data_pairs.join("\n");
        let secret = Sha256::digest(token);
        let mut mac =
            HmacSha256::new_from_slice(secret.as_slice()).ok()?;
        mac.update(data_check_string.as_bytes());
        let expected_hash = hex::encode(mac.finalize().into_bytes());

        let provided_hash = hash?;

        if expected_hash != provided_hash {
            return None
        }
        Some(user_json?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl TelegramInitData {
        fn to_init_data(self) -> String {
            todo!()
        }
    }

    // TODO: add test that creates a telegram init data with a valid hash and checks
    // TODO: add test that creates a telegram init data with an invalid hash and checks
}