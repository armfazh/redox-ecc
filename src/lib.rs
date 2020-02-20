//! This is documentation for the `redox-ecc` crate.
//!
//! The foo crate is meant to be used for bar.

// #![warn(missing_docs)]

extern crate num_bigint;
use crypto::digest::Digest;
use num_bigint::{BigInt, BigUint};
use std::ops::{Add, Mul};

mod macros;

pub mod field;

pub mod edwards;
pub mod h2c;
pub mod montgomery;
pub mod weierstrass;

pub trait FromFactory<T: Sized> {
    type Output;
    fn from(&self, _: T) -> Self::Output;
}

/// Field is a fabric to instante a finite field. The type `Elt` determines the type of its elements.
pub trait Field {
    type Elt;
    fn new(&self, _: BigInt) -> Self::Elt;
    fn zero(&self) -> Self::Elt;
    fn one(&self) -> Self::Elt;
}

pub trait CMov {
    fn cmov(x: &Self, y: &Self, b: bool) -> Self;
}

pub trait Sqrt {
    fn is_square(&self) -> bool;
    fn sqrt(&self) -> Self;
}

#[derive(Copy, Clone)]
pub enum Sgn0Choice {
    Sgn0BE,
    Sgn0LE,
}

pub trait Sgn0 {
    fn sgn0_be(&self) -> i32;
    fn sgn0_le(&self) -> i32;
    #[inline]
    fn sgn0(&self, s: Sgn0Choice) -> i32 {
        match s {
            Sgn0Choice::Sgn0BE => self.sgn0_be(),
            Sgn0Choice::Sgn0LE => self.sgn0_le(),
        }
    }
}

pub trait HashToField
where
    Self: Sized,
{
    type Output;
    fn hash<D: Digest + Copy + Sized>(
        &self,
        hash_func: D,
        msg: &[u8],
        dst: &[u8],
        ctr: u8,
        l: usize,
    ) -> Self::Output;
}

pub trait Point: Sized + Add<Output = Self> {}

pub trait Scalar<P>: Sized + Mul<P, Output = P>
where
    P: Point,
{
}

/// Curve trait allows to implement elliptic curve operations.
pub trait EllipticCurve: PartialEq {
    type F: Field;
    type P: Point;
    type S: Scalar<Self::P>;
    type Coordinates;
    fn new_point(&self, _: Self::Coordinates) -> Self::P;
    fn new_scalar(&self, _: BigInt) -> Self::S;
    fn identity(&self) -> Self::P;
    fn get_generator(&self) -> Self::P;
    fn is_on_curve(&self, _: &Self::P) -> bool;
    fn get_order(&self) -> BigUint;
    fn get_cofactor(&self) -> BigInt;
    fn get_field(&self) -> Self::F;
}

#[cfg(test)]
mod tests;

/// Returns the version of the crate.
pub fn version() -> &'static str {
    private_version();
    env!("CARGO_PKG_VERSION")
}

fn private_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
