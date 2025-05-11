use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shigunaru::{create_computed, Signal};

pub fn signal_benchmark(c: &mut Criterion) {
    c.bench_function("signal_create_and_get", |b| {
        b.iter(|| {
            let signal = Signal::new(black_box(42));
            black_box(*signal.get().borrow())
        })
    });

    c.bench_function("signal_update", |b| {
        let signal = Signal::new(0);
        b.iter(|| {
            for i in 0..100 {
                signal.set(black_box(i));
            }
        })
    });

    c.bench_function("computed_signal", |b| {
        b.iter(|| {
            let base = Signal::new(black_box(5));
            let base_for_computed = base.clone();

            let computed = create_computed(move || *base_for_computed.get().borrow() * 2);

            black_box(computed.value());

            base.set(black_box(10));
            black_box(computed.value());
        })
    });

    c.bench_function("nested_computed", |b| {
        b.iter(|| {
            let a = Signal::new(black_box(1));
            let b = Signal::new(black_box(2));

            let a_for_sum = a.clone();
            let b_for_sum = b.clone();
            let sum =
                create_computed(move || *a_for_sum.get().borrow() + *b_for_sum.get().borrow());

            let sum_for_triple = sum.signal().clone();
            let triple = create_computed(move || *sum_for_triple.get().borrow() * 3);

            black_box(triple.value());

            a.set(black_box(5));
            b.set(black_box(7));

            black_box(triple.value());
        })
    });
}

criterion_group!(benches, signal_benchmark);
criterion_main!(benches);
