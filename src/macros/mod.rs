//! This is documentation for the `macros` module.
//!
//! The macros module is meant to be used for bar.

#[doc(hidden)]
#[macro_export]
macro_rules! do_if_eq {
    ($x:expr, $y:expr, $body:stmt, $error: expr) => {
        if $x == $y {
            $body
        } else {
            panic!($error)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_unary_op {
    ($target:ident,
     $trait:ident,
     $name:ident,
     $method:ident) => {
        impl<'a> $trait for &'a $target {
            type Output = $target;
            #[inline]
            fn $name(self) -> Self::Output {
                self.$method()
            }
        }
        impl $trait for $target {
            type Output = $target;
            #[inline]
            fn $name(self) -> Self::Output {
                self.$method()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_binary_op {
    ($target:ident,
     $trait:ident,
     $name:ident,
     $method:ident,
     $field:ident,
     $error:ident) => {
        impl<'a, 'b> $trait<&'b $target> for &'a $target {
            type Output = $target;
            #[inline]
            fn $name(self, other: &$target) -> Self::Output {
                do_if_eq!(self.$field, other.$field, self.$method(&other), $error)
            }
        }
        impl<'a> $trait<&'a $target> for $target {
            type Output = $target;
            #[inline]
            fn $name(self, other: &'a Self) -> Self::Output {
                do_if_eq!(self.$field, other.$field, self.$method(&other), $error)
            }
        }
        impl $trait for $target {
            type Output = $target;
            #[inline]
            fn $name(self, other: Self) -> Self::Output {
                do_if_eq!(self.$field, other.$field, self.$method(&other), $error)
            }
        }
    };
}
