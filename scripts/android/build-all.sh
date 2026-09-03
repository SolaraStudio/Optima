#!/bin/bash
set -e

# Force dlopen for fontconfig (avoids pkg-config cross-compilation issues)
export RUST_FONTCONFIG_DLOPEN=1
export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true

for target in aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android; do
    echo "Building for $target"
    cargo build --target "$target" --release
done

JNILIBS=android/app/src/main/jniLibs
mkdir -p "$JNILIBS/arm64-v8a"
mkdir -p "$JNILIBS/armeabi-v7a"
mkdir -p "$JNILIBS/x86"
mkdir -p "$JNILIBS/x86_64"

cp target/aarch64-linux-android/release/*.so "$JNILIBS/arm64-v8a/"
cp target/armv7-linux-androideabi/release/*.so "$JNILIBS/armeabi-v7a/"
cp target/i686-linux-android/release/*.so "$JNILIBS/x86/"
cp target/x86_64-linux-android/release/*.so "$JNILIBS/x86_64/"

cd android
./gradlew assembleRelease
