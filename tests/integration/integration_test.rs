// ===========================================================================
// Optima — Integration Tests
// ===========================================================================
//
// CDYLIB LIMITATION:
// These integration tests exercise the Engine through its public API to
// verify end-to-end correctness (HTML loading → navigation → rendering).
// Because the crate is `crate-type = ["cdylib"]`, these tests cannot
// currently be compiled by `cargo test`.  To enable them, add `"lib"` to
// the crate-type array in Cargo.toml:
//
//     [lib]
//     crate-type = ["cdylib", "lib"]
//
// After that change, run:
//     cargo test --test integration_test
// ===========================================================================

use optima::api::engine::engine::Engine;
use optima::api::config::config::EngineConfig;

#[test]
fn load_full_html_document_and_check_state() {
    let mut engine = Engine::new(EngineConfig::new());

    let html = r#"<!DOCTYPE html>
<html lang="en">
<head><title>Integration</title></head>
<body>
    <h1>Hello Optima</h1>
    <p>This is a test document.</p>
</body>
</html>"#;

    let result = engine.load_html(html, "https://example.com/test");
    assert!(result.is_ok(), "load_html should succeed for a full document");

    let nav = engine.navigation();
    assert_eq!(nav.current_url(), Some("https://example.com/test"));
    assert!(!nav.is_loading());

    let doc = engine.document();
    assert!(doc.is_some(), "document should be present after loading HTML");
}

#[test]
fn navigation_across_multiple_pages() {
    let mut engine = Engine::new(EngineConfig::new());

    // Page 1
    engine.load_html("<p>Page 1</p>", "https://site.com/page1").unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://site.com/page1"));
    assert!(!engine.navigation().can_go_back());

    // Page 2
    engine.load_html("<p>Page 2</p>", "https://site.com/page2").unwrap();
    assert!(engine.navigation().can_go_back());
    assert!(!engine.navigation().can_go_forward());

    // Page 3
    engine.load_html("<p>Page 3</p>", "https://site.com/page3").unwrap();
    assert!(engine.navigation().can_go_back());
    assert_eq!(engine.navigation().history_count(), 3);

    // Go back twice
    engine.go_back().unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://site.com/page2"));

    engine.go_back().unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://site.com/page1"));

    // Go forward
    engine.go_forward().unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://site.com/page2"));
}

#[test]
fn css_injection_and_style_recalculation() {
    let mut engine = Engine::new(EngineConfig::new());

    engine
        .load_html(
            "<div class=\"box\">Styled content</div>",
            "https://example.com/css",
        )
        .unwrap();

    // Inject multiple stylesheets
    engine.inject_css(".box { width: 100px; height: 100px; }").unwrap();
    engine.inject_css(".box { background-color: blue; }").unwrap();
    engine.inject_css("div { font-family: sans-serif; }").unwrap();

    engine.recalculate_styles();

    // Should not panic and document should still be valid
    let doc = engine.document();
    assert!(doc.is_some());
}

#[test]
fn viewport_resize_and_render() {
    let mut engine = Engine::new(EngineConfig::new());
    engine.load_html("<p>Resize test</p>", "https://example.com").unwrap();

    // Desktop viewport
    engine.set_viewport(1920, 1080);
    let desktop_output = engine.render();

    // Mobile viewport
    engine.set_viewport(375, 812);
    let mobile_output = engine.render();

    // Both should produce a Vec<u8> (currently empty, but no panic)
    let _ = desktop_output;
    let _ = mobile_output;
}

#[test]
fn reload_restores_same_url() {
    let mut engine = Engine::new(EngineConfig::new());
    engine.load_url("https://example.com/reload").unwrap();
    let original_url = engine.navigation().current_url().map(String::from);

    engine.reload().unwrap();
    assert_eq!(engine.navigation().current_url(), original_url.as_deref());
}

#[test]
fn start_stop_lifecycle() {
    let mut engine = Engine::new(EngineConfig::new());
    engine.load_html("<p>Lifecycle</p>", "https://example.com").unwrap();

    engine.start();
    assert!(!engine.navigation().is_loading());

    engine.stop();
    assert!(!engine.navigation().is_loading());
}

#[test]
fn tick_after_each_operation() {
    let mut engine = Engine::new(EngineConfig::new());

    engine.load_html("<p>Tick test</p>", "https://example.com").unwrap();
    engine.tick();

    engine.inject_css("p { color: green; }").unwrap();
    engine.tick();

    engine.set_viewport(800, 600);
    engine.tick();

    engine.recalculate_styles();
    engine.tick();

    let _ = engine.render();
    engine.tick();
}

#[test]
fn config_customization_reflected() {
    let cfg = EngineConfig::new()
        .with_user_agent("OptimaTest/1.0")
        .with_javascript(false)
        .with_css(true)
        .with_images(false)
        .with_font_size(14);

    let engine = Engine::new(cfg);

    let c = engine.config();
    assert_eq!(c.user_agent, "OptimaTest/1.0");
    assert!(!c.javascript_enabled);
    assert!(c.css_enabled);
    assert!(!c.images_enabled);
    assert_eq!(c.default_font_size, 14);
}
