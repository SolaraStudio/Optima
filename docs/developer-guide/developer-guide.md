# Developer Guide

This guide walks through the Optima codebase module by module, explains how to add new features, and covers the build and test workflow.

---

## Getting Started

1. Clone the repository.
2. Install Rust (stable, edition 2024) and Android NDK.
3. Install Android targets: `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`
4. Build: `./scripts/android/build-all.sh`

---

## Module Walkthrough

### `src/api/` — Public API Layer

The entry point for consumers of the engine. Contains:

- **`engine/`** — The `Engine` struct. This is the main type that holds the document, stylesheets, navigation state, and event dispatcher. All public methods live here.
- **`config/`** — `EngineConfig` with builder pattern. Constructed via `EngineConfig::new()` or `EngineConfig::from_settings()`.
- **`navigation/`** — `NavigationState` with a `VecDeque<String>` URL history and cursor.
- **`events/`** — `EventDispatcher` with `HashMap<String, Vec<Callback>>` for event listeners.
- **`version/`** — `Version` struct and compile-time constants.
- **`compat/`** — `Compat` for platform detection (`Android`, `Desktop`, `Web`, `Unknown`).
- **`public/`** — `PublicApi` for high-level init.
- **`private/`** — `PrivateApi` wrapping an `Engine` instance.
- **`internal/`** — `InternalEngine` exposing raw document and stylesheet access.

### `src/dom/` — Document Object Model

A W3C-inspired DOM implementation:

- **`node/`** — `Node` with `NodeType` enum (`Document`, `Element`, `Text`, `Comment`, `Doctype`, `DocumentFragment`). Tree manipulation via `append_child`, `remove_child`.
- **`document/`** — `Document` as the root. Factory methods: `create_element`, `create_text_node`, `create_comment`. Query methods: `get_element_by_id`, `get_elements_by_tag_name`, `get_elements_by_class_name`, `query_selector`, `query_selector_all`.
- **`element/`** — `Element` wrapping an `Rc<RefCell<Node>>`. Attribute access, `inner_html`/`outer_html`, `set_inner_html`.
- **`text/`**, **`comment/`**, **`doctype/`**, **`fragment/`** — specialized node types.
- **`mutation/`**, **`observer/`** — Mutation records and observers.
- **`traversal/`** — `TreeWalker` for depth-first traversal.
- **`range/`**, **`selection/`** — Range and selection support.

### `src/css/` — CSS Engine

A full CSS parser and cascade engine:

- **`tokenizer/`** — `CSSParser` that tokenizes CSS text.
- **`ast/`** — Abstract syntax tree types for CSS.
- **`selector/`** — `Selector` matching against DOM elements.
- **`specificity/`** — `Specificity` comparison for cascade ordering.
- **`cascade/`** — `Cascade` resolver that determines which declarations win.
- **`declaration/`** — Individual CSS property declarations.
- **`value/`** — CSS value types.
- **`computed/`** — `ComputedStyle` — the final computed style for an element.
- **`inheritance/`** — Property inheritance rules.
- **`stylesheet/`** — `Stylesheet` containing parsed rules.
- **`rule/`** — CSS rule types (style, media, font-face).
- **`media/`** — `MediaQuery` evaluation.
- **`font_face/`** — `FontFaceRule` for custom font loading.
- **`animation/`**, **`keyframes/`** — CSS animations.
- **`transition/`** — CSS transitions.
- **`transform/`** — CSS transform parsing.
- **`colors/`** — `Color` type and parsing.
- **`units/`** — `Length`, `LengthUnit`, and unit conversion.

### `src/layout/` — Layout Engine

Computes positions and sizes:

- **`box_model/`** — `BoxModel` with margin, border, padding, content.
- **`block/`** — Block-level layout.
- **`block_formatting/`** — Block formatting context.
- **`inline/`** — Inline-level layout.
- **`inline_formatting/`** — Inline formatting context.
- **`line/`** — Line box construction.
- **`flex/`** — Flexbox layout (delegates to `taffy`).
- **`grid/`** — CSS Grid layout (delegates to `taffy`).
- **`table/`** — Table layout.
- **`container/`** — Container queries.
- **`flow/`** — `FlowContext` for normal flow.
- **`fragment/`** — `Fragment` — a positioned, sized rectangle in the layout tree.
- **`positioned/`** — Absolutely and fixed positioned elements.
- **`measure/`** — Size measurement and constraints.
- **`resolve/`** — Style-to-layout property resolution.

### `src/text/` — Text Rendering

- **`font/`** — Font loading and management.
- **`shaping/`** — Text shaping via `rustybuzz`.
- **`layout/`** — Text layout and line breaking.
- **`line/`** — Line box construction.
- **`glyph/`** — Individual glyph data.
- **`render/`** — Glyph rasterization via `fontdue`.
- **`run/`** — Text runs (contiguous styled text).
- **`measure/`** — Text measurement.
- **`break_/`** — Word and line break opportunities.
- **`fallback/`** — Font fallback chain.

### `src/render/` — GPU Rendering

- **`backend/`** — `RenderBackend` managing `wgpu::Device`, `Queue`, and `Surface`.
- **`vello/`** — `VelloRenderer` producing `RenderOp` (Rect, Path, Text, Group).
- **`paint/`** — `PaintCommand` for fill, stroke, gradient.
- **`path/`** — Vector path definitions.
- **`shapes/`** — Primitive shapes.
- **`transform/`** — `RenderTransform` (affine matrix).
- **`clip/`**, **`mask/`** — Clipping and masking.
- **`blend/`** — Blend modes.
- **`filter/`** — CSS filters (blur, brightness, etc.).
- **`gradient/`** — Gradient rendering.
- **`effect/`** — Visual effects (shadows, etc.).
- **`image/`** — Image rendering.

### `src/net/` — Networking

- **`fetch/`** — `Fetch` with `Request`, `Response`, `Method` (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS).
- **`http/`** — `HttpClient` wrapping `reqwest`.
- **`https/`** — TLS configuration.
- **`cache/`** — HTTP response cache.
- **`cookie/`** — Cookie management.
- **`header/`** — `Headers` map.
- **`body/`** — `Body` type for request/response bodies.
- **`dns/`** — DNS resolution.
- **`proxy/`** — Proxy configuration.
- **`redirect/`** — Redirect handling.
- **`resource/`** — Resource loading.
- **`retry/`** — Retry logic.
- **`timeout/`** — Request timeout handling.

### `src/media/` — Audio and Video

- **`audio/`**, **`video/`** — Media element handling.
- **`decoder/`**, **`encoder/`** — Media codec operations.
- **`demuxer/`** — Container format demuxing.
- **`playback/`** — Playback control.
- **`renderer/`** — Audio/video rendering.
- **`stream/`** — Stream management.
- **`sync/`** — Audio/video synchronization.
- **`volume/`** — Volume control.
- **`subtitle/`** — Subtitle rendering.
- **`codec/`** — Codec negotiation.
- **`autoplay/`** — Autoplay policies.

### `src/events/` — Input Events

Event types organized by input device:

- Mouse: `ClickEvent`, `MouseEvent`, `MouseDownEvent`, `MouseMoveEvent`, `MouseUpEvent`
- Touch: `TouchEvent`, `TouchStartEvent`, `TouchMoveEvent`, `TouchEndEvent`
- Keyboard: `KeyEvent`, `KeyDownEvent`, `KeyUpEvent`
- Pointer: `PointerEvent`
- Focus: `FocusEvent`
- Scroll: `ScrollEvent`
- Resize: `ResizeEvent`
- Gesture: `GestureEvent`

### `src/android/` — Android Integration

JNI wrappers for Android platform APIs:

- **`surface/`** — `AndroidSurface` for wgpu surface management.
- **`display/`** — `AndroidDisplay` for display metrics.
- **`fonts/`** — `AndroidFonts` for system font access.
- **`input/`** — `AndroidInput` for touch/key events.
- **`view/`** — `AndroidView` for view dimensions and layout.
- **`window/`** — `AndroidWindow` for window flags.
- **`lifecycle/`** — `AndroidLifecycle` tracking Activity lifecycle states (Created, Started, Resumed, Paused, Stopped, Destroyed).
- **`context/`** — `AndroidContext` for app context.
- **`activity/`** — `AndroidActivity` reference.
- **`texture/`** — `AndroidTexture` for texture management.
- **`assets/`** — `AndroidAssets` for asset access.
- **`resource/`** — `AndroidResource` for Android resource access.

### `src/jni/` — JNI Bridge

- **`bridge/`** — The five `Java_org_optima_OptimaEngine_*` native functions.
- **`handles/`** — `HandleTable` for mapping integer IDs to Rust objects across the JNI boundary.
- **`callbacks/`** — `CallbackRegistry` for registering and invoking callbacks by ID.
- **`conversions/`** — Type conversion helpers (Rust ↔ JNI).
- **`errors/`** — JNI error handling.
- **`types/`** — JNI type definitions.

### `src/devtools/` — Chrome DevTools Protocol

- **`server/`** — `DevToolsServer` with client management.
- **`protocol/`** — `DevToolsProtocol` message parsing.
- **`backend/`** — `DevToolsBackend` command handlers.
- **`client/`** — `DevToolsClient` connection handling.
- **`messages/`** — `DevToolsMessage` types (request, response, event).

### `src/security/` — Security

- `SecurityPolicy` enum: `SameOrigin`, `NoCORS`, `CORS { origins }`, `Strict`.
- `SecurityManager` with `allowed_origins`, `blocked_origins`, and `sandbox_enabled`.

### `src/resource/` — Resource Management

- `ResourceManager` — URL-keyed store for loaded resources (images, fonts, scripts, styles, audio, video).
- `ResourceType` enum for categorizing resources.
- `Resource` struct with `id`, `url`, `data`, and `loaded` status.

---

## Adding a New Feature

### Adding a New CSS Property

1. Add the property name to the CSS parser in `src/css/tokenizer/`.
2. Add a value type in `src/css/value/` if needed.
3. Add the declaration in `src/css/declaration/`.
4. Add inheritance rules in `src/css/inheritance/`.
5. Add computed style handling in `src/css/computed/`.
6. Add layout support in `src/layout/` if the property affects layout.
7. Add rendering support in `src/render/` if the property affects visual output.
8. Add tests in `tests/unit/`.

### Adding a New DOM API

1. Add the method to the appropriate type in `src/dom/` (e.g., `Document`, `Element`, `Node`).
2. Follow the existing pattern of `Rc<RefCell<Node>>` access.
3. Add query/traversal support if needed.
4. Expose through `Engine::document()` if it should be accessible from the API.

### Adding a New Event Type

1. Create the event struct in `src/events/`.
2. Add it to `src/events/mod.rs` exports.
3. Wire it up in `src/api/events/events.rs` or the event dispatcher.
4. Add JNI forwarding in `src/jni/` if needed for Android.

### Adding a New Render Operation

1. Add a variant to `RenderOp` in `src/render/vello/vello.rs`.
2. Add the rendering implementation in the appropriate render module.
3. Update `VelloRenderer` with the new method.

---

## Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --test unit

# Run a specific test
cargo test test_css_specificity

# Run benchmarks
cargo bench
```

Tests are in `tests/{unit,integration,e2e,benchmarks}/`. Test resources are in `tests/resources/`.

---

## Common Patterns

### Builder Pattern

Used by `EngineConfig`, `Request`, `Settings`, `FeatureFlags`:

```rust
let config = EngineConfig::new()
    .with_user_agent("MyApp/1.0")
    .with_font_size(18);
```

### Ref-Counted Shared State

DOM nodes use `Rc<RefCell<Node>>`:

```rust
let node = Rc::new(RefCell::new(Node::create_element("div")));
node.borrow_mut().set_attribute("class", "container");
```

### Error Handling

- `Engine` methods return `Result<(), String>`.
- Internal modules use `anyhow::Result` or `thiserror`-based error types.
- JNI errors are converted to Java exceptions.

### Platform Abstraction

`src/platform/` contains platform-specific implementations:

- `android/` — Android-specific code
- `desktop/` — Desktop-specific code (Linux, macOS, Windows)
- `common/` — Shared platform utilities
