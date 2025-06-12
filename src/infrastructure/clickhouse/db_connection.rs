use clickhouse::{Client, error::Error};

use crate::config::Config;

pub async fn db_connect(config: &Config) -> Result<Client, Error> {
    let client = Client::default()
        .with_url(config.clickhouse_url.to_owned())
        .with_user(config.clickhouse_user.to_owned())
        .with_password(config.clickhouse_password.to_owned());
    test_connection(&client).await?;

    Ok(client)
}

pub async fn test_connection(client: &Client) -> Result<(), Error> {
    let result: String = client.query("SELECT version()").fetch_one().await?;

    println!("Connected to ClickHouse version: {}", result);
    Ok(())
}
