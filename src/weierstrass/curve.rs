//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint};

use num_traits::identities::Zero;

use std::str::FromStr;

use crate::field::Fp;
use crate::weierstrass::point::{Coordinates, Point};
use crate::weierstrass::scalar::Scalar;
use crate::weierstrass::CurveID;
use crate::EllipticCurve;
use crate::Field;
use crate::{do_if_eq, FromFactory};

/// This is an elliptic curve defined by the Weierstrass equation `y^2=x^3+ax+b`.
///
/// **Atention** This implementation only supports curves of prime order.
#[derive(Clone, std::cmp::PartialEq)]
pub struct Curve {
    f: Fp,
    pub(super) a: Fp,
    pub(super) b: Fp,
    pub(super) r: BigUint,
}

impl EllipticCurve for Curve {
    type Field = Fp;
    type Point = Point;
    type Coordinates = Coordinates;
    type Scalar = Scalar;
    fn new_point(&self, c: Self::Coordinates) -> Self::Point {
        let e = self.clone();
        let pt = Point { e, c };
        do_if_eq!(self.is_on_curve(&pt), true, pt, ERR_ECC_NEW)
    }
    fn new_scalar(&self, k: BigInt) -> Self::Scalar {
        Scalar::new(k, &self.r)
    }
    fn identity(&self) -> Self::Point {
        self.new_point(Coordinates {
            x: self.f.zero(),
            y: self.f.one(),
            z: self.f.zero(),
        })
    }
    fn is_on_curve(&self, p: &Self::Point) -> bool {
        let p = &p.c;
        let x3 = &p.x * &p.x * &p.x;
        let bz = &self.b * &p.z;
        let ax = &self.a * &p.x;
        let zz = &p.z * &(ax + &bz);
        let zy = &p.z * &(zz - &(&p.y * &p.y));
        let eq = x3 + &zy;
        eq.is_zero()
    }
    fn get_order(&self) -> BigUint {
        self.r.clone()
    }
}

impl std::convert::From<&CurveID> for Curve {
    fn from(id: &CurveID) -> Curve {
        let params = id.0;
        let f = <Fp as From<BigUint>>::from(BigUint::from_str(params.p).unwrap());
        let a = f.from(params.a);
        let b = f.from(params.b);
        let r = BigUint::from_str(params.r).unwrap();
        Curve { f, a, b, r }
    }
}

impl std::fmt::Display for Curve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Weierstrass Curve y^2=x^3+ax+b\na: {}\nb: {}",
            self.a, self.b,
        )
    }
}

const ERR_ECC_NEW: &str = "not valid point";
