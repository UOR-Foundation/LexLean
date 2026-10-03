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
    C0(u64, u64),
}

pub fn f0() -> (Option<u64>, (Option<u64>, (Option<u64>, (u64, (u64, (u64, (u64, (u64, ())))))))) {
    (f1(Some(3u64)), (f2(Some(4u64)), (f3(Ok::<u64, bool>(5u64)), (f4(None::<u64>), (f5(Some(6u64)), (f6(9u64), (f7(false), (f8(Adt0::C0(1u64, 2u64)), f9(Some(1u64))))))))))
}

pub fn f1(v0: Option<u64>) -> Option<u64> {
    v0
}

pub fn f2(v0: Option<u64>) -> Option<u64> {
    let m2 = v0;
    match m2 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            Some({
                let a3 = v1;
                let a4 = 1u64;
                nat_sub(a3, a4)
            })
        }
    }
}

pub fn f3(v0: Result<u64, bool>) -> Option<u64> {
    let m5 = v0;
    match m5 {
        Ok(v1) => {
            Some(v1)
        }
        Err(_) => {
            None::<u64>
        }
    }
}

pub fn f4(v0: Option<u64>) -> u64 {
    let m6 = v0;
    match m6 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f5(v0: Option<u64>) -> u64 {
    let m7 = v0;
    match m7 {
        None => {
            7u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f6(v0: u64) -> u64 {
    let m8 = v0;
    if m8 == 0 {
        0u64
    } else {
        m8 - 1
    }
}

pub fn f7(v0: bool) -> u64 {
    if v0 {
        200u64
    } else {
        let m9 = false;
        if m9 {
            1u64
        } else {
            200u64
        }
    }
}

pub fn f8(v0: Adt0) -> u64 {
    let m12 = {
        let m11 = v0.clone();
        match m11 {
            Adt0::C0(v1, v2) => {
                Adt0::C0(v2, v1)
            }
        }
    };
    match m12 {
        Adt0::C0(h10, _) => {
            h10
        }
    }
}

pub fn f9(v0: Option<u64>) {
    let m13 = v0;
    match m13 {
        None => {
            f10(1u64)
        }
        Some(_) => {}
    }
}

pub fn f10(v0: u64) {
    let _: bool = {
        let a14 = v0;
        let a15 = 1u64;
        nat_eq(a14, a15)
    };
}

pub fn run() -> (Option<u64>, (Option<u64>, (Option<u64>, (u64, (u64, (u64, (u64, (u64, ())))))))) {
    f0()
}
