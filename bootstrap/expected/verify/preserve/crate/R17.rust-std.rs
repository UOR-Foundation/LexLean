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
    F7,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt0>, p1: (Bytes, Adt0)) -> List<Adt0> {
        match self {
            Fn0::F7 => f7(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F11,
}

impl Fn1 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: Adt0) -> R<List<(Bytes, Adt0)>> {
        match self {
            Fn1::F11 => f11(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F13,
}

impl Fn2 {
    pub fn apply(&self, p0: u64, p1: Adt0) -> R<u64> {
        match self {
            Fn2::F13 => f13(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F15,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn3::F15 => f15(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F20,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (List<Adt0>, List<Adt0>))) -> Option<(u64, (List<Adt0>, List<Adt0>))> {
        match self {
            Fn4::F20 => f20(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F22,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn5::F22 => f22(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F25,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn6::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F27,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn7::F27 => f27(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F33,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn8::F33 => f33(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F44(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn9::F44(k0) => f44(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F47(u64, Bytes),
    F50(u64, Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn10::F47(k0, k1) => Ok(f47(*k0, k1.clone(), p0)),
            Fn10::F50(k0, k1) => f50(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F49,
}

impl Fn11 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn11::F49 => f49(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F52,
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (u64, Bytes))) -> Option<(u64, (u64, Bytes))> {
        match self {
            Fn12::F52 => f52(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F60,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn13::F60 => f60(p0),
        }
    }
}

pub fn f0(v0: List<Adt0>, v1: u64, v2: Bytes) -> R<(u64, List<Adt0>)> {
    let v3: List<Adt0> = f1(v0.clone())?;
    let v4: u64 = {
        let a1 = v0;
        length_list(a1)
    };
    let a2 = v4;
    let a3 = v1;
    if nat_le(a2, a3) {
        Ok((f2(v3.clone(), 70u64)?, v3))
    } else {
        let v5: List<Adt0> = f3(List::cons(f4(v2, v1, v4)?, f3(f5(v3, v1)?)));
        Ok((f2(v5.clone(), 70u64)?, v5))
    }
}

pub fn f1(v0: List<Adt0>) -> R<List<Adt0>> {
    Ok(f6(Fn0::F7, List::<Adt0>::nil(), f8(f9(f8(f10(Fn1::F11, List::<(Bytes, Adt0)>::nil(), v0)?))?)))
}

pub fn f2(v0: List<Adt0>, v1: u64) -> R<u64> {
    let m4 = v0.clone();
    match m4.uncons() {
        None => {
            Ok(v1)
        }
        Some((_, _)) => {
            f12(Fn2::F13, 0u64, v0)
        }
    }
}

pub fn f3(v0: List<Adt0>) -> List<Adt0> {
    f14(Fn3::F15, List::<Adt0>::nil(), v0)
}

pub fn f4(v0: Bytes, v1: u64, v2: u64) -> R<Adt0> {
    Ok(Adt0::C0(f16(Str::lit("LLS8002")), f17(f17(f17(f17(f17(f17(f17(f17(f16(Str::lit("max_diagnostics exceeded in phase ")), v0), f16(Str::lit(": configured "))), f18(v1)?), f16(Str::lit(", observed "))), f18(v2)?), f16(Str::lit("; the first "))), f18(v1)?), f16(Str::lit(" diagnostics are reported"))), None::<Adt1>, List::<Adt2>::nil(), List::<Adt3>::nil(), List::<Bytes>::nil(), List::<Bytes>::nil()))
}

pub fn f5(v0: List<Adt0>, v1: u64) -> R<List<Adt0>> {
    Ok(f3({
        let (_, h5) = {
            let (_, h6) = {
                let (h7, _) = f19(Fn4::F20, {
                    let a8 = v1;
                    let a9 = 1u64;
                    nat_add(a8, a9)?
                }, (v1, (v0, List::<Adt0>::nil())));
                h7
            };
            h6
        };
        h5
    }))
}

pub fn f6(v0: Fn0, p13: List<Adt0>, p14: List<(Bytes, Adt0)>) -> List<Adt0> {
    let m12 = (p13, p14);
    {
        let mut m12 = m12;
        loop {
            let step = {
                let (v1, v2) = m12;
                let m10 = v2;
                match m10.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<(Bytes, Adt0)>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<(Bytes, Adt0)>), List<Adt0>>(({
                            let c11: Fn0 = v0.clone();
                            c11.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m12 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: List<Adt0>, v1: (Bytes, Adt0)) -> List<Adt0> {
    List::cons({
        let (_, h15) = v1;
        h15
    }, v0)
}

pub fn f8(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f21(Fn5::F22, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f9(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f23(f24(Fn6::F25, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m20 = {
        let (h16, _) = f26(Fn7::F27, {
            let a18 = {
                let a17 = v0;
                length_list(a17)
            };
            let a19 = 1u64;
            nat_add(a18, a19)?
        }, v1)?;
        h16
    };
    match m20.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f10(v0: Fn1, p24: List<(Bytes, Adt0)>, p25: List<Adt0>) -> R<List<(Bytes, Adt0)>> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m21 = v2;
                match m21.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<Adt0>), R<List<(Bytes, Adt0)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<Adt0>), R<List<(Bytes, Adt0)>>>(({
                            let c22: Fn1 = v0.clone();
                            c22.apply(v1, v3)?
                        }, v4))
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

pub fn f11(v0: List<(Bytes, Adt0)>, v1: Adt0) -> R<List<(Bytes, Adt0)>> {
    Ok(List::cons(f28(v1)?, v0))
}

pub fn f12(v0: Fn2, p29: u64, p30: List<Adt0>) -> R<u64> {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v1, v2) = m28;
                let m26 = v2;
                match m26.uncons() {
                    None => {
                        Err::<(u64, List<Adt0>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt0>), R<u64>>(({
                            let c27: Fn2 = v0.clone();
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

pub fn f13(v0: u64, v1: Adt0) -> R<u64> {
    let v2: u64 = f29(match v1 {
        Adt0::C0(h31, _, _, _, _, _, _) => {
            h31
        }
    })?;
    let a32 = v0;
    let a33 = v2;
    if nat_lt(a32, a33) {
        Ok(v2)
    } else {
        Ok(v0)
    }
}

pub fn f14(v0: Fn3, p37: List<Adt0>, p38: List<Adt0>) -> List<Adt0> {
    let m36 = (p37, p38);
    {
        let mut m36 = m36;
        loop {
            let step = {
                let (v1, v2) = m36;
                let m34 = v2;
                match m34.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c35: Fn3 = v0.clone();
                            c35.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m36 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f16(v0: Str) -> Bytes {
    let a39 = v0;
    utf8_encode(a39)
}

pub fn f17(v0: Bytes, v1: Bytes) -> Bytes {
    let a40 = v0;
    let a41 = v1;
    append_bytes(a40, a41)
}

pub fn f18(v0: u64) -> R<Bytes> {
    f30(v0)
}

pub fn f19(v0: Fn4, p46: u64, p47: (u64, (List<Adt0>, List<Adt0>))) -> ((u64, (List<Adt0>, List<Adt0>)), bool) {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m42 = v1;
                if m42 == 0 {
                    Err::<(u64, (u64, (List<Adt0>, List<Adt0>))), ((u64, (List<Adt0>, List<Adt0>)), bool)>((v2, false))
                } else {
                    let v3 = m42 - 1;
                    let m44 = {
                        let c43: Fn4 = v0.clone();
                        c43.apply(v2.clone())
                    };
                    match m44 {
                        None => {
                            Err::<(u64, (u64, (List<Adt0>, List<Adt0>))), ((u64, (List<Adt0>, List<Adt0>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (List<Adt0>, List<Adt0>))), ((u64, (List<Adt0>, List<Adt0>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: (u64, (List<Adt0>, List<Adt0>))) -> Option<(u64, (List<Adt0>, List<Adt0>))> {
    f31(v0)
}

pub fn f21(v0: Fn5, p51: List<(Bytes, Adt0)>, p52: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v1, v2) = m50;
                let m48 = v2;
                match m48.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c49: Fn5 = v0.clone();
                            c49.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m50 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f23(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f32(Fn8::F33, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f24(v0: Fn6, p56: List<List<(Bytes, Adt0)>>, p57: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m55 = (p56, p57);
    {
        let mut m55 = m55;
        loop {
            let step = {
                let (v1, v2) = m55;
                let m53 = v2;
                match m53.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c54: Fn6 = v0.clone();
                            c54.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m55 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f26(v0: Fn7, p62: u64, p63: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m61 = (p62, p63);
    {
        let mut m61 = m61;
        loop {
            let step = {
                let (v1, v2) = m61;
                let m58 = v1;
                if m58 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m58 - 1;
                    let m60 = {
                        let c59: Fn7 = v0.clone();
                        c59.apply(v2.clone())?
                    };
                    match m60 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m61 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f34(v0)
}

pub fn f28(v0: Adt0) -> R<(Bytes, Adt0)> {
    Ok((f35(v0.clone())?, v0))
}

pub fn f29(v0: Bytes) -> R<u64> {
    if f36(v0.clone()) {
        Ok(3u64)
    } else {
        let m64 = f37(v0, 2u64);
        match m64 {
            None => {
                Ok(1u64)
            }
            Some(v1) => {
                let v3: u64 = {
                    let v2: u8 = v1;
                    let a109 = {
                        let a85 = {
                            let a73 = {
                                let a67 = {
                                    let a65 = v2;
                                    let a66 = 128u8;
                                    fixed_u8::bit_and(a65, a66)
                                };
                                let a68 = 128u8;
                                if equal(a67, a68) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a74 = {
                                let a71 = {
                                    let a69 = v2;
                                    let a70 = 64u8;
                                    fixed_u8::bit_and(a69, a70)
                                };
                                let a72 = 64u8;
                                if equal(a71, a72) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a73, a74)?
                        };
                        let a86 = {
                            let a83 = {
                                let a77 = {
                                    let a75 = v2;
                                    let a76 = 32u8;
                                    fixed_u8::bit_and(a75, a76)
                                };
                                let a78 = 32u8;
                                if equal(a77, a78) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a84 = {
                                let a81 = {
                                    let a79 = v2;
                                    let a80 = 16u8;
                                    fixed_u8::bit_and(a79, a80)
                                };
                                let a82 = 16u8;
                                if equal(a81, a82) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a83, a84)?
                        };
                        nat_add(a85, a86)?
                    };
                    let a110 = {
                        let a107 = {
                            let a95 = {
                                let a89 = {
                                    let a87 = v2;
                                    let a88 = 8u8;
                                    fixed_u8::bit_and(a87, a88)
                                };
                                let a90 = 8u8;
                                if equal(a89, a90) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a96 = {
                                let a93 = {
                                    let a91 = v2;
                                    let a92 = 4u8;
                                    fixed_u8::bit_and(a91, a92)
                                };
                                let a94 = 4u8;
                                if equal(a93, a94) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a95, a96)?
                        };
                        let a108 = {
                            let a105 = {
                                let a99 = {
                                    let a97 = v2;
                                    let a98 = 2u8;
                                    fixed_u8::bit_and(a97, a98)
                                };
                                let a100 = 2u8;
                                if equal(a99, a100) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a106 = {
                                let a103 = {
                                    let a101 = v2;
                                    let a102 = 1u8;
                                    fixed_u8::bit_and(a101, a102)
                                };
                                let a104 = 1u8;
                                if equal(a103, a104) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a105, a106)?
                        };
                        nat_add(a107, a108)?
                    };
                    nat_add(a109, a110)?
                };
                let a111 = v3;
                let a112 = 67u64;
                if nat_eq(a111, a112) {
                    Ok(2u64)
                } else {
                    let a113 = v3;
                    let a114 = 83u64;
                    if nat_eq(a113, a114) {
                        Ok(4u64)
                    } else {
                        let a115 = v3;
                        let a116 = 73u64;
                        if nat_eq(a115, a116) {
                            Ok(70u64)
                        } else {
                            Ok(1u64)
                        }
                    }
                }
            }
        }
    }
}

pub fn f30(v0: u64) -> R<Bytes> {
    f38(f39(), 10u64, v0)
}

pub fn f31(v0: (u64, (List<Adt0>, List<Adt0>))) -> Option<(u64, (List<Adt0>, List<Adt0>))> {
    let a118 = {
        let (h117, _) = v0.clone();
        h117
    };
    let a119 = 0u64;
    if nat_eq(a118, a119) {
        None::<(u64, (List<Adt0>, List<Adt0>))>
    } else {
        let m122 = {
            let (h120, _) = {
                let (_, h121) = v0.clone();
                h121
            };
            h120
        };
        match m122.uncons() {
            None => {
                None::<(u64, (List<Adt0>, List<Adt0>))>
            }
            Some((v1, v2)) => {
                Some(({
                    let a124 = {
                        let (h123, _) = v0;
                        h123
                    };
                    let a125 = 1u64;
                    nat_sub(a124, a125)
                }, (v2, List::cons(v1, {
                    let (_, h126) = {
                        let (_, h127) = v0;
                        h127
                    };
                    h126
                }))))
            }
        }
    }
}

pub fn f32(v0: Fn8, p131: List<List<(Bytes, Adt0)>>, p132: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m130 = (p131, p132);
    {
        let mut m130 = m130;
        loop {
            let step = {
                let (v1, v2) = m130;
                let m128 = v2;
                match m128.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c129: Fn8 = v0.clone();
                            c129.apply(v1, v3)
                        }, v4))
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

pub fn f33(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f34(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a134 = {
        let a133 = v0.clone();
        length_list(a133)
    };
    let a135 = 1u64;
    if nat_le(a134, a135) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f40(v0)?))
    }
}

pub fn f35(v0: Adt0) -> R<Bytes> {
    let v2: (Bytes, u64) = {
        let m137 = match v0.clone() {
            Adt0::C0(_, _, h136, _, _, _, _) => {
                h136
            }
        };
        match m137 {
            None => {
                (Bytes::lit(&[]), 0u64)
            }
            Some(v1) => {
                (match v1.clone() {
                    Adt1::C0(h138, _, _, _, _, _, _) => {
                        h138
                    }
                }, match v1 {
                    Adt1::C0(_, h139, _, _, _, _, _) => {
                        h139
                    }
                })
            }
        }
    };
    Ok(f17(f17(f17(f41({
        let (h140, _) = v2;
        h140
    })?, f42({
        let (_, h141) = v2;
        h141
    })), match v0.clone() {
        Adt0::C0(h142, _, _, _, _, _, _) => {
            h142
        }
    }), match v0 {
        Adt0::C0(_, h143, _, _, _, _, _) => {
            h143
        }
    }))
}

pub fn f36(v0: Bytes) -> bool {
    f43(Fn9::F44(v0), false, List::cons(f16(Str::lit("LLB6004")), List::cons(f16(Str::lit("LLV7001")), List::cons(f16(Str::lit("LLV7007")), List::cons(f16(Str::lit("LLV7008")), List::cons(f16(Str::lit("LLV7009")), List::cons(f16(Str::lit("LLV7010")), List::cons(f16(Str::lit("LLV7012")), List::cons(f16(Str::lit("LLV7014")), List::cons(f16(Str::lit("LLV7017")), List::cons(f16(Str::lit("LLV7018")), List::<Bytes>::nil())))))))))))
}

pub fn f37(v0: Bytes, v1: u64) -> Option<u8> {
    let a144 = v0;
    let a145 = v1;
    index_bytes(a144, a145)
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a146 = v2;
    let a147 = 0u64;
    if nat_eq(a146, a147) {
        Ok(f45(v0, 0u64))
    } else {
        let (_, h148) = {
            let (h149, _) = f46(Fn10::F47(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h149
        };
        Ok(h148)
    }
}

pub fn f39() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f40(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f23({
        let (_, h150) = {
            let (h151, _) = f48(Fn11::F49, {
                let a153 = {
                    let a152 = v0.clone();
                    length_list(a152)
                };
                let a154 = 1u64;
                nat_add(a153, a154)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h151
        };
        h150
    }))
}

pub fn f41(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a155 = v0.clone();
        length_bytes(a155)
    };
    Ok(f17({
        let (_, h156) = {
            let (h157, _) = f46(Fn10::F50(v1, v0), {
                let a158 = v1;
                let a159 = 1u64;
                nat_add(a158, a159)?
            }, (0u64, Bytes::lit(&[])))?;
            h157
        };
        h156
    }, Bytes::lit(&[0x00, 0x00])))
}

pub fn f42(v0: u64) -> Bytes {
    let (_, h160) = {
        let (_, h161) = {
            let (h162, _) = f51(Fn12::F52, 9u64, (8u64, (v0, Bytes::lit(&[]))));
            h162
        };
        h161
    };
    h160
}

pub fn f43(v0: Fn9, p166: bool, p167: List<Bytes>) -> bool {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v1, v2) = m165;
                let m163 = v2;
                match m163.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c164: Fn9 = v0.clone();
                            c164.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a168 = v0;
        let a169 = v2;
        equal(a168, a169)
    }
}

pub fn f45(v0: Bytes, v1: u64) -> Bytes {
    let m173 = {
        let a170 = v0;
        let a171 = v1;
        let a172 = 1u64;
        slice_bytes(a170, a171, a172)
    };
    match m173 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f46(v0: Fn10, p178: u64, p179: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m177 = (p178, p179);
    {
        let mut m177 = m177;
        loop {
            let step = {
                let (v1, v2) = m177;
                let m174 = v1;
                if m174 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m174 - 1;
                    let m176 = {
                        let c175: Fn10 = v0.clone();
                        c175.apply(v2.clone())?
                    };
                    match m176 {
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
                Ok(next) => { m177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f53(v1, v0, v2)
}

pub fn f48(v0: Fn11, p184: u64, p185: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m183 = (p184, p185);
    {
        let mut m183 = m183;
        loop {
            let step = {
                let (v1, v2) = m183;
                let m180 = v1;
                if m180 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m180 - 1;
                    let m182 = {
                        let c181: Fn11 = v0.clone();
                        c181.apply(v2.clone())?
                    };
                    match m182 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f54(v0)
}

pub fn f50(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f55(v1, v0, v2)
}

pub fn f51(v0: Fn12, p190: u64, p191: (u64, (u64, Bytes))) -> ((u64, (u64, Bytes)), bool) {
    let m189 = (p190, p191);
    {
        let mut m189 = m189;
        loop {
            let step = {
                let (v1, v2) = m189;
                let m186 = v1;
                if m186 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), ((u64, (u64, Bytes)), bool)>((v2, false))
                } else {
                    let v3 = m186 - 1;
                    let m188 = {
                        let c187: Fn12 = v0.clone();
                        c187.apply(v2.clone())
                    };
                    match m188 {
                        None => {
                            Err::<(u64, (u64, (u64, Bytes))), ((u64, (u64, Bytes)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, Bytes))), ((u64, (u64, Bytes)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m189 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: (u64, (u64, Bytes))) -> Option<(u64, (u64, Bytes))> {
    f56(v0)
}

pub fn f53(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h192, _) = v2;
        h192
    };
    let a193 = v3;
    let a194 = 0u64;
    if nat_eq(a193, a194) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a195 = v3;
            let a196 = v1;
            let a197 = 0u64;
            nat_quot(a195, a196, a197)
        }, {
            let a202 = f45(v0, {
                let a198 = v3;
                let a199 = v1;
                let a200 = 0u64;
                nat_rem(a198, a199, a200)
            });
            let a203 = {
                let (_, h201) = v2;
                h201
            };
            append_bytes(a202, a203)
        }))
    }
}

pub fn f54(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h204, _) = v0;
        h204
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h205) = v0;
        h205
    };
    let m206 = v1;
    match m206.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m207 = v4;
            match m207.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f57(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f55(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v3: u64 = {
        let (h208, _) = v2;
        h208
    };
    let a209 = v3;
    let a210 = v1;
    if nat_lt(a209, a210) {
        let m211 = f37(v0, v3);
        match m211 {
            None => {
                Ok(None::<(u64, Bytes)>)
            }
            Some(v4) => {
                let a212 = v4;
                let a213 = 0u8;
                if equal(a212, a213) {
                    Ok(Some(({
                        let a214 = v3;
                        let a215 = 1u64;
                        nat_add(a214, a215)?
                    }, f17({
                        let (_, h216) = v2;
                        h216
                    }, Bytes::lit(&[0x00, 0x01])))))
                } else {
                    Ok(Some(({
                        let a217 = v3;
                        let a218 = 1u64;
                        nat_add(a217, a218)?
                    }, f17({
                        let (_, h219) = v2;
                        h219
                    }, f58({
                        let v5: u8 = v4;
                        let a264 = {
                            let a240 = {
                                let a228 = {
                                    let a222 = {
                                        let a220 = v5;
                                        let a221 = 128u8;
                                        fixed_u8::bit_and(a220, a221)
                                    };
                                    let a223 = 128u8;
                                    if equal(a222, a223) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a229 = {
                                    let a226 = {
                                        let a224 = v5;
                                        let a225 = 64u8;
                                        fixed_u8::bit_and(a224, a225)
                                    };
                                    let a227 = 64u8;
                                    if equal(a226, a227) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a228, a229)?
                            };
                            let a241 = {
                                let a238 = {
                                    let a232 = {
                                        let a230 = v5;
                                        let a231 = 32u8;
                                        fixed_u8::bit_and(a230, a231)
                                    };
                                    let a233 = 32u8;
                                    if equal(a232, a233) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a239 = {
                                    let a236 = {
                                        let a234 = v5;
                                        let a235 = 16u8;
                                        fixed_u8::bit_and(a234, a235)
                                    };
                                    let a237 = 16u8;
                                    if equal(a236, a237) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a238, a239)?
                            };
                            nat_add(a240, a241)?
                        };
                        let a265 = {
                            let a262 = {
                                let a250 = {
                                    let a244 = {
                                        let a242 = v5;
                                        let a243 = 8u8;
                                        fixed_u8::bit_and(a242, a243)
                                    };
                                    let a245 = 8u8;
                                    if equal(a244, a245) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a251 = {
                                    let a248 = {
                                        let a246 = v5;
                                        let a247 = 4u8;
                                        fixed_u8::bit_and(a246, a247)
                                    };
                                    let a249 = 4u8;
                                    if equal(a248, a249) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a250, a251)?
                            };
                            let a263 = {
                                let a260 = {
                                    let a254 = {
                                        let a252 = v5;
                                        let a253 = 2u8;
                                        fixed_u8::bit_and(a252, a253)
                                    };
                                    let a255 = 2u8;
                                    if equal(a254, a255) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a261 = {
                                    let a258 = {
                                        let a256 = v5;
                                        let a257 = 1u8;
                                        fixed_u8::bit_and(a256, a257)
                                    };
                                    let a259 = 1u8;
                                    if equal(a258, a259) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a260, a261)?
                            };
                            nat_add(a262, a263)?
                        };
                        nat_add(a264, a265)?
                    })))))
                }
            }
        }
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f56(v0: (u64, (u64, Bytes))) -> Option<(u64, (u64, Bytes))> {
    let a267 = {
        let (h266, _) = v0.clone();
        h266
    };
    let a268 = 0u64;
    if nat_eq(a267, a268) {
        None::<(u64, (u64, Bytes))>
    } else {
        Some(({
            let a270 = {
                let (h269, _) = v0.clone();
                h269
            };
            let a271 = 1u64;
            nat_sub(a270, a271)
        }, ({
            let a274 = {
                let (h272, _) = {
                    let (_, h273) = v0.clone();
                    h273
                };
                h272
            };
            let a275 = 256u64;
            let a276 = 0u64;
            nat_quot(a274, a275, a276)
        }, f17(f58({
            let a279 = {
                let (h277, _) = {
                    let (_, h278) = v0.clone();
                    h278
                };
                h277
            };
            let a280 = 256u64;
            let a281 = 0u64;
            nat_rem(a279, a280, a281)
        }), {
            let (_, h282) = {
                let (_, h283) = v0;
                h283
            };
            h282
        }))))
    }
}

pub fn f57(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a288 = {
            let a286 = {
                let a284 = v0.clone();
                length_list(a284)
            };
            let a287 = {
                let a285 = v1.clone();
                length_list(a285)
            };
            nat_add(a286, a287)?
        };
        let a289 = 1u64;
        nat_add(a288, a289)?
    };
    Ok(f8({
        let (_, h290) = {
            let (_, h291) = {
                let (h292, _) = f59(Fn13::F60, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h292
            };
            h291
        };
        h290
    }))
}

pub fn f58(v0: u64) -> Bytes {
    f45(f61(), v0)
}

pub fn f59(v0: Fn13, p297: u64, p298: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v1, v2) = m296;
                let m293 = v1;
                if m293 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m293 - 1;
                    let m295 = {
                        let c294: Fn13 = v0.clone();
                        c294.apply(v2.clone())
                    };
                    match m295 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v3, v4))
                        }
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

pub fn f60(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f62(v0)
}

pub fn f61() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f62(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h299, _) = v0.clone();
        h299
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h300, _) = {
            let (_, h301) = v0.clone();
            h301
        };
        h300
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h302) = {
            let (_, h303) = v0;
            h303
        };
        h302
    };
    let m304 = v1.clone();
    match m304.uncons() {
        None => {
            let m305 = v2;
            match m305.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m306 = v2.clone();
            match m306.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f63({
                        let (h307, _) = v8.clone();
                        h307
                    }, {
                        let (h308, _) = v6.clone();
                        h308
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

pub fn f63(v0: Bytes, v1: Bytes) -> bool {
    let a313 = {
        let a309 = v0;
        let a310 = v1;
        compare_bytes(a309, a310)
    };
    let a314 = {
        let a311 = Bytes::lit(&[]);
        let a312 = Bytes::lit(&[0x01]);
        compare_bytes(a311, a312)
    };
    equal(a313, a314)
}
