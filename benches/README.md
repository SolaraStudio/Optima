# Optima Engine - Benchmarks

Criterion-based benchmarks for the Optima WebView engine.

## Prerequisites

`criterion` must be listed as a dev-dependency in `Cargo.toml`, and each benchmark
must be registered as a `[[bench]]` target with `harness = false`. Example entries
for `Cargo.toml`:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "full"
harness = false
path = "benches/full/full.rs"

[[bench]]
name = "layout"
harness = false
path = "benches/layout/layout.rs"

[[bench]]
name = "parse"
harness = false
path = "benches/parse/parse.rs"

[[bench]]
name = "render"
harness = false
path = "benches/render/render.rs"
```

## Running

```bash
cargo bench
```

Run a single benchmark suite:

```bash
cargo bench --bench full
cargo bench --bench layout
cargo bench --bench parse
cargo bench --bench render
```

HTML reports are generated under `target/criterion/`.

## What each benchmark measures

| Benchmark | Measures |
|-----------|----------|
| `full` | End-to-end lifecycle: create an `Engine`, load a representative HTML document, and produce rendered pixels. |
| `layout` | Repeatedly calls `set_viewport` then `render` on a layout-heavy document (CSS Grid, flexbox, stacked blocks) to measure re-layout cost. |
| `parse` | Generates a large HTML string (~200 sections × 10 rows × 8 columns) and times `load_html` (DOM/CSS parsing) without rendering. |
| `render` | Pre-parses a styled document once, then benchmarks `render()` in isolation to measure rasterization / compositing cost. |

## Note on cdylib crate type

Optima's `Cargo.toml` sets `[lib] crate-type = ["cdylib"]` for FFI consumers.
Criterion benchmarks need a Rust library to link against. If `cargo bench` fails
to resolve the crate, either add `"lib"` to the `crate-type` list (alongside
`"cdylib"`) or use `--lib` to force the staticlib target. The benchmark code
itself targets the public API and will work against either target form.

## Import paths

All benchmarks import from:

- `optima::api::engine::Engine`
- `optima::api::config::EngineConfig`

Adjust these paths if the module layout differs in your build.
