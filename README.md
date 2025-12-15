# Callix

A flexible, configuration-driven HTTP client library for Rust, designed for easy integration with AI APIs and RESTful services.

## Features

- **Configuration-Driven** - Define providers and endpoints in YAML
- **Auto Retry** - Built-in retry mechanism with configurable delays
- **Template Engine** - Dynamic variable substitution in URLs and bodies
- **Multi-Provider** - Support for OpenAI, Gemini, Claude, OpenRouter, and more
- **Type-Safe** - Full Rust type safety with serde integration
- **Zero Config** - Works out of the box with default configurations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
callix = "0.1.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```
### Step-by-Step Setup

1. **Create a new Rust project**
```bash
   cargo new my_callix_project
   cd my_callix_project
```

2. **Add dependencies**

   Using `cargo add` (Rust 1.62+):
```bash
   cargo add callix
   cargo add tokio --features full
   cargo add serde_json
```

   Or manually in `Cargo.toml`:
```toml
   [dependencies]
   callix = "0.1.0"
   tokio = { version = "1", features = ["full"] }
   serde_json = "1.0"
```

3. **Write your first code in src/main.rs**
```rust
   use callix::CallixBuilder;
   use std::time::Duration;

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       let callix = CallixBuilder::new()
           .timeout(Duration::from_secs(60))
           .build()?;

       println!("Callix is ready!");
       Ok(())
   }
```

4. **Run your project**
```bash
   cargo run
```

### Optional: Environment Variables Setup

For API keys, create a `.env` file:
```bash
# .env
OPENAI_API_KEY=sk-your-key-here
GEMINI_API_KEY=your-key-here
```

Add `dotenv`:
```bash
cargo add dotenv
```

Load in your code:
```rust
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;
    // Use api_key...
    Ok(())
}
```

## Quick Start

### Basic Usage

```rust
use callix::CallixBuilder;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let callix = CallixBuilder::new()
        .timeout(Duration::from_secs(60))
        .retries(3)
        .build()?;

    // Make a request to Gemini
    let response = callix
        .request("gemini", "generate")?
        .var("API_KEY", "your-api-key")
        .var("model", "gemini-2.0-flash-exp")
        .var("prompt", "Hello, world!")
        .send()
        .await?;

    // Handle the response
    if response.is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    }

    Ok(())
}
```

### OpenAI Example

```rust
use serde_json::json;

let response = callix
    .request("openai", "chat")?
    .var("API_KEY", "sk-...")
    .var("model", "gpt-4")
    .var("messages", json!([
        {"role": "user", "content": "Explain Rust ownership"}
    ]))
    .send()
    .await?;

let json: serde_json::Value = response.json().await?;
println!("{}", json["choices"][0]["message"]["content"]);
```

### OpenRouter Example

```rust
// Access 100+ AI models with one API key
let response = callix
    .request("openrouter", "chat")?
    .var("API_KEY", "your-openrouter-key")
    .var("model", "anthropic/claude-3.5-sonnet")
    .var("messages", json!([
        {"role": "user", "content": "Hello!"}
    ]))
    .send()
    .await?;
```

## Configuration

### Default Configuration

Callix comes with built-in configurations for popular AI providers:

- **OpenAI** (GPT-4, GPT-3.5)
- **Google Gemini** (Gemini Pro, Flash)
- **Anthropic Claude** (Claude 3.5 Sonnet, Opus, Haiku)

### Custom Configuration

Create a `config.yaml`:

```yaml
providers:
  my_api:
    base_url: "https://api.example.com"
    headers:
      Authorization: "Bearer {{API_KEY}}"
      Content-Type: "application/json"
    endpoints:
      predict:
        path: "/v1/predict"
        method: "POST"
        body_template: |
          {
            "input": "{{text}}",
            "model": "{{model}}"
          }
```

Then use it:

```rust
let callix = CallixBuilder::new()
    .config("config.yaml")
    .build()?;

let response = callix
    .request("my_api", "predict")?
    .var("API_KEY", "secret")
    .var("text", "Hello")
    .var("model", "v2")
    .send()
    .await?;
```

## Advanced Features

### Custom Headers

```rust
let response = callix
    .request("openai", "chat")?
    .var("API_KEY", "sk-...")
    .header("X-Custom-Header", "value")
    .header("X-Request-ID", "12345")
    .send()
    .await?;
```

### Retry Configuration

```rust
let callix = CallixBuilder::new()
    .retries(5)
    .retry_delay(Duration::from_secs(2))
    .timeout(Duration::from_secs(120))
    .build()?;
```

### Error Handling

```rust
match response.status() {
    200..=299 => println!("Success!"),
    400 => println!("Bad Request"),
    401 => println!("Unauthorized"),
    429 => println!("Rate Limited"),
    500..=599 => println!("Server Error"),
    _ => println!("Unexpected Error"),
}
```

## Examples

Check out the [examples](examples/) directory for more:

- [`openai.rs`](examples/openai.rs) - OpenAI ChatGPT integration
- [`gemini.rs`](examples/gemini.rs) - Google Gemini API
- [`anthropic.rs`](examples/anthropic.rs) - Anthropic Claude API

Run an example:

```bash
cargo run --example openai
```

## Features Flags

```toml
[dependencies]
callix = { version = "0.1", features = ["rustls-tls"] }
```

Available features:
- `native-tls` (default) - Use native TLS
- `rustls-tls` - Use Rustls (pure Rust)
- `blocking` - Blocking HTTP client
- `cookies` - Cookie store support
- `gzip`, `brotli` - Compression support
- `stream` - Streaming response support

## Documentation

- [Full Documentation](https://docs.rs/callix)
- [Wiki & Advanced Guide](README.wiki.md)
- [API Reference](https://docs.rs/callix)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

Built with:
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime

## Contact

- GitHub Issues: [Report a bug](https://github.com/naseridev/callix/issues)
- Discussions: [Ask a question](https://github.com/naseridev/callix/discussions)
