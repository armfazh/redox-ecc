//! This is documentation for the `ellipticcurve` module.
//!
//! The ellipticcurve module is meant to be used for bar.

use num_bigint::{BigInt, BigUint};

use std::fmt::Display;

use crate::field::Field;
use crate::ops::{AddRef, DivRef, MulRef, NegRef, ScMulRef, SubRef};
/// EcScalar models the behaviour of a scalar to multiply points.
pub trait EcScalar: Display + AddRef + SubRef + MulRef + DivRef + NegRef {}

/// EcPoint models the behaviour of a point on an elliptic curve.
pub trait EcPoint<T>: Display + AddRef + SubRef + NegRef + ScMulRef<T>
where
    T: EcScalar,
{
    fn is_zero(&self) -> bool;
}

/// Curve trait allows to implement elliptic curve operations.
pub trait EllipticCurve {
    type F: Field;
    type Scalar: EcScalar;
    type Point: EcPoint<Self::Scalar>;
    type Coordinates;
    fn identity(&self) -> Self::Point;
    fn new_point(&self, _: Self::Coordinates) -> Self::Point;
    fn new_scalar(&self, _: BigInt) -> Self::Scalar;
    fn get_generator(&self) -> Self::Point;
    fn is_on_curve(&self, _: &Self::Point) -> bool;
    fn get_order(&self) -> BigUint;
    fn get_cofactor(&self) -> BigInt;
    fn get_field(&self) -> Self::F;
}

/// Rational map between two elliptic curves.
pub trait RationalMap {
    type E0: EllipticCurve;
    type E1: EllipticCurve;
    fn domain(&self) -> Self::E0;
    fn codomain(&self) -> Self::E1;
    fn push(&self, p: <Self::E0 as EllipticCurve>::Point) -> <Self::E1 as EllipticCurve>::Point;
    fn pull(&self, p: <Self::E1 as EllipticCurve>::Point) -> <Self::E0 as EllipticCurve>::Point;
}

/// MapToCurve is a deterministic function from an element of the field F
/// to a point on an elliptic curve E defined over F.
pub trait MapToCurve {
    type E: EllipticCurve;
    fn map(
        &self,
        _: <<Self::E as EllipticCurve>::F as Field>::Elt,
    ) -> <Self::E as EllipticCurve>::Point;
}
