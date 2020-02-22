use crate::EllipticCurve;
use crate::Field;
use crypto::digest::Digest;

/// HashToField hashes a string msg of any length into an element of a field F.
/// This function is parametrized by a cryptographic hash function.
pub trait HashToField: Field {
    fn hash<D: Digest + Copy>(
        &self,
        hash_func: D,
        msg: &[u8],
        dst: &[u8],
        ctr: u8,
        l: usize,
    ) -> <Self as Field>::Elt;
}

/// MapToCurve is a deterministic function from an element of the field F
/// to a point on an elliptic curve E defined over F.
pub trait MapToCurve {
    type E: EllipticCurve;
    fn map(
        &self,
        _: <<Self::E as EllipticCurve>::F as Field>::Elt,
    ) -> <Self::E as EllipticCurve>::P;
}

/// EncodeToCurve is a function that outputs a point on an elliptic curve from an
/// arbitrary string.
pub trait EncodeToCurve {
    type E: EllipticCurve;
    fn hash(&self, msg: &[u8], dst: &[u8]) -> <Self::E as EllipticCurve>::P;
}

pub struct Encoding<E, D, M>
where
    E: EllipticCurve,
    D: Digest + Copy,
    M: MapToCurve<E = E>,
    <E as EllipticCurve>::F: HashToField,
{
    pub e: E,
    pub h: fn() -> D,
    pub map_to_curve: M,
    pub l: usize,
    pub ro: bool,
}

impl<E, D, M> EncodeToCurve for Encoding<E, D, M>
where
    E: EllipticCurve,
    D: Digest + Copy,
    M: MapToCurve<E = E>,
    <E as EllipticCurve>::F: HashToField,
{
    type E = E;
    fn hash(&self, msg: &[u8], dst: &[u8]) -> <E as EllipticCurve>::P {
        let f = self.e.get_field();
        let p = if self.ro {
            let u0 = f.hash((self.h)(), msg, dst, 0u8, self.l);
            let u1 = f.hash((self.h)(), msg, dst, 1u8, self.l);
            let p0 = self.map_to_curve.map(u0);
            let p1 = self.map_to_curve.map(u1);
            p0 + p1
        } else {
            let u = f.hash((self.h)(), msg, dst, 2u8, self.l);
            self.map_to_curve.map(u)
        };
        let h = self.e.new_scalar(self.e.get_cofactor());
        h * p
    }
}
