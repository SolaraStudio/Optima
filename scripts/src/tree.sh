#!/bin/bash
set -e

cd "$(dirname "$0")/../.."

echo "Generating src tree overview"
find src -name "*.rs" -printf '%h\n' | sort -u | sed 's|src/|src/|'

echo ""
echo "Public entry points:"
grep -rn "pub struct Engine\|pub fn new" src/api/engine/engine.rs | head
