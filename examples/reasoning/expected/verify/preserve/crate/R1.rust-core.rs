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
    f2(v0.clone())
}

pub fn f2(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    f3(v0.clone())
}

pub fn f3(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    let v1: (Adt1, bool) = f4(v0.clone())?;
    let m3 = {
        let (_, h2) = v1.clone();
        h2
    };
    if m3 {
        Ok(f5(v0.clone(), {
            let (h4, _) = v1.clone();
            h4
        }))
    } else {
        Ok(Err::<u64, (bool, bool)>((false, false)))
    }
}

pub fn f4(v0: Adt0) -> R<(Adt1, bool)> {
    f6(Fn0::F7, 11u64, f8(v0.clone()))
}

pub fn f5(v0: Adt0, v1: Adt1) -> Result<u64, (bool, bool)> {
    let m5 = f9(v0.clone(), v1.clone());
    match m5 {
        None => {
            let m6 = f10(v1.clone());
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

pub fn f6(v0: Fn0, v1: u64, v2: Adt1) -> R<(Adt1, bool)> {
    let m7 = v1;
    if m7 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m7 - 1;
        let m9 = {
            let c8: Fn0 = v0.clone();
            c8.apply(v2.clone())?
        };
        match m9 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f6(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn f7(v0: Adt1) -> R<Option<Adt1>> {
    let m10 = f11(v0.clone())?;
    match m10 {
        None => {
            Ok(None::<Adt1>)
        }
        Some(v1) => {
            f12(v0.clone(), v1.clone())
        }
    }
}

pub fn f8(v0: Adt0) -> Adt1 {
    Adt1::C0(v0.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f9(v0: Adt0, v1: Adt1) -> Option<u64> {
    let m11 = f10(v1.clone());
    match m11 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            if f13(v0.clone(), v2) {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f10(v0: Adt1) -> Option<u64> {
    Some(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h12) => {
            h12
        }
    })
}

pub fn f11(v0: Adt1) -> R<Option<Adt2>> {
    let m16 = {
        let m14 = {
            let m13 = if f14(v0.clone()) {
                Some(Adt2::C0)
            } else {
                None::<Adt2>
            };
            match m13 {
                None => {
                    if f15(v0.clone()) {
                        Some(Adt2::C1)
                    } else {
                        None::<Adt2>
                    }
                }
                Some(v1) => {
                    Some(v1.clone())
                }
            }
        };
        match m14 {
            None => {
                let m15 = if f16(v0.clone()) {
                    Some(Adt2::C2)
                } else {
                    None::<Adt2>
                };
                match m15 {
                    None => {
                        if f17(v0.clone()) {
                            Some(Adt2::C3)
                        } else {
                            None::<Adt2>
                        }
                    }
                    Some(v3) => {
                        Some(v3.clone())
                    }
                }
            }
            Some(v2) => {
                Some(v2.clone())
            }
        }
    };
    match m16 {
        None => {
            let m18 = {
                let m17 = if f18(v0.clone())? {
                    Some(Adt2::C4)
                } else {
                    None::<Adt2>
                };
                match m17 {
                    None => {
                        if f19(v0.clone()) {
                            Some(Adt2::C5)
                        } else {
                            None::<Adt2>
                        }
                    }
                    Some(v5) => {
                        Some(v5.clone())
                    }
                }
            };
            match m18 {
                None => {
                    let m19 = if f20(v0.clone()) {
                        Some(Adt2::C6)
                    } else {
                        None::<Adt2>
                    };
                    match m19 {
                        None => {
                            if f21(v0.clone()) {
                                Ok(Some(Adt2::C7))
                            } else {
                                Ok(None::<Adt2>)
                            }
                        }
                        Some(v7) => {
                            Ok(Some(v7.clone()))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Some(v6.clone()))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4.clone()))
        }
    }
}

pub fn f12(v0: Adt1, v1: Adt2) -> R<Option<Adt1>> {
    let m20 = v1.clone();
    match m20 {
        Adt2::C0 => {
            Ok(f22(v0.clone()))
        }
        Adt2::C1 => {
            Ok(f23(v0.clone()))
        }
        Adt2::C2 => {
            Ok(f24(v0.clone()))
        }
        Adt2::C3 => {
            Ok(f25(v0.clone()))
        }
        Adt2::C4 => {
            f26(v0.clone())
        }
        Adt2::C5 => {
            Ok(f27(v0.clone()))
        }
        Adt2::C6 => {
            Ok(f28(v0.clone()))
        }
        Adt2::C7 => {
            f29(v0.clone())
        }
    }
}

pub fn f13(v0: Adt0, v1: u64) -> bool {
    let a21 = v1;
    let a22 = 3u64;
    if nat_le(a21, a22) {
        let a23 = v1;
        let a24 = 3u64;
        if nat_eq(a23, a24) {
            true
        } else {
            let a28 = if f30(v0.clone()) {
                let a26 = match v0.clone() {
                    Adt0::C0(_, _, _, _, _, h25) => {
                        h25
                    }
                };
                let a27 = 1u64;
                if nat_eq(a26, a27) {
                    if f31(v0.clone()) {
                        f32(v0.clone())
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };
            bool_not(a28)
        }
    } else {
        false
    }
}

pub fn f14(v0: Adt1) -> bool {
    let a30 = match v0.clone() {
        Adt1::C0(_, h29, _, _, _, _, _, _, _) => {
            h29
        }
    };
    let a31 = 0u64;
    if nat_eq(a30, a31) {
        f31(match v0.clone() {
            Adt1::C0(h32, _, _, _, _, _, _, _, _) => {
                h32
            }
        })
    } else {
        false
    }
}

pub fn f15(v0: Adt1) -> bool {
    let a34 = match v0.clone() {
        Adt1::C0(_, _, h33, _, _, _, _, _, _) => {
            h33
        }
    };
    let a35 = 0u64;
    if nat_eq(a34, a35) {
        f32(match v0.clone() {
            Adt1::C0(h36, _, _, _, _, _, _, _, _) => {
                h36
            }
        })
    } else {
        false
    }
}

pub fn f16(v0: Adt1) -> bool {
    let a38 = match v0.clone() {
        Adt1::C0(_, _, _, h37, _, _, _, _, _) => {
            h37
        }
    };
    let a39 = 0u64;
    if nat_eq(a38, a39) {
        f33(match v0.clone() {
            Adt1::C0(h40, _, _, _, _, _, _, _, _) => {
                h40
            }
        })
    } else {
        false
    }
}

pub fn f17(v0: Adt1) -> bool {
    let a42 = match v0.clone() {
        Adt1::C0(_, _, _, _, h41, _, _, _, _) => {
            h41
        }
    };
    let a43 = 0u64;
    if nat_eq(a42, a43) {
        f34(match v0.clone() {
            Adt1::C0(h44, _, _, _, _, _, _, _, _) => {
                h44
            }
        })
    } else {
        false
    }
}

pub fn f18(v0: Adt1) -> R<bool> {
    let a46 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h45, _, _, _) => {
            h45
        }
    };
    let a47 = 0u64;
    if nat_eq(a46, a47) {
        let a58 = 2u64;
        let a59 = {
            let a56 = {
                let a53 = {
                    let a50 = match v0.clone() {
                        Adt1::C0(_, h48, _, _, _, _, _, _, _) => {
                            h48
                        }
                    };
                    let a51 = match v0.clone() {
                        Adt1::C0(_, _, h49, _, _, _, _, _, _) => {
                            h49
                        }
                    };
                    nat_add(a50, a51)?
                };
                let a54 = match v0.clone() {
                    Adt1::C0(_, _, _, h52, _, _, _, _, _) => {
                        h52
                    }
                };
                nat_add(a53, a54)?
            };
            let a57 = match v0.clone() {
                Adt1::C0(_, _, _, _, h55, _, _, _, _) => {
                    h55
                }
            };
            nat_add(a56, a57)?
        };
        Ok(nat_le(a58, a59))
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt1) -> bool {
    let a61 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h60, _, _) => {
            h60
        }
    };
    let a62 = 0u64;
    if nat_eq(a61, a62) {
        let a64 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h63, _, _, _) => {
                h63
            }
        };
        let a65 = 1u64;
        if nat_eq(a64, a65) {
            let a69 = {
                let m68 = match v0.clone() {
                    Adt1::C0(h67, _, _, _, _, _, _, _, _) => {
                        h67
                    }
                };
                match m68 {
                    Adt0::C0(_, _, _, _, _, h66) => {
                        h66
                    }
                }
            };
            let a70 = 1u64;
            nat_eq(a69, a70)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f20(v0: Adt1) -> bool {
    let a72 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h71, _) => {
            h71
        }
    };
    let a73 = 0u64;
    if nat_eq(a72, a73) {
        let a75 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h74, _, _) => {
                h74
            }
        };
        let a76 = 1u64;
        if nat_eq(a75, a76) {
            f30(match v0.clone() {
                Adt1::C0(h77, _, _, _, _, _, _, _, _) => {
                    h77
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
    let a79 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h78) => {
            h78
        }
    };
    let a80 = 3u64;
    if nat_lt(a79, a80) {
        let a82 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h81) => {
                h81
            }
        };
        let a83 = 0u64;
        if if nat_eq(a82, a83) {
            let a85 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h84, _, _, _) => {
                    h84
                }
            };
            let a86 = 1u64;
            nat_eq(a85, a86)
        } else {
            false
        } {
            true
        } else {
            let a88 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h87) => {
                    h87
                }
            };
            let a89 = 1u64;
            if if nat_eq(a88, a89) {
                let a91 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h90, _, _) => {
                        h90
                    }
                };
                let a92 = 1u64;
                nat_eq(a91, a92)
            } else {
                false
            } {
                true
            } else {
                let a94 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h93) => {
                        h93
                    }
                };
                let a95 = 2u64;
                if nat_eq(a94, a95) {
                    let a97 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h96, _) => {
                            h96
                        }
                    };
                    let a98 = 1u64;
                    nat_eq(a97, a98)
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
        Some(f35(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f23(v0: Adt1) -> Option<Adt1> {
    if f15(v0.clone()) {
        Some(f36(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f24(v0: Adt1) -> Option<Adt1> {
    if f16(v0.clone()) {
        Some(f37(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f25(v0: Adt1) -> Option<Adt1> {
    if f17(v0.clone()) {
        Some(f38(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f26(v0: Adt1) -> R<Option<Adt1>> {
    if f18(v0.clone())? {
        Ok(Some(f39(v0.clone())))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f27(v0: Adt1) -> Option<Adt1> {
    if f19(v0.clone()) {
        Some(f40(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f28(v0: Adt1) -> Option<Adt1> {
    if f20(v0.clone()) {
        Some(f41(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f29(v0: Adt1) -> R<Option<Adt1>> {
    if f21(v0.clone()) {
        Ok(Some(f42(v0.clone())?))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f30(v0: Adt0) -> bool {
    let a100 = match v0.clone() {
        Adt0::C0(_, _, _, _, h99, _) => {
            h99
        }
    };
    let a101 = 90u64;
    nat_lt(a100, a101)
}

pub fn f31(v0: Adt0) -> bool {
    let a103 = 380u64;
    let a104 = match v0.clone() {
        Adt0::C0(h102, _, _, _, _, _) => {
            h102
        }
    };
    nat_lt(a103, a104)
}

pub fn f32(v0: Adt0) -> bool {
    let a106 = 90u64;
    let a107 = match v0.clone() {
        Adt0::C0(_, h105, _, _, _, _) => {
            h105
        }
    };
    nat_lt(a106, a107)
}

pub fn f33(v0: Adt0) -> bool {
    let a109 = 20u64;
    let a110 = match v0.clone() {
        Adt0::C0(_, _, h108, _, _, _) => {
            h108
        }
    };
    nat_lt(a109, a110)
}

pub fn f34(v0: Adt0) -> bool {
    let a112 = 12u64;
    let a113 = match v0.clone() {
        Adt0::C0(_, _, _, h111, _, _) => {
            h111
        }
    };
    nat_lt(a112, a113)
}

pub fn f35(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h114, _, _, _, _, _, _, _, _) => {
            h114
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, h115, _, _, _, _, _, _) => {
            h115
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h116, _, _, _, _, _) => {
            h116
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h117, _, _, _, _) => {
            h117
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h118, _, _, _) => {
            h118
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h119, _, _) => {
            h119
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h120, _) => {
            h120
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h121) => {
            h121
        }
    })
}

pub fn f36(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h122, _, _, _, _, _, _, _, _) => {
            h122
        }
    }, match v0.clone() {
        Adt1::C0(_, h123, _, _, _, _, _, _, _) => {
            h123
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, h124, _, _, _, _, _) => {
            h124
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h125, _, _, _, _) => {
            h125
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h126, _, _, _) => {
            h126
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h127, _, _) => {
            h127
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h128, _) => {
            h128
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h129) => {
            h129
        }
    })
}

pub fn f37(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h130, _, _, _, _, _, _, _, _) => {
            h130
        }
    }, match v0.clone() {
        Adt1::C0(_, h131, _, _, _, _, _, _, _) => {
            h131
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h132, _, _, _, _, _, _) => {
            h132
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, h133, _, _, _, _) => {
            h133
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h134, _, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h135, _, _) => {
            h135
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h136, _) => {
            h136
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h137) => {
            h137
        }
    })
}

pub fn f38(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h138, _, _, _, _, _, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt1::C0(_, h139, _, _, _, _, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h140, _, _, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h141, _, _, _, _, _) => {
            h141
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h142, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h143, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h144, _) => {
            h144
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h145) => {
            h145
        }
    })
}

pub fn f39(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h146, _, _, _, _, _, _, _, _) => {
            h146
        }
    }, match v0.clone() {
        Adt1::C0(_, h147, _, _, _, _, _, _, _) => {
            h147
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h148, _, _, _, _, _, _) => {
            h148
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h149, _, _, _, _, _) => {
            h149
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h150, _, _, _, _) => {
            h150
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h151, _, _) => {
            h151
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h152, _) => {
            h152
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h153) => {
            h153
        }
    })
}

pub fn f40(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h154, _, _, _, _, _, _, _, _) => {
            h154
        }
    }, match v0.clone() {
        Adt1::C0(_, h155, _, _, _, _, _, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h156, _, _, _, _, _, _) => {
            h156
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h157, _, _, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h158, _, _, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h159, _, _, _) => {
            h159
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h160, _) => {
            h160
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h161) => {
            h161
        }
    })
}

pub fn f41(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h162, _, _, _, _, _, _, _, _) => {
            h162
        }
    }, match v0.clone() {
        Adt1::C0(_, h163, _, _, _, _, _, _, _) => {
            h163
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h164, _, _, _, _, _, _) => {
            h164
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h165, _, _, _, _, _) => {
            h165
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h166, _, _, _, _) => {
            h166
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h167, _, _, _) => {
            h167
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h168, _, _) => {
            h168
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h169) => {
            h169
        }
    })
}

pub fn f42(v0: Adt1) -> R<Adt1> {
    Ok(Adt1::C0(match v0.clone() {
        Adt1::C0(h170, _, _, _, _, _, _, _, _) => {
            h170
        }
    }, match v0.clone() {
        Adt1::C0(_, h171, _, _, _, _, _, _, _) => {
            h171
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h172, _, _, _, _, _, _) => {
            h172
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h173, _, _, _, _, _) => {
            h173
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h174, _, _, _, _) => {
            h174
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h175, _, _, _) => {
            h175
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h176, _, _) => {
            h176
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h177, _) => {
            h177
        }
    }, {
        let a179 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h178) => {
                h178
            }
        };
        let a180 = 1u64;
        nat_add(a179, a180)?
    }))
}
