use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use redox_ecc::ellipticcurve::EllipticCurve;
use redox_ecc::instances::{GetCurve, P256, P384, P521};
use redox_ecc::ops::FromFactory;

fn arith(c: &mut Criterion) {
    for id in [P256, P384, P521].iter() {
        let ec = id.get();
        let f = ec.get_field();
        let mut x0 = f.from(-1i64);
        let mut x1 = f.from(-1i64);
        let mut x2 = f.from(-1i64);
        let y0 = f.from(15i64);
        let y1 = f.from(15i64);

        c.bench(
            format!("{}/fp", id).as_str(),
            Benchmark::new("add", move |b| b.iter(|| x0 = &x0 + &y0))
                .with_function("mul", move |b| b.iter(|| x1 = &x1 * &y1))
                .with_function("inv", move |b| b.iter(|| x2 = 1u32 / &x2))
                .sample_size(10),
        );
    }
}

criterion_group!(field_bench, arith);
criterion_main!(field_bench);
