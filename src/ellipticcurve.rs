use num_bigint::{BigInt, BigUint};
use std::ops::{Add, Mul};

use crate::field::Field;

pub trait Scalar {}

/// Curve trait allows to implement elliptic curve operations.
pub trait EllipticCurve: PartialEq {
    type F: Field;
    type S: Mul<Self::P, Output = Self::P> + Clone;
    type P: Add<Output = Self::P> + Mul<Self::S, Output = Self::P>;
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
