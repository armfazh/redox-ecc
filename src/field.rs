extern crate num_bigint;

use num_bigint::BigUint;
use num_traits::cast::FromPrimitive;
use num_traits::identities::Zero;
use std::ops::Add;

pub fn new_field(modulus: BigUint) -> PrimeField {
    PrimeField { modulus: modulus }
}

pub struct PrimeField {
    modulus: BigUint,
}

impl std::fmt::Display for PrimeField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GF({:#x})", self.modulus)
    }
}

impl PrimeField {
    pub fn new(&self) -> PrimeFieldElement {
        PrimeFieldElement {
            n: BigUint::zero(),
            p: &self.modulus,
        }
    }
    pub fn elt(&self, n: u64) -> PrimeFieldElement {
        PrimeFieldElement {
            n: BigUint::from_u64(n).unwrap(),
            p: &self.modulus,
        }
    }
}
/*
    Implementation
*/
pub struct PrimeFieldElement<'a> {
    pub n: BigUint,
    pub p: &'a BigUint,
}

impl std::fmt::Display for PrimeFieldElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}", self.n)
    }
}

impl<'c, 'a> Add<&'a PrimeFieldElement<'c>> for PrimeFieldElement<'c> {
    type Output = PrimeFieldElement<'c>;
    fn add(self, other: &Self) -> Self {
        Self {
            n: (self.n + &other.n) % self.p,
            p: self.p,
        }
    }
}
impl<'a, 'b, 'c> Add<&'b PrimeFieldElement<'c>> for &'a PrimeFieldElement<'c> {
    type Output = PrimeFieldElement<'c>;
    fn add(self, other: &PrimeFieldElement) -> PrimeFieldElement<'c> {
        PrimeFieldElement {
            n: (&self.n + &other.n) % self.p,
            p: self.p,
        }
    }
}

// impl<'a> Mul<&'a PrimeFieldElement> for PrimeFieldElement {
//     type Output = PrimeFieldElement;
//     fn mul(self, other: &Self) -> Self {
//         Self {
//             n: (self.n * &other.n) % &self.p,
//             p: self.p,
//         }
//     }
// }
// impl<'a, 'b> Mul<&'b PrimeFieldElement> for &'a PrimeFieldElement {
//     type Output = PrimeFieldElement;
//     fn mul(self, other: &PrimeFieldElement) -> PrimeFieldElement {
//         PrimeFieldElement {
//             n: (&self.n * &other.n) % &self.p,
//             p: self.p.clone(),
//         }
//     }
// }
//
// impl Sub for PrimeFieldElement {
//     type Output = PrimeFieldElement;
//     fn sub(self, other: Self) -> Self {
//         Self {
//             n: (self.n - &other.n) % &self.p,
//             p: self.p.clone(),
//         }
//     }
// }
