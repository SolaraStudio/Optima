# Optima Test Suite

## Layout

```
tests/
├── README.md                        # This file
├── unit/
│   └── mod.rs                       # Unit-style tests for Engine API
├── integration/
│   └── integration_test.rs          # End-to-end integration tests
├── e2e/
│   └── e2e.rs                       # High-level scenario flow tests
├── benchmarks/
│   └── benchmark.rs                 # Timing-based performance tests
└── resources/
    ├── README.md                    # Fixture directory documentation
    ├── sample.html                  # Sample HTML document fixture
    └── sample.css                   # Sample CSS stylesheet fixture
```

## Running Tests

```bash
# Run all non-ignored tests (unit, integration, e2e):
cargo test

# Run a specific test file:
cargo test --test integration_test
cargo test --test e2e

# Run unit tests (requires lib target):
cargo test --lib

# Run benchmarks (ignored by default):
cargo test --test benchmark -- --ignored

# Run benchmarks with output visible:
cargo test --test benchmark -- --ignored --nocapture

# Run a single named test:
cargo test scenario_browsing_session
```

## Important: cdylib Limitation

The crate is currently declared as `crate-type = ["cdylib"]` in
`Cargo.toml`.  Cargo's integration test harness compiles each file in
`tests/` as a separate crate that links against the **lib** target.  A
`cdylib` target cannot be used as a dependency by integration tests.

**As a result, `cargo test` will fail until the crate also exposes a lib
target.**  To fix this, add `"lib"` to the crate-type array:

```toml
[lib]
crate-type = ["cdylib", "lib"]
```

The test files are written as syntactically valid Rust that will compile
correctly once the lib target is available.  They use only `std` imports
and reference real types from the `optima` crate's public API.

## Test Categories

### Unit Tests (`tests/unit/mod.rs`)
Fine-grained tests for individual Engine methods: construction,
`load_html`, `load_url`, `inject_css`, `set_viewport`, `render`,
navigation state, tick, dispatch, and reload.

### Integration Tests (`tests/integration/integration_test.rs`)
Multi-step tests that exercise the Engine through complete workflows:
loading HTML documents, navigating between pages, injecting CSS, resizing
viewports, and verifying final state.

### E2E Scenario Tests (`tests/e2e/e2e.rs`)
Story-driven tests simulating real user sessions: browsing a site,
using a single-page app, stress-cycling through many pages, and
accumulating CSS then reloading.

### Benchmarks (`tests/benchmarks/benchmark.rs`)
Performance sanity checks using `std::time::Instant`.  These are
`#[ignore]`-d so they do not run on normal `cargo test`.  They are not
statistical benchmarks — use `criterion` for rigorous profiling.

## Test Resources (`tests/resources/`)
Fixture files (HTML, CSS, fonts, images) used by tests.  See
`tests/resources/README.md` for details on adding new fixtures.
