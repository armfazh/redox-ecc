//!
//! `redox_ecc` is a reference implementation of elliptic curve operations.
//! The purpose of this library is to provide mathematical operations for elliptic curves.
//!
//! # Warning
//!
//! This implementation is **not** protected against any kind of attack, including
//! side-channel attacks. Do not use this code for securing any application.
//!
//! # Adding Points
//!
//! ```
//!  use redox_ecc::ellipticcurve::EllipticCurve;
//!  use redox_ecc::instances::{GetCurve, CURVE25519, P256};
//!
//!  let ec = P256.get();
//!  let g0 = ec.get_generator();
//!  let g1 = ec.get_generator();
//!  println!("{}\n2G: {} ", ec, g0 + g1);
//!
//!  let ec = CURVE25519.get();
//!  let g0 = ec.get_generator();
//!  let g1 = ec.get_generator();
//!  println!("{}\n2G: {} ", ec, g0 + g1);
//! ```
//!

// Declaring our library as `no-std` unconditionally lets us be consistent
// in how we `use` items from `std` or `core`
#![no_std]

// We always pull in `std` during tests, because it's just easier
// to write tests when you can assume you're on a capable platform
#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

// When we're building for a no-std target, we pull in `core`, but alias
// it as `std` so the `use` statements are the same between `std` and `core`.
#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

// #![warn(missing_docs)]

#[macro_use]
extern crate doc_comment;

#[macro_use]
extern crate impl_ops;

mod macros;

pub mod field;
pub mod ops;
pub mod primefield;
pub mod quadraticfield;

pub mod ellipticcurve;

pub mod edwards;
// pub mod montgomery;
// pub mod weierstrass;

// pub mod instances;

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
