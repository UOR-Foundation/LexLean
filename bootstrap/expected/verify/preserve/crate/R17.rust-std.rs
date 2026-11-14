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
    C0(List<Bytes>, List<Adt1>, List<u64>, List<Adt6>, List<Adt7>),
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
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Adt10, List<Adt11>, List<u64>, List<Adt12>, List<Adt1>, u64, List<u64>, List<Bytes>, List<Adt13>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64),
    C1(u64),
    C2,
    C3(u64),
    C4,
    C5(u64),
    C6,
    C7(u64),
    C8(u64),
    C9,
    C10(u64),
    C11,
    C12(u64),
    C13(u64),
    C14(u64),
    C15(u64),
    C16,
    C17,
    C18,
    C19,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, Bytes),
    C1(Bytes, bool),
    C2(Bytes, Bytes, u64),
    C3(List<Adt4>),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt9, List<Adt15>, List<Adt7>, bool, bool),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Adt6, Option<Adt16>, Option<Adt16>, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u64),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(List<Bytes>, Adt10, bool, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Adt18, Adt10),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64, List<Bytes>, bool, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Bytes, Adt10),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(bool, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, Adt6),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u64, u64, Option<Result<u64, Adt8>>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Adt6, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Adt28, u64, u64),
}

#[derive(Clone)]
pub enum Adt28 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11(Bytes),
    C12(Bytes),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(u64, List<Adt27>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(u64, u64, u64, List<Adt31>, Adt6, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(List<Adt6>),
    C1(Bytes, u64, List<(Bytes, Adt6)>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Adt2, Adt10),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt10, List<Adt34>, List<Adt2>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(u64),
    C1(u64),
    C2,
    C3,
    C4(u64, bool),
    C5(bool),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(u64, Adt10),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(List<Adt4>, Adt10, bool, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(List<Adt2>, Adt10),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(List<Adt2>, Adt10, bool, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(u64, Option<u64>),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt14) -> R<Option<Adt14>> {
        match self {
            Fn0::F3(k0) => f3(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9,
    F159,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt6) -> List<Adt6> {
        match self {
            Fn1::F9 => f9(p0, p1),
            Fn1::F159 => f159(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F19(Bytes),
    F75(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt17) -> R<Option<Adt17>> {
        match self {
            Fn2::F19(k0) => f19(k0.clone(), p0),
            Fn2::F75(k0) => f75(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F24(List<Adt13>),
}

impl Fn3 {
    pub fn apply(&self, p0: Result<List<Adt6>, Adt8>, p1: Adt15) -> Result<List<Adt6>, Adt8> {
        match self {
            Fn3::F24(k0) => f24(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F26,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt15>, p1: Adt15) -> List<Adt15> {
        match self {
            Fn4::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F28,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn5::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F30,
}

impl Fn6 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn6::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F32,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt7) -> List<Adt7> {
        match self {
            Fn7::F32 => f32(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F48,
    F251,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn8::F48 => f48(p0, p1),
            Fn8::F251 => f251(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F56(Bytes),
    F61(Bytes),
    F62(Bytes),
    F109(Bytes),
    F167(u64, Bytes),
    F240(u64, Bytes),
    F302(Bytes),
    F303(Bytes),
    F307(Bytes),
    F322(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn9::F56(k0) => f56(k0.clone(), p0),
            Fn9::F61(k0) => f61(k0.clone(), p0),
            Fn9::F62(k0) => f62(k0.clone(), p0),
            Fn9::F109(k0) => f109(k0.clone(), p0),
            Fn9::F167(k0, k1) => f167(*k0, k1.clone(), p0),
            Fn9::F240(k0, k1) => f240(*k0, k1.clone(), p0),
            Fn9::F302(k0) => f302(k0.clone(), p0),
            Fn9::F303(k0) => f303(k0.clone(), p0),
            Fn9::F307(k0) => f307(k0.clone(), p0),
            Fn9::F322(k0) => f322(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F78,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Adt6>, p1: Bytes) -> List<Adt6> {
        match self {
            Fn10::F78 => f78(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F81(u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt20) -> R<Option<Adt20>> {
        match self {
            Fn11::F81(k0, k1) => f81(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F85(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn12::F85(k0) => f85(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F90(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn13::F90(k0) => f90(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F94(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Option<u64>, p1: Adt13) -> Option<u64> {
        match self {
            Fn14::F94(k0) => f94(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F103(Bytes, Adt6),
}

impl Fn15 {
    pub fn apply(&self, p0: Adt22, p1: Adt23) -> Adt22 {
        match self {
            Fn15::F103(k0, k1) => f103(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F106,
}

impl Fn16 {
    pub fn apply(&self, p0: Adt6, p1: Adt23) -> Adt6 {
        match self {
            Fn16::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F112(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: Adt24) -> R<Option<Adt24>> {
        match self {
            Fn17::F112(k0) => f112(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F116(u64, Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn18::F116(k0, k1) => f116(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F118,
}

impl Fn19 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> Bytes {
        match self {
            Fn19::F118 => f118(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F120(u64, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: Adt25) -> R<Option<Adt25>> {
        match self {
            Fn20::F120(k0, k1) => f120(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F124,
    F292,
}

impl Fn21 {
    pub fn apply(&self, p0: Adt6, p1: Adt6) -> Adt6 {
        match self {
            Fn21::F124 => f124(p0, p1),
            Fn21::F292 => f292(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F129,
}

impl Fn22 {
    pub fn apply(&self, p0: Adt26) -> Option<Adt26> {
        match self {
            Fn22::F129 => f129(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F153(u64, Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: Adt29) -> R<Option<Adt29>> {
        match self {
            Fn23::F153(k0, k1) => f153(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F161,
}

impl Fn24 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt23) -> List<Adt23> {
        match self {
            Fn24::F161 => f161(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F186,
}

impl Fn25 {
    pub fn apply(&self, p0: List<Adt27>, p1: Adt27) -> List<Adt27> {
        match self {
            Fn25::F186 => f186(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F188(u64, List<Adt27>),
}

impl Fn26 {
    pub fn apply(&self, p0: Adt30) -> R<Option<Adt30>> {
        match self {
            Fn26::F188(k0, k1) => f188(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F195(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: Adt33) -> R<Option<Adt33>> {
        match self {
            Fn27::F195(k0) => f195(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F197(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: Adt36) -> R<Option<Adt36>> {
        match self {
            Fn28::F197(k0) => f197(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F200(Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: Adt37) -> R<Option<Adt37>> {
        match self {
            Fn29::F200(k0) => f200(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F214,
}

impl Fn30 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn30::F214 => f214(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F217(u64, Bytes),
}

impl Fn31 {
    pub fn apply(&self, p0: Adt39) -> R<Option<Adt39>> {
        match self {
            Fn31::F217(k0, k1) => f217(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F232(u64, Bytes),
}

impl Fn32 {
    pub fn apply(&self, p0: Adt24) -> R<Adt24> {
        match self {
            Fn32::F232(k0, k1) => f232(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F235,
}

impl Fn33 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn33::F235 => f235(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F237(Bytes),
}

impl Fn34 {
    pub fn apply(&self, p0: Adt40, p1: Bytes) -> R<Adt40> {
        match self {
            Fn34::F237(k0) => f237(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F250,
}

impl Fn35 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn35::F250 => f250(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F266,
}

impl Fn36 {
    pub fn apply(&self, p0: Adt2) -> Adt2 {
        match self {
            Fn36::F266 => f266(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F279(Bytes),
}

impl Fn37 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt6)) -> bool {
        match self {
            Fn37::F279(k0) => f279(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F281,
}

impl Fn38 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn38::F281 => f281(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F294,
}

impl Fn39 {
    pub fn apply(&self, p0: Adt6, p1: (Bytes, Adt6)) -> Adt6 {
        match self {
            Fn39::F294 => f294(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn40 {
    F298,
}

impl Fn40 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>, p1: (Bytes, Adt6)) -> List<List<(Bytes, Adt6)>> {
        match self {
            Fn40::F298 => f298(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn41 {
    F300,
}

impl Fn41 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
        match self {
            Fn41::F300 => f300(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn42 {
    F306(Bytes),
}

impl Fn42 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn42::F306(k0) => f306(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn43 {
    F309(u64, Bytes),
}

impl Fn43 {
    pub fn apply(&self, p0: Adt41) -> R<Option<Adt41>> {
        match self {
            Fn43::F309(k0, k1) => f309(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn44 {
    F311,
}

impl Fn44 {
    pub fn apply(&self, p0: List<(Bytes, Adt6)>, p1: (Bytes, Adt6)) -> List<(Bytes, Adt6)> {
        match self {
            Fn44::F311 => f311(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn45 {
    F313,
}

impl Fn45 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>, p1: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
        match self {
            Fn45::F313 => f313(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn46 {
    F327,
}

impl Fn46 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
        match self {
            Fn46::F327 => f327(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn47 {
    F333,
}

impl Fn47 {
    pub fn apply(&self, p0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
        match self {
            Fn47::F333 => f333(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Result<Adt0, Adt8>> {
    let m1 = f1(v0.clone())?;
    match m1 {
        Ok(v2) => {
            let v3: Adt9 = Adt9::C0({
                let (_, h2) = v2;
                h2
            }, List::<Adt11>::nil(), List::<u64>::nil(), List::<Adt12>::nil(), List::<Adt1>::nil(), 0u64, List::<u64>::nil(), List::<Bytes>::nil(), List::<Adt13>::nil(), None::<Adt8>);
            let v4: Adt14 = {
                let (h3, _) = f2(Fn0::F3(v0.clone()), {
                    let a5 = {
                        let a4 = v0;
                        length_bytes(a4)
                    };
                    let a6 = 2u64;
                    nat_add(a5, a6)?
                }, Adt14::C0(v3, List::<Adt15>::nil(), List::<Adt7>::nil(), false, false))?;
                h3
            };
            let v5: Adt9 = match v4.clone() {
                Adt14::C0(h7, _, _, _, _) => {
                    h7
                }
            };
            let m9 = match v5.clone() {
                Adt9::C0(_, _, _, _, _, _, _, _, _, h8) => {
                    h8
                }
            };
            match m9 {
                None => {
                    let m12 = f4(match v5.clone() {
                        Adt9::C0(_, _, _, _, _, _, _, _, h10, _) => {
                            h10
                        }
                    }, f5(match v4.clone() {
                        Adt14::C0(_, h11, _, _, _) => {
                            h11
                        }
                    }));
                    match m12 {
                        Ok(v8) => {
                            Ok(Ok::<Adt0, Adt8>(Adt0::C0({
                                let (h13, _) = v2;
                                h13
                            }, f6(match v5.clone() {
                                Adt9::C0(_, _, _, _, h14, _, _, _, _, _) => {
                                    h14
                                }
                            }), f7(match v5 {
                                Adt9::C0(_, _, _, _, _, _, h15, _, _, _) => {
                                    h15
                                }
                            }), f8(Fn1::F9, List::<Adt6>::nil(), v8), f10(match v4 {
                                Adt14::C0(_, _, h16, _, _) => {
                                    h16
                                }
                            }))))
                        }
                        Err(v7) => {
                            Ok(Err::<Adt0, Adt8>(v7))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Err::<Adt0, Adt8>(v6))
                }
            }
        }
        Err(v1) => {
            Ok(Err::<Adt0, Adt8>(v1))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<(List<Bytes>, Adt10), Adt8>> {
    let m17 = f11(v0.clone(), 0u64)?;
    match m17 {
        Ok(v2) => {
            if f12({
                let a19 = match v2.clone() {
                    Adt18::C0(h18, _, _, _) => {
                        h18
                    }
                };
                let a20 = 20u64;
                nat_eq(a19, a20)
            }, f13(f14(v0.clone(), v2.clone()), f15(Str::lit("imports")))) {
                let m21 = f16(v0.clone(), f17(v2), 4u64, f15(Str::lit("`[` after `imports`")))?;
                match m21 {
                    Ok(v4) => {
                        let v5: Adt17 = {
                            let (h22, _) = f18(Fn2::F19(v0.clone()), {
                                let a24 = {
                                    let a23 = v0.clone();
                                    length_bytes(a23)
                                };
                                let a25 = 2u64;
                                nat_add(a24, a25)?
                            }, Adt17::C0(List::<Bytes>::nil(), match v4 {
                                Adt19::C0(_, h26) => {
                                    h26
                                }
                            }, false, None::<Adt8>))?;
                            h22
                        };
                        let m28 = match v5.clone() {
                            Adt17::C0(_, _, _, h27) => {
                                h27
                            }
                        };
                        match m28 {
                            None => {
                                let m30 = f16(v0, match v5.clone() {
                                    Adt17::C0(_, h29, _, _) => {
                                        h29
                                    }
                                }, 5u64, f15(Str::lit("`,` or `]`")))?;
                                match m30 {
                                    Ok(v8) => {
                                        Ok(Ok::<(List<Bytes>, Adt10), Adt8>((f20(match v5 {
                                            Adt17::C0(h31, _, _, _) => {
                                                h31
                                            }
                                        }), match v8 {
                                            Adt19::C0(_, h32) => {
                                                h32
                                            }
                                        })))
                                    }
                                    Err(v7) => {
                                        Ok(Err::<(List<Bytes>, Adt10), Adt8>(v7))
                                    }
                                }
                            }
                            Some(v6) => {
                                Ok(Err::<(List<Bytes>, Adt10), Adt8>(v6))
                            }
                        }
                    }
                    Err(v3) => {
                        Ok(Err::<(List<Bytes>, Adt10), Adt8>(v3))
                    }
                }
            } else {
                Ok(Err::<(List<Bytes>, Adt10), Adt8>(f21(match v2 {
                    Adt18::C0(_, h33, _, _) => {
                        h33
                    }
                }, f15(Str::lit("a core module begins with its `imports`")))))
            }
        }
        Err(v1) => {
            Ok(Err::<(List<Bytes>, Adt10), Adt8>(v1))
        }
    }
}

pub fn f2(v0: Fn0, p38: u64, p39: Adt14) -> R<(Adt14, bool)> {
    let m37 = (p38, p39);
    {
        let mut m37 = m37;
        loop {
            let step = {
                let (v1, v2) = m37;
                let m34 = v1;
                if m34 == 0 {
                    Err::<(u64, Adt14), R<(Adt14, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m34 - 1;
                    let m36 = {
                        let c35: Fn0 = v0.clone();
                        c35.apply(v2.clone())?
                    };
                    match m36 {
                        None => {
                            Err::<(u64, Adt14), R<(Adt14, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt14), R<(Adt14, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m37 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Bytes, v1: Adt14) -> R<Option<Adt14>> {
    f22(v0, v1)
}

pub fn f4(v0: List<Adt13>, v1: List<Adt15>) -> Result<List<Adt6>, Adt8> {
    f23(Fn3::F24(v0), Ok::<List<Adt6>, Adt8>(List::<Adt6>::nil()), v1)
}

pub fn f5(v0: List<Adt15>) -> List<Adt15> {
    f25(Fn4::F26, List::<Adt15>::nil(), v0)
}

pub fn f6(v0: List<Adt1>) -> List<Adt1> {
    f27(Fn5::F28, List::<Adt1>::nil(), v0)
}

pub fn f7(v0: List<u64>) -> List<u64> {
    f29(Fn6::F30, List::<u64>::nil(), v0)
}

pub fn f8(v0: Fn1, p43: List<Adt6>, p44: List<Adt6>) -> List<Adt6> {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v1, v2) = m42;
                let m40 = v2;
                match m40.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>(({
                            let c41: Fn1 = v0.clone();
                            c41.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m42 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(Adt6::C4(std::rc::Rc::new(v1)), v0)
}

pub fn f10(v0: List<Adt7>) -> List<Adt7> {
    f31(Fn7::F32, List::<Adt7>::nil(), v0)
}

pub fn f11(v0: Bytes, v1: u64) -> R<Result<Adt18, Adt8>> {
    let v2: u64 = f33(v0.clone(), v1)?;
    let v3: u64 = f34(v0.clone(), v2)?;
    let v4: bool = {
        let a45 = v2;
        let a46 = 0u64;
        if nat_eq(a45, a46) {
            true
        } else {
            let a49 = f34(v0.clone(), {
                let a47 = v2;
                let a48 = 1u64;
                nat_sub(a47, a48)
            })?;
            let a50 = 10u64;
            nat_eq(a49, a50)
        }
    };
    let v5: u64 = f34(v0.clone(), {
        let a51 = v2;
        let a52 = 1u64;
        nat_add(a51, a52)?
    })?;
    let a53 = v3;
    let a54 = 9u64;
    if if nat_eq(a53, a54) {
        true
    } else {
        let a55 = v3;
        let a56 = 13u64;
        nat_eq(a55, a56)
    } {
        Ok(f35(v2, f15(Str::lit("a tab or a carriage return is not part of the notation"))))
    } else {
        let a57 = v3;
        let a58 = 256u64;
        if nat_eq(a57, a58) {
            Ok(f36(21u64, v2, v2, v4))
        } else {
            let a59 = v3;
            let a60 = 40u64;
            if nat_eq(a59, a60) {
                Ok(f36(0u64, v2, {
                    let a61 = v2;
                    let a62 = 1u64;
                    nat_add(a61, a62)?
                }, v4))
            } else {
                let a63 = v3;
                let a64 = 41u64;
                if nat_eq(a63, a64) {
                    Ok(f36(1u64, v2, {
                        let a65 = v2;
                        let a66 = 1u64;
                        nat_add(a65, a66)?
                    }, v4))
                } else {
                    let a67 = v3;
                    let a68 = 123u64;
                    if nat_eq(a67, a68) {
                        Ok(f36(2u64, v2, {
                            let a69 = v2;
                            let a70 = 1u64;
                            nat_add(a69, a70)?
                        }, v4))
                    } else {
                        let a71 = v3;
                        let a72 = 125u64;
                        if nat_eq(a71, a72) {
                            Ok(f36(3u64, v2, {
                                let a73 = v2;
                                let a74 = 1u64;
                                nat_add(a73, a74)?
                            }, v4))
                        } else {
                            let a75 = v3;
                            let a76 = 91u64;
                            if nat_eq(a75, a76) {
                                Ok(f36(4u64, v2, {
                                    let a77 = v2;
                                    let a78 = 1u64;
                                    nat_add(a77, a78)?
                                }, v4))
                            } else {
                                let a79 = v3;
                                let a80 = 93u64;
                                if nat_eq(a79, a80) {
                                    Ok(f36(5u64, v2, {
                                        let a81 = v2;
                                        let a82 = 1u64;
                                        nat_add(a81, a82)?
                                    }, v4))
                                } else {
                                    let a83 = v3;
                                    let a84 = 44u64;
                                    if nat_eq(a83, a84) {
                                        Ok(f36(6u64, v2, {
                                            let a85 = v2;
                                            let a86 = 1u64;
                                            nat_add(a85, a86)?
                                        }, v4))
                                    } else {
                                        let a87 = v3;
                                        let a88 = 59u64;
                                        if nat_eq(a87, a88) {
                                            Ok(f36(9u64, v2, {
                                                let a89 = v2;
                                                let a90 = 1u64;
                                                nat_add(a89, a90)?
                                            }, v4))
                                        } else {
                                            let a91 = v3;
                                            let a92 = 43u64;
                                            if nat_eq(a91, a92) {
                                                Ok(f36(12u64, v2, {
                                                    let a93 = v2;
                                                    let a94 = 1u64;
                                                    nat_add(a93, a94)?
                                                }, v4))
                                            } else {
                                                let a95 = v3;
                                                let a96 = 58u64;
                                                if nat_eq(a95, a96) {
                                                    let a97 = v5;
                                                    let a98 = 61u64;
                                                    if nat_eq(a97, a98) {
                                                        Ok(f36(8u64, v2, {
                                                            let a99 = v2;
                                                            let a100 = 2u64;
                                                            nat_add(a99, a100)?
                                                        }, v4))
                                                    } else {
                                                        Ok(f36(7u64, v2, {
                                                            let a101 = v2;
                                                            let a102 = 1u64;
                                                            nat_add(a101, a102)?
                                                        }, v4))
                                                    }
                                                } else {
                                                    let a103 = v3;
                                                    let a104 = 45u64;
                                                    if nat_eq(a103, a104) {
                                                        let a105 = v5;
                                                        let a106 = 62u64;
                                                        if nat_eq(a105, a106) {
                                                            Ok(f36(10u64, v2, {
                                                                let a107 = v2;
                                                                let a108 = 2u64;
                                                                nat_add(a107, a108)?
                                                            }, v4))
                                                        } else {
                                                            Ok(f36(13u64, v2, {
                                                                let a109 = v2;
                                                                let a110 = 1u64;
                                                                nat_add(a109, a110)?
                                                            }, v4))
                                                        }
                                                    } else {
                                                        if f12({
                                                            let a111 = v3;
                                                            let a112 = 61u64;
                                                            nat_eq(a111, a112)
                                                        }, {
                                                            let a113 = v5;
                                                            let a114 = 62u64;
                                                            nat_eq(a113, a114)
                                                        }) {
                                                            Ok(f36(11u64, v2, {
                                                                let a115 = v2;
                                                                let a116 = 2u64;
                                                                nat_add(a115, a116)?
                                                            }, v4))
                                                        } else {
                                                            if f12({
                                                                let a117 = v3;
                                                                let a118 = 46u64;
                                                                nat_eq(a117, a118)
                                                            }, {
                                                                let a119 = v5;
                                                                let a120 = 123u64;
                                                                nat_eq(a119, a120)
                                                            }) {
                                                                Ok(f36(14u64, v2, {
                                                                    let a121 = v2;
                                                                    let a122 = 2u64;
                                                                    nat_add(a121, a122)?
                                                                }, v4))
                                                            } else {
                                                                let a123 = v3;
                                                                let a124 = 34u64;
                                                                if nat_eq(a123, a124) {
                                                                    let m125 = f37(v0, v2)?;
                                                                    match m125 {
                                                                        Ok(v6) => {
                                                                            Ok(f36(19u64, v2, v6, v4))
                                                                        }
                                                                        Err(v7) => {
                                                                            Ok(Err::<Adt18, Adt8>(v7))
                                                                        }
                                                                    }
                                                                } else {
                                                                    if f12({
                                                                        let a126 = v3;
                                                                        let a127 = 64u64;
                                                                        nat_eq(a126, a127)
                                                                    }, f38(v0.clone(), {
                                                                        let a128 = v2;
                                                                        let a129 = 1u64;
                                                                        nat_add(a128, a129)?
                                                                    })?) {
                                                                        Ok(f36(16u64, v2, f39(v0, {
                                                                            let a130 = v2;
                                                                            let a131 = 1u64;
                                                                            nat_add(a130, a131)?
                                                                        })?, v4))
                                                                    } else {
                                                                        let a132 = v3;
                                                                        let a133 = 64u64;
                                                                        if if nat_eq(a132, a133) {
                                                                            true
                                                                        } else {
                                                                            let a134 = v3;
                                                                            let a135 = 35u64;
                                                                            nat_eq(a134, a135)
                                                                        } {
                                                                            let v8: u64 = f40(v0.clone(), {
                                                                                let a136 = v2;
                                                                                let a137 = 1u64;
                                                                                nat_add(a136, a137)?
                                                                            })?;
                                                                            let a140 = v8;
                                                                            let a141 = {
                                                                                let a138 = v2;
                                                                                let a139 = 1u64;
                                                                                nat_add(a138, a139)?
                                                                            };
                                                                            if nat_eq(a140, a141) {
                                                                                Ok(f35(v2, f15(Str::lit("`@` and `#` are followed by a number or, after `@`, a name"))))
                                                                            } else {
                                                                                if f41(v0, v8)? {
                                                                                    Ok(f35(v8, f15(Str::lit("a number ends where its text does"))))
                                                                                } else {
                                                                                    Ok(f36({
                                                                                        let a142 = v3;
                                                                                        let a143 = 64u64;
                                                                                        if nat_eq(a142, a143) {
                                                                                            15u64
                                                                                        } else {
                                                                                            17u64
                                                                                        }
                                                                                    }, v2, v8, v4))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if f42(v3) {
                                                                                let v9: u64 = f40(v0.clone(), v2)?;
                                                                                if f41(v0, v9)? {
                                                                                    Ok(f35(v2, f15(Str::lit("a name does not begin with a digit; quote it"))))
                                                                                } else {
                                                                                    Ok(f36(18u64, v2, v9, v4))
                                                                                }
                                                                            } else {
                                                                                if f38(v0.clone(), v2)? {
                                                                                    let v10: u64 = f39(v0.clone(), v2)?;
                                                                                    Ok(f36(20u64, v2, if f12({
                                                                                        let a146 = f34(v0.clone(), {
                                                                                            let a144 = v10;
                                                                                            let a145 = 1u64;
                                                                                            nat_sub(a144, a145)
                                                                                        })?;
                                                                                        let a147 = 46u64;
                                                                                        nat_eq(a146, a147)
                                                                                    }, {
                                                                                        let a148 = f34(v0, v10)?;
                                                                                        let a149 = 123u64;
                                                                                        nat_eq(a148, a149)
                                                                                    }) {
                                                                                        let a150 = v10;
                                                                                        let a151 = 1u64;
                                                                                        nat_sub(a150, a151)
                                                                                    } else {
                                                                                        v10
                                                                                    }, v4))
                                                                                } else {
                                                                                    Ok(f35(v2, f43(f43(f15(Str::lit("unexpected `")), f44(v0, v2, f45(v3))), f15(Str::lit("`")))))
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
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f12(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f13(v0: Bytes, v1: Bytes) -> bool {
    let a152 = v0;
    let a153 = v1;
    equal(a152, a153)
}

pub fn f14(v0: Bytes, v1: Adt18) -> Bytes {
    f44(v0, match v1.clone() {
        Adt18::C0(_, h154, _, _) => {
            h154
        }
    }, {
        let a157 = match v1.clone() {
            Adt18::C0(_, _, h155, _) => {
                h155
            }
        };
        let a158 = match v1 {
            Adt18::C0(_, h156, _, _) => {
                h156
            }
        };
        nat_sub(a157, a158)
    })
}

pub fn f15(v0: Str) -> Bytes {
    let a159 = v0;
    utf8_encode(a159)
}

pub fn f16(v0: Bytes, v1: Adt10, v2: u64, v3: Bytes) -> R<Result<Adt19, Adt8>> {
    let m161 = f11(v0, match v1 {
        Adt10::C0(h160, _) => {
            h160
        }
    })?;
    match m161 {
        Ok(v4) => {
            let a163 = match v4.clone() {
                Adt18::C0(h162, _, _, _) => {
                    h162
                }
            };
            let a164 = v2;
            if nat_eq(a163, a164) {
                Ok(Ok::<Adt19, Adt8>(Adt19::C0(v4.clone(), f17(v4))))
            } else {
                Ok(Err::<Adt19, Adt8>(f21(match v4 {
                    Adt18::C0(_, h165, _, _) => {
                        h165
                    }
                }, f43(f15(Str::lit("expected ")), v3))))
            }
        }
        Err(v5) => {
            Ok(Err::<Adt19, Adt8>(v5))
        }
    }
}

pub fn f17(v0: Adt18) -> Adt10 {
    Adt10::C0(match v0.clone() {
        Adt18::C0(_, _, h166, _) => {
            h166
        }
    }, match v0 {
        Adt18::C0(_, _, h167, _) => {
            h167
        }
    })
}

pub fn f18(v0: Fn2, p172: u64, p173: Adt17) -> R<(Adt17, bool)> {
    let m171 = (p172, p173);
    {
        let mut m171 = m171;
        loop {
            let step = {
                let (v1, v2) = m171;
                let m168 = v1;
                if m168 == 0 {
                    Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m168 - 1;
                    let m170 = {
                        let c169: Fn2 = v0.clone();
                        c169.apply(v2.clone())?
                    };
                    match m170 {
                        None => {
                            Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt17), R<(Adt17, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Bytes, v1: Adt17) -> R<Option<Adt17>> {
    f46(v0, v1)
}

pub fn f20(v0: List<Bytes>) -> List<Bytes> {
    f47(Fn8::F48, List::<Bytes>::nil(), v0)
}

pub fn f21(v0: u64, v1: Bytes) -> Adt8 {
    Adt8::C0(v0, v1)
}

pub fn f22(v0: Bytes, v1: Adt14) -> R<Option<Adt14>> {
    let v2: Adt9 = match v1.clone() {
        Adt14::C0(h174, _, _, _, _) => {
            h174
        }
    };
    if if match v1.clone() {
        Adt14::C0(_, _, _, _, h175) => {
            h175
        }
    } {
        true
    } else {
        let m177 = match v2.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, h176) => {
                h176
            }
        };
        match m177 {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    } {
        Ok(None::<Adt14>)
    } else {
        Ok(Some({
            let m181 = f11(v0.clone(), {
                let m180 = match v2.clone() {
                    Adt9::C0(h179, _, _, _, _, _, _, _, _, _) => {
                        h179
                    }
                };
                match m180 {
                    Adt10::C0(h178, _) => {
                        h178
                    }
                }
            })?;
            match m181 {
                Ok(v5) => {
                    let a183 = match v5.clone() {
                        Adt18::C0(h182, _, _, _) => {
                            h182
                        }
                    };
                    let a184 = 21u64;
                    if nat_eq(a183, a184) {
                        Adt14::C0(v2, match v1.clone() {
                            Adt14::C0(_, h185, _, _, _) => {
                                h185
                            }
                        }, match v1.clone() {
                            Adt14::C0(_, _, h186, _, _) => {
                                h186
                            }
                        }, match v1 {
                            Adt14::C0(_, _, _, h187, _) => {
                                h187
                            }
                        }, true)
                    } else {
                        if !match v5.clone() {
                            Adt18::C0(_, _, _, h188) => {
                                h188
                            }
                        } {
                            f49(v1, f21(match v5 {
                                Adt18::C0(_, h189, _, _) => {
                                    h189
                                }
                            }, f15(Str::lit("a declaration or a node begins its line"))))
                        } else {
                            let a191 = match v5.clone() {
                                Adt18::C0(h190, _, _, _) => {
                                    h190
                                }
                            };
                            let a192 = 15u64;
                            if if nat_eq(a191, a192) {
                                true
                            } else {
                                let a194 = match v5.clone() {
                                    Adt18::C0(h193, _, _, _) => {
                                        h193
                                    }
                                };
                                let a195 = 16u64;
                                nat_eq(a194, a195)
                            } {
                                Adt14::C0(f50(v0, v2)?, match v1.clone() {
                                    Adt14::C0(_, h196, _, _, _) => {
                                        h196
                                    }
                                }, match v1 {
                                    Adt14::C0(_, _, h197, _, _) => {
                                        h197
                                    }
                                }, true, false)
                            } else {
                                if match v1.clone() {
                                    Adt14::C0(_, _, _, h198, _) => {
                                        h198
                                    }
                                } {
                                    f49(v1, f21(match v5 {
                                        Adt18::C0(_, h199, _, _) => {
                                            h199
                                        }
                                    }, f15(Str::lit("the declarations come before the nodes"))))
                                } else {
                                    let m201 = f51(v0, match v2.clone() {
                                        Adt9::C0(h200, _, _, _, _, _, _, _, _, _) => {
                                            h200
                                        }
                                    })?;
                                    match m201 {
                                        Ok(v7) => {
                                            Adt14::C0(f52(v2.clone(), {
                                                let (_, h202) = v7.clone();
                                                h202
                                            }, match v2 {
                                                Adt9::C0(_, h203, _, _, _, _, _, _, _, _) => {
                                                    h203
                                                }
                                            }), List::cons({
                                                let (h204, _) = v7;
                                                h204
                                            }, match v1.clone() {
                                                Adt14::C0(_, h205, _, _, _) => {
                                                    h205
                                                }
                                            }), List::cons(Adt7::C0(match v5 {
                                                Adt18::C0(_, h206, _, _) => {
                                                    h206
                                                }
                                            }, {
                                                let m209 = {
                                                    let (_, h208) = v7;
                                                    h208
                                                };
                                                match m209 {
                                                    Adt10::C0(_, h207) => {
                                                        h207
                                                    }
                                                }
                                            }), match v1 {
                                                Adt14::C0(_, _, h210, _, _) => {
                                                    h210
                                                }
                                            }), false, false)
                                        }
                                        Err(v6) => {
                                            f49(v1, v6)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(v4) => {
                    f49(v1, v4)
                }
            }
        }))
    }
}

pub fn f23(v0: Fn3, p214: Result<List<Adt6>, Adt8>, p215: List<Adt15>) -> Result<List<Adt6>, Adt8> {
    let m213 = (p214, p215);
    {
        let mut m213 = m213;
        loop {
            let step = {
                let (v1, v2) = m213;
                let m211 = v2;
                match m211.uncons() {
                    None => {
                        Err::<(Result<List<Adt6>, Adt8>, List<Adt15>), Result<List<Adt6>, Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt6>, Adt8>, List<Adt15>), Result<List<Adt6>, Adt8>>(({
                            let c212: Fn3 = v0.clone();
                            c212.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<Adt13>, v1: Result<List<Adt6>, Adt8>, v2: Adt15) -> Result<List<Adt6>, Adt8> {
    let m216 = v1.clone();
    match m216 {
        Ok(v4) => {
            let m218 = f53(v0.clone(), match v2.clone() {
                Adt15::C0(_, h217, _, _) => {
                    h217
                }
            });
            match m218 {
                Ok(v6) => {
                    let m220 = f53(v0, match v2.clone() {
                        Adt15::C0(_, _, h219, _) => {
                            h219
                        }
                    });
                    match m220 {
                        Ok(v8) => {
                            Ok::<List<Adt6>, Adt8>(List::cons(f54(f54(match v2 {
                                Adt15::C0(h221, _, _, _) => {
                                    h221
                                }
                            }, f15(Str::lit("type")), v6), f15(Str::lit("value")), v8), v4))
                        }
                        Err(v7) => {
                            Err::<List<Adt6>, Adt8>(v7)
                        }
                    }
                }
                Err(v5) => {
                    Err::<List<Adt6>, Adt8>(v5)
                }
            }
        }
        Err(_) => {
            v1
        }
    }
}

pub fn f25(v0: Fn4, p225: List<Adt15>, p226: List<Adt15>) -> List<Adt15> {
    let m224 = (p225, p226);
    {
        let mut m224 = m224;
        loop {
            let step = {
                let (v1, v2) = m224;
                let m222 = v2;
                match m222.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>(({
                            let c223: Fn4 = v0.clone();
                            c223.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: List<Adt15>, v1: Adt15) -> List<Adt15> {
    List::cons(v1, v0)
}

pub fn f27(v0: Fn5, p230: List<Adt1>, p231: List<Adt1>) -> List<Adt1> {
    let m229 = (p230, p231);
    {
        let mut m229 = m229;
        loop {
            let step = {
                let (v1, v2) = m229;
                let m227 = v2;
                match m227.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c228: Fn5 = v0.clone();
                            c228.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f29(v0: Fn6, p235: List<u64>, p236: List<u64>) -> List<u64> {
    let m234 = (p235, p236);
    {
        let mut m234 = m234;
        loop {
            let step = {
                let (v1, v2) = m234;
                let m232 = v2;
                match m232.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c233: Fn6 = v0.clone();
                            c233.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f31(v0: Fn7, p240: List<Adt7>, p241: List<Adt7>) -> List<Adt7> {
    let m239 = (p240, p241);
    {
        let mut m239 = m239;
        loop {
            let step = {
                let (v1, v2) = m239;
                let m237 = v2;
                match m237.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt7>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt7>), List<Adt7>>(({
                            let c238: Fn7 = v0.clone();
                            c238.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f33(v0: Bytes, v1: u64) -> R<u64> {
    let (h242, _) = f55(Fn9::F56(v0.clone()), {
        let a244 = {
            let a243 = v0;
            length_bytes(a243)
        };
        let a245 = 1u64;
        nat_add(a244, a245)?
    }, v1)?;
    Ok(h242)
}

pub fn f34(v0: Bytes, v1: u64) -> R<u64> {
    let m246 = f57(v0, v1);
    match m246 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a291 = {
                let a267 = {
                    let a255 = {
                        let a249 = {
                            let a247 = v3;
                            let a248 = 128u8;
                            fixed_u8::bit_and(a247, a248)
                        };
                        let a250 = 128u8;
                        if equal(a249, a250) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a256 = {
                        let a253 = {
                            let a251 = v3;
                            let a252 = 64u8;
                            fixed_u8::bit_and(a251, a252)
                        };
                        let a254 = 64u8;
                        if equal(a253, a254) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a255, a256)?
                };
                let a268 = {
                    let a265 = {
                        let a259 = {
                            let a257 = v3;
                            let a258 = 32u8;
                            fixed_u8::bit_and(a257, a258)
                        };
                        let a260 = 32u8;
                        if equal(a259, a260) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a266 = {
                        let a263 = {
                            let a261 = v3;
                            let a262 = 16u8;
                            fixed_u8::bit_and(a261, a262)
                        };
                        let a264 = 16u8;
                        if equal(a263, a264) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a265, a266)?
                };
                nat_add(a267, a268)?
            };
            let a292 = {
                let a289 = {
                    let a277 = {
                        let a271 = {
                            let a269 = v3;
                            let a270 = 8u8;
                            fixed_u8::bit_and(a269, a270)
                        };
                        let a272 = 8u8;
                        if equal(a271, a272) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a278 = {
                        let a275 = {
                            let a273 = v3;
                            let a274 = 4u8;
                            fixed_u8::bit_and(a273, a274)
                        };
                        let a276 = 4u8;
                        if equal(a275, a276) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a277, a278)?
                };
                let a290 = {
                    let a287 = {
                        let a281 = {
                            let a279 = v3;
                            let a280 = 2u8;
                            fixed_u8::bit_and(a279, a280)
                        };
                        let a282 = 2u8;
                        if equal(a281, a282) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a288 = {
                        let a285 = {
                            let a283 = v3;
                            let a284 = 1u8;
                            fixed_u8::bit_and(a283, a284)
                        };
                        let a286 = 1u8;
                        if equal(a285, a286) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a287, a288)?
                };
                nat_add(a289, a290)?
            };
            nat_add(a291, a292)
        }
    }
}

pub fn f35(v0: u64, v1: Bytes) -> Result<Adt18, Adt8> {
    Err::<Adt18, Adt8>(f21(v0, v1))
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: bool) -> Result<Adt18, Adt8> {
    Ok::<Adt18, Adt8>(Adt18::C0(v0, v1, v2, v3))
}

pub fn f37(v0: Bytes, v1: u64) -> R<Result<u64, Adt8>> {
    let v2: Adt20 = f58(v0.clone(), {
        let a293 = v0;
        length_bytes(a293)
    }, v1)?;
    let m295 = match v2.clone() {
        Adt20::C0(_, _, _, h294) => {
            h294
        }
    };
    match m295 {
        None => {
            if match v2.clone() {
                Adt20::C0(_, _, h296, _) => {
                    h296
                }
            } {
                Ok(Ok::<u64, Adt8>(match v2 {
                    Adt20::C0(h297, _, _, _) => {
                        h297
                    }
                }))
            } else {
                Ok(Err::<u64, Adt8>(f21(v1, f15(Str::lit("unterminated string")))))
            }
        }
        Some(v3) => {
            Ok(Err::<u64, Adt8>(v3))
        }
    }
}

pub fn f38(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f34(v0.clone(), v1)?;
    let a298 = 256u64;
    let a299 = v2;
    if nat_le(a298, a299) {
        Ok(false)
    } else {
        if f59(v2) {
            Ok(true)
        } else {
            let a300 = v2;
            let a301 = 95u64;
            if nat_eq(a300, a301) {
                Ok(true)
            } else {
                Ok(f12({
                    let a302 = 128u64;
                    let a303 = v2;
                    nat_le(a302, a303)
                }, {
                    let a304 = f60(v0, v1)?;
                    let a305 = 0u64;
                    nat_eq(a304, a305)
                }))
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<u64> {
    let (h306, _) = f55(Fn9::F61(v0.clone()), {
        let a308 = {
            let a307 = v0;
            length_bytes(a307)
        };
        let a309 = 1u64;
        nat_add(a308, a309)?
    }, v1)?;
    Ok(h306)
}

pub fn f40(v0: Bytes, v1: u64) -> R<u64> {
    let (h310, _) = f55(Fn9::F62(v0.clone()), {
        let a312 = {
            let a311 = v0;
            length_bytes(a311)
        };
        let a313 = 1u64;
        nat_add(a312, a313)?
    }, v1)?;
    Ok(h310)
}

pub fn f41(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f34(v0.clone(), v1)?;
    let a314 = 256u64;
    let a315 = v2;
    if nat_le(a314, a315) {
        Ok(false)
    } else {
        if if f59(v2) {
            true
        } else {
            f42(v2)
        } {
            Ok(true)
        } else {
            if f63(f44(v0.clone(), v1, 1u64), List::cons(f15(Str::lit("_")), List::cons(f15(Str::lit("'")), List::cons(f15(Str::lit("!")), List::cons(f15(Str::lit("?")), List::cons(f15(Str::lit(".")), List::cons(f15(Str::lit("@")), List::cons(f15(Str::lit("#")), List::<Bytes>::nil())))))))) {
                Ok(true)
            } else {
                let a316 = v2;
                let a317 = 128u64;
                if nat_lt(a316, a317) {
                    Ok(false)
                } else {
                    let a318 = f60(v0, v1)?;
                    let a319 = 0u64;
                    Ok(nat_eq(a318, a319))
                }
            }
        }
    }
}

pub fn f42(v0: u64) -> bool {
    f12({
        let a320 = 48u64;
        let a321 = v0;
        nat_le(a320, a321)
    }, {
        let a322 = v0;
        let a323 = 57u64;
        nat_le(a322, a323)
    })
}

pub fn f43(v0: Bytes, v1: Bytes) -> Bytes {
    let a324 = v0;
    let a325 = v1;
    append_bytes(a324, a325)
}

pub fn f44(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m329 = {
        let a326 = v0;
        let a327 = v1;
        let a328 = v2;
        slice_bytes(a326, a327, a328)
    };
    match m329 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f45(v0: u64) -> u64 {
    let a330 = v0;
    let a331 = 128u64;
    if nat_lt(a330, a331) {
        1u64
    } else {
        let a332 = v0;
        let a333 = 224u64;
        if nat_lt(a332, a333) {
            2u64
        } else {
            let a334 = v0;
            let a335 = 240u64;
            if nat_lt(a334, a335) {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f46(v0: Bytes, v1: Adt17) -> R<Option<Adt17>> {
    f64(v0, v1, f15(Str::lit("a module name")), true)
}

pub fn f47(v0: Fn8, p339: List<Bytes>, p340: List<Bytes>) -> List<Bytes> {
    let m338 = (p339, p340);
    {
        let mut m338 = m338;
        loop {
            let step = {
                let (v1, v2) = m338;
                let m336 = v2;
                match m336.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c337: Fn8 = v0.clone();
                            c337.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f49(v0: Adt14, v1: Adt8) -> Adt14 {
    Adt14::C0(f65(match v0.clone() {
        Adt14::C0(h341, _, _, _, _) => {
            h341
        }
    }, v1), match v0.clone() {
        Adt14::C0(_, h342, _, _, _) => {
            h342
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h343, _, _) => {
            h343
        }
    }, match v0 {
        Adt14::C0(_, _, _, h344, _) => {
            h344
        }
    }, true)
}

pub fn f50(v0: Bytes, v1: Adt9) -> R<Adt9> {
    let m348 = f11(v0.clone(), {
        let m347 = match v1.clone() {
            Adt9::C0(h346, _, _, _, _, _, _, _, _, _) => {
                h346
            }
        };
        match m347 {
            Adt10::C0(h345, _) => {
                h345
            }
        }
    })?;
    match m348 {
        Ok(v3) => {
            let v4: bool = {
                let a350 = match v3.clone() {
                    Adt18::C0(h349, _, _, _) => {
                        h349
                    }
                };
                let a351 = 15u64;
                nat_eq(a350, a351)
            };
            let v5: u64 = if f66(v0.clone(), v3.clone())? {
                0u64
            } else {
                f67(v0.clone(), v3.clone())?
            };
            let v6: Bytes = f44(v0.clone(), {
                let a353 = match v3.clone() {
                    Adt18::C0(_, h352, _, _) => {
                        h352
                    }
                };
                let a354 = 1u64;
                nat_add(a353, a354)?
            }, {
                let a359 = {
                    let a357 = match v3.clone() {
                        Adt18::C0(_, _, h355, _) => {
                            h355
                        }
                    };
                    let a358 = match v3.clone() {
                        Adt18::C0(_, h356, _, _) => {
                            h356
                        }
                    };
                    nat_sub(a357, a358)
                };
                let a360 = 1u64;
                nat_sub(a359, a360)
            });
            if f12(v4, f66(v0.clone(), v3.clone())?) {
                Ok(f65(v1, f21(match v3 {
                    Adt18::C0(_, h361, _, _) => {
                        h361
                    }
                }, f15(Str::lit("the number is too large")))))
            } else {
                let m362 = f16(v0.clone(), f17(v3.clone()), 8u64, f15(Str::lit("`:=` after the label")))?;
                match m362 {
                    Ok(v8) => {
                        let v9: u64 = match v1.clone() {
                            Adt9::C0(_, _, _, _, _, h363, _, _, _, _) => {
                                h363
                            }
                        };
                        let v10: Adt9 = f68(v0.clone(), f52(v1, match v8 {
                            Adt19::C0(_, h364) => {
                                h364
                            }
                        }, List::<Adt11>::nil()))?;
                        let m366 = match v10.clone() {
                            Adt9::C0(_, _, _, _, _, _, _, _, _, h365) => {
                                h365
                            }
                        };
                        match m366 {
                            None => {
                                let v12: u64 = f69(match v10.clone() {
                                    Adt9::C0(_, _, h367, _, _, _, _, _, _, _) => {
                                        h367
                                    }
                                });
                                let v13: Adt9 = f70(v10.clone(), match v10.clone() {
                                    Adt9::C0(h368, _, _, _, _, _, _, _, _, _) => {
                                        h368
                                    }
                                }, List::<Adt11>::nil(), f71(match v10.clone() {
                                    Adt9::C0(_, _, h369, _, _, _, _, _, _, _) => {
                                        h369
                                    }
                                }), match v10.clone() {
                                    Adt9::C0(_, _, _, h370, _, _, _, _, _, _) => {
                                        h370
                                    }
                                }, match v10.clone() {
                                    Adt9::C0(_, _, _, _, _, _, _, h371, _, _) => {
                                        h371
                                    }
                                });
                                let a372 = v12;
                                let a373 = v9;
                                if nat_lt(a372, a373) {
                                    Ok(f65(v13, f21(match v3 {
                                        Adt18::C0(_, h374, _, _) => {
                                            h374
                                        }
                                    }, f15(Str::lit("a statement defines a node, not another name for one")))))
                                } else {
                                    let a378 = {
                                        let a375 = v12;
                                        let a376 = 1u64;
                                        nat_add(a375, a376)?
                                    };
                                    let a379 = match v10.clone() {
                                        Adt9::C0(_, _, _, _, _, h377, _, _, _, _) => {
                                            h377
                                        }
                                    };
                                    if !nat_eq(a378, a379) {
                                        Ok(f65(v13, f21(match v3 {
                                            Adt18::C0(_, h380, _, _) => {
                                                h380
                                            }
                                        }, f15(Str::lit("a statement defines the node its term ends with")))))
                                    } else {
                                        if f12(v4, {
                                            let a381 = v5;
                                            let a382 = v12;
                                            !nat_eq(a381, a382)
                                        }) {
                                            Ok(f65(v13, f21(match v3 {
                                                Adt18::C0(_, h383, _, _) => {
                                                    h383
                                                }
                                            }, f43(f43(f43(f43(f15(Str::lit("this statement defines node ")), f72(v12)), f15(Str::lit(", not node "))), f72(v5)), f15(Str::lit(": the label is the number of the node"))))))
                                        } else {
                                            if if v4 {
                                                false
                                            } else {
                                                let m385 = f73(match v10 {
                                                    Adt9::C0(_, _, _, _, _, _, _, _, h384, _) => {
                                                        h384
                                                    }
                                                }, v6.clone());
                                                match m385 {
                                                    None => {
                                                        false
                                                    }
                                                    Some(_) => {
                                                        true
                                                    }
                                                }
                                            } {
                                                Ok(f65(v13, f21(match v3 {
                                                    Adt18::C0(_, h386, _, _) => {
                                                        h386
                                                    }
                                                }, f43(f43(f15(Str::lit("`@")), v6), f15(Str::lit("` names two statements"))))))
                                            } else {
                                                let v15: Adt9 = if v4 {
                                                    v13
                                                } else {
                                                    Adt9::C0(match v13.clone() {
                                                        Adt9::C0(h387, _, _, _, _, _, _, _, _, _) => {
                                                            h387
                                                        }
                                                    }, match v13.clone() {
                                                        Adt9::C0(_, h388, _, _, _, _, _, _, _, _) => {
                                                            h388
                                                        }
                                                    }, match v13.clone() {
                                                        Adt9::C0(_, _, h389, _, _, _, _, _, _, _) => {
                                                            h389
                                                        }
                                                    }, match v13.clone() {
                                                        Adt9::C0(_, _, _, h390, _, _, _, _, _, _) => {
                                                            h390
                                                        }
                                                    }, match v13.clone() {
                                                        Adt9::C0(_, _, _, _, h391, _, _, _, _, _) => {
                                                            h391
                                                        }
                                                    }, match v13.clone() {
                                                        Adt9::C0(_, _, _, _, _, h392, _, _, _, _) => {
                                                            h392
                                                        }
                                                    }, match v13.clone() {
                                                        Adt9::C0(_, _, _, _, _, _, h393, _, _, _) => {
                                                            h393
                                                        }
                                                    }, match v13.clone() {
                                                        Adt9::C0(_, _, _, _, _, _, _, h394, _, _) => {
                                                            h394
                                                        }
                                                    }, List::cons(Adt13::C0(v6, v12), match v13.clone() {
                                                        Adt9::C0(_, _, _, _, _, _, _, _, h395, _) => {
                                                            h395
                                                        }
                                                    }), match v13 {
                                                        Adt9::C0(_, _, _, _, _, _, _, _, _, h396) => {
                                                            h396
                                                        }
                                                    })
                                                };
                                                let m400 = f11(v0, {
                                                    let m399 = match v15.clone() {
                                                        Adt9::C0(h398, _, _, _, _, _, _, _, _, _) => {
                                                            h398
                                                        }
                                                    };
                                                    match m399 {
                                                        Adt10::C0(h397, _) => {
                                                            h397
                                                        }
                                                    }
                                                })?;
                                                match m400 {
                                                    Ok(v17) => {
                                                        if f12({
                                                            let a402 = match v17.clone() {
                                                                Adt18::C0(h401, _, _, _) => {
                                                                    h401
                                                                }
                                                            };
                                                            let a403 = 21u64;
                                                            !nat_eq(a402, a403)
                                                        }, !match v17.clone() {
                                                            Adt18::C0(_, _, _, h404) => {
                                                                h404
                                                            }
                                                        }) {
                                                            Ok(f65(v15, f21(match v17 {
                                                                Adt18::C0(_, h405, _, _) => {
                                                                    h405
                                                                }
                                                            }, f15(Str::lit("a statement ends at the end of its line")))))
                                                        } else {
                                                            Ok(v15)
                                                        }
                                                    }
                                                    Err(v16) => {
                                                        Ok(f65(v15, v16))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Some(_) => {
                                Ok(v10)
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f65(v1, v7))
                    }
                }
            }
        }
        Err(v2) => {
            Ok(f65(v1, v2))
        }
    }
}

pub fn f51(v0: Bytes, v1: Adt10) -> R<Result<(Adt15, Adt10), Adt8>> {
    let m407 = f11(v0.clone(), match v1 {
        Adt10::C0(h406, _) => {
            h406
        }
    })?;
    match m407 {
        Ok(v3) => {
            let v4: Bytes = f14(v0.clone(), v3.clone());
            if f63(v4.clone(), List::cons(f15(Str::lit("abbrev")), List::cons(f15(Str::lit("definition")), List::cons(f15(Str::lit("theorem")), List::cons(f15(Str::lit("inductive")), List::cons(f15(Str::lit("constructor")), List::cons(f15(Str::lit("recursor")), List::<Bytes>::nil()))))))) {
                let m408 = f74(v0.clone(), f17(v3.clone()), f15(Str::lit("the declaration's name")))?;
                match m408 {
                    Ok(v6) => {
                        let m412 = f11(v0.clone(), {
                            let m411 = match v6.clone() {
                                Adt21::C0(_, h410) => {
                                    h410
                                }
                            };
                            match m411 {
                                Adt10::C0(h409, _) => {
                                    h409
                                }
                            }
                        })?;
                        match m412 {
                            Ok(v8) => {
                                let a414 = match v8.clone() {
                                    Adt18::C0(h413, _, _, _) => {
                                        h413
                                    }
                                };
                                let a415 = 14u64;
                                if nat_eq(a414, a415) {
                                    let v9: Adt17 = {
                                        let (h416, _) = f18(Fn2::F75(v0.clone()), {
                                            let a418 = {
                                                let a417 = v0.clone();
                                                length_bytes(a417)
                                            };
                                            let a419 = 2u64;
                                            nat_add(a418, a419)?
                                        }, Adt17::C0(List::<Bytes>::nil(), f17(v8), false, None::<Adt8>))?;
                                        h416
                                    };
                                    let m421 = match v9.clone() {
                                        Adt17::C0(_, _, _, h420) => {
                                            h420
                                        }
                                    };
                                    match m421 {
                                        None => {
                                            let m423 = f16(v0.clone(), match v9.clone() {
                                                Adt17::C0(_, h422, _, _) => {
                                                    h422
                                                }
                                            }, 3u64, f15(Str::lit("`}` after the universe parameters")))?;
                                            match m423 {
                                                Ok(v12) => {
                                                    f76(v0, v4, match v6 {
                                                        Adt21::C0(h424, _) => {
                                                            h424
                                                        }
                                                    }, f77(Fn10::F78, List::<Adt6>::nil(), match v9 {
                                                        Adt17::C0(h425, _, _, _) => {
                                                            h425
                                                        }
                                                    }), match v12 {
                                                        Adt19::C0(_, h426) => {
                                                            h426
                                                        }
                                                    }, match v3 {
                                                        Adt18::C0(_, h427, _, _) => {
                                                            h427
                                                        }
                                                    })
                                                }
                                                Err(v11) => {
                                                    Ok(Err::<(Adt15, Adt10), Adt8>(v11))
                                                }
                                            }
                                        }
                                        Some(v10) => {
                                            Ok(Err::<(Adt15, Adt10), Adt8>(v10))
                                        }
                                    }
                                } else {
                                    f76(v0, v4, match v6.clone() {
                                        Adt21::C0(h428, _) => {
                                            h428
                                        }
                                    }, List::<Adt6>::nil(), match v6 {
                                        Adt21::C0(_, h429) => {
                                            h429
                                        }
                                    }, match v3 {
                                        Adt18::C0(_, h430, _, _) => {
                                            h430
                                        }
                                    })
                                }
                            }
                            Err(v7) => {
                                Ok(Err::<(Adt15, Adt10), Adt8>(v7))
                            }
                        }
                    }
                    Err(v5) => {
                        Ok(Err::<(Adt15, Adt10), Adt8>(v5))
                    }
                }
            } else {
                Ok(Err::<(Adt15, Adt10), Adt8>(f21(match v3 {
                    Adt18::C0(_, h431, _, _) => {
                        h431
                    }
                }, f43(f43(f15(Str::lit("`")), v4), f15(Str::lit("` does not begin a declaration or a node"))))))
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt15, Adt10), Adt8>(v2))
        }
    }
}

pub fn f52(v0: Adt9, v1: Adt10, v2: List<Adt11>) -> Adt9 {
    f70(v0.clone(), v1, v2, match v0.clone() {
        Adt9::C0(_, _, h432, _, _, _, _, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, h433, _, _, _, _, _, _) => {
            h433
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, h434, _, _) => {
            h434
        }
    })
}

pub fn f53(v0: List<Adt13>, v1: Option<Adt16>) -> Result<Option<u64>, Adt8> {
    let m435 = v1;
    match m435 {
        None => {
            Ok::<Option<u64>, Adt8>(None::<u64>)
        }
        Some(v2) => {
            let m436 = v2;
            match m436 {
                Adt16::C0(v3) => {
                    Ok::<Option<u64>, Adt8>(Some(v3))
                }
                Adt16::C1(v4, v5) => {
                    let m437 = f73(v0, v4.clone());
                    match m437 {
                        None => {
                            Err::<Option<u64>, Adt8>(f21(v5, f43(f43(f15(Str::lit("no statement is named `@")), v4), f15(Str::lit("`")))))
                        }
                        Some(v6) => {
                            Ok::<Option<u64>, Adt8>(Some(v6))
                        }
                    }
                }
            }
        }
    }
}

pub fn f54(v0: Adt6, v1: Bytes, v2: Option<u64>) -> Adt6 {
    let m438 = v2;
    match m438 {
        None => {
            v0
        }
        Some(v3) => {
            f79(v0, v1, Adt6::C1(false, v3))
        }
    }
}

pub fn f55(v0: Fn9, p443: u64, p444: u64) -> R<(u64, bool)> {
    let m442 = (p443, p444);
    {
        let mut m442 = m442;
        loop {
            let step = {
                let (v1, v2) = m442;
                let m439 = v1;
                if m439 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m439 - 1;
                    let m441 = {
                        let c440: Fn9 = v0.clone();
                        c440.apply(v2)?
                    };
                    match m441 {
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
                Ok(next) => { m442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f34(v0, v1)?;
    let a445 = v2;
    let a446 = 32u64;
    if if nat_eq(a445, a446) {
        true
    } else {
        let a447 = v2;
        let a448 = 10u64;
        nat_eq(a447, a448)
    } {
        Ok(Some({
            let a449 = v1;
            let a450 = 1u64;
            nat_add(a449, a450)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f57(v0: Bytes, v1: u64) -> Option<u8> {
    let a451 = v0;
    let a452 = v1;
    index_bytes(a451, a452)
}

pub fn f58(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let (h453, _) = f80(Fn11::F81(v1, v0), {
        let a456 = {
            let a454 = v1;
            let a455 = v2;
            nat_sub(a454, a455)
        };
        let a457 = 1u64;
        nat_add(a456, a457)?
    }, Adt20::C0({
        let a458 = v2;
        let a459 = 1u64;
        nat_add(a458, a459)?
    }, List::<Bytes>::nil(), false, None::<Adt8>))?;
    Ok(h453)
}

pub fn f59(v0: u64) -> bool {
    if f12({
        let a460 = 65u64;
        let a461 = v0;
        nat_le(a460, a461)
    }, {
        let a462 = v0;
        let a463 = 90u64;
        nat_le(a462, a463)
    }) {
        true
    } else {
        f12({
            let a464 = 97u64;
            let a465 = v0;
            nat_le(a464, a465)
        }, {
            let a466 = v0;
            let a467 = 122u64;
            nat_le(a466, a467)
        })
    }
}

pub fn f60(v0: Bytes, v1: u64) -> R<u64> {
    if if f82(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f83(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f83(v0.clone(), v1, 194u8) {
            if if f83(v0.clone(), {
                let a468 = v1;
                let a469 = 1u64;
                nat_add(a468, a469)?
            }, 133u8) {
                true
            } else {
                f83(v0, {
                    let a470 = v1;
                    let a471 = 1u64;
                    nat_add(a470, a471)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f83(v0.clone(), v1, 225u8) {
                if if f83(v0.clone(), {
                    let a472 = v1;
                    let a473 = 1u64;
                    nat_add(a472, a473)?
                }, 154u8) {
                    f83(v0, {
                        let a474 = v1;
                        let a475 = 2u64;
                        nat_add(a474, a475)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f83(v0.clone(), v1, 226u8) {
                    if f83(v0.clone(), {
                        let a476 = v1;
                        let a477 = 1u64;
                        nat_add(a476, a477)?
                    }, 128u8) {
                        if if f82(v0.clone(), {
                            let a478 = v1;
                            let a479 = 2u64;
                            nat_add(a478, a479)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f83(v0.clone(), {
                                let a480 = v1;
                                let a481 = 2u64;
                                nat_add(a480, a481)?
                            }, 168u8) {
                                true
                            } else {
                                if f83(v0.clone(), {
                                    let a482 = v1;
                                    let a483 = 2u64;
                                    nat_add(a482, a483)?
                                }, 169u8) {
                                    true
                                } else {
                                    f83(v0, {
                                        let a484 = v1;
                                        let a485 = 2u64;
                                        nat_add(a484, a485)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f83(v0.clone(), {
                            let a486 = v1;
                            let a487 = 1u64;
                            nat_add(a486, a487)?
                        }, 129u8) {
                            f83(v0, {
                                let a488 = v1;
                                let a489 = 2u64;
                                nat_add(a488, a489)?
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
                    if f83(v0.clone(), v1, 227u8) {
                        if if f83(v0.clone(), {
                            let a490 = v1;
                            let a491 = 1u64;
                            nat_add(a490, a491)?
                        }, 128u8) {
                            f83(v0, {
                                let a492 = v1;
                                let a493 = 2u64;
                                nat_add(a492, a493)?
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

pub fn f61(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f41(v0.clone(), v1)? {
        Ok(Some({
            let a494 = v1;
            let a495 = f45(f34(v0, v1)?);
            nat_add(a494, a495)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f62(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f42(f34(v0, v1)?) {
        Ok(Some({
            let a496 = v1;
            let a497 = 1u64;
            nat_add(a496, a497)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f63(v0: Bytes, v1: List<Bytes>) -> bool {
    f84(Fn12::F85(v0), false, v1)
}

pub fn f64(v0: Bytes, v1: Adt17, v2: Bytes, v3: bool) -> R<Option<Adt17>> {
    if match v1.clone() {
        Adt17::C0(_, _, h498, _) => {
            h498
        }
    } {
        Ok(None::<Adt17>)
    } else {
        Ok(Some({
            let m502 = f11(v0.clone(), {
                let m501 = match v1.clone() {
                    Adt17::C0(_, h500, _, _) => {
                        h500
                    }
                };
                match m501 {
                    Adt10::C0(h499, _) => {
                        h499
                    }
                }
            })?;
            match m502 {
                Ok(v5) => {
                    if f12(v3, {
                        let a504 = match v5 {
                            Adt18::C0(h503, _, _, _) => {
                                h503
                            }
                        };
                        let a505 = 5u64;
                        nat_eq(a504, a505)
                    }) {
                        Adt17::C0(match v1.clone() {
                            Adt17::C0(h506, _, _, _) => {
                                h506
                            }
                        }, match v1 {
                            Adt17::C0(_, h507, _, _) => {
                                h507
                            }
                        }, true, None::<Adt8>)
                    } else {
                        let m509 = f74(v0.clone(), match v1.clone() {
                            Adt17::C0(_, h508, _, _) => {
                                h508
                            }
                        }, v2)?;
                        match m509 {
                            Ok(v7) => {
                                let m513 = f11(v0, {
                                    let m512 = match v7.clone() {
                                        Adt21::C0(_, h511) => {
                                            h511
                                        }
                                    };
                                    match m512 {
                                        Adt10::C0(h510, _) => {
                                            h510
                                        }
                                    }
                                })?;
                                match m513 {
                                    Ok(v9) => {
                                        let a515 = match v9.clone() {
                                            Adt18::C0(h514, _, _, _) => {
                                                h514
                                            }
                                        };
                                        let a516 = 6u64;
                                        if nat_eq(a515, a516) {
                                            Adt17::C0(List::cons(match v7 {
                                                Adt21::C0(h517, _) => {
                                                    h517
                                                }
                                            }, match v1 {
                                                Adt17::C0(h518, _, _, _) => {
                                                    h518
                                                }
                                            }), f17(v9), false, None::<Adt8>)
                                        } else {
                                            Adt17::C0(List::cons(match v7.clone() {
                                                Adt21::C0(h519, _) => {
                                                    h519
                                                }
                                            }, match v1 {
                                                Adt17::C0(h520, _, _, _) => {
                                                    h520
                                                }
                                            }), match v7 {
                                                Adt21::C0(_, h521) => {
                                                    h521
                                                }
                                            }, true, None::<Adt8>)
                                        }
                                    }
                                    Err(v8) => {
                                        f86(v1, v8)
                                    }
                                }
                            }
                            Err(v6) => {
                                f86(v1, v6)
                            }
                        }
                    }
                }
                Err(v4) => {
                    f86(v1, v4)
                }
            }
        }))
    }
}

pub fn f65(v0: Adt9, v1: Adt8) -> Adt9 {
    Adt9::C0(match v0.clone() {
        Adt9::C0(h522, _, _, _, _, _, _, _, _, _) => {
            h522
        }
    }, match v0.clone() {
        Adt9::C0(_, h523, _, _, _, _, _, _, _, _) => {
            h523
        }
    }, match v0.clone() {
        Adt9::C0(_, _, h524, _, _, _, _, _, _, _) => {
            h524
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, h525, _, _, _, _, _, _) => {
            h525
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h526, _, _, _, _, _) => {
            h526
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h527, _, _, _, _) => {
            h527
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h528, _, _, _) => {
            h528
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h529, _, _) => {
            h529
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, h530, _) => {
            h530
        }
    }, Some(v1))
}

pub fn f66(v0: Bytes, v1: Adt18) -> R<bool> {
    f87(f44(v0, {
        let a532 = match v1.clone() {
            Adt18::C0(_, h531, _, _) => {
                h531
            }
        };
        let a533 = 1u64;
        nat_add(a532, a533)?
    }, {
        let a538 = {
            let a536 = match v1.clone() {
                Adt18::C0(_, _, h534, _) => {
                    h534
                }
            };
            let a537 = match v1 {
                Adt18::C0(_, h535, _, _) => {
                    h535
                }
            };
            nat_sub(a536, a537)
        };
        let a539 = 1u64;
        nat_sub(a538, a539)
    }))
}

pub fn f67(v0: Bytes, v1: Adt18) -> R<u64> {
    f88(f44(v0, {
        let a541 = match v1.clone() {
            Adt18::C0(_, h540, _, _) => {
                h540
            }
        };
        let a542 = 1u64;
        nat_add(a541, a542)?
    }, {
        let a547 = {
            let a545 = match v1.clone() {
                Adt18::C0(_, _, h543, _) => {
                    h543
                }
            };
            let a546 = match v1 {
                Adt18::C0(_, h544, _, _) => {
                    h544
                }
            };
            nat_sub(a545, a546)
        };
        let a548 = 1u64;
        nat_sub(a547, a548)
    }))
}

pub fn f68(v0: Bytes, v1: Adt9) -> R<Adt9> {
    let (h549, _) = f89(Fn13::F90(v0.clone()), {
        let a553 = {
            let a551 = {
                let a550 = v0;
                length_bytes(a550)
            };
            let a552 = 8u64;
            nat_mul(a551, a552)?
        };
        let a554 = 16u64;
        nat_add(a553, a554)?
    }, f52(v1.clone(), match v1 {
        Adt9::C0(h555, _, _, _, _, _, _, _, _, _) => {
            h555
        }
    }, List::cons(Adt11::C0(0u64), List::<Adt11>::nil())))?;
    Ok(h549)
}

pub fn f69(v0: List<u64>) -> u64 {
    let m556 = v0;
    match m556.uncons() {
        None => {
            0u64
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f70(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: List<u64>, v4: List<Adt12>, v5: List<Bytes>) -> Adt9 {
    Adt9::C0(v1, v2, v3, v4, match v0.clone() {
        Adt9::C0(_, _, _, _, h557, _, _, _, _, _) => {
            h557
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h558, _, _, _, _) => {
            h558
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h559, _, _, _) => {
            h559
        }
    }, v5, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h560, _) => {
            h560
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h561) => {
            h561
        }
    })
}

pub fn f71(v0: List<u64>) -> List<u64> {
    let m562 = v0.clone();
    match m562.uncons() {
        None => {
            v0
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f72(v0: u64) -> Bytes {
    f91(f92(), 10u64, v0)
}

pub fn f73(v0: List<Adt13>, v1: Bytes) -> Option<u64> {
    f93(Fn14::F94(v1), None::<u64>, v0)
}

pub fn f74(v0: Bytes, v1: Adt10, v2: Bytes) -> R<Result<Adt21, Adt8>> {
    let m564 = f11(v0.clone(), match v1 {
        Adt10::C0(h563, _) => {
            h563
        }
    })?;
    match m564 {
        Ok(v3) => {
            let v4: Bytes = f14(v0.clone(), v3.clone());
            let a566 = match v3.clone() {
                Adt18::C0(h565, _, _, _) => {
                    h565
                }
            };
            let a567 = 20u64;
            if nat_eq(a566, a567) {
                if f63(v4.clone(), f95()) {
                    Ok(Err::<Adt21, Adt8>(f21(match v3 {
                        Adt18::C0(_, h568, _, _) => {
                            h568
                        }
                    }, f43(f43(f15(Str::lit("`")), v4), f15(Str::lit("` is a keyword; quote it to write a name"))))))
                } else {
                    Ok(Ok::<Adt21, Adt8>(Adt21::C0(v4, f17(v3))))
                }
            } else {
                let a570 = match v3.clone() {
                    Adt18::C0(h569, _, _, _) => {
                        h569
                    }
                };
                let a571 = 19u64;
                if nat_eq(a570, a571) {
                    Ok(Ok::<Adt21, Adt8>(Adt21::C0(f96(v0, match v3.clone() {
                        Adt18::C0(_, h572, _, _) => {
                            h572
                        }
                    })?, f17(v3))))
                } else {
                    Ok(Err::<Adt21, Adt8>(f21(match v3 {
                        Adt18::C0(_, h573, _, _) => {
                            h573
                        }
                    }, f43(f15(Str::lit("expected ")), v2))))
                }
            }
        }
        Err(v5) => {
            Ok(Err::<Adt21, Adt8>(v5))
        }
    }
}

pub fn f75(v0: Bytes, v1: Adt17) -> R<Option<Adt17>> {
    f97(v0, v1)
}

pub fn f76(v0: Bytes, v1: Bytes, v2: Bytes, v3: List<Adt6>, v4: Adt10, v5: u64) -> R<Result<(Adt15, Adt10), Adt8>> {
    let m575 = f11(v0.clone(), match v4.clone() {
        Adt10::C0(h574, _) => {
            h574
        }
    })?;
    match m575 {
        Ok(v7) => {
            let v14: Result<(Option<Adt16>, (Option<Adt16>, Adt10)), Adt8> = {
                let a577 = match v7.clone() {
                    Adt18::C0(h576, _, _, _) => {
                        h576
                    }
                };
                let a578 = 7u64;
                if nat_eq(a577, a578) {
                    let m579 = f98(v0.clone(), f17(v7), f15(Str::lit("the type")))?;
                    match m579 {
                        Ok(v9) => {
                            let m583 = f11(v0.clone(), {
                                let m582 = {
                                    let (_, h581) = v9.clone();
                                    h581
                                };
                                match m582 {
                                    Adt10::C0(h580, _) => {
                                        h580
                                    }
                                }
                            })?;
                            match m583 {
                                Ok(v11) => {
                                    let a585 = match v11.clone() {
                                        Adt18::C0(h584, _, _, _) => {
                                            h584
                                        }
                                    };
                                    let a586 = 8u64;
                                    if nat_eq(a585, a586) {
                                        let m587 = f98(v0.clone(), f17(v11), f15(Str::lit("the value")))?;
                                        match m587 {
                                            Ok(v13) => {
                                                Ok::<(Option<Adt16>, (Option<Adt16>, Adt10)), Adt8>((Some({
                                                    let (h588, _) = v9;
                                                    h588
                                                }), (Some({
                                                    let (h589, _) = v13;
                                                    h589
                                                }), {
                                                    let (_, h590) = v13;
                                                    h590
                                                })))
                                            }
                                            Err(v12) => {
                                                Err::<(Option<Adt16>, (Option<Adt16>, Adt10)), Adt8>(v12)
                                            }
                                        }
                                    } else {
                                        Ok::<(Option<Adt16>, (Option<Adt16>, Adt10)), Adt8>((Some({
                                            let (h591, _) = v9;
                                            h591
                                        }), (None::<Adt16>, {
                                            let (_, h592) = v9;
                                            h592
                                        })))
                                    }
                                }
                                Err(v10) => {
                                    Err::<(Option<Adt16>, (Option<Adt16>, Adt10)), Adt8>(v10)
                                }
                            }
                        }
                        Err(v8) => {
                            Err::<(Option<Adt16>, (Option<Adt16>, Adt10)), Adt8>(v8)
                        }
                    }
                } else {
                    Ok::<(Option<Adt16>, (Option<Adt16>, Adt10)), Adt8>((None::<Adt16>, (None::<Adt16>, v4)))
                }
            };
            let m593 = v14;
            match m593 {
                Ok(v16) => {
                    let v17: Adt10 = {
                        let (_, h594) = {
                            let (_, h595) = v16.clone();
                            h595
                        };
                        h594
                    };
                    let v19: bool = {
                        let m597 = {
                            let (h596, _) = v16.clone();
                            h596
                        };
                        match m597 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        }
                    };
                    let m599 = f11(v0.clone(), match v17.clone() {
                        Adt10::C0(h598, _) => {
                            h598
                        }
                    })?;
                    match m599 {
                        Ok(v21) => {
                            let v38: Result<(Adt6, Adt10), Adt8> = if f12({
                                let a601 = match v21.clone() {
                                    Adt18::C0(h600, _, _, _) => {
                                        h600
                                    }
                                };
                                let a602 = 2u64;
                                nat_eq(a601, a602)
                            }, !match v21.clone() {
                                Adt18::C0(_, _, _, h603) => {
                                    h603
                                }
                            }) {
                                let m605 = f99(v0.clone(), match v21.clone() {
                                    Adt18::C0(_, h604, _, _) => {
                                        h604
                                    }
                                })?;
                                match m605 {
                                    Ok(v23) => {
                                        let v24: u64 = match v21 {
                                            Adt18::C0(_, h606, _, _) => {
                                                h606
                                            }
                                        };
                                        let m609 = f100(f44(v0, v24, {
                                            let a607 = v23;
                                            let a608 = v24;
                                            nat_sub(a607, a608)
                                        }))?;
                                        match m609 {
                                            Ok(v26) => {
                                                let m614 = v26;
                                                match m614 {
                                                    Adt6::C0(_) => {
                                                        Err::<(Adt6, Adt10), Adt8>(f21(v24, f15(Str::lit("the members of a declaration are an object"))))
                                                    }
                                                    Adt6::C1(_, _) => {
                                                        Err::<(Adt6, Adt10), Adt8>(f21(v24, f15(Str::lit("the members of a declaration are an object"))))
                                                    }
                                                    Adt6::C2(_) => {
                                                        Err::<(Adt6, Adt10), Adt8>(f21(v24, f15(Str::lit("the members of a declaration are an object"))))
                                                    }
                                                    Adt6::C3(_) => {
                                                        Err::<(Adt6, Adt10), Adt8>(f21(v24, f15(Str::lit("the members of a declaration are an object"))))
                                                    }
                                                    Adt6::C4(r615) => {
                                                        let v27 = (*r615).clone();
                                                        Ok::<(Adt6, Adt10), Adt8>((v27, Adt10::C0(v23, v23)))
                                                    }
                                                    Adt6::C5 => {
                                                        Err::<(Adt6, Adt10), Adt8>(f21(v24, f15(Str::lit("the members of a declaration are an object"))))
                                                    }
                                                    Adt6::C6(_, _) => {
                                                        Err::<(Adt6, Adt10), Adt8>(f21(v24, f15(Str::lit("the members of a declaration are an object"))))
                                                    }
                                                    Adt6::C7(_, _, _) => {
                                                        Err::<(Adt6, Adt10), Adt8>(f21(v24, f15(Str::lit("the members of a declaration are an object"))))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Err::<(Adt6, Adt10), Adt8>(f21({
                                                    let a611 = v24;
                                                    let a612 = match v25.clone() {
                                                        Adt8::C0(h610, _) => {
                                                            h610
                                                        }
                                                    };
                                                    nat_add(a611, a612)?
                                                }, match v25 {
                                                    Adt8::C0(_, h613) => {
                                                        h613
                                                    }
                                                }))
                                            }
                                        }
                                    }
                                    Err(v22) => {
                                        Err::<(Adt6, Adt10), Adt8>(v22)
                                    }
                                }
                            } else {
                                Ok::<(Adt6, Adt10), Adt8>((Adt6::C5, v17))
                            };
                            let m616 = v38;
                            match m616 {
                                Ok(v40) => {
                                    let v41: Adt6 = f79(f79({
                                        let (h617, _) = v40;
                                        h617
                                    }, f15(Str::lit("kind")), Adt6::C2(v1)), f15(Str::lit("name")), Adt6::C2(v2));
                                    if v19 {
                                        Ok(Ok::<(Adt15, Adt10), Adt8>((Adt15::C0(f79(v41, f15(Str::lit("levels")), Adt6::C3(std::rc::Rc::new(f101(v3)))), {
                                            let (h618, _) = v16;
                                            h618
                                        }, {
                                            let (h619, _) = {
                                                let (_, h620) = v16;
                                                h620
                                            };
                                            h619
                                        }, v5), {
                                            let (_, h621) = v40;
                                            h621
                                        })))
                                    } else {
                                        let a623 = 0u64;
                                        let a624 = {
                                            let a622 = v3;
                                            length_list(a622)
                                        };
                                        if nat_lt(a623, a624) {
                                            Ok(Err::<(Adt15, Adt10), Adt8>(f21(v5, f15(Str::lit("a row without a type states no universe parameters")))))
                                        } else {
                                            Ok(Ok::<(Adt15, Adt10), Adt8>((Adt15::C0(v41, {
                                                let (h625, _) = v16;
                                                h625
                                            }, {
                                                let (h626, _) = {
                                                    let (_, h627) = v16;
                                                    h627
                                                };
                                                h626
                                            }, v5), {
                                                let (_, h628) = v40;
                                                h628
                                            })))
                                        }
                                    }
                                }
                                Err(v39) => {
                                    Ok(Err::<(Adt15, Adt10), Adt8>(v39))
                                }
                            }
                        }
                        Err(v20) => {
                            Ok(Err::<(Adt15, Adt10), Adt8>(v20))
                        }
                    }
                }
                Err(v15) => {
                    Ok(Err::<(Adt15, Adt10), Adt8>(v15))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<(Adt15, Adt10), Adt8>(v6))
        }
    }
}

pub fn f77(v0: Fn10, p632: List<Adt6>, p633: List<Bytes>) -> List<Adt6> {
    let m631 = (p632, p633);
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v1, v2) = m631;
                let m629 = v2;
                match m629.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Bytes>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Bytes>), List<Adt6>>(({
                            let c630: Fn10 = v0.clone();
                            c630.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<Adt6>, v1: Bytes) -> List<Adt6> {
    List::cons(Adt6::C2(v1), v0)
}

pub fn f79(v0: Adt6, v1: Bytes, v2: Adt6) -> Adt6 {
    let v3: Adt22 = f102(Fn15::F103(v1.clone(), v2.clone()), Adt22::C0(false, List::<Adt23>::nil()), f104(v0));
    let v4: List<Adt23> = if match v3.clone() {
        Adt22::C0(h634, _) => {
            h634
        }
    } {
        match v3 {
            Adt22::C0(_, h635) => {
                h635
            }
        }
    } else {
        List::cons(Adt23::C0(v1, v2), match v3 {
            Adt22::C0(_, h636) => {
                h636
            }
        })
    };
    f105(Fn16::F106, Adt6::C5, v4)
}

pub fn f80(v0: Fn11, p641: u64, p642: Adt20) -> R<(Adt20, bool)> {
    let m640 = (p641, p642);
    {
        let mut m640 = m640;
        loop {
            let step = {
                let (v1, v2) = m640;
                let m637 = v1;
                if m637 == 0 {
                    Err::<(u64, Adt20), R<(Adt20, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m637 - 1;
                    let m639 = {
                        let c638: Fn11 = v0.clone();
                        c638.apply(v2.clone())?
                    };
                    match m639 {
                        None => {
                            Err::<(u64, Adt20), R<(Adt20, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt20), R<(Adt20, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m640 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: u64, v1: Bytes, v2: Adt20) -> R<Option<Adt20>> {
    f107(v1, v0, v2)
}

pub fn f82(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m643 = f57(v0, v1);
    match m643 {
        None => {
            false
        }
        Some(v4) => {
            f108(v4, v2, v3)
        }
    }
}

pub fn f83(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m644 = f57(v0, v1);
    match m644 {
        None => {
            false
        }
        Some(v3) => {
            let a645 = v3;
            let a646 = v2;
            equal(a645, a646)
        }
    }
}

pub fn f84(v0: Fn12, p650: bool, p651: List<Bytes>) -> bool {
    let m649 = (p650, p651);
    {
        let mut m649 = m649;
        loop {
            let step = {
                let (v1, v2) = m649;
                let m647 = v2;
                match m647.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c648: Fn12 = v0.clone();
                            c648.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a652 = v2;
        let a653 = v0;
        equal(a652, a653)
    }
}

pub fn f86(v0: Adt17, v1: Adt8) -> Adt17 {
    Adt17::C0(match v0.clone() {
        Adt17::C0(h654, _, _, _) => {
            h654
        }
    }, match v0 {
        Adt17::C0(_, h655, _, _) => {
            h655
        }
    }, true, Some(v1))
}

pub fn f87(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let (h656, _) = f55(Fn9::F109(v0.clone()), {
            let a658 = {
                let a657 = v0.clone();
                length_bytes(a657)
            };
            let a659 = 1u64;
            nat_add(a658, a659)?
        }, 0u64)?;
        h656
    };
    let v2: Bytes = f44(v0.clone(), v1, {
        let a661 = {
            let a660 = v0;
            length_bytes(a660)
        };
        let a662 = v1;
        nat_sub(a661, a662)
    });
    let v3: u64 = {
        let a663 = v2.clone();
        length_bytes(a663)
    };
    let a664 = 20u64;
    let a665 = v3;
    if nat_lt(a664, a665) {
        Ok(true)
    } else {
        let a666 = v3;
        let a667 = 20u64;
        if nat_eq(a666, a667) {
            Ok(f110(Bytes::lit(&[0x31, 0x38, 0x34, 0x34, 0x36, 0x37, 0x34, 0x34, 0x30, 0x37, 0x33, 0x37, 0x30, 0x39, 0x35, 0x35, 0x31, 0x36, 0x31, 0x35]), v2))
        } else {
            Ok(false)
        }
    }
}

pub fn f88(v0: Bytes) -> R<u64> {
    let m673 = {
        let (h669, _) = f111(Fn17::F112(v0.clone()), {
            let a671 = {
                let a670 = v0;
                length_bytes(a670)
            };
            let a672 = 1u64;
            nat_add(a671, a672)?
        }, Adt24::C0(0u64, 0u64))?;
        h669
    };
    match m673 {
        Adt24::C0(h668, _) => {
            Ok(h668)
        }
    }
}

pub fn f89(v0: Fn13, p678: u64, p679: Adt9) -> R<(Adt9, bool)> {
    let m677 = (p678, p679);
    {
        let mut m677 = m677;
        loop {
            let step = {
                let (v1, v2) = m677;
                let m674 = v1;
                if m674 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m674 - 1;
                    let m676 = {
                        let c675: Fn13 = v0.clone();
                        c675.apply(v2.clone())?
                    };
                    match m676 {
                        None => {
                            Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt9), R<(Adt9, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: Adt9) -> R<Option<Adt9>> {
    f113(v0, v1)
}

pub fn f91(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a680 = v2;
    let a681 = 0u64;
    if nat_eq(a680, a681) {
        f114(v0, 0u64)
    } else {
        let (_, h682) = {
            let (h683, _) = f115(Fn18::F116(v1, v0), v2, (v2, Bytes::lit(&[])));
            h683
        };
        h682
    }
}

pub fn f92() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f93(v0: Fn14, p687: Option<u64>, p688: List<Adt13>) -> Option<u64> {
    let m686 = (p687, p688);
    {
        let mut m686 = m686;
        loop {
            let step = {
                let (v1, v2) = m686;
                let m684 = v2;
                match m684.uncons() {
                    None => {
                        Err::<(Option<u64>, List<Adt13>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<Adt13>), Option<u64>>(({
                            let c685: Fn14 = v0.clone();
                            c685.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Bytes, v1: Option<u64>, v2: Adt13) -> Option<u64> {
    let m689 = v1;
    match m689 {
        None => {
            if f13(match v2.clone() {
                Adt13::C0(h690, _) => {
                    h690
                }
            }, v0) {
                Some(match v2 {
                    Adt13::C0(_, h691) => {
                        h691
                    }
                })
            } else {
                None::<u64>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f95() -> List<Bytes> {
    List::cons(f15(Str::lit("fun")), List::cons(f15(Str::lit("let")), List::cons(f15(Str::lit("have")), List::cons(f15(Str::lit("proof")), List::cons(f15(Str::lit("Sort")), List::cons(f15(Str::lit("proj")), List::cons(f15(Str::lit("annot")), List::cons(f15(Str::lit("const")), List::cons(f15(Str::lit("max")), List::cons(f15(Str::lit("imax")), List::cons(f15(Str::lit("true")), List::cons(f15(Str::lit("false")), List::cons(f15(Str::lit("bool")), List::cons(f15(Str::lit("nat")), List::cons(f15(Str::lit("int")), List::cons(f15(Str::lit("str")), List::cons(f15(Str::lit("name")), List::<Bytes>::nil())))))))))))))))))
}

pub fn f96(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt20 = f58(v0.clone(), {
        let a692 = v0;
        length_bytes(a692)
    }, v1)?;
    Ok(f117(Fn19::F118, f15(Str::lit("")), match v2 {
        Adt20::C0(_, h693, _, _) => {
            h693
        }
    }))
}

pub fn f97(v0: Bytes, v1: Adt17) -> R<Option<Adt17>> {
    f64(v0, v1, f15(Str::lit("a universe parameter")), false)
}

pub fn f98(v0: Bytes, v1: Adt10, v2: Bytes) -> R<Result<(Adt16, Adt10), Adt8>> {
    let m695 = f11(v0.clone(), match v1 {
        Adt10::C0(h694, _) => {
            h694
        }
    })?;
    match m695 {
        Ok(v4) => {
            let a697 = match v4.clone() {
                Adt18::C0(h696, _, _, _) => {
                    h696
                }
            };
            let a698 = 15u64;
            if nat_eq(a697, a698) {
                let v5: u64 = if f66(v0.clone(), v4.clone())? {
                    0u64
                } else {
                    f67(v0.clone(), v4.clone())?
                };
                if f66(v0, v4.clone())? {
                    Ok(Err::<(Adt16, Adt10), Adt8>(f21(match v4 {
                        Adt18::C0(_, h699, _, _) => {
                            h699
                        }
                    }, f15(Str::lit("the number is too large")))))
                } else {
                    Ok(Ok::<(Adt16, Adt10), Adt8>((Adt16::C0(v5), f17(v4))))
                }
            } else {
                let a701 = match v4.clone() {
                    Adt18::C0(h700, _, _, _) => {
                        h700
                    }
                };
                let a702 = 16u64;
                if nat_eq(a701, a702) {
                    Ok(Ok::<(Adt16, Adt10), Adt8>((Adt16::C1(f44(v0, {
                        let a704 = match v4.clone() {
                            Adt18::C0(_, h703, _, _) => {
                                h703
                            }
                        };
                        let a705 = 1u64;
                        nat_add(a704, a705)?
                    }, {
                        let a710 = {
                            let a708 = match v4.clone() {
                                Adt18::C0(_, _, h706, _) => {
                                    h706
                                }
                            };
                            let a709 = match v4.clone() {
                                Adt18::C0(_, h707, _, _) => {
                                    h707
                                }
                            };
                            nat_sub(a708, a709)
                        };
                        let a711 = 1u64;
                        nat_sub(a710, a711)
                    }), match v4.clone() {
                        Adt18::C0(_, h712, _, _) => {
                            h712
                        }
                    }), f17(v4))))
                } else {
                    Ok(Err::<(Adt16, Adt10), Adt8>(f21(match v4 {
                        Adt18::C0(_, h713, _, _) => {
                            h713
                        }
                    }, f43(f15(Str::lit("expected `@N`, ")), v2))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(Adt16, Adt10), Adt8>(v3))
        }
    }
}

pub fn f99(v0: Bytes, v1: u64) -> R<Result<u64, Adt8>> {
    let m720 = {
        let m719 = {
            let (h715, _) = f119(Fn20::F120(v1, v0.clone()), {
                let a717 = {
                    let a716 = v0;
                    length_bytes(a716)
                };
                let a718 = 2u64;
                nat_add(a717, a718)?
            }, Adt25::C0(v1, 0u64, None::<Result<u64, Adt8>>))?;
            h715
        };
        match m719 {
            Adt25::C0(_, _, h714) => {
                h714
            }
        }
    };
    match m720 {
        None => {
            Ok(Err::<u64, Adt8>(f21(v1, f15(Str::lit("the braces of the declaration are not closed")))))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f100(v0: Bytes) -> R<Result<Adt6, Adt8>> {
    let m721 = f121(v0.clone())?;
    match m721 {
        Ok(v1) => {
            f122(v1, {
                let a722 = v0;
                length_bytes(a722)
            })
        }
        Err(v2) => {
            Ok(Err::<Adt6, Adt8>(v2))
        }
    }
}

pub fn f101(v0: List<Adt6>) -> Adt6 {
    f123(Fn21::F124, Adt6::C5, f125(v0))
}

pub fn f102(v0: Fn15, p726: Adt22, p727: List<Adt23>) -> Adt22 {
    let m725 = (p726, p727);
    {
        let mut m725 = m725;
        loop {
            let step = {
                let (v1, v2) = m725;
                let m723 = v2;
                match m723.uncons() {
                    None => {
                        Err::<(Adt22, List<Adt23>), Adt22>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt22, List<Adt23>), Adt22>(({
                            let c724: Fn15 = v0.clone();
                            c724.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: Adt6, v2: Adt22, v3: Adt23) -> Adt22 {
    f126(v2, v3, v0, v1)
}

pub fn f104(v0: Adt6) -> List<Adt23> {
    f127({
        let m730 = {
            let (h729, _) = f128(Fn22::F129, 1000000u64, Adt26::C0(v0, List::<Adt23>::nil()));
            h729
        };
        match m730 {
            Adt26::C0(_, h728) => {
                h728
            }
        }
    })
}

pub fn f105(v0: Fn16, p734: Adt6, p735: List<Adt23>) -> Adt6 {
    let m733 = (p734, p735);
    {
        let mut m733 = m733;
        loop {
            let step = {
                let (v1, v2) = m733;
                let m731 = v2;
                match m731.uncons() {
                    None => {
                        Err::<(Adt6, List<Adt23>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<Adt23>), Adt6>(({
                            let c732: Fn16 = v0.clone();
                            c732.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: Adt6, v1: Adt23) -> Adt6 {
    Adt6::C7(match v1.clone() {
        Adt23::C0(h736, _) => {
            h736
        }
    }, std::rc::Rc::new(match v1 {
        Adt23::C0(_, h737) => {
            h737
        }
    }), std::rc::Rc::new(v0))
}

pub fn f107(v0: Bytes, v1: u64, v2: Adt20) -> R<Option<Adt20>> {
    let m739 = match v2.clone() {
        Adt20::C0(_, _, _, h738) => {
            h738
        }
    };
    match m739 {
        None => {
            if match v2.clone() {
                Adt20::C0(_, _, h740, _) => {
                    h740
                }
            } {
                Ok(None::<Adt20>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt20::C0(h741, _, _, _) => {
                        h741
                    }
                };
                let a742 = v4;
                let a743 = v1;
                if nat_lt(a742, a743) {
                    if f130(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt20::C0({
                            let a744 = v4;
                            let a745 = 1u64;
                            nat_add(a744, a745)?
                        }, match v2 {
                            Adt20::C0(_, h746, _, _) => {
                                h746
                            }
                        }, true, None::<Adt8>)))
                    } else {
                        if f130(v0.clone(), v4, 92u8) {
                            Ok(Some(f131(v0, v1, v2)?))
                        } else {
                            let a747 = f132(v0.clone(), v4)?;
                            let a748 = 32u64;
                            if nat_lt(a747, a748) {
                                Ok(Some(f133(v2, v4, f134(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f135(v0.clone(), v1, v4)?;
                                Ok(Some(f136(v2, v5, f44(v0, v4, {
                                    let a749 = v5;
                                    let a750 = v4;
                                    nat_sub(a749, a750)
                                }))))
                            }
                        }
                    }
                } else {
                    Ok(None::<Adt20>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt20>)
        }
    }
}

pub fn f108(v0: u8, v1: u8, v2: u8) -> bool {
    let m753 = {
        let a751 = v0;
        let a752 = v1;
        fixed_u8::checked_sub(a751, a752)
    };
    match m753 {
        None => {
            false
        }
        Some(_) => {
            let m756 = {
                let a754 = v2;
                let a755 = v0;
                fixed_u8::checked_sub(a754, a755)
            };
            match m756 {
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

pub fn f109(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m757 = f57(v0, v1);
    match m757 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a804 = {
                let v3: u8 = v2;
                let a802 = {
                    let a778 = {
                        let a766 = {
                            let a760 = {
                                let a758 = v3;
                                let a759 = 128u8;
                                fixed_u8::bit_and(a758, a759)
                            };
                            let a761 = 128u8;
                            if equal(a760, a761) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a767 = {
                            let a764 = {
                                let a762 = v3;
                                let a763 = 64u8;
                                fixed_u8::bit_and(a762, a763)
                            };
                            let a765 = 64u8;
                            if equal(a764, a765) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a766, a767)?
                    };
                    let a779 = {
                        let a776 = {
                            let a770 = {
                                let a768 = v3;
                                let a769 = 32u8;
                                fixed_u8::bit_and(a768, a769)
                            };
                            let a771 = 32u8;
                            if equal(a770, a771) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a777 = {
                            let a774 = {
                                let a772 = v3;
                                let a773 = 16u8;
                                fixed_u8::bit_and(a772, a773)
                            };
                            let a775 = 16u8;
                            if equal(a774, a775) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a776, a777)?
                    };
                    nat_add(a778, a779)?
                };
                let a803 = {
                    let a800 = {
                        let a788 = {
                            let a782 = {
                                let a780 = v3;
                                let a781 = 8u8;
                                fixed_u8::bit_and(a780, a781)
                            };
                            let a783 = 8u8;
                            if equal(a782, a783) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a789 = {
                            let a786 = {
                                let a784 = v3;
                                let a785 = 4u8;
                                fixed_u8::bit_and(a784, a785)
                            };
                            let a787 = 4u8;
                            if equal(a786, a787) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a788, a789)?
                    };
                    let a801 = {
                        let a798 = {
                            let a792 = {
                                let a790 = v3;
                                let a791 = 2u8;
                                fixed_u8::bit_and(a790, a791)
                            };
                            let a793 = 2u8;
                            if equal(a792, a793) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a799 = {
                            let a796 = {
                                let a794 = v3;
                                let a795 = 1u8;
                                fixed_u8::bit_and(a794, a795)
                            };
                            let a797 = 1u8;
                            if equal(a796, a797) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a798, a799)?
                    };
                    nat_add(a800, a801)?
                };
                nat_add(a802, a803)?
            };
            let a805 = 48u64;
            if nat_eq(a804, a805) {
                Ok(Some({
                    let a806 = v1;
                    let a807 = 1u64;
                    nat_add(a806, a807)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f110(v0: Bytes, v1: Bytes) -> bool {
    let a812 = {
        let a808 = v0;
        let a809 = v1;
        compare_bytes(a808, a809)
    };
    let a813 = {
        let a810 = Bytes::lit(&[]);
        let a811 = Bytes::lit(&[0x01]);
        compare_bytes(a810, a811)
    };
    equal(a812, a813)
}

pub fn f111(v0: Fn17, p818: u64, p819: Adt24) -> R<(Adt24, bool)> {
    let m817 = (p818, p819);
    {
        let mut m817 = m817;
        loop {
            let step = {
                let (v1, v2) = m817;
                let m814 = v1;
                if m814 == 0 {
                    Err::<(u64, Adt24), R<(Adt24, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m814 - 1;
                    let m816 = {
                        let c815: Fn17 = v0.clone();
                        c815.apply(v2.clone())?
                    };
                    match m816 {
                        None => {
                            Err::<(u64, Adt24), R<(Adt24, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt24), R<(Adt24, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: Bytes, v1: Adt24) -> R<Option<Adt24>> {
    let m821 = f57(v0, match v1.clone() {
        Adt24::C0(_, h820) => {
            h820
        }
    });
    match m821 {
        None => {
            Ok(None::<Adt24>)
        }
        Some(v2) => {
            Ok(Some(Adt24::C0({
                let a873 = {
                    let a823 = match v1.clone() {
                        Adt24::C0(h822, _) => {
                            h822
                        }
                    };
                    let a824 = 10u64;
                    nat_mul(a823, a824)?
                };
                let a874 = {
                    let a871 = {
                        let v3: u8 = v2;
                        let a869 = {
                            let a845 = {
                                let a833 = {
                                    let a827 = {
                                        let a825 = v3;
                                        let a826 = 128u8;
                                        fixed_u8::bit_and(a825, a826)
                                    };
                                    let a828 = 128u8;
                                    if equal(a827, a828) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a834 = {
                                    let a831 = {
                                        let a829 = v3;
                                        let a830 = 64u8;
                                        fixed_u8::bit_and(a829, a830)
                                    };
                                    let a832 = 64u8;
                                    if equal(a831, a832) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a833, a834)?
                            };
                            let a846 = {
                                let a843 = {
                                    let a837 = {
                                        let a835 = v3;
                                        let a836 = 32u8;
                                        fixed_u8::bit_and(a835, a836)
                                    };
                                    let a838 = 32u8;
                                    if equal(a837, a838) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a844 = {
                                    let a841 = {
                                        let a839 = v3;
                                        let a840 = 16u8;
                                        fixed_u8::bit_and(a839, a840)
                                    };
                                    let a842 = 16u8;
                                    if equal(a841, a842) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a843, a844)?
                            };
                            nat_add(a845, a846)?
                        };
                        let a870 = {
                            let a867 = {
                                let a855 = {
                                    let a849 = {
                                        let a847 = v3;
                                        let a848 = 8u8;
                                        fixed_u8::bit_and(a847, a848)
                                    };
                                    let a850 = 8u8;
                                    if equal(a849, a850) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a856 = {
                                    let a853 = {
                                        let a851 = v3;
                                        let a852 = 4u8;
                                        fixed_u8::bit_and(a851, a852)
                                    };
                                    let a854 = 4u8;
                                    if equal(a853, a854) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a855, a856)?
                            };
                            let a868 = {
                                let a865 = {
                                    let a859 = {
                                        let a857 = v3;
                                        let a858 = 2u8;
                                        fixed_u8::bit_and(a857, a858)
                                    };
                                    let a860 = 2u8;
                                    if equal(a859, a860) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a866 = {
                                    let a863 = {
                                        let a861 = v3;
                                        let a862 = 1u8;
                                        fixed_u8::bit_and(a861, a862)
                                    };
                                    let a864 = 1u8;
                                    if equal(a863, a864) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a865, a866)?
                            };
                            nat_add(a867, a868)?
                        };
                        nat_add(a869, a870)?
                    };
                    let a872 = 48u64;
                    nat_sub(a871, a872)
                };
                nat_add(a873, a874)?
            }, {
                let a876 = match v1 {
                    Adt24::C0(_, h875) => {
                        h875
                    }
                };
                let a877 = 1u64;
                nat_add(a876, a877)?
            })))
        }
    }
}

pub fn f113(v0: Bytes, v1: Adt9) -> R<Option<Adt9>> {
    let m879 = match v1.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h878) => {
            h878
        }
    };
    match m879 {
        None => {
            let m881 = match v1.clone() {
                Adt9::C0(_, h880, _, _, _, _, _, _, _, _) => {
                    h880
                }
            };
            match m881.uncons() {
                None => {
                    Ok(None::<Adt9>)
                }
                Some((v3, v4)) => {
                    let v5: Adt10 = match v1.clone() {
                        Adt9::C0(h882, _, _, _, _, _, _, _, _, _) => {
                            h882
                        }
                    };
                    Ok(Some({
                        let m883 = v3;
                        match m883 {
                            Adt11::C0(v6) => {
                                f137(v0, v1, v4, v6)?
                            }
                            Adt11::C1(v7) => {
                                f138(v0, v1, v4, v7)?
                            }
                            Adt11::C2 => {
                                f139(v0, v1, v4)?
                            }
                            Adt11::C3(v8) => {
                                f140(v0, v1, v4, v8, 11u64, f15(Str::lit("`=>` after the binder")), Adt11::C4)?
                            }
                            Adt11::C4 => {
                                f141(v1, v4, true)?
                            }
                            Adt11::C5(v9) => {
                                f140(v0, v1, v4, v9, 10u64, f15(Str::lit("`->` after the binder")), Adt11::C6)?
                            }
                            Adt11::C6 => {
                                f141(v1, v4, false)?
                            }
                            Adt11::C7(v10) => {
                                let m884 = f16(v0, v5, 8u64, f15(Str::lit("`:=`")))?;
                                match m884 {
                                    Ok(v12) => {
                                        f52(v1, match v12 {
                                            Adt19::C0(_, h885) => {
                                                h885
                                            }
                                        }, List::cons(Adt11::C0({
                                            let a886 = v10;
                                            let a887 = 1u64;
                                            nat_add(a886, a887)?
                                        }), List::cons(Adt11::C8(v10), v4)))
                                    }
                                    Err(v11) => {
                                        f65(v1, v11)
                                    }
                                }
                            }
                            Adt11::C8(v13) => {
                                f140(v0, v1, v4, v13, 9u64, f15(Str::lit("`;` after the value")), Adt11::C9)?
                            }
                            Adt11::C9 => {
                                f142(v1, v4)?
                            }
                            Adt11::C10(v14) => {
                                let m889 = f11(v0, match v5.clone() {
                                    Adt10::C0(h888, _) => {
                                        h888
                                    }
                                })?;
                                match m889 {
                                    Ok(v16) => {
                                        let a891 = match v16.clone() {
                                            Adt18::C0(h890, _, _, _) => {
                                                h890
                                            }
                                        };
                                        let a892 = 10u64;
                                        if nat_eq(a891, a892) {
                                            f70(v1.clone(), f17(v16), List::cons(Adt11::C0({
                                                let a893 = v14;
                                                let a894 = 1u64;
                                                nat_add(a893, a894)?
                                            }), List::cons(Adt11::C11, v4)), match v1.clone() {
                                                Adt9::C0(_, _, h895, _, _, _, _, _, _, _) => {
                                                    h895
                                                }
                                            }, match v1.clone() {
                                                Adt9::C0(_, _, _, h896, _, _, _, _, _, _) => {
                                                    h896
                                                }
                                            }, List::cons(f15(Str::lit("")), match v1 {
                                                Adt9::C0(_, _, _, _, _, _, _, h897, _, _) => {
                                                    h897
                                                }
                                            }))
                                        } else {
                                            f52(v1, v5, v4)
                                        }
                                    }
                                    Err(v15) => {
                                        f65(v1, v15)
                                    }
                                }
                            }
                            Adt11::C11 => {
                                let v17: u64 = f69(match v1.clone() {
                                    Adt9::C0(_, _, h898, _, _, _, _, _, _, _) => {
                                        h898
                                    }
                                });
                                let v18: u64 = f69(f71(match v1.clone() {
                                    Adt9::C0(_, _, h899, _, _, _, _, _, _, _) => {
                                        h899
                                    }
                                }));
                                f143(v1.clone(), v5, v4, f71(f71(match v1.clone() {
                                    Adt9::C0(_, _, h900, _, _, _, _, _, _, _) => {
                                        h900
                                    }
                                })), match v1.clone() {
                                    Adt9::C0(_, _, _, h901, _, _, _, _, _, _) => {
                                        h901
                                    }
                                }, f144(match v1 {
                                    Adt9::C0(_, _, _, _, _, _, _, h902, _, _) => {
                                        h902
                                    }
                                }), Adt1::C5(f15(Str::lit("")), Adt3::C0, v18, v17))?
                            }
                            Adt11::C12(v19) => {
                                f145(v0, v1, v4, v19)?
                            }
                            Adt11::C13(v20) => {
                                let m903 = f146(v0, v5.clone())?;
                                match m903 {
                                    Ok(v22) => {
                                        if v22 {
                                            f52(v1, v5, List::cons(Adt11::C15({
                                                let a904 = v20;
                                                let a905 = 1u64;
                                                nat_add(a904, a905)?
                                            }), List::cons(Adt11::C14(v20), v4)))
                                        } else {
                                            f52(v1, v5, v4)
                                        }
                                    }
                                    Err(v21) => {
                                        f65(v1, v21)
                                    }
                                }
                            }
                            Adt11::C14(v23) => {
                                let v24: u64 = f69(match v1.clone() {
                                    Adt9::C0(_, _, h906, _, _, _, _, _, _, _) => {
                                        h906
                                    }
                                });
                                let v25: u64 = f69(f71(match v1.clone() {
                                    Adt9::C0(_, _, h907, _, _, _, _, _, _, _) => {
                                        h907
                                    }
                                }));
                                f143(v1.clone(), v5, List::cons(Adt11::C13(v23), v4), f71(f71(match v1.clone() {
                                    Adt9::C0(_, _, h908, _, _, _, _, _, _, _) => {
                                        h908
                                    }
                                })), match v1.clone() {
                                    Adt9::C0(_, _, _, h909, _, _, _, _, _, _) => {
                                        h909
                                    }
                                }, match v1 {
                                    Adt9::C0(_, _, _, _, _, _, _, h910, _, _) => {
                                        h910
                                    }
                                }, Adt1::C3(v25, v24))?
                            }
                            Adt11::C15(v26) => {
                                f147(v0, v1, v4, v26)?
                            }
                            Adt11::C16 => {
                                let m911 = f16(v0, v5, 1u64, f15(Str::lit("`)`")))?;
                                match m911 {
                                    Ok(v28) => {
                                        f52(v1, match v28 {
                                            Adt19::C0(_, h912) => {
                                                h912
                                            }
                                        }, v4)
                                    }
                                    Err(v27) => {
                                        f65(v1, v27)
                                    }
                                }
                            }
                            Adt11::C17 => {
                                f148(v0, v1, v4)?
                            }
                            Adt11::C18 => {
                                f149(v1, v4)?
                            }
                            Adt11::C19 => {
                                f149(v1, v4)?
                            }
                        }
                    }))
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt9>)
        }
    }
}

pub fn f114(v0: Bytes, v1: u64) -> Bytes {
    let m916 = {
        let a913 = v0;
        let a914 = v1;
        let a915 = 1u64;
        slice_bytes(a913, a914, a915)
    };
    match m916 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f115(v0: Fn18, p921: u64, p922: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m920 = (p921, p922);
    {
        let mut m920 = m920;
        loop {
            let step = {
                let (v1, v2) = m920;
                let m917 = v1;
                if m917 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m917 - 1;
                    let m919 = {
                        let c918: Fn18 = v0.clone();
                        c918.apply(v2.clone())
                    };
                    match m919 {
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
                Ok(next) => { m920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f150(v1, v0, v2)
}

pub fn f117(v0: Fn19, p926: Bytes, p927: List<Bytes>) -> Bytes {
    let m925 = (p926, p927);
    {
        let mut m925 = m925;
        loop {
            let step = {
                let (v1, v2) = m925;
                let m923 = v2;
                match m923.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let c924: Fn19 = v0.clone();
                            c924.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Bytes, v1: Bytes) -> Bytes {
    f43(v1, v0)
}

pub fn f119(v0: Fn20, p932: u64, p933: Adt25) -> R<(Adt25, bool)> {
    let m931 = (p932, p933);
    {
        let mut m931 = m931;
        loop {
            let step = {
                let (v1, v2) = m931;
                let m928 = v1;
                if m928 == 0 {
                    Err::<(u64, Adt25), R<(Adt25, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m928 - 1;
                    let m930 = {
                        let c929: Fn20 = v0.clone();
                        c929.apply(v2.clone())?
                    };
                    match m930 {
                        None => {
                            Err::<(u64, Adt25), R<(Adt25, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt25), R<(Adt25, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: u64, v1: Bytes, v2: Adt25) -> R<Option<Adt25>> {
    f151(v1, v0, v2)
}

pub fn f121(v0: Bytes) -> R<Result<List<Adt27>, Adt8>> {
    let v1: u64 = {
        let a934 = v0.clone();
        length_bytes(a934)
    };
    let v2: Adt29 = {
        let (h935, _) = f152(Fn23::F153(v1, v0), {
            let a936 = v1;
            let a937 = 1u64;
            nat_add(a936, a937)?
        }, Adt29::C0(0u64, List::<Adt27>::nil(), None::<Adt8>))?;
        h935
    };
    let m939 = match v2.clone() {
        Adt29::C0(_, _, h938) => {
            h938
        }
    };
    match m939 {
        None => {
            Ok(Ok::<List<Adt27>, Adt8>(f154(match v2 {
                Adt29::C0(_, h940, _) => {
                    h940
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt27>, Adt8>(v3))
        }
    }
}

pub fn f122(v0: List<Adt27>, v1: u64) -> R<Result<Adt6, Adt8>> {
    let v2: Adt30 = f155(v0.clone(), v1, 0u64, 0u64)?;
    let m942 = match v2.clone() {
        Adt30::C0(_, _, _, _, _, h941) => {
            h941
        }
    };
    match m942 {
        None => {
            let a944 = match v2.clone() {
                Adt30::C0(_, _, h943, _, _, _) => {
                    h943
                }
            };
            let a945 = 9u64;
            if nat_eq(a944, a945) {
                let a948 = match v2.clone() {
                    Adt30::C0(h946, _, _, _, _, _) => {
                        h946
                    }
                };
                let a949 = {
                    let a947 = v0.clone();
                    length_list(a947)
                };
                if nat_lt(a948, a949) {
                    Ok(Err::<Adt6, Adt8>(f156(f157(v0, v1, match v2 {
                        Adt30::C0(h950, _, _, _, _, _) => {
                            h950
                        }
                    }), f158(Str::lit("unexpected text after the value")))))
                } else {
                    Ok(Ok::<Adt6, Adt8>(match v2 {
                        Adt30::C0(_, _, _, _, h951, _) => {
                            h951
                        }
                    }))
                }
            } else {
                Ok(Err::<Adt6, Adt8>(f156(v1, f158(Str::lit("the reader ran out of steps")))))
            }
        }
        Some(v3) => {
            Ok(Err::<Adt6, Adt8>(v3))
        }
    }
}

pub fn f123(v0: Fn21, p955: Adt6, p956: List<Adt6>) -> Adt6 {
    let m954 = (p955, p956);
    {
        let mut m954 = m954;
        loop {
            let step = {
                let (v1, v2) = m954;
                let m952 = v2;
                match m952.uncons() {
                    None => {
                        Err::<(Adt6, List<Adt6>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<Adt6>), Adt6>(({
                            let c953: Fn21 = v0.clone();
                            c953.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: Adt6, v1: Adt6) -> Adt6 {
    Adt6::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f125(v0: List<Adt6>) -> List<Adt6> {
    f8(Fn1::F159, List::<Adt6>::nil(), v0)
}

pub fn f126(v0: Adt22, v1: Adt23, v2: Bytes, v3: Adt6) -> Adt22 {
    if f13(match v1.clone() {
        Adt23::C0(h957, _) => {
            h957
        }
    }, v2.clone()) {
        v0
    } else {
        if f12(!match v0.clone() {
            Adt22::C0(h958, _) => {
                h958
            }
        }, f110(v2.clone(), match v1.clone() {
            Adt23::C0(h959, _) => {
                h959
            }
        })) {
            Adt22::C0(true, List::cons(v1, List::cons(Adt23::C0(v2, v3), match v0 {
                Adt22::C0(_, h960) => {
                    h960
                }
            })))
        } else {
            Adt22::C0(match v0.clone() {
                Adt22::C0(h961, _) => {
                    h961
                }
            }, List::cons(v1, match v0 {
                Adt22::C0(_, h962) => {
                    h962
                }
            }))
        }
    }
}

pub fn f127(v0: List<Adt23>) -> List<Adt23> {
    f160(Fn24::F161, List::<Adt23>::nil(), v0)
}

pub fn f128(v0: Fn22, p967: u64, p968: Adt26) -> (Adt26, bool) {
    let m966 = (p967, p968);
    {
        let mut m966 = m966;
        loop {
            let step = {
                let (v1, v2) = m966;
                let m963 = v1;
                if m963 == 0 {
                    Err::<(u64, Adt26), (Adt26, bool)>((v2, false))
                } else {
                    let v3 = m963 - 1;
                    let m965 = {
                        let c964: Fn22 = v0.clone();
                        c964.apply(v2.clone())
                    };
                    match m965 {
                        None => {
                            Err::<(u64, Adt26), (Adt26, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt26), (Adt26, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m966 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Adt26) -> Option<Adt26> {
    let m970 = match v0.clone() {
        Adt26::C0(h969, _) => {
            h969
        }
    };
    match m970 {
        Adt6::C0(_) => {
            None::<Adt26>
        }
        Adt6::C1(_, _) => {
            None::<Adt26>
        }
        Adt6::C2(_) => {
            None::<Adt26>
        }
        Adt6::C3(_) => {
            None::<Adt26>
        }
        Adt6::C4(_) => {
            None::<Adt26>
        }
        Adt6::C5 => {
            None::<Adt26>
        }
        Adt6::C6(_, _) => {
            None::<Adt26>
        }
        Adt6::C7(v1, r972, r973) => {
            let v2 = (*r972).clone();
            let v3 = (*r973).clone();
            Some(Adt26::C0(v3, List::cons(Adt23::C0(v1, v2), match v0 {
                Adt26::C0(_, h971) => {
                    h971
                }
            })))
        }
    }
}

pub fn f130(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m974 = f57(v0, v1);
    match m974 {
        None => {
            false
        }
        Some(v3) => {
            let a975 = v3;
            let a976 = v2;
            equal(a975, a976)
        }
    }
}

pub fn f131(v0: Bytes, v1: u64, v2: Adt20) -> R<Adt20> {
    let v3: u64 = match v2.clone() {
        Adt20::C0(h977, _, _, _) => {
            h977
        }
    };
    let m980 = f57(v0.clone(), {
        let a978 = v3;
        let a979 = 1u64;
        nat_add(a978, a979)?
    });
    match m980 {
        None => {
            Ok(Adt20::C0(v1, match v2 {
                Adt20::C0(_, h981, _, _) => {
                    h981
                }
            }, false, None::<Adt8>))
        }
        Some(v4) => {
            let a982 = v4;
            let a983 = 34u8;
            if equal(a982, a983) {
                Ok(f136(v2, {
                    let a984 = v3;
                    let a985 = 2u64;
                    nat_add(a984, a985)?
                }, f134(Str::lit("\""))))
            } else {
                let a986 = v4;
                let a987 = 92u8;
                if equal(a986, a987) {
                    Ok(f136(v2, {
                        let a988 = v3;
                        let a989 = 2u64;
                        nat_add(a988, a989)?
                    }, f134(Str::lit("\\"))))
                } else {
                    let a990 = v4;
                    let a991 = 47u8;
                    if equal(a990, a991) {
                        Ok(f136(v2, {
                            let a992 = v3;
                            let a993 = 2u64;
                            nat_add(a992, a993)?
                        }, f134(Str::lit("/"))))
                    } else {
                        let a994 = v4;
                        let a995 = 98u8;
                        if equal(a994, a995) {
                            Ok(f136(v2, {
                                let a996 = v3;
                                let a997 = 2u64;
                                nat_add(a996, a997)?
                            }, f162(8u64)))
                        } else {
                            let a998 = v4;
                            let a999 = 102u8;
                            if equal(a998, a999) {
                                Ok(f136(v2, {
                                    let a1000 = v3;
                                    let a1001 = 2u64;
                                    nat_add(a1000, a1001)?
                                }, f162(12u64)))
                            } else {
                                let a1002 = v4;
                                let a1003 = 110u8;
                                if equal(a1002, a1003) {
                                    Ok(f136(v2, {
                                        let a1004 = v3;
                                        let a1005 = 2u64;
                                        nat_add(a1004, a1005)?
                                    }, f162(10u64)))
                                } else {
                                    let a1006 = v4;
                                    let a1007 = 114u8;
                                    if equal(a1006, a1007) {
                                        Ok(f136(v2, {
                                            let a1008 = v3;
                                            let a1009 = 2u64;
                                            nat_add(a1008, a1009)?
                                        }, f162(13u64)))
                                    } else {
                                        let a1010 = v4;
                                        let a1011 = 116u8;
                                        if equal(a1010, a1011) {
                                            Ok(f136(v2, {
                                                let a1012 = v3;
                                                let a1013 = 2u64;
                                                nat_add(a1012, a1013)?
                                            }, f162(9u64)))
                                        } else {
                                            let a1014 = v4;
                                            let a1015 = 117u8;
                                            if equal(a1014, a1015) {
                                                f163(v0, v2)
                                            } else {
                                                Ok(f133(v2, v3, f164(f164(f134(Str::lit("`\\")), f44(v0.clone(), {
                                                    let a1016 = v3;
                                                    let a1017 = 1u64;
                                                    nat_add(a1016, a1017)?
                                                }, f165(v0, {
                                                    let a1018 = v3;
                                                    let a1019 = 1u64;
                                                    nat_add(a1018, a1019)?
                                                })?)), f134(Str::lit("` is not an escape")))))
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

pub fn f132(v0: Bytes, v1: u64) -> R<u64> {
    let m1020 = f57(v0, v1);
    match m1020 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1065 = {
                let a1041 = {
                    let a1029 = {
                        let a1023 = {
                            let a1021 = v3;
                            let a1022 = 128u8;
                            fixed_u8::bit_and(a1021, a1022)
                        };
                        let a1024 = 128u8;
                        if equal(a1023, a1024) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1030 = {
                        let a1027 = {
                            let a1025 = v3;
                            let a1026 = 64u8;
                            fixed_u8::bit_and(a1025, a1026)
                        };
                        let a1028 = 64u8;
                        if equal(a1027, a1028) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1029, a1030)?
                };
                let a1042 = {
                    let a1039 = {
                        let a1033 = {
                            let a1031 = v3;
                            let a1032 = 32u8;
                            fixed_u8::bit_and(a1031, a1032)
                        };
                        let a1034 = 32u8;
                        if equal(a1033, a1034) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1040 = {
                        let a1037 = {
                            let a1035 = v3;
                            let a1036 = 16u8;
                            fixed_u8::bit_and(a1035, a1036)
                        };
                        let a1038 = 16u8;
                        if equal(a1037, a1038) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1039, a1040)?
                };
                nat_add(a1041, a1042)?
            };
            let a1066 = {
                let a1063 = {
                    let a1051 = {
                        let a1045 = {
                            let a1043 = v3;
                            let a1044 = 8u8;
                            fixed_u8::bit_and(a1043, a1044)
                        };
                        let a1046 = 8u8;
                        if equal(a1045, a1046) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1052 = {
                        let a1049 = {
                            let a1047 = v3;
                            let a1048 = 4u8;
                            fixed_u8::bit_and(a1047, a1048)
                        };
                        let a1050 = 4u8;
                        if equal(a1049, a1050) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1051, a1052)?
                };
                let a1064 = {
                    let a1061 = {
                        let a1055 = {
                            let a1053 = v3;
                            let a1054 = 2u8;
                            fixed_u8::bit_and(a1053, a1054)
                        };
                        let a1056 = 2u8;
                        if equal(a1055, a1056) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1062 = {
                        let a1059 = {
                            let a1057 = v3;
                            let a1058 = 1u8;
                            fixed_u8::bit_and(a1057, a1058)
                        };
                        let a1060 = 1u8;
                        if equal(a1059, a1060) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1061, a1062)?
                };
                nat_add(a1063, a1064)?
            };
            nat_add(a1065, a1066)
        }
    }
}

pub fn f133(v0: Adt20, v1: u64, v2: Bytes) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h1067, _, _, _) => {
            h1067
        }
    }, match v0 {
        Adt20::C0(_, h1068, _, _) => {
            h1068
        }
    }, false, Some(f166(v1, v2)))
}

pub fn f134(v0: Str) -> Bytes {
    let a1069 = v0;
    utf8_encode(a1069)
}

pub fn f135(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1070, _) = f55(Fn9::F167(v1, v0), {
        let a1071 = v1;
        let a1072 = v2;
        nat_sub(a1071, a1072)
    }, v2)?;
    Ok(h1070)
}

pub fn f136(v0: Adt20, v1: u64, v2: Bytes) -> Adt20 {
    Adt20::C0(v1, List::cons(v2, match v0 {
        Adt20::C0(_, h1073, _, _) => {
            h1073
        }
    }), false, None::<Adt8>)
}

pub fn f137(v0: Bytes, v1: Adt9, v2: List<Adt11>, v3: u64) -> R<Adt9> {
    let v4: Adt10 = match v1.clone() {
        Adt9::C0(h1074, _, _, _, _, _, _, _, _, _) => {
            h1074
        }
    };
    let m1076 = f11(v0.clone(), match v4.clone() {
        Adt10::C0(h1075, _) => {
            h1075
        }
    })?;
    match m1076 {
        Ok(v6) => {
            let m1078 = f168(v3, match v6.clone() {
                Adt18::C0(_, h1077, _, _) => {
                    h1077
                }
            });
            match m1078 {
                None => {
                    let v8: Bytes = {
                        let a1080 = match v6.clone() {
                            Adt18::C0(h1079, _, _, _) => {
                                h1079
                            }
                        };
                        let a1081 = 20u64;
                        if nat_eq(a1080, a1081) {
                            f14(v0.clone(), v6.clone())
                        } else {
                            f15(Str::lit(""))
                        }
                    };
                    if f13(v8.clone(), f15(Str::lit("fun"))) {
                        Ok(f52(v1, f17(v6), List::cons(Adt11::C1(v3), List::cons(Adt11::C3(v3), v2))))
                    } else {
                        if if f13(v8.clone(), f15(Str::lit("let"))) {
                            true
                        } else {
                            f13(v8.clone(), f15(Str::lit("have")))
                        } {
                            let m1082 = f169(v0.clone(), f17(v6))?;
                            match m1082 {
                                Ok(v10) => {
                                    let m1084 = f16(v0, match v10.clone() {
                                        Adt21::C0(_, h1083) => {
                                            h1083
                                        }
                                    }, 7u64, f15(Str::lit("`:` after the name")))?;
                                    match m1084 {
                                        Ok(v12) => {
                                            Ok(f70(v1.clone(), match v12 {
                                                Adt19::C0(_, h1085) => {
                                                    h1085
                                                }
                                            }, List::cons(Adt11::C0({
                                                let a1086 = v3;
                                                let a1087 = 1u64;
                                                nat_add(a1086, a1087)?
                                            }), List::cons(Adt11::C7(v3), v2)), match v1.clone() {
                                                Adt9::C0(_, _, h1088, _, _, _, _, _, _, _) => {
                                                    h1088
                                                }
                                            }, List::cons(Adt12::C1(match v10 {
                                                Adt21::C0(h1089, _) => {
                                                    h1089
                                                }
                                            }, f13(v8, f15(Str::lit("have")))), match v1.clone() {
                                                Adt9::C0(_, _, _, h1090, _, _, _, _, _, _) => {
                                                    h1090
                                                }
                                            }), match v1 {
                                                Adt9::C0(_, _, _, _, _, _, _, h1091, _, _) => {
                                                    h1091
                                                }
                                            }))
                                        }
                                        Err(v11) => {
                                            Ok(f65(v1, v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f65(v1, v9))
                                }
                            }
                        } else {
                            let m1092 = f170(v0, v4.clone())?;
                            match m1092 {
                                Ok(v14) => {
                                    if v14 {
                                        Ok(f52(v1, v4, List::cons(Adt11::C1(v3), List::cons(Adt11::C5(v3), v2))))
                                    } else {
                                        Ok(f52(v1, v4, List::cons(Adt11::C12(v3), List::cons(Adt11::C10(v3), v2))))
                                    }
                                }
                                Err(v13) => {
                                    Ok(f65(v1, v13))
                                }
                            }
                        }
                    }
                }
                Some(v7) => {
                    Ok(f65(v1, v7))
                }
            }
        }
        Err(v5) => {
            Ok(f65(v1, v5))
        }
    }
}

pub fn f138(v0: Bytes, v1: Adt9, v2: List<Adt11>, v3: u64) -> R<Adt9> {
    let v4: Adt10 = match v1.clone() {
        Adt9::C0(h1093, _, _, _, _, _, _, _, _, _) => {
            h1093
        }
    };
    let m1095 = f11(v0.clone(), match v4 {
        Adt10::C0(h1094, _) => {
            h1094
        }
    })?;
    match m1095 {
        Ok(v6) => {
            let v7: u64 = match v6.clone() {
                Adt18::C0(h1096, _, _, _) => {
                    h1096
                }
            };
            let v8: Adt10 = f17(v6.clone());
            let v11: Result<(u64, Adt10), Adt8> = {
                let a1097 = v7;
                let a1098 = 0u64;
                if nat_eq(a1097, a1098) {
                    Ok::<(u64, Adt10), Adt8>((0u64, v8))
                } else {
                    let a1099 = v7;
                    let a1100 = 4u64;
                    if nat_eq(a1099, a1100) {
                        Ok::<(u64, Adt10), Adt8>((3u64, v8))
                    } else {
                        let a1101 = v7;
                        let a1102 = 2u64;
                        if nat_eq(a1101, a1102) {
                            let m1104 = f11(v0.clone(), match v8.clone() {
                                Adt10::C0(h1103, _) => {
                                    h1103
                                }
                            })?;
                            match m1104 {
                                Ok(v10) => {
                                    let a1106 = match v10.clone() {
                                        Adt18::C0(h1105, _, _, _) => {
                                            h1105
                                        }
                                    };
                                    let a1107 = 2u64;
                                    if nat_eq(a1106, a1107) {
                                        Ok::<(u64, Adt10), Adt8>((2u64, f17(v10)))
                                    } else {
                                        Ok::<(u64, Adt10), Adt8>((1u64, v8))
                                    }
                                }
                                Err(v9) => {
                                    Err::<(u64, Adt10), Adt8>(v9)
                                }
                            }
                        } else {
                            Err::<(u64, Adt10), Adt8>(f21(match v6 {
                                Adt18::C0(_, h1108, _, _) => {
                                    h1108
                                }
                            }, f15(Str::lit("expected a binder"))))
                        }
                    }
                }
            };
            let m1109 = v11;
            match m1109 {
                Ok(v13) => {
                    let m1111 = f169(v0.clone(), {
                        let (_, h1110) = v13;
                        h1110
                    })?;
                    match m1111 {
                        Ok(v15) => {
                            let m1113 = f16(v0, match v15.clone() {
                                Adt21::C0(_, h1112) => {
                                    h1112
                                }
                            }, 7u64, f15(Str::lit("`:` after the binder's name")))?;
                            match m1113 {
                                Ok(v17) => {
                                    Ok(f70(v1.clone(), match v17 {
                                        Adt19::C0(_, h1114) => {
                                            h1114
                                        }
                                    }, List::cons(Adt11::C0({
                                        let a1115 = v3;
                                        let a1116 = 1u64;
                                        nat_add(a1115, a1116)?
                                    }), List::cons(Adt11::C2, v2)), match v1.clone() {
                                        Adt9::C0(_, _, h1117, _, _, _, _, _, _, _) => {
                                            h1117
                                        }
                                    }, List::cons(Adt12::C0({
                                        let (h1118, _) = v13;
                                        h1118
                                    }, match v15 {
                                        Adt21::C0(h1119, _) => {
                                            h1119
                                        }
                                    }), match v1.clone() {
                                        Adt9::C0(_, _, _, h1120, _, _, _, _, _, _) => {
                                            h1120
                                        }
                                    }), match v1 {
                                        Adt9::C0(_, _, _, _, _, _, _, h1121, _, _) => {
                                            h1121
                                        }
                                    }))
                                }
                                Err(v16) => {
                                    Ok(f65(v1, v16))
                                }
                            }
                        }
                        Err(v14) => {
                            Ok(f65(v1, v14))
                        }
                    }
                }
                Err(v12) => {
                    Ok(f65(v1, v12))
                }
            }
        }
        Err(v5) => {
            Ok(f65(v1, v5))
        }
    }
}

pub fn f139(v0: Bytes, v1: Adt9, v2: List<Adt11>) -> R<Adt9> {
    let v3: u64 = f171(match v1.clone() {
        Adt9::C0(_, _, _, h1122, _, _, _, _, _, _) => {
            h1122
        }
    });
    let m1124 = f16(v0.clone(), match v1.clone() {
        Adt9::C0(h1123, _, _, _, _, _, _, _, _, _) => {
            h1123
        }
    }, f172(v3), f15(Str::lit("the binder's closing bracket")))?;
    match m1124 {
        Ok(v5) => {
            let a1125 = v3;
            let a1126 = 2u64;
            if nat_eq(a1125, a1126) {
                let m1128 = f16(v0, match v5 {
                    Adt19::C0(_, h1127) => {
                        h1127
                    }
                }, 3u64, f15(Str::lit("`}}`")))?;
                match m1128 {
                    Ok(v7) => {
                        Ok(f52(v1, match v7 {
                            Adt19::C0(_, h1129) => {
                                h1129
                            }
                        }, v2))
                    }
                    Err(v6) => {
                        Ok(f65(v1, v6))
                    }
                }
            } else {
                Ok(f52(v1, match v5 {
                    Adt19::C0(_, h1130) => {
                        h1130
                    }
                }, v2))
            }
        }
        Err(v4) => {
            Ok(f65(v1, v4))
        }
    }
}

pub fn f140(v0: Bytes, v1: Adt9, v2: List<Adt11>, v3: u64, v4: u64, v5: Bytes, v6: Adt11) -> R<Adt9> {
    let m1132 = f16(v0, match v1.clone() {
        Adt9::C0(h1131, _, _, _, _, _, _, _, _, _) => {
            h1131
        }
    }, v4, v5)?;
    match m1132 {
        Ok(v8) => {
            Ok(f70(v1.clone(), match v8 {
                Adt19::C0(_, h1133) => {
                    h1133
                }
            }, List::cons(Adt11::C0({
                let a1134 = v3;
                let a1135 = 1u64;
                nat_add(a1134, a1135)?
            }), List::cons(v6, v2)), match v1.clone() {
                Adt9::C0(_, _, h1136, _, _, _, _, _, _, _) => {
                    h1136
                }
            }, match v1.clone() {
                Adt9::C0(_, _, _, h1137, _, _, _, _, _, _) => {
                    h1137
                }
            }, List::cons(f173(match v1.clone() {
                Adt9::C0(_, _, _, h1138, _, _, _, _, _, _) => {
                    h1138
                }
            }), match v1 {
                Adt9::C0(_, _, _, _, _, _, _, h1139, _, _) => {
                    h1139
                }
            })))
        }
        Err(v7) => {
            Ok(f65(v1, v7))
        }
    }
}

pub fn f141(v0: Adt9, v1: List<Adt11>, v2: bool) -> R<Adt9> {
    let v3: List<Adt12> = match v0.clone() {
        Adt9::C0(_, _, _, h1140, _, _, _, _, _, _) => {
            h1140
        }
    };
    let v4: u64 = f69(match v0.clone() {
        Adt9::C0(_, _, h1141, _, _, _, _, _, _, _) => {
            h1141
        }
    });
    let v5: u64 = f69(f71(match v0.clone() {
        Adt9::C0(_, _, h1142, _, _, _, _, _, _, _) => {
            h1142
        }
    }));
    let v6: Bytes = f173(v3.clone());
    let v7: Adt3 = f174(f171(v3.clone()));
    f143(v0.clone(), match v0.clone() {
        Adt9::C0(h1143, _, _, _, _, _, _, _, _, _) => {
            h1143
        }
    }, v1, f71(f71(match v0.clone() {
        Adt9::C0(_, _, h1144, _, _, _, _, _, _, _) => {
            h1144
        }
    })), f175(v3), f144(match v0 {
        Adt9::C0(_, _, _, _, _, _, _, h1145, _, _) => {
            h1145
        }
    }), if v2 {
        Adt1::C4(v6, v7, v5, v4)
    } else {
        Adt1::C5(v6, v7, v5, v4)
    })
}

pub fn f142(v0: Adt9, v1: List<Adt11>) -> R<Adt9> {
    let v2: List<u64> = match v0.clone() {
        Adt9::C0(_, _, h1146, _, _, _, _, _, _, _) => {
            h1146
        }
    };
    let v3: u64 = f69(v2.clone());
    let v4: u64 = f69(f71(v2.clone()));
    let v5: u64 = f69(f71(f71(v2.clone())));
    let m1148 = match v0.clone() {
        Adt9::C0(_, _, _, h1147, _, _, _, _, _, _) => {
            h1147
        }
    };
    match m1148.uncons() {
        None => {
            Ok(v0)
        }
        Some((v6, v7)) => {
            let m1149 = v6;
            match m1149 {
                Adt12::C0(_, _) => {
                    Ok(v0)
                }
                Adt12::C1(v8, v9) => {
                    f143(v0.clone(), match v0.clone() {
                        Adt9::C0(h1150, _, _, _, _, _, _, _, _, _) => {
                            h1150
                        }
                    }, v1, f71(f71(f71(v2))), v7, f144(match v0 {
                        Adt9::C0(_, _, _, _, _, _, _, h1151, _, _) => {
                            h1151
                        }
                    }), Adt1::C6(v8, v5, v4, v3, v9))
                }
                Adt12::C2(_, _, _) => {
                    Ok(v0)
                }
                Adt12::C3(_) => {
                    Ok(v0)
                }
                Adt12::C4(_) => {
                    Ok(v0)
                }
            }
        }
    }
}

pub fn f143(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: List<u64>, v4: List<Adt12>, v5: List<Bytes>, v6: Adt1) -> R<Adt9> {
    Ok(Adt9::C0(v1, v2, List::cons(match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1152, _, _, _, _) => {
            h1152
        }
    }, v3), v4, List::cons(v6, match v0.clone() {
        Adt9::C0(_, _, _, _, h1153, _, _, _, _, _) => {
            h1153
        }
    }), {
        let a1155 = match v0.clone() {
            Adt9::C0(_, _, _, _, _, h1154, _, _, _, _) => {
                h1154
            }
        };
        let a1156 = 1u64;
        nat_add(a1155, a1156)?
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1157, _, _, _) => {
            h1157
        }
    }, v5, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1158, _) => {
            h1158
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1159) => {
            h1159
        }
    }))
}

pub fn f144(v0: List<Bytes>) -> List<Bytes> {
    let m1160 = v0.clone();
    match m1160.uncons() {
        None => {
            v0
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f145(v0: Bytes, v1: Adt9, v2: List<Adt11>, v3: u64) -> R<Adt9> {
    let v4: Adt10 = match v1.clone() {
        Adt9::C0(h1161, _, _, _, _, _, _, _, _, _) => {
            h1161
        }
    };
    let m1163 = f11(v0.clone(), match v4.clone() {
        Adt10::C0(h1162, _) => {
            h1162
        }
    })?;
    match m1163 {
        Ok(v6) => {
            let v7: Bytes = {
                let a1165 = match v6.clone() {
                    Adt18::C0(h1164, _, _, _) => {
                        h1164
                    }
                };
                let a1166 = 20u64;
                if nat_eq(a1165, a1166) {
                    f14(v0.clone(), v6.clone())
                } else {
                    f15(Str::lit(""))
                }
            };
            if f13(v7.clone(), f15(Str::lit("Sort"))) {
                let m1169 = f176(v0, f17(v6), {
                    let a1167 = v3;
                    let a1168 = 1u64;
                    nat_add(a1167, a1168)?
                })?;
                match m1169 {
                    Ok(v9) => {
                        f143(v1.clone(), match v9.clone() {
                            Adt32::C0(_, h1170) => {
                                h1170
                            }
                        }, v2, match v1.clone() {
                            Adt9::C0(_, _, h1171, _, _, _, _, _, _, _) => {
                                h1171
                            }
                        }, match v1.clone() {
                            Adt9::C0(_, _, _, h1172, _, _, _, _, _, _) => {
                                h1172
                            }
                        }, match v1 {
                            Adt9::C0(_, _, _, _, _, _, _, h1173, _, _) => {
                                h1173
                            }
                        }, Adt1::C1(match v9 {
                            Adt32::C0(h1174, _) => {
                                h1174
                            }
                        }))
                    }
                    Err(v8) => {
                        Ok(f65(v1, v8))
                    }
                }
            } else {
                if f13(v7.clone(), f15(Str::lit("proj"))) {
                    let m1175 = f74(v0.clone(), f17(v6), f15(Str::lit("a structure name")))?;
                    match m1175 {
                        Ok(v11) => {
                            let m1177 = f74(v0.clone(), match v11.clone() {
                                Adt21::C0(_, h1176) => {
                                    h1176
                                }
                            }, f15(Str::lit("a projection name")))?;
                            match m1177 {
                                Ok(v13) => {
                                    let m1179 = f177(v0, match v13.clone() {
                                        Adt21::C0(_, h1178) => {
                                            h1178
                                        }
                                    }, f15(Str::lit("a field index")))?;
                                    match m1179 {
                                        Ok(v15) => {
                                            Ok(f70(v1.clone(), match v15.clone() {
                                                Adt35::C0(_, h1180) => {
                                                    h1180
                                                }
                                            }, List::cons(Adt11::C15({
                                                let a1181 = v3;
                                                let a1182 = 1u64;
                                                nat_add(a1181, a1182)?
                                            }), List::cons(Adt11::C18, v2)), match v1.clone() {
                                                Adt9::C0(_, _, h1183, _, _, _, _, _, _, _) => {
                                                    h1183
                                                }
                                            }, List::cons(Adt12::C2(match v13 {
                                                Adt21::C0(h1184, _) => {
                                                    h1184
                                                }
                                            }, match v11 {
                                                Adt21::C0(h1185, _) => {
                                                    h1185
                                                }
                                            }, match v15 {
                                                Adt35::C0(h1186, _) => {
                                                    h1186
                                                }
                                            }), match v1.clone() {
                                                Adt9::C0(_, _, _, h1187, _, _, _, _, _, _) => {
                                                    h1187
                                                }
                                            }), match v1 {
                                                Adt9::C0(_, _, _, _, _, _, _, h1188, _, _) => {
                                                    h1188
                                                }
                                            }))
                                        }
                                        Err(v14) => {
                                            Ok(f65(v1, v14))
                                        }
                                    }
                                }
                                Err(v12) => {
                                    Ok(f65(v1, v12))
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(f65(v1, v10))
                        }
                    }
                } else {
                    if f13(v7, f15(Str::lit("annot"))) {
                        let m1189 = f178(v0, f17(v6))?;
                        match m1189 {
                            Ok(v17) => {
                                Ok(f70(v1.clone(), {
                                    let (_, h1190) = v17;
                                    h1190
                                }, List::cons(Adt11::C15({
                                    let a1191 = v3;
                                    let a1192 = 1u64;
                                    nat_add(a1191, a1192)?
                                }), List::cons(Adt11::C19, v2)), match v1.clone() {
                                    Adt9::C0(_, _, h1193, _, _, _, _, _, _, _) => {
                                        h1193
                                    }
                                }, List::cons(Adt12::C3({
                                    let (h1194, _) = v17;
                                    h1194
                                }), match v1.clone() {
                                    Adt9::C0(_, _, _, h1195, _, _, _, _, _, _) => {
                                        h1195
                                    }
                                }), match v1 {
                                    Adt9::C0(_, _, _, _, _, _, _, h1196, _, _) => {
                                        h1196
                                    }
                                }))
                            }
                            Err(v16) => {
                                Ok(f65(v1, v16))
                            }
                        }
                    } else {
                        Ok(f52(v1, v4, List::cons(Adt11::C15(v3), List::cons(Adt11::C13(v3), v2))))
                    }
                }
            }
        }
        Err(v5) => {
            Ok(f65(v1, v5))
        }
    }
}

pub fn f146(v0: Bytes, v1: Adt10) -> R<Result<bool, Adt8>> {
    let m1198 = f11(v0.clone(), match v1 {
        Adt10::C0(h1197, _) => {
            h1197
        }
    })?;
    match m1198 {
        Ok(v3) => {
            let v4: u64 = match v3.clone() {
                Adt18::C0(h1199, _, _, _) => {
                    h1199
                }
            };
            Ok(Ok::<bool, Adt8>(if match v3.clone() {
                Adt18::C0(_, _, _, h1200) => {
                    h1200
                }
            } {
                false
            } else {
                let a1201 = v4;
                let a1202 = 0u64;
                if if nat_eq(a1201, a1202) {
                    true
                } else {
                    f12({
                        let a1203 = 15u64;
                        let a1204 = v4;
                        nat_le(a1203, a1204)
                    }, {
                        let a1205 = v4;
                        let a1206 = 19u64;
                        nat_le(a1205, a1206)
                    })
                } {
                    true
                } else {
                    let a1207 = v4;
                    let a1208 = 20u64;
                    if nat_eq(a1207, a1208) {
                        !f63(f14(v0, v3), List::cons(f15(Str::lit("fun")), List::cons(f15(Str::lit("let")), List::cons(f15(Str::lit("have")), List::cons(f15(Str::lit("Sort")), List::cons(f15(Str::lit("proj")), List::cons(f15(Str::lit("annot")), List::<Bytes>::nil())))))))
                    } else {
                        false
                    }
                }
            }))
        }
        Err(v2) => {
            Ok(Err::<bool, Adt8>(v2))
        }
    }
}

pub fn f147(v0: Bytes, v1: Adt9, v2: List<Adt11>, v3: u64) -> R<Adt9> {
    let m1212 = f11(v0.clone(), {
        let m1211 = match v1.clone() {
            Adt9::C0(h1210, _, _, _, _, _, _, _, _, _) => {
                h1210
            }
        };
        match m1211 {
            Adt10::C0(h1209, _) => {
                h1209
            }
        }
    })?;
    match m1212 {
        Ok(v5) => {
            let m1214 = f168(v3, match v5.clone() {
                Adt18::C0(_, h1213, _, _) => {
                    h1213
                }
            });
            match m1214 {
                None => {
                    let v7: u64 = match v5.clone() {
                        Adt18::C0(h1215, _, _, _) => {
                            h1215
                        }
                    };
                    let v8: Adt10 = f17(v5.clone());
                    let a1216 = v7;
                    let a1217 = 15u64;
                    if nat_eq(a1216, a1217) {
                        let v9: u64 = if f66(v0.clone(), v5.clone())? {
                            0u64
                        } else {
                            f67(v0.clone(), v5.clone())?
                        };
                        if f66(v0, v5.clone())? {
                            Ok(f65(v1, f21(match v5 {
                                Adt18::C0(_, h1218, _, _) => {
                                    h1218
                                }
                            }, f15(Str::lit("the number is too large")))))
                        } else {
                            let a1220 = match v1.clone() {
                                Adt9::C0(_, _, _, _, _, h1219, _, _, _, _) => {
                                    h1219
                                }
                            };
                            let a1221 = v9;
                            if nat_le(a1220, a1221) {
                                Ok(f65(v1, f21(match v5 {
                                    Adt18::C0(_, h1222, _, _) => {
                                        h1222
                                    }
                                }, f43(f43(f15(Str::lit("`@")), f72(v9)), f15(Str::lit("` is not a node that has been written"))))))
                            } else {
                                Ok(f179(v1, v8, v2, v9))
                            }
                        }
                    } else {
                        let a1223 = v7;
                        let a1224 = 16u64;
                        if nat_eq(a1223, a1224) {
                            let v10: Bytes = f44(v0, {
                                let a1226 = match v5.clone() {
                                    Adt18::C0(_, h1225, _, _) => {
                                        h1225
                                    }
                                };
                                let a1227 = 1u64;
                                nat_add(a1226, a1227)?
                            }, {
                                let a1232 = {
                                    let a1230 = match v5.clone() {
                                        Adt18::C0(_, _, h1228, _) => {
                                            h1228
                                        }
                                    };
                                    let a1231 = match v5.clone() {
                                        Adt18::C0(_, h1229, _, _) => {
                                            h1229
                                        }
                                    };
                                    nat_sub(a1230, a1231)
                                };
                                let a1233 = 1u64;
                                nat_sub(a1232, a1233)
                            });
                            let m1235 = f73(match v1.clone() {
                                Adt9::C0(_, _, _, _, _, _, _, _, h1234, _) => {
                                    h1234
                                }
                            }, v10.clone());
                            match m1235 {
                                None => {
                                    Ok(f65(v1, f21(match v5 {
                                        Adt18::C0(_, h1236, _, _) => {
                                            h1236
                                        }
                                    }, f43(f43(f15(Str::lit("no statement written before this is named `@")), v10), f15(Str::lit("`"))))))
                                }
                                Some(v11) => {
                                    Ok(f179(v1, v8, v2, v11))
                                }
                            }
                        } else {
                            let a1237 = v7;
                            let a1238 = 17u64;
                            if nat_eq(a1237, a1238) {
                                let v12: u64 = if f66(v0.clone(), v5.clone())? {
                                    0u64
                                } else {
                                    f67(v0.clone(), v5.clone())?
                                };
                                if f66(v0, v5.clone())? {
                                    Ok(f65(v1, f21(match v5 {
                                        Adt18::C0(_, h1239, _, _) => {
                                            h1239
                                        }
                                    }, f15(Str::lit("the number is too large")))))
                                } else {
                                    f143(v1.clone(), v8, v2, match v1.clone() {
                                        Adt9::C0(_, _, h1240, _, _, _, _, _, _, _) => {
                                            h1240
                                        }
                                    }, match v1.clone() {
                                        Adt9::C0(_, _, _, h1241, _, _, _, _, _, _) => {
                                            h1241
                                        }
                                    }, match v1 {
                                        Adt9::C0(_, _, _, _, _, _, _, h1242, _, _) => {
                                            h1242
                                        }
                                    }, Adt1::C0(v12))
                                }
                            } else {
                                let a1243 = v7;
                                let a1244 = 18u64;
                                if nat_eq(a1243, a1244) {
                                    let v13: Bytes = f14(v0, v5.clone());
                                    if f12({
                                        let a1246 = 1u64;
                                        let a1247 = {
                                            let a1245 = v13.clone();
                                            length_bytes(a1245)
                                        };
                                        nat_lt(a1246, a1247)
                                    }, f180(v13.clone(), f15(Str::lit("0")))) {
                                        Ok(f65(v1, f21(match v5 {
                                            Adt18::C0(_, h1248, _, _) => {
                                                h1248
                                            }
                                        }, f15(Str::lit("a numeral has no redundant leading zero")))))
                                    } else {
                                        f143(v1.clone(), v8, v2, match v1.clone() {
                                            Adt9::C0(_, _, h1249, _, _, _, _, _, _, _) => {
                                                h1249
                                            }
                                        }, match v1.clone() {
                                            Adt9::C0(_, _, _, h1250, _, _, _, _, _, _) => {
                                                h1250
                                            }
                                        }, match v1 {
                                            Adt9::C0(_, _, _, _, _, _, _, h1251, _, _) => {
                                                h1251
                                            }
                                        }, Adt1::C7(v13))
                                    }
                                } else {
                                    let a1252 = v7;
                                    let a1253 = 19u64;
                                    if nat_eq(a1252, a1253) {
                                        f143(v1.clone(), v8, v2, match v1.clone() {
                                            Adt9::C0(_, _, h1254, _, _, _, _, _, _, _) => {
                                                h1254
                                            }
                                        }, match v1.clone() {
                                            Adt9::C0(_, _, _, h1255, _, _, _, _, _, _) => {
                                                h1255
                                            }
                                        }, match v1 {
                                            Adt9::C0(_, _, _, _, _, _, _, h1256, _, _) => {
                                                h1256
                                            }
                                        }, Adt1::C8(f181(f96(v0, match v5 {
                                            Adt18::C0(_, h1257, _, _) => {
                                                h1257
                                            }
                                        })?)?))
                                    } else {
                                        let a1258 = v7;
                                        let a1259 = 0u64;
                                        if nat_eq(a1258, a1259) {
                                            Ok(f52(v1, v8, List::cons(Adt11::C0({
                                                let a1260 = v3;
                                                let a1261 = 1u64;
                                                nat_add(a1260, a1261)?
                                            }), List::cons(Adt11::C16, v2))))
                                        } else {
                                            let a1262 = v7;
                                            let a1263 = 20u64;
                                            if nat_eq(a1262, a1263) {
                                                f182(v0, v1, v2, v3, v5)
                                            } else {
                                                Ok(f65(v1, f21(match v5 {
                                                    Adt18::C0(_, h1264, _, _) => {
                                                        h1264
                                                    }
                                                }, f15(Str::lit("expected a term")))))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v6) => {
                    Ok(f65(v1, v6))
                }
            }
        }
        Err(v4) => {
            Ok(f65(v1, v4))
        }
    }
}

pub fn f148(v0: Bytes, v1: Adt9, v2: List<Adt11>) -> R<Adt9> {
    let m1266 = f16(v0, match v1.clone() {
        Adt9::C0(h1265, _, _, _, _, _, _, _, _, _) => {
            h1265
        }
    }, 1u64, f15(Str::lit("`)`")))?;
    match m1266 {
        Ok(v4) => {
            let v5: u64 = f69(match v1.clone() {
                Adt9::C0(_, _, h1267, _, _, _, _, _, _, _) => {
                    h1267
                }
            });
            let m1269 = match v1.clone() {
                Adt9::C0(_, _, _, h1268, _, _, _, _, _, _) => {
                    h1268
                }
            };
            match m1269.uncons() {
                None => {
                    Ok(v1)
                }
                Some((v6, v7)) => {
                    let m1270 = v6;
                    match m1270 {
                        Adt12::C0(_, _) => {
                            Ok(v1)
                        }
                        Adt12::C1(_, _) => {
                            Ok(v1)
                        }
                        Adt12::C2(_, _, _) => {
                            Ok(v1)
                        }
                        Adt12::C3(_) => {
                            Ok(v1)
                        }
                        Adt12::C4(v8) => {
                            if !f12({
                                let a1274 = {
                                    let a1271 = v5;
                                    let a1272 = 1u64;
                                    nat_add(a1271, a1272)?
                                };
                                let a1275 = match v1.clone() {
                                    Adt9::C0(_, _, _, _, _, h1273, _, _, _, _) => {
                                        h1273
                                    }
                                };
                                nat_eq(a1274, a1275)
                            }, {
                                let a1276 = v8;
                                let a1277 = v5;
                                nat_le(a1276, a1277)
                            }) {
                                Ok(f65(v1, f21({
                                    let m1280 = match v4 {
                                        Adt19::C0(h1279, _) => {
                                            h1279
                                        }
                                    };
                                    match m1280 {
                                        Adt18::C0(_, h1278, _, _) => {
                                            h1278
                                        }
                                    }
                                }, f15(Str::lit("`proof` marks a node written inside it, not a reference")))))
                            } else {
                                Ok(Adt9::C0(match v4 {
                                    Adt19::C0(_, h1281) => {
                                        h1281
                                    }
                                }, v2, match v1.clone() {
                                    Adt9::C0(_, _, h1282, _, _, _, _, _, _, _) => {
                                        h1282
                                    }
                                }, v7, match v1.clone() {
                                    Adt9::C0(_, _, _, _, h1283, _, _, _, _, _) => {
                                        h1283
                                    }
                                }, match v1.clone() {
                                    Adt9::C0(_, _, _, _, _, h1284, _, _, _, _) => {
                                        h1284
                                    }
                                }, List::cons(v5, match v1.clone() {
                                    Adt9::C0(_, _, _, _, _, _, h1285, _, _, _) => {
                                        h1285
                                    }
                                }), match v1.clone() {
                                    Adt9::C0(_, _, _, _, _, _, _, h1286, _, _) => {
                                        h1286
                                    }
                                }, match v1.clone() {
                                    Adt9::C0(_, _, _, _, _, _, _, _, h1287, _) => {
                                        h1287
                                    }
                                }, match v1 {
                                    Adt9::C0(_, _, _, _, _, _, _, _, _, h1288) => {
                                        h1288
                                    }
                                }))
                            }
                        }
                    }
                }
            }
        }
        Err(v3) => {
            Ok(f65(v1, v3))
        }
    }
}

pub fn f149(v0: Adt9, v1: List<Adt11>) -> R<Adt9> {
    let v2: u64 = f69(match v0.clone() {
        Adt9::C0(_, _, h1289, _, _, _, _, _, _, _) => {
            h1289
        }
    });
    let v3: List<u64> = f71(match v0.clone() {
        Adt9::C0(_, _, h1290, _, _, _, _, _, _, _) => {
            h1290
        }
    });
    let m1292 = match v0.clone() {
        Adt9::C0(_, _, _, h1291, _, _, _, _, _, _) => {
            h1291
        }
    };
    match m1292.uncons() {
        None => {
            Ok(v0)
        }
        Some((v4, v5)) => {
            let m1293 = v4;
            match m1293 {
                Adt12::C0(_, _) => {
                    Ok(v0)
                }
                Adt12::C1(_, _) => {
                    Ok(v0)
                }
                Adt12::C2(v6, v7, v8) => {
                    f143(v0.clone(), match v0.clone() {
                        Adt9::C0(h1294, _, _, _, _, _, _, _, _, _) => {
                            h1294
                        }
                    }, v1, v3, v5, match v0 {
                        Adt9::C0(_, _, _, _, _, _, _, h1295, _, _) => {
                            h1295
                        }
                    }, Adt1::C10(v6, v7, v8, v2))
                }
                Adt12::C3(v9) => {
                    f143(v0.clone(), match v0.clone() {
                        Adt9::C0(h1296, _, _, _, _, _, _, _, _, _) => {
                            h1296
                        }
                    }, v1, v3, v5, match v0 {
                        Adt9::C0(_, _, _, _, _, _, _, h1297, _, _) => {
                            h1297
                        }
                    }, Adt1::C9(v9, v2))
                }
                Adt12::C4(_) => {
                    Ok(v0)
                }
            }
        }
    }
}

pub fn f150(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1298, _) = v2;
        h1298
    };
    let a1299 = v3;
    let a1300 = 0u64;
    if nat_eq(a1299, a1300) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1301 = v3;
            let a1302 = v1;
            let a1303 = 0u64;
            nat_quot(a1301, a1302, a1303)
        }, {
            let a1308 = f114(v0, {
                let a1304 = v3;
                let a1305 = v1;
                let a1306 = 0u64;
                nat_rem(a1304, a1305, a1306)
            });
            let a1309 = {
                let (_, h1307) = v2;
                h1307
            };
            append_bytes(a1308, a1309)
        }))
    }
}

pub fn f151(v0: Bytes, v1: u64, v2: Adt25) -> R<Option<Adt25>> {
    let m1311 = match v2.clone() {
        Adt25::C0(_, _, h1310) => {
            h1310
        }
    };
    match m1311 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt25::C0(h1312, _, _) => {
                    h1312
                }
            };
            let v5: u64 = match v2 {
                Adt25::C0(_, h1313, _) => {
                    h1313
                }
            };
            let v6: u64 = f34(v0.clone(), v4)?;
            let a1314 = v6;
            let a1315 = 256u64;
            if nat_eq(a1314, a1315) {
                Ok(f183(v4, v5, Some(Err::<u64, Adt8>(f21(v1, f15(Str::lit("the braces of the declaration are not closed")))))))
            } else {
                let a1316 = v6;
                let a1317 = 34u64;
                if nat_eq(a1316, a1317) {
                    let m1318 = f37(v0, v4)?;
                    match m1318 {
                        Ok(v8) => {
                            Ok(f183(v8, v5, None::<Result<u64, Adt8>>))
                        }
                        Err(v7) => {
                            Ok(f183(v4, v5, Some(Err::<u64, Adt8>(v7))))
                        }
                    }
                } else {
                    let a1319 = v6;
                    let a1320 = 123u64;
                    if nat_eq(a1319, a1320) {
                        Ok(f183({
                            let a1321 = v4;
                            let a1322 = 1u64;
                            nat_add(a1321, a1322)?
                        }, {
                            let a1323 = v5;
                            let a1324 = 1u64;
                            nat_add(a1323, a1324)?
                        }, None::<Result<u64, Adt8>>))
                    } else {
                        let a1325 = v6;
                        let a1326 = 125u64;
                        if nat_eq(a1325, a1326) {
                            let a1327 = v5;
                            let a1328 = 1u64;
                            if nat_eq(a1327, a1328) {
                                Ok(f183({
                                    let a1329 = v4;
                                    let a1330 = 1u64;
                                    nat_add(a1329, a1330)?
                                }, 0u64, Some(Ok::<u64, Adt8>({
                                    let a1331 = v4;
                                    let a1332 = 1u64;
                                    nat_add(a1331, a1332)?
                                }))))
                            } else {
                                Ok(f183({
                                    let a1333 = v4;
                                    let a1334 = 1u64;
                                    nat_add(a1333, a1334)?
                                }, {
                                    let a1335 = v5;
                                    let a1336 = 1u64;
                                    nat_sub(a1335, a1336)
                                }, None::<Result<u64, Adt8>>))
                            }
                        } else {
                            Ok(f183({
                                let a1337 = v4;
                                let a1338 = 1u64;
                                nat_add(a1337, a1338)?
                            }, v5, None::<Result<u64, Adt8>>))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt25>)
        }
    }
}

pub fn f152(v0: Fn23, p1343: u64, p1344: Adt29) -> R<(Adt29, bool)> {
    let m1342 = (p1343, p1344);
    {
        let mut m1342 = m1342;
        loop {
            let step = {
                let (v1, v2) = m1342;
                let m1339 = v1;
                if m1339 == 0 {
                    Err::<(u64, Adt29), R<(Adt29, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1339 - 1;
                    let m1341 = {
                        let c1340: Fn23 = v0.clone();
                        c1340.apply(v2.clone())?
                    };
                    match m1341 {
                        None => {
                            Err::<(u64, Adt29), R<(Adt29, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt29), R<(Adt29, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: u64, v1: Bytes, v2: Adt29) -> R<Option<Adt29>> {
    f184(v1, v0, v2)
}

pub fn f154(v0: List<Adt27>) -> List<Adt27> {
    f185(Fn25::F186, List::<Adt27>::nil(), v0)
}

pub fn f155(v0: List<Adt27>, v1: u64, v2: u64, v3: u64) -> R<Adt30> {
    let (h1345, _) = f187(Fn26::F188(v1, v0.clone()), {
        let a1349 = {
            let a1347 = {
                let a1346 = v0;
                length_list(a1346)
            };
            let a1348 = 4u64;
            nat_mul(a1347, a1348)?
        };
        let a1350 = 8u64;
        nat_add(a1349, a1350)?
    }, Adt30::C0(v2, v3, 0u64, List::<Adt31>::nil(), Adt6::C5, None::<Adt8>))?;
    Ok(h1345)
}

pub fn f156(v0: u64, v1: Bytes) -> Adt8 {
    Adt8::C0(v0, v1)
}

pub fn f157(v0: List<Adt27>, v1: u64, v2: u64) -> u64 {
    let m1351 = f189(v0, v2);
    match m1351 {
        None => {
            v1
        }
        Some(v3) => {
            match v3 {
                Adt27::C0(_, h1352, _) => {
                    h1352
                }
            }
        }
    }
}

pub fn f158(v0: Str) -> Bytes {
    let a1353 = v0;
    utf8_encode(a1353)
}

pub fn f159(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(v1, v0)
}

pub fn f160(v0: Fn24, p1357: List<Adt23>, p1358: List<Adt23>) -> List<Adt23> {
    let m1356 = (p1357, p1358);
    {
        let mut m1356 = m1356;
        loop {
            let step = {
                let (v1, v2) = m1356;
                let m1354 = v2;
                match m1354.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt23>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt23>), List<Adt23>>(({
                            let c1355: Fn24 = v0.clone();
                            c1355.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: List<Adt23>, v1: Adt23) -> List<Adt23> {
    List::cons(v1, v0)
}

pub fn f162(v0: u64) -> Bytes {
    f44(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f163(v0: Bytes, v1: Adt20) -> R<Adt20> {
    let v2: u64 = match v1.clone() {
        Adt20::C0(h1359, _, _, _) => {
            h1359
        }
    };
    let m1362 = f190(v0, {
        let a1360 = v2;
        let a1361 = 2u64;
        nat_add(a1360, a1361)?
    })?;
    match m1362 {
        None => {
            Ok(f133(v1, v2, f134(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a1363 = 55296u64;
            let a1364 = v3;
            if nat_le(a1363, a1364) {
                let a1365 = v3;
                let a1366 = 57343u64;
                if nat_le(a1365, a1366) {
                    Ok(f133(v1, v2, f164(f164(f134(Str::lit("`\\u")), f191(v3)), f134(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f136(v1, {
                        let a1367 = v2;
                        let a1368 = 6u64;
                        nat_add(a1367, a1368)?
                    }, f192(v3)?))
                }
            } else {
                Ok(f136(v1, {
                    let a1369 = v2;
                    let a1370 = 6u64;
                    nat_add(a1369, a1370)?
                }, f192(v3)?))
            }
        }
    }
}

pub fn f164(v0: Bytes, v1: Bytes) -> Bytes {
    let a1371 = v0;
    let a1372 = v1;
    append_bytes(a1371, a1372)
}

pub fn f165(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f132(v0, v1)?;
    let a1373 = v2;
    let a1374 = 128u64;
    if nat_lt(a1373, a1374) {
        Ok(1u64)
    } else {
        let a1375 = v2;
        let a1376 = 224u64;
        if nat_lt(a1375, a1376) {
            Ok(2u64)
        } else {
            let a1377 = v2;
            let a1378 = 240u64;
            if nat_lt(a1377, a1378) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f166(v0: u64, v1: Bytes) -> Adt8 {
    Adt8::C0(v0, v1)
}

pub fn f167(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f193(v1, v0, v2)
}

pub fn f168(v0: u64, v1: u64) -> Option<Adt8> {
    let a1379 = 256u64;
    let a1380 = v0;
    if nat_lt(a1379, a1380) {
        Some(f21(v1, f15(Str::lit("nested more deeply than the notation reads"))))
    } else {
        None::<Adt8>
    }
}

pub fn f169(v0: Bytes, v1: Adt10) -> R<Result<Adt21, Adt8>> {
    let m1382 = f11(v0.clone(), match v1.clone() {
        Adt10::C0(h1381, _) => {
            h1381
        }
    })?;
    match m1382 {
        Ok(v2) => {
            if f12({
                let a1384 = match v2.clone() {
                    Adt18::C0(h1383, _, _, _) => {
                        h1383
                    }
                };
                let a1385 = 20u64;
                nat_eq(a1384, a1385)
            }, f13(f14(v0.clone(), v2.clone()), f15(Str::lit("_")))) {
                Ok(Ok::<Adt21, Adt8>(Adt21::C0(f15(Str::lit("")), f17(v2))))
            } else {
                f74(v0, v1, f15(Str::lit("a binder's name")))
            }
        }
        Err(v3) => {
            Ok(Err::<Adt21, Adt8>(v3))
        }
    }
}

pub fn f170(v0: Bytes, v1: Adt10) -> R<Result<bool, Adt8>> {
    let m1387 = f11(v0.clone(), match v1 {
        Adt10::C0(h1386, _) => {
            h1386
        }
    })?;
    match m1387 {
        Ok(v3) => {
            let a1389 = match v3.clone() {
                Adt18::C0(h1388, _, _, _) => {
                    h1388
                }
            };
            let a1390 = 2u64;
            if if nat_eq(a1389, a1390) {
                true
            } else {
                let a1392 = match v3.clone() {
                    Adt18::C0(h1391, _, _, _) => {
                        h1391
                    }
                };
                let a1393 = 4u64;
                nat_eq(a1392, a1393)
            } {
                Ok(Ok::<bool, Adt8>(true))
            } else {
                let a1395 = match v3.clone() {
                    Adt18::C0(h1394, _, _, _) => {
                        h1394
                    }
                };
                let a1396 = 0u64;
                if nat_eq(a1395, a1396) {
                    let m1398 = f11(v0.clone(), match v3 {
                        Adt18::C0(_, _, h1397, _) => {
                            h1397
                        }
                    })?;
                    match m1398 {
                        Ok(v5) => {
                            let a1400 = match v5.clone() {
                                Adt18::C0(h1399, _, _, _) => {
                                    h1399
                                }
                            };
                            let a1401 = 20u64;
                            if if nat_eq(a1400, a1401) {
                                true
                            } else {
                                let a1403 = match v5.clone() {
                                    Adt18::C0(h1402, _, _, _) => {
                                        h1402
                                    }
                                };
                                let a1404 = 19u64;
                                nat_eq(a1403, a1404)
                            } {
                                let m1406 = f11(v0, match v5 {
                                    Adt18::C0(_, _, h1405, _) => {
                                        h1405
                                    }
                                })?;
                                match m1406 {
                                    Ok(v7) => {
                                        Ok(Ok::<bool, Adt8>({
                                            let a1408 = match v7 {
                                                Adt18::C0(h1407, _, _, _) => {
                                                    h1407
                                                }
                                            };
                                            let a1409 = 7u64;
                                            nat_eq(a1408, a1409)
                                        }))
                                    }
                                    Err(v6) => {
                                        Ok(Err::<bool, Adt8>(v6))
                                    }
                                }
                            } else {
                                Ok(Ok::<bool, Adt8>(false))
                            }
                        }
                        Err(v4) => {
                            Ok(Err::<bool, Adt8>(v4))
                        }
                    }
                } else {
                    Ok(Ok::<bool, Adt8>(false))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<bool, Adt8>(v2))
        }
    }
}

pub fn f171(v0: List<Adt12>) -> u64 {
    let m1410 = v0;
    match m1410.uncons() {
        None => {
            0u64
        }
        Some((v1, _)) => {
            let m1411 = v1;
            match m1411 {
                Adt12::C0(v3, _) => {
                    v3
                }
                Adt12::C1(_, _) => {
                    0u64
                }
                Adt12::C2(_, _, _) => {
                    0u64
                }
                Adt12::C3(_) => {
                    0u64
                }
                Adt12::C4(_) => {
                    0u64
                }
            }
        }
    }
}

pub fn f172(v0: u64) -> u64 {
    let a1412 = v0;
    let a1413 = 0u64;
    if nat_eq(a1412, a1413) {
        1u64
    } else {
        let a1414 = v0;
        let a1415 = 3u64;
        if nat_eq(a1414, a1415) {
            5u64
        } else {
            3u64
        }
    }
}

pub fn f173(v0: List<Adt12>) -> Bytes {
    let m1416 = v0;
    match m1416.uncons() {
        None => {
            f15(Str::lit(""))
        }
        Some((v1, _)) => {
            let m1417 = v1;
            match m1417 {
                Adt12::C0(_, v4) => {
                    v4
                }
                Adt12::C1(v5, _) => {
                    v5
                }
                Adt12::C2(_, _, _) => {
                    f15(Str::lit(""))
                }
                Adt12::C3(_) => {
                    f15(Str::lit(""))
                }
                Adt12::C4(_) => {
                    f15(Str::lit(""))
                }
            }
        }
    }
}

pub fn f174(v0: u64) -> Adt3 {
    let a1418 = v0;
    let a1419 = 0u64;
    if nat_eq(a1418, a1419) {
        Adt3::C0
    } else {
        let a1420 = v0;
        let a1421 = 1u64;
        if nat_eq(a1420, a1421) {
            Adt3::C1
        } else {
            let a1422 = v0;
            let a1423 = 2u64;
            if nat_eq(a1422, a1423) {
                Adt3::C2
            } else {
                Adt3::C3
            }
        }
    }
}

pub fn f175(v0: List<Adt12>) -> List<Adt12> {
    let m1424 = v0.clone();
    match m1424.uncons() {
        None => {
            v0
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f176(v0: Bytes, v1: Adt10, v2: u64) -> R<Result<Adt32, Adt8>> {
    let v3: Adt33 = {
        let (h1425, _) = f194(Fn27::F195(v0.clone()), {
            let a1429 = {
                let a1427 = {
                    let a1426 = v0;
                    length_bytes(a1426)
                };
                let a1428 = 4u64;
                nat_mul(a1427, a1428)?
            };
            let a1430 = 16u64;
            nat_add(a1429, a1430)?
        }, Adt33::C0(v1.clone(), List::cons(Adt34::C0(v2), List::<Adt34>::nil()), List::<Adt2>::nil(), None::<Adt8>))?;
        h1425
    };
    let m1432 = match v3.clone() {
        Adt33::C0(_, _, _, h1431) => {
            h1431
        }
    };
    match m1432 {
        None => {
            let m1434 = match v3.clone() {
                Adt33::C0(_, _, h1433, _) => {
                    h1433
                }
            };
            match m1434.uncons() {
                None => {
                    Ok(Err::<Adt32, Adt8>(f21(match v1 {
                        Adt10::C0(h1436, _) => {
                            h1436
                        }
                    }, f15(Str::lit("expected a universe level")))))
                }
                Some((v5, _)) => {
                    Ok(Ok::<Adt32, Adt8>(Adt32::C0(v5, match v3 {
                        Adt33::C0(h1435, _, _, _) => {
                            h1435
                        }
                    })))
                }
            }
        }
        Some(v4) => {
            Ok(Err::<Adt32, Adt8>(v4))
        }
    }
}

pub fn f177(v0: Bytes, v1: Adt10, v2: Bytes) -> R<Result<Adt35, Adt8>> {
    let m1437 = f16(v0.clone(), v1, 18u64, v2)?;
    match m1437 {
        Ok(v3) => {
            let v4: Adt18 = match v3.clone() {
                Adt19::C0(h1438, _) => {
                    h1438
                }
            };
            let v5: Bytes = f14(v0, v4.clone());
            if f12({
                let a1440 = 1u64;
                let a1441 = {
                    let a1439 = v5.clone();
                    length_bytes(a1439)
                };
                nat_lt(a1440, a1441)
            }, f180(v5.clone(), f15(Str::lit("0")))) {
                Ok(Err::<Adt35, Adt8>(f21(match v4 {
                    Adt18::C0(_, h1442, _, _) => {
                        h1442
                    }
                }, f15(Str::lit("a numeral has no redundant leading zero")))))
            } else {
                let v6: u64 = if f87(v5.clone())? {
                    0u64
                } else {
                    f88(v5.clone())?
                };
                if f87(v5)? {
                    Ok(Err::<Adt35, Adt8>(f21(match v4 {
                        Adt18::C0(_, h1443, _, _) => {
                            h1443
                        }
                    }, f15(Str::lit("the number is too large")))))
                } else {
                    Ok(Ok::<Adt35, Adt8>(Adt35::C0(v6, match v3 {
                        Adt19::C0(_, h1444) => {
                            h1444
                        }
                    })))
                }
            }
        }
        Err(v7) => {
            Ok(Err::<Adt35, Adt8>(v7))
        }
    }
}

pub fn f178(v0: Bytes, v1: Adt10) -> R<Result<(List<Adt4>, Adt10), Adt8>> {
    let m1445 = f16(v0.clone(), v1, 4u64, f15(Str::lit("`[` after `annot`")))?;
    match m1445 {
        Ok(v3) => {
            let v4: Adt36 = {
                let (h1446, _) = f196(Fn28::F197(v0.clone()), {
                    let a1448 = {
                        let a1447 = v0.clone();
                        length_bytes(a1447)
                    };
                    let a1449 = 2u64;
                    nat_add(a1448, a1449)?
                }, Adt36::C0(List::<Adt4>::nil(), match v3 {
                    Adt19::C0(_, h1450) => {
                        h1450
                    }
                }, false, None::<Adt8>))?;
                h1446
            };
            let m1452 = match v4.clone() {
                Adt36::C0(_, _, _, h1451) => {
                    h1451
                }
            };
            match m1452 {
                None => {
                    let m1454 = f16(v0, match v4.clone() {
                        Adt36::C0(_, h1453, _, _) => {
                            h1453
                        }
                    }, 5u64, f15(Str::lit("`,` or `]`")))?;
                    match m1454 {
                        Ok(v7) => {
                            Ok(Ok::<(List<Adt4>, Adt10), Adt8>((f198(match v4 {
                                Adt36::C0(h1455, _, _, _) => {
                                    h1455
                                }
                            }), match v7 {
                                Adt19::C0(_, h1456) => {
                                    h1456
                                }
                            })))
                        }
                        Err(v6) => {
                            Ok(Err::<(List<Adt4>, Adt10), Adt8>(v6))
                        }
                    }
                }
                Some(v5) => {
                    Ok(Err::<(List<Adt4>, Adt10), Adt8>(v5))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(List<Adt4>, Adt10), Adt8>(v2))
        }
    }
}

pub fn f179(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64) -> Adt9 {
    f70(v0.clone(), v1, v2, List::cons(v3, match v0.clone() {
        Adt9::C0(_, _, h1457, _, _, _, _, _, _, _) => {
            h1457
        }
    }), match v0.clone() {
        Adt9::C0(_, _, _, h1458, _, _, _, _, _, _) => {
            h1458
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, h1459, _, _) => {
            h1459
        }
    })
}

pub fn f180(v0: Bytes, v1: Bytes) -> bool {
    let m1464 = {
        let a1461 = v0;
        let a1462 = 0u64;
        let a1463 = {
            let a1460 = v1.clone();
            length_bytes(a1460)
        };
        slice_bytes(a1461, a1462, a1463)
    };
    match m1464 {
        None => {
            false
        }
        Some(v2) => {
            let a1465 = v2;
            let a1466 = v1;
            equal(a1465, a1466)
        }
    }
}

pub fn f181(v0: Bytes) -> R<List<u64>> {
    Ok(f7({
        let m1472 = {
            let (h1468, _) = f199(Fn29::F200(v0.clone()), {
                let a1470 = {
                    let a1469 = v0;
                    length_bytes(a1469)
                };
                let a1471 = 1u64;
                nat_add(a1470, a1471)?
            }, Adt37::C0(0u64, List::<u64>::nil()))?;
            h1468
        };
        match m1472 {
            Adt37::C0(_, h1467) => {
                h1467
            }
        }
    }))
}

pub fn f182(v0: Bytes, v1: Adt9, v2: List<Adt11>, v3: u64, v4: Adt18) -> R<Adt9> {
    let v5: Bytes = f14(v0.clone(), v4.clone());
    let v6: Adt10 = f17(v4.clone());
    if f13(v5.clone(), f15(Str::lit("proof"))) {
        let m1473 = f16(v0, v6, 0u64, f15(Str::lit("`(` after `proof`")))?;
        match m1473 {
            Ok(v8) => {
                Ok(f70(v1.clone(), match v8 {
                    Adt19::C0(_, h1474) => {
                        h1474
                    }
                }, List::cons(Adt11::C0({
                    let a1475 = v3;
                    let a1476 = 1u64;
                    nat_add(a1475, a1476)?
                }), List::cons(Adt11::C17, v2)), match v1.clone() {
                    Adt9::C0(_, _, h1477, _, _, _, _, _, _, _) => {
                        h1477
                    }
                }, List::cons(Adt12::C4(match v1.clone() {
                    Adt9::C0(_, _, _, _, _, h1478, _, _, _, _) => {
                        h1478
                    }
                }), match v1.clone() {
                    Adt9::C0(_, _, _, h1479, _, _, _, _, _, _) => {
                        h1479
                    }
                }), match v1 {
                    Adt9::C0(_, _, _, _, _, _, _, h1480, _, _) => {
                        h1480
                    }
                }))
            }
            Err(v7) => {
                Ok(f65(v1, v7))
            }
        }
    } else {
        if f13(v5.clone(), f15(Str::lit("nat"))) {
            let m1481 = f16(v0.clone(), v6, 19u64, f15(Str::lit("a quoted number after `nat`")))?;
            match m1481 {
                Ok(v10) => {
                    f143(v1.clone(), match v10.clone() {
                        Adt19::C0(_, h1482) => {
                            h1482
                        }
                    }, v2, match v1.clone() {
                        Adt9::C0(_, _, h1483, _, _, _, _, _, _, _) => {
                            h1483
                        }
                    }, match v1.clone() {
                        Adt9::C0(_, _, _, h1484, _, _, _, _, _, _) => {
                            h1484
                        }
                    }, match v1 {
                        Adt9::C0(_, _, _, _, _, _, _, h1485, _, _) => {
                            h1485
                        }
                    }, Adt1::C7(f96(v0, {
                        let m1488 = match v10 {
                            Adt19::C0(h1487, _) => {
                                h1487
                            }
                        };
                        match m1488 {
                            Adt18::C0(_, h1486, _, _) => {
                                h1486
                            }
                        }
                    })?))
                }
                Err(v9) => {
                    Ok(f65(v1, v9))
                }
            }
        } else {
            if f13(v5.clone(), f15(Str::lit("const"))) {
                let m1489 = f16(v0.clone(), v6, 19u64, f15(Str::lit("a quoted name after `const`")))?;
                match m1489 {
                    Ok(v12) => {
                        let m1491 = f201(v0.clone(), match v12.clone() {
                            Adt19::C0(_, h1490) => {
                                h1490
                            }
                        }, v3)?;
                        match m1491 {
                            Ok(v14) => {
                                f143(v1.clone(), match v14.clone() {
                                    Adt38::C0(_, h1492) => {
                                        h1492
                                    }
                                }, v2, match v1.clone() {
                                    Adt9::C0(_, _, h1493, _, _, _, _, _, _, _) => {
                                        h1493
                                    }
                                }, match v1.clone() {
                                    Adt9::C0(_, _, _, h1494, _, _, _, _, _, _) => {
                                        h1494
                                    }
                                }, match v1 {
                                    Adt9::C0(_, _, _, _, _, _, _, h1495, _, _) => {
                                        h1495
                                    }
                                }, Adt1::C2(f96(v0, {
                                    let m1498 = match v12 {
                                        Adt19::C0(h1497, _) => {
                                            h1497
                                        }
                                    };
                                    match m1498 {
                                        Adt18::C0(_, h1496, _, _) => {
                                            h1496
                                        }
                                    }
                                })?, match v14 {
                                    Adt38::C0(h1499, _) => {
                                        h1499
                                    }
                                }))
                            }
                            Err(v13) => {
                                Ok(f65(v1, v13))
                            }
                        }
                    }
                    Err(v11) => {
                        Ok(f65(v1, v11))
                    }
                }
            } else {
                if f63(v5.clone(), f95()) {
                    Ok(f65(v1, f21(match v4 {
                        Adt18::C0(_, h1500, _, _) => {
                            h1500
                        }
                    }, f43(f43(f15(Str::lit("`")), v5), f15(Str::lit("` is a keyword; a term that begins with it is written in parentheses"))))))
                } else {
                    if f13(v5.clone(), f15(Str::lit("_"))) {
                        Ok(f65(v1, f21(match v4 {
                            Adt18::C0(_, h1501, _, _) => {
                                h1501
                            }
                        }, f15(Str::lit("`_` is the name of a binder, not a constant")))))
                    } else {
                        f202(v0, v1, v2, v3, v4, v5)
                    }
                }
            }
        }
    }
}

pub fn f183(v0: u64, v1: u64, v2: Option<Result<u64, Adt8>>) -> Option<Adt25> {
    Some(Adt25::C0(v0, v1, v2))
}

pub fn f184(v0: Bytes, v1: u64, v2: Adt29) -> R<Option<Adt29>> {
    let m1503 = match v2.clone() {
        Adt29::C0(_, _, h1502) => {
            h1502
        }
    };
    match m1503 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt29::C0(h1504, _, _) => {
                    h1504
                }
            };
            let a1505 = v4;
            let a1506 = v1;
            if nat_lt(a1505, a1506) {
                let v5: u64 = f60(v0.clone(), v4)?;
                let a1507 = 0u64;
                let a1508 = v5;
                if nat_lt(a1507, a1508) {
                    Ok(Some(Adt29::C0({
                        let a1509 = v4;
                        let a1510 = v5;
                        nat_add(a1509, a1510)?
                    }, match v2 {
                        Adt29::C0(_, h1511, _) => {
                            h1511
                        }
                    }, None::<Adt8>)))
                } else {
                    if f130(v0.clone(), v4, 123u8) {
                        Ok(f203(v2, Adt28::C0, {
                            let a1512 = v4;
                            let a1513 = 1u64;
                            nat_add(a1512, a1513)?
                        }))
                    } else {
                        if f130(v0.clone(), v4, 125u8) {
                            Ok(f203(v2, Adt28::C1, {
                                let a1514 = v4;
                                let a1515 = 1u64;
                                nat_add(a1514, a1515)?
                            }))
                        } else {
                            if f130(v0.clone(), v4, 91u8) {
                                Ok(f203(v2, Adt28::C2, {
                                    let a1516 = v4;
                                    let a1517 = 1u64;
                                    nat_add(a1516, a1517)?
                                }))
                            } else {
                                if f130(v0.clone(), v4, 93u8) {
                                    Ok(f203(v2, Adt28::C3, {
                                        let a1518 = v4;
                                        let a1519 = 1u64;
                                        nat_add(a1518, a1519)?
                                    }))
                                } else {
                                    if f130(v0.clone(), v4, 40u8) {
                                        Ok(f203(v2, Adt28::C4, {
                                            let a1520 = v4;
                                            let a1521 = 1u64;
                                            nat_add(a1520, a1521)?
                                        }))
                                    } else {
                                        if f130(v0.clone(), v4, 41u8) {
                                            Ok(f203(v2, Adt28::C5, {
                                                let a1522 = v4;
                                                let a1523 = 1u64;
                                                nat_add(a1522, a1523)?
                                            }))
                                        } else {
                                            if f130(v0.clone(), v4, 44u8) {
                                                Ok(f203(v2, Adt28::C6, {
                                                    let a1524 = v4;
                                                    let a1525 = 1u64;
                                                    nat_add(a1524, a1525)?
                                                }))
                                            } else {
                                                if f130(v0.clone(), v4, 59u8) {
                                                    Ok(f203(v2, Adt28::C10, {
                                                        let a1526 = v4;
                                                        let a1527 = 1u64;
                                                        nat_add(a1526, a1527)?
                                                    }))
                                                } else {
                                                    if f130(v0.clone(), v4, 58u8) {
                                                        f204(v0, v2)
                                                    } else {
                                                        if f130(v0.clone(), v4, 34u8) {
                                                            f205(v0, v1, v2)
                                                        } else {
                                                            f206(v0, v1, v2)
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
            } else {
                Ok(None::<Adt29>)
            }
        }
        Some(_) => {
            Ok(None::<Adt29>)
        }
    }
}

pub fn f185(v0: Fn25, p1531: List<Adt27>, p1532: List<Adt27>) -> List<Adt27> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v1, v2) = m1530;
                let m1528 = v2;
                match m1528.uncons() {
                    None => {
                        Err::<(List<Adt27>, List<Adt27>), List<Adt27>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt27>, List<Adt27>), List<Adt27>>(({
                            let c1529: Fn25 = v0.clone();
                            c1529.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: List<Adt27>, v1: Adt27) -> List<Adt27> {
    List::cons(v1, v0)
}

pub fn f187(v0: Fn26, p1537: u64, p1538: Adt30) -> R<(Adt30, bool)> {
    let m1536 = (p1537, p1538);
    {
        let mut m1536 = m1536;
        loop {
            let step = {
                let (v1, v2) = m1536;
                let m1533 = v1;
                if m1533 == 0 {
                    Err::<(u64, Adt30), R<(Adt30, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1533 - 1;
                    let m1535 = {
                        let c1534: Fn26 = v0.clone();
                        c1534.apply(v2.clone())?
                    };
                    match m1535 {
                        None => {
                            Err::<(u64, Adt30), R<(Adt30, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt30), R<(Adt30, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: u64, v1: List<Adt27>, v2: Adt30) -> R<Option<Adt30>> {
    f207(v1, v0, v2)
}

pub fn f189(v0: List<Adt27>, v1: u64) -> Option<Adt27> {
    let a1539 = v0;
    let a1540 = v1;
    index_list(a1539, a1540)
}

pub fn f190(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1541 = f208(v0.clone(), v1)?;
    match m1541 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1544 = f208(v0.clone(), {
                let a1542 = v1;
                let a1543 = 1u64;
                nat_add(a1542, a1543)?
            })?;
            match m1544 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m1547 = f208(v0.clone(), {
                        let a1545 = v1;
                        let a1546 = 2u64;
                        nat_add(a1545, a1546)?
                    })?;
                    match m1547 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m1550 = f208(v0, {
                                let a1548 = v1;
                                let a1549 = 3u64;
                                nat_add(a1548, a1549)?
                            })?;
                            match m1550 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a1561 = {
                                            let a1559 = {
                                                let a1557 = {
                                                    let a1555 = {
                                                        let a1553 = {
                                                            let a1551 = v2;
                                                            let a1552 = 16u64;
                                                            nat_mul(a1551, a1552)?
                                                        };
                                                        let a1554 = v3;
                                                        nat_add(a1553, a1554)?
                                                    };
                                                    let a1556 = 16u64;
                                                    nat_mul(a1555, a1556)?
                                                };
                                                let a1558 = v4;
                                                nat_add(a1557, a1558)?
                                            };
                                            let a1560 = 16u64;
                                            nat_mul(a1559, a1560)?
                                        };
                                        let a1562 = v5;
                                        nat_add(a1561, a1562)?
                                    }))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f191(v0: u64) -> Bytes {
    f164(f164(f209(v0, 4096u64), f209(v0, 256u64)), f164(f209(v0, 16u64), f210(v0)))
}

pub fn f192(v0: u64) -> R<Bytes> {
    let a1563 = v0;
    let a1564 = 128u64;
    if nat_lt(a1563, a1564) {
        Ok(f162(v0))
    } else {
        let a1565 = v0;
        let a1566 = 2048u64;
        if nat_lt(a1565, a1566) {
            Ok(f164(f162({
                let a1570 = 192u64;
                let a1571 = {
                    let a1567 = v0;
                    let a1568 = 64u64;
                    let a1569 = 0u64;
                    nat_quot(a1567, a1568, a1569)
                };
                nat_add(a1570, a1571)?
            }), f162({
                let a1575 = 128u64;
                let a1576 = {
                    let a1572 = v0;
                    let a1573 = 64u64;
                    let a1574 = 0u64;
                    nat_rem(a1572, a1573, a1574)
                };
                nat_add(a1575, a1576)?
            })))
        } else {
            Ok(f164(f162({
                let a1580 = 224u64;
                let a1581 = {
                    let a1577 = v0;
                    let a1578 = 4096u64;
                    let a1579 = 0u64;
                    nat_quot(a1577, a1578, a1579)
                };
                nat_add(a1580, a1581)?
            }), f164(f162({
                let a1588 = 128u64;
                let a1589 = {
                    let a1585 = {
                        let a1582 = v0;
                        let a1583 = 64u64;
                        let a1584 = 0u64;
                        nat_quot(a1582, a1583, a1584)
                    };
                    let a1586 = 64u64;
                    let a1587 = 0u64;
                    nat_rem(a1585, a1586, a1587)
                };
                nat_add(a1588, a1589)?
            }), f162({
                let a1593 = 128u64;
                let a1594 = {
                    let a1590 = v0;
                    let a1591 = 64u64;
                    let a1592 = 0u64;
                    nat_rem(a1590, a1591, a1592)
                };
                nat_add(a1593, a1594)?
            }))))
        }
    }
}

pub fn f193(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1595 = v2;
    let a1596 = v1;
    if nat_lt(a1595, a1596) {
        let v3: u64 = f132(v0, v2)?;
        let a1597 = v3;
        let a1598 = 32u64;
        if nat_lt(a1597, a1598) {
            Ok(None::<u64>)
        } else {
            let a1599 = v3;
            let a1600 = 34u64;
            if nat_eq(a1599, a1600) {
                Ok(None::<u64>)
            } else {
                let a1601 = v3;
                let a1602 = 92u64;
                if nat_eq(a1601, a1602) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1603 = v2;
                        let a1604 = 1u64;
                        nat_add(a1603, a1604)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f194(v0: Fn27, p1609: u64, p1610: Adt33) -> R<(Adt33, bool)> {
    let m1608 = (p1609, p1610);
    {
        let mut m1608 = m1608;
        loop {
            let step = {
                let (v1, v2) = m1608;
                let m1605 = v1;
                if m1605 == 0 {
                    Err::<(u64, Adt33), R<(Adt33, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1605 - 1;
                    let m1607 = {
                        let c1606: Fn27 = v0.clone();
                        c1606.apply(v2.clone())?
                    };
                    match m1607 {
                        None => {
                            Err::<(u64, Adt33), R<(Adt33, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt33), R<(Adt33, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Bytes, v1: Adt33) -> R<Option<Adt33>> {
    f211(v0, v1)
}

pub fn f196(v0: Fn28, p1615: u64, p1616: Adt36) -> R<(Adt36, bool)> {
    let m1614 = (p1615, p1616);
    {
        let mut m1614 = m1614;
        loop {
            let step = {
                let (v1, v2) = m1614;
                let m1611 = v1;
                if m1611 == 0 {
                    Err::<(u64, Adt36), R<(Adt36, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1611 - 1;
                    let m1613 = {
                        let c1612: Fn28 = v0.clone();
                        c1612.apply(v2.clone())?
                    };
                    match m1613 {
                        None => {
                            Err::<(u64, Adt36), R<(Adt36, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt36), R<(Adt36, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Bytes, v1: Adt36) -> R<Option<Adt36>> {
    f212(v0, v1)
}

pub fn f198(v0: List<Adt4>) -> List<Adt4> {
    f213(Fn30::F214, List::<Adt4>::nil(), v0)
}

pub fn f199(v0: Fn29, p1621: u64, p1622: Adt37) -> R<(Adt37, bool)> {
    let m1620 = (p1621, p1622);
    {
        let mut m1620 = m1620;
        loop {
            let step = {
                let (v1, v2) = m1620;
                let m1617 = v1;
                if m1617 == 0 {
                    Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1617 - 1;
                    let m1619 = {
                        let c1618: Fn29 = v0.clone();
                        c1618.apply(v2.clone())?
                    };
                    match m1619 {
                        None => {
                            Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt37), R<(Adt37, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Bytes, v1: Adt37) -> R<Option<Adt37>> {
    f215(v0, v1)
}

pub fn f201(v0: Bytes, v1: Adt10, v2: u64) -> R<Result<Adt38, Adt8>> {
    let m1624 = f11(v0.clone(), match v1.clone() {
        Adt10::C0(h1623, _) => {
            h1623
        }
    })?;
    match m1624 {
        Ok(v4) => {
            let a1626 = match v4.clone() {
                Adt18::C0(h1625, _, _, _) => {
                    h1625
                }
            };
            let a1627 = 14u64;
            if nat_eq(a1626, a1627) {
                let v5: Adt39 = {
                    let (h1628, _) = f216(Fn31::F217(v2, v0.clone()), {
                        let a1630 = {
                            let a1629 = v0.clone();
                            length_bytes(a1629)
                        };
                        let a1631 = 2u64;
                        nat_add(a1630, a1631)?
                    }, Adt39::C0(List::<Adt2>::nil(), f17(v4), false, None::<Adt8>))?;
                    h1628
                };
                let m1633 = match v5.clone() {
                    Adt39::C0(_, _, _, h1632) => {
                        h1632
                    }
                };
                match m1633 {
                    None => {
                        let m1635 = f16(v0, match v5.clone() {
                            Adt39::C0(_, h1634, _, _) => {
                                h1634
                            }
                        }, 3u64, f15(Str::lit("`}` after the levels")))?;
                        match m1635 {
                            Ok(v7) => {
                                Ok(Ok::<Adt38, Adt8>(Adt38::C0(f218(match v5 {
                                    Adt39::C0(h1636, _, _, _) => {
                                        h1636
                                    }
                                }), match v7 {
                                    Adt19::C0(_, h1637) => {
                                        h1637
                                    }
                                })))
                            }
                            Err(v8) => {
                                Ok(Err::<Adt38, Adt8>(v8))
                            }
                        }
                    }
                    Some(v6) => {
                        Ok(Err::<Adt38, Adt8>(v6))
                    }
                }
            } else {
                Ok(Ok::<Adt38, Adt8>(Adt38::C0(List::<Adt2>::nil(), v1)))
            }
        }
        Err(v3) => {
            Ok(Err::<Adt38, Adt8>(v3))
        }
    }
}

pub fn f202(v0: Bytes, v1: Adt9, v2: List<Adt11>, v3: u64, v4: Adt18, v5: Bytes) -> R<Adt9> {
    let v6: Adt10 = f17(v4.clone());
    let m1639 = f219(match v1.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1638, _, _) => {
            h1638
        }
    }, v5.clone())?;
    match m1639 {
        None => {
            let m1649 = f201(v0, v6, v3)?;
            match m1649 {
                Ok(v11) => {
                    f143(v1.clone(), match v11.clone() {
                        Adt38::C0(_, h1650) => {
                            h1650
                        }
                    }, v2, match v1.clone() {
                        Adt9::C0(_, _, h1651, _, _, _, _, _, _, _) => {
                            h1651
                        }
                    }, match v1.clone() {
                        Adt9::C0(_, _, _, h1652, _, _, _, _, _, _) => {
                            h1652
                        }
                    }, match v1 {
                        Adt9::C0(_, _, _, _, _, _, _, h1653, _, _) => {
                            h1653
                        }
                    }, Adt1::C2(v5, match v11 {
                        Adt38::C0(h1654, _) => {
                            h1654
                        }
                    }))
                }
                Err(v10) => {
                    Ok(f65(v1, v10))
                }
            }
        }
        Some(v7) => {
            let m1641 = f11(v0, match v6.clone() {
                Adt10::C0(h1640, _) => {
                    h1640
                }
            })?;
            match m1641 {
                Ok(v9) => {
                    let a1643 = match v9 {
                        Adt18::C0(h1642, _, _, _) => {
                            h1642
                        }
                    };
                    let a1644 = 14u64;
                    if nat_eq(a1643, a1644) {
                        Ok(f65(v1, f21(match v4 {
                            Adt18::C0(_, _, h1645, _) => {
                                h1645
                            }
                        }, f43(f43(f15(Str::lit("`")), v5), f15(Str::lit("` is a bound variable and has no universe levels"))))))
                    } else {
                        f143(v1.clone(), v6, v2, match v1.clone() {
                            Adt9::C0(_, _, h1646, _, _, _, _, _, _, _) => {
                                h1646
                            }
                        }, match v1.clone() {
                            Adt9::C0(_, _, _, h1647, _, _, _, _, _, _) => {
                                h1647
                            }
                        }, match v1 {
                            Adt9::C0(_, _, _, _, _, _, _, h1648, _, _) => {
                                h1648
                            }
                        }, Adt1::C0(v7))
                    }
                }
                Err(v8) => {
                    Ok(f65(v1, v8))
                }
            }
        }
    }
}

pub fn f203(v0: Adt29, v1: Adt28, v2: u64) -> Option<Adt29> {
    Some(Adt29::C0(v2, List::cons(Adt27::C0(v1, match v0.clone() {
        Adt29::C0(h1655, _, _) => {
            h1655
        }
    }, v2), match v0 {
        Adt29::C0(_, h1656, _) => {
            h1656
        }
    }), None::<Adt8>))
}

pub fn f204(v0: Bytes, v1: Adt29) -> R<Option<Adt29>> {
    let v2: u64 = match v1.clone() {
        Adt29::C0(h1657, _, _) => {
            h1657
        }
    };
    if f130(v0.clone(), {
        let a1658 = v2;
        let a1659 = 1u64;
        nat_add(a1658, a1659)?
    }, 61u8) {
        Ok(f203(v1, Adt28::C8, {
            let a1660 = v2;
            let a1661 = 2u64;
            nat_add(a1660, a1661)?
        }))
    } else {
        if f130(v0, {
            let a1662 = v2;
            let a1663 = 1u64;
            nat_add(a1662, a1663)?
        }, 58u8) {
            Ok(f203(v1, Adt28::C9, {
                let a1664 = v2;
                let a1665 = 2u64;
                nat_add(a1664, a1665)?
            }))
        } else {
            Ok(f203(v1, Adt28::C7, {
                let a1666 = v2;
                let a1667 = 1u64;
                nat_add(a1666, a1667)?
            }))
        }
    }
}

pub fn f205(v0: Bytes, v1: u64, v2: Adt29) -> R<Option<Adt29>> {
    let v3: u64 = match v2.clone() {
        Adt29::C0(h1668, _, _) => {
            h1668
        }
    };
    let v4: Adt20 = f58(v0, v1, v3)?;
    let m1670 = match v4.clone() {
        Adt20::C0(_, _, _, h1669) => {
            h1669
        }
    };
    match m1670 {
        None => {
            if match v4.clone() {
                Adt20::C0(_, _, h1672, _) => {
                    h1672
                }
            } {
                Ok(f203(v2, Adt28::C11(f220(match v4.clone() {
                    Adt20::C0(_, h1673, _, _) => {
                        h1673
                    }
                })), match v4 {
                    Adt20::C0(h1674, _, _, _) => {
                        h1674
                    }
                }))
            } else {
                Ok(Some(Adt29::C0(v3, match v2 {
                    Adt29::C0(_, h1675, _) => {
                        h1675
                    }
                }, Some(f166(v3, f134(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt29::C0(v3, match v2 {
                Adt29::C0(_, h1671, _) => {
                    h1671
                }
            }, Some(v5))))
        }
    }
}

pub fn f206(v0: Bytes, v1: u64, v2: Adt29) -> R<Option<Adt29>> {
    let v3: u64 = match v2.clone() {
        Adt29::C0(h1676, _, _) => {
            h1676
        }
    };
    let v4: u64 = f221(v0.clone(), v1, v3)?;
    Ok(f203(v2, Adt28::C12(f44(v0, v3, {
        let a1677 = v4;
        let a1678 = v3;
        nat_sub(a1677, a1678)
    })), v4))
}

pub fn f207(v0: List<Adt27>, v1: u64, v2: Adt30) -> R<Option<Adt30>> {
    let m1680 = match v2.clone() {
        Adt30::C0(_, _, _, _, _, h1679) => {
            h1679
        }
    };
    match m1680 {
        None => {
            let a1682 = match v2.clone() {
                Adt30::C0(_, _, h1681, _, _, _) => {
                    h1681
                }
            };
            let a1683 = 9u64;
            if nat_eq(a1682, a1683) {
                Ok(None::<Adt30>)
            } else {
                Ok(Some(f222(v0, v1, v2)?))
            }
        }
        Some(_) => {
            Ok(None::<Adt30>)
        }
    }
}

pub fn f208(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1684 = f57(v0, v1);
    match m1684 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f223(v2)
        }
    }
}

pub fn f209(v0: u64, v1: u64) -> Bytes {
    f210({
        let a1685 = v0;
        let a1686 = v1;
        let a1687 = 0u64;
        nat_quot(a1685, a1686, a1687)
    })
}

pub fn f210(v0: u64) -> Bytes {
    f44(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1688 = v0;
        let a1689 = 16u64;
        let a1690 = 0u64;
        nat_rem(a1688, a1689, a1690)
    }, 1u64)
}

pub fn f211(v0: Bytes, v1: Adt33) -> R<Option<Adt33>> {
    let m1692 = match v1.clone() {
        Adt33::C0(_, _, _, h1691) => {
            h1691
        }
    };
    match m1692 {
        None => {
            let m1694 = match v1.clone() {
                Adt33::C0(_, h1693, _, _) => {
                    h1693
                }
            };
            match m1694.uncons() {
                None => {
                    Ok(None::<Adt33>)
                }
                Some((v3, v4)) => {
                    let v5: Adt10 = match v1.clone() {
                        Adt33::C0(h1695, _, _, _) => {
                            h1695
                        }
                    };
                    Ok(Some({
                        let m1696 = v3;
                        match m1696 {
                            Adt34::C0(v6) => {
                                f224(v1.clone(), v5, List::cons(Adt34::C1(v6), List::cons(Adt34::C2, v4)), match v1 {
                                    Adt33::C0(_, _, h1697, _) => {
                                        h1697
                                    }
                                })
                            }
                            Adt34::C1(v7) => {
                                f225(v0, v1, v4, v7)?
                            }
                            Adt34::C2 => {
                                let m1710 = f11(v0.clone(), match v5.clone() {
                                    Adt10::C0(h1709, _) => {
                                        h1709
                                    }
                                })?;
                                match m1710 {
                                    Ok(v18) => {
                                        let a1712 = match v18.clone() {
                                            Adt18::C0(h1711, _, _, _) => {
                                                h1711
                                            }
                                        };
                                        let a1713 = 12u64;
                                        if nat_eq(a1712, a1713) {
                                            let v19: Adt10 = f17(v18);
                                            let m1715 = f11(v0.clone(), match v19.clone() {
                                                Adt10::C0(h1714, _) => {
                                                    h1714
                                                }
                                            })?;
                                            match m1715 {
                                                Ok(v21) => {
                                                    let m1716 = f177(v0, v19, f15(Str::lit("a number after `+`")))?;
                                                    match m1716 {
                                                        Ok(v22) => {
                                                            let a1718 = 1024u64;
                                                            let a1719 = match v22.clone() {
                                                                Adt35::C0(h1717, _) => {
                                                                    h1717
                                                                }
                                                            };
                                                            if nat_lt(a1718, a1719) {
                                                                f226(v1, f21(match v21 {
                                                                    Adt18::C0(_, h1720, _, _) => {
                                                                        h1720
                                                                    }
                                                                }, f15(Str::lit("a level is raised by at most 1024 at once"))))
                                                            } else {
                                                                f224(v1.clone(), match v22.clone() {
                                                                    Adt35::C0(_, h1721) => {
                                                                        h1721
                                                                    }
                                                                }, List::cons(Adt34::C2, v4), f228(match v1 {
                                                                    Adt33::C0(_, _, h1722, _) => {
                                                                        h1722
                                                                    }
                                                                }, match v22 {
                                                                    Adt35::C0(h1723, _) => {
                                                                        h1723
                                                                    }
                                                                }))
                                                            }
                                                        }
                                                        Err(v23) => {
                                                            f226(v1, v23)
                                                        }
                                                    }
                                                }
                                                Err(v20) => {
                                                    f226(v1, v20)
                                                }
                                            }
                                        } else {
                                            f224(v1.clone(), v5, v4, match v1 {
                                                Adt33::C0(_, _, h1724, _) => {
                                                    h1724
                                                }
                                            })
                                        }
                                    }
                                    Err(v17) => {
                                        f226(v1, v17)
                                    }
                                }
                            }
                            Adt34::C3 => {
                                let m1698 = f16(v0, v5, 1u64, f15(Str::lit("`)`")))?;
                                match m1698 {
                                    Ok(v8) => {
                                        f224(v1.clone(), match v8 {
                                            Adt19::C0(_, h1699) => {
                                                h1699
                                            }
                                        }, v4, match v1 {
                                            Adt33::C0(_, _, h1700, _) => {
                                                h1700
                                            }
                                        })
                                    }
                                    Err(v9) => {
                                        f226(v1, v9)
                                    }
                                }
                            }
                            Adt34::C4(v10, v11) => {
                                let m1701 = f16(v0, v5, 6u64, f15(Str::lit("`,`")))?;
                                match m1701 {
                                    Ok(v12) => {
                                        f224(v1.clone(), match v12 {
                                            Adt19::C0(_, h1702) => {
                                                h1702
                                            }
                                        }, List::cons(Adt34::C0({
                                            let a1703 = v10;
                                            let a1704 = 1u64;
                                            nat_add(a1703, a1704)?
                                        }), List::cons(Adt34::C5(v11), v4)), match v1 {
                                            Adt33::C0(_, _, h1705, _) => {
                                                h1705
                                            }
                                        })
                                    }
                                    Err(v13) => {
                                        f226(v1, v13)
                                    }
                                }
                            }
                            Adt34::C5(v14) => {
                                let m1706 = f16(v0, v5, 1u64, f15(Str::lit("`)`")))?;
                                match m1706 {
                                    Ok(v15) => {
                                        f224(v1.clone(), match v15 {
                                            Adt19::C0(_, h1707) => {
                                                h1707
                                            }
                                        }, v4, f227(match v1 {
                                            Adt33::C0(_, _, h1708, _) => {
                                                h1708
                                            }
                                        }, v14))
                                    }
                                    Err(v16) => {
                                        f226(v1, v16)
                                    }
                                }
                            }
                        }
                    }))
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt33>)
        }
    }
}

pub fn f212(v0: Bytes, v1: Adt36) -> R<Option<Adt36>> {
    if match v1.clone() {
        Adt36::C0(_, _, h1725, _) => {
            h1725
        }
    } {
        Ok(None::<Adt36>)
    } else {
        Ok(Some({
            let m1729 = f11(v0.clone(), {
                let m1728 = match v1.clone() {
                    Adt36::C0(_, h1727, _, _) => {
                        h1727
                    }
                };
                match m1728 {
                    Adt10::C0(h1726, _) => {
                        h1726
                    }
                }
            })?;
            match m1729 {
                Ok(v3) => {
                    let a1731 = match v3 {
                        Adt18::C0(h1730, _, _, _) => {
                            h1730
                        }
                    };
                    let a1732 = 5u64;
                    if nat_eq(a1731, a1732) {
                        Adt36::C0(match v1.clone() {
                            Adt36::C0(h1733, _, _, _) => {
                                h1733
                            }
                        }, match v1 {
                            Adt36::C0(_, h1734, _, _) => {
                                h1734
                            }
                        }, true, None::<Adt8>)
                    } else {
                        let m1736 = f74(v0.clone(), match v1.clone() {
                            Adt36::C0(_, h1735, _, _) => {
                                h1735
                            }
                        }, f15(Str::lit("an entry's key")))?;
                        match m1736 {
                            Ok(v5) => {
                                let m1738 = f16(v0.clone(), match v5.clone() {
                                    Adt21::C0(_, h1737) => {
                                        h1737
                                    }
                                }, 8u64, f15(Str::lit("`:=` after the key")))?;
                                match m1738 {
                                    Ok(v7) => {
                                        let m1742 = f11(v0.clone(), {
                                            let m1741 = match v7.clone() {
                                                Adt19::C0(_, h1740) => {
                                                    h1740
                                                }
                                            };
                                            match m1741 {
                                                Adt10::C0(h1739, _) => {
                                                    h1739
                                                }
                                            }
                                        })?;
                                        match m1742 {
                                            Ok(v9) => {
                                                let a1744 = match v9.clone() {
                                                    Adt18::C0(h1743, _, _, _) => {
                                                        h1743
                                                    }
                                                };
                                                let a1745 = 20u64;
                                                if nat_eq(a1744, a1745) {
                                                    let m1747 = f230(v0.clone(), match v7 {
                                                        Adt19::C0(_, h1746) => {
                                                            h1746
                                                        }
                                                    }, v9)?;
                                                    match m1747 {
                                                        Ok(v11) => {
                                                            let v12: List<Adt4> = List::cons(Adt4::C0(match v5 {
                                                                Adt21::C0(h1748, _) => {
                                                                    h1748
                                                                }
                                                            }, {
                                                                let (h1749, _) = v11.clone();
                                                                h1749
                                                            }), match v1.clone() {
                                                                Adt36::C0(h1750, _, _, _) => {
                                                                    h1750
                                                                }
                                                            });
                                                            let m1754 = f11(v0, {
                                                                let m1753 = {
                                                                    let (_, h1752) = v11.clone();
                                                                    h1752
                                                                };
                                                                match m1753 {
                                                                    Adt10::C0(h1751, _) => {
                                                                        h1751
                                                                    }
                                                                }
                                                            })?;
                                                            match m1754 {
                                                                Ok(v14) => {
                                                                    let a1756 = match v14.clone() {
                                                                        Adt18::C0(h1755, _, _, _) => {
                                                                            h1755
                                                                        }
                                                                    };
                                                                    let a1757 = 6u64;
                                                                    if nat_eq(a1756, a1757) {
                                                                        Adt36::C0(v12, f17(v14), false, None::<Adt8>)
                                                                    } else {
                                                                        Adt36::C0(v12, {
                                                                            let (_, h1758) = v11;
                                                                            h1758
                                                                        }, true, None::<Adt8>)
                                                                    }
                                                                }
                                                                Err(v13) => {
                                                                    f229(v1, v13)
                                                                }
                                                            }
                                                        }
                                                        Err(v10) => {
                                                            f229(v1, v10)
                                                        }
                                                    }
                                                } else {
                                                    f229(v1, f21(match v9 {
                                                        Adt18::C0(_, h1759, _, _) => {
                                                            h1759
                                                        }
                                                    }, f15(Str::lit("expected the type of the value"))))
                                                }
                                            }
                                            Err(v8) => {
                                                f229(v1, v8)
                                            }
                                        }
                                    }
                                    Err(v6) => {
                                        f229(v1, v6)
                                    }
                                }
                            }
                            Err(v4) => {
                                f229(v1, v4)
                            }
                        }
                    }
                }
                Err(v2) => {
                    f229(v1, v2)
                }
            }
        }))
    }
}

pub fn f213(v0: Fn30, p1763: List<Adt4>, p1764: List<Adt4>) -> List<Adt4> {
    let m1762 = (p1763, p1764);
    {
        let mut m1762 = m1762;
        loop {
            let step = {
                let (v1, v2) = m1762;
                let m1760 = v2;
                match m1760.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c1761: Fn30 = v0.clone();
                            c1761.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1762 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f215(v0: Bytes, v1: Adt37) -> R<Option<Adt37>> {
    let v2: u64 = match v1.clone() {
        Adt37::C0(h1765, _) => {
            h1765
        }
    };
    let v3: u64 = f34(v0.clone(), v2)?;
    let a1766 = 256u64;
    let a1767 = v3;
    if nat_le(a1766, a1767) {
        Ok(None::<Adt37>)
    } else {
        let v4: u64 = f45(v3);
        let v5: u64 = {
            let a1768 = v4;
            let a1769 = 1u64;
            if nat_eq(a1768, a1769) {
                v3
            } else {
                let a1770 = v4;
                let a1771 = 2u64;
                if nat_eq(a1770, a1771) {
                    let a1772 = v3;
                    let a1773 = 192u64;
                    nat_sub(a1772, a1773)
                } else {
                    let a1774 = v4;
                    let a1775 = 3u64;
                    if nat_eq(a1774, a1775) {
                        let a1776 = v3;
                        let a1777 = 224u64;
                        nat_sub(a1776, a1777)
                    } else {
                        let a1778 = v3;
                        let a1779 = 240u64;
                        nat_sub(a1778, a1779)
                    }
                }
            }
        };
        Ok(Some(Adt37::C0({
            let a1780 = v2;
            let a1781 = v4;
            nat_add(a1780, a1781)?
        }, List::cons({
            let m1785 = f231(Fn32::F232(v2, v0), {
                let a1783 = v4;
                let a1784 = 1u64;
                nat_sub(a1783, a1784)
            }, Adt24::C0(v5, 1u64))?;
            match m1785 {
                Adt24::C0(h1782, _) => {
                    h1782
                }
            }
        }, match v1 {
            Adt37::C0(_, h1786) => {
                h1786
            }
        }))))
    }
}

pub fn f216(v0: Fn31, p1791: u64, p1792: Adt39) -> R<(Adt39, bool)> {
    let m1790 = (p1791, p1792);
    {
        let mut m1790 = m1790;
        loop {
            let step = {
                let (v1, v2) = m1790;
                let m1787 = v1;
                if m1787 == 0 {
                    Err::<(u64, Adt39), R<(Adt39, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1787 - 1;
                    let m1789 = {
                        let c1788: Fn31 = v0.clone();
                        c1788.apply(v2.clone())?
                    };
                    match m1789 {
                        None => {
                            Err::<(u64, Adt39), R<(Adt39, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt39), R<(Adt39, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: u64, v1: Bytes, v2: Adt39) -> R<Option<Adt39>> {
    f233(v1, v0, v2)
}

pub fn f218(v0: List<Adt2>) -> List<Adt2> {
    f234(Fn33::F235, List::<Adt2>::nil(), v0)
}

pub fn f219(v0: List<Bytes>, v1: Bytes) -> R<Option<u64>> {
    let m1794 = f236(Fn34::F237(v1), Adt40::C0(0u64, None::<u64>), v0)?;
    match m1794 {
        Adt40::C0(_, h1793) => {
            Ok(h1793)
        }
    }
}

pub fn f220(v0: List<Bytes>) -> Bytes {
    f238(f239(v0))
}

pub fn f221(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1795, _) = f55(Fn9::F240(v1, v0), {
        let a1796 = v1;
        let a1797 = v2;
        nat_sub(a1796, a1797)
    }, v2)?;
    Ok(h1795)
}

pub fn f222(v0: List<Adt27>, v1: u64, v2: Adt30) -> R<Adt30> {
    let v3: u64 = match v2.clone() {
        Adt30::C0(_, _, h1798, _, _, _) => {
            h1798
        }
    };
    let a1799 = v3;
    let a1800 = 0u64;
    if nat_eq(a1799, a1800) {
        f241(v0, v1, v2)
    } else {
        let a1801 = v3;
        let a1802 = 1u64;
        if nat_eq(a1801, a1802) {
            f242(v0, v2)
        } else {
            let a1803 = v3;
            let a1804 = 2u64;
            if nat_eq(a1803, a1804) {
                f243(v0, v1, v2)
            } else {
                let a1805 = v3;
                let a1806 = 3u64;
                if nat_eq(a1805, a1806) {
                    f244(v0, v2)
                } else {
                    let a1807 = v3;
                    let a1808 = 4u64;
                    if nat_eq(a1807, a1808) {
                        f245(v0, v1, v2, 3u64, f158(Str::lit("expected `,` or `]`")))
                    } else {
                        f245(v0, v1, v2, 1u64, f158(Str::lit("expected `,` or `}`")))
                    }
                }
            }
        }
    }
}

pub fn f223(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1853 = {
            let a1829 = {
                let a1817 = {
                    let a1811 = {
                        let a1809 = v1;
                        let a1810 = 128u8;
                        fixed_u8::bit_and(a1809, a1810)
                    };
                    let a1812 = 128u8;
                    if equal(a1811, a1812) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1818 = {
                    let a1815 = {
                        let a1813 = v1;
                        let a1814 = 64u8;
                        fixed_u8::bit_and(a1813, a1814)
                    };
                    let a1816 = 64u8;
                    if equal(a1815, a1816) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1817, a1818)?
            };
            let a1830 = {
                let a1827 = {
                    let a1821 = {
                        let a1819 = v1;
                        let a1820 = 32u8;
                        fixed_u8::bit_and(a1819, a1820)
                    };
                    let a1822 = 32u8;
                    if equal(a1821, a1822) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1828 = {
                    let a1825 = {
                        let a1823 = v1;
                        let a1824 = 16u8;
                        fixed_u8::bit_and(a1823, a1824)
                    };
                    let a1826 = 16u8;
                    if equal(a1825, a1826) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1827, a1828)?
            };
            nat_add(a1829, a1830)?
        };
        let a1854 = {
            let a1851 = {
                let a1839 = {
                    let a1833 = {
                        let a1831 = v1;
                        let a1832 = 8u8;
                        fixed_u8::bit_and(a1831, a1832)
                    };
                    let a1834 = 8u8;
                    if equal(a1833, a1834) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1840 = {
                    let a1837 = {
                        let a1835 = v1;
                        let a1836 = 4u8;
                        fixed_u8::bit_and(a1835, a1836)
                    };
                    let a1838 = 4u8;
                    if equal(a1837, a1838) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1839, a1840)?
            };
            let a1852 = {
                let a1849 = {
                    let a1843 = {
                        let a1841 = v1;
                        let a1842 = 2u8;
                        fixed_u8::bit_and(a1841, a1842)
                    };
                    let a1844 = 2u8;
                    if equal(a1843, a1844) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1850 = {
                    let a1847 = {
                        let a1845 = v1;
                        let a1846 = 1u8;
                        fixed_u8::bit_and(a1845, a1846)
                    };
                    let a1848 = 1u8;
                    if equal(a1847, a1848) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1849, a1850)?
            };
            nat_add(a1851, a1852)?
        };
        nat_add(a1853, a1854)?
    };
    let a1855 = v2;
    let a1856 = 48u64;
    if nat_lt(a1855, a1856) {
        Ok(None::<u64>)
    } else {
        let a1857 = v2;
        let a1858 = 57u64;
        if nat_le(a1857, a1858) {
            Ok(Some({
                let a1859 = v2;
                let a1860 = 48u64;
                nat_sub(a1859, a1860)
            }))
        } else {
            let a1861 = v2;
            let a1862 = 65u64;
            if nat_lt(a1861, a1862) {
                Ok(None::<u64>)
            } else {
                let a1863 = v2;
                let a1864 = 70u64;
                if nat_le(a1863, a1864) {
                    Ok(Some({
                        let a1865 = v2;
                        let a1866 = 55u64;
                        nat_sub(a1865, a1866)
                    }))
                } else {
                    let a1867 = v2;
                    let a1868 = 97u64;
                    if nat_lt(a1867, a1868) {
                        Ok(None::<u64>)
                    } else {
                        let a1869 = v2;
                        let a1870 = 102u64;
                        if nat_le(a1869, a1870) {
                            Ok(Some({
                                let a1871 = v2;
                                let a1872 = 87u64;
                                nat_sub(a1871, a1872)
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f224(v0: Adt33, v1: Adt10, v2: List<Adt34>, v3: List<Adt2>) -> Adt33 {
    Adt33::C0(v1, v2, v3, match v0 {
        Adt33::C0(_, _, _, h1873) => {
            h1873
        }
    })
}

pub fn f225(v0: Bytes, v1: Adt33, v2: List<Adt34>, v3: u64) -> R<Adt33> {
    let v4: Adt10 = match v1.clone() {
        Adt33::C0(h1874, _, _, _) => {
            h1874
        }
    };
    let m1876 = f11(v0.clone(), match v4.clone() {
        Adt10::C0(h1875, _) => {
            h1875
        }
    })?;
    match m1876 {
        Ok(v6) => {
            let m1878 = f168(v3, match v6.clone() {
                Adt18::C0(_, h1877, _, _) => {
                    h1877
                }
            });
            match m1878 {
                None => {
                    let v8: u64 = match v6.clone() {
                        Adt18::C0(h1879, _, _, _) => {
                            h1879
                        }
                    };
                    let v9: Bytes = f14(v0.clone(), v6.clone());
                    let a1880 = v8;
                    let a1881 = 18u64;
                    if nat_eq(a1880, a1881) {
                        let m1882 = f177(v0, v4, f15(Str::lit("a level")))?;
                        match m1882 {
                            Ok(v10) => {
                                let a1884 = 1024u64;
                                let a1885 = match v10.clone() {
                                    Adt35::C0(h1883, _) => {
                                        h1883
                                    }
                                };
                                if nat_lt(a1884, a1885) {
                                    Ok(f226(v1, f21(match v6 {
                                        Adt18::C0(_, h1886, _, _) => {
                                            h1886
                                        }
                                    }, f15(Str::lit("a level is at most 1024")))))
                                } else {
                                    Ok(f224(v1.clone(), match v10.clone() {
                                        Adt35::C0(_, h1887) => {
                                            h1887
                                        }
                                    }, v2, List::cons(f246(match v10 {
                                        Adt35::C0(h1888, _) => {
                                            h1888
                                        }
                                    }, Adt2::C0), match v1 {
                                        Adt33::C0(_, _, h1889, _) => {
                                            h1889
                                        }
                                    })))
                                }
                            }
                            Err(v11) => {
                                Ok(f226(v1, v11))
                            }
                        }
                    } else {
                        let a1890 = v8;
                        let a1891 = 0u64;
                        if nat_eq(a1890, a1891) {
                            Ok(f224(v1.clone(), f17(v6), List::cons(Adt34::C0({
                                let a1892 = v3;
                                let a1893 = 1u64;
                                nat_add(a1892, a1893)?
                            }), List::cons(Adt34::C3, v2)), match v1 {
                                Adt33::C0(_, _, h1894, _) => {
                                    h1894
                                }
                            }))
                        } else {
                            if f12({
                                let a1895 = v8;
                                let a1896 = 20u64;
                                nat_eq(a1895, a1896)
                            }, if f13(v9.clone(), f15(Str::lit("max"))) {
                                true
                            } else {
                                f13(v9.clone(), f15(Str::lit("imax")))
                            }) {
                                let m1897 = f16(v0, f17(v6), 0u64, f15(Str::lit("`(` after `max`")))?;
                                match m1897 {
                                    Ok(v12) => {
                                        Ok(f224(v1.clone(), match v12 {
                                            Adt19::C0(_, h1898) => {
                                                h1898
                                            }
                                        }, List::cons(Adt34::C0({
                                            let a1899 = v3;
                                            let a1900 = 1u64;
                                            nat_add(a1899, a1900)?
                                        }), List::cons(Adt34::C4(v3, f13(v9, f15(Str::lit("max")))), v2)), match v1 {
                                            Adt33::C0(_, _, h1901, _) => {
                                                h1901
                                            }
                                        }))
                                    }
                                    Err(v13) => {
                                        Ok(f226(v1, v13))
                                    }
                                }
                            } else {
                                let a1902 = v8;
                                let a1903 = 20u64;
                                if if nat_eq(a1902, a1903) {
                                    true
                                } else {
                                    let a1904 = v8;
                                    let a1905 = 19u64;
                                    nat_eq(a1904, a1905)
                                } {
                                    let m1906 = f74(v0, v4, f15(Str::lit("a level")))?;
                                    match m1906 {
                                        Ok(v14) => {
                                            Ok(f224(v1.clone(), match v14.clone() {
                                                Adt21::C0(_, h1907) => {
                                                    h1907
                                                }
                                            }, v2, List::cons(Adt2::C4(match v14 {
                                                Adt21::C0(h1908, _) => {
                                                    h1908
                                                }
                                            }), match v1 {
                                                Adt33::C0(_, _, h1909, _) => {
                                                    h1909
                                                }
                                            })))
                                        }
                                        Err(v15) => {
                                            Ok(f226(v1, v15))
                                        }
                                    }
                                } else {
                                    Ok(f226(v1, f21(match v6 {
                                        Adt18::C0(_, h1910, _, _) => {
                                            h1910
                                        }
                                    }, f15(Str::lit("expected a universe level")))))
                                }
                            }
                        }
                    }
                }
                Some(v7) => {
                    Ok(f226(v1, v7))
                }
            }
        }
        Err(v5) => {
            Ok(f226(v1, v5))
        }
    }
}

pub fn f226(v0: Adt33, v1: Adt8) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt33::C0(h1911, _, _, _) => {
            h1911
        }
    }, match v0.clone() {
        Adt33::C0(_, h1912, _, _) => {
            h1912
        }
    }, match v0 {
        Adt33::C0(_, _, h1913, _) => {
            h1913
        }
    }, Some(v1))
}

pub fn f227(v0: List<Adt2>, v1: bool) -> List<Adt2> {
    let m1914 = v0.clone();
    match m1914.uncons() {
        None => {
            v0
        }
        Some((v2, v3)) => {
            let m1915 = v3;
            match m1915.uncons() {
                None => {
                    v0
                }
                Some((v4, v5)) => {
                    List::cons(if v1 {
                        Adt2::C2(std::rc::Rc::new(v4), std::rc::Rc::new(v2))
                    } else {
                        Adt2::C3(std::rc::Rc::new(v4), std::rc::Rc::new(v2))
                    }, v5)
                }
            }
        }
    }
}

pub fn f228(v0: List<Adt2>, v1: u64) -> List<Adt2> {
    let m1916 = v0.clone();
    match m1916.uncons() {
        None => {
            v0
        }
        Some((v2, v3)) => {
            List::cons(f246(v1, v2), v3)
        }
    }
}

pub fn f229(v0: Adt36, v1: Adt8) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h1917, _, _, _) => {
            h1917
        }
    }, match v0 {
        Adt36::C0(_, h1918, _, _) => {
            h1918
        }
    }, true, Some(v1))
}

pub fn f230(v0: Bytes, _: Adt10, v2: Adt18) -> R<Result<(Adt5, Adt10), Adt8>> {
    let v3: Bytes = f14(v0.clone(), v2.clone());
    let v4: Adt10 = f17(v2.clone());
    if f13(v3.clone(), f15(Str::lit("bool"))) {
        let m1920 = f11(v0.clone(), match v4 {
            Adt10::C0(h1919, _) => {
                h1919
            }
        })?;
        match m1920 {
            Ok(v6) => {
                if f12({
                    let a1922 = match v6.clone() {
                        Adt18::C0(h1921, _, _, _) => {
                            h1921
                        }
                    };
                    let a1923 = 20u64;
                    nat_eq(a1922, a1923)
                }, f13(f14(v0.clone(), v6.clone()), f15(Str::lit("true")))) {
                    Ok(Ok::<(Adt5, Adt10), Adt8>((Adt5::C0(true), f17(v6))))
                } else {
                    if f12({
                        let a1925 = match v6.clone() {
                            Adt18::C0(h1924, _, _, _) => {
                                h1924
                            }
                        };
                        let a1926 = 20u64;
                        nat_eq(a1925, a1926)
                    }, f13(f14(v0, v6.clone()), f15(Str::lit("false")))) {
                        Ok(Ok::<(Adt5, Adt10), Adt8>((Adt5::C0(false), f17(v6))))
                    } else {
                        Ok(Err::<(Adt5, Adt10), Adt8>(f21(match v6 {
                            Adt18::C0(_, h1927, _, _) => {
                                h1927
                            }
                        }, f15(Str::lit("expected `true` or `false`")))))
                    }
                }
            }
            Err(v5) => {
                Ok(Err::<(Adt5, Adt10), Adt8>(v5))
            }
        }
    } else {
        if if f13(v3.clone(), f15(Str::lit("nat"))) {
            true
        } else {
            f13(v3.clone(), f15(Str::lit("int")))
        } {
            let m1928 = f247(v0, v4, f13(v3.clone(), f15(Str::lit("int"))))?;
            match m1928 {
                Ok(v8) => {
                    Ok(Ok::<(Adt5, Adt10), Adt8>((if f13(v3, f15(Str::lit("int"))) {
                        Adt5::C2(match v8.clone() {
                            Adt21::C0(h1929, _) => {
                                h1929
                            }
                        })
                    } else {
                        Adt5::C1(match v8.clone() {
                            Adt21::C0(h1930, _) => {
                                h1930
                            }
                        })
                    }, match v8 {
                        Adt21::C0(_, h1931) => {
                            h1931
                        }
                    })))
                }
                Err(v7) => {
                    Ok(Err::<(Adt5, Adt10), Adt8>(v7))
                }
            }
        } else {
            if f13(v3.clone(), f15(Str::lit("str"))) {
                let m1932 = f16(v0.clone(), v4, 19u64, f15(Str::lit("a string")))?;
                match m1932 {
                    Ok(v10) => {
                        Ok(Ok::<(Adt5, Adt10), Adt8>((Adt5::C3(f96(v0, {
                            let m1935 = match v10.clone() {
                                Adt19::C0(h1934, _) => {
                                    h1934
                                }
                            };
                            match m1935 {
                                Adt18::C0(_, h1933, _, _) => {
                                    h1933
                                }
                            }
                        })?), match v10 {
                            Adt19::C0(_, h1936) => {
                                h1936
                            }
                        })))
                    }
                    Err(v9) => {
                        Ok(Err::<(Adt5, Adt10), Adt8>(v9))
                    }
                }
            } else {
                if f13(v3.clone(), f15(Str::lit("name"))) {
                    let m1937 = f74(v0, v4, f15(Str::lit("a name")))?;
                    match m1937 {
                        Ok(v12) => {
                            Ok(Ok::<(Adt5, Adt10), Adt8>((Adt5::C4(match v12.clone() {
                                Adt21::C0(h1938, _) => {
                                    h1938
                                }
                            }), match v12 {
                                Adt21::C0(_, h1939) => {
                                    h1939
                                }
                            })))
                        }
                        Err(v11) => {
                            Ok(Err::<(Adt5, Adt10), Adt8>(v11))
                        }
                    }
                } else {
                    Ok(Err::<(Adt5, Adt10), Adt8>(f21(match v2 {
                        Adt18::C0(_, h1940, _, _) => {
                            h1940
                        }
                    }, f43(f43(f15(Str::lit("`")), v3), f15(Str::lit("` is not the type of an annotation value"))))))
                }
            }
        }
    }
}

pub fn f231(v0: Fn32, p1944: u64, p1945: Adt24) -> R<Adt24> {
    let m1943 = (p1944, p1945);
    {
        let mut m1943 = m1943;
        loop {
            let step = {
                let (v1, v2) = m1943;
                let m1941 = v1;
                if m1941 == 0 {
                    Err::<(u64, Adt24), R<Adt24>>(Ok(v2))
                } else {
                    let v3 = m1941 - 1;
                    Ok::<(u64, Adt24), R<Adt24>>((v3, {
                        let c1942: Fn32 = v0.clone();
                        c1942.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1943 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: u64, v1: Bytes, v2: Adt24) -> R<Adt24> {
    Ok(Adt24::C0({
        let a1954 = {
            let a1947 = match v2.clone() {
                Adt24::C0(h1946, _) => {
                    h1946
                }
            };
            let a1948 = 64u64;
            nat_mul(a1947, a1948)?
        };
        let a1955 = {
            let a1952 = f34(v1, {
                let a1950 = v0;
                let a1951 = match v2.clone() {
                    Adt24::C0(_, h1949) => {
                        h1949
                    }
                };
                nat_add(a1950, a1951)?
            })?;
            let a1953 = 128u64;
            nat_sub(a1952, a1953)
        };
        nat_add(a1954, a1955)?
    }, {
        let a1957 = match v2 {
            Adt24::C0(_, h1956) => {
                h1956
            }
        };
        let a1958 = 1u64;
        nat_add(a1957, a1958)?
    }))
}

pub fn f233(v0: Bytes, v1: u64, v2: Adt39) -> R<Option<Adt39>> {
    if match v2.clone() {
        Adt39::C0(_, _, h1959, _) => {
            h1959
        }
    } {
        Ok(None::<Adt39>)
    } else {
        Ok(Some({
            let m1963 = f176(v0.clone(), match v2.clone() {
                Adt39::C0(_, h1960, _, _) => {
                    h1960
                }
            }, {
                let a1961 = v1;
                let a1962 = 1u64;
                nat_add(a1961, a1962)?
            })?;
            match m1963 {
                Ok(v4) => {
                    let v5: List<Adt2> = List::cons(match v4.clone() {
                        Adt32::C0(h1964, _) => {
                            h1964
                        }
                    }, match v2.clone() {
                        Adt39::C0(h1965, _, _, _) => {
                            h1965
                        }
                    });
                    let m1969 = f11(v0, {
                        let m1968 = match v4.clone() {
                            Adt32::C0(_, h1967) => {
                                h1967
                            }
                        };
                        match m1968 {
                            Adt10::C0(h1966, _) => {
                                h1966
                            }
                        }
                    })?;
                    match m1969 {
                        Ok(v7) => {
                            let a1971 = match v7.clone() {
                                Adt18::C0(h1970, _, _, _) => {
                                    h1970
                                }
                            };
                            let a1972 = 6u64;
                            if nat_eq(a1971, a1972) {
                                Adt39::C0(v5, f17(v7), false, None::<Adt8>)
                            } else {
                                Adt39::C0(v5, match v4 {
                                    Adt32::C0(_, h1973) => {
                                        h1973
                                    }
                                }, true, None::<Adt8>)
                            }
                        }
                        Err(v6) => {
                            f248(v2, v6)
                        }
                    }
                }
                Err(v3) => {
                    f248(v2, v3)
                }
            }
        }))
    }
}

pub fn f234(v0: Fn33, p1977: List<Adt2>, p1978: List<Adt2>) -> List<Adt2> {
    let m1976 = (p1977, p1978);
    {
        let mut m1976 = m1976;
        loop {
            let step = {
                let (v1, v2) = m1976;
                let m1974 = v2;
                match m1974.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c1975: Fn33 = v0.clone();
                            c1975.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f236(v0: Fn34, p1982: Adt40, p1983: List<Bytes>) -> R<Adt40> {
    let m1981 = (p1982, p1983);
    {
        let mut m1981 = m1981;
        loop {
            let step = {
                let (v1, v2) = m1981;
                let m1979 = v2;
                match m1979.uncons() {
                    None => {
                        Err::<(Adt40, List<Bytes>), R<Adt40>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt40, List<Bytes>), R<Adt40>>(({
                            let c1980: Fn34 = v0.clone();
                            c1980.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Bytes, v1: Adt40, v2: Bytes) -> R<Adt40> {
    Ok(Adt40::C0({
        let a1985 = match v1.clone() {
            Adt40::C0(h1984, _) => {
                h1984
            }
        };
        let a1986 = 1u64;
        nat_add(a1985, a1986)?
    }, {
        let m1988 = match v1.clone() {
            Adt40::C0(_, h1987) => {
                h1987
            }
        };
        match m1988 {
            None => {
                if f12({
                    let a1991 = 0u64;
                    let a1992 = {
                        let a1990 = v2.clone();
                        length_bytes(a1990)
                    };
                    nat_lt(a1991, a1992)
                }, f13(v2, v0)) {
                    Some(match v1 {
                        Adt40::C0(h1993, _) => {
                            h1993
                        }
                    })
                } else {
                    None::<u64>
                }
            }
            Some(_) => {
                match v1 {
                    Adt40::C0(_, h1989) => {
                        h1989
                    }
                }
            }
        }
    }))
}

pub fn f238(v0: List<Bytes>) -> Bytes {
    let m1996 = {
        let (h1994, _) = f249(Fn35::F250, {
            let a1995 = v0.clone();
            length_list(a1995)
        }, v0);
        h1994
    };
    match m1996.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f239(v0: List<Bytes>) -> List<Bytes> {
    f47(Fn8::F251, List::<Bytes>::nil(), v0)
}

pub fn f240(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f252(v1, v0, v2)
}

pub fn f241(v0: List<Adt27>, v1: u64, v2: Adt30) -> R<Adt30> {
    let v3: u64 = match v2.clone() {
        Adt30::C0(h1997, _, _, _, _, _) => {
            h1997
        }
    };
    let a1999 = 256u64;
    let a2000 = match v2.clone() {
        Adt30::C0(_, h1998, _, _, _, _) => {
            h1998
        }
    };
    if nat_lt(a1999, a2000) {
        Ok(f253(v2, f157(v0, v1, v3), f158(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v4: u64 = f254(v0.clone(), v3);
        let a2001 = v4;
        let a2002 = 99u64;
        if nat_eq(a2001, a2002) {
            Ok(f253(v2, v1, f158(Str::lit("expected a value, found the end of the text"))))
        } else {
            let a2003 = v4;
            let a2004 = 11u64;
            if nat_eq(a2003, a2004) {
                Ok(f255(v2, {
                    let a2005 = v3;
                    let a2006 = 1u64;
                    nat_add(a2005, a2006)?
                }, Adt6::C2(f256(v0, v3))))
            } else {
                let a2007 = v4;
                let a2008 = 2u64;
                if nat_eq(a2007, a2008) {
                    f257(v2, {
                        let a2009 = v3;
                        let a2010 = 1u64;
                        nat_add(a2009, a2010)?
                    }, Adt31::C0(List::<Adt6>::nil()), 1u64)
                } else {
                    let a2011 = v4;
                    let a2012 = 0u64;
                    if nat_eq(a2011, a2012) {
                        f257(v2, {
                            let a2013 = v3;
                            let a2014 = 1u64;
                            nat_add(a2013, a2014)?
                        }, Adt31::C1(Bytes::lit(&[]), 0u64, List::<(Bytes, Adt6)>::nil()), 2u64)
                    } else {
                        let a2015 = v4;
                        let a2016 = 12u64;
                        if nat_eq(a2015, a2016) {
                            f258(v0.clone(), v1, v2, f157(v0, v1, v3))
                        } else {
                            Ok(f253(v2, f157(v0.clone(), v1, v3), f259(f158(Str::lit("expected a value, found ")), f260(v0, v3)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f242(v0: List<Adt27>, v1: Adt30) -> R<Adt30> {
    let a2018 = f254(v0, match v1.clone() {
        Adt30::C0(h2017, _, _, _, _, _) => {
            h2017
        }
    });
    let a2019 = 3u64;
    if nat_eq(a2018, a2019) {
        f261(v1)
    } else {
        Ok(Adt30::C0(match v1.clone() {
            Adt30::C0(h2020, _, _, _, _, _) => {
                h2020
            }
        }, match v1.clone() {
            Adt30::C0(_, h2021, _, _, _, _) => {
                h2021
            }
        }, 0u64, match v1 {
            Adt30::C0(_, _, _, h2022, _, _) => {
                h2022
            }
        }, Adt6::C5, None::<Adt8>))
    }
}

pub fn f243(v0: List<Adt27>, v1: u64, v2: Adt30) -> R<Adt30> {
    let m2024 = match v2.clone() {
        Adt30::C0(_, _, _, h2023, _, _) => {
            h2023
        }
    };
    match m2024.uncons() {
        None => {
            Ok(f253(v2, 0u64, f158(Str::lit("no object is open"))))
        }
        Some((v3, v4)) => {
            let m2025 = v3;
            match m2025 {
                Adt31::C0(_) => {
                    Ok(f253(v2, 0u64, f158(Str::lit("no object is open"))))
                }
                Adt31::C1(_, _, v8) => {
                    let a2027 = f254(v0.clone(), match v2.clone() {
                        Adt30::C0(h2026, _, _, _, _, _) => {
                            h2026
                        }
                    });
                    let a2028 = 1u64;
                    if nat_eq(a2027, a2028) {
                        f261(v2)
                    } else {
                        f262(v0, v1, v2, v8, v4)
                    }
                }
            }
        }
    }
}

pub fn f244(v0: List<Adt27>, v1: Adt30) -> R<Adt30> {
    let m2030 = match v1.clone() {
        Adt30::C0(_, _, _, h2029, _, _) => {
            h2029
        }
    };
    match m2030.uncons() {
        None => {
            Ok(Adt30::C0(match v1.clone() {
                Adt30::C0(h2031, _, _, _, _, _) => {
                    h2031
                }
            }, match v1.clone() {
                Adt30::C0(_, h2032, _, _, _, _) => {
                    h2032
                }
            }, 9u64, List::<Adt31>::nil(), match v1 {
                Adt30::C0(_, _, _, _, h2033, _) => {
                    h2033
                }
            }, None::<Adt8>))
        }
        Some((v2, v3)) => {
            let m2034 = v2;
            match m2034 {
                Adt31::C0(v4) => {
                    f263(v0, v1.clone(), List::cons(Adt31::C0(List::cons(match v1 {
                        Adt30::C0(_, _, _, _, h2035, _) => {
                            h2035
                        }
                    }, v4)), v3), 1u64, 4u64)
                }
                Adt31::C1(v5, v6, v7) => {
                    if f264(v7.clone(), v5.clone()) {
                        Ok(f253(v1, v6, f259(f259(f158(Str::lit("the member `")), v5), f158(Str::lit("` is written twice")))))
                    } else {
                        f263(v0, v1.clone(), List::cons(Adt31::C1(Bytes::lit(&[]), 0u64, List::cons((v5, match v1 {
                            Adt30::C0(_, _, _, _, h2036, _) => {
                                h2036
                            }
                        }), v7)), v3), 2u64, 5u64)
                    }
                }
            }
        }
    }
}

pub fn f245(v0: List<Adt27>, v1: u64, v2: Adt30, v3: u64, v4: Bytes) -> R<Adt30> {
    let a2038 = f254(v0.clone(), match v2.clone() {
        Adt30::C0(h2037, _, _, _, _, _) => {
            h2037
        }
    });
    let a2039 = v3;
    if nat_eq(a2038, a2039) {
        f261(v2)
    } else {
        Ok(f253(v2.clone(), f157(v0, v1, match v2 {
            Adt30::C0(h2040, _, _, _, _, _) => {
                h2040
            }
        }), v4))
    }
}

pub fn f246(v0: u64, v1: Adt2) -> Adt2 {
    f265(Fn36::F266, v0, v1)
}

pub fn f247(v0: Bytes, v1: Adt10, v2: bool) -> R<Result<Adt21, Adt8>> {
    let m2042 = f11(v0.clone(), match v1 {
        Adt10::C0(h2041, _) => {
            h2041
        }
    })?;
    match m2042 {
        Ok(v4) => {
            let v5: u64 = match v4.clone() {
                Adt18::C0(h2043, _, _, _) => {
                    h2043
                }
            };
            let a2044 = v5;
            let a2045 = 19u64;
            if nat_eq(a2044, a2045) {
                Ok(Ok::<Adt21, Adt8>(Adt21::C0(f96(v0, match v4.clone() {
                    Adt18::C0(_, h2046, _, _) => {
                        h2046
                    }
                })?, f17(v4))))
            } else {
                if f12({
                    let a2047 = v5;
                    let a2048 = 13u64;
                    nat_eq(a2047, a2048)
                }, v2) {
                    let m2049 = f16(v0.clone(), f17(v4), 18u64, f15(Str::lit("a number after `-`")))?;
                    match m2049 {
                        Ok(v7) => {
                            let v8: Bytes = f14(v0, match v7.clone() {
                                Adt19::C0(h2050, _) => {
                                    h2050
                                }
                            });
                            if if f12({
                                let a2052 = 1u64;
                                let a2053 = {
                                    let a2051 = v8.clone();
                                    length_bytes(a2051)
                                };
                                nat_lt(a2052, a2053)
                            }, f180(v8.clone(), f15(Str::lit("0")))) {
                                true
                            } else {
                                f13(v8.clone(), f15(Str::lit("0")))
                            } {
                                Ok(Err::<Adt21, Adt8>(f21({
                                    let m2056 = match v7 {
                                        Adt19::C0(h2055, _) => {
                                            h2055
                                        }
                                    };
                                    match m2056 {
                                        Adt18::C0(_, h2054, _, _) => {
                                            h2054
                                        }
                                    }
                                }, f15(Str::lit("a numeral has no redundant leading zero")))))
                            } else {
                                Ok(Ok::<Adt21, Adt8>(Adt21::C0(f43(f15(Str::lit("-")), v8), match v7 {
                                    Adt19::C0(_, h2057) => {
                                        h2057
                                    }
                                })))
                            }
                        }
                        Err(v6) => {
                            Ok(Err::<Adt21, Adt8>(v6))
                        }
                    }
                } else {
                    let a2058 = v5;
                    let a2059 = 18u64;
                    if nat_eq(a2058, a2059) {
                        let v9: Bytes = f14(v0, v4.clone());
                        if f12({
                            let a2061 = 1u64;
                            let a2062 = {
                                let a2060 = v9.clone();
                                length_bytes(a2060)
                            };
                            nat_lt(a2061, a2062)
                        }, f180(v9.clone(), f15(Str::lit("0")))) {
                            Ok(Err::<Adt21, Adt8>(f21(match v4 {
                                Adt18::C0(_, h2063, _, _) => {
                                    h2063
                                }
                            }, f15(Str::lit("a numeral has no redundant leading zero")))))
                        } else {
                            Ok(Ok::<Adt21, Adt8>(Adt21::C0(v9, f17(v4))))
                        }
                    } else {
                        Ok(Err::<Adt21, Adt8>(f21(match v4 {
                            Adt18::C0(_, h2064, _, _) => {
                                h2064
                            }
                        }, f15(Str::lit("expected a number")))))
                    }
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt21, Adt8>(v3))
        }
    }
}

pub fn f248(v0: Adt39, v1: Adt8) -> Adt39 {
    Adt39::C0(match v0.clone() {
        Adt39::C0(h2065, _, _, _) => {
            h2065
        }
    }, match v0 {
        Adt39::C0(_, h2066, _, _) => {
            h2066
        }
    }, true, Some(v1))
}

pub fn f249(v0: Fn35, p2071: u64, p2072: List<Bytes>) -> (List<Bytes>, bool) {
    let m2070 = (p2071, p2072);
    {
        let mut m2070 = m2070;
        loop {
            let step = {
                let (v1, v2) = m2070;
                let m2067 = v1;
                if m2067 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m2067 - 1;
                    let m2069 = {
                        let c2068: Fn35 = v0.clone();
                        c2068.apply(v2.clone())
                    };
                    match m2069 {
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
                Ok(next) => { m2070 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: List<Bytes>) -> Option<List<Bytes>> {
    f267(v0)
}

pub fn f251(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f252(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a2073 = v2;
    let a2074 = v1;
    if nat_lt(a2073, a2074) {
        if f268(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a2075 = v2;
                let a2076 = 1u64;
                nat_add(a2075, a2076)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f253(v0: Adt30, v1: u64, v2: Bytes) -> Adt30 {
    Adt30::C0(match v0.clone() {
        Adt30::C0(h2077, _, _, _, _, _) => {
            h2077
        }
    }, match v0.clone() {
        Adt30::C0(_, h2078, _, _, _, _) => {
            h2078
        }
    }, 9u64, match v0.clone() {
        Adt30::C0(_, _, _, h2079, _, _) => {
            h2079
        }
    }, match v0 {
        Adt30::C0(_, _, _, _, h2080, _) => {
            h2080
        }
    }, Some(f156(v1, v2)))
}

pub fn f254(v0: List<Adt27>, v1: u64) -> u64 {
    let m2081 = f189(v0, v1);
    match m2081 {
        None => {
            99u64
        }
        Some(v2) => {
            f269(match v2 {
                Adt27::C0(h2082, _, _) => {
                    h2082
                }
            })
        }
    }
}

pub fn f255(v0: Adt30, v1: u64, v2: Adt6) -> Adt30 {
    Adt30::C0(v1, match v0.clone() {
        Adt30::C0(_, h2083, _, _, _, _) => {
            h2083
        }
    }, 3u64, match v0 {
        Adt30::C0(_, _, _, h2084, _, _) => {
            h2084
        }
    }, v2, None::<Adt8>)
}

pub fn f256(v0: List<Adt27>, v1: u64) -> Bytes {
    let m2085 = f189(v0, v1);
    match m2085 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f270(match v2 {
                Adt27::C0(h2086, _, _) => {
                    h2086
                }
            })
        }
    }
}

pub fn f257(v0: Adt30, v1: u64, v2: Adt31, v3: u64) -> R<Adt30> {
    Ok(Adt30::C0(v1, {
        let a2088 = match v0.clone() {
            Adt30::C0(_, h2087, _, _, _, _) => {
                h2087
            }
        };
        let a2089 = 1u64;
        nat_add(a2088, a2089)?
    }, v3, List::cons(v2, match v0 {
        Adt30::C0(_, _, _, h2090, _, _) => {
            h2090
        }
    }), Adt6::C5, None::<Adt8>))
}

pub fn f258(v0: List<Adt27>, _: u64, v2: Adt30, v3: u64) -> R<Adt30> {
    let v4: u64 = match v2.clone() {
        Adt30::C0(h2091, _, _, _, _, _) => {
            h2091
        }
    };
    let v5: Bytes = f256(v0.clone(), v4);
    let a2092 = v5.clone();
    let a2093 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a2092, a2093) {
        Ok(f255(v2, {
            let a2094 = v4;
            let a2095 = 1u64;
            nat_add(a2094, a2095)?
        }, Adt6::C0(true)))
    } else {
        let a2096 = v5.clone();
        let a2097 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        if equal(a2096, a2097) {
            Ok(f255(v2, {
                let a2098 = v4;
                let a2099 = 1u64;
                nat_add(a2098, a2099)?
            }, Adt6::C0(false)))
        } else {
            if f271(v5.clone())? {
                let m2100 = f272(v5.clone())?;
                match m2100 {
                    None => {
                        Ok(f253(v2, v3, f259(f259(f158(Str::lit("`")), v5), f158(Str::lit("` does not fit a 64-bit integer")))))
                    }
                    Some(v6) => {
                        Ok(f255(v2, {
                            let a2101 = v4;
                            let a2102 = 1u64;
                            nat_add(a2101, a2102)?
                        }, v6))
                    }
                }
            } else {
                if f273(v5.clone())? {
                    let a2105 = f254(v0, {
                        let a2103 = v4;
                        let a2104 = 1u64;
                        nat_add(a2103, a2104)?
                    });
                    let a2106 = 0u64;
                    if nat_eq(a2105, a2106) {
                        f257(v2, {
                            let a2107 = v4;
                            let a2108 = 2u64;
                            nat_add(a2107, a2108)?
                        }, Adt31::C1(Bytes::lit(&[]), 0u64, List::cons((f158(Str::lit("kind")), Adt6::C2(v5)), List::<(Bytes, Adt6)>::nil())), 2u64)
                    } else {
                        Ok(f255(v2, {
                            let a2109 = v4;
                            let a2110 = 1u64;
                            nat_add(a2109, a2110)?
                        }, Adt6::C2(v5)))
                    }
                } else {
                    Ok(f253(v2, v3, f259(f259(f158(Str::lit("`")), v5), f158(Str::lit("` is not a name; quote it to write a string")))))
                }
            }
        }
    }
}

pub fn f259(v0: Bytes, v1: Bytes) -> Bytes {
    let a2111 = v0;
    let a2112 = v1;
    append_bytes(a2111, a2112)
}

pub fn f260(v0: List<Adt27>, v1: u64) -> R<Bytes> {
    let m2113 = f189(v0, v1);
    match m2113 {
        None => {
            Ok(f158(Str::lit("the end of the text")))
        }
        Some(v2) => {
            f274(match v2 {
                Adt27::C0(h2114, _, _) => {
                    h2114
                }
            })
        }
    }
}

pub fn f261(v0: Adt30) -> R<Adt30> {
    let m2116 = match v0.clone() {
        Adt30::C0(_, _, _, h2115, _, _) => {
            h2115
        }
    };
    match m2116.uncons() {
        None => {
            Ok(f253(v0, 0u64, f158(Str::lit("nothing to close"))))
        }
        Some((v1, v2)) => {
            let m2117 = v1;
            match m2117 {
                Adt31::C0(v3) => {
                    Ok(Adt30::C0({
                        let a2119 = match v0.clone() {
                            Adt30::C0(h2118, _, _, _, _, _) => {
                                h2118
                            }
                        };
                        let a2120 = 1u64;
                        nat_add(a2119, a2120)?
                    }, {
                        let a2122 = match v0 {
                            Adt30::C0(_, h2121, _, _, _, _) => {
                                h2121
                            }
                        };
                        let a2123 = 1u64;
                        nat_sub(a2122, a2123)
                    }, 3u64, v2, Adt6::C3(std::rc::Rc::new(f275(v3))), None::<Adt8>))
                }
                Adt31::C1(_, _, v6) => {
                    Ok(Adt30::C0({
                        let a2125 = match v0.clone() {
                            Adt30::C0(h2124, _, _, _, _, _) => {
                                h2124
                            }
                        };
                        let a2126 = 1u64;
                        nat_add(a2125, a2126)?
                    }, {
                        let a2128 = match v0 {
                            Adt30::C0(_, h2127, _, _, _, _) => {
                                h2127
                            }
                        };
                        let a2129 = 1u64;
                        nat_sub(a2128, a2129)
                    }, 3u64, v2, Adt6::C4(std::rc::Rc::new(f276(f277(v6)?))), None::<Adt8>))
                }
            }
        }
    }
}

pub fn f262(v0: List<Adt27>, v1: u64, v2: Adt30, v3: List<(Bytes, Adt6)>, v4: List<Adt31>) -> R<Adt30> {
    let v5: u64 = match v2.clone() {
        Adt30::C0(h2130, _, _, _, _, _) => {
            h2130
        }
    };
    let v6: u64 = f254(v0.clone(), v5);
    let v7: Bytes = f256(v0.clone(), v5);
    let a2131 = v6;
    let a2132 = 11u64;
    if if nat_eq(a2131, a2132) {
        true
    } else {
        let a2133 = v6;
        let a2134 = 12u64;
        if nat_eq(a2133, a2134) {
            f273(v7.clone())?
        } else {
            false
        }
    } {
        let a2137 = f254(v0.clone(), {
            let a2135 = v5;
            let a2136 = 1u64;
            nat_add(a2135, a2136)?
        });
        let a2138 = 7u64;
        if nat_eq(a2137, a2138) {
            Ok(Adt30::C0({
                let a2139 = v5;
                let a2140 = 2u64;
                nat_add(a2139, a2140)?
            }, match v2 {
                Adt30::C0(_, h2141, _, _, _, _) => {
                    h2141
                }
            }, 0u64, List::cons(Adt31::C1(v7, f157(v0, v1, v5), v3), v4), Adt6::C5, None::<Adt8>))
        } else {
            Ok(f253(v2, f157(v0, v1, {
                let a2142 = v5;
                let a2143 = 1u64;
                nat_add(a2142, a2143)?
            }), f158(Str::lit("expected `:` after a member name"))))
        }
    } else {
        Ok(f253(v2, f157(v0, v1, v5), f158(Str::lit("expected a member name"))))
    }
}

pub fn f263(v0: List<Adt27>, v1: Adt30, v2: List<Adt31>, v3: u64, v4: u64) -> R<Adt30> {
    let a2145 = f254(v0, match v1.clone() {
        Adt30::C0(h2144, _, _, _, _, _) => {
            h2144
        }
    });
    let a2146 = 6u64;
    if nat_eq(a2145, a2146) {
        Ok(Adt30::C0({
            let a2148 = match v1.clone() {
                Adt30::C0(h2147, _, _, _, _, _) => {
                    h2147
                }
            };
            let a2149 = 1u64;
            nat_add(a2148, a2149)?
        }, match v1 {
            Adt30::C0(_, h2150, _, _, _, _) => {
                h2150
            }
        }, v3, v2, Adt6::C5, None::<Adt8>))
    } else {
        Ok(Adt30::C0(match v1.clone() {
            Adt30::C0(h2151, _, _, _, _, _) => {
                h2151
            }
        }, match v1 {
            Adt30::C0(_, h2152, _, _, _, _) => {
                h2152
            }
        }, v4, v2, Adt6::C5, None::<Adt8>))
    }
}

pub fn f264(v0: List<(Bytes, Adt6)>, v1: Bytes) -> bool {
    f278(Fn37::F279(v1), false, v0)
}

pub fn f265(v0: Fn36, p2156: u64, p2157: Adt2) -> Adt2 {
    let m2155 = (p2156, p2157);
    {
        let mut m2155 = m2155;
        loop {
            let step = {
                let (v1, v2) = m2155;
                let m2153 = v1;
                if m2153 == 0 {
                    Err::<(u64, Adt2), Adt2>(v2)
                } else {
                    let v3 = m2153 - 1;
                    Ok::<(u64, Adt2), Adt2>((v3, {
                        let c2154: Fn36 = v0.clone();
                        c2154.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: Adt2) -> Adt2 {
    Adt2::C1(std::rc::Rc::new(v0))
}

pub fn f267(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m2158 = v0.clone();
    match m2158.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m2159 = v2;
            match m2159.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f280(Fn38::F281, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m2161 = {
                        let (h2160, _) = v5;
                        h2160
                    };
                    match m2161 {
                        None => {
                            Some(f239({
                                let (_, h2163) = v5;
                                h2163
                            }))
                        }
                        Some(v6) => {
                            Some(f239(List::cons(v6, {
                                let (_, h2162) = v5;
                                h2162
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f268(v0: Bytes, v1: u64) -> R<bool> {
    let a2164 = 0u64;
    let a2165 = f60(v0.clone(), v1)?;
    if nat_lt(a2164, a2165) {
        Ok(true)
    } else {
        Ok(f282(v0, v1))
    }
}

pub fn f269(v0: Adt28) -> u64 {
    let m2166 = v0;
    match m2166 {
        Adt28::C0 => {
            0u64
        }
        Adt28::C1 => {
            1u64
        }
        Adt28::C2 => {
            2u64
        }
        Adt28::C3 => {
            3u64
        }
        Adt28::C4 => {
            4u64
        }
        Adt28::C5 => {
            5u64
        }
        Adt28::C6 => {
            6u64
        }
        Adt28::C7 => {
            7u64
        }
        Adt28::C8 => {
            8u64
        }
        Adt28::C9 => {
            9u64
        }
        Adt28::C10 => {
            10u64
        }
        Adt28::C11(_) => {
            11u64
        }
        Adt28::C12(_) => {
            12u64
        }
    }
}

pub fn f270(v0: Adt28) -> Bytes {
    let m2167 = v0;
    match m2167 {
        Adt28::C0 => {
            Bytes::lit(&[])
        }
        Adt28::C1 => {
            Bytes::lit(&[])
        }
        Adt28::C2 => {
            Bytes::lit(&[])
        }
        Adt28::C3 => {
            Bytes::lit(&[])
        }
        Adt28::C4 => {
            Bytes::lit(&[])
        }
        Adt28::C5 => {
            Bytes::lit(&[])
        }
        Adt28::C6 => {
            Bytes::lit(&[])
        }
        Adt28::C7 => {
            Bytes::lit(&[])
        }
        Adt28::C8 => {
            Bytes::lit(&[])
        }
        Adt28::C9 => {
            Bytes::lit(&[])
        }
        Adt28::C10 => {
            Bytes::lit(&[])
        }
        Adt28::C11(v1) => {
            v1
        }
        Adt28::C12(v2) => {
            v2
        }
    }
}

pub fn f271(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2168 = f283(v0.clone(), 0u64)?;
        let a2169 = 45u64;
        if nat_eq(a2168, a2169) {
            1u64
        } else {
            0u64
        }
    };
    let a2171 = v1;
    let a2172 = {
        let a2170 = v0.clone();
        length_bytes(a2170)
    };
    if nat_lt(a2171, a2172) {
        let a2174 = f284(v0.clone(), v1)?;
        let a2175 = {
            let a2173 = v0;
            length_bytes(a2173)
        };
        Ok(nat_eq(a2174, a2175))
    } else {
        Ok(false)
    }
}

pub fn f272(v0: Bytes) -> R<Option<Adt6>> {
    let v1: bool = {
        let a2176 = f283(v0.clone(), 0u64)?;
        let a2177 = 45u64;
        nat_eq(a2176, a2177)
    };
    let v2: u64 = if v1 {
        1u64
    } else {
        0u64
    };
    let v3: u64 = f285(v0.clone(), v2)?;
    let v4: Bytes = f44(v0.clone(), v3, {
        let a2179 = {
            let a2178 = v0;
            length_bytes(a2178)
        };
        let a2180 = v3;
        nat_sub(a2179, a2180)
    });
    let a2182 = {
        let a2181 = v4.clone();
        length_bytes(a2181)
    };
    let a2183 = 0u64;
    if nat_eq(a2182, a2183) {
        Ok(Some(Adt6::C1(false, 0u64)))
    } else {
        let a2185 = 19u64;
        let a2186 = {
            let a2184 = v4.clone();
            length_bytes(a2184)
        };
        if nat_lt(a2185, a2186) {
            Ok(None::<Adt6>)
        } else {
            let m2187 = f286(v4)?;
            match m2187 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v5) => {
                    if f287(v1, v5) {
                        Ok(Some(Adt6::C1(v1, v5)))
                    } else {
                        Ok(None::<Adt6>)
                    }
                }
            }
        }
    }
}

pub fn f273(v0: Bytes) -> R<bool> {
    let a2189 = {
        let a2188 = v0.clone();
        length_bytes(a2188)
    };
    let a2190 = 0u64;
    if nat_eq(a2189, a2190) {
        Ok(false)
    } else {
        if f288(f283(v0.clone(), 0u64)?) {
            let a2192 = f289(v0.clone(), 1u64)?;
            let a2193 = {
                let a2191 = v0.clone();
                length_bytes(a2191)
            };
            if nat_eq(a2192, a2193) {
                let a2194 = f290(v0);
                Ok(bool_not(a2194))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f274(v0: Adt28) -> R<Bytes> {
    let m2195 = v0;
    match m2195 {
        Adt28::C0 => {
            Ok(f158(Str::lit("`{`")))
        }
        Adt28::C1 => {
            Ok(f158(Str::lit("`}`")))
        }
        Adt28::C2 => {
            Ok(f158(Str::lit("`[`")))
        }
        Adt28::C3 => {
            Ok(f158(Str::lit("`]`")))
        }
        Adt28::C4 => {
            Ok(f158(Str::lit("`(`")))
        }
        Adt28::C5 => {
            Ok(f158(Str::lit("`)`")))
        }
        Adt28::C6 => {
            Ok(f158(Str::lit("`,`")))
        }
        Adt28::C7 => {
            Ok(f158(Str::lit("`:`")))
        }
        Adt28::C8 => {
            Ok(f158(Str::lit("`:=`")))
        }
        Adt28::C9 => {
            Ok(f158(Str::lit("`::`")))
        }
        Adt28::C10 => {
            Ok(f158(Str::lit("`;`")))
        }
        Adt28::C11(v1) => {
            Ok(f259(f158(Str::lit("the string ")), f291(v1)?))
        }
        Adt28::C12(v2) => {
            Ok(f259(f259(f158(Str::lit("`")), v2), f158(Str::lit("`"))))
        }
    }
}

pub fn f275(v0: List<Adt6>) -> Adt6 {
    f123(Fn21::F292, Adt6::C5, v0)
}

pub fn f276(v0: List<(Bytes, Adt6)>) -> Adt6 {
    f293(Fn39::F294, Adt6::C5, f295(v0))
}

pub fn f277(v0: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    let v1: List<List<(Bytes, Adt6)>> = f296(f297(Fn40::F298, List::<List<(Bytes, Adt6)>>::nil(), v0.clone()));
    let m2200 = {
        let (h2196, _) = f299(Fn41::F300, {
            let a2198 = {
                let a2197 = v0;
                length_list(a2197)
            };
            let a2199 = 1u64;
            nat_add(a2198, a2199)?
        }, v1)?;
        h2196
    };
    match m2200.uncons() {
        None => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f278(v0: Fn37, p2204: bool, p2205: List<(Bytes, Adt6)>) -> bool {
    let m2203 = (p2204, p2205);
    {
        let mut m2203 = m2203;
        loop {
            let step = {
                let (v1, v2) = m2203;
                let m2201 = v2;
                match m2201.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt6)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt6)>), bool>(({
                            let c2202: Fn37 = v0.clone();
                            c2202.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Bytes, v1: bool, v2: (Bytes, Adt6)) -> bool {
    if v1 {
        true
    } else {
        let a2207 = {
            let (h2206, _) = v2;
            h2206
        };
        let a2208 = v0;
        equal(a2207, a2208)
    }
}

pub fn f280(v0: Fn38, p2212: (Option<Bytes>, List<Bytes>), p2213: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m2211 = (p2212, p2213);
    {
        let mut m2211 = m2211;
        loop {
            let step = {
                let (v1, v2) = m2211;
                let m2209 = v2;
                match m2209.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c2210: Fn38 = v0.clone();
                            c2210.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f301(v0, v1)
}

pub fn f282(v0: Bytes, v1: u64) -> bool {
    if f130(v0.clone(), v1, 123u8) {
        true
    } else {
        if f130(v0.clone(), v1, 125u8) {
            true
        } else {
            if f130(v0.clone(), v1, 91u8) {
                true
            } else {
                if f130(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f130(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f130(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f130(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f130(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f130(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f130(v0, v1, 34u8)
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

pub fn f283(v0: Bytes, v1: u64) -> R<u64> {
    let m2214 = f57(v0, v1);
    match m2214 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a2259 = {
                let a2235 = {
                    let a2223 = {
                        let a2217 = {
                            let a2215 = v3;
                            let a2216 = 128u8;
                            fixed_u8::bit_and(a2215, a2216)
                        };
                        let a2218 = 128u8;
                        if equal(a2217, a2218) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a2224 = {
                        let a2221 = {
                            let a2219 = v3;
                            let a2220 = 64u8;
                            fixed_u8::bit_and(a2219, a2220)
                        };
                        let a2222 = 64u8;
                        if equal(a2221, a2222) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a2223, a2224)?
                };
                let a2236 = {
                    let a2233 = {
                        let a2227 = {
                            let a2225 = v3;
                            let a2226 = 32u8;
                            fixed_u8::bit_and(a2225, a2226)
                        };
                        let a2228 = 32u8;
                        if equal(a2227, a2228) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a2234 = {
                        let a2231 = {
                            let a2229 = v3;
                            let a2230 = 16u8;
                            fixed_u8::bit_and(a2229, a2230)
                        };
                        let a2232 = 16u8;
                        if equal(a2231, a2232) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a2233, a2234)?
                };
                nat_add(a2235, a2236)?
            };
            let a2260 = {
                let a2257 = {
                    let a2245 = {
                        let a2239 = {
                            let a2237 = v3;
                            let a2238 = 8u8;
                            fixed_u8::bit_and(a2237, a2238)
                        };
                        let a2240 = 8u8;
                        if equal(a2239, a2240) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a2246 = {
                        let a2243 = {
                            let a2241 = v3;
                            let a2242 = 4u8;
                            fixed_u8::bit_and(a2241, a2242)
                        };
                        let a2244 = 4u8;
                        if equal(a2243, a2244) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a2245, a2246)?
                };
                let a2258 = {
                    let a2255 = {
                        let a2249 = {
                            let a2247 = v3;
                            let a2248 = 2u8;
                            fixed_u8::bit_and(a2247, a2248)
                        };
                        let a2250 = 2u8;
                        if equal(a2249, a2250) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a2256 = {
                        let a2253 = {
                            let a2251 = v3;
                            let a2252 = 1u8;
                            fixed_u8::bit_and(a2251, a2252)
                        };
                        let a2254 = 1u8;
                        if equal(a2253, a2254) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a2255, a2256)?
                };
                nat_add(a2257, a2258)?
            };
            nat_add(a2259, a2260)
        }
    }
}

pub fn f284(v0: Bytes, v1: u64) -> R<u64> {
    let (h2261, _) = f55(Fn9::F302(v0.clone()), {
        let a2263 = {
            let a2262 = v0;
            length_bytes(a2262)
        };
        let a2264 = 1u64;
        nat_add(a2263, a2264)?
    }, v1)?;
    Ok(h2261)
}

pub fn f285(v0: Bytes, v1: u64) -> R<u64> {
    let (h2265, _) = f55(Fn9::F303(v0.clone()), {
        let a2267 = {
            let a2266 = v0;
            length_bytes(a2266)
        };
        let a2268 = 1u64;
        nat_add(a2267, a2268)?
    }, v1)?;
    Ok(h2265)
}

pub fn f286(v0: Bytes) -> R<Option<u64>> {
    if f304(v0.clone())? {
        Ok(Some({
            let (_, h2269) = {
                let (h2270, _) = f305(Fn42::F306(v0.clone()), {
                    let a2271 = v0;
                    length_bytes(a2271)
                }, (0u64, 0u64))?;
                h2270
            };
            h2269
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f287(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2272 = 0u64;
        let a2273 = v1;
        if nat_lt(a2272, a2273) {
            let a2274 = v1;
            let a2275 = 9223372036854775808u64;
            nat_le(a2274, a2275)
        } else {
            false
        }
    } else {
        let a2276 = v1;
        let a2277 = 9223372036854775807u64;
        nat_le(a2276, a2277)
    }
}

pub fn f288(v0: u64) -> bool {
    let a2278 = 64u64;
    let a2279 = v0;
    if if nat_lt(a2278, a2279) {
        let a2280 = v0;
        let a2281 = 91u64;
        nat_lt(a2280, a2281)
    } else {
        false
    } {
        true
    } else {
        let a2282 = 96u64;
        let a2283 = v0;
        if if nat_lt(a2282, a2283) {
            let a2284 = v0;
            let a2285 = 123u64;
            nat_lt(a2284, a2285)
        } else {
            false
        } {
            true
        } else {
            let a2286 = v0;
            let a2287 = 95u64;
            nat_eq(a2286, a2287)
        }
    }
}

pub fn f289(v0: Bytes, v1: u64) -> R<u64> {
    let (h2288, _) = f55(Fn9::F307(v0.clone()), {
        let a2290 = {
            let a2289 = v0;
            length_bytes(a2289)
        };
        let a2291 = 1u64;
        nat_add(a2290, a2291)?
    }, v1)?;
    Ok(h2288)
}

pub fn f290(v0: Bytes) -> bool {
    let a2292 = v0.clone();
    let a2293 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a2292, a2293) {
        true
    } else {
        let a2294 = v0;
        let a2295 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a2294, a2295)
    }
}

pub fn f291(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a2296 = v0.clone();
        length_bytes(a2296)
    };
    let v2: Adt41 = {
        let (h2297, _) = f308(Fn43::F309(v1, v0), {
            let a2298 = v1;
            let a2299 = 1u64;
            nat_add(a2298, a2299)?
        }, Adt41::C0(0u64, List::<Bytes>::nil()))?;
        h2297
    };
    Ok(f164(f164(f134(Str::lit("\"")), f220(match v2 {
        Adt41::C0(_, h2300) => {
            h2300
        }
    })), f134(Str::lit("\""))))
}

pub fn f292(v0: Adt6, v1: Adt6) -> Adt6 {
    Adt6::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f293(v0: Fn39, p2304: Adt6, p2305: List<(Bytes, Adt6)>) -> Adt6 {
    let m2303 = (p2304, p2305);
    {
        let mut m2303 = m2303;
        loop {
            let step = {
                let (v1, v2) = m2303;
                let m2301 = v2;
                match m2301.uncons() {
                    None => {
                        Err::<(Adt6, List<(Bytes, Adt6)>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<(Bytes, Adt6)>), Adt6>(({
                            let c2302: Fn39 = v0.clone();
                            c2302.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Adt6, v1: (Bytes, Adt6)) -> Adt6 {
    Adt6::C7({
        let (h2306, _) = v1;
        h2306
    }, std::rc::Rc::new({
        let (_, h2307) = v1;
        h2307
    }), std::rc::Rc::new(v0))
}

pub fn f295(v0: List<(Bytes, Adt6)>) -> List<(Bytes, Adt6)> {
    f310(Fn44::F311, List::<(Bytes, Adt6)>::nil(), v0)
}

pub fn f296(v0: List<List<(Bytes, Adt6)>>) -> List<List<(Bytes, Adt6)>> {
    f312(Fn45::F313, List::<List<(Bytes, Adt6)>>::nil(), v0)
}

pub fn f297(v0: Fn40, p2311: List<List<(Bytes, Adt6)>>, p2312: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
    let m2310 = (p2311, p2312);
    {
        let mut m2310 = m2310;
        loop {
            let step = {
                let (v1, v2) = m2310;
                let m2308 = v2;
                match m2308.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt6)>>, List<(Bytes, Adt6)>), List<List<(Bytes, Adt6)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt6)>>, List<(Bytes, Adt6)>), List<List<(Bytes, Adt6)>>>(({
                            let c2309: Fn40 = v0.clone();
                            c2309.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: List<List<(Bytes, Adt6)>>, v1: (Bytes, Adt6)) -> List<List<(Bytes, Adt6)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt6)>::nil()), v0)
}

pub fn f299(v0: Fn41, p2317: u64, p2318: List<List<(Bytes, Adt6)>>) -> R<(List<List<(Bytes, Adt6)>>, bool)> {
    let m2316 = (p2317, p2318);
    {
        let mut m2316 = m2316;
        loop {
            let step = {
                let (v1, v2) = m2316;
                let m2313 = v1;
                if m2313 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2313 - 1;
                    let m2315 = {
                        let c2314: Fn41 = v0.clone();
                        c2314.apply(v2.clone())?
                    };
                    match m2315 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
    f314(v0)
}

pub fn f301(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m2320 = {
        let (h2319, _) = v0;
        h2319
    };
    match m2320 {
        None => {
            (Some(v1), {
                let (_, h2321) = v0;
                h2321
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a2322 = v2;
                let a2323 = v1;
                append_bytes(a2322, a2323)
            }, {
                let (_, h2324) = v0;
                h2324
            }))
        }
    }
}

pub fn f302(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f315(v0, v1)
}

pub fn f303(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f316(v0, v1)
}

pub fn f304(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2325 = v0.clone();
        length_bytes(a2325)
    };
    let a2326 = 0u64;
    let a2327 = v1;
    if nat_lt(a2326, a2327) {
        let a2328 = f317(v0.clone())?;
        let a2329 = v1;
        if nat_eq(a2328, a2329) {
            let a2330 = 1u64;
            let a2331 = v1;
            if nat_lt(a2330, a2331) {
                let m2334 = {
                    let a2332 = v0;
                    let a2333 = 0u64;
                    index_bytes(a2332, a2333)
                };
                match m2334 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2335 = v2;
                        let a2336 = 48u8;
                        Ok(!equal(a2335, a2336))
                    }
                }
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f305(v0: Fn42, p2341: u64, p2342: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2340 = (p2341, p2342);
    {
        let mut m2340 = m2340;
        loop {
            let step = {
                let (v1, v2) = m2340;
                let m2337 = v1;
                if m2337 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2337 - 1;
                    let m2339 = {
                        let c2338: Fn42 = v0.clone();
                        c2338.apply(v2)?
                    };
                    match m2339 {
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
                Ok(next) => { m2340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f318(v0, v1)
}

pub fn f307(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f319(v0, v1)
}

pub fn f308(v0: Fn43, p2347: u64, p2348: Adt41) -> R<(Adt41, bool)> {
    let m2346 = (p2347, p2348);
    {
        let mut m2346 = m2346;
        loop {
            let step = {
                let (v1, v2) = m2346;
                let m2343 = v1;
                if m2343 == 0 {
                    Err::<(u64, Adt41), R<(Adt41, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2343 - 1;
                    let m2345 = {
                        let c2344: Fn43 = v0.clone();
                        c2344.apply(v2.clone())?
                    };
                    match m2345 {
                        None => {
                            Err::<(u64, Adt41), R<(Adt41, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt41), R<(Adt41, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(v0: u64, v1: Bytes, v2: Adt41) -> R<Option<Adt41>> {
    f320(v1, v0, v2)
}

pub fn f310(v0: Fn44, p2352: List<(Bytes, Adt6)>, p2353: List<(Bytes, Adt6)>) -> List<(Bytes, Adt6)> {
    let m2351 = (p2352, p2353);
    {
        let mut m2351 = m2351;
        loop {
            let step = {
                let (v1, v2) = m2351;
                let m2349 = v2;
                match m2349.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt6)>, List<(Bytes, Adt6)>), List<(Bytes, Adt6)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt6)>, List<(Bytes, Adt6)>), List<(Bytes, Adt6)>>(({
                            let c2350: Fn44 = v0.clone();
                            c2350.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: List<(Bytes, Adt6)>, v1: (Bytes, Adt6)) -> List<(Bytes, Adt6)> {
    List::cons(v1, v0)
}

pub fn f312(v0: Fn45, p2357: List<List<(Bytes, Adt6)>>, p2358: List<List<(Bytes, Adt6)>>) -> List<List<(Bytes, Adt6)>> {
    let m2356 = (p2357, p2358);
    {
        let mut m2356 = m2356;
        loop {
            let step = {
                let (v1, v2) = m2356;
                let m2354 = v2;
                match m2354.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), List<List<(Bytes, Adt6)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), List<List<(Bytes, Adt6)>>>(({
                            let c2355: Fn45 = v0.clone();
                            c2355.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: List<List<(Bytes, Adt6)>>, v1: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
    List::cons(v1, v0)
}

pub fn f314(v0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
    let a2360 = {
        let a2359 = v0.clone();
        length_list(a2359)
    };
    let a2361 = 1u64;
    if nat_le(a2360, a2361) {
        Ok(None::<List<List<(Bytes, Adt6)>>>)
    } else {
        Ok(Some(f321(v0)?))
    }
}

pub fn f315(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f283(v0, v1)?;
    let a2362 = 47u64;
    let a2363 = v2;
    if nat_lt(a2362, a2363) {
        let a2364 = v2;
        let a2365 = 58u64;
        if nat_lt(a2364, a2365) {
            Ok(Some({
                let a2366 = v1;
                let a2367 = 1u64;
                nat_add(a2366, a2367)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f316(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2368 = f283(v0, v1)?;
    let a2369 = 48u64;
    if nat_eq(a2368, a2369) {
        Ok(Some({
            let a2370 = v1;
            let a2371 = 1u64;
            nat_add(a2370, a2371)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f317(v0: Bytes) -> R<u64> {
    let (h2372, _) = f55(Fn9::F322(v0.clone()), {
        let a2373 = v0;
        length_bytes(a2373)
    }, 0u64)?;
    Ok(h2372)
}

pub fn f318(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2377 = {
        let a2375 = v0;
        let a2376 = {
            let (h2374, _) = v1;
            h2374
        };
        index_bytes(a2375, a2376)
    };
    match m2377 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2379 = {
                    let (h2378, _) = v1;
                    h2378
                };
                let a2380 = 1u64;
                nat_add(a2379, a2380)?
            }, {
                let a2384 = {
                    let a2382 = {
                        let (_, h2381) = v1;
                        h2381
                    };
                    let a2383 = 10u64;
                    nat_mul(a2382, a2383)?
                };
                let a2385 = f323(v2)?;
                nat_add(a2384, a2385)?
            })))
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f324(f283(v0, v1)?) {
        Ok(Some({
            let a2386 = v1;
            let a2387 = 1u64;
            nat_add(a2386, a2387)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f320(v0: Bytes, v1: u64, v2: Adt41) -> R<Option<Adt41>> {
    let v3: u64 = match v2.clone() {
        Adt41::C0(h2388, _) => {
            h2388
        }
    };
    let a2389 = v3;
    let a2390 = v1;
    if nat_lt(a2389, a2390) {
        let v4: u64 = f132(v0.clone(), v3)?;
        let a2391 = v4;
        let a2392 = 32u64;
        if if nat_lt(a2391, a2392) {
            true
        } else {
            let a2393 = v4;
            let a2394 = 34u64;
            if nat_eq(a2393, a2394) {
                true
            } else {
                let a2395 = v4;
                let a2396 = 92u64;
                nat_eq(a2395, a2396)
            }
        } {
            Ok(Some(Adt41::C0({
                let a2397 = v3;
                let a2398 = 1u64;
                nat_add(a2397, a2398)?
            }, List::cons(f325(v4), match v2 {
                Adt41::C0(_, h2399) => {
                    h2399
                }
            }))))
        } else {
            let v5: u64 = f135(v0.clone(), v1, v3)?;
            Ok(Some(Adt41::C0(v5, List::cons(f44(v0, v3, {
                let a2400 = v5;
                let a2401 = v3;
                nat_sub(a2400, a2401)
            }), match v2 {
                Adt41::C0(_, h2402) => {
                    h2402
                }
            }))))
        }
    } else {
        Ok(None::<Adt41>)
    }
}

pub fn f321(v0: List<List<(Bytes, Adt6)>>) -> R<List<List<(Bytes, Adt6)>>> {
    Ok(f296({
        let (_, h2403) = {
            let (h2404, _) = f326(Fn46::F327, {
                let a2406 = {
                    let a2405 = v0.clone();
                    length_list(a2405)
                };
                let a2407 = 1u64;
                nat_add(a2406, a2407)?
            }, (v0, List::<List<(Bytes, Adt6)>>::nil()))?;
            h2404
        };
        h2403
    }))
}

pub fn f322(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2410 = {
        let a2408 = v0;
        let a2409 = v1;
        index_bytes(a2408, a2409)
    };
    match m2410 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f328(v2)? {
                Ok(Some({
                    let a2411 = v1;
                    let a2412 = 1u64;
                    nat_add(a2411, a2412)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f323(v0: u8) -> R<u64> {
    let a2459 = {
        let v1: u8 = v0;
        let a2457 = {
            let a2433 = {
                let a2421 = {
                    let a2415 = {
                        let a2413 = v1;
                        let a2414 = 128u8;
                        fixed_u8::bit_and(a2413, a2414)
                    };
                    let a2416 = 128u8;
                    if equal(a2415, a2416) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2422 = {
                    let a2419 = {
                        let a2417 = v1;
                        let a2418 = 64u8;
                        fixed_u8::bit_and(a2417, a2418)
                    };
                    let a2420 = 64u8;
                    if equal(a2419, a2420) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2421, a2422)?
            };
            let a2434 = {
                let a2431 = {
                    let a2425 = {
                        let a2423 = v1;
                        let a2424 = 32u8;
                        fixed_u8::bit_and(a2423, a2424)
                    };
                    let a2426 = 32u8;
                    if equal(a2425, a2426) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2432 = {
                    let a2429 = {
                        let a2427 = v1;
                        let a2428 = 16u8;
                        fixed_u8::bit_and(a2427, a2428)
                    };
                    let a2430 = 16u8;
                    if equal(a2429, a2430) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2431, a2432)?
            };
            nat_add(a2433, a2434)?
        };
        let a2458 = {
            let a2455 = {
                let a2443 = {
                    let a2437 = {
                        let a2435 = v1;
                        let a2436 = 8u8;
                        fixed_u8::bit_and(a2435, a2436)
                    };
                    let a2438 = 8u8;
                    if equal(a2437, a2438) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2444 = {
                    let a2441 = {
                        let a2439 = v1;
                        let a2440 = 4u8;
                        fixed_u8::bit_and(a2439, a2440)
                    };
                    let a2442 = 4u8;
                    if equal(a2441, a2442) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2443, a2444)?
            };
            let a2456 = {
                let a2453 = {
                    let a2447 = {
                        let a2445 = v1;
                        let a2446 = 2u8;
                        fixed_u8::bit_and(a2445, a2446)
                    };
                    let a2448 = 2u8;
                    if equal(a2447, a2448) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2454 = {
                    let a2451 = {
                        let a2449 = v1;
                        let a2450 = 1u8;
                        fixed_u8::bit_and(a2449, a2450)
                    };
                    let a2452 = 1u8;
                    if equal(a2451, a2452) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2453, a2454)?
            };
            nat_add(a2455, a2456)?
        };
        nat_add(a2457, a2458)?
    };
    let a2460 = 48u64;
    Ok(nat_sub(a2459, a2460))
}

pub fn f324(v0: u64) -> bool {
    if f288(v0) {
        true
    } else {
        let a2461 = 47u64;
        let a2462 = v0;
        if if nat_lt(a2461, a2462) {
            let a2463 = v0;
            let a2464 = 58u64;
            nat_lt(a2463, a2464)
        } else {
            false
        } {
            true
        } else {
            let a2465 = v0;
            let a2466 = 46u64;
            if nat_eq(a2465, a2466) {
                true
            } else {
                let a2467 = v0;
                let a2468 = 39u64;
                if nat_eq(a2467, a2468) {
                    true
                } else {
                    let a2469 = v0;
                    let a2470 = 47u64;
                    if nat_eq(a2469, a2470) {
                        true
                    } else {
                        let a2471 = v0;
                        let a2472 = 45u64;
                        nat_eq(a2471, a2472)
                    }
                }
            }
        }
    }
}

pub fn f325(v0: u64) -> Bytes {
    let a2473 = v0;
    let a2474 = 34u64;
    if nat_eq(a2473, a2474) {
        f134(Str::lit("\\\""))
    } else {
        let a2475 = v0;
        let a2476 = 92u64;
        if nat_eq(a2475, a2476) {
            f134(Str::lit("\\\\"))
        } else {
            let a2477 = v0;
            let a2478 = 8u64;
            if nat_eq(a2477, a2478) {
                f134(Str::lit("\\b"))
            } else {
                let a2479 = v0;
                let a2480 = 12u64;
                if nat_eq(a2479, a2480) {
                    f134(Str::lit("\\f"))
                } else {
                    let a2481 = v0;
                    let a2482 = 10u64;
                    if nat_eq(a2481, a2482) {
                        f134(Str::lit("\\n"))
                    } else {
                        let a2483 = v0;
                        let a2484 = 13u64;
                        if nat_eq(a2483, a2484) {
                            f134(Str::lit("\\r"))
                        } else {
                            let a2485 = v0;
                            let a2486 = 9u64;
                            if nat_eq(a2485, a2486) {
                                f134(Str::lit("\\t"))
                            } else {
                                f164(f134(Str::lit("\\u00")), f329(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f326(v0: Fn46, p2491: u64, p2492: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)> {
    let m2490 = (p2491, p2492);
    {
        let mut m2490 = m2490;
        loop {
            let step = {
                let (v1, v2) = m2490;
                let m2487 = v1;
                if m2487 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2487 - 1;
                    let m2489 = {
                        let c2488: Fn46 = v0.clone();
                        c2488.apply(v2.clone())?
                    };
                    match m2489 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
    f330(v0)
}

pub fn f328(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2537 = {
            let a2513 = {
                let a2501 = {
                    let a2495 = {
                        let a2493 = v1;
                        let a2494 = 128u8;
                        fixed_u8::bit_and(a2493, a2494)
                    };
                    let a2496 = 128u8;
                    if equal(a2495, a2496) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2502 = {
                    let a2499 = {
                        let a2497 = v1;
                        let a2498 = 64u8;
                        fixed_u8::bit_and(a2497, a2498)
                    };
                    let a2500 = 64u8;
                    if equal(a2499, a2500) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2501, a2502)?
            };
            let a2514 = {
                let a2511 = {
                    let a2505 = {
                        let a2503 = v1;
                        let a2504 = 32u8;
                        fixed_u8::bit_and(a2503, a2504)
                    };
                    let a2506 = 32u8;
                    if equal(a2505, a2506) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2512 = {
                    let a2509 = {
                        let a2507 = v1;
                        let a2508 = 16u8;
                        fixed_u8::bit_and(a2507, a2508)
                    };
                    let a2510 = 16u8;
                    if equal(a2509, a2510) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2511, a2512)?
            };
            nat_add(a2513, a2514)?
        };
        let a2538 = {
            let a2535 = {
                let a2523 = {
                    let a2517 = {
                        let a2515 = v1;
                        let a2516 = 8u8;
                        fixed_u8::bit_and(a2515, a2516)
                    };
                    let a2518 = 8u8;
                    if equal(a2517, a2518) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2524 = {
                    let a2521 = {
                        let a2519 = v1;
                        let a2520 = 4u8;
                        fixed_u8::bit_and(a2519, a2520)
                    };
                    let a2522 = 4u8;
                    if equal(a2521, a2522) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2523, a2524)?
            };
            let a2536 = {
                let a2533 = {
                    let a2527 = {
                        let a2525 = v1;
                        let a2526 = 2u8;
                        fixed_u8::bit_and(a2525, a2526)
                    };
                    let a2528 = 2u8;
                    if equal(a2527, a2528) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2534 = {
                    let a2531 = {
                        let a2529 = v1;
                        let a2530 = 1u8;
                        fixed_u8::bit_and(a2529, a2530)
                    };
                    let a2532 = 1u8;
                    if equal(a2531, a2532) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2533, a2534)?
            };
            nat_add(a2535, a2536)?
        };
        nat_add(a2537, a2538)?
    };
    let a2539 = 48u64;
    let a2540 = v2;
    if nat_le(a2539, a2540) {
        let a2541 = v2;
        let a2542 = 57u64;
        Ok(nat_le(a2541, a2542))
    } else {
        Ok(false)
    }
}

pub fn f329(v0: u64) -> Bytes {
    f164(f209(v0, 16u64), f210(v0))
}

pub fn f330(v0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
    let v1: List<List<(Bytes, Adt6)>> = {
        let (h2543, _) = v0;
        h2543
    };
    let v2: List<List<(Bytes, Adt6)>> = {
        let (_, h2544) = v0;
        h2544
    };
    let m2545 = v1;
    match m2545.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>)
        }
        Some((v3, v4)) => {
            let m2546 = v4;
            match m2546.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt6)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f331(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f331(v0: List<(Bytes, Adt6)>, v1: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    let v2: u64 = {
        let a2551 = {
            let a2549 = {
                let a2547 = v0.clone();
                length_list(a2547)
            };
            let a2550 = {
                let a2548 = v1.clone();
                length_list(a2548)
            };
            nat_add(a2549, a2550)?
        };
        let a2552 = 1u64;
        nat_add(a2551, a2552)?
    };
    Ok(f295({
        let (_, h2553) = {
            let (_, h2554) = {
                let (h2555, _) = f332(Fn47::F333, v2, (v0, (v1, List::<(Bytes, Adt6)>::nil())));
                h2555
            };
            h2554
        };
        h2553
    }))
}

pub fn f332(v0: Fn47, p2560: u64, p2561: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool) {
    let m2559 = (p2560, p2561);
    {
        let mut m2559 = m2559;
        loop {
            let step = {
                let (v1, v2) = m2559;
                let m2556 = v1;
                if m2556 == 0 {
                    Err::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v2, false))
                } else {
                    let v3 = m2556 - 1;
                    let m2558 = {
                        let c2557: Fn47 = v0.clone();
                        c2557.apply(v2.clone())
                    };
                    match m2558 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333(v0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
    f334(v0)
}

pub fn f334(v0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
    let v1: List<(Bytes, Adt6)> = {
        let (h2562, _) = v0.clone();
        h2562
    };
    let v2: List<(Bytes, Adt6)> = {
        let (h2563, _) = {
            let (_, h2564) = v0.clone();
            h2564
        };
        h2563
    };
    let v3: List<(Bytes, Adt6)> = {
        let (_, h2565) = {
            let (_, h2566) = v0;
            h2566
        };
        h2565
    };
    let m2567 = v1.clone();
    match m2567.uncons() {
        None => {
            let m2568 = v2;
            match m2568.uncons() {
                None => {
                    None::<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt6)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2569 = v2.clone();
            match m2569.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt6)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f110({
                        let (h2570, _) = v8.clone();
                        h2570
                    }, {
                        let (h2571, _) = v6.clone();
                        h2571
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
