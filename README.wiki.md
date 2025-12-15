# Callix Wiki - Complete Documentation

> Comprehensive guide to using Callix - A flexible HTTP client library for Rust

## Table of Contents

- [1. Introduction](#1-introduction)
- [2. Architecture](#2-architecture)
- [3. Installation](#3-installation)
- [4. Core Concepts](#4-core-concepts)
- [5. API Reference](#5-api-reference)
- [6. Configuration Guide](#6-configuration-guide)
- [7. Template Engine](#7-template-engine)
- [8. Error Handling](#8-error-handling)
- [9. Advanced Usage](#9-advanced-usage)
- [10. Best Practices](#10-best-practices)
- [11. Security](#11-security)
- [12. Performance](#12-performance)
- [13. Troubleshooting](#13-troubleshooting)
- [14. Examples](#14-examples)

---

## 1. Introduction

### 1.1 What is Callix?

Callix is a configuration-driven HTTP client library for Rust that simplifies integration with REST APIs, particularly AI services. It provides a clean, type-safe interface for making HTTP requests with built-in retry logic, templating, and configuration management.

### 1.2 Key Features

- **Configuration-Driven Design**: Define API endpoints in YAML files
- **Template Engine**: Dynamic variable substitution in URLs, headers, and request bodies
- **Retry Mechanism**: Automatic retry with exponential backoff
- **Type Safety**: Full Rust type safety with compile-time guarantees
- **Multi-Provider Support**: Built-in configurations for popular AI services
- **Flexible**: Easily extendable for custom APIs
- **Async/Await**: Built on top of Tokio and Reqwest

### 1.3 Use Cases

- Integrating with AI APIs (OpenAI, Claude, Gemini, etc.)
- Building API clients for microservices
- Creating webhook handlers
- Implementing rate-limited API calls
- Multi-provider fail-over systems

---

## 2. Architecture

### 2.1 Component Overview

```
┌─────────────────────────────────────────┐
│           CallixBuilder                 │
│  (Configuration & Client Setup)         │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│              Callix                     │
│  (Main Client Instance)                 │
└────────────────┬────────────────────────┘
                 │
      ┌──────────┴──────────┐
      ▼                     ▼
┌──────────┐          ┌──────────┐
│ Provider │          │  Config  │
└──────────┘          └──────────┘
      │                     │
      ▼                     ▼
┌─────────────────────────────────────────┐
│          RequestBuilder                 │
│  (Fluent API for Building Requests)     │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│          TemplateEngine                 │
│  (Variable Substitution)                │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│         HTTP Request (Reqwest)          │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│          CallixResponse                 │
│  (Response Handling)                    │
└─────────────────────────────────────────┘
```

### 2.2 Core Modules

#### client.rs
- `Callix`: Main client struct
- `parse_method()`: HTTP method parser

#### config.rs
- `Config`: Configuration container
- `ProviderConfig`: Provider-specific settings
- `EndpointConfig`: Endpoint definitions

#### request.rs
- `RequestBuilder`: Fluent API for building requests
- Request execution and retry logic

#### response.rs
- `CallixResponse`: Wrapper around reqwest::Response
- Convenience methods for parsing responses

#### template.rs
- `TemplateEngine`: Variable substitution engine
- Template validation

#### error.rs
- `CallixError`: Comprehensive error types
- Error conversion implementations

---

## 3. Installation

### 3.1 Basic Installation

Add to `Cargo.toml`:

```toml
[dependencies]
callix = "0.1.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

### 3.2 Feature Flags

```toml
[dependencies]
callix = { version = "0.1", features = ["rustls-tls", "gzip"] }
```

Available features:

| Feature | Description | Default |
|---------|-------------|---------|
| `native-tls` | Use system's native TLS | Yes |
| `rustls-tls` | Use Rustls (pure Rust) | No |
| `blocking` | Blocking HTTP client | No |
| `cookies` | Cookie store support | No |
| `gzip` | Gzip compression | No |
| `brotli` | Brotli compression | No |
| `stream` | Streaming responses | No |

### 3.3 Minimum Supported Rust Version (MSRV)

Callix requires Rust 1.75 or higher.

---

## 4. Core Concepts

### 4.1 Providers

A **Provider** represents an API service (e.g., OpenAI, Gemini). Each provider has:
- Base URL
- Default headers
- Multiple endpoints
- Timeout settings

```rust
let provider = callix.provider("openai")?;
println!("Base URL: {}", provider.base_url());
```

### 4.2 Endpoints

An **Endpoint** represents a specific API operation. Each endpoint has:
- Path (relative to base URL)
- HTTP method
- Body template
- Query parameters

```rust
let response = callix.request("openai", "chat")?;
```

### 4.3 Variables

**Variables** are placeholders in templates that get replaced with actual values:

```yaml
body_template: |
  {
    "model": "{{model}}",
    "prompt": "{{prompt}}"
  }
```

```rust
.var("model", "gpt-4")
.var("prompt", "Hello!")
```

### 4.4 Request Flow

1. **Build Client**: Create Callix instance with configuration
2. **Select Endpoint**: Choose provider and endpoint
3. **Set Variables**: Provide template variables
4. **Add Headers** (optional): Custom headers
5. **Send Request**: Execute with retry logic
6. **Handle Response**: Parse and process result

---

## 5. API Reference

### 5.1 CallixBuilder

#### `CallixBuilder::new()`
Creates a new builder with default settings.

```rust
let builder = CallixBuilder::new();
```

#### `config(path: impl Into<String>) -> Self`
Specify custom configuration file.

```rust
let builder = CallixBuilder::new()
    .config("my-config.yaml");
```

#### `timeout(duration: Duration) -> Self`
Set request timeout.

```rust
let builder = CallixBuilder::new()
    .timeout(Duration::from_secs(60));
```

#### `retries(count: u32) -> Self`
Set maximum retry attempts.

```rust
let builder = CallixBuilder::new()
    .retries(5);
```

#### `retry_delay(duration: Duration) -> Self`
Set delay between retries.

```rust
let builder = CallixBuilder::new()
    .retry_delay(Duration::from_secs(2));
```

#### `build() -> Result<Callix>`
Build the Callix client.

```rust
let callix = CallixBuilder::new().build()?;
```

### 5.2 Callix

#### `provider(name: &str) -> Result<Provider>`
Get a provider by name.

```rust
let provider = callix.provider("openai")?;
```

#### `request(provider: &str, endpoint: &str) -> Result<RequestBuilder>`
Create a new request builder.

```rust
let builder = callix.request("openai", "chat")?;
```

### 5.3 RequestBuilder

#### `var<T: Serialize>(key: impl Into<String>, value: T) -> Self`
Set a template variable.

```rust
let builder = builder
    .var("model", "gpt-4")
    .var("temperature", 0.7)
    .var("max_tokens", 100);
```

#### `vars(variables: HashMap<String, Value>) -> Self`
Set multiple variables at once.

```rust
use std::collections::HashMap;

let mut vars = HashMap::new();
vars.insert("model".to_string(), json!("gpt-4"));
vars.insert("temperature".to_string(), json!(0.7));

let builder = builder.vars(vars);
```

#### `header(key: impl Into<String>, value: impl Into<String>) -> Self`
Add a custom header.

```rust
let builder = builder
    .header("X-Custom-Header", "value")
    .header("X-Request-ID", "12345");
```

#### `async send(self) -> Result<CallixResponse>`
Send the request.

```rust
let response = builder.send().await?;
```

### 5.4 CallixResponse

#### `status() -> u16`
Get HTTP status code.

```rust
let status = response.status();
println!("Status: {}", status);
```

#### `is_success() -> bool`
Check if status is 2xx.

```rust
if response.is_success() {
    println!("Success!");
}
```

#### `headers() -> &HeaderMap`
Get response headers.

```rust
for (key, value) in response.headers() {
    println!("{}: {:?}", key, value);
}
```

#### `async text(self) -> Result<String>`
Get response body as text.

```rust
let body = response.text().await?;
println!("{}", body);
```

#### `async json<T: DeserializeOwned>(self) -> Result<T>`
Parse response as JSON.

```rust
let json: serde_json::Value = response.json().await?;
println!("{:#?}", json);
```

#### `async bytes(self) -> Result<Vec<u8>>`
Get response as raw bytes.

```rust
let bytes = response.bytes().await?;
```

---

## 6. Configuration Guide

### 6.1 Configuration File Structure

```yaml
providers:
  provider_name:
    base_url: "https://api.example.com"
    headers:
      Header-Name: "{{VARIABLE}}"
    timeout: 60  # seconds, optional
    endpoints:
      endpoint_name:
        path: "/v1/endpoint"
        method: "POST"
        body_template: |
          {
            "key": "{{value}}"
          }
        query_params:
          param: "{{param_value}}"
```

### 6.2 Default Configuration

Callix includes default configurations for:

#### OpenAI
```yaml
openai:
  base_url: "https://api.openai.com"
  endpoints:
    chat: /v1/chat/completions
    embeddings: /v1/embeddings
    completions: /v1/completions
```

#### Google Gemini
```yaml
gemini:
  base_url: "https://generativelanguage.googleapis.com"
  endpoints:
    generate: /v1beta/models/{{model}}:generateContent
```

#### Anthropic Claude
```yaml
anthropic:
  base_url: "https://api.anthropic.com"
  endpoints:
    messages: /v1/messages
```

#### OpenRouter
```yaml
openrouter:
  base_url: "https://openrouter.ai/api"
  endpoints:
    chat: /v1/chat/completions
    models: /v1/models
```

### 6.3 Custom Configuration Example

```yaml
providers:
  my_service:
    base_url: "https://api.myservice.com"
    headers:
      Authorization: "Bearer {{API_KEY}}"
      Content-Type: "application/json"
      X-API-Version: "2.0"
    timeout: 30
    endpoints:
      # Simple GET endpoint
      get_user:
        path: "/users/{{user_id}}"
        method: "GET"

      # POST with body template
      create_user:
        path: "/users"
        method: "POST"
        body_template: |
          {
            "name": "{{name}}",
            "email": "{{email}}",
            "age": {{age}}
          }

      # With query parameters
      search:
        path: "/search"
        method: "GET"
        query_params:
          q: "{{query}}"
          limit: "{{limit}}"
          offset: "{{offset}}"

      # Complex nested JSON
      analyze:
        path: "/analyze"
        method: "POST"
        body_template: |
          {
            "data": {{data}},
            "options": {
              "model": "{{model}}",
              "threshold": {{threshold}}
            }
          }
```

### 6.4 Environment Variables

Load API keys from environment variables:

```rust
use std::env;

let api_key = env::var("OPENAI_API_KEY")?;

let response = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key)
    .send()
    .await?;
```

With `.env` file:

```bash
# .env
OPENAI_API_KEY=sk-...
GEMINI_API_KEY=AIza...
ANTHROPIC_API_KEY=sk-ant-...
```

```rust
use dotenv::dotenv;

dotenv().ok();
let api_key = env::var("OPENAI_API_KEY")?;
```

---

## 7. Template Engine

### 7.1 Variable Substitution

Templates use `{{variable_name}}` syntax:

```rust
// In config:
// path: "/users/{{user_id}}/posts/{{post_id}}"

let response = callix
    .request("api", "get_post")?
    .var("user_id", 123)
    .var("post_id", 456)
    .send()
    .await?;

// Resulting URL: /users/123/posts/456
```

### 7.2 Supported Data Types

```rust
// String
.var("name", "Alice")

// Number
.var("age", 30)
.var("score", 95.5)

// Boolean
.var("active", true)

// Null
.var("optional", serde_json::Value::Null)

// JSON Object
.var("user", json!({
    "id": 1,
    "name": "Alice"
}))

// JSON Array
.var("tags", json!(["rust", "programming"]))
```

### 7.3 Complex Templates

```yaml
body_template: |
  {
    "model": "{{model}}",
    "messages": {{messages}},
    "settings": {
      "temperature": {{temperature}},
      "max_tokens": {{max_tokens}},
      "stream": {{stream}}
    },
    "metadata": {{metadata}}
  }
```

```rust
let response = callix
    .request("api", "chat")?
    .var("model", "gpt-4")
    .var("messages", json!([
        {"role": "user", "content": "Hello"}
    ]))
    .var("temperature", 0.7)
    .var("max_tokens", 100)
    .var("stream", false)
    .var("metadata", json!({
        "user_id": "123",
        "session_id": "abc"
    }))
    .send()
    .await?;
```

### 7.4 Template Validation

Callix validates templates at runtime:

```rust
// This will fail if {{API_KEY}} is not provided
let result = callix
    .request("openai", "chat")?
    // .var("API_KEY", "sk-...") // Missing!
    .var("model", "gpt-4")
    .send()
    .await;

assert!(matches!(result, Err(CallixError::TemplateError(_))));
```

---

## 8. Error Handling

### 8.1 Error Types

```rust
pub enum CallixError {
    ConfigNotFound(String),
    InvalidConfig(String),
    ProviderNotFound(String),
    EndpointNotFound(String),
    HttpError(reqwest::Error),
    TemplateError(String),
    SerializationError(String),
    TimeoutError,
    MaxRetriesExceeded,
    InvalidMethod(String),
}
```

### 8.2 Error Handling Patterns

#### Basic Error Handling
```rust
match callix.request("openai", "chat") {
    Ok(builder) => {
        // Use builder
    }
    Err(CallixError::ProviderNotFound(name)) => {
        eprintln!("Provider not found: {}", name);
    }
    Err(CallixError::EndpointNotFound(name)) => {
        eprintln!("Endpoint not found: {}", name);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

#### Comprehensive Error Handling
```rust
let result = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key)
    .var("model", "gpt-4")
    .var("messages", messages)
    .send()
    .await;

match result {
    Ok(response) => {
        match response.status() {
            200..=299 => {
                let json = response.json().await?;
                // Process success
            }
            400 => {
                eprintln!("Bad request");
            }
            401 => {
                eprintln!("Unauthorized - check API key");
            }
            429 => {
                eprintln!("Rate limited - retry later");
            }
            500..=599 => {
                eprintln!("Server error");
            }
            _ => {
                eprintln!("Unexpected status: {}", response.status());
            }
        }
    }
    Err(CallixError::TimeoutError) => {
        eprintln!("Request timed out");
    }
    Err(CallixError::MaxRetriesExceeded) => {
        eprintln!("Max retries exceeded");
    }
    Err(CallixError::HttpError(e)) => {
        eprintln!("HTTP error: {}", e);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

#### Using Result Combinators
```rust
let content = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key)
    .var("model", "gpt-4")
    .var("messages", messages)
    .send()
    .await?
    .json::<serde_json::Value>()
    .await?
    ["choices"][0]["message"]["content"]
    .as_str()
    .ok_or_else(|| CallixError::SerializationError("No content".into()))?;
```

---

## 9. Advanced Usage

### 9.1 Concurrent Requests

```rust
use tokio::task;

let callix = CallixBuilder::new().build()?;

let handle1 = task::spawn({
    let callix = callix.clone();
    async move {
        callix
            .request("openai", "chat")?
            .var("API_KEY", "sk-...")
            .var("model", "gpt-4")
            .var("messages", json!([{"role": "user", "content": "Hello"}]))
            .send()
            .await
    }
});

let handle2 = task::spawn({
    let callix = callix.clone();
    async move {
        callix
            .request("gemini", "generate")?
            .var("API_KEY", "AIza...")
            .var("model", "gemini-pro")
            .var("prompt", "Hello")
            .send()
            .await
    }
});

let (result1, result2) = tokio::join!(handle1, handle2);
```

### 9.2 Batch Processing

```rust
let prompts = vec!["Prompt 1", "Prompt 2", "Prompt 3"];

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
    tokio::time::sleep(Duration::from_millis(500)).await;
}
```

### 9.3 Custom Retry Logic

```rust
use tokio::time::sleep;

let mut attempts = 0;
let max_attempts = 5;

loop {
    attempts += 1;

    match callix.request("openai", "chat")?.send().await {
        Ok(response) if response.is_success() => {
            return Ok(response);
        }
        Ok(response) if response.status() == 429 => {
            if attempts >= max_attempts {
                return Err("Max retries exceeded".into());
            }
            let delay = 2_u64.pow(attempts); // Exponential backoff
            sleep(Duration::from_secs(delay)).await;
        }
        Err(e) => return Err(e),
        _ => return Err("Unexpected error".into()),
    }
}
```

### 9.4 Response Streaming

```rust
// Enable stream feature in Cargo.toml
let response = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key)
    .var("stream", true)
    .send()
    .await?;

// Process stream (requires stream feature)
```

### 9.5 Request Middleware

```rust
struct LoggingMiddleware;

impl LoggingMiddleware {
    async fn log_request<F, Fut>(
        &self,
        f: F,
    ) -> Result<CallixResponse>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<CallixResponse>>,
    {
        let start = Instant::now();
        println!("Request started");

        let result = f().await;

        let elapsed = start.elapsed();
        println!("Request completed in {:?}", elapsed);

        result
    }
}
```

---

## 10. Best Practices

### 10.1 Configuration Management

**DO:**
- Store API keys in environment variables
- Use separate configs for dev/prod
- Version control your config files (without secrets)

**DON'T:**
- Hardcode API keys in code
- Commit API keys to git
- Use production keys in development

### 10.2 Error Handling

**DO:**
- Always handle errors explicitly
- Log errors with context
- Implement exponential backoff for retries
- Check response status codes

**DON'T:**
- Use unwrap() in production
- Ignore error types
- Retry indefinitely
- Assume all 2xx responses are valid

### 10.3 Performance

**DO:**
- Reuse Callix instances
- Use concurrent requests when possible
- Implement connection pooling
- Set appropriate timeouts

**DON'T:**
- Create new clients for each request
- Block the async runtime
- Use excessive retry attempts
- Set infinite timeouts

### 10.4 Security

**DO:**
- Validate all input data
- Use HTTPS everywhere
- Rotate API keys regularly
- Implement rate limiting

**DON'T:**
- Trust user input blindly
- Log sensitive data
- Use deprecated TLS versions
- Share API keys across environments

---

## 11. Security

### 11.1 API Key Management

```rust
// Good: Load from environment
let api_key = env::var("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY must be set");

// Bad: Hardcoded
let api_key = "sk-hardcoded-key"; // Never do this!
```

### 11.2 TLS Configuration

```rust
// Use Rustls for pure Rust TLS
let callix = CallixBuilder::new()
    .build()?;

// Or explicitly use native TLS
```

```toml
# Cargo.toml
callix = { version = "0.1", features = ["rustls-tls"] }
```

### 11.3 Request Validation

```rust
// Validate inputs before making requests
fn validate_model(model: &str) -> Result<()> {
    let valid_models = ["gpt-4", "gpt-3.5-turbo"];
    if !valid_models.contains(&model) {
        return Err("Invalid model".into());
    }
    Ok(())
}

validate_model(&user_input)?;

let response = callix
    .request("openai", "chat")?
    .var("model", user_input)
    .send()
    .await?;
```

### 11.4 Sensitive Data

```rust
// Don't log sensitive information
let response = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key) // Don't log this!
    .send()
    .await?;

// Good logging
println!("Request completed with status: {}", response.status());

// Bad logging
println!("Full response: {:?}", response); // Might contain sensitive data
```

---

## 12. Performance

### 12.1 Client Reuse

```rust
// Good: Single client instance
let callix = CallixBuilder::new().build()?;

for _ in 0..100 {
    let response = callix.request("openai", "chat")?.send().await?;
}

// Bad: Creating client each time
for _ in 0..100 {
    let callix = CallixBuilder::new().build()?; // Expensive!
    let response = callix.request("openai", "chat")?.send().await?;
}
```

### 12.2 Concurrent Requests

```rust
use futures::future::join_all;

let futures: Vec<_> = (0..10)
    .map(|i| {
        let callix = callix.clone();
        async move {
            callix
                .request("openai", "chat")?
                .var("API_KEY", api_key)
                .var("messages", json!([
                    {"role": "user", "content": format!("Request {}", i)}
                ]))
                .send()
                .await
        }
    })
    .collect();

let results = join_all(futures).await;
```

### 12.3 Timeout Configuration

```rust
// Balance between reliability and performance
let callix = CallixBuilder::new()
    .timeout(Duration::from_secs(30))  // Reasonable default
    .retries(3)                        // Not too many
    .retry_delay(Duration::from_secs(1)) // Quick retry
    .build()?;
```

### 12.4 Connection Pooling

Reqwest (underlying HTTP client) handles connection pooling automatically. Reusing the Callix instance ensures connection reuse.

---

## 13. Troubleshooting

### 13.1 Common Issues

#### Config Not Found
```
Error: Config file not found: config.yaml
```

**Solution:**
- Check file path is correct
- Use absolute path if needed
- Verify file permissions

```rust
let callix = CallixBuilder::new()
    .config("/absolute/path/to/config.yaml")
    .build()?;
```

#### Provider Not Found
```
Error: Provider not found: my_provider
```

**Solution:**
- Check provider name spelling
- Verify provider exists in config
- Use `default_config()` to see available providers

#### Template Error
```
Error: Template error: Missing variable: API_KEY
```

**Solution:**
- Ensure all required variables are set
- Check variable names match template

```rust
let response = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key)  // Must provide all required vars
    .var("model", "gpt-4")
    .send()
    .await?;
```

#### Timeout Error
```
Error: Request timeout
```

**Solution:**
- Increase timeout duration
- Check network connectivity
- Verify API endpoint is responding

```rust
let callix = CallixBuilder::new()
    .timeout(Duration::from_secs(120))
    .build()?;
```

#### Max Retries Exceeded
```
Error: Max retries exceeded
```

**Solution:**
- Check API status
- Verify API key is valid
- Reduce retry count or increase delay

### 13.2 Debugging Tips

#### Enable Logging
```rust
env_logger::init();

let response = callix
    .request("openai", "chat")?
    .send()
    .await?;
```

#### Inspect Requests
```rust
let response = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key)
    .send()
    .await?;

println!("Status: {}", response.status());
println!("Headers: {:?}", response.headers());
```

#### Test Configuration
```rust
let config = Config::from_file("config.yaml")?;
println!("Loaded providers: {:?}", config.providers.keys());

let provider = config.get_provider("openai")?;
println!("Base URL: {}", provider.base_url);
```

---

## 14. Examples

### 14.1 OpenAI ChatGPT

```rust
use callix::CallixBuilder;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let callix = CallixBuilder::new().build()?;

    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", "sk-...")
        .var("model", "gpt-4")
        .var("messages", json!([
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Explain Rust ownership"}
        ]))
        .var("temperature", 0.7)
        .var("max_tokens", 500)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap();

    println!("{}", content);
    Ok(())
}
```

### 14.2 Google Gemini

```rust
let response = callix
    .request("gemini", "generate")?
    .var("API_KEY", "AIza...")
    .var("model", "gemini-2.0-flash-exp")
    .var("prompt", "Write a poem about programming")
    .send()
    .await?;

let json: serde_json::Value = response.json().await?;
let text = json["candidates"][0]["content"]["parts"][0]["text"]
    .as_str()
    .unwrap();

println!("{}", text);
```

### 14.3 Anthropic Claude

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

let json: serde_json::Value = response.json().await?;
let content = json["content"][0]["text"].as_str().unwrap();

println!("{}", content);
```

### 14.4 OpenRouter

```rust
// Access 100+ models with one API key
let response = callix
    .request("openrouter", "chat")?
    .var("API_KEY", "your-openrouter-key")
    .var("model", "meta-llama/llama-3.1-8b-instruct:free")
    .var("messages", json!([
        {"role": "user", "content": "Hello!"}
    ]))
    .header("HTTP-Referer", "https://your-site.com")
    .send()
    .await?;
```

### 14.5 Custom API

```yaml
# custom-api.yaml
providers:
  weather:
    base_url: "https://api.weather.com"
    headers:
      X-API-Key: "{{API_KEY}}"
    endpoints:
      current:
        path: "/v1/current"
        method: "GET"
        query_params:
          city: "{{city}}"
```

```rust
let callix = CallixBuilder::new()
    .config("custom-api.yaml")
    .build()?;

let response = callix
    .request("weather", "current")?
    .var("API_KEY", "your-key")
    .var("city", "Tehran")
    .send()
    .await?;
```

---

## Appendix A: Default Configuration

See the built-in `default-config.yaml` for all default providers and endpoints.

## Appendix B: Error Reference

Complete list of error types and their meanings.

## Appendix C: Migration Guide

Guide for migrating from other HTTP clients to Callix.

## Appendix D: Contributing

Guidelines for contributing to the Callix project.
