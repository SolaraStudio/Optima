# Contributing to Optima

Welcome! Optima is an open-source WebView engine written in Rust, compiled to a C dynamic library (`liboptima.so`) for Android, developed by SolaraStudio. This guide covers how to set up your development environment, build, test, and contribute.

---

## Prerequisites

- **Rust** (stable, edition 2024) via [rustup](https://rustup.rs/)
- **Android NDK** (for cross-compilation to Android targets)
- **Android SDK** with API level 35 (compileSdk) and min SDK 24
- **JDK 17** (required by the Gradle build)
- **Git**

### Required Rust Targets

Install all Android targets:

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

### NDK Setup

The `.cargo/config.toml` expects the following NDK linkers on your `PATH`:

| Target | Linker |
|--------|--------|
| `aarch64-linux-android` | `aarch64-linux-android-clang` |
| `armv7-linux-androideabi` | `armv7a-linux-androideabi21-clang` |
| `i686-linux-android` | `i686-linux-android21-clang` |
| `x86_64-linux-android` | `x86_64-linux-android21-clang` |

These are provided by the Android NDK toolchain. Make sure `ANDROID_NDK_HOME` is set and the toolchain `bin/` is on your `PATH`.

---

## Building

### Cross-compile for Android (single target)

```bash
cargo build --target aarch64-linux-android --release
```

### Build for all Android architectures

Use the provided script:

```bash
./scripts/android/build-all.sh
```

This compiles for all four ABIs, copies the resulting `.so` files into `android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86,x86_64}/`, and runs `./gradlew assembleRelease` to produce the AAR.

### Build via Gradle

From the `android/` directory:

```bash
# Build Rust libraries then assemble the Android library
./gradlew buildRustAll assembleRelease

# Or just copy pre-built libs and assemble
./gradlew copyRustLibs assembleRelease
```

### Environment Variables

| Variable | Purpose |
|----------|---------|
| `RUST_FONTCONFIG_DLOPEN` | Set to `1` to avoid pkg-config cross-compilation issues |
| `PKG_CONFIG_ALLOW_CROSS` | Set to `1` for cross-compilation |
| `OPTIMA_VERSION` | Override version string in the AAR (default: `0.150.10-SNAPSHOT`) |
| `VERSION_SUFFIX` | Version suffix (default: `SNAPSHOT`) |

---

## Testing

### Unit Tests

```bash
cargo test
```

Tests are organized under `tests/`:

- `tests/unit/` — unit tests for individual modules
- `tests/integration/` — integration tests across modules
- `tests/e2e/` — end-to-end tests
- `tests/benchmarks/` — performance benchmarks
- `tests/resources/` — test fixtures (HTML, CSS, images)

### Benchmarks

Run benchmarks from the `benches/` directory:

```bash
cargo bench
```

Benchmark suites:

- `benches/parse/` — HTML/CSS parsing benchmarks
- `benches/layout/` — layout calculation benchmarks
- `benches/render/` — rendering pipeline benchmarks
- `benches/full/` — full pipeline benchmarks

### Running on Android

1. Connect an Android device or start an emulator (API 24+).
2. Build the library: `./scripts/android/build-all.sh`
3. Install the AAR or run the example app via Android Studio.

---

## Project Structure

```
Optima/
├── src/                    Rust source code
│   ├── api/                Public and internal API
│   ├── android/            Android platform integration
│   ├── config/             Configuration and settings
│   ├── css/                CSS parser and cascade
│   ├── devtools/           Chrome DevTools Protocol
│   ├── dom/                DOM tree implementation
│   ├── events/             Input event handling
│   ├── jni/                JNI bridge layer
│   ├── layout/             CSS layout engine
│   ├── media/              Audio/video support
│   ├── net/                HTTP client and networking
│   ├── platform/           Platform abstractions
│   ├── render/             GPU rendering (Vello/wgpu)
│   ├── resource/           Resource management
│   ├── security/           Security policies
│   ├── text/               Font shaping and text layout
│   └── utils/              Shared utilities
├── android/                Android library project (AAR)
├── tests/                  Test suites
├── benches/                Benchmarks
├── examples/               Example code
├── scripts/                Build and CI scripts
├── docs/                   Documentation
├── resources/              Runtime resources
├── Cargo.toml              Rust package manifest
├── .cargo/config.toml      Cargo build configuration
└── version.txt             Current version string
```

---

## Code Style

- Follow standard Rust formatting (`cargo fmt`).
- Use `clippy` for linting: `cargo clippy -- -D warnings`.
- Keep functions focused and well-named.
- Prefer `anyhow`/`thiserror` for error handling.
- Use `Rc<RefCell<T>>` for shared mutable state in the DOM (single-threaded).
- Use `Arc<Mutex<T>>` for shared state in multi-threaded contexts (DevTools, networking).

---

## Making Changes

1. **Fork** the repository on GitHub.
2. **Create a branch** for your feature or fix.
3. **Make your changes** following the code style guidelines.
4. **Add tests** for new functionality.
5. **Run the full test suite** (`cargo test` and `cargo clippy`).
6. **Submit a pull request** against the `main` branch.

### Commit Messages

Use concise, descriptive commit messages. Prefix with the module when applicable:

```
css: fix specificity calculation for attribute selectors
dom: add querySelectorAll support
render: optimize Vello batch rendering
```

### What to Contribute

We welcome contributions in these areas:

- **Bug fixes** — especially in layout, CSS parsing, and rendering
- **New CSS features** — additional properties, selectors, units
- **DOM improvements** — missing DOM APIs, mutation observers
- **Performance** — profiling results and optimizations
- **Tests** — coverage for edge cases
- **Documentation** — API docs, examples, guides
- **Platform support** — desktop builds, additional Android features

---

## Release Process

1. Update `version.txt` with the new version.
2. Update `Cargo.toml` version field.
3. Update `src/api/version/version.rs` constants if the major/minor/patch changed.
4. Create a release commit and tag.
5. CI builds the AAR and publishes to GitHub Packages.

### Publishing the AAR

```bash
cd android
./gradlew publishReleasePublicationToGitHubPackagesRepository
```

Requires `GITHUB_ACTOR` and `GITHUB_TOKEN` environment variables.

---

## Questions?

Open an issue on the [GitHub repository](https://github.com/SolaraStudio/Optima) or reach out to the maintainers.
