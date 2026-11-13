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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt1, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: Bytes, p1: (Bytes, Bytes)) -> Bytes {
        match self {
            Fn0::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn1::F12(k0, k1, k2) => f12(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F15,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn2::F15 => f15(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn3::F23 => f23(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F39,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F39 => f39(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> R<Bytes> {
    f1(f2(Str::lit("lexlean-source-v1")), f3(f3(f4(f2(Str::lit("project")), v0), f4(f2(Str::lit("lock")), v1)), f5(Fn0::F6, Bytes::lit(&[]), v2)))
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f7(f8(f3(f3(v0, Bytes::lit(&[0x00])), v1))?)
}

pub fn f2(v0: Str) -> Bytes {
    let a1 = v0;
    utf8_encode(a1)
}

pub fn f3(v0: Bytes, v1: Bytes) -> Bytes {
    let a2 = v0;
    let a3 = v1;
    append_bytes(a2, a3)
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    f3(f3(f3(f9({
        let a4 = v0.clone();
        length_bytes(a4)
    }, 4u64), v0), f9({
        let a5 = v1.clone();
        length_bytes(a5)
    }, 8u64)), v1)
}

pub fn f5(v0: Fn0, p9: Bytes, p10: List<(Bytes, Bytes)>) -> Bytes {
    let m8 = (p9, p10);
    {
        let mut m8 = m8;
        loop {
            let step = {
                let (v1, v2) = m8;
                let m6 = v2;
                match m6.uncons() {
                    None => {
                        Err::<(Bytes, List<(Bytes, Bytes)>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<(Bytes, Bytes)>), Bytes>(({
                            let c7: Fn0 = v0.clone();
                            c7.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Bytes, v1: (Bytes, Bytes)) -> Bytes {
    f3(v0, f3(f4(f2(Str::lit("path")), {
        let (h11, _) = v1;
        h11
    }), f4(f2(Str::lit("source")), {
        let (_, h12) = v1;
        h12
    })))
}

pub fn f7(v0: Adt0) -> R<Bytes> {
    Ok(f3(f3(f3(f3(f3(f3(f3(f10(match v0.clone() {
        Adt0::C0(h13, _, _, _, _, _, _, _) => {
            h13
        }
    })?, f10(match v0.clone() {
        Adt0::C0(_, h14, _, _, _, _, _, _) => {
            h14
        }
    })?), f10(match v0.clone() {
        Adt0::C0(_, _, h15, _, _, _, _, _) => {
            h15
        }
    })?), f10(match v0.clone() {
        Adt0::C0(_, _, _, h16, _, _, _, _) => {
            h16
        }
    })?), f10(match v0.clone() {
        Adt0::C0(_, _, _, _, h17, _, _, _) => {
            h17
        }
    })?), f10(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h18, _, _) => {
            h18
        }
    })?), f10(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h19, _) => {
            h19
        }
    })?), f10(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h20) => {
            h20
        }
    })?))
}

pub fn f8(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a21 = v0.clone();
        length_bytes(a21)
    };
    let v2: u64 = {
        let a27 = {
            let a24 = {
                let a22 = v1;
                let a23 = 72u64;
                nat_add(a22, a23)?
            };
            let a25 = 64u64;
            let a26 = 0u64;
            nat_quot(a24, a25, a26)
        };
        let a28 = 64u64;
        nat_mul(a27, a28)?
    };
    let (_, h29) = f11(Fn1::F12(v0, v1, v2), {
        let a30 = v2;
        let a31 = 64u64;
        let a32 = 0u64;
        nat_quot(a30, a31, a32)
    }, (0u64, f13()))?;
    Ok(h29)
}

pub fn f9(v0: u64, v1: u64) -> Bytes {
    let (_, h33) = f14(Fn2::F15, v1, (v0, Bytes::lit(&[])));
    h33
}

pub fn f10(v0: u32) -> R<Bytes> {
    Ok(f3(f3(f3(f16({
        let v1: u32 = f17(v0, 24u32);
        let a222 = {
            let a126 = {
                let a78 = {
                    let a54 = {
                        let a42 = {
                            let a36 = {
                                let a34 = v1;
                                let a35 = 2147483648u32;
                                fixed_u32::bit_and(a34, a35)
                            };
                            let a37 = 2147483648u32;
                            if equal(a36, a37) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a43 = {
                            let a40 = {
                                let a38 = v1;
                                let a39 = 1073741824u32;
                                fixed_u32::bit_and(a38, a39)
                            };
                            let a41 = 1073741824u32;
                            if equal(a40, a41) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a42, a43)?
                    };
                    let a55 = {
                        let a52 = {
                            let a46 = {
                                let a44 = v1;
                                let a45 = 536870912u32;
                                fixed_u32::bit_and(a44, a45)
                            };
                            let a47 = 536870912u32;
                            if equal(a46, a47) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a53 = {
                            let a50 = {
                                let a48 = v1;
                                let a49 = 268435456u32;
                                fixed_u32::bit_and(a48, a49)
                            };
                            let a51 = 268435456u32;
                            if equal(a50, a51) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a52, a53)?
                    };
                    nat_add(a54, a55)?
                };
                let a79 = {
                    let a76 = {
                        let a64 = {
                            let a58 = {
                                let a56 = v1;
                                let a57 = 134217728u32;
                                fixed_u32::bit_and(a56, a57)
                            };
                            let a59 = 134217728u32;
                            if equal(a58, a59) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a65 = {
                            let a62 = {
                                let a60 = v1;
                                let a61 = 67108864u32;
                                fixed_u32::bit_and(a60, a61)
                            };
                            let a63 = 67108864u32;
                            if equal(a62, a63) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a64, a65)?
                    };
                    let a77 = {
                        let a74 = {
                            let a68 = {
                                let a66 = v1;
                                let a67 = 33554432u32;
                                fixed_u32::bit_and(a66, a67)
                            };
                            let a69 = 33554432u32;
                            if equal(a68, a69) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a75 = {
                            let a72 = {
                                let a70 = v1;
                                let a71 = 16777216u32;
                                fixed_u32::bit_and(a70, a71)
                            };
                            let a73 = 16777216u32;
                            if equal(a72, a73) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a74, a75)?
                    };
                    nat_add(a76, a77)?
                };
                nat_add(a78, a79)?
            };
            let a127 = {
                let a124 = {
                    let a100 = {
                        let a88 = {
                            let a82 = {
                                let a80 = v1;
                                let a81 = 8388608u32;
                                fixed_u32::bit_and(a80, a81)
                            };
                            let a83 = 8388608u32;
                            if equal(a82, a83) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a89 = {
                            let a86 = {
                                let a84 = v1;
                                let a85 = 4194304u32;
                                fixed_u32::bit_and(a84, a85)
                            };
                            let a87 = 4194304u32;
                            if equal(a86, a87) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a88, a89)?
                    };
                    let a101 = {
                        let a98 = {
                            let a92 = {
                                let a90 = v1;
                                let a91 = 2097152u32;
                                fixed_u32::bit_and(a90, a91)
                            };
                            let a93 = 2097152u32;
                            if equal(a92, a93) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a99 = {
                            let a96 = {
                                let a94 = v1;
                                let a95 = 1048576u32;
                                fixed_u32::bit_and(a94, a95)
                            };
                            let a97 = 1048576u32;
                            if equal(a96, a97) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a98, a99)?
                    };
                    nat_add(a100, a101)?
                };
                let a125 = {
                    let a122 = {
                        let a110 = {
                            let a104 = {
                                let a102 = v1;
                                let a103 = 524288u32;
                                fixed_u32::bit_and(a102, a103)
                            };
                            let a105 = 524288u32;
                            if equal(a104, a105) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a111 = {
                            let a108 = {
                                let a106 = v1;
                                let a107 = 262144u32;
                                fixed_u32::bit_and(a106, a107)
                            };
                            let a109 = 262144u32;
                            if equal(a108, a109) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a110, a111)?
                    };
                    let a123 = {
                        let a120 = {
                            let a114 = {
                                let a112 = v1;
                                let a113 = 131072u32;
                                fixed_u32::bit_and(a112, a113)
                            };
                            let a115 = 131072u32;
                            if equal(a114, a115) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a121 = {
                            let a118 = {
                                let a116 = v1;
                                let a117 = 65536u32;
                                fixed_u32::bit_and(a116, a117)
                            };
                            let a119 = 65536u32;
                            if equal(a118, a119) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a120, a121)?
                    };
                    nat_add(a122, a123)?
                };
                nat_add(a124, a125)?
            };
            nat_add(a126, a127)?
        };
        let a223 = {
            let a220 = {
                let a172 = {
                    let a148 = {
                        let a136 = {
                            let a130 = {
                                let a128 = v1;
                                let a129 = 32768u32;
                                fixed_u32::bit_and(a128, a129)
                            };
                            let a131 = 32768u32;
                            if equal(a130, a131) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a137 = {
                            let a134 = {
                                let a132 = v1;
                                let a133 = 16384u32;
                                fixed_u32::bit_and(a132, a133)
                            };
                            let a135 = 16384u32;
                            if equal(a134, a135) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a136, a137)?
                    };
                    let a149 = {
                        let a146 = {
                            let a140 = {
                                let a138 = v1;
                                let a139 = 8192u32;
                                fixed_u32::bit_and(a138, a139)
                            };
                            let a141 = 8192u32;
                            if equal(a140, a141) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a147 = {
                            let a144 = {
                                let a142 = v1;
                                let a143 = 4096u32;
                                fixed_u32::bit_and(a142, a143)
                            };
                            let a145 = 4096u32;
                            if equal(a144, a145) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a146, a147)?
                    };
                    nat_add(a148, a149)?
                };
                let a173 = {
                    let a170 = {
                        let a158 = {
                            let a152 = {
                                let a150 = v1;
                                let a151 = 2048u32;
                                fixed_u32::bit_and(a150, a151)
                            };
                            let a153 = 2048u32;
                            if equal(a152, a153) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a159 = {
                            let a156 = {
                                let a154 = v1;
                                let a155 = 1024u32;
                                fixed_u32::bit_and(a154, a155)
                            };
                            let a157 = 1024u32;
                            if equal(a156, a157) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a158, a159)?
                    };
                    let a171 = {
                        let a168 = {
                            let a162 = {
                                let a160 = v1;
                                let a161 = 512u32;
                                fixed_u32::bit_and(a160, a161)
                            };
                            let a163 = 512u32;
                            if equal(a162, a163) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a169 = {
                            let a166 = {
                                let a164 = v1;
                                let a165 = 256u32;
                                fixed_u32::bit_and(a164, a165)
                            };
                            let a167 = 256u32;
                            if equal(a166, a167) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a168, a169)?
                    };
                    nat_add(a170, a171)?
                };
                nat_add(a172, a173)?
            };
            let a221 = {
                let a218 = {
                    let a194 = {
                        let a182 = {
                            let a176 = {
                                let a174 = v1;
                                let a175 = 128u32;
                                fixed_u32::bit_and(a174, a175)
                            };
                            let a177 = 128u32;
                            if equal(a176, a177) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a183 = {
                            let a180 = {
                                let a178 = v1;
                                let a179 = 64u32;
                                fixed_u32::bit_and(a178, a179)
                            };
                            let a181 = 64u32;
                            if equal(a180, a181) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a182, a183)?
                    };
                    let a195 = {
                        let a192 = {
                            let a186 = {
                                let a184 = v1;
                                let a185 = 32u32;
                                fixed_u32::bit_and(a184, a185)
                            };
                            let a187 = 32u32;
                            if equal(a186, a187) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a193 = {
                            let a190 = {
                                let a188 = v1;
                                let a189 = 16u32;
                                fixed_u32::bit_and(a188, a189)
                            };
                            let a191 = 16u32;
                            if equal(a190, a191) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a192, a193)?
                    };
                    nat_add(a194, a195)?
                };
                let a219 = {
                    let a216 = {
                        let a204 = {
                            let a198 = {
                                let a196 = v1;
                                let a197 = 8u32;
                                fixed_u32::bit_and(a196, a197)
                            };
                            let a199 = 8u32;
                            if equal(a198, a199) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a205 = {
                            let a202 = {
                                let a200 = v1;
                                let a201 = 4u32;
                                fixed_u32::bit_and(a200, a201)
                            };
                            let a203 = 4u32;
                            if equal(a202, a203) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a204, a205)?
                    };
                    let a217 = {
                        let a214 = {
                            let a208 = {
                                let a206 = v1;
                                let a207 = 2u32;
                                fixed_u32::bit_and(a206, a207)
                            };
                            let a209 = 2u32;
                            if equal(a208, a209) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a215 = {
                            let a212 = {
                                let a210 = v1;
                                let a211 = 1u32;
                                fixed_u32::bit_and(a210, a211)
                            };
                            let a213 = 1u32;
                            if equal(a212, a213) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a214, a215)?
                    };
                    nat_add(a216, a217)?
                };
                nat_add(a218, a219)?
            };
            nat_add(a220, a221)?
        };
        nat_add(a222, a223)?
    }), f16({
        let v2: u32 = {
            let a224 = f17(v0, 16u32);
            let a225 = 255u32;
            fixed_u32::bit_and(a224, a225)
        };
        let a414 = {
            let a318 = {
                let a270 = {
                    let a246 = {
                        let a234 = {
                            let a228 = {
                                let a226 = v2;
                                let a227 = 2147483648u32;
                                fixed_u32::bit_and(a226, a227)
                            };
                            let a229 = 2147483648u32;
                            if equal(a228, a229) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a235 = {
                            let a232 = {
                                let a230 = v2;
                                let a231 = 1073741824u32;
                                fixed_u32::bit_and(a230, a231)
                            };
                            let a233 = 1073741824u32;
                            if equal(a232, a233) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a234, a235)?
                    };
                    let a247 = {
                        let a244 = {
                            let a238 = {
                                let a236 = v2;
                                let a237 = 536870912u32;
                                fixed_u32::bit_and(a236, a237)
                            };
                            let a239 = 536870912u32;
                            if equal(a238, a239) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a245 = {
                            let a242 = {
                                let a240 = v2;
                                let a241 = 268435456u32;
                                fixed_u32::bit_and(a240, a241)
                            };
                            let a243 = 268435456u32;
                            if equal(a242, a243) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a244, a245)?
                    };
                    nat_add(a246, a247)?
                };
                let a271 = {
                    let a268 = {
                        let a256 = {
                            let a250 = {
                                let a248 = v2;
                                let a249 = 134217728u32;
                                fixed_u32::bit_and(a248, a249)
                            };
                            let a251 = 134217728u32;
                            if equal(a250, a251) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a257 = {
                            let a254 = {
                                let a252 = v2;
                                let a253 = 67108864u32;
                                fixed_u32::bit_and(a252, a253)
                            };
                            let a255 = 67108864u32;
                            if equal(a254, a255) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a256, a257)?
                    };
                    let a269 = {
                        let a266 = {
                            let a260 = {
                                let a258 = v2;
                                let a259 = 33554432u32;
                                fixed_u32::bit_and(a258, a259)
                            };
                            let a261 = 33554432u32;
                            if equal(a260, a261) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a267 = {
                            let a264 = {
                                let a262 = v2;
                                let a263 = 16777216u32;
                                fixed_u32::bit_and(a262, a263)
                            };
                            let a265 = 16777216u32;
                            if equal(a264, a265) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a266, a267)?
                    };
                    nat_add(a268, a269)?
                };
                nat_add(a270, a271)?
            };
            let a319 = {
                let a316 = {
                    let a292 = {
                        let a280 = {
                            let a274 = {
                                let a272 = v2;
                                let a273 = 8388608u32;
                                fixed_u32::bit_and(a272, a273)
                            };
                            let a275 = 8388608u32;
                            if equal(a274, a275) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a281 = {
                            let a278 = {
                                let a276 = v2;
                                let a277 = 4194304u32;
                                fixed_u32::bit_and(a276, a277)
                            };
                            let a279 = 4194304u32;
                            if equal(a278, a279) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a280, a281)?
                    };
                    let a293 = {
                        let a290 = {
                            let a284 = {
                                let a282 = v2;
                                let a283 = 2097152u32;
                                fixed_u32::bit_and(a282, a283)
                            };
                            let a285 = 2097152u32;
                            if equal(a284, a285) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a291 = {
                            let a288 = {
                                let a286 = v2;
                                let a287 = 1048576u32;
                                fixed_u32::bit_and(a286, a287)
                            };
                            let a289 = 1048576u32;
                            if equal(a288, a289) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a290, a291)?
                    };
                    nat_add(a292, a293)?
                };
                let a317 = {
                    let a314 = {
                        let a302 = {
                            let a296 = {
                                let a294 = v2;
                                let a295 = 524288u32;
                                fixed_u32::bit_and(a294, a295)
                            };
                            let a297 = 524288u32;
                            if equal(a296, a297) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a303 = {
                            let a300 = {
                                let a298 = v2;
                                let a299 = 262144u32;
                                fixed_u32::bit_and(a298, a299)
                            };
                            let a301 = 262144u32;
                            if equal(a300, a301) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a302, a303)?
                    };
                    let a315 = {
                        let a312 = {
                            let a306 = {
                                let a304 = v2;
                                let a305 = 131072u32;
                                fixed_u32::bit_and(a304, a305)
                            };
                            let a307 = 131072u32;
                            if equal(a306, a307) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a313 = {
                            let a310 = {
                                let a308 = v2;
                                let a309 = 65536u32;
                                fixed_u32::bit_and(a308, a309)
                            };
                            let a311 = 65536u32;
                            if equal(a310, a311) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a312, a313)?
                    };
                    nat_add(a314, a315)?
                };
                nat_add(a316, a317)?
            };
            nat_add(a318, a319)?
        };
        let a415 = {
            let a412 = {
                let a364 = {
                    let a340 = {
                        let a328 = {
                            let a322 = {
                                let a320 = v2;
                                let a321 = 32768u32;
                                fixed_u32::bit_and(a320, a321)
                            };
                            let a323 = 32768u32;
                            if equal(a322, a323) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a329 = {
                            let a326 = {
                                let a324 = v2;
                                let a325 = 16384u32;
                                fixed_u32::bit_and(a324, a325)
                            };
                            let a327 = 16384u32;
                            if equal(a326, a327) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a328, a329)?
                    };
                    let a341 = {
                        let a338 = {
                            let a332 = {
                                let a330 = v2;
                                let a331 = 8192u32;
                                fixed_u32::bit_and(a330, a331)
                            };
                            let a333 = 8192u32;
                            if equal(a332, a333) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a339 = {
                            let a336 = {
                                let a334 = v2;
                                let a335 = 4096u32;
                                fixed_u32::bit_and(a334, a335)
                            };
                            let a337 = 4096u32;
                            if equal(a336, a337) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a338, a339)?
                    };
                    nat_add(a340, a341)?
                };
                let a365 = {
                    let a362 = {
                        let a350 = {
                            let a344 = {
                                let a342 = v2;
                                let a343 = 2048u32;
                                fixed_u32::bit_and(a342, a343)
                            };
                            let a345 = 2048u32;
                            if equal(a344, a345) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a351 = {
                            let a348 = {
                                let a346 = v2;
                                let a347 = 1024u32;
                                fixed_u32::bit_and(a346, a347)
                            };
                            let a349 = 1024u32;
                            if equal(a348, a349) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a350, a351)?
                    };
                    let a363 = {
                        let a360 = {
                            let a354 = {
                                let a352 = v2;
                                let a353 = 512u32;
                                fixed_u32::bit_and(a352, a353)
                            };
                            let a355 = 512u32;
                            if equal(a354, a355) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a361 = {
                            let a358 = {
                                let a356 = v2;
                                let a357 = 256u32;
                                fixed_u32::bit_and(a356, a357)
                            };
                            let a359 = 256u32;
                            if equal(a358, a359) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a360, a361)?
                    };
                    nat_add(a362, a363)?
                };
                nat_add(a364, a365)?
            };
            let a413 = {
                let a410 = {
                    let a386 = {
                        let a374 = {
                            let a368 = {
                                let a366 = v2;
                                let a367 = 128u32;
                                fixed_u32::bit_and(a366, a367)
                            };
                            let a369 = 128u32;
                            if equal(a368, a369) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a375 = {
                            let a372 = {
                                let a370 = v2;
                                let a371 = 64u32;
                                fixed_u32::bit_and(a370, a371)
                            };
                            let a373 = 64u32;
                            if equal(a372, a373) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a374, a375)?
                    };
                    let a387 = {
                        let a384 = {
                            let a378 = {
                                let a376 = v2;
                                let a377 = 32u32;
                                fixed_u32::bit_and(a376, a377)
                            };
                            let a379 = 32u32;
                            if equal(a378, a379) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a385 = {
                            let a382 = {
                                let a380 = v2;
                                let a381 = 16u32;
                                fixed_u32::bit_and(a380, a381)
                            };
                            let a383 = 16u32;
                            if equal(a382, a383) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a384, a385)?
                    };
                    nat_add(a386, a387)?
                };
                let a411 = {
                    let a408 = {
                        let a396 = {
                            let a390 = {
                                let a388 = v2;
                                let a389 = 8u32;
                                fixed_u32::bit_and(a388, a389)
                            };
                            let a391 = 8u32;
                            if equal(a390, a391) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a397 = {
                            let a394 = {
                                let a392 = v2;
                                let a393 = 4u32;
                                fixed_u32::bit_and(a392, a393)
                            };
                            let a395 = 4u32;
                            if equal(a394, a395) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a396, a397)?
                    };
                    let a409 = {
                        let a406 = {
                            let a400 = {
                                let a398 = v2;
                                let a399 = 2u32;
                                fixed_u32::bit_and(a398, a399)
                            };
                            let a401 = 2u32;
                            if equal(a400, a401) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a407 = {
                            let a404 = {
                                let a402 = v2;
                                let a403 = 1u32;
                                fixed_u32::bit_and(a402, a403)
                            };
                            let a405 = 1u32;
                            if equal(a404, a405) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a406, a407)?
                    };
                    nat_add(a408, a409)?
                };
                nat_add(a410, a411)?
            };
            nat_add(a412, a413)?
        };
        nat_add(a414, a415)?
    })), f16({
        let v3: u32 = {
            let a416 = f17(v0, 8u32);
            let a417 = 255u32;
            fixed_u32::bit_and(a416, a417)
        };
        let a606 = {
            let a510 = {
                let a462 = {
                    let a438 = {
                        let a426 = {
                            let a420 = {
                                let a418 = v3;
                                let a419 = 2147483648u32;
                                fixed_u32::bit_and(a418, a419)
                            };
                            let a421 = 2147483648u32;
                            if equal(a420, a421) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a427 = {
                            let a424 = {
                                let a422 = v3;
                                let a423 = 1073741824u32;
                                fixed_u32::bit_and(a422, a423)
                            };
                            let a425 = 1073741824u32;
                            if equal(a424, a425) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a426, a427)?
                    };
                    let a439 = {
                        let a436 = {
                            let a430 = {
                                let a428 = v3;
                                let a429 = 536870912u32;
                                fixed_u32::bit_and(a428, a429)
                            };
                            let a431 = 536870912u32;
                            if equal(a430, a431) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a437 = {
                            let a434 = {
                                let a432 = v3;
                                let a433 = 268435456u32;
                                fixed_u32::bit_and(a432, a433)
                            };
                            let a435 = 268435456u32;
                            if equal(a434, a435) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a436, a437)?
                    };
                    nat_add(a438, a439)?
                };
                let a463 = {
                    let a460 = {
                        let a448 = {
                            let a442 = {
                                let a440 = v3;
                                let a441 = 134217728u32;
                                fixed_u32::bit_and(a440, a441)
                            };
                            let a443 = 134217728u32;
                            if equal(a442, a443) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a449 = {
                            let a446 = {
                                let a444 = v3;
                                let a445 = 67108864u32;
                                fixed_u32::bit_and(a444, a445)
                            };
                            let a447 = 67108864u32;
                            if equal(a446, a447) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a448, a449)?
                    };
                    let a461 = {
                        let a458 = {
                            let a452 = {
                                let a450 = v3;
                                let a451 = 33554432u32;
                                fixed_u32::bit_and(a450, a451)
                            };
                            let a453 = 33554432u32;
                            if equal(a452, a453) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a459 = {
                            let a456 = {
                                let a454 = v3;
                                let a455 = 16777216u32;
                                fixed_u32::bit_and(a454, a455)
                            };
                            let a457 = 16777216u32;
                            if equal(a456, a457) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a458, a459)?
                    };
                    nat_add(a460, a461)?
                };
                nat_add(a462, a463)?
            };
            let a511 = {
                let a508 = {
                    let a484 = {
                        let a472 = {
                            let a466 = {
                                let a464 = v3;
                                let a465 = 8388608u32;
                                fixed_u32::bit_and(a464, a465)
                            };
                            let a467 = 8388608u32;
                            if equal(a466, a467) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a473 = {
                            let a470 = {
                                let a468 = v3;
                                let a469 = 4194304u32;
                                fixed_u32::bit_and(a468, a469)
                            };
                            let a471 = 4194304u32;
                            if equal(a470, a471) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a472, a473)?
                    };
                    let a485 = {
                        let a482 = {
                            let a476 = {
                                let a474 = v3;
                                let a475 = 2097152u32;
                                fixed_u32::bit_and(a474, a475)
                            };
                            let a477 = 2097152u32;
                            if equal(a476, a477) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a483 = {
                            let a480 = {
                                let a478 = v3;
                                let a479 = 1048576u32;
                                fixed_u32::bit_and(a478, a479)
                            };
                            let a481 = 1048576u32;
                            if equal(a480, a481) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a482, a483)?
                    };
                    nat_add(a484, a485)?
                };
                let a509 = {
                    let a506 = {
                        let a494 = {
                            let a488 = {
                                let a486 = v3;
                                let a487 = 524288u32;
                                fixed_u32::bit_and(a486, a487)
                            };
                            let a489 = 524288u32;
                            if equal(a488, a489) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a495 = {
                            let a492 = {
                                let a490 = v3;
                                let a491 = 262144u32;
                                fixed_u32::bit_and(a490, a491)
                            };
                            let a493 = 262144u32;
                            if equal(a492, a493) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a494, a495)?
                    };
                    let a507 = {
                        let a504 = {
                            let a498 = {
                                let a496 = v3;
                                let a497 = 131072u32;
                                fixed_u32::bit_and(a496, a497)
                            };
                            let a499 = 131072u32;
                            if equal(a498, a499) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a505 = {
                            let a502 = {
                                let a500 = v3;
                                let a501 = 65536u32;
                                fixed_u32::bit_and(a500, a501)
                            };
                            let a503 = 65536u32;
                            if equal(a502, a503) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a504, a505)?
                    };
                    nat_add(a506, a507)?
                };
                nat_add(a508, a509)?
            };
            nat_add(a510, a511)?
        };
        let a607 = {
            let a604 = {
                let a556 = {
                    let a532 = {
                        let a520 = {
                            let a514 = {
                                let a512 = v3;
                                let a513 = 32768u32;
                                fixed_u32::bit_and(a512, a513)
                            };
                            let a515 = 32768u32;
                            if equal(a514, a515) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a521 = {
                            let a518 = {
                                let a516 = v3;
                                let a517 = 16384u32;
                                fixed_u32::bit_and(a516, a517)
                            };
                            let a519 = 16384u32;
                            if equal(a518, a519) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a520, a521)?
                    };
                    let a533 = {
                        let a530 = {
                            let a524 = {
                                let a522 = v3;
                                let a523 = 8192u32;
                                fixed_u32::bit_and(a522, a523)
                            };
                            let a525 = 8192u32;
                            if equal(a524, a525) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a531 = {
                            let a528 = {
                                let a526 = v3;
                                let a527 = 4096u32;
                                fixed_u32::bit_and(a526, a527)
                            };
                            let a529 = 4096u32;
                            if equal(a528, a529) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a530, a531)?
                    };
                    nat_add(a532, a533)?
                };
                let a557 = {
                    let a554 = {
                        let a542 = {
                            let a536 = {
                                let a534 = v3;
                                let a535 = 2048u32;
                                fixed_u32::bit_and(a534, a535)
                            };
                            let a537 = 2048u32;
                            if equal(a536, a537) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a543 = {
                            let a540 = {
                                let a538 = v3;
                                let a539 = 1024u32;
                                fixed_u32::bit_and(a538, a539)
                            };
                            let a541 = 1024u32;
                            if equal(a540, a541) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a542, a543)?
                    };
                    let a555 = {
                        let a552 = {
                            let a546 = {
                                let a544 = v3;
                                let a545 = 512u32;
                                fixed_u32::bit_and(a544, a545)
                            };
                            let a547 = 512u32;
                            if equal(a546, a547) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a553 = {
                            let a550 = {
                                let a548 = v3;
                                let a549 = 256u32;
                                fixed_u32::bit_and(a548, a549)
                            };
                            let a551 = 256u32;
                            if equal(a550, a551) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a552, a553)?
                    };
                    nat_add(a554, a555)?
                };
                nat_add(a556, a557)?
            };
            let a605 = {
                let a602 = {
                    let a578 = {
                        let a566 = {
                            let a560 = {
                                let a558 = v3;
                                let a559 = 128u32;
                                fixed_u32::bit_and(a558, a559)
                            };
                            let a561 = 128u32;
                            if equal(a560, a561) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a567 = {
                            let a564 = {
                                let a562 = v3;
                                let a563 = 64u32;
                                fixed_u32::bit_and(a562, a563)
                            };
                            let a565 = 64u32;
                            if equal(a564, a565) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a566, a567)?
                    };
                    let a579 = {
                        let a576 = {
                            let a570 = {
                                let a568 = v3;
                                let a569 = 32u32;
                                fixed_u32::bit_and(a568, a569)
                            };
                            let a571 = 32u32;
                            if equal(a570, a571) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a577 = {
                            let a574 = {
                                let a572 = v3;
                                let a573 = 16u32;
                                fixed_u32::bit_and(a572, a573)
                            };
                            let a575 = 16u32;
                            if equal(a574, a575) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a576, a577)?
                    };
                    nat_add(a578, a579)?
                };
                let a603 = {
                    let a600 = {
                        let a588 = {
                            let a582 = {
                                let a580 = v3;
                                let a581 = 8u32;
                                fixed_u32::bit_and(a580, a581)
                            };
                            let a583 = 8u32;
                            if equal(a582, a583) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a589 = {
                            let a586 = {
                                let a584 = v3;
                                let a585 = 4u32;
                                fixed_u32::bit_and(a584, a585)
                            };
                            let a587 = 4u32;
                            if equal(a586, a587) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a588, a589)?
                    };
                    let a601 = {
                        let a598 = {
                            let a592 = {
                                let a590 = v3;
                                let a591 = 2u32;
                                fixed_u32::bit_and(a590, a591)
                            };
                            let a593 = 2u32;
                            if equal(a592, a593) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a599 = {
                            let a596 = {
                                let a594 = v3;
                                let a595 = 1u32;
                                fixed_u32::bit_and(a594, a595)
                            };
                            let a597 = 1u32;
                            if equal(a596, a597) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a598, a599)?
                    };
                    nat_add(a600, a601)?
                };
                nat_add(a602, a603)?
            };
            nat_add(a604, a605)?
        };
        nat_add(a606, a607)?
    })), f16({
        let v4: u32 = {
            let a608 = v0;
            let a609 = 255u32;
            fixed_u32::bit_and(a608, a609)
        };
        let a798 = {
            let a702 = {
                let a654 = {
                    let a630 = {
                        let a618 = {
                            let a612 = {
                                let a610 = v4;
                                let a611 = 2147483648u32;
                                fixed_u32::bit_and(a610, a611)
                            };
                            let a613 = 2147483648u32;
                            if equal(a612, a613) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a619 = {
                            let a616 = {
                                let a614 = v4;
                                let a615 = 1073741824u32;
                                fixed_u32::bit_and(a614, a615)
                            };
                            let a617 = 1073741824u32;
                            if equal(a616, a617) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a618, a619)?
                    };
                    let a631 = {
                        let a628 = {
                            let a622 = {
                                let a620 = v4;
                                let a621 = 536870912u32;
                                fixed_u32::bit_and(a620, a621)
                            };
                            let a623 = 536870912u32;
                            if equal(a622, a623) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a629 = {
                            let a626 = {
                                let a624 = v4;
                                let a625 = 268435456u32;
                                fixed_u32::bit_and(a624, a625)
                            };
                            let a627 = 268435456u32;
                            if equal(a626, a627) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a628, a629)?
                    };
                    nat_add(a630, a631)?
                };
                let a655 = {
                    let a652 = {
                        let a640 = {
                            let a634 = {
                                let a632 = v4;
                                let a633 = 134217728u32;
                                fixed_u32::bit_and(a632, a633)
                            };
                            let a635 = 134217728u32;
                            if equal(a634, a635) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a641 = {
                            let a638 = {
                                let a636 = v4;
                                let a637 = 67108864u32;
                                fixed_u32::bit_and(a636, a637)
                            };
                            let a639 = 67108864u32;
                            if equal(a638, a639) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a640, a641)?
                    };
                    let a653 = {
                        let a650 = {
                            let a644 = {
                                let a642 = v4;
                                let a643 = 33554432u32;
                                fixed_u32::bit_and(a642, a643)
                            };
                            let a645 = 33554432u32;
                            if equal(a644, a645) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a651 = {
                            let a648 = {
                                let a646 = v4;
                                let a647 = 16777216u32;
                                fixed_u32::bit_and(a646, a647)
                            };
                            let a649 = 16777216u32;
                            if equal(a648, a649) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a650, a651)?
                    };
                    nat_add(a652, a653)?
                };
                nat_add(a654, a655)?
            };
            let a703 = {
                let a700 = {
                    let a676 = {
                        let a664 = {
                            let a658 = {
                                let a656 = v4;
                                let a657 = 8388608u32;
                                fixed_u32::bit_and(a656, a657)
                            };
                            let a659 = 8388608u32;
                            if equal(a658, a659) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a665 = {
                            let a662 = {
                                let a660 = v4;
                                let a661 = 4194304u32;
                                fixed_u32::bit_and(a660, a661)
                            };
                            let a663 = 4194304u32;
                            if equal(a662, a663) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a664, a665)?
                    };
                    let a677 = {
                        let a674 = {
                            let a668 = {
                                let a666 = v4;
                                let a667 = 2097152u32;
                                fixed_u32::bit_and(a666, a667)
                            };
                            let a669 = 2097152u32;
                            if equal(a668, a669) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a675 = {
                            let a672 = {
                                let a670 = v4;
                                let a671 = 1048576u32;
                                fixed_u32::bit_and(a670, a671)
                            };
                            let a673 = 1048576u32;
                            if equal(a672, a673) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a674, a675)?
                    };
                    nat_add(a676, a677)?
                };
                let a701 = {
                    let a698 = {
                        let a686 = {
                            let a680 = {
                                let a678 = v4;
                                let a679 = 524288u32;
                                fixed_u32::bit_and(a678, a679)
                            };
                            let a681 = 524288u32;
                            if equal(a680, a681) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a687 = {
                            let a684 = {
                                let a682 = v4;
                                let a683 = 262144u32;
                                fixed_u32::bit_and(a682, a683)
                            };
                            let a685 = 262144u32;
                            if equal(a684, a685) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a686, a687)?
                    };
                    let a699 = {
                        let a696 = {
                            let a690 = {
                                let a688 = v4;
                                let a689 = 131072u32;
                                fixed_u32::bit_and(a688, a689)
                            };
                            let a691 = 131072u32;
                            if equal(a690, a691) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a697 = {
                            let a694 = {
                                let a692 = v4;
                                let a693 = 65536u32;
                                fixed_u32::bit_and(a692, a693)
                            };
                            let a695 = 65536u32;
                            if equal(a694, a695) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a696, a697)?
                    };
                    nat_add(a698, a699)?
                };
                nat_add(a700, a701)?
            };
            nat_add(a702, a703)?
        };
        let a799 = {
            let a796 = {
                let a748 = {
                    let a724 = {
                        let a712 = {
                            let a706 = {
                                let a704 = v4;
                                let a705 = 32768u32;
                                fixed_u32::bit_and(a704, a705)
                            };
                            let a707 = 32768u32;
                            if equal(a706, a707) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a713 = {
                            let a710 = {
                                let a708 = v4;
                                let a709 = 16384u32;
                                fixed_u32::bit_and(a708, a709)
                            };
                            let a711 = 16384u32;
                            if equal(a710, a711) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a712, a713)?
                    };
                    let a725 = {
                        let a722 = {
                            let a716 = {
                                let a714 = v4;
                                let a715 = 8192u32;
                                fixed_u32::bit_and(a714, a715)
                            };
                            let a717 = 8192u32;
                            if equal(a716, a717) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a723 = {
                            let a720 = {
                                let a718 = v4;
                                let a719 = 4096u32;
                                fixed_u32::bit_and(a718, a719)
                            };
                            let a721 = 4096u32;
                            if equal(a720, a721) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a722, a723)?
                    };
                    nat_add(a724, a725)?
                };
                let a749 = {
                    let a746 = {
                        let a734 = {
                            let a728 = {
                                let a726 = v4;
                                let a727 = 2048u32;
                                fixed_u32::bit_and(a726, a727)
                            };
                            let a729 = 2048u32;
                            if equal(a728, a729) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a735 = {
                            let a732 = {
                                let a730 = v4;
                                let a731 = 1024u32;
                                fixed_u32::bit_and(a730, a731)
                            };
                            let a733 = 1024u32;
                            if equal(a732, a733) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a734, a735)?
                    };
                    let a747 = {
                        let a744 = {
                            let a738 = {
                                let a736 = v4;
                                let a737 = 512u32;
                                fixed_u32::bit_and(a736, a737)
                            };
                            let a739 = 512u32;
                            if equal(a738, a739) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a745 = {
                            let a742 = {
                                let a740 = v4;
                                let a741 = 256u32;
                                fixed_u32::bit_and(a740, a741)
                            };
                            let a743 = 256u32;
                            if equal(a742, a743) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a744, a745)?
                    };
                    nat_add(a746, a747)?
                };
                nat_add(a748, a749)?
            };
            let a797 = {
                let a794 = {
                    let a770 = {
                        let a758 = {
                            let a752 = {
                                let a750 = v4;
                                let a751 = 128u32;
                                fixed_u32::bit_and(a750, a751)
                            };
                            let a753 = 128u32;
                            if equal(a752, a753) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a759 = {
                            let a756 = {
                                let a754 = v4;
                                let a755 = 64u32;
                                fixed_u32::bit_and(a754, a755)
                            };
                            let a757 = 64u32;
                            if equal(a756, a757) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a758, a759)?
                    };
                    let a771 = {
                        let a768 = {
                            let a762 = {
                                let a760 = v4;
                                let a761 = 32u32;
                                fixed_u32::bit_and(a760, a761)
                            };
                            let a763 = 32u32;
                            if equal(a762, a763) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a769 = {
                            let a766 = {
                                let a764 = v4;
                                let a765 = 16u32;
                                fixed_u32::bit_and(a764, a765)
                            };
                            let a767 = 16u32;
                            if equal(a766, a767) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a768, a769)?
                    };
                    nat_add(a770, a771)?
                };
                let a795 = {
                    let a792 = {
                        let a780 = {
                            let a774 = {
                                let a772 = v4;
                                let a773 = 8u32;
                                fixed_u32::bit_and(a772, a773)
                            };
                            let a775 = 8u32;
                            if equal(a774, a775) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a781 = {
                            let a778 = {
                                let a776 = v4;
                                let a777 = 4u32;
                                fixed_u32::bit_and(a776, a777)
                            };
                            let a779 = 4u32;
                            if equal(a778, a779) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a780, a781)?
                    };
                    let a793 = {
                        let a790 = {
                            let a784 = {
                                let a782 = v4;
                                let a783 = 2u32;
                                fixed_u32::bit_and(a782, a783)
                            };
                            let a785 = 2u32;
                            if equal(a784, a785) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a791 = {
                            let a788 = {
                                let a786 = v4;
                                let a787 = 1u32;
                                fixed_u32::bit_and(a786, a787)
                            };
                            let a789 = 1u32;
                            if equal(a788, a789) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a790, a791)?
                    };
                    nat_add(a792, a793)?
                };
                nat_add(a794, a795)?
            };
            nat_add(a796, a797)?
        };
        nat_add(a798, a799)?
    })))
}

pub fn f11(v0: Fn1, p803: u64, p804: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m802 = (p803, p804);
    {
        let mut m802 = m802;
        loop {
            let step = {
                let (v1, v2) = m802;
                let m800 = v1;
                if m800 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m800 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c801: Fn1 = v0.clone();
                        c801.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a806 = {
            let (h805, _) = v3.clone();
            h805
        };
        let a807 = 64u64;
        nat_add(a806, a807)?
    }, f18({
        let (_, h808) = v3;
        h808
    }, f19(v0, v1, v2, {
        let (h809, _) = v3;
        h809
    })?)?))
}

pub fn f13() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f14(v0: Fn2, p813: u64, p814: (u64, Bytes)) -> (u64, Bytes) {
    let m812 = (p813, p814);
    {
        let mut m812 = m812;
        loop {
            let step = {
                let (v1, v2) = m812;
                let m810 = v1;
                if m810 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m810 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c811: Fn2 = v0.clone();
                        c811.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a816 = {
            let (h815, _) = v0.clone();
            h815
        };
        let a817 = 256u64;
        let a818 = 0u64;
        nat_quot(a816, a817, a818)
    }, f3(f16({
        let a820 = {
            let (h819, _) = v0;
            h819
        };
        let a821 = 256u64;
        let a822 = 0u64;
        nat_rem(a820, a821, a822)
    }), {
        let (_, h823) = v0;
        h823
    }))
}

pub fn f16(v0: u64) -> Bytes {
    f20(f21(), v0)
}

pub fn f17(v0: u32, v1: u32) -> u32 {
    let m826 = {
        let a824 = v0;
        let a825 = v1;
        fixed_u32::shift_right(a824, a825)
    };
    match m826 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f18(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f22(Fn3::F23, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h827, _, _, _, _, _, _, _) => {
            h827
        }
    }, match v0.clone() {
        Adt0::C0(_, h828, _, _, _, _, _, _) => {
            h828
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h829, _, _, _, _, _) => {
            h829
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h830, _, _, _, _) => {
            h830
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h831, _, _, _) => {
            h831
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h832, _, _) => {
            h832
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h833, _) => {
            h833
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h834) => {
            h834
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f24(match v0.clone() {
        Adt0::C0(h835, _, _, _, _, _, _, _) => {
            h835
        }
    }, match v2.clone() {
        Adt2::C0(h836, _, _, _, _, _, _, _, _, _) => {
            h836
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, h837, _, _, _, _, _, _) => {
            h837
        }
    }, match v2.clone() {
        Adt2::C0(_, h838, _, _, _, _, _, _, _, _) => {
            h838
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, h839, _, _, _, _, _) => {
            h839
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h840, _, _, _, _, _, _, _) => {
            h840
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, _, h841, _, _, _, _) => {
            h841
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h842, _, _, _, _, _, _) => {
            h842
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, _, _, h843, _, _, _) => {
            h843
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h844, _, _, _, _, _) => {
            h844
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h845, _, _) => {
            h845
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h846, _, _, _, _) => {
            h846
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h847, _) => {
            h847
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h848, _, _, _) => {
            h848
        }
    }), f24(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h849) => {
            h849
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h850, _, _) => {
            h850
        }
    })))
}

pub fn f19(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f25(v0.clone(), v1, v2, v3)?, f25(v0.clone(), v1, v2, {
        let a851 = v3;
        let a852 = 4u64;
        nat_add(a851, a852)?
    })?, f25(v0.clone(), v1, v2, {
        let a853 = v3;
        let a854 = 8u64;
        nat_add(a853, a854)?
    })?, f25(v0.clone(), v1, v2, {
        let a855 = v3;
        let a856 = 12u64;
        nat_add(a855, a856)?
    })?, f25(v0.clone(), v1, v2, {
        let a857 = v3;
        let a858 = 16u64;
        nat_add(a857, a858)?
    })?, f25(v0.clone(), v1, v2, {
        let a859 = v3;
        let a860 = 20u64;
        nat_add(a859, a860)?
    })?, f25(v0.clone(), v1, v2, {
        let a861 = v3;
        let a862 = 24u64;
        nat_add(a861, a862)?
    })?, f25(v0.clone(), v1, v2, {
        let a863 = v3;
        let a864 = 28u64;
        nat_add(a863, a864)?
    })?, f25(v0.clone(), v1, v2, {
        let a865 = v3;
        let a866 = 32u64;
        nat_add(a865, a866)?
    })?, f25(v0.clone(), v1, v2, {
        let a867 = v3;
        let a868 = 36u64;
        nat_add(a867, a868)?
    })?, f25(v0.clone(), v1, v2, {
        let a869 = v3;
        let a870 = 40u64;
        nat_add(a869, a870)?
    })?, f25(v0.clone(), v1, v2, {
        let a871 = v3;
        let a872 = 44u64;
        nat_add(a871, a872)?
    })?, f25(v0.clone(), v1, v2, {
        let a873 = v3;
        let a874 = 48u64;
        nat_add(a873, a874)?
    })?, f25(v0.clone(), v1, v2, {
        let a875 = v3;
        let a876 = 52u64;
        nat_add(a875, a876)?
    })?, f25(v0.clone(), v1, v2, {
        let a877 = v3;
        let a878 = 56u64;
        nat_add(a877, a878)?
    })?, f25(v0, v1, v2, {
        let a879 = v3;
        let a880 = 60u64;
        nat_add(a879, a880)?
    })?))
}

pub fn f20(v0: Bytes, v1: u64) -> Bytes {
    let m884 = {
        let a881 = v0;
        let a882 = v1;
        let a883 = 1u64;
        slice_bytes(a881, a882, a883)
    };
    match m884 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f21() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f22(v0: Fn3, p888: u64, p889: Adt2) -> R<Adt2> {
    let m887 = (p888, p889);
    {
        let mut m887 = m887;
        loop {
            let step = {
                let (v1, v2) = m887;
                let m885 = v1;
                if m885 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m885 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c886: Fn3 = v0.clone();
                        c886.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Adt2) -> R<Adt2> {
    f26(v0)
}

pub fn f24(v0: u32, v1: u32) -> u32 {
    let m892 = {
        let a890 = f27(v0);
        let a891 = f27(v1);
        fixed_u64::checked_add(a890, a891)
    };
    match m892 {
        None => {
            0u32
        }
        Some(v2) => {
            f28(v2)
        }
    }
}

pub fn f25(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a895 = {
        let a893 = v3;
        let a894 = 8u64;
        nat_add(a893, a894)?
    };
    let a896 = v2;
    if nat_eq(a895, a896) {
        Ok(f29({
            let a899 = {
                let a897 = v1;
                let a898 = 8u64;
                nat_mul(a897, a898)?
            };
            let a900 = 4294967296u64;
            let a901 = 0u64;
            nat_quot(a899, a900, a901)
        }))
    } else {
        let a904 = {
            let a902 = v3;
            let a903 = 4u64;
            nat_add(a902, a903)?
        };
        let a905 = v2;
        if nat_eq(a904, a905) {
            Ok(f29({
                let a908 = {
                    let a906 = v1;
                    let a907 = 8u64;
                    nat_mul(a906, a907)?
                };
                let a909 = 4294967296u64;
                let a910 = 0u64;
                nat_rem(a908, a909, a910)
            }))
        } else {
            Ok(f30(f31(v0.clone(), v1, v3), f31(v0.clone(), v1, {
                let a911 = v3;
                let a912 = 1u64;
                nat_add(a911, a912)?
            }), f31(v0.clone(), v1, {
                let a913 = v3;
                let a914 = 2u64;
                nat_add(a913, a914)?
            }), f31(v0, v1, {
                let a915 = v3;
                let a916 = 3u64;
                nat_add(a915, a916)?
            })))
        }
    }
}

pub fn f26(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f32(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h917) => {
            h917
        }
    });
    let v2: u32 = f24(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h918, _, _) => {
            h918
        }
    }, f24(f33(match v0.clone() {
        Adt2::C0(_, _, _, _, h919, _, _, _, _, _) => {
            h919
        }
    }), f24(f34(match v0.clone() {
        Adt2::C0(_, _, _, _, h920, _, _, _, _, _) => {
            h920
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h921, _, _, _, _) => {
            h921
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h922, _, _, _) => {
            h922
        }
    }), f24(v1, {
        let m925 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h924, _) => {
                h924
            }
        };
        match m925 {
            Adt1::C0(h923, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h923
            }
        }
    }))));
    let v3: u32 = f24(f35(match v0.clone() {
        Adt2::C0(h926, _, _, _, _, _, _, _, _, _) => {
            h926
        }
    }), f36(match v0.clone() {
        Adt2::C0(h927, _, _, _, _, _, _, _, _, _) => {
            h927
        }
    }, match v0.clone() {
        Adt2::C0(_, h928, _, _, _, _, _, _, _, _) => {
            h928
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h929, _, _, _, _, _, _, _) => {
            h929
        }
    }));
    Ok(Adt2::C0(f24(v2, v3), match v0.clone() {
        Adt2::C0(h930, _, _, _, _, _, _, _, _, _) => {
            h930
        }
    }, match v0.clone() {
        Adt2::C0(_, h931, _, _, _, _, _, _, _, _) => {
            h931
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h932, _, _, _, _, _, _, _) => {
            h932
        }
    }, f24(match v0.clone() {
        Adt2::C0(_, _, _, h933, _, _, _, _, _, _) => {
            h933
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h934, _, _, _, _, _) => {
            h934
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h935, _, _, _, _) => {
            h935
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h936, _, _, _) => {
            h936
        }
    }, f37(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h937, _) => {
            h937
        }
    }), {
        let a939 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h938) => {
                h938
            }
        };
        let a940 = 1u64;
        nat_add(a939, a940)?
    }))
}

pub fn f27(v0: u32) -> u64 {
    let m942 = {
        let a941 = v0;
        fixed_u64::convert(i128::from(a941))
    };
    match m942 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f28(v0: u64) -> u32 {
    let m946 = {
        let a945 = {
            let a943 = v0;
            let a944 = 4294967295u64;
            fixed_u64::bit_and(a943, a944)
        };
        fixed_u32::convert(i128::from(a945))
    };
    match m946 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f29(v0: u64) -> u32 {
    let (_, h947) = {
        let (_, h948) = f38(Fn4::F39, 32u64, (v0, (1u32, 0u32)));
        h948
    };
    h947
}

pub fn f30(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a953 = f40(v0, 24u32);
    let a954 = {
        let a951 = f40(v1, 16u32);
        let a952 = {
            let a949 = f40(v2, 8u32);
            let a950 = v3;
            fixed_u32::bit_or(a949, a950)
        };
        fixed_u32::bit_or(a951, a952)
    };
    fixed_u32::bit_or(a953, a954)
}

pub fn f31(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a955 = v2;
    let a956 = v1;
    if nat_lt(a955, a956) {
        let m959 = {
            let a957 = v0;
            let a958 = v2;
            index_bytes(a957, a958)
        };
        match m959 {
            None => {
                0u32
            }
            Some(v3) => {
                let m961 = {
                    let a960 = v3;
                    fixed_u32::convert(i128::from(a960))
                };
                match m961 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a962 = v2;
        let a963 = v1;
        if nat_eq(a962, a963) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f32(v0: u64) -> u32 {
    let a964 = v0;
    let a965 = 32u64;
    if nat_lt(a964, a965) {
        let a966 = v0;
        let a967 = 16u64;
        if nat_lt(a966, a967) {
            let a968 = v0;
            let a969 = 8u64;
            if nat_lt(a968, a969) {
                let a970 = v0;
                let a971 = 4u64;
                if nat_lt(a970, a971) {
                    let a972 = v0;
                    let a973 = 2u64;
                    if nat_lt(a972, a973) {
                        let a974 = v0;
                        let a975 = 1u64;
                        if nat_lt(a974, a975) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a976 = v0;
                        let a977 = 3u64;
                        if nat_lt(a976, a977) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a978 = v0;
                    let a979 = 6u64;
                    if nat_lt(a978, a979) {
                        let a980 = v0;
                        let a981 = 5u64;
                        if nat_lt(a980, a981) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a982 = v0;
                        let a983 = 7u64;
                        if nat_lt(a982, a983) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a984 = v0;
                let a985 = 12u64;
                if nat_lt(a984, a985) {
                    let a986 = v0;
                    let a987 = 10u64;
                    if nat_lt(a986, a987) {
                        let a988 = v0;
                        let a989 = 9u64;
                        if nat_lt(a988, a989) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a990 = v0;
                        let a991 = 11u64;
                        if nat_lt(a990, a991) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a992 = v0;
                    let a993 = 14u64;
                    if nat_lt(a992, a993) {
                        let a994 = v0;
                        let a995 = 13u64;
                        if nat_lt(a994, a995) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a996 = v0;
                        let a997 = 15u64;
                        if nat_lt(a996, a997) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a998 = v0;
            let a999 = 24u64;
            if nat_lt(a998, a999) {
                let a1000 = v0;
                let a1001 = 20u64;
                if nat_lt(a1000, a1001) {
                    let a1002 = v0;
                    let a1003 = 18u64;
                    if nat_lt(a1002, a1003) {
                        let a1004 = v0;
                        let a1005 = 17u64;
                        if nat_lt(a1004, a1005) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1006 = v0;
                        let a1007 = 19u64;
                        if nat_lt(a1006, a1007) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1008 = v0;
                    let a1009 = 22u64;
                    if nat_lt(a1008, a1009) {
                        let a1010 = v0;
                        let a1011 = 21u64;
                        if nat_lt(a1010, a1011) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1012 = v0;
                        let a1013 = 23u64;
                        if nat_lt(a1012, a1013) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1014 = v0;
                let a1015 = 28u64;
                if nat_lt(a1014, a1015) {
                    let a1016 = v0;
                    let a1017 = 26u64;
                    if nat_lt(a1016, a1017) {
                        let a1018 = v0;
                        let a1019 = 25u64;
                        if nat_lt(a1018, a1019) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1020 = v0;
                        let a1021 = 27u64;
                        if nat_lt(a1020, a1021) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1022 = v0;
                    let a1023 = 30u64;
                    if nat_lt(a1022, a1023) {
                        let a1024 = v0;
                        let a1025 = 29u64;
                        if nat_lt(a1024, a1025) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1026 = v0;
                        let a1027 = 31u64;
                        if nat_lt(a1026, a1027) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1028 = v0;
        let a1029 = 48u64;
        if nat_lt(a1028, a1029) {
            let a1030 = v0;
            let a1031 = 40u64;
            if nat_lt(a1030, a1031) {
                let a1032 = v0;
                let a1033 = 36u64;
                if nat_lt(a1032, a1033) {
                    let a1034 = v0;
                    let a1035 = 34u64;
                    if nat_lt(a1034, a1035) {
                        let a1036 = v0;
                        let a1037 = 33u64;
                        if nat_lt(a1036, a1037) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1038 = v0;
                        let a1039 = 35u64;
                        if nat_lt(a1038, a1039) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1040 = v0;
                    let a1041 = 38u64;
                    if nat_lt(a1040, a1041) {
                        let a1042 = v0;
                        let a1043 = 37u64;
                        if nat_lt(a1042, a1043) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1044 = v0;
                        let a1045 = 39u64;
                        if nat_lt(a1044, a1045) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1046 = v0;
                let a1047 = 44u64;
                if nat_lt(a1046, a1047) {
                    let a1048 = v0;
                    let a1049 = 42u64;
                    if nat_lt(a1048, a1049) {
                        let a1050 = v0;
                        let a1051 = 41u64;
                        if nat_lt(a1050, a1051) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1052 = v0;
                        let a1053 = 43u64;
                        if nat_lt(a1052, a1053) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1054 = v0;
                    let a1055 = 46u64;
                    if nat_lt(a1054, a1055) {
                        let a1056 = v0;
                        let a1057 = 45u64;
                        if nat_lt(a1056, a1057) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1058 = v0;
                        let a1059 = 47u64;
                        if nat_lt(a1058, a1059) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1060 = v0;
            let a1061 = 56u64;
            if nat_lt(a1060, a1061) {
                let a1062 = v0;
                let a1063 = 52u64;
                if nat_lt(a1062, a1063) {
                    let a1064 = v0;
                    let a1065 = 50u64;
                    if nat_lt(a1064, a1065) {
                        let a1066 = v0;
                        let a1067 = 49u64;
                        if nat_lt(a1066, a1067) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1068 = v0;
                        let a1069 = 51u64;
                        if nat_lt(a1068, a1069) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1070 = v0;
                    let a1071 = 54u64;
                    if nat_lt(a1070, a1071) {
                        let a1072 = v0;
                        let a1073 = 53u64;
                        if nat_lt(a1072, a1073) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1074 = v0;
                        let a1075 = 55u64;
                        if nat_lt(a1074, a1075) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1076 = v0;
                let a1077 = 60u64;
                if nat_lt(a1076, a1077) {
                    let a1078 = v0;
                    let a1079 = 58u64;
                    if nat_lt(a1078, a1079) {
                        let a1080 = v0;
                        let a1081 = 57u64;
                        if nat_lt(a1080, a1081) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1082 = v0;
                        let a1083 = 59u64;
                        if nat_lt(a1082, a1083) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1084 = v0;
                    let a1085 = 62u64;
                    if nat_lt(a1084, a1085) {
                        let a1086 = v0;
                        let a1087 = 61u64;
                        if nat_lt(a1086, a1087) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1088 = v0;
                        let a1089 = 63u64;
                        if nat_lt(a1088, a1089) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f33(v0: u32) -> u32 {
    let a1092 = {
        let a1090 = f41(v0, 6u32, 26u32);
        let a1091 = f41(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1090, a1091)
    };
    let a1093 = f41(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1092, a1093)
}

pub fn f34(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1099 = {
        let a1094 = v0;
        let a1095 = v1;
        fixed_u32::bit_and(a1094, a1095)
    };
    let a1100 = {
        let a1097 = {
            let a1096 = v0;
            fixed_u32::bit_not(a1096)
        };
        let a1098 = v2;
        fixed_u32::bit_and(a1097, a1098)
    };
    fixed_u32::bit_xor(a1099, a1100)
}

pub fn f35(v0: u32) -> u32 {
    let a1103 = {
        let a1101 = f41(v0, 2u32, 30u32);
        let a1102 = f41(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1101, a1102)
    };
    let a1104 = f41(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1103, a1104)
}

pub fn f36(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1113 = {
        let a1109 = {
            let a1105 = v0;
            let a1106 = v1;
            fixed_u32::bit_and(a1105, a1106)
        };
        let a1110 = {
            let a1107 = v0;
            let a1108 = v2;
            fixed_u32::bit_and(a1107, a1108)
        };
        fixed_u32::bit_xor(a1109, a1110)
    };
    let a1114 = {
        let a1111 = v1;
        let a1112 = v2;
        fixed_u32::bit_and(a1111, a1112)
    };
    fixed_u32::bit_xor(a1113, a1114)
}

pub fn f37(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h1115, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1115
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1116, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1116
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1117, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1117
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1118, _, _, _, _, _, _, _, _, _, _, _) => {
            h1118
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1119, _, _, _, _, _, _, _, _, _, _) => {
            h1119
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h1120, _, _, _, _, _, _, _, _, _) => {
            h1120
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1121, _, _, _, _, _, _, _, _) => {
            h1121
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h1122, _, _, _, _, _, _, _) => {
            h1122
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h1123, _, _, _, _, _, _) => {
            h1123
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1124, _, _, _, _, _) => {
            h1124
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h1125, _, _, _, _) => {
            h1125
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1126, _, _, _) => {
            h1126
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1127, _, _) => {
            h1127
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1128, _) => {
            h1128
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1129) => {
            h1129
        }
    }, f24(f42(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1130, _) => {
            h1130
        }
    }), f24(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h1131, _, _, _, _, _, _) => {
            h1131
        }
    }, f24(f43(match v0.clone() {
        Adt1::C0(_, h1132, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1132
        }
    }), match v0 {
        Adt1::C0(h1133, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1133
        }
    }))))
}

pub fn f38(v0: Fn4, p1137: u64, p1138: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1136 = (p1137, p1138);
    {
        let mut m1136 = m1136;
        loop {
            let step = {
                let (v1, v2) = m1136;
                let m1134 = v1;
                if m1134 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1134 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1135: Fn4 = v0.clone();
                        c1135.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1139, _) = v0;
        h1139
    };
    let v2: u32 = {
        let (h1140, _) = {
            let (_, h1141) = v0;
            h1141
        };
        h1140
    };
    let v3: u32 = {
        let (_, h1142) = {
            let (_, h1143) = v0;
            h1143
        };
        h1142
    };
    ({
        let a1144 = v1;
        let a1145 = 2u64;
        let a1146 = 0u64;
        nat_quot(a1144, a1145, a1146)
    }, (f40(v2, 1u32), {
        let a1150 = {
            let a1147 = v1;
            let a1148 = 2u64;
            let a1149 = 0u64;
            nat_rem(a1147, a1148, a1149)
        };
        let a1151 = 1u64;
        if nat_eq(a1150, a1151) {
            let a1152 = v3;
            let a1153 = v2;
            fixed_u32::bit_or(a1152, a1153)
        } else {
            v3
        }
    }))
}

pub fn f40(v0: u32, v1: u32) -> u32 {
    let m1156 = {
        let a1154 = v0;
        let a1155 = v1;
        fixed_u32::shift_left(a1154, a1155)
    };
    match m1156 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f41(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1157 = f17(v0, v1);
    let a1158 = f40(v0, v2);
    fixed_u32::bit_or(a1157, a1158)
}

pub fn f42(v0: u32) -> u32 {
    let a1161 = {
        let a1159 = f41(v0, 17u32, 15u32);
        let a1160 = f41(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1159, a1160)
    };
    let a1162 = f17(v0, 10u32);
    fixed_u32::bit_xor(a1161, a1162)
}

pub fn f43(v0: u32) -> u32 {
    let a1165 = {
        let a1163 = f41(v0, 7u32, 25u32);
        let a1164 = f41(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1163, a1164)
    };
    let a1166 = f17(v0, 3u32);
    fixed_u32::bit_xor(a1165, a1166)
}
