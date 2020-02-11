extern crate num_bigint;

use num_bigint::BigUint;
use redox_ecc::curve::WeierstassCurve;
use redox_ecc::field::PrimeField;
use redox_ecc::version;

fn main() {
    println!("{}", version());
    println!("Example!");

    let f = PrimeField::new(BigUint::from(53u64));
    let a = f.elt(3u64);
    let b = f.elt(2u64);

    println!("F: {}", f);
    println!("a: {} ", a);
    println!("b: {} ", b);
    let curve = WeierstassCurve { a, b };
    println!("E: {} ", curve);
    let g0 = curve.new_point(f.elt(46u64), f.elt(3u64), f.elt(1u64));
    let g1 = curve.new_point(f.elt(46u64), f.elt(3u64), f.elt(1u64));
    println!("g0: {} ", g0);
    println!("g1: {} ", g1);
    let g2 = g0 + g1;
    println!("g2: {} ", g2);
}
