# Cargo build outputs
# This directory documents where cargo emits build artifacts for Optima.
#
#   target/                       <- default cargo output (git-ignored)
#   target/aarch64-linux-android/release/liboptima.so
#   target/armv7-linux-androideabi/release/liboptima.so
#   target/i686-linux-android/release/liboptima.so
#   target/x86_64-linux-android/release/liboptima.so
#
# The Android library (`liboptima.so`) is copied into
# `android/app/src/main/jniLibs/<abi>/` by scripts/android/build-all.sh.
