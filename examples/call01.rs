extern crate num_bigint;

// use num_bigint::{BigInt, BigUint};

use redox_ecc::edwards;
use redox_ecc::edwards::EDWARDS448;
use redox_ecc::version;
use redox_ecc::weierstrass;
use redox_ecc::weierstrass::P256;

fn main() {
    println!("{}", version());
    println!("Example!");
    /*
    let f = Fp::create(BigUint::from(53u64));
    let a = f.from(-3);
    let b = f.from(6);
    let r = BigUint::from(41u64);
    println!("F: {}", f);
    println!("a: {} ", a);
    println!("b: {} ", b);
    println!("r: {} ", r);
    let curve = weierstrass::Curve { f, a, b, r };
    println!("E: {} ", curve);
    let g0 = curve.new_point(weierstrass::Coordinates {
        x: curve.f.from(41u64),
        y: curve.f.from(13u64),
        z: curve.f.one(),
    });
    let g1 = curve.new_point(weierstrass::Coordinates {
        x: curve.f.from(41u64),
        y: curve.f.from(13u64),
        z: curve.f.one(),
    });
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

    for (i, ki) in uno.iter_lr().enumerate() {
        println!("i: {}, ki: {:?}", i, ki);
    }
    for (i, ki) in uno.iter_rl().enumerate() {
        println!("i: {}, ki: {:?}", i, ki);
    }
    */

    let ec = weierstrass::Curve::from(P256);
    println!("E: {} ", P256);
    println!("E: {} ", ec);
    let ec = edwards::Curve::from(EDWARDS448);
    println!("E: {} ", EDWARDS448);
    println!("E: {} ", ec);
    // let mut gg = ec.get_generator();
    // println!("G: {} ", gg);
    // gg = &gg + &gg;
    // gg.normalize();
    // println!("G: {} ", gg);
}
