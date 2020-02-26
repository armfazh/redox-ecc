//! This is documentation for the `macros` module.
//!
//! The macros module is meant to be used for bar.

#[doc(hidden)]
#[macro_export]
macro_rules! do_if_eq {
    ($cond:expr, $body:stmt, $error: expr) => {
        if $cond {
            $body
        } else {
            panic!($error)
        }
    };
}
