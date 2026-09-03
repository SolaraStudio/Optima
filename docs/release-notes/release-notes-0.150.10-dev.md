# Release Notes — Optima 0.150.10-dev

**Development Pre-release** — SolaraStudio

This is a development pre-release of Optima, a WebView engine written in Rust for Android.

---

## Engine Core

- **`Engine` struct** (`src/api/engine/`) with full lifecycle management: `new`, `load_html`, `load_url`, `inject_css`, `set_viewport`, `render`, `tick`, `start`, `stop`, `reload`, `go_back`, `go_forward`.
- **`EngineConfig`** (`src/api/config/`) with builder pattern: `with_user_agent`, `with_javascript`, `with_images`, `with_css`, `with_font_size`, `with_timeout`. Supports construction from `Settings` via `from_settings`.
- **`NavigationState`** (`src/api/navigation/`) with URL history, back/forward navigation, and loading state.
- **`EventDispatcher`** (`src/api/events/`) for event listener management.
- **`Version`** API (`src/api/version/`) with semantic versioning and compatibility checks.

## DOM

- Full DOM tree implementation (`src/dom/`) with `Document`, `Node`, `Element`, `Text`, `Comment`, `Doctype`, `DocumentFragment`.
- Node types: `Document`, `Element`, `Text`, `Comment`, `Doctype`, `DocumentFragment`.
- Tree manipulation: `append_child`, `remove_child`, `clone_node`.
- Element queries: `get_element_by_id`, `get_elements_by_tag_name`, `get_elements_by_class_name`, `query_selector`, `query_selector_all`.
- Element API: `tag_name`, `get_attribute`, `set_attribute`, `inner_html`, `outer_html`, `set_inner_html`.
- Mutation observer and mutation records.
- Tree walker for DOM traversal.
- Range and selection support.

## CSS

- CSS tokenizer and parser (`src/css/tokenizer/`, `src/css/ast/`).
- Selector matching (`src/css/selector/`) with specificity calculation (`src/css/specificity/`).
- Cascade resolution (`src/css/cascade/`) and property inheritance (`src/css/inheritance/`).
- Computed styles (`src/css/computed/`).
- Stylesheet and rule management (`src/css/stylesheet/`, `src/css/rule/`).
- Media queries (`src/css/media/`).
- `@font-face` rules (`src/css/font_face/`).
- CSS animations and keyframes (`src/css/animation/`, `src/css/keyframes/`).
- CSS transitions (`src/css/transition/`).
- CSS transforms (`src/css/transform/`).
- Color parsing (`src/css/colors/`).
- CSS units and length conversion (`src/css/units/`).
- CSS value types (`src/css/value/`).
- Declaration handling (`src/css/declaration/`).

## Layout

- Block and inline formatting contexts.
- Flexbox layout via Taffy (`src/layout/flex/`).
- CSS Grid layout via Taffy (`src/layout/grid/`).
- Table layout (`src/layout/table/`).
- Box model computation (`src/layout/box_model/`).
- Positioned elements (absolute, relative, fixed) (`src/layout/positioned/`).
- Line breaking and line boxes (`src/layout/line/`).
- Fragment tree output (`src/layout/fragment/`).
- Size measurement and constraints (`src/layout/measure/`).
- Container queries (`src/layout/container/`).
- Flow context management (`src/layout/flow/`).

## Text

- Font loading and management (`src/text/font/`).
- Text shaping via rustybuzz (`src/text/shaping/`).
- Text layout and line breaking (`src/text/layout/`, `src/text/break_/`).
- Glyph rendering via fontdue (`src/text/glyph/`, `src/text/render/`).
- Text runs (`src/text/run/`).
- Text measurement (`src/text/measure/`).
- Font fallback chain (`src/text/fallback/`).

## Rendering

- GPU rendering via Vello (`src/render/vello/`).
- wgpu backend for device/queue management (`src/render/backend/`).
- Render operations: Rect, Path, Text, Group.
- Paint commands: fill, stroke, gradient (`src/render/paint/`).
- Vector paths (`src/render/path/`).
- Affine transforms (`src/render/transform/`).
- Clipping and masking (`src/render/clip/`, `src/render/mask/`).
- Blend modes (`src/render/blend/`).
- CSS filters (`src/render/filter/`).
- Gradient rendering (`src/render/gradient/`).
- Visual effects (`src/render/effect/`).
- Image rendering (`src/render/image/`).
- Shape primitives (`src/render/shapes/`).

## Networking

- HTTP client via reqwest (`src/net/http/`, `src/net/https/`).
- Fetch API with Request/Response/Method types (`src/net/fetch/`).
- HTTP response caching (`src/net/cache/`).
- Cookie management (`src/net/cookie/`).
- DNS resolution (`src/net/dns/`).
- Header management (`src/net/header/`).
- Body handling (`src/net/body/`).
- Redirect following (`src/net/redirect/`).
- Proxy support (`src/net/proxy/`).
- Retry logic (`src/net/retry/`).
- Timeout handling (`src/net/timeout/`).
- Resource loading (`src/net/resource/`).

## Media

- Audio decoding and playback (`src/media/audio/`, `src/media/playback/`).
- Video decoding (`src/media/video/`, `src/media/decoder/`).
- Container format demuxing (`src/media/demuxer/`).
- Codec negotiation (`src/media/codec/`).
- Audio/video synchronization (`src/media/sync/`).
- Volume control (`src/media/volume/`).
- Subtitle support (`src/media/subtitle/`).
- Autoplay policies (`src/media/autoplay/`).
- Media streaming (`src/media/stream/`).
- Media rendering (`src/media/renderer/`).

## Input Events

- Mouse events: click, mousedown, mousemove, mouseup (`src/events/`).
- Touch events: touchstart, touchmove, touchend (`src/events/`).
- Keyboard events: keydown, keyup (`src/events/`).
- Pointer events (`src/events/pointer/`).
- Focus events (`src/events/focus/`).
- Scroll events (`src/events/scroll/`).
- Resize events (`src/events/resize/`).
- Gesture events (`src/events/gesture/`).

## Android Integration

- **JNI Bridge** (`src/jni/bridge/`) with five native functions:
  - `Java_org_optima_OptimaEngine_nativeInit`
  - `Java_org_optima_OptimaEngine_nativeLoadHtml`
  - `Java_org_optima_OptimaEngine_nativeLoadCss`
  - `Java_org_optima_OptimaEngine_nativeRender`
  - `Java_org_optima_OptimaEngine_nativeDestroy`
- **Handle table** (`src/jni/handles/`) for managing Rust objects across JNI boundary.
- **Callback registry** (`src/jni/callbacks/`) for cross-language callbacks.
- **`OptimaEngine.kt`** — Kotlin wrapper class with `loadHtml`, `loadCss`, `render`, `destroy`.
- **`SystemFontHelper.kt`** — System font discovery from Android font directories.
- **Android platform wrappers** (`src/android/`):
  - `AndroidSurface` — wgpu surface management
  - `AndroidDisplay` — display metrics
  - `AndroidFonts` — system font access via JNI
  - `AndroidInput` — touch/key event forwarding
  - `AndroidView` — view dimensions
  - `AndroidWindow` — window flags
  - `AndroidLifecycle` — Activity lifecycle tracking
  - `AndroidContext` — app context
  - `AndroidActivity` — Activity reference
  - `AndroidTexture` — texture management
  - `AndroidAssets` — asset access
  - `AndroidResource` — Android resource access

## Configuration

- **`Settings`** (`src/config/settings/`) — top-level configuration.
- **`Preferences`** (`src/config/preferences/`) — user preferences (font, language, dark mode, user agent).
- **`FeatureFlags`** (`src/config/feature/`) — optional feature toggles (GPU, WebGL, WebSocket, etc.).
- **`DebugConfig`** (`src/config/debug/`) — debug options.
- **`EnvConfig`** (`src/config/env/`) — environment configuration.
- **`ReleaseConfig`** (`src/config/release/`) — release-specific settings.
- **`Flags`** (`src/config/flags/`) — build flags.

## DevTools

- Chrome DevTools Protocol implementation (`src/devtools/`).
- `DevToolsServer` with client management.
- `DevToolsProtocol` message parsing.
- `DevToolsBackend` command handlers.
- `DevToolsClient` connection handling.
- JSON message types.

## Security

- Security policies: SameOrigin, NoCORS, CORS, Strict (`src/security/`).
- Origin allow/block lists.
- Sandbox mode (enabled by default).
- Protocol restrictions (`allow_file_protocol` disabled by default).

## Resource Management

- URL-keyed resource store (`src/resource/`).
- Resource types: Image, Font, Script, Style, Audio, Video, Document.
- Resource lifecycle management (insert, get, remove, clear).

## Build

- `build-all.sh` — cross-compiles for all Android ABIs and produces AAR.
- Gradle tasks: `buildRustAll`, `copyRustLibs`.
- Maven publishing to GitHub Packages.
- `.cargo/config.toml` — NDK linker configuration for all four targets.
- Release profile: LTO enabled, single codegen unit, stripped symbols.
