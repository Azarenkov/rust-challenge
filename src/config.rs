use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: String,
    pub clickhouse_url: String,
    pub clickhouse_user: String,
    pub clickhouse_password: String,
    pub data_generation_count: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok();
        envy::from_env::<Config>()
    }
}
