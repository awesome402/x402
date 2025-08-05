//! Rust implementation of the awesome402 protocol.
//!
//! This crate provides the core data structures and logic for working with the awesome402 protocol,
//! including a reference facilitator implementation for on-chain verification and settlement.
//!
//! It is designed for reuse across all awesome402 roles:
//! - _Facilitator_: a server that verifies and settles awesome402 payments (see [`facilitator`] and [`facilitator_local`])
//! - _Seller_: a payment-gated service that consumes shared types from [`types`]
//! - _Buyer_: a client that constructs and submits awesome402-compliant payments
//!
//! Modules:
//! - [`facilitator`] — defines the [`facilitator::Facilitator`] trait used to validate and settle awesome402 payments.
//! - [`facilitator_local`] — a concrete implementation of [`facilitator::Facilitator`].
//! - [`network`] — enumerates supported Ethereum-compatible networks and known token deployments.
//! - [`provider_cache`] — dynamic initialization and caching of Ethereum JSON-RPC providers.
//! - [`telemetry`] — OpenTelemetry instrumentation setup for tracing and observability.
//! - [`types`] — all shared awesome402 protocol structures and payload formats.

pub mod chain;
pub mod config;
pub mod facilitator;
pub mod facilitator_local;
pub mod handlers;
pub mod networks;
pub mod proto;
pub mod scheme;
pub mod timestamp;
pub mod util;

// Hidden re-exports just for macro expansion.
#[doc(hidden)]
pub mod __reexports {
    pub use alloy_primitives;
    pub use solana_pubkey;
}
