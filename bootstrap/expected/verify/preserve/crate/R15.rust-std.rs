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
    C0(Bytes, Bytes, Option<Adt1>, List<Adt2>, List<Adt3>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Adt1),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, Option<Adt1>),
}

#[derive(Clone)]
pub enum Fn0 {
    F15,
    F27,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn0::F15 => f15(p0, p1),
            Fn0::F27 => Ok(f27(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt2) -> R<List<Bytes>> {
        match self {
            Fn1::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt3) -> R<List<Bytes>> {
        match self {
            Fn2::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F26(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn3::F26(k0) => f26(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F30,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn4::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F33,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn5::F33 => f33(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F35(Bytes),
    F40(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn6::F35(k0) => f35(k0.clone(), p0),
            Fn6::F40(k0, k1) => Ok(f40(*k0, k1.clone(), p0)),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    let v1: List<Bytes> = List::cons(f1(f2(Str::lit("severity")), f3(f2(Str::lit("error")))?)?, List::cons(f1(f2(Str::lit("spec")), f3(f2(Str::lit("lexlean/diagnostic/1")))?)?, List::<Bytes>::nil()));
    let v3: List<Bytes> = {
        let m2 = match v0.clone() {
            Adt0::C0(_, _, h1, _, _, _, _) => {
                h1
            }
        };
        match m2 {
            None => {
                v1
            }
            Some(v2) => {
                List::cons(f1(f2(Str::lit("primary")), f4(v2)?)?, v1)
            }
        }
    };
    Ok(f5(List::cons(f1(f2(Str::lit("causes")), f6(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h3) => {
            h3
        }
    })?)?, List::cons(f1(f2(Str::lit("code")), f3(match v0.clone() {
        Adt0::C0(h4, _, _, _, _, _, _) => {
            h4
        }
    })?)?, List::cons(f1(f2(Str::lit("help")), f6(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h5, _) => {
            h5
        }
    })?)?, List::cons(f1(f2(Str::lit("labels")), f7(match v0.clone() {
        Adt0::C0(_, _, _, h6, _, _, _) => {
            h6
        }
    })?)?, List::cons(f1(f2(Str::lit("message")), f3(match v0.clone() {
        Adt0::C0(_, h7, _, _, _, _, _) => {
            h7
        }
    })?)?, List::cons(f1(f2(Str::lit("notes")), f8(match v0 {
        Adt0::C0(_, _, _, _, h8, _, _) => {
            h8
        }
    })?)?, v3))))))))
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f9(f9(f3(v0)?, f2(Str::lit(":"))), v1))
}

pub fn f2(v0: Str) -> Bytes {
    let a9 = v0;
    utf8_encode(a9)
}

pub fn f3(v0: Bytes) -> R<Bytes> {
    f10(v0)
}

pub fn f4(v0: Adt1) -> R<Bytes> {
    Ok(f5(List::cons(f1(f2(Str::lit("byte_end")), f11(match v0.clone() {
        Adt1::C0(_, _, h10, _, _, _, _) => {
            h10
        }
    })?)?, List::cons(f1(f2(Str::lit("byte_start")), f11(match v0.clone() {
        Adt1::C0(_, h11, _, _, _, _, _) => {
            h11
        }
    })?)?, List::cons(f1(f2(Str::lit("column_end")), f11(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h12) => {
            h12
        }
    })?)?, List::cons(f1(f2(Str::lit("column_start")), f11(match v0.clone() {
        Adt1::C0(_, _, _, _, h13, _, _) => {
            h13
        }
    })?)?, List::cons(f1(f2(Str::lit("line_end")), f11(match v0.clone() {
        Adt1::C0(_, _, _, _, _, h14, _) => {
            h14
        }
    })?)?, List::cons(f1(f2(Str::lit("line_start")), f11(match v0.clone() {
        Adt1::C0(_, _, _, h15, _, _, _) => {
            h15
        }
    })?)?, List::cons(f1(f2(Str::lit("path")), f3(match v0 {
        Adt1::C0(h16, _, _, _, _, _, _) => {
            h16
        }
    })?)?, List::<Bytes>::nil())))))))))
}

pub fn f5(v0: List<Bytes>) -> Bytes {
    f9(f9(f2(Str::lit("{")), f12(v0, f2(Str::lit(",")))), f2(Str::lit("}")))
}

pub fn f6(v0: List<Bytes>) -> R<Bytes> {
    Ok(f13(f14(Fn0::F15, List::<Bytes>::nil(), f16(v0)?)?))
}

pub fn f7(v0: List<Adt2>) -> R<Bytes> {
    Ok(f13(f17(Fn1::F18, List::<Bytes>::nil(), f19(v0))?))
}

pub fn f8(v0: List<Adt3>) -> R<Bytes> {
    Ok(f13(f20(Fn2::F21, List::<Bytes>::nil(), f22(v0))?))
}

pub fn f9(v0: Bytes, v1: Bytes) -> Bytes {
    let a17 = v0;
    let a18 = v1;
    append_bytes(a17, a18)
}

pub fn f10(v0: Bytes) -> R<Bytes> {
    f23(v0)
}

pub fn f11(v0: u64) -> R<Bytes> {
    f24(v0)
}

pub fn f12(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m19 = f25(Fn3::F26(v1), None::<Bytes>, v0);
    match m19 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f13(v0: List<Bytes>) -> Bytes {
    f9(f9(f2(Str::lit("[")), f12(v0, f2(Str::lit(",")))), f2(Str::lit("]")))
}

pub fn f14(v0: Fn0, p23: List<Bytes>, p24: List<Bytes>) -> R<List<Bytes>> {
    let m22 = (p23, p24);
    {
        let mut m22 = m22;
        loop {
            let step = {
                let (v1, v2) = m22;
                let m20 = v2;
                match m20.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c21: Fn0 = v0.clone();
                            c21.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m22 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f3(v1)?, v0))
}

pub fn f16(v0: List<Bytes>) -> R<List<Bytes>> {
    f14(Fn0::F27, List::<Bytes>::nil(), v0)
}

pub fn f17(v0: Fn1, p28: List<Bytes>, p29: List<Adt2>) -> R<List<Bytes>> {
    let m27 = (p28, p29);
    {
        let mut m27 = m27;
        loop {
            let step = {
                let (v1, v2) = m27;
                let m25 = v2;
                match m25.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(({
                            let c26: Fn1 = v0.clone();
                            c26.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m27 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<Bytes>, v1: Adt2) -> R<List<Bytes>> {
    Ok(List::cons(f28(v1)?, v0))
}

pub fn f19(v0: List<Adt2>) -> List<Adt2> {
    f29(Fn4::F30, List::<Adt2>::nil(), v0)
}

pub fn f20(v0: Fn2, p33: List<Bytes>, p34: List<Adt3>) -> R<List<Bytes>> {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v1, v2) = m32;
                let m30 = v2;
                match m30.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt3>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt3>), R<List<Bytes>>>(({
                            let c31: Fn2 = v0.clone();
                            c31.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m32 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: List<Bytes>, v1: Adt3) -> R<List<Bytes>> {
    Ok(List::cons(f31(v1)?, v0))
}

pub fn f22(v0: List<Adt3>) -> List<Adt3> {
    f32(Fn5::F33, List::<Adt3>::nil(), v0)
}

pub fn f23(v0: Bytes) -> R<Bytes> {
    let a40 = {
        let a38 = Bytes::lit(&[0x22]);
        let a39 = {
            let (_, h35) = {
                let (h36, _) = f34(Fn6::F35(v0.clone()), {
                    let a37 = v0;
                    length_bytes(a37)
                }, (0u64, Bytes::lit(&[])))?;
                h36
            };
            h35
        };
        append_bytes(a38, a39)
    };
    let a41 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a40, a41))
}

pub fn f24(v0: u64) -> R<Bytes> {
    f36(f37(), 10u64, v0)
}

pub fn f25(v0: Fn3, p45: Option<Bytes>, p46: List<Bytes>) -> Option<Bytes> {
    let m44 = (p45, p46);
    {
        let mut m44 = m44;
        loop {
            let step = {
                let (v1, v2) = m44;
                let m42 = v2;
                match m42.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c43: Fn3 = v0.clone();
                            c43.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m44 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m47 = v1;
    match m47 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a50 = {
                    let a48 = v3;
                    let a49 = v0;
                    append_bytes(a48, a49)
                };
                let a51 = v2;
                append_bytes(a50, a51)
            })
        }
    }
}

pub fn f27(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f28(v0: Adt2) -> R<Bytes> {
    Ok(f5(List::cons(f1(f2(Str::lit("message")), f3(match v0.clone() {
        Adt2::C0(h52, _) => {
            h52
        }
    })?)?, List::cons(f1(f2(Str::lit("span")), f4(match v0 {
        Adt2::C0(_, h53) => {
            h53
        }
    })?)?, List::<Bytes>::nil()))))
}

pub fn f29(v0: Fn4, p57: List<Adt2>, p58: List<Adt2>) -> List<Adt2> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v1, v2) = m56;
                let m54 = v2;
                match m54.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c55: Fn4 = v0.clone();
                            c55.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f31(v0: Adt3) -> R<Bytes> {
    let m60 = match v0.clone() {
        Adt3::C0(_, h59) => {
            h59
        }
    };
    match m60 {
        None => {
            Ok(f5(List::cons(f1(f2(Str::lit("message")), f3(match v0 {
                Adt3::C0(h62, _) => {
                    h62
                }
            })?)?, List::<Bytes>::nil())))
        }
        Some(v1) => {
            Ok(f5(List::cons(f1(f2(Str::lit("message")), f3(match v0 {
                Adt3::C0(h61, _) => {
                    h61
                }
            })?)?, List::cons(f1(f2(Str::lit("span")), f4(v1)?)?, List::<Bytes>::nil()))))
        }
    }
}

pub fn f32(v0: Fn5, p66: List<Adt3>, p67: List<Adt3>) -> List<Adt3> {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v1, v2) = m65;
                let m63 = v2;
                match m63.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c64: Fn5 = v0.clone();
                            c64.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m65 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f34(v0: Fn6, p72: u64, p73: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m71 = (p72, p73);
    {
        let mut m71 = m71;
        loop {
            let step = {
                let (v1, v2) = m71;
                let m68 = v1;
                if m68 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m68 - 1;
                    let m70 = {
                        let c69: Fn6 = v0.clone();
                        c69.apply(v2.clone())?
                    };
                    match m70 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m71 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f38(v0, v1)
}

pub fn f36(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a74 = v2;
    let a75 = 0u64;
    if nat_eq(a74, a75) {
        Ok(f39(v0, 0u64))
    } else {
        let (_, h76) = {
            let (h77, _) = f34(Fn6::F40(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h77
        };
        Ok(h76)
    }
}

pub fn f37() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f38(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m81 = {
        let a79 = v0.clone();
        let a80 = {
            let (h78, _) = v1.clone();
            h78
        };
        index_bytes(a79, a80)
    };
    match m81 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a83 = {
                    let (h82, _) = v1.clone();
                    h82
                };
                let a84 = 1u64;
                nat_add(a83, a84)?
            }, {
                let a87 = {
                    let (_, h85) = v1;
                    h85
                };
                let a88 = f41(v0, {
                    let (h86, _) = v1;
                    h86
                }, v2)?;
                append_bytes(a87, a88)
            })))
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> Bytes {
    let m92 = {
        let a89 = v0;
        let a90 = v1;
        let a91 = 1u64;
        slice_bytes(a89, a90, a91)
    };
    match m92 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f42(v1, v0, v2)
}

pub fn f41(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a137 = {
            let a113 = {
                let a101 = {
                    let a95 = {
                        let a93 = v3;
                        let a94 = 128u8;
                        fixed_u8::bit_and(a93, a94)
                    };
                    let a96 = 128u8;
                    if equal(a95, a96) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a102 = {
                    let a99 = {
                        let a97 = v3;
                        let a98 = 64u8;
                        fixed_u8::bit_and(a97, a98)
                    };
                    let a100 = 64u8;
                    if equal(a99, a100) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a101, a102)?
            };
            let a114 = {
                let a111 = {
                    let a105 = {
                        let a103 = v3;
                        let a104 = 32u8;
                        fixed_u8::bit_and(a103, a104)
                    };
                    let a106 = 32u8;
                    if equal(a105, a106) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a112 = {
                    let a109 = {
                        let a107 = v3;
                        let a108 = 16u8;
                        fixed_u8::bit_and(a107, a108)
                    };
                    let a110 = 16u8;
                    if equal(a109, a110) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a111, a112)?
            };
            nat_add(a113, a114)?
        };
        let a138 = {
            let a135 = {
                let a123 = {
                    let a117 = {
                        let a115 = v3;
                        let a116 = 8u8;
                        fixed_u8::bit_and(a115, a116)
                    };
                    let a118 = 8u8;
                    if equal(a117, a118) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a124 = {
                    let a121 = {
                        let a119 = v3;
                        let a120 = 4u8;
                        fixed_u8::bit_and(a119, a120)
                    };
                    let a122 = 4u8;
                    if equal(a121, a122) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a123, a124)?
            };
            let a136 = {
                let a133 = {
                    let a127 = {
                        let a125 = v3;
                        let a126 = 2u8;
                        fixed_u8::bit_and(a125, a126)
                    };
                    let a128 = 2u8;
                    if equal(a127, a128) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a134 = {
                    let a131 = {
                        let a129 = v3;
                        let a130 = 1u8;
                        fixed_u8::bit_and(a129, a130)
                    };
                    let a132 = 1u8;
                    if equal(a131, a132) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a133, a134)?
            };
            nat_add(a135, a136)?
        };
        nat_add(a137, a138)?
    };
    let a139 = v4;
    let a140 = 34u64;
    if nat_eq(a139, a140) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a141 = v4;
        let a142 = 92u64;
        if nat_eq(a141, a142) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a143 = v4;
            let a144 = 8u64;
            if nat_eq(a143, a144) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a145 = v4;
                let a146 = 12u64;
                if nat_eq(a145, a146) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a147 = v4;
                    let a148 = 10u64;
                    if nat_eq(a147, a148) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a149 = v4;
                        let a150 = 13u64;
                        if nat_eq(a149, a150) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a151 = v4;
                            let a152 = 9u64;
                            if nat_eq(a151, a152) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a153 = v4;
                                let a154 = 32u64;
                                if nat_lt(a153, a154) {
                                    let a155 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a156 = f43(v2)?;
                                    Ok(append_bytes(a155, a156))
                                } else {
                                    Ok(f44(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h157, _) = v2;
        h157
    };
    let a158 = v3;
    let a159 = 0u64;
    if nat_eq(a158, a159) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a160 = v3;
            let a161 = v1;
            let a162 = 0u64;
            nat_quot(a160, a161, a162)
        }, {
            let a167 = f39(v0, {
                let a163 = v3;
                let a164 = v1;
                let a165 = 0u64;
                nat_rem(a163, a164, a165)
            });
            let a168 = {
                let (_, h166) = v2;
                h166
            };
            append_bytes(a167, a168)
        }))
    }
}

pub fn f43(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a213 = {
            let a189 = {
                let a177 = {
                    let a171 = {
                        let a169 = v1;
                        let a170 = 128u8;
                        fixed_u8::bit_and(a169, a170)
                    };
                    let a172 = 128u8;
                    if equal(a171, a172) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a178 = {
                    let a175 = {
                        let a173 = v1;
                        let a174 = 64u8;
                        fixed_u8::bit_and(a173, a174)
                    };
                    let a176 = 64u8;
                    if equal(a175, a176) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a177, a178)?
            };
            let a190 = {
                let a187 = {
                    let a181 = {
                        let a179 = v1;
                        let a180 = 32u8;
                        fixed_u8::bit_and(a179, a180)
                    };
                    let a182 = 32u8;
                    if equal(a181, a182) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a188 = {
                    let a185 = {
                        let a183 = v1;
                        let a184 = 16u8;
                        fixed_u8::bit_and(a183, a184)
                    };
                    let a186 = 16u8;
                    if equal(a185, a186) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a187, a188)?
            };
            nat_add(a189, a190)?
        };
        let a214 = {
            let a211 = {
                let a199 = {
                    let a193 = {
                        let a191 = v1;
                        let a192 = 8u8;
                        fixed_u8::bit_and(a191, a192)
                    };
                    let a194 = 8u8;
                    if equal(a193, a194) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a200 = {
                    let a197 = {
                        let a195 = v1;
                        let a196 = 4u8;
                        fixed_u8::bit_and(a195, a196)
                    };
                    let a198 = 4u8;
                    if equal(a197, a198) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a199, a200)?
            };
            let a212 = {
                let a209 = {
                    let a203 = {
                        let a201 = v1;
                        let a202 = 2u8;
                        fixed_u8::bit_and(a201, a202)
                    };
                    let a204 = 2u8;
                    if equal(a203, a204) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a210 = {
                    let a207 = {
                        let a205 = v1;
                        let a206 = 1u8;
                        fixed_u8::bit_and(a205, a206)
                    };
                    let a208 = 1u8;
                    if equal(a207, a208) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a209, a210)?
            };
            nat_add(a211, a212)?
        };
        nat_add(a213, a214)?
    };
    let a221 = f39(f45(), {
        let a215 = v2;
        let a216 = 16u64;
        let a217 = 0u64;
        nat_quot(a215, a216, a217)
    });
    let a222 = f39(f45(), {
        let a218 = v2;
        let a219 = 16u64;
        let a220 = 0u64;
        nat_rem(a218, a219, a220)
    });
    Ok(append_bytes(a221, a222))
}

pub fn f44(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m226 = {
        let a223 = v0;
        let a224 = v1;
        let a225 = v2;
        slice_bytes(a223, a224, a225)
    };
    match m226 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f45() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}
