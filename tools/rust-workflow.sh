#!/usr/bin/env bash
#
# Locally run the significant commands done in the
# `.github/workflows/rust.yml` file.
#
# Developers must manually update this script when `rust.yml` changes.
#

set -euo pipefail

cd "$(dirname -- "${0}")/.."

set -x

cargo msrv verify
cargo build --lib --verbose
cargo build --lib --verbose --release
cargo test --verbose --all-features --future-incompat-report
cargo check --all-targets --release
cargo clippy --no-deps --all-targets --all-features
cargo doc --locked --release --frozen --no-deps
cargo publish --dry-run --allow-dirty
cargo fmt --verbose --check
cargo doc --locked --release --frozen --no-deps -v
cargo clippy --no-deps --verbose --all-targets --all-features -- -D warnings
