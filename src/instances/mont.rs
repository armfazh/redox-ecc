use crate::instances::GetCurve;
use crate::montgomery::{Curve, Params};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct MtCurveID(&'static Params);

impl GetCurve for MtCurveID {
    type E = Curve;
    #[inline]
    fn get(&self) -> Self::E {
        Curve::from(self.0)
    }
}

impl std::fmt::Display for MtCurveID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

/// CURVE25519 is the curve25519 elliptic curve as specified in RFC-7748.
pub static CURVE25519: MtCurveID = MtCurveID(CURVE25519_PARAMS);
/// CURVE448 is the curve448 elliptic curve as specified in RFC-7748.
pub static CURVE448: MtCurveID = MtCurveID(CURVE448_PARAMS);

static CURVE25519_PARAMS: &Params = &Params {
    name: "curve25519",
    p: "57896044618658097711785492504343953926634992332820282019728792003956564819949",
    a: "486662",
    b: "1",
    s: "1",
    r: "7237005577332262213973186563042994240857116359379907606001950938285454250989",
    h: "8",
    gx: "9",
    gy: "43114425171068552920764898935933967039370386198203806730763910166200978582548",
};

static CURVE448_PARAMS: &Params =  &Params {
    name: "curve448",
    p: "726838724295606890549323807888004534353641360687318060281490199180612328166730772686396383698676545930088884461843637361053498018365439",
    a: "156326",
    b: "1",
    s: "3",
    r: "181709681073901722637330951972001133588410340171829515070372549795146003961539585716195755291692375963310293709091662304773755859649779",
    h: "4",
    gx: "5",
    gy: "355293926785568175264127502063783334808976399387714271831880898435169088786967410002932673765864550910142774147268105838985595290606362",
};
