use criterion::{criterion_group, criterion_main, Criterion};

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
        let mut group = c.benchmark_group(format!("{}/fp", id).as_str());
        group.sample_size(10);
        group.bench_function("add", move |b| b.iter(|| x0 = &x0 + &y0));
        group.bench_function("mul", move |b| b.iter(|| x1 = &x1 * &y1));
        group.bench_function("inv", move |b| b.iter(|| x2 = 1u32 / &x2));
        group.finish();
    }
}

criterion_group!(field_bench, arith);
criterion_main!(field_bench);
