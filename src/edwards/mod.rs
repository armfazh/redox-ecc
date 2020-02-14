//! This is documentation for the `edwards` module.
//!
//! The edwards module is meant to be used for bar.
mod curve;
mod point;
mod scalar;

// include!("point.in");
pub use crate::edwards::curve::Curve;
pub use crate::edwards::point::{Point, ProyCoordinates};
pub use crate::edwards::scalar::Scalar;
pub const TWISTED_EDWARDS: crate::EllipticCurveModel = ();
