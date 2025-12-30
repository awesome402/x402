# x402-reqwest

**Wrapper around [`reqwest`](https://crates.io/crates/reqwest) that transparently handles HTTP `402 Payment Required` responses using the [x402 protocol](https://x402.org/).**

This crate enables your [`reqwest-middleware`](https://crates.io/crates/reqwest-middleware)-based HTTP clients to:
- Detect `402 Payment Required` responses
- Build and sign [x402](https://x402.org) payment payloads
- Retry the request with the `X-Payment` header attached
- Respect client-defined preferences like token priority and per-token payment caps

All in all: **automatically pay for resources using the x402 protocol**.

Built with [`reqwest-middleware`] and compatible with any [`alloy::Signer`](https://alloy.rs).

## Features

- Pluggable middleware
- EIP-712-compatible signing with [`alloy`](https://alloy.rs)
- Fluent builder-style configuration
- Token preferences & per-asset payment limits

## Installation

Add the dependency:

```toml
# Cargo.toml
x402-reqwest = "0.4"
```

To enable tracing:

```toml
x402-reqwest = { version = "0.4", features = ["telemetry"] }
```

## 💡 Examples

```rust
use reqwest::Client;
use x402_reqwest::{ReqwestWithPayments, X402Client};
use alloy_signer_local::PrivateKeySigner;
use x402::scheme::v2_eip155_exact::client::V2Eip155ExactClient;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer: PrivateKeySigner = "0x...".parse()?; // never hardcode real keys!
    
    // Build x402 client with scheme handlers
    let x402_client = X402Client::new()
        .register(V2Eip155ExactClient::new(Arc::new(signer)));

    let client = Client::new()
        .with_payments(x402_client)
        .build();

    let res = client
        .get("https://example.com/protected")
        .send()
        .await?;

    println!("Status: {}", res.status());
    Ok(())
}
```

## How it works
1.	A 402 Payment Required is received from a server.
2.	The middleware parses the Payment-Required response body.
3.	A compatible payment requirement is selected, based on client preferences.
4.	A signed payload is created (compatible with [EIP-3009](https://eips.ethereum.org/EIPS/eip-3009) `TransferWithAuthorization`).
5.	The payload is base64-encoded into an `X-Payment` header.
6.	The request is retried, now with the payment inside the header.

## License

[MIT](LICENSE)
