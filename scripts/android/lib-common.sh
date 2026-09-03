#!/bin/bash
# Shared helpers for the Optima Android build scripts.
# Sources functions only; safe to `source` from other scripts.

# Map a Rust target triple to the Android ABI directory name.
# Returns via echo; unknown targets map to the raw triple with '-' -> '_'.
rust_target_to_abi() {
    case "$1" in
        aarch64-linux-android)  echo "arm64-v8a" ;;
        armv7-linux-androideabi) echo "armeabi-v7a" ;;
        i686-linux-android)     echo "x86" ;;
        x86_64-linux-android)   echo "x86_64" ;;
        *)                      echo "${1//-/_}" ;;
    esac
}

# The standard set of Android ABIs Optima ships.
all_targets() {
    echo "aarch64-linux-android"
    echo "armv7-linux-androideabi"
    echo "i686-linux-android"
    echo "x86_64-linux-android"
}

# Copy a built .so for a given target into jniLibs.
#   copy_so <rust_target>
copy_so() {
    local target="$1"
    local abi
    abi="$(rust_target_to_abi "$target")"
    local jnilibs="android/app/src/main/jniLibs"
    local dest="$jnilibs/$abi"

    if [ ! -f "target/$target/release/liboptima.so" ]; then
        echo "ERROR: target/$target/release/liboptima.so not found" >&2
        return 1
    fi

    mkdir -p "$dest"
    cp "target/$target/release/liboptima.so" "$dest/"
    echo "Copied $target -> $dest/liboptima.so"
}

# Build (or just rebuild) a single target with the mandatory env flags.
build_target() {
    local target="$1"
    export RUST_FONTCONFIG_DLOPEN=1
    export PKG_CONFIG_ALLOW_CROSS=1
    export CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true
    echo "Building for $target"
    cargo build --target "$target" --release
}
