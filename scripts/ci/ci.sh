#!/bin/bash
set -e

# CI entrypoint. Runs the checks that must pass before merging.
cd "$(dirname "$0")/../.."

echo "==> cargo fmt --check"
cargo fmt --all -- --check

echo "==> cargo check"
cargo check --all-targets

echo "==> cargo build --release (lib)"
cargo build --release --lib

echo "==> CI checks passed"
