extern crate num_bigint;

use num_bigint::BigUint;
use num_traits::cast::FromPrimitive;
use std::ops::{Add, Mul, Sub};

pub struct PrimeField {
    modulus: BigUint,
}

impl std::fmt::Display for PrimeField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GF({:#x})", self.modulus)
    }
}

impl PrimeField {
    pub fn new(modulus: BigUint) -> PrimeField {
        PrimeField { modulus: modulus }
    }
    pub fn elt(&self, n: u64) -> PrimeFieldElement {
        PrimeFieldElement {
            n: BigUint::from_u64(n).unwrap(),
            p: self.modulus.clone(),
        }
    }
}
/*
    Implementation
*/
#[derive(Clone)]
pub struct PrimeFieldElement {
    n: BigUint,
    p: BigUint,
}

impl std::fmt::Display for PrimeFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}", self.n)
    }
}

impl Add for PrimeFieldElement {
    type Output = PrimeFieldElement;
    fn add(self, other: Self) -> Self {
        Self {
            n: (self.n + other.n) % &self.p,
            p: self.p,
        }
    }
}

impl<'a> Add<&'a PrimeFieldElement> for PrimeFieldElement {
    type Output = PrimeFieldElement;
    fn add(self, other: &Self) -> Self {
        Self {
            n: (self.n + &other.n) % &self.p,
            p: self.p,
        }
    }
}
impl<'a> Sub<&'a PrimeFieldElement> for PrimeFieldElement {
    type Output = PrimeFieldElement;
    fn sub(self, other: &Self) -> Self {
        Self {
            n: (self.n - &other.n) % &self.p,
            p: self.p,
        }
    }
}
impl<'a> Mul<&'a PrimeFieldElement> for PrimeFieldElement {
    type Output = PrimeFieldElement;
    fn mul(self, other: &Self) -> Self {
        Self {
            n: (self.n * &other.n) % &self.p,
            p: self.p,
        }
    }
}
impl<'a, 'b> Add<&'b PrimeFieldElement> for &'a PrimeFieldElement {
    type Output = PrimeFieldElement;
    fn add(self, other: &PrimeFieldElement) -> PrimeFieldElement {
        PrimeFieldElement {
            n: (&self.n + &other.n) % &self.p,
            p: self.p.clone(),
        }
    }
}
impl<'a, 'b> Sub<&'b PrimeFieldElement> for &'a PrimeFieldElement {
    type Output = PrimeFieldElement;
    fn sub(self, other: &PrimeFieldElement) -> PrimeFieldElement {
        PrimeFieldElement {
            n: (&self.n - &other.n) % &self.p,
            p: self.p.clone(),
        }
    }
}
impl<'a, 'b> Mul<&'b PrimeFieldElement> for &'a PrimeFieldElement {
    type Output = PrimeFieldElement;
    fn mul(self, other: &PrimeFieldElement) -> PrimeFieldElement {
        PrimeFieldElement {
            n: (&self.n * &other.n) % &self.p,
            p: self.p.clone(),
        }
    }
}

impl num_traits::identities::Zero for PrimeFieldElement {
    fn zero() -> Self {
        unimplemented!()
    }
    fn is_zero(&self) -> bool {
        (&self.n % &self.p).is_zero()
    }
}
