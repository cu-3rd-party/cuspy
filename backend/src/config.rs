use std::path::PathBuf;
use clap::Parser;

#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env, default_value = "0.0.0.0:3000")]
    pub bind_address: std::net::SocketAddr,

    #[clap(long, env)]
    pub admin_secret: String,

    #[clap(long, env)]
    pub jwt_secret: String,

    #[clap(long, env)]
    pub upload_path: PathBuf,

    #[arg(env = "S3_ACCESS_KEY", default_value = "")]
    pub access_key: String,

    #[arg(env = "S3_SECRET_KEY", default_value = "")]
    pub secret_key: String,

    #[arg(env = "S3_ENDPOINT", default_value = "")]
    pub endpoint: String,

    #[arg(env = "S3_REGION", default_value = "us-east-1")]
    pub region: String,

    #[arg(env = "S3_BUCKET_NAME", default_value = "cukiller")]
    pub bucket_name: String,

    #[clap(long, env)]
    #[cfg(feature = "telegram-auth")]
    pub telegram_bot_token: String,

    #[clap(long, env)]
    #[cfg(feature = "telegram-auth")]
    pub public_webapp_url: String,
}
