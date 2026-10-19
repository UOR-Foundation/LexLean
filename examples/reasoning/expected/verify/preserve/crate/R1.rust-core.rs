#![no_std]
#![forbid(unsafe_code)]
use core::cmp::Ordering;
use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;
pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);
pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }
pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub fn nat_add(a: u64, b: u64) -> R<u64> { a.checked_add(b).ok_or(Overflow) }
pub fn nat_sub(a: u64, b: u64) -> u64 { a.saturating_sub(b) }
pub fn nat_mul(a: u64, b: u64) -> R<u64> { a.checked_mul(b).ok_or(Overflow) }
pub fn nat_quot(a: u64, b: u64, z: u64) -> u64 { a.checked_div(b).unwrap_or(z) }
pub fn nat_rem(a: u64, b: u64, z: u64) -> u64 { a.checked_rem(b).unwrap_or(z) }
pub fn nat_eq(a: u64, b: u64) -> bool { a == b }
pub fn nat_le(a: u64, b: u64) -> bool { a <= b }
pub fn nat_lt(a: u64, b: u64) -> bool { a < b }
pub fn nat_succ(a: u64) -> R<u64> { a.checked_add(1).ok_or(Overflow) }
pub fn int_add(a: i64, b: i64) -> R<i64> { a.checked_add(b).ok_or(Overflow) }
pub fn int_sub(a: i64, b: i64) -> R<i64> { a.checked_sub(b).ok_or(Overflow) }
pub fn int_mul(a: i64, b: i64) -> R<i64> { a.checked_mul(b).ok_or(Overflow) }
pub fn int_neg(a: i64) -> R<i64> { a.checked_neg().ok_or(Overflow) }
pub fn int_quot(a: i64, b: i64, z: i64) -> R<i64> { if b == 0 { Ok(z) } else { a.checked_div(b).ok_or(Overflow) } }
pub fn int_rem(a: i64, b: i64, z: i64) -> i64 { if b == 0 { z } else { a.wrapping_rem(b) } }
pub fn bool_not(a: bool) -> bool { !a }
pub fn bool_and(a: bool, b: bool) -> bool { a && b }
pub fn bool_or(a: bool, b: bool) -> bool { a || b }

pub trait Same { fn same(&self, other: &Self) -> bool; }
pub trait Key { fn key(&self, other: &Self) -> Ordering; }
macro_rules! scalar {
    ($($t:ty),*) => { $(
        impl Same for $t { fn same(&self, other: &Self) -> bool { self == other } }
        impl Key for $t { fn key(&self, other: &Self) -> Ordering { self.cmp(other) } }
    )* };
}
scalar!(bool, u8, u16, u32, u64, i8, i16, i32, i64);
impl Same for Ordering { fn same(&self, other: &Self) -> bool { self == other } }
impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering {
        match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided }
    }
}
pub fn equal<T: Same>(a: T, b: T) -> bool { a.same(&b) }
pub fn compare<T: Key>(a: T, b: T) -> Ordering { a.key(&b) }

macro_rules! fixed {
    ($m:ident, $t:ident) => {
        pub mod $m {
            pub fn checked_add(a: $t, b: $t) -> Option<$t> { a.checked_add(b) }
            pub fn checked_sub(a: $t, b: $t) -> Option<$t> { a.checked_sub(b) }
            pub fn checked_mul(a: $t, b: $t) -> Option<$t> { a.checked_mul(b) }
            pub fn checked_quot(a: $t, b: $t) -> Option<$t> { a.checked_div(b) }
            pub fn bit_and(a: $t, b: $t) -> $t { a & b }
            pub fn bit_or(a: $t, b: $t) -> $t { a | b }
            pub fn bit_xor(a: $t, b: $t) -> $t { a ^ b }
            pub fn bit_not(a: $t) -> $t { !a }
            pub fn shift_left(a: $t, amount: u32) -> Option<$t> { if amount < $t::BITS { Some(a.wrapping_shl(amount)) } else { None } }
            pub fn shift_right(a: $t, amount: u32) -> Option<$t> { if amount < $t::BITS { Some(a.wrapping_shr(amount)) } else { None } }
            pub fn convert(a: i128) -> Option<$t> { $t::try_from(a).ok() }
        }
    };
}
fixed!(fixed_u8, u8); fixed!(fixed_u16, u16); fixed!(fixed_u32, u32); fixed!(fixed_u64, u64);
fixed!(fixed_i8, i8); fixed!(fixed_i16, i16); fixed!(fixed_i32, i32); fixed!(fixed_i64, i64);
pub fn checked_neg_i8(a: i8) -> Option<i8> { a.checked_neg() }
pub fn checked_neg_i16(a: i16) -> Option<i16> { a.checked_neg() }
pub fn checked_neg_i32(a: i32) -> Option<i32> { a.checked_neg() }
pub fn checked_neg_i64(a: i64) -> Option<i64> { a.checked_neg() }

#[derive(Clone)]
pub enum Adt0 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt0, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Fn0 {
    F7,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F7 => f7(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let m1 = f1(Adt0::C0(v0, v1, v2, v3, v4, v5))?;
    match m1 {
        Ok(v6) => {
            Ok(v6)
        }
        Err(_) => {
            Ok(9u64)
        }
    }
}

pub fn f1(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    f2(v0)
}

pub fn f2(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    f3(v0)
}

pub fn f3(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    let v1: (Adt1, bool) = f4(v0.clone())?;
    let m3 = {
        let (_, h2) = v1;
        h2
    };
    if m3 {
        Ok(f5(v0, {
            let (h4, _) = v1;
            h4
        }))
    } else {
        Ok(Err::<u64, (bool, bool)>((false, false)))
    }
}

pub fn f4(v0: Adt0) -> R<(Adt1, bool)> {
    f6(Fn0::F7, 11u64, f8(v0))
}

pub fn f5(v0: Adt0, v1: Adt1) -> Result<u64, (bool, bool)> {
    let m5 = f9(v0, v1.clone());
    match m5 {
        None => {
            let m6 = f10(v1);
            match m6 {
                None => {
                    Err::<u64, (bool, bool)>((false, true))
                }
                Some(_) => {
                    Err::<u64, (bool, bool)>((true, false))
                }
            }
        }
        Some(v2) => {
            Ok::<u64, (bool, bool)>(v2)
        }
    }
}

pub fn f6(v0: Fn0, p11: u64, p12: Adt1) -> R<(Adt1, bool)> {
    let m10 = (p11, p12);
    {
        let mut m10 = m10;
        loop {
            let step = {
                let (v1, v2) = m10;
                let m7 = v1;
                if m7 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m7 - 1;
                    let m9 = {
                        let c8: Fn0 = v0.clone();
                        c8.apply(v2.clone())?
                    };
                    match m9 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m10 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt1) -> R<Option<Adt1>> {
    let m13 = f11(v0.clone())?;
    match m13 {
        None => {
            Ok(None::<Adt1>)
        }
        Some(v1) => {
            f12(v0, v1)
        }
    }
}

pub fn f8(v0: Adt0) -> Adt1 {
    Adt1::C0(v0, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f9(v0: Adt0, v1: Adt1) -> Option<u64> {
    let m14 = f10(v1);
    match m14 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            if f13(v0, v2) {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f10(v0: Adt1) -> Option<u64> {
    Some(match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h15) => {
            h15
        }
    })
}

pub fn f11(v0: Adt1) -> R<Option<Adt2>> {
    let m19 = {
        let m17 = {
            let m16 = if f14(v0.clone()) {
                Some(Adt2::C0)
            } else {
                None::<Adt2>
            };
            match m16 {
                None => {
                    if f15(v0.clone()) {
                        Some(Adt2::C1)
                    } else {
                        None::<Adt2>
                    }
                }
                Some(v1) => {
                    Some(v1)
                }
            }
        };
        match m17 {
            None => {
                let m18 = if f16(v0.clone()) {
                    Some(Adt2::C2)
                } else {
                    None::<Adt2>
                };
                match m18 {
                    None => {
                        if f17(v0.clone()) {
                            Some(Adt2::C3)
                        } else {
                            None::<Adt2>
                        }
                    }
                    Some(v3) => {
                        Some(v3)
                    }
                }
            }
            Some(v2) => {
                Some(v2)
            }
        }
    };
    match m19 {
        None => {
            let m21 = {
                let m20 = if f18(v0.clone())? {
                    Some(Adt2::C4)
                } else {
                    None::<Adt2>
                };
                match m20 {
                    None => {
                        if f19(v0.clone()) {
                            Some(Adt2::C5)
                        } else {
                            None::<Adt2>
                        }
                    }
                    Some(v5) => {
                        Some(v5)
                    }
                }
            };
            match m21 {
                None => {
                    let m22 = if f20(v0.clone()) {
                        Some(Adt2::C6)
                    } else {
                        None::<Adt2>
                    };
                    match m22 {
                        None => {
                            if f21(v0) {
                                Ok(Some(Adt2::C7))
                            } else {
                                Ok(None::<Adt2>)
                            }
                        }
                        Some(v7) => {
                            Ok(Some(v7))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f12(v0: Adt1, v1: Adt2) -> R<Option<Adt1>> {
    let m23 = v1;
    match m23 {
        Adt2::C0 => {
            Ok(f22(v0))
        }
        Adt2::C1 => {
            Ok(f23(v0))
        }
        Adt2::C2 => {
            Ok(f24(v0))
        }
        Adt2::C3 => {
            Ok(f25(v0))
        }
        Adt2::C4 => {
            f26(v0)
        }
        Adt2::C5 => {
            Ok(f27(v0))
        }
        Adt2::C6 => {
            Ok(f28(v0))
        }
        Adt2::C7 => {
            f29(v0)
        }
    }
}

pub fn f13(v0: Adt0, v1: u64) -> bool {
    let a24 = v1;
    let a25 = 3u64;
    if nat_le(a24, a25) {
        let a26 = v1;
        let a27 = 3u64;
        if nat_eq(a26, a27) {
            true
        } else {
            let a31 = if f30(v0.clone()) {
                let a29 = match v0.clone() {
                    Adt0::C0(_, _, _, _, _, h28) => {
                        h28
                    }
                };
                let a30 = 1u64;
                if nat_eq(a29, a30) {
                    if f31(v0.clone()) {
                        f32(v0)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };
            bool_not(a31)
        }
    } else {
        false
    }
}

pub fn f14(v0: Adt1) -> bool {
    let a33 = match v0.clone() {
        Adt1::C0(_, h32, _, _, _, _, _, _, _) => {
            h32
        }
    };
    let a34 = 0u64;
    if nat_eq(a33, a34) {
        f31(match v0 {
            Adt1::C0(h35, _, _, _, _, _, _, _, _) => {
                h35
            }
        })
    } else {
        false
    }
}

pub fn f15(v0: Adt1) -> bool {
    let a37 = match v0.clone() {
        Adt1::C0(_, _, h36, _, _, _, _, _, _) => {
            h36
        }
    };
    let a38 = 0u64;
    if nat_eq(a37, a38) {
        f32(match v0 {
            Adt1::C0(h39, _, _, _, _, _, _, _, _) => {
                h39
            }
        })
    } else {
        false
    }
}

pub fn f16(v0: Adt1) -> bool {
    let a41 = match v0.clone() {
        Adt1::C0(_, _, _, h40, _, _, _, _, _) => {
            h40
        }
    };
    let a42 = 0u64;
    if nat_eq(a41, a42) {
        f33(match v0 {
            Adt1::C0(h43, _, _, _, _, _, _, _, _) => {
                h43
            }
        })
    } else {
        false
    }
}

pub fn f17(v0: Adt1) -> bool {
    let a45 = match v0.clone() {
        Adt1::C0(_, _, _, _, h44, _, _, _, _) => {
            h44
        }
    };
    let a46 = 0u64;
    if nat_eq(a45, a46) {
        f34(match v0 {
            Adt1::C0(h47, _, _, _, _, _, _, _, _) => {
                h47
            }
        })
    } else {
        false
    }
}

pub fn f18(v0: Adt1) -> R<bool> {
    let a49 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h48, _, _, _) => {
            h48
        }
    };
    let a50 = 0u64;
    if nat_eq(a49, a50) {
        let a61 = 2u64;
        let a62 = {
            let a59 = {
                let a56 = {
                    let a53 = match v0.clone() {
                        Adt1::C0(_, h51, _, _, _, _, _, _, _) => {
                            h51
                        }
                    };
                    let a54 = match v0.clone() {
                        Adt1::C0(_, _, h52, _, _, _, _, _, _) => {
                            h52
                        }
                    };
                    nat_add(a53, a54)?
                };
                let a57 = match v0.clone() {
                    Adt1::C0(_, _, _, h55, _, _, _, _, _) => {
                        h55
                    }
                };
                nat_add(a56, a57)?
            };
            let a60 = match v0 {
                Adt1::C0(_, _, _, _, h58, _, _, _, _) => {
                    h58
                }
            };
            nat_add(a59, a60)?
        };
        Ok(nat_le(a61, a62))
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt1) -> bool {
    let a64 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h63, _, _) => {
            h63
        }
    };
    let a65 = 0u64;
    if nat_eq(a64, a65) {
        let a67 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h66, _, _, _) => {
                h66
            }
        };
        let a68 = 1u64;
        if nat_eq(a67, a68) {
            let a72 = {
                let m71 = match v0 {
                    Adt1::C0(h70, _, _, _, _, _, _, _, _) => {
                        h70
                    }
                };
                match m71 {
                    Adt0::C0(_, _, _, _, _, h69) => {
                        h69
                    }
                }
            };
            let a73 = 1u64;
            nat_eq(a72, a73)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f20(v0: Adt1) -> bool {
    let a75 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h74, _) => {
            h74
        }
    };
    let a76 = 0u64;
    if nat_eq(a75, a76) {
        let a78 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h77, _, _) => {
                h77
            }
        };
        let a79 = 1u64;
        if nat_eq(a78, a79) {
            f30(match v0 {
                Adt1::C0(h80, _, _, _, _, _, _, _, _) => {
                    h80
                }
            })
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f21(v0: Adt1) -> bool {
    let a82 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h81) => {
            h81
        }
    };
    let a83 = 3u64;
    if nat_lt(a82, a83) {
        let a85 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h84) => {
                h84
            }
        };
        let a86 = 0u64;
        if if nat_eq(a85, a86) {
            let a88 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h87, _, _, _) => {
                    h87
                }
            };
            let a89 = 1u64;
            nat_eq(a88, a89)
        } else {
            false
        } {
            true
        } else {
            let a91 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h90) => {
                    h90
                }
            };
            let a92 = 1u64;
            if if nat_eq(a91, a92) {
                let a94 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h93, _, _) => {
                        h93
                    }
                };
                let a95 = 1u64;
                nat_eq(a94, a95)
            } else {
                false
            } {
                true
            } else {
                let a97 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h96) => {
                        h96
                    }
                };
                let a98 = 2u64;
                if nat_eq(a97, a98) {
                    let a100 = match v0 {
                        Adt1::C0(_, _, _, _, _, _, _, h99, _) => {
                            h99
                        }
                    };
                    let a101 = 1u64;
                    nat_eq(a100, a101)
                } else {
                    false
                }
            }
        }
    } else {
        false
    }
}

pub fn f22(v0: Adt1) -> Option<Adt1> {
    if f14(v0.clone()) {
        Some(f35(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f23(v0: Adt1) -> Option<Adt1> {
    if f15(v0.clone()) {
        Some(f36(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f24(v0: Adt1) -> Option<Adt1> {
    if f16(v0.clone()) {
        Some(f37(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f25(v0: Adt1) -> Option<Adt1> {
    if f17(v0.clone()) {
        Some(f38(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f26(v0: Adt1) -> R<Option<Adt1>> {
    if f18(v0.clone())? {
        Ok(Some(f39(v0)))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f27(v0: Adt1) -> Option<Adt1> {
    if f19(v0.clone()) {
        Some(f40(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f28(v0: Adt1) -> Option<Adt1> {
    if f20(v0.clone()) {
        Some(f41(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f29(v0: Adt1) -> R<Option<Adt1>> {
    if f21(v0.clone()) {
        Ok(Some(f42(v0)?))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f30(v0: Adt0) -> bool {
    let a103 = match v0 {
        Adt0::C0(_, _, _, _, h102, _) => {
            h102
        }
    };
    let a104 = 90u64;
    nat_lt(a103, a104)
}

pub fn f31(v0: Adt0) -> bool {
    let a106 = 380u64;
    let a107 = match v0 {
        Adt0::C0(h105, _, _, _, _, _) => {
            h105
        }
    };
    nat_lt(a106, a107)
}

pub fn f32(v0: Adt0) -> bool {
    let a109 = 90u64;
    let a110 = match v0 {
        Adt0::C0(_, h108, _, _, _, _) => {
            h108
        }
    };
    nat_lt(a109, a110)
}

pub fn f33(v0: Adt0) -> bool {
    let a112 = 20u64;
    let a113 = match v0 {
        Adt0::C0(_, _, h111, _, _, _) => {
            h111
        }
    };
    nat_lt(a112, a113)
}

pub fn f34(v0: Adt0) -> bool {
    let a115 = 12u64;
    let a116 = match v0 {
        Adt0::C0(_, _, _, h114, _, _) => {
            h114
        }
    };
    nat_lt(a115, a116)
}

pub fn f35(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h117, _, _, _, _, _, _, _, _) => {
            h117
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, h118, _, _, _, _, _, _) => {
            h118
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h119, _, _, _, _, _) => {
            h119
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h120, _, _, _, _) => {
            h120
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h121, _, _, _) => {
            h121
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h122, _, _) => {
            h122
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h123, _) => {
            h123
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h124) => {
            h124
        }
    })
}

pub fn f36(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h125, _, _, _, _, _, _, _, _) => {
            h125
        }
    }, match v0.clone() {
        Adt1::C0(_, h126, _, _, _, _, _, _, _) => {
            h126
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, h127, _, _, _, _, _) => {
            h127
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h128, _, _, _, _) => {
            h128
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h129, _, _, _) => {
            h129
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h130, _, _) => {
            h130
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h131, _) => {
            h131
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h132) => {
            h132
        }
    })
}

pub fn f37(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h133, _, _, _, _, _, _, _, _) => {
            h133
        }
    }, match v0.clone() {
        Adt1::C0(_, h134, _, _, _, _, _, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h135, _, _, _, _, _, _) => {
            h135
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, h136, _, _, _, _) => {
            h136
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h137, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h138, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h139, _) => {
            h139
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h140) => {
            h140
        }
    })
}

pub fn f38(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h141, _, _, _, _, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt1::C0(_, h142, _, _, _, _, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h143, _, _, _, _, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h144, _, _, _, _, _) => {
            h144
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h145, _, _, _) => {
            h145
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h146, _, _) => {
            h146
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h147, _) => {
            h147
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h148) => {
            h148
        }
    })
}

pub fn f39(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h149, _, _, _, _, _, _, _, _) => {
            h149
        }
    }, match v0.clone() {
        Adt1::C0(_, h150, _, _, _, _, _, _, _) => {
            h150
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h151, _, _, _, _, _, _) => {
            h151
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h152, _, _, _, _, _) => {
            h152
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h153, _, _, _, _) => {
            h153
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h154, _, _) => {
            h154
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h155, _) => {
            h155
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h156) => {
            h156
        }
    })
}

pub fn f40(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h157, _, _, _, _, _, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt1::C0(_, h158, _, _, _, _, _, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h159, _, _, _, _, _, _) => {
            h159
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h160, _, _, _, _, _) => {
            h160
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h161, _, _, _, _) => {
            h161
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h162, _, _, _) => {
            h162
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h163, _) => {
            h163
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h164) => {
            h164
        }
    })
}

pub fn f41(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h165, _, _, _, _, _, _, _, _) => {
            h165
        }
    }, match v0.clone() {
        Adt1::C0(_, h166, _, _, _, _, _, _, _) => {
            h166
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h167, _, _, _, _, _, _) => {
            h167
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h168, _, _, _, _, _) => {
            h168
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h169, _, _, _, _) => {
            h169
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h170, _, _, _) => {
            h170
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h171, _, _) => {
            h171
        }
    }, 1u64, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h172) => {
            h172
        }
    })
}

pub fn f42(v0: Adt1) -> R<Adt1> {
    Ok(Adt1::C0(match v0.clone() {
        Adt1::C0(h173, _, _, _, _, _, _, _, _) => {
            h173
        }
    }, match v0.clone() {
        Adt1::C0(_, h174, _, _, _, _, _, _, _) => {
            h174
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h175, _, _, _, _, _, _) => {
            h175
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h176, _, _, _, _, _) => {
            h176
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h177, _, _, _, _) => {
            h177
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h178, _, _, _) => {
            h178
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h179, _, _) => {
            h179
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h180, _) => {
            h180
        }
    }, {
        let a182 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h181) => {
                h181
            }
        };
        let a183 = 1u64;
        nat_add(a182, a183)?
    }))
}
