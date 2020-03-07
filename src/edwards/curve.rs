//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::identities::Zero;

use std::str::FromStr;

use crate::do_if_eq;
use crate::edwards::point::{Point, ProyCoordinates};
use crate::edwards::scalar::Scalar;
use crate::ellipticcurve::EllipticCurve;
use crate::field::{Field, FromFactory};
use crate::primefield::{Fp, FpElt};

/// This is an elliptic curve defined in the twisted Edwards model and defined by the equation:
/// ax^2+y^2=1+dx^2y^2.
///
#[derive(Clone, PartialEq)]
pub struct Curve {
    pub(super) f: Fp,
    pub(super) a: FpElt,
    pub(super) d: FpElt,
    pub(super) r: BigUint,
    pub(super) gx: FpElt,
    pub(super) gy: FpElt,
    pub(super) h: BigUint,
}

impl EllipticCurve for Curve {
    type F = Fp;
    type Scalar = Scalar;
    type Point = Point;
    type Coordinates = ProyCoordinates;
    fn new_point(&self, c: Self::Coordinates) -> Self::Point {
        let e = self.clone();
        let pt = Point { e, c };
        do_if_eq!(self.is_on_curve(&pt), pt, ERR_ECC_NEW)
    }
    fn new_scalar(&self, k: BigInt) -> Self::Scalar {
        Scalar::new(k, &self.r)
    }
    fn identity(&self) -> Self::Point {
        let f = &self.f;
        self.new_point(ProyCoordinates {
            x: f.zero(),
            y: f.one(),
            t: f.zero(),
            z: f.one(),
        })
    }
    fn is_on_curve(&self, p: &Self::Point) -> bool {
        let p = &p.c;
        let x2 = &p.x ^ 2u32;
        let y2 = &p.y ^ 2u32;
        let t2 = &p.t ^ 2u32;
        let z2 = &p.z ^ 2u32;
        let l1 = x2 * &self.a + y2;
        let r1 = t2 * &self.d + z2;
        let l2 = &p.x * &p.y;
        let r2 = &p.t * &p.z;
        let e1 = l1 - r1;
        let e2 = l2 - r2;
        e1.is_zero() && e2.is_zero()
    }
    fn get_order(&self) -> BigUint {
        self.r.clone()
    }
    fn get_cofactor(&self) -> BigInt {
        self.h.to_bigint().unwrap()
    }
    fn get_field(&self) -> Self::F {
        self.f.clone()
    }
    fn get_generator(&self) -> Self::Point {
        self.new_point(ProyCoordinates {
            x: self.gx.clone(),
            y: self.gy.clone(),
            t: &self.gx * &self.gy,
            z: self.f.one(),
        })
    }
}

impl std::fmt::Display for Curve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Twisted Edwards Curve ax^2+y^2=1+dx^2y^2\na: {}\nd: {}",
            self.a, self.d,
        )
    }
}

#[derive(PartialEq, Eq)]
pub struct Params {
    pub name: &'static str,
    pub p: &'static str,
    pub a: &'static str,
    pub d: &'static str,
    pub r: &'static str,
    pub h: &'static str,
    pub gx: &'static str,
    pub gy: &'static str,
}

impl<'a> std::convert::From<&'a Params> for Curve {
    fn from(params: &'a Params) -> Curve {
        let f = Fp::new(BigUint::from_str(params.p).unwrap());
        Curve {
            a: f.from(params.a),
            d: f.from(params.d),
            r: BigUint::from_str(params.r).unwrap(),
            h: BigUint::from_str(params.h).unwrap(),
            gx: f.from(params.gx),
            gy: f.from(params.gy),
            f,
        }
    }
}

const ERR_ECC_NEW: &str = "not valid point";
