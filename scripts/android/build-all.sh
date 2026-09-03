#!/bin/bash
# Build Optima for ALL supported Android ABIs, copy the .so files into jniLibs,
# and assemble the AAR. For a single (possibly unknown) target use
# scripts/android/build-target.sh instead.
set -e

cd "$(dirname "$0")/../.."

# shellcheck source=lib-common.sh
source scripts/android/lib-common.sh

for target in $(all_targets); do
    build_target "$target"
done

for target in $(all_targets); do
    copy_so "$target"
done

echo "==> Assembling AAR"
cd android
./gradlew assembleRelease
