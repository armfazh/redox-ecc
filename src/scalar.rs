//! This is documentation for the `scalar` module.
//!
//! The scalar module is meant to be used for bar.

extern crate num_bigint;
extern crate num_integer;

use num_bigint::BigInt;
use num_integer::Integer;

use std::ops::{Add, Mul, Sub};

use crate::curve::WeierstrassProjectivePoint;

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

impl std::cmp::PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        let a = self.reduce();
        let b = other.reduce();
        a.k == b.k && self.r == other.r
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
    fn add_mod(&self, x: &Scalar, y: &Scalar) -> Self {
        self.from(&x.k + &y.k)
    }
    #[inline]
    fn sub_mod(&self, x: &Scalar, y: &Scalar) -> Self {
        self.from(&x.k - &y.k)
    }
    #[inline]
    fn mul_mod(&self, x: &Scalar, y: &Scalar) -> Self {
        self.from(&x.k * &y.k)
    }
}

impl Mul<WeierstrassProjectivePoint> for Scalar {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: Self::Output) -> Self::Output {
        other * &self
    }
}

impl<'b> Mul<&'b WeierstrassProjectivePoint> for Scalar {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: &'b Self::Output) -> Self::Output {
        other * &self
    }
}

impl<'a, 'b> Mul<&'b WeierstrassProjectivePoint> for &'a Scalar {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: &'b Self::Output) -> Self::Output {
        other * &self
    }
}

const ERR_BIN_OP: &'static str = "elements of different groups";

macro_rules! impl_bin_op {
    ($trait:ident, $name:ident, $method:ident) => {
        impl<'a, 'b, 'c> $trait<&'b Scalar> for &'a Scalar {
            type Output = Scalar;
            #[inline]
            fn $name(self, other: &Scalar) -> Self::Output {
                do_if_eq!(self.r, other.r, self.$method(&self, &other), ERR_BIN_OP)
            }
        }
        impl<'a, 'c> $trait<&'a Scalar> for Scalar {
            type Output = Scalar;
            #[inline]
            fn $name(self, other: &Self) -> Self::Output {
                do_if_eq!(self.r, other.r, self.$method(&self, &other), ERR_BIN_OP)
            }
        }
        impl<'c> $trait for Scalar {
            type Output = Scalar;
            #[inline]
            fn $name(self, other: Self) -> Self::Output {
                do_if_eq!(self.r, other.r, self.$method(&self, &other), ERR_BIN_OP)
            }
        }
    };
}

impl_bin_op!(Add, add, add_mod);
impl_bin_op!(Sub, sub, sub_mod);
impl_bin_op!(Mul, mul, mul_mod);

pub struct Iterino {
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
    pub fn left_to_right(&self) -> Iterino {
        let l = self.k.bits();
        let i = l - 1usize;
        let (_, v) = self.k.to_u32_digits();
        let is_lr = true;
        Iterino { l, i, v, is_lr }
    }
    pub fn right_to_left(&self) -> Iterino {
        let l = self.k.bits();
        let i = 0usize;
        let (_, v) = self.k.to_u32_digits();
        let is_lr = false;
        Iterino { l, i, v, is_lr }
    }
}
