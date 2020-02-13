//! This is documentation for the `scalar` module.
//!
//! The scalar module is meant to be used for bar.

extern crate num_bigint;
extern crate num_integer;

use num_bigint::BigInt;
use num_integer::Integer;

use crate::curve::WeierstrassProjectivePoint;

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone)]
pub struct Scalar {
    pub(super) k: BigInt,
    pub(super) r: BigInt,
}

impl std::fmt::Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.k)
    }
}

impl Scalar {
    pub fn reduce(&self) -> Scalar {
        Scalar {
            k: self.k.mod_floor(&self.r),
            r: self.r.clone(),
        }
    }
    #[inline]
    fn from(&self, n: BigInt) -> Scalar {
        Scalar {
            k: n.mod_floor(&self.r),
            r: self.r.clone(),
        }
    }
    #[inline]
    fn neg_mod(&self) -> Self {
        self.from(-&self.k)
    }
    #[inline]
    fn add_mod(&self, other: &Scalar) -> Self {
        self.from(&self.k + &other.k)
    }
    #[inline]
    fn sub_mod(&self, other: &Scalar) -> Self {
        self.from(&self.k - &other.k)
    }
    #[inline]
    fn mul_mod(&self, other: &Scalar) -> Self {
        self.from(&self.k * &other.k)
    }
}

impl std::cmp::PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        let a = self.reduce();
        let b = other.reduce();
        self.r == other.r && a.k == b.k
    }
}

impl<'a, 'b> Mul<&'b WeierstrassProjectivePoint> for &'a Scalar {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: &'b WeierstrassProjectivePoint) -> Self::Output {
        other * &self
    }
}
impl<'b> Mul<&'b WeierstrassProjectivePoint> for Scalar {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: &'b WeierstrassProjectivePoint) -> Self::Output {
        other * &self
    }
}
impl Mul<WeierstrassProjectivePoint> for Scalar {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: WeierstrassProjectivePoint) -> Self::Output {
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

impl_binary_op!(Scalar, Add, add, add_mod, r, ERR_BIN_OP);
impl_binary_op!(Scalar, Sub, sub, sub_mod, r, ERR_BIN_OP);
impl_binary_op!(Scalar, Mul, mul, mul_mod, r, ERR_BIN_OP);
impl_unary_op!(Scalar, Neg, neg, neg_mod);

const ERR_BIN_OP: &'static str = "elements of different groups";
