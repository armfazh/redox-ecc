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

#[doc(hidden)]
#[macro_export]
macro_rules! make_trait {
    (binary, $trait:ident, $name:ident) => {
        doc_comment!(
            concat!(
                stringify!($name),
                " with support for references as parameters."
            ),
            pub trait $name<'a>: Sized + std::ops::$trait<Output = Self>
            where
                &'a Self: 'a + std::ops::$trait<Self, Output = Self>,
                for<'b> Self: std::ops::$trait<&'b Self, Output = Self>,
                for<'b> &'a Self: std::ops::$trait<&'b Self, Output = Self>,
            {
            }
        );
    };
    (unary, $trait:ident, $name:ident) => {
        doc_comment!(
            concat!(
                stringify!($name),
                " with support for references as parameters."
            ),
            pub trait $name<'a>: Sized + std::ops::$trait<Output = Self>
            where
                &'a Self: 'a + std::ops::$trait<Output = Self>,
            {
            }
        );
    };
}
