# Frequently Asked Questions

---

## General

### What is Optima?

Optima is a lightweight WebView engine written in Rust. It parses HTML/CSS, performs layout, renders to GPU via Vello/wgpu, and is primarily designed for Android. It is compiled to a C dynamic library (`liboptima.so`) and exposed to Android apps through a JNI bridge.

### Who develops Optima?

Optima is developed by SolaraStudio.

### What is the current version?

The current development version is **0.150.10-dev** (as recorded in `version.txt` and `Cargo.toml`).

### What platforms are supported?

**Android** is the primary target (API 24+). Desktop support (Linux, macOS, Windows) exists in the codebase under `src/platform/desktop/` but is secondary. The `Compat` struct in `src/api/compat/` detects the platform at compile time.

---

## Building

### How do I build for Android?

```bash
./scripts/android/build-all.sh
```

This cross-compiles for all four Android ABIs (arm64-v8a, armeabi-v7a, x86, x86_64) and copies the `.so` files into the Android project's `jniLibs` directory.

### What NDK version do I need?

The build expects Android NDK clang toolchains on your `PATH`. The linkers are configured in `.cargo/config.toml`. Use NDK r25+ for compatibility.

### Why do I get "fontconfig" errors during cross-compilation?

Set the environment variable `RUST_FONTCONFIG_DLOPEN=1` before building. This is already set in `build-all.sh` and avoids pkg-config cross-compilation issues.

### How do I build just one architecture?

```bash
cargo build --target aarch64-linux-android --release
```

---

## Integration

### How do I add Optima to my Android app?

1. Build the AAR: `cd android && ./gradlew assembleRelease`
2. Add the AAR to your project's dependencies.
3. Load the native library: `System.loadLibrary("optima")`
4. Create an engine: `val engine = OptimaEngine()`
5. Load content: `engine.loadHtml("<h1>Hello</h1>")`
6. Render: `engine.render()`
7. Destroy when done: `engine.destroy()`

### What does `OptimaEngine.destroy()` do?

It calls `nativeDestroy` which drops the Rust `Engine` object and frees all associated memory. You **must** call `destroy()` when you're done with the engine to avoid memory leaks.

### Can I use Optima from Kotlin/Java only?

The primary interface is the Kotlin `OptimaEngine` class. However, you can also call the JNI native functions directly from Java if you prefer. The five native functions are `nativeInit`, `nativeLoadHtml`, `nativeLoadCss`, `nativeRender`, and `nativeDestroy`, all under `org.optima.OptimaEngine`.

---

## HTML/CSS Support

### What HTML features are supported?

Optima implements a DOM tree with support for standard HTML elements, attributes, text nodes, comments, and doctypes. The `Document` API supports `getElementById`, `getElementsByTagName`, `getElementsByClassName`, `querySelector`, and `querySelectorAll`.

### What CSS features are supported?

The CSS engine supports:

- Selectors (class, ID, tag, attribute, compound)
- Cascade and specificity
- Property inheritance
- Box model (margin, border, padding, content)
- Flexbox and CSS Grid (via Taffy)
- Table layout
- Inline and block formatting contexts
- CSS animations and keyframes
- CSS transitions
- CSS transforms
- Media queries
- `@font-face` rules
- Color values and units
- Gradients

### Does Optima support JavaScript?

JavaScript is listed as an `EngineConfig` option (`javascript_enabled`) and is enabled by default, but the JavaScript engine integration is not yet complete. The flag exists for future use.

### Does Optima support WebGL/WebRTC?

Both are configurable via `EngineConfig` (`enable_webgl`, `enable_webrtc`) and `FeatureFlags` (`enable_webgl`), but are disabled by default. WebGL support is planned for future development.

---

## Rendering

### How does rendering work?

`Engine::render()` returns a `Vec<u8>` of raw RGBA pixel data. The rendering pipeline uses Vello (a 2D vector graphics renderer) backed by wgpu for GPU acceleration. The `RenderBackend` in `src/render/backend/` manages the wgpu device, queue, and surface.

### What is the default viewport size?

800×600 pixels. Change it with `engine.set_viewport(width, height)`.

### How do I get the rendered pixels?

```rust
let pixels: Vec<u8> = engine.render();
// pixels is RGBA, size = width * height * 4
```

---

## Networking

### How does networking work?

Optima uses `reqwest` for HTTP requests (with `rustls-tls` for TLS). The networking stack includes:

- HTTP and HTTPS support
- Response caching
- Cookie management
- DNS resolution
- Redirect handling
- Proxy support
- Retry logic
- Timeout configuration

### What fetch methods are supported?

`Fetch` supports GET, POST, PUT, DELETE, PATCH, HEAD, and OPTIONS methods. Use `Fetch::blocking_get(url)` for simple requests or `Fetch::execute(&request)` for full control.

---

## Fonts

### How are fonts loaded on Android?

`SystemFontHelper.kt` scans standard Android font directories:

- `/system/fonts`
- `/system/fonts/googlefonts`
- `/product/fonts`
- `/vendor/fonts`

It finds `.ttf` and `.otf` files and maps font family names to file paths using `Typeface.createFromFile()`.

### Can I use custom fonts?

Use `@font-face` rules in CSS. The font face module (`src/css/font_face/`) parses `@font-face` declarations, and the networking stack can download fonts from URLs.

---

## DevTools

### Does Optima support Chrome DevTools?

Optima implements a subset of the Chrome DevTools Protocol (`src/devtools/`). The `DevToolsServer` manages client connections, and the `DevToolsBackend` handles protocol commands. This allows debugging tools to connect to the engine.

---

## Debugging

### How do I enable debug features?

Configure `DebugConfig` through `Settings`. The `config/debug/` module contains debug-specific configuration options.

### How do I log from Rust?

Optima uses the `log` crate with `android_logger` for Android. Configure logging level through the `DebugConfig` in `Settings`.
