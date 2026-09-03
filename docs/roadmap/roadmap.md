# Optima Roadmap

This document outlines the planned development direction for Optima. Items are organized by priority and phase.

---

## Current Status (v0.150.10-dev)

Optima is in active development. The core pipeline (HTML → DOM → CSS → Layout → Render) is functional. The engine compiles to `liboptima.so` for Android and is exposed via a JNI bridge.

### What Works

- DOM tree construction and manipulation
- CSS parsing, cascade, and style resolution
- Layout (block, inline, flex, grid, table)
- GPU rendering via Vello/wgpu
- Text shaping and font rendering
- Android JNI bridge with Kotlin wrapper
- Basic networking (HTTP, caching, cookies)
- Audio/video framework (decoding, playback, sync)
- Input event system (mouse, touch, keyboard)
- DevTools protocol (partial)
- Security policies (CORS, sandboxing)
- System font discovery on Android

---

## Phase 1: Core Stability

**Goal:** Make the engine reliable for basic WebView use cases.

### HTML Parser Improvements
- [ ] Robust HTML parser (currently simplified wrapping)
- [ ] Handle malformed HTML gracefully
- [ ] Support for `<head>`, `<meta>`, `<link>`, `<script>` parsing
- [ ] Form element support

### CSS Completeness
- [ ] Complete CSS property coverage for common properties
- [ ] CSS custom properties (variables)
- [ ] `calc()` function support
- [ ] `clamp()`, `min()`, `max()` functions
- [ ] CSS nesting
- [ ] Container queries (full support)

### Layout Polish
- [ ] Fix edge cases in block layout
- [ ] Improve inline layout for mixed inline/block content
- [ ] Table layout refinements
- [ ] `position: sticky` support
- [ ] `overflow` handling (scroll, auto, hidden)

### Rendering
- [ ] Complete paint command coverage
- [ ] Image rendering (JPEG, PNG, SVG, WebP)
- [ ] Rounded corners and border radius
- [ ] Box shadow rendering
- [ ] Opacity and transparency

---

## Phase 2: Feature Parity

**Goal:** Match essential WebView functionality for production apps.

### Networking
- [ ] Full fetch API implementation
- [ ] WebSocket support
- [ ] Service worker support
- [ ] CORS preflight handling
- [ ] Resource prioritization and lazy loading

### JavaScript
- [ ] JavaScript engine integration (prepare interface)
- [ ] DOM manipulation from JS
- [ ] Event handling from JS
- [ ] `console.log` bridging

### Media
- [ ] HTML5 `<video>` element rendering
- [ ] HTML5 `<audio>` element playback
- [ ] `<canvas>` element support
- [ ] `<iframe>` support (sandboxed)

### Accessibility
- [ ] ARIA attribute support in DOM
- [ ] Accessibility tree generation
- [ ] Screen reader integration hooks

### Input
- [ ] Complete mouse event propagation
- [ ] Touch event gesture recognition
- [ ] Focus management and tab navigation
- [ ] Text input (IME support)

---

## Phase 3: Advanced Features

**Goal:** Differentiate Optima with advanced capabilities.

### Performance
- [ ] Incremental layout (dirty flag propagation)
- [ ] Render caching and invalidation
- [ ] GPU-accelerated compositing layers
- [ ] Parallel layout for independent subtrees
- [ ] Memory-mapped resource loading

### Rendering Enhancements
- [ ] WebGL support
- [ ] CSS filter effects (full coverage)
- [ ] CSS backdrop-filter
- [ ] SVG rendering
- [ ] `<canvas>` 2D context
- [ ] Hardware-accelerated video compositing

### DevTools
- [ ] Full Chrome DevTools Protocol coverage
- [ ] Network inspection
- [ ] Performance profiling
- [ ] Memory profiling
- [ ] CSS inspector

### Platform
- [ ] Desktop builds (Linux, macOS, Windows)
- [ ] Embedded rendering (headless mode)
- [ ] Multi-window support
- [ ] Plugin architecture for extensions

---

## Phase 4: Ecosystem

**Goal:** Build a thriving ecosystem around Optima.

### Developer Experience
- [ ] Comprehensive API documentation
- [ ] Code examples and cookbook
- [ ] CI/CD pipeline improvements
- [ ] Automated performance regression testing
- [ ] Fuzzing infrastructure

### Testing
- [ ] Web Platform Tests (WPT) compatibility testing
- [ ] CSS spec compliance test suite
- [ ] Layout regression testing
- [ ] Visual regression testing (screenshot comparison)

### Community
- [ ] Contribution guidelines refinement
- [ ] Issue templates
- [ ] Discussion forums
- [ ] Regular release cadence

---

## Versioning Plan

Optima follows semantic versioning:

- **0.150.x** — Current development series
- **0.151.0** — Core stability release
- **0.152.0** — Feature parity release
- **1.0.0** — First stable release

Minor versions (0.15x) may contain breaking API changes. Patch versions (0.150.x) are bug fixes only.

---

## Platform Priorities

1. **Android** (primary) — Full feature coverage
2. **Desktop** (secondary) — Core features, development and testing
3. **Headless** (tertiary) — Server-side rendering, testing, screenshots

---

## Getting Involved

Check the [Contributing Guide](../contributing/CONTRIBUTING.md) for how to help with development. Priority areas for community contributions:

- CSS property implementations
- Layout edge case fixes
- Test coverage
- Documentation
- Performance benchmarks
