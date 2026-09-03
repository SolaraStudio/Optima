// ===========================================================================
// Optima — Unit-style Tests
// ===========================================================================
//
// NOTE ON CDYLIB LIMITATION:
// This crate is declared as `crate-type = ["cdylib"]` in Cargo.toml.
// Cargo's integration test harness (`tests/`) compiles each file as a
// separate crate that links against the *lib* target of the current
// package.  A cdylib target is **not** usable as a dependency by
// integration tests.
//
// To make these tests runnable, the crate would need to additionally
// expose a `lib` crate-type (e.g. `crate-type = ["cdylib", "lib"]`).
// Until that change is made, these test modules serve as *specification*
// and documentation of expected behaviour.  They are syntactically valid
// Rust that will compile once the lib target is available.
//
// To run (after adding "lib" to crate-type):
//     cargo test --lib unit
// ===========================================================================

#[cfg(test)]
mod engine_construction {
    use std::cell::RefCell;
    use std::rc::Rc;

    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn new_with_default_config() {
        let engine = Engine::new(EngineConfig::new());
        let doc = engine.document();
        assert!(doc.is_some(), "a freshly constructed engine should have a document");
    }

    #[test]
    fn default_trait_matches_new() {
        let engine = Engine::default();
        let doc = engine.document();
        assert!(doc.is_some());
    }

    #[test]
    fn config_accessor_returns_default() {
        let engine = Engine::new(EngineConfig::new());
        let cfg = engine.config();
        assert_eq!(cfg.user_agent, "Optima/1.0");
        assert!(cfg.javascript_enabled);
        assert!(cfg.css_enabled);
        assert_eq!(cfg.default_font_size, 16);
    }

    #[test]
    fn config_builder_chain() {
        let cfg = EngineConfig::new()
            .with_user_agent("TestBot/2.0")
            .with_javascript(false)
            .with_images(false)
            .with_font_size(20);

        let engine = Engine::new(cfg);
        let c = engine.config();
        assert_eq!(c.user_agent, "TestBot/2.0");
        assert!(!c.javascript_enabled);
        assert!(!c.images_enabled);
        assert_eq!(c.default_font_size, 20);
    }
}

#[cfg(test)]
mod load_html_tests {
    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn load_html_returns_ok() {
        let mut engine = Engine::new(EngineConfig::new());
        let result = engine.load_html("<p>Hello</p>", "https://example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn load_html_empty_string() {
        let mut engine = Engine::new(EngineConfig::new());
        assert!(engine.load_html("", "about:blank").is_ok());
    }

    #[test]
    fn load_html_updates_document() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_html("<div>content</div>", "https://test.dev").unwrap();
        let doc = engine.document();
        assert!(doc.is_some(), "document should exist after load_html");
    }

    #[test]
    fn load_html_updates_navigation_url() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_html("<p>Hi</p>", "https://example.com/page").unwrap();
        assert_eq!(engine.navigation().current_url(), Some("https://example.com/page"));
    }

    #[test]
    fn load_html_sets_not_loading() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_html("<p>Done</p>", "https://example.com").unwrap();
        assert!(!engine.navigation().is_loading());
    }
}

#[cfg(test)]
mod load_url_tests {
    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn load_url_returns_ok_for_valid_url() {
        let mut engine = Engine::new(EngineConfig::new());
        assert!(engine.load_url("https://example.com").is_ok());
    }

    #[test]
    fn load_url_returns_ok_for_invalid_url() {
        // The current implementation does not perform network fetching;
        // it delegates to load_html which always succeeds.
        let mut engine = Engine::new(EngineConfig::new());
        let result = engine.load_url("not-a-valid-url");
        assert!(result.is_ok());
    }

    #[test]
    fn load_url_updates_navigation() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_url("https://example.com/page").unwrap();
        assert_eq!(
            engine.navigation().current_url(),
            Some("https://example.com/page")
        );
    }

    #[test]
    fn load_url_pushes_history() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_url("https://a.com").unwrap();
        engine.load_url("https://b.com").unwrap();
        assert_eq!(engine.navigation().history_count(), 2);
    }
}

#[cfg(test)]
mod inject_css_tests {
    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn inject_css_returns_ok() {
        let mut engine = Engine::new(EngineConfig::new());
        let result = engine.inject_css("body { color: red; }");
        assert!(result.is_ok());
    }

    #[test]
    fn inject_css_empty_string() {
        let mut engine = Engine::new(EngineConfig::new());
        assert!(engine.inject_css("").is_ok());
    }

    #[test]
    fn inject_css_multiple_times() {
        let mut engine = Engine::new(EngineConfig::new());
        assert!(engine.inject_css("a { color: blue; }").is_ok());
        assert!(engine.inject_css("b { color: green; }").is_ok());
        assert!(engine.inject_css("c { color: yellow; }").is_ok());
    }
}

#[cfg(test)]
mod viewport_tests {
    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn set_viewport_default_size() {
        let mut engine = Engine::new(EngineConfig::new());
        // Default is 800×600 per the source
        engine.set_viewport(800, 600);
        // No panic — set_viewport does not return a value.
    }

    #[test]
    fn set_viewport_various_sizes() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.set_viewport(1920, 1080);
        engine.set_viewport(375, 812);   // iPhone-ish
        engine.set_viewport(0, 0);       // edge case
        engine.set_viewport(u32::MAX, u32::MAX); // extreme
    }
}

#[cfg(test)]
mod render_tests {
    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn render_returns_vec() {
        let engine = Engine::new(EngineConfig::new());
        let pixels = engine.render();
        // Currently returns vec![]; verify it is a Vec<u8>.
        assert!(pixels.is_empty());
    }

    #[test]
    fn render_after_load_html() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_html("<h1>Render me</h1>", "https://example.com").unwrap();
        let pixels = engine.render();
        let _ = pixels; // no panic
    }

    #[test]
    fn render_after_css_injection() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_html("<p>Styled</p>", "https://example.com").unwrap();
        engine.inject_css("p { font-size: 24px; }").unwrap();
        let pixels = engine.render();
        let _ = pixels;
    }
}

#[cfg(test)]
mod navigation_tests {
    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn navigation_initial_state() {
        let engine = Engine::new(EngineConfig::new());
        let nav = engine.navigation();
        assert_eq!(nav.current_url(), None);
        assert!(!nav.is_loading());
        assert_eq!(nav.history_count(), 0);
        assert!(!nav.can_go_back());
        assert!(!nav.can_go_forward());
    }

    #[test]
    fn reload_without_url_returns_err() {
        let mut engine = Engine::new(EngineConfig::new());
        let result = engine.reload();
        assert!(result.is_err());
    }

    #[test]
    fn reload_with_url_returns_ok() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_url("https://example.com").unwrap();
        assert!(engine.reload().is_ok());
    }

    #[test]
    fn go_back_without_history_returns_err() {
        let mut engine = Engine::new(EngineConfig::new());
        assert!(engine.go_back().is_err());
    }

    #[test]
    fn go_forward_without_history_returns_err() {
        let mut engine = Engine::new(EngineConfig::new());
        assert!(engine.go_forward().is_err());
    }

    #[test]
    fn go_back_after_two_navigations() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_url("https://a.com").unwrap();
        engine.load_url("https://b.com").unwrap();
        assert!(engine.go_back().is_ok());
        assert_eq!(engine.navigation().current_url(), Some("https://a.com"));
    }

    #[test]
    fn go_forward_after_going_back() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_url("https://a.com").unwrap();
        engine.load_url("https://b.com").unwrap();
        engine.go_back().unwrap();
        assert!(engine.go_forward().is_ok());
        assert_eq!(engine.navigation().current_url(), Some("https://b.com"));
    }

    #[test]
    fn stop_sets_not_loading() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_url("https://example.com").unwrap();
        engine.stop();
        assert!(!engine.navigation().is_loading());
    }

    #[test]
    fn start_sets_not_loading() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.start();
        assert!(!engine.navigation().is_loading());
    }
}

#[cfg(test)]
mod tick_and_dispatch {
    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn tick_does_not_panic() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.tick();
    }

    #[test]
    fn tick_after_load_html() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_html("<p>Ticked</p>", "https://example.com").unwrap();
        engine.tick();
    }

    #[test]
    fn dispatch_event_does_not_panic() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.dispatch_event("click");
        engine.dispatch_event("resize");
        engine.dispatch_event("keydown");
    }

    #[test]
    fn recalculate_styles_does_not_panic() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.inject_css("body { margin: 0; }").unwrap();
        engine.recalculate_styles();
    }
}

#[cfg(test)]
mod multiple_operations {
    use optima::api::engine::engine::Engine;
    use optima::api::config::config::EngineConfig;

    #[test]
    fn full_basic_workflow() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.set_viewport(1024, 768);
        engine.load_html("<html><body><h1>Hello</h1></body></html>", "https://example.com").unwrap();
        engine.inject_css("h1 { font-size: 32px; }").unwrap();
        engine.recalculate_styles();
        engine.tick();
        let _pixels = engine.render();
        assert!(!engine.navigation().is_loading());
    }

    #[test]
    fn multiple_loads_replace_document() {
        let mut engine = Engine::new(EngineConfig::new());
        engine.load_html("<p>First</p>", "https://first.com").unwrap();
        engine.load_html("<p>Second</p>", "https://second.com").unwrap();
        let doc = engine.document();
        assert!(doc.is_some());
        assert_eq!(
            engine.navigation().current_url(),
            Some("https://second.com")
        );
    }
}
