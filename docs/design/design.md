# Optima Architecture Overview

Optima is a WebView engine written in Rust, compiled to a C dynamic library (`liboptima.so`) for Android. It parses HTML/CSS, performs layout, renders with Vello/wgpu, handles networking, fonts, audio/video, and exposes a JNI bridge for Android integration.

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────┐
│                  Android App                     │
│   OptimaEngine.kt  /  SystemFontHelper.kt       │
└────────────────────────┬────────────────────────┘
                         │ JNI
┌────────────────────────▼────────────────────────┐
│                 JNI Bridge (jni/)                │
│  bridge.rs · handles · callbacks · conversions  │
└────────────────────────┬────────────────────────┘
                         │
┌────────────────────────▼────────────────────────┐
│              Public API (api/)                   │
│  Engine · EngineConfig · NavigationState        │
│  EventDispatcher · Version · Compat             │
└────────────────────────┬────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
   ┌─────────┐    ┌──────────┐    ┌───────────┐
   │  CSS    │    │   DOM    │    │  Events   │
   │ (css/)  │    │  (dom/)  │    │ (events/) │
   └────┬────┘    └────┬─────┘    └───────────┘
        │              │
        ▼              ▼
   ┌─────────────────────────┐
   │    Layout Engine        │
   │   (layout/)            │
   │  block · flex · grid   │
   │  table · inline · flow │
   └────────────┬────────────┘
                │
                ▼
   ┌─────────────────────────┐
   │   Rendering Pipeline    │
   │   (render/)             │
   │  Vello · wgpu · paint   │
   └────────────┬────────────┘
                │
                ▼
   ┌─────────────────────────┐
   │     Output (GPU)        │
   └─────────────────────────┘
```

---

## Pipeline: HTML to Pixels

The engine processes content through a well-defined pipeline:

### 1. Input

HTML and CSS are provided via `Engine::load_html()` or `Engine::inject_css()`. For Android, the JNI bridge forwards these from `OptimaEngine.kt`.

### 2. Parsing

- **HTML** is parsed into a DOM tree (`src/dom/`). The `Document` struct is the root, containing `Node` objects organized in a tree. Nodes have types: `Element`, `Text`, `Comment`, `Doctype`, `DocumentFragment`.
- **CSS** is tokenized and parsed into `Stylesheet` objects (`src/css/`). The CSS parser handles selectors, declarations, `@media`, `@font-face`, animations, and keyframes.

### 3. Style Resolution

The CSS cascade (`src/css/cascade/`) resolves which styles apply to each DOM element. It computes `ComputedStyle` objects by:

- Matching selectors against elements (`src/css/selector/`)
- Computing specificity (`src/css/specificity/`)
- Handling inheritance (`src/css/inheritance/`)
- Resolving cascade layers and specificity conflicts

### 4. Layout

The layout engine (`src/layout/`) computes the position and size of every element using the CSS box model (`src/layout/box_model/`). It supports:

- **Block formatting context** — standard block-level layout
- **Inline formatting context** — text and inline elements
- **Flexbox** — `src/layout/flex/` (via `taffy`)
- **CSS Grid** — `src/layout/grid/` (via `taffy`)
- **Table layout** — `src/layout/table/`
- **Line breaking** — `src/layout/line/`
- **Positioned elements** — `src/layout/positioned/` (absolute, relative, fixed)
- **Measurement** — `src/layout/measure/` (sizing constraints)

The layout engine produces a tree of `Fragment` objects (`src/layout/fragment/`), each representing a rectangular region with position and size.

### 5. Text Shaping

Before layout, text content is shaped by the text subsystem (`src/text/`):

- **Font loading** — `src/text/font/` loads and manages fonts
- **Text shaping** — `src/text/shaping/` (via `rustybuzz`) handles bidirectional text, ligatures, and glyph positioning
- **Line breaking** — `src/text/break_/` determines word and line break opportunities
- **Glyph rendering** — `src/text/glyph/` and `src/text/render/` rasterize glyphs (via `fontdue`)

### 6. Rendering

The rendering pipeline (`src/render/`) converts the layout tree to GPU draw calls:

- **Vello renderer** — `src/render/vello/` is the primary renderer, producing `RenderOp` objects (rectangles, paths, text)
- **wgpu backend** — `src/render/backend/` manages the GPU device, queue, and surface
- **Paint commands** — `src/render/paint/` defines fill, stroke, and gradient paints
- **Transforms** — `src/render/transform/` handles affine transforms
- **Clipping and masking** — `src/render/clip/` and `src/render/mask/`
- **Effects** — `src/render/effect/` for shadows, blur, and filters

The final output is an RGBA pixel buffer returned by `Engine::render()`.

---

## Module Dependency Graph

```
api → dom, css, events, navigation, config
jni → api, dom, css
android → jni, platform
css → (standalone parser + cascade)
dom → (standalone tree)
layout → dom, css (box model, styles)
text → font data (rustybuzz, fontdue)
render → layout fragments, paint commands, vello/wgpu
net → reqwest (HTTP), cache, cookies
media → symphonia (audio), yscv-video (video), cpal (playback)
events → dom (event dispatch)
devtools → api, serde_json (Chrome DevTools Protocol)
security → origin policy, CORS
resource → URL-keyed resource store
config → settings, preferences, feature flags
```

---

## Key Design Decisions

### Single-Threaded DOM

The DOM uses `Rc<RefCell<Node>>` for shared mutable access within a single thread. This is appropriate for a WebView engine where the DOM is primarily accessed from the rendering thread. Multi-threaded access (e.g., from DevTools or networking) is handled through `Arc<Mutex<T>>` at the API boundary.

### Taffy for Layout

The layout engine delegates flexbox and grid calculations to [Taffy](https://github.com/DioxusLabs/taffy), a Rust layout library. Block and inline layout are implemented natively for finer control over CSS spec compliance.

### Vello for Rendering

[Vello](https://github.com/linebender/vello) is used as the GPU rendering backend, leveraging [wgpu](https://github.com/gfx-rs/wgpu) for cross-platform GPU access. Vello produces high-quality 2D vector graphics with GPU acceleration.

### JNI Bridge Pattern

The JNI bridge follows a pointer-passing pattern: the Rust `Engine` is heap-allocated and its raw pointer is passed to Java/Kotlin as a `jlong`. Each JNI function casts the pointer back and calls the corresponding `Engine` method. This avoids the overhead of repeated object allocation and keeps the Rust ownership model intact.

### Android-First

The primary target is Android. The `src/android/` module provides JNI wrappers for Android platform APIs (surface, display, fonts, input, lifecycle). Desktop support exists in `src/platform/desktop/` but is secondary.

---

## Threading Model

| Component | Thread | Notes |
|-----------|--------|-------|
| DOM | Main thread | Single-threaded, `Rc<RefCell>` |
| CSS parsing | Main thread | Synchronous |
| Layout | Main thread | Synchronous |
| Rendering | Main thread | GPU submission via wgpu |
| Networking | Tokio runtime | Async via `reqwest` |
| Audio/Video | Background threads | `symphonia` + `cpal` |
| DevTools | Background threads | `Arc<Mutex>` shared state |

---

## Configuration Layers

Configuration flows through three layers:

1. **`Settings`** (`src/config/settings/`) — top-level container with debug, env, features, preferences, release config
2. **`EngineConfig`** (`src/api/config/`) — engine-specific settings derived from `Settings`
3. **`FeatureFlags`** (`src/config/feature/`) — optional feature toggles (GPU, WebGL, WebSocket, etc.)

The `Integration` struct ties them together, creating an `Engine` from `Settings` via `EngineConfig::from_settings()`.

---

## Security Model

The `SecurityManager` (`src/security/`) enforces:

- **Same-origin policy** by default
- **CORS** with configurable allowed/blocked origins
- **Sandbox mode** (enabled by default)
- **Protocol restrictions** — `allow_file_protocol` is off by default

Origin checking is performed before network requests and resource loading.

---

## DevTools Protocol

Optima implements a subset of the Chrome DevTools Protocol (`src/devtools/`):

- **Server** (`src/devtools/server/`) — manages client connections
- **Protocol** (`src/devtools/protocol/`) — message parsing and routing
- **Backend** (`src/devtools/backend/`) — command handlers
- **Messages** (`src/devtools/messages/`) — JSON message types

The server runs on a background thread and communicates with the engine through the backend.
