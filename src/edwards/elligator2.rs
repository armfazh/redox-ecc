use crate::edwards::Curve as TeCurve;
use crate::ellipticcurve::{EllipticCurve, MapToCurve, RationalMap};
use crate::field::{Field, Sgn0Endianness};
use crate::montgomery::Curve as MtCurve;
use crate::montgomery::Ell2 as MtEll2;
use crate::primefield::FpElt;

pub struct Ell2 {
    ratmap: Box<dyn RationalMap<E0 = TeCurve, E1 = MtCurve> + 'static>,
    map_to_curve: Box<dyn MapToCurve<E = MtCurve> + 'static>,
}

impl Ell2 {
    pub fn new(
        e: TeCurve,
        z: FpElt,
        sgn0: Sgn0Endianness,
        ratmap: Option<Box<dyn RationalMap<E0 = TeCurve, E1 = MtCurve>>>,
    ) -> Ell2 {
        let (map_to_curve, ratmap) = match ratmap {
            None => {
                // If no ratmap is provided, it must use the cannonical birational map.
                todo!()
            }
            Some(r) => {
                if r.domain() != e {
                    panic!("Domain of rational map is incompatible with curve")
                }
                let mt_curve = r.codomain();
                (Box::new(MtEll2::new(mt_curve, z, sgn0)), r)
            }
        };
        Ell2 {
            map_to_curve,
            ratmap,
        }
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
