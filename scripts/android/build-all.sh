#!/bin/bash
set -e

for target in aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android; do
    echo "Building for $target"
    cargo build --target "$target" --release
done

mkdir -p android/src/main/jniLibs/arm64-v8a
mkdir -p android/src/main/jniLibs/armeabi-v7a
mkdir -p android/src/main/jniLibs/x86
mkdir -p android/src/main/jniLibs/x86_64

cp target/aarch64-linux-android/release/*.so android/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/*.so android/src/main/jniLibs/armeabi-v7a/
cp target/i686-linux-android/release/*.so android/src/main/jniLibs/x86/
cp target/x86_64-linux-android/release/*.so android/src/main/jniLibs/x86_64/

cd android
./gradlew assembleRelease
