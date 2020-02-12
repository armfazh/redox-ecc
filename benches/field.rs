use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use redox_ecc::{P256, P384, P521};

fn arith(c: &mut Criterion) {
    for curve in [&P256, &P384, &P521].iter() {
        let f = curve.get_field();
        let mut x0 = f.elt(15i64);
        let mut x1 = f.elt(15i64);
        let y0 = f.elt(15i64);
        let y1 = f.elt(15i64);

        c.bench(
            format!("{}/fp", curve.name).as_str(),
            Benchmark::new("add", move |b| b.iter(|| x0 = &x0 + &y0))
                .with_function("mul", move |b| b.iter(|| x1 = &x1 * &y1))
                .sample_size(10),
        );
    }
}

criterion_group!(field_bench, arith);
criterion_main!(field_bench);
