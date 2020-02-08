use crate::field::{PrimeField, PrimeFieldElement};

pub struct WeierstassCurve<'a> {
    pub f: &'a PrimeField,
    pub a: PrimeFieldElement<'a>,
    pub b: PrimeFieldElement<'a>,
}

impl std::fmt::Display for WeierstassCurve<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Weierstrass Curve y^2=x^3+ax+b\na: {}\nb: {}",
            self.a, self.b,
        )
    }
}

pub struct WeierstrassProjectivePoint<'a> {
    x: PrimeFieldElement<'a>,
    y: PrimeFieldElement<'a>,
    z: PrimeFieldElement<'a>,
}

impl std::fmt::Display for WeierstrassProjectivePoint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nx: {}\ny: {}\nz: {}", self.x, self.y, self.z)
    }
}

impl<'a> WeierstassCurve<'a> {
    pub fn new_point(
        &self,
        x: PrimeFieldElement<'a>,
        y: PrimeFieldElement<'a>,
        z: PrimeFieldElement<'a>,
    ) -> WeierstrassProjectivePoint<'a> {
        let p = WeierstrassProjectivePoint { x, y, z };
        if self.is_on_curve(&p) {
            return p;
        } else {
            panic!("not a valid point: {}", p)
        }
    }
    pub fn is_on_curve(&self, p: &WeierstrassProjectivePoint) -> bool {
        use num_traits::identities::Zero;
        return ((&p.y.n * &p.y.n)
            - (&p.x.n * &p.x.n * &p.x.n + &self.a.n * &p.x.n + &self.b.n) % p.x.p)
            .is_zero();
    }
}
