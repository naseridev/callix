use callix::CallixBuilder;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let callix = CallixBuilder::new()
        .timeout(Duration::from_secs(90))
        .retries(2)
        .build()?;

    let response = callix
        .request("anthropic", "messages")?
        .var("API_KEY", "sk-ant-your-api-key-here")
        .var("model", "claude-3-5-sonnet-20241022")
        .var("max_tokens", 1024)
        .var(
            "messages",
            json!([
                {
                    "role": "user",
                    "content": "Write a short poem about programming"
                }
            ]),
        )
        .send()
        .await?;

    if response.is_success() {
        let json_response: serde_json::Value = response.json().await?;
        println!("Full Response: {:#?}", json_response);

        if let Some(content) = json_response["content"][0]["text"].as_str() {
            println!("\nClaude says:\n{}", content);
        }
    } else {
        println!("Request failed with status: {}", response.status());
        let error_body = response.text().await?;
        println!("Error details: {}", error_body);
    }

    Ok(())
}
