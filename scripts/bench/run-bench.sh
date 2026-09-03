#!/bin/bash
set -e

cd "$(dirname "$0")/../.."

echo "Running Optima benchmarks (criterion)"
cargo bench "$@"
