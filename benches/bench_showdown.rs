use bytes_diff_showdown as util;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::ThreadRng, thread_rng, Rng};

fn random_bytes(rng: &mut ThreadRng, len: usize) -> Vec<u8> {
    (0..len).map(|_| rng.gen()).collect()
}

fn two_random_byte_vecs_with_same_base(rng: &mut ThreadRng, len: usize) -> (Vec<u8>, Vec<u8>) {
    let half_len = len / 2;
    let base = random_bytes(rng, half_len);
    let suffix1 = random_bytes(rng, half_len);
    let suffix2 = random_bytes(rng, half_len);
    let mut k1 = base.clone();
    k1.extend(suffix1);
    let mut k2 = base;
    k2.extend(suffix2);
    (k1, k2)
}

fn bench_bytes_diff(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut group = c.benchmark_group("bytes_diff_versus");
    group.throughput(Throughput::Elements(1));
    let sizes = [16, 83, 500, 669];
    for size in sizes {
        group.bench_with_input(
            BenchmarkId::new("bytes_diff_original", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_original(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_original_128", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_original_128(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_chunked8", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_chunked8(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_chunked16", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_chunked16(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_chunked32", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_chunked32(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_chunked64", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_chunked64(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_chunked128", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_chunked128(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_naive", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_naive(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_functional", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_functional(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_functional_naive", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_functional_naive(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_bitwise", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_bitwise(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_chunked_original", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_chunked_original(&k1[..], &k2[..]));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_diff_hybrid_original", size),
            &size,
            |b, size| {
                let (k1, k2) = two_random_byte_vecs_with_same_base(&mut rng, *size);
                b.iter(|| util::bytes_diff_hybrid_original(&k1[..], &k2[..]));
            },
        );
    }
}

criterion_group! {
    name = benches_common;
    config = Criterion::default();
    targets = bench_bytes_diff
}

criterion_main!(benches_common);
