# Optima API Reference

Version: 0.150.10-dev

This document covers the public Rust API of the Optima WebView engine, the JNI bridge, and the Android Kotlin wrapper.

---

## Core Modules

| Module | Path | Description |
|--------|------|-------------|
| `api::engine` | `src/api/engine/` | `Engine` struct — the main entry point |
| `api::config` | `src/api/config/` | `EngineConfig` — configuration builder |
| `api::navigation` | `src/api/navigation/` | `NavigationState` — URL history and navigation |
| `api::events` | `src/api/events/` | `EventDispatcher` — event listener management |
| `api::version` | `src/api/version/` | Version constants and `Version` struct |
| `api::compat` | `src/api/compat/` | `Compat` — platform detection and compatibility |
| `api::public` | `src/api/public/` | `PublicApi` — high-level initialization API |
| `api::private` | `src/api/private/` | `PrivateApi` — internal engine wrapper |
| `api::internal` | `src/api/internal/` | `InternalEngine` — low-level engine internals |

---

## Engine

```rust
use optima::api::config::EngineConfig;
use optima::api::engine::Engine;

let config = EngineConfig::new();
let mut engine = Engine::new(config);
```

### `Engine::new(config: EngineConfig) -> Self`

Creates a new engine instance with the given configuration. The engine starts with an empty document, an 800×600 viewport, and no navigation history.

### `Engine::load_html(&mut self, html: &str, base_url: &str) -> Result<(), String>`

Parses the provided HTML string and loads it as the current document. The `base_url` parameter sets the base URL for resolving relative resource references and is used for navigation history.

### `Engine::load_url(&mut self, url: &str) -> Result<(), String>`

Loads a URL by updating the navigation state. The URL is recorded in the navigation history and becomes the current page.

### `Engine::inject_css(&mut self, css: &str) -> Result<(), String>`

Injects a CSS stylesheet into the current document. The stylesheet is parsed and added to the engine's internal stylesheets list. Call `recalculate_styles()` afterward to apply the injected styles.

### `Engine::recalculate_styles(&mut self)`

Re-runs the CSS cascade and style resolution for the entire document tree. Call this after injecting styles or mutating the DOM.

### `Engine::set_viewport(&mut self, width: u32, height: u32)`

Sets the viewport dimensions in CSS pixels. The default is 800×600.

### `Engine::render(&self) -> Vec<u8>`

Renders the current document to an RGBA pixel buffer. Returns a `Vec<u8>` containing raw pixel data. The buffer size is `width × height × 4` bytes.

### `Engine::document(&self) -> Option<Rc<RefCell<Document>>>`

Returns a reference-counted pointer to the current DOM `Document`, allowing inspection and mutation of the document tree.

### `Engine::navigation(&self) -> &NavigationState`

Returns a reference to the current navigation state (history, loading status, current URL).

### `Engine::config(&self) -> &EngineConfig`

Returns a reference to the engine's configuration.

### `Engine::dispatch_event(&mut self, event_type: &str)`

Dispatches an event by type name to all registered listeners.

### `Engine::reload(&mut self) -> Result<(), String>`

Reloads the current URL. Returns an error if no URL has been loaded.

### `Engine::go_back(&mut self) -> Result<(), String>`

Navigates to the previous entry in history. Returns an error if there is no back history.

### `Engine::go_forward(&mut self) -> Result<(), String>`

Navigates to the next entry in history. Returns an error if there is no forward history.

### `Engine::stop(&mut self)`

Cancels the current loading operation.

### `Engine::start(&mut self)`

Marks the engine as ready for rendering.

### `Engine::tick(&mut self)`

Advances the engine's internal state (animations, timers, pending tasks).

---

## EngineConfig

```rust
use optima::api::config::EngineConfig;

let config = EngineConfig::new()
    .with_user_agent("MyApp/2.0")
    .with_javascript(true)
    .with_images(true)
    .with_css(true)
    .with_font_size(18)
    .with_timeout(std::time::Duration::from_secs(60));
```

### `EngineConfig::new() -> Self`

Creates a configuration with sensible defaults:

| Field | Default |
|-------|---------|
| `user_agent` | `"Optima/1.0"` |
| `javascript_enabled` | `true` |
| `images_enabled` | `true` |
| `css_enabled` | `true` |
| `default_font_size` | `16` |
| `default_font_family` | `"sans-serif"` |
| `enable_webgl` | `false` |
| `enable_webrtc` | `false` |
| `cache_enabled` | `true` |
| `cache_size` | `50 MB` |
| `timeout` | `30 seconds` |
| `max_connections` | `6` |
| `allow_file_protocol` | `false` |

### Builder Methods

- `with_user_agent(ua: &str) -> Self`
- `with_javascript(enabled: bool) -> Self`
- `with_images(enabled: bool) -> Self`
- `with_css(enabled: bool) -> Self`
- `with_font_size(size: u32) -> Self`
- `with_timeout(duration: Duration) -> Self`

### `EngineConfig::from_settings(settings: &Settings) -> Self`

Constructs an `EngineConfig` from a `Settings` object, mapping feature flags and preferences to config fields.

---

## NavigationState

Maintains a `VecDeque<String>` URL history with a cursor index.

- `update_url(url: &str)` — adds or updates the current entry
- `current_url() -> Option<&str>` — returns the current URL
- `back() -> Option<&str>` — moves the cursor backward
- `forward() -> Option<&str>` — moves the cursor forward
- `set_loading(loading: bool)` / `is_loading() -> bool`
- `can_go_back() -> bool` / `can_go_forward() -> bool`
- `history_count() -> usize`

---

## EventDispatcher

```rust
use optima::api::events::EventDispatcher;

let mut dispatcher = EventDispatcher::new();
dispatcher.add_listener("click", |event_type| {
    println!("Got event: {}", event_type);
});
dispatcher.dispatch("click");
```

- `add_listener(event_type, callback)` — registers a callback
- `dispatch(event_type)` — fires all listeners for the type
- `remove_listeners(event_type)` — removes listeners for a type
- `clear()` — removes all listeners

---

## Version

```rust
use optima::api::version::{Version, VERSION, MAJOR, MINOR, PATCH};

let v = Version::current(); // 0.150.10
assert!(v.is_compatible(&Version::new(0, 150, 0)));
```

Constants: `ENGINE_NAME = "Optima"`, `VERSION = "0.150.10-dev"`, `MAJOR = 0`, `MINOR = 150`, `PATCH = 10`.

---

## Integration

The `Integration` struct combines an `Engine` with `Settings` for higher-level lifecycle management:

```rust
use optima::integration::integration::Integration;
use optima::config::settings::Settings;

let mut integration = Integration::new(Settings::default());
integration.initialize().unwrap();
integration.load_html("<h1>Hello</h1>", "")?;
integration.tick();
integration.shutdown();
```

---

## JNI Bridge

The JNI bridge exposes five native functions under the `org.optima.OptimaEngine` Java class:

| JNI Function | Rust Signature | Description |
|---|---|---|
| `nativeInit()` | `-> jlong` | Allocates an `Engine` and returns its pointer |
| `nativeLoadHtml(ptr, html)` | `-> void` | Loads HTML into the engine |
| `nativeLoadCss(ptr, css)` | `-> void` | Injects CSS into the engine |
| `nativeRender(ptr)` | `-> void` | Triggers a render pass |
| `nativeDestroy(ptr)` | `-> void` | Drops the engine and frees memory |

The bridge is implemented in `src/jni/bridge/bridge.rs`. Engine pointers are passed as `jlong` (raw pointer cast).

---

## Android Kotlin API

### `OptimaEngine`

```kotlin
val engine = OptimaEngine()
engine.loadHtml("<h1>Hello</h1>")
engine.loadCss("body { margin: 0; }")
engine.render()
engine.destroy() // must call to free native memory
```

Loads `liboptima.so` via `System.loadLibrary("optima")` in the companion object.

### `SystemFontHelper`

```kotlin
val fonts: Map<String, String> = SystemFontHelper.getSystemFonts()
// Maps font family names to file paths on the device
```

Scans `/system/fonts`, `/system/fonts/googlefonts`, `/product/fonts`, and `/vendor/fonts` for `.ttf` and `.otf` files. Uses `Typeface.createFromFile()` to resolve family names.

---

## Internal Module Structure

```
src/
├── api/          Public, private, and internal API layers
├── android/      Android platform integration (JNI wrappers)
├── config/       Settings, preferences, feature flags
├── console/      Console API implementation
├── css/          CSS parsing, cascade, selectors, animations
├── devtools/     Chrome DevTools Protocol implementation
├── dom/          DOM tree (Document, Node, Element, Text)
├── events/       Input event types (mouse, touch, keyboard, etc.)
├── integration/  High-level integration helper
├── jni/          JNI bridge, handles, callbacks, type conversions
├── layout/       CSS layout engine (block, flex, grid, table)
├── media/        Audio/video decoding and playback
├── net/          HTTP client, fetch, cache, cookies
├── platform/     Platform-specific abstractions
├── render/       GPU rendering via Vello/wgpu
├── resource/     Resource loading and management
├── security/     Security policies (CORS, sandboxing)
├── text/         Font shaping, text layout, glyph rendering
└── utils/        Shared utilities
```
