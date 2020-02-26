use crate::EllipticCurve;
use crate::Field;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HashID {
    SHA256,
    SHA384,
    SHA512,
}
/// HashToField hashes a string msg of any length into an element of a field F.
/// This function is parametrized by a cryptographic hash function.
pub trait HashToField<F: Field> {
    fn hash(&self, h: HashID, msg: &[u8], dst: &[u8], ctr: u8, l: usize) -> <F as Field>::Elt;
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
    fn hash(&self, msg: &[u8]) -> <Self::E as EllipticCurve>::P;
}

pub struct Encoding<E>
where
    E: EllipticCurve,
{
    pub dst: Vec<u8>,
    pub e: E,
    pub h: HashID,
    pub map_to_curve: Box<dyn MapToCurve<E = E> + 'static>,
    pub hash_to_field: Box<dyn HashToField<<E as EllipticCurve>::F> + 'static>,
    pub cofactor: <E as EllipticCurve>::S,
    pub l: usize,
    pub ro: bool,
}

impl<E> EncodeToCurve for Encoding<E>
where
    E: EllipticCurve,
{
    type E = E;
    fn hash(&self, msg: &[u8]) -> <E as EllipticCurve>::P {
        let p = if self.ro {
            let u0 = self.hash_to_field.hash(self.h, msg, &self.dst, 0u8, self.l);
            let u1 = self.hash_to_field.hash(self.h, msg, &self.dst, 1u8, self.l);
            let p0 = self.map_to_curve.map(u0);
            let p1 = self.map_to_curve.map(u1);
            p0 + p1
        } else {
            let u = self.hash_to_field.hash(self.h, msg, &self.dst, 2u8, self.l);
            self.map_to_curve.map(u)
        };
        p * self.cofactor.clone()
    }
}
