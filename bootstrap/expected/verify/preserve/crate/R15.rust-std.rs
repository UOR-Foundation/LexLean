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
    C0(Bytes, List<Bytes>, List<Adt1>, List<u64>, List<Adt6>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64),
    C1(Adt2),
    C2(Bytes, List<Adt2>),
    C3(u64, u64),
    C4(Bytes, Adt3, u64, u64),
    C5(Bytes, Adt3, u64, u64),
    C6(Bytes, u64, u64, u64, bool),
    C7(Bytes),
    C8(List<u64>),
    C9(List<Adt4>, u64),
    C10(Bytes, Bytes, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(std::rc::Rc<Adt2>),
    C2(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, Adt5),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(bool),
    C1(Bytes),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt6>),
    C4(std::rc::Rc<Adt6>),
    C5,
    C6(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C7(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F4,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt6) -> R<Bytes> {
        match self {
            Fn0::F4 => f4(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7,
}

impl Fn1 {
    pub fn apply(&self, p0: Bytes) -> R<Bytes> {
        match self {
            Fn1::F7 => f7(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F9,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt1) -> R<Bytes> {
        match self {
            Fn2::F9 => f9(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F11,
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Bytes> {
        match self {
            Fn3::F11 => f11(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F14(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Bytes) -> (bool, Bytes) {
        match self {
            Fn4::F14(k0) => f14(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F16(Bytes),
    F63(Bytes),
    F71(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F16(k0) => f16(k0.clone(), p0),
            Fn5::F63(k0) => f63(k0.clone(), p0),
            Fn5::F71(k0, k1) => Ok(f71(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F19(Fn0),
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt6) -> R<List<Bytes>> {
        match self {
            Fn6::F19(k0) => f19(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F22(Fn1),
    F32,
    F77,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn7::F22(k0) => f22(k0.clone(), p0, p1),
            Fn7::F32 => Ok(f32(p0, p1)),
            Fn7::F77 => Ok(f77(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F24(Fn2),
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt1) -> R<List<Bytes>> {
        match self {
            Fn8::F24(k0) => f24(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F27(Fn3),
    F37,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: u64) -> R<List<Bytes>> {
        match self {
            Fn9::F27(k0) => f27(k0.clone(), p0, p1),
            Fn9::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F39,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt4) -> R<List<Bytes>> {
        match self {
            Fn10::F39 => f39(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F42(u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn11::F42(k0, k1) => f42(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F55,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt2) -> R<List<Bytes>> {
        match self {
            Fn12::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F66,
    F114,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn13::F66 => f66(p0, p1),
            Fn13::F114 => f114(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F69,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn14::F69 => f69(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F76,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Bytes>) -> R<Option<List<Bytes>>> {
        match self {
            Fn15::F76 => f76(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F80(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn16::F80(k0) => f80(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F87(u64, Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn17::F87(k0, k1) => f87(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F92,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn18::F92 => f92(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F94,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn19::F94 => f94(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F99,
}

impl Fn20 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn20::F99 => f99(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F102,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F108,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F108 => f108(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F113,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn23::F113 => f113(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    Ok(f1(List::cons(f2(Str::lit("{'declarations':["))?, List::cons(f1(f3(match v0.clone() {
        Adt0::C0(_, _, _, _, h1) => {
            h1
        }
    }, Fn0::F4)?, f5(Str::lit(","))), List::cons(f2(Str::lit("],'imports':["))?, List::cons(f1(f6(match v0.clone() {
        Adt0::C0(_, h2, _, _, _) => {
            h2
        }
    }, Fn1::F7)?, f5(Str::lit(","))), List::cons(f2(Str::lit("],'nodes':["))?, List::cons(f1(f8(match v0.clone() {
        Adt0::C0(_, _, h3, _, _) => {
            h3
        }
    }, Fn2::F9)?, f5(Str::lit(","))), List::cons(f2(Str::lit("],'proof_nodes':["))?, List::cons(f1(f10(match v0.clone() {
        Adt0::C0(_, _, _, h4, _) => {
            h4
        }
    }, Fn3::F11)?, f5(Str::lit(","))), List::cons(f2(Str::lit("],'spec':"))?, List::cons(f12(match v0 {
        Adt0::C0(h5, _, _, _, _) => {
            h5
        }
    })?, List::cons(f5(Str::lit("}")), List::<Bytes>::nil()))))))))))), f5(Str::lit(""))))
}

pub fn f1(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let (_, h6) = f13(Fn4::F14(v1), (true, f5(Str::lit(""))), v0);
    h6
}

pub fn f2(v0: Str) -> R<Bytes> {
    let v1: Bytes = f5(v0);
    let (_, h7) = {
        let (h8, _) = f15(Fn5::F16(v1.clone()), {
            let a10 = {
                let a9 = v1;
                length_bytes(a9)
            };
            let a11 = 1u64;
            nat_add(a10, a11)?
        }, (0u64, Bytes::lit(&[])))?;
        h8
    };
    Ok(h7)
}

pub fn f3(v0: List<Adt6>, v1: Fn0) -> R<List<Bytes>> {
    f17(f18(Fn6::F19(v1), List::<Bytes>::nil(), v0)?)
}

pub fn f4(v0: Adt6) -> R<Bytes> {
    f20(v0)
}

pub fn f5(v0: Str) -> Bytes {
    let a12 = v0;
    utf8_encode(a12)
}

pub fn f6(v0: List<Bytes>, v1: Fn1) -> R<List<Bytes>> {
    f17(f21(Fn7::F22(v1), List::<Bytes>::nil(), v0)?)
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    f12(v0)
}

pub fn f8(v0: List<Adt1>, v1: Fn2) -> R<List<Bytes>> {
    f17(f23(Fn8::F24(v1), List::<Bytes>::nil(), v0)?)
}

pub fn f9(v0: Adt1) -> R<Bytes> {
    f25(v0)
}

pub fn f10(v0: List<u64>, v1: Fn3) -> R<List<Bytes>> {
    f17(f26(Fn9::F27(v1), List::<Bytes>::nil(), v0)?)
}

pub fn f11(v0: u64) -> R<Bytes> {
    f28(v0)
}

pub fn f12(v0: Bytes) -> R<Bytes> {
    f29(v0)
}

pub fn f13(v0: Fn4, p16: (bool, Bytes), p17: List<Bytes>) -> (bool, Bytes) {
    let m15 = (p16, p17);
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, v2) = m15;
                let m13 = v2;
                match m13.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Bytes>), (bool, Bytes)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Bytes>), (bool, Bytes)>(({
                            let c14: Fn4 = v0.clone();
                            c14.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m15 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: (bool, Bytes), v2: Bytes) -> (bool, Bytes) {
    (false, {
        let (h18, _) = v1;
        if h18 {
            v2
        } else {
            f30(f30({
                let (_, h19) = v1;
                h19
            }, v0), v2)
        }
    })
}

pub fn f15(v0: Fn5, p24: u64, p25: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m20 = v1;
                if m20 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m20 - 1;
                    let m22 = {
                        let c21: Fn5 = v0.clone();
                        c21.apply(v2.clone())?
                    };
                    match m22 {
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
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f31(v0, v1)
}

pub fn f17(v0: List<Bytes>) -> R<List<Bytes>> {
    f21(Fn7::F32, List::<Bytes>::nil(), v0)
}

pub fn f18(v0: Fn6, p29: List<Bytes>, p30: List<Adt6>) -> R<List<Bytes>> {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v1, v2) = m28;
                let m26 = v2;
                match m26.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt6>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt6>), R<List<Bytes>>>(({
                            let c27: Fn6 = v0.clone();
                            c27.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m28 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Fn0, v1: List<Bytes>, v2: Adt6) -> R<List<Bytes>> {
    Ok(List::cons({
        let c31: Fn0 = v0;
        c31.apply(v2)?
    }, v1))
}

pub fn f20(v0: Adt6) -> R<Bytes> {
    f33(v0)
}

pub fn f21(v0: Fn7, p35: List<Bytes>, p36: List<Bytes>) -> R<List<Bytes>> {
    let m34 = (p35, p36);
    {
        let mut m34 = m34;
        loop {
            let step = {
                let (v1, v2) = m34;
                let m32 = v2;
                match m32.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c33: Fn7 = v0.clone();
                            c33.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m34 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Fn1, v1: List<Bytes>, v2: Bytes) -> R<List<Bytes>> {
    Ok(List::cons({
        let c37: Fn1 = v0;
        c37.apply(v2)?
    }, v1))
}

pub fn f23(v0: Fn8, p41: List<Bytes>, p42: List<Adt1>) -> R<List<Bytes>> {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v1, v2) = m40;
                let m38 = v2;
                match m38.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt1>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt1>), R<List<Bytes>>>(({
                            let c39: Fn8 = v0.clone();
                            c39.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m40 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Fn2, v1: List<Bytes>, v2: Adt1) -> R<List<Bytes>> {
    Ok(List::cons({
        let c43: Fn2 = v0;
        c43.apply(v2)?
    }, v1))
}

pub fn f25(v0: Adt1) -> R<Bytes> {
    let m44 = v0;
    match m44 {
        Adt1::C0(v1) => {
            Ok(f30(f30(f2(Str::lit("{'i':"))?, f28(v1)?), f2(Str::lit(",'k':'b'}"))?))
        }
        Adt1::C1(v2) => {
            Ok(f30(f30(f2(Str::lit("{'k':'s','l':"))?, f34(v2)?), f5(Str::lit("}"))))
        }
        Adt1::C2(v3, v4) => {
            Ok(f30(f30(f30(f30(f2(Str::lit("{'k':'c','n':"))?, f12(v3)?), f2(Str::lit(",'u':["))?), f35(v4)?), f5(Str::lit("]}"))))
        }
        Adt1::C3(v5, v6) => {
            Ok(f30(f30(f30(f30(f2(Str::lit("{'f':"))?, f28(v5)?), f2(Str::lit(",'k':'a','x':"))?), f28(v6)?), f5(Str::lit("}"))))
        }
        Adt1::C4(v7, v8, v9, v10) => {
            f36(f5(Str::lit("l")), v7, v8, v9, v10)
        }
        Adt1::C5(v11, v12, v13, v14) => {
            f36(f5(Str::lit("p")), v11, v12, v13, v14)
        }
        Adt1::C6(v15, v16, v17, v18, v19) => {
            Ok(f30(f30(f30(f30(f30(f30(f30(f30(f2(Str::lit("{'body':"))?, f28(v18)?), f2(Str::lit(",'d':"))?), if v19 {
                f5(Str::lit("true"))
            } else {
                f5(Str::lit("false"))
            }), f2(Str::lit(",'k':'e','n':"))?), f12(v15)?), f30(f2(Str::lit(",'t':"))?, f28(v16)?)), f30(f2(Str::lit(",'v':"))?, f28(v17)?)), f5(Str::lit("}"))))
        }
        Adt1::C7(v20) => {
            Ok(f30(f30(f2(Str::lit("{'k':'n','v':"))?, f12(v20)?), f5(Str::lit("}"))))
        }
        Adt1::C8(v21) => {
            Ok(f30(f30(f2(Str::lit("{'k':'t','v':["))?, f1(f17(f26(Fn9::F37, List::<Bytes>::nil(), v21)?)?, f5(Str::lit(",")))), f5(Str::lit("]}"))))
        }
        Adt1::C9(v22, v23) => {
            Ok(f30(f30(f30(f30(f2(Str::lit("{'k':'d','m':["))?, f1(f17(f38(Fn10::F39, List::<Bytes>::nil(), v22)?)?, f5(Str::lit(",")))), f2(Str::lit("],'x':"))?), f28(v23)?), f5(Str::lit("}"))))
        }
        Adt1::C10(v24, v25, v26, v27) => {
            Ok(f30(f30(f30(f30(f30(f30(f30(f30(f2(Str::lit("{'i':"))?, f28(v26)?), f2(Str::lit(",'k':'j','n':"))?), f12(v24)?), f2(Str::lit(",'s':"))?), f12(v25)?), f2(Str::lit(",'x':"))?), f28(v27)?), f5(Str::lit("}"))))
        }
    }
}

pub fn f26(v0: Fn9, p48: List<Bytes>, p49: List<u64>) -> R<List<Bytes>> {
    let m47 = (p48, p49);
    {
        let mut m47 = m47;
        loop {
            let step = {
                let (v1, v2) = m47;
                let m45 = v2;
                match m45.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<u64>), R<List<Bytes>>>(({
                            let c46: Fn9 = v0.clone();
                            c46.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m47 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Fn3, v1: List<Bytes>, v2: u64) -> R<List<Bytes>> {
    Ok(List::cons({
        let c50: Fn3 = v0;
        c50.apply(v2)?
    }, v1))
}

pub fn f28(v0: u64) -> R<Bytes> {
    f40(v0)
}

pub fn f29(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a51 = v0.clone();
        length_bytes(a51)
    };
    let v2: Adt7 = {
        let (h52, _) = f41(Fn11::F42(v1, v0), {
            let a53 = v1;
            let a54 = 1u64;
            nat_add(a53, a54)?
        }, Adt7::C0(0u64, List::<Bytes>::nil()))?;
        h52
    };
    Ok(f43(f43(f44(Str::lit("\"")), f45(match v2 {
        Adt7::C0(_, h55) => {
            h55
        }
    })?), f44(Str::lit("\""))))
}

pub fn f30(v0: Bytes, v1: Bytes) -> Bytes {
    let a56 = v0;
    let a57 = v1;
    append_bytes(a56, a57)
}

pub fn f31(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m59 = f46(v0.clone(), {
        let (h58, _) = v1.clone();
        h58
    });
    match m59 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a61 = {
                    let (h60, _) = v1.clone();
                    h60
                };
                let a62 = 1u64;
                nat_add(a61, a62)?
            }, f30({
                let (_, h63) = v1;
                h63
            }, {
                let a110 = {
                    let v3: u8 = v2;
                    let a108 = {
                        let a84 = {
                            let a72 = {
                                let a66 = {
                                    let a64 = v3;
                                    let a65 = 128u8;
                                    fixed_u8::bit_and(a64, a65)
                                };
                                let a67 = 128u8;
                                if equal(a66, a67) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a73 = {
                                let a70 = {
                                    let a68 = v3;
                                    let a69 = 64u8;
                                    fixed_u8::bit_and(a68, a69)
                                };
                                let a71 = 64u8;
                                if equal(a70, a71) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a72, a73)?
                        };
                        let a85 = {
                            let a82 = {
                                let a76 = {
                                    let a74 = v3;
                                    let a75 = 32u8;
                                    fixed_u8::bit_and(a74, a75)
                                };
                                let a77 = 32u8;
                                if equal(a76, a77) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a83 = {
                                let a80 = {
                                    let a78 = v3;
                                    let a79 = 16u8;
                                    fixed_u8::bit_and(a78, a79)
                                };
                                let a81 = 16u8;
                                if equal(a80, a81) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a82, a83)?
                        };
                        nat_add(a84, a85)?
                    };
                    let a109 = {
                        let a106 = {
                            let a94 = {
                                let a88 = {
                                    let a86 = v3;
                                    let a87 = 8u8;
                                    fixed_u8::bit_and(a86, a87)
                                };
                                let a89 = 8u8;
                                if equal(a88, a89) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a95 = {
                                let a92 = {
                                    let a90 = v3;
                                    let a91 = 4u8;
                                    fixed_u8::bit_and(a90, a91)
                                };
                                let a93 = 4u8;
                                if equal(a92, a93) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a94, a95)?
                        };
                        let a107 = {
                            let a104 = {
                                let a98 = {
                                    let a96 = v3;
                                    let a97 = 2u8;
                                    fixed_u8::bit_and(a96, a97)
                                };
                                let a99 = 2u8;
                                if equal(a98, a99) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a105 = {
                                let a102 = {
                                    let a100 = v3;
                                    let a101 = 1u8;
                                    fixed_u8::bit_and(a100, a101)
                                };
                                let a103 = 1u8;
                                if equal(a102, a103) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a104, a105)?
                        };
                        nat_add(a106, a107)?
                    };
                    nat_add(a108, a109)?
                };
                let a111 = 39u64;
                if nat_eq(a110, a111) {
                    Bytes::lit(&[0x22])
                } else {
                    f47(v0, {
                        let (h112, _) = v1;
                        h112
                    }, 1u64)
                }
            }))))
        }
    }
}

pub fn f32(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f33(v0: Adt6) -> R<Bytes> {
    let m113 = v0;
    match m113 {
        Adt6::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt6::C1(v2, v3) => {
            f48(v2, v3)
        }
        Adt6::C2(v4) => {
            f49(v4)
        }
        Adt6::C3(r114) => {
            let v5 = (*r114).clone();
            Ok(f50(f51(v5)?))
        }
        Adt6::C4(r115) => {
            let v6 = (*r115).clone();
            f52(f53(v6)?)
        }
        Adt6::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt6::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt6::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f34(v0: Adt2) -> R<Bytes> {
    let m116 = v0;
    match m116 {
        Adt2::C0 => {
            f2(Str::lit("{'k':'z'}"))
        }
        Adt2::C1(r117) => {
            let v1 = (*r117).clone();
            Ok(f30(f30(f2(Str::lit("{'a':"))?, f34(v1)?), f2(Str::lit(",'k':'s'}"))?))
        }
        Adt2::C2(r118, r119) => {
            let v2 = (*r118).clone();
            let v3 = (*r119).clone();
            Ok(f30(f30(f30(f30(f2(Str::lit("{'a':["))?, f34(v2)?), f5(Str::lit(","))), f34(v3)?), f2(Str::lit("],'k':'m'}"))?))
        }
        Adt2::C3(r120, r121) => {
            let v4 = (*r120).clone();
            let v5 = (*r121).clone();
            Ok(f30(f30(f30(f30(f2(Str::lit("{'a':["))?, f34(v4)?), f5(Str::lit(","))), f34(v5)?), f2(Str::lit("],'k':'i'}"))?))
        }
        Adt2::C4(v6) => {
            Ok(f30(f30(f2(Str::lit("{'a':"))?, f12(v6)?), f2(Str::lit(",'k':'p'}"))?))
        }
    }
}

pub fn f35(v0: List<Adt2>) -> R<Bytes> {
    Ok(f1(f17(f54(Fn12::F55, List::<Bytes>::nil(), v0)?)?, f5(Str::lit(","))))
}

pub fn f36(v0: Bytes, v1: Bytes, v2: Adt3, v3: u64, v4: u64) -> R<Bytes> {
    Ok(f30(f30(f30(f30(f30(f30(f30(f30(f2(Str::lit("{'b':'"))?, f56(v2)), f2(Str::lit("','k':'"))?), v0), f2(Str::lit("','n':"))?), f12(v1)?), f30(f2(Str::lit(",'t':"))?, f28(v3)?)), f30(f2(Str::lit(",'v':"))?, f28(v4)?)), f5(Str::lit("}"))))
}

pub fn f37(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    Ok(List::cons(f28(v1)?, v0))
}

pub fn f38(v0: Fn10, p125: List<Bytes>, p126: List<Adt4>) -> R<List<Bytes>> {
    let m124 = (p125, p126);
    {
        let mut m124 = m124;
        loop {
            let step = {
                let (v1, v2) = m124;
                let m122 = v2;
                match m122.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt4>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt4>), R<List<Bytes>>>(({
                            let c123: Fn10 = v0.clone();
                            c123.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m124 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: List<Bytes>, v1: Adt4) -> R<List<Bytes>> {
    Ok(List::cons(f57(v1)?, v0))
}

pub fn f40(v0: u64) -> R<Bytes> {
    f58(f59(), 10u64, v0)
}

pub fn f41(v0: Fn11, p131: u64, p132: Adt7) -> R<(Adt7, bool)> {
    let m130 = (p131, p132);
    {
        let mut m130 = m130;
        loop {
            let step = {
                let (v1, v2) = m130;
                let m127 = v1;
                if m127 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m127 - 1;
                    let m129 = {
                        let c128: Fn11 = v0.clone();
                        c128.apply(v2.clone())?
                    };
                    match m129 {
                        None => {
                            Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt7), R<(Adt7, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: u64, v1: Bytes, v2: Adt7) -> R<Option<Adt7>> {
    f60(v1, v0, v2)
}

pub fn f43(v0: Bytes, v1: Bytes) -> Bytes {
    let a133 = v0;
    let a134 = v1;
    append_bytes(a133, a134)
}

pub fn f44(v0: Str) -> Bytes {
    let a135 = v0;
    utf8_encode(a135)
}

pub fn f45(v0: List<Bytes>) -> R<Bytes> {
    f61(f62(v0)?)
}

pub fn f46(v0: Bytes, v1: u64) -> Option<u8> {
    let a136 = v0;
    let a137 = v1;
    index_bytes(a136, a137)
}

pub fn f47(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m141 = {
        let a138 = v0;
        let a139 = v1;
        let a140 = v2;
        slice_bytes(a138, a139, a140)
    };
    match m141 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f48(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a142 = 0u64;
        let a143 = v1;
        nat_lt(a142, a143)
    } else {
        false
    } {
        let a144 = Bytes::lit(&[0x2d]);
        let a145 = f40(v1)?;
        Ok(append_bytes(a144, a145))
    } else {
        f40(v1)
    }
}

pub fn f49(v0: Bytes) -> R<Bytes> {
    let a151 = {
        let a149 = Bytes::lit(&[0x22]);
        let a150 = {
            let (_, h146) = {
                let (h147, _) = f15(Fn5::F63(v0.clone()), {
                    let a148 = v0;
                    length_bytes(a148)
                }, (0u64, Bytes::lit(&[])))?;
                h147
            };
            h146
        };
        append_bytes(a149, a150)
    };
    let a152 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a151, a152))
}

pub fn f50(v0: List<Bytes>) -> Bytes {
    let a155 = {
        let a153 = Bytes::lit(&[0x5b]);
        let a154 = f64(v0, Bytes::lit(&[0x2c]));
        append_bytes(a153, a154)
    };
    let a156 = Bytes::lit(&[0x5d]);
    append_bytes(a155, a156)
}

pub fn f51(v0: Adt6) -> R<List<Bytes>> {
    let m157 = v0;
    match m157 {
        Adt6::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C6(r158, r159) => {
            let v7 = (*r158).clone();
            let v8 = (*r159).clone();
            Ok(List::cons(f33(v7)?, f51(v8)?))
        }
        Adt6::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f52(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f65(Fn13::F66, List::<(Bytes, Bytes)>::nil(), f67(v0)?);
    let v2: List<Bytes> = f68(Fn14::F69, List::<Bytes>::nil(), v1)?;
    let a162 = {
        let a160 = Bytes::lit(&[0x7b]);
        let a161 = f64(v2, Bytes::lit(&[0x2c]));
        append_bytes(a160, a161)
    };
    let a163 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a162, a163))
}

pub fn f53(v0: Adt6) -> R<List<(Bytes, Bytes)>> {
    let m164 = v0;
    match m164 {
        Adt6::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C7(v9, r165, r166) => {
            let v10 = (*r165).clone();
            let v11 = (*r166).clone();
            Ok(List::cons((v9, f33(v10)?), f53(v11)?))
        }
    }
}

pub fn f54(v0: Fn12, p170: List<Bytes>, p171: List<Adt2>) -> R<List<Bytes>> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v1, v2) = m169;
                let m167 = v2;
                match m167.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(({
                            let c168: Fn12 = v0.clone();
                            c168.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m169 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<Bytes>, v1: Adt2) -> R<List<Bytes>> {
    Ok(List::cons(f34(v1)?, v0))
}

pub fn f56(v0: Adt3) -> Bytes {
    let m172 = v0;
    match m172 {
        Adt3::C0 => {
            f5(Str::lit("e"))
        }
        Adt3::C1 => {
            f5(Str::lit("i"))
        }
        Adt3::C2 => {
            f5(Str::lit("s"))
        }
        Adt3::C3 => {
            f5(Str::lit("c"))
        }
    }
}

pub fn f57(v0: Adt4) -> R<Bytes> {
    Ok(f30(f30(f30(f30(f2(Str::lit("{'n':"))?, f12(match v0.clone() {
        Adt4::C0(h173, _) => {
            h173
        }
    })?), f2(Str::lit(",'v':"))?), {
        let m175 = match v0 {
            Adt4::C0(_, h174) => {
                h174
            }
        };
        match m175 {
            Adt5::C0(v1) => {
                f30(f30(f2(Str::lit("{'t':'b','v':"))?, if v1 {
                    f5(Str::lit("true"))
                } else {
                    f5(Str::lit("false"))
                }), f5(Str::lit("}")))
            }
            Adt5::C1(v2) => {
                f30(f30(f2(Str::lit("{'t':'n','v':"))?, f12(v2)?), f5(Str::lit("}")))
            }
            Adt5::C2(v3) => {
                f30(f30(f2(Str::lit("{'t':'i','v':"))?, f12(v3)?), f5(Str::lit("}")))
            }
            Adt5::C3(v4) => {
                f30(f30(f2(Str::lit("{'t':'s','v':"))?, f12(v4)?), f5(Str::lit("}")))
            }
            Adt5::C4(v5) => {
                f30(f30(f2(Str::lit("{'t':'m','v':"))?, f12(v5)?), f5(Str::lit("}")))
            }
        }
    }), f5(Str::lit("}"))))
}

pub fn f58(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a176 = v2;
    let a177 = 0u64;
    if nat_eq(a176, a177) {
        Ok(f70(v0, 0u64))
    } else {
        let (_, h178) = {
            let (h179, _) = f15(Fn5::F71(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h179
        };
        Ok(h178)
    }
}

pub fn f59() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f60(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h180, _) => {
            h180
        }
    };
    let a181 = v3;
    let a182 = v1;
    if nat_lt(a181, a182) {
        let v4: u64 = f72(v0.clone(), v3)?;
        let a183 = v4;
        let a184 = 32u64;
        if if nat_lt(a183, a184) {
            true
        } else {
            let a185 = v4;
            let a186 = 34u64;
            if nat_eq(a185, a186) {
                true
            } else {
                let a187 = v4;
                let a188 = 92u64;
                nat_eq(a187, a188)
            }
        } {
            Ok(Some(Adt7::C0({
                let a189 = v3;
                let a190 = 1u64;
                nat_add(a189, a190)?
            }, List::cons(f73(v4), match v2 {
                Adt7::C0(_, h191) => {
                    h191
                }
            }))))
        } else {
            let v5: u64 = f74(v0.clone(), v1, v3)?;
            Ok(Some(Adt7::C0(v5, List::cons(f47(v0, v3, {
                let a192 = v5;
                let a193 = v3;
                nat_sub(a192, a193)
            }), match v2 {
                Adt7::C0(_, h194) => {
                    h194
                }
            }))))
        }
    } else {
        Ok(None::<Adt7>)
    }
}

pub fn f61(v0: List<Bytes>) -> R<Bytes> {
    let m197 = {
        let (h195, _) = f75(Fn15::F76, {
            let a196 = v0.clone();
            length_list(a196)
        }, v0)?;
        h195
    };
    match m197.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, _)) => {
            Ok(v1)
        }
    }
}

pub fn f62(v0: List<Bytes>) -> R<List<Bytes>> {
    f21(Fn7::F77, List::<Bytes>::nil(), v0)
}

pub fn f63(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f78(v0, v1)
}

pub fn f64(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m198 = f79(Fn16::F80(v1), None::<Bytes>, v0);
    match m198 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f65(v0: Fn13, p202: List<(Bytes, Bytes)>, p203: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m201 = (p202, p203);
    {
        let mut m201 = m201;
        loop {
            let step = {
                let (v1, v2) = m201;
                let m199 = v2;
                match m199.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c200: Fn13 = v0.clone();
                            c200.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f81(v0, v1)
}

pub fn f67(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f82(v0)
}

pub fn f68(v0: Fn14, p207: List<Bytes>, p208: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m206 = (p207, p208);
    {
        let mut m206 = m206;
        loop {
            let step = {
                let (v1, v2) = m206;
                let m204 = v2;
                match m204.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c205: Fn14 = v0.clone();
                            c205.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f83(v1)?, v0))
}

pub fn f70(v0: Bytes, v1: u64) -> Bytes {
    let m212 = {
        let a209 = v0;
        let a210 = v1;
        let a211 = 1u64;
        slice_bytes(a209, a210, a211)
    };
    match m212 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f71(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f84(v1, v0, v2)
}

pub fn f72(v0: Bytes, v1: u64) -> R<u64> {
    let m213 = f46(v0, v1);
    match m213 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a258 = {
                let a234 = {
                    let a222 = {
                        let a216 = {
                            let a214 = v3;
                            let a215 = 128u8;
                            fixed_u8::bit_and(a214, a215)
                        };
                        let a217 = 128u8;
                        if equal(a216, a217) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a223 = {
                        let a220 = {
                            let a218 = v3;
                            let a219 = 64u8;
                            fixed_u8::bit_and(a218, a219)
                        };
                        let a221 = 64u8;
                        if equal(a220, a221) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a222, a223)?
                };
                let a235 = {
                    let a232 = {
                        let a226 = {
                            let a224 = v3;
                            let a225 = 32u8;
                            fixed_u8::bit_and(a224, a225)
                        };
                        let a227 = 32u8;
                        if equal(a226, a227) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a233 = {
                        let a230 = {
                            let a228 = v3;
                            let a229 = 16u8;
                            fixed_u8::bit_and(a228, a229)
                        };
                        let a231 = 16u8;
                        if equal(a230, a231) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a232, a233)?
                };
                nat_add(a234, a235)?
            };
            let a259 = {
                let a256 = {
                    let a244 = {
                        let a238 = {
                            let a236 = v3;
                            let a237 = 8u8;
                            fixed_u8::bit_and(a236, a237)
                        };
                        let a239 = 8u8;
                        if equal(a238, a239) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a245 = {
                        let a242 = {
                            let a240 = v3;
                            let a241 = 4u8;
                            fixed_u8::bit_and(a240, a241)
                        };
                        let a243 = 4u8;
                        if equal(a242, a243) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a244, a245)?
                };
                let a257 = {
                    let a254 = {
                        let a248 = {
                            let a246 = v3;
                            let a247 = 2u8;
                            fixed_u8::bit_and(a246, a247)
                        };
                        let a249 = 2u8;
                        if equal(a248, a249) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a255 = {
                        let a252 = {
                            let a250 = v3;
                            let a251 = 1u8;
                            fixed_u8::bit_and(a250, a251)
                        };
                        let a253 = 1u8;
                        if equal(a252, a253) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a254, a255)?
                };
                nat_add(a256, a257)?
            };
            nat_add(a258, a259)
        }
    }
}

pub fn f73(v0: u64) -> Bytes {
    let a260 = v0;
    let a261 = 34u64;
    if nat_eq(a260, a261) {
        f44(Str::lit("\\\""))
    } else {
        let a262 = v0;
        let a263 = 92u64;
        if nat_eq(a262, a263) {
            f44(Str::lit("\\\\"))
        } else {
            let a264 = v0;
            let a265 = 8u64;
            if nat_eq(a264, a265) {
                f44(Str::lit("\\b"))
            } else {
                let a266 = v0;
                let a267 = 12u64;
                if nat_eq(a266, a267) {
                    f44(Str::lit("\\f"))
                } else {
                    let a268 = v0;
                    let a269 = 10u64;
                    if nat_eq(a268, a269) {
                        f44(Str::lit("\\n"))
                    } else {
                        let a270 = v0;
                        let a271 = 13u64;
                        if nat_eq(a270, a271) {
                            f44(Str::lit("\\r"))
                        } else {
                            let a272 = v0;
                            let a273 = 9u64;
                            if nat_eq(a272, a273) {
                                f44(Str::lit("\\t"))
                            } else {
                                f43(f44(Str::lit("\\u00")), f85(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h274, _) = f86(Fn17::F87(v1, v0), {
        let a275 = v1;
        let a276 = v2;
        nat_sub(a275, a276)
    }, v2)?;
    Ok(h274)
}

pub fn f75(v0: Fn15, p281: u64, p282: List<Bytes>) -> R<(List<Bytes>, bool)> {
    let m280 = (p281, p282);
    {
        let mut m280 = m280;
        loop {
            let step = {
                let (v1, v2) = m280;
                let m277 = v1;
                if m277 == 0 {
                    Err::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m277 - 1;
                    let m279 = {
                        let c278: Fn15 = v0.clone();
                        c278.apply(v2.clone())?
                    };
                    match m279 {
                        None => {
                            Err::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m280 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: List<Bytes>) -> R<Option<List<Bytes>>> {
    f88(v0)
}

pub fn f77(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f78(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m286 = {
        let a284 = v0.clone();
        let a285 = {
            let (h283, _) = v1.clone();
            h283
        };
        index_bytes(a284, a285)
    };
    match m286 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a288 = {
                    let (h287, _) = v1.clone();
                    h287
                };
                let a289 = 1u64;
                nat_add(a288, a289)?
            }, {
                let a292 = {
                    let (_, h290) = v1;
                    h290
                };
                let a293 = f89(v0, {
                    let (h291, _) = v1;
                    h291
                }, v2)?;
                append_bytes(a292, a293)
            })))
        }
    }
}

pub fn f79(v0: Fn16, p297: Option<Bytes>, p298: List<Bytes>) -> Option<Bytes> {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v1, v2) = m296;
                let m294 = v2;
                match m294.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c295: Fn16 = v0.clone();
                            c295.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m299 = v1;
    match m299 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a302 = {
                    let a300 = v3;
                    let a301 = v0;
                    append_bytes(a300, a301)
                };
                let a303 = v2;
                append_bytes(a302, a303)
            })
        }
    }
}

pub fn f81(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m304 = v0.clone();
    match m304.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a307 = {
                let (h305, _) = v2;
                h305
            };
            let a308 = {
                let (h306, _) = v1.clone();
                h306
            };
            if equal(a307, a308) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f82(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f90(f91(Fn18::F92, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m313 = {
        let (h309, _) = f93(Fn19::F94, {
            let a311 = {
                let a310 = v0;
                length_list(a310)
            };
            let a312 = 1u64;
            nat_add(a311, a312)?
        }, v1)?;
        h309
    };
    match m313.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f83(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a318 = {
        let a315 = f49({
            let (h314, _) = v0;
            h314
        })?;
        let a316 = Bytes::lit(&[0x3a]);
        append_bytes(a315, a316)
    };
    let a319 = {
        let (_, h317) = v0;
        h317
    };
    Ok(append_bytes(a318, a319))
}

pub fn f84(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h320, _) = v2;
        h320
    };
    let a321 = v3;
    let a322 = 0u64;
    if nat_eq(a321, a322) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a323 = v3;
            let a324 = v1;
            let a325 = 0u64;
            nat_quot(a323, a324, a325)
        }, {
            let a330 = f70(v0, {
                let a326 = v3;
                let a327 = v1;
                let a328 = 0u64;
                nat_rem(a326, a327, a328)
            });
            let a331 = {
                let (_, h329) = v2;
                h329
            };
            append_bytes(a330, a331)
        }))
    }
}

pub fn f85(v0: u64) -> Bytes {
    f43(f95(v0, 16u64), f96(v0))
}

pub fn f86(v0: Fn17, p336: u64, p337: u64) -> R<(u64, bool)> {
    let m335 = (p336, p337);
    {
        let mut m335 = m335;
        loop {
            let step = {
                let (v1, v2) = m335;
                let m332 = v1;
                if m332 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m332 - 1;
                    let m334 = {
                        let c333: Fn17 = v0.clone();
                        c333.apply(v2)?
                    };
                    match m334 {
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
                Ok(next) => { m335 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f97(v1, v0, v2)
}

pub fn f88(v0: List<Bytes>) -> R<Option<List<Bytes>>> {
    let m338 = v0.clone();
    match m338.uncons() {
        None => {
            Ok(None::<List<Bytes>>)
        }
        Some((_, v2)) => {
            let m339 = v2;
            match m339.uncons() {
                None => {
                    Ok(None::<List<Bytes>>)
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f98(Fn20::F99, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m341 = {
                        let (h340, _) = v5;
                        h340
                    };
                    match m341 {
                        None => {
                            Ok(Some(f62({
                                let (_, h343) = v5;
                                h343
                            })?))
                        }
                        Some(v6) => {
                            Ok(Some(f62(List::cons(v6, {
                                let (_, h342) = v5;
                                h342
                            }))?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f89(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a388 = {
            let a364 = {
                let a352 = {
                    let a346 = {
                        let a344 = v3;
                        let a345 = 128u8;
                        fixed_u8::bit_and(a344, a345)
                    };
                    let a347 = 128u8;
                    if equal(a346, a347) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a353 = {
                    let a350 = {
                        let a348 = v3;
                        let a349 = 64u8;
                        fixed_u8::bit_and(a348, a349)
                    };
                    let a351 = 64u8;
                    if equal(a350, a351) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a352, a353)?
            };
            let a365 = {
                let a362 = {
                    let a356 = {
                        let a354 = v3;
                        let a355 = 32u8;
                        fixed_u8::bit_and(a354, a355)
                    };
                    let a357 = 32u8;
                    if equal(a356, a357) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a363 = {
                    let a360 = {
                        let a358 = v3;
                        let a359 = 16u8;
                        fixed_u8::bit_and(a358, a359)
                    };
                    let a361 = 16u8;
                    if equal(a360, a361) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a362, a363)?
            };
            nat_add(a364, a365)?
        };
        let a389 = {
            let a386 = {
                let a374 = {
                    let a368 = {
                        let a366 = v3;
                        let a367 = 8u8;
                        fixed_u8::bit_and(a366, a367)
                    };
                    let a369 = 8u8;
                    if equal(a368, a369) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a375 = {
                    let a372 = {
                        let a370 = v3;
                        let a371 = 4u8;
                        fixed_u8::bit_and(a370, a371)
                    };
                    let a373 = 4u8;
                    if equal(a372, a373) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a374, a375)?
            };
            let a387 = {
                let a384 = {
                    let a378 = {
                        let a376 = v3;
                        let a377 = 2u8;
                        fixed_u8::bit_and(a376, a377)
                    };
                    let a379 = 2u8;
                    if equal(a378, a379) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a385 = {
                    let a382 = {
                        let a380 = v3;
                        let a381 = 1u8;
                        fixed_u8::bit_and(a380, a381)
                    };
                    let a383 = 1u8;
                    if equal(a382, a383) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a384, a385)?
            };
            nat_add(a386, a387)?
        };
        nat_add(a388, a389)?
    };
    let a390 = v4;
    let a391 = 34u64;
    if nat_eq(a390, a391) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a392 = v4;
        let a393 = 92u64;
        if nat_eq(a392, a393) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a394 = v4;
            let a395 = 8u64;
            if nat_eq(a394, a395) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a396 = v4;
                let a397 = 12u64;
                if nat_eq(a396, a397) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a398 = v4;
                    let a399 = 10u64;
                    if nat_eq(a398, a399) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a400 = v4;
                        let a401 = 13u64;
                        if nat_eq(a400, a401) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a402 = v4;
                            let a403 = 9u64;
                            if nat_eq(a402, a403) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a404 = v4;
                                let a405 = 32u64;
                                if nat_lt(a404, a405) {
                                    let a406 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a407 = f100(v2)?;
                                    Ok(append_bytes(a406, a407))
                                } else {
                                    Ok(f47(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f90(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f101(Fn21::F102, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f91(v0: Fn18, p411: List<List<(Bytes, Bytes)>>, p412: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m410 = (p411, p412);
    {
        let mut m410 = m410;
        loop {
            let step = {
                let (v1, v2) = m410;
                let m408 = v2;
                match m408.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c409: Fn18 = v0.clone();
                            c409.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f93(v0: Fn19, p417: u64, p418: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m416 = (p417, p418);
    {
        let mut m416 = m416;
        loop {
            let step = {
                let (v1, v2) = m416;
                let m413 = v1;
                if m413 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m413 - 1;
                    let m415 = {
                        let c414: Fn19 = v0.clone();
                        c414.apply(v2.clone())?
                    };
                    match m415 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m416 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f103(v0)
}

pub fn f95(v0: u64, v1: u64) -> Bytes {
    f96({
        let a419 = v0;
        let a420 = v1;
        let a421 = 0u64;
        nat_quot(a419, a420, a421)
    })
}

pub fn f96(v0: u64) -> Bytes {
    f47(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a422 = v0;
        let a423 = 16u64;
        let a424 = 0u64;
        nat_rem(a422, a423, a424)
    }, 1u64)
}

pub fn f97(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a425 = v2;
    let a426 = v1;
    if nat_lt(a425, a426) {
        let v3: u64 = f72(v0, v2)?;
        let a427 = v3;
        let a428 = 32u64;
        if nat_lt(a427, a428) {
            Ok(None::<u64>)
        } else {
            let a429 = v3;
            let a430 = 34u64;
            if nat_eq(a429, a430) {
                Ok(None::<u64>)
            } else {
                let a431 = v3;
                let a432 = 92u64;
                if nat_eq(a431, a432) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a433 = v2;
                        let a434 = 1u64;
                        nat_add(a433, a434)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f98(v0: Fn20, p438: (Option<Bytes>, List<Bytes>), p439: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m437 = (p438, p439);
    {
        let mut m437 = m437;
        loop {
            let step = {
                let (v1, v2) = m437;
                let m435 = v2;
                match m435.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c436: Fn20 = v0.clone();
                            c436.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f104(v0, v1)
}

pub fn f100(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a484 = {
            let a460 = {
                let a448 = {
                    let a442 = {
                        let a440 = v1;
                        let a441 = 128u8;
                        fixed_u8::bit_and(a440, a441)
                    };
                    let a443 = 128u8;
                    if equal(a442, a443) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a449 = {
                    let a446 = {
                        let a444 = v1;
                        let a445 = 64u8;
                        fixed_u8::bit_and(a444, a445)
                    };
                    let a447 = 64u8;
                    if equal(a446, a447) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a448, a449)?
            };
            let a461 = {
                let a458 = {
                    let a452 = {
                        let a450 = v1;
                        let a451 = 32u8;
                        fixed_u8::bit_and(a450, a451)
                    };
                    let a453 = 32u8;
                    if equal(a452, a453) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a459 = {
                    let a456 = {
                        let a454 = v1;
                        let a455 = 16u8;
                        fixed_u8::bit_and(a454, a455)
                    };
                    let a457 = 16u8;
                    if equal(a456, a457) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a458, a459)?
            };
            nat_add(a460, a461)?
        };
        let a485 = {
            let a482 = {
                let a470 = {
                    let a464 = {
                        let a462 = v1;
                        let a463 = 8u8;
                        fixed_u8::bit_and(a462, a463)
                    };
                    let a465 = 8u8;
                    if equal(a464, a465) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a471 = {
                    let a468 = {
                        let a466 = v1;
                        let a467 = 4u8;
                        fixed_u8::bit_and(a466, a467)
                    };
                    let a469 = 4u8;
                    if equal(a468, a469) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a470, a471)?
            };
            let a483 = {
                let a480 = {
                    let a474 = {
                        let a472 = v1;
                        let a473 = 2u8;
                        fixed_u8::bit_and(a472, a473)
                    };
                    let a475 = 2u8;
                    if equal(a474, a475) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a481 = {
                    let a478 = {
                        let a476 = v1;
                        let a477 = 1u8;
                        fixed_u8::bit_and(a476, a477)
                    };
                    let a479 = 1u8;
                    if equal(a478, a479) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a480, a481)?
            };
            nat_add(a482, a483)?
        };
        nat_add(a484, a485)?
    };
    let a492 = f70(f105(), {
        let a486 = v2;
        let a487 = 16u64;
        let a488 = 0u64;
        nat_quot(a486, a487, a488)
    });
    let a493 = f70(f105(), {
        let a489 = v2;
        let a490 = 16u64;
        let a491 = 0u64;
        nat_rem(a489, a490, a491)
    });
    Ok(append_bytes(a492, a493))
}

pub fn f101(v0: Fn21, p497: List<List<(Bytes, Bytes)>>, p498: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m496 = (p497, p498);
    {
        let mut m496 = m496;
        loop {
            let step = {
                let (v1, v2) = m496;
                let m494 = v2;
                match m494.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c495: Fn21 = v0.clone();
                            c495.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f103(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a500 = {
        let a499 = v0.clone();
        length_list(a499)
    };
    let a501 = 1u64;
    if nat_le(a500, a501) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f106(v0)?))
    }
}

pub fn f104(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m503 = {
        let (h502, _) = v0;
        h502
    };
    match m503 {
        None => {
            (Some(v1), {
                let (_, h504) = v0;
                h504
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a505 = v2;
                let a506 = v1;
                append_bytes(a505, a506)
            }, {
                let (_, h507) = v0;
                h507
            }))
        }
    }
}

pub fn f105() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f106(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f90({
        let (_, h508) = {
            let (h509, _) = f107(Fn22::F108, {
                let a511 = {
                    let a510 = v0.clone();
                    length_list(a510)
                };
                let a512 = 1u64;
                nat_add(a511, a512)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h509
        };
        h508
    }))
}

pub fn f107(v0: Fn22, p517: u64, p518: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m516 = (p517, p518);
    {
        let mut m516 = m516;
        loop {
            let step = {
                let (v1, v2) = m516;
                let m513 = v1;
                if m513 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m513 - 1;
                    let m515 = {
                        let c514: Fn22 = v0.clone();
                        c514.apply(v2.clone())?
                    };
                    match m515 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m516 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f109(v0)
}

pub fn f109(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h519, _) = v0;
        h519
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h520) = v0;
        h520
    };
    let m521 = v1;
    match m521.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m522 = v4;
            match m522.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f110(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f110(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a527 = {
            let a525 = {
                let a523 = v0.clone();
                length_list(a523)
            };
            let a526 = {
                let a524 = v1.clone();
                length_list(a524)
            };
            nat_add(a525, a526)?
        };
        let a528 = 1u64;
        nat_add(a527, a528)?
    };
    Ok(f111({
        let (_, h529) = {
            let (_, h530) = {
                let (h531, _) = f112(Fn23::F113, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h531
            };
            h530
        };
        h529
    }))
}

pub fn f111(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f65(Fn13::F114, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f112(v0: Fn23, p536: u64, p537: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m535 = (p536, p537);
    {
        let mut m535 = m535;
        loop {
            let step = {
                let (v1, v2) = m535;
                let m532 = v1;
                if m532 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m532 - 1;
                    let m534 = {
                        let c533: Fn23 = v0.clone();
                        c533.apply(v2.clone())
                    };
                    match m534 {
                        None => {
                            Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f115(v0)
}

pub fn f114(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f115(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h538, _) = v0.clone();
        h538
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h539, _) = {
            let (_, h540) = v0.clone();
            h540
        };
        h539
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h541) = {
            let (_, h542) = v0;
            h542
        };
        h541
    };
    let m543 = v1.clone();
    match m543.uncons() {
        None => {
            let m544 = v2;
            match m544.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m545 = v2.clone();
            match m545.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f116({
                        let (h546, _) = v8.clone();
                        h546
                    }, {
                        let (h547, _) = v6.clone();
                        h547
                    }) {
                        Some((v1, (v9, List::cons(v8, v3))))
                    } else {
                        Some((v7, (v2, List::cons(v6, v3))))
                    }
                }
            }
        }
    }
}

pub fn f116(v0: Bytes, v1: Bytes) -> bool {
    let a552 = {
        let a548 = v0;
        let a549 = v1;
        compare_bytes(a548, a549)
    };
    let a553 = {
        let a550 = Bytes::lit(&[]);
        let a551 = Bytes::lit(&[0x01]);
        compare_bytes(a550, a551)
    };
    equal(a552, a553)
}
