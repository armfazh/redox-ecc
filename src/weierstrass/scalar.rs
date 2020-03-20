//! This is documentation for the `scalar` module.
//!
//! The scalar module is meant to be used for bar.

use impl_ops::impl_op_ex;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_integer::Integer;

use std::ops;
use std::ops::{Div, Mul};

use crate::do_if_eq;
use crate::ellipticcurve::EcScalar;
use crate::weierstrass::point::Point;
use crate::ops::Serialize;

#[derive(Clone, PartialEq)]
pub struct Scalar {
    pub(super) k: BigInt,
    pub(super) r: BigInt,
}

impl Scalar {
    pub fn new(k: BigInt, r: &BigUint) -> Self {
        let r = r.to_bigint().unwrap();
        let k = k.mod_floor(&r);
        Scalar { k, r }
    }
}

impl EcScalar for Scalar {}
impl Serialize for Scalar {
    /// serializes the field element into big-endian bytes
    fn to_bytes_be(&self) -> Vec<u8> {
        let field_len = (self.r.bits()+7)/8;
        let mut bytes = self.k.to_biguint().unwrap().to_bytes_be();
        let mut out = vec![0; field_len-bytes.len()];
        if out.len() > 0 {
            out.append(&mut bytes);
        } else {
            out = bytes;
        }
        out
    }
    fn to_bytes_le(&self) -> Vec<u8> {
        let mut buf = self.to_bytes_be();
        buf.reverse();
        buf
    }
}

impl Scalar {
    #[inline]
    fn red(&self, k: BigInt) -> Self {
        let k = k.mod_floor(&self.r);
        let r = self.r.clone();
        Scalar { k, r }
    }
    #[inline]
    pub fn inv_mod(&self) -> Scalar {
        let exp = &self.r - 2u32;
        self.red(self.k.modpow(&exp, &self.r))
    }
}

impl_op_ex!(+|a: &Scalar, b: &Scalar| -> Scalar {
    do_if_eq!(a.r == b.r, a.red(&a.k + &b.k), ERR_BIN_OP)
});
impl_op_ex!(-|a: &Scalar, b: &Scalar| -> Scalar {
    do_if_eq!(a.r == b.r, a.red(&a.k - &b.k), ERR_BIN_OP)
});
impl_op_ex!(*|a: &Scalar, b: &Scalar| -> Scalar {
    do_if_eq!(a.r == b.r, a.red(&a.k * &b.k), ERR_BIN_OP)
});
impl_op_ex!(/|a: &Scalar, b: &Scalar| -> Scalar {
    #[allow(clippy::suspicious_arithmetic_impl)] {
        a * b.inv_mod()
    }
});
impl_op_ex!(-|a: &Scalar| -> Scalar { a.red(-&a.k) });

impl<'a> Div<&'a Scalar> for u32 {
    type Output = Scalar;
    #[inline]
    fn div(self, other: &Scalar) -> Self::Output {
        do_if_eq!(self == 1u32, other.inv_mod(), ERR_INV_OP)
    }
}

impl<'a, 'b> Mul<&'b Point> for &'a Scalar {
    type Output = Point;
    #[inline]
    fn mul(self, other: &'b Point) -> Self::Output {
        other * self
    }
}
impl<'b> Mul<&'b Point> for Scalar {
    type Output = Point;
    #[inline]
    fn mul(self, other: &'b Point) -> Self::Output {
        other * &self
    }
}
impl Mul<Point> for Scalar {
    type Output = Point;
    #[inline]
    fn mul(self, other: Point) -> Self::Output {
        other * &self
    }
}

struct Iterino {
    l: usize,
    i: usize,
    v: std::vec::Vec<u32>,
    is_lr: bool,
}

impl std::iter::Iterator for Iterino {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.l {
            let bit = self.v[self.i / 32usize] >> (self.i % 32);
            let b = (bit & 1) != 0;
            if self.is_lr {
                let (x, _) = self.i.overflowing_sub(1usize);
                self.i = x
            } else {
                self.i += 1usize
            }
            Some(b)
        } else {
            None
        }
    }
}

impl Scalar {
    pub fn iter_lr(&self) -> impl std::iter::Iterator<Item = bool> {
        let l = self.k.bits();
        let i = l - 1usize;
        let (_, v) = self.k.to_u32_digits();
        let is_lr = true;
        Iterino { l, i, v, is_lr }
    }
    pub fn iter_rl(&self) -> impl std::iter::Iterator<Item = bool> {
        let l = self.k.bits();
        let i = 0usize;
        let (_, v) = self.k.to_u32_digits();
        let is_lr = false;
        Iterino { l, i, v, is_lr }
    }
}

impl std::fmt::Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.k)
    }
}

const ERR_BIN_OP: &str = "elements of different groups";
const ERR_INV_OP: &str = "numerator must be 1u32";
