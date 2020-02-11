#[macro_use]
extern crate bencher;
use bencher::benchmark_group;
use bencher::Bencher;

extern crate num_bigint;
use num_bigint::BigUint;
use redox_ecc::curve::WeierstassCurve;
use redox_ecc::field::PrimeField;

fn ec_add(b: &mut Bencher) {
    let f = PrimeField::new(BigUint::from(53u64));
    let aa = f.elt(3u64);
    let bb = f.elt(2u64);
    let curve = WeierstassCurve { a: aa, b: bb };
    let mut g0 = curve.new_point(f.elt(46u64), f.elt(3u64), f.elt(1u64));
    let g1 = curve.new_point(f.elt(46u64), f.elt(3u64), f.elt(1u64));

    b.iter(|| {
        for _ in 0..1000 {
            g0 = &g0 + &g1;
        }
    });
}

benchmark_group!(curve_arith, ec_add);
benchmark_main!(curve_arith);
