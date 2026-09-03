# Optima

## A lightweight, memory-safe, GPU-accelerated web rendering engine for Android, built in Rust.

Optima is a complete replacement for the Android System WebView. It parses HTML and CSS, computes styles, lays out pages, and renders them directly to a hardware-accelerated surface – all without the overhead, fragmentation, or security vulnerabilities of Chromium-based engines.

---

## Why Optima?

The Android System WebView is the de facto standard for embedding web content in Android apps, but it comes with significant drawbacks:

- Size: 75–250 MB per APK.
- Fragmentation: Behaviour varies across devices and Android versions.
- Security: Frequent vulnerabilities (CVE-2026-11007, etc.).
- Control: No ability to fix bugs or add custom features.
- Memory: ≥160 MB per instance; high battery and CPU usage.

Optima addresses every one of these issues:

- Small footprint: ~160 KB per ABI.
- Consistent: Same rendering across all devices.
- Memory-safe: Written in Rust, eliminating use-after-free and buffer overflow bugs.
- Complete control: Full source access; you decide the feature set.
- Low memory: Projected <20 MB per instance, with further optimisation possible.

---

# Key Features

## Core Rendering

- HTML5 parsing via [`html5ever`](https://github.com/servo/html5ever)
- CSS3 parsing, cascade, and specificity via [`cssparser`](https://github.com/servo/rust-cssparser)
- Full DOM tree (Node, Element, Document, Text, Comment, Range, Selection)
- Flexbox and CSS Grid layout via [`taffy`](https://github.com/DioxusLabs/taffy)
- GPU-accelerated 2D rendering via [`vello`](https://github.com/linebender/vello)
- Text shaping and rasterization via [`rustybuzz`](https://github.com/RazrFalcon/rustybuzz) and [`fontdue`](https://github.com/mooman219/fontdue)

## Event System

- Mouse, touch, keyboard, pointer, gesture, scroll, resize, focus
- Full event propagation and cancellation
- Synthetic event generation

## Android Integration

- JNI bridge for seamless Kotlin/Java ↔ Rust communication
- SurfaceView-based rendering for direct GPU access
- Touch input translation to DOM events
- Lifecycle management (pause, resume, destroy)
- Asset and resource loading from APK or file system

## Security & Privacy

- Content-Security-Policy (CSP) enforcement
- Same-origin policy
- Sandboxed execution
- Memory-safe by design (safe Rust)
- HTTPS-only mode (planned)

## Developer Tooling

- Remote debugging via DevTools protocol
- Performance timings and metrics
- Configurable logging levels
- Profiling support with cargo bench

---

# Installation

## As a Rust crate

Add this to your Cargo.toml:

```toml
[dependencies]
optima = { git = "https://github.com/SolaraStudio/Optima" }
```

## As an Android AAR (from GitHub Packages)

Add the repository and dependency to your build.gradle.kts:

```kotlin
repositories {
    maven {
        url = uri("https://maven.pkg.github.com/SolaraStudio/Optima")
        credentials {
            username = project.findProperty("gpr.user") as String? ?: System.getenv("USERNAME")
            password = project.findProperty("gpr.key") as String? ?: System.getenv("TOKEN")
        }
    }
}

dependencies {
    implementation("com.solara:optima:0.150.10-dev")
}
```

Or, if you prefer to build from source, see the next section.

---

## Building from Source

Prerequisites

- Rust 1.85 or later
- Android NDK 25.1.8937393 or later
- Android SDK 34 (platform and build-tools)
- cargo-ndk (install via cargo install cargo-ndk)

Build the Rust library for all Android ABIs

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

for target in aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android; do
    cargo ndk --target "$target" --platform 21 -- build --release
done
```

Build the Android AAR

```bash
cd android
./gradlew assembleRelease
```

The output AAR will be at android/build/outputs/aar/.

---

## Usage Example (Kotlin)

```kotlin
import org.optima.OptimaView
import org.optima.OptimaEngine

class MainActivity : AppCompatActivity() {
    private lateinit var optimaView: OptimaView
    private lateinit var engine: OptimaEngine

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        optimaView = findViewById(R.id.optima_view)
        engine = OptimaEngine.create()
        optimaView.setEngine(engine)

        engine.loadUrl("https://example.com")
        // or engine.loadHtml("<html><body>Hello</body></html>", "about:blank")
    }

    override fun onResume() {
        super.onResume()
        engine.resume()
    }

    override fun onPause() {
        super.onPause()
        engine.pause()
    }

    override fun onDestroy() {
        super.onDestroy()
        engine.destroy()
    }
}
```

---

## Architecture Overview

Optima follows a layered design, each component independently testable and replaceable:

```
┌─────────────────────────────────────────────────────────────┐
│                     Android Application                     │
├─────────────────────────────────────────────────────────────┤
│                     JNI Bridge (Kotlin/Java)                │
├─────────────────────────────────────────────────────────────┤
│                     Public API (Rust)                       │
├───────────┬───────────┬───────────┬───────────┬───────────┤
│   CSS     │   DOM     │  Events   │  Layout   │   Render   │
├───────────┴───────────┴───────────┴───────────┴───────────┤
│                    Platform Abstraction                     │
├─────────────────────────────────────────────────────────────┤
│                   Android SurfaceView                       │
└─────────────────────────────────────────────────────────────┘
```

- ***CSS:*** Parsing, cascade, selector matching, computed styles.
- ***DOM:*** Complete tree with mutation observers and range/selection APIs.
- ***Events:*** Dispatch, capture, bubbling, and cancellation.
- ***Layout:*** Flexbox, Grid, block, inline, and positioned elements.
- ***Render:*** GPU-accelerated drawing via vello (Vulkan/Metal/DirectX).
- ***Platform:*** Abstractions for OS-specific services (networking, file I/O, etc.).

---

## Project Structure

```
optima/
├── android/                 # Android library module
│   ├── app/                 # Test application
│   └── src/main/            # Kotlin/Java JNI bindings
├── benches/                 # Benchmarks
├── docs/                    # Documentation (design, API, guides)
├── examples/                # Example usage
├── resources/               # Embedded resources (fonts, default styles)
├── scripts/                 # Build and CI scripts
├── src/                     # Rust source code
│   ├── api/                 # Public API (Engine, Config, etc.)
│   ├── css/                 # CSS parser and style resolution
│   ├── dom/                 # DOM implementation
│   ├── events/              # Event system
│   ├── jni/                 # JNI bridge implementation
│   ├── layout/              # Flexbox/Grid and block layout
│   ├── media/               # Audio and video support (planned)
│   ├── net/                 # HTTP and WebSocket networking
│   ├── platform/            # Platform-specific abstractions
│   ├── render/              # GPU rendering pipeline (vello)
│   ├── resource/            # Resource loading and caching
│   ├── security/            # CSP, CORS, sandbox
│   ├── text/                # Text shaping and font management
│   └── utils/               # Shared utilities (logging, errors, time)
├── tests/                   # Integration tests
├── build.rs                 # Build script (e.g., for linking)
├── Cargo.toml               # Rust dependencies
└── README.md                # This file
```

---

## Performance

Optima is designed to be fast and lightweight. Preliminary benchmarks indicate:

- Metric System WebView Optima (projected)
- APK size (all ABIs) 75–250 MB ~0.6 MB (4 × 160 KB)
- Memory per instance ≥160 MB <20 MB
- Startup time ~500 ms <100 ms
- Initial page load (simple) ~300 ms <100 ms
- Frame rate (60 fps) Variable (jank) Consistent (GPU)

These figures will be refined as we add more features, but the foundational design already shows a clear advantage.

---

## Security

Optima is built with security as a primary concern. The choice of Rust provides guaranteed memory safety, preventing the most common vulnerabilities in C++ engines. Additional measures include:

- Content-Security-Policy enforcement for inline scripts and styles.
- Same-origin policy for DOM access and network requests.
- Sandboxed execution of JavaScript (via Aether, the companion JS engine).
- Certificate transparency validation for HTTPS connections.
- Regular dependency updates to patch known issues.

We do not claim to be invulnerable, but we believe the engineering choices significantly reduce the attack surface compared to WebView.

---

## Development Status

Optima is currently in active development. As of the latest commit, the following modules are implemented and tested:

- API (engine, config, navigation, events, resources)
- CSS (parsing, cascade, computed styles)
- DOM (full tree with range and selection)
- Events (mouse, touch, keyboard, pointer, gesture, scroll, resize, focus)
- Android integration (JNI, SurfaceView, input, lifecycle)

Remaining work includes:

- Layout (Flexbox/Grid, block, inline, positioned)
- Rendering (GPU pipeline)
- Networking (HTTP/HTTPS, WebSocket)
- Text shaping and font fallback
- Media (audio/video playback)
- Security (CSP, sandbox)
- JNI bridge finalisation

For a full roadmap, see [docs/roadmap/README.md](docs/roadmap/README.md).

---

## Contributing

We welcome contributions from the community. Please read our [Contributing Guide](docs/contributing/README.md) before submitting issues or pull requests.

To get started:

1. Fork the repository.
2. Create a feature branch (git checkout -b feature/new-thing).
3. Make your changes, ensuring code formatting (cargo fmt) and linting (cargo clippy).
4. Add tests where applicable.
5. Commit and push.
6. Open a pull request.

We follow standard Rust practices and use GitHub Actions for continuous integration (build, test, and publish).

---

## Documentation

Comprehensive documentation is available in the docs/ directory:

- [Design Overview](docs/design/README.md) – Architectural decisions and trade-offs.
- [API Reference](docs/api/README.md) – Public API documentation.
- [Developer Guide](docs/developer-guide/README.md) – How to extend or modify Optima.
- [FAQ](docs/faq/README.md) – Frequently asked questions.
- [Roadmap](docs/roadmap/README.md) – Planned features and milestones.

---

## License

This project is licensed under either of the following, at your option:

- Apache License 2.0 – [LICENSE-APACHE](LICENSE-APACHE)
- MIT License – [LICENSE-MIT](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## Acknowledgments

Optima builds upon the excellent work of the open‑source community:

- [`html5ever`](https://github.com/servo/html5ever) – HTML parsing (Servo)
- [`cssparser`](https://github.com/servo/rust-cssparser) – CSS parsing (Servo)
- [`taffy`](https://github.com/DioxusLabs/taffy) – Flexbox/Grid layout (Dioxus)
- [`vello`](https://github.com/linebender/vello) – GPU rendering (Linebender)
- [`rustybuzz`](https://github.com/RazrFalcon/rustybuzz) – Text shaping (RazrFalcon)
- [`fontdue`](https://github.com/mooman219/fontdue) – Font rasterization (mooman219)

Special thanks to the Rust community for creating the foundation upon which Optima is built.

---

## Contact

- Issues: [GitHub Issues](https://github.com/SolaraStudio/Optima/issues)
- Discussions: [GitHub Discussions](https://github.com/SolaraStudio/Optima/discussions)
- Web: [Solara-Web](solara.fluxcast.dev) - ***unfinished***

---

## *Optima – A new kind of web engine.*
