// json_fixer/benches/fix_benchmark.rs
use criterion::{criterion_group, criterion_main, Criterion};
use json_fix::fix_json_syntax;

fn benchmark_fix_json_syntax(c: &mut Criterion) {
    let broken_json = r#"{"emotion": "hopeful, \"score": 80}"#;

    c.bench_function("fix_json_string", |b| {
        b.iter(|| {
            let _ = fix_json_syntax(broken_json);
        });
    });
}

criterion_group!(benches, benchmark_fix_json_syntax);
criterion_main!(benches);
