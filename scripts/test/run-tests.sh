#!/bin/bash
set -e

cd "$(dirname "$0")/../.."

echo "Running Optima test suite"
cargo test "$@"

echo ""
echo "Hint: run ignored benchmarks with: cargo test -- --ignored"
