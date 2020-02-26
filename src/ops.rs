#[doc(hidden)]
macro_rules! make_trait {
    (binary, $trait:ident, $name:ident) => {
        doc_comment!(
            concat!(
                stringify!($name),
                " with support for references as parameters."
            ),
            pub trait $name: Sized + std::ops::$trait<Self, Output = Self> {
                // for<'a, 'b> &'a Self: std::ops::$trait<Self, Output = Self>
                //     + std::ops::$trait<&'b Self, Output = Self>
            }
        );
    };
    (unary, $trait:ident, $name:ident) => {
        doc_comment!(
            concat!(
                stringify!($name),
                " with support for references as parameters."
            ),
            pub trait $name: Sized + std::ops::$trait<Output = Self> {
                // for<'a> &'a Self: 'a + std::ops::$trait<Output = Self>,
            }
        );
    };
}

make_trait!(binary, Add, AddRef);
make_trait!(binary, Sub, SubRef);
make_trait!(binary, Mul, MulRef);
make_trait!(binary, Div, DivRef);
make_trait!(unary, Neg, NegRef);

pub trait PowRef<'a, X>: Sized + num_traits::pow::Pow<X, Output = Self>
where
    &'a Self: 'a + num_traits::pow::Pow<X, Output = Self>,
    for<'b> Self: num_traits::pow::Pow<&'b X, Output = Self>,
    for<'b> &'a Self: num_traits::pow::Pow<&'b X, Output = Self>,
{
}
