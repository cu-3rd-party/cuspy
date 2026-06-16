#[derive(clap::Parser, Clone, Debug)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env, default_value = "0.0.0.0:3000")]
    pub bind_address: std::net::SocketAddr,

    #[clap(long, env)]
    pub admin_secret: String,

    #[clap(long, env)]
    pub jwt_secret: String,

    #[clap(long, env, default_value = "http://localhost:5173")]
    pub cors_origin: String,

    #[clap(env = "S3_ACCESS_KEY", default_value = "")]
    pub s3_access_key: String,

    #[clap(env = "S3_SECRET_KEY", default_value = "")]
    pub s3_secret_key: String,

    #[clap(env = "S3_ENDPOINT", default_value = "")]
    pub s3_endpoint: String,

    #[clap(env = "S3_REGION", default_value = "us-east-1")]
    pub s3_region: String,

    #[clap(env = "S3_BUCKET_NAME", default_value = "cukiller")]
    pub s3_bucket_name: String,

    #[clap(long, env)]
    #[cfg(feature = "telegram")]
    pub telegram_bot_token: String,

    #[clap(long, env)]
    #[cfg(feature = "telegram")]
    pub public_webapp_url: String,
}
