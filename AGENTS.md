# CITA Cloud Common Rust (cloud-common-rs)

## Project Information
- **Name**: cloud-common-rs
- **Description**: Common Rust utilities and protocol definitions for CITA Cloud ecosystem.
- **Repository**: [cloud-common-rs](https://github.com/cita-cloud/cloud-common-rs)

## Core Components

### 1. cloud-proto
Contains gRPC protocol definitions and generated Rust code for CITA Cloud microservices.
- **Services**: Blockchain, Consensus, Controller, Crypto, Executor, Network, Storage.
- **Tools**: Includes `client` module for easy service connection.

### 2. cloud-util
Provides common utilities for building CITA Cloud components.
- **Tracing**: Distributed tracing with OpenTelemetry (v0.31) and Jaeger/OTLP support.
- **Metrics**: Prometheus metrics collection via `axum`.
- **Cryptography**: Common crypto traits and implementations.
- **Signal Handling**: Graceful shutdown support.
- **Storage**: Storage traits abstraction.

## Dependencies
- **Rust Toolchain**: 1.85+ (2024 edition).
- **Protobuf Compiler**: `protoc` (required for building `cloud-proto`).
- **OpenSSL**: `openssl-sys` dependency.

## Usage

### Updating Protocols
Initialize and update git submodules for proto files:
```bash
sh update_proto.sh
```

### Building
Build the project in release mode:
```bash
cargo build --release --all --all-features
```

### Using Tracing (Example)
```rust
use cloud_util::tracer::{init_tracer, LogConfig};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = LogConfig {
        service_name: "my-service".to_string(),
        agent_endpoint: Some("grpc://localhost:4317".to_string()), // OTLP endpoint
        ..Default::default()
    };
    
    // Initialize tracer
    init_tracer("my-domain".to_string(), &config)?;
    
    // ... application logic ...
    
    Ok(())
}
```

### Using gRPC Client (Example)
```rust
use cloud_proto::client::ClientOptions;

async fn connect() {
    let opts = ClientOptions::new(
        "client-name".to_string(),
        "http://localhost:50004".to_string()
    );
    
    let controller = opts.connect_controller();
    // ...
}
```
