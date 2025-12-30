## Awesome402

> Rust implementation of the x402 protocol for blockchain payments over HTTP

**Awesome402** is a Rust implementation of the [x402 protocol](https://docs.cdp.coinbase.com/x402/docs/overview), enabling blockchain payments directly through HTTP using the `402 Payment Required` status code.

The protocol enables:
- Servers to declare payment requirements for specific routes
- Clients to send cryptographically signed payment payloads
- Facilitators to verify and settle payments on-chain

## Features

### Payment-Gated Routes (Axum)

> ⚠️ **Note**: The `x402-axum` middleware is currently being refactored. For now, call the facilitator HTTP endpoints directly from your Axum handlers.

Protect your Axum routes with on-chain payments:

```rust
use x402_axum::{X402Middleware, IntoPriceTag};
use x402::network::{Network, USDCDeployment};

let x402 = X402Middleware::try_from("https://facilitator.example.com/").unwrap();
let usdc = USDCDeployment::by_network(Network::BaseSepolia);

let app = Router::new().route(
    "/paid-content",
    get(handler).layer(
        x402.with_price_tag(usdc.amount("0.025").pay_to("0xYourAddress").unwrap())
    ),
);
```

See [crates/x402-axum](./crates/x402-axum/README.md) for details.

### Send Payments (Reqwest)

Use `x402-reqwest` to automatically handle payment flows in your HTTP client:

```rust
use x402_reqwest::{ReqwestWithPayments, X402Client};
use alloy_signer_local::PrivateKeySigner;

let signer: PrivateKeySigner = "0x...".parse()?;
let client = reqwest::Client::new()
    .with_payments(X402Client::new().register(signer))
    .build();

let res = client
    .get("https://example.com/protected")
    .send()
    .await?;
```

See [crates/x402-reqwest](./crates/x402-reqwest/README.md) for details.

## Installation

### Using Docker (Recommended)

Pull the pre-built image:

```shell
docker pull ghcr.io/awesome402/x402:latest
docker run -d \
  -v $(pwd)/config.json:/app/config.json \
  -p 8080:8080 \
  ghcr.io/awesome402/x402:latest --config /app/config.json
```

Or build from source:

```shell
docker build -t x402 .
docker run -d \
  -v $(pwd)/config.json:/app/config.json \
  -p 8080:8080 \
  x402 --config /app/config.json
```

> ⚠️ **Docker Note**: Set `"host": "0.0.0.0"` in your config for external access (not `127.0.0.1`)

With environment variables:

```shell
docker run -d \
  -v $(pwd)/config.json:/app/config.json \
  -e EVM_PRIVATE_KEY=0x... \
  -e SOLANA_PRIVATE_KEY=... \
  -p 8080:8080 \
  ghcr.io/awesome402/x402:latest --config /app/config.json
```

### From Source

Prerequisites: Rust 1.80+

```shell
# Build the facilitator
cargo build --bin x402

# Run with config
cargo run --bin x402 -- --config config.json
```

> 💡 **Tip**: Use `--bin x402` to skip middleware crates that are being refactored

## Configuration

The facilitator reads configuration from a JSON file. Create a `config.json`:

```json
{
  "port": 8080,
  "host": "0.0.0.0",
  "chains": {
    "eip155:84532": {
      "eip1559": true,
      "flashblocks": true,
      "signers": ["$EVM_PRIVATE_KEY"],
      "rpc": [
        {
          "http": "https://sepolia.base.org",
          "rate_limit": 50
        }
      ]
    },
    "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG": {
      "signer": "$SOLANA_PRIVATE_KEY",
      "rpc": "https://api.devnet.solana.com",
      "pubsub": "wss://api.devnet.solana.com"
    }
  },
  "schemes": [
    {
      "id": "v1-eip155-exact",
      "chains": "eip155:*"
    },
    {
      "id": "v2-eip155-exact",
      "chains": "eip155:*"
    },
    {
      "id": "v1-solana-exact",
      "chains": "solana:*"
    },
    {
      "id": "v2-solana-exact",
      "chains": "solana:*"
    }
  ]
}
```

### Configuration Structure

- **`port`**: HTTP server port (default: 8080)
- **`host`**: Bind address (`0.0.0.0` for Docker, `127.0.0.1` for local)
- **`chains`**: Map of CAIP-2 chain identifiers to chain configuration
  - **EVM chains** (`eip155:*`): Configure `signers`, `rpc` endpoints, `eip1559`, `flashblocks`
  - **Solana chains** (`solana:*`): Configure `signer`, `rpc`, `pubsub`
- **`schemes`**: Payment schemes to enable
  - `id`: Format `v{version}-{namespace}-{name}` (e.g., `v2-eip155-exact`)
  - `chains`: Pattern like `eip155:*` or specific chain like `eip155:84532`

### Environment Variables

Reference environment variables in your config using `$VAR` or `${VAR}`:

```json
{
  "signers": ["$EVM_PRIVATE_KEY"]
}
```

Then set them:

```shell
export EVM_PRIVATE_KEY=0xdeadbeef...
```

### Example Configs

- `config.test.json` - Local testing (host: 127.0.0.1)
- `config.docker.json` - Docker deployments (host: 0.0.0.0)
- `config.example.json` - Full example with all schemes

## Supported Networks

Configure any network in your `config.json`. Common chain identifiers:

| Network              | CAIP-2 Chain ID                                       | Notes                            |
| :------------------- | :---------------------------------------------------- | :------------------------------- |
| Base Sepolia Testnet | `eip155:84532`                                        | Testnet, Recommended for testing |
| Base Mainnet         | `eip155:8453`                                         | Mainnet                          |
| Ethereum Mainnet     | `eip155:1`                                            | Mainnet                          |
| Solana Mainnet       | `solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp`             | Mainnet                          |
| Solana Devnet        | `solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG` | Testnet, Recommended for testing |

## What is a Facilitator?

The facilitator is a service that handles:

- **Payment verification**: Confirms client payment payloads match declared requirements
- **Payment settlement**: Submits validated payments to the blockchain

This means servers don't need to:
- Connect directly to blockchains
- Implement complex cryptographic logic
- Handle blockchain-specific payment flows

The facilitator **never holds user funds**—it's a stateless verification and execution layer for signed payment payloads.

For more details, see the [x402 protocol documentation](https://docs.cdp.coinbase.com/x402/docs/overview).

## Development

Build and test locally:

```shell
# Build all crates
cargo build

# Run with test config
cargo run --bin x402 -- --config config.test.json

# Run tests
cargo test
```

## Related Resources

- [x402 Protocol Overview](https://docs.cdp.coinbase.com/x402/docs/overview)
- [Facilitator Documentation](https://docs.cdp.coinbase.com/x402/docs/facilitator)

## License

[MIT](LICENSE)
