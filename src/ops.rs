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
                    + for<'a> std::ops::$trait<&'a Self, Output = Self>
                    + std::ops::$trait<Self, Output = Self>,
            {
                // for<'a, 'b> &'a Self: Sized
                //     + std::ops::$trait<&'b Self, Output = Self>
                //     + std::ops::$trait<Self, Output = Self>,
            }
        );
        impl<T> $name for T
        where
            for<'a, 'b> T: Sized
                + 'a
                + 'static
                + std::ops::$trait<&'b T, Output = T>
                + std::ops::$trait<T, Output = T>,
        {
            // for<'a, 'b> &'a T:
            // Sized + std::ops::$trait<&'b T, Output = T> + std::ops::$trait<T, Output = T>,
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

/// The Serialize trait performs type serialization into bytes
pub trait Serialize {
    fn to_bytes_be(&self) -> Vec<u8>;
    fn to_bytes_le(&self) -> Vec<u8>;
}

/// The Deserialize trait recovers native types from arrays of bytes
pub trait Deserialize {
    type Deser;
    fn from_bytes_be(&self, _: &[u8]) -> Result<Self::Deser, std::io::Error>;
    fn from_bytes_le(&self, _: &[u8]) -> Result<Self::Deser, std::io::Error>;
}

pub trait IntoFactory<T, Out>: Sized {
    fn lift(self, _: &T) -> Out;
}

impl<T, U, O> IntoFactory<U, O> for T
where
    U: FromFactory<T, Output = O>,
{
    fn lift(self, f: &U) -> O {
        FromFactory::from(f, self)
    }
}

pub trait FromFactory<T: Sized> {
    type Output;
    fn from(&self, _: T) -> Self::Output;
}
