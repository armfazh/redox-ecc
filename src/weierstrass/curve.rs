//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint};

use num_traits::identities::Zero;

use crate::do_if_eq;
use crate::field::Fp;
use crate::weierstrass::point::WeierstrassPoint;
use crate::weierstrass::scalar::WeierstrassScalar;
use crate::EllipticCurve;
use crate::Field;

/// This is an elliptic curve defined by the Weierstrass equation `y^2=x^3+ax+b`.
///
/// **Atention** This implementation only supports curves of prime order.
#[derive(Clone, std::cmp::PartialEq)]
pub struct WeierstrassCurve {
    pub f: Fp,
    pub a: Fp,
    pub b: Fp,
    pub r: BigUint,
}

impl EllipticCurve for WeierstrassCurve {
    type Point = WeierstrassPoint;
    type Scalar = WeierstrassScalar;
    fn new_point(&self, px: Fp, py: Fp, pz: Fp) -> Self::Point {
        let e = self.clone();
        let p = WeierstrassPoint {
            e,
            x: px,
            y: py,
            z: pz,
        };
        do_if_eq!(self.is_on_curve(&p), true, p, ERR_ECC_NEW)
    }
    fn new_scalar(&self, k: BigInt) -> Self::Scalar {
        WeierstrassScalar::new(k, &self.r)
    }
    fn identity(&self) -> Self::Point {
        self.new_point(self.f.zero(), self.f.one(), self.f.zero())
    }
    fn is_on_curve(&self, p: &Self::Point) -> bool {
        let x3 = &p.x * &p.x * &p.x;
        let bz = &self.b * &p.z;
        let ax = &self.a * &p.x;
        let zz = &p.z * &(ax + &bz);
        let zy = &p.z * &(zz - &(&p.y * &p.y));
        let eq = x3 + &zy;
        eq.is_zero()
    }
}

impl std::fmt::Display for WeierstrassCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Weierstrass Curve y^2=x^3+ax+b\na: {}\nb: {}",
            self.a, self.b,
        )
    }
}

const ERR_ECC_NEW: &str = "not valid point";
