# Callix Wiki - Complete Documentation

> Comprehensive technical guide to Callix - A flexible, configuration-driven HTTP client library for Rust

## Table of Contents

1. [Introduction](#1-introduction)
2. [Architecture](#2-architecture)
3. [Getting Started](#3-getting-started)
4. [Core Concepts](#4-core-concepts)
5. [API Reference](#5-api-reference)
6. [Configuration Guide](#6-configuration-guide)
7. [Template Engine](#7-template-engine)
8. [Error Handling](#8-error-handling)
9. [Advanced Usage](#9-advanced-usage)
10. [Best Practices](#10-best-practices)
11. [Security](#11-security)
12. [Performance](#12-performance)
13. [Troubleshooting](#13-troubleshooting)
14. [Cookbook](#14-cookbook)

---

## 1. Introduction

### 1.1 What is Callix?

Callix is a modern, configuration-driven HTTP client library for Rust that simplifies integration with REST APIs, particularly AI services. It provides a clean, type-safe interface for making HTTP requests with built-in retry logic, templating, and configuration management.

### 1.2 Key Features

- **Configuration-Driven Design**: Define API endpoints once in YAML files
- **Template Engine**: Dynamic variable substitution in URLs, headers, and request bodies
- **Automatic Retry**: Configurable retry mechanism with customizable delays
- **Type Safety**: Full Rust type safety with compile-time guarantees
- **Multi-Provider Support**: Pre-configured for OpenAI, Claude, Gemini
- **Flexible**: Easily extendable for custom APIs
- **Async/Await**: Built on Tokio and Reqwest for high performance
- **Zero Config**: Works immediately with default configurations

### 1.3 Use Cases

- **AI API Integration** - Connect to OpenAI, Claude, Gemini, and other LLM providers
- **Microservice Communication** - Build HTTP clients for internal services
- **Webhook Handlers** - Process and respond to webhook events
- **Rate-Limited APIs** - Handle API rate limits with automatic retry
- **Multi-Provider Systems** - Implement failover across multiple providers

### 1.4 Design Philosophy

Callix follows these principles:

- **Configuration over Code**: Define endpoints in YAML, not hardcoded strings
- **Type Safety First**: Leverage Rust's type system for correctness
- **Ergonomic API**: Fluent builder pattern for intuitive usage
- **Sensible Defaults**: Work out of the box for common scenarios
- **Extensibility**: Easy to customize for specific needs

---

## 2. Architecture

### 2.1 Component Overview

```
┌─────────────────────────────────────────┐
│           CallixBuilder                 │
│   Configuration & Client Setup          │
│   - timeout, retries, config path       │
└────────────────┬────────────────────────┘
                 │ build()
                 ▼
┌─────────────────────────────────────────┐
│              Callix                     │
│         Main Client Instance            │
│   - config: Config                      │
│   - client: reqwest::Client             │
│   - max_retries: u32                    │
│   - retry_delay: Duration               │
└────────────────┬────────────────────────┘
                 │ request(provider, endpoint)
      ┌──────────┴──────────┐
      ▼                     ▼
┌────────────┐        ┌────────────┐
│ Provider   │        │   Config   │
│ Config     │◄───────┤   Manager  │
└────────────┘        └────────────┘
      │                     │
      ▼                     ▼
┌─────────────────────────────────────────┐
│          RequestBuilder                 │
│   Fluent API for Building Requests      │
│   - var(), vars(), header()             │
└────────────────┬────────────────────────┘
                 │ send()
                 ▼
┌─────────────────────────────────────────┐
│        TemplateEngine                   │
│      Variable Substitution              │
│   - render(template, variables)         │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│    HTTP Request (Reqwest Client)        │
│   - Retry logic with exponential        │
│     backoff                              │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│          CallixResponse                 │
│       Response Handling                 │
│   - status(), is_success()              │
│   - text(), json(), bytes()             │
└─────────────────────────────────────────┘
```

### 2.2 Data Flow

1. **Initialization**: `CallixBuilder` creates `Callix` instance with configuration
2. **Request Creation**: `Callix::request()` returns `RequestBuilder` with provider/endpoint config
3. **Variable Binding**: `RequestBuilder::var()` adds template variables
4. **Template Rendering**: `TemplateEngine` substitutes variables in URL, headers, body
5. **HTTP Execution**: Reqwest sends request with retry logic
6. **Response Handling**: `CallixResponse` provides convenient response parsing

### 2.3 Core Modules

#### `client.rs`
Contains the main client and utility functions:
- `Callix` - Main client struct managing configuration and HTTP client
- `parse_method()` - Converts string method names to `reqwest::Method`

**Key Responsibilities:**
- Maintain configuration and HTTP client
- Provide request builder factory
- Parse and validate HTTP methods

#### `config.rs`
Manages configuration loading and validation:
- `Config` - Top-level configuration container
- `ProviderConfig` - Provider-specific settings (base URL, headers, timeout)
- `EndpointConfig` - Individual endpoint definitions (path, method, body template)

**Key Responsibilities:**
- Load YAML configuration files
- Provide default configurations for popular AI services
- Validate configuration structure

#### `request.rs`
Handles request building and execution:
- `RequestBuilder` - Fluent API for constructing requests
- Request execution with automatic retry logic
- URL building with query parameter support

**Key Responsibilities:**
- Build complete HTTP requests from templates
- Execute requests with retry logic
- Manage custom headers and variables

#### `response.rs`
Wraps and simplifies response handling:
- `CallixResponse` - Wrapper around `reqwest::Response`
- Convenience methods for common response operations

**Key Responsibilities:**
- Provide type-safe response parsing
- Handle response status checking
- Extract response data in various formats

#### `template.rs`
Powers the variable substitution system:
- `TemplateEngine` - Template parsing and rendering
- Support for nested JSON values in templates

**Key Responsibilities:**
- Parse templates with `{{variable}}` syntax
- Substitute variables with provided values
- Handle complex JSON structures

#### `error.rs`
Defines comprehensive error types:
- `CallixError` - Enumeration of all possible errors
- Type conversions from underlying errors
- Clear error messages for debugging

**Key Responsibilities:**
- Provide descriptive error types
- Enable proper error handling
- Convert errors from dependencies

---

## 3. Getting Started

### 3.1 Installation

Add Callix to your project:

```toml
[dependencies]
callix = "0.1.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

### 3.2 Your First Request

```rust
use callix::CallixBuilder;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let callix = CallixBuilder::new()
        .timeout(Duration::from_secs(60))
        .build()?;

    // Make request
    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", "sk-...")
        .var("model", "gpt-3.5-turbo")
        .var("messages", json!([
            {"role": "user", "content": "Hello!"}
        ]))
        .send()
        .await?;

    // Handle response
    if response.is_success() {
        let json: serde_json::Value = response.json().await?;
        println!("{}", json["choices"][0]["message"]["content"]);
    }

    Ok(())
}
```

### 3.3 Understanding the Flow

1. **Build Client**: `CallixBuilder` configures the client
2. **Select Endpoint**: `request("provider", "endpoint")` chooses the API
3. **Set Variables**: `var()` provides template values
4. **Send Request**: `send()` executes with retry logic
5. **Parse Response**: `json()` or `text()` extracts data

---

## 4. Core Concepts

### 4.1 Providers

A **Provider** represents an external service or API. Each provider has:
- Base URL
- Default headers
- Optional timeout
- Multiple endpoints

```yaml
providers:
  openai:
    base_url: "https://api.openai.com"
    headers:
      Authorization: "Bearer {{API_KEY}}"
      Content-Type: "application/json"
    timeout: 60
```

### 4.2 Endpoints

An **Endpoint** represents a specific API operation within a provider:

```yaml
endpoints:
  chat:
    path: "/v1/chat/completions"
    method: "POST"
    body_template: |
      {
        "model": "{{model}}",
        "messages": {{messages}}
      }
```

**Components:**
- `path` - URL path relative to base URL
- `method` - HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- `body_template` - Optional request body with variable placeholders
- `query_params` - Optional URL query parameters

### 4.3 Variables

**Variables** are placeholders in templates that get replaced with actual values:

```rust
.var("model", "gpt-4")
.var("temperature", 0.7)
.var("messages", json!([...]))
```

Variables can be:
- Strings
- Numbers (integers, floats)
- Booleans
- JSON objects
- JSON arrays
- Null values

### 4.4 Template Syntax

Templates use `{{variable_name}}` syntax:

```yaml
body_template: |
  {
    "model": "{{model}}",
    "messages": {{messages}},
    "temperature": {{temperature}}
  }
```

The template engine:
- Replaces `{{name}}` with the value of variable `name`
- Preserves JSON structure for object/array variables
- Validates that all variables are provided

### 4.5 Request Lifecycle

```
CallixBuilder::build()
       ↓
   Callix instance created
       ↓
Callix::request(provider, endpoint)
       ↓
   RequestBuilder created
       ↓
RequestBuilder::var(...) (repeat as needed)
       ↓
RequestBuilder::send()
       ↓
   Template rendering
       ↓
   HTTP request execution (with retry)
       ↓
   CallixResponse returned
```

---

## 5. API Reference

### 5.1 CallixBuilder

The builder for configuring and creating a `Callix` client.

#### `CallixBuilder::new() -> Self`

Creates a new builder with default settings:
- Timeout: 30 seconds
- Retries: 3
- Retry delay: 1 second
- Config: Default (built-in providers)

```rust
let builder = CallixBuilder::new();
```

#### `config(self, path: impl Into<String>) -> Self`

Specifies a custom configuration file path.

```rust
let builder = CallixBuilder::new()
    .config("my-config.yaml");
```

#### `timeout(self, duration: Duration) -> Self`

Sets the request timeout duration.

```rust
let builder = CallixBuilder::new()
    .timeout(Duration::from_secs(60));
```

#### `retries(self, count: u32) -> Self`

Sets the maximum number of retry attempts.

```rust
let builder = CallixBuilder::new()
    .retries(5);
```

#### `retry_delay(self, duration: Duration) -> Self`

Sets the delay between retry attempts.

```rust
let builder = CallixBuilder::new()
    .retry_delay(Duration::from_secs(2));
```

#### `build(self) -> Result<Callix>`

Builds and returns the `Callix` client.

```rust
let callix = CallixBuilder::new()
    .timeout(Duration::from_secs(60))
    .retries(3)
    .build()?;
```

**Returns:** `Result<Callix>` - The configured client or an error

**Errors:**
- `CallixError::ConfigNotFound` - If config file doesn't exist
- `CallixError::InvalidConfig` - If config is malformed

### 5.2 Callix

The main HTTP client instance.

#### `request(&self, provider: &str, endpoint: &str) -> Result<RequestBuilder>`

Creates a new request builder for the specified provider and endpoint.

```rust
let builder = callix.request("openai", "chat")?;
```

**Parameters:**
- `provider` - Provider name from configuration
- `endpoint` - Endpoint name within the provider

**Returns:** `Result<RequestBuilder>` - A request builder or an error

**Errors:**
- `CallixError::ProviderNotFound` - If provider doesn't exist
- `CallixError::EndpointNotFound` - If endpoint doesn't exist

### 5.3 RequestBuilder

Fluent API for constructing and executing HTTP requests.

#### `var<T: Serialize>(self, key: impl Into<String>, value: T) -> Self`

Sets a single template variable.

```rust
let builder = builder
    .var("model", "gpt-4")
    .var("temperature", 0.7)
    .var("max_tokens", 100);
```

**Parameters:**
- `key` - Variable name (must match template placeholders)
- `value` - Value (automatically serialized to JSON)

**Returns:** `Self` - The builder for chaining

**Supported Types:**
- Primitives: `String`, `&str`, `i32`, `u64`, `f64`, `bool`
- JSON: `serde_json::Value`
- Custom types implementing `Serialize`

#### `vars(self, variables: HashMap<String, Value>) -> Self`

Sets multiple template variables at once.

```rust
use std::collections::HashMap;
use serde_json::{json, Value};

let mut vars = HashMap::new();
vars.insert("model".to_string(), json!("gpt-4"));
vars.insert("temperature".to_string(), json!(0.7));

let builder = builder.vars(vars);
```

**Parameters:**
- `variables` - HashMap of variable names to JSON values

**Returns:** `Self` - The builder for chaining

#### `header(self, key: impl Into<String>, value: impl Into<String>) -> Self`

Adds a custom HTTP header to the request.

```rust
let builder = builder
    .header("X-Custom-Header", "value")
    .header("X-Request-ID", "12345");
```

**Parameters:**
- `key` - Header name
- `value` - Header value

**Returns:** `Self` - The builder for chaining

**Note:** Custom headers override provider default headers if they share the same name.

#### `async send(self) -> Result<CallixResponse>`

Executes the HTTP request with automatic retry logic.

```rust
let response = builder.send().await?;
```

**Returns:** `Result<CallixResponse>` - The response or an error

**Errors:**
- `CallixError::HttpError` - HTTP-related errors
- `CallixError::TemplateError` - Missing or invalid template variables
- `CallixError::TimeoutError` - Request timeout exceeded
- `CallixError::MaxRetriesExceeded` - All retry attempts failed

**Retry Logic:**
- Retries on network errors and 5xx server errors
- Uses configured retry delay between attempts
- Stops after max retries exceeded

### 5.4 CallixResponse

Wrapper around `reqwest::Response` with convenience methods.

#### `status(&self) -> u16`

Returns the HTTP status code.

```rust
let status = response.status();
println!("Status: {}", status);
```

**Returns:** `u16` - HTTP status code (e.g., 200, 404, 500)

#### `is_success(&self) -> bool`

Checks if the status code is in the 2xx success range.

```rust
if response.is_success() {
    println!("Request succeeded!");
}
```

**Returns:** `bool` - `true` if status is 200-299

#### `headers(&self) -> &HeaderMap`

Returns a reference to the response headers.

```rust
for (key, value) in response.headers() {
    println!("{}: {:?}", key, value);
}
```

**Returns:** `&HeaderMap` - Reference to response headers

#### `async text(self) -> Result<String>`

Consumes the response and returns the body as a UTF-8 string.

```rust
let body = response.text().await?;
println!("{}", body);
```

**Returns:** `Result<String>` - Response body or an error

**Errors:**
- `CallixError::HttpError` - If body cannot be read or decoded

#### `async json<T: DeserializeOwned>(self) -> Result<T>`

Consumes the response and deserializes the body as JSON.

```rust
let json: serde_json::Value = response.json().await?;
println!("{:#?}", json);

// Or deserialize to a specific type
#[derive(Deserialize)]
struct ApiResponse {
    result: String,
    code: i32,
}

let data: ApiResponse = response.json().await?;
```

**Returns:** `Result<T>` - Deserialized data or an error

**Errors:**
- `CallixError::HttpError` - If body cannot be read or parsed

#### `async bytes(self) -> Result<Vec<u8>>`

Consumes the response and returns the raw body as bytes.

```rust
let bytes = response.bytes().await?;
println!("Received {} bytes", bytes.len());
```

**Returns:** `Result<Vec<u8>>` - Response body bytes or an error

**Errors:**
- `CallixError::HttpError` - If body cannot be read

---

## 6. Configuration Guide

### 6.1 Configuration File Structure

```yaml
providers:
  provider_name:
    base_url: "https://api.example.com"
    headers:
      Header-Name: "{{VARIABLE}}"
      Another-Header: "static-value"
    timeout: 60  # Optional, in seconds
    endpoints:
      endpoint_name:
        path: "/v1/endpoint"
        method: "POST"
        body_template: |
          {
            "key": "{{value}}"
          }
        query_params:  # Optional
          param: "{{param_value}}"
```

### 6.2 Provider Configuration

Each provider requires:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `base_url` | String | Yes | Base URL for all endpoints |
| `headers` | Map | No | Default headers for all requests |
| `timeout` | Integer | No | Request timeout in seconds |
| `endpoints` | Map | Yes | Endpoint definitions |

**Example:**

```yaml
providers:
  my_service:
    base_url: "https://api.myservice.com"
    headers:
      Authorization: "Bearer {{API_KEY}}"
      User-Agent: "Callix/0.1.0"
    timeout: 30
    endpoints:
      # ... endpoint definitions
```

### 6.3 Endpoint Configuration

Each endpoint requires:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | String | Yes | URL path (can contain variables) |
| `method` | String | Yes | HTTP method |
| `body_template` | String | No | Request body template |
| `query_params` | Map | No | Query parameter definitions |

**Supported HTTP Methods:**
- GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS

**Example:**

```yaml
endpoints:
  create_user:
    path: "/users"
    method: "POST"
    body_template: |
      {
        "name": "{{name}}",
        "email": "{{email}}",
        "role": "{{role}}"
      }

  get_user:
    path: "/users/{{user_id}}"
    method: "GET"

  search_users:
    path: "/users/search"
    method: "GET"
    query_params:
      q: "{{query}}"
      limit: "{{limit}}"
      offset: "{{offset}}"
```

### 6.4 Default Configuration

Callix includes pre-configured providers for popular AI services.

#### OpenAI Configuration

```yaml
openai:
  base_url: "https://api.openai.com"
  headers:
    Authorization: "Bearer {{API_KEY}}"
    Content-Type: "application/json"
  endpoints:
    chat:
      path: "/v1/chat/completions"
      method: "POST"
      body_template: |
        {
          "model": "{{model}}",
          "messages": {{messages}}
        }

    completions:
      path: "/v1/completions"
      method: "POST"
      body_template: |
        {
          "model": "{{model}}",
          "prompt": "{{prompt}}"
        }

    embeddings:
      path: "/v1/embeddings"
      method: "POST"
      body_template: |
        {
          "model": "{{model}}",
          "input": "{{input}}"
        }
```

#### Google Gemini Configuration

```yaml
gemini:
  base_url: "https://generativelanguage.googleapis.com"
  endpoints:
    generate:
      path: "/v1beta/models/{{model}}:generateContent"
      method: "POST"
      body_template: |
        {
          "contents": [{
            "parts": [{
              "text": "{{prompt}}"
            }]
          }]
        }
      query_params:
        key: "{{API_KEY}}"
```

#### Anthropic Claude Configuration

```yaml
anthropic:
  base_url: "https://api.anthropic.com"
  headers:
    x-api-key: "{{API_KEY}}"
    anthropic-version: "2023-06-01"
    Content-Type: "application/json"
  endpoints:
    messages:
      path: "/v1/messages"
      method: "POST"
      body_template: |
        {
          "model": "{{model}}",
          "max_tokens": {{max_tokens}},
          "messages": {{messages}}
        }
```

### 6.5 Custom Configuration Examples

#### REST API with Authentication

```yaml
providers:
  rest_api:
    base_url: "https://api.example.com"
    headers:
      Authorization: "Bearer {{API_TOKEN}}"
      X-API-Version: "2.0"
    timeout: 30
    endpoints:
      list_items:
        path: "/items"
        method: "GET"
        query_params:
          page: "{{page}}"
          per_page: "{{per_page}}"

      create_item:
        path: "/items"
        method: "POST"
        body_template: |
          {
            "title": "{{title}}",
            "description": "{{description}}",
            "metadata": {{metadata}}
          }

      update_item:
        path: "/items/{{item_id}}"
        method: "PUT"
        body_template: |
          {
            "title": "{{title}}",
            "status": "{{status}}"
          }

      delete_item:
        path: "/items/{{item_id}}"
        method: "DELETE"
```

#### Weather API

```yaml
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
          units: "{{units}}"

      forecast:
        path: "/v1/forecast"
        method: "GET"
        query_params:
          city: "{{city}}"
          days: "{{days}}"
```

#### Microservice Communication

```yaml
providers:
  user_service:
    base_url: "http://user-service:8080"
    headers:
      X-Service-Token: "{{SERVICE_TOKEN}}"
    timeout: 10
    endpoints:
      get_profile:
        path: "/api/users/{{user_id}}"
        method: "GET"

      update_profile:
        path: "/api/users/{{user_id}}"
        method: "PATCH"
        body_template: |
          {
            "fields": {{fields}}
          }
```

### 6.6 Environment Variables

Load sensitive data from environment variables:

```rust
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    // Read API keys
    let openai_key = env::var("OPENAI_API_KEY")?;
    let gemini_key = env::var("GEMINI_API_KEY")?;

    let callix = CallixBuilder::new().build()?;

    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", openai_key)
        .var("model", "gpt-4")
        .var("messages", json!([...]))
        .send()
        .await?;

    Ok(())
}
```

**.env file:**

```bash
# .env
OPENAI_API_KEY=sk-...
GEMINI_API_KEY=AIza...
ANTHROPIC_API_KEY=sk-ant-...
SERVICE_TOKEN=secret123
```

**Important:** Never commit `.env` files to version control. Add to `.gitignore`:

```gitignore
.env
.env.local
.env.*.local
```

---

## 7. Template Engine

### 7.1 Overview

The template engine provides variable substitution in configuration strings using `{{variable_name}}` syntax.

### 7.2 Basic Variable Substitution

```rust
// Configuration:
// path: "/users/{{user_id}}/posts/{{post_id}}"

let response = callix
    .request("api", "get_post")?
    .var("user_id", 123)
    .var("post_id", 456)
    .send()
    .await?;

// Result: /users/123/posts/456
```

### 7.3 Supported Data Types

#### Strings

```rust
.var("name", "Alice")
.var("email", "alice@example.com")
```

**Template:**
```json
{
  "user": {
    "name": "{{name}}",
    "email": "{{email}}"
  }
}
```

**Result:**
```json
{
  "user": {
    "name": "Alice",
    "email": "alice@example.com"
  }
}
```

#### Numbers

```rust
.var("age", 30)
.var("temperature", 0.7)
.var("max_tokens", 1000)
```

**Template:**
```json
{
  "age": {{age}},
  "settings": {
    "temperature": {{temperature}},
    "max_tokens": {{max_tokens}}
  }
}
```

**Result:**
```json
{
  "age": 30,
  "settings": {
    "temperature": 0.7,
    "max_tokens": 1000
  }
}
```

#### Booleans

```rust
.var("active", true)
.var("stream", false)
```

**Template:**
```json
{
  "active": {{active}},
  "stream": {{stream}}
}
```

**Result:**
```json
{
  "active": true,
  "stream": false
}
```

#### Null Values

```rust
.var("optional", serde_json::Value::Null)
```

**Template:**
```json
{
  "optional": {{optional}}
}
```

**Result:**
```json
{
  "optional": null
}
```

#### JSON Objects

```rust
.var("user", json!({
    "id": 1,
    "name": "Alice",
    "roles": ["admin", "user"]
}))
```

**Template:**
```json
{
  "user": {{user}}
}
```

**Result:**
```json
{
  "user": {
    "id": 1,
    "name": "Alice",
    "roles": ["admin", "user"]
  }
}
```

#### JSON Arrays

```rust
.var("messages", json!([
    {"role": "system", "content": "You are helpful"},
    {"role": "user", "content": "Hello"}
]))
```

**Template:**
```json
{
  "messages": {{messages}}
}
```

**Result:**
```json
{
  "messages": [
    {"role": "system", "content": "You are helpful"},
    {"role": "user", "content": "Hello"}
  ]
}
```

### 7.4 Complex Template Examples

#### Nested Structures

```rust
let response = callix
    .request("api", "complex")?
    .var("model", "gpt-4")
    .var("messages", json!([
        {"role": "user", "content": "Hello"}
    ]))
    .var("temperature", 0.7)
    .var("max_tokens", 100)
    .var("stream", false)
    .var("metadata", json!({
        "user_id": "123",
        "session_id": "abc",
        "timestamp": 1234567890
    }))
    .send()
    .await?;
```

**Template:**
```json
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

#### Dynamic Arrays

```rust
let tags = vec!["rust", "programming", "async"];

.var("tags", json!(tags))
```

**Template:**
```json
{
  "post": {
    "title": "{{title}}",
    "tags": {{tags}}
  }
}
```

### 7.5 Template Validation

The template engine validates templates at runtime:

```rust
// This will fail with CallixError::TemplateError
let result = callix
    .request("openai", "chat")?
    // Missing required variable: API_KEY
    .var("model", "gpt-4")
    .send()
    .await;

assert!(result.is_err());
```

**Best Practice:** Always provide all required variables before calling `send()`.

### 7.6 Template Performance

The template engine:
- **Fast Path**: If no `{{` found, returns original string without allocation
- **Minimal Allocations**: Only allocates when substitution is needed
- **Early Validation**: Errors on missing variables before making HTTP request

```rust
// Fast path: no template variables
path: "/static/path"  // No allocation

// Slow path: requires substitution
path: "/users/{{user_id}}"  // Allocates new string
```

---

## 8. Error Handling

### 8.1 Error Types

```rust
pub enum CallixError {
    ConfigNotFound,
    InvalidConfig,
    ProviderNotFound,
    EndpointNotFound(String),
    HttpError(reqwest::Error),
    TemplateError,
    TimeoutError,
    MaxRetriesExceeded,
    InvalidMethod,
}
```

### 8.2 Error Descriptions

| Error | Description | Common Causes |
|-------|-------------|---------------|
| `ConfigNotFound` | Configuration file not found | Wrong file path, missing file |
| `InvalidConfig` | Configuration is malformed | Invalid YAML syntax, missing required fields |
| `ProviderNotFound` | Provider doesn't exist | Typo in provider name, provider not in config |
| `EndpointNotFound` | Endpoint doesn't exist | Typo in endpoint name, endpoint not defined |
| `HttpError` | HTTP request failed | Network issues, server errors, invalid response |
| `TemplateError` | Template rendering failed | Missing variable, invalid JSON in variable |
| `TimeoutError` | Request timed out | Server not responding, timeout too short |
| `MaxRetriesExceeded` | All retry attempts failed | Persistent server error, network issues |
| `InvalidMethod` | HTTP method is invalid | Unsupported method in configuration |

### 8.3 Error Handling Patterns

#### Basic Error Handling

```rust
match callix.request("openai", "chat") {
    Ok(builder) => {
        // Use the builder
        let response = builder.send().await?;
    }
    Err(CallixError::ProviderNotFound) => {
        eprintln!("Provider 'openai' not found in configuration");
    }
    Err(CallixError::EndpointNotFound(name)) => {
        eprintln!("Endpoint '{}' not found", name);
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
                // Process successful response
                Ok(json)
            }
            400 => {
                eprintln!("Bad request - check your input parameters");
                Err("Invalid request".into())
            }
            401 => {
                eprintln!("Unauthorized - verify your API key");
                Err("Authentication failed".into())
            }
            429 => {
                eprintln!("Rate limited - please retry after a delay");
                Err("Rate limit exceeded".into())
            }
            500..=599 => {
                eprintln!("Server error - the API is experiencing issues");
                Err("Server error".into())
            }
            code => {
                eprintln!("Unexpected status code: {}", code);
                Err(format!("HTTP {}", code).into())
            }
        }
    }
    Err(CallixError::TimeoutError) => {
        eprintln!("Request timed out");
        Err("Timeout".into())
    }
    Err(CallixError::MaxRetriesExceeded) => {
        eprintln!("Max retries exceeded - service may be down");
        Err("Max retries exceeded".into())
    }
    Err(CallixError::TemplateError) => {
        eprintln!("Template error - check your variables");
        Err("Template error".into())
    }
    Err(CallixError::HttpError(e)) => {
        eprintln!("HTTP error: {}", e);
        Err(Box::new(e))
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
        Err(Box::new(e))
    }
}
```

#### Using Result Combinators

```rust
// Chaining operations with ?
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
    .ok_or("No content in response")?;

println!("Response: {}", content);
```

#### Custom Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Callix error: {0}")]
    Callix(#[from] CallixError),

    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("API returned invalid response")]
    InvalidResponse,
}

async fn call_api() -> Result<String, AppError> {
    let api_key = std::env::var("API_KEY")?;

    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", api_key)
        .send()
        .await?;

    if !response.is_success() {
        return Err(AppError::InvalidResponse);
    }

    let json: serde_json::Value = response.json().await?;

    json["choices"][0]["message"]["content"]
        .as_str()
        .map(String::from)
        .ok_or(AppError::InvalidResponse)
}
```

### 8.4 Retry Error Handling

```rust
use tokio::time::sleep;
use std::time::Duration;

async fn call_with_backoff() -> Result<CallixResponse, Box<dyn std::error::Error>> {
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
                    return Err("Rate limit exceeded after max retries".into());
                }
                // Exponential backoff
                let delay = 2_u64.pow(attempts);
                println!("Rate limited, waiting {}s...", delay);
                sleep(Duration::from_secs(delay)).await;
            }
            Ok(response) => {
                return Err(format!("HTTP {}", response.status()).into());
            }
            Err(e) if attempts < max_attempts => {
                println!("Attempt {} failed, retrying...", attempts);
                sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
}
```

---

## 9. Advanced Usage

### 9.1 Concurrent Requests

Execute multiple requests concurrently using Tokio's join utilities:

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
            .var("messages", json!([
                {"role": "user", "content": "Task 1"}
            ]))
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
            .var("prompt", "Task 2")
            .send()
            .await
    }
});

let (result1, result2) = tokio::join!(handle1, handle2);
```

### 9.2 Using `join_all` for Dynamic Lists

```rust
use futures::future::join_all;

let prompts = vec![
    "Explain machine learning",
    "What is quantum computing?",
    "Describe neural networks"
];

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

for (prompt, result) in prompts.iter().zip(results.iter()) {
    match result {
        Ok(response) => println!("{}: Success", prompt),
        Err(e) => println!("{}: Error - {}", prompt, e),
    }
}
```

### 9.3 Rate Limiting

Implement rate limiting for API calls:

```rust
use tokio::time::{sleep, Duration};

struct RateLimiter {
    calls_per_minute: u32,
    delay: Duration,
}

impl RateLimiter {
    fn new(calls_per_minute: u32) -> Self {
        let delay = Duration::from_secs(60) / calls_per_minute;
        Self { calls_per_minute, delay }
    }

    async fn call<F, Fut, T>(&self, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let result = f().await;
        sleep(self.delay).await;
        result
    }
}

// Usage
let limiter = RateLimiter::new(10); // 10 calls per minute

for prompt in prompts {
    let response = limiter.call(|| {
        callix
            .request("openai", "chat")?
            .var("API_KEY", api_key)
            .var("messages", json!([{"role": "user", "content": prompt}]))
            .send()
    }).await?;
}
```

### 9.4 Connection Pooling

Reqwest (and thus Callix) automatically manages connection pooling. To optimize:

```rust
// Create ONE client instance and reuse it
let callix = CallixBuilder::new()
    .timeout(Duration::from_secs(60))
    .build()?;

// Reuse for all requests
for i in 0..100 {
    let response = callix.request("api", "endpoint")?.send().await?;
}
```

**Don't do this:**

```rust
// Bad: Creates new client each time
for i in 0..100 {
    let callix = CallixBuilder::new().build()?; // Expensive!
    let response = callix.request("api", "endpoint")?.send().await?;
}
```

### 9.5 Streaming Responses

For APIs that support streaming (requires `stream` feature):

```rust
// Note: Streaming requires enabling the 'stream' feature
// [dependencies]
// callix = { version = "0.1", features = ["stream"] }

let response = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key)
    .var("model", "gpt-4")
    .var("stream", true)
    .var("messages", messages)
    .send()
    .await?;

// Process stream
use futures::StreamExt;
let mut stream = response.bytes_stream();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    // Process each chunk
    println!("Received {} bytes", chunk.len());
}
```

### 9.6 Request Middleware Pattern

Create reusable middleware for common operations:

```rust
use std::time::Instant;

struct LoggingMiddleware;

impl LoggingMiddleware {
    async fn execute<F, Fut>(f: F) -> Result<CallixResponse>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<CallixResponse>>,
    {
        let start = Instant::now();
        println!("[REQUEST] Starting...");

        let result = f().await;

        let elapsed = start.elapsed();
        match &result {
            Ok(response) => {
                println!("[SUCCESS] Status: {} in {:?}", response.status(), elapsed);
            }
            Err(e) => {
                println!("[ERROR] Failed in {:?}: {}", elapsed, e);
            }
        }

        result
    }
}

// Usage
let response = LoggingMiddleware::execute(|| {
    callix
        .request("openai", "chat")?
        .var("API_KEY", api_key)
        .send()
}).await?;
```

### 9.7 Multi-Provider Failover

Implement failover across multiple providers:

```rust
async fn call_with_failover(prompts: Vec<&str>) -> Result<String> {
    let providers = vec![
        ("openai", "chat"),
        ("anthropic", "messages"),
        ("gemini", "generate"),
    ];

    for (provider, endpoint) in providers {
        match attempt_call(provider, endpoint, prompts).await {
            Ok(result) => {
                println!("Success with {}", provider);
                return Ok(result);
            }
            Err(e) => {
                println!("Failed with {}: {}", provider, e);
                continue;
            }
        }
    }

    Err("All providers failed".into())
}

async fn attempt_call(
    provider: &str,
    endpoint: &str,
    prompt: &str,
) -> Result<String> {
    let response = callix
        .request(provider, endpoint)?
        .var("API_KEY", get_api_key(provider)?)
        .var("model", get_model(provider))
        .var("messages", json!([{"role": "user", "content": prompt}]))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    extract_content(&json, provider)
}
```

### 9.8 Request Caching

Implement simple response caching:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

struct CachedClient {
    callix: Callix,
    cache: Arc<Mutex<HashMap<String, serde_json::Value>>>,
}

impl CachedClient {
    fn new(callix: Callix) -> Self {
        Self {
            callix,
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn get_or_fetch(
        &self,
        key: &str,
        provider: &str,
        endpoint: &str,
    ) -> Result<serde_json::Value> {
        // Check cache
        {
            let cache = self.cache.lock().await;
            if let Some(value) = cache.get(key) {
                return Ok(value.clone());
            }
        }

        // Fetch from API
        let response = self.callix
            .request(provider, endpoint)?
            .var("API_KEY", api_key)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        // Store in cache
        {
            let mut cache = self.cache.lock().await;
            cache.insert(key.to_string(), json.clone());
        }

        Ok(json)
    }
}
```

---

## 10. Best Practices

### 10.1 Configuration Management

**DO:**

```rust
// Store API keys in environment variables
let api_key = env::var("OPENAI_API_KEY")?;

// Use separate configs for different environments
let config = if cfg!(debug_assertions) {
    "config.dev.yaml"
} else {
    "config.prod.yaml"
};

// Version control your config files (without secrets)
// config.yaml.example
```

**DON'T:**

```rust
// Never hardcode API keys
let api_key = "sk-hardcoded-key"; // NEVER DO THIS!

// Don't commit API keys to git
// .env files should be in .gitignore

// Don't use production keys in development
```

### 10.2 Error Handling

**DO:**

```rust
// Always handle errors explicitly
match response.status() {
    200..=299 => Ok(response.json().await?),
    401 => Err("Unauthorized".into()),
    _ => Err(format!("HTTP {}", response.status()).into()),
}

// Log errors with context
eprintln!("Failed to call {} endpoint: {}", endpoint, e);

// Implement exponential backoff for retries
let delay = 2_u64.pow(attempt);
```

**DON'T:**

```rust
// Never use unwrap() in production
let response = builder.send().await.unwrap(); // AVOID!

// Don't ignore error types
if let Err(_) = result {
    // No information about what went wrong
}

// Don't retry indefinitely
loop {
    // This will never stop!
}
```

### 10.3 Performance

**DO:**

```rust
// Reuse Callix instances
let callix = CallixBuilder::new().build()?;
for _ in 0..100 {
    callix.request("api", "endpoint")?.send().await?;
}

// Use concurrent requests when possible
let futures = /* ... */;
let results = join_all(futures).await;

// Set appropriate timeouts
.timeout(Duration::from_secs(30))

// Use connection pooling (automatic with reqwest)
```

**DON'T:**

```rust
// Don't create new clients repeatedly
for _ in 0..100 {
    let callix = CallixBuilder::new().build()?; // Expensive!
}

// Don't block the async runtime
std::thread::sleep(Duration::from_secs(1)); // Use tokio::time::sleep

// Don't set excessively long timeouts
.timeout(Duration::from_secs(3600)) // 1 hour is too long

// Don't make sequential calls when concurrent is possible
for item in items {
    process(item).await; // Should be concurrent
}
```

### 10.4 Security

**DO:**

```rust
// Validate all input
fn validate_model(model: &str) -> Result<()> {
    let valid = ["gpt-4", "gpt-3.5-turbo"];
    if valid.contains(&model) {
        Ok(())
    } else {
        Err("Invalid model".into())
    }
}

// Use HTTPS everywhere (default in Callix)

// Rotate API keys regularly

// Implement rate limiting
```

**DON'T:**

```rust
// Don't trust user input blindly
.var("model", user_input) // Validate first!

// Don't log sensitive data
println!("API Key: {}", api_key); // NEVER!

// Don't share API keys across environments

// Don't use deprecated TLS versions
```

### 10.5 Testing

**DO:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_building() {
        let callix = CallixBuilder::new().build().unwrap();
        let builder = callix.request("openai", "chat").unwrap();
        // Assert on builder properties
    }

    #[tokio::test]
    async fn test_template_rendering() {
        // Test template engine directly
    }
}

// Use mock servers for testing
// Use test configurations separate from production
```

**DON'T:**

```rust
// Don't make real API calls in tests
#[tokio::test]
async fn test_api() {
    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", "real-key") // Don't use real keys!
        .send()
        .await?;
}

// Don't skip error case testing
```

---

## 11. Security

### 11.1 API Key Management

**Environment Variables (Recommended):**

```rust
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    // Read API key
    let api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set");

    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", api_key)
        .send()
        .await?;

    Ok(())
}
```

**.env file:**

```bash
OPENAI_API_KEY=sk-...
GEMINI_API_KEY=AIza...
```

**Add to .gitignore:**

```gitignore
.env
.env.local
.env.*.local
```

### 11.2 TLS Configuration

Callix uses secure HTTPS connections by default via reqwest.

**Using Rustls (Pure Rust TLS):**

```toml
[dependencies]
callix = { version = "0.1", features = ["rustls-tls"], default-features = false }
```

**Using Native TLS (System):**

```toml
[dependencies]
callix = { version = "0.1", features = ["native-tls"] }
```

### 11.3 Input Validation

Always validate user input before using in requests:

```rust
fn validate_model(model: &str) -> Result<(), String> {
    let valid_models = [
        "gpt-4",
        "gpt-3.5-turbo",
        "claude-3-5-sonnet-20241022",
    ];

    if valid_models.contains(&model) {
        Ok(())
    } else {
        Err(format!("Invalid model: {}", model))
    }
}

// Usage
let user_model = get_user_input();
validate_model(&user_model)?;

let response = callix
    .request("openai", "chat")?
    .var("model", user_model)
    .send()
    .await?;
```

### 11.4 Sensitive Data Handling

**DO:**

```rust
// Don't log full requests
println!("Making request to {}", provider);

// Don't log responses that might contain sensitive data
if response.is_success() {
    println!("Request succeeded");
}

// Sanitize error messages
match result {
    Err(e) => eprintln!("Request failed: {}", sanitize_error(&e)),
    Ok(_) => { /* ... */ }
}
```

**DON'T:**

```rust
// Never log API keys
println!("Using API key: {}", api_key); // NEVER!

// Don't log full request bodies
println!("Request body: {:?}", body); // May contain sensitive data

// Don't expose internal errors to users
return Err(format!("Database error: {}", db_error)); // Too much info
```

### 11.5 Rate Limiting

Implement rate limiting to prevent abuse:

```rust
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;

struct RateLimiter {
    last_call: Arc<Mutex<Instant>>,
    min_interval: Duration,
}

impl RateLimiter {
    fn new(calls_per_second: u32) -> Self {
        Self {
            last_call: Arc::new(Mutex::new(Instant::now())),
            min_interval: Duration::from_secs(1) / calls_per_second,
        }
    }

    async fn wait(&self) {
        let mut last = self.last_call.lock().await;
        let elapsed = last.elapsed();

        if elapsed < self.min_interval {
            tokio::time::sleep(self.min_interval - elapsed).await;
        }

        *last = Instant::now();
    }
}
```

---

## 12. Performance

### 12.1 Client Reuse

**Correct:**

```rust
// Create ONE client for your application
let callix = CallixBuilder::new()
    .timeout(Duration::from_secs(60))
    .build()?;

// Reuse it for all requests
for item in items {
    let response = callix
        .request("api", "endpoint")?
        .var("data", item)
        .send()
        .await?;
}
```

**Wrong:**

```rust
// Don't create new clients for each request
for item in items {
    let callix = CallixBuilder::new().build()?; // Expensive!
    let response = callix
        .request("api", "endpoint")?
        .send()
        .await?;
}
```

### 12.2 Concurrent Requests

Use `join_all` for concurrent execution:

```rust
use futures::future::join_all;

let futures: Vec<_> = items.iter().map(|item| {
    let callix = callix.clone();
    async move {
        callix
            .request("api", "endpoint")?
            .var("data", item)
            .send()
            .await
    }
}).collect();

let results = join_all(futures).await;
```

**Benchmark:**
- Sequential: 10 requests × 100ms = 1000ms
- Concurrent: max(100ms) = 100ms (10x faster)

### 12.3 Timeout Configuration

Set realistic timeouts:

```rust
// Good defaults
let callix = CallixBuilder::new()
    .timeout(Duration::from_secs(30))  // Reasonable for most APIs
    .build()?;

// Adjust based on endpoint
// Quick endpoints
.timeout(Duration::from_secs(10))

// LLM streaming
.timeout(Duration::from_secs(120))
```

### 12.4 Memory Usage

Callix is designed to be memory-efficient:

- **Template Engine**: Fast path for strings without variables
- **Connection Pooling**: Automatic via reqwest
- **Response Streaming**: Use `bytes_stream()` for large responses

```rust
// For large responses, stream instead of loading into memory
let mut stream = response.bytes_stream();
while let Some(chunk) = stream.next().await {
    process_chunk(chunk?);
}
```

### 12.5 Benchmarking

Example benchmark using `criterion`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_request(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let callix = CallixBuilder::new().build().unwrap();

    c.bench_function("simple request", |b| {
        b.iter(|| {
            rt.block_on(async {
                callix
                    .request("api", "endpoint")
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
            })
        })
    });
}

criterion_group!(benches, benchmark_request);
criterion_main!(benches);
```

---

## 13. Troubleshooting

### 13.1 Common Issues

#### Config Not Found

**Error:**
```
Error: Config file not found: config.yaml
```

**Solutions:**

```rust
// Use absolute path
let callix = CallixBuilder::new()
    .config("/absolute/path/to/config.yaml")
    .build()?;

// Or verify relative path
use std::path::Path;
assert!(Path::new("config.yaml").exists());

// Or use default config
let callix = CallixBuilder::new().build()?; // Uses built-in config
```

#### Provider Not Found

**Error:**
```
Error: Provider not found
```

**Solutions:**

```rust
// Check spelling
callix.request("openai", "chat")?  // Correct
callix.request("OpenAI", "chat")?  // Wrong (case-sensitive)

// Verify provider exists in config
let config = Config::from_file("config.yaml")?;
println!("Available providers: {:?}", config.providers.keys());

// Use default providers
let callix = CallixBuilder::new().build()?;
// Available: openai, gemini, anthropic
```

#### Endpoint Not Found

**Error:**
```
Error: Endpoint not found: chats
```

**Solutions:**

```rust
// Check endpoint name
callix.request("openai", "chat")?       // Correct
callix.request("openai", "chats")?      // Wrong
callix.request("openai", "completions")? // Correct

// List available endpoints
let provider = config.get_provider("openai")?;
println!("Endpoints: {:?}", provider.endpoints.keys());
```

#### Template Error

**Error:**
```
Error: Template error
```

**Solutions:**

```rust
// Ensure all variables are provided
callix.request("openai", "chat")?
    .var("API_KEY", api_key)  // Required!
    .var("model", "gpt-4")    // Required!
    .var("messages", messages) // Required!
    .send()
    .await?;

// Check variable names match template
// Template: {{model}}
.var("model", "gpt-4")  // Correct
.var("Model", "gpt-4")  // Wrong (case-sensitive)
```

#### Timeout Error

**Error:**
```
Error: Request timeout
```

**Solutions:**

```rust
// Increase timeout
let callix = CallixBuilder::new()
    .timeout(Duration::from_secs(120))
    .build()?;

// Check network connectivity
// Check API endpoint status
// Verify API is responding
```

#### Max Retries Exceeded

**Error:**
```
Error: Max retries exceeded
```

**Solutions:**

```rust
// Check API status
// Verify API key is valid
// Check rate limits

// Increase retry count and delay
let callix = CallixBuilder::new()
    .retries(10)
    .retry_delay(Duration::from_secs(5))
    .build()?;
```

### 13.2 Debugging Tips

#### Enable Logging

```rust
// Add env_logger to Cargo.toml
// [dependencies]
// env_logger = "0.10"

use env_logger;

fn main() {
    env_logger::init();
    // Your code
}

// Run with: RUST_LOG=debug cargo run
```

#### Inspect Requests

```rust
let response = callix
    .request("openai", "chat")?
    .var("API_KEY", api_key)
    .send()
    .await?;

println!("Status: {}", response.status());
println!("Headers: {:#?}", response.headers());

if !response.is_success() {
    let body = response.text().await?;
    println!("Error body: {}", body);
}
```

#### Test Configuration

```rust
// Verify config loads correctly
let config = Config::from_file("config.yaml")?;
println!("Providers: {:?}", config.providers.keys());

for (name, provider) in &config.providers {
    println!("Provider: {}", name);
    println!("  Base URL: {}", provider.base_url);
    println!("  Endpoints: {:?}", provider.endpoints.keys());
}
```

#### Test Template Rendering

```rust
use callix::template::TemplateEngine;
use std::collections::HashMap;
use serde_json::json;

let mut vars = HashMap::new();
vars.insert("name".to_string(), json!("Alice"));

let template = "Hello, {{name}}!";
let result = TemplateEngine::render(template, &vars)?;
println!("Result: {}", result);
```

---

## 14. Cookbook

### 14.1 OpenAI ChatGPT

```rust
use callix::CallixBuilder;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let callix = CallixBuilder::new().build()?;

    let response = callix
        .request("openai", "chat")?
        .var("API_KEY", env::var("OPENAI_API_KEY")?)
        .var("model", "gpt-4")
        .var("messages", json!([
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": "Explain Rust ownership in simple terms"
            }
        ]))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap();

    println!("ChatGPT: {}", content);
    Ok(())
}
```

### 14.2 Google Gemini

```rust
let response = callix
    .request("gemini", "generate")?
    .var("API_KEY", env::var("GEMINI_API_KEY")?)
    .var("model", "gemini-2.0-flash-exp")
    .var("prompt", "Write a haiku about programming")
    .send()
    .await?;

let json: serde_json::Value = response.json().await?;
let text = json["candidates"][0]["content"]["parts"][0]["text"]
    .as_str()
    .unwrap();

println!("Gemini: {}", text);
```

### 14.3 Anthropic Claude

```rust
let response = callix
    .request("anthropic", "messages")?
    .var("API_KEY", env::var("ANTHROPIC_API_KEY")?)
    .var("model", "claude-3-5-sonnet-20241022")
    .var("max_tokens", 1024)
    .var("messages", json!([
        {
            "role": "user",
            "content": "Explain quantum entanglement"
        }
    ]))
    .send()
    .await?;

let json: serde_json::Value = response.json().await?;
let content = json["content"][0]["text"].as_str().unwrap();

println!("Claude: {}", content);
```

### 14.4 Custom REST API

```yaml
# custom-api.yaml
providers:
  myapi:
    base_url: "https://api.example.com"
    headers:
      Authorization: "Bearer {{API_TOKEN}}"
      Content-Type: "application/json"
    endpoints:
      get_users:
        path: "/users"
        method: "GET"
        query_params:
          page: "{{page}}"
          limit: "{{limit}}"

      create_user:
        path: "/users"
        method: "POST"
        body_template: |
          {
            "name": "{{name}}",
            "email": "{{email}}"
          }
```

```rust
let callix = CallixBuilder::new()
    .config("custom-api.yaml")
    .build()?;

// Get users
let response = callix
    .request("myapi", "get_users")?
    .var("API_TOKEN", token)
    .var("page", 1)
    .var("limit", 10)
    .send()
    .await?;

// Create user
let response = callix
    .request("myapi", "create_user")?
    .var("API_TOKEN", token)
    .var("name", "Alice")
    .var("email", "alice@example.com")
    .send()
    .await?;
```

### 14.5 Batch Processing with Progress

```rust
use indicatif::{ProgressBar, ProgressStyle};

let prompts = vec![
    "Prompt 1",
    "Prompt 2",
    "Prompt 3",
];

let progress = ProgressBar::new(prompts.len() as u64);
progress.set_style(
    ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
        .unwrap()
);

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

    progress.inc(1);
    progress.set_message(format!("Processed: {}", prompt));
}

progress.finish_with_message("Done!");
```

### 14.6 Error Recovery and Fallback

```rust
async fn call_with_fallback(prompt: &str) -> Result<String> {
    // Try primary provider
    match try_openai(prompt).await {
        Ok(result) => return Ok(result),
        Err(e) => {
            eprintln!("OpenAI failed: {}, trying Claude...", e);
        }
    }

    // Fallback to Claude
    match try_claude(prompt).await {
        Ok(result) => return Ok(result),
        Err(e) => {
            eprintln!("Claude failed: {}, trying Gemini...", e);
        }
    }

    // Last resort: Gemini
    try_gemini(prompt).await
}
```

---

## Appendix

### A. Error Reference

Complete enumeration of all `CallixError` variants with descriptions and common causes.

### B. Default Configuration

Full listing of built-in provider configurations.

### C. Migration Guide

Guide for migrating from other HTTP clients (reqwest, hyper, ureq) to Callix.

### D. Contributing

Guidelines for contributing to the Callix project:
- Code style
- Testing requirements
- Documentation standards
- Pull request process

---

**End of Documentation**

For the latest updates, visit: https://github.com/naseridev/callix