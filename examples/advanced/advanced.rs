use std::time::Duration;

use optima::api::config::EngineConfig;
use optima::api::engine::Engine;

fn main() {
    let config = EngineConfig::new()
        .with_user_agent("OptimaAdvanced/1.0")
        .with_javascript(true)
        .with_images(true)
        .with_css(true)
        .with_font_size(18)
        .with_timeout(Duration::from_secs(60));

    let mut engine = Engine::new(config);

    engine.set_viewport(1920, 1080);

    engine.load_url("https://example.com").expect("failed to load URL");

    let css = "
        body { background: #1a1a2e; color: #e0e0e0; font-size: 18px; }
        h1 { color: #0f3460; }
    ";
    engine.inject_css(css).expect("failed to inject CSS");

    let html = "<h1>Styled Content</h1><p>Rendered with Optima.</p>";
    engine.load_html(html, "https://app.local").expect("failed to load HTML");

    engine.tick();

    let _nav = engine.navigation();
    let _config_ref = engine.config();

    engine.dispatch_event("load");

    for frame in 0..5 {
        engine.tick();
        let _pixels: Vec<u8> = engine.render();
        println!("rendered frame {frame}");
    }

    engine.stop();
    engine.start();

    if engine.reload().is_err() {
        println!("nothing to reload");
    }
}
