extern crate num_bigint;

use num_bigint::{BigInt, BigUint};
use redox_ecc::curve::WeierstrassCurve;
use redox_ecc::field::PrimeField;
use redox_ecc::version;

fn main() {
    println!("{}", version());
    println!("Example!");

    let f = PrimeField::new(BigUint::from(53u64));
    let a = f.elt(-3i64);
    let b = f.elt(6i64);
    let r = BigUint::from(41u64);
    println!("F: {}", f);
    println!("a: {} ", a);
    println!("b: {} ", b);
    println!("r: {} ", r);
    let curve = WeierstrassCurve {
        f: f.clone(),
        a,
        b,
        r,
    };
    println!("E: {} ", curve);
    let g0 = curve.new_point(f.elt(41i64), f.elt(13i64), f.elt(1i64));
    let g1 = curve.new_point(f.elt(41i64), f.elt(13i64), f.elt(1i64));
    println!("g0: {} ", g0);
    println!("g1: {} ", g1);
    let g2 = g0 + g1;
    println!("g2: {} ", g2);
    let uno = curve.new_scalar(BigInt::from(1153i64));
    let mut g3 = &uno * &g2;
    g3.normalize();
    println!("g3: {} ", g3);
    let mut g4 = g2 * &uno;
    g4.normalize();
    println!("g4: {} ", g4);

    for (i, ki) in uno.left_to_right().enumerate() {
        println!("i: {}, ki: {:?}", i, ki);
    }
    for (i, ki) in uno.right_to_left().enumerate() {
        println!("i: {}, ki: {:?}", i, ki);
    }

    use redox_ecc::P521;
    let ec = P521.get_curve();
    let mut gg = P521.get_generator();
    println!("E: {} ", P521.name);
    println!("E: {} ", ec);
    println!("G: {} ", gg);
    gg = &gg + &gg;
    gg.normalize();
    println!("G: {} ", gg);
}
