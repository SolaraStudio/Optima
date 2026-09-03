use criterion::{criterion_group, criterion_main, Criterion};

// Import path: `optima::api::engine::Engine` / `optima::api::config::EngineConfig`
use optima::api::config::EngineConfig;
use optima::api::engine::Engine;

fn build_large_html() -> String {
    let mut html = String::from(
        "<!DOCTYPE html><html><head><style>\
         body{font-family:sans-serif;margin:0;padding:0}\
         .row{display:flex;gap:8px;margin:4px 0}\
         .cell{padding:6px 10px;background:#f0f0f0;border:1px solid #ccc;flex:1}\
         .header{background:#333;color:#fff;padding:10px}\
         .sidebar{width:200px;background:#eee;padding:10px}\
         .content{flex:1;padding:10px}\
         .footer{background:#222;color:#aaa;padding:10px;text-align:center}\
         </style></head><body>",
    );

    for section in 0..200 {
        html.push_str(&format!(
            "<div class=\"row\"><div class=\"header\">Section {section}</div></div>",
        ));
        for row in 0..10 {
            html.push_str("<div class=\"row\">");
            for col in 0..8 {
                html.push_str(&format!(
                    "<div class=\"cell\">s{section}r{row}c{col} Lorem ipsum dolor sit amet \
                     consectetur adipiscing elit</div>"
                ));
            }
            html.push_str("</div>");
        }
    }

    html.push_str("<div class=\"footer\">End of document</div></body></html>");
    html
}

fn bench_parse(c: &mut Criterion) {
    let html = build_large_html();

    c.bench_function("load_html_large_document", |b| {
        b.iter_batched(
            || {
                let config = EngineConfig::new();
                Engine::new(config)
            },
            |mut engine| {
                engine.load_html(&html, "https://bench.example.com/parse").unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
