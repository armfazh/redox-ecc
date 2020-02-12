extern crate num_bigint;
use crate::num_bigint::ToBigInt;

use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use std::ops::Sub;

use redox_ecc::{P256, P384, P521};

fn arith(c: &mut Criterion) {
    for curve in [&P256, &P384, &P521].iter() {
        let mut g0 = curve.get_generator();
        let mut g1 = curve.get_generator();
        let ec = curve.get_curve();
        let order = &ec.r;
        let k = ec.new_scalar(order.sub(1u32).to_bigint().unwrap());
        c.bench(
            format!("{}/ec", curve.name).as_str(),
            Benchmark::new("add", move |b| b.iter(|| g0 = &g0 + &g0))
                .with_function("mul", move |b| b.iter(|| g1 = &k * &g1))
                .sample_size(10),
        );
    }
}

criterion_group!(curve_bench, arith);
criterion_main!(curve_bench);
