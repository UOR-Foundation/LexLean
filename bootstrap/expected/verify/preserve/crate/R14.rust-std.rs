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
    F10(Bytes, u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn0::F10(k0, k1, k2) => f10(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F13,
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn1::F13 => f13(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn2::F21 => f21(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F37,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn3::F37 => f37(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    f1(f2(Str::lit("lexlean-pdf-recipe-v1")), f3(f3(f3(f3(f4(f2(Str::lit("tex")), v0), f4(f2(Str::lit("program")), v1)), f4(f2(Str::lit("version-output")), v2)), f4(f2(Str::lit("argv")), v3)), f4(f2(Str::lit("resources")), v4)))
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f5(f6(f3(f3(v0, Bytes::lit(&[0x00])), v1))?)
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
    f3(f3(f3(f7({
        let a4 = v0.clone();
        length_bytes(a4)
    }, 4u64), v0), f7({
        let a5 = v1.clone();
        length_bytes(a5)
    }, 8u64)), v1)
}

pub fn f5(v0: Adt0) -> R<Bytes> {
    Ok(f3(f3(f3(f3(f3(f3(f3(f8(match v0.clone() {
        Adt0::C0(h6, _, _, _, _, _, _, _) => {
            h6
        }
    })?, f8(match v0.clone() {
        Adt0::C0(_, h7, _, _, _, _, _, _) => {
            h7
        }
    })?), f8(match v0.clone() {
        Adt0::C0(_, _, h8, _, _, _, _, _) => {
            h8
        }
    })?), f8(match v0.clone() {
        Adt0::C0(_, _, _, h9, _, _, _, _) => {
            h9
        }
    })?), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, h10, _, _, _) => {
            h10
        }
    })?), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h11, _, _) => {
            h11
        }
    })?), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h12, _) => {
            h12
        }
    })?), f8(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h13) => {
            h13
        }
    })?))
}

pub fn f6(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a14 = v0.clone();
        length_bytes(a14)
    };
    let v2: u64 = {
        let a20 = {
            let a17 = {
                let a15 = v1;
                let a16 = 72u64;
                nat_add(a15, a16)?
            };
            let a18 = 64u64;
            let a19 = 0u64;
            nat_quot(a17, a18, a19)
        };
        let a21 = 64u64;
        nat_mul(a20, a21)?
    };
    let (_, h22) = f9(Fn0::F10(v0, v1, v2), {
        let a23 = v2;
        let a24 = 64u64;
        let a25 = 0u64;
        nat_quot(a23, a24, a25)
    }, (0u64, f11()))?;
    Ok(h22)
}

pub fn f7(v0: u64, v1: u64) -> Bytes {
    let (_, h26) = f12(Fn1::F13, v1, (v0, Bytes::lit(&[])));
    h26
}

pub fn f8(v0: u32) -> R<Bytes> {
    Ok(f3(f3(f3(f14({
        let v1: u32 = f15(v0, 24u32);
        let a215 = {
            let a119 = {
                let a71 = {
                    let a47 = {
                        let a35 = {
                            let a29 = {
                                let a27 = v1;
                                let a28 = 2147483648u32;
                                fixed_u32::bit_and(a27, a28)
                            };
                            let a30 = 2147483648u32;
                            if equal(a29, a30) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a36 = {
                            let a33 = {
                                let a31 = v1;
                                let a32 = 1073741824u32;
                                fixed_u32::bit_and(a31, a32)
                            };
                            let a34 = 1073741824u32;
                            if equal(a33, a34) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a35, a36)?
                    };
                    let a48 = {
                        let a45 = {
                            let a39 = {
                                let a37 = v1;
                                let a38 = 536870912u32;
                                fixed_u32::bit_and(a37, a38)
                            };
                            let a40 = 536870912u32;
                            if equal(a39, a40) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a46 = {
                            let a43 = {
                                let a41 = v1;
                                let a42 = 268435456u32;
                                fixed_u32::bit_and(a41, a42)
                            };
                            let a44 = 268435456u32;
                            if equal(a43, a44) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a45, a46)?
                    };
                    nat_add(a47, a48)?
                };
                let a72 = {
                    let a69 = {
                        let a57 = {
                            let a51 = {
                                let a49 = v1;
                                let a50 = 134217728u32;
                                fixed_u32::bit_and(a49, a50)
                            };
                            let a52 = 134217728u32;
                            if equal(a51, a52) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a58 = {
                            let a55 = {
                                let a53 = v1;
                                let a54 = 67108864u32;
                                fixed_u32::bit_and(a53, a54)
                            };
                            let a56 = 67108864u32;
                            if equal(a55, a56) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a57, a58)?
                    };
                    let a70 = {
                        let a67 = {
                            let a61 = {
                                let a59 = v1;
                                let a60 = 33554432u32;
                                fixed_u32::bit_and(a59, a60)
                            };
                            let a62 = 33554432u32;
                            if equal(a61, a62) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a68 = {
                            let a65 = {
                                let a63 = v1;
                                let a64 = 16777216u32;
                                fixed_u32::bit_and(a63, a64)
                            };
                            let a66 = 16777216u32;
                            if equal(a65, a66) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a67, a68)?
                    };
                    nat_add(a69, a70)?
                };
                nat_add(a71, a72)?
            };
            let a120 = {
                let a117 = {
                    let a93 = {
                        let a81 = {
                            let a75 = {
                                let a73 = v1;
                                let a74 = 8388608u32;
                                fixed_u32::bit_and(a73, a74)
                            };
                            let a76 = 8388608u32;
                            if equal(a75, a76) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a82 = {
                            let a79 = {
                                let a77 = v1;
                                let a78 = 4194304u32;
                                fixed_u32::bit_and(a77, a78)
                            };
                            let a80 = 4194304u32;
                            if equal(a79, a80) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a81, a82)?
                    };
                    let a94 = {
                        let a91 = {
                            let a85 = {
                                let a83 = v1;
                                let a84 = 2097152u32;
                                fixed_u32::bit_and(a83, a84)
                            };
                            let a86 = 2097152u32;
                            if equal(a85, a86) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a92 = {
                            let a89 = {
                                let a87 = v1;
                                let a88 = 1048576u32;
                                fixed_u32::bit_and(a87, a88)
                            };
                            let a90 = 1048576u32;
                            if equal(a89, a90) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a91, a92)?
                    };
                    nat_add(a93, a94)?
                };
                let a118 = {
                    let a115 = {
                        let a103 = {
                            let a97 = {
                                let a95 = v1;
                                let a96 = 524288u32;
                                fixed_u32::bit_and(a95, a96)
                            };
                            let a98 = 524288u32;
                            if equal(a97, a98) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a104 = {
                            let a101 = {
                                let a99 = v1;
                                let a100 = 262144u32;
                                fixed_u32::bit_and(a99, a100)
                            };
                            let a102 = 262144u32;
                            if equal(a101, a102) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a103, a104)?
                    };
                    let a116 = {
                        let a113 = {
                            let a107 = {
                                let a105 = v1;
                                let a106 = 131072u32;
                                fixed_u32::bit_and(a105, a106)
                            };
                            let a108 = 131072u32;
                            if equal(a107, a108) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a114 = {
                            let a111 = {
                                let a109 = v1;
                                let a110 = 65536u32;
                                fixed_u32::bit_and(a109, a110)
                            };
                            let a112 = 65536u32;
                            if equal(a111, a112) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a113, a114)?
                    };
                    nat_add(a115, a116)?
                };
                nat_add(a117, a118)?
            };
            nat_add(a119, a120)?
        };
        let a216 = {
            let a213 = {
                let a165 = {
                    let a141 = {
                        let a129 = {
                            let a123 = {
                                let a121 = v1;
                                let a122 = 32768u32;
                                fixed_u32::bit_and(a121, a122)
                            };
                            let a124 = 32768u32;
                            if equal(a123, a124) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a130 = {
                            let a127 = {
                                let a125 = v1;
                                let a126 = 16384u32;
                                fixed_u32::bit_and(a125, a126)
                            };
                            let a128 = 16384u32;
                            if equal(a127, a128) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a129, a130)?
                    };
                    let a142 = {
                        let a139 = {
                            let a133 = {
                                let a131 = v1;
                                let a132 = 8192u32;
                                fixed_u32::bit_and(a131, a132)
                            };
                            let a134 = 8192u32;
                            if equal(a133, a134) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a140 = {
                            let a137 = {
                                let a135 = v1;
                                let a136 = 4096u32;
                                fixed_u32::bit_and(a135, a136)
                            };
                            let a138 = 4096u32;
                            if equal(a137, a138) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a139, a140)?
                    };
                    nat_add(a141, a142)?
                };
                let a166 = {
                    let a163 = {
                        let a151 = {
                            let a145 = {
                                let a143 = v1;
                                let a144 = 2048u32;
                                fixed_u32::bit_and(a143, a144)
                            };
                            let a146 = 2048u32;
                            if equal(a145, a146) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a152 = {
                            let a149 = {
                                let a147 = v1;
                                let a148 = 1024u32;
                                fixed_u32::bit_and(a147, a148)
                            };
                            let a150 = 1024u32;
                            if equal(a149, a150) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a151, a152)?
                    };
                    let a164 = {
                        let a161 = {
                            let a155 = {
                                let a153 = v1;
                                let a154 = 512u32;
                                fixed_u32::bit_and(a153, a154)
                            };
                            let a156 = 512u32;
                            if equal(a155, a156) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a162 = {
                            let a159 = {
                                let a157 = v1;
                                let a158 = 256u32;
                                fixed_u32::bit_and(a157, a158)
                            };
                            let a160 = 256u32;
                            if equal(a159, a160) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a161, a162)?
                    };
                    nat_add(a163, a164)?
                };
                nat_add(a165, a166)?
            };
            let a214 = {
                let a211 = {
                    let a187 = {
                        let a175 = {
                            let a169 = {
                                let a167 = v1;
                                let a168 = 128u32;
                                fixed_u32::bit_and(a167, a168)
                            };
                            let a170 = 128u32;
                            if equal(a169, a170) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a176 = {
                            let a173 = {
                                let a171 = v1;
                                let a172 = 64u32;
                                fixed_u32::bit_and(a171, a172)
                            };
                            let a174 = 64u32;
                            if equal(a173, a174) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a175, a176)?
                    };
                    let a188 = {
                        let a185 = {
                            let a179 = {
                                let a177 = v1;
                                let a178 = 32u32;
                                fixed_u32::bit_and(a177, a178)
                            };
                            let a180 = 32u32;
                            if equal(a179, a180) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a186 = {
                            let a183 = {
                                let a181 = v1;
                                let a182 = 16u32;
                                fixed_u32::bit_and(a181, a182)
                            };
                            let a184 = 16u32;
                            if equal(a183, a184) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a185, a186)?
                    };
                    nat_add(a187, a188)?
                };
                let a212 = {
                    let a209 = {
                        let a197 = {
                            let a191 = {
                                let a189 = v1;
                                let a190 = 8u32;
                                fixed_u32::bit_and(a189, a190)
                            };
                            let a192 = 8u32;
                            if equal(a191, a192) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a198 = {
                            let a195 = {
                                let a193 = v1;
                                let a194 = 4u32;
                                fixed_u32::bit_and(a193, a194)
                            };
                            let a196 = 4u32;
                            if equal(a195, a196) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a197, a198)?
                    };
                    let a210 = {
                        let a207 = {
                            let a201 = {
                                let a199 = v1;
                                let a200 = 2u32;
                                fixed_u32::bit_and(a199, a200)
                            };
                            let a202 = 2u32;
                            if equal(a201, a202) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a208 = {
                            let a205 = {
                                let a203 = v1;
                                let a204 = 1u32;
                                fixed_u32::bit_and(a203, a204)
                            };
                            let a206 = 1u32;
                            if equal(a205, a206) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a207, a208)?
                    };
                    nat_add(a209, a210)?
                };
                nat_add(a211, a212)?
            };
            nat_add(a213, a214)?
        };
        nat_add(a215, a216)?
    }), f14({
        let v2: u32 = {
            let a217 = f15(v0, 16u32);
            let a218 = 255u32;
            fixed_u32::bit_and(a217, a218)
        };
        let a407 = {
            let a311 = {
                let a263 = {
                    let a239 = {
                        let a227 = {
                            let a221 = {
                                let a219 = v2;
                                let a220 = 2147483648u32;
                                fixed_u32::bit_and(a219, a220)
                            };
                            let a222 = 2147483648u32;
                            if equal(a221, a222) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a228 = {
                            let a225 = {
                                let a223 = v2;
                                let a224 = 1073741824u32;
                                fixed_u32::bit_and(a223, a224)
                            };
                            let a226 = 1073741824u32;
                            if equal(a225, a226) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a227, a228)?
                    };
                    let a240 = {
                        let a237 = {
                            let a231 = {
                                let a229 = v2;
                                let a230 = 536870912u32;
                                fixed_u32::bit_and(a229, a230)
                            };
                            let a232 = 536870912u32;
                            if equal(a231, a232) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a238 = {
                            let a235 = {
                                let a233 = v2;
                                let a234 = 268435456u32;
                                fixed_u32::bit_and(a233, a234)
                            };
                            let a236 = 268435456u32;
                            if equal(a235, a236) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a237, a238)?
                    };
                    nat_add(a239, a240)?
                };
                let a264 = {
                    let a261 = {
                        let a249 = {
                            let a243 = {
                                let a241 = v2;
                                let a242 = 134217728u32;
                                fixed_u32::bit_and(a241, a242)
                            };
                            let a244 = 134217728u32;
                            if equal(a243, a244) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a250 = {
                            let a247 = {
                                let a245 = v2;
                                let a246 = 67108864u32;
                                fixed_u32::bit_and(a245, a246)
                            };
                            let a248 = 67108864u32;
                            if equal(a247, a248) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a249, a250)?
                    };
                    let a262 = {
                        let a259 = {
                            let a253 = {
                                let a251 = v2;
                                let a252 = 33554432u32;
                                fixed_u32::bit_and(a251, a252)
                            };
                            let a254 = 33554432u32;
                            if equal(a253, a254) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a260 = {
                            let a257 = {
                                let a255 = v2;
                                let a256 = 16777216u32;
                                fixed_u32::bit_and(a255, a256)
                            };
                            let a258 = 16777216u32;
                            if equal(a257, a258) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a259, a260)?
                    };
                    nat_add(a261, a262)?
                };
                nat_add(a263, a264)?
            };
            let a312 = {
                let a309 = {
                    let a285 = {
                        let a273 = {
                            let a267 = {
                                let a265 = v2;
                                let a266 = 8388608u32;
                                fixed_u32::bit_and(a265, a266)
                            };
                            let a268 = 8388608u32;
                            if equal(a267, a268) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a274 = {
                            let a271 = {
                                let a269 = v2;
                                let a270 = 4194304u32;
                                fixed_u32::bit_and(a269, a270)
                            };
                            let a272 = 4194304u32;
                            if equal(a271, a272) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a273, a274)?
                    };
                    let a286 = {
                        let a283 = {
                            let a277 = {
                                let a275 = v2;
                                let a276 = 2097152u32;
                                fixed_u32::bit_and(a275, a276)
                            };
                            let a278 = 2097152u32;
                            if equal(a277, a278) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a284 = {
                            let a281 = {
                                let a279 = v2;
                                let a280 = 1048576u32;
                                fixed_u32::bit_and(a279, a280)
                            };
                            let a282 = 1048576u32;
                            if equal(a281, a282) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a283, a284)?
                    };
                    nat_add(a285, a286)?
                };
                let a310 = {
                    let a307 = {
                        let a295 = {
                            let a289 = {
                                let a287 = v2;
                                let a288 = 524288u32;
                                fixed_u32::bit_and(a287, a288)
                            };
                            let a290 = 524288u32;
                            if equal(a289, a290) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a296 = {
                            let a293 = {
                                let a291 = v2;
                                let a292 = 262144u32;
                                fixed_u32::bit_and(a291, a292)
                            };
                            let a294 = 262144u32;
                            if equal(a293, a294) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a295, a296)?
                    };
                    let a308 = {
                        let a305 = {
                            let a299 = {
                                let a297 = v2;
                                let a298 = 131072u32;
                                fixed_u32::bit_and(a297, a298)
                            };
                            let a300 = 131072u32;
                            if equal(a299, a300) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a306 = {
                            let a303 = {
                                let a301 = v2;
                                let a302 = 65536u32;
                                fixed_u32::bit_and(a301, a302)
                            };
                            let a304 = 65536u32;
                            if equal(a303, a304) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a305, a306)?
                    };
                    nat_add(a307, a308)?
                };
                nat_add(a309, a310)?
            };
            nat_add(a311, a312)?
        };
        let a408 = {
            let a405 = {
                let a357 = {
                    let a333 = {
                        let a321 = {
                            let a315 = {
                                let a313 = v2;
                                let a314 = 32768u32;
                                fixed_u32::bit_and(a313, a314)
                            };
                            let a316 = 32768u32;
                            if equal(a315, a316) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a322 = {
                            let a319 = {
                                let a317 = v2;
                                let a318 = 16384u32;
                                fixed_u32::bit_and(a317, a318)
                            };
                            let a320 = 16384u32;
                            if equal(a319, a320) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a321, a322)?
                    };
                    let a334 = {
                        let a331 = {
                            let a325 = {
                                let a323 = v2;
                                let a324 = 8192u32;
                                fixed_u32::bit_and(a323, a324)
                            };
                            let a326 = 8192u32;
                            if equal(a325, a326) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a332 = {
                            let a329 = {
                                let a327 = v2;
                                let a328 = 4096u32;
                                fixed_u32::bit_and(a327, a328)
                            };
                            let a330 = 4096u32;
                            if equal(a329, a330) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a331, a332)?
                    };
                    nat_add(a333, a334)?
                };
                let a358 = {
                    let a355 = {
                        let a343 = {
                            let a337 = {
                                let a335 = v2;
                                let a336 = 2048u32;
                                fixed_u32::bit_and(a335, a336)
                            };
                            let a338 = 2048u32;
                            if equal(a337, a338) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a344 = {
                            let a341 = {
                                let a339 = v2;
                                let a340 = 1024u32;
                                fixed_u32::bit_and(a339, a340)
                            };
                            let a342 = 1024u32;
                            if equal(a341, a342) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a343, a344)?
                    };
                    let a356 = {
                        let a353 = {
                            let a347 = {
                                let a345 = v2;
                                let a346 = 512u32;
                                fixed_u32::bit_and(a345, a346)
                            };
                            let a348 = 512u32;
                            if equal(a347, a348) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a354 = {
                            let a351 = {
                                let a349 = v2;
                                let a350 = 256u32;
                                fixed_u32::bit_and(a349, a350)
                            };
                            let a352 = 256u32;
                            if equal(a351, a352) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a353, a354)?
                    };
                    nat_add(a355, a356)?
                };
                nat_add(a357, a358)?
            };
            let a406 = {
                let a403 = {
                    let a379 = {
                        let a367 = {
                            let a361 = {
                                let a359 = v2;
                                let a360 = 128u32;
                                fixed_u32::bit_and(a359, a360)
                            };
                            let a362 = 128u32;
                            if equal(a361, a362) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a368 = {
                            let a365 = {
                                let a363 = v2;
                                let a364 = 64u32;
                                fixed_u32::bit_and(a363, a364)
                            };
                            let a366 = 64u32;
                            if equal(a365, a366) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a367, a368)?
                    };
                    let a380 = {
                        let a377 = {
                            let a371 = {
                                let a369 = v2;
                                let a370 = 32u32;
                                fixed_u32::bit_and(a369, a370)
                            };
                            let a372 = 32u32;
                            if equal(a371, a372) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a378 = {
                            let a375 = {
                                let a373 = v2;
                                let a374 = 16u32;
                                fixed_u32::bit_and(a373, a374)
                            };
                            let a376 = 16u32;
                            if equal(a375, a376) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a377, a378)?
                    };
                    nat_add(a379, a380)?
                };
                let a404 = {
                    let a401 = {
                        let a389 = {
                            let a383 = {
                                let a381 = v2;
                                let a382 = 8u32;
                                fixed_u32::bit_and(a381, a382)
                            };
                            let a384 = 8u32;
                            if equal(a383, a384) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a390 = {
                            let a387 = {
                                let a385 = v2;
                                let a386 = 4u32;
                                fixed_u32::bit_and(a385, a386)
                            };
                            let a388 = 4u32;
                            if equal(a387, a388) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a389, a390)?
                    };
                    let a402 = {
                        let a399 = {
                            let a393 = {
                                let a391 = v2;
                                let a392 = 2u32;
                                fixed_u32::bit_and(a391, a392)
                            };
                            let a394 = 2u32;
                            if equal(a393, a394) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a400 = {
                            let a397 = {
                                let a395 = v2;
                                let a396 = 1u32;
                                fixed_u32::bit_and(a395, a396)
                            };
                            let a398 = 1u32;
                            if equal(a397, a398) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a399, a400)?
                    };
                    nat_add(a401, a402)?
                };
                nat_add(a403, a404)?
            };
            nat_add(a405, a406)?
        };
        nat_add(a407, a408)?
    })), f14({
        let v3: u32 = {
            let a409 = f15(v0, 8u32);
            let a410 = 255u32;
            fixed_u32::bit_and(a409, a410)
        };
        let a599 = {
            let a503 = {
                let a455 = {
                    let a431 = {
                        let a419 = {
                            let a413 = {
                                let a411 = v3;
                                let a412 = 2147483648u32;
                                fixed_u32::bit_and(a411, a412)
                            };
                            let a414 = 2147483648u32;
                            if equal(a413, a414) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a420 = {
                            let a417 = {
                                let a415 = v3;
                                let a416 = 1073741824u32;
                                fixed_u32::bit_and(a415, a416)
                            };
                            let a418 = 1073741824u32;
                            if equal(a417, a418) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a419, a420)?
                    };
                    let a432 = {
                        let a429 = {
                            let a423 = {
                                let a421 = v3;
                                let a422 = 536870912u32;
                                fixed_u32::bit_and(a421, a422)
                            };
                            let a424 = 536870912u32;
                            if equal(a423, a424) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a430 = {
                            let a427 = {
                                let a425 = v3;
                                let a426 = 268435456u32;
                                fixed_u32::bit_and(a425, a426)
                            };
                            let a428 = 268435456u32;
                            if equal(a427, a428) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a429, a430)?
                    };
                    nat_add(a431, a432)?
                };
                let a456 = {
                    let a453 = {
                        let a441 = {
                            let a435 = {
                                let a433 = v3;
                                let a434 = 134217728u32;
                                fixed_u32::bit_and(a433, a434)
                            };
                            let a436 = 134217728u32;
                            if equal(a435, a436) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a442 = {
                            let a439 = {
                                let a437 = v3;
                                let a438 = 67108864u32;
                                fixed_u32::bit_and(a437, a438)
                            };
                            let a440 = 67108864u32;
                            if equal(a439, a440) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a441, a442)?
                    };
                    let a454 = {
                        let a451 = {
                            let a445 = {
                                let a443 = v3;
                                let a444 = 33554432u32;
                                fixed_u32::bit_and(a443, a444)
                            };
                            let a446 = 33554432u32;
                            if equal(a445, a446) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a452 = {
                            let a449 = {
                                let a447 = v3;
                                let a448 = 16777216u32;
                                fixed_u32::bit_and(a447, a448)
                            };
                            let a450 = 16777216u32;
                            if equal(a449, a450) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a451, a452)?
                    };
                    nat_add(a453, a454)?
                };
                nat_add(a455, a456)?
            };
            let a504 = {
                let a501 = {
                    let a477 = {
                        let a465 = {
                            let a459 = {
                                let a457 = v3;
                                let a458 = 8388608u32;
                                fixed_u32::bit_and(a457, a458)
                            };
                            let a460 = 8388608u32;
                            if equal(a459, a460) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a466 = {
                            let a463 = {
                                let a461 = v3;
                                let a462 = 4194304u32;
                                fixed_u32::bit_and(a461, a462)
                            };
                            let a464 = 4194304u32;
                            if equal(a463, a464) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a465, a466)?
                    };
                    let a478 = {
                        let a475 = {
                            let a469 = {
                                let a467 = v3;
                                let a468 = 2097152u32;
                                fixed_u32::bit_and(a467, a468)
                            };
                            let a470 = 2097152u32;
                            if equal(a469, a470) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a476 = {
                            let a473 = {
                                let a471 = v3;
                                let a472 = 1048576u32;
                                fixed_u32::bit_and(a471, a472)
                            };
                            let a474 = 1048576u32;
                            if equal(a473, a474) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a475, a476)?
                    };
                    nat_add(a477, a478)?
                };
                let a502 = {
                    let a499 = {
                        let a487 = {
                            let a481 = {
                                let a479 = v3;
                                let a480 = 524288u32;
                                fixed_u32::bit_and(a479, a480)
                            };
                            let a482 = 524288u32;
                            if equal(a481, a482) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a488 = {
                            let a485 = {
                                let a483 = v3;
                                let a484 = 262144u32;
                                fixed_u32::bit_and(a483, a484)
                            };
                            let a486 = 262144u32;
                            if equal(a485, a486) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a487, a488)?
                    };
                    let a500 = {
                        let a497 = {
                            let a491 = {
                                let a489 = v3;
                                let a490 = 131072u32;
                                fixed_u32::bit_and(a489, a490)
                            };
                            let a492 = 131072u32;
                            if equal(a491, a492) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a498 = {
                            let a495 = {
                                let a493 = v3;
                                let a494 = 65536u32;
                                fixed_u32::bit_and(a493, a494)
                            };
                            let a496 = 65536u32;
                            if equal(a495, a496) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a497, a498)?
                    };
                    nat_add(a499, a500)?
                };
                nat_add(a501, a502)?
            };
            nat_add(a503, a504)?
        };
        let a600 = {
            let a597 = {
                let a549 = {
                    let a525 = {
                        let a513 = {
                            let a507 = {
                                let a505 = v3;
                                let a506 = 32768u32;
                                fixed_u32::bit_and(a505, a506)
                            };
                            let a508 = 32768u32;
                            if equal(a507, a508) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a514 = {
                            let a511 = {
                                let a509 = v3;
                                let a510 = 16384u32;
                                fixed_u32::bit_and(a509, a510)
                            };
                            let a512 = 16384u32;
                            if equal(a511, a512) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a513, a514)?
                    };
                    let a526 = {
                        let a523 = {
                            let a517 = {
                                let a515 = v3;
                                let a516 = 8192u32;
                                fixed_u32::bit_and(a515, a516)
                            };
                            let a518 = 8192u32;
                            if equal(a517, a518) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a524 = {
                            let a521 = {
                                let a519 = v3;
                                let a520 = 4096u32;
                                fixed_u32::bit_and(a519, a520)
                            };
                            let a522 = 4096u32;
                            if equal(a521, a522) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a523, a524)?
                    };
                    nat_add(a525, a526)?
                };
                let a550 = {
                    let a547 = {
                        let a535 = {
                            let a529 = {
                                let a527 = v3;
                                let a528 = 2048u32;
                                fixed_u32::bit_and(a527, a528)
                            };
                            let a530 = 2048u32;
                            if equal(a529, a530) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a536 = {
                            let a533 = {
                                let a531 = v3;
                                let a532 = 1024u32;
                                fixed_u32::bit_and(a531, a532)
                            };
                            let a534 = 1024u32;
                            if equal(a533, a534) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a535, a536)?
                    };
                    let a548 = {
                        let a545 = {
                            let a539 = {
                                let a537 = v3;
                                let a538 = 512u32;
                                fixed_u32::bit_and(a537, a538)
                            };
                            let a540 = 512u32;
                            if equal(a539, a540) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a546 = {
                            let a543 = {
                                let a541 = v3;
                                let a542 = 256u32;
                                fixed_u32::bit_and(a541, a542)
                            };
                            let a544 = 256u32;
                            if equal(a543, a544) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a545, a546)?
                    };
                    nat_add(a547, a548)?
                };
                nat_add(a549, a550)?
            };
            let a598 = {
                let a595 = {
                    let a571 = {
                        let a559 = {
                            let a553 = {
                                let a551 = v3;
                                let a552 = 128u32;
                                fixed_u32::bit_and(a551, a552)
                            };
                            let a554 = 128u32;
                            if equal(a553, a554) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a560 = {
                            let a557 = {
                                let a555 = v3;
                                let a556 = 64u32;
                                fixed_u32::bit_and(a555, a556)
                            };
                            let a558 = 64u32;
                            if equal(a557, a558) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a559, a560)?
                    };
                    let a572 = {
                        let a569 = {
                            let a563 = {
                                let a561 = v3;
                                let a562 = 32u32;
                                fixed_u32::bit_and(a561, a562)
                            };
                            let a564 = 32u32;
                            if equal(a563, a564) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a570 = {
                            let a567 = {
                                let a565 = v3;
                                let a566 = 16u32;
                                fixed_u32::bit_and(a565, a566)
                            };
                            let a568 = 16u32;
                            if equal(a567, a568) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a569, a570)?
                    };
                    nat_add(a571, a572)?
                };
                let a596 = {
                    let a593 = {
                        let a581 = {
                            let a575 = {
                                let a573 = v3;
                                let a574 = 8u32;
                                fixed_u32::bit_and(a573, a574)
                            };
                            let a576 = 8u32;
                            if equal(a575, a576) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a582 = {
                            let a579 = {
                                let a577 = v3;
                                let a578 = 4u32;
                                fixed_u32::bit_and(a577, a578)
                            };
                            let a580 = 4u32;
                            if equal(a579, a580) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a581, a582)?
                    };
                    let a594 = {
                        let a591 = {
                            let a585 = {
                                let a583 = v3;
                                let a584 = 2u32;
                                fixed_u32::bit_and(a583, a584)
                            };
                            let a586 = 2u32;
                            if equal(a585, a586) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a592 = {
                            let a589 = {
                                let a587 = v3;
                                let a588 = 1u32;
                                fixed_u32::bit_and(a587, a588)
                            };
                            let a590 = 1u32;
                            if equal(a589, a590) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a591, a592)?
                    };
                    nat_add(a593, a594)?
                };
                nat_add(a595, a596)?
            };
            nat_add(a597, a598)?
        };
        nat_add(a599, a600)?
    })), f14({
        let v4: u32 = {
            let a601 = v0;
            let a602 = 255u32;
            fixed_u32::bit_and(a601, a602)
        };
        let a791 = {
            let a695 = {
                let a647 = {
                    let a623 = {
                        let a611 = {
                            let a605 = {
                                let a603 = v4;
                                let a604 = 2147483648u32;
                                fixed_u32::bit_and(a603, a604)
                            };
                            let a606 = 2147483648u32;
                            if equal(a605, a606) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a612 = {
                            let a609 = {
                                let a607 = v4;
                                let a608 = 1073741824u32;
                                fixed_u32::bit_and(a607, a608)
                            };
                            let a610 = 1073741824u32;
                            if equal(a609, a610) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a611, a612)?
                    };
                    let a624 = {
                        let a621 = {
                            let a615 = {
                                let a613 = v4;
                                let a614 = 536870912u32;
                                fixed_u32::bit_and(a613, a614)
                            };
                            let a616 = 536870912u32;
                            if equal(a615, a616) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a622 = {
                            let a619 = {
                                let a617 = v4;
                                let a618 = 268435456u32;
                                fixed_u32::bit_and(a617, a618)
                            };
                            let a620 = 268435456u32;
                            if equal(a619, a620) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a621, a622)?
                    };
                    nat_add(a623, a624)?
                };
                let a648 = {
                    let a645 = {
                        let a633 = {
                            let a627 = {
                                let a625 = v4;
                                let a626 = 134217728u32;
                                fixed_u32::bit_and(a625, a626)
                            };
                            let a628 = 134217728u32;
                            if equal(a627, a628) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a634 = {
                            let a631 = {
                                let a629 = v4;
                                let a630 = 67108864u32;
                                fixed_u32::bit_and(a629, a630)
                            };
                            let a632 = 67108864u32;
                            if equal(a631, a632) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a633, a634)?
                    };
                    let a646 = {
                        let a643 = {
                            let a637 = {
                                let a635 = v4;
                                let a636 = 33554432u32;
                                fixed_u32::bit_and(a635, a636)
                            };
                            let a638 = 33554432u32;
                            if equal(a637, a638) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a644 = {
                            let a641 = {
                                let a639 = v4;
                                let a640 = 16777216u32;
                                fixed_u32::bit_and(a639, a640)
                            };
                            let a642 = 16777216u32;
                            if equal(a641, a642) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a643, a644)?
                    };
                    nat_add(a645, a646)?
                };
                nat_add(a647, a648)?
            };
            let a696 = {
                let a693 = {
                    let a669 = {
                        let a657 = {
                            let a651 = {
                                let a649 = v4;
                                let a650 = 8388608u32;
                                fixed_u32::bit_and(a649, a650)
                            };
                            let a652 = 8388608u32;
                            if equal(a651, a652) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a658 = {
                            let a655 = {
                                let a653 = v4;
                                let a654 = 4194304u32;
                                fixed_u32::bit_and(a653, a654)
                            };
                            let a656 = 4194304u32;
                            if equal(a655, a656) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a657, a658)?
                    };
                    let a670 = {
                        let a667 = {
                            let a661 = {
                                let a659 = v4;
                                let a660 = 2097152u32;
                                fixed_u32::bit_and(a659, a660)
                            };
                            let a662 = 2097152u32;
                            if equal(a661, a662) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a668 = {
                            let a665 = {
                                let a663 = v4;
                                let a664 = 1048576u32;
                                fixed_u32::bit_and(a663, a664)
                            };
                            let a666 = 1048576u32;
                            if equal(a665, a666) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a667, a668)?
                    };
                    nat_add(a669, a670)?
                };
                let a694 = {
                    let a691 = {
                        let a679 = {
                            let a673 = {
                                let a671 = v4;
                                let a672 = 524288u32;
                                fixed_u32::bit_and(a671, a672)
                            };
                            let a674 = 524288u32;
                            if equal(a673, a674) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a680 = {
                            let a677 = {
                                let a675 = v4;
                                let a676 = 262144u32;
                                fixed_u32::bit_and(a675, a676)
                            };
                            let a678 = 262144u32;
                            if equal(a677, a678) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a679, a680)?
                    };
                    let a692 = {
                        let a689 = {
                            let a683 = {
                                let a681 = v4;
                                let a682 = 131072u32;
                                fixed_u32::bit_and(a681, a682)
                            };
                            let a684 = 131072u32;
                            if equal(a683, a684) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a690 = {
                            let a687 = {
                                let a685 = v4;
                                let a686 = 65536u32;
                                fixed_u32::bit_and(a685, a686)
                            };
                            let a688 = 65536u32;
                            if equal(a687, a688) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a689, a690)?
                    };
                    nat_add(a691, a692)?
                };
                nat_add(a693, a694)?
            };
            nat_add(a695, a696)?
        };
        let a792 = {
            let a789 = {
                let a741 = {
                    let a717 = {
                        let a705 = {
                            let a699 = {
                                let a697 = v4;
                                let a698 = 32768u32;
                                fixed_u32::bit_and(a697, a698)
                            };
                            let a700 = 32768u32;
                            if equal(a699, a700) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a706 = {
                            let a703 = {
                                let a701 = v4;
                                let a702 = 16384u32;
                                fixed_u32::bit_and(a701, a702)
                            };
                            let a704 = 16384u32;
                            if equal(a703, a704) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a705, a706)?
                    };
                    let a718 = {
                        let a715 = {
                            let a709 = {
                                let a707 = v4;
                                let a708 = 8192u32;
                                fixed_u32::bit_and(a707, a708)
                            };
                            let a710 = 8192u32;
                            if equal(a709, a710) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a716 = {
                            let a713 = {
                                let a711 = v4;
                                let a712 = 4096u32;
                                fixed_u32::bit_and(a711, a712)
                            };
                            let a714 = 4096u32;
                            if equal(a713, a714) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a715, a716)?
                    };
                    nat_add(a717, a718)?
                };
                let a742 = {
                    let a739 = {
                        let a727 = {
                            let a721 = {
                                let a719 = v4;
                                let a720 = 2048u32;
                                fixed_u32::bit_and(a719, a720)
                            };
                            let a722 = 2048u32;
                            if equal(a721, a722) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a728 = {
                            let a725 = {
                                let a723 = v4;
                                let a724 = 1024u32;
                                fixed_u32::bit_and(a723, a724)
                            };
                            let a726 = 1024u32;
                            if equal(a725, a726) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a727, a728)?
                    };
                    let a740 = {
                        let a737 = {
                            let a731 = {
                                let a729 = v4;
                                let a730 = 512u32;
                                fixed_u32::bit_and(a729, a730)
                            };
                            let a732 = 512u32;
                            if equal(a731, a732) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a738 = {
                            let a735 = {
                                let a733 = v4;
                                let a734 = 256u32;
                                fixed_u32::bit_and(a733, a734)
                            };
                            let a736 = 256u32;
                            if equal(a735, a736) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a737, a738)?
                    };
                    nat_add(a739, a740)?
                };
                nat_add(a741, a742)?
            };
            let a790 = {
                let a787 = {
                    let a763 = {
                        let a751 = {
                            let a745 = {
                                let a743 = v4;
                                let a744 = 128u32;
                                fixed_u32::bit_and(a743, a744)
                            };
                            let a746 = 128u32;
                            if equal(a745, a746) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a752 = {
                            let a749 = {
                                let a747 = v4;
                                let a748 = 64u32;
                                fixed_u32::bit_and(a747, a748)
                            };
                            let a750 = 64u32;
                            if equal(a749, a750) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a751, a752)?
                    };
                    let a764 = {
                        let a761 = {
                            let a755 = {
                                let a753 = v4;
                                let a754 = 32u32;
                                fixed_u32::bit_and(a753, a754)
                            };
                            let a756 = 32u32;
                            if equal(a755, a756) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a762 = {
                            let a759 = {
                                let a757 = v4;
                                let a758 = 16u32;
                                fixed_u32::bit_and(a757, a758)
                            };
                            let a760 = 16u32;
                            if equal(a759, a760) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a761, a762)?
                    };
                    nat_add(a763, a764)?
                };
                let a788 = {
                    let a785 = {
                        let a773 = {
                            let a767 = {
                                let a765 = v4;
                                let a766 = 8u32;
                                fixed_u32::bit_and(a765, a766)
                            };
                            let a768 = 8u32;
                            if equal(a767, a768) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a774 = {
                            let a771 = {
                                let a769 = v4;
                                let a770 = 4u32;
                                fixed_u32::bit_and(a769, a770)
                            };
                            let a772 = 4u32;
                            if equal(a771, a772) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a773, a774)?
                    };
                    let a786 = {
                        let a783 = {
                            let a777 = {
                                let a775 = v4;
                                let a776 = 2u32;
                                fixed_u32::bit_and(a775, a776)
                            };
                            let a778 = 2u32;
                            if equal(a777, a778) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a784 = {
                            let a781 = {
                                let a779 = v4;
                                let a780 = 1u32;
                                fixed_u32::bit_and(a779, a780)
                            };
                            let a782 = 1u32;
                            if equal(a781, a782) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a783, a784)?
                    };
                    nat_add(a785, a786)?
                };
                nat_add(a787, a788)?
            };
            nat_add(a789, a790)?
        };
        nat_add(a791, a792)?
    })))
}

pub fn f9(v0: Fn0, p796: u64, p797: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m795 = (p796, p797);
    {
        let mut m795 = m795;
        loop {
            let step = {
                let (v1, v2) = m795;
                let m793 = v1;
                if m793 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m793 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c794: Fn0 = v0.clone();
                        c794.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a799 = {
            let (h798, _) = v3.clone();
            h798
        };
        let a800 = 64u64;
        nat_add(a799, a800)?
    }, f16({
        let (_, h801) = v3;
        h801
    }, f17(v0, v1, v2, {
        let (h802, _) = v3;
        h802
    })?)?))
}

pub fn f11() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f12(v0: Fn1, p806: u64, p807: (u64, Bytes)) -> (u64, Bytes) {
    let m805 = (p806, p807);
    {
        let mut m805 = m805;
        loop {
            let step = {
                let (v1, v2) = m805;
                let m803 = v1;
                if m803 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m803 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c804: Fn1 = v0.clone();
                        c804.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a809 = {
            let (h808, _) = v0.clone();
            h808
        };
        let a810 = 256u64;
        let a811 = 0u64;
        nat_quot(a809, a810, a811)
    }, f3(f14({
        let a813 = {
            let (h812, _) = v0;
            h812
        };
        let a814 = 256u64;
        let a815 = 0u64;
        nat_rem(a813, a814, a815)
    }), {
        let (_, h816) = v0;
        h816
    }))
}

pub fn f14(v0: u64) -> Bytes {
    f18(f19(), v0)
}

pub fn f15(v0: u32, v1: u32) -> u32 {
    let m819 = {
        let a817 = v0;
        let a818 = v1;
        fixed_u32::shift_right(a817, a818)
    };
    match m819 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f16(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f20(Fn2::F21, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h820, _, _, _, _, _, _, _) => {
            h820
        }
    }, match v0.clone() {
        Adt0::C0(_, h821, _, _, _, _, _, _) => {
            h821
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h822, _, _, _, _, _) => {
            h822
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h823, _, _, _, _) => {
            h823
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h824, _, _, _) => {
            h824
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h825, _, _) => {
            h825
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h826, _) => {
            h826
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h827) => {
            h827
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f22(match v0.clone() {
        Adt0::C0(h828, _, _, _, _, _, _, _) => {
            h828
        }
    }, match v2.clone() {
        Adt2::C0(h829, _, _, _, _, _, _, _, _, _) => {
            h829
        }
    }), f22(match v0.clone() {
        Adt0::C0(_, h830, _, _, _, _, _, _) => {
            h830
        }
    }, match v2.clone() {
        Adt2::C0(_, h831, _, _, _, _, _, _, _, _) => {
            h831
        }
    }), f22(match v0.clone() {
        Adt0::C0(_, _, h832, _, _, _, _, _) => {
            h832
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h833, _, _, _, _, _, _, _) => {
            h833
        }
    }), f22(match v0.clone() {
        Adt0::C0(_, _, _, h834, _, _, _, _) => {
            h834
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h835, _, _, _, _, _, _) => {
            h835
        }
    }), f22(match v0.clone() {
        Adt0::C0(_, _, _, _, h836, _, _, _) => {
            h836
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h837, _, _, _, _, _) => {
            h837
        }
    }), f22(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h838, _, _) => {
            h838
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h839, _, _, _, _) => {
            h839
        }
    }), f22(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h840, _) => {
            h840
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h841, _, _, _) => {
            h841
        }
    }), f22(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h842) => {
            h842
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h843, _, _) => {
            h843
        }
    })))
}

pub fn f17(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f23(v0.clone(), v1, v2, v3)?, f23(v0.clone(), v1, v2, {
        let a844 = v3;
        let a845 = 4u64;
        nat_add(a844, a845)?
    })?, f23(v0.clone(), v1, v2, {
        let a846 = v3;
        let a847 = 8u64;
        nat_add(a846, a847)?
    })?, f23(v0.clone(), v1, v2, {
        let a848 = v3;
        let a849 = 12u64;
        nat_add(a848, a849)?
    })?, f23(v0.clone(), v1, v2, {
        let a850 = v3;
        let a851 = 16u64;
        nat_add(a850, a851)?
    })?, f23(v0.clone(), v1, v2, {
        let a852 = v3;
        let a853 = 20u64;
        nat_add(a852, a853)?
    })?, f23(v0.clone(), v1, v2, {
        let a854 = v3;
        let a855 = 24u64;
        nat_add(a854, a855)?
    })?, f23(v0.clone(), v1, v2, {
        let a856 = v3;
        let a857 = 28u64;
        nat_add(a856, a857)?
    })?, f23(v0.clone(), v1, v2, {
        let a858 = v3;
        let a859 = 32u64;
        nat_add(a858, a859)?
    })?, f23(v0.clone(), v1, v2, {
        let a860 = v3;
        let a861 = 36u64;
        nat_add(a860, a861)?
    })?, f23(v0.clone(), v1, v2, {
        let a862 = v3;
        let a863 = 40u64;
        nat_add(a862, a863)?
    })?, f23(v0.clone(), v1, v2, {
        let a864 = v3;
        let a865 = 44u64;
        nat_add(a864, a865)?
    })?, f23(v0.clone(), v1, v2, {
        let a866 = v3;
        let a867 = 48u64;
        nat_add(a866, a867)?
    })?, f23(v0.clone(), v1, v2, {
        let a868 = v3;
        let a869 = 52u64;
        nat_add(a868, a869)?
    })?, f23(v0.clone(), v1, v2, {
        let a870 = v3;
        let a871 = 56u64;
        nat_add(a870, a871)?
    })?, f23(v0, v1, v2, {
        let a872 = v3;
        let a873 = 60u64;
        nat_add(a872, a873)?
    })?))
}

pub fn f18(v0: Bytes, v1: u64) -> Bytes {
    let m877 = {
        let a874 = v0;
        let a875 = v1;
        let a876 = 1u64;
        slice_bytes(a874, a875, a876)
    };
    match m877 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f19() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f20(v0: Fn2, p881: u64, p882: Adt2) -> R<Adt2> {
    let m880 = (p881, p882);
    {
        let mut m880 = m880;
        loop {
            let step = {
                let (v1, v2) = m880;
                let m878 = v1;
                if m878 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m878 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c879: Fn2 = v0.clone();
                        c879.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m880 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt2) -> R<Adt2> {
    f24(v0)
}

pub fn f22(v0: u32, v1: u32) -> u32 {
    let m885 = {
        let a883 = f25(v0);
        let a884 = f25(v1);
        fixed_u64::checked_add(a883, a884)
    };
    match m885 {
        None => {
            0u32
        }
        Some(v2) => {
            f26(v2)
        }
    }
}

pub fn f23(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a888 = {
        let a886 = v3;
        let a887 = 8u64;
        nat_add(a886, a887)?
    };
    let a889 = v2;
    if nat_eq(a888, a889) {
        Ok(f27({
            let a892 = {
                let a890 = v1;
                let a891 = 8u64;
                nat_mul(a890, a891)?
            };
            let a893 = 4294967296u64;
            let a894 = 0u64;
            nat_quot(a892, a893, a894)
        }))
    } else {
        let a897 = {
            let a895 = v3;
            let a896 = 4u64;
            nat_add(a895, a896)?
        };
        let a898 = v2;
        if nat_eq(a897, a898) {
            Ok(f27({
                let a901 = {
                    let a899 = v1;
                    let a900 = 8u64;
                    nat_mul(a899, a900)?
                };
                let a902 = 4294967296u64;
                let a903 = 0u64;
                nat_rem(a901, a902, a903)
            }))
        } else {
            Ok(f28(f29(v0.clone(), v1, v3), f29(v0.clone(), v1, {
                let a904 = v3;
                let a905 = 1u64;
                nat_add(a904, a905)?
            }), f29(v0.clone(), v1, {
                let a906 = v3;
                let a907 = 2u64;
                nat_add(a906, a907)?
            }), f29(v0, v1, {
                let a908 = v3;
                let a909 = 3u64;
                nat_add(a908, a909)?
            })))
        }
    }
}

pub fn f24(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f30(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h910) => {
            h910
        }
    });
    let v2: u32 = f22(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h911, _, _) => {
            h911
        }
    }, f22(f31(match v0.clone() {
        Adt2::C0(_, _, _, _, h912, _, _, _, _, _) => {
            h912
        }
    }), f22(f32(match v0.clone() {
        Adt2::C0(_, _, _, _, h913, _, _, _, _, _) => {
            h913
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h914, _, _, _, _) => {
            h914
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h915, _, _, _) => {
            h915
        }
    }), f22(v1, {
        let m918 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h917, _) => {
                h917
            }
        };
        match m918 {
            Adt1::C0(h916, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        }
    }))));
    let v3: u32 = f22(f33(match v0.clone() {
        Adt2::C0(h919, _, _, _, _, _, _, _, _, _) => {
            h919
        }
    }), f34(match v0.clone() {
        Adt2::C0(h920, _, _, _, _, _, _, _, _, _) => {
            h920
        }
    }, match v0.clone() {
        Adt2::C0(_, h921, _, _, _, _, _, _, _, _) => {
            h921
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h922, _, _, _, _, _, _, _) => {
            h922
        }
    }));
    Ok(Adt2::C0(f22(v2, v3), match v0.clone() {
        Adt2::C0(h923, _, _, _, _, _, _, _, _, _) => {
            h923
        }
    }, match v0.clone() {
        Adt2::C0(_, h924, _, _, _, _, _, _, _, _) => {
            h924
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h925, _, _, _, _, _, _, _) => {
            h925
        }
    }, f22(match v0.clone() {
        Adt2::C0(_, _, _, h926, _, _, _, _, _, _) => {
            h926
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h927, _, _, _, _, _) => {
            h927
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h928, _, _, _, _) => {
            h928
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h929, _, _, _) => {
            h929
        }
    }, f35(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h930, _) => {
            h930
        }
    }), {
        let a932 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h931) => {
                h931
            }
        };
        let a933 = 1u64;
        nat_add(a932, a933)?
    }))
}

pub fn f25(v0: u32) -> u64 {
    let m935 = {
        let a934 = v0;
        fixed_u64::convert(i128::from(a934))
    };
    match m935 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f26(v0: u64) -> u32 {
    let m939 = {
        let a938 = {
            let a936 = v0;
            let a937 = 4294967295u64;
            fixed_u64::bit_and(a936, a937)
        };
        fixed_u32::convert(i128::from(a938))
    };
    match m939 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f27(v0: u64) -> u32 {
    let (_, h940) = {
        let (_, h941) = f36(Fn3::F37, 32u64, (v0, (1u32, 0u32)));
        h941
    };
    h940
}

pub fn f28(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a946 = f38(v0, 24u32);
    let a947 = {
        let a944 = f38(v1, 16u32);
        let a945 = {
            let a942 = f38(v2, 8u32);
            let a943 = v3;
            fixed_u32::bit_or(a942, a943)
        };
        fixed_u32::bit_or(a944, a945)
    };
    fixed_u32::bit_or(a946, a947)
}

pub fn f29(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a948 = v2;
    let a949 = v1;
    if nat_lt(a948, a949) {
        let m952 = {
            let a950 = v0;
            let a951 = v2;
            index_bytes(a950, a951)
        };
        match m952 {
            None => {
                0u32
            }
            Some(v3) => {
                let m954 = {
                    let a953 = v3;
                    fixed_u32::convert(i128::from(a953))
                };
                match m954 {
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
        let a955 = v2;
        let a956 = v1;
        if nat_eq(a955, a956) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f30(v0: u64) -> u32 {
    let a957 = v0;
    let a958 = 32u64;
    if nat_lt(a957, a958) {
        let a959 = v0;
        let a960 = 16u64;
        if nat_lt(a959, a960) {
            let a961 = v0;
            let a962 = 8u64;
            if nat_lt(a961, a962) {
                let a963 = v0;
                let a964 = 4u64;
                if nat_lt(a963, a964) {
                    let a965 = v0;
                    let a966 = 2u64;
                    if nat_lt(a965, a966) {
                        let a967 = v0;
                        let a968 = 1u64;
                        if nat_lt(a967, a968) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a969 = v0;
                        let a970 = 3u64;
                        if nat_lt(a969, a970) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a971 = v0;
                    let a972 = 6u64;
                    if nat_lt(a971, a972) {
                        let a973 = v0;
                        let a974 = 5u64;
                        if nat_lt(a973, a974) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a975 = v0;
                        let a976 = 7u64;
                        if nat_lt(a975, a976) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a977 = v0;
                let a978 = 12u64;
                if nat_lt(a977, a978) {
                    let a979 = v0;
                    let a980 = 10u64;
                    if nat_lt(a979, a980) {
                        let a981 = v0;
                        let a982 = 9u64;
                        if nat_lt(a981, a982) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a983 = v0;
                        let a984 = 11u64;
                        if nat_lt(a983, a984) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a985 = v0;
                    let a986 = 14u64;
                    if nat_lt(a985, a986) {
                        let a987 = v0;
                        let a988 = 13u64;
                        if nat_lt(a987, a988) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a989 = v0;
                        let a990 = 15u64;
                        if nat_lt(a989, a990) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a991 = v0;
            let a992 = 24u64;
            if nat_lt(a991, a992) {
                let a993 = v0;
                let a994 = 20u64;
                if nat_lt(a993, a994) {
                    let a995 = v0;
                    let a996 = 18u64;
                    if nat_lt(a995, a996) {
                        let a997 = v0;
                        let a998 = 17u64;
                        if nat_lt(a997, a998) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a999 = v0;
                        let a1000 = 19u64;
                        if nat_lt(a999, a1000) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1001 = v0;
                    let a1002 = 22u64;
                    if nat_lt(a1001, a1002) {
                        let a1003 = v0;
                        let a1004 = 21u64;
                        if nat_lt(a1003, a1004) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1005 = v0;
                        let a1006 = 23u64;
                        if nat_lt(a1005, a1006) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1007 = v0;
                let a1008 = 28u64;
                if nat_lt(a1007, a1008) {
                    let a1009 = v0;
                    let a1010 = 26u64;
                    if nat_lt(a1009, a1010) {
                        let a1011 = v0;
                        let a1012 = 25u64;
                        if nat_lt(a1011, a1012) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1013 = v0;
                        let a1014 = 27u64;
                        if nat_lt(a1013, a1014) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1015 = v0;
                    let a1016 = 30u64;
                    if nat_lt(a1015, a1016) {
                        let a1017 = v0;
                        let a1018 = 29u64;
                        if nat_lt(a1017, a1018) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1019 = v0;
                        let a1020 = 31u64;
                        if nat_lt(a1019, a1020) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1021 = v0;
        let a1022 = 48u64;
        if nat_lt(a1021, a1022) {
            let a1023 = v0;
            let a1024 = 40u64;
            if nat_lt(a1023, a1024) {
                let a1025 = v0;
                let a1026 = 36u64;
                if nat_lt(a1025, a1026) {
                    let a1027 = v0;
                    let a1028 = 34u64;
                    if nat_lt(a1027, a1028) {
                        let a1029 = v0;
                        let a1030 = 33u64;
                        if nat_lt(a1029, a1030) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1031 = v0;
                        let a1032 = 35u64;
                        if nat_lt(a1031, a1032) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1033 = v0;
                    let a1034 = 38u64;
                    if nat_lt(a1033, a1034) {
                        let a1035 = v0;
                        let a1036 = 37u64;
                        if nat_lt(a1035, a1036) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1037 = v0;
                        let a1038 = 39u64;
                        if nat_lt(a1037, a1038) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1039 = v0;
                let a1040 = 44u64;
                if nat_lt(a1039, a1040) {
                    let a1041 = v0;
                    let a1042 = 42u64;
                    if nat_lt(a1041, a1042) {
                        let a1043 = v0;
                        let a1044 = 41u64;
                        if nat_lt(a1043, a1044) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1045 = v0;
                        let a1046 = 43u64;
                        if nat_lt(a1045, a1046) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1047 = v0;
                    let a1048 = 46u64;
                    if nat_lt(a1047, a1048) {
                        let a1049 = v0;
                        let a1050 = 45u64;
                        if nat_lt(a1049, a1050) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1051 = v0;
                        let a1052 = 47u64;
                        if nat_lt(a1051, a1052) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1053 = v0;
            let a1054 = 56u64;
            if nat_lt(a1053, a1054) {
                let a1055 = v0;
                let a1056 = 52u64;
                if nat_lt(a1055, a1056) {
                    let a1057 = v0;
                    let a1058 = 50u64;
                    if nat_lt(a1057, a1058) {
                        let a1059 = v0;
                        let a1060 = 49u64;
                        if nat_lt(a1059, a1060) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1061 = v0;
                        let a1062 = 51u64;
                        if nat_lt(a1061, a1062) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1063 = v0;
                    let a1064 = 54u64;
                    if nat_lt(a1063, a1064) {
                        let a1065 = v0;
                        let a1066 = 53u64;
                        if nat_lt(a1065, a1066) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1067 = v0;
                        let a1068 = 55u64;
                        if nat_lt(a1067, a1068) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1069 = v0;
                let a1070 = 60u64;
                if nat_lt(a1069, a1070) {
                    let a1071 = v0;
                    let a1072 = 58u64;
                    if nat_lt(a1071, a1072) {
                        let a1073 = v0;
                        let a1074 = 57u64;
                        if nat_lt(a1073, a1074) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1075 = v0;
                        let a1076 = 59u64;
                        if nat_lt(a1075, a1076) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1077 = v0;
                    let a1078 = 62u64;
                    if nat_lt(a1077, a1078) {
                        let a1079 = v0;
                        let a1080 = 61u64;
                        if nat_lt(a1079, a1080) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1081 = v0;
                        let a1082 = 63u64;
                        if nat_lt(a1081, a1082) {
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

pub fn f31(v0: u32) -> u32 {
    let a1085 = {
        let a1083 = f39(v0, 6u32, 26u32);
        let a1084 = f39(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1083, a1084)
    };
    let a1086 = f39(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1085, a1086)
}

pub fn f32(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1092 = {
        let a1087 = v0;
        let a1088 = v1;
        fixed_u32::bit_and(a1087, a1088)
    };
    let a1093 = {
        let a1090 = {
            let a1089 = v0;
            fixed_u32::bit_not(a1089)
        };
        let a1091 = v2;
        fixed_u32::bit_and(a1090, a1091)
    };
    fixed_u32::bit_xor(a1092, a1093)
}

pub fn f33(v0: u32) -> u32 {
    let a1096 = {
        let a1094 = f39(v0, 2u32, 30u32);
        let a1095 = f39(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1094, a1095)
    };
    let a1097 = f39(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1096, a1097)
}

pub fn f34(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1106 = {
        let a1102 = {
            let a1098 = v0;
            let a1099 = v1;
            fixed_u32::bit_and(a1098, a1099)
        };
        let a1103 = {
            let a1100 = v0;
            let a1101 = v2;
            fixed_u32::bit_and(a1100, a1101)
        };
        fixed_u32::bit_xor(a1102, a1103)
    };
    let a1107 = {
        let a1104 = v1;
        let a1105 = v2;
        fixed_u32::bit_and(a1104, a1105)
    };
    fixed_u32::bit_xor(a1106, a1107)
}

pub fn f35(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h1108, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1108
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1109, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1109
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1110, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1110
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1111, _, _, _, _, _, _, _, _, _, _, _) => {
            h1111
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1112, _, _, _, _, _, _, _, _, _, _) => {
            h1112
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h1113, _, _, _, _, _, _, _, _, _) => {
            h1113
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1114, _, _, _, _, _, _, _, _) => {
            h1114
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h1115, _, _, _, _, _, _, _) => {
            h1115
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h1116, _, _, _, _, _, _) => {
            h1116
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1117, _, _, _, _, _) => {
            h1117
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h1118, _, _, _, _) => {
            h1118
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1119, _, _, _) => {
            h1119
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1120, _, _) => {
            h1120
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1121, _) => {
            h1121
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1122) => {
            h1122
        }
    }, f22(f40(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1123, _) => {
            h1123
        }
    }), f22(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h1124, _, _, _, _, _, _) => {
            h1124
        }
    }, f22(f41(match v0.clone() {
        Adt1::C0(_, h1125, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1125
        }
    }), match v0 {
        Adt1::C0(h1126, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1126
        }
    }))))
}

pub fn f36(v0: Fn3, p1130: u64, p1131: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1129 = (p1130, p1131);
    {
        let mut m1129 = m1129;
        loop {
            let step = {
                let (v1, v2) = m1129;
                let m1127 = v1;
                if m1127 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1127 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1128: Fn3 = v0.clone();
                        c1128.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1132, _) = v0;
        h1132
    };
    let v2: u32 = {
        let (h1133, _) = {
            let (_, h1134) = v0;
            h1134
        };
        h1133
    };
    let v3: u32 = {
        let (_, h1135) = {
            let (_, h1136) = v0;
            h1136
        };
        h1135
    };
    ({
        let a1137 = v1;
        let a1138 = 2u64;
        let a1139 = 0u64;
        nat_quot(a1137, a1138, a1139)
    }, (f38(v2, 1u32), {
        let a1143 = {
            let a1140 = v1;
            let a1141 = 2u64;
            let a1142 = 0u64;
            nat_rem(a1140, a1141, a1142)
        };
        let a1144 = 1u64;
        if nat_eq(a1143, a1144) {
            let a1145 = v3;
            let a1146 = v2;
            fixed_u32::bit_or(a1145, a1146)
        } else {
            v3
        }
    }))
}

pub fn f38(v0: u32, v1: u32) -> u32 {
    let m1149 = {
        let a1147 = v0;
        let a1148 = v1;
        fixed_u32::shift_left(a1147, a1148)
    };
    match m1149 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f39(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1150 = f15(v0, v1);
    let a1151 = f38(v0, v2);
    fixed_u32::bit_or(a1150, a1151)
}

pub fn f40(v0: u32) -> u32 {
    let a1154 = {
        let a1152 = f39(v0, 17u32, 15u32);
        let a1153 = f39(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1152, a1153)
    };
    let a1155 = f15(v0, 10u32);
    fixed_u32::bit_xor(a1154, a1155)
}

pub fn f41(v0: u32) -> u32 {
    let a1158 = {
        let a1156 = f39(v0, 7u32, 25u32);
        let a1157 = f39(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1156, a1157)
    };
    let a1159 = f15(v0, 3u32);
    fixed_u32::bit_xor(a1158, a1159)
}
