# KuberNodes Rust SDK

Official Rust SDK for [KuberNodes](https://kubernodes.com) — multi-chain RPC infrastructure.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kubernodes = "1.0"
```

## Quick Start

```rust
use kubernodes::KuberNodesClient;

#[tokio::main]
async fn main() -> Result<(), kubernodes::Error> {
    let client = KuberNodesClient::builder("pk_live_your_key")
        .chain_id(1) // Ethereum mainnet
        .build()?;

    let block = client.get_block_number().await?;
    println!("Latest block: {block}");

    Ok(())
}
```

## Requirements

- Rust 1.70+ (2021 edition)
- Tokio runtime

## Features

- Typed RPC method wrappers for all supported chains
- Async/await with Tokio
- Automatic retries with exponential backoff
- Builder pattern configuration
- `thiserror`-based error types
- `serde` serialization/deserialization

## Configuration

```rust
let client = KuberNodesClient::builder("pk_live_your_key")
    .base_url("https://your-gateway.kubernodes.com")
    .chain_id(1)
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .build()?;
```

## Running Tests

```bash
cargo test
```

## License

MIT
