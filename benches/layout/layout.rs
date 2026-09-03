use criterion::{criterion_group, criterion_main, Criterion};

// Import path: `optima::api::engine::Engine` / `optima::api::config::EngineConfig`
use optima::api::config::EngineConfig;
use optima::api::engine::Engine;

const LAYOUT_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    .grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 8px; padding: 16px; }
    .cell { background: #ddd; height: 120px; display: flex; align-items: center; justify-content: center; font-size: 14px; }
    .flex-wrap { display: flex; flex-wrap: wrap; gap: 4px; padding: 16px; }
    .flex-item { width: 80px; height: 60px; background: #cde; }
    .stack { padding: 16px; }
    .stack > div { margin-bottom: 8px; padding: 12px; background: #edc; border-left: 4px solid #a80; }
  </style>
</head>
<body>
  <div class="grid">
    <div class="cell">1</div><div class="cell">2</div><div class="cell">3</div><div class="cell">4</div>
    <div class="cell">5</div><div class="cell">6</div><div class="cell">7</div><div class="cell">8</div>
    <div class="cell">9</div><div class="cell">10</div><div class="cell">11</div><div class="cell">12</div>
    <div class="cell">13</div><div class="cell">14</div><div class="cell">15</div><div class="cell">16</div>
    <div class="cell">17</div><div class="cell">18</div><div class="cell">19</div><div class="cell">20</div>
    <div class="cell">21</div><div class="cell">22</div><div class="cell">23</div><div class="cell">24</div>
  </div>
  <div class="flex-wrap">
    <div class="flex-item"></div><div class="flex-item"></div><div class="flex-item"></div>
    <div class="flex-item"></div><div class="flex-item"></div><div class="flex-item"></div>
    <div class="flex-item"></div><div class="flex-item"></div><div class="flex-item"></div>
    <div class="flex-item"></div><div class="flex-item"></div><div class="flex-item"></div>
  </div>
  <div class="stack">
    <div>Stack item 1 with some content to measure text layout performance.</div>
    <div>Stack item 2 with additional text that contributes to line-breaking work.</div>
    <div>Stack item 3 with more content for the layout engine to process.</div>
    <div>Stack item 4 with yet more text to keep the layout busy.</div>
    <div>Stack item 5 with final content block.</div>
  </div>
</body>
</html>"#;

fn bench_layout(c: &mut Criterion) {
    let config = EngineConfig::new();
    let mut engine = Engine::new(config);
    engine.load_html(LAYOUT_HTML, "https://bench.example.com/layout").unwrap();

    c.bench_function("set_viewport_and_layout", |b| {
        b.iter(|| {
            engine.set_viewport(1280, 720);
            engine.render()
        });
    });
}

criterion_group!(benches, bench_layout);
criterion_main!(benches);
