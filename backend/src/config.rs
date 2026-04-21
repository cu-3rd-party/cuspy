#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env, default_value = "0.0.0.0:3000")]
    pub bind_address: std::net::SocketAddr,
}
