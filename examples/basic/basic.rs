use optima::api::config::EngineConfig;
use optima::api::engine::Engine;

fn main() {
    let config = EngineConfig::new();
    let mut engine = Engine::new(config);

    let html = "<h1>Hello from Optima</h1><p>This is a basic example.</p>";
    engine.load_html(html, "https://example.com").expect("failed to load HTML");

    engine.set_viewport(1280, 720);

    loop {
        engine.tick();

        let _pixels: Vec<u8> = engine.render();

        if let Some(doc) = engine.document() {
            let _doc = doc.borrow();
        }

        engine.start();
    }
}
