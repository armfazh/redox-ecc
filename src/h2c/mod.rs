use crate::field::{FpElt, PrimeField};
use crate::weierstrass;
use crate::EllipticCurve;
use crate::Field;
use crate::HashToField;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub trait HashToPoint<E, H, M, D>
where
    E: EllipticCurve,
    H: HashToField<Output = <<E as EllipticCurve>::F as Field>::Elt>,
    M: Mapping<<E as EllipticCurve>::F, E>,
    D: Digest + std::marker::Copy,
{
    fn get_curve(&self) -> E;
    fn get_hash_to_field(&self) -> H;
    fn get_map(&self) -> M;
    fn get_hash(&self) -> D;
    fn encode_to_curve(&self, msg: &[u8], dst: &[u8]) -> <E as EllipticCurve>::P {
        let e = self.get_curve();
        let f = self.get_hash_to_field();
        let u = f.hash(self.get_hash(), msg, dst, 2u8, 18usize);
        let p = self.get_map().map(u);
        let h = e.new_scalar(e.get_cofactor());
        h * p
    }
    fn hash_to_curve(&self, msg: &[u8], dst: &[u8]) -> <E as EllipticCurve>::P {
        let e = self.get_curve();
        let f = self.get_hash_to_field();
        let u0 = f.hash(self.get_hash(), msg, dst, 0u8, 18usize);
        let u1 = f.hash(self.get_hash(), msg, dst, 1u8, 18usize);
        let m = self.get_map();
        let p0 = m.map(u0);
        let p1 = m.map(u1);
        let p = p0 + p1;
        let h = e.new_scalar(e.get_cofactor());
        h * p
    }
}

pub struct Encoding<E, D, M>
where
    E: EllipticCurve,
    D: Digest + std::marker::Copy,
    M: Mapping<<E as EllipticCurve>::F, E>,
{
    e: E,
    hash_func: D,
    l: usize,
    mapping: M,
    ro: bool,
}

// impl<E, H, M, D> HashToPoint<E, H, M, D> for Encoding<E, D, M>
// where
//     E: EllipticCurve,
//     H: HashToField<Output = <<E as EllipticCurve>::F as Field>::Elt>,
//     M: Mapping<<E as EllipticCurve>::F, E>,
//     D: Digest + std::marker::Copy,
// {
//     fn get_curve(&self) -> E {
//         self.e
//     }
//     fn get_hash_to_field(&self) -> H {
//         self.e.get_field().get_hash_to_field()
//     }
//     fn get_map(&self) -> M {
//         SSWU::new(self.e)
//     }
//     fn get_hash(&self) -> D {
//         Sha256::new()
//     }
// }

pub trait Mapping<F, E>: Sized
where
    F: Field,
    E: EllipticCurve,
{
    fn map(&self, _: <F as Field>::Elt) -> <E as EllipticCurve>::P;
}
