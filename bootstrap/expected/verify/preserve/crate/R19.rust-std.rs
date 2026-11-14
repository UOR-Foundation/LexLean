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
    C0(Bytes, Bytes, Bytes),
    C1(Bytes),
    C2(u64),
    C3(std::rc::Rc<Adt0>),
    C4,
    C5(Bytes),
    C6(std::rc::Rc<Adt0>),
    C7(std::rc::Rc<Adt0>),
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(Bytes),
    C13,
    C14(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, bool, List<Adt2>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes),
    C1(std::rc::Rc<Adt4>),
    C2,
    C3(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, List<List<Adt4>>, Option<Adt4>, Option<Adt1>, bool),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(List<Adt7>, List<Adt0>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt4),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn0::F5(k0, k1) => f5(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9(u64, List<Adt2>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn1::F9(k0, k1) => f9(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F13(u64),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn2::F13(k0) => f13(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F16,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn3::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F43,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt4, p1: Adt4) -> Adt4 {
        match self {
            Fn4::F43 => f43(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F46,
}

impl Fn5 {
    pub fn apply(&self, p0: (Adt4, List<Adt4>)) -> Option<(Adt4, List<Adt4>)> {
        match self {
            Fn5::F46 => f46(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F55,
}

impl Fn6 {
    pub fn apply(&self, p0: (List<Adt0>, List<Adt0>)) -> Option<(List<Adt0>, List<Adt0>)> {
        match self {
            Fn6::F55 => f55(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F57,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn7::F57 => f57(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F61(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn8::F61(k0, k1) => f61(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F63,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn9::F63 => f63(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F73,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt4) -> List<Adt7> {
        match self {
            Fn10::F73 => f73(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F75,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn11::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F82(u64, Bytes),
    F84(u64, Bytes),
    F87(Bytes, u64, Bytes, u64),
    F92(u64, Bytes),
    F104(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn12::F82(k0, k1) => f82(*k0, k1.clone(), p0),
            Fn12::F84(k0, k1) => f84(*k0, k1.clone(), p0),
            Fn12::F87(k0, k1, k2, k3) => f87(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn12::F92(k0, k1) => f92(*k0, k1.clone(), p0),
            Fn12::F104(k0) => f104(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F95(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn13::F95(k0) => f95(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F97(bool, u64, Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn14::F97(k0, k1, k2) => f97(*k0, *k1, k2.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: u64) -> R<Result<Adt0, Adt1>> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v3) => {
            let m2 = f2(v3.clone(), v1)?;
            match m2 {
                Ok(v5) => {
                    f3(v5, {
                        let a3 = v3;
                        length_list(a3)
                    })
                }
                Err(v4) => {
                    Ok(Err::<Adt0, Adt1>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt0, Adt1>(v2))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt2>, Adt1>> {
    let v1: u64 = {
        let a4 = v0.clone();
        length_bytes(a4)
    };
    let v2: Adt3 = {
        let (h5, _) = f4(Fn0::F5(v1, v0.clone()), {
            let a6 = v1;
            let a7 = 1u64;
            nat_add(a6, a7)?
        }, Adt3::C0(0u64, 0u64, false, List::<Adt2>::nil(), None::<Adt1>))?;
        h5
    };
    let m9 = match v2.clone() {
        Adt3::C0(_, _, _, _, h8) => {
            h8
        }
    };
    match m9 {
        None => {
            Ok(Ok::<List<Adt2>, Adt1>(f6(f7(v0, v2))))
        }
        Some(v3) => {
            Ok(Err::<List<Adt2>, Adt1>(v3))
        }
    }
}

pub fn f2(v0: List<Adt2>, v1: u64) -> R<Result<Adt4, Adt1>> {
    let v2: Adt5 = {
        let (h10, _) = f8(Fn1::F9(v1, v0.clone()), {
            let a12 = {
                let a11 = v0;
                length_list(a11)
            };
            let a13 = 2u64;
            nat_add(a12, a13)?
        }, Adt5::C0(0u64, List::<List<Adt4>>::nil(), None::<Adt4>, None::<Adt1>, false))?;
        h10
    };
    let m15 = match v2.clone() {
        Adt5::C0(_, _, _, h14, _) => {
            h14
        }
    };
    match m15 {
        None => {
            let m17 = match v2 {
                Adt5::C0(_, _, h16, _, _) => {
                    h16
                }
            };
            match m17 {
                None => {
                    Ok(Err::<Adt4, Adt1>(f10(f11(Str::lit("unexpected end of expression")))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt4, Adt1>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt4, Adt1>(v3))
        }
    }
}

pub fn f3(v0: Adt4, v1: u64) -> R<Result<Adt0, Adt1>> {
    let v2: Adt6 = {
        let (h18, _) = f12(Fn2::F13(v1), {
            let a21 = {
                let a19 = v1;
                let a20 = 2u64;
                nat_mul(a19, a20)?
            };
            let a22 = 2u64;
            nat_add(a21, a22)?
        }, Adt6::C0(List::cons(Adt7::C0(v0), List::<Adt7>::nil()), List::<Adt0>::nil(), None::<Adt1>))?;
        h18
    };
    let m24 = match v2.clone() {
        Adt6::C0(_, _, h23) => {
            h23
        }
    };
    match m24 {
        None => {
            let m26 = match v2 {
                Adt6::C0(_, h25, _) => {
                    h25
                }
            };
            match m26.uncons() {
                None => {
                    Ok(Err::<Adt0, Adt1>(f10(f11(Str::lit("malformed render expression")))))
                }
                Some((v4, v5)) => {
                    let m27 = v5;
                    match m27.uncons() {
                        None => {
                            Ok(Ok::<Adt0, Adt1>(v4))
                        }
                        Some((_, _)) => {
                            Ok(Err::<Adt0, Adt1>(f10(f11(Str::lit("malformed render expression")))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt0, Adt1>(v3))
        }
    }
}

pub fn f4(v0: Fn0, p32: u64, p33: Adt3) -> R<(Adt3, bool)> {
    let m31 = (p32, p33);
    {
        let mut m31 = m31;
        loop {
            let step = {
                let (v1, v2) = m31;
                let m28 = v1;
                if m28 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m28 - 1;
                    let m30 = {
                        let c29: Fn0 = v0.clone();
                        c29.apply(v2.clone())?
                    };
                    match m30 {
                        None => {
                            Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), R<(Adt3, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m31 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: u64, v1: Bytes, v2: Adt3) -> R<Option<Adt3>> {
    f14(v1, v0, v2)
}

pub fn f6(v0: List<Adt2>) -> List<Adt2> {
    f15(Fn3::F16, List::<Adt2>::nil(), v0)
}

pub fn f7(v0: Bytes, v1: Adt3) -> List<Adt2> {
    if match v1.clone() {
        Adt3::C0(_, _, h34, _, _) => {
            h34
        }
    } {
        List::cons(Adt2::C2(f17(v0, match v1.clone() {
            Adt3::C0(_, h35, _, _, _) => {
                h35
            }
        }, {
            let a38 = match v1.clone() {
                Adt3::C0(h36, _, _, _, _) => {
                    h36
                }
            };
            let a39 = match v1.clone() {
                Adt3::C0(_, h37, _, _, _) => {
                    h37
                }
            };
            nat_sub(a38, a39)
        })), match v1 {
            Adt3::C0(_, _, _, h40, _) => {
                h40
            }
        })
    } else {
        match v1 {
            Adt3::C0(_, _, _, h41, _) => {
                h41
            }
        }
    }
}

pub fn f8(v0: Fn1, p46: u64, p47: Adt5) -> R<(Adt5, bool)> {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m42 = v1;
                if m42 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m42 - 1;
                    let m44 = {
                        let c43: Fn1 = v0.clone();
                        c43.apply(v2.clone())?
                    };
                    match m44 {
                        None => {
                            Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), R<(Adt5, bool)>>((v3, v4))
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

pub fn f9(v0: u64, v1: List<Adt2>, v2: Adt5) -> R<Option<Adt5>> {
    f18(v1, v0, v2)
}

pub fn f10(v0: Bytes) -> Adt1 {
    Adt1::C0(0u64, v0)
}

pub fn f11(v0: Str) -> Bytes {
    let a48 = v0;
    utf8_encode(a48)
}

pub fn f12(v0: Fn2, p53: u64, p54: Adt6) -> R<(Adt6, bool)> {
    let m52 = (p53, p54);
    {
        let mut m52 = m52;
        loop {
            let step = {
                let (v1, v2) = m52;
                let m49 = v1;
                if m49 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m49 - 1;
                    let m51 = {
                        let c50: Fn2 = v0.clone();
                        c50.apply(v2.clone())?
                    };
                    match m51 {
                        None => {
                            Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), R<(Adt6, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m52 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: u64, v1: Adt6) -> R<Option<Adt6>> {
    f19(v0, v1)
}

pub fn f14(v0: Bytes, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let m56 = match v2.clone() {
        Adt3::C0(_, _, _, _, h55) => {
            h55
        }
    };
    match m56 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt3::C0(h57, _, _, _, _) => {
                    h57
                }
            };
            let a58 = v4;
            let a59 = v1;
            if nat_lt(a58, a59) {
                let v5: u64 = f20(v0.clone(), v4)?;
                let a60 = v5;
                let a61 = 40u64;
                if nat_eq(a60, a61) {
                    Ok(Some(Adt3::C0({
                        let a62 = v4;
                        let a63 = 1u64;
                        nat_add(a62, a63)?
                    }, match v2.clone() {
                        Adt3::C0(_, h64, _, _, _) => {
                            h64
                        }
                    }, false, List::cons(Adt2::C0, f7(v0, v2)), None::<Adt1>)))
                } else {
                    let a65 = v5;
                    let a66 = 41u64;
                    if nat_eq(a65, a66) {
                        Ok(Some(Adt3::C0({
                            let a67 = v4;
                            let a68 = 1u64;
                            nat_add(a67, a68)?
                        }, match v2.clone() {
                            Adt3::C0(_, h69, _, _, _) => {
                                h69
                            }
                        }, false, List::cons(Adt2::C1, f7(v0, v2)), None::<Adt1>)))
                    } else {
                        let a70 = v5;
                        let a71 = 32u64;
                        if if nat_eq(a70, a71) {
                            true
                        } else {
                            let a72 = v5;
                            let a73 = 10u64;
                            nat_eq(a72, a73)
                        } {
                            Ok(Some(Adt3::C0({
                                let a74 = v4;
                                let a75 = 1u64;
                                nat_add(a74, a75)?
                            }, match v2.clone() {
                                Adt3::C0(_, h76, _, _, _) => {
                                    h76
                                }
                            }, false, f7(v0, v2), None::<Adt1>)))
                        } else {
                            let a77 = v5;
                            let a78 = 32u64;
                            if if nat_lt(a77, a78) {
                                true
                            } else {
                                let a79 = v5;
                                let a80 = 127u64;
                                nat_eq(a79, a80)
                            } {
                                Ok(Some(Adt3::C0(v4, match v2.clone() {
                                    Adt3::C0(_, h81, _, _, _) => {
                                        h81
                                    }
                                }, match v2.clone() {
                                    Adt3::C0(_, _, h82, _, _) => {
                                        h82
                                    }
                                }, match v2 {
                                    Adt3::C0(_, _, _, h83, _) => {
                                        h83
                                    }
                                }, Some(f10(f11(Str::lit("whitespace is one or more ASCII spaces or LF")))))))
                            } else {
                                if match v2.clone() {
                                    Adt3::C0(_, _, h84, _, _) => {
                                        h84
                                    }
                                } {
                                    Ok(Some(Adt3::C0({
                                        let a85 = v4;
                                        let a86 = 1u64;
                                        nat_add(a85, a86)?
                                    }, match v2.clone() {
                                        Adt3::C0(_, h87, _, _, _) => {
                                            h87
                                        }
                                    }, true, match v2 {
                                        Adt3::C0(_, _, _, h88, _) => {
                                            h88
                                        }
                                    }, None::<Adt1>)))
                                } else {
                                    Ok(Some(Adt3::C0({
                                        let a89 = v4;
                                        let a90 = 1u64;
                                        nat_add(a89, a90)?
                                    }, v4, true, match v2 {
                                        Adt3::C0(_, _, _, h91, _) => {
                                            h91
                                        }
                                    }, None::<Adt1>)))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(None::<Adt3>)
            }
        }
        Some(_) => {
            Ok(None::<Adt3>)
        }
    }
}

pub fn f15(v0: Fn3, p95: List<Adt2>, p96: List<Adt2>) -> List<Adt2> {
    let m94 = (p95, p96);
    {
        let mut m94 = m94;
        loop {
            let step = {
                let (v1, v2) = m94;
                let m92 = v2;
                match m92.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c93: Fn3 = v0.clone();
                            c93.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m94 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f17(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m100 = {
        let a97 = v0;
        let a98 = v1;
        let a99 = v2;
        slice_bytes(a97, a98, a99)
    };
    match m100 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f18(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let m102 = match v2.clone() {
        Adt5::C0(_, _, _, h101, _) => {
            h101
        }
    };
    match m102 {
        None => {
            if match v2.clone() {
                Adt5::C0(_, _, _, _, h103) => {
                    h103
                }
            } {
                Ok(None::<Adt5>)
            } else {
                let m107 = {
                    let a105 = v0;
                    let a106 = match v2.clone() {
                        Adt5::C0(h104, _, _, _, _) => {
                            h104
                        }
                    };
                    index_list(a105, a106)
                };
                match m107 {
                    None => {
                        let m109 = match v2.clone() {
                            Adt5::C0(_, h108, _, _, _) => {
                                h108
                            }
                        };
                        match m109.uncons() {
                            None => {
                                let m111 = match v2.clone() {
                                    Adt5::C0(_, _, h110, _, _) => {
                                        h110
                                    }
                                };
                                match m111 {
                                    None => {
                                        Ok(Some(f21(v2, f10(f11(Str::lit("unexpected end of expression"))))))
                                    }
                                    Some(_) => {
                                        Ok(Some(Adt5::C0(match v2.clone() {
                                            Adt5::C0(h112, _, _, _, _) => {
                                                h112
                                            }
                                        }, List::<List<Adt4>>::nil(), match v2 {
                                            Adt5::C0(_, _, h113, _, _) => {
                                                h113
                                            }
                                        }, None::<Adt1>, true)))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                Ok(Some(f21(v2, f10(f11(Str::lit("unexpected end of expression"))))))
                            }
                        }
                    }
                    Some(v7) => {
                        let m115 = match v2.clone() {
                            Adt5::C0(_, _, h114, _, _) => {
                                h114
                            }
                        };
                        match m115 {
                            None => {
                                Ok(Some(f22(v1, v2, v7)?))
                            }
                            Some(_) => {
                                Ok(Some(f21(v2, f10(f11(Str::lit("trailing tokens after the expression"))))))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f19(v0: u64, v1: Adt6) -> R<Option<Adt6>> {
    let m117 = match v1.clone() {
        Adt6::C0(_, _, h116) => {
            h116
        }
    };
    match m117 {
        None => {
            let m119 = match v1.clone() {
                Adt6::C0(h118, _, _) => {
                    h118
                }
            };
            match m119.uncons() {
                None => {
                    Ok(None::<Adt6>)
                }
                Some((v3, v4)) => {
                    let m120 = v3;
                    match m120 {
                        Adt7::C0(v5) => {
                            Ok(Some(f23(v1, v4, v5, v0)?))
                        }
                        Adt7::C1(v6, v7) => {
                            Ok(Some(f24(v1, v4, v6, v7)))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt6>)
        }
    }
}

pub fn f20(v0: Bytes, v1: u64) -> R<u64> {
    let m121 = f25(v0, v1);
    match m121 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a166 = {
                let a142 = {
                    let a130 = {
                        let a124 = {
                            let a122 = v3;
                            let a123 = 128u8;
                            fixed_u8::bit_and(a122, a123)
                        };
                        let a125 = 128u8;
                        if equal(a124, a125) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a131 = {
                        let a128 = {
                            let a126 = v3;
                            let a127 = 64u8;
                            fixed_u8::bit_and(a126, a127)
                        };
                        let a129 = 64u8;
                        if equal(a128, a129) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a130, a131)?
                };
                let a143 = {
                    let a140 = {
                        let a134 = {
                            let a132 = v3;
                            let a133 = 32u8;
                            fixed_u8::bit_and(a132, a133)
                        };
                        let a135 = 32u8;
                        if equal(a134, a135) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a141 = {
                        let a138 = {
                            let a136 = v3;
                            let a137 = 16u8;
                            fixed_u8::bit_and(a136, a137)
                        };
                        let a139 = 16u8;
                        if equal(a138, a139) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a140, a141)?
                };
                nat_add(a142, a143)?
            };
            let a167 = {
                let a164 = {
                    let a152 = {
                        let a146 = {
                            let a144 = v3;
                            let a145 = 8u8;
                            fixed_u8::bit_and(a144, a145)
                        };
                        let a147 = 8u8;
                        if equal(a146, a147) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a153 = {
                        let a150 = {
                            let a148 = v3;
                            let a149 = 4u8;
                            fixed_u8::bit_and(a148, a149)
                        };
                        let a151 = 4u8;
                        if equal(a150, a151) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a152, a153)?
                };
                let a165 = {
                    let a162 = {
                        let a156 = {
                            let a154 = v3;
                            let a155 = 2u8;
                            fixed_u8::bit_and(a154, a155)
                        };
                        let a157 = 2u8;
                        if equal(a156, a157) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a163 = {
                        let a160 = {
                            let a158 = v3;
                            let a159 = 1u8;
                            fixed_u8::bit_and(a158, a159)
                        };
                        let a161 = 1u8;
                        if equal(a160, a161) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a162, a163)?
                };
                nat_add(a164, a165)?
            };
            nat_add(a166, a167)
        }
    }
}

pub fn f21(v0: Adt5, v1: Adt1) -> Adt5 {
    Adt5::C0(match v0.clone() {
        Adt5::C0(h168, _, _, _, _) => {
            h168
        }
    }, match v0.clone() {
        Adt5::C0(_, h169, _, _, _) => {
            h169
        }
    }, match v0 {
        Adt5::C0(_, _, h170, _, _) => {
            h170
        }
    }, Some(v1), false)
}

pub fn f22(v0: u64, v1: Adt5, v2: Adt2) -> R<Adt5> {
    let m171 = v2;
    match m171 {
        Adt2::C0 => {
            let a174 = {
                let a173 = match v1.clone() {
                    Adt5::C0(_, h172, _, _, _) => {
                        h172
                    }
                };
                length_list(a173)
            };
            let a175 = v0;
            if nat_lt(a174, a175) {
                Ok(Adt5::C0({
                    let a177 = match v1.clone() {
                        Adt5::C0(h176, _, _, _, _) => {
                            h176
                        }
                    };
                    let a178 = 1u64;
                    nat_add(a177, a178)?
                }, List::cons(List::<Adt4>::nil(), match v1.clone() {
                    Adt5::C0(_, h179, _, _, _) => {
                        h179
                    }
                }), match v1 {
                    Adt5::C0(_, _, h180, _, _) => {
                        h180
                    }
                }, None::<Adt1>, false))
            } else {
                Ok(f21(v1, Adt1::C0(1u64, f26(f11(Str::lit("max_scope_depth exceeded: configured ")), f27(v0)))))
            }
        }
        Adt2::C1 => {
            let m182 = match v1.clone() {
                Adt5::C0(_, h181, _, _, _) => {
                    h181
                }
            };
            match m182.uncons() {
                None => {
                    Ok(f21(v1, f10(f11(Str::lit("unbalanced `)`")))))
                }
                Some((v3, v4)) => {
                    f28(v1, Adt4::C1(std::rc::Rc::new(f29(v3))), v4)
                }
            }
        }
        Adt2::C2(v5) => {
            f28(v1.clone(), Adt4::C0(v5), match v1 {
                Adt5::C0(_, h183, _, _, _) => {
                    h183
                }
            })
        }
    }
}

pub fn f23(v0: Adt6, v1: List<Adt7>, v2: Adt4, v3: u64) -> R<Adt6> {
    let m184 = v2.clone();
    match m184 {
        Adt4::C0(_) => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
        Adt4::C1(r187) => {
            let v4 = (*r187).clone();
            let m185 = f30(v4, v3)?;
            match m185.uncons() {
                None => {
                    Ok(f31(v0, f10(f11(Str::lit("expected an LRE form")))))
                }
                Some((v5, v6)) => {
                    let m186 = v5;
                    match m186 {
                        Adt4::C0(v7) => {
                            f32(v0, v1, v7, v6, v3)
                        }
                        Adt4::C1(_) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LRE form")))))
                        }
                        Adt4::C2 => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LRE form")))))
                        }
                        Adt4::C3(_, _) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LRE form")))))
                        }
                    }
                }
            }
        }
        Adt4::C2 => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
        Adt4::C3(_, _) => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
    }
}

pub fn f24(v0: Adt6, v1: List<Adt7>, v2: Bytes, v3: u64) -> Adt6 {
    let v4: (List<Adt0>, List<Adt0>) = f34(match v0.clone() {
        Adt6::C0(_, h188, _) => {
            h188
        }
    }, v3);
    let v5: List<Adt0> = {
        let (h189, _) = v4;
        h189
    };
    let a190 = v2.clone();
    let a191 = f11(Str::lit("seq"));
    if equal(a190, a191) {
        f35(Adt6::C0(v1.clone(), {
            let (_, h192) = v4;
            h192
        }, None::<Adt1>), v1, Adt0::C3(std::rc::Rc::new(f36(v5))))
    } else {
        if f37(v2.clone()) {
            let m193 = v5;
            match m193.uncons() {
                None => {
                    f31(v0, f10(f11(Str::lit("missing operand"))))
                }
                Some((v6, _)) => {
                    f35(Adt6::C0(v1.clone(), {
                        let (_, h194) = v4;
                        h194
                    }, None::<Adt1>), v1, f38(v2, v6))
                }
            }
        } else {
            let m195 = v5;
            match m195.uncons() {
                None => {
                    f31(v0, f10(f11(Str::lit("missing operand"))))
                }
                Some((v8, v9)) => {
                    let m196 = v9;
                    match m196.uncons() {
                        None => {
                            f31(v0, f10(f11(Str::lit("missing operand"))))
                        }
                        Some((v10, _)) => {
                            f35(Adt6::C0(v1.clone(), {
                                let (_, h197) = v4;
                                h197
                            }, None::<Adt1>), v1, f39(v2, v8, v10))
                        }
                    }
                }
            }
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> Option<u8> {
    let a198 = v0;
    let a199 = v1;
    index_bytes(a198, a199)
}

pub fn f26(v0: Bytes, v1: Bytes) -> Bytes {
    let a200 = v0;
    let a201 = v1;
    append_bytes(a200, a201)
}

pub fn f27(v0: u64) -> Bytes {
    f40(f41(), 10u64, v0)
}

pub fn f28(v0: Adt5, v1: Adt4, v2: List<List<Adt4>>) -> R<Adt5> {
    let m202 = v2;
    match m202.uncons() {
        None => {
            Ok(Adt5::C0({
                let a204 = match v0 {
                    Adt5::C0(h203, _, _, _, _) => {
                        h203
                    }
                };
                let a205 = 1u64;
                nat_add(a204, a205)?
            }, List::<List<Adt4>>::nil(), Some(v1), None::<Adt1>, false))
        }
        Some((v3, v4)) => {
            Ok(Adt5::C0({
                let a207 = match v0.clone() {
                    Adt5::C0(h206, _, _, _, _) => {
                        h206
                    }
                };
                let a208 = 1u64;
                nat_add(a207, a208)?
            }, List::cons(List::cons(v1, v3), v4), match v0 {
                Adt5::C0(_, _, h209, _, _) => {
                    h209
                }
            }, None::<Adt1>, false))
        }
    }
}

pub fn f29(v0: List<Adt4>) -> Adt4 {
    f42(Fn4::F43, Adt4::C2, v0)
}

pub fn f30(v0: Adt4, v1: u64) -> R<List<Adt4>> {
    Ok(f44({
        let (_, h210) = {
            let (h211, _) = f45(Fn5::F46, {
                let a212 = v1;
                let a213 = 1u64;
                nat_add(a212, a213)?
            }, (v0, List::<Adt4>::nil()));
            h211
        };
        h210
    }))
}

pub fn f31(v0: Adt6, v1: Adt1) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h214, _, _) => {
            h214
        }
    }, match v0 {
        Adt6::C0(_, h215, _) => {
            h215
        }
    }, Some(v1))
}

pub fn f32(v0: Adt6, v1: List<Adt7>, v2: Bytes, v3: List<Adt4>, v4: u64) -> R<Adt6> {
    let v5: u64 = {
        let a216 = v3.clone();
        length_list(a216)
    };
    let a217 = v2.clone();
    let a218 = f11(Str::lit("form"));
    if equal(a217, a218) {
        f47(v0, v1, v3, v4)
    } else {
        let a219 = v2.clone();
        let a220 = f11(Str::lit("self-form"));
        if equal(a219, a220) {
            f48(v0, v1, v3, v4, f11(Str::lit("a form ID")), f11(Str::lit("a self form is `(self-form form-id)`")), 0u64)
        } else {
            let a221 = v2.clone();
            let a222 = f11(Str::lit("slot"));
            if equal(a221, a222) {
                f49(v0, v1, v3, v4)
            } else {
                let a223 = v2.clone();
                let a224 = f11(Str::lit("space"));
                if equal(a223, a224) {
                    let a225 = v5;
                    let a226 = 0u64;
                    if nat_eq(a225, a226) {
                        Ok(f35(v0, v1, Adt0::C4))
                    } else {
                        Ok(f31(v0, f10(f11(Str::lit("`(space)` takes no operand")))))
                    }
                } else {
                    let a227 = v2.clone();
                    let a228 = f11(Str::lit("token"));
                    if equal(a227, a228) {
                        f48(v0, v1, v3, v4, f11(Str::lit("a renderer-token ID")), f11(Str::lit("a token is `(token renderer-token-id)`")), 1u64)
                    } else {
                        let a229 = v2.clone();
                        let a230 = f11(Str::lit("operator-name"));
                        if equal(a229, a230) {
                            f48(v0, v1, v3, v4, f11(Str::lit("an operator name")), f11(Str::lit("an operator name is `(operator-name identifier)`")), 2u64)
                        } else {
                            let a231 = v2.clone();
                            let a232 = f11(Str::lit("seq"));
                            if equal(a231, a232) {
                                let a233 = v5;
                                let a234 = 0u64;
                                if nat_eq(a233, a234) {
                                    Ok(f31(v0, f10(f11(Str::lit("`seq` has at least one item")))))
                                } else {
                                    Ok(Adt6::C0(f50(v3, List::cons(Adt7::C1(v2, v5), v1)), match v0 {
                                        Adt6::C0(_, h235, _) => {
                                            h235
                                        }
                                    }, None::<Adt1>))
                                }
                            } else {
                                if f37(v2.clone()) {
                                    let a236 = v5;
                                    let a237 = 1u64;
                                    if nat_eq(a236, a237) {
                                        Ok(Adt6::C0(List::cons(Adt7::C0(f51(v3, 0u64)), List::cons(Adt7::C1(v2, 1u64), v1)), match v0 {
                                            Adt6::C0(_, h238, _) => {
                                                h238
                                            }
                                        }, None::<Adt1>))
                                    } else {
                                        Ok(f31(v0, f10(f26(f52(v2), f11(Str::lit(" takes exactly one operand"))))))
                                    }
                                } else {
                                    if f53(v2.clone()) {
                                        let a239 = v5;
                                        let a240 = 2u64;
                                        if nat_eq(a239, a240) {
                                            Ok(Adt6::C0(List::cons(Adt7::C0(f51(v3.clone(), 0u64)), List::cons(Adt7::C0(f51(v3, 1u64)), List::cons(Adt7::C1(v2, 2u64), v1))), match v0 {
                                                Adt6::C0(_, h241, _) => {
                                                    h241
                                                }
                                            }, None::<Adt1>))
                                        } else {
                                            Ok(f31(v0, f10(f26(f52(v2), f11(Str::lit(" takes exactly two operands"))))))
                                        }
                                    } else {
                                        Ok(f31(v0, f10(f26(f52(v2), f11(Str::lit(" is not an LRE form"))))))
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

pub fn f33(v0: Adt4) -> Bytes {
    let m242 = v0;
    match m242 {
        Adt4::C0(v1) => {
            f52(v1)
        }
        Adt4::C1(r246) => {
            let v2 = (*r246).clone();
            let m243 = v2;
            match m243 {
                Adt4::C0(_) => {
                    f11(Str::lit("a list"))
                }
                Adt4::C1(_) => {
                    f11(Str::lit("a list"))
                }
                Adt4::C2 => {
                    f11(Str::lit("a list"))
                }
                Adt4::C3(r245, _) => {
                    let v3 = (*r245).clone();
                    let m244 = v3;
                    match m244 {
                        Adt4::C0(v5) => {
                            f26(f26(f11(Str::lit("`(")), v5), f11(Str::lit(" ...)`")))
                        }
                        Adt4::C1(_) => {
                            f11(Str::lit("a list"))
                        }
                        Adt4::C2 => {
                            f11(Str::lit("a list"))
                        }
                        Adt4::C3(_, _) => {
                            f11(Str::lit("a list"))
                        }
                    }
                }
            }
        }
        Adt4::C2 => {
            f11(Str::lit("a list"))
        }
        Adt4::C3(_, _) => {
            f11(Str::lit("a list"))
        }
    }
}

pub fn f34(v0: List<Adt0>, v1: u64) -> (List<Adt0>, List<Adt0>) {
    let (h247, _) = f54(Fn6::F55, v1, (List::<Adt0>::nil(), v0));
    h247
}

pub fn f35(v0: Adt6, v1: List<Adt7>, v2: Adt0) -> Adt6 {
    Adt6::C0(v1, List::cons(v2, match v0 {
        Adt6::C0(_, h248, _) => {
            h248
        }
    }), None::<Adt1>)
}

pub fn f36(v0: List<Adt0>) -> Adt0 {
    f56(Fn7::F57, Adt0::C13, f58(v0))
}

pub fn f37(v0: Bytes) -> bool {
    let a249 = v0.clone();
    let a250 = f11(Str::lit("group"));
    if equal(a249, a250) {
        true
    } else {
        let a251 = v0.clone();
        let a252 = f11(Str::lit("paren"));
        if equal(a251, a252) {
            true
        } else {
            let a253 = v0;
            let a254 = f11(Str::lit("bracket"));
            equal(a253, a254)
        }
    }
}

pub fn f38(v0: Bytes, v1: Adt0) -> Adt0 {
    let a255 = v0.clone();
    let a256 = f11(Str::lit("group"));
    if equal(a255, a256) {
        Adt0::C6(std::rc::Rc::new(v1))
    } else {
        let a257 = v0;
        let a258 = f11(Str::lit("paren"));
        if equal(a257, a258) {
            Adt0::C7(std::rc::Rc::new(v1))
        } else {
            Adt0::C8(std::rc::Rc::new(v1))
        }
    }
}

pub fn f39(v0: Bytes, v1: Adt0, v2: Adt0) -> Adt0 {
    let a259 = v0.clone();
    let a260 = f11(Str::lit("sub"));
    if equal(a259, a260) {
        Adt0::C9(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
    } else {
        let a261 = v0;
        let a262 = f11(Str::lit("sup"));
        if equal(a261, a262) {
            Adt0::C10(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        } else {
            Adt0::C11(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a263 = v2;
    let a264 = 0u64;
    if nat_eq(a263, a264) {
        f59(v0, 0u64)
    } else {
        let (_, h265) = {
            let (h266, _) = f60(Fn8::F61(v1, v0), v2, (v2, Bytes::lit(&[])));
            h266
        };
        h265
    }
}

pub fn f41() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f42(v0: Fn4, p270: Adt4, p271: List<Adt4>) -> Adt4 {
    let m269 = (p270, p271);
    {
        let mut m269 = m269;
        loop {
            let step = {
                let (v1, v2) = m269;
                let m267 = v2;
                match m267.uncons() {
                    None => {
                        Err::<(Adt4, List<Adt4>), Adt4>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<Adt4>), Adt4>(({
                            let c268: Fn4 = v0.clone();
                            c268.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Adt4, v1: Adt4) -> Adt4 {
    Adt4::C3(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f44(v0: List<Adt4>) -> List<Adt4> {
    f62(Fn9::F63, List::<Adt4>::nil(), v0)
}

pub fn f45(v0: Fn5, p276: u64, p277: (Adt4, List<Adt4>)) -> ((Adt4, List<Adt4>), bool) {
    let m275 = (p276, p277);
    {
        let mut m275 = m275;
        loop {
            let step = {
                let (v1, v2) = m275;
                let m272 = v1;
                if m272 == 0 {
                    Err::<(u64, (Adt4, List<Adt4>)), ((Adt4, List<Adt4>), bool)>((v2, false))
                } else {
                    let v3 = m272 - 1;
                    let m274 = {
                        let c273: Fn5 = v0.clone();
                        c273.apply(v2.clone())
                    };
                    match m274 {
                        None => {
                            Err::<(u64, (Adt4, List<Adt4>)), ((Adt4, List<Adt4>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt4, List<Adt4>)), ((Adt4, List<Adt4>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m275 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: (Adt4, List<Adt4>)) -> Option<(Adt4, List<Adt4>)> {
    f64(v0)
}

pub fn f47(v0: Adt6, v1: List<Adt7>, v2: List<Adt4>, v3: u64) -> R<Adt6> {
    let a279 = {
        let a278 = v2.clone();
        length_list(a278)
    };
    let a280 = 2u64;
    if nat_eq(a279, a280) {
        let m281 = f65(f51(v2.clone(), 0u64), f11(Str::lit("a qualified ID")), v3);
        match m281 {
            Ok(v4) => {
                let m282 = f66(v4)?;
                match m282 {
                    Ok(v5) => {
                        let m283 = f65(f51(v2, 1u64), f11(Str::lit("a form ID")), v3);
                        match m283 {
                            Ok(v6) => {
                                if f67(v6.clone())? {
                                    Ok(f35(v0, v1, Adt0::C0({
                                        let (h284, _) = v5;
                                        h284
                                    }, {
                                        let (_, h285) = v5;
                                        h285
                                    }, v6)))
                                } else {
                                    Ok(f31(v0, f10(f26(f52(v6), f11(Str::lit(" is not a form ID"))))))
                                }
                            }
                            Err(v7) => {
                                Ok(f31(v0, v7))
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(f31(v0, v8))
                    }
                }
            }
            Err(v9) => {
                Ok(f31(v0, v9))
            }
        }
    } else {
        Ok(f31(v0, f10(f11(Str::lit("a form reference is `(form qualified-id form-id)`")))))
    }
}

pub fn f48(v0: Adt6, v1: List<Adt7>, v2: List<Adt4>, v3: u64, v4: Bytes, v5: Bytes, v6: u64) -> R<Adt6> {
    let a287 = {
        let a286 = v2.clone();
        length_list(a286)
    };
    let a288 = 1u64;
    if nat_eq(a287, a288) {
        let m289 = f65(f51(v2, 0u64), v4, v3);
        match m289 {
            Ok(v7) => {
                let a290 = v6;
                let a291 = 0u64;
                if nat_eq(a290, a291) {
                    if f67(v7.clone())? {
                        Ok(f35(v0, v1, Adt0::C1(v7)))
                    } else {
                        Ok(f31(v0, f10(f26(f52(v7), f11(Str::lit(" is not a form ID"))))))
                    }
                } else {
                    let a292 = v6;
                    let a293 = 1u64;
                    if nat_eq(a292, a293) {
                        if f68(v7.clone())? {
                            Ok(f35(v0, v1, Adt0::C5(v7)))
                        } else {
                            Ok(f31(v0, f10(f26(f52(v7), f11(Str::lit(" is not a renderer-token ID"))))))
                        }
                    } else {
                        if f69(v7.clone())? {
                            Ok(f35(v0, v1, Adt0::C12(v7)))
                        } else {
                            Ok(f31(v0, f10(f26(f52(v7), f11(Str::lit(" is not an operator name"))))))
                        }
                    }
                }
            }
            Err(v8) => {
                Ok(f31(v0, v8))
            }
        }
    } else {
        Ok(f31(v0, f10(v5)))
    }
}

pub fn f49(v0: Adt6, v1: List<Adt7>, v2: List<Adt4>, v3: u64) -> R<Adt6> {
    let a295 = {
        let a294 = v2.clone();
        length_list(a294)
    };
    let a296 = 1u64;
    if nat_eq(a295, a296) {
        let m297 = f65(f51(v2, 0u64), f11(Str::lit("a slot index")), v3);
        match m297 {
            Ok(v4) => {
                let m298 = f70(v4.clone())?;
                match m298 {
                    None => {
                        let v6: u64 = f71(v4.clone())?;
                        let a299 = 4294967295u64;
                        let a300 = v6;
                        if nat_lt(a299, a300) {
                            Ok(f31(v0, f10(f26(f52(v4), f11(Str::lit(" is not a slot index"))))))
                        } else {
                            Ok(f35(v0, v1, Adt0::C2(v6)))
                        }
                    }
                    Some(_) => {
                        Ok(f31(v0, f10(f26(f52(v4), f11(Str::lit(" is not a slot index"))))))
                    }
                }
            }
            Err(v7) => {
                Ok(f31(v0, v7))
            }
        }
    } else {
        Ok(f31(v0, f10(f11(Str::lit("a slot is `(slot decimal)`")))))
    }
}

pub fn f50(v0: List<Adt4>, v1: List<Adt7>) -> List<Adt7> {
    f72(Fn10::F73, v1, f44(v0))
}

pub fn f51(v0: List<Adt4>, v1: u64) -> Adt4 {
    let m303 = {
        let a301 = v0;
        let a302 = v1;
        index_list(a301, a302)
    };
    match m303 {
        None => {
            Adt4::C2
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f52(v0: Bytes) -> Bytes {
    f26(f26(f11(Str::lit("`")), v0), f11(Str::lit("`")))
}

pub fn f53(v0: Bytes) -> bool {
    let a304 = v0.clone();
    let a305 = f11(Str::lit("sub"));
    if equal(a304, a305) {
        true
    } else {
        let a306 = v0.clone();
        let a307 = f11(Str::lit("sup"));
        if equal(a306, a307) {
            true
        } else {
            let a308 = v0;
            let a309 = f11(Str::lit("frac"));
            equal(a308, a309)
        }
    }
}

pub fn f54(v0: Fn6, p314: u64, p315: (List<Adt0>, List<Adt0>)) -> ((List<Adt0>, List<Adt0>), bool) {
    let m313 = (p314, p315);
    {
        let mut m313 = m313;
        loop {
            let step = {
                let (v1, v2) = m313;
                let m310 = v1;
                if m310 == 0 {
                    Err::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v2, false))
                } else {
                    let v3 = m310 - 1;
                    let m312 = {
                        let c311: Fn6 = v0.clone();
                        c311.apply(v2.clone())
                    };
                    match m312 {
                        None => {
                            Err::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v3, v4))
                        }
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

pub fn f55(v0: (List<Adt0>, List<Adt0>)) -> Option<(List<Adt0>, List<Adt0>)> {
    let m317 = {
        let (_, h316) = v0;
        h316
    };
    match m317.uncons() {
        None => {
            None::<(List<Adt0>, List<Adt0>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h318, _) = v0;
                h318
            }), v2))
        }
    }
}

pub fn f56(v0: Fn7, p322: Adt0, p323: List<Adt0>) -> Adt0 {
    let m321 = (p322, p323);
    {
        let mut m321 = m321;
        loop {
            let step = {
                let (v1, v2) = m321;
                let m319 = v2;
                match m319.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c320: Fn7 = v0.clone();
                            c320.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C14(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f58(v0: List<Adt0>) -> List<Adt0> {
    f74(Fn11::F75, List::<Adt0>::nil(), v0)
}

pub fn f59(v0: Bytes, v1: u64) -> Bytes {
    let m327 = {
        let a324 = v0;
        let a325 = v1;
        let a326 = 1u64;
        slice_bytes(a324, a325, a326)
    };
    match m327 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f60(v0: Fn8, p332: u64, p333: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m331 = (p332, p333);
    {
        let mut m331 = m331;
        loop {
            let step = {
                let (v1, v2) = m331;
                let m328 = v1;
                if m328 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m328 - 1;
                    let m330 = {
                        let c329: Fn8 = v0.clone();
                        c329.apply(v2.clone())
                    };
                    match m330 {
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
                Ok(next) => { m331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f76(v1, v0, v2)
}

pub fn f62(v0: Fn9, p337: List<Adt4>, p338: List<Adt4>) -> List<Adt4> {
    let m336 = (p337, p338);
    {
        let mut m336 = m336;
        loop {
            let step = {
                let (v1, v2) = m336;
                let m334 = v2;
                match m334.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c335: Fn9 = v0.clone();
                            c335.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m336 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f64(v0: (Adt4, List<Adt4>)) -> Option<(Adt4, List<Adt4>)> {
    let m340 = {
        let (h339, _) = v0;
        h339
    };
    match m340 {
        Adt4::C0(_) => {
            None::<(Adt4, List<Adt4>)>
        }
        Adt4::C1(_) => {
            None::<(Adt4, List<Adt4>)>
        }
        Adt4::C2 => {
            None::<(Adt4, List<Adt4>)>
        }
        Adt4::C3(r342, r343) => {
            let v3 = (*r342).clone();
            let v4 = (*r343).clone();
            Some((v4, List::cons(v3, {
                let (_, h341) = v0;
                h341
            })))
        }
    }
}

pub fn f65(v0: Adt4, v1: Bytes, _: u64) -> Result<Bytes, Adt1> {
    let m344 = v0.clone();
    match m344 {
        Adt4::C0(v3) => {
            Ok::<Bytes, Adt1>(v3)
        }
        Adt4::C1(_) => {
            Err::<Bytes, Adt1>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
        Adt4::C2 => {
            Err::<Bytes, Adt1>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
        Adt4::C3(_, _) => {
            Err::<Bytes, Adt1>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
    }
}

pub fn f66(v0: Bytes) -> R<Result<(Bytes, Bytes), Adt1>> {
    let m345 = f77(v0.clone(), f11(Str::lit("::")))?;
    match m345 {
        None => {
            Ok(Err::<(Bytes, Bytes), Adt1>(f10(f26(f52(v0), f11(Str::lit(" is not a qualified `package::entry` ID"))))))
        }
        Some(v1) => {
            let v2: Bytes = f17(v0.clone(), 0u64, v1);
            let v3: Bytes = f17(v0.clone(), {
                let a346 = v1;
                let a347 = 2u64;
                nat_add(a346, a347)?
            }, {
                let a351 = {
                    let a348 = v0;
                    length_bytes(a348)
                };
                let a352 = {
                    let a349 = v1;
                    let a350 = 2u64;
                    nat_add(a349, a350)?
                };
                nat_sub(a351, a352)
            });
            if f78(v2.clone())? {
                if f79(v3.clone())? {
                    Ok(Ok::<(Bytes, Bytes), Adt1>((v2, v3)))
                } else {
                    Ok(Err::<(Bytes, Bytes), Adt1>(f10(f26(f52(v3), f11(Str::lit(" is not a valid local entry ID"))))))
                }
            } else {
                Ok(Err::<(Bytes, Bytes), Adt1>(f10(f26(f52(v2), f11(Str::lit(" is not a valid package ID"))))))
            }
        }
    }
}

pub fn f67(v0: Bytes) -> R<bool> {
    f68(v0)
}

pub fn f68(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a353 = v0.clone();
        length_bytes(a353)
    };
    let a354 = v1;
    let a355 = 0u64;
    if nat_eq(a354, a355) {
        Ok(false)
    } else {
        if f80(f20(v0.clone(), 0u64)?) {
            let a357 = {
                let (h356, _) = f81(Fn12::F82(v1, v0), v1, 1u64)?;
                h356
            };
            let a358 = v1;
            Ok(nat_eq(a357, a358))
        } else {
            Ok(false)
        }
    }
}

pub fn f69(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a359 = v0.clone();
        length_bytes(a359)
    };
    let a360 = v1;
    let a361 = 0u64;
    if nat_eq(a360, a361) {
        Ok(false)
    } else {
        if f83(f20(v0.clone(), 0u64)?) {
            let a363 = {
                let (h362, _) = f81(Fn12::F84(v1, v0), v1, 1u64)?;
                h362
            };
            let a364 = v1;
            Ok(nat_eq(a363, a364))
        } else {
            Ok(false)
        }
    }
}

pub fn f70(v0: Bytes) -> R<Option<Adt1>> {
    let v1: u64 = {
        let a365 = v0.clone();
        length_bytes(a365)
    };
    let a366 = v1;
    let a367 = 0u64;
    if if nat_eq(a366, a367) {
        true
    } else {
        !f85(v0.clone())?
    } {
        Ok(Some(f10(f26(f52(v0), f11(Str::lit(" is not a decimal"))))))
    } else {
        let a368 = 1u64;
        let a369 = v1;
        if if nat_lt(a368, a369) {
            let a370 = f20(v0.clone(), 0u64)?;
            let a371 = 48u64;
            nat_eq(a370, a371)
        } else {
            false
        } {
            Ok(Some(f10(f26(f52(v0), f11(Str::lit(" has a redundant leading zero"))))))
        } else {
            let a372 = 20u64;
            let a373 = v1;
            if if nat_lt(a372, a373) {
                true
            } else {
                let a374 = v1;
                let a375 = 20u64;
                if nat_eq(a374, a375) {
                    let a380 = {
                        let a376 = v0.clone();
                        let a377 = f11(Str::lit("18446744073709551615"));
                        compare_bytes(a376, a377)
                    };
                    let a381 = {
                        let a378 = Bytes::lit(&[0x01]);
                        let a379 = Bytes::lit(&[]);
                        compare_bytes(a378, a379)
                    };
                    equal(a380, a381)
                } else {
                    false
                }
            } {
                Ok(Some(f10(f26(f52(v0), f11(Str::lit(" does not fit 64 bits"))))))
            } else {
                Ok(None::<Adt1>)
            }
        }
    }
}

pub fn f71(v0: Bytes) -> R<u64> {
    let m382 = f86(v0)?;
    match m382 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f72(v0: Fn10, p386: List<Adt7>, p387: List<Adt4>) -> List<Adt7> {
    let m385 = (p386, p387);
    {
        let mut m385 = m385;
        loop {
            let step = {
                let (v1, v2) = m385;
                let m383 = v2;
                match m383.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt4>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt4>), List<Adt7>>(({
                            let c384: Fn10 = v0.clone();
                            c384.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<Adt7>, v1: Adt4) -> List<Adt7> {
    List::cons(Adt7::C0(v1), v0)
}

pub fn f74(v0: Fn11, p391: List<Adt0>, p392: List<Adt0>) -> List<Adt0> {
    let m390 = (p391, p392);
    {
        let mut m390 = m390;
        loop {
            let step = {
                let (v1, v2) = m390;
                let m388 = v2;
                match m388.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c389: Fn11 = v0.clone();
                            c389.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f76(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h393, _) = v2;
        h393
    };
    let a394 = v3;
    let a395 = 0u64;
    if nat_eq(a394, a395) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a396 = v3;
            let a397 = v1;
            let a398 = 0u64;
            nat_quot(a396, a397, a398)
        }, {
            let a403 = f59(v0, {
                let a399 = v3;
                let a400 = v1;
                let a401 = 0u64;
                nat_rem(a399, a400, a401)
            });
            let a404 = {
                let (_, h402) = v2;
                h402
            };
            append_bytes(a403, a404)
        }))
    }
}

pub fn f77(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a405 = v0.clone();
        length_bytes(a405)
    };
    let v3: u64 = {
        let a406 = v1.clone();
        length_bytes(a406)
    };
    let v4: u64 = {
        let (h407, _) = f81(Fn12::F87(v1.clone(), v3, v0.clone(), v2), {
            let a408 = v2;
            let a409 = 1u64;
            nat_add(a408, a409)?
        }, 0u64)?;
        h407
    };
    if f88(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f78(v0: Bytes) -> R<bool> {
    f89(v0, false)
}

pub fn f79(v0: Bytes) -> R<bool> {
    f89(v0, true)
}

pub fn f80(v0: u64) -> bool {
    let a410 = v0;
    let a411 = 97u64;
    if nat_lt(a410, a411) {
        false
    } else {
        let a412 = v0;
        let a413 = 122u64;
        nat_le(a412, a413)
    }
}

pub fn f81(v0: Fn12, p418: u64, p419: u64) -> R<(u64, bool)> {
    let m417 = (p418, p419);
    {
        let mut m417 = m417;
        loop {
            let step = {
                let (v1, v2) = m417;
                let m414 = v1;
                if m414 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m414 - 1;
                    let m416 = {
                        let c415: Fn12 = v0.clone();
                        c415.apply(v2)?
                    };
                    match m416 {
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
                Ok(next) => { m417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f90(v1, v0, v2)
}

pub fn f83(v0: u64) -> bool {
    if f80(v0) {
        true
    } else {
        let a420 = v0;
        let a421 = 65u64;
        if nat_lt(a420, a421) {
            false
        } else {
            let a422 = v0;
            let a423 = 90u64;
            nat_le(a422, a423)
        }
    }
}

pub fn f84(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f91(v1, v0, v2)
}

pub fn f85(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a424 = v0.clone();
        length_bytes(a424)
    };
    let a428 = {
        let (h425, _) = f81(Fn12::F92(v1, v0), {
            let a426 = v1;
            let a427 = 1u64;
            nat_add(a426, a427)?
        }, 0u64)?;
        h425
    };
    let a429 = v1;
    Ok(nat_eq(a428, a429))
}

pub fn f86(v0: Bytes) -> R<Option<u64>> {
    if f93(v0.clone())? {
        Ok(Some({
            let (_, h430) = {
                let (h431, _) = f94(Fn13::F95(v0.clone()), {
                    let a432 = v0;
                    length_bytes(a432)
                }, (0u64, 0u64))?;
                h431
            };
            h430
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f87(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f88(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a435 = {
            let a433 = v4;
            let a434 = v1;
            nat_add(a433, a434)?
        };
        let a436 = v3;
        if nat_lt(a435, a436) {
            Ok(Some({
                let a437 = v4;
                let a438 = 1u64;
                nat_add(a437, a438)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f88(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m443 = {
        let a440 = v0;
        let a441 = v2;
        let a442 = {
            let a439 = v1.clone();
            length_bytes(a439)
        };
        slice_bytes(a440, a441, a442)
    };
    match m443 {
        None => {
            false
        }
        Some(v3) => {
            let a444 = v3;
            let a445 = v1;
            equal(a444, a445)
        }
    }
}

pub fn f89(v0: Bytes, v1: bool) -> R<bool> {
    let v2: u64 = {
        let a446 = v0.clone();
        length_bytes(a446)
    };
    let v3: Adt8 = {
        let (h447, _) = f96(Fn14::F97(v1, v2, v0), {
            let a448 = v2;
            let a449 = 1u64;
            nat_add(a448, a449)?
        }, Adt8::C0(0u64, true, true, true))?;
        h447
    };
    if match v3.clone() {
        Adt8::C0(_, _, _, h450) => {
            h450
        }
    } {
        Ok(!match v3 {
            Adt8::C0(_, h451, _, _) => {
                h451
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f90(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a452 = v2;
    let a453 = v1;
    if nat_lt(a452, a453) {
        let v3: u64 = f20(v0, v2)?;
        if if f80(v3) {
            true
        } else {
            if f98(v3) {
                true
            } else {
                let a454 = v3;
                let a455 = 45u64;
                nat_eq(a454, a455)
            }
        } {
            Ok(Some({
                let a456 = v2;
                let a457 = 1u64;
                nat_add(a456, a457)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f91(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a458 = v2;
    let a459 = v1;
    if nat_lt(a458, a459) {
        let v3: u64 = f20(v0, v2)?;
        if if f99(v3) {
            true
        } else {
            let a460 = v3;
            let a461 = 95u64;
            nat_eq(a460, a461)
        } {
            Ok(Some({
                let a462 = v2;
                let a463 = 1u64;
                nat_add(a462, a463)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f92(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f100(v1, v0, v2)
}

pub fn f93(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a464 = v0.clone();
        length_bytes(a464)
    };
    let a465 = 0u64;
    let a466 = v1;
    if nat_lt(a465, a466) {
        let a467 = f101(v0.clone())?;
        let a468 = v1;
        if nat_eq(a467, a468) {
            let a469 = 1u64;
            let a470 = v1;
            if nat_lt(a469, a470) {
                let m473 = {
                    let a471 = v0;
                    let a472 = 0u64;
                    index_bytes(a471, a472)
                };
                match m473 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a474 = v2;
                        let a475 = 48u8;
                        Ok(!equal(a474, a475))
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

pub fn f94(v0: Fn13, p480: u64, p481: (u64, u64)) -> R<((u64, u64), bool)> {
    let m479 = (p480, p481);
    {
        let mut m479 = m479;
        loop {
            let step = {
                let (v1, v2) = m479;
                let m476 = v1;
                if m476 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m476 - 1;
                    let m478 = {
                        let c477: Fn13 = v0.clone();
                        c477.apply(v2)?
                    };
                    match m478 {
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
                Ok(next) => { m479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f102(v0, v1)
}

pub fn f96(v0: Fn14, p486: u64, p487: Adt8) -> R<(Adt8, bool)> {
    let m485 = (p486, p487);
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v1, v2) = m485;
                let m482 = v1;
                if m482 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m482 - 1;
                    let m484 = {
                        let c483: Fn14 = v0.clone();
                        c483.apply(v2.clone())?
                    };
                    match m484 {
                        None => {
                            Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt8), R<(Adt8, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: bool, v1: u64, v2: Bytes, v3: Adt8) -> R<Option<Adt8>> {
    f103(v2, v1, v0, v3)
}

pub fn f98(v0: u64) -> bool {
    let a488 = v0;
    let a489 = 48u64;
    if nat_lt(a488, a489) {
        false
    } else {
        let a490 = v0;
        let a491 = 57u64;
        nat_le(a490, a491)
    }
}

pub fn f99(v0: u64) -> bool {
    if f83(v0) {
        true
    } else {
        f98(v0)
    }
}

pub fn f100(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a492 = v2;
    let a493 = v1;
    if nat_lt(a492, a493) {
        if f98(f20(v0, v2)?) {
            Ok(Some({
                let a494 = v2;
                let a495 = 1u64;
                nat_add(a494, a495)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f101(v0: Bytes) -> R<u64> {
    let (h496, _) = f81(Fn12::F104(v0.clone()), {
        let a497 = v0;
        length_bytes(a497)
    }, 0u64)?;
    Ok(h496)
}

pub fn f102(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m501 = {
        let a499 = v0;
        let a500 = {
            let (h498, _) = v1;
            h498
        };
        index_bytes(a499, a500)
    };
    match m501 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a503 = {
                    let (h502, _) = v1;
                    h502
                };
                let a504 = 1u64;
                nat_add(a503, a504)?
            }, {
                let a508 = {
                    let a506 = {
                        let (_, h505) = v1;
                        h505
                    };
                    let a507 = 10u64;
                    nat_mul(a506, a507)?
                };
                let a509 = f105(v2)?;
                nat_add(a508, a509)?
            })))
        }
    }
}

pub fn f103(v0: Bytes, v1: u64, v2: bool, v3: Adt8) -> R<Option<Adt8>> {
    if match v3.clone() {
        Adt8::C0(_, _, _, h510) => {
            h510
        }
    } {
        let a512 = match v3.clone() {
            Adt8::C0(h511, _, _, _) => {
                h511
            }
        };
        let a513 = v1;
        if nat_lt(a512, a513) {
            let v4: u64 = f20(v0, match v3.clone() {
                Adt8::C0(h514, _, _, _) => {
                    h514
                }
            })?;
            let a515 = v4;
            let a516 = 46u64;
            if nat_eq(a515, a516) {
                if match v3.clone() {
                    Adt8::C0(_, h517, _, _) => {
                        h517
                    }
                } {
                    Ok(f106(v3))
                } else {
                    f107(v3, true, false)
                }
            } else {
                if match v3.clone() {
                    Adt8::C0(_, h518, _, _) => {
                        h518
                    }
                } {
                    if f80(v4) {
                        f107(v3.clone(), false, match v3 {
                            Adt8::C0(_, _, h519, _) => {
                                h519
                            }
                        })
                    } else {
                        Ok(f106(v3))
                    }
                } else {
                    if if f80(v4) {
                        true
                    } else {
                        f98(v4)
                    } {
                        f107(v3.clone(), false, match v3 {
                            Adt8::C0(_, _, h520, _) => {
                                h520
                            }
                        })
                    } else {
                        let a521 = v4;
                        let a522 = 45u64;
                        if if nat_eq(a521, a522) {
                            if v2 {
                                true
                            } else {
                                !match v3.clone() {
                                    Adt8::C0(_, _, h523, _) => {
                                        h523
                                    }
                                }
                            }
                        } else {
                            false
                        } {
                            f107(v3.clone(), false, match v3 {
                                Adt8::C0(_, _, h524, _) => {
                                    h524
                                }
                            })
                        } else {
                            Ok(f106(v3))
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt8>)
        }
    } else {
        Ok(None::<Adt8>)
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m527 = {
        let a525 = v0;
        let a526 = v1;
        index_bytes(a525, a526)
    };
    match m527 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f108(v2)? {
                Ok(Some({
                    let a528 = v1;
                    let a529 = 1u64;
                    nat_add(a528, a529)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f105(v0: u8) -> R<u64> {
    let a576 = {
        let v1: u8 = v0;
        let a574 = {
            let a550 = {
                let a538 = {
                    let a532 = {
                        let a530 = v1;
                        let a531 = 128u8;
                        fixed_u8::bit_and(a530, a531)
                    };
                    let a533 = 128u8;
                    if equal(a532, a533) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a539 = {
                    let a536 = {
                        let a534 = v1;
                        let a535 = 64u8;
                        fixed_u8::bit_and(a534, a535)
                    };
                    let a537 = 64u8;
                    if equal(a536, a537) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a538, a539)?
            };
            let a551 = {
                let a548 = {
                    let a542 = {
                        let a540 = v1;
                        let a541 = 32u8;
                        fixed_u8::bit_and(a540, a541)
                    };
                    let a543 = 32u8;
                    if equal(a542, a543) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a549 = {
                    let a546 = {
                        let a544 = v1;
                        let a545 = 16u8;
                        fixed_u8::bit_and(a544, a545)
                    };
                    let a547 = 16u8;
                    if equal(a546, a547) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a548, a549)?
            };
            nat_add(a550, a551)?
        };
        let a575 = {
            let a572 = {
                let a560 = {
                    let a554 = {
                        let a552 = v1;
                        let a553 = 8u8;
                        fixed_u8::bit_and(a552, a553)
                    };
                    let a555 = 8u8;
                    if equal(a554, a555) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a561 = {
                    let a558 = {
                        let a556 = v1;
                        let a557 = 4u8;
                        fixed_u8::bit_and(a556, a557)
                    };
                    let a559 = 4u8;
                    if equal(a558, a559) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a560, a561)?
            };
            let a573 = {
                let a570 = {
                    let a564 = {
                        let a562 = v1;
                        let a563 = 2u8;
                        fixed_u8::bit_and(a562, a563)
                    };
                    let a565 = 2u8;
                    if equal(a564, a565) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a571 = {
                    let a568 = {
                        let a566 = v1;
                        let a567 = 1u8;
                        fixed_u8::bit_and(a566, a567)
                    };
                    let a569 = 1u8;
                    if equal(a568, a569) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a570, a571)?
            };
            nat_add(a572, a573)?
        };
        nat_add(a574, a575)?
    };
    let a577 = 48u64;
    Ok(nat_sub(a576, a577))
}

pub fn f106(v0: Adt8) -> Option<Adt8> {
    Some(Adt8::C0(match v0.clone() {
        Adt8::C0(h578, _, _, _) => {
            h578
        }
    }, match v0.clone() {
        Adt8::C0(_, h579, _, _) => {
            h579
        }
    }, match v0 {
        Adt8::C0(_, _, h580, _) => {
            h580
        }
    }, false))
}

pub fn f107(v0: Adt8, v1: bool, v2: bool) -> R<Option<Adt8>> {
    Ok(Some(Adt8::C0({
        let a582 = match v0 {
            Adt8::C0(h581, _, _, _) => {
                h581
            }
        };
        let a583 = 1u64;
        nat_add(a582, a583)?
    }, v1, v2, true)))
}

pub fn f108(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a628 = {
            let a604 = {
                let a592 = {
                    let a586 = {
                        let a584 = v1;
                        let a585 = 128u8;
                        fixed_u8::bit_and(a584, a585)
                    };
                    let a587 = 128u8;
                    if equal(a586, a587) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a593 = {
                    let a590 = {
                        let a588 = v1;
                        let a589 = 64u8;
                        fixed_u8::bit_and(a588, a589)
                    };
                    let a591 = 64u8;
                    if equal(a590, a591) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a592, a593)?
            };
            let a605 = {
                let a602 = {
                    let a596 = {
                        let a594 = v1;
                        let a595 = 32u8;
                        fixed_u8::bit_and(a594, a595)
                    };
                    let a597 = 32u8;
                    if equal(a596, a597) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a603 = {
                    let a600 = {
                        let a598 = v1;
                        let a599 = 16u8;
                        fixed_u8::bit_and(a598, a599)
                    };
                    let a601 = 16u8;
                    if equal(a600, a601) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a602, a603)?
            };
            nat_add(a604, a605)?
        };
        let a629 = {
            let a626 = {
                let a614 = {
                    let a608 = {
                        let a606 = v1;
                        let a607 = 8u8;
                        fixed_u8::bit_and(a606, a607)
                    };
                    let a609 = 8u8;
                    if equal(a608, a609) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a615 = {
                    let a612 = {
                        let a610 = v1;
                        let a611 = 4u8;
                        fixed_u8::bit_and(a610, a611)
                    };
                    let a613 = 4u8;
                    if equal(a612, a613) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a614, a615)?
            };
            let a627 = {
                let a624 = {
                    let a618 = {
                        let a616 = v1;
                        let a617 = 2u8;
                        fixed_u8::bit_and(a616, a617)
                    };
                    let a619 = 2u8;
                    if equal(a618, a619) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a625 = {
                    let a622 = {
                        let a620 = v1;
                        let a621 = 1u8;
                        fixed_u8::bit_and(a620, a621)
                    };
                    let a623 = 1u8;
                    if equal(a622, a623) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a624, a625)?
            };
            nat_add(a626, a627)?
        };
        nat_add(a628, a629)?
    };
    let a630 = 48u64;
    let a631 = v2;
    if nat_le(a630, a631) {
        let a632 = v2;
        let a633 = 57u64;
        Ok(nat_le(a632, a633))
    } else {
        Ok(false)
    }
}
