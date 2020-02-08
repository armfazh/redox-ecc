pub mod curve;
pub mod field;
use std::str;

// extern crate num_bigint;
//
// use num_bigint::BigUint;

pub fn version() -> &'static str {
    private_version();
    env!("CARGO_PKG_VERSION")
}

fn private_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests;
