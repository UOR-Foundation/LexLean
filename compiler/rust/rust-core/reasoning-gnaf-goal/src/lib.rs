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

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<(u64, (u64, (u64, (u64, u64))))> {
    Ok((f1(v0)?, (f1(v1)?, (f1(v2)?, (f1(v3)?, f1(v4)?)))))
}

pub fn f1(v0: u64) -> R<u64> {
    let a51 = {
        let a41 = {
            let a31 = 2u64;
            let a32 = {
                let a29 = {
                    let a13 = {
                        let a4 = {
                            let a1 = v0;
                            let a2 = 1u64;
                            let a3 = 0u64;
                            nat_quot(a1, a2, a3)
                        };
                        let a5 = 2u64;
                        let a6 = 0u64;
                        nat_rem(a4, a5, a6)
                    };
                    let a14 = {
                        let a10 = {
                            let a7 = v0;
                            let a8 = 2u64;
                            let a9 = 0u64;
                            nat_quot(a7, a8, a9)
                        };
                        let a11 = 2u64;
                        let a12 = 0u64;
                        nat_rem(a10, a11, a12)
                    };
                    nat_add(a13, a14)?
                };
                let a30 = {
                    let a27 = {
                        let a18 = {
                            let a15 = v0;
                            let a16 = 4u64;
                            let a17 = 0u64;
                            nat_quot(a15, a16, a17)
                        };
                        let a19 = 2u64;
                        let a20 = 0u64;
                        nat_rem(a18, a19, a20)
                    };
                    let a28 = {
                        let a24 = {
                            let a21 = v0;
                            let a22 = 8u64;
                            let a23 = 0u64;
                            nat_quot(a21, a22, a23)
                        };
                        let a25 = 2u64;
                        let a26 = 0u64;
                        nat_rem(a24, a25, a26)
                    };
                    nat_add(a27, a28)?
                };
                nat_add(a29, a30)?
            };
            nat_le(a31, a32)
        };
        let a42 = {
            let a39 = {
                let a36 = {
                    let a33 = v0;
                    let a34 = 16u64;
                    let a35 = 0u64;
                    nat_quot(a33, a34, a35)
                };
                let a37 = 2u64;
                let a38 = 0u64;
                nat_rem(a36, a37, a38)
            };
            let a40 = 1u64;
            nat_eq(a39, a40)
        };
        bool_and(a41, a42)
    };
    let a52 = {
        let a49 = {
            let a46 = {
                let a43 = v0;
                let a44 = 32u64;
                let a45 = 0u64;
                nat_quot(a43, a44, a45)
            };
            let a47 = 2u64;
            let a48 = 0u64;
            nat_rem(a46, a47, a48)
        };
        let a50 = 1u64;
        nat_eq(a49, a50)
    };
    if bool_and(a51, a52) {
        Ok(3u64)
    } else {
        let a93 = {
            let a83 = 2u64;
            let a84 = {
                let a81 = {
                    let a65 = {
                        let a56 = {
                            let a53 = v0;
                            let a54 = 1u64;
                            let a55 = 0u64;
                            nat_quot(a53, a54, a55)
                        };
                        let a57 = 2u64;
                        let a58 = 0u64;
                        nat_rem(a56, a57, a58)
                    };
                    let a66 = {
                        let a62 = {
                            let a59 = v0;
                            let a60 = 2u64;
                            let a61 = 0u64;
                            nat_quot(a59, a60, a61)
                        };
                        let a63 = 2u64;
                        let a64 = 0u64;
                        nat_rem(a62, a63, a64)
                    };
                    nat_add(a65, a66)?
                };
                let a82 = {
                    let a79 = {
                        let a70 = {
                            let a67 = v0;
                            let a68 = 4u64;
                            let a69 = 0u64;
                            nat_quot(a67, a68, a69)
                        };
                        let a71 = 2u64;
                        let a72 = 0u64;
                        nat_rem(a70, a71, a72)
                    };
                    let a80 = {
                        let a76 = {
                            let a73 = v0;
                            let a74 = 8u64;
                            let a75 = 0u64;
                            nat_quot(a73, a74, a75)
                        };
                        let a77 = 2u64;
                        let a78 = 0u64;
                        nat_rem(a76, a77, a78)
                    };
                    nat_add(a79, a80)?
                };
                nat_add(a81, a82)?
            };
            nat_le(a83, a84)
        };
        let a94 = {
            let a91 = {
                let a88 = {
                    let a85 = v0;
                    let a86 = 16u64;
                    let a87 = 0u64;
                    nat_quot(a85, a86, a87)
                };
                let a89 = 2u64;
                let a90 = 0u64;
                nat_rem(a88, a89, a90)
            };
            let a92 = 1u64;
            nat_eq(a91, a92)
        };
        if bool_and(a93, a94) {
            Ok(2u64)
        } else {
            let a125 = 2u64;
            let a126 = {
                let a123 = {
                    let a107 = {
                        let a98 = {
                            let a95 = v0;
                            let a96 = 1u64;
                            let a97 = 0u64;
                            nat_quot(a95, a96, a97)
                        };
                        let a99 = 2u64;
                        let a100 = 0u64;
                        nat_rem(a98, a99, a100)
                    };
                    let a108 = {
                        let a104 = {
                            let a101 = v0;
                            let a102 = 2u64;
                            let a103 = 0u64;
                            nat_quot(a101, a102, a103)
                        };
                        let a105 = 2u64;
                        let a106 = 0u64;
                        nat_rem(a104, a105, a106)
                    };
                    nat_add(a107, a108)?
                };
                let a124 = {
                    let a121 = {
                        let a112 = {
                            let a109 = v0;
                            let a110 = 4u64;
                            let a111 = 0u64;
                            nat_quot(a109, a110, a111)
                        };
                        let a113 = 2u64;
                        let a114 = 0u64;
                        nat_rem(a112, a113, a114)
                    };
                    let a122 = {
                        let a118 = {
                            let a115 = v0;
                            let a116 = 8u64;
                            let a117 = 0u64;
                            nat_quot(a115, a116, a117)
                        };
                        let a119 = 2u64;
                        let a120 = 0u64;
                        nat_rem(a118, a119, a120)
                    };
                    nat_add(a121, a122)?
                };
                nat_add(a123, a124)?
            };
            if nat_le(a125, a126) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64) -> R<(u64, (u64, (u64, (u64, u64))))> {
    f0(p0, p1, p2, p3, p4)
}
