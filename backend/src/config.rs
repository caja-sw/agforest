use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,

    pub worker_threads: usize,
    pub max_blocking_threads: usize,

    pub database_url: String,
    pub database_connection_limit: u32,
}
