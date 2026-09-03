use criterion::{criterion_group, criterion_main, Criterion};

// Import path: `optima::api::engine::Engine` / `optima::api::config::EngineConfig`
use optima::api::config::EngineConfig;
use optima::api::engine::Engine;

const RENDER_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { background: #fff; }
    .banner { width: 100%; height: 200px; background: linear-gradient(135deg, #667eea, #764ba2);
              display: flex; align-items: center; justify-content: center; color: #fff;
              font-size: 32px; font-weight: bold; }
    .grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; padding: 24px; }
    .card { background: #fafafa; border: 1px solid #ddd; border-radius: 8px;
            padding: 20px; min-height: 160px; }
    .card h3 { margin-bottom: 8px; }
    .card p { color: #555; line-height: 1.5; }
  </style>
</head>
<body>
  <div class="banner">Optima Render Benchmark</div>
  <div class="grid">
    <div class="card"><h3>Card 1</h3><p>Render benchmark with styled cards that produce non-trivial pixel output.</p></div>
    <div class="card"><h3>Card 2</h3><p>Each card contributes gradients, borders, and text to the final pixel buffer.</p></div>
    <div class="card"><h3>Card 3</h3><p>The render call produces a raw RGBA Vec&lt;u8&gt; from the composited layout tree.</p></div>
    <div class="card"><h3>Card 4</h3><p>Grid layout ensures the compositor processes multiple columns and rows.</p></div>
    <div class="card"><h3>Card 5</h3><p>Borders and rounded corners add overdraw to stress the rasterizer.</p></div>
    <div class="card"><h3>Card 6</h3><p>Final card in the 3x2 grid layout for the render benchmark.</p></div>
  </div>
</body>
</html>"#;

fn bench_render(c: &mut Criterion) {
    let mut engine = Engine::new(EngineConfig::new());
    engine.set_viewport(1920, 1080);
    engine.load_html(RENDER_HTML, "https://bench.example.com/render").unwrap();

    c.bench_function("render_pixels", |b| {
        b.iter(|| {
            let pixels: Vec<u8> = engine.render();
            pixels
        });
    });
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
