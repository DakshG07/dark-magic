use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline]
fn padding(value_align: usize, offset: usize) -> usize{
    value_align*(offset/value_align) + value_align - offset
    //value_align - (offset % value_align) 
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("padding_one", |b| b.iter(|| padding(black_box(8), black_box(3))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);