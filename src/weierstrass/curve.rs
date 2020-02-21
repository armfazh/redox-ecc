//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, ToBigInt};

use num_traits::identities::Zero;

use std::str::FromStr;

use crate::field::{FpElt, PrimeField};
use crate::weierstrass::point::{Point, ProyCoordinates};
use crate::weierstrass::scalar::Scalar;
use crate::weierstrass::CurveID;
use crate::{do_if_eq, FromFactory};
use crate::{EllipticCurve, Field};

/// This is an elliptic curve defined by the Weierstrass equation `y^2=x^3+ax+b`.
///
/// **Atention** This implementation only supports curves of prime order.
#[derive(Clone, std::cmp::PartialEq)]
pub struct Curve {
    pub(super) f: PrimeField,
    pub(super) a: FpElt,
    pub(super) b: FpElt,
    pub(super) r: BigUint,
    gx: FpElt,
    gy: FpElt,
    h: BigUint,
}

impl EllipticCurve for Curve {
    type F = PrimeField;
    type P = Point;
    type Coordinates = ProyCoordinates;
    type S = Scalar;
    fn new_point(&self, c: Self::Coordinates) -> Self::P {
        let e = self.clone();
        let pt = Point { e, c };
        do_if_eq!(self.is_on_curve(&pt), true, pt, ERR_ECC_NEW)
    }
    fn new_scalar(&self, k: BigInt) -> Self::S {
        Scalar::new(k, &self.r)
    }
    fn identity(&self) -> Self::P {
        self.new_point(ProyCoordinates {
            x: self.f.zero(),
            y: self.f.one(),
            z: self.f.zero(),
        })
    }
    fn is_on_curve(&self, p: &Self::P) -> bool {
        let p = &p.c;
        let x3 = &p.x * &(&p.x ^ 2u32);
        let bz = &self.b * &p.z;
        let ax = &self.a * &p.x;
        let zz = &p.z * &(ax + &bz);
        let zy = &p.z * &(zz - &(&p.y ^ 2u32));
        let eq = x3 + &zy;
        eq.is_zero()
    }
    fn get_order(&self) -> BigUint {
        self.r.clone()
    }
    fn get_field(&self) -> Self::F {
        self.f.clone()
    }
    fn get_cofactor(&self) -> BigInt {
        self.h.to_bigint().unwrap()
    }
    fn get_generator(&self) -> Self::P {
        self.new_point(ProyCoordinates {
            x: self.gx.clone(),
            y: self.gy.clone(),
            z: self.f.one(),
        })
    }
}

impl std::convert::From<&CurveID> for Curve {
    fn from(id: &CurveID) -> Curve {
        let params = id.0;
        let f = PrimeField::create(BigUint::from_str(params.p).unwrap());
        let a = f.from(params.a);
        let b = f.from(params.b);
        let gx = f.from(params.gx);
        let gy = f.from(params.gy);
        let r = BigUint::from_str(params.r).unwrap();
        let h = BigUint::from_str(params.h).unwrap();
        Curve {
            f,
            a,
            b,
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
            "Weierstrass Curve y^2=x^3+ax+b\na: {}\nb: {}",
            self.a, self.b,
        )
    }
}

const ERR_ECC_NEW: &str = "not valid point";
