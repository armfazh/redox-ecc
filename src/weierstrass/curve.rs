//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, ToBigInt, Sign};

use num_traits::identities::Zero;

use std::str::FromStr;
use std::io::{Error,ErrorKind};

use subtle;
use subtle::ConditionallySelectable;

use crate::do_if_eq;
use crate::ellipticcurve::EllipticCurve;
use crate::field::{Field, FromFactory, Sgn0, Sqrt};
use crate::primefield::{Fp, FpElt};
use crate::weierstrass::point::{Point, ProyCoordinates};
use crate::weierstrass::scalar::Scalar;

/// This is an elliptic curve defined by the Weierstrass equation `y^2=x^3+ax+b`.
///
/// **Atention** This implementation only supports curves of prime order.
#[derive(Clone, std::cmp::PartialEq)]
pub struct Curve {
    pub(super) f: Fp,
    pub(super) a: FpElt,
    pub(super) b: FpElt,
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
        self.new_point(ProyCoordinates {
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
    fn get_field(&self) -> Self::F {
        self.f.clone()
    }
    fn get_cofactor(&self) -> BigInt {
        self.h.to_bigint().unwrap()
    }
    fn get_generator(&self) -> Self::Point {
        self.new_point(ProyCoordinates {
            x: self.gx.clone(),
            y: self.gy.clone(),
            z: self.f.one(),
        })
    }
    fn serialize(&self, p: &Self::Point, compress: bool, buf: &mut [u8]) {
        let p_coords = &p.c;
        let x = &p_coords.x;
        let y = &p_coords.y;
        let mut x_bytes = x.to_bytes_be();
        let mut y_bytes = y.to_bytes_be();
        let out_bytes = match compress {
            true => {
                let choice = (y.sgn0_le() > 0) as u8;
                // if sign > 0: tag = 0x02; elif sign < 0: tag = 0x03;
                let tag = u8::conditional_select(&0x02, &0x03, choice.into());
                // sign should be 1/-1, so this cast should work
                let mut o = vec![tag];
                o.append(&mut x_bytes);
                o
            },
            _ => {
                let mut o: Vec<u8> = vec![0x04];
                o.append(&mut x_bytes);
                o.append(&mut y_bytes);
                o
            }
        };
        buf.copy_from_slice(&out_bytes);
    }
    fn deserialize(&self, buf: &[u8]) -> Result<Self::Point,Error> {
        let p = self.f.get_modulus();
        let max_bytes = (p.bits()+7)/8;
        let tag = buf[0];
        match tag {
            0x04 => {
                if buf.len() != 2*max_bytes+1 {
                    return Err(Error::new(ErrorKind::Other, "Invalid bytes for deserialization"));
                }
                let x = self.f.elt(BigInt::from_bytes_be(Sign::Plus, &buf[1..max_bytes+1]));
                let y = self.f.elt(BigInt::from_bytes_be(Sign::Plus, &buf[max_bytes+1..]));
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
                let x = self.f.elt(BigInt::from_bytes_be(Sign::Plus, &buf[1..max_bytes+1]));
                let xx = &x * &x;
                let xx_a = &xx + &self.a;
                let xxx_ax = &xx_a * &x;
                let xxx_ax_b = &xxx_ax + &self.b;
                let y_sqrt = xxx_ax_b.sqrt();
                let y_sgn_bit = (y_sqrt.sgn0_le() == -1) as u8;
                let parity_bit = (0x03 == tag) as u8;
                let parity_cmp = (parity_bit == y_sgn_bit) as u8;
                let mask = i8::conditional_select(&1, &(-1), parity_cmp.into());
                let y = self.f.elt(BigInt::from(mask)) * &y_sqrt;
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
    use crate::instances::{P256, P384, P521};
    use crate::ellipticcurve::EllipticCurve;

    #[test]
    fn point_serialization() {
        for &id in [P256,P384,P521].iter() {
            let ec = id.get();
            let gen = ec.get_generator();
            let field_len = (ec.f.get_modulus().bits()+7)/8;
            let mut ser = vec![0; 2*field_len+1];
            ec.serialize(&gen, false, &mut ser);
            let deser = ec.deserialize(&ser).unwrap();
            assert!(ec.is_on_curve(&deser), "decompressed point validity check for {:?}", id.0.name);
            let gen_z = gen.c.z;
            let gen_z_sq = &gen_z * &gen_z;
            let des_z = deser.c.z;
            let des_z_sq = &des_z * &des_z;
            assert!(
                gen.c.x/&gen_z_sq == deser.c.x/&des_z_sq,
                "decompressed x coordinate equality check for {:?}", id.0.name
            );
            assert!(
                gen.c.y/(&gen_z_sq * &gen_z) == deser.c.y/(&des_z_sq * &des_z),
                "decompressed y coordinate equality check for {:?}", id.0.name
            );
        }
    }

    #[test]
    fn point_serialization_compressed() {
        for &id in [P256,P384,P521].iter() {
            let ec = id.get();
            let gen = ec.get_generator();
            let field_len = (ec.f.get_modulus().bits()+7)/8;
            let mut ser = vec![0; field_len+1];
            ec.serialize(&gen, true, &mut ser);
            let deser = ec.deserialize(&ser).unwrap();
            assert!(ec.is_on_curve(&deser), "compressed point validity check for {:?}", id.0.name);
            let gen_z = gen.c.z;
            let gen_z_sq = &gen_z * &gen_z;
            let des_z = deser.c.z;
            let des_z_sq = &des_z * &des_z;
            assert!(
                gen.c.x/&gen_z_sq == deser.c.x/&des_z_sq,
                "compressed x coordinate equality check for {:?}", id.0.name
            );
            assert!(
                gen.c.y/(&gen_z_sq * &gen_z) == deser.c.y/(&des_z_sq * &des_z),
                "compressed y coordinate equality check for {:?}", id.0.name
            );
        }
    }
}