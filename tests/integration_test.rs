use rust_challenge::{config::Config, run};
use serde_json::Value;
use std::time::Duration;

fn create_test_config() -> Config {
    Config {
        port: "8081".to_string(),
        clickhouse_url: std::env::var("CLICKHOUSE_URL")
            .unwrap_or_else(|_| "http://localhost:8123".to_string()),
        clickhouse_user: "default".to_string(),
        clickhouse_password: "123".to_string(),
        data_generation_count: 30,
    }
}

async fn make_request(
    port: &str,
    method: &str,
    path: &str,
) -> Result<(u16, String), reqwest::Error> {
    let url = format!("http://localhost:{}{}", port, path);
    let client = reqwest::Client::new();

    let response = match method {
        "GET" => client.get(&url).send().await?,
        "POST" => client.post(&url).send().await?,
        "PUT" => client.put(&url).send().await?,
        "DELETE" => client.delete(&url).send().await?,
        _ => panic!("Unsupported method: {}", method),
    };

    let status = response.status().as_u16();
    let body = response.text().await?;
    Ok((status, body))
}

#[actix_web::test]
#[ignore]
async fn test_integration() {
    let config = create_test_config();
    let port = config.port.clone();

    actix_web::rt::spawn(async move {
        if let Err(e) = run(&config).await {
            eprintln!("Server error: {}", e);
        }
    });

    actix_web::rt::time::sleep(Duration::from_secs(4)).await;

    // Get request
    println!("Testing GET request...");
    let (status, body) = make_request(&port, "GET", "/api/v1/stats/get_all")
        .await
        .expect("GET request failed");

    assert_eq!(status, 200, "GET should return 200");

    let json: Value = serde_json::from_str(&body).expect("Invalid JSON");
    assert!(json.is_array(), "Response should be array");

    let stats = json.as_array().unwrap();
    assert!(!stats.is_empty(), "Should have stats");

    println!("GET test passed - {} stats received", stats.len());

    // Post request
    println!("Testing POST request...");
    let (status, _) = make_request(&port, "POST", "/api/v1/stats/get_all")
        .await
        .expect("POST request failed");

    assert_eq!(status, 405, "POST should return 405");
    println!("POST test passed - 405 received");

    // Put request
    println!("Testing PUT request...");
    let (status, _) = make_request(&port, "PUT", "/api/v1/stats/get_all")
        .await
        .expect("PUT request failed");

    assert_eq!(status, 405, "PUT should return 405");
    println!("PUT test passed - 405 received");

    // Delete request
    println!("Testing DELETE request...");
    let (status, _) = make_request(&port, "DELETE", "/api/v1/stats/get_all")
        .await
        .expect("DELETE request failed");

    assert_eq!(status, 405, "DELETE should return 405");
    println!("DELETE test passed - 405 received");

    println!("All tests passed!");
}

#[actix_web::test]
async fn test_config() {
    println!("Testing config creation");

    let config = create_test_config();

    assert_eq!(config.port, "8081");
    assert!(!config.clickhouse_url.is_empty());
    assert!(config.data_generation_count > 0);

    println!("Config test passed");
}
