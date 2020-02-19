//! This is documentation for the `redox-ecc` crate.
//!
//! The foo crate is meant to be used for bar.

// #![warn(missing_docs)]

extern crate num_bigint;
use num_bigint::{BigInt, BigUint};

mod macros;

pub mod field;

pub mod edwards;
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
/// Curve trait allows to implement elliptic curve operations.
pub trait EllipticCurve: PartialEq {
    type Field;
    type Point;
    type Coordinates;
    type Scalar;
    fn new_point(&self, _: Self::Coordinates) -> Self::Point;
    fn new_scalar(&self, _: BigInt) -> Self::Scalar;
    fn identity(&self) -> Self::Point;
    fn get_generator(&self) -> Self::Point;
    fn is_on_curve(&self, _: &Self::Point) -> bool;
    fn get_order(&self) -> BigUint;
    fn get_field(&self) -> Self::Field;
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
