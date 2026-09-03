#!/bin/bash
set -e

cd "$(dirname "$0")/../.."

VERSION="${1:-$(cat version.txt)}"

echo "Bumping version to $VERSION"
echo "$VERSION" > version.txt
cargo set-version "$VERSION"

echo "Creating tag v$VERSION"
git add version.txt Cargo.toml
git commit -m "chore(release): bump version to $VERSION"
git tag "v$VERSION"

echo "Release tag v$VERSION ready. Push with:"
echo "  git push origin main --tags"
