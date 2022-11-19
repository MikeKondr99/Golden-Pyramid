#[allow(unused)]
use std::time::Duration;

use criterion::*;
use pyramid::*;
//use rand::prelude::*;

// number of layers;
const SIZES: [usize; 15] = [
    10_000, 20_000, 30_000, 40_000, 50_000, 100_000, 200_000, 300_000, 400_000, 500_000, 600_000,
    700_000, 800_000, 900_000, 1_000_000,
];

fn gen_data(size: usize) -> Vec<u32> {
    (0..size).map(|_| 0).collect()
}
#[allow(non_snake_case)]
fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pyramid");
    for layers in SIZES.iter() {
        group.throughput(Throughput::Elements(*layers as u64));
        group.bench_with_input(BenchmarkId::new("Simple", layers), layers, |b, &size| {
            let layer1: Vec<_> = gen_data(size);
            let mut rest: Vec<_> = gen_data(size - 1);
            b.iter(|| Simple::algorithm(&layer1[..], &mut rest[..], size));
        });
        group.bench_with_input(
            BenchmarkId::new("Vectorization", layers),
            layers,
            |b, &size| {
                let layer1: Vec<_> = gen_data(size);
                let mut rest: Vec<_> = gen_data(size - 1);
                b.iter(|| Vectorization::algorithm(&layer1[..], &mut rest[..], size));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
