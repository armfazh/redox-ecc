use crate::field::{FpElt, PrimeField};
use crate::weierstrass;
use crate::EllipticCurve;
use crate::Field;
use crate::HashToField;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha2::Sha512;

pub enum HashID {
    SHA256,
    SHA512,
}

#[derive(PartialEq, Eq)]
pub struct Suite(pub &'static str);

pub trait Mapping<F, E>: Sized
where
    F: Field,
    E: EllipticCurve,
{
    fn map(&self, _: <F as Field>::Elt) -> <E as EllipticCurve>::P;
}

pub trait HashToPoint<E, H, M, D>
where
    E: EllipticCurve,
    H: HashToField<Output = <<E as EllipticCurve>::F as Field>::Elt>,
    M: Mapping<<E as EllipticCurve>::F, E>,
    D: Digest + Clone,
{
    fn get_curve(&self) -> &E;
    fn get_hash_to_field(&self) -> &H;
    fn get_map(&self) -> &M;
    fn get_hash(&self) -> D;
    fn get_size(&self) -> usize;
    fn is_random_oracle(&self) -> bool;
    fn hash(&self, msg: &[u8], dst: &[u8]) -> <E as EllipticCurve>::P {
        if self.is_random_oracle() {
            let e = self.get_curve();
            let f = self.get_hash_to_field();
            let u0 = f.hash(self.get_hash(), msg, dst, 0u8, self.get_size());
            let u1 = f.hash(self.get_hash(), msg, dst, 1u8, self.get_size());
            let m = self.get_map();
            let p0 = m.map(u0);
            let p1 = m.map(u1);
            let p = p0 + p1;
            let h = e.new_scalar(e.get_cofactor());
            h * p
        } else {
            let e = self.get_curve();
            let f = self.get_hash_to_field();
            let u = f.hash(self.get_hash(), msg, dst, 2u8, self.get_size());
            let p = self.get_map().map(u);
            let h = e.new_scalar(e.get_cofactor());
            h * p
        }
    }
}
