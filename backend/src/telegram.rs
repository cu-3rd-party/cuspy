use hmac::digest::Digest;
use hmac::{Hmac, Mac};
use http::HeaderValue;
use serde::Deserialize;
use sha2::Sha256;
use url::form_urlencoded;

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
        let mut mac = HmacSha256::new_from_slice(secret.as_slice()).ok()?;
        mac.update(data_check_string.as_bytes());
        let expected_hash = hex::encode(mac.finalize().into_bytes());

        let provided_hash = hash?;

        if expected_hash != provided_hash {
            return None;
        }
        Some(user_json?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::form_urlencoded::byte_serialize;

    impl TelegramInitData {
        fn to_init_data(&self, token: &str) -> String {
            let user = serde_json::to_string(&self.user).expect("serialize telegram user");
            let user_encoded = byte_serialize(user.as_bytes()).collect::<String>();

            let mut data_pairs = vec![
                "auth_date=1700000000".to_string(),
                "query_id=test-query".to_string(),
                format!("user={user}"),
            ];
            data_pairs.sort();

            let data_check_string = data_pairs.join("\n");
            let secret = Sha256::digest(token.as_bytes());
            let mut mac = HmacSha256::new_from_slice(secret.as_slice()).expect("hmac key");
            mac.update(data_check_string.as_bytes());
            let hash = hex::encode(mac.finalize().into_bytes());

            format!("query_id=test-query&user={user_encoded}&auth_date=1700000000&hash={hash}")
        }
    }

    #[test]
    fn parses_init_data_with_valid_hash() {
        let token = "test-bot-token";
        let init_data = TelegramInitData {
            user: TelegramUser { id: 42 },
        };

        let header = init_data.to_init_data(token);
        let parsed = TelegramInitData::from_header(token, &header).expect("valid init data");

        assert_eq!(parsed.user.id, 42);
    }

    #[test]
    fn rejects_init_data_with_invalid_hash() {
        let token = "test-bot-token";
        let init_data = TelegramInitData {
            user: TelegramUser { id: 42 },
        };

        let mut header = init_data.to_init_data(token);
        header.push('0');

        assert!(TelegramInitData::from_header(token, &header).is_none());
    }
}
