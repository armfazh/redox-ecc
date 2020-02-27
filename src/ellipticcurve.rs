//! This is documentation for the `ellipticcurve` module.
//!
//! The ellipticcurve module is meant to be used for bar.

use num_bigint::{BigInt, BigUint};

use crate::field::Field;
use crate::ops::{AddRef, DivRef, MulRef, NegRef, ScMulRef, SubRef};

/// EcScalar models the behaviour of a scalar to multiply points.
pub trait EcScalar: AddRef + SubRef + MulRef + DivRef + NegRef {}

/// EcPoint models the behaviour of a point on an elliptic curve.
pub trait EcPoint<T>: AddRef + SubRef + NegRef + ScMulRef<T>
where
    T: EcScalar,
{
}

/// Curve trait allows to implement elliptic curve operations.
pub trait EllipticCurve: Sized {
    type F: Field;
    type Scalar: EcScalar;
    type Point: EcPoint<Self::Scalar>;
    type Coordinates;
    fn new_point(&self, _: Self::Coordinates) -> Self::Point;
    fn new_scalar(&self, _: BigInt) -> Self::Scalar;
    fn identity(&self) -> Self::Point;
    fn get_generator(&self) -> Self::Point;
    fn is_on_curve(&self, _: &Self::Point) -> bool;
    fn get_order(&self) -> BigUint;
    fn get_cofactor(&self) -> BigInt;
}
