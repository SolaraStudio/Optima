use criterion::{criterion_group, criterion_main, Criterion};

// Import path: `optima::api::engine::Engine` / `optima::api::config::EngineConfig`
use optima::api::config::EngineConfig;
use optima::api::engine::Engine;

const HTML_DOC: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Full Benchmark</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { font-family: sans-serif; line-height: 1.6; }
    .container { max-width: 960px; margin: 0 auto; padding: 1rem; }
    header { background: #222; color: #fff; padding: 2rem; text-align: center; }
    nav { display: flex; gap: 1rem; background: #444; padding: 0.75rem 1rem; }
    nav a { color: #fff; text-decoration: none; }
    main { display: grid; grid-template-columns: 2fr 1fr; gap: 1.5rem; padding: 1.5rem 0; }
    article { background: #f9f9f9; padding: 1.5rem; border-radius: 4px; }
    aside { background: #eee; padding: 1rem; border-radius: 4px; }
    footer { background: #222; color: #aaa; text-align: center; padding: 1rem; margin-top: 2rem; }
    ul { list-style: disc inside; }
    li { margin: 0.25rem 0; }
  </style>
</head>
<body>
  <header><h1>Optima Engine</h1><p>Full benchmark page</p></header>
  <nav>
    <a href="#">Home</a><a href="#">About</a><a href="#">Docs</a><a href="#">GitHub</a>
  </nav>
  <div class="container">
    <main>
      <article>
        <h2>Article Title</h2>
        <p>This is a representative HTML document used to benchmark the full lifecycle of the Optima engine, from construction through parsing, layout, and pixel rendering.</p>
        <ul>
          <li>Engine creation</li>
          <li>HTML parsing</li>
          <li>CSS layout</li>
          <li>Render output</li>
        </ul>
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
      </article>
      <aside>
        <h3>Sidebar</h3>
        <p>Additional layout weight from a sidebar column.</p>
        <ul>
          <li>Widget A</li>
          <li>Widget B</li>
          <li>Widget C</li>
        </ul>
      </aside>
    </main>
  </div>
  <footer>&copy; 2026 SolaraStudio</footer>
</body>
</html>"#;

fn bench_full(c: &mut Criterion) {
    c.bench_function("full_engine_lifecycle", |b| {
        b.iter(|| {
            let config = EngineConfig::new();
            let mut engine = Engine::new(config);
            engine.set_viewport(1280, 720);
            engine.load_html(HTML_DOC, "https://bench.example.com/").unwrap();
            engine.render()
        });
    });
}

criterion_group!(benches, bench_full);
criterion_main!(benches);
