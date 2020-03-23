use crate::edwards;
use crate::edwards::Curve as EdCurve;
use crate::edwards::Point as TePoint;
use crate::ellipticcurve::{EcPoint, EllipticCurve, Isogeny, RationalMap};
use crate::field::Field;
use crate::instances::{
    GetCurve, BLS12381G1, BLS12381G1_11ISO, CURVE25519, CURVE448, EDWARDS25519, EDWARDS448,
    SECP256K1, SECP256K1_3ISO,
};
use crate::montgomery;
use crate::montgomery::Curve as MtCurve;
use crate::montgomery::Point as MtPoint;
use crate::ops::FromFactory;
use crate::primefield::FpElt;
use crate::weierstrass::Curve as WeCurve;

pub fn edwards25519_to_curve25519() -> impl RationalMap<E0 = EdCurve, E1 = MtCurve> {
    let e0 = EDWARDS25519.get();
    let e1 = CURVE25519.get();
    let f = e0.get_field();
    let invsqr_d =
        f.from("6853475219497561581579357271197624642482790079785650197046958215289687604742");
    Ed2Mt25519 { e0, e1, invsqr_d }
}

struct Ed2Mt25519 {
    e0: EdCurve,
    e1: MtCurve,
    invsqr_d: FpElt,
}

impl RationalMap for Ed2Mt25519 {
    type E0 = EdCurve;
    type E1 = MtCurve;

    fn domain(&self) -> Self::E0 {
        self.e0.clone()
    }
    fn codomain(&self) -> Self::E1 {
        self.e1.clone()
    }
    fn push(&self, p: TePoint) -> MtPoint {
        if p.is_zero() {
            self.e1.identity()
        } else {
            let (x, y, z) = (&p.c.x, &p.c.y, &p.c.z);
            let t0 = z + y;
            let xx = x * &t0;
            let yy = &self.invsqr_d * z * t0;
            let zz = x * (z - y);
            self.e1.new_proy_point(montgomery::ProyCoordinates {
                x: xx,
                y: yy,
                z: zz,
            })
        }
    }
    fn pull(&self, p: MtPoint) -> TePoint {
        if p.is_zero() {
            self.e0.identity()
        } else if p.is_two_torsion() {
            let ff = self.e0.get_field();
            self.e0.new_proy_point(edwards::ProyCoordinates {
                x: ff.zero(),
                y: -ff.one(),
                t: ff.zero(),
                z: ff.one(),
            })
        } else {
            let (x, y, z) = (&p.c.x, &p.c.y, &p.c.z);
            let add = x + z;
            let sub = x - z;
            let xx = &self.invsqr_d * x * &add;
            let yy = y * &sub;
            let tt = &self.invsqr_d * x * sub;
            let zz = y * add;
            self.e0.new_proy_point(edwards::ProyCoordinates {
                x: xx,
                y: yy,
                t: tt,
                z: zz,
            })
        }
    }
}

pub fn edwards448_to_curve448() -> impl RationalMap<E0 = EdCurve, E1 = MtCurve> {
    let e0 = EDWARDS448.get();
    let e1 = CURVE448.get();
    Ed4isoMt448 { e0, e1 }
}

struct Ed4isoMt448 {
    e0: EdCurve,
    e1: MtCurve,
}

impl RationalMap for Ed4isoMt448 {
    type E0 = EdCurve;
    type E1 = MtCurve;

    fn domain(&self) -> Self::E0 {
        self.e0.clone()
    }
    fn codomain(&self) -> Self::E1 {
        self.e1.clone()
    }
    fn push(&self, p: TePoint) -> MtPoint {
        if p.is_zero() {
            self.e1.identity()
        } else {
            let (x, y, z) = (&p.c.x, &p.c.y, &p.c.z);
            let ff = self.e1.get_field();
            let f2 = ff.from(2);
            let x2 = x ^ 2u32;
            let y2 = y ^ 2u32;
            let z2 = z ^ 2u32;
            let zz = x * &x2; // zz = x^3
            let xx = x * &y2; // xx = x*y^2
            let yy = y * (f2 * z2 - y2 - x2); // yy = y*(2z^2-y^2-x^2)
            self.e1.new_proy_point(montgomery::ProyCoordinates {
                x: xx,
                y: yy,
                z: zz,
            })
        }
    }
    fn pull(&self, p: MtPoint) -> TePoint {
        if p.is_zero() {
            self.e0.identity()
        } else {
            let ff = self.e0.get_field();
            let (f2, f4) = (&ff.from(2), &ff.from(4));
            let (x, y, z) = (&p.c.x, &p.c.y, &p.c.z);
            let x2 = x ^ 2u32;
            let x3 = &x2 * x;
            let x4 = &x2 ^ 2u32;
            let x5 = &x3 * &x2;
            let y2 = y ^ 2u32;
            let z2 = z ^ 2u32;
            let z3 = &z2 * z;
            let z4 = &z2 ^ 2u32;
            let xx = f4 * y * z * (&x2 - &z2);
            let z0 = x4 - f2 * &x2 * &z2 + &z4 + f4 * &y2 * &z2;
            let yy = -(&x5 - f2 * &x3 * &z2 + x * &z4 - f4 * x * &y2 * &z2);
            let z1 = x5 - f2 * &x3 * &z2 + x * z4 - f2 * x2 * &y2 * z - f2 * y2 * z3;
            let tt = &xx * &yy;
            let xx = &xx * &z1;
            let yy = &yy * &z0;
            let zz = z0 * z1;
            self.e0.new_proy_point(edwards::ProyCoordinates {
                x: xx,
                y: yy,
                t: tt,
                z: zz,
            })
        }
    }
}

struct Isosecp256k1 {
    e0: WeCurve,
    e1: WeCurve,
    x_num: Vec<FpElt>,
    x_den: Vec<FpElt>,
    y_num: Vec<FpElt>,
    y_den: Vec<FpElt>,
}

/// Returns a 3-degree isogeny from SECP256K1_3ISO to the SECP256K1 elliptic curve.
pub fn get_isogeny_secp256k1() -> impl Isogeny<E0 = WeCurve, E1 = WeCurve> {
    let curve = SECP256K1.get();
    let f = curve.get_field();
    Isosecp256k1 {
        x_num: vec![
            f.from("0x8e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38daaaaa8c7"),
            f.from("0x07d3d4c80bc321d5b9f315cea7fd44c5d595d2fc0bf63b92dfff1044f17c6581"),
            f.from("0x534c328d23f234e6e2a413deca25caece4506144037c40314ecbd0b53d9dd262"),
            f.from("0x8e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38daaaaa88c"),
        ],
        x_den: vec![
            f.from("0xd35771193d94918a9ca34ccbb7b640dd86cd409542f8487d9fe6b745781eb49b"),
            f.from("0xedadc6f64383dc1df7c4b2d51b54225406d36b641f5e41bbc52a56612a8c6d14"),
            f.one(),
            f.zero(),
        ],
        y_num: vec![
            f.from("0x4bda12f684bda12f684bda12f684bda12f684bda12f684bda12f684b8e38e23c"),
            f.from("0xc75e0c32d5cb7c0fa9d0a54b12a0a6d5647ab046d686da6fdffc90fc201d71a3"),
            f.from("0x29a6194691f91a73715209ef6512e576722830a201be2018a765e85a9ecee931"),
            f.from("0x2f684bda12f684bda12f684bda12f684bda12f684bda12f684bda12f38e38d84"),
        ],
        y_den: vec![
            f.from("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffff93b"),
            f.from("0x7a06534bb8bdb49fd5e9e6632722c2989467c1bfc8e8d978dfb425d2685c2573"),
            f.from("0x6484aa716545ca2cf3a70c3fa8fe337e0a3d21162f0d6299a7bf8192bfd2a76f"),
            f.one(),
        ],
        e0: SECP256K1_3ISO.get(),
        e1: curve,
    }
}

impl Isogeny for Isosecp256k1 {
    type E0 = WeCurve;
    type E1 = WeCurve;
    fn domain(&self) -> Self::E0 {
        self.e0.clone()
    }
    fn codomain(&self) -> Self::E1 {
        self.e1.clone()
    }
    fn push(&self, p: <Self::E0 as EllipticCurve>::Point) -> <Self::E1 as EllipticCurve>::Point {
        let f = self.e0.get_field();
        let x = p.c.x;
        let y = p.c.y;
        let mut x_num = f.zero();
        let mut x_den = f.zero();
        let mut y_num = f.zero();
        let mut y_den = f.zero();

        for i in (0..4).rev() {
            x_num = (x_num * &x) + &self.x_num[i];
            x_den = (x_den * &x) + &self.x_den[i];
            y_num = (y_num * &x) + &self.y_num[i];
            y_den = (y_den * &x) + &self.y_den[i];
        }
        let xx = x_num / x_den;
        let yy = y * (y_num / y_den);
        self.e1.new_point(xx, yy)
    }
}

struct IsoBls12381G1 {
    e0: WeCurve,
    e1: WeCurve,
    x_num: Vec<FpElt>,
    x_den: Vec<FpElt>,
    y_num: Vec<FpElt>,
    y_den: Vec<FpElt>,
}

/// Returns a 11-degree isogeny from BLS12381G1_11ISO to the BLS12381G1 elliptic curve.
pub fn get_isogeny_bls12381g1() -> impl Isogeny<E0 = WeCurve, E1 = WeCurve> {
    let curve = BLS12381G1.get();
    let f = curve.get_field();
    IsoBls12381G1 {
        x_num: vec![
            f.from("0x11a05f2b1e833340b809101dd99815856b303e88a2d7005ff2627b56cdb4e2c85610c2d5f2e62d6eaeac1662734649b7"),
            f.from("0x17294ed3e943ab2f0588bab22147a81c7c17e75b2f6a8417f565e33c70d1e86b4838f2a6f318c356e834eef1b3cb83bb"),
            f.from("0xd54005db97678ec1d1048c5d10a9a1bce032473295983e56878e501ec68e25c958c3e3d2a09729fe0179f9dac9edcb0"),
            f.from("0x1778e7166fcc6db74e0609d307e55412d7f5e4656a8dbf25f1b33289f1b330835336e25ce3107193c5b388641d9b6861"),
            f.from("0xe99726a3199f4436642b4b3e4118e5499db995a1257fb3f086eeb65982fac18985a286f301e77c451154ce9ac8895d9"),
            f.from("0x1630c3250d7313ff01d1201bf7a74ab5db3cb17dd952799b9ed3ab9097e68f90a0870d2dcae73d19cd13c1c66f652983"),
            f.from("0xd6ed6553fe44d296a3726c38ae652bfb11586264f0f8ce19008e218f9c86b2a8da25128c1052ecaddd7f225a139ed84"),
            f.from("0x17b81e7701abdbe2e8743884d1117e53356de5ab275b4db1a682c62ef0f2753339b7c8f8c8f475af9ccb5618e3f0c88e"),
            f.from("0x80d3cf1f9a78fc47b90b33563be990dc43b756ce79f5574a2c596c928c5d1de4fa295f296b74e956d71986a8497e317"),
            f.from("0x169b1f8e1bcfa7c42e0c37515d138f22dd2ecb803a0c5c99676314baf4bb1b7fa3190b2edc0327797f241067be390c9e"),
            f.from("0x10321da079ce07e272d8ec09d2565b0dfa7dccdde6787f96d50af36003b14866f69b771f8c285decca67df3f1605fb7b"),
            f.from("0x6e08c248e260e70bd1e962381edee3d31d79d7e22c837bc23c0bf1bc24c6b68c24b1b80b64d391fa9c8ba2e8ba2d229"),
        ],
        x_den: vec![
            f.from("0x8ca8d548cff19ae18b2e62f4bd3fa6f01d5ef4ba35b48ba9c9588617fc8ac62b558d681be343df8993cf9fa40d21b1c"),
            f.from("0x12561a5deb559c4348b4711298e536367041e8ca0cf0800c0126c2588c48bf5713daa8846cb026e9e5c8276ec82b3bff"),
            f.from("0xb2962fe57a3225e8137e629bff2991f6f89416f5a718cd1fca64e00b11aceacd6a3d0967c94fedcfcc239ba5cb83e19"),
            f.from("0x3425581a58ae2fec83aafef7c40eb545b08243f16b1655154cca8abc28d6fd04976d5243eecf5c4130de8938dc62cd8"),
            f.from("0x13a8e162022914a80a6f1d5f43e7a07dffdfc759a12062bb8d6b44e833b306da9bd29ba81f35781d539d395b3532a21e"),
            f.from("0xe7355f8e4e667b955390f7f0506c6e9395735e9ce9cad4d0a43bcef24b8982f7400d24bc4228f11c02df9a29f6304a5"),
            f.from("0x772caacf16936190f3e0c63e0596721570f5799af53a1894e2e073062aede9cea73b3538f0de06cec2574496ee84a3a"),
            f.from("0x14a7ac2a9d64a8b230b3f5b074cf01996e7f63c21bca68a81996e1cdf9822c580fa5b9489d11e2d311f7d99bbdcc5a5e"),
            f.from("0xa10ecf6ada54f825e920b3dafc7a3cce07f8d1d7161366b74100da67f39883503826692abba43704776ec3a79a1d641"),
            f.from("0x95fc13ab9e92ad4476d6e3eb3a56680f682b4ee96f7d03776df533978f31c1593174e4b4b7865002d6384d168ecdd0a"),
            f.one(),
            f.zero(),
        ],
        y_num: vec![
            f.from("0x90d97c81ba24ee0259d1f094980dcfa11ad138e48a869522b52af6c956543d3cd0c7aee9b3ba3c2be9845719707bb33"),
            f.from("0x134996a104ee5811d51036d776fb46831223e96c254f383d0f906343eb67ad34d6c56711962fa8bfe097e75a2e41c696"),
            f.from("0xcc786baa966e66f4a384c86a3b49942552e2d658a31ce2c344be4b91400da7d26d521628b00523b8dfe240c72de1f6"),
            f.from("0x1f86376e8981c217898751ad8746757d42aa7b90eeb791c09e4a3ec03251cf9de405aba9ec61deca6355c77b0e5f4cb"),
            f.from("0x8cc03fdefe0ff135caf4fe2a21529c4195536fbe3ce50b879833fd221351adc2ee7f8dc099040a841b6daecf2e8fedb"),
            f.from("0x16603fca40634b6a2211e11db8f0a6a074a7d0d4afadb7bd76505c3d3ad5544e203f6326c95a807299b23ab13633a5f0"),
            f.from("0x4ab0b9bcfac1bbcb2c977d027796b3ce75bb8ca2be184cb5231413c4d634f3747a87ac2460f415ec961f8855fe9d6f2"),
            f.from("0x987c8d5333ab86fde9926bd2ca6c674170a05bfe3bdd81ffd038da6c26c842642f64550fedfe935a15e4ca31870fb29"),
            f.from("0x9fc4018bd96684be88c9e221e4da1bb8f3abd16679dc26c1e8b6e6a1f20cabe69d65201c78607a360370e577bdba587"),
            f.from("0xe1bba7a1186bdb5223abde7ada14a23c42a0ca7915af6fe06985e7ed1e4d43b9b3f7055dd4eba6f2bafaaebca731c30"),
            f.from("0x19713e47937cd1be0dfd0b8f1d43fb93cd2fcbcb6caf493fd1183e416389e61031bf3a5cce3fbafce813711ad011c132"),
            f.from("0x18b46a908f36f6deb918c143fed2edcc523559b8aaf0c2462e6bfe7f911f643249d9cdf41b44d606ce07c8a4d0074d8e"),
            f.from("0xb182cac101b9399d155096004f53f447aa7b12a3426b08ec02710e807b4633f06c851c1919211f20d4c04f00b971ef8"),
            f.from("0x245a394ad1eca9b72fc00ae7be315dc757b3b080d4c158013e6632d3c40659cc6cf90ad1c232a6442d9d3f5db980133"),
            f.from("0x5c129645e44cf1102a159f748c4a3fc5e673d81d7e86568d9ab0f5d396a7ce46ba1049b6579afb7866b1e715475224b"),
            f.from("0x15e6be4e990f03ce4ea50b3b42df2eb5cb181d8f84965a3957add4fa95af01b2b665027efec01c7704b456be69c8b604"),
        ],
        y_den: vec![
            f.from("0x16112c4c3a9c98b252181140fad0eae9601a6de578980be6eec3232b5be72e7a07f3688ef60c206d01479253b03663c1"),
            f.from("0x1962d75c2381201e1a0cbd6c43c348b885c84ff731c4d59ca4a10356f453e01f78a4260763529e3532f6102c2e49a03d"),
            f.from("0x58df3306640da276faaae7d6e8eb15778c4855551ae7f310c35a5dd279cd2eca6757cd636f96f891e2538b53dbf67f2"),
            f.from("0x16b7d288798e5395f20d23bf89edb4d1d115c5dbddbcd30e123da489e726af41727364f2c28297ada8d26d98445f5416"),
            f.from("0xbe0e079545f43e4b00cc912f8228ddcc6d19c9f0f69bbb0542eda0fc9dec916a20b15dc0fd2ededda39142311a5001d"),
            f.from("0x8d9e5297186db2d9fb266eaac783182b70152c65550d881c5ecd87b6f0f5a6449f38db9dfa9cce202c6477faaf9b7ac"),
            f.from("0x166007c08a99db2fc3ba8734ace9824b5eecfdfa8d0cf8ef5dd365bc400a0051d5fa9c01a58b1fb93d1a1399126a775c"),
            f.from("0x16a3ef08be3ea7ea03bcddfabba6ff6ee5a4375efa1f4fd7feb34fd206357132b920f5b00801dee460ee415a15812ed9"),
            f.from("0x1866c8ed336c61231a1be54fd1d74cc4f9fb0ce4c6af5920abc5750c4bf39b4852cfe2f7bb9248836b233d9d55535d4a"),
            f.from("0x167a55cda70a6e1cea820597d94a84903216f763e13d87bb5308592e7ea7d4fbc7385ea3d529b35e346ef48bb8913f55"),
            f.from("0x4d2f259eea405bd48f010a01ad2911d9c6dd039bb61a6290e591b36e636a5c871a5c29f4f83060400f8b49cba8f6aa8"),
            f.from("0xaccbb67481d033ff5852c1e48c50c477f94ff8aefce42d28c0f9a88cea7913516f968986f7ebbea9684b529e2561092"),
            f.from("0xad6b9514c767fe3c3613144b45f1496543346d98adf02267d5ceef9a00d9b8693000763e3b90ac11e99b138573345cc"),
            f.from("0x2660400eb2e4f3b628bdd0d53cd76f2bf565b94e72927c1cb748df27942480e420517bd8714cc80d1fadc1326ed06f7"),
            f.from("0xe0fa1d816ddc03e6b24255e0d7819c171c40f65e273b853324efcd6356caa205ca2f570f13497804415473a1d634b8f"),
            f.one(),
        ],
        e0: BLS12381G1_11ISO.get(),
        e1: curve,
    }
}

impl Isogeny for IsoBls12381G1 {
    type E0 = WeCurve;
    type E1 = WeCurve;
    fn domain(&self) -> Self::E0 {
        self.e0.clone()
    }
    fn codomain(&self) -> Self::E1 {
        self.e1.clone()
    }
    fn push(&self, p: <Self::E0 as EllipticCurve>::Point) -> <Self::E1 as EllipticCurve>::Point {
        let f = self.e0.get_field();
        let x = p.c.x;
        let y = p.c.y;
        let mut x_num = f.zero();
        let mut x_den = f.zero();
        let mut y_num = f.zero();
        let mut y_den = f.zero();

        for i in (0..self.x_num.len()).rev() {
            x_num = (x_num * &x) + &self.x_num[i];
            x_den = (x_den * &x) + &self.x_den[i];
        }
        for i in (0..self.y_num.len()).rev() {
            y_num = (y_num * &x) + &self.y_num[i];
            y_den = (y_den * &x) + &self.y_den[i];
        }

        let xx = x_num / x_den;
        let yy = y * (y_num / y_den);
        self.e1.new_point(xx, yy)
    }
}
