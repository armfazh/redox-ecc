#[macro_use]
extern crate bencher;
use bencher::{benchmark_group, Bencher};

extern crate num_bigint;
use crate::num_bigint::ToBigInt;

use std::ops::Sub;

use redox_ecc::CurveID;
use redox_ecc::{P256, P384, P521};

fn add(b: &mut Bencher, curve: &CurveID) {
    let mut g = curve.get_generator();

    b.iter(|| {
        g = &g + &g;
    });
}

fn mul(b: &mut Bencher, curve: &CurveID) {
    let ec = curve.get_curve();
    let mut g = curve.get_generator();
    let order = &ec.r;
    let k = ec.new_scalar(order.sub(1u32).to_bigint().unwrap());
    b.iter(|| {
        g = &k * &g;
    });
}

fn add_p256(b: &mut Bencher) {
    add(b, &P256)
}
fn add_p384(b: &mut Bencher) {
    add(b, &P384)
}
fn add_p521(b: &mut Bencher) {
    add(b, &P521)
}
fn mul_p256(b: &mut Bencher) {
    mul(b, &P256)
}
fn mul_p384(b: &mut Bencher) {
    mul(b, &P384)
}
fn mul_p521(b: &mut Bencher) {
    mul(b, &P521)
}
benchmark_group!(
    curve_bench,
    add_p256,
    add_p384,
    add_p521,
    mul_p256,
    mul_p384,
    mul_p521
);
benchmark_main!(curve_bench);
