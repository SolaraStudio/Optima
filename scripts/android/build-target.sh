#!/bin/bash
# Build Optima for a SINGLE, possibly unknown, Rust target triple, then copy the
# resulting .so into jniLibs. Pass the target as the first argument.
#
#   examples:
#     scripts/android/build-target.sh aarch64-linux-android
#     scripts/android/build-target.sh x86_64-linux-android
#     scripts/android/build-target.sh riscv64gc-unknown-linux-gnu   # unknown -> mapped dir
set -e

cd "$(dirname "$0")/../.."

# shellcheck source=lib-common.sh
source scripts/android/lib-common.sh

if [ $# -lt 1 ]; then
    echo "Usage: $0 <rust-target-triple>" >&2
    echo "Known ABIs map to: arm64-v8a, armeabi-v7a, x86, x86_64." >&2
    echo "Unknown targets map to a directory named after the triple." >&2
    exit 1
fi

TARGET="$1"
ABI="$(rust_target_to_abi "$TARGET")"

echo "==> Target : $TARGET"
echo "==> ABI dir: $ABI"

build_target "$TARGET"
copy_so "$TARGET"

echo "==> Done. liboptima.so for $TARGET in android/app/src/main/jniLibs/$ABI/"
