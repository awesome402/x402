build-all:
  cargo build
  cd crates/awesome402-axum && cargo build
  cd crates/awesome402-reqwest && cargo build
  cd examples/awesome402-axum-example && cargo build
  cd examples/awesome402-reqwest-example && cargo build

format-all:
  cargo fmt
  cd crates/awesome402-axum && cargo fmt
  cd crates/awesome402-reqwest && cargo fmt
  cd examples/awesome402-axum-example && cargo fmt
  cd examples/awesome402-reqwest-example && cargo fmt

fmt-all: format-all

clippy-all:
  cargo clippy
  cd crates/awesome402-axum && cargo clippy
  cd crates/awesome402-reqwest && cargo clippy
  cd examples/awesome402-axum-example && cargo clippy
  cd examples/awesome402-reqwest-example && cargo clippy

check-all:
  cargo check
  cd crates/awesome402-axum && cargo check
  cd crates/awesome402-reqwest && cargo check
  cd examples/awesome402-axum-example && cargo check
  cd examples/awesome402-reqwest-example && cargo check
