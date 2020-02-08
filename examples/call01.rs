extern crate num_bigint;

use num_bigint::BigUint;
use redox_ecc::curve::WeierstassCurve;
use redox_ecc::field::new_field;
use redox_ecc::version;

fn main() {
    println!("{}", version());
    println!("Example!");

    let f = &new_field(BigUint::from(53u64));
    let a = f.elt(3u64);
    let b = f.elt(2u64);

    println!("F: {}", f);
    println!("a: {} ", a);
    println!("b: {} ", b);
    let curve = WeierstassCurve { f, a, b };
    println!("E: {} ", curve);
    let g = curve.new_point(f.elt(46u64), f.elt(3u64), f.elt(1u64));
    println!("G: {} ", g);
}
