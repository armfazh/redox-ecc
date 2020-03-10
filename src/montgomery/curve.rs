//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, ToBigInt, Sign};

use num_traits::identities::{Zero,One};

use std::str::FromStr;
use std::io::{Error,ErrorKind};

use crate::do_if_eq;
use crate::ellipticcurve::EllipticCurve;
use crate::field::{Field, FromFactory, Sgn0, Sqrt};
use crate::primefield::{Fp, FpElt};
use crate::montgomery::point::{Point, ProyCoordinates};
use crate::montgomery::scalar::Scalar;

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
    fn deserialize(&self, buf: &[u8]) -> Result<Self::Point,std::io::Error> {
        let p = self.f.get_modulus();
        let max_bytes = (p.bits()+7)/8;
        if buf.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "Input buffer is empty."));
        }
        let tag = buf[0];
        // check x coordinate is in the valid range, Sign::Plus => > 0
        let x_val = BigInt::from_bytes_be(Sign::Plus, &buf[1..max_bytes+1]);
        if x_val > &p-&BigInt::one() {
            return Err(Error::new(ErrorKind::Other, "Invalid x coordinate"));
        }
        match tag {
            0x00 => {
                // return point of infinity
                if buf.len() != 1 {
                    return Err(Error::new(ErrorKind::Other, "Point at infinity should just be a single zero byte"));
                }
                Ok(self.identity())
            },
            0x04 => {
                if buf.len() != 2*max_bytes+1 {
                    return Err(Error::new(ErrorKind::Other, "Invalid bytes for deserialization"));
                }
                let x = self.f.elt(x_val);
                let y_val = BigInt::from_bytes_be(Sign::Plus, &buf[max_bytes+1..]);
                if y_val > &p-&BigInt::one() {
                    return Err(Error::new(ErrorKind::Other, "Invalid y coordinate"));
                }
                let y = self.f.elt(y_val);
                Ok(self.new_point(ProyCoordinates {
                    x: x,
                    y: y,
                    z: self.f.one()
                }))
            }
            0x02 | 0x03 => {
                if buf.len() != max_bytes+1 {
                    return Err(Error::new(ErrorKind::Other, "Invalid bytes for deserialization"));
                }
                // recompute y coordinate
                let one = self.f.one();
                let x = self.f.elt(x_val);
                let x_a = &x + &self.a;
                let xx_ax = &x_a * &x;
                let xx_ax_1 = &xx_ax + &one;
                let byy = &xx_ax_1 * &x;
                let b_inv = &one/&self.b;
                let yy = &byy * b_inv;
                let y_sqrt = yy.sqrt();
                let s = y_sqrt.sgn0_le();
                let deser_tag = (((s>>1)&0x1)+2) as u8;
                let mut y = y_sqrt;
                if tag != deser_tag {
                    y = -y;
                }
                Ok(self.new_point(ProyCoordinates {
                    x: x,
                    y: y,
                    z: self.f.one()
                }))
            }
            _ => Err(Error::new(ErrorKind::Other, "Invalid tag specified"))
        }
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

// tests for ser/deser
#[cfg(test)]
mod tests {
    use crate::instances::{CURVE25519,CURVE448,GetCurve};
    use crate::ellipticcurve::{EllipticCurve,EcPoint};

    #[test]
    fn point_serialization() {
        for &id in [CURVE25519,CURVE448].iter() {
            let ec = id.get();
            let gen = ec.get_generator();
            let ser = gen.serialize(false);
            let deser = ec.deserialize(&ser).unwrap();
            assert!(ec.is_on_curve(&deser), "decompressed point validity check for {}", id);
            assert!(
                gen == deser,
                "decompressed point equality check for {}", id
            );
        }
    }

    #[test]
    fn point_serialization_compressed() {
        for &id in [CURVE25519,CURVE448].iter() {
            let ec = id.get();
            let gen = ec.get_generator();
            let ser = gen.serialize(true);
            let deser = ec.deserialize(&ser).unwrap();
            assert!(ec.is_on_curve(&deser), "compressed point validity check for {}", id);
            assert!(
                gen == deser,
                "compressed point equality check for {}", id
            );
        }
    }
}