use std::env;
use std::time::Instant;

use optima::api::config::EngineConfig;
use optima::api::engine::Engine;

fn main() {
    let iterations: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let config = EngineConfig::new();
    let mut engine = Engine::new(config);

    let html = "<h1>Benchmark Page</h1><p>Loaded for benchmarking.</p>";
    let base_url = "https://bench.local";

    engine.set_viewport(1280, 720);

    // warm up
    engine.load_html(html, base_url).unwrap();
    engine.tick();
    let _ = engine.render();

    let start = Instant::now();

    for i in 0..iterations {
        engine.load_html(html, base_url).unwrap();
        engine.tick();
        let _pixels: Vec<u8> = engine.render();

        if (i + 1) % 10 == 0 {
            println!("completed {}/{} iterations", i + 1, iterations);
        }
    }

    let elapsed = start.elapsed();
    let avg_us = elapsed.as_micros() as f64 / iterations as f64;

    println!("\n--- Benchmark Results ---");
    println!("iterations : {iterations}");
    println!("total time : {:.3?}", elapsed);
    println!("avg / iter : {avg_us:.1} us");
}
