## Awesome402

> Rust implementation of the x402 protocol for blockchain payments over HTTP

This repository provides:

- `x402` (core crate):
  - Core protocol types, facilitator traits, and logic for on-chain payment verification and settlement
  - Facilitator binary - production-grade HTTP server to verify and settle payments
- [`x402-axum`](./crates/x402-axum) - Axum middleware for accepting payments
- [`x402-reqwest`](./crates/x402-reqwest) - Wrapper for reqwest for transparent payments
- [`x402-axum-example`](./examples/x402-axum-example) - Example of `x402-axum` usage
- [`x402-reqwest-example`](./examples/x402-reqwest-example) - Example of `x402-reqwest` usage

## About Awesome402

**Awesome402** is a Rust implementation of the [x402 protocol](https://docs.cdp.coinbase.com/x402/docs/overview), a proposed standard for making blockchain payments directly through HTTP using native `402 Payment Required` status code.

Servers declare payment requirements for specific routes. Clients send cryptographically signed payment payloads. Facilitators verify and settle payments on-chain.

## Getting Started

### Run facilitator

```shell
docker run -v $(pwd)/config.json:/app/config.json -p 8080:8080 ghcr.io/x402/x402-facilitator
```

Or build locally:
```shell
docker build -t x402 .
docker run -v $(pwd)/config.json:/app/config.json -p 8080:8080 x402
```

See the [Facilitator](#facilitator) section below for full usage details

### Protect Axum Routes

Use `x402-axum` to gate your routes behind on-chain payments:

```rust
let x402 = X402Middleware::try_from("https://x402.org/facilitator/").unwrap();
let usdc = USDCDeployment::by_network(Network::BaseSepolia);

let app = Router::new().route("/paid-content", get(handler).layer( 
        x402.with_price_tag(usdc.amount("0.025").pay_to("0xYourAddress").unwrap())
    ),
);
```

See [`x402-axum` crate docs](./crates/x402-axum/README.md).

### Send x402 payments

Use `x402-reqwest` to send payments:

```rust
let signer: PrivateKeySigner = "0x...".parse()?; // never hardcode real keys!

let client = reqwest::Client::new()
    .with_payments(signer)
    .prefer(USDCDeployment::by_network(Network::Base))
    .max(USDCDeployment::by_network(Network::Base).amount("1.00")?)
    .build();

let res = client
    .get("https://example.com/protected")
    .send()
    .await?;
```

See [`x402-reqwest` crate docs](./crates/x402-reqwest/README.md).
```

## Facilitator

The `x402` crate (this repo) provides a runnable facilitator binary for **Awesome402**. The _Facilitator_ role simplifies adoption of Awesome402 by handling:
- **Payment verification**: Confirming that client-submitted payment payloads match the declared requirements.
- **Payment settlement**: Submitting validated payments to the blockchain and monitoring their confirmation.

By using a Facilitator, servers (sellers) do not need to:
- Connect directly to a blockchain.
- Implement complex cryptographic or blockchain-specific payment logic.

Instead, they can rely on the Facilitator to perform verification and settlement, reducing operational overhead and accelerating Awesome402 adoption.
The Facilitator **never holds user funds**. It acts solely as a stateless verification and execution layer for signed payment payloads.

For a detailed overview of the x402 payment flow and Facilitator role, see the [x402 protocol documentation](https://docs.cdp.coinbase.com/x402/docs/overview).

### Usage

#### 1. Create a configuration file

Create a `config.json` file with your chain and scheme configuration:

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
      "slug": "v1:eip155:exact",
      "chains": "eip155:*"
    },
    {
      "slug": "v2:eip155:exact",
      "chains": "eip155:*"
    },
    {
      "slug": "v1:solana:exact",
      "chains": "solana:*"
    },
    {
      "slug": "v2:solana:exact",
      "chains": "solana:*"
    }
  ]
}
```

**Configuration structure:**

- **`chains`**: A map of CAIP-2 chain identifiers to chain-specific configuration
  - EVM chains (`eip155:*`): Configure `signers` (array of private keys), `rpc` endpoints, and optional `eip1559`/`flashblocks` flags
  - Solana chains (`solana:*`): Configure `signer` (single private key), `rpc` endpoint, and optional `pubsub` endpoint
- **`schemes`**: List of payment schemes to enable
  - `slug`: Scheme identifier in format `v{version}:{namespace}:{name}` (e.g., `v2:eip155:exact`)
  - `chains`: Chain pattern to match (e.g., `eip155:*` for all EVM chains, `eip155:84532` for specific chain)

**Environment variable references:**

Private keys can reference environment variables using `$VAR` or `${VAR}` syntax:
```json
"signers": ["$EVM_PRIVATE_KEY"]
```

Then set the environment variable:
```shell
export EVM_PRIVATE_KEY=0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
```

#### 2. Build and Run with Docker

Prebuilt Docker images are available at [GitHub Container Registry](https://github.com/orgs/x402/packages/container/package/x402-facilitator): `ghcr.io/x402/x402-facilitator`

Run the container:
```shell
docker run -v $(pwd)/config.json:/app/config.json -p 8080:8080 ghcr.io/x402/x402-facilitator
```

Or build a Docker image locally:
```shell
docker build -t x402 .
docker run -v $(pwd)/config.json:/app/config.json -p 8080:8080 x402
```

You can also pass environment variables for private keys:
```shell
docker run -v $(pwd)/config.json:/app/config.json \
  -e EVM_PRIVATE_KEY=0x... \
  -e SOLANA_PRIVATE_KEY=... \
  -p 8080:8080 ghcr.io/x402/x402-facilitator
```

The container:
* Exposes port `8080` (or a port you configure in `config.json`).
* Starts on http://localhost:8080 by default.
* Requires minimal runtime dependencies (based on `debian:bullseye-slim`).

#### 3. Point your application to your Facilitator

If you are building an Awesome402-powered application, update the Facilitator URL to point to your self-hosted instance.

> ℹ️ **Tip:** For production deployments, ensure your Facilitator is reachable via HTTPS and protect it against public abuse.

<details>


```typescript
import { Hono } from "hono";
import { serve } from "@hono/node-server";
import { paymentMiddleware } from "x402-hono";

const app = new Hono();

// Configure the payment middleware
app.use(paymentMiddleware(
  "0xYourAddress", // Your receiving wallet address
  {
    "/protected-route": {
      price: "$0.10",
      network: "base-sepolia",
      config: {
        description: "Access to premium content",
      }
    }
  },
  {
    url: "http://your-validator.url/", // 👈 Your self-hosted Facilitator
  }
));

// Implement your protected route
app.get("/protected-route", (c) => {
  return c.json({ message: "This content is behind a paywall" });
});

serve({
  fetch: app.fetch,
  port: 3000
});
```

</details>

<details>
<summary>If you use `x402-axum`</summary>

```rust
let x402 = X402Middleware::try_from("http://your-validator.url/").unwrap();  // 👈 Your self-hosted Facilitator
let usdc = USDCDeployment::by_network(Network::BaseSepolia);

let app = Router::new().route("/paid-content", get(handler).layer( 
        x402.with_price_tag(usdc.amount("0.025").pay_to("0xYourAddress").unwrap())
    ),
);
```

</details>

### Configuration

The service reads configuration from a JSON file (`config.json` by default) or via CLI argument `--config <path>`.

#### Configuration File Structure

```json
{
  "port": 8080,
  "host": "0.0.0.0",
  "chains": { ... },
  "schemes": [ ... ]
}
```

### Supported Networks

The Facilitator supports any network you configure in `config.json`. Common chain identifiers:

| Network                   | CAIP-2 Chain ID                              | Notes                            |
|:--------------------------|:---------------------------------------------|:---------------------------------|
| Base Sepolia Testnet      | `eip155:84532`                               | Testnet, Recommended for testing |
| Base Mainnet              | `eip155:8453`                                | Mainnet                          |
| Ethereum Mainnet          | `eip155:1`                                   | Mainnet                          |
| Solana Mainnet            | `solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp`    | Mainnet                          |
| Solana Devnet             | `solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG` | Testnet, Recommended for testing |

Networks are enabled by adding them to the `chains` section in your `config.json`.

> ℹ️ **Tip:** For initial development and testing, you can start with Base Sepolia (`eip155:84532`) or Solana Devnet only.

### Development

Prerequisites:
- Rust 1.80+
- `cargo` and a working toolchain

Build locally:
```shell
cargo build
```

Run with a config file:
```shell
cargo run -- --config config.json
```

Or place `config.json` in the current directory (it will be auto-detected):
```shell
cargo run
```

## Related Resources

* [x402 Protocol Overview by Coinbase](https://docs.cdp.coinbase.com/x402/docs/overview)
* [Facilitator Documentation by Coinbase](https://docs.cdp.coinbase.com/x402/docs/facilitator)

## Contributions and feedback welcome!
Feel free to open issues or pull requests to improve Awesome402 in the Rust ecosystem.

## License

[MIT](LICENSE)
