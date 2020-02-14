//! This is documentation for the `redox-ecc` crate.
//!
//! The foo crate is meant to be used for bar.

// #![warn(missing_docs)]

extern crate num_bigint;
use num_bigint::{BigInt, BigUint};
use std::str::FromStr;

mod inst;
mod macros;

pub mod field;

pub mod edwards;
pub mod weierstrass;

/// P256 is the NIST P-256 elliptic curve.
pub static P256: CurveID = CurveID(&inst::P256_PARAMS);
/// P384 is the NIST P-384 elliptic curve.
pub static P384: CurveID = CurveID(&inst::P384_PARAMS);
/// P521 is the NIST P-521 elliptic curve.
pub static P521: CurveID = CurveID(&inst::P521_PARAMS);
/// SECP256K1 is a 256-bit elliptic curve knwon as secp256k1.
pub static SECP256K1: CurveID = CurveID(&inst::SECP256K1_PARAMS);
/// EDWARDS25519 is the edwards25519 elliptic curve as specified in RFC-7748.
pub static EDWARDS25519: CurveID = CurveID(&inst::EDWARDS25519_PARAMS);
/// EDWARDS448 is the edwards448 elliptic curve as specified in RFC-7748.
pub static EDWARDS448: CurveID = CurveID(&inst::EDWARDS448_PARAMS);

pub struct CurveID(&'static inst::CurveParams);

impl CurveID {
    pub fn get_field(&self) -> field::Fp {
        field::Fp::create(BigUint::from_str(self.0.p).unwrap())
    }
    pub fn get_curve(&self) -> weierstrass::Curve {
        let f = self.get_field();
        let a = f.from(self.0.a);
        let b = f.from(self.0.b);
        let r = BigUint::from_str(self.0.r).unwrap();
        weierstrass::Curve { f, a, b, r }
    }
    pub fn get_generator(&self) -> weierstrass::Point {
        let e = self.get_curve();
        let p = weierstrass::Coordinates {
            x: e.f.from(self.0.gx),
            y: e.f.from(self.0.gy),
            z: e.f.one(),
        };
        e.new_point(p)
    }
}

impl std::fmt::Display for CurveID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

pub trait FromFactory<T: Sized> {
    fn from(&self, _: T) -> Self;
}

pub type EllipticCurveModel = ();
/// Field is a fabric to instante a finite field. The type `Elt` determines the type of its elements.
pub trait Field {
    type Elt;
    fn new(&self, _: BigInt) -> Self::Elt;
    fn zero(&self) -> Self::Elt;
    fn one(&self) -> Self::Elt;
}
/// Curve trait allows to implement elliptic curve operations.
pub trait EllipticCurve: PartialEq {
    type Field;
    type Point;
    type Coordinates;
    type Scalar;
    fn new_point(&self, _: Self::Coordinates) -> Self::Point;
    fn new_scalar(&self, _: BigInt) -> Self::Scalar;
    fn identity(&self) -> Self::Point;
    fn is_on_curve(&self, _: &Self::Point) -> bool;
    fn get_order(&self) -> BigUint;
}

#[cfg(test)]
mod tests;

/// Returns the version of the crate.
pub fn version() -> &'static str {
    private_version();
    env!("CARGO_PKG_VERSION")
}

fn private_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
