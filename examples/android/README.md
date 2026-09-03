# Optima Android Example

Optima ships as a `cdylib`. On Android the Rust code is compiled into a shared
library (`.so`) and exposed to Kotlin/Java through JNI. The files under
`examples/` are **host/desktop reference code**; the Android integration
requires the JNI glue that lives in `src/android/` and `src/jni/`.

## Building the AAR

1. **Native library** – build every ABI with the helper script:

   ```sh
   scripts/android/build-all.sh
   ```

   This calls `cargo ndk` for each target (`arm64-v8a`, `armeabi-v7a`,
   `x86_64`, `x86`) and places the outputs where Gradle expects them.

2. **Gradle package** – assemble the release AAR:

   ```sh
   ./gradlew assembleRelease
   ```

   The resulting `.aar` is in `app/build/outputs/aar/`.

## Kotlin Usage

```kotlin
package org.optima.example

import org.optima.OptimaEngine

class OptimaBridge {

    companion object {
        init {
            System.loadLibrary("optima")
        }
    }

    private var handle: Long = 0L

    fun init() {
        handle = OptimaEngine.nativeInit()
    }

    fun loadHtml(html: String, baseUrl: String) {
        OptimaEngine.nativeLoadHtml(handle, html, baseUrl)
    }

    fun loadCss(css: String) {
        OptimaEngine.nativeLoadCss(handle, css)
    }

    fun render(): ByteArray {
        return OptimaEngine.nativeRender(handle)
    }

    fun destroy() {
        OptimaEngine.nativeDestroy(handle)
        handle = 0L
    }
}
```

### JNI methods implemented in Rust (`src/android/`)

| Kotlin call                | Rust entry point         |
|----------------------------|--------------------------|
| `nativeInit()`             | creates `Engine` + config |
| `nativeLoadHtml(h, html, url)` | calls `Engine::load_html` |
| `nativeLoadCss(h, css)`    | calls `Engine::inject_css` |
| `nativeRender(h)`          | calls `Engine::render` → `Vec<u8>` |
| `nativeDestroy(h)`         | drops the `Engine`        |

### Typical Activity lifecycle

```kotlin
class EngineActivity : AppCompatActivity() {

    private lateinit var bridge: OptimaBridge

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        bridge = OptimaBridge()
        bridge.init()
        bridge.loadHtml("<h1>Hello</h1>", "https://example.com")
    }

    override fun onDestroy() {
        bridge.destroy()
        super.onDestroy()
    }
}
```

> **Note:** For a complete working app you still need a `SurfaceView` or
> `TextureView` to display the `Vec<u8>` pixels returned by `nativeRender`,
> plus a render loop that calls it every frame. The snippet above shows only
> the JNI bridge pattern.
