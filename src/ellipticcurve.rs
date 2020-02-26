//! This is documentation for the `ellipticcurve` module.
//!
//! The ellipticcurve module is meant to be used for bar.

use num_bigint::{BigInt, BigUint};

use crate::field::Field;
use crate::ops::{AddRef, DivRef, MulRef, NegRef, SubRef};
use std::ops::Add;

/// EcScalar models the behaviour of a scalar to multiply points.
pub trait EcScalar
where
    Self: AddRef + SubRef + MulRef + DivRef + NegRef,
{
}

/// EcPoint models the behaviour of a point on an elliptic curve.
pub trait EcPoint {}

//     Sized
//     + Add<E::Point, Output = E::Point>
//     + for<'a> Add<&'a E::Point, Output = E::Point>
//     + Sub<E::Point, Output = E::Point>
// {
// }
// impl<'a, E> Add<&'a E::Point> for E::Point where
//     E: EllipticCurve // Self: Sized + Add<Output = E::Point> + Sub<Output = E::Point>,
// {
// }
//
// impl<T, E> EcPoint<E> for T
// where
//     E: EllipticCurve,
//     Self: Sized
//
//         + Sub<Self, Output = Self>,
// {
// }

/// Curve trait allows to implement elliptic curve operations.
pub trait EllipticCurve: Sized {
    type F: Field;
    type Scalar: EcScalar;
    type Point: Add<Output = Self::Point>;
    type Coordinates;
    fn new_point(&self, _: Self::Coordinates) -> Self::Point;
    fn new_scalar(&self, _: BigInt) -> Self::Scalar;
    fn identity(&self) -> Self::Point;
    fn get_generator(&self) -> Self::Point;
    fn is_on_curve(&self, _: &Self::Point) -> bool;
    fn get_order(&self) -> BigUint;
    fn get_cofactor(&self) -> BigInt;
}
