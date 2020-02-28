use num_traits::identities::Zero;

use crate::edwards::Curve as TeCurve;
use crate::edwards::ProyCoordinates;
use crate::ellipticcurve::{EllipticCurve, RationalMap};
use crate::field::{CMov, Field, FromFactory, Sgn0, Sgn0Endianness, Sqrt};
use crate::h2c::MapToCurve;
use crate::montgomery::h2c::Ell2 as MtEll2;
use crate::montgomery::Curve as MtCurve;
use crate::primefield::FpElt;

pub struct Ell2 {
    sgn0: Sgn0Endianness,
    ratmap: Box<dyn RationalMap<E0 = TeCurve, E1 = MtCurve> + 'static>,
    map_to_curve: Box<dyn MapToCurve<E = MtCurve> + 'static>,
}

impl Ell2 {
    pub fn new(
        e: TeCurve,
        ratmap: Box<dyn RationalMap<E0 = TeCurve, E1 = MtCurve>>,
        sgn0: Sgn0Endianness,
    ) -> Ell2 {
        let ell2 = MtEll2::new(e1, z);
        Ell2 { ratmap, sgn0, ell2 }
    }
}

impl MapToCurve for Ell2 {
    type E = TeCurve;
    fn map(
        &self,
        u: <<Self::E as EllipticCurve>::F as Field>::Elt,
    ) -> <Self::E as EllipticCurve>::Point {
        self.ratmap.pull(self.map_to_curve.map(u))
    }
}
