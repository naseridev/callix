# Callix

A flexible, configuration-driven HTTP client library for Rust, designed for seamless integration with AI APIs and RESTful services.

## Features

- **Configuration-Driven** - Define providers and endpoints in YAML for easy management
- **Auto Retry** - Built-in retry mechanism with configurable delays and exponential backoff
- **Template Engine** - Dynamic variable substitution in URLs, headers, and request bodies
- **Multi-Provider Support** - Pre-configured for OpenAI, Gemini, Claude, and more
- **Type-Safe** - Full Rust type safety with serde integration
- **Zero Config** - Works out of the box with default configurations for popular AI services
- **Async/Await** - Built on Tokio and Reqwest for high-performance async operations

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Examples](#examples)
- [Documentation](#documentation)
- [Contributing](#contributing)

## Installation

Add Callix to your `Cargo.toml`:

```toml
[dependencies]
callix = "0.1.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

### Step-by-Step Setup

1. **Create a new Rust project**:
   ```bash
   cargo new my_callix_project
   cd my_callix_project
   ```

2. **Add dependencies** using `cargo add` (Rust 1.62+):
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

3. **Write your first code** in `src/main.rs`:
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

4. **Run your project**:
   ```bash
   cargo run
   ```

### Optional: Environment Variables Setup

For secure API key management, create a `.env` file:

```bash
# .env
OPENAI_API_KEY=sk-your-key-here
GEMINI_API_KEY=your-key-here
ANTHROPIC_API_KEY=sk-ant-your-key-here
```

Add the `dotenv` crate:
```bash
cargo add dotenv
```

Load environment variables in your code:
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
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let callix = CallixBuilder::new()
        .timeout(Duration::from_secs(60))
        .retries(3)
        .retry_delay(Duration::from_secs(1))
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
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "Explain Rust ownership in simple terms"}
    ]))
    .send()
    .await?;

let json: serde_json::Value = response.json().await?;
println!("{}", json["choices"][0]["message"]["content"]);
```

### Anthropic Claude Example

```rust
let response = callix
    .request("anthropic", "messages")?
    .var("API_KEY", "sk-ant-...")
    .var("model", "claude-3-5-sonnet-20241022")
    .var("max_tokens", 1024)
    .var("messages", json!([
        {"role": "user", "content": "Explain quantum computing"}
    ]))
    .send()
    .await?;
```

## Configuration

### Default Configuration

Callix includes pre-configured settings for popular AI providers:

- **OpenAI** - GPT-4, GPT-3.5 Turbo
- **Google Gemini** - Gemini Pro, Gemini Flash
- **Anthropic Claude** - Claude 3.5 Sonnet, Opus, Haiku

### Custom Configuration

Create a `config.yaml` file to define your own API endpoints:

```yaml
providers:
  my_api:
    base_url: "https://api.example.com"
    headers:
      Authorization: "Bearer {{API_KEY}}"
      Content-Type: "application/json"
    timeout: 30  # seconds (optional)
    endpoints:
      predict:
        path: "/v1/predict"
        method: "POST"
        body_template: |
          {
            "input": "{{text}}",
            "model": "{{model}}",
            "temperature": {{temperature}}
          }
        query_params:
          version: "{{api_version}}"
```

Use your custom configuration:

```rust
let callix = CallixBuilder::new()
    .config("config.yaml")
    .build()?;

let response = callix
    .request("my_api", "predict")?
    .var("API_KEY", "secret")
    .var("text", "Hello")
    .var("model", "v2")
    .var("temperature", 0.7)
    .var("api_version", "latest")
    .send()
    .await?;
```

## Examples

### Custom Headers

```rust
let response = callix
    .request("openai", "chat")?
    .var("API_KEY", "sk-...")
    .var("model", "gpt-4")
    .var("messages", json!([
        {"role": "user", "content": "Hello"}
    ]))
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
    200..=299 => {
        let json: serde_json::Value = response.json().await?;
        println!("Success: {:#?}", json);
    }
    400 => println!("Bad Request - check your input"),
    401 => println!("Unauthorized - verify your API key"),
    429 => println!("Rate Limited - please retry later"),
    500..=599 => println!("Server Error - try again"),
    _ => println!("Unexpected status: {}", response.status()),
}
```

### Batch Processing

```rust
let prompts = vec![
    "Explain machine learning",
    "What is quantum computing?",
    "Describe neural networks"
];

for prompt in prompts {
    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", api_key)
        .var("model", "gpt-3.5-turbo")
        .var("messages", json!([
            {"role": "user", "content": prompt}
        ]))
        .send()
        .await?;

    // Process response
    let json: serde_json::Value = response.json().await?;
    println!("{}: {}", prompt, json["choices"][0]["message"]["content"]);

    // Rate limiting
    tokio::time::sleep(Duration::from_millis(500)).await;
}
```

### Concurrent Requests

```rust
use futures::future::join_all;

let prompts = vec!["Prompt 1", "Prompt 2", "Prompt 3"];

let futures: Vec<_> = prompts.iter().map(|&prompt| {
    let callix = callix.clone();
    let api_key = api_key.clone();
    async move {
        callix
            .request("openai", "chat")?
            .var("API_KEY", api_key)
            .var("model", "gpt-3.5-turbo")
            .var("messages", json!([
                {"role": "user", "content": prompt}
            ]))
            .send()
            .await
    }
}).collect();

let results = join_all(futures).await;
```

## Documentation

- **[Full API Documentation](https://docs.rs/callix)** - Complete API reference
- **[Wiki & Advanced Guide](README.wiki.md)** - Comprehensive guide with advanced usage
- **[Examples Directory](examples/)** - Working code examples
  - [`openai.rs`](examples/openai.rs) - OpenAI ChatGPT integration
  - [`gemini.rs`](examples/gemini.rs) - Google Gemini API
  - [`anthropic.rs`](examples/anthropic.rs) - Anthropic Claude API

### Running Examples

```bash
cargo run --example openai
cargo run --example gemini
cargo run --example anthropic
```

## Feature Flags

Customize Callix with feature flags:

```toml
[dependencies]
callix = { version = "0.1", features = ["rustls-tls", "gzip"] }
```

### Available Features

| Feature | Description | Default |
|---------|-------------|---------|
| `native-tls` | Use system's native TLS | ✓ |
| `rustls-tls` | Use Rustls (pure Rust TLS) | ✗ |
| `blocking` | Blocking HTTP client support | ✗ |
| `cookies` | Cookie store support | ✗ |
| `gzip` | Gzip compression | ✗ |
| `brotli` | Brotli compression | ✗ |
| `stream` | Streaming response support | ✗ |

## Architecture

### Component Overview

```
CallixBuilder → Callix → RequestBuilder → HTTP Request → CallixResponse
                  ↓           ↓
               Config    TemplateEngine
```

### Core Modules

- **`client`** - Main client implementation and HTTP method parsing
- **`config`** - Configuration management and provider definitions
- **`request`** - Request building and execution with retry logic
- **`response`** - Response handling and parsing utilities
- **`template`** - Variable substitution and template rendering
- **`error`** - Comprehensive error types and conversions

## Minimum Supported Rust Version (MSRV)

Callix requires **Rust 1.75** or higher.

## Contributing

Contributions are welcome! Here's how to get started:

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Development Setup

```bash
git clone https://github.com/naseridev/callix.git
cd callix
cargo build
cargo test
```

### Guidelines

- Follow Rust naming conventions and idioms
- Add tests for new features
- Update documentation
- Run `cargo fmt` and `cargo clippy` before committing

## Security

- **Never hardcode API keys** - Use environment variables
- **Validate all user input** - Sanitize before sending requests
- **Use HTTPS** - All default configurations use secure connections
- **Rotate keys regularly** - Follow security best practices

## Acknowledgments

Built with these amazing Rust crates:

- [**reqwest**](https://github.com/seanmonstar/reqwest) - High-performance HTTP client
- [**serde**](https://github.com/serde-rs/serde) - Serialization framework
- [**tokio**](https://github.com/tokio-rs/tokio) - Async runtime
- [**serde_yaml**](https://github.com/dtolnay/serde-yaml) - YAML configuration parsing

## Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/naseridev/callix/issues)
- **Discussions**: [Ask questions or share ideas](https://github.com/naseridev/callix/discussions)
- **Documentation**: [Read the docs](https://docs.rs/callix)
