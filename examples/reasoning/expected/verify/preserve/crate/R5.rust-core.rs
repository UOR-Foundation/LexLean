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
    F5,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F5 => f5(p0),
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
    let v1: (Adt1, bool) = f2(v0.clone())?;
    let m3 = {
        let (_, h2) = v1.clone();
        h2
    };
    if m3 {
        Ok(f3(v0.clone(), {
            let (h4, _) = v1.clone();
            h4
        }))
    } else {
        Ok(Err::<u64, (bool, bool)>((false, false)))
    }
}

pub fn f2(v0: Adt0) -> R<(Adt1, bool)> {
    f4(Fn0::F5, 11u64, f6(v0.clone()))
}

pub fn f3(v0: Adt0, v1: Adt1) -> Result<u64, (bool, bool)> {
    let m5 = f7(v0.clone(), v1.clone());
    match m5 {
        None => {
            let m6 = f8(v1.clone());
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

pub fn f4(v0: Fn0, v1: u64, v2: Adt1) -> R<(Adt1, bool)> {
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
                f4(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn f5(v0: Adt1) -> R<Option<Adt1>> {
    let m10 = f9(v0.clone())?;
    match m10 {
        None => {
            Ok(None::<Adt1>)
        }
        Some(v1) => {
            f10(v0.clone(), v1.clone())
        }
    }
}

pub fn f6(v0: Adt0) -> Adt1 {
    Adt1::C0(v0.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f7(_: Adt0, v1: Adt1) -> Option<u64> {
    f8(v1.clone())
}

pub fn f8(v0: Adt1) -> Option<u64> {
    Some(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h11) => {
            h11
        }
    })
}

pub fn f9(v0: Adt1) -> R<Option<Adt2>> {
    let m15 = {
        let m13 = {
            let m12 = if f11(v0.clone()) {
                Some(Adt2::C0)
            } else {
                None::<Adt2>
            };
            match m12 {
                None => {
                    if f12(v0.clone()) {
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
        match m13 {
            None => {
                let m14 = if f13(v0.clone()) {
                    Some(Adt2::C2)
                } else {
                    None::<Adt2>
                };
                match m14 {
                    None => {
                        if f14(v0.clone()) {
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
    match m15 {
        None => {
            let m17 = {
                let m16 = if f15(v0.clone())? {
                    Some(Adt2::C4)
                } else {
                    None::<Adt2>
                };
                match m16 {
                    None => {
                        if f16(v0.clone()) {
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
            match m17 {
                None => {
                    let m18 = if f17(v0.clone()) {
                        Some(Adt2::C6)
                    } else {
                        None::<Adt2>
                    };
                    match m18 {
                        None => {
                            if f18(v0.clone()) {
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

pub fn f10(v0: Adt1, v1: Adt2) -> R<Option<Adt1>> {
    let m19 = v1.clone();
    match m19 {
        Adt2::C0 => {
            Ok(f19(v0.clone()))
        }
        Adt2::C1 => {
            Ok(f20(v0.clone()))
        }
        Adt2::C2 => {
            Ok(f21(v0.clone()))
        }
        Adt2::C3 => {
            Ok(f22(v0.clone()))
        }
        Adt2::C4 => {
            f23(v0.clone())
        }
        Adt2::C5 => {
            Ok(f24(v0.clone()))
        }
        Adt2::C6 => {
            Ok(f25(v0.clone()))
        }
        Adt2::C7 => {
            f26(v0.clone())
        }
    }
}

pub fn f11(v0: Adt1) -> bool {
    let a21 = match v0.clone() {
        Adt1::C0(_, h20, _, _, _, _, _, _, _) => {
            h20
        }
    };
    let a22 = 0u64;
    if nat_eq(a21, a22) {
        f27(match v0.clone() {
            Adt1::C0(h23, _, _, _, _, _, _, _, _) => {
                h23
            }
        })
    } else {
        false
    }
}

pub fn f12(v0: Adt1) -> bool {
    let a25 = match v0.clone() {
        Adt1::C0(_, _, h24, _, _, _, _, _, _) => {
            h24
        }
    };
    let a26 = 0u64;
    if nat_eq(a25, a26) {
        f28(match v0.clone() {
            Adt1::C0(h27, _, _, _, _, _, _, _, _) => {
                h27
            }
        })
    } else {
        false
    }
}

pub fn f13(v0: Adt1) -> bool {
    let a29 = match v0.clone() {
        Adt1::C0(_, _, _, h28, _, _, _, _, _) => {
            h28
        }
    };
    let a30 = 0u64;
    if nat_eq(a29, a30) {
        f29(match v0.clone() {
            Adt1::C0(h31, _, _, _, _, _, _, _, _) => {
                h31
            }
        })
    } else {
        false
    }
}

pub fn f14(v0: Adt1) -> bool {
    let a33 = match v0.clone() {
        Adt1::C0(_, _, _, _, h32, _, _, _, _) => {
            h32
        }
    };
    let a34 = 0u64;
    if nat_eq(a33, a34) {
        f30(match v0.clone() {
            Adt1::C0(h35, _, _, _, _, _, _, _, _) => {
                h35
            }
        })
    } else {
        false
    }
}

pub fn f15(v0: Adt1) -> R<bool> {
    let a37 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h36, _, _, _) => {
            h36
        }
    };
    let a38 = 0u64;
    if nat_eq(a37, a38) {
        let a49 = 2u64;
        let a50 = {
            let a47 = {
                let a44 = {
                    let a41 = match v0.clone() {
                        Adt1::C0(_, h39, _, _, _, _, _, _, _) => {
                            h39
                        }
                    };
                    let a42 = match v0.clone() {
                        Adt1::C0(_, _, h40, _, _, _, _, _, _) => {
                            h40
                        }
                    };
                    nat_add(a41, a42)?
                };
                let a45 = match v0.clone() {
                    Adt1::C0(_, _, _, h43, _, _, _, _, _) => {
                        h43
                    }
                };
                nat_add(a44, a45)?
            };
            let a48 = match v0.clone() {
                Adt1::C0(_, _, _, _, h46, _, _, _, _) => {
                    h46
                }
            };
            nat_add(a47, a48)?
        };
        Ok(nat_le(a49, a50))
    } else {
        Ok(false)
    }
}

pub fn f16(v0: Adt1) -> bool {
    let a52 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h51, _, _) => {
            h51
        }
    };
    let a53 = 0u64;
    if nat_eq(a52, a53) {
        let a55 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h54, _, _, _) => {
                h54
            }
        };
        let a56 = 1u64;
        if nat_eq(a55, a56) {
            let a60 = {
                let m59 = match v0.clone() {
                    Adt1::C0(h58, _, _, _, _, _, _, _, _) => {
                        h58
                    }
                };
                match m59 {
                    Adt0::C0(_, _, _, _, _, h57) => {
                        h57
                    }
                }
            };
            let a61 = 1u64;
            nat_eq(a60, a61)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f17(v0: Adt1) -> bool {
    let a63 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h62, _) => {
            h62
        }
    };
    let a64 = 0u64;
    if nat_eq(a63, a64) {
        let a66 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h65, _, _) => {
                h65
            }
        };
        let a67 = 1u64;
        if nat_eq(a66, a67) {
            f31(match v0.clone() {
                Adt1::C0(h68, _, _, _, _, _, _, _, _) => {
                    h68
                }
            })
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f18(v0: Adt1) -> bool {
    let a70 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h69) => {
            h69
        }
    };
    let a71 = 3u64;
    if nat_lt(a70, a71) {
        let a73 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h72) => {
                h72
            }
        };
        let a74 = 0u64;
        if if nat_eq(a73, a74) {
            let a76 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h75, _, _, _) => {
                    h75
                }
            };
            let a77 = 1u64;
            nat_eq(a76, a77)
        } else {
            false
        } {
            true
        } else {
            let a79 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h78) => {
                    h78
                }
            };
            let a80 = 1u64;
            if if nat_eq(a79, a80) {
                let a82 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h81, _, _) => {
                        h81
                    }
                };
                let a83 = 1u64;
                nat_eq(a82, a83)
            } else {
                false
            } {
                true
            } else {
                let a85 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h84) => {
                        h84
                    }
                };
                let a86 = 2u64;
                if nat_eq(a85, a86) {
                    let a88 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h87, _) => {
                            h87
                        }
                    };
                    let a89 = 1u64;
                    nat_eq(a88, a89)
                } else {
                    false
                }
            }
        }
    } else {
        false
    }
}

pub fn f19(v0: Adt1) -> Option<Adt1> {
    if f11(v0.clone()) {
        Some(f32(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f20(v0: Adt1) -> Option<Adt1> {
    if f12(v0.clone()) {
        Some(f33(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f21(v0: Adt1) -> Option<Adt1> {
    if f13(v0.clone()) {
        Some(f34(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f22(v0: Adt1) -> Option<Adt1> {
    if f14(v0.clone()) {
        Some(f35(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f23(v0: Adt1) -> R<Option<Adt1>> {
    if f15(v0.clone())? {
        Ok(Some(f36(v0.clone())))
    } else {
        Ok(None::<Adt1>)
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
    if f18(v0.clone()) {
        Ok(Some(f39(v0.clone())?))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f27(v0: Adt0) -> bool {
    let a91 = 380u64;
    let a92 = match v0.clone() {
        Adt0::C0(h90, _, _, _, _, _) => {
            h90
        }
    };
    nat_lt(a91, a92)
}

pub fn f28(v0: Adt0) -> bool {
    let a94 = 90u64;
    let a95 = match v0.clone() {
        Adt0::C0(_, h93, _, _, _, _) => {
            h93
        }
    };
    nat_lt(a94, a95)
}

pub fn f29(v0: Adt0) -> bool {
    let a97 = 20u64;
    let a98 = match v0.clone() {
        Adt0::C0(_, _, h96, _, _, _) => {
            h96
        }
    };
    nat_lt(a97, a98)
}

pub fn f30(v0: Adt0) -> bool {
    let a100 = 12u64;
    let a101 = match v0.clone() {
        Adt0::C0(_, _, _, h99, _, _) => {
            h99
        }
    };
    nat_lt(a100, a101)
}

pub fn f31(v0: Adt0) -> bool {
    let a103 = match v0.clone() {
        Adt0::C0(_, _, _, _, h102, _) => {
            h102
        }
    };
    let a104 = 90u64;
    nat_lt(a103, a104)
}

pub fn f32(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h105, _, _, _, _, _, _, _, _) => {
            h105
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, h106, _, _, _, _, _, _) => {
            h106
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h107, _, _, _, _, _) => {
            h107
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h108, _, _, _, _) => {
            h108
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h109, _, _, _) => {
            h109
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h110, _, _) => {
            h110
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h111, _) => {
            h111
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h112) => {
            h112
        }
    })
}

pub fn f33(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h113, _, _, _, _, _, _, _, _) => {
            h113
        }
    }, match v0.clone() {
        Adt1::C0(_, h114, _, _, _, _, _, _, _) => {
            h114
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, h115, _, _, _, _, _) => {
            h115
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h116, _, _, _, _) => {
            h116
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h117, _, _, _) => {
            h117
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h118, _, _) => {
            h118
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h119, _) => {
            h119
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h120) => {
            h120
        }
    })
}

pub fn f34(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h121, _, _, _, _, _, _, _, _) => {
            h121
        }
    }, match v0.clone() {
        Adt1::C0(_, h122, _, _, _, _, _, _, _) => {
            h122
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h123, _, _, _, _, _, _) => {
            h123
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, h124, _, _, _, _) => {
            h124
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h125, _, _, _) => {
            h125
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h126, _, _) => {
            h126
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h127, _) => {
            h127
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h128) => {
            h128
        }
    })
}

pub fn f35(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h129, _, _, _, _, _, _, _, _) => {
            h129
        }
    }, match v0.clone() {
        Adt1::C0(_, h130, _, _, _, _, _, _, _) => {
            h130
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h131, _, _, _, _, _, _) => {
            h131
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h132, _, _, _, _, _) => {
            h132
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h133, _, _, _) => {
            h133
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h134, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h135, _) => {
            h135
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h136) => {
            h136
        }
    })
}

pub fn f36(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h137, _, _, _, _, _, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt1::C0(_, h138, _, _, _, _, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h139, _, _, _, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h140, _, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h141, _, _, _, _) => {
            h141
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h142, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h143, _) => {
            h143
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h144) => {
            h144
        }
    })
}

pub fn f37(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h145, _, _, _, _, _, _, _, _) => {
            h145
        }
    }, match v0.clone() {
        Adt1::C0(_, h146, _, _, _, _, _, _, _) => {
            h146
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h147, _, _, _, _, _, _) => {
            h147
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h148, _, _, _, _, _) => {
            h148
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h149, _, _, _, _) => {
            h149
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h150, _, _, _) => {
            h150
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h151, _) => {
            h151
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h152) => {
            h152
        }
    })
}

pub fn f38(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h153, _, _, _, _, _, _, _, _) => {
            h153
        }
    }, match v0.clone() {
        Adt1::C0(_, h154, _, _, _, _, _, _, _) => {
            h154
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h155, _, _, _, _, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h156, _, _, _, _, _) => {
            h156
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h157, _, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h158, _, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h159, _, _) => {
            h159
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h160) => {
            h160
        }
    })
}

pub fn f39(v0: Adt1) -> R<Adt1> {
    Ok(Adt1::C0(match v0.clone() {
        Adt1::C0(h161, _, _, _, _, _, _, _, _) => {
            h161
        }
    }, match v0.clone() {
        Adt1::C0(_, h162, _, _, _, _, _, _, _) => {
            h162
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h163, _, _, _, _, _, _) => {
            h163
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h164, _, _, _, _, _) => {
            h164
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h165, _, _, _, _) => {
            h165
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h166, _, _, _) => {
            h166
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h167, _, _) => {
            h167
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h168, _) => {
            h168
        }
    }, {
        let a170 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h169) => {
                h169
            }
        };
        let a171 = 1u64;
        nat_add(a170, a171)?
    }))
}
