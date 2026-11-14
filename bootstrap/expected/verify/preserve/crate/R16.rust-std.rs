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
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt8>, std::rc::Rc<Adt8>),
    C3(u64, std::rc::Rc<Adt8>),
    C4,
    C5,
    C6(std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, List<Adt10>, List<u64>, List<u64>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64, u64, u64, Adt11),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt12),
    C1(Adt12, List<Adt12>),
    C2(Adt12),
    C3(Adt12),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C3(u64, std::rc::Rc<Adt12>),
    C4,
    C5(std::rc::Rc<Adt12>),
    C6(u64),
    C7(Bytes, Bytes),
    C8(Bytes, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(List<Bytes>, u64, List<Adt14>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, bool, Adt8),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<Adt10>, List<Adt11>, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt12, List<Adt12>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, bool),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64, bool),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, bool, u64),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, u64, bool, List<Adt14>, List<Adt14>, Option<bool>),
}

#[derive(Clone)]
pub enum Fn0 {
    F9,
}

impl Fn0 {
    pub fn apply(&self, p0: Result<Bytes, Bytes>, p1: Adt6) -> R<Result<Bytes, Bytes>> {
        match self {
            Fn0::F9 => f9(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt8>, p1: Bytes) -> R<List<Adt8>> {
        match self {
            Fn1::F15 => f15(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17,
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Option<Bytes>) -> Option<Bytes> {
        match self {
            Fn2::F17 => f17(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F19,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Option<Bytes>), p1: Adt1) -> R<(u64, Option<Bytes>)> {
        match self {
            Fn3::F19 => f19(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F21(u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (bool, Option<u64>), p1: u64) -> (bool, Option<u64>) {
        match self {
            Fn4::F21(k0) => f21(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F23(u64),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt6) -> Option<Bytes> {
        match self {
            Fn5::F23(k0) => f23(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F26,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt9, p1: Adt1) -> R<Adt9> {
        match self {
            Fn6::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F29,
}

impl Fn7 {
    pub fn apply(&self, p0: Bytes, p1: Adt10) -> R<Bytes> {
        match self {
            Fn7::F29 => f29(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F35,
}

impl Fn8 {
    pub fn apply(&self, p0: (bool, Adt8), p1: Adt8) -> (bool, Adt8) {
        match self {
            Fn8::F35 => f35(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F38(u64),
    F155(u64),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Bytes>, p1: u64) -> Option<Bytes> {
        match self {
            Fn9::F38(k0) => f38(*k0, p0, p1),
            Fn9::F155(k0) => f155(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F42(u64),
}

impl Fn10 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn10::F42(k0) => f42(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F47,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt6) -> R<List<Bytes>> {
        match self {
            Fn11::F47 => f47(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F56,
    F79,
}

impl Fn12 {
    pub fn apply(&self, p0: List<u64>, p1: Adt6) -> List<u64> {
        match self {
            Fn12::F56 => f56(p0, p1),
            Fn12::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F58,
}

impl Fn13 {
    pub fn apply(&self, p0: List<u64>, p1: Adt1) -> List<u64> {
        match self {
            Fn13::F58 => f58(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F60,
}

impl Fn14 {
    pub fn apply(&self, p0: List<u64>, p1: (Bytes, u64)) -> List<u64> {
        match self {
            Fn14::F60 => f60(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F64,
}

impl Fn15 {
    pub fn apply(&self, p0: List<(Bytes, u64)>, p1: u64) -> List<(Bytes, u64)> {
        match self {
            Fn15::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F70,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Adt10>, p1: Adt10) -> List<Adt10> {
        match self {
            Fn16::F70 => f70(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F72(u64, u64),
}

impl Fn17 {
    pub fn apply(&self, p0: Adt13) -> R<Option<Adt13>> {
        match self {
            Fn17::F72(k0, k1) => f72(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F81(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: (Adt6, Option<Adt6>)) -> Option<(Adt6, Option<Adt6>)> {
        match self {
            Fn18::F81(k0) => f81(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F84,
}

impl Fn19 {
    pub fn apply(&self, p0: (Adt6, List<Adt6>)) -> Option<(Adt6, List<Adt6>)> {
        match self {
            Fn19::F84 => f84(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F86(Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Bytes) -> (bool, Bytes) {
        match self {
            Fn20::F86(k0) => f86(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F89,
}

impl Fn21 {
    pub fn apply(&self, p0: (Adt6, List<(Bytes, Adt6)>)) -> Option<(Adt6, List<(Bytes, Adt6)>)> {
        match self {
            Fn21::F89 => f89(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F91,
}

impl Fn22 {
    pub fn apply(&self, p0: Adt6, p1: (Bytes, Adt6)) -> Adt6 {
        match self {
            Fn22::F91 => f91(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F102,
}

impl Fn23 {
    pub fn apply(&self, p0: Adt15, p1: u64) -> Adt15 {
        match self {
            Fn23::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F108,
    F109,
    F154,
}

impl Fn24 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn24::F108 => f108(p0, p1),
            Fn24::F109 => f109(p0, p1),
            Fn24::F154 => f154(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F111,
}

impl Fn25 {
    pub fn apply(&self, p0: List<(Bytes, u64)>, p1: (Bytes, u64)) -> List<(Bytes, u64)> {
        match self {
            Fn25::F111 => f111(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F114,
}

impl Fn26 {
    pub fn apply(&self, p0: List<List<(Bytes, u64)>>, p1: (Bytes, u64)) -> List<List<(Bytes, u64)>> {
        match self {
            Fn26::F114 => f114(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F116,
}

impl Fn27 {
    pub fn apply(&self, p0: List<List<(Bytes, u64)>>) -> R<Option<List<List<(Bytes, u64)>>>> {
        match self {
            Fn27::F116 => f116(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F121,
}

impl Fn28 {
    pub fn apply(&self, p0: Adt12, p1: Adt12) -> Adt12 {
        match self {
            Fn28::F121 => f121(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F133(u64, Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: Adt16) -> R<Option<Adt16>> {
        match self {
            Fn29::F133(k0, k1) => f133(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F139,
}

impl Fn30 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt6) -> List<Adt6> {
        match self {
            Fn30::F139 => f139(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F145,
}

impl Fn31 {
    pub fn apply(&self, p0: List<Adt8>, p1: Adt8) -> List<Adt8> {
        match self {
            Fn31::F145 => f145(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F147(Option<Bytes>),
    F285,
}

impl Fn32 {
    pub fn apply(&self, p0: List<(Bytes, Adt8)>, p1: (Bytes, Adt8)) -> R<List<(Bytes, Adt8)>> {
        match self {
            Fn32::F147(k0) => f147(k0.clone(), p0, p1),
            Fn32::F285 => Ok(f285(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F149,
}

impl Fn33 {
    pub fn apply(&self, p0: List<Adt8>, p1: (Bytes, Adt8)) -> List<Adt8> {
        match self {
            Fn33::F149 => f149(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F152(u64),
}

impl Fn34 {
    pub fn apply(&self, p0: (u64, List<u64>)) -> R<Option<(u64, List<u64>)>> {
        match self {
            Fn34::F152(k0) => f152(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F167,
}

impl Fn35 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn35::F167 => f167(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F170,
}

impl Fn36 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt4) -> R<List<Bytes>> {
        match self {
            Fn36::F170 => f170(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F172,
}

impl Fn37 {
    pub fn apply(&self, p0: List<List<(Bytes, u64)>>, p1: List<(Bytes, u64)>) -> List<List<(Bytes, u64)>> {
        match self {
            Fn37::F172 => f172(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F175,
}

impl Fn38 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn38::F175 => f175(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F177(Bytes, u64),
}

impl Fn39 {
    pub fn apply(&self, p0: Adt18, p1: Bytes) -> R<Adt18> {
        match self {
            Fn39::F177(k0, k1) => f177(k0.clone(), *k1, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn40 {
    F179(Bytes),
}

impl Fn40 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn40::F179(k0) => f179(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn41 {
    F181,
    F182,
}

impl Fn41 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn41::F181 => f181(p0, p1),
            Fn41::F182 => f182(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn42 {
    F185,
}

impl Fn42 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn42::F185 => f185(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn43 {
    F187,
    F219,
}

impl Fn43 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn43::F187 => f187(p0, p1),
            Fn43::F219 => f219(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn44 {
    F192(Bytes),
    F238(Bytes),
    F243(Bytes),
    F257(u64, Bytes),
}

impl Fn44 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn44::F192(k0) => f192(k0.clone(), p0),
            Fn44::F238(k0) => f238(k0.clone(), p0),
            Fn44::F243(k0) => f243(k0.clone(), p0),
            Fn44::F257(k0, k1) => f257(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn45 {
    F194(Bytes),
}

impl Fn45 {
    pub fn apply(&self, p0: Adt19) -> R<Option<Adt19>> {
        match self {
            Fn45::F194(k0) => f194(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn46 {
    F198(u64, Bytes),
}

impl Fn46 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn46::F198(k0, k1) => f198(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn47 {
    F207,
}

impl Fn47 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>, p1: (Bytes, Adt8)) -> List<List<(Bytes, Adt8)>> {
        match self {
            Fn47::F207 => f207(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn48 {
    F209,
}

impl Fn48 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
        match self {
            Fn48::F209 => f209(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn49 {
    F214,
}

impl Fn49 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt2) -> R<List<Bytes>> {
        match self {
            Fn49::F214 => f214(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn50 {
    F241,
}

impl Fn50 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>, p1: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
        match self {
            Fn50::F241 => f241(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn51 {
    F246,
}

impl Fn51 {
    pub fn apply(&self, p0: (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)) -> R<Option<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)>> {
        match self {
            Fn51::F246 => f246(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn52 {
    F248(Bytes),
}

impl Fn52 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn52::F248(k0) => f248(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn53 {
    F251,
}

impl Fn53 {
    pub fn apply(&self, p0: Adt21) -> R<Option<Adt21>> {
        match self {
            Fn53::F251 => f251(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn54 {
    F253,
}

impl Fn54 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn54::F253 => f253(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn55 {
    F263(u64),
}

impl Fn55 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn55::F263(k0) => f263(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn56 {
    F271,
}

impl Fn56 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
        match self {
            Fn56::F271 => f271(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn57 {
    F278,
}

impl Fn57 {
    pub fn apply(&self, p0: (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))) -> Option<(List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))> {
        match self {
            Fn57::F278 => f278(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn58 {
    F283,
}

impl Fn58 {
    pub fn apply(&self, p0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
        match self {
            Fn58::F283 => f283(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<Bytes, Adt7>> {
    if f1(match v0.clone() {
        Adt0::C0(h1, _, _, _, _) => {
            h1
        }
    }, f2(Str::lit("lexlean/core-module/2"))) {
        let v1: u64 = {
            let a3 = match v0.clone() {
                Adt0::C0(_, _, h2, _, _) => {
                    h2
                }
            };
            length_list(a3)
        };
        let m7 = f3(List::cons(f4(match v0.clone() {
            Adt0::C0(_, _, h4, _, _) => {
                h4
            }
        })?, List::cons(f5(match v0.clone() {
            Adt0::C0(_, _, _, h5, _) => {
                h5
            }
        }, v1), List::cons(f6(match v0.clone() {
            Adt0::C0(_, _, _, _, h6) => {
                h6
            }
        }, v1), List::<Option<Bytes>>::nil()))));
        match m7 {
            None => {
                let v3: Result<Bytes, Bytes> = f8(Fn0::F9, Ok::<Bytes, Bytes>(f2(Str::lit(""))), match v0.clone() {
                    Adt0::C0(_, _, _, _, h8) => {
                        h8
                    }
                })?;
                let m9 = v3;
                match m9 {
                    Ok(v5) => {
                        let m10 = f10(v0.clone())?;
                        match m10 {
                            Ok(v7) => {
                                Ok(Ok::<Bytes, Adt7>(f11(f11(f11(f11(f12(Adt8::C2(std::rc::Rc::new(Adt8::C1(f2(Str::lit("imports ")))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(f13(f2(Str::lit("[")), f2(Str::lit("]")), f14(Fn1::F15, List::<Adt8>::nil(), match v0 {
                                    Adt0::C0(_, h11, _, _, _) => {
                                        h11
                                    }
                                })?)), std::rc::Rc::new(Adt8::C0)))))?, f2(Str::lit("\u{a}\u{a}"))), v5), f2(Str::lit("\u{a}"))), v7)))
                            }
                            Err(v6) => {
                                Ok(f7(v6))
                            }
                        }
                    }
                    Err(v4) => {
                        Ok(f7(v4))
                    }
                }
            }
            Some(v2) => {
                Ok(f7(v2))
            }
        }
    } else {
        Ok(f7(f11(f11(f2(Str::lit("the notation names the schema `lexlean/core-module/2`, not `")), match v0 {
            Adt0::C0(h12, _, _, _, _) => {
                h12
            }
        }), f2(Str::lit("`")))))
    }
}

pub fn f1(v0: Bytes, v1: Bytes) -> bool {
    let a13 = v0;
    let a14 = v1;
    equal(a13, a14)
}

pub fn f2(v0: Str) -> Bytes {
    let a15 = v0;
    utf8_encode(a15)
}

pub fn f3(v0: List<Option<Bytes>>) -> Option<Bytes> {
    f16(Fn2::F17, None::<Bytes>, v0)
}

pub fn f4(v0: List<Adt1>) -> R<Option<Bytes>> {
    let (_, h16) = f18(Fn3::F19, (0u64, None::<Bytes>), v0)?;
    Ok(h16)
}

pub fn f5(v0: List<u64>, v1: u64) -> Option<Bytes> {
    let v2: (bool, Option<u64>) = f20(Fn4::F21(v1), (true, None::<u64>), v0);
    let (h17, _) = v2;
    if h17 {
        None::<Bytes>
    } else {
        Some(f2(Str::lit("the proof nodes are not strictly increasing nodes of the module")))
    }
}

pub fn f6(v0: List<Adt6>, v1: u64) -> Option<Bytes> {
    f22(Fn5::F23(v1), None::<Bytes>, v0)
}

pub fn f7(v0: Bytes) -> Result<Bytes, Adt7> {
    Err::<Bytes, Adt7>(Adt7::C0(0u64, v0))
}

pub fn f8(v0: Fn0, p21: Result<Bytes, Bytes>, p22: List<Adt6>) -> R<Result<Bytes, Bytes>> {
    let m20 = (p21, p22);
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, v2) = m20;
                let m18 = v2;
                match m18.uncons() {
                    None => {
                        Err::<(Result<Bytes, Bytes>, List<Adt6>), R<Result<Bytes, Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<Bytes, Bytes>, List<Adt6>), R<Result<Bytes, Bytes>>>(({
                            let c19: Fn0 = v0.clone();
                            c19.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Result<Bytes, Bytes>, v1: Adt6) -> R<Result<Bytes, Bytes>> {
    let m23 = v0.clone();
    match m23 {
        Ok(v2) => {
            let m24 = f24(v1)?;
            match m24 {
                Ok(v3) => {
                    Ok(Ok::<Bytes, Bytes>(f11(f11(v2, v3), f2(Str::lit("\u{a}")))))
                }
                Err(v4) => {
                    Ok(Err::<Bytes, Bytes>(v4))
                }
            }
        }
        Err(_) => {
            Ok(v0)
        }
    }
}

pub fn f10(v0: Adt0) -> R<Result<Bytes, Bytes>> {
    let v1: Adt9 = f25(Fn6::F26, Adt9::C0(0u64, List::<Adt10>::nil(), f27(match v0.clone() {
        Adt0::C0(_, _, h25, _, _) => {
            h25
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h26) => {
            h26
        }
    })?, match v0.clone() {
        Adt0::C0(_, _, _, h27, _) => {
            h27
        }
    }, None::<Bytes>), match v0 {
        Adt0::C0(_, _, h28, _, _) => {
            h28
        }
    })?;
    let m30 = match v1.clone() {
        Adt9::C0(_, _, _, _, h29) => {
            h29
        }
    };
    match m30 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f28(Fn7::F29, f2(Str::lit("")), f30(match v1 {
                Adt9::C0(_, h31, _, _, _) => {
                    h31
                }
            }))?))
        }
        Some(v2) => {
            Ok(Err::<Bytes, Bytes>(v2))
        }
    }
}

pub fn f11(v0: Bytes, v1: Bytes) -> Bytes {
    let a32 = v0;
    let a33 = v1;
    append_bytes(a32, a33)
}

pub fn f12(v0: Adt8) -> R<Bytes> {
    f31(v0.clone(), 100u64, f32(v0)?)
}

pub fn f13(v0: Bytes, v1: Bytes, v2: List<Adt8>) -> Adt8 {
    let m34 = v2.clone();
    match m34.uncons() {
        None => {
            Adt8::C1(f33(v0, v1))
        }
        Some((_, _)) => {
            let v5: (bool, Adt8) = f34(Fn8::F35, (true, Adt8::C0), v2);
            Adt8::C6(std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C1(v0)), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C3(2u64, std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C5), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new({
                let (_, h35) = v5;
                h35
            }), std::rc::Rc::new(Adt8::C0))))))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C5), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C1(v1)), std::rc::Rc::new(Adt8::C0))))))))))
        }
    }
}

pub fn f14(v0: Fn1, p39: List<Adt8>, p40: List<Bytes>) -> R<List<Adt8>> {
    let m38 = (p39, p40);
    {
        let mut m38 = m38;
        loop {
            let step = {
                let (v1, v2) = m38;
                let m36 = v2;
                match m36.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<Bytes>), R<List<Adt8>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<Bytes>), R<List<Adt8>>>(({
                            let c37: Fn1 = v0.clone();
                            c37.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m38 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<Adt8>, v1: Bytes) -> R<List<Adt8>> {
    Ok(List::cons(Adt8::C1(f36(v1)?), v0))
}

pub fn f16(v0: Fn2, p44: Option<Bytes>, p45: List<Option<Bytes>>) -> Option<Bytes> {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m41 = v2;
                match m41.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Option<Bytes>>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Option<Bytes>>), Option<Bytes>>(({
                            let c42: Fn2 = v0.clone();
                            c42.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m43 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m46 = v0.clone();
    match m46 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f18(v0: Fn3, p50: (u64, Option<Bytes>), p51: List<Adt1>) -> R<(u64, Option<Bytes>)> {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v1, v2) = m49;
                let m47 = v2;
                match m47.uncons() {
                    None => {
                        Err::<((u64, Option<Bytes>), List<Adt1>), R<(u64, Option<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Option<Bytes>), List<Adt1>), R<(u64, Option<Bytes>)>>(({
                            let c48: Fn3 = v0.clone();
                            c48.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: (u64, Option<Bytes>), v1: Adt1) -> R<(u64, Option<Bytes>)> {
    let v2: u64 = {
        let (h52, _) = v0.clone();
        h52
    };
    Ok(({
        let a53 = v2;
        let a54 = 1u64;
        nat_add(a53, a54)?
    }, {
        let m56 = {
            let (_, h55) = v0.clone();
            h55
        };
        match m56 {
            None => {
                f37(Fn9::F38(v2), None::<Bytes>, f39(v1))
            }
            Some(_) => {
                let (_, h57) = v0;
                h57
            }
        }
    }))
}

pub fn f20(v0: Fn4, p61: (bool, Option<u64>), p62: List<u64>) -> (bool, Option<u64>) {
    let m60 = (p61, p62);
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v1, v2) = m60;
                let m58 = v2;
                match m58.uncons() {
                    None => {
                        Err::<((bool, Option<u64>), List<u64>), (bool, Option<u64>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Option<u64>), List<u64>), (bool, Option<u64>)>(({
                            let c59: Fn4 = v0.clone();
                            c59.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: u64, v1: (bool, Option<u64>), v2: u64) -> (bool, Option<u64>) {
    (f40({
        let (h63, _) = v1;
        h63
    }, f40({
        let a64 = v2;
        let a65 = v0;
        nat_lt(a64, a65)
    }, {
        let m67 = {
            let (_, h66) = v1;
            h66
        };
        match m67 {
            None => {
                true
            }
            Some(v3) => {
                let a68 = v3;
                let a69 = v2;
                nat_lt(a68, a69)
            }
        }
    })), Some(v2))
}

pub fn f22(v0: Fn5, p73: Option<Bytes>, p74: List<Adt6>) -> Option<Bytes> {
    let m72 = (p73, p74);
    {
        let mut m72 = m72;
        loop {
            let step = {
                let (v1, v2) = m72;
                let m70 = v2;
                match m70.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt6>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt6>), Option<Bytes>>(({
                            let c71: Fn5 = v0.clone();
                            c71.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m72 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: u64, v1: Option<Bytes>, v2: Adt6) -> Option<Bytes> {
    let m75 = v1.clone();
    match m75 {
        None => {
            if f41(Fn10::F42(v0), false, f43(v2.clone())) {
                Some(f11(f11(f2(Str::lit("`")), f44(f45(v2, f2(Str::lit("name"))))), f2(Str::lit("` has a root that is not a node of the module"))))
            } else {
                None::<Bytes>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f24(v0: Adt6) -> R<Result<Bytes, Bytes>> {
    let m76 = v0.clone();
    match m76 {
        Adt6::C0(_) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C1(_, _) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C2(_) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C3(_) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C4(r85) => {
            let v1 = (*r85).clone();
            let m77 = f45(v0.clone(), f2(Str::lit("kind")));
            match m77 {
                None => {
                    Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                }
                Some(v2) => {
                    let m78 = v2;
                    match m78 {
                        Adt6::C0(_) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C1(_, _) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C2(v3) => {
                            let v4: List<Bytes> = f46(Fn11::F47, List::<Bytes>::nil(), f48(f45(v0.clone(), f2(Str::lit("levels")))))?;
                            let v7: Bytes = f11(f11(f11(f11(f11(v3, f2(Str::lit(" "))), f36(f44(f45(v0.clone(), f2(Str::lit("name")))))?), {
                                let a80 = {
                                    let a79 = v4.clone();
                                    length_list(a79)
                                };
                                let a81 = 0u64;
                                if nat_eq(a80, a81) {
                                    f2(Str::lit(""))
                                } else {
                                    f11(f11(f2(Str::lit(".{")), f49(v4, f2(Str::lit(", ")))), f2(Str::lit("}")))
                                }
                            }), {
                                let m82 = f50(f45(v0.clone(), f2(Str::lit("type"))));
                                match m82 {
                                    None => {
                                        f2(Str::lit(""))
                                    }
                                    Some(v5) => {
                                        f11(f2(Str::lit(" : @")), f51(v5))
                                    }
                                }
                            }), {
                                let m83 = f50(f45(v0, f2(Str::lit("value"))));
                                match m83 {
                                    None => {
                                        f2(Str::lit(""))
                                    }
                                    Some(v6) => {
                                        f11(f2(Str::lit(" := @")), f51(v6))
                                    }
                                }
                            });
                            let v8: Adt6 = f52(v1);
                            Ok(Ok::<Bytes, Bytes>(f12({
                                let m84 = v8.clone();
                                match m84 {
                                    Adt6::C0(_) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C1(_, _) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C2(_) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C3(_) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C4(_) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C5 => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C6(_, _) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C7(_, _, _) => {
                                        Adt8::C2(std::rc::Rc::new(Adt8::C1(v7)), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C1(f2(Str::lit(" ")))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(f53(Adt6::C4(std::rc::Rc::new(v8)))?), std::rc::Rc::new(Adt8::C0))))))
                                    }
                                }
                            })?))
                        }
                        Adt6::C3(_) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C4(_) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C5 => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C6(_, _) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C7(_, _, _) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                    }
                }
            }
        }
        Adt6::C5 => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C6(_, _) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C7(_, _, _) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
    }
}

pub fn f25(v0: Fn6, p89: Adt9, p90: List<Adt1>) -> R<Adt9> {
    let m88 = (p89, p90);
    {
        let mut m88 = m88;
        loop {
            let step = {
                let (v1, v2) = m88;
                let m86 = v2;
                match m86.uncons() {
                    None => {
                        Err::<(Adt9, List<Adt1>), R<Adt9>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt9, List<Adt1>), R<Adt9>>(({
                            let c87: Fn6 = v0.clone();
                            c87.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m88 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Adt9, v1: Adt1) -> R<Adt9> {
    f54(v0, v1)
}

pub fn f27(v0: List<Adt1>, v1: List<Adt6>) -> R<List<u64>> {
    let v2: List<u64> = f55(Fn12::F56, f57(Fn13::F58, List::<u64>::nil(), v0), v1);
    Ok(f59(Fn14::F60, List::<u64>::nil(), f61(f62(f63(Fn15::F64, List::<(Bytes, u64)>::nil(), v2))?)))
}

pub fn f28(v0: Fn7, p94: Bytes, p95: List<Adt10>) -> R<Bytes> {
    let m93 = (p94, p95);
    {
        let mut m93 = m93;
        loop {
            let step = {
                let (v1, v2) = m93;
                let m91 = v2;
                match m91.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt10>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt10>), R<Bytes>>(({
                            let c92: Fn7 = v0.clone();
                            c92.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m93 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Bytes, v1: Adt10) -> R<Bytes> {
    Ok(f11(v0, f11(f12(f65(f66(List::cons(f67(f11(f11(f2(Str::lit("@")), f51(match v1.clone() {
        Adt10::C0(h96, _, _, _, _) => {
            h96
        }
    })), f2(Str::lit(" := ")))), List::cons(Adt12::C3(2u64, std::rc::Rc::new(f68(match v1 {
        Adt10::C0(_, _, _, _, h97) => {
            h97
        }
    }))), List::<Adt12>::nil()))), List::<Bytes>::nil())?)?, f2(Str::lit("\u{a}")))))
}

pub fn f30(v0: List<Adt10>) -> List<Adt10> {
    f69(Fn16::F70, List::<Adt10>::nil(), v0)
}

pub fn f31(v0: Adt8, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt13 = {
        let (h98, _) = f71(Fn17::F72(v2, v1), {
            let a99 = v2;
            let a100 = 1u64;
            nat_add(a99, a100)?
        }, Adt13::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt14::C0(0u64, false, v0), List::<Adt14>::nil())))?;
        h98
    };
    Ok(f73(match v3 {
        Adt13::C0(h101, _, _) => {
            h101
        }
    }))
}

pub fn f32(v0: Adt8) -> R<u64> {
    let m102 = v0;
    match m102 {
        Adt8::C0 => {
            Ok(1u64)
        }
        Adt8::C1(_) => {
            Ok(1u64)
        }
        Adt8::C2(r111, r112) => {
            let v2 = (*r111).clone();
            let v3 = (*r112).clone();
            let v4: u64 = f32(v2)?;
            let v5: u64 = f32(v3)?;
            let a105 = {
                let a103 = 1u64;
                let a104 = v4;
                nat_add(a103, a104)?
            };
            let a106 = v5;
            nat_add(a105, a106)
        }
        Adt8::C3(_, r113) => {
            let v7 = (*r113).clone();
            let v8: u64 = f32(v7)?;
            let a107 = 1u64;
            let a108 = v8;
            nat_add(a107, a108)
        }
        Adt8::C4 => {
            Ok(1u64)
        }
        Adt8::C5 => {
            Ok(1u64)
        }
        Adt8::C6(r114) => {
            let v9 = (*r114).clone();
            let v10: u64 = f32(v9)?;
            let a109 = 1u64;
            let a110 = v10;
            nat_add(a109, a110)
        }
    }
}

pub fn f33(v0: Bytes, v1: Bytes) -> Bytes {
    let a115 = v0;
    let a116 = v1;
    append_bytes(a115, a116)
}

pub fn f34(v0: Fn8, p120: (bool, Adt8), p121: List<Adt8>) -> (bool, Adt8) {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v1, v2) = m119;
                let m117 = v2;
                match m117.uncons() {
                    None => {
                        Err::<((bool, Adt8), List<Adt8>), (bool, Adt8)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt8), List<Adt8>), (bool, Adt8)>(({
                            let c118: Fn8 = v0.clone();
                            c118.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: (bool, Adt8), v1: Adt8) -> (bool, Adt8) {
    f74(v0, v1)
}

pub fn f36(v0: Bytes) -> R<Bytes> {
    if f75(v0.clone())? {
        Ok(v0)
    } else {
        f76(v0)
    }
}

pub fn f37(v0: Fn9, p125: Option<Bytes>, p126: List<u64>) -> Option<Bytes> {
    let m124 = (p125, p126);
    {
        let mut m124 = m124;
        loop {
            let step = {
                let (v1, v2) = m124;
                let m122 = v2;
                match m122.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<u64>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<u64>), Option<Bytes>>(({
                            let c123: Fn9 = v0.clone();
                            c123.apply(v1, v3)
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

pub fn f38(v0: u64, v1: Option<Bytes>, v2: u64) -> Option<Bytes> {
    let m127 = v1.clone();
    match m127 {
        None => {
            let a128 = v0;
            let a129 = v2;
            if nat_le(a128, a129) {
                Some(f11(f11(f11(f11(f2(Str::lit("node ")), f51(v0)), f2(Str::lit(" points to the node "))), f51(v2)), f2(Str::lit(", which is not earlier"))))
            } else {
                None::<Bytes>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f39(v0: Adt1) -> List<u64> {
    let m130 = v0;
    match m130 {
        Adt1::C0(_) => {
            List::<u64>::nil()
        }
        Adt1::C1(_) => {
            List::<u64>::nil()
        }
        Adt1::C2(_, _) => {
            List::<u64>::nil()
        }
        Adt1::C3(v1, v2) => {
            List::cons(v1, List::cons(v2, List::<u64>::nil()))
        }
        Adt1::C4(_, _, v5, v6) => {
            List::cons(v5, List::cons(v6, List::<u64>::nil()))
        }
        Adt1::C5(_, _, v9, v10) => {
            List::cons(v9, List::cons(v10, List::<u64>::nil()))
        }
        Adt1::C6(_, v12, v13, v14, _) => {
            List::cons(v12, List::cons(v13, List::cons(v14, List::<u64>::nil())))
        }
        Adt1::C7(_) => {
            List::<u64>::nil()
        }
        Adt1::C8(_) => {
            List::<u64>::nil()
        }
        Adt1::C9(_, v17) => {
            List::cons(v17, List::<u64>::nil())
        }
        Adt1::C10(_, _, _, v21) => {
            List::cons(v21, List::<u64>::nil())
        }
    }
}

pub fn f40(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f41(v0: Fn10, p134: bool, p135: List<u64>) -> bool {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v1, v2) = m133;
                let m131 = v2;
                match m131.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c132: Fn10 = v0.clone();
                            c132.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a136 = v0;
        let a137 = v2;
        nat_le(a136, a137)
    }
}

pub fn f43(v0: Adt6) -> List<u64> {
    let v1: Option<Adt6> = f45(v0.clone(), f2(Str::lit("eqn_info")));
    let v2: List<u64> = f77(f77(f77(f77(List::<u64>::nil(), f50(f45(v0.clone(), f2(Str::lit("type"))))), f50(f45(v0.clone(), f2(Str::lit("value"))))), f50(f78(v1.clone(), f2(Str::lit("type"))))), f50(f78(v1, f2(Str::lit("value")))));
    f55(Fn12::F79, v2, f48(f78(f78(f45(v0, f2(Str::lit("inductive"))), f2(Str::lit("structure"))), f2(Str::lit("fields")))))
}

pub fn f44(v0: Option<Adt6>) -> Bytes {
    let m138 = v0;
    match m138 {
        None => {
            f2(Str::lit(""))
        }
        Some(v1) => {
            let m139 = v1;
            match m139 {
                Adt6::C0(_) => {
                    f2(Str::lit(""))
                }
                Adt6::C1(_, _) => {
                    f2(Str::lit(""))
                }
                Adt6::C2(v2) => {
                    v2
                }
                Adt6::C3(_) => {
                    f2(Str::lit(""))
                }
                Adt6::C4(_) => {
                    f2(Str::lit(""))
                }
                Adt6::C5 => {
                    f2(Str::lit(""))
                }
                Adt6::C6(_, _) => {
                    f2(Str::lit(""))
                }
                Adt6::C7(_, _, _) => {
                    f2(Str::lit(""))
                }
            }
        }
    }
}

pub fn f45(v0: Adt6, v1: Bytes) -> Option<Adt6> {
    let m140 = v0;
    match m140 {
        Adt6::C0(_) => {
            None::<Adt6>
        }
        Adt6::C1(_, _) => {
            None::<Adt6>
        }
        Adt6::C2(_) => {
            None::<Adt6>
        }
        Adt6::C3(_) => {
            None::<Adt6>
        }
        Adt6::C4(r143) => {
            let v2 = (*r143).clone();
            let (_, h141) = {
                let (h142, _) = f80(Fn18::F81(v1), 1000000u64, (v2, None::<Adt6>));
                h142
            };
            h141
        }
        Adt6::C5 => {
            None::<Adt6>
        }
        Adt6::C6(_, _) => {
            None::<Adt6>
        }
        Adt6::C7(_, _, _) => {
            None::<Adt6>
        }
    }
}

pub fn f46(v0: Fn11, p147: List<Bytes>, p148: List<Adt6>) -> R<List<Bytes>> {
    let m146 = (p147, p148);
    {
        let mut m146 = m146;
        loop {
            let step = {
                let (v1, v2) = m146;
                let m144 = v2;
                match m144.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt6>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt6>), R<List<Bytes>>>(({
                            let c145: Fn11 = v0.clone();
                            c145.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: List<Bytes>, v1: Adt6) -> R<List<Bytes>> {
    let a149 = v0;
    let a150 = List::cons(f36(f44(Some(v1)))?, List::<Bytes>::nil());
    Ok(append_list(a149, a150))
}

pub fn f48(v0: Option<Adt6>) -> List<Adt6> {
    let m151 = v0;
    match m151 {
        None => {
            List::<Adt6>::nil()
        }
        Some(v1) => {
            let m152 = v1;
            match m152 {
                Adt6::C0(_) => {
                    List::<Adt6>::nil()
                }
                Adt6::C1(_, _) => {
                    List::<Adt6>::nil()
                }
                Adt6::C2(_) => {
                    List::<Adt6>::nil()
                }
                Adt6::C3(r155) => {
                    let v2 = (*r155).clone();
                    f82({
                        let (_, h153) = {
                            let (h154, _) = f83(Fn19::F84, 1000000u64, (v2, List::<Adt6>::nil()));
                            h154
                        };
                        h153
                    })
                }
                Adt6::C4(_) => {
                    List::<Adt6>::nil()
                }
                Adt6::C5 => {
                    List::<Adt6>::nil()
                }
                Adt6::C6(_, _) => {
                    List::<Adt6>::nil()
                }
                Adt6::C7(_, _, _) => {
                    List::<Adt6>::nil()
                }
            }
        }
    }
}

pub fn f49(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let (_, h156) = f85(Fn20::F86(v1), (true, f2(Str::lit(""))), v0);
    h156
}

pub fn f50(v0: Option<Adt6>) -> Option<u64> {
    let m157 = v0;
    match m157 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m158 = v1;
            match m158 {
                Adt6::C0(_) => {
                    None::<u64>
                }
                Adt6::C1(v2, v3) => {
                    if v2 {
                        None::<u64>
                    } else {
                        Some(v3)
                    }
                }
                Adt6::C2(_) => {
                    None::<u64>
                }
                Adt6::C3(_) => {
                    None::<u64>
                }
                Adt6::C4(_) => {
                    None::<u64>
                }
                Adt6::C5 => {
                    None::<u64>
                }
                Adt6::C6(_, _) => {
                    None::<u64>
                }
                Adt6::C7(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f51(v0: u64) -> Bytes {
    f87(v0)
}

pub fn f52(v0: Adt6) -> Adt6 {
    let v1: List<(Bytes, Adt6)> = {
        let (_, h159) = {
            let (h160, _) = f88(Fn21::F89, 1000000u64, (v0, List::<(Bytes, Adt6)>::nil()));
            h160
        };
        h159
    };
    f90(Fn22::F91, Adt6::C5, v1)
}

pub fn f53(v0: Adt6) -> R<Adt8> {
    let m161 = v0;
    match m161 {
        Adt6::C0(v1) => {
            Ok(Adt8::C1(if v1 {
                Bytes::lit(&[0x74, 0x72, 0x75, 0x65])
            } else {
                Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65])
            }))
        }
        Adt6::C1(v2, v3) => {
            Ok(Adt8::C1(f92(v2, v3)))
        }
        Adt6::C2(v4) => {
            Ok(Adt8::C1(f93(v4)?))
        }
        Adt6::C3(r162) => {
            let v5 = (*r162).clone();
            Ok(f13(f94(Str::lit("[")), f94(Str::lit("]")), f95(f96(v5)?)))
        }
        Adt6::C4(r163) => {
            let v6 = (*r163).clone();
            f97(f98(v6.clone())?, f99(v6)?)
        }
        Adt6::C5 => {
            Ok(Adt8::C0)
        }
        Adt6::C6(_, _) => {
            Ok(Adt8::C0)
        }
        Adt6::C7(_, _, _) => {
            Ok(Adt8::C0)
        }
    }
}

pub fn f54(v0: Adt9, v1: Adt1) -> R<Adt9> {
    let m165 = match v0.clone() {
        Adt9::C0(_, _, _, _, h164) => {
            h164
        }
    };
    match m165 {
        None => {
            let v3: u64 = match v0.clone() {
                Adt9::C0(h166, _, _, _, _) => {
                    h166
                }
            };
            let v4: (u64, List<u64>) = f100(match v0.clone() {
                Adt9::C0(_, _, h167, _, _) => {
                    h167
                }
            }, v3)?;
            let v7: bool = {
                let m169 = match v0.clone() {
                    Adt9::C0(_, _, _, h168, _) => {
                        h168
                    }
                };
                match m169.uncons() {
                    None => {
                        false
                    }
                    Some((v5, _)) => {
                        let a170 = v5;
                        let a171 = v3;
                        nat_eq(a170, a171)
                    }
                }
            };
            let v8: Adt15 = f101(Fn23::F102, Adt15::C0(match v0.clone() {
                Adt9::C0(_, h172, _, _, _) => {
                    h172
                }
            }, List::<Adt11>::nil(), 0u64), f103(f39(v1.clone())));
            let v36: Option<Bytes> = {
                let m173 = v1.clone();
                match m173 {
                    Adt1::C0(_) => {
                        None::<Bytes>
                    }
                    Adt1::C1(_) => {
                        None::<Bytes>
                    }
                    Adt1::C2(_, _) => {
                        None::<Bytes>
                    }
                    Adt1::C3(_, _) => {
                        None::<Bytes>
                    }
                    Adt1::C4(_, _, _, _) => {
                        None::<Bytes>
                    }
                    Adt1::C5(_, _, _, _) => {
                        None::<Bytes>
                    }
                    Adt1::C6(_, _, _, _, _) => {
                        None::<Bytes>
                    }
                    Adt1::C7(_) => {
                        None::<Bytes>
                    }
                    Adt1::C8(v9) => {
                        f104(v3, v9)
                    }
                    Adt1::C9(_, _) => {
                        None::<Bytes>
                    }
                    Adt1::C10(_, _, _, _) => {
                        None::<Bytes>
                    }
                }
            };
            let v39: u64 = {
                let m175 = match v8.clone() {
                    Adt15::C0(h174, _, _) => {
                        h174
                    }
                };
                match m175.uncons() {
                    None => {
                        0u64
                    }
                    Some((v37, _)) => {
                        let a177 = match v37 {
                            Adt10::C0(h176, _, _, _, _) => {
                                h176
                            }
                        };
                        let a178 = 1u64;
                        nat_add(a177, a178)?
                    }
                }
            };
            Ok(Adt9::C0({
                let a179 = v3;
                let a180 = 1u64;
                nat_add(a179, a180)?
            }, List::cons(Adt10::C0(v3, v39, {
                let a182 = match v8.clone() {
                    Adt15::C0(_, _, h181) => {
                        h181
                    }
                };
                let a183 = 1u64;
                nat_add(a182, a183)?
            }, {
                let (h184, _) = v4;
                h184
            }, f105(f106(v1, match v8.clone() {
                Adt15::C0(_, h185, _) => {
                    h185
                }
            })?, v7)), match v8 {
                Adt15::C0(h186, _, _) => {
                    h186
                }
            }), {
                let (_, h187) = v4;
                h187
            }, if v7 {
                let m189 = match v0 {
                    Adt9::C0(_, _, _, h188, _) => {
                        h188
                    }
                };
                match m189.uncons() {
                    None => {
                        List::<u64>::nil()
                    }
                    Some((_, v41)) => {
                        v41
                    }
                }
            } else {
                match v0 {
                    Adt9::C0(_, _, _, h190, _) => {
                        h190
                    }
                }
            }, v36))
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f55(v0: Fn12, p194: List<u64>, p195: List<Adt6>) -> List<u64> {
    let m193 = (p194, p195);
    {
        let mut m193 = m193;
        loop {
            let step = {
                let (v1, v2) = m193;
                let m191 = v2;
                match m191.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt6>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt6>), List<u64>>(({
                            let c192: Fn12 = v0.clone();
                            c192.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: List<u64>, v1: Adt6) -> List<u64> {
    f107(Fn24::F108, v0, f43(v1))
}

pub fn f57(v0: Fn13, p199: List<u64>, p200: List<Adt1>) -> List<u64> {
    let m198 = (p199, p200);
    {
        let mut m198 = m198;
        loop {
            let step = {
                let (v1, v2) = m198;
                let m196 = v2;
                match m196.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt1>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt1>), List<u64>>(({
                            let c197: Fn13 = v0.clone();
                            c197.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: List<u64>, v1: Adt1) -> List<u64> {
    f107(Fn24::F109, v0, f39(v1))
}

pub fn f59(v0: Fn14, p204: List<u64>, p205: List<(Bytes, u64)>) -> List<u64> {
    let m203 = (p204, p205);
    {
        let mut m203 = m203;
        loop {
            let step = {
                let (v1, v2) = m203;
                let m201 = v2;
                match m201.uncons() {
                    None => {
                        Err::<(List<u64>, List<(Bytes, u64)>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<(Bytes, u64)>), List<u64>>(({
                            let c202: Fn14 = v0.clone();
                            c202.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: List<u64>, v1: (Bytes, u64)) -> List<u64> {
    List::cons({
        let (_, h206) = v1;
        h206
    }, v0)
}

pub fn f61(v0: List<(Bytes, u64)>) -> List<(Bytes, u64)> {
    f110(Fn25::F111, List::<(Bytes, u64)>::nil(), v0)
}

pub fn f62(v0: List<(Bytes, u64)>) -> R<List<(Bytes, u64)>> {
    let v1: List<List<(Bytes, u64)>> = f112(f113(Fn26::F114, List::<List<(Bytes, u64)>>::nil(), v0.clone()));
    let m211 = {
        let (h207, _) = f115(Fn27::F116, {
            let a209 = {
                let a208 = v0;
                length_list(a208)
            };
            let a210 = 1u64;
            nat_add(a209, a210)?
        }, v1)?;
        h207
    };
    match m211.uncons() {
        None => {
            Ok(List::<(Bytes, u64)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f63(v0: Fn15, p215: List<(Bytes, u64)>, p216: List<u64>) -> List<(Bytes, u64)> {
    let m214 = (p215, p216);
    {
        let mut m214 = m214;
        loop {
            let step = {
                let (v1, v2) = m214;
                let m212 = v2;
                match m212.uncons() {
                    None => {
                        Err::<(List<(Bytes, u64)>, List<u64>), List<(Bytes, u64)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, u64)>, List<u64>), List<(Bytes, u64)>>(({
                            let c213: Fn15 = v0.clone();
                            c213.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<(Bytes, u64)>, v1: u64) -> List<(Bytes, u64)> {
    List::cons((f117(v1), v1), v0)
}

pub fn f65(v0: Adt12, v1: List<Bytes>) -> R<Adt8> {
    let m217 = v0;
    match m217 {
        Adt12::C0 => {
            Ok(Adt8::C0)
        }
        Adt12::C1(v2) => {
            Ok(Adt8::C1(v2))
        }
        Adt12::C2(r218, r219) => {
            let v3 = (*r218).clone();
            let v4 = (*r219).clone();
            Ok(Adt8::C2(std::rc::Rc::new(f65(v3, v1.clone())?), std::rc::Rc::new(f65(v4, v1)?)))
        }
        Adt12::C3(v5, r220) => {
            let v6 = (*r220).clone();
            Ok(Adt8::C3(v5, std::rc::Rc::new(f65(v6, v1)?)))
        }
        Adt12::C4 => {
            Ok(Adt8::C4)
        }
        Adt12::C5(r221) => {
            let v7 = (*r221).clone();
            Ok(Adt8::C6(std::rc::Rc::new(f65(v7, v1)?)))
        }
        Adt12::C6(v8) => {
            Ok(Adt8::C1(f118(v1, v8)?))
        }
        Adt12::C7(v9, v10) => {
            Ok(Adt8::C1(if f40(f75(v9.clone())?, !f119(v9.clone(), v1)) {
                f11(v9, v10)
            } else {
                f11(f11(f2(Str::lit("const ")), f76(v9)?), v10)
            }))
        }
        Adt12::C8(v11, r222) => {
            let v12 = (*r222).clone();
            f65(v12, List::cons(v11, v1))
        }
    }
}

pub fn f66(v0: List<Adt12>) -> Adt12 {
    f120(Fn28::F121, Adt12::C0, f122(v0))
}

pub fn f67(v0: Bytes) -> Adt12 {
    Adt12::C1(v0)
}

pub fn f68(v0: Adt11) -> Adt12 {
    let m223 = v0;
    match m223 {
        Adt11::C0(v1) => {
            v1
        }
        Adt11::C1(v2, v3) => {
            f123(v2, v3)
        }
        Adt11::C2(v4) => {
            v4
        }
        Adt11::C3(v5) => {
            v5
        }
    }
}

pub fn f69(v0: Fn16, p227: List<Adt10>, p228: List<Adt10>) -> List<Adt10> {
    let m226 = (p227, p228);
    {
        let mut m226 = m226;
        loop {
            let step = {
                let (v1, v2) = m226;
                let m224 = v2;
                match m224.uncons() {
                    None => {
                        Err::<(List<Adt10>, List<Adt10>), List<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt10>, List<Adt10>), List<Adt10>>(({
                            let c225: Fn16 = v0.clone();
                            c225.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Adt10>, v1: Adt10) -> List<Adt10> {
    List::cons(v1, v0)
}

pub fn f71(v0: Fn17, p233: u64, p234: Adt13) -> R<(Adt13, bool)> {
    let m232 = (p233, p234);
    {
        let mut m232 = m232;
        loop {
            let step = {
                let (v1, v2) = m232;
                let m229 = v1;
                if m229 == 0 {
                    Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m229 - 1;
                    let m231 = {
                        let c230: Fn17 = v0.clone();
                        c230.apply(v2.clone())?
                    };
                    match m231 {
                        None => {
                            Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt13), R<(Adt13, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    f124(v1, v0, v2)
}

pub fn f73(v0: List<Bytes>) -> Bytes {
    f125(f126(v0))
}

pub fn f74(v0: (bool, Adt8), v1: Adt8) -> (bool, Adt8) {
    let (h235, _) = v0;
    if h235 {
        (false, Adt8::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt8::C0)))
    } else {
        (false, Adt8::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C1(f94(Str::lit(",")))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C4), std::rc::Rc::new({
            let (_, h236) = v0;
            h236
        })))))))
    }
}

pub fn f75(v0: Bytes) -> R<bool> {
    Ok(f40({
        let a238 = 0u64;
        let a239 = {
            let a237 = v0.clone();
            length_bytes(a237)
        };
        nat_lt(a238, a239)
    }, f40(f127(v0.clone())?, f40(f128(v0.clone())?, f40(!f129(v0.clone(), f2(Str::lit("."))), f40(!f1(v0.clone(), f2(Str::lit("_"))), f40(!f119(v0.clone(), f130()), !f131(v0)?)))))))
}

pub fn f76(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a240 = v0.clone();
        length_bytes(a240)
    };
    let v2: Adt16 = {
        let (h241, _) = f132(Fn29::F133(v1, v0), {
            let a242 = v1;
            let a243 = 1u64;
            nat_add(a242, a243)?
        }, Adt16::C0(0u64, List::<Bytes>::nil()))?;
        h241
    };
    Ok(f134(f134(f135(Str::lit("\"")), f136(match v2 {
        Adt16::C0(_, h244) => {
            h244
        }
    })), f135(Str::lit("\""))))
}

pub fn f77(v0: List<u64>, v1: Option<u64>) -> List<u64> {
    let m245 = v1;
    match m245 {
        None => {
            v0
        }
        Some(v2) => {
            let a246 = v0;
            let a247 = List::cons(v2, List::<u64>::nil());
            append_list(a246, a247)
        }
    }
}

pub fn f78(v0: Option<Adt6>, v1: Bytes) -> Option<Adt6> {
    let m248 = v0;
    match m248 {
        None => {
            None::<Adt6>
        }
        Some(v2) => {
            f45(v2, v1)
        }
    }
}

pub fn f79(v0: List<u64>, v1: Adt6) -> List<u64> {
    f77(v0, f50(f45(v1, f2(Str::lit("auto_param")))))
}

pub fn f80(v0: Fn18, p253: u64, p254: (Adt6, Option<Adt6>)) -> ((Adt6, Option<Adt6>), bool) {
    let m252 = (p253, p254);
    {
        let mut m252 = m252;
        loop {
            let step = {
                let (v1, v2) = m252;
                let m249 = v1;
                if m249 == 0 {
                    Err::<(u64, (Adt6, Option<Adt6>)), ((Adt6, Option<Adt6>), bool)>((v2, false))
                } else {
                    let v3 = m249 - 1;
                    let m251 = {
                        let c250: Fn18 = v0.clone();
                        c250.apply(v2.clone())
                    };
                    match m251 {
                        None => {
                            Err::<(u64, (Adt6, Option<Adt6>)), ((Adt6, Option<Adt6>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt6, Option<Adt6>)), ((Adt6, Option<Adt6>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: (Adt6, Option<Adt6>)) -> Option<(Adt6, Option<Adt6>)> {
    f137(v0, v1)
}

pub fn f82(v0: List<Adt6>) -> List<Adt6> {
    f138(Fn30::F139, List::<Adt6>::nil(), v0)
}

pub fn f83(v0: Fn19, p259: u64, p260: (Adt6, List<Adt6>)) -> ((Adt6, List<Adt6>), bool) {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v1, v2) = m258;
                let m255 = v1;
                if m255 == 0 {
                    Err::<(u64, (Adt6, List<Adt6>)), ((Adt6, List<Adt6>), bool)>((v2, false))
                } else {
                    let v3 = m255 - 1;
                    let m257 = {
                        let c256: Fn19 = v0.clone();
                        c256.apply(v2.clone())
                    };
                    match m257 {
                        None => {
                            Err::<(u64, (Adt6, List<Adt6>)), ((Adt6, List<Adt6>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt6, List<Adt6>)), ((Adt6, List<Adt6>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: (Adt6, List<Adt6>)) -> Option<(Adt6, List<Adt6>)> {
    f140(v0)
}

pub fn f85(v0: Fn20, p264: (bool, Bytes), p265: List<Bytes>) -> (bool, Bytes) {
    let m263 = (p264, p265);
    {
        let mut m263 = m263;
        loop {
            let step = {
                let (v1, v2) = m263;
                let m261 = v2;
                match m261.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Bytes>), (bool, Bytes)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Bytes>), (bool, Bytes)>(({
                            let c262: Fn20 = v0.clone();
                            c262.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m263 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Bytes, v1: (bool, Bytes), v2: Bytes) -> (bool, Bytes) {
    (false, {
        let (h266, _) = v1;
        if h266 {
            v2
        } else {
            f11(f11({
                let (_, h267) = v1;
                h267
            }, v0), v2)
        }
    })
}

pub fn f87(v0: u64) -> Bytes {
    f141(f142(), 10u64, v0)
}

pub fn f88(v0: Fn21, p272: u64, p273: (Adt6, List<(Bytes, Adt6)>)) -> ((Adt6, List<(Bytes, Adt6)>), bool) {
    let m271 = (p272, p273);
    {
        let mut m271 = m271;
        loop {
            let step = {
                let (v1, v2) = m271;
                let m268 = v1;
                if m268 == 0 {
                    Err::<(u64, (Adt6, List<(Bytes, Adt6)>)), ((Adt6, List<(Bytes, Adt6)>), bool)>((v2, false))
                } else {
                    let v3 = m268 - 1;
                    let m270 = {
                        let c269: Fn21 = v0.clone();
                        c269.apply(v2.clone())
                    };
                    match m270 {
                        None => {
                            Err::<(u64, (Adt6, List<(Bytes, Adt6)>)), ((Adt6, List<(Bytes, Adt6)>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt6, List<(Bytes, Adt6)>)), ((Adt6, List<(Bytes, Adt6)>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: (Adt6, List<(Bytes, Adt6)>)) -> Option<(Adt6, List<(Bytes, Adt6)>)> {
    let m275 = {
        let (h274, _) = v0;
        h274
    };
    match m275 {
        Adt6::C0(_) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C1(_, _) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C2(_) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C3(_) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C4(_) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C5 => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C6(_, _) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C7(v1, r278, r279) => {
            let v2 = (*r278).clone();
            let v3 = (*r279).clone();
            Some((v3, if f119(v1.clone(), List::cons(f2(Str::lit("kind")), List::cons(f2(Str::lit("name")), List::cons(f2(Str::lit("levels")), List::cons(f2(Str::lit("type")), List::cons(f2(Str::lit("value")), List::<Bytes>::nil())))))) {
                let (_, h276) = v0;
                h276
            } else {
                List::cons((v1, v2), {
                    let (_, h277) = v0;
                    h277
                })
            }))
        }
    }
}

pub fn f90(v0: Fn22, p283: Adt6, p284: List<(Bytes, Adt6)>) -> Adt6 {
    let m282 = (p283, p284);
    {
        let mut m282 = m282;
        loop {
            let step = {
                let (v1, v2) = m282;
                let m280 = v2;
                match m280.uncons() {
                    None => {
                        Err::<(Adt6, List<(Bytes, Adt6)>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<(Bytes, Adt6)>), Adt6>(({
                            let c281: Fn22 = v0.clone();
                            c281.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: Adt6, v1: (Bytes, Adt6)) -> Adt6 {
    Adt6::C7({
        let (h285, _) = v1;
        h285
    }, std::rc::Rc::new({
        let (_, h286) = v1;
        h286
    }), std::rc::Rc::new(v0))
}

pub fn f92(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a287 = 0u64;
        let a288 = v1;
        nat_lt(a287, a288)
    } else {
        false
    } {
        let a289 = Bytes::lit(&[0x2d]);
        let a290 = f87(v1);
        append_bytes(a289, a290)
    } else {
        f87(v1)
    }
}

pub fn f93(v0: Bytes) -> R<Bytes> {
    if f143(v0.clone())? {
        Ok(v0)
    } else {
        f76(v0)
    }
}

pub fn f94(v0: Str) -> Bytes {
    let a291 = v0;
    utf8_encode(a291)
}

pub fn f95(v0: List<Adt8>) -> List<Adt8> {
    f144(Fn31::F145, List::<Adt8>::nil(), v0)
}

pub fn f96(v0: Adt6) -> R<List<Adt8>> {
    let m292 = v0;
    match m292 {
        Adt6::C0(_) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C5 => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C6(r293, r294) => {
            let v7 = (*r293).clone();
            let v8 = (*r294).clone();
            Ok(List::cons(f53(v7)?, f96(v8)?))
        }
        Adt6::C7(_, _, _) => {
            Ok(List::<Adt8>::nil())
        }
    }
}

pub fn f97(v0: Option<Bytes>, v1: List<(Bytes, Adt8)>) -> R<Adt8> {
    let v2: List<(Bytes, Adt8)> = f146(Fn32::F147(v0.clone()), List::<(Bytes, Adt8)>::nil(), v1)?;
    Ok(f13({
        let m295 = v0;
        match m295 {
            None => {
                f94(Str::lit("{"))
            }
            Some(v3) => {
                f33(v3, f94(Str::lit("{")))
            }
        }
    }, f94(Str::lit("}")), f148(Fn33::F149, List::<Adt8>::nil(), f150(v2)?)))
}

pub fn f98(p303: Adt6) -> R<Option<Bytes>> {
    let m302 = p303;
    {
        let mut m302 = m302;
        loop {
            let step = {
                let v0 = m302;
                let m296 = v0;
                match m296 {
                    Adt6::C0(_) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C1(_, _) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C2(_) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C3(_) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C4(_) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C5 => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C6(_, _) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C7(v9, r300, r301) => {
                        let v10 = (*r300).clone();
                        let v11 = (*r301).clone();
                        let a297 = v9;
                        let a298 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
                        if equal(a297, a298) {
                            let m299 = v10;
                            match m299 {
                                Adt6::C0(_) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C1(_, _) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C2(v12) => {
                                    if f143(v12.clone())? {
                                        Err::<Adt6, R<Option<Bytes>>>(Ok(Some(v12)))
                                    } else {
                                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                    }
                                }
                                Adt6::C3(_) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C4(_) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C5 => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C6(_, _) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C7(_, _, _) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                            }
                        } else {
                            Ok::<Adt6, R<Option<Bytes>>>(v11)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: Adt6) -> R<List<(Bytes, Adt8)>> {
    let m304 = v0;
    match m304 {
        Adt6::C0(_) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C5 => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C6(_, _) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C7(v9, r305, r306) => {
            let v10 = (*r305).clone();
            let v11 = (*r306).clone();
            Ok(List::cons((v9, f53(v10)?), f99(v11)?))
        }
    }
}

pub fn f100(v0: List<u64>, v1: u64) -> R<(u64, List<u64>)> {
    let (h307, _) = f151(Fn34::F152(v1), {
        let a309 = {
            let a308 = v0.clone();
            length_list(a308)
        };
        let a310 = 1u64;
        nat_add(a309, a310)?
    }, (0u64, v0))?;
    Ok(h307)
}

pub fn f101(v0: Fn23, p314: Adt15, p315: List<u64>) -> Adt15 {
    let m313 = (p314, p315);
    {
        let mut m313 = m313;
        loop {
            let step = {
                let (v1, v2) = m313;
                let m311 = v2;
                match m311.uncons() {
                    None => {
                        Err::<(Adt15, List<u64>), Adt15>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt15, List<u64>), Adt15>(({
                            let c312: Fn23 = v0.clone();
                            c312.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Adt15, v1: u64) -> Adt15 {
    f153(v0, v1)
}

pub fn f103(v0: List<u64>) -> List<u64> {
    f107(Fn24::F154, List::<u64>::nil(), v0)
}

pub fn f104(v0: u64, v1: List<u64>) -> Option<Bytes> {
    f37(Fn9::F155(v0), None::<Bytes>, v1)
}

pub fn f105(v0: Adt11, v1: bool) -> Adt11 {
    if v1 {
        Adt11::C0(f66(List::cons(f67(f2(Str::lit("proof("))), List::cons(Adt12::C3(2u64, std::rc::Rc::new(f68(v0))), List::cons(f67(f2(Str::lit(")"))), List::<Adt12>::nil())))))
    } else {
        v0
    }
}

pub fn f106(v0: Adt1, v1: List<Adt11>) -> R<Adt11> {
    let m316 = v0;
    match m316 {
        Adt1::C0(v2) => {
            Ok(Adt11::C0(Adt12::C6(v2)))
        }
        Adt1::C1(v3) => {
            Ok(Adt11::C2(f67(f11(f2(Str::lit("Sort ")), f156(v3)?))))
        }
        Adt1::C2(v4, v5) => {
            Ok(Adt11::C0(Adt12::C7(v4, f157(v5)?)))
        }
        Adt1::C3(_, _) => {
            let v8: Adt17 = f158(f159(v1.clone(), 0u64));
            Ok(Adt11::C1(match v8.clone() {
                Adt17::C0(h317, _) => {
                    h317
                }
            }, List::cons(f160(f159(v1, 1u64)), match v8 {
                Adt17::C0(_, h318) => {
                    h318
                }
            })))
        }
        Adt1::C4(v9, v10, _, _) => {
            Ok(Adt11::C3(Adt12::C5(std::rc::Rc::new(f66(List::cons(f67(f2(Str::lit("fun "))), List::cons(f161(v10, v9.clone(), f159(v1.clone(), 0u64))?, List::cons(f67(f2(Str::lit(" =>"))), List::cons(Adt12::C4, List::cons(Adt12::C8(v9, std::rc::Rc::new(f68(f159(v1, 1u64)))), List::<Adt12>::nil()))))))))))
        }
        Adt1::C5(v13, v14, _, _) => {
            Ok(Adt11::C3(Adt12::C5(std::rc::Rc::new(f66(List::cons(if f40({
                let a320 = {
                    let a319 = v13.clone();
                    length_bytes(a319)
                };
                let a321 = 0u64;
                nat_eq(a320, a321)
            }, f162(v14.clone())) {
                f163(f159(v1.clone(), 0u64))
            } else {
                f161(v14, v13.clone(), f159(v1.clone(), 0u64))?
            }, List::cons(f67(f2(Str::lit(" ->"))), List::cons(Adt12::C4, List::cons(Adt12::C8(v13, std::rc::Rc::new(f68(f159(v1, 1u64)))), List::<Adt12>::nil())))))))))
        }
        Adt1::C6(v17, _, _, _, v21) => {
            Ok(Adt11::C3(Adt12::C5(std::rc::Rc::new(f66(List::cons(f67(f11(f11(if v21 {
                f2(Str::lit("have "))
            } else {
                f2(Str::lit("let "))
            }, f164(v17.clone())?), f2(Str::lit(" : ")))), List::cons(f68(f159(v1.clone(), 0u64)), List::cons(f67(f2(Str::lit(" := "))), List::cons(f68(f159(v1.clone(), 1u64)), List::cons(f67(f2(Str::lit(";"))), List::cons(Adt12::C4, List::cons(Adt12::C8(v17, std::rc::Rc::new(f68(f159(v1, 2u64)))), List::<Adt12>::nil()))))))))))))
        }
        Adt1::C7(v22) => {
            Ok(Adt11::C0(f67(if f165(v22.clone())? {
                v22
            } else {
                f11(f2(Str::lit("nat ")), f76(v22)?)
            })))
        }
        Adt1::C8(v23) => {
            Ok(Adt11::C0(f67(f76(f166(Fn35::F167, f2(Str::lit("")), v23)?)?)))
        }
        Adt1::C9(v24, _) => {
            Ok(Adt11::C2(Adt12::C5(std::rc::Rc::new(f66(List::cons(f67(f11(f11(f2(Str::lit("annot [")), f49(f168(f169(Fn36::F170, List::<Bytes>::nil(), v24)?), f2(Str::lit(", ")))), f2(Str::lit("]")))), List::cons(Adt12::C4, List::cons(f160(f159(v1, 0u64)), List::<Adt12>::nil()))))))))
        }
        Adt1::C10(v26, v27, v28, _) => {
            Ok(Adt11::C2(Adt12::C5(std::rc::Rc::new(f66(List::cons(f67(f11(f11(f11(f11(f11(f2(Str::lit("proj ")), f36(v27)?), f2(Str::lit(" "))), f36(v26)?), f2(Str::lit(" "))), f51(v28))), List::cons(Adt12::C4, List::cons(f160(f159(v1, 0u64)), List::<Adt12>::nil()))))))))
        }
    }
}

pub fn f107(v0: Fn24, p325: List<u64>, p326: List<u64>) -> List<u64> {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v1, v2) = m324;
                let m322 = v2;
                match m322.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c323: Fn24 = v0.clone();
                            c323.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f109(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f110(v0: Fn25, p330: List<(Bytes, u64)>, p331: List<(Bytes, u64)>) -> List<(Bytes, u64)> {
    let m329 = (p330, p331);
    {
        let mut m329 = m329;
        loop {
            let step = {
                let (v1, v2) = m329;
                let m327 = v2;
                match m327.uncons() {
                    None => {
                        Err::<(List<(Bytes, u64)>, List<(Bytes, u64)>), List<(Bytes, u64)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, u64)>, List<(Bytes, u64)>), List<(Bytes, u64)>>(({
                            let c328: Fn25 = v0.clone();
                            c328.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: List<(Bytes, u64)>, v1: (Bytes, u64)) -> List<(Bytes, u64)> {
    List::cons(v1, v0)
}

pub fn f112(v0: List<List<(Bytes, u64)>>) -> List<List<(Bytes, u64)>> {
    f171(Fn37::F172, List::<List<(Bytes, u64)>>::nil(), v0)
}

pub fn f113(v0: Fn26, p335: List<List<(Bytes, u64)>>, p336: List<(Bytes, u64)>) -> List<List<(Bytes, u64)>> {
    let m334 = (p335, p336);
    {
        let mut m334 = m334;
        loop {
            let step = {
                let (v1, v2) = m334;
                let m332 = v2;
                match m332.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, u64)>>, List<(Bytes, u64)>), List<List<(Bytes, u64)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, u64)>>, List<(Bytes, u64)>), List<List<(Bytes, u64)>>>(({
                            let c333: Fn26 = v0.clone();
                            c333.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<List<(Bytes, u64)>>, v1: (Bytes, u64)) -> List<List<(Bytes, u64)>> {
    List::cons(List::cons(v1, List::<(Bytes, u64)>::nil()), v0)
}

pub fn f115(v0: Fn27, p341: u64, p342: List<List<(Bytes, u64)>>) -> R<(List<List<(Bytes, u64)>>, bool)> {
    let m340 = (p341, p342);
    {
        let mut m340 = m340;
        loop {
            let step = {
                let (v1, v2) = m340;
                let m337 = v1;
                if m337 == 0 {
                    Err::<(u64, List<List<(Bytes, u64)>>), R<(List<List<(Bytes, u64)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m337 - 1;
                    let m339 = {
                        let c338: Fn27 = v0.clone();
                        c338.apply(v2.clone())?
                    };
                    match m339 {
                        None => {
                            Err::<(u64, List<List<(Bytes, u64)>>), R<(List<List<(Bytes, u64)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, u64)>>), R<(List<List<(Bytes, u64)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<List<(Bytes, u64)>>) -> R<Option<List<List<(Bytes, u64)>>>> {
    f173(v0)
}

pub fn f117(v0: u64) -> Bytes {
    let (_, h343) = f174(Fn38::F175, 8u64, (v0, f2(Str::lit(""))));
    h343
}

pub fn f118(v0: List<Bytes>, v1: u64) -> R<Bytes> {
    let v2: Bytes = f11(f2(Str::lit("#")), f51(v1));
    let m346 = {
        let a344 = v0.clone();
        let a345 = v1;
        index_list(a344, a345)
    };
    match m346 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            if f40(f75(v3.clone())?, {
                let m348 = f176(Fn39::F177(v3.clone(), v1), Adt18::C0(0u64, false), v0)?;
                !match m348 {
                    Adt18::C0(_, h347) => {
                        h347
                    }
                }
            }) {
                Ok(v3)
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f119(v0: Bytes, v1: List<Bytes>) -> bool {
    f178(Fn40::F179(v0), false, v1)
}

pub fn f120(v0: Fn28, p352: Adt12, p353: List<Adt12>) -> Adt12 {
    let m351 = (p352, p353);
    {
        let mut m351 = m351;
        loop {
            let step = {
                let (v1, v2) = m351;
                let m349 = v2;
                match m349.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt12>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt12>), Adt12>(({
                            let c350: Fn28 = v0.clone();
                            c350.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C2(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f122(v0: List<Adt12>) -> List<Adt12> {
    f180(Fn41::F181, List::<Adt12>::nil(), v0)
}

pub fn f123(v0: Adt12, v1: List<Adt12>) -> Adt12 {
    let m354 = v1.clone();
    match m354.uncons() {
        None => {
            v0
        }
        Some((_, _)) => {
            Adt12::C5(std::rc::Rc::new(f66(List::cons(v0, List::cons(Adt12::C3(2u64, std::rc::Rc::new(f66(f180(Fn41::F182, List::<Adt12>::nil(), v1)))), List::<Adt12>::nil())))))
        }
    }
}

pub fn f124(v0: u64, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    let m356 = match v2.clone() {
        Adt13::C0(_, _, h355) => {
            h355
        }
    };
    match m356.uncons() {
        None => {
            Ok(None::<Adt13>)
        }
        Some((v3, v4)) => {
            Ok(Some(f183(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f125(v0: List<Bytes>) -> Bytes {
    let m359 = {
        let (h357, _) = f184(Fn42::F185, {
            let a358 = v0.clone();
            length_list(a358)
        }, v0);
        h357
    };
    match m359.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f126(v0: List<Bytes>) -> List<Bytes> {
    f186(Fn43::F187, List::<Bytes>::nil(), v0)
}

pub fn f127(v0: Bytes) -> R<bool> {
    let v1: u64 = f188(v0.clone(), 0u64)?;
    if f189(v1) {
        Ok(true)
    } else {
        let a360 = v1;
        let a361 = 95u64;
        if nat_eq(a360, a361) {
            Ok(true)
        } else {
            Ok(f40({
                let a362 = 192u64;
                let a363 = v1;
                nat_le(a362, a363)
            }, {
                let a364 = f190(v0, 0u64)?;
                let a365 = 0u64;
                nat_eq(a364, a365)
            }))
        }
    }
}

pub fn f128(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let (h366, _) = f191(Fn44::F192(v0.clone()), {
            let a368 = {
                let a367 = v0.clone();
                length_bytes(a367)
            };
            let a369 = 1u64;
            nat_add(a368, a369)?
        }, 0u64)?;
        h366
    };
    let a371 = v1;
    let a372 = {
        let a370 = v0;
        length_bytes(a370)
    };
    Ok(nat_eq(a371, a372))
}

pub fn f129(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a373 = v0.clone();
        length_bytes(a373)
    };
    let v3: u64 = {
        let a374 = v1.clone();
        length_bytes(a374)
    };
    let a375 = v3;
    let a376 = v2;
    if nat_le(a375, a376) {
        let m382 = {
            let a379 = v0;
            let a380 = {
                let a377 = v2;
                let a378 = v3;
                nat_sub(a377, a378)
            };
            let a381 = v3;
            slice_bytes(a379, a380, a381)
        };
        match m382 {
            None => {
                false
            }
            Some(v4) => {
                let a383 = v4;
                let a384 = v1;
                equal(a383, a384)
            }
        }
    } else {
        false
    }
}

pub fn f130() -> List<Bytes> {
    List::cons(f2(Str::lit("fun")), List::cons(f2(Str::lit("let")), List::cons(f2(Str::lit("have")), List::cons(f2(Str::lit("proof")), List::cons(f2(Str::lit("Sort")), List::cons(f2(Str::lit("proj")), List::cons(f2(Str::lit("annot")), List::cons(f2(Str::lit("const")), List::cons(f2(Str::lit("max")), List::cons(f2(Str::lit("imax")), List::cons(f2(Str::lit("true")), List::cons(f2(Str::lit("false")), List::cons(f2(Str::lit("bool")), List::cons(f2(Str::lit("nat")), List::cons(f2(Str::lit("int")), List::cons(f2(Str::lit("str")), List::cons(f2(Str::lit("name")), List::<Bytes>::nil())))))))))))))))))
}

pub fn f131(v0: Bytes) -> R<bool> {
    let m390 = {
        let (h386, _) = f193(Fn45::F194(v0.clone()), {
            let a388 = {
                let a387 = v0;
                length_bytes(a387)
            };
            let a389 = 1u64;
            nat_add(a388, a389)?
        }, Adt19::C0(0u64, false))?;
        h386
    };
    match m390 {
        Adt19::C0(_, h385) => {
            Ok(h385)
        }
    }
}

pub fn f132(v0: Fn29, p395: u64, p396: Adt16) -> R<(Adt16, bool)> {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v1, v2) = m394;
                let m391 = v1;
                if m391 == 0 {
                    Err::<(u64, Adt16), R<(Adt16, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m391 - 1;
                    let m393 = {
                        let c392: Fn29 = v0.clone();
                        c392.apply(v2.clone())?
                    };
                    match m393 {
                        None => {
                            Err::<(u64, Adt16), R<(Adt16, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt16), R<(Adt16, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: u64, v1: Bytes, v2: Adt16) -> R<Option<Adt16>> {
    f195(v1, v0, v2)
}

pub fn f134(v0: Bytes, v1: Bytes) -> Bytes {
    let a397 = v0;
    let a398 = v1;
    append_bytes(a397, a398)
}

pub fn f135(v0: Str) -> Bytes {
    let a399 = v0;
    utf8_encode(a399)
}

pub fn f136(v0: List<Bytes>) -> Bytes {
    f125(f126(v0))
}

pub fn f137(v0: Bytes, v1: (Adt6, Option<Adt6>)) -> Option<(Adt6, Option<Adt6>)> {
    let m401 = {
        let (_, h400) = v1;
        h400
    };
    match m401 {
        None => {
            let m403 = {
                let (h402, _) = v1;
                h402
            };
            match m403 {
                Adt6::C0(_) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C1(_, _) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C2(_) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C3(_) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C4(_) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C5 => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C6(_, _) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C7(v3, r404, r405) => {
                    let v4 = (*r404).clone();
                    let v5 = (*r405).clone();
                    Some(if f1(v3, v0) {
                        (Adt6::C5, Some(v4))
                    } else {
                        (v5, None::<Adt6>)
                    })
                }
            }
        }
        Some(_) => {
            None::<(Adt6, Option<Adt6>)>
        }
    }
}

pub fn f138(v0: Fn30, p409: List<Adt6>, p410: List<Adt6>) -> List<Adt6> {
    let m408 = (p409, p410);
    {
        let mut m408 = m408;
        loop {
            let step = {
                let (v1, v2) = m408;
                let m406 = v2;
                match m406.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>(({
                            let c407: Fn30 = v0.clone();
                            c407.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(v1, v0)
}

pub fn f140(v0: (Adt6, List<Adt6>)) -> Option<(Adt6, List<Adt6>)> {
    let m412 = {
        let (h411, _) = v0;
        h411
    };
    match m412 {
        Adt6::C0(_) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C1(_, _) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C2(_) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C3(_) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C4(_) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C5 => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C6(r414, r415) => {
            let v1 = (*r414).clone();
            let v2 = (*r415).clone();
            Some((v2, List::cons(v1, {
                let (_, h413) = v0;
                h413
            })))
        }
        Adt6::C7(_, _, _) => {
            None::<(Adt6, List<Adt6>)>
        }
    }
}

pub fn f141(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a416 = v2;
    let a417 = 0u64;
    if nat_eq(a416, a417) {
        f196(v0, 0u64)
    } else {
        let (_, h418) = {
            let (h419, _) = f197(Fn46::F198(v1, v0), v2, (v2, Bytes::lit(&[])));
            h419
        };
        h418
    }
}

pub fn f142() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f143(v0: Bytes) -> R<bool> {
    let a421 = {
        let a420 = v0.clone();
        length_bytes(a420)
    };
    let a422 = 0u64;
    if nat_eq(a421, a422) {
        Ok(false)
    } else {
        if f199(f200(v0.clone(), 0u64)?) {
            let a424 = f201(v0.clone(), 1u64)?;
            let a425 = {
                let a423 = v0.clone();
                length_bytes(a423)
            };
            if nat_eq(a424, a425) {
                let a426 = f202(v0);
                Ok(bool_not(a426))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f144(v0: Fn31, p430: List<Adt8>, p431: List<Adt8>) -> List<Adt8> {
    let m429 = (p430, p431);
    {
        let mut m429 = m429;
        loop {
            let step = {
                let (v1, v2) = m429;
                let m427 = v2;
                match m427.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<Adt8>), List<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<Adt8>), List<Adt8>>(({
                            let c428: Fn31 = v0.clone();
                            c428.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: List<Adt8>, v1: Adt8) -> List<Adt8> {
    List::cons(v1, v0)
}

pub fn f146(v0: Fn32, p435: List<(Bytes, Adt8)>, p436: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    let m434 = (p435, p436);
    {
        let mut m434 = m434;
        loop {
            let step = {
                let (v1, v2) = m434;
                let m432 = v2;
                match m432.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt8)>, List<(Bytes, Adt8)>), R<List<(Bytes, Adt8)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt8)>, List<(Bytes, Adt8)>), R<List<(Bytes, Adt8)>>>(({
                            let c433: Fn32 = v0.clone();
                            c433.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Option<Bytes>, v1: List<(Bytes, Adt8)>, v2: (Bytes, Adt8)) -> R<List<(Bytes, Adt8)>> {
    let m437 = v0;
    match m437 {
        None => {
            Ok(List::cons((f203({
                let (h444, _) = v2.clone();
                h444
            }), f204({
                let (h445, _) = v2;
                h445
            }, {
                let (_, h446) = v2;
                h446
            })?), v1))
        }
        Some(_) => {
            let a439 = {
                let (h438, _) = v2.clone();
                h438
            };
            let a440 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
            if equal(a439, a440) {
                Ok(v1)
            } else {
                Ok(List::cons((f203({
                    let (h441, _) = v2.clone();
                    h441
                }), f204({
                    let (h442, _) = v2;
                    h442
                }, {
                    let (_, h443) = v2;
                    h443
                })?), v1))
            }
        }
    }
}

pub fn f148(v0: Fn33, p450: List<Adt8>, p451: List<(Bytes, Adt8)>) -> List<Adt8> {
    let m449 = (p450, p451);
    {
        let mut m449 = m449;
        loop {
            let step = {
                let (v1, v2) = m449;
                let m447 = v2;
                match m447.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<(Bytes, Adt8)>), List<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<(Bytes, Adt8)>), List<Adt8>>(({
                            let c448: Fn33 = v0.clone();
                            c448.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: List<Adt8>, v1: (Bytes, Adt8)) -> List<Adt8> {
    List::cons({
        let (_, h452) = v1;
        h452
    }, v0)
}

pub fn f150(v0: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    let v1: List<List<(Bytes, Adt8)>> = f205(f206(Fn47::F207, List::<List<(Bytes, Adt8)>>::nil(), v0.clone()));
    let m457 = {
        let (h453, _) = f208(Fn48::F209, {
            let a455 = {
                let a454 = v0;
                length_list(a454)
            };
            let a456 = 1u64;
            nat_add(a455, a456)?
        }, v1)?;
        h453
    };
    match m457.uncons() {
        None => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f151(v0: Fn34, p462: u64, p463: (u64, List<u64>)) -> R<((u64, List<u64>), bool)> {
    let m461 = (p462, p463);
    {
        let mut m461 = m461;
        loop {
            let step = {
                let (v1, v2) = m461;
                let m458 = v1;
                if m458 == 0 {
                    Err::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m458 - 1;
                    let m460 = {
                        let c459: Fn34 = v0.clone();
                        c459.apply(v2.clone())?
                    };
                    match m460 {
                        None => {
                            Err::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: u64, v1: (u64, List<u64>)) -> R<Option<(u64, List<u64>)>> {
    let m465 = {
        let (_, h464) = v1;
        h464
    };
    match m465.uncons() {
        None => {
            Ok(None::<(u64, List<u64>)>)
        }
        Some((v2, v3)) => {
            let a466 = v2;
            let a467 = v0;
            if nat_eq(a466, a467) {
                Ok(Some(({
                    let a469 = {
                        let (h468, _) = v1;
                        h468
                    };
                    let a470 = 1u64;
                    nat_add(a469, a470)?
                }, v3)))
            } else {
                Ok(None::<(u64, List<u64>)>)
            }
        }
    }
}

pub fn f153(v0: Adt15, v1: u64) -> Adt15 {
    let v2: Adt15 = Adt15::C0(match v0.clone() {
        Adt15::C0(h471, _, _) => {
            h471
        }
    }, List::cons(Adt11::C0(f67(f11(f2(Str::lit("@")), f51(v1)))), match v0.clone() {
        Adt15::C0(_, h472, _) => {
            h472
        }
    }), match v0.clone() {
        Adt15::C0(_, _, h473) => {
            h473
        }
    });
    let m475 = match v0.clone() {
        Adt15::C0(h474, _, _) => {
            h474
        }
    };
    match m475.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            if f40({
                let a477 = match v3.clone() {
                    Adt10::C0(h476, _, _, _, _) => {
                        h476
                    }
                };
                let a478 = v1;
                nat_eq(a477, a478)
            }, f40({
                let a480 = match v3.clone() {
                    Adt10::C0(_, _, _, h479, _) => {
                        h479
                    }
                };
                let a481 = 1u64;
                nat_eq(a480, a481)
            }, {
                let a483 = match v3.clone() {
                    Adt10::C0(_, _, h482, _, _) => {
                        h482
                    }
                };
                let a484 = 48u64;
                nat_lt(a483, a484)
            })) {
                Adt15::C0(v4, List::cons(match v3.clone() {
                    Adt10::C0(_, _, _, _, h485) => {
                        h485
                    }
                }, match v0.clone() {
                    Adt15::C0(_, h486, _) => {
                        h486
                    }
                }), {
                    let a489 = match v0.clone() {
                        Adt15::C0(_, _, h487) => {
                            h487
                        }
                    };
                    let a490 = match v3.clone() {
                        Adt10::C0(_, _, h488, _, _) => {
                            h488
                        }
                    };
                    if nat_lt(a489, a490) {
                        match v3 {
                            Adt10::C0(_, _, h491, _, _) => {
                                h491
                            }
                        }
                    } else {
                        match v0 {
                            Adt15::C0(_, _, h492) => {
                                h492
                            }
                        }
                    }
                })
            } else {
                v2
            }
        }
    }
}

pub fn f154(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f155(v0: u64, v1: Option<Bytes>, v2: u64) -> Option<Bytes> {
    let m493 = v1.clone();
    match m493 {
        None => {
            if f210(v2) {
                None::<Bytes>
            } else {
                Some(f11(f11(f11(f11(f2(Str::lit("node ")), f51(v0)), f2(Str::lit(": "))), f51(v2)), f2(Str::lit(" is not a Unicode scalar value"))))
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f156(v0: Adt2) -> R<Bytes> {
    Ok(f211(f212(v0)?))
}

pub fn f157(v0: List<Adt2>) -> R<Bytes> {
    let a495 = {
        let a494 = v0.clone();
        length_list(a494)
    };
    let a496 = 0u64;
    if nat_eq(a495, a496) {
        Ok(f2(Str::lit("")))
    } else {
        Ok(f11(f11(f2(Str::lit(".{")), f49(f168(f213(Fn49::F214, List::<Bytes>::nil(), v0)?), f2(Str::lit(", ")))), f2(Str::lit("}"))))
    }
}

pub fn f158(v0: Adt11) -> Adt17 {
    let m497 = v0;
    match m497 {
        Adt11::C0(v1) => {
            Adt17::C0(v1, List::<Adt12>::nil())
        }
        Adt11::C1(v2, v3) => {
            Adt17::C0(v2, v3)
        }
        Adt11::C2(v4) => {
            Adt17::C0(f215(v4), List::<Adt12>::nil())
        }
        Adt11::C3(v5) => {
            Adt17::C0(f215(v5), List::<Adt12>::nil())
        }
    }
}

pub fn f159(v0: List<Adt11>, v1: u64) -> Adt11 {
    let m500 = {
        let a498 = v0;
        let a499 = v1;
        index_list(a498, a499)
    };
    match m500 {
        None => {
            Adt11::C0(Adt12::C0)
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f160(v0: Adt11) -> Adt12 {
    let m501 = v0.clone();
    match m501 {
        Adt11::C0(v1) => {
            v1
        }
        Adt11::C1(_, _) => {
            f215(f68(v0))
        }
        Adt11::C2(v4) => {
            f215(v4)
        }
        Adt11::C3(v5) => {
            f215(v5)
        }
    }
}

pub fn f161(v0: Adt3, v1: Bytes, v2: Adt11) -> R<Adt12> {
    let v3: (Bytes, Bytes) = {
        let m502 = v0;
        match m502 {
            Adt3::C0 => {
                (f2(Str::lit("(")), f2(Str::lit(")")))
            }
            Adt3::C1 => {
                (f2(Str::lit("{")), f2(Str::lit("}")))
            }
            Adt3::C2 => {
                (f2(Str::lit("{{")), f2(Str::lit("}}")))
            }
            Adt3::C3 => {
                (f2(Str::lit("[")), f2(Str::lit("]")))
            }
        }
    };
    Ok(f66(List::cons(f67(f11(f11({
        let (h503, _) = v3;
        h503
    }, f164(v1)?), f2(Str::lit(" : ")))), List::cons(f68(v2), List::cons(f67({
        let (_, h504) = v3;
        h504
    }), List::<Adt12>::nil())))))
}

pub fn f162(v0: Adt3) -> bool {
    let m505 = v0;
    match m505 {
        Adt3::C0 => {
            true
        }
        Adt3::C1 => {
            false
        }
        Adt3::C2 => {
            false
        }
        Adt3::C3 => {
            false
        }
    }
}

pub fn f163(v0: Adt11) -> Adt12 {
    let m506 = v0.clone();
    match m506 {
        Adt11::C0(v2) => {
            v2
        }
        Adt11::C1(_, _) => {
            f68(v0)
        }
        Adt11::C2(v5) => {
            v5
        }
        Adt11::C3(v1) => {
            f215(v1)
        }
    }
}

pub fn f164(v0: Bytes) -> R<Bytes> {
    let a508 = {
        let a507 = v0.clone();
        length_bytes(a507)
    };
    let a509 = 0u64;
    if nat_eq(a508, a509) {
        Ok(f2(Str::lit("_")))
    } else {
        f36(v0)
    }
}

pub fn f165(v0: Bytes) -> R<bool> {
    Ok(f40({
        let a511 = 0u64;
        let a512 = {
            let a510 = v0.clone();
            length_bytes(a510)
        };
        nat_lt(a511, a512)
    }, f40({
        let a514 = f216(v0.clone(), 0u64)?;
        let a515 = {
            let a513 = v0.clone();
            length_bytes(a513)
        };
        nat_eq(a514, a515)
    }, if f1(v0.clone(), f2(Str::lit("0"))) {
        true
    } else {
        !f217(v0, f2(Str::lit("0")))
    })))
}

pub fn f166(v0: Fn35, p519: Bytes, p520: List<u64>) -> R<Bytes> {
    let m518 = (p519, p520);
    {
        let mut m518 = m518;
        loop {
            let step = {
                let (v1, v2) = m518;
                let m516 = v2;
                match m516.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c517: Fn35 = v0.clone();
                            c517.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Bytes, v1: u64) -> R<Bytes> {
    Ok(f11(v0, f218(v1)?))
}

pub fn f168(v0: List<Bytes>) -> List<Bytes> {
    f186(Fn43::F219, List::<Bytes>::nil(), v0)
}

pub fn f169(v0: Fn36, p524: List<Bytes>, p525: List<Adt4>) -> R<List<Bytes>> {
    let m523 = (p524, p525);
    {
        let mut m523 = m523;
        loop {
            let step = {
                let (v1, v2) = m523;
                let m521 = v2;
                match m521.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt4>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt4>), R<List<Bytes>>>(({
                            let c522: Fn36 = v0.clone();
                            c522.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: List<Bytes>, v1: Adt4) -> R<List<Bytes>> {
    Ok(List::cons(f220(v1)?, v0))
}

pub fn f171(v0: Fn37, p529: List<List<(Bytes, u64)>>, p530: List<List<(Bytes, u64)>>) -> List<List<(Bytes, u64)>> {
    let m528 = (p529, p530);
    {
        let mut m528 = m528;
        loop {
            let step = {
                let (v1, v2) = m528;
                let m526 = v2;
                match m526.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), List<List<(Bytes, u64)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), List<List<(Bytes, u64)>>>(({
                            let c527: Fn37 = v0.clone();
                            c527.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: List<List<(Bytes, u64)>>, v1: List<(Bytes, u64)>) -> List<List<(Bytes, u64)>> {
    List::cons(v1, v0)
}

pub fn f173(v0: List<List<(Bytes, u64)>>) -> R<Option<List<List<(Bytes, u64)>>>> {
    let a532 = {
        let a531 = v0.clone();
        length_list(a531)
    };
    let a533 = 1u64;
    if nat_le(a532, a533) {
        Ok(None::<List<List<(Bytes, u64)>>>)
    } else {
        Ok(Some(f221(v0)?))
    }
}

pub fn f174(v0: Fn38, p537: u64, p538: (u64, Bytes)) -> (u64, Bytes) {
    let m536 = (p537, p538);
    {
        let mut m536 = m536;
        loop {
            let step = {
                let (v1, v2) = m536;
                let m534 = v1;
                if m534 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m534 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c535: Fn38 = v0.clone();
                        c535.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a540 = {
            let (h539, _) = v0.clone();
            h539
        };
        let a541 = 256u64;
        let a542 = 0u64;
        nat_quot(a540, a541, a542)
    }, f11(f222({
        let a544 = {
            let (h543, _) = v0;
            h543
        };
        let a545 = 256u64;
        let a546 = 0u64;
        nat_rem(a544, a545, a546)
    }), {
        let (_, h547) = v0;
        h547
    }))
}

pub fn f176(v0: Fn39, p551: Adt18, p552: List<Bytes>) -> R<Adt18> {
    let m550 = (p551, p552);
    {
        let mut m550 = m550;
        loop {
            let step = {
                let (v1, v2) = m550;
                let m548 = v2;
                match m548.uncons() {
                    None => {
                        Err::<(Adt18, List<Bytes>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Bytes>), R<Adt18>>(({
                            let c549: Fn39 = v0.clone();
                            c549.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes, v1: u64, v2: Adt18, v3: Bytes) -> R<Adt18> {
    f223(v2, v3, v0, v1)
}

pub fn f178(v0: Fn40, p556: bool, p557: List<Bytes>) -> bool {
    let m555 = (p556, p557);
    {
        let mut m555 = m555;
        loop {
            let step = {
                let (v1, v2) = m555;
                let m553 = v2;
                match m553.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c554: Fn40 = v0.clone();
                            c554.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a558 = v2;
        let a559 = v0;
        equal(a558, a559)
    }
}

pub fn f180(v0: Fn41, p563: List<Adt12>, p564: List<Adt12>) -> List<Adt12> {
    let m562 = (p563, p564);
    {
        let mut m562 = m562;
        loop {
            let step = {
                let (v1, v2) = m562;
                let m560 = v2;
                match m560.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c561: Fn41 = v0.clone();
                            c561.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    List::cons(v1, v0)
}

pub fn f182(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    List::cons(Adt12::C4, List::cons(v1, v0))
}

pub fn f183(v0: u64, v1: u64, v2: Adt13, v3: Adt14, v4: List<Adt14>) -> R<Adt13> {
    let v5: u64 = match v3.clone() {
        Adt14::C0(h565, _, _) => {
            h565
        }
    };
    let v6: bool = match v3.clone() {
        Adt14::C0(_, h566, _) => {
            h566
        }
    };
    let m568 = match v3 {
        Adt14::C0(_, _, h567) => {
            h567
        }
    };
    match m568 {
        Adt8::C0 => {
            Ok(Adt13::C0(match v2.clone() {
                Adt13::C0(h569, _, _) => {
                    h569
                }
            }, match v2 {
                Adt13::C0(_, h570, _) => {
                    h570
                }
            }, v4))
        }
        Adt8::C1(v7) => {
            Ok(Adt13::C0(List::cons(v7.clone(), match v2.clone() {
                Adt13::C0(h571, _, _) => {
                    h571
                }
            }), {
                let a573 = match v2 {
                    Adt13::C0(_, h572, _) => {
                        h572
                    }
                };
                let a574 = f224(v7)?;
                nat_add(a573, a574)?
            }, v4))
        }
        Adt8::C2(r591, r592) => {
            let v8 = (*r591).clone();
            let v9 = (*r592).clone();
            Ok(Adt13::C0(match v2.clone() {
                Adt13::C0(h575, _, _) => {
                    h575
                }
            }, match v2 {
                Adt13::C0(_, h576, _) => {
                    h576
                }
            }, List::cons(Adt14::C0(v5, v6, v8), List::cons(Adt14::C0(v5, v6, v9), v4))))
        }
        Adt8::C3(v10, r593) => {
            let v11 = (*r593).clone();
            Ok(Adt13::C0(match v2.clone() {
                Adt13::C0(h577, _, _) => {
                    h577
                }
            }, match v2 {
                Adt13::C0(_, h578, _) => {
                    h578
                }
            }, List::cons(Adt14::C0({
                let a579 = v5;
                let a580 = v10;
                nat_add(a579, a580)?
            }, v6, v11), v4)))
        }
        Adt8::C4 => {
            if v6 {
                Ok(Adt13::C0(List::cons({
                    let a581 = Str::lit(" ");
                    utf8_encode(a581)
                }, match v2.clone() {
                    Adt13::C0(h582, _, _) => {
                        h582
                    }
                }), {
                    let a584 = match v2 {
                        Adt13::C0(_, h583, _) => {
                            h583
                        }
                    };
                    let a585 = 1u64;
                    nat_add(a584, a585)?
                }, v4))
            } else {
                f225(v2, v5, v4)
            }
        }
        Adt8::C5 => {
            if v6 {
                Ok(Adt13::C0(match v2.clone() {
                    Adt13::C0(h586, _, _) => {
                        h586
                    }
                }, match v2 {
                    Adt13::C0(_, h587, _) => {
                        h587
                    }
                }, v4))
            } else {
                f225(v2, v5, v4)
            }
        }
        Adt8::C6(r594) => {
            let v12 = (*r594).clone();
            let v13: bool = f226(v1, v0, match v2.clone() {
                Adt13::C0(_, h588, _) => {
                    h588
                }
            }, v4.clone(), Adt14::C0(v5, true, v12.clone()))?;
            Ok(Adt13::C0(match v2.clone() {
                Adt13::C0(h589, _, _) => {
                    h589
                }
            }, match v2 {
                Adt13::C0(_, h590, _) => {
                    h590
                }
            }, List::cons(Adt14::C0(v5, v13, v12), v4)))
        }
    }
}

pub fn f184(v0: Fn42, p599: u64, p600: List<Bytes>) -> (List<Bytes>, bool) {
    let m598 = (p599, p600);
    {
        let mut m598 = m598;
        loop {
            let step = {
                let (v1, v2) = m598;
                let m595 = v1;
                if m595 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m595 - 1;
                    let m597 = {
                        let c596: Fn42 = v0.clone();
                        c596.apply(v2.clone())
                    };
                    match m597 {
                        None => {
                            Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), (List<Bytes>, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m598 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: List<Bytes>) -> Option<List<Bytes>> {
    f227(v0)
}

pub fn f186(v0: Fn43, p604: List<Bytes>, p605: List<Bytes>) -> List<Bytes> {
    let m603 = (p604, p605);
    {
        let mut m603 = m603;
        loop {
            let step = {
                let (v1, v2) = m603;
                let m601 = v2;
                match m601.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c602: Fn43 = v0.clone();
                            c602.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m603 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f188(v0: Bytes, v1: u64) -> R<u64> {
    let m606 = f228(v0, v1);
    match m606 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a651 = {
                let a627 = {
                    let a615 = {
                        let a609 = {
                            let a607 = v3;
                            let a608 = 128u8;
                            fixed_u8::bit_and(a607, a608)
                        };
                        let a610 = 128u8;
                        if equal(a609, a610) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a616 = {
                        let a613 = {
                            let a611 = v3;
                            let a612 = 64u8;
                            fixed_u8::bit_and(a611, a612)
                        };
                        let a614 = 64u8;
                        if equal(a613, a614) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a615, a616)?
                };
                let a628 = {
                    let a625 = {
                        let a619 = {
                            let a617 = v3;
                            let a618 = 32u8;
                            fixed_u8::bit_and(a617, a618)
                        };
                        let a620 = 32u8;
                        if equal(a619, a620) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a626 = {
                        let a623 = {
                            let a621 = v3;
                            let a622 = 16u8;
                            fixed_u8::bit_and(a621, a622)
                        };
                        let a624 = 16u8;
                        if equal(a623, a624) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a625, a626)?
                };
                nat_add(a627, a628)?
            };
            let a652 = {
                let a649 = {
                    let a637 = {
                        let a631 = {
                            let a629 = v3;
                            let a630 = 8u8;
                            fixed_u8::bit_and(a629, a630)
                        };
                        let a632 = 8u8;
                        if equal(a631, a632) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a638 = {
                        let a635 = {
                            let a633 = v3;
                            let a634 = 4u8;
                            fixed_u8::bit_and(a633, a634)
                        };
                        let a636 = 4u8;
                        if equal(a635, a636) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a637, a638)?
                };
                let a650 = {
                    let a647 = {
                        let a641 = {
                            let a639 = v3;
                            let a640 = 2u8;
                            fixed_u8::bit_and(a639, a640)
                        };
                        let a642 = 2u8;
                        if equal(a641, a642) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a648 = {
                        let a645 = {
                            let a643 = v3;
                            let a644 = 1u8;
                            fixed_u8::bit_and(a643, a644)
                        };
                        let a646 = 1u8;
                        if equal(a645, a646) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a647, a648)?
                };
                nat_add(a649, a650)?
            };
            nat_add(a651, a652)
        }
    }
}

pub fn f189(v0: u64) -> bool {
    if f40({
        let a653 = 65u64;
        let a654 = v0;
        nat_le(a653, a654)
    }, {
        let a655 = v0;
        let a656 = 90u64;
        nat_le(a655, a656)
    }) {
        true
    } else {
        f40({
            let a657 = 97u64;
            let a658 = v0;
            nat_le(a657, a658)
        }, {
            let a659 = v0;
            let a660 = 122u64;
            nat_le(a659, a660)
        })
    }
}

pub fn f190(v0: Bytes, v1: u64) -> R<u64> {
    if if f229(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f230(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f230(v0.clone(), v1, 194u8) {
            if if f230(v0.clone(), {
                let a661 = v1;
                let a662 = 1u64;
                nat_add(a661, a662)?
            }, 133u8) {
                true
            } else {
                f230(v0, {
                    let a663 = v1;
                    let a664 = 1u64;
                    nat_add(a663, a664)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f230(v0.clone(), v1, 225u8) {
                if if f230(v0.clone(), {
                    let a665 = v1;
                    let a666 = 1u64;
                    nat_add(a665, a666)?
                }, 154u8) {
                    f230(v0, {
                        let a667 = v1;
                        let a668 = 2u64;
                        nat_add(a667, a668)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f230(v0.clone(), v1, 226u8) {
                    if f230(v0.clone(), {
                        let a669 = v1;
                        let a670 = 1u64;
                        nat_add(a669, a670)?
                    }, 128u8) {
                        if if f229(v0.clone(), {
                            let a671 = v1;
                            let a672 = 2u64;
                            nat_add(a671, a672)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f230(v0.clone(), {
                                let a673 = v1;
                                let a674 = 2u64;
                                nat_add(a673, a674)?
                            }, 168u8) {
                                true
                            } else {
                                if f230(v0.clone(), {
                                    let a675 = v1;
                                    let a676 = 2u64;
                                    nat_add(a675, a676)?
                                }, 169u8) {
                                    true
                                } else {
                                    f230(v0, {
                                        let a677 = v1;
                                        let a678 = 2u64;
                                        nat_add(a677, a678)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f230(v0.clone(), {
                            let a679 = v1;
                            let a680 = 1u64;
                            nat_add(a679, a680)?
                        }, 129u8) {
                            f230(v0, {
                                let a681 = v1;
                                let a682 = 2u64;
                                nat_add(a681, a682)?
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
                    if f230(v0.clone(), v1, 227u8) {
                        if if f230(v0.clone(), {
                            let a683 = v1;
                            let a684 = 1u64;
                            nat_add(a683, a684)?
                        }, 128u8) {
                            f230(v0, {
                                let a685 = v1;
                                let a686 = 2u64;
                                nat_add(a685, a686)?
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

pub fn f191(v0: Fn44, p691: u64, p692: u64) -> R<(u64, bool)> {
    let m690 = (p691, p692);
    {
        let mut m690 = m690;
        loop {
            let step = {
                let (v1, v2) = m690;
                let m687 = v1;
                if m687 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m687 - 1;
                    let m689 = {
                        let c688: Fn44 = v0.clone();
                        c688.apply(v2)?
                    };
                    match m689 {
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
                Ok(next) => { m690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f40({
        let a694 = v1;
        let a695 = {
            let a693 = v0.clone();
            length_bytes(a693)
        };
        nat_lt(a694, a695)
    }, f231(v0, v1)?) {
        Ok(Some({
            let a696 = v1;
            let a697 = 1u64;
            nat_add(a696, a697)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f193(v0: Fn45, p702: u64, p703: Adt19) -> R<(Adt19, bool)> {
    let m701 = (p702, p703);
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v1, v2) = m701;
                let m698 = v1;
                if m698 == 0 {
                    Err::<(u64, Adt19), R<(Adt19, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m698 - 1;
                    let m700 = {
                        let c699: Fn45 = v0.clone();
                        c699.apply(v2.clone())?
                    };
                    match m700 {
                        None => {
                            Err::<(u64, Adt19), R<(Adt19, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt19), R<(Adt19, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes, v1: Adt19) -> R<Option<Adt19>> {
    f232(v0, v1)
}

pub fn f195(v0: Bytes, v1: u64, v2: Adt16) -> R<Option<Adt16>> {
    let v3: u64 = match v2.clone() {
        Adt16::C0(h704, _) => {
            h704
        }
    };
    let a705 = v3;
    let a706 = v1;
    if nat_lt(a705, a706) {
        let v4: u64 = f233(v0.clone(), v3)?;
        let a707 = v4;
        let a708 = 32u64;
        if if nat_lt(a707, a708) {
            true
        } else {
            let a709 = v4;
            let a710 = 34u64;
            if nat_eq(a709, a710) {
                true
            } else {
                let a711 = v4;
                let a712 = 92u64;
                nat_eq(a711, a712)
            }
        } {
            Ok(Some(Adt16::C0({
                let a713 = v3;
                let a714 = 1u64;
                nat_add(a713, a714)?
            }, List::cons(f234(v4), match v2 {
                Adt16::C0(_, h715) => {
                    h715
                }
            }))))
        } else {
            let v5: u64 = f235(v0.clone(), v1, v3)?;
            Ok(Some(Adt16::C0(v5, List::cons(f236(v0, v3, {
                let a716 = v5;
                let a717 = v3;
                nat_sub(a716, a717)
            }), match v2 {
                Adt16::C0(_, h718) => {
                    h718
                }
            }))))
        }
    } else {
        Ok(None::<Adt16>)
    }
}

pub fn f196(v0: Bytes, v1: u64) -> Bytes {
    let m722 = {
        let a719 = v0;
        let a720 = v1;
        let a721 = 1u64;
        slice_bytes(a719, a720, a721)
    };
    match m722 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f197(v0: Fn46, p727: u64, p728: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m726 = (p727, p728);
    {
        let mut m726 = m726;
        loop {
            let step = {
                let (v1, v2) = m726;
                let m723 = v1;
                if m723 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m723 - 1;
                    let m725 = {
                        let c724: Fn46 = v0.clone();
                        c724.apply(v2.clone())
                    };
                    match m725 {
                        None => {
                            Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f237(v1, v0, v2)
}

pub fn f199(v0: u64) -> bool {
    let a729 = 64u64;
    let a730 = v0;
    if if nat_lt(a729, a730) {
        let a731 = v0;
        let a732 = 91u64;
        nat_lt(a731, a732)
    } else {
        false
    } {
        true
    } else {
        let a733 = 96u64;
        let a734 = v0;
        if if nat_lt(a733, a734) {
            let a735 = v0;
            let a736 = 123u64;
            nat_lt(a735, a736)
        } else {
            false
        } {
            true
        } else {
            let a737 = v0;
            let a738 = 95u64;
            nat_eq(a737, a738)
        }
    }
}

pub fn f200(v0: Bytes, v1: u64) -> R<u64> {
    let m739 = f228(v0, v1);
    match m739 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a784 = {
                let a760 = {
                    let a748 = {
                        let a742 = {
                            let a740 = v3;
                            let a741 = 128u8;
                            fixed_u8::bit_and(a740, a741)
                        };
                        let a743 = 128u8;
                        if equal(a742, a743) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a749 = {
                        let a746 = {
                            let a744 = v3;
                            let a745 = 64u8;
                            fixed_u8::bit_and(a744, a745)
                        };
                        let a747 = 64u8;
                        if equal(a746, a747) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a748, a749)?
                };
                let a761 = {
                    let a758 = {
                        let a752 = {
                            let a750 = v3;
                            let a751 = 32u8;
                            fixed_u8::bit_and(a750, a751)
                        };
                        let a753 = 32u8;
                        if equal(a752, a753) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a759 = {
                        let a756 = {
                            let a754 = v3;
                            let a755 = 16u8;
                            fixed_u8::bit_and(a754, a755)
                        };
                        let a757 = 16u8;
                        if equal(a756, a757) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a758, a759)?
                };
                nat_add(a760, a761)?
            };
            let a785 = {
                let a782 = {
                    let a770 = {
                        let a764 = {
                            let a762 = v3;
                            let a763 = 8u8;
                            fixed_u8::bit_and(a762, a763)
                        };
                        let a765 = 8u8;
                        if equal(a764, a765) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a771 = {
                        let a768 = {
                            let a766 = v3;
                            let a767 = 4u8;
                            fixed_u8::bit_and(a766, a767)
                        };
                        let a769 = 4u8;
                        if equal(a768, a769) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a770, a771)?
                };
                let a783 = {
                    let a780 = {
                        let a774 = {
                            let a772 = v3;
                            let a773 = 2u8;
                            fixed_u8::bit_and(a772, a773)
                        };
                        let a775 = 2u8;
                        if equal(a774, a775) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a781 = {
                        let a778 = {
                            let a776 = v3;
                            let a777 = 1u8;
                            fixed_u8::bit_and(a776, a777)
                        };
                        let a779 = 1u8;
                        if equal(a778, a779) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a780, a781)?
                };
                nat_add(a782, a783)?
            };
            nat_add(a784, a785)
        }
    }
}

pub fn f201(v0: Bytes, v1: u64) -> R<u64> {
    let (h786, _) = f191(Fn44::F238(v0.clone()), {
        let a788 = {
            let a787 = v0;
            length_bytes(a787)
        };
        let a789 = 1u64;
        nat_add(a788, a789)?
    }, v1)?;
    Ok(h786)
}

pub fn f202(v0: Bytes) -> bool {
    let a790 = v0.clone();
    let a791 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a790, a791) {
        true
    } else {
        let a792 = v0;
        let a793 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a792, a793)
    }
}

pub fn f203(v0: Bytes) -> Bytes {
    f33(f222(f239(v0.clone())), v0)
}

pub fn f204(v0: Bytes, v1: Adt8) -> R<Adt8> {
    Ok(Adt8::C2(std::rc::Rc::new(Adt8::C1(f33(f93(v0)?, f94(Str::lit(": "))))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt8::C0)))))
}

pub fn f205(v0: List<List<(Bytes, Adt8)>>) -> List<List<(Bytes, Adt8)>> {
    f240(Fn50::F241, List::<List<(Bytes, Adt8)>>::nil(), v0)
}

pub fn f206(v0: Fn47, p797: List<List<(Bytes, Adt8)>>, p798: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
    let m796 = (p797, p798);
    {
        let mut m796 = m796;
        loop {
            let step = {
                let (v1, v2) = m796;
                let m794 = v2;
                match m794.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt8)>>, List<(Bytes, Adt8)>), List<List<(Bytes, Adt8)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt8)>>, List<(Bytes, Adt8)>), List<List<(Bytes, Adt8)>>>(({
                            let c795: Fn47 = v0.clone();
                            c795.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: List<List<(Bytes, Adt8)>>, v1: (Bytes, Adt8)) -> List<List<(Bytes, Adt8)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt8)>::nil()), v0)
}

pub fn f208(v0: Fn48, p803: u64, p804: List<List<(Bytes, Adt8)>>) -> R<(List<List<(Bytes, Adt8)>>, bool)> {
    let m802 = (p803, p804);
    {
        let mut m802 = m802;
        loop {
            let step = {
                let (v1, v2) = m802;
                let m799 = v1;
                if m799 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m799 - 1;
                    let m801 = {
                        let c800: Fn48 = v0.clone();
                        c800.apply(v2.clone())?
                    };
                    match m801 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
    f242(v0)
}

pub fn f210(v0: u64) -> bool {
    f40({
        let a805 = v0;
        let a806 = 1114111u64;
        nat_le(a805, a806)
    }, !f40({
        let a807 = 55296u64;
        let a808 = v0;
        nat_le(a807, a808)
    }, {
        let a809 = v0;
        let a810 = 57343u64;
        nat_le(a809, a810)
    }))
}

pub fn f211(v0: Adt20) -> Bytes {
    let a812 = match v0.clone() {
        Adt20::C0(_, _, h811) => {
            h811
        }
    };
    let a813 = 0u64;
    if nat_eq(a812, a813) {
        if match v0.clone() {
            Adt20::C0(_, h814, _) => {
                h814
            }
        } {
            f2(Str::lit("0"))
        } else {
            match v0 {
                Adt20::C0(h815, _, _) => {
                    h815
                }
            }
        }
    } else {
        if match v0.clone() {
            Adt20::C0(_, h816, _) => {
                h816
            }
        } {
            f51(match v0 {
                Adt20::C0(_, _, h817) => {
                    h817
                }
            })
        } else {
            f11(f11(match v0.clone() {
                Adt20::C0(h818, _, _) => {
                    h818
                }
            }, f2(Str::lit("+"))), f51(match v0 {
                Adt20::C0(_, _, h819) => {
                    h819
                }
            }))
        }
    }
}

pub fn f212(v0: Adt2) -> R<Adt20> {
    let m820 = v0;
    match m820 {
        Adt2::C0 => {
            Ok(Adt20::C0(f2(Str::lit("")), true, 0u64))
        }
        Adt2::C1(r826) => {
            let v2 = (*r826).clone();
            let v3: Adt20 = f212(v2)?;
            Ok(Adt20::C0(match v3.clone() {
                Adt20::C0(h821, _, _) => {
                    h821
                }
            }, match v3.clone() {
                Adt20::C0(_, h822, _) => {
                    h822
                }
            }, {
                let a824 = match v3 {
                    Adt20::C0(_, _, h823) => {
                        h823
                    }
                };
                let a825 = 1u64;
                nat_add(a824, a825)?
            }))
        }
        Adt2::C2(r827, r828) => {
            let v4 = (*r827).clone();
            let v5 = (*r828).clone();
            let v6: Adt20 = f212(v4)?;
            let v7: Adt20 = f212(v5)?;
            Ok(Adt20::C0(f11(f11(f11(f11(f2(Str::lit("max(")), f211(v6)), f2(Str::lit(", "))), f211(v7)), f2(Str::lit(")"))), false, 0u64))
        }
        Adt2::C3(r829, r830) => {
            let v8 = (*r829).clone();
            let v9 = (*r830).clone();
            let v10: Adt20 = f212(v8)?;
            let v11: Adt20 = f212(v9)?;
            Ok(Adt20::C0(f11(f11(f11(f11(f2(Str::lit("imax(")), f211(v10)), f2(Str::lit(", "))), f211(v11)), f2(Str::lit(")"))), false, 0u64))
        }
        Adt2::C4(v1) => {
            Ok(Adt20::C0(f36(v1)?, false, 0u64))
        }
    }
}

pub fn f213(v0: Fn49, p834: List<Bytes>, p835: List<Adt2>) -> R<List<Bytes>> {
    let m833 = (p834, p835);
    {
        let mut m833 = m833;
        loop {
            let step = {
                let (v1, v2) = m833;
                let m831 = v2;
                match m831.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(({
                            let c832: Fn49 = v0.clone();
                            c832.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: List<Bytes>, v1: Adt2) -> R<List<Bytes>> {
    Ok(List::cons(f156(v1)?, v0))
}

pub fn f215(v0: Adt12) -> Adt12 {
    f66(List::cons(f67(f2(Str::lit("("))), List::cons(v0, List::cons(f67(f2(Str::lit(")"))), List::<Adt12>::nil()))))
}

pub fn f216(v0: Bytes, v1: u64) -> R<u64> {
    let (h836, _) = f191(Fn44::F243(v0.clone()), {
        let a838 = {
            let a837 = v0;
            length_bytes(a837)
        };
        let a839 = 1u64;
        nat_add(a838, a839)?
    }, v1)?;
    Ok(h836)
}

pub fn f217(v0: Bytes, v1: Bytes) -> bool {
    let m844 = {
        let a841 = v0;
        let a842 = 0u64;
        let a843 = {
            let a840 = v1.clone();
            length_bytes(a840)
        };
        slice_bytes(a841, a842, a843)
    };
    match m844 {
        None => {
            false
        }
        Some(v2) => {
            let a845 = v2;
            let a846 = v1;
            equal(a845, a846)
        }
    }
}

pub fn f218(v0: u64) -> R<Bytes> {
    let a847 = v0;
    let a848 = 128u64;
    if nat_lt(a847, a848) {
        Ok(f222(v0))
    } else {
        let a849 = v0;
        let a850 = 2048u64;
        if nat_lt(a849, a850) {
            Ok(f11(f222({
                let a854 = 192u64;
                let a855 = {
                    let a851 = v0;
                    let a852 = 64u64;
                    let a853 = 0u64;
                    nat_quot(a851, a852, a853)
                };
                nat_add(a854, a855)?
            }), f222({
                let a859 = 128u64;
                let a860 = {
                    let a856 = v0;
                    let a857 = 64u64;
                    let a858 = 0u64;
                    nat_rem(a856, a857, a858)
                };
                nat_add(a859, a860)?
            })))
        } else {
            let a861 = v0;
            let a862 = 65536u64;
            if nat_lt(a861, a862) {
                Ok(f11(f11(f222({
                    let a866 = 224u64;
                    let a867 = {
                        let a863 = v0;
                        let a864 = 4096u64;
                        let a865 = 0u64;
                        nat_quot(a863, a864, a865)
                    };
                    nat_add(a866, a867)?
                }), f222({
                    let a874 = 128u64;
                    let a875 = {
                        let a871 = {
                            let a868 = v0;
                            let a869 = 64u64;
                            let a870 = 0u64;
                            nat_quot(a868, a869, a870)
                        };
                        let a872 = 64u64;
                        let a873 = 0u64;
                        nat_rem(a871, a872, a873)
                    };
                    nat_add(a874, a875)?
                })), f222({
                    let a879 = 128u64;
                    let a880 = {
                        let a876 = v0;
                        let a877 = 64u64;
                        let a878 = 0u64;
                        nat_rem(a876, a877, a878)
                    };
                    nat_add(a879, a880)?
                })))
            } else {
                Ok(f11(f11(f11(f222({
                    let a884 = 240u64;
                    let a885 = {
                        let a881 = v0;
                        let a882 = 262144u64;
                        let a883 = 0u64;
                        nat_quot(a881, a882, a883)
                    };
                    nat_add(a884, a885)?
                }), f222({
                    let a892 = 128u64;
                    let a893 = {
                        let a889 = {
                            let a886 = v0;
                            let a887 = 4096u64;
                            let a888 = 0u64;
                            nat_quot(a886, a887, a888)
                        };
                        let a890 = 64u64;
                        let a891 = 0u64;
                        nat_rem(a889, a890, a891)
                    };
                    nat_add(a892, a893)?
                })), f222({
                    let a900 = 128u64;
                    let a901 = {
                        let a897 = {
                            let a894 = v0;
                            let a895 = 64u64;
                            let a896 = 0u64;
                            nat_quot(a894, a895, a896)
                        };
                        let a898 = 64u64;
                        let a899 = 0u64;
                        nat_rem(a897, a898, a899)
                    };
                    nat_add(a900, a901)?
                })), f222({
                    let a905 = 128u64;
                    let a906 = {
                        let a902 = v0;
                        let a903 = 64u64;
                        let a904 = 0u64;
                        nat_rem(a902, a903, a904)
                    };
                    nat_add(a905, a906)?
                })))
            }
        }
    }
}

pub fn f219(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f220(v0: Adt4) -> R<Bytes> {
    Ok(f11(f11(f36(match v0.clone() {
        Adt4::C0(h907, _) => {
            h907
        }
    })?, f2(Str::lit(" := "))), {
        let m909 = match v0 {
            Adt4::C0(_, h908) => {
                h908
            }
        };
        match m909 {
            Adt5::C0(v1) => {
                if v1 {
                    f2(Str::lit("bool true"))
                } else {
                    f2(Str::lit("bool false"))
                }
            }
            Adt5::C1(v2) => {
                f11(f2(Str::lit("nat ")), if f165(v2.clone())? {
                    v2
                } else {
                    f76(v2)?
                })
            }
            Adt5::C2(v3) => {
                f11(f2(Str::lit("int ")), if f244(v3.clone())? {
                    v3
                } else {
                    f76(v3)?
                })
            }
            Adt5::C3(v4) => {
                f11(f2(Str::lit("str ")), f76(v4)?)
            }
            Adt5::C4(v5) => {
                f11(f2(Str::lit("name ")), f36(v5)?)
            }
        }
    }))
}

pub fn f221(v0: List<List<(Bytes, u64)>>) -> R<List<List<(Bytes, u64)>>> {
    Ok(f112({
        let (_, h910) = {
            let (h911, _) = f245(Fn51::F246, {
                let a913 = {
                    let a912 = v0.clone();
                    length_list(a912)
                };
                let a914 = 1u64;
                nat_add(a913, a914)?
            }, (v0, List::<List<(Bytes, u64)>>::nil()))?;
            h911
        };
        h910
    }))
}

pub fn f222(v0: u64) -> Bytes {
    f236(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f223(v0: Adt18, v1: Bytes, v2: Bytes, v3: u64) -> R<Adt18> {
    Ok(Adt18::C0({
        let a916 = match v0.clone() {
            Adt18::C0(h915, _) => {
                h915
            }
        };
        let a917 = 1u64;
        nat_add(a916, a917)?
    }, if match v0.clone() {
        Adt18::C0(_, h918) => {
            h918
        }
    } {
        true
    } else {
        f40({
            let a920 = match v0 {
                Adt18::C0(h919, _) => {
                    h919
                }
            };
            let a921 = v3;
            nat_lt(a920, a921)
        }, f1(v1, v2))
    }))
}

pub fn f224(v0: Bytes) -> R<u64> {
    let (_, h922) = {
        let (h923, _) = f247(Fn52::F248(v0.clone()), {
            let a924 = v0;
            length_bytes(a924)
        }, (0u64, 0u64))?;
        h923
    };
    Ok(h922)
}

pub fn f225(v0: Adt13, v1: u64, v2: List<Adt14>) -> R<Adt13> {
    Ok(Adt13::C0(f249(match v0 {
        Adt13::C0(h925, _, _) => {
            h925
        }
    }, v1)?, v1, v2))
}

pub fn f226(v0: u64, v1: u64, v2: u64, v3: List<Adt14>, v4: Adt14) -> R<bool> {
    let v5: Adt21 = {
        let (h926, _) = f250(Fn53::F251, {
            let a927 = v0;
            let a928 = 1u64;
            nat_add(a927, a928)?
        }, Adt21::C0(0u64, {
            let a929 = v1;
            let a930 = v2;
            nat_sub(a929, a930)
        }, {
            let a931 = v1;
            let a932 = v2;
            nat_lt(a931, a932)
        }, List::cons(v4, List::<Adt14>::nil()), v3, None::<bool>))?;
        h926
    };
    let m934 = match v5 {
        Adt21::C0(_, _, _, _, _, h933) => {
            h933
        }
    };
    match m934 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f227(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m935 = v0.clone();
    match m935.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m936 = v2;
            match m936.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f252(Fn54::F253, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m938 = {
                        let (h937, _) = v5;
                        h937
                    };
                    match m938 {
                        None => {
                            Some(f126({
                                let (_, h940) = v5;
                                h940
                            }))
                        }
                        Some(v6) => {
                            Some(f126(List::cons(v6, {
                                let (_, h939) = v5;
                                h939
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f228(v0: Bytes, v1: u64) -> Option<u8> {
    let a941 = v0;
    let a942 = v1;
    index_bytes(a941, a942)
}

pub fn f229(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m943 = f228(v0, v1);
    match m943 {
        None => {
            false
        }
        Some(v4) => {
            f254(v4, v2, v3)
        }
    }
}

pub fn f230(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m944 = f228(v0, v1);
    match m944 {
        None => {
            false
        }
        Some(v3) => {
            let a945 = v3;
            let a946 = v2;
            equal(a945, a946)
        }
    }
}

pub fn f231(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f188(v0.clone(), v1)?;
    if if f189(v2) {
        true
    } else {
        f255(v2)
    } {
        Ok(true)
    } else {
        if f119(f236(v0.clone(), v1, 1u64), List::cons(f2(Str::lit("_")), List::cons(f2(Str::lit("'")), List::cons(f2(Str::lit("!")), List::cons(f2(Str::lit("?")), List::cons(f2(Str::lit(".")), List::cons(f2(Str::lit("@")), List::cons(f2(Str::lit("#")), List::<Bytes>::nil())))))))) {
            Ok(true)
        } else {
            let a947 = v2;
            let a948 = 128u64;
            if nat_lt(a947, a948) {
                Ok(false)
            } else {
                let a949 = v2;
                let a950 = 192u64;
                if nat_lt(a949, a950) {
                    Ok(true)
                } else {
                    let a951 = f190(v0, v1)?;
                    let a952 = 0u64;
                    Ok(nat_eq(a951, a952))
                }
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: Adt19) -> R<Option<Adt19>> {
    let v2: u64 = match v1.clone() {
        Adt19::C0(h953, _) => {
            h953
        }
    };
    if if match v1 {
        Adt19::C0(_, h954) => {
            h954
        }
    } {
        true
    } else {
        let a956 = {
            let a955 = v0.clone();
            length_bytes(a955)
        };
        let a957 = v2;
        nat_le(a956, a957)
    } {
        Ok(None::<Adt19>)
    } else {
        if f255(f188(v0.clone(), v2)?) {
            let v3: u64 = f216(v0.clone(), v2)?;
            let v4: u64 = f188(v0.clone(), {
                let a958 = v2;
                let a959 = 1u64;
                nat_sub(a958, a959)
            })?;
            let v5: bool = f40({
                let a960 = 0u64;
                let a961 = v2;
                nat_lt(a960, a961)
            }, if f189(v4) {
                true
            } else {
                let a962 = v4;
                let a963 = 95u64;
                if nat_eq(a962, a963) {
                    true
                } else {
                    let a964 = v4;
                    let a965 = 39u64;
                    nat_eq(a964, a965)
                }
            });
            Ok(Some(Adt19::C0(v3, f40({
                let a968 = 1u64;
                let a969 = {
                    let a966 = v3;
                    let a967 = v2;
                    nat_sub(a966, a967)
                };
                nat_lt(a968, a969)
            }, f40({
                let a970 = f188(v0, v2)?;
                let a971 = 48u64;
                nat_eq(a970, a971)
            }, !v5)))))
        } else {
            Ok(Some(Adt19::C0({
                let a972 = v2;
                let a973 = 1u64;
                nat_add(a972, a973)?
            }, false)))
        }
    }
}

pub fn f233(v0: Bytes, v1: u64) -> R<u64> {
    let m974 = f228(v0, v1);
    match m974 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1019 = {
                let a995 = {
                    let a983 = {
                        let a977 = {
                            let a975 = v3;
                            let a976 = 128u8;
                            fixed_u8::bit_and(a975, a976)
                        };
                        let a978 = 128u8;
                        if equal(a977, a978) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a984 = {
                        let a981 = {
                            let a979 = v3;
                            let a980 = 64u8;
                            fixed_u8::bit_and(a979, a980)
                        };
                        let a982 = 64u8;
                        if equal(a981, a982) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a983, a984)?
                };
                let a996 = {
                    let a993 = {
                        let a987 = {
                            let a985 = v3;
                            let a986 = 32u8;
                            fixed_u8::bit_and(a985, a986)
                        };
                        let a988 = 32u8;
                        if equal(a987, a988) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a994 = {
                        let a991 = {
                            let a989 = v3;
                            let a990 = 16u8;
                            fixed_u8::bit_and(a989, a990)
                        };
                        let a992 = 16u8;
                        if equal(a991, a992) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a993, a994)?
                };
                nat_add(a995, a996)?
            };
            let a1020 = {
                let a1017 = {
                    let a1005 = {
                        let a999 = {
                            let a997 = v3;
                            let a998 = 8u8;
                            fixed_u8::bit_and(a997, a998)
                        };
                        let a1000 = 8u8;
                        if equal(a999, a1000) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1006 = {
                        let a1003 = {
                            let a1001 = v3;
                            let a1002 = 4u8;
                            fixed_u8::bit_and(a1001, a1002)
                        };
                        let a1004 = 4u8;
                        if equal(a1003, a1004) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1005, a1006)?
                };
                let a1018 = {
                    let a1015 = {
                        let a1009 = {
                            let a1007 = v3;
                            let a1008 = 2u8;
                            fixed_u8::bit_and(a1007, a1008)
                        };
                        let a1010 = 2u8;
                        if equal(a1009, a1010) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1016 = {
                        let a1013 = {
                            let a1011 = v3;
                            let a1012 = 1u8;
                            fixed_u8::bit_and(a1011, a1012)
                        };
                        let a1014 = 1u8;
                        if equal(a1013, a1014) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1015, a1016)?
                };
                nat_add(a1017, a1018)?
            };
            nat_add(a1019, a1020)
        }
    }
}

pub fn f234(v0: u64) -> Bytes {
    let a1021 = v0;
    let a1022 = 34u64;
    if nat_eq(a1021, a1022) {
        f135(Str::lit("\\\""))
    } else {
        let a1023 = v0;
        let a1024 = 92u64;
        if nat_eq(a1023, a1024) {
            f135(Str::lit("\\\\"))
        } else {
            let a1025 = v0;
            let a1026 = 8u64;
            if nat_eq(a1025, a1026) {
                f135(Str::lit("\\b"))
            } else {
                let a1027 = v0;
                let a1028 = 12u64;
                if nat_eq(a1027, a1028) {
                    f135(Str::lit("\\f"))
                } else {
                    let a1029 = v0;
                    let a1030 = 10u64;
                    if nat_eq(a1029, a1030) {
                        f135(Str::lit("\\n"))
                    } else {
                        let a1031 = v0;
                        let a1032 = 13u64;
                        if nat_eq(a1031, a1032) {
                            f135(Str::lit("\\r"))
                        } else {
                            let a1033 = v0;
                            let a1034 = 9u64;
                            if nat_eq(a1033, a1034) {
                                f135(Str::lit("\\t"))
                            } else {
                                f134(f135(Str::lit("\\u00")), f256(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f235(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1035, _) = f191(Fn44::F257(v1, v0), {
        let a1036 = v1;
        let a1037 = v2;
        nat_sub(a1036, a1037)
    }, v2)?;
    Ok(h1035)
}

pub fn f236(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m1041 = {
        let a1038 = v0;
        let a1039 = v1;
        let a1040 = v2;
        slice_bytes(a1038, a1039, a1040)
    };
    match m1041 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f237(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1042, _) = v2;
        h1042
    };
    let a1043 = v3;
    let a1044 = 0u64;
    if nat_eq(a1043, a1044) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1045 = v3;
            let a1046 = v1;
            let a1047 = 0u64;
            nat_quot(a1045, a1046, a1047)
        }, {
            let a1052 = f196(v0, {
                let a1048 = v3;
                let a1049 = v1;
                let a1050 = 0u64;
                nat_rem(a1048, a1049, a1050)
            });
            let a1053 = {
                let (_, h1051) = v2;
                h1051
            };
            append_bytes(a1052, a1053)
        }))
    }
}

pub fn f238(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f258(v0, v1)
}

pub fn f239(v0: Bytes) -> u64 {
    let a1054 = v0.clone();
    let a1055 = Bytes::lit(&[0x6e, 0x61, 0x6d, 0x65]);
    if equal(a1054, a1055) {
        0u64
    } else {
        let a1056 = v0.clone();
        let a1057 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
        if equal(a1056, a1057) {
            1u64
        } else {
            let a1058 = v0.clone();
            let a1059 = Bytes::lit(&[0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
            if equal(a1058, a1059) {
                2u64
            } else {
                let a1060 = v0.clone();
                let a1061 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65]);
                if equal(a1060, a1061) {
                    3u64
                } else {
                    let a1062 = v0.clone();
                    let a1063 = Bytes::lit(&[0x72, 0x65, 0x73, 0x75, 0x6c, 0x74]);
                    if equal(a1062, a1063) {
                        4u64
                    } else {
                        let a1064 = v0.clone();
                        let a1065 = Bytes::lit(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74]);
                        if equal(a1064, a1065) {
                            5u64
                        } else {
                            let a1066 = v0.clone();
                            let a1067 = Bytes::lit(&[0x66, 0x69, 0x65, 0x6c, 0x64, 0x73]);
                            if equal(a1066, a1067) {
                                6u64
                            } else {
                                let a1068 = v0.clone();
                                let a1069 = Bytes::lit(&[0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x73]);
                                if equal(a1068, a1069) {
                                    7u64
                                } else {
                                    let a1070 = v0.clone();
                                    let a1071 = Bytes::lit(&[0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73]);
                                    if equal(a1070, a1071) {
                                        8u64
                                    } else {
                                        let a1072 = v0.clone();
                                        let a1073 = Bytes::lit(&[0x62, 0x6f, 0x64, 0x79]);
                                        if equal(a1072, a1073) {
                                            9u64
                                        } else {
                                            let a1074 = v0.clone();
                                            let a1075 = Bytes::lit(&[0x70, 0x72, 0x6f, 0x6f, 0x66]);
                                            if equal(a1074, a1075) {
                                                10u64
                                            } else {
                                                let a1076 = v0;
                                                let a1077 = Bytes::lit(&[0x61, 0x78, 0x69, 0x6f, 0x6d, 0x73]);
                                                if equal(a1076, a1077) {
                                                    11u64
                                                } else {
                                                    12u64
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f240(v0: Fn50, p1081: List<List<(Bytes, Adt8)>>, p1082: List<List<(Bytes, Adt8)>>) -> List<List<(Bytes, Adt8)>> {
    let m1080 = (p1081, p1082);
    {
        let mut m1080 = m1080;
        loop {
            let step = {
                let (v1, v2) = m1080;
                let m1078 = v2;
                match m1078.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), List<List<(Bytes, Adt8)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), List<List<(Bytes, Adt8)>>>(({
                            let c1079: Fn50 = v0.clone();
                            c1079.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1080 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: List<List<(Bytes, Adt8)>>, v1: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
    List::cons(v1, v0)
}

pub fn f242(v0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
    let a1084 = {
        let a1083 = v0.clone();
        length_list(a1083)
    };
    let a1085 = 1u64;
    if nat_le(a1084, a1085) {
        Ok(None::<List<List<(Bytes, Adt8)>>>)
    } else {
        Ok(Some(f259(v0)?))
    }
}

pub fn f243(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f40({
        let a1087 = v1;
        let a1088 = {
            let a1086 = v0.clone();
            length_bytes(a1086)
        };
        nat_lt(a1087, a1088)
    }, f255(f188(v0, v1)?)) {
        Ok(Some({
            let a1089 = v1;
            let a1090 = 1u64;
            nat_add(a1089, a1090)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f244(v0: Bytes) -> R<bool> {
    let v1: Bytes = if f217(v0.clone(), f2(Str::lit("-"))) {
        f236(v0.clone(), 1u64, {
            let a1092 = {
                let a1091 = v0.clone();
                length_bytes(a1091)
            };
            let a1093 = 1u64;
            nat_sub(a1092, a1093)
        })
    } else {
        v0.clone()
    };
    Ok(f40(f165(v1)?, !f1(v0, f2(Str::lit("-0")))))
}

pub fn f245(v0: Fn51, p1098: u64, p1099: (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)) -> R<((List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), bool)> {
    let m1097 = (p1098, p1099);
    {
        let mut m1097 = m1097;
        loop {
            let step = {
                let (v1, v2) = m1097;
                let m1094 = v1;
                if m1094 == 0 {
                    Err::<(u64, (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)), R<((List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1094 - 1;
                    let m1096 = {
                        let c1095: Fn51 = v0.clone();
                        c1095.apply(v2.clone())?
                    };
                    match m1096 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)), R<((List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)), R<((List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1097 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)) -> R<Option<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)>> {
    f260(v0)
}

pub fn f247(v0: Fn52, p1104: u64, p1105: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1103 = (p1104, p1105);
    {
        let mut m1103 = m1103;
        loop {
            let step = {
                let (v1, v2) = m1103;
                let m1100 = v1;
                if m1100 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1100 - 1;
                    let m1102 = {
                        let c1101: Fn52 = v0.clone();
                        c1101.apply(v2)?
                    };
                    match m1102 {
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
                Ok(next) => { m1103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f248(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f261(v0, v1)
}

pub fn f249(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a1106 = v1;
        let a1107 = 256u64;
        let a1108 = 0u64;
        nat_quot(a1106, a1107, a1108)
    };
    let v3: List<Bytes> = {
        let (_, h1109) = {
            let (h1110, _) = f262(Fn55::F263(v2), {
                let a1111 = v2;
                let a1112 = 1u64;
                nat_add(a1111, a1112)?
            }, (0u64, List::cons({
                let a1113 = Str::lit("\u{a}");
                utf8_encode(a1113)
            }, v0)))?;
            h1110
        };
        h1109
    };
    Ok(List::cons(f236(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a1114 = v1;
        let a1115 = 256u64;
        let a1116 = 0u64;
        nat_rem(a1114, a1115, a1116)
    }), v3))
}

pub fn f250(v0: Fn53, p1121: u64, p1122: Adt21) -> R<(Adt21, bool)> {
    let m1120 = (p1121, p1122);
    {
        let mut m1120 = m1120;
        loop {
            let step = {
                let (v1, v2) = m1120;
                let m1117 = v1;
                if m1117 == 0 {
                    Err::<(u64, Adt21), R<(Adt21, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1117 - 1;
                    let m1119 = {
                        let c1118: Fn53 = v0.clone();
                        c1118.apply(v2.clone())?
                    };
                    match m1119 {
                        None => {
                            Err::<(u64, Adt21), R<(Adt21, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt21), R<(Adt21, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: Adt21) -> R<Option<Adt21>> {
    f264(v0)
}

pub fn f252(v0: Fn54, p1126: (Option<Bytes>, List<Bytes>), p1127: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m1125 = (p1126, p1127);
    {
        let mut m1125 = m1125;
        loop {
            let step = {
                let (v1, v2) = m1125;
                let m1123 = v2;
                match m1123.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c1124: Fn54 = v0.clone();
                            c1124.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f265(v0, v1)
}

pub fn f254(v0: u8, v1: u8, v2: u8) -> bool {
    let m1130 = {
        let a1128 = v0;
        let a1129 = v1;
        fixed_u8::checked_sub(a1128, a1129)
    };
    match m1130 {
        None => {
            false
        }
        Some(_) => {
            let m1133 = {
                let a1131 = v2;
                let a1132 = v0;
                fixed_u8::checked_sub(a1131, a1132)
            };
            match m1133 {
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

pub fn f255(v0: u64) -> bool {
    f40({
        let a1134 = 48u64;
        let a1135 = v0;
        nat_le(a1134, a1135)
    }, {
        let a1136 = v0;
        let a1137 = 57u64;
        nat_le(a1136, a1137)
    })
}

pub fn f256(v0: u64) -> Bytes {
    f134(f266(v0, 16u64), f267(v0))
}

pub fn f257(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f268(v1, v0, v2)
}

pub fn f258(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f269(f200(v0, v1)?) {
        Ok(Some({
            let a1138 = v1;
            let a1139 = 1u64;
            nat_add(a1138, a1139)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f259(v0: List<List<(Bytes, Adt8)>>) -> R<List<List<(Bytes, Adt8)>>> {
    Ok(f205({
        let (_, h1140) = {
            let (h1141, _) = f270(Fn56::F271, {
                let a1143 = {
                    let a1142 = v0.clone();
                    length_list(a1142)
                };
                let a1144 = 1u64;
                nat_add(a1143, a1144)?
            }, (v0, List::<List<(Bytes, Adt8)>>::nil()))?;
            h1141
        };
        h1140
    }))
}

pub fn f260(v0: (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)) -> R<Option<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)>> {
    let v1: List<List<(Bytes, u64)>> = {
        let (h1145, _) = v0;
        h1145
    };
    let v2: List<List<(Bytes, u64)>> = {
        let (_, h1146) = v0;
        h1146
    };
    let m1147 = v1;
    match m1147.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)>)
        }
        Some((v3, v4)) => {
            let m1148 = v4;
            match m1148.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, u64)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f272(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f261(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1150 = f228(v0, {
        let (h1149, _) = v1;
        h1149
    });
    match m1150 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1195 = {
                    let a1171 = {
                        let a1159 = {
                            let a1153 = {
                                let a1151 = v3;
                                let a1152 = 128u8;
                                fixed_u8::bit_and(a1151, a1152)
                            };
                            let a1154 = 128u8;
                            if equal(a1153, a1154) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1160 = {
                            let a1157 = {
                                let a1155 = v3;
                                let a1156 = 64u8;
                                fixed_u8::bit_and(a1155, a1156)
                            };
                            let a1158 = 64u8;
                            if equal(a1157, a1158) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1159, a1160)?
                    };
                    let a1172 = {
                        let a1169 = {
                            let a1163 = {
                                let a1161 = v3;
                                let a1162 = 32u8;
                                fixed_u8::bit_and(a1161, a1162)
                            };
                            let a1164 = 32u8;
                            if equal(a1163, a1164) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1170 = {
                            let a1167 = {
                                let a1165 = v3;
                                let a1166 = 16u8;
                                fixed_u8::bit_and(a1165, a1166)
                            };
                            let a1168 = 16u8;
                            if equal(a1167, a1168) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1169, a1170)?
                    };
                    nat_add(a1171, a1172)?
                };
                let a1196 = {
                    let a1193 = {
                        let a1181 = {
                            let a1175 = {
                                let a1173 = v3;
                                let a1174 = 8u8;
                                fixed_u8::bit_and(a1173, a1174)
                            };
                            let a1176 = 8u8;
                            if equal(a1175, a1176) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1182 = {
                            let a1179 = {
                                let a1177 = v3;
                                let a1178 = 4u8;
                                fixed_u8::bit_and(a1177, a1178)
                            };
                            let a1180 = 4u8;
                            if equal(a1179, a1180) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1181, a1182)?
                    };
                    let a1194 = {
                        let a1191 = {
                            let a1185 = {
                                let a1183 = v3;
                                let a1184 = 2u8;
                                fixed_u8::bit_and(a1183, a1184)
                            };
                            let a1186 = 2u8;
                            if equal(a1185, a1186) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1192 = {
                            let a1189 = {
                                let a1187 = v3;
                                let a1188 = 1u8;
                                fixed_u8::bit_and(a1187, a1188)
                            };
                            let a1190 = 1u8;
                            if equal(a1189, a1190) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1191, a1192)?
                    };
                    nat_add(a1193, a1194)?
                };
                nat_add(a1195, a1196)?
            };
            let a1197 = v4;
            let a1198 = 128u64;
            if nat_lt(a1197, a1198) {
                Ok(Some(({
                    let a1200 = {
                        let (h1199, _) = v1;
                        h1199
                    };
                    let a1201 = 1u64;
                    nat_add(a1200, a1201)?
                }, {
                    let a1203 = {
                        let (_, h1202) = v1;
                        h1202
                    };
                    let a1204 = 1u64;
                    nat_add(a1203, a1204)?
                })))
            } else {
                let a1205 = v4;
                let a1206 = 192u64;
                if nat_lt(a1205, a1206) {
                    Ok(Some(({
                        let a1208 = {
                            let (h1207, _) = v1;
                            h1207
                        };
                        let a1209 = 1u64;
                        nat_add(a1208, a1209)?
                    }, {
                        let (_, h1210) = v1;
                        h1210
                    })))
                } else {
                    Ok(Some(({
                        let a1212 = {
                            let (h1211, _) = v1;
                            h1211
                        };
                        let a1213 = 1u64;
                        nat_add(a1212, a1213)?
                    }, {
                        let a1215 = {
                            let (_, h1214) = v1;
                            h1214
                        };
                        let a1216 = 1u64;
                        nat_add(a1215, a1216)?
                    })))
                }
            }
        }
    }
}

pub fn f262(v0: Fn55, p1221: u64, p1222: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m1220 = (p1221, p1222);
    {
        let mut m1220 = m1220;
        loop {
            let step = {
                let (v1, v2) = m1220;
                let m1217 = v1;
                if m1217 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1217 - 1;
                    let m1219 = {
                        let c1218: Fn55 = v0.clone();
                        c1218.apply(v2.clone())?
                    };
                    match m1219 {
                        None => {
                            Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f273(v0, v1)
}

pub fn f264(v0: Adt21) -> R<Option<Adt21>> {
    let m1224 = match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1223) => {
            h1223
        }
    };
    match m1224 {
        None => {
            if if match v0.clone() {
                Adt21::C0(_, _, h1225, _, _, _) => {
                    h1225
                }
            } {
                true
            } else {
                let a1228 = match v0.clone() {
                    Adt21::C0(_, h1226, _, _, _, _) => {
                        h1226
                    }
                };
                let a1229 = match v0.clone() {
                    Adt21::C0(h1227, _, _, _, _, _) => {
                        h1227
                    }
                };
                nat_lt(a1228, a1229)
            } {
                Ok(Some(f274(v0.clone(), match v0.clone() {
                    Adt21::C0(h1230, _, _, _, _, _) => {
                        h1230
                    }
                }, match v0.clone() {
                    Adt21::C0(_, _, _, h1231, _, _) => {
                        h1231
                    }
                }, match v0 {
                    Adt21::C0(_, _, _, _, h1232, _) => {
                        h1232
                    }
                }, Some(false))))
            } else {
                let m1234 = match v0.clone() {
                    Adt21::C0(_, _, _, h1233, _, _) => {
                        h1233
                    }
                };
                match m1234.uncons() {
                    None => {
                        let m1237 = match v0.clone() {
                            Adt21::C0(_, _, _, _, h1236, _) => {
                                h1236
                            }
                        };
                        match m1237.uncons() {
                            None => {
                                Ok(Some(f274(v0.clone(), match v0 {
                                    Adt21::C0(h1238, _, _, _, _, _) => {
                                        h1238
                                    }
                                }, List::<Adt14>::nil(), List::<Adt14>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f275(v0, v4, List::<Adt14>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f275(v0.clone(), v2, v3, match v0 {
                            Adt21::C0(_, _, _, _, h1235, _) => {
                                h1235
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt21>)
        }
    }
}

pub fn f265(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m1240 = {
        let (h1239, _) = v0;
        h1239
    };
    match m1240 {
        None => {
            (Some(v1), {
                let (_, h1241) = v0;
                h1241
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a1242 = v2;
                let a1243 = v1;
                append_bytes(a1242, a1243)
            }, {
                let (_, h1244) = v0;
                h1244
            }))
        }
    }
}

pub fn f266(v0: u64, v1: u64) -> Bytes {
    f267({
        let a1245 = v0;
        let a1246 = v1;
        let a1247 = 0u64;
        nat_quot(a1245, a1246, a1247)
    })
}

pub fn f267(v0: u64) -> Bytes {
    f236(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1248 = v0;
        let a1249 = 16u64;
        let a1250 = 0u64;
        nat_rem(a1248, a1249, a1250)
    }, 1u64)
}

pub fn f268(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1251 = v2;
    let a1252 = v1;
    if nat_lt(a1251, a1252) {
        let v3: u64 = f233(v0, v2)?;
        let a1253 = v3;
        let a1254 = 32u64;
        if nat_lt(a1253, a1254) {
            Ok(None::<u64>)
        } else {
            let a1255 = v3;
            let a1256 = 34u64;
            if nat_eq(a1255, a1256) {
                Ok(None::<u64>)
            } else {
                let a1257 = v3;
                let a1258 = 92u64;
                if nat_eq(a1257, a1258) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1259 = v2;
                        let a1260 = 1u64;
                        nat_add(a1259, a1260)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f269(v0: u64) -> bool {
    if f199(v0) {
        true
    } else {
        let a1261 = 47u64;
        let a1262 = v0;
        if if nat_lt(a1261, a1262) {
            let a1263 = v0;
            let a1264 = 58u64;
            nat_lt(a1263, a1264)
        } else {
            false
        } {
            true
        } else {
            let a1265 = v0;
            let a1266 = 46u64;
            if nat_eq(a1265, a1266) {
                true
            } else {
                let a1267 = v0;
                let a1268 = 39u64;
                if nat_eq(a1267, a1268) {
                    true
                } else {
                    let a1269 = v0;
                    let a1270 = 47u64;
                    if nat_eq(a1269, a1270) {
                        true
                    } else {
                        let a1271 = v0;
                        let a1272 = 45u64;
                        nat_eq(a1271, a1272)
                    }
                }
            }
        }
    }
}

pub fn f270(v0: Fn56, p1277: u64, p1278: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)> {
    let m1276 = (p1277, p1278);
    {
        let mut m1276 = m1276;
        loop {
            let step = {
                let (v1, v2) = m1276;
                let m1273 = v1;
                if m1273 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1273 - 1;
                    let m1275 = {
                        let c1274: Fn56 = v0.clone();
                        c1274.apply(v2.clone())?
                    };
                    match m1275 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f271(v0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
    f276(v0)
}

pub fn f272(v0: List<(Bytes, u64)>, v1: List<(Bytes, u64)>) -> R<List<(Bytes, u64)>> {
    let v2: u64 = {
        let a1283 = {
            let a1281 = {
                let a1279 = v0.clone();
                length_list(a1279)
            };
            let a1282 = {
                let a1280 = v1.clone();
                length_list(a1280)
            };
            nat_add(a1281, a1282)?
        };
        let a1284 = 1u64;
        nat_add(a1283, a1284)?
    };
    Ok(f61({
        let (_, h1285) = {
            let (_, h1286) = {
                let (h1287, _) = f277(Fn57::F278, v2, (v0, (v1, List::<(Bytes, u64)>::nil())));
                h1287
            };
            h1286
        };
        h1285
    }))
}

pub fn f273(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a1289 = {
        let (h1288, _) = v1.clone();
        h1288
    };
    let a1290 = v0;
    if nat_lt(a1289, a1290) {
        Ok(Some(({
            let a1292 = {
                let (h1291, _) = v1;
                h1291
            };
            let a1293 = 1u64;
            nat_add(a1292, a1293)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h1294) = v1;
            h1294
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f274(v0: Adt21, v1: u64, v2: List<Adt14>, v3: List<Adt14>, v4: Option<bool>) -> Adt21 {
    Adt21::C0(v1, match v0.clone() {
        Adt21::C0(_, h1295, _, _, _, _) => {
            h1295
        }
    }, match v0 {
        Adt21::C0(_, _, h1296, _, _, _) => {
            h1296
        }
    }, v2, v3, v4)
}

pub fn f275(v0: Adt21, v1: Adt14, v2: List<Adt14>, v3: List<Adt14>) -> R<Adt21> {
    let v4: u64 = match v1.clone() {
        Adt14::C0(h1297, _, _) => {
            h1297
        }
    };
    let v5: bool = match v1.clone() {
        Adt14::C0(_, h1298, _) => {
            h1298
        }
    };
    let v6: u64 = match v0.clone() {
        Adt21::C0(h1299, _, _, _, _, _) => {
            h1299
        }
    };
    let m1301 = match v1 {
        Adt14::C0(_, _, h1300) => {
            h1300
        }
    };
    match m1301 {
        Adt8::C0 => {
            Ok(f274(v0, v6, v2, v3, None::<bool>))
        }
        Adt8::C1(v7) => {
            Ok(f274(v0, {
                let a1302 = v6;
                let a1303 = f224(v7)?;
                nat_add(a1302, a1303)?
            }, v2, v3, None::<bool>))
        }
        Adt8::C2(r1308, r1309) => {
            let v8 = (*r1308).clone();
            let v9 = (*r1309).clone();
            Ok(f274(v0, v6, List::cons(Adt14::C0(v4, v5, v8), List::cons(Adt14::C0(v4, v5, v9), v2)), v3, None::<bool>))
        }
        Adt8::C3(v10, r1310) => {
            let v11 = (*r1310).clone();
            Ok(f274(v0, v6, List::cons(Adt14::C0({
                let a1304 = v4;
                let a1305 = v10;
                nat_add(a1304, a1305)?
            }, v5, v11), v2), v3, None::<bool>))
        }
        Adt8::C4 => {
            if v5 {
                Ok(f274(v0, {
                    let a1306 = v6;
                    let a1307 = 1u64;
                    nat_add(a1306, a1307)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f274(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt8::C5 => {
            if v5 {
                Ok(f274(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f274(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt8::C6(r1311) => {
            let v12 = (*r1311).clone();
            Ok(f274(v0, v6, List::cons(Adt14::C0(v4, true, v12), v2), v3, None::<bool>))
        }
    }
}

pub fn f276(v0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
    let v1: List<List<(Bytes, Adt8)>> = {
        let (h1312, _) = v0;
        h1312
    };
    let v2: List<List<(Bytes, Adt8)>> = {
        let (_, h1313) = v0;
        h1313
    };
    let m1314 = v1;
    match m1314.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>)
        }
        Some((v3, v4)) => {
            let m1315 = v4;
            match m1315.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt8)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f279(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f277(v0: Fn57, p1320: u64, p1321: (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))) -> ((List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>)), bool) {
    let m1319 = (p1320, p1321);
    {
        let mut m1319 = m1319;
        loop {
            let step = {
                let (v1, v2) = m1319;
                let m1316 = v1;
                if m1316 == 0 {
                    Err::<(u64, (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))), ((List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>)), bool)>((v2, false))
                } else {
                    let v3 = m1316 - 1;
                    let m1318 = {
                        let c1317: Fn57 = v0.clone();
                        c1317.apply(v2.clone())
                    };
                    match m1318 {
                        None => {
                            Err::<(u64, (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))), ((List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))), ((List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))) -> Option<(List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))> {
    f280(v0)
}

pub fn f279(v0: List<(Bytes, Adt8)>, v1: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    let v2: u64 = {
        let a1326 = {
            let a1324 = {
                let a1322 = v0.clone();
                length_list(a1322)
            };
            let a1325 = {
                let a1323 = v1.clone();
                length_list(a1323)
            };
            nat_add(a1324, a1325)?
        };
        let a1327 = 1u64;
        nat_add(a1326, a1327)?
    };
    f281({
        let (_, h1328) = {
            let (_, h1329) = {
                let (h1330, _) = f282(Fn58::F283, v2, (v0, (v1, List::<(Bytes, Adt8)>::nil())));
                h1330
            };
            h1329
        };
        h1328
    })
}

pub fn f280(v0: (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))) -> Option<(List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))> {
    let v1: List<(Bytes, u64)> = {
        let (h1331, _) = v0.clone();
        h1331
    };
    let v2: List<(Bytes, u64)> = {
        let (h1332, _) = {
            let (_, h1333) = v0.clone();
            h1333
        };
        h1332
    };
    let v3: List<(Bytes, u64)> = {
        let (_, h1334) = {
            let (_, h1335) = v0;
            h1335
        };
        h1334
    };
    let m1336 = v1.clone();
    match m1336.uncons() {
        None => {
            let m1337 = v2;
            match m1337.uncons() {
                None => {
                    None::<(List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, u64)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1338 = v2.clone();
            match m1338.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, u64)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f284({
                        let (h1339, _) = v8.clone();
                        h1339
                    }, {
                        let (h1340, _) = v6.clone();
                        h1340
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

pub fn f281(v0: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    f146(Fn32::F285, List::<(Bytes, Adt8)>::nil(), v0)
}

pub fn f282(v0: Fn58, p1345: u64, p1346: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool) {
    let m1344 = (p1345, p1346);
    {
        let mut m1344 = m1344;
        loop {
            let step = {
                let (v1, v2) = m1344;
                let m1341 = v1;
                if m1341 == 0 {
                    Err::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v2, false))
                } else {
                    let v3 = m1341 - 1;
                    let m1343 = {
                        let c1342: Fn58 = v0.clone();
                        c1342.apply(v2.clone())
                    };
                    match m1343 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
    f286(v0)
}

pub fn f284(v0: Bytes, v1: Bytes) -> bool {
    let a1351 = {
        let a1347 = v0;
        let a1348 = v1;
        compare_bytes(a1347, a1348)
    };
    let a1352 = {
        let a1349 = Bytes::lit(&[]);
        let a1350 = Bytes::lit(&[0x01]);
        compare_bytes(a1349, a1350)
    };
    equal(a1351, a1352)
}

pub fn f285(v0: List<(Bytes, Adt8)>, v1: (Bytes, Adt8)) -> List<(Bytes, Adt8)> {
    List::cons(v1, v0)
}

pub fn f286(v0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
    let v1: List<(Bytes, Adt8)> = {
        let (h1353, _) = v0.clone();
        h1353
    };
    let v2: List<(Bytes, Adt8)> = {
        let (h1354, _) = {
            let (_, h1355) = v0.clone();
            h1355
        };
        h1354
    };
    let v3: List<(Bytes, Adt8)> = {
        let (_, h1356) = {
            let (_, h1357) = v0;
            h1357
        };
        h1356
    };
    let m1358 = v1.clone();
    match m1358.uncons() {
        None => {
            let m1359 = v2;
            match m1359.uncons() {
                None => {
                    None::<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt8)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1360 = v2.clone();
            match m1360.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt8)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f284({
                        let (h1361, _) = v8.clone();
                        h1361
                    }, {
                        let (h1362, _) = v6.clone();
                        h1362
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
