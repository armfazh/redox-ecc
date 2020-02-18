//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint};

use num_traits::identities::Zero;

use std::str::FromStr;

use crate::edwards::point::{Point, ProyCoordinates};
use crate::edwards::scalar::Scalar;
use crate::edwards::CurveID;
use crate::field::Fp;
use crate::EllipticCurve;
use crate::Field;
use crate::{do_if_eq, FromFactory};

/// This is an elliptic curve defined in the twisted Edwards model and defined by the equation:
/// ax^2+y^2=1+dx^2y^2.
///
#[derive(Clone, PartialEq)]
pub struct Curve {
    f: Fp,
    pub(super) a: Fp,
    pub(super) d: Fp,
    pub(super) r: BigUint,
    gx: Fp,
    gy: Fp,
    h: BigUint,
}

impl EllipticCurve for Curve {
    type Field = Fp;
    type Point = Point;
    type Coordinates = ProyCoordinates;
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
        let x2 = &p.x * &p.x;
        let y2 = &p.y * &p.y;
        let t2 = &p.t * &p.t;
        let z2 = &p.z * &p.z;
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
    fn get_field(&self) -> Self::Field {
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

impl std::convert::From<&CurveID> for Curve {
    fn from(id: &CurveID) -> Curve {
        let params = id.0;
        let f = <Fp as From<BigUint>>::from(BigUint::from_str(params.p).unwrap());
        let a = f.from(params.a);
        let d = f.from(params.d);
        let gx = f.from(params.gx);
        let gy = f.from(params.gy);
        let r = BigUint::from_str(params.r).unwrap();
        let h = BigUint::from_str(params.h).unwrap();
        Curve {
            f,
            a,
            d,
            r,
            h,
            gx,
            gy,
        }
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

const ERR_ECC_NEW: &str = "not valid point";
