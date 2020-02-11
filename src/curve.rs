use crate::field::PrimeFieldElement;
use std::ops::Add;

pub struct WeierstassCurve {
    // pub f: &'a PrimeField,
    pub a: PrimeFieldElement,
    pub b: PrimeFieldElement,
}

impl std::fmt::Display for WeierstassCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Weierstrass Curve y^2=x^3+ax+b\na: {}\nb: {}",
            self.a, self.b,
        )
    }
}

#[derive(Clone)]
pub struct WeierstrassProjectivePoint {
    x: PrimeFieldElement,
    y: PrimeFieldElement,
    z: PrimeFieldElement,
}

impl std::fmt::Display for WeierstrassProjectivePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nx: {}\ny: {}\nz: {}", self.x, self.y, self.z)
    }
}

impl WeierstassCurve {
    pub fn new_point(
        &self,
        x: PrimeFieldElement,
        y: PrimeFieldElement,
        z: PrimeFieldElement,
    ) -> WeierstrassProjectivePoint {
        let p = WeierstrassProjectivePoint { x, y, z };
        if self.is_on_curve(&p) {
            return p;
        } else {
            panic!("not a valid point: {}", p)
        }
    }
    pub fn is_on_curve(&self, p: &WeierstrassProjectivePoint) -> bool {
        use num_traits::identities::Zero;
        return ((&p.y * &p.y) - &((&p.x * &p.x + &self.a) * &p.x + &self.b)).is_zero();
    }
}

impl Add for WeierstrassProjectivePoint {
    type Output = WeierstrassProjectivePoint;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl<'a> Add<&'a WeierstrassProjectivePoint> for WeierstrassProjectivePoint {
    type Output = WeierstrassProjectivePoint;
    fn add(self, other: &Self) -> Self {
        Self {
            x: self.x + &other.x,
            y: self.y + &other.y,
            z: self.z + &other.z,
        }
    }
}

impl<'a, 'b> Add<&'b WeierstrassProjectivePoint> for &'a WeierstrassProjectivePoint {
    type Output = WeierstrassProjectivePoint;
    fn add(self, other: &WeierstrassProjectivePoint) -> WeierstrassProjectivePoint {
        WeierstrassProjectivePoint {
            x: &self.x + &other.x,
            y: &self.y + &other.y,
            z: &self.z + &other.z,
        }
    }
}
