//! This is documentation for the `field` module.
//!
//! The field module is meant to be used for bar.

use num_bigint::BigInt;
use num_traits::identities::{One, Zero};
use std::fmt::Display;
use std::ops::BitXor;

use crate::ops::{AddRef, DivRef, FromFactory, MulRef, Serialize, SubRef};

/// Sqrt trait adds square-root calculation and quadratic-residue testing.
pub trait Sqrt {
    /// Determines whether an element is a quadratic residue.
    fn is_square(&self) -> bool;
    /// Returns one square-root if the element is a quadratic residue. Otherwise, output is arbitrary.
    /// There is no notion of principal square-root in the ouput.
    fn sqrt(&self) -> Self;
}

#[derive(Copy, Clone)]
pub enum Sgn0Endianness {
    BigEndian,
    LittleEndian,
}

pub trait Sgn0 {
    fn sgn0_be(&self) -> i32;
    fn sgn0_le(&self) -> i32;
    #[inline]
    fn new(s: Sgn0Endianness) -> fn(_: &Self) -> i32 {
        match s {
            Sgn0Endianness::BigEndian => Self::sgn0_be,
            Sgn0Endianness::LittleEndian => Self::sgn0_le,
        }
    }
}

pub trait CMov: Clone {
    fn cmov(x: &Self, y: &Self, b: bool) -> Self {
        if b {
            y.clone()
        } else {
            x.clone()
        }
    }
}

pub trait FieldElement:
    Display + Eq + Zero + One + AddRef + SubRef + MulRef + DivRef + BitXor<u32> + Serialize
{
}

pub trait FromFactoryStr<Out>: for<'a> FromFactory<&'a str, Output = Out> {}

impl<T, U> FromFactoryStr<U> for T where T: for<'a> FromFactory<&'a str, Output = U> {}

pub trait FromFactoryPrimitive<Out>
where
    Self: FromFactory<u8, Output = Out>
        + FromFactory<u16, Output = Out>
        + FromFactory<u32, Output = Out>
        + FromFactory<u64, Output = Out>
        + FromFactory<i8, Output = Out>
        + FromFactory<i16, Output = Out>
        + FromFactory<i32, Output = Out>
        + FromFactory<i64, Output = Out>,
{
}

impl<T, U> FromFactoryPrimitive<U> for T where
    T: FromFactory<u8, Output = U>
        + FromFactory<u16, Output = U>
        + FromFactory<u32, Output = U>
        + FromFactory<u64, Output = U>
        + FromFactory<i8, Output = U>
        + FromFactory<i16, Output = U>
        + FromFactory<i32, Output = U>
        + FromFactory<i64, Output = U>
{
}

/// Field is a fabric to instante a finite field.
pub trait Field
where
    Self: FromFactoryPrimitive<<Self as Field>::Elt> + FromFactoryStr<<Self as Field>::Elt>,
{
    /// `Elt` determines the type of field elements.
    type Elt: FieldElement;
    fn elt(&self, _: BigInt) -> Self::Elt;
    fn zero(&self) -> Self::Elt;
    fn one(&self) -> Self::Elt;
    fn get_modulus(&self) -> BigInt;
    fn size_bytes(&self) -> usize;
}
