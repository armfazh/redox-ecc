//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};

use num_traits::identities::Zero;

use std::io::{Error, ErrorKind};
use std::str::FromStr;

use crate::do_if_eq;
use crate::ellipticcurve::{Decode, EllipticCurve};
use crate::field::{Field, Sgn0, Sqrt};
use crate::ops::FromFactory;
use crate::primefield::{Fp, FpElt};
use crate::weierstrass::point::{Point, ProyCoordinates};
use crate::weierstrass::scalar::Scalar;

/// This is an elliptic curve defined by the Weierstrass equation `y^2=x^3+ax+b`.
///
/// **Atention** This implementation only supports curves of prime order.
#[derive(Clone, std::cmp::PartialEq)]
pub struct Curve {
    f: Fp,
    pub(super) a: FpElt,
    pub(super) b: FpElt,
    pub(super) r: BigUint,
    pub(super) gx: FpElt,
    pub(super) gy: FpElt,
    pub(super) h: BigUint,
}
impl Curve {
    pub(crate) fn new_proy_point(&self, c: ProyCoordinates) -> Point {
        let e = self.clone();
        let pt = Point { e, c };
        do_if_eq!(self.is_on_curve(&pt), pt, ERR_ECC_NEW)
    }
}

impl EllipticCurve for Curve {
    type F = Fp;
    type Scalar = Scalar;
    type Point = Point;
    fn new_point(&self, x: <Self::F as Field>::Elt, y: <Self::F as Field>::Elt) -> Self::Point {
        let e = self.clone();
        let f = e.get_field();
        let pt = Point {
            c: ProyCoordinates { x, y, z: f.one() },
            e,
        };
        do_if_eq!(self.is_on_curve(&pt), pt, ERR_ECC_NEW)
    }
    fn new_scalar(&self, k: BigInt) -> Self::Scalar {
        Scalar::new(k, &self.r)
    }
    fn identity(&self) -> Self::Point {
        self.new_proy_point(ProyCoordinates {
            x: self.f.zero(),
            y: self.f.one(),
            z: self.f.zero(),
        })
    }
    fn is_on_curve(&self, p: &Self::Point) -> bool {
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
    fn get_field(&self) -> &Self::F {
        &self.f
    }
    fn get_cofactor(&self) -> BigInt {
        self.h.to_bigint().unwrap()
    }
    fn get_generator(&self) -> Self::Point {
        self.new_proy_point(ProyCoordinates {
            x: self.gx.clone(),
            y: self.gy.clone(),
            z: self.f.one(),
        })
    }
}

impl Decode for Curve {
    type Deser = Point;
    fn decode(&self, buf: &[u8]) -> Result<Self::Deser, Error> {
        let p = self.f.get_modulus();
        let max_bytes = (p.bits() + 7) / 8;
        if buf.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "Input buffer is empty."));
        }
        let tag = buf[0];
        // check x coordinate is in the valid range, Sign::Plus => > 0
        let x_val = BigInt::from_bytes_be(Sign::Plus, &buf[1..max_bytes + 1]);
        if x_val >= p {
            return Err(Error::new(ErrorKind::Other, "Invalid x coordinate"));
        }
        match tag {
            0x00 => {
                // return point of infinity
                if buf.len() != 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Point at infinity should just be a single zero byte",
                    ));
                }
                Ok(self.identity())
            }
            0x04 => {
                if buf.len() != 2 * max_bytes + 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Invalid bytes for deserialization",
                    ));
                }
                let x = self.f.elt(x_val);
                let y_val = BigInt::from_bytes_be(Sign::Plus, &buf[max_bytes + 1..]);
                if y_val >= p {
                    return Err(Error::new(ErrorKind::Other, "Invalid y coordinate"));
                }
                let y = self.f.elt(y_val);
                Ok(self.new_point(x, y))
            }
            0x02 | 0x03 => {
                if buf.len() != max_bytes + 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Invalid bytes for deserialization",
                    ));
                }
                // recompute y coordinate
                let x = self.f.elt(x_val);
                let xx = &x * &x;
                let xx_a = &xx + &self.a;
                let xxx_ax = &xx_a * &x;
                let xxx_ax_b = &xxx_ax + &self.b;
                let y_sqrt = xxx_ax_b.sqrt();
                let s = y_sqrt.sgn0_le();
                let deser_tag = (((s >> 1) & 0x1) + 2) as u8;
                let mut y = y_sqrt;
                if tag != deser_tag {
                    y = -y;
                }
                Ok(self.new_point(x, y))
            }
            _ => Err(Error::new(ErrorKind::Other, "Invalid tag specified")),
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

#[derive(PartialEq, Eq)]
pub struct Params {
    pub name: &'static str,
    pub p: &'static str,
    pub a: &'static str,
    pub b: &'static str,
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
            r: BigUint::from_str(params.r).unwrap(),
            h: BigUint::from_str(params.h).unwrap(),
            gx: f.from(params.gx),
            gy: f.from(params.gy),
            f,
        }
    }
}

const ERR_ECC_NEW: &str = "not valid point";

// tests for ser/deser
#[cfg(test)]
mod tests {
    use crate::ellipticcurve::{Decode, EllipticCurve, Encode};
    use crate::field::Field;
    use crate::instances::{GetCurve, P256, P384, P521};

    #[test]
    fn point_serialization() {
        for &id in [P256, P384, P521].iter() {
            let ec = id.get();
            let modulus = ec.get_field().get_modulus();
            let gen = ec.get_generator();
            let ser = gen.encode(false);
            assert_eq!(ser.len(), 2 * ((modulus.bits() + 7) / 8) + 1);
            let deser = ec.decode(&ser).unwrap();
            assert!(
                ec.is_on_curve(&deser),
                "decompressed point validity check for {}",
                id
            );
            assert!(gen == deser, "decompressed point equality check for {}", id);
        }
    }

    #[test]
    fn point_serialization_compressed() {
        for &id in [P256, P384, P521].iter() {
            let ec = id.get();
            let modulus = ec.get_field().get_modulus();
            let gen = ec.get_generator();
            let ser = gen.encode(true);
            assert_eq!(ser.len(), ((modulus.bits() + 7) / 8) + 1);
            let deser = ec.decode(&ser).unwrap();
            assert!(
                ec.is_on_curve(&deser),
                "compressed point validity check for {}",
                id
            );
            assert!(gen == deser, "compressed point equality check for {}", id);
        }
    }
}
