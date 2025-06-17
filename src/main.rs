use anyhow::Context;
use anyhow::Result;
use rust_challenge::{config::Config, run};

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_env().context("Failed to load config from environment")?;

    run(&config).await?;
    Ok(())
}

//check coommit
