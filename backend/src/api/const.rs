use std::time::Duration;

pub const AUTH_HEADER_PREFIX: &str = "Bearer ";
pub const USER_TOKEN_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 7);
