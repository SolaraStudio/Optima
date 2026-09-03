#!/bin/bash
set -e

cd "$(dirname "$0")/../.."

echo "Disk usage of build artifacts (target/)"
du -sh target 2>/dev/null || echo "no target dir"

echo ""
echo "Largest crates in registry cache:"
du -sh ~/.cargo/registry/cache/*/* 2>/dev/null | sort -rh | head -10 || true
