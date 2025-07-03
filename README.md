# Axum Log Util

A Rust procedural macro crate that provides logging utilities for HTTP request
handlers.

## Overview

`axum_log_util` is a procedural macro crate designed to automatically add
logging capabilities to async HTTP request handler functions. It wraps functions
with logging statements that record when endpoints are executed and their
response status codes.

## Features

- **Automatic Request Logging**: Logs when an endpoint function starts executing
- **Response Status Logging**: Logs the HTTP status code when the endpoint
  completes
- **Async Function Support**: Designed specifically for async HTTP handlers
- **Zero Configuration**: Simply add the attribute macro to your functions

## Usage

Add the `#[log_request]` attribute to your async HTTP handler functions:

```rust
use axum_log_util::log_request;

#[log_request]
pub async fn my_endpoint() -> Response {
    // Your endpoint logic here
    Response::new()
}
```

### What it does

The macro transforms your function to include logging statements:

1. **Before execution**: Logs `"Executing Endpoint: {function_name}"`
2. **After execution**: Logs `"{status_code}: Endpoint {function_name}"`

### Example Output

```
INFO: Executing Endpoint: my_endpoint
INFO: 200 OK: Endpoint my_endpoint
```

## Dependencies

- `syn` (v2.0): For parsing Rust syntax
- `quote` (v1.0): For generating Rust code
- `proc-macro`: Standard library for procedural macros

## Technical Details

This crate is a procedural macro library (`proc-macro = true`) that operates at
compile time to transform your functions. It:

1. Parses the input function using `syn`
2. Extracts the function name, inputs, and body
3. Wraps the original function body with logging statements
4. Returns the modified function using `quote!`

## Requirements

- Rust 2021 Edition
- Functions must be `async` and return a `Response` type
- Requires a logging framework (like `log`, `tracing`, etc.) to be configured in
  your application

## Development

To test the macro:

```bash
cargo test
```

To see the expanded macro output:

```bash
cargo expand
```
