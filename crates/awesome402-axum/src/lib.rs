//! Axum middleware and helpers for enforcing [awesome402](https://www.awesome402.org) payments.
//!
//! This crate provides an [`Awesome402Middleware`] Axum layer for protecting routes with payment enforcement,
//! as well as a [`FacilitatorClient`] for communicating with remote awesome402 facilitators.
//!
//! ## Quickstart
//!
//! ```rust,no_run
//! use axum::{Router, routing::get, Json};
//! use axum::response::IntoResponse;
//! use http::StatusCode;
//! use serde_json::json;
//! use awesome402_axum::{Awesome402Middleware, IntoPriceTag};
//! use awesome402::network::{Network, USDCDeployment};
//!
//! let awesome402 = Awesome402Middleware::try_from("https://facilitator.example.com/").unwrap();
//! // You can construct `TokenAsset` manually. Here we use known USDC on Base Sepolia
//! let usdc = USDCDeployment::by_network(Network::BaseSepolia).pay_to("0xADDRESS");
//!
//! let app: Router = Router::new().route(
//!     "/paywall",
//!     get(my_handler).layer(
//!         awesome402.with_description("Premium Content")
//!             .with_price_tag(usdc.amount(0.025).unwrap()),
//!     ),
//! );
//!
//! async fn my_handler() -> impl IntoResponse {
//!     (StatusCode::OK, Json(json!({ "hello": "world" })))
//! }
//! ```
//! See [`Awesome402Middleware`] for full configuration options.
//! For low-level interaction with the facilitator, see [`facilitator_client::FacilitatorClient`].
//!
//! ## Defining Prices
//!
//! To define price tags for your protected routes, see the [`price`] module.
//! It provides builder-style helpers like [`IntoPriceTag`] and types like [`PriceTag`]
//! for working with tokens, networks, and payment amounts.

pub mod facilitator_client;
pub mod layer;
pub mod price;

pub use layer::Awesome402Middleware;
pub use price::*;
