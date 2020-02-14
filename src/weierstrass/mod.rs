//! This is documentation for the `weierstrass` module.
//!
//! The Weierstrass module is meant to be used for bar.
mod curve;
mod point;
mod scalar;
pub use crate::weierstrass::curve::WeierstrassCurve as Curve;
pub use crate::weierstrass::point::WeierstrassPoint as Point;
pub use crate::weierstrass::scalar::WeierstrassScalar as Scalar;
