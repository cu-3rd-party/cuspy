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
    #[cfg(feature = "telegram-auth")]
    pub telegram_bot_token: String,

    #[clap(long, env)]
    #[cfg(feature = "telegram-auth")]
    pub public_webapp_url: String,
}
