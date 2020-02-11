#[macro_use]
extern crate bencher;
use crate::num_bigint::ToBigInt;
use bencher::benchmark_group;
use bencher::Bencher;
use std::ops::Sub;

extern crate num_bigint;
use num_bigint::BigUint;
use redox_ecc::curve::WeierstrassCurve;
use redox_ecc::field::PrimeField;

fn ec_add(ben: &mut Bencher) {
    let f = &PrimeField::new(BigUint::from(53u64));
    let a = f.elt(-3i64);
    let b = f.elt(5i64);
    let r = BigUint::from(63u64);
    let curve = WeierstrassCurve { f, a, b, r };
    let mut g0 = curve.new_point(&f.elt(22i64), &f.elt(27i64), &f.elt(1i64));
    let g1 = curve.new_point(&f.elt(22i64), &f.elt(27i64), &f.elt(1i64));

    ben.iter(|| {
        g0 = &g0 + &g1;
    });
}

fn ec_mul(ben: &mut Bencher) {
    let f = &PrimeField::new(BigUint::from(53u64));
    let a = f.elt(-3i64);
    let b = f.elt(5i64);
    let r = BigUint::from(63u64);
    let curve = WeierstrassCurve { f, a, b, r };
    let g0 = curve.new_point(&f.elt(22i64), &f.elt(27i64), &f.elt(1i64));
    let order = &curve.r;
    let k = curve.new_scalar(order.sub(1u32).to_bigint().unwrap());

    ben.iter(|| {
        let _ = &k * &g0;
    });
}

benchmark_group!(curve_arith, ec_add, ec_mul);
benchmark_main!(curve_arith);
