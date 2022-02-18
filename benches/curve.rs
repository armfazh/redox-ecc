extern crate num_bigint;
use crate::num_bigint::BigInt;

use criterion::{criterion_group, criterion_main, Criterion};

use redox_ecc::ellipticcurve::EllipticCurve;
use redox_ecc::instances::{GetCurve, P256, P384, P521};

fn arith(c: &mut Criterion) {
    for id in [P256, P384, P521].iter() {
        let ec = id.get();
        let mut g0 = ec.get_generator();
        let mut g1 = g0.clone();
        let k = ec.new_scalar(BigInt::from(-1));
        let mut group = c.benchmark_group(format!("{}/ec", id).as_str());
        group.sample_size(10);
        group.bench_function("add", move |b| b.iter(|| g0 = &g0 + &g0));
        group.bench_function("mul", move |b| b.iter(|| g1 = &k * &g1));
        group.finish();
    }
}

criterion_group!(curve_bench, arith);
criterion_main!(curve_bench);
