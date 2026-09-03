#!/bin/bash
set -e

cd "$(dirname "$0")/../.."

echo "Checking code with cargo check (fast, no full codegen)"
cargo check

echo "Building cdylib library"
cargo build --lib
