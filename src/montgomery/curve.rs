//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, ToBigInt};

use num_traits::identities::Zero;

use std::str::FromStr;

use crate::do_if_eq;
use crate::ellipticcurve::EllipticCurve;
use crate::field::{Field, FromFactory};
use crate::montgomery::point::{Point, ProyCoordinates};
use crate::montgomery::scalar::Scalar;
use crate::primefield::{Fp, FpElt};

/// This is an elliptic curve defined in Montgomery from and defined by the equation:
/// by^2=x^3+ax^2+x.
///
#[derive(Clone, PartialEq)]
pub struct Curve {
    pub(super) f: Fp,
    pub(super) a: FpElt,
    pub(super) b: FpElt,
    pub(super) s: FpElt,
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
            z: f.zero(),
        })
    }
    fn is_on_curve(&self, p: &Self::Point) -> bool {
        let p = &p.c;
        let l = &self.b * &(&p.y ^ 2u32) * &p.z;
        let r = &p.x * &((&p.x ^ 2u32) + &self.a * &p.x * &p.z + &(&p.z ^ 2u32));
        let e = l - r;
        e.is_zero()
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
            z: self.f.one(),
        })
    }
    fn deserialize(&self, _: &[u8]) -> Result<Self::Point,std::io::Error> {
        panic!("unimplemented!");
    }
}

impl std::fmt::Display for Curve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Montgomery Curve by^2=x^3+ax^2+x\na: {}\nb: {}",
            self.a, self.b,
        )
    }
}

#[derive(PartialEq, Eq)]
pub struct Params {
    pub name: &'static str,
    pub p: &'static str,
    pub a: &'static str,
    pub b: &'static str,
    pub s: &'static str,
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
            b: f.from(params.b),
            s: f.from(params.s),
            r: BigUint::from_str(params.r).unwrap(),
            h: BigUint::from_str(params.h).unwrap(),
            gx: f.from(params.gx),
            gy: f.from(params.gy),
            f,
        }
    }
}

const ERR_ECC_NEW: &str = "not valid point";
