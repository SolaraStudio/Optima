# Optima Examples

Because the crate is built as a `cdylib` (shared library for Android/FFI),
these examples are **desktop reference programs** that show how to use the
`Engine` and `EngineConfig` API. They are not wired into `Cargo.toml` as
`[[example]]` entries — see the top-level config if you want `cargo run --example`.

| File | Description |
|------|-------------|
| `basic/basic.rs` | Minimal usage: create an `Engine`, load HTML, set viewport, render, tick. |
| `advanced/advanced.rs` | Builder-configured `Engine`, URL loading, CSS injection, navigation, multi-frame render. |
| `benchmarks/bench.rs` | Times `load_html` + `render` over N iterations, prints throughput. Pass iteration count as CLI arg. |
| `android/README.md` | Build instructions for the Android AAR and Kotlin JNI usage guide. |
