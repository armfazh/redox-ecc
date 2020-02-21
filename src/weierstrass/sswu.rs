use crate::weierstrass::{P256, P521};
use crate::HashToField;
use num_traits::identities::Zero;

use crate::field::{FpElt, PrimeField};
use crate::h2c::{HashID, HashToPoint, Mapping, Suite};
use crate::weierstrass::{Curve, Point, ProyCoordinates};
use crate::{CMov, Field, Sgn0};
use crate::{EllipticCurve, Sgn0Choice};
use crate::{FromFactory, Sqrt};
use crypto::digest::Digest;
use crypto::sha2::{Sha256, Sha512};

#[derive(Clone)]
pub struct SSWU {
    e: Curve,
    c1: FpElt,
    c2: FpElt,
    z: FpElt,
    sgn0: Sgn0Choice,
}

impl SSWU {
    pub fn new(e: Curve, z: FpElt, sgn0: Sgn0Choice) -> SSWU {
        if !SSWU::verify(&e, &z) {
            panic!("wrong input parameters")
        } else {
            let c1 = -&e.b * (1u32 / &e.a);
            let c2 = -(1u32 / &z);
            SSWU { e, c1, c2, z, sgn0 }
        }
    }
    fn verify(e: &Curve, z: &FpElt) -> bool {
        let f = e.get_field();
        let precond1 = !e.a.is_zero(); //              A != 0
        let precond2 = !e.b.is_zero(); //              B != 0
        let cond1 = !z.is_square(); //                 Z is non-square
        let cond2 = *z != f.from(-1); //               Z != -1
        let x = &e.b * &(1u32 / &(z * &e.a)); //       B/(Z*A)
        let gx = &x * &((&x ^ 2u32) + &e.a) + &e.b; // g(B/(Z*A))
        let cond4 = gx.is_square(); //                 g(B/(Z*A)) is square
        precond1 && precond2 && cond1 && cond2 && cond4
    }
}

impl Mapping<PrimeField, Curve> for SSWU {
    fn map(&self, u: FpElt) -> Point {
        let f = self.e.get_field();
        let cmov = FpElt::cmov;
        let s = self.sgn0;
        let mut t1 = &u ^ 2u32; //          0.   t1 = u^2
        t1 = &self.z * &t1; //              1.   t1 = Z * u^2
        let mut t2 = &t1 ^ 2u32; //         2.   t2 = t1^2
        let mut x1 = &t1 + &t2; //          3.   x1 = t1 + t2
        x1 = 1u32 / &x1; //                 4.   x1 = inv0(x1)
        let e1 = x1.is_zero(); //           5.   e1 = x1 == 0
        x1 = x1 + f.one(); //               6.   x1 = x1 + 1
        x1 = cmov(&x1, &self.c2, e1); //    7.   x1 = CMOV(x1, c2, e1)
        x1 = x1 * &self.c1; //              8.   x1 = x1 * c1
        let mut gx1 = &x1 ^ 2u32; //        9.  gx1 = x1^2
        gx1 = gx1 + &self.e.a; //           10. gx1 = gx1 + A
        gx1 = gx1 * &x1; //                 11. gx1 = gx1 * x1
        gx1 = gx1 + &self.e.b; //           12. gx1 = gx1 + B
        let x2 = &t1 * &x1; //              13.  x2 = t1 * x1
        t2 = t1 * t2; //                    14.  t2 = t1 * t2
        let gx2 = &gx1 * &t2; //            15. gx2 = gx1 * t2
        let e2 = gx1.is_square(); //        16.  e2 = is_square(gx1)
        let x = cmov(&x2, &x1, e2); //      17.   x = CMOV(x2, x1, e2)
        let y2 = cmov(&gx2, &gx1, e2); //   18.  y2 = CMOV(gx2, gx1, e2)
        let mut y = y2.sqrt(); //           19.   y = sqrt(y2)
        let e3 = u.sgn0(s) == y.sgn0(s); // 20.  e3 = sgn0(u) == sgn0(y)
        y = cmov(&(-&y), &y, e3); //        21.   y = CMOV(-y, y, e3)
        let z = f.one();
        self.e.new_point(ProyCoordinates { x, y, z })
    }
}

pub struct Encoding<D, M>
where
    D: Digest + Clone,
    M: Mapping<<Curve as EllipticCurve>::F, Curve>,
{
    pub e: Curve,
    pub hash_func: D,
    pub l: usize,
    pub ro: bool,
    pub map: M,
}

impl<D, M> HashToPoint<Curve, PrimeField, M, D> for Encoding<D, M>
where
    D: Digest + Clone,
    M: Mapping<<Curve as EllipticCurve>::F, Curve>,
{
    fn get_curve(&self) -> &Curve {
        &self.e
    }
    fn get_hash_to_field(&self) -> &PrimeField {
        &self.e.f
    }
    fn get_map(&self) -> &M {
        &self.map
    }
    fn get_hash(&self) -> D {
        self.hash_func.clone()
    }
    fn get_size(&self) -> usize {
        self.l
    }
    fn is_random_oracle(&self) -> bool {
        self.ro
    }
}

impl Encoding<Sha256, SSWU> {
    pub fn meto(&self) -> String {
        String::from("hola")
    }
}
impl Encoding<Sha512, SSWU> {}

// impl<D, M> From<Suite> for Encoding<D, M>
// where
//     D: Digest,
//     M: Mapping<<Curve as EllipticCurve>::F, Curve>,
// {
//     fn from(s: Suite) -> Self {
//         match s {
//             P256_SHA256_SSWU_NU_ => {
//                 let e = Curve::from(P256);
//                 let f = e.get_field();
//                 let l = 48;
//                 let z = f.from(-10);
//                 let ro = true;
//                 let s = Sgn0Choice::Sgn0LE;
//                 let map = SSWU::new(e.clone(), z, s);
//                 let hash_func = Sha256::new();
//                 Encoding {
//                     e,
//                     hash_func,
//                     l,
//                     map,
//                     ro,
//                 }
//             }
//             P521_SHA512_SSWU_NU_ => {
//                 let e = Curve::from(P521);
//                 let f = e.get_field();
//                 let l = 96;
//                 let z = f.from(-4);
//                 let ro = false;
//                 let s = Sgn0Choice::Sgn0LE;
//                 let map = SSWU::new(e.clone(), z, s);
//                 let hash_func = Sha512::new();
//                 Encoding {
//                     e,
//                     hash_func,
//                     l,
//                     map,
//                     ro,
//                 }
//             }
//             _ => panic!("Suite not supported"),
//         }
//     }
// }

pub const P256_SHA256_SSWU_NU_: Suite = Suite("P256_SHA256_SSWU_NU_");
pub const P521_SHA512_SSWU_NU_: Suite = Suite("P521_SHA512_SSWU_NU_");
