//! This is documentation for the `ellipticcurve` module.
//!
//! The ellipticcurve module is meant to be used for bar.

use heapless::Vec;
use num_bigint::{BigInt, BigUint};

use std::fmt::Display;

use crate::field::Field;
use crate::ops::{AddRef, DeserError, DivRef, MulRef, NegRef, ScMulRef, Serialize, SubRef};
/// EcScalar models the behaviour of a scalar to multiply points.
pub trait EcScalar: Display + AddRef + SubRef + MulRef + DivRef + NegRef {}

/// EcPoint models the behaviour of a point on an elliptic curve.
pub trait EcPoint<T>: Display + AddRef + SubRef + NegRef + ScMulRef<T> + Encode + Eq
where
    T: EcScalar,
{
    fn is_zero(&self) -> bool;
}

/// Encode provides functionality for encoding elliptic curve points as
/// octet-strings
pub trait Encode {
    fn encode<const C: usize>(&self, compress: bool) -> Vec<u8, C>;
}

/// Decode provides functionality for decoding octet-strings into
/// elliptic curve points
pub trait Decode {
    type Deser;
    fn decode(&self, _: &[u8]) -> Result<Self::Deser, DeserError>;
}

/// Curve trait allows to implement elliptic curve operations.
pub trait EllipticCurve: Decode {
    type F: Field;
    type Scalar: EcScalar;
    type Point: EcPoint<Self::Scalar>;
    fn identity(&self) -> Self::Point;
    fn new_point(&self, x: <Self::F as Field>::Elt, y: <Self::F as Field>::Elt) -> Self::Point;
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

/// Isogeny is a rational map between two elliptic curves.
pub trait Isogeny {
    type E0: EllipticCurve;
    type E1: EllipticCurve;
    fn domain(&self) -> Self::E0;
    fn codomain(&self) -> Self::E1;
    fn push(&self, p: <Self::E0 as EllipticCurve>::Point) -> <Self::E1 as EllipticCurve>::Point;
}

/// MapToCurve is a deterministic function from an element of the field F
/// to a point on an elliptic curve E defined over F.
pub trait MapToCurve {
    type E: EllipticCurve;
    fn map(
        &self,
        _: &<<Self::E as EllipticCurve>::F as Field>::Elt,
    ) -> <Self::E as EllipticCurve>::Point;
}
