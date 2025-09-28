# awesome402-reqwest-example

<div align="center">
<table><tr><td>
🔧 <strong>Protocol v2 Update Coming</strong> — This example is being updated for awesome402 protocol v2. Stay tuned! For v1 examples, see the <code>protocol-x402-v1</code> branch.
</td></tr></table>
</div>

An example client that uses [`awesome402-reqwest`](https://crates.io/crates/awesome402-reqwest) to pay for HTTP requests using the awesome402 protocol.

This small demo shows how to configure a reqwest client to:
- Interact with x402-protected endpoints
- Apply token preferences and per-token payment caps

## What it does

On startup, this example:
- Reads your private key from env variable `PRIVATE_KEY`
- Builds a `reqwest` client using [`reqwest-middleware`](https://crates.io/crates/reqwest-middleware) and  [`awesome402-reqwest`](https://crates.io/crates/awesome402-reqwest)
- Sends a request to a protected endpoint

If the server responds with a 402 Payment Required, the client:
-	Parses the server’s requirements
-	Selects a supported token (e.g. USDC on Base Sepolia)
-	Signs a `TransferWithAuthorization`
-	Retries with the signed payment attached

The best part? **You don’t have to worry about any of this.**
Just set your token preferences and treat it like any other `reqwest` HTTP client.

# Prerequisites
- A private key with testnet funds (Base Sepolia USDC)
-	Rust + Cargo
-	`PRIVATE_KEY` set in your environment (or in `.env` file, see `.env.example`)

## Running the Example
```shell
# 1. Clone the repo and cd into this example folder
# 2. Create `.env` file
cp .env.example .env
# 3. Set your PRIVATE_KEY inside `.env`
# 4. Run
cargo run
```
You should see the request succeed and print the server’s response.

## Behind the scenes

This example uses:
-	[`awesome402-reqwest`](https://crates.io/crates/awesome402-reqwest) to intercept 402s and attach signed payments
-	[`alloy`](https://alloy.rs) for signing
-	[`dotenvy`](https://crates.io/crates/dotenvy) to load the `.env` file
-	[`awesome402`](https://crates.io/crates/awesome402) for token/network definitions and amount conversion

## Related
- [`awesome402`](https://crates.io/crates/awesome402): Common types and facilitator logic
- [`awesome402-axum`](https://crates.io/crates/awesome402-axum): Axum server-side middleware to accept payments
- [`awesome402-reqwest`](https://crates.io/crates/awesome402-reqwest): The crate this example is showcasing
