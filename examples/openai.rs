use callix::CallixBuilder;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let callix = CallixBuilder::new()
        .timeout(Duration::from_secs(60))
        .retries(3)
        .build()?;

    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", "sk-your-api-key-here")
        .var("model", "gpt-4")
        .var(
            "messages",
            json!([
                {
                    "role": "system",
                    "content": "You are a helpful assistant."
                },
                {
                    "role": "user",
                    "content": "Hello! How are you?"
                }
            ]),
        )
        .var("temperature", 0.7)
        .var("max_tokens", 150)
        .send()
        .await?;

    if response.is_success() {
        let json_response: serde_json::Value = response.json().await?;
        println!("Response: {:#?}", json_response);

        if let Some(content) = json_response["choices"][0]["message"]["content"].as_str() {
            println!("\nAssistant: {}", content);
        }
    } else {
        println!("Failed with status: {}", response.status());
    }

    Ok(())
}
