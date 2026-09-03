// ===========================================================================
// Optima — End-to-End Scenario Tests
// ===========================================================================
//
// CDYLIB LIMITATION:
// Same as other test files: this crate currently only exposes a cdylib
// target.  Add `"lib"` to crate-type in Cargo.toml to enable:
//     cargo test --test e2e
// ===========================================================================

use optima::api::engine::engine::Engine;
use optima::api::config::config::EngineConfig;

/// Scenario: User opens a page, navigates to a second page, goes back,
/// then forwards — verifying the full navigation stack.
#[test]
fn scenario_browsing_session() {
    let mut engine = Engine::new(EngineConfig::new());

    // Step 1 — User opens the homepage
    let home_html = r#"<html><body>
        <nav><a href="/about">About</a></nav>
        <h1>Welcome</h1>
    </body></html>"#;
    engine.load_html(home_html, "https://mysite.com/").unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://mysite.com/"));

    // Step 2 — User clicks "About"
    let about_html = r#"<html><body>
        <nav><a href="/">Home</a></nav>
        <h1>About Us</h1>
        <p>We build browsers.</p>
    </body></html>"#;
    engine.load_html(about_html, "https://mysite.com/about").unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://mysite.com/about"));
    assert!(engine.navigation().can_go_back());

    // Step 3 — User clicks "Home"
    engine.load_html(home_html, "https://mysite.com/").unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://mysite.com/"));

    // Step 4 — User goes back to About
    engine.go_back().unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://mysite.com/about"));

    // Step 5 — User goes forward to Home
    engine.go_forward().unwrap();
    assert_eq!(engine.navigation().current_url(), Some("https://mysite.com/"));
}

/// Scenario: A page with inline CSS is loaded, styles are injected at
/// runtime, and the engine renders without error.
#[test]
fn scenario_styled_page_rendering() {
    let mut engine = Engine::new(EngineConfig::new());
    engine.set_viewport(1280, 720);

    let html = r#"<!DOCTYPE html>
<html>
<head>
    <style>body { margin: 0; font-family: sans-serif; }</style>
</head>
<body>
    <header style="background: #333; color: white;">
        <h1>Styled Page</h1>
    </header>
    <main>
        <p>This page has initial styling.</p>
    </main>
</body>
</html>"#;

    engine.load_html(html, "https://styled.example.com").unwrap();

    // Dynamically inject additional CSS
    engine.inject_css("header { padding: 16px; }").unwrap();
    engine.inject_css("main { padding: 24px; }").unwrap();

    engine.recalculate_styles();
    engine.tick();

    let pixels = engine.render();
    let _ = pixels;

    assert!(!engine.navigation().is_loading());
}

/// Scenario: Simulate a single-page application (SPA) that only changes
/// content via load_html (no full page reload).
#[test]
fn scenario_spa_navigation() {
    let mut engine = Engine::new(EngineConfig::new());

    // Initial page load
    engine.load_html("<div id=\"app\">Home View</div>", "https://spa.app/").unwrap();

    // SPA route change — no full URL change expected in a real SPA,
    // but we use the engine API as-is.
    engine.load_html("<div id=\"app\">Profile View</div>", "https://spa.app/#/profile").unwrap();
    assert_eq!(
        engine.navigation().current_url(),
        Some("https://spa.app/#/profile")
    );

    // Another route
    engine.load_html("<div id=\"app\">Settings View</div>", "https://spa.app/#/settings").unwrap();
    assert_eq!(
        engine.navigation().current_url(),
        Some("https://spa.app/#/settings")
    );
}

/// Scenario: Multiple CSS injections followed by a full reload.
#[test]
fn scenario_css_accumulation_and_reload() {
    let mut engine = Engine::new(EngineConfig::new());

    engine.load_html("<p>Content</p>", "https://example.com").unwrap();

    // Inject many stylesheets
    for i in 0..10 {
        engine
            .inject_css(&format!(".rule{} {{ value: {}; }}", i, i))
            .unwrap();
    }

    engine.recalculate_styles();

    // Reload should reset the page but keep working
    engine.reload().unwrap();
    assert!(!engine.navigation().is_loading());

    let doc = engine.document();
    assert!(doc.is_some());
}

/// Scenario: Stress the engine with rapid load/viewport/render cycles.
#[test]
fn scenario_rapid_cycling() {
    let mut engine = Engine::new(EngineConfig::new());

    for i in 0..50 {
        let html = format!("<p>Cycle {}</p>", i);
        engine.load_html(&html, &format!("https://test.dev/{}", i)).unwrap();
        engine.set_viewport(800 + i, 600 + i);
        let _ = engine.render();
        engine.tick();
    }

    assert_eq!(
        engine.navigation().current_url(),
        Some("https://test.dev/49")
    );
    assert_eq!(engine.navigation().history_count(), 50);
}
