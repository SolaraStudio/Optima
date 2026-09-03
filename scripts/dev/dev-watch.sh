#!/bin/bash
set -e

cd "$(dirname "$0")/../.."

echo "Starting Optima dev loop (watch + auto rebuild lib)"
cargo watch -x "build --lib"
