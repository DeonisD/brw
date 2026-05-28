use std::net::IpAddr;

pub struct Config {
    pub database_url: String,
    pub server_host: IpAddr,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/rwetd".into()),
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".into())
                .parse()
                .expect("Invalid SERVER_HOST"),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("Invalid SERVER_PORT"),
        }
    }
}
