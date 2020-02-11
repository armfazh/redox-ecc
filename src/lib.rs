//! This is documentation for the `redox-ecc` crate.
//!
//! The foo crate is meant to be used for bar.

// #![warn(missing_docs)]

#[macro_use]
macro_rules! do_if_eq {
    ($x:expr, $y:expr, $body:stmt, $error: expr) => {
        if $x == $y {
            $body
        } else {
            panic!($error)
        }
    };
}

pub mod curve;
pub mod field;
pub mod scalar;

/// Returns the version of the crate.
pub fn version() -> &'static str {
    private_version();
    env!("CARGO_PKG_VERSION")
}

fn private_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests;
