# User Guide: Embedding Optima

This guide explains how to integrate Optima into your Android application.

---

## Overview

Optima is compiled to `liboptima.so` and packaged as an Android AAR (Android Archive) library. You interact with it through the Kotlin `OptimaEngine` class, which wraps the native JNI bridge.

---

## Prerequisites

- Android Studio with Gradle
- Android SDK, compileSdk 36
- Min SDK 24 (Android 7.0+)
- NDK (for building from source)
- JDK 17

---

## Adding Optima to Your Project

### Option 1: Pre-built AAR

1. Obtain the AAR file (from a release or by building).
2. Place it in your project's `libs/` directory.
3. Add to your `build.gradle.kts`:

```kotlin
dependencies {
    implementation(files("libs/optima-release.aar"))
    implementation("androidx.core:core-ktx:1.15.0")
}
```

### Option 2: Build from Source

1. Clone the Optima repository.
2. Build the AAR:

```bash
./scripts/android/build-all.sh
```

Or via Gradle:

```bash
cd android
./gradlew buildRustAll assembleRelease
```

3. Find the AAR at `android/app/build/outputs/aar/optima-release.aar`.
4. Copy it to your app project.

### Option 3: Maven (GitHub Packages)

If published to GitHub Packages:

```kotlin
repositories {
    maven {
        url = uri("https://maven.pkg.github.com/SolaraStudio/Optima")
        credentials {
            username = project.findProperty("gpr.user") as String? ?: System.getenv("GITHUB_ACTOR")
            password = project.findProperty("gpr.key") as String? ?: System.getenv("GITHUB_TOKEN")
        }
    }
}

dependencies {
    implementation("org.optima:optima:0.150.10-SNAPSHOT")
}
```

---

## Basic Usage

### Loading the Native Library

The native library is loaded automatically when you first use `OptimaEngine`. The `companion object` calls `System.loadLibrary("optima")`.

### Creating an Engine

```kotlin
val engine = OptimaEngine()
```

This allocates a Rust `Engine` instance and stores its pointer.

### Loading HTML Content

```kotlin
val html = """
    <!DOCTYPE html>
    <html>
    <head>
        <style>
            body { font-family: sans-serif; margin: 0; padding: 20px; }
            h1 { color: #333; }
            p { color: #666; }
        </style>
    </head>
    <body>
        <h1>Hello from Optima!</h1>
        <p>This is rendered by the Optima WebView engine.</p>
    </body>
    </html>
""".trimIndent()

engine.loadHtml(html)
```

### Injecting CSS

```kotlin
val css = "body { background-color: #f0f0f0; }"
engine.loadCss(css)
```

### Rendering

```kotlin
engine.render()
```

This triggers the rendering pipeline. The engine processes the DOM, applies styles, performs layout, and renders to the GPU.

### Cleanup

```kotlin
// When done with the engine
engine.destroy()
```

**Always call `destroy()`** to free the native Rust memory. The engine is not garbage-collected by the JVM — you must explicitly release it.

---

## Complete Example

```kotlin
class MainActivity : AppCompatActivity() {
    private var engine: OptimaEngine? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        engine = OptimaEngine()

        val html = """
            <html>
            <body>
                <h1>My App</h1>
                <p>Rendered by Optima v0.150.10</p>
            </body>
            </html>
        """.trimIndent()

        engine?.loadHtml(html)
        engine?.loadCss("body { font-size: 18px; }")
        engine?.render()
    }

    override fun onDestroy() {
        engine?.destroy()
        engine = null
        super.onDestroy()
    }
}
```

---

## System Font Access

Optima can discover system fonts on Android:

```kotlin
val fonts: Map<String, String> = SystemFontHelper.getSystemFonts()
// Returns: { "Roboto" → "/system/fonts/Roboto-Regular.ttf", ... }
```

This scans `/system/fonts`, `/system/fonts/googlefonts`, `/product/fonts`, and `/vendor/fonts` for `.ttf` and `.otf` files.

---

## Architecture Integration

### Using with SurfaceView/TextureView

For rendering to a surface, integrate with the Android surface system:

1. Create a `SurfaceView` or `TextureView` in your layout.
2. Pass the surface to the native layer for wgpu rendering.
3. Call `engine.render()` on each frame or when content changes.

### Lifecycle Management

Optima's `AndroidLifecycle` tracks Activity lifecycle states. Ensure you:

- Call `engine.destroy()` in `onDestroy()`.
- Pause rendering when the Activity is paused/stopped.
- Resume when the Activity is resumed.

### Threading

- The engine is single-threaded (DOM access uses `Rc<RefCell>`).
- Network requests run on background threads via Tokio.
- Audio/video decode runs on dedicated threads.
- Keep engine operations on the main thread.

---

## Build Configuration

### Gradle Tasks

| Task | Description |
|------|-------------|
| `buildRustAll` | Runs `build-all.sh` to compile Rust for all ABIs |
| `copyRustLibs` | Copies pre-built `.so` files to `jniLibs/` |
| `assembleRelease` | Builds the release AAR |

### ABI Support

Optima supports four Android ABIs:

- `arm64-v8a` (64-bit ARM, most devices)
- `armeabi-v7a` (32-bit ARM, older devices)
- `x86` (32-bit x86, emulators)
- `x86_64` (64-bit x86, emulators)

### Publishing

The AAR is published to GitHub Packages with version `0.150.10-{SNAPSHOT|RELEASE}`.

Set environment variables for publishing:

```bash
export GITHUB_ACTOR=your-username
export GITHUB_TOKEN=your-token
export OPTIMA_VERSION=0.150.10
export VERSION_SUFFIX=RELEASE
```

---

## Troubleshooting

### `UnsatisfiedLinkError: no optima in path`

Ensure `liboptima.so` is in the correct `jniLibs` directory for your ABI. Check that `System.loadLibrary("optima")` is called before creating an `OptimaEngine`.

### Rendering shows blank

1. Ensure you've called `engine.loadHtml(html)` before `engine.render()`.
2. Check that the HTML content is valid.
3. Verify the viewport size is appropriate.

### Memory leaks

Always call `engine.destroy()` when done. The Rust engine allocates heap memory that is not managed by the JVM garbage collector.

### Font issues

Use `SystemFontHelper.getSystemFonts()` to verify which fonts are available. Pass CSS `@font-face` rules to load custom fonts.
