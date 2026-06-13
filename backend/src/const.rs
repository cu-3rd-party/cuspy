use std::time::Duration;

pub const AUTH_TOKEN_TTL: Duration = Duration::from_secs(60 * 5);
pub const REFRESH_TOKEN_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 14);
pub const AUTH_HEADER_PREFIX: &str = "Bearer ";
