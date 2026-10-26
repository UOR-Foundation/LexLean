#![forbid(unsafe_code)]
use core::cmp::Ordering;

use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;

pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);

pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }

pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}

pub trait Key {
    fn key(&self, other: &Self) -> Ordering;
}

impl Same for bool {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for bool {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for Ordering {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering { match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided, } }
}

pub fn equal<T: Same>(a: T, b: T) -> bool { a.same(&b) }

pub fn compare<T: Key>(a: T, b: T) -> Ordering { a.key(&b) }

pub fn nat_add(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_sub(p0: u64, p1: u64) -> u64 {
    u64::saturating_sub(p0, p1)
}

pub fn nat_mul(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_quot(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_div(p0, p1).unwrap_or(p2)
}

pub fn nat_rem(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_rem(p0, p1).unwrap_or(p2)
}

pub fn nat_eq(p0: u64, p1: u64) -> bool {
    core::cmp::PartialEq::eq(&p0, &p1)
}

pub fn nat_le(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::le(&p0, &p1)
}

pub fn nat_lt(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::lt(&p0, &p1)
}

pub fn nat_succ(p0: u64) -> R<u64> {
    match u64::checked_add(p0, 1u64) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn int_add(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_sub(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_sub(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_mul(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_neg(p0: i64) -> R<i64> {
    match i64::checked_neg(p0) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_quot(p0: i64, p1: i64, p2: i64) -> R<i64> {
    { let value = { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { Some(z) } else { a.checked_div(b) } }; match value {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    } }
}

pub fn int_rem(p0: i64, p1: i64, p2: i64) -> i64 {
    { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { z } else { a.wrapping_rem(b) } }
}

pub fn bool_not(p0: bool) -> bool {
    !p0
}

pub fn bool_and(p0: bool, p1: bool) -> bool {
    if p0 {
        p1
    } else {
        false
    }
}

pub fn bool_or(p0: bool, p1: bool) -> bool {
    if p0 {
        true
    } else {
        p1
    }
}

pub mod fixed_u8 {
    pub fn checked_add(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u8, p1: u8) -> u8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u8, p1: u8) -> u8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u8, p1: u8) -> u8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u8) -> u8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u8> {
        u8::try_from(p0).ok()
    }
}

pub mod fixed_u16 {
    pub fn checked_add(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u16, p1: u16) -> u16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u16, p1: u16) -> u16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u16, p1: u16) -> u16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u16) -> u16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u16> {
        u16::try_from(p0).ok()
    }
}

pub mod fixed_u32 {
    pub fn checked_add(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u32, p1: u32) -> u32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u32, p1: u32) -> u32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u32, p1: u32) -> u32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u32) -> u32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u32> {
        u32::try_from(p0).ok()
    }
}

pub mod fixed_u64 {
    pub fn checked_add(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u64, p1: u64) -> u64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u64, p1: u64) -> u64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u64, p1: u64) -> u64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u64) -> u64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u64> {
        u64::try_from(p0).ok()
    }
}

pub mod fixed_i8 {
    pub fn checked_add(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i8, p1: i8) -> i8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i8, p1: i8) -> i8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i8, p1: i8) -> i8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i8) -> i8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i8> {
        i8::try_from(p0).ok()
    }
}

pub fn checked_neg_i8(p0: i8) -> Option<i8> {
    i8::checked_neg(p0)
}

pub mod fixed_i16 {
    pub fn checked_add(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i16, p1: i16) -> i16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i16, p1: i16) -> i16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i16, p1: i16) -> i16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i16) -> i16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i16> {
        i16::try_from(p0).ok()
    }
}

pub fn checked_neg_i16(p0: i16) -> Option<i16> {
    i16::checked_neg(p0)
}

pub mod fixed_i32 {
    pub fn checked_add(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i32, p1: i32) -> i32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i32, p1: i32) -> i32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i32, p1: i32) -> i32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i32) -> i32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i32> {
        i32::try_from(p0).ok()
    }
}

pub fn checked_neg_i32(p0: i32) -> Option<i32> {
    i32::checked_neg(p0)
}

pub mod fixed_i64 {
    pub fn checked_add(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i64, p1: i64) -> i64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i64, p1: i64) -> i64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i64, p1: i64) -> i64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i64) -> i64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i64> {
        i64::try_from(p0).ok()
    }
}

pub fn checked_neg_i64(p0: i64) -> Option<i64> {
    i64::checked_neg(p0)
}
use core::str;

use std::mem;

use std::rc::Rc;

#[derive(Clone)]
pub struct Str(Rc<str>);

impl Str {
    pub fn lit(text: &str) -> Str { Str(Rc::from(text)) }
    pub fn text(&self) -> &str { &self.0 }
    pub fn length(&self) -> u64 { chars(&self.0) }
    pub fn utf8_encode(&self) -> Bytes { tick(self.0.len() as u64); Bytes::lit(self.0.as_bytes()) }
    pub fn split_exact(&self, delimiter: &Str, maximum: u32) -> Option<List<Str>> { if delimiter.0.is_empty() { return None; }; chars(&self.0); let mut fields: Vec<Str> = Vec::new(); for piece in self.0.split(&*delimiter.0) { fields.push(Str::lit(piece)); }; if fields.len() as u128 > u128::from(maximum) { return None; }; Some(List::onto(fields, List::nil())) }
    pub fn join(texts: List<Str>, delimiter: &Str) -> Str { let mut out = String::new(); let mut first = true; let mut cursor = &texts; while let Repr::Cell(cell) = &cursor.0 { tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&cell.head.0); out.push_str(&cell.head.0); cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for text in buffer[..*len].iter().rev() { tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&text.0); out.push_str(&text.0); } }, _ => { }, }; Str::lit(&out) }
    pub fn parse_int(text: &Str) -> Option<Option<i64>> { match canonical_decimal(&text.0) { None => Some(None), Some(Some(n)) => match i64::try_from(n) { Ok(value) => Some(Some(value)), Err(_) => None, }, Some(None) => None, } }
    pub fn format_u8(a: u8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u8(text: &Str) -> Option<u8> { match canonical_decimal(&text.0) { Some(Some(n)) => u8::try_from(n).ok(), _ => None, } }
    pub fn format_u16(a: u16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u16(text: &Str) -> Option<u16> { match canonical_decimal(&text.0) { Some(Some(n)) => u16::try_from(n).ok(), _ => None, } }
    pub fn format_u32(a: u32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u32(text: &Str) -> Option<u32> { match canonical_decimal(&text.0) { Some(Some(n)) => u32::try_from(n).ok(), _ => None, } }
    pub fn format_u64(a: u64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u64(text: &Str) -> Option<u64> { match canonical_decimal(&text.0) { Some(Some(n)) => u64::try_from(n).ok(), _ => None, } }
    pub fn format_i8(a: i8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i8(text: &Str) -> Option<i8> { match canonical_decimal(&text.0) { Some(Some(n)) => i8::try_from(n).ok(), _ => None, } }
    pub fn format_i16(a: i16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i16(text: &Str) -> Option<i16> { match canonical_decimal(&text.0) { Some(Some(n)) => i16::try_from(n).ok(), _ => None, } }
    pub fn format_i32(a: i32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i32(text: &Str) -> Option<i32> { match canonical_decimal(&text.0) { Some(Some(n)) => i32::try_from(n).ok(), _ => None, } }
    pub fn format_i64(a: i64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i64(text: &Str) -> Option<i64> { match canonical_decimal(&text.0) { Some(Some(n)) => i64::try_from(n).ok(), _ => None, } }
}

#[derive(Clone)]
pub struct Bytes(Rc<[u8]>);

impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes { Bytes(Rc::from(octets)) }
    pub fn octets(&self) -> &[u8] { &self.0 }
    pub fn append(a: Bytes, b: Bytes) -> Bytes { tick((a.0.len() + b.0.len()) as u64); let mut out = Vec::with_capacity(a.0.len() + b.0.len()); out.extend_from_slice(&a.0); out.extend_from_slice(&b.0); Bytes(Rc::from(out)) }
    pub fn length(&self) -> u64 { self.0.len() as u64 }
    pub fn index(&self, i: u64) -> Option<u8> { match usize::try_from(i) { Ok(at) => self.0.get(at).copied(), Err(_) => None, } }
    pub fn slice(&self, start: u64, count: u64) -> Option<Bytes> { let end = u128::from(start) + u128::from(count); if end > self.0.len() as u128 { return None; }; tick(count); Some(Bytes::lit(&self.0[start as usize..end as usize])) }
    pub fn utf8_decode(&self) -> Option<Str> { tick(self.0.len() as u64); match str::from_utf8(&self.0) { Ok(text) => Some(Str::lit(text)), Err(_) => None, } }
    pub fn compare(&self, other: &Bytes) -> Ordering { tick(1 + self.0.len().min(other.0.len()) as u64); self.0.cmp(&other.0) }
}

pub struct Node<T> { head: T, tail: List<T> }

pub enum Repr<T> { Nil, Run { buffer: Rc<Vec<T>>, len: usize }, Cell(Rc<Node<T>>) }

pub struct List<T>(Repr<T>);

impl<T> Clone for List<T> {
    fn clone(&self) -> Self { List(match &self.0 { Repr::Nil => Repr::Nil, Repr::Run { buffer, len } => Repr::Run { buffer: buffer.clone(), len: *len }, Repr::Cell(cell) => Repr::Cell(cell.clone()), }) }
}

impl<T> List<T> {
    fn take(&mut self) -> Repr<T> { mem::replace(&mut self.0, Repr::Nil) }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) { let mut next = match self.tail.take() { Repr::Cell(cell) => Some(cell), _ => None, }; while let Some(cell) = next { match Rc::try_unwrap(cell) { Ok(mut node) => next = match node.tail.take() { Repr::Cell(cell) => Some(cell), _ => None, }, Err(_) => break, } } }
}

impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(Repr::Nil) }
    pub fn cons(head: T, mut tail: List<T>) -> Self { List(match tail.take() { Repr::Nil => { let mut buffer = Vec::with_capacity(16); buffer.push(head); Repr::Run { buffer: Rc::new(buffer), len: 1 } }, Repr::Run { mut buffer, len } => { match Rc::get_mut(&mut buffer) { Some(cells) => { cells.truncate(len); cells.push(head); Repr::Run { buffer, len: len + 1 } }, None => Repr::Cell(Rc::new(Node { head, tail: List(Repr::Run { buffer, len }) })), } }, rest => Repr::Cell(Rc::new(Node { head, tail: List(rest) })), }) }
    pub fn uncons(&self) -> Option<(T, List<T>)> { match &self.0 { Repr::Nil => None, Repr::Run { buffer, len } => { let head = buffer[len - 1].clone(); Some((head, List(if *len == 1 { Repr::Nil } else { Repr::Run { buffer: buffer.clone(), len: len - 1 } }))) }, Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())), } }
    pub fn length(&self) -> u64 { let mut count = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); count += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for _ in buffer[..*len].iter().rev() { tick(1); count += 1; } }, _ => { }, }; count }
    pub fn append(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }
    pub fn index(&self, i: u64) -> Option<T> { let mut position = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); if position == i { return Some(cell.head.clone()); }; position += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); if position == i { return Some(item.clone()); }; position += 1; } }, _ => { }, }; None }
    pub fn slice(&self, start: u64, count: u64) -> Option<List<T>> { let end = u128::from(start) + u128::from(count); let mut part = Vec::new(); let mut position: u128 = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); if position >= end { return Some(List::onto(part, List::nil())); }; if position >= u128::from(start) { part.push(cell.head.clone()); }; position += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); if position >= end { return Some(List::onto(part, List::nil())); }; if position >= u128::from(start) { part.push(item.clone()); }; position += 1; } }, _ => { }, }; if position < end { None } else { Some(List::onto(part, List::nil())) } }
    fn items(&self) -> Vec<T> { let mut out = Vec::new(); let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); out.push(cell.head.clone()); cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); out.push(item.clone()); } }, _ => { }, }; out }
    fn onto(mut items: Vec<T>, tail: List<T>) -> List<T> { let mut out = tail; while let Some(item) = items.pop() { tick(1); out = List::cons(item, out); }; out }
}

pub fn chars(text: &str) -> u64 { let mut count = 0; for _ in text.chars() { tick(1); count += 1; }; count }

impl Same for Str {
    fn same(&self, other: &Self) -> bool { let mut left = self.0.chars(); let mut right = other.0.chars(); loop { tick(1); match (left.next(), right.next()) { (None, None) => return true, (a, b) if a != b => return false, _ => { }, } } }
}

impl Same for Bytes {
    fn same(&self, other: &Self) -> bool { tick(1 + self.0.len().min(other.0.len()) as u64); self.0 == other.0 }
}

impl Key for Str {
    fn key(&self, other: &Self) -> Ordering { let mut left = self.0.chars(); let mut right = other.0.chars(); loop { tick(1); match (left.next(), right.next()) { (None, None) => return Ordering::Equal, (None, Some(_)) => return Ordering::Less, (Some(_), None) => return Ordering::Greater, (Some(a), Some(b)) => match a.cmp(&b) { Ordering::Equal => { }, decided => return decided, }, } } }
}

pub fn canonical_decimal(text: &str) -> Option<Option<i128>> { let digits = match text.strip_prefix('-') { Some(rest) => rest, None => text, }; let mut canonical = !digits.is_empty() && (digits == "0" || !digits.starts_with('0')) && text != "-0"; for c in digits.chars() { tick(1); canonical = canonical && c.is_ascii_digit(); }; if !canonical { return None; }; Some(text.parse::<i128>().ok()) }

pub fn append_list<T: Clone>(p0: List<T>, p1: List<T>) -> List<T> {
    List::append(p0, p1)
}

pub fn length_list<T: Clone>(p0: List<T>) -> u64 {
    List::length(&p0)
}

pub fn index_list<T: Clone>(p0: List<T>, p1: u64) -> Option<T> {
    List::index(&p0, p1)
}

pub fn slice_list<T: Clone>(p0: List<T>, p1: u64, p2: u64) -> Option<List<T>> {
    List::slice(&p0, p1, p2)
}

pub fn append_bytes(p0: Bytes, p1: Bytes) -> Bytes {
    Bytes::append(p0, p1)
}

pub fn length_bytes(p0: Bytes) -> u64 {
    Bytes::length(&p0)
}

pub fn length_string(p0: Str) -> u64 {
    Str::length(&p0)
}

pub fn index_bytes(p0: Bytes, p1: u64) -> Option<u8> {
    Bytes::index(&p0, p1)
}

pub fn slice_bytes(p0: Bytes, p1: u64, p2: u64) -> Option<Bytes> {
    Bytes::slice(&p0, p1, p2)
}

pub fn utf8_encode(p0: Str) -> Bytes {
    Str::utf8_encode(&p0)
}

pub fn utf8_decode(p0: Bytes) -> Option<Str> {
    Bytes::utf8_decode(&p0)
}

pub fn compare_bytes(p0: Bytes, p1: Bytes) -> Ordering {
    Bytes::compare(&p0, &p1)
}

pub fn split_exact(p0: Str, p1: Str, p2: u32) -> Option<List<Str>> {
    Str::split_exact(&p0, &p1, p2)
}

pub fn join(p0: List<Str>, p1: Str) -> Str {
    Str::join(p0, &p1)
}

pub fn format_u8(p0: u8) -> Str {
    Str::format_u8(p0)
}

pub fn parse_u8(p0: Str) -> Option<u8> {
    Str::parse_u8(&p0)
}

pub fn format_u16(p0: u16) -> Str {
    Str::format_u16(p0)
}

pub fn parse_u16(p0: Str) -> Option<u16> {
    Str::parse_u16(&p0)
}

pub fn format_u32(p0: u32) -> Str {
    Str::format_u32(p0)
}

pub fn parse_u32(p0: Str) -> Option<u32> {
    Str::parse_u32(&p0)
}

pub fn format_u64(p0: u64) -> Str {
    Str::format_u64(p0)
}

pub fn parse_u64(p0: Str) -> Option<u64> {
    Str::parse_u64(&p0)
}

pub fn format_i8(p0: i8) -> Str {
    Str::format_i8(p0)
}

pub fn parse_i8(p0: Str) -> Option<i8> {
    Str::parse_i8(&p0)
}

pub fn format_i16(p0: i16) -> Str {
    Str::format_i16(p0)
}

pub fn parse_i16(p0: Str) -> Option<i16> {
    Str::parse_i16(&p0)
}

pub fn format_i32(p0: i32) -> Str {
    Str::format_i32(p0)
}

pub fn parse_i32(p0: Str) -> Option<i32> {
    Str::parse_i32(&p0)
}

pub fn format_i64(p0: i64) -> Str {
    Str::format_i64(p0)
}

pub fn parse_i64(p0: Str) -> Option<i64> {
    Str::parse_i64(&p0)
}

pub fn parse_int(p0: Str) -> R<Option<i64>> {
    match Str::parse_int(&p0) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<Option<i64>, Overflow>(Overflow)
        }
    }
}

pub fn format_int(p0: i64) -> Str {
    Str::format_i64(p0)
}

#[derive(Clone)]
pub enum Adt0 {
    C0(Bytes, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Adt2>, List<Bytes>),
    C1(List<Adt2>, List<Bytes>, Bytes, Option<Bytes>),
    C2(Adt0),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, List<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1, p1: Bytes) -> R<Adt1> {
        match self {
            Fn0::F3 => f3(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: Result<List<(Bytes, List<Bytes>)>, Adt0>, p1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
        match self {
            Fn1::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16(Bytes),
    F92(u8, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn2::F16(k0) => f16(k0.clone(), p0),
            Fn2::F92(k0, k1) => f92(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23,
}

impl Fn3 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn3::F23 => f23(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F25,
}

impl Fn4 {
    pub fn apply(&self, p0: u64, p1: Adt2) -> R<u64> {
        match self {
            Fn4::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F28,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn5::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F37,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F39(Bytes),
    F63(Bytes),
    F64(u64, Bytes),
    F81(Bytes, Bytes),
    F86(Bytes, u64, Bytes, u64),
    F93(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F39(k0) => f39(k0.clone(), p0),
            Fn7::F63(k0) => f63(k0.clone(), p0),
            Fn7::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn7::F81(k0, k1) => Ok(f81(k0.clone(), k1.clone(), p0)),
            Fn7::F86(k0, k1, k2, k3) => f86(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn7::F93(k0) => f93(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F72(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn8::F72(k0) => f72(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F84,
}

impl Fn9 {
    pub fn apply(&self, p0: Result<List<Bytes>, Bytes>, p1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
        match self {
            Fn9::F84 => f84(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Bytes>) -> R<Result<List<(Bytes, List<Bytes>)>, Adt0>> {
    f1(f2(Fn0::F3, Adt1::C0(List::<Adt2>::nil(), v1.clone()), f4(v0)?)?, v1)
}

pub fn f1(v0: Adt1, v1: List<Bytes>) -> R<Result<List<(Bytes, List<Bytes>)>, Adt0>> {
    let m1 = v0;
    match m1 {
        Adt1::C0(v7, _) => {
            let v9: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f6(f6(f7(Str::lit("expected ")), f8(f9(v1.clone())?)?), f7(Str::lit(" audit records, found "))), f8(f10(v7.clone())?)?), None::<Bytes>);
            let a2 = f10(v7.clone())?;
            let a3 = f9(v1)?;
            if nat_eq(a2, a3) {
                Ok(f11(Fn1::F12, Ok::<List<(Bytes, List<Bytes>)>, Adt0>(List::<(Bytes, List<Bytes>)>::nil()), f13(v7)))
            } else {
                Ok(v9)
            }
        }
        Adt1::C1(_, _, v5, v6) => {
            Ok(f5(f6(f7(Str::lit("unterminated axiom payload: ")), v5), v6))
        }
        Adt1::C2(v2) => {
            Ok(Err::<List<(Bytes, List<Bytes>)>, Adt0>(v2))
        }
    }
}

pub fn f2(v0: Fn0, p7: Adt1, p8: List<Bytes>) -> R<Adt1> {
    let m6 = (p7, p8);
    {
        let mut m6 = m6;
        loop {
            let step = {
                let (v1, v2) = m6;
                let m4 = v2;
                match m4.uncons() {
                    None => {
                        Err::<(Adt1, List<Bytes>), R<Adt1>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<Bytes>), R<Adt1>>(({
                            let c5: Fn0 = v0.clone();
                            c5.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m6 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Bytes) -> R<Adt1> {
    f14(v0, v1)
}

pub fn f4(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a9 = v0.clone();
        length_bytes(a9)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h10, _) = f15(Fn2::F16(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h10
    };
    let v3: u64 = {
        let (h11, _) = {
            let (_, h12) = v2.clone();
            h12
        };
        h11
    };
    let v4: List<Bytes> = {
        let (_, h13) = {
            let (_, h14) = v2;
            h14
        };
        h13
    };
    let a15 = v3;
    let a16 = v1;
    if nat_lt(a15, a16) {
        Ok(f17(List::cons(f18(v0, v3, {
            let a17 = v1;
            let a18 = v3;
            nat_sub(a17, a18)
        }), v4)))
    } else {
        Ok(f17(v4))
    }
}

pub fn f5(v0: Bytes, v1: Option<Bytes>) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    Err::<List<(Bytes, List<Bytes>)>, Adt0>(f19(v0, v1))
}

pub fn f6(v0: Bytes, v1: Bytes) -> Bytes {
    let a19 = v0;
    let a20 = v1;
    append_bytes(a19, a20)
}

pub fn f7(v0: Str) -> Bytes {
    let a21 = v0;
    utf8_encode(a21)
}

pub fn f8(v0: u64) -> R<Bytes> {
    let v1: Bytes = f7(Str::lit("0"));
    let v2: Bytes = f20({
        let a58 = f21(v0, 10000000000000000000u64);
        let a59 = {
            let a56 = f21(v0, 1000000000000000000u64);
            let a57 = {
                let a54 = f21(v0, 100000000000000000u64);
                let a55 = {
                    let a52 = f21(v0, 10000000000000000u64);
                    let a53 = {
                        let a50 = f21(v0, 1000000000000000u64);
                        let a51 = {
                            let a48 = f21(v0, 100000000000000u64);
                            let a49 = {
                                let a46 = f21(v0, 10000000000000u64);
                                let a47 = {
                                    let a44 = f21(v0, 1000000000000u64);
                                    let a45 = {
                                        let a42 = f21(v0, 100000000000u64);
                                        let a43 = {
                                            let a40 = f21(v0, 10000000000u64);
                                            let a41 = {
                                                let a38 = f21(v0, 1000000000u64);
                                                let a39 = {
                                                    let a36 = f21(v0, 100000000u64);
                                                    let a37 = {
                                                        let a34 = f21(v0, 10000000u64);
                                                        let a35 = {
                                                            let a32 = f21(v0, 1000000u64);
                                                            let a33 = {
                                                                let a30 = f21(v0, 100000u64);
                                                                let a31 = {
                                                                    let a28 = f21(v0, 10000u64);
                                                                    let a29 = {
                                                                        let a26 = f21(v0, 1000u64);
                                                                        let a27 = {
                                                                            let a24 = f21(v0, 100u64);
                                                                            let a25 = {
                                                                                let a22 = f21(v0, 10u64);
                                                                                let a23 = f21(v0, 1u64);
                                                                                append_bytes(a22, a23)
                                                                            };
                                                                            append_bytes(a24, a25)
                                                                        };
                                                                        append_bytes(a26, a27)
                                                                    };
                                                                    append_bytes(a28, a29)
                                                                };
                                                                append_bytes(a30, a31)
                                                            };
                                                            append_bytes(a32, a33)
                                                        };
                                                        append_bytes(a34, a35)
                                                    };
                                                    append_bytes(a36, a37)
                                                };
                                                append_bytes(a38, a39)
                                            };
                                            append_bytes(a40, a41)
                                        };
                                        append_bytes(a42, a43)
                                    };
                                    append_bytes(a44, a45)
                                };
                                append_bytes(a46, a47)
                            };
                            append_bytes(a48, a49)
                        };
                        append_bytes(a50, a51)
                    };
                    append_bytes(a52, a53)
                };
                append_bytes(a54, a55)
            };
            append_bytes(a56, a57)
        };
        append_bytes(a58, a59)
    })?;
    let a60 = v0;
    let a61 = 0u64;
    if nat_eq(a60, a61) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f9(v0: List<Bytes>) -> R<u64> {
    f22(Fn3::F23, 0u64, v0)
}

pub fn f10(v0: List<Adt2>) -> R<u64> {
    f24(Fn4::F25, 0u64, v0)
}

pub fn f11(v0: Fn1, p65: Result<List<(Bytes, List<Bytes>)>, Adt0>, p66: List<Adt2>) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    let m64 = (p65, p66);
    {
        let mut m64 = m64;
        loop {
            let step = {
                let (v1, v2) = m64;
                let m62 = v2;
                match m62.uncons() {
                    None => {
                        Err::<(Result<List<(Bytes, List<Bytes>)>, Adt0>, List<Adt2>), Result<List<(Bytes, List<Bytes>)>, Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<(Bytes, List<Bytes>)>, Adt0>, List<Adt2>), Result<List<(Bytes, List<Bytes>)>, Adt0>>(({
                            let c63: Fn1 = v0.clone();
                            c63.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m64 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Result<List<(Bytes, List<Bytes>)>, Adt0>, v1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    f26(v0, v1)
}

pub fn f13(v0: List<Adt2>) -> List<Adt2> {
    f27(Fn5::F28, List::<Adt2>::nil(), v0)
}

pub fn f14(v0: Adt1, v1: Bytes) -> R<Adt1> {
    let m67 = v0.clone();
    match m67 {
        Adt1::C0(v8, v9) => {
            let a70 = 0u64;
            let a71 = {
                let a69 = f32(v1.clone())?;
                length_bytes(a69)
            };
            if nat_eq(a70, a71) {
                Ok(v0)
            } else {
                let m72 = f33(f32(v1.clone())?)?;
                match m72 {
                    None => {
                        Ok(f31(f6(f7(Str::lit("unrecognized audit line: ")), f32(v1)?), f34(v9)))
                    }
                    Some(v10) => {
                        f30(v8, v9.clone(), v10, f34(v9))
                    }
                }
            }
        }
        Adt1::C1(v3, v4, v5, v6) => {
            let m68 = f29(v1)?;
            match m68 {
                None => {
                    Ok(f31(f6(f7(Str::lit("unterminated axiom payload: ")), v5), v6))
                }
                Some(v7) => {
                    f30(v3, v4, f6(f6(v5, f7(Str::lit(" "))), v7), v6)
                }
            }
        }
        Adt1::C2(_) => {
            Ok(v0)
        }
    }
}

pub fn f15(v0: Fn2, p77: u64, p78: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, v2) = m76;
                let m73 = v1;
                if m73 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m73 - 1;
                    let m75 = {
                        let c74: Fn2 = v0.clone();
                        c74.apply(v2.clone())?
                    };
                    match m75 {
                        None => {
                            Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m76 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f35(v0, v1)
}

pub fn f17(v0: List<Bytes>) -> List<Bytes> {
    f36(Fn6::F37, List::<Bytes>::nil(), v0)
}

pub fn f18(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m82 = {
        let a79 = v0;
        let a80 = v1;
        let a81 = v2;
        slice_bytes(a79, a80, a81)
    };
    match m82 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f19(v0: Bytes, v1: Option<Bytes>) -> Adt0 {
    Adt0::C0(v0, v1)
}

pub fn f20(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h83, _) = f38(Fn7::F39(v0.clone()), {
            let a84 = v0.clone();
            length_bytes(a84)
        }, 0u64)?;
        h83
    };
    Ok(f18(v0.clone(), v1, {
        let a86 = {
            let a85 = v0;
            length_bytes(a85)
        };
        let a87 = v1;
        nat_sub(a86, a87)
    }))
}

pub fn f21(v0: u64, v1: u64) -> Bytes {
    f18(f7(Str::lit("0123456789")), {
        let a91 = {
            let a88 = v0;
            let a89 = v1;
            let a90 = 0u64;
            nat_quot(a88, a89, a90)
        };
        let a92 = 10u64;
        let a93 = 0u64;
        nat_rem(a91, a92, a93)
    }, 1u64)
}

pub fn f22(v0: Fn3, p97: u64, p98: List<Bytes>) -> R<u64> {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v1, v2) = m96;
                let m94 = v2;
                match m94.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c95: Fn3 = v0.clone();
                            c95.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m96 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: u64, _: Bytes) -> R<u64> {
    let a99 = v0;
    let a100 = 1u64;
    nat_add(a99, a100)
}

pub fn f24(v0: Fn4, p104: u64, p105: List<Adt2>) -> R<u64> {
    let m103 = (p104, p105);
    {
        let mut m103 = m103;
        loop {
            let step = {
                let (v1, v2) = m103;
                let m101 = v2;
                match m101.uncons() {
                    None => {
                        Err::<(u64, List<Adt2>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt2>), R<u64>>(({
                            let c102: Fn4 = v0.clone();
                            c102.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: u64, _: Adt2) -> R<u64> {
    let a106 = v0;
    let a107 = 1u64;
    nat_add(a106, a107)
}

pub fn f26(v0: Result<List<(Bytes, List<Bytes>)>, Adt0>, v1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    let m108 = v0.clone();
    match m108 {
        Ok(v3) => {
            let v5: Bytes = {
                let m110 = match v1.clone() {
                    Adt2::C0(_, _, h109) => {
                        h109
                    }
                };
                match m110 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v4) => {
                        v4
                    }
                }
            };
            let v6: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f7(Str::lit("duplicate record for ")), f40(match v1.clone() {
                Adt2::C0(h111, _, _) => {
                    h111
                }
            })), Some(v5.clone()));
            let v7: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f6(f6(f7(Str::lit("expected a record for ")), f40(v5.clone())), f7(Str::lit(", found "))), f40(match v1.clone() {
                Adt2::C0(h112, _, _) => {
                    h112
                }
            })), Some(v5.clone()));
            let a114 = match v1.clone() {
                Adt2::C0(h113, _, _) => {
                    h113
                }
            };
            let a115 = v5;
            if equal(a114, a115) {
                let m118 = f41(match v1.clone() {
                    Adt2::C0(h116, _, _) => {
                        h116
                    }
                }, match v1 {
                    Adt2::C0(_, h117, _) => {
                        h117
                    }
                }, v3);
                match m118 {
                    None => {
                        v6
                    }
                    Some(v8) => {
                        Ok::<List<(Bytes, List<Bytes>)>, Adt0>(v8)
                    }
                }
            } else {
                v7
            }
        }
        Err(_) => {
            v0
        }
    }
}

pub fn f27(v0: Fn5, p122: List<Adt2>, p123: List<Adt2>) -> List<Adt2> {
    let m121 = (p122, p123);
    {
        let mut m121 = m121;
        loop {
            let step = {
                let (v1, v2) = m121;
                let m119 = v2;
                match m119.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c120: Fn5 = v0.clone();
                            c120.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f29(v0: Bytes) -> R<Option<Bytes>> {
    let v1: Bytes = f32(v0.clone())?;
    if f42(v0)? {
        let a125 = 0u64;
        let a126 = {
            let a124 = v1.clone();
            length_bytes(a124)
        };
        if nat_eq(a125, a126) {
            Ok(None::<Bytes>)
        } else {
            Ok(Some(v1))
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f30(v0: List<Adt2>, v1: List<Bytes>, v2: Bytes, v3: Option<Bytes>) -> R<Adt1> {
    if f43(v2.clone()) {
        let m127 = f44(v2)?;
        match m127 {
            Ok(v5) => {
                Ok(Adt1::C0(List::cons(Adt2::C0({
                    let (h128, _) = v5;
                    h128
                }, {
                    let (_, h129) = v5;
                    h129
                }, v3), v0), f45(v1)))
            }
            Err(v4) => {
                Ok(f31(v4, v3))
            }
        }
    } else {
        Ok(Adt1::C1(v0, v1, v2, v3))
    }
}

pub fn f31(v0: Bytes, v1: Option<Bytes>) -> Adt1 {
    Adt1::C2(f19(v0, v1))
}

pub fn f32(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f46(v0.clone())?;
    let v2: u64 = f47(v0.clone(), v1)?;
    Ok(f18(v0, v1, {
        let a130 = v2;
        let a131 = v1;
        nat_sub(a130, a131)
    }))
}

pub fn f33(v0: Bytes) -> R<Option<Bytes>> {
    if f48(v0.clone(), f7(Str::lit("'"))) {
        Ok(Some(v0))
    } else {
        let m132 = f49(v0.clone(), f7(Str::lit("info: ")));
        match m132 {
            None => {
                f50(v0)
            }
            Some(v1) => {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f34(v0: List<Bytes>) -> Option<Bytes> {
    let m133 = v0;
    match m133.uncons() {
        None => {
            None::<Bytes>
        }
        Some((v1, _)) => {
            Some(v1)
        }
    }
}

pub fn f35(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h134, _) = v1.clone();
        h134
    };
    let v3: u64 = {
        let (h135, _) = {
            let (_, h136) = v1.clone();
            h136
        };
        h135
    };
    let v4: List<Bytes> = {
        let (_, h137) = {
            let (_, h138) = v1;
            h138
        };
        h137
    };
    let m139 = f51(v0.clone(), v2);
    match m139 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a140 = v5;
            let a141 = 10u8;
            if equal(a140, a141) {
                Ok(Some(({
                    let a142 = v2;
                    let a143 = 1u64;
                    nat_add(a142, a143)?
                }, ({
                    let a144 = v2;
                    let a145 = 1u64;
                    nat_add(a144, a145)?
                }, List::cons(f52(f18(v0, v3, {
                    let a146 = v2;
                    let a147 = v3;
                    nat_sub(a146, a147)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a148 = v2;
                    let a149 = 1u64;
                    nat_add(a148, a149)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f36(v0: Fn6, p153: List<Bytes>, p154: List<Bytes>) -> List<Bytes> {
    let m152 = (p153, p154);
    {
        let mut m152 = m152;
        loop {
            let step = {
                let (v1, v2) = m152;
                let m150 = v2;
                match m150.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c151: Fn6 = v0.clone();
                            c151.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f38(v0: Fn7, p159: u64, p160: u64) -> R<(u64, bool)> {
    let m158 = (p159, p160);
    {
        let mut m158 = m158;
        loop {
            let step = {
                let (v1, v2) = m158;
                let m155 = v1;
                if m155 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m155 - 1;
                    let m157 = {
                        let c156: Fn7 = v0.clone();
                        c156.apply(v2)?
                    };
                    match m157 {
                        None => {
                            Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, u64), R<(u64, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f53(v0, v1)
}

pub fn f40(v0: Bytes) -> Bytes {
    f6(f6(f7(Str::lit("`")), v0), f7(Str::lit("`")))
}

pub fn f41(v0: Bytes, v1: List<Bytes>, v2: List<(Bytes, List<Bytes>)>) -> Option<List<(Bytes, List<Bytes>)>> {
    let m161 = v2;
    match m161.uncons() {
        None => {
            Some(List::cons((v0, v1), List::<(Bytes, List<Bytes>)>::nil()))
        }
        Some((v3, v4)) => {
            let a163 = v0.clone();
            let a164 = {
                let (h162, _) = v3.clone();
                h162
            };
            if equal(a163, a164) {
                None::<List<(Bytes, List<Bytes>)>>
            } else {
                if f54(v0.clone(), {
                    let (h165, _) = v3.clone();
                    h165
                }) {
                    Some(List::cons((v0, v1), List::cons(v3, v4)))
                } else {
                    let m166 = f41(v0, v1, v4);
                    match m166 {
                        None => {
                            None::<List<(Bytes, List<Bytes>)>>
                        }
                        Some(v5) => {
                            Some(List::cons(v3, v5))
                        }
                    }
                }
            }
        }
    }
}

pub fn f42(v0: Bytes) -> R<bool> {
    let a167 = 0u64;
    let a168 = f55(v0, 0u64)?;
    Ok(nat_lt(a167, a168))
}

pub fn f43(v0: Bytes) -> bool {
    let m169 = f56(v0.clone(), f7(Str::lit("]")));
    match m169 {
        None => {
            let m170 = f56(v0, f7(Str::lit(" does not depend on any axioms")));
            match m170 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
        Some(_) => {
            true
        }
    }
}

pub fn f44(v0: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m171 = f49(v0.clone(), f7(Str::lit("'")));
    match m171 {
        None => {
            Ok(f57(v0))
        }
        Some(v1) => {
            let m172 = f56(v1.clone(), f7(Str::lit("' does not depend on any axioms")));
            match m172 {
                None => {
                    let m173 = f56(v1, f7(Str::lit("]")));
                    match m173 {
                        None => {
                            Ok(f57(v0))
                        }
                        Some(v5) => {
                            f62(v0, v5)
                        }
                    }
                }
                Some(v2) => {
                    let v3: Result<(Bytes, List<Bytes>), Bytes> = f58(v2.clone(), List::<Bytes>::nil());
                    let v4: Result<(Bytes, List<Bytes>), Bytes> = f59(f60(v2.clone()));
                    if f61(v2)? {
                        Ok(v3)
                    } else {
                        Ok(v4)
                    }
                }
            }
        }
    }
}

pub fn f45(v0: List<Bytes>) -> List<Bytes> {
    let m174 = v0;
    match m174.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f46(v0: Bytes) -> R<u64> {
    let (h175, _) = f38(Fn7::F63(v0.clone()), {
        let a176 = v0;
        length_bytes(a176)
    }, 0u64)?;
    Ok(h175)
}

pub fn f47(v0: Bytes, v1: u64) -> R<u64> {
    let (h177, _) = f38(Fn7::F64(v1, v0.clone()), {
        let a178 = v0.clone();
        length_bytes(a178)
    }, {
        let a179 = v0;
        length_bytes(a179)
    })?;
    Ok(h177)
}

pub fn f48(v0: Bytes, v1: Bytes) -> bool {
    let m184 = {
        let a181 = v0;
        let a182 = 0u64;
        let a183 = {
            let a180 = v1.clone();
            length_bytes(a180)
        };
        slice_bytes(a181, a182, a183)
    };
    match m184 {
        None => {
            false
        }
        Some(v2) => {
            let a185 = v2;
            let a186 = v1;
            equal(a185, a186)
        }
    }
}

pub fn f49(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    if f48(v0.clone(), v1.clone()) {
        Some(f18(v0.clone(), {
            let a187 = v1.clone();
            length_bytes(a187)
        }, {
            let a190 = {
                let a188 = v0;
                length_bytes(a188)
            };
            let a191 = {
                let a189 = v1;
                length_bytes(a189)
            };
            nat_sub(a190, a191)
        }))
    } else {
        None::<Bytes>
    }
}

pub fn f50(v0: Bytes) -> R<Option<Bytes>> {
    let m192 = f65(v0, f7(Str::lit(":")))?;
    match m192 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let m194 = f65({
                let (_, h193) = v1;
                h193
            }, f7(Str::lit(":")))?;
            match m194 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    let m196 = f65({
                        let (_, h195) = v2;
                        h195
                    }, f7(Str::lit(":")))?;
                    match m196 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v3) => {
                            let v4: Bytes = {
                                let (h197, _) = v2;
                                h197
                            };
                            let v5: Bytes = {
                                let (h198, _) = v3;
                                h198
                            };
                            let v6: Bytes = {
                                let (_, h199) = v3;
                                h199
                            };
                            let v7: Option<Bytes> = f66(v5, v6)?;
                            if f67(v4)? {
                                Ok(v7)
                            } else {
                                Ok(None::<Bytes>)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> Option<u8> {
    let a200 = v0;
    let a201 = v1;
    index_bytes(a200, a201)
}

pub fn f52(v0: Bytes) -> Bytes {
    if f68(v0.clone(), Bytes::lit(&[0x0d])) {
        f18(v0.clone(), 0u64, {
            let a203 = {
                let a202 = v0;
                length_bytes(a202)
            };
            let a204 = 1u64;
            nat_sub(a203, a204)
        })
    } else {
        v0
    }
}

pub fn f53(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f69(v0, v1, 48u8) {
        Ok(Some({
            let a205 = v1;
            let a206 = 1u64;
            nat_add(a205, a206)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f54(v0: Bytes, v1: Bytes) -> bool {
    let a211 = {
        let a207 = v0;
        let a208 = v1;
        compare_bytes(a207, a208)
    };
    let a212 = {
        let a209 = Bytes::lit(&[]);
        let a210 = Bytes::lit(&[0x01]);
        compare_bytes(a209, a210)
    };
    equal(a211, a212)
}

pub fn f55(v0: Bytes, v1: u64) -> R<u64> {
    if if f70(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f69(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f69(v0.clone(), v1, 194u8) {
            if if f69(v0.clone(), {
                let a213 = v1;
                let a214 = 1u64;
                nat_add(a213, a214)?
            }, 133u8) {
                true
            } else {
                f69(v0, {
                    let a215 = v1;
                    let a216 = 1u64;
                    nat_add(a215, a216)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f69(v0.clone(), v1, 225u8) {
                if if f69(v0.clone(), {
                    let a217 = v1;
                    let a218 = 1u64;
                    nat_add(a217, a218)?
                }, 154u8) {
                    f69(v0, {
                        let a219 = v1;
                        let a220 = 2u64;
                        nat_add(a219, a220)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f69(v0.clone(), v1, 226u8) {
                    if f69(v0.clone(), {
                        let a221 = v1;
                        let a222 = 1u64;
                        nat_add(a221, a222)?
                    }, 128u8) {
                        if if f70(v0.clone(), {
                            let a223 = v1;
                            let a224 = 2u64;
                            nat_add(a223, a224)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f69(v0.clone(), {
                                let a225 = v1;
                                let a226 = 2u64;
                                nat_add(a225, a226)?
                            }, 168u8) {
                                true
                            } else {
                                if f69(v0.clone(), {
                                    let a227 = v1;
                                    let a228 = 2u64;
                                    nat_add(a227, a228)?
                                }, 169u8) {
                                    true
                                } else {
                                    f69(v0, {
                                        let a229 = v1;
                                        let a230 = 2u64;
                                        nat_add(a229, a230)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f69(v0.clone(), {
                            let a231 = v1;
                            let a232 = 1u64;
                            nat_add(a231, a232)?
                        }, 129u8) {
                            f69(v0, {
                                let a233 = v1;
                                let a234 = 2u64;
                                nat_add(a233, a234)?
                            }, 159u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    if f69(v0.clone(), v1, 227u8) {
                        if if f69(v0.clone(), {
                            let a235 = v1;
                            let a236 = 1u64;
                            nat_add(a235, a236)?
                        }, 128u8) {
                            f69(v0, {
                                let a237 = v1;
                                let a238 = 2u64;
                                nat_add(a237, a238)?
                            }, 128u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    if f68(v0.clone(), v1.clone()) {
        Some(f18(v0.clone(), 0u64, {
            let a241 = {
                let a239 = v0;
                length_bytes(a239)
            };
            let a242 = {
                let a240 = v1;
                length_bytes(a240)
            };
            nat_sub(a241, a242)
        }))
    } else {
        None::<Bytes>
    }
}

pub fn f57(v0: Bytes) -> Result<(Bytes, List<Bytes>), Bytes> {
    f59(f6(f7(Str::lit("unrecognized axiom payload: ")), v0))
}

pub fn f58(v0: Bytes, v1: List<Bytes>) -> Result<(Bytes, List<Bytes>), Bytes> {
    Ok::<(Bytes, List<Bytes>), Bytes>((v0, v1))
}

pub fn f59(v0: Bytes) -> Result<(Bytes, List<Bytes>), Bytes> {
    Err::<(Bytes, List<Bytes>), Bytes>(v0)
}

pub fn f60(v0: Bytes) -> Bytes {
    f6(f40(v0), f7(Str::lit(" is not a Lean name")))
}

pub fn f61(v0: Bytes) -> R<bool> {
    let a246 = {
        let (_, h243) = {
            let (h244, _) = f71(Fn8::F72(v0.clone()), {
                let a245 = v0;
                length_bytes(a245)
            }, (0u64, 0u64))?;
            h244
        };
        h243
    };
    let a247 = 1u64;
    Ok(nat_eq(a246, a247))
}

pub fn f62(v0: Bytes, v1: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m248 = f73(v1, f7(Str::lit("' depends on axioms: [")))?;
    match m248 {
        None => {
            Ok(f57(v0))
        }
        Some(v2) => {
            let v3: Bytes = {
                let (h249, _) = v2;
                h249
            };
            let v4: Bytes = {
                let (_, h250) = v2;
                h250
            };
            let v5: Result<(Bytes, List<Bytes>), Bytes> = f74(v3.clone(), v4)?;
            let v6: Result<(Bytes, List<Bytes>), Bytes> = f59(f60(v3.clone()));
            if f61(v3)? {
                Ok(v5)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f63(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a251 = 0u64;
    let a252 = f55(v0.clone(), v1)?;
    if nat_lt(a251, a252) {
        Ok(Some({
            let a253 = v1;
            let a254 = f55(v0, v1)?;
            nat_add(a253, a254)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f64(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a255 = 0u64;
    let a256 = f75(v1.clone(), v2)?;
    if if nat_lt(a255, a256) {
        let a259 = {
            let a257 = v0;
            let a258 = f75(v1.clone(), v2)?;
            nat_add(a257, a258)?
        };
        let a260 = v2;
        nat_le(a259, a260)
    } else {
        false
    } {
        Ok(Some({
            let a261 = v2;
            let a262 = f75(v1, v2)?;
            nat_sub(a261, a262)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f65(v0: Bytes, v1: Bytes) -> R<Option<(Bytes, Bytes)>> {
    let m263 = f76(v0.clone(), v1.clone())?;
    match m263 {
        None => {
            Ok(None::<(Bytes, Bytes)>)
        }
        Some(v2) => {
            Ok(Some((f18(v0.clone(), 0u64, v2), f18(v0.clone(), {
                let a265 = v2;
                let a266 = {
                    let a264 = v1.clone();
                    length_bytes(a264)
                };
                nat_add(a265, a266)?
            }, {
                let a271 = {
                    let a267 = v0;
                    length_bytes(a267)
                };
                let a272 = {
                    let a269 = v2;
                    let a270 = {
                        let a268 = v1;
                        length_bytes(a268)
                    };
                    nat_add(a269, a270)?
                };
                nat_sub(a271, a272)
            }))))
        }
    }
}

pub fn f66(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m273 = f65(v0.clone(), f7(Str::lit("-")))?;
    match m273 {
        None => {
            let v9: Option<Bytes> = f77(v1);
            if f67(v0)? {
                Ok(v9)
            } else {
                Ok(None::<Bytes>)
            }
        }
        Some(v2) => {
            let v3: Bytes = {
                let (h274, _) = v2;
                h274
            };
            let v4: Bytes = {
                let (_, h275) = v2;
                h275
            };
            let m276 = f65(v1, f7(Str::lit(":")))?;
            match m276 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v5) => {
                    let v6: Bytes = {
                        let (h277, _) = v5;
                        h277
                    };
                    let v7: Bytes = {
                        let (_, h278) = v5;
                        h278
                    };
                    let v8: Option<Bytes> = f77(v7);
                    if f67(v3)? {
                        if f67(v4)? {
                            if f67(v6)? {
                                Ok(v8)
                            } else {
                                Ok(None::<Bytes>)
                            }
                        } else {
                            Ok(None::<Bytes>)
                        }
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f67(v0: Bytes) -> R<bool> {
    f78(f32(v0)?)
}

pub fn f68(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a279 = v0.clone();
        length_bytes(a279)
    };
    let v3: u64 = {
        let a280 = v1.clone();
        length_bytes(a280)
    };
    let a281 = v3;
    let a282 = v2;
    if nat_le(a281, a282) {
        let m288 = {
            let a285 = v0;
            let a286 = {
                let a283 = v2;
                let a284 = v3;
                nat_sub(a283, a284)
            };
            let a287 = v3;
            slice_bytes(a285, a286, a287)
        };
        match m288 {
            None => {
                false
            }
            Some(v4) => {
                let a289 = v4;
                let a290 = v1;
                equal(a289, a290)
            }
        }
    } else {
        false
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m291 = f51(v0, v1);
    match m291 {
        None => {
            false
        }
        Some(v3) => {
            let a292 = v3;
            let a293 = v2;
            equal(a292, a293)
        }
    }
}

pub fn f70(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m294 = f51(v0, v1);
    match m294 {
        None => {
            false
        }
        Some(v4) => {
            f79(v4, v2, v3)
        }
    }
}

pub fn f71(v0: Fn8, p299: u64, p300: (u64, u64)) -> R<((u64, u64), bool)> {
    let m298 = (p299, p300);
    {
        let mut m298 = m298;
        loop {
            let step = {
                let (v1, v2) = m298;
                let m295 = v1;
                if m295 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m295 - 1;
                    let m297 = {
                        let c296: Fn8 = v0.clone();
                        c296.apply(v2)?
                    };
                    match m297 {
                        None => {
                            Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, u64)), R<((u64, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f80(v0, v1)
}

pub fn f73(v0: Bytes, v1: Bytes) -> R<Option<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a301 = v0.clone();
        length_bytes(a301)
    };
    let v3: u64 = {
        let a302 = v1.clone();
        length_bytes(a302)
    };
    let a303 = v2;
    let a304 = v3;
    if nat_lt(a303, a304) {
        Ok(None::<(Bytes, Bytes)>)
    } else {
        let v4: u64 = {
            let (h305, _) = f38(Fn7::F81(v1.clone(), v0.clone()), {
                let a306 = v2;
                let a307 = 1u64;
                nat_add(a306, a307)?
            }, {
                let a308 = v2;
                let a309 = v3;
                nat_sub(a308, a309)
            })?;
            h305
        };
        if f82(v0.clone(), v1, v4) {
            Ok(Some((f18(v0.clone(), 0u64, v4), f18(v0, {
                let a310 = v4;
                let a311 = v3;
                nat_add(a310, a311)?
            }, {
                let a314 = v2;
                let a315 = {
                    let a312 = v4;
                    let a313 = v3;
                    nat_add(a312, a313)?
                };
                nat_sub(a314, a315)
            }))))
        } else {
            Ok(None::<(Bytes, Bytes)>)
        }
    }
}

pub fn f74(v0: Bytes, v1: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m316 = f83(Fn9::F84, Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()), f85(v1, 44u8)?)?;
    match m316 {
        Ok(v3) => {
            let m317 = v3.clone();
            match m317.uncons() {
                None => {
                    Ok(f59(f7(Str::lit("an axiom list cannot be empty"))))
                }
                Some((_, _)) => {
                    Ok(f58(v0, v3))
                }
            }
        }
        Err(v2) => {
            Ok(f59(v2))
        }
    }
}

pub fn f75(v0: Bytes, v1: u64) -> R<u64> {
    let a318 = v1;
    let a319 = 0u64;
    if nat_eq(a318, a319) {
        Ok(0u64)
    } else {
        let a322 = f55(v0.clone(), {
            let a320 = v1;
            let a321 = 1u64;
            nat_sub(a320, a321)
        })?;
        let a323 = 1u64;
        if nat_eq(a322, a323) {
            Ok(1u64)
        } else {
            let a324 = 2u64;
            let a325 = v1;
            if if nat_le(a324, a325) {
                let a328 = f55(v0.clone(), {
                    let a326 = v1;
                    let a327 = 2u64;
                    nat_sub(a326, a327)
                })?;
                let a329 = 2u64;
                nat_eq(a328, a329)
            } else {
                false
            } {
                Ok(2u64)
            } else {
                let a330 = 3u64;
                let a331 = v1;
                if if nat_le(a330, a331) {
                    let a334 = f55(v0, {
                        let a332 = v1;
                        let a333 = 3u64;
                        nat_sub(a332, a333)
                    })?;
                    let a335 = 3u64;
                    nat_eq(a334, a335)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            }
        }
    }
}

pub fn f76(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a336 = v0.clone();
        length_bytes(a336)
    };
    let v3: u64 = {
        let a337 = v1.clone();
        length_bytes(a337)
    };
    let v4: u64 = {
        let (h338, _) = f38(Fn7::F86(v1.clone(), v3, v0.clone(), v2), {
            let a339 = v2;
            let a340 = 1u64;
            nat_add(a339, a340)?
        }, 0u64)?;
        h338
    };
    if f82(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f77(v0: Bytes) -> Option<Bytes> {
    let m341 = f49(v0, f7(Str::lit(" ")));
    match m341 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f49(v1, f7(Str::lit("info: ")))
        }
    }
}

pub fn f78(v0: Bytes) -> R<bool> {
    let v2: Bytes = {
        let m342 = f49(v0.clone(), f7(Str::lit("+")));
        match m342 {
            None => {
                v0
            }
            Some(v1) => {
                v1
            }
        }
    };
    let v3: bool = f87(f20(v2.clone())?);
    let a344 = 0u64;
    let a345 = {
        let a343 = v2.clone();
        length_bytes(a343)
    };
    if nat_lt(a344, a345) {
        if f88(v2)? {
            Ok(v3)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f79(v0: u8, v1: u8, v2: u8) -> bool {
    let m348 = {
        let a346 = v0;
        let a347 = v1;
        fixed_u8::checked_sub(a346, a347)
    };
    match m348 {
        None => {
            false
        }
        Some(_) => {
            let m351 = {
                let a349 = v2;
                let a350 = v0;
                fixed_u8::checked_sub(a349, a350)
            };
            match m351 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f80(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a353 = {
        let (_, h352) = v1;
        h352
    };
    let a354 = 2u64;
    if nat_eq(a353, a354) {
        Ok(None::<(u64, u64)>)
    } else {
        let m356 = f51(v0, {
            let (h355, _) = v1;
            h355
        });
        match m356 {
            None => {
                Ok(None::<(u64, u64)>)
            }
            Some(v2) => {
                Ok(Some(({
                    let a358 = {
                        let (h357, _) = v1;
                        h357
                    };
                    let a359 = 1u64;
                    nat_add(a358, a359)?
                }, f89({
                    let (_, h360) = v1;
                    h360
                }, v2))))
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: Bytes, v2: u64) -> Option<u64> {
    f90(v1, v0, v2)
}

pub fn f82(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m365 = {
        let a362 = v0;
        let a363 = v2;
        let a364 = {
            let a361 = v1.clone();
            length_bytes(a361)
        };
        slice_bytes(a362, a363, a364)
    };
    match m365 {
        None => {
            false
        }
        Some(v3) => {
            let a366 = v3;
            let a367 = v1;
            equal(a366, a367)
        }
    }
}

pub fn f83(v0: Fn9, p371: Result<List<Bytes>, Bytes>, p372: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m370 = (p371, p372);
    {
        let mut m370 = m370;
        loop {
            let step = {
                let (v1, v2) = m370;
                let m368 = v2;
                match m368.uncons() {
                    None => {
                        Err::<(Result<List<Bytes>, Bytes>, List<Bytes>), R<Result<List<Bytes>, Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Bytes>, Bytes>, List<Bytes>), R<Result<List<Bytes>, Bytes>>>(({
                            let c369: Fn9 = v0.clone();
                            c369.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Result<List<Bytes>, Bytes>, v1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
    f91(v0, v1)
}

pub fn f85(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a373 = v0.clone();
        length_bytes(a373)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h374, _) = f15(Fn2::F92(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h374
    };
    let v4: u64 = {
        let (h375, _) = {
            let (_, h376) = v3.clone();
            h376
        };
        h375
    };
    Ok(f17(List::cons(f18(v0, v4, {
        let a377 = v2;
        let a378 = v4;
        nat_sub(a377, a378)
    }), {
        let (_, h379) = {
            let (_, h380) = v3;
            h380
        };
        h379
    })))
}

pub fn f86(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f82(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a383 = {
            let a381 = v4;
            let a382 = v1;
            nat_add(a381, a382)?
        };
        let a384 = v3;
        if nat_lt(a383, a384) {
            Ok(Some({
                let a385 = v4;
                let a386 = 1u64;
                nat_add(a385, a386)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f87(v0: Bytes) -> bool {
    let a388 = {
        let a387 = v0.clone();
        length_bytes(a387)
    };
    let a389 = 20u64;
    if nat_lt(a388, a389) {
        true
    } else {
        let a391 = {
            let a390 = v0.clone();
            length_bytes(a390)
        };
        let a392 = 20u64;
        if nat_eq(a391, a392) {
            let a399 = {
                let a397 = {
                    let a393 = v0;
                    let a394 = f7(Str::lit("18446744073709551615"));
                    compare_bytes(a393, a394)
                };
                let a398 = {
                    let a395 = Bytes::lit(&[0x01]);
                    let a396 = Bytes::lit(&[0x00]);
                    compare_bytes(a395, a396)
                };
                equal(a397, a398)
            };
            bool_not(a399)
        } else {
            false
        }
    }
}

pub fn f88(v0: Bytes) -> R<bool> {
    let a403 = {
        let (h400, _) = f38(Fn7::F93(v0.clone()), {
            let a401 = v0.clone();
            length_bytes(a401)
        }, 0u64)?;
        h400
    };
    let a404 = {
        let a402 = v0;
        length_bytes(a402)
    };
    Ok(nat_eq(a403, a404))
}

pub fn f89(v0: u64, v1: u8) -> u64 {
    let a405 = v0;
    let a406 = 2u64;
    if nat_eq(a405, a406) {
        2u64
    } else {
        let a407 = v1;
        let a408 = 46u8;
        if equal(a407, a408) {
            let a409 = v0;
            let a410 = 1u64;
            if nat_eq(a409, a410) {
                0u64
            } else {
                2u64
            }
        } else {
            if f94(v1) {
                1u64
            } else {
                let a411 = v0;
                let a412 = 1u64;
                if nat_eq(a411, a412) {
                    if f95(v1) {
                        1u64
                    } else {
                        2u64
                    }
                } else {
                    2u64
                }
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: Bytes, v2: u64) -> Option<u64> {
    if f82(v0, v1, v2) {
        None::<u64>
    } else {
        let a413 = 0u64;
        let a414 = v2;
        if nat_lt(a413, a414) {
            Some({
                let a415 = v2;
                let a416 = 1u64;
                nat_sub(a415, a416)
            })
        } else {
            None::<u64>
        }
    }
}

pub fn f91(v0: Result<List<Bytes>, Bytes>, v1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
    let m417 = v0.clone();
    match m417 {
        Ok(v3) => {
            let v4: Bytes = f32(v1)?;
            if f61(v4.clone())? {
                let m418 = f96(v4.clone(), v3);
                match m418 {
                    None => {
                        Ok(Err::<List<Bytes>, Bytes>(f6(f7(Str::lit("duplicate axiom name ")), f40(v4))))
                    }
                    Some(v5) => {
                        Ok(Ok::<List<Bytes>, Bytes>(v5))
                    }
                }
            } else {
                Ok(Err::<List<Bytes>, Bytes>(f60(v4)))
            }
        }
        Err(_) => {
            Ok(v0)
        }
    }
}

pub fn f92(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f97(v1, v0, v2)
}

pub fn f93(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f98(v0, v1)
}

pub fn f94(v0: u8) -> bool {
    if f79(v0, 65u8, 90u8) {
        true
    } else {
        if f79(v0, 97u8, 122u8) {
            true
        } else {
            let a419 = v0;
            let a420 = 95u8;
            equal(a419, a420)
        }
    }
}

pub fn f95(v0: u8) -> bool {
    if f94(v0) {
        true
    } else {
        if f79(v0, 48u8, 57u8) {
            true
        } else {
            let a421 = v0;
            let a422 = 63u8;
            if equal(a421, a422) {
                true
            } else {
                let a423 = v0;
                let a424 = 39u8;
                equal(a423, a424)
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: List<Bytes>) -> Option<List<Bytes>> {
    let m425 = v1;
    match m425.uncons() {
        None => {
            Some(List::cons(v0, List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let a426 = v0.clone();
            let a427 = v2.clone();
            if equal(a426, a427) {
                None::<List<Bytes>>
            } else {
                if f54(v0.clone(), v2.clone()) {
                    Some(List::cons(v0, List::cons(v2, v3)))
                } else {
                    let m428 = f96(v0, v3);
                    match m428 {
                        None => {
                            None::<List<Bytes>>
                        }
                        Some(v4) => {
                            Some(List::cons(v2, v4))
                        }
                    }
                }
            }
        }
    }
}

pub fn f97(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h429, _) = v2.clone();
        h429
    };
    let v4: u64 = {
        let (h430, _) = {
            let (_, h431) = v2.clone();
            h431
        };
        h430
    };
    let v5: List<Bytes> = {
        let (_, h432) = {
            let (_, h433) = v2;
            h433
        };
        h432
    };
    let m434 = f51(v0.clone(), v3);
    match m434 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a435 = v6;
            let a436 = v1;
            if equal(a435, a436) {
                Ok(Some(({
                    let a437 = v3;
                    let a438 = 1u64;
                    nat_add(a437, a438)?
                }, ({
                    let a439 = v3;
                    let a440 = 1u64;
                    nat_add(a439, a440)?
                }, List::cons(f18(v0, v4, {
                    let a441 = v3;
                    let a442 = v4;
                    nat_sub(a441, a442)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a443 = v3;
                    let a444 = 1u64;
                    nat_add(a443, a444)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f98(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f70(v0, v1, 48u8, 57u8) {
        Ok(Some({
            let a445 = v1;
            let a446 = 1u64;
            nat_add(a445, a446)?
        }))
    } else {
        Ok(None::<u64>)
    }
}
