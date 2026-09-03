// ===========================================================================
// Optima — Benchmark-Style Timing Tests
// ===========================================================================
//
// These tests use `std::time::Instant` to measure performance.
// They are marked `#[ignore]` so they do NOT run during normal
// `cargo test` invocations.  Run them with:
//
//     cargo test --test benchmark -- --ignored
//
// CDYLIB LIMITATION:
// Like other integration tests in this directory, these require a lib
// target.  Add `"lib"` to crate-type in Cargo.toml to enable.
//
// These are NOT proper statistical benchmarks (use criterion for that);
// they are quick sanity checks for obvious regressions.
// ===========================================================================

use std::time::Instant;

use optima::api::engine::engine::Engine;
use optima::api::config::config::EngineConfig;

#[test]
#[ignore]
fn bench_engine_construction() {
    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _engine = Engine::new(EngineConfig::new());
    }
    let elapsed = start.elapsed();
    eprintln!(
        "Engine::new x{}: {:?} ({:?} per op)",
        iterations,
        elapsed,
        elapsed / iterations
    );
    // Sanity: creating 10k engines should take under 1 second
    assert!(
        elapsed.as_secs() < 1,
        "Engine construction too slow: {:?}",
        elapsed
    );
}

#[test]
#[ignore]
fn bench_load_html_small() {
    let mut engine = Engine::new(EngineConfig::new());
    let html = "<p>Small</p>";
    let iterations = 5_000;

    let start = Instant::now();
    for i in 0..iterations {
        engine.load_html(html, &format!("https://bench.dev/{}", i)).unwrap();
    }
    let elapsed = start.elapsed();
    eprintln!(
        "load_html (small) x{}: {:?} ({:?} per op)",
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
#[ignore]
fn bench_load_html_large() {
    let mut engine = Engine::new(EngineConfig::new());

    // Build a reasonably large HTML document
    let mut html = String::from("<html><body>");
    for i in 0..1_000 {
        html.push_str(&format!("<div id=\"d{}\">Content paragraph {}</div>", i, i));
    }
    html.push_str("</body></html>");

    let iterations = 100;
    let start = Instant::now();
    for i in 0..iterations {
        engine.load_html(&html, &format!("https://bench.dev/large/{}", i)).unwrap();
    }
    let elapsed = start.elapsed();
    eprintln!(
        "load_html (1k elements) x{}: {:?} ({:?} per op)",
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
#[ignore]
fn bench_inject_css() {
    let mut engine = Engine::new(EngineConfig::new());
    engine.load_html("<p>Styled</p>", "https://bench.dev").unwrap();

    let css = ".a { color: red; } .b { color: blue; } .c { margin: 10px; }";
    let iterations = 10_000;

    let start = Instant::now();
    for _ in 0..iterations {
        engine.inject_css(css).unwrap();
    }
    let elapsed = start.elapsed();
    eprintln!(
        "inject_css x{}: {:?} ({:?} per op)",
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
#[ignore]
fn bench_render_after_load() {
    let mut engine = Engine::new(EngineConfig::new());
    let html = r#"<!DOCTYPE html>
<html><body>
<h1>Bench</h1>
<p>This is a render benchmark page with some content.</p>
<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>
</body></html>"#;

    engine.set_viewport(1920, 1080);
    let iterations = 1_000;

    let start = Instant::now();
    for i in 0..iterations {
        engine.load_html(html, &format!("https://bench.dev/render/{}", i)).unwrap();
        let _ = engine.render();
    }
    let elapsed = start.elapsed();
    eprintln!(
        "load_html + render x{}: {:?} ({:?} per op)",
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
#[ignore]
fn bench_tick() {
    let mut engine = Engine::new(EngineConfig::new());
    engine.load_html("<p>Tick</p>", "https://bench.dev").unwrap();
    engine.inject_css("p { font-size: 14px; }").unwrap();

    let iterations = 50_000;
    let start = Instant::now();
    for _ in 0..iterations {
        engine.tick();
    }
    let elapsed = start.elapsed();
    eprintln!(
        "tick x{}: {:?} ({:?} per op)",
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
#[ignore]
fn bench_navigation_history() {
    let iterations = 5_000;
    let mut engine = Engine::new(EngineConfig::new());

    // Build history
    for i in 0..iterations {
        engine
            .load_url(&format!("https://bench.dev/nav/{}", i))
            .unwrap();
    }

    // Time back-navigation
    let start = Instant::now();
    for _ in 0..iterations {
        engine.go_back().unwrap();
    }
    let back_elapsed = start.elapsed();

    // Time forward-navigation
    let start = Instant::now();
    for _ in 0..iterations {
        engine.go_forward().unwrap();
    }
    let fwd_elapsed = start.elapsed();

    eprintln!(
        "go_back x{}: {:?} | go_forward x{}: {:?}",
        iterations, back_elapsed, iterations, fwd_elapsed
    );
}

#[test]
#[ignore]
fn bench_full_workflow() {
    let iterations = 1_000;
    let start = Instant::now();

    for i in 0..iterations {
        let mut engine = Engine::new(EngineConfig::new());
        engine.set_viewport(1024, 768);
        engine
            .load_html(
                &format!("<h1>Workflow {}</h1><p>Body text</p>", i),
                &format!("https://bench.dev/wf/{}", i),
            )
            .unwrap();
        engine.inject_css("h1 { color: navy; }").unwrap();
        engine.recalculate_styles();
        engine.tick();
        let _ = engine.render();
    }

    let elapsed = start.elapsed();
    eprintln!(
        "full workflow x{}: {:?} ({:?} per op)",
        iterations,
        elapsed,
        elapsed / iterations
    );
}
