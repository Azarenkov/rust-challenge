use anyhow::Result;
use config::Config;
use infrastructure::app_setup::{AppDependencies, server};

pub mod config;
pub mod domain;
pub mod infrastructure;
mod jobs;
pub mod presentation;

pub async fn run(config: &Config) -> Result<()> {
    let deps = AppDependencies::init(config).await?;

    server(deps.app_state, &config.port).await?;

    Ok(())
}
