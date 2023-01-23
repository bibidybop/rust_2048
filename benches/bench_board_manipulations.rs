use criterion::{black_box, criterion_group, criterion_main, Criterion};
use game_2048::board_manipulations::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let tiles = [0, 2, 4, 8, 0, 2, 2, 8, 0, 0, 0, 0, 16, 0, 0, 16];
    // c.bench_function("LEFT", |b| b.iter(|| left(black_box(&tiles))));
    c.bench_function("NEXT FIELDS", |b| {
        b.iter(|| next_tiles_after_spawn(black_box(&tiles)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
