#[doc(hidden)]
macro_rules! make_trait {
    (binary, $trait:ident, $name:ident) => {
        doc_comment!(
            concat!(
                stringify!($name),
                " with support for references as parameters."
            ),
            pub trait $name
            where
                Self: Sized
                    + for<'b> std::ops::$trait<&'b Self, Output = Self>
                    + std::ops::$trait<Self, Output = Self>,
            {
                // for<'a, 'b> &'a Self: std::ops::$trait<&'b Self, Output = Self>,
                // for<'a, 'b> &'a Self: std::ops::$trait<&'b Self, Output = Self>,
            }
        );
        impl<T> $name for T where
            T: Sized
                + std::ops::$trait<T, Output = T>
                + for<'b> std::ops::$trait<&'b T, Output = T>
        {
        }
    };
    (unary, $trait:ident, $name:ident) => {
        doc_comment!(
            concat!(
                stringify!($name),
                " with support for references as parameters."
            ),
            pub trait $name: Sized + std::ops::$trait<Output = Self> {}
        );
        impl<T> $name for T where T: Sized + std::ops::$trait<Output = Self> {}
    };
    (action, $trait:ident, $name:ident) => {
        doc_comment!(
            concat!(
                stringify!($name),
                " with support for references as parameters."
            ),
            pub trait $name<U>
            where
                U: Sized,
                Self: Sized
                    + std::ops::$trait<U, Output = Self>
                    + for<'b> std::ops::$trait<&'b U, Output = Self>,
            {
                // for<'a, 'b> &'a Self: std::ops::$trait<&'b Self, Output = Self>,
                // for<'a, 'b> &'a Self: std::ops::$trait<&'b Self, Output = Self>,
            }
        );
    };
}

make_trait!(binary, Add, AddRef);
make_trait!(binary, Sub, SubRef);
make_trait!(binary, Mul, MulRef);
make_trait!(binary, Div, DivRef);
make_trait!(unary, Neg, NegRef);
make_trait!(action, Mul, ScMulRef);
