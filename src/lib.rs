//! `redox_ecc` is a reference implementation of elliptic curve operations.
//!
//! The purpose of this library is to provide mathematical operations used in some elliptic curves.
//!
//! # Warning
//!
//! This is implementation is **not** protected against any kind of attack, including
//! side-channel attacks. Do not use this code for securing any application.
//!
//! # Hash to Curve Example
//!
//! ```
//!  use redox_ecc::ellipticcurve::EllipticCurve;
//!  use redox_ecc::h2c::HashToCurve;
//!  use redox_ecc::instances::{CURVE25519, P256};
//!  use redox_ecc::suites::{
//!      EDWARDS25519_SHA256_EDELL2_RO_, EDWARDS25519_SHA512_EDELL2_NU_, EDWARDS25519_SHA512_EDELL2_RO_,
//!      EDWARDS448_SHA512_EDELL2_NU_, EDWARDS448_SHA512_EDELL2_RO_, P256_SHA256_SSWU_RO_,
//!  };
//!  let msg = "This is a message string".as_bytes();
//!  let dst = "QUUX-V01-CS02".as_bytes();
//!
//!  let ec = P256.get();
//!  let g0 = ec.get_generator();
//!  let g1 = ec.get_generator();
//!  println!("{}\n2G: {} ", ec, g0 + g1);
//!  let suite = P256_SHA256_SSWU_RO_;
//!  let h = suite.get(dst);
//!  let mut p = h.hash(msg);
//!  p.normalize();
//!  println!("enc: {} {} ", suite, p);
//!
//!  let ec = CURVE25519.get();
//!  let g0 = ec.get_generator();
//!  let g1 = ec.get_generator();
//!  println!("{}\n2G: {} ", ec, g0 + g1);
//!  let suite = EDWARDS25519_SHA512_EDELL2_RO_;
//!  let h = suite.get(dst);
//!  let mut p = h.hash(msg);
//!  p.normalize();
//!  println!("enc: {} {} ", suite, p);
//! ```
//!

// #![warn(missing_docs)]

#[macro_use]
extern crate doc_comment;

#[macro_use]
extern crate impl_ops;

mod macros;

pub mod field;
pub mod ops;
pub mod primefield;

pub mod ellipticcurve;
pub mod h2c;

pub mod edwards;
pub mod montgomery;
pub mod weierstrass;

pub mod instances;
pub mod suites;

#[cfg(test)]
mod tests;

/// Returns the version of the crate.
pub fn version() -> &'static str {
    private_version();
    env!("CARGO_PKG_VERSION")
}

fn private_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
