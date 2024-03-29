//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use num_traits::identities::Zero;

use std::io::{Error, ErrorKind};
use std::str::FromStr;

use crate::do_if_eq;
use crate::edwards::point::{Point, ProyCoordinates};
use crate::edwards::scalar::Scalar;
use crate::ellipticcurve::{Decode, EllipticCurve};
use crate::field::{Field, Sgn0, Sqrt};
use crate::ops::FromFactory;
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
            c: ProyCoordinates {
                t: &x * &y,
                x,
                y,
                z: f.one(),
            },
            e,
        };
        do_if_eq!(self.is_on_curve(&pt), pt, ERR_ECC_NEW)
    }

    fn new_scalar(&self, k: BigInt) -> Self::Scalar {
        Scalar::new(k, &self.r)
    }
    fn identity(&self) -> Self::Point {
        let f = &self.f;
        self.new_proy_point(ProyCoordinates {
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
        self.new_proy_point(ProyCoordinates {
            x: self.gx.clone(),
            y: self.gy.clone(),
            t: &self.gx * &self.gy,
            z: self.f.one(),
        })
    }
}

impl Decode for Curve {
    type Deser = Point;
    // based on https://tools.ietf.org/html/rfc8032#section-5.2.3
    fn decode(&self, buf: &[u8]) -> Result<Self::Deser, Error> {
        let modulus = self.get_field().get_modulus();
        let size = (modulus.bits() as usize + 1 + 7) / 8;
        // step 1
        if buf.len() != size {
            return Err(Error::new(ErrorKind::Other, "Wrong input buffer size."));
        }
        let last_byte = size - 1;
        let x_0 = (buf[last_byte] >> 7) & 0x01;
        let mut y_bytes = buf.to_vec();
        y_bytes[last_byte] &= &127; // clear msb
        let y_zz = BigInt::from_bytes_le(Sign::Plus, &y_bytes);
        if y_zz >= modulus {
            return Err(Error::new(ErrorKind::Other, "Invalid y value chosen"));
        }
        let y = self.f.elt(y_zz);

        // step 2
        let yy = &y * &y;
        let minus_one = -self.f.one();
        let u = &yy + &minus_one;
        let v = (&self.d * &yy) - &self.a;
        let u_inv_v = u / v;
        let x_sqrt = u_inv_v.sqrt();

        // step 4 (step 3 is unnecessary)
        if x_sqrt == self.f.zero() && x_0 == 0x01 {
            return Err(Error::new(
                ErrorKind::Other,
                "Failed decoding on square root",
            ));
        }
        let tag = ((x_sqrt.sgn0() >> 1) & 0x01) as u8;
        let mut x = x_sqrt;
        if tag != x_0 {
            x = -x;
        }
        Ok(self.new_point(x, y))
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

// tests for ser/deser
#[cfg(test)]
mod tests {
    use crate::ellipticcurve::{Decode, EllipticCurve, Encode};
    use crate::field::Field;
    use crate::instances::{GetCurve, EDWARDS25519, EDWARDS448};

    #[test]
    fn point_serialization() {
        for &id in [EDWARDS25519, EDWARDS448].iter() {
            let ec = id.get();
            let modulus = ec.get_field().get_modulus();
            let gen = ec.get_generator();
            let ser = gen.encode(false); // compression does not exist
            assert_eq!(ser.len(), (modulus.bits() as usize + 1 + 7) / 8);
            let deser = ec.decode(&ser).unwrap();
            assert!(
                ec.is_on_curve(&deser),
                "decompressed point validity check for {}",
                id
            );
            assert!(gen == deser, "decompressed point equality check for {}", id);
        }
    }
}
