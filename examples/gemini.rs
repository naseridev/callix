use callix::CallixBuilder;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let callix = CallixBuilder::new()
        .timeout(Duration::from_secs(60))
        .retries(3)
        .build()?;

    let response = callix
        .request("gemini", "generate")?
        .var("API_KEY", "sk-your-api-key-here")
        .var("model", "gemini-2.5-flash")
        .var("prompt", "say my name")
        .send()
        .await?;

    if response.is_success() {
        let body = response.text().await?;
        println!("Success: {}", body);
    } else {
        println!("Failed with status: {}", response.status());
    }

    Ok(())
}
