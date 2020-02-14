extern crate num_bigint;
use crate::num_bigint::BigInt;

use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use redox_ecc::EllipticCurve;
use redox_ecc::{P256, P384, P521};

fn arith(c: &mut Criterion) {
    for curve in [&P256, &P384, &P521].iter() {
        let ec = curve.get_curve();
        let mut g0 = curve.get_generator();
        let mut g1 = g0.clone();
        let k = ec.new_scalar(BigInt::from(-1i32));
        c.bench(
            format!("{}/ec", curve).as_str(),
            Benchmark::new("add", move |b| b.iter(|| g0 = &g0 + &g0))
                .with_function("mul", move |b| b.iter(|| g1 = &k * &g1))
                .sample_size(10),
        );
    }
}

criterion_group!(curve_bench, arith);
criterion_main!(curve_bench);
