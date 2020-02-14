//! This is documentation for the `weierstrass` module.
//!
//! The Weierstrass module is meant to be used for bar.
mod curve;
mod point;
mod scalar;
pub use crate::weierstrass::curve::Curve;
pub use crate::weierstrass::point::{Coordinates, Point};
pub use crate::weierstrass::scalar::Scalar;
pub const WEIERSTRASS: crate::EllipticCurveModel = ();
