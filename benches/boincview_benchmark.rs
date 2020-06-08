use boincview::LocalDuration;
use chrono::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn format_duration(n: i64) {
    let duration = Duration::seconds(n);

    duration.formatted(Some("d h:m:s".to_string()));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("format_duration (d h:m:s) - 123456", |b| b.iter(|| format_duration(black_box(123456))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
