use callix::CallixBuilder;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let callix = CallixBuilder::new()
        .timeout(Duration::from_secs(30))
        .retries(3)
        .retry_delay(Duration::from_secs(2))
        .build()?;

    let response = callix
        .request("custom_api", "create_user")?
        .var("API_KEY", "your-secret-key")
        .var("username", "john_doe")
        .var("email", "john@example.com")
        .var("full_name", "John Doe")
        .var("age", 25)
        .header("X-Request-ID", "req-12345")
        .header("X-Client-Version", "1.0.0")
        .send()
        .await?;

    match response.status() {
        200..=299 => {
            println!("User created successfully!");
            let json_response: serde_json::Value = response.json().await?;
            println!("Response: {:#?}", json_response);
        }
        400 => {
            println!("Bad request - invalid data");
            let error = response.text().await?;
            println!("Error: {}", error);
        }
        401 => {
            println!("Unauthorized - check your API key");
        }
        429 => {
            println!("Rate limited - too many requests");
        }
        _ => {
            println!("Unexpected status: {}", response.status());
        }
    }

    Ok(())
}
