//! This is documentation for the `redox-ecc` crate.
//!
//! The foo crate is meant to be used for bar.

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
