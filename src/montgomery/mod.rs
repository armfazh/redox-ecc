//! This is documentation for the `montgomery` module.
//!
//! The montgomery module is meant to be used for bar.

mod curve;
mod elligator2;
mod point;
mod scalar;

pub use crate::montgomery::curve::{Curve, CurveID, Params};
pub use crate::montgomery::elligator2::Ell2;
pub use crate::montgomery::point::{Point, ProyCoordinates};
pub use crate::montgomery::scalar::Scalar;
