extern crate num_bigint;

use crypto::sha2::Sha256;
use std::convert::From;

// use num_bigint::BigUint;
// use num_integer::Integer;

// use redox_ecc::edwards;
// use redox_ecc::edwards::EDWARDS448;
// use redox_ecc::field::{FpElt, PrimeField};
// use redox_ecc::montgomery;
// use redox_ecc::montgomery::CURVE25519;
use redox_ecc::version;
use redox_ecc::weierstrass;
// use redox_ecc::weierstrass::ProyCoordinates;
// use redox_ecc::h2c;
use redox_ecc::h2c::EncodeToCurve;

use redox_ecc::h2c::HashToField;
use redox_ecc::weierstrass::P256;
use redox_ecc::weierstrass::{P256_SHA256_SSWU_NU_, P384_SHA512_SSWU_NU_, P521_SHA512_SSWU_NU_};
use redox_ecc::weierstrass::{P256_SHA256_SSWU_RO_, P384_SHA512_SSWU_RO_, P521_SHA512_SSWU_RO_};
use redox_ecc::weierstrass::{P256_SHA256_SVDW_NU_, P384_SHA512_SVDW_NU_, P521_SHA512_SVDW_NU_};
use redox_ecc::weierstrass::{P256_SHA256_SVDW_RO_, P384_SHA512_SVDW_RO_, P521_SHA512_SVDW_RO_};
use redox_ecc::EllipticCurve;

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

    // println!("N: {} ", &a);
    // println!("N: {} ", &b);
    // println!("N: {} ", b.sqrt());

    let ec = weierstrass::Curve::from(P256);
    let f = ec.get_field();

    let msg = "hola".as_bytes();
    let dst = "mundo".as_bytes();

    let a = f.hash(Sha256::new(), msg, dst, 0u8, 48usize);
    println!("a: {} ", f);
    println!("a: {} ", a);
    let msg = "hola".as_bytes();
    let dst = "mundo".as_bytes();

    macro_rules! x {
        ($suite:ident ) => {
            let enc = $suite.new(dst);
            let mut p = enc.hash(msg, dst);
            p.normalize();
            println!("enc: {} {} ", $suite, p);
        };
    }
    x!(P256_SHA256_SSWU_NU_);
    x!(P256_SHA256_SSWU_RO_);
    x!(P256_SHA256_SVDW_NU_);
    x!(P256_SHA256_SVDW_RO_);
    x!(P384_SHA512_SSWU_NU_);
    x!(P384_SHA512_SSWU_RO_);
    x!(P384_SHA512_SVDW_NU_);
    x!(P384_SHA512_SVDW_RO_);
    x!(P521_SHA512_SSWU_NU_);
    x!(P521_SHA512_SSWU_RO_);
    x!(P521_SHA512_SVDW_NU_);
    x!(P521_SHA512_SVDW_RO_);

    // println!("N: {} ", P256);
    // let gg = ec.get_generator();
    // let f = ec.get_field();
    // let g2 = ec.new_point(ProyCoordinates {
    //     x: f.from("51317554015454129980312020699350903676485190487572340293004311540924363220810"),
    //     y: f.from("40717246725065776267553779947826866259129181902823445522925495338883756281207"),
    //     z: f.from(1),
    // });
    // let mut g3 = &gg + &g2;
    // g3.normalize();
    // println!("G: {} ", g3);
    // let ec = edwards::Curve::from(EDWARDS448);
    // let gg = ec.get_generator();
    // println!("E: {} ", EDWARDS448);
    // println!("E: {} ", ec);
    // println!("G: {} ", gg);
    // let ec = montgomery::Curve::from(CURVE25519);
    // let gg = ec.get_generator();
    // println!("E: {} ", CURVE25519);
    // println!("E: {} ", ec);
    // println!("G: {} ", gg);
}
