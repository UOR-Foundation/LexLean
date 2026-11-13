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
    C0(Bytes, Bytes, Bytes, Bytes, Option<Bytes>, List<Bytes>, List<Bytes>, List<(Bytes, Bytes)>, List<Adt1>, Adt6, Option<Bytes>, Option<Adt12>, Option<Adt12>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Bytes),
    C1(Adt2),
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3),
    C1(u64),
    C2(Adt4, Adt3),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, List<u64>),
    C4(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C6(u64, u64, Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C7(Bytes, std::rc::Rc<Adt2>),
    C8,
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(u64, u64, Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64),
    C1(Bytes),
    C2(std::rc::Rc<Adt3>),
    C3(std::rc::Rc<Adt3>),
    C4(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C5,
    C6(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt5),
    C1(Bytes, Bytes, Bytes, Bytes, Bytes),
    C2(Bytes, Bytes, Bytes),
    C3(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, List<Adt1>, Adt2, std::rc::Rc<Adt6>),
    C1(Adt7),
    C2,
    C3(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes, Adt8, Adt2, Adt9, Adt11),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Adt2, Adt2),
    C1(Adt2, Adt10),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(std::rc::Rc<Adt10>),
    C1(List<u64>),
    C2(Adt2),
    C3(Adt2),
    C4(Adt2, std::rc::Rc<Adt10>),
    C5,
    C6(Adt2),
    C7,
    C8,
    C9(u64, Adt2, std::rc::Rc<Adt10>),
    C10(Option<u64>, std::rc::Rc<Adt10>),
    C11(Option<u64>, Adt2),
    C12(std::rc::Rc<Adt10>),
    C13(Adt2, std::rc::Rc<Adt10>),
    C14(Adt2, std::rc::Rc<Adt10>),
    C15(Adt4, Adt2, std::rc::Rc<Adt10>),
    C16,
    C17(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C18(bool, Adt2, std::rc::Rc<Adt10>),
    C19(Bytes, Bytes, List<(u64, Bytes)>, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C20(Adt2, Adt2, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1(List<Bytes>),
    C2(List<Bytes>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt12>),
    C4(std::rc::Rc<Adt12>),
    C5,
    C6(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C7(Bytes, std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(List<(u64, u64)>, u64, bool),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F20,
}

impl Fn0 {
    pub fn apply(&self, p0: (List<Adt12>, Adt13), p1: Adt1) -> R<(List<Adt12>, Adt13)> {
        match self {
            Fn0::F20 => f20(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F22,
    F56,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt12, p1: Adt12) -> Adt12 {
        match self {
            Fn1::F22 => f22(p0, p1),
            Fn1::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F30,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt12>, p1: Bytes) -> List<Adt12> {
        match self {
            Fn2::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F35(Bytes),
    F84(u64, Bytes),
    F113(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F35(k0) => f35(k0.clone(), p0),
            Fn3::F84(k0, k1) => Ok(f84(*k0, k1.clone(), p0)),
            Fn3::F113(k0) => f113(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F38,
    F132,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn4::F38 => f38(p0, p1),
            Fn4::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F41,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn5::F41 => f41(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F59,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F59 => f59(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F61(Adt0),
}

impl Fn7 {
    pub fn apply(&self, p0: Adt12, p1: (Bytes, Bytes)) -> Adt12 {
        match self {
            Fn7::F61(k0) => f61(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F66(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn8::F66(k0) => f66(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F81,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn9::F81 => f81(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F88,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn10::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F90,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn11::F90 => f90(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F93(u64),
}

impl Fn12 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn12::F93(k0) => f93(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F97,
    F119,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt12, p1: u64) -> Adt12 {
        match self {
            Fn13::F97 => f97(p0, p1),
            Fn13::F119 => f119(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F105,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt12, p1: Bytes) -> Adt12 {
        match self {
            Fn14::F105 => f105(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F111,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F111 => f111(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F115,
}

impl Fn16 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn16::F115 => f115(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F118,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<u64>, Adt13), p1: u64) -> R<(List<u64>, Adt13)> {
        match self {
            Fn17::F118 => f118(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F121,
}

impl Fn18 {
    pub fn apply(&self, p0: List<u64>, p1: (u64, Bytes)) -> List<u64> {
        match self {
            Fn18::F121 => f121(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F126,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn19::F126 => f126(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F131,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn20::F131 => f131(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0)?)
}

pub fn f1(v0: Adt12) -> R<Bytes> {
    f3(v0)
}

pub fn f2(v0: Adt0) -> R<Adt12> {
    let v1: (Adt12, Adt13) = f4(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h1, _, _, _, _) => {
            h1
        }
    }, f5())?;
    let v2: (Adt12, Adt13) = f6(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h2, _, _, _) => {
            h2
        }
    }, {
        let (_, h3) = v1;
        h3
    })?;
    Ok(f7(f8(Str::lit("name"), f9(match v0.clone() {
        Adt0::C0(h4, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4
        }
    }), f8(Str::lit("lean_module"), f9(match v0.clone() {
        Adt0::C0(_, h5, _, _, _, _, _, _, _, _, _, _, _) => {
            h5
        }
    }), f8(Str::lit("glossary"), f10(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h6, _, _, _, _, _, _, _) => {
            h6
        }
    }), f8(Str::lit("imports"), f10(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h7, _, _, _, _, _, _) => {
            h7
        }
    }), f8(Str::lit("title"), {
        let (h8, _) = v1;
        h8
    }, f8(Str::lit("blocks"), f11({
        let (h9, _) = v2;
        h9
    }), f12(v0)))))))))
}

pub fn f3(v0: Adt12) -> R<Bytes> {
    let m10 = v0;
    match m10 {
        Adt12::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt12::C1(v2, v3) => {
            f13(v2, v3)
        }
        Adt12::C2(v4) => {
            f14(v4)
        }
        Adt12::C3(r11) => {
            let v5 = (*r11).clone();
            Ok(f15(f16(v5)?))
        }
        Adt12::C4(r12) => {
            let v6 = (*r12).clone();
            f17(f18(v6)?)
        }
        Adt12::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt12::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt12::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: List<Adt1>, v1: Adt13) -> R<(Adt12, Adt13)> {
    let v2: (List<Adt12>, Adt13) = f19(Fn0::F20, (List::<Adt12>::nil(), v1), v0)?;
    Ok((f11(f21(Fn1::F22, f23(), {
        let (h13, _) = v2;
        h13
    })), {
        let (_, h14) = v2;
        h14
    }))
}

pub fn f5() -> Adt13 {
    Adt13::C0(List::<(u64, u64)>::nil(), 0u64, false)
}

pub fn f6(v0: Adt6, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m15 = v0;
    match m15 {
        Adt6::C0(v2, v3, v4, r28) => {
            let v5 = (*r28).clone();
            let v6: (Adt12, Adt13) = f4(v3, v1)?;
            let v7: (Adt12, Adt13) = f24(v4, {
                let (_, h16) = v6;
                h16
            })?;
            let v8: (Adt12, Adt13) = f6(v5, {
                let (_, h17) = v7;
                h17
            })?;
            Ok((f7(f8(Str::lit("k"), f25(Str::lit("section")), f8(Str::lit("component"), f9(v2), f8(Str::lit("heading"), {
                let (h18, _) = v6;
                h18
            }, f8(Str::lit("params"), f11({
                let (h19, _) = v7;
                h19
            }), f8(Str::lit("blocks"), f11({
                let (h20, _) = v8;
                h20
            }), f23())))))), {
                let (_, h21) = v8;
                h21
            }))
        }
        Adt6::C1(v9) => {
            let v10: (Adt12, Adt13) = f26(v9, v1)?;
            Ok((f7(f8(Str::lit("k"), f25(Str::lit("declaration")), f8(Str::lit("d"), {
                let (h22, _) = v10;
                h22
            }, f23()))), {
                let (_, h23) = v10;
                h23
            }))
        }
        Adt6::C2 => {
            Ok((f23(), v1))
        }
        Adt6::C3(r29, r30) => {
            let v11 = (*r29).clone();
            let v12 = (*r30).clone();
            let v13: (Adt12, Adt13) = f6(v11, v1)?;
            let v14: (Adt12, Adt13) = f6(v12, {
                let (_, h24) = v13;
                h24
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h25, _) = v13;
                h25
            }), std::rc::Rc::new({
                let (h26, _) = v14;
                h26
            })), {
                let (_, h27) = v14;
                h27
            }))
        }
    }
}

pub fn f7(v0: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(v0))
}

pub fn f8(v0: Str, v1: Adt12, v2: Adt12) -> Adt12 {
    Adt12::C7(f27(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f9(v0: Bytes) -> Adt12 {
    Adt12::C2(v0)
}

pub fn f10(v0: List<Bytes>) -> Adt12 {
    f11(f28(f29(Fn2::F30, List::<Adt12>::nil(), f31(v0))))
}

pub fn f11(v0: Adt12) -> Adt12 {
    Adt12::C3(std::rc::Rc::new(v0))
}

pub fn f12(v0: Adt0) -> Adt12 {
    let v2: Adt12 = {
        let m32 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, _, h31) => {
                h31
            }
        };
        match m32 {
            None => {
                f23()
            }
            Some(v1) => {
                f8(Str::lit("tooling"), v1, f23())
            }
        }
    };
    let v4: Adt12 = {
        let m34 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, h33, _) => {
                h33
            }
        };
        match m34 {
            None => {
                v2
            }
            Some(v3) => {
                f8(Str::lit("semantic"), v3, v2)
            }
        }
    };
    let v6: Adt12 = {
        let m36 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, h35, _, _) => {
                h35
            }
        };
        match m36 {
            None => {
                v4
            }
            Some(v5) => {
                f8(Str::lit("core"), f9(v5), v4)
            }
        }
    };
    let v7: Adt12 = f32(v0.clone());
    let v19: Adt12 = {
        let m37 = v7.clone();
        match m37 {
            Adt12::C0(_) => {
                v6
            }
            Adt12::C1(_, _) => {
                v6
            }
            Adt12::C2(_) => {
                v6
            }
            Adt12::C3(_) => {
                v6
            }
            Adt12::C4(_) => {
                v6
            }
            Adt12::C5 => {
                v6
            }
            Adt12::C6(_, _) => {
                v6
            }
            Adt12::C7(_, _, _) => {
                f8(Str::lit("import_lean"), f7(v7), v6)
            }
        }
    };
    let m39 = match v0 {
        Adt0::C0(_, _, _, _, h38, _, _, _, _, _, _, _, _) => {
            h38
        }
    };
    match m39 {
        None => {
            v19
        }
        Some(v20) => {
            f8(Str::lit("origin"), f9(v20), v19)
        }
    }
}

pub fn f13(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a40 = 0u64;
        let a41 = v1;
        nat_lt(a40, a41)
    } else {
        false
    } {
        let a42 = Bytes::lit(&[0x2d]);
        let a43 = f33(v1)?;
        Ok(append_bytes(a42, a43))
    } else {
        f33(v1)
    }
}

pub fn f14(v0: Bytes) -> R<Bytes> {
    let a49 = {
        let a47 = Bytes::lit(&[0x22]);
        let a48 = {
            let (_, h44) = {
                let (h45, _) = f34(Fn3::F35(v0.clone()), {
                    let a46 = v0;
                    length_bytes(a46)
                }, (0u64, Bytes::lit(&[])))?;
                h45
            };
            h44
        };
        append_bytes(a47, a48)
    };
    let a50 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a49, a50))
}

pub fn f15(v0: List<Bytes>) -> Bytes {
    let a53 = {
        let a51 = Bytes::lit(&[0x5b]);
        let a52 = f36(v0, Bytes::lit(&[0x2c]));
        append_bytes(a51, a52)
    };
    let a54 = Bytes::lit(&[0x5d]);
    append_bytes(a53, a54)
}

pub fn f16(v0: Adt12) -> R<List<Bytes>> {
    let m55 = v0;
    match m55 {
        Adt12::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C6(r56, r57) => {
            let v7 = (*r56).clone();
            let v8 = (*r57).clone();
            Ok(List::cons(f3(v7)?, f16(v8)?))
        }
        Adt12::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f17(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f37(Fn4::F38, List::<(Bytes, Bytes)>::nil(), f39(v0)?);
    let v2: List<Bytes> = f40(Fn5::F41, List::<Bytes>::nil(), v1)?;
    let a60 = {
        let a58 = Bytes::lit(&[0x7b]);
        let a59 = f36(v2, Bytes::lit(&[0x2c]));
        append_bytes(a58, a59)
    };
    let a61 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a60, a61))
}

pub fn f18(v0: Adt12) -> R<List<(Bytes, Bytes)>> {
    let m62 = v0;
    match m62 {
        Adt12::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C7(v9, r63, r64) => {
            let v10 = (*r63).clone();
            let v11 = (*r64).clone();
            Ok(List::cons((v9, f3(v10)?), f18(v11)?))
        }
    }
}

pub fn f19(v0: Fn0, p68: (List<Adt12>, Adt13), p69: List<Adt1>) -> R<(List<Adt12>, Adt13)> {
    let m67 = (p68, p69);
    {
        let mut m67 = m67;
        loop {
            let step = {
                let (v1, v2) = m67;
                let m65 = v2;
                match m65.uncons() {
                    None => {
                        Err::<((List<Adt12>, Adt13), List<Adt1>), R<(List<Adt12>, Adt13)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt12>, Adt13), List<Adt1>), R<(List<Adt12>, Adt13)>>(({
                            let c66: Fn0 = v0.clone();
                            c66.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m67 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: (List<Adt12>, Adt13), v1: Adt1) -> R<(List<Adt12>, Adt13)> {
    let v2: (Adt12, Adt13) = f42(v1, {
        let (_, h70) = v0;
        h70
    })?;
    Ok((List::cons({
        let (h71, _) = v2;
        h71
    }, {
        let (h72, _) = v0;
        h72
    }), {
        let (_, h73) = v2;
        h73
    }))
}

pub fn f21(v0: Fn1, p77: Adt12, p78: List<Adt12>) -> Adt12 {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, v2) = m76;
                let m74 = v2;
                match m74.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt12>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt12>), Adt12>(({
                            let c75: Fn1 = v0.clone();
                            c75.apply(v1, v3)
                        }, v4))
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

pub fn f22(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f23() -> Adt12 {
    Adt12::C5
}

pub fn f24(v0: Adt2, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m79 = v0;
    match m79 {
        Adt2::C0(v2) => {
            Ok((Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("sort")), f8(Str::lit("u"), f43(v2), Adt12::C5)))), v1))
        }
        Adt2::C1(v3) => {
            let v4: (Adt14, Adt13) = f44(v1, v3)?;
            Ok((f45({
                let (h80, _) = v4;
                h80
            }), {
                let (_, h81) = v4;
                h81
            }))
        }
        Adt2::C2(v5, v6) => {
            Ok((f46(v5, v6)?, v1))
        }
        Adt2::C3(r114, r115, v9) => {
            let v7 = (*r114).clone();
            let v8 = (*r115).clone();
            let v10: (Adt12, Adt13) = f24(v7, v1)?;
            let v11: (Adt12, Adt13) = f24(v8, {
                let (_, h82) = v10;
                h82
            })?;
            Ok((f47({
                let (h83, _) = v10;
                h83
            }, {
                let (h84, _) = v11;
                h84
            }, v9), {
                let (_, h85) = v11;
                h85
            }))
        }
        Adt2::C4(r116, r117) => {
            let v12 = (*r116).clone();
            let v13 = (*r117).clone();
            let v14: (Adt12, Adt13) = f24(v12, v1)?;
            let v15: (Adt12, Adt13) = f24(v13, {
                let (_, h86) = v14;
                h86
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("pi")), f8(Str::lit("b"), Adt12::C3(std::rc::Rc::new({
                let (h87, _) = v14;
                h87
            })), f8(Str::lit("v"), {
                let (h88, _) = v15;
                h88
            }, Adt12::C5))))), {
                let (_, h89) = v15;
                h89
            }))
        }
        Adt2::C5(r118, r119) => {
            let v16 = (*r118).clone();
            let v17 = (*r119).clone();
            let v18: (Adt12, Adt13) = f24(v16, v1)?;
            let v19: (Adt12, Adt13) = f24(v17, {
                let (_, h90) = v18;
                h90
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("lam")), f8(Str::lit("b"), Adt12::C3(std::rc::Rc::new({
                let (h91, _) = v18;
                h91
            })), f8(Str::lit("v"), {
                let (h92, _) = v19;
                h92
            }, Adt12::C5))))), {
                let (_, h93) = v19;
                h93
            }))
        }
        Adt2::C6(v20, v21, v22, r120, r121, r122) => {
            let v23 = (*r120).clone();
            let v24 = (*r121).clone();
            let v25 = (*r122).clone();
            let v26: (Adt12, Adt13) = f24(v23, v1)?;
            let v27: (Adt12, Adt13) = f24(v24, {
                let (_, h94) = v26;
                h94
            })?;
            let v28: (u64, Adt13) = f48({
                let (_, h95) = v27;
                h95
            }, v20)?;
            let v29: (Adt12, Adt13) = f24(v25, {
                let (_, h96) = v28;
                h96
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("let")), f8(Str::lit("b"), f49({
                let (h97, _) = v28;
                h97
            }, v21, v22, {
                let (h98, _) = v26;
                h98
            }), f8(Str::lit("x"), {
                let (h99, _) = v27;
                h99
            }, f8(Str::lit("v"), {
                let (h100, _) = v29;
                h100
            }, Adt12::C5)))))), {
                let (_, h101) = v29;
                h101
            }))
        }
        Adt2::C7(v30, r123) => {
            let v31 = (*r123).clone();
            let v32: (Adt12, Adt13) = f24(v31, v1)?;
            Ok((Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("nat")), f8(Str::lit("d"), Adt12::C2(v30), f8(Str::lit("t"), {
                let (h102, _) = v32;
                h102
            }, Adt12::C5))))), {
                let (_, h103) = v32;
                h103
            }))
        }
        Adt2::C8 => {
            Ok((Adt12::C5, v1))
        }
        Adt2::C9(r124, r125) => {
            let v33 = (*r124).clone();
            let v34 = (*r125).clone();
            let v35: (Adt12, Adt13) = f24(v33, v1)?;
            let v36: (Adt12, Adt13) = f24(v34, {
                let (_, h104) = v35;
                h104
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h105, _) = v35;
                h105
            }), std::rc::Rc::new({
                let (h106, _) = v36;
                h106
            })), {
                let (_, h107) = v36;
                h107
            }))
        }
        Adt2::C10(v37, v38, v39, r126, r127) => {
            let v40 = (*r126).clone();
            let v41 = (*r127).clone();
            let v42: (Adt12, Adt13) = f24(v40, v1)?;
            let v43: (u64, Adt13) = f48({
                let (_, h108) = v42;
                h108
            }, v37)?;
            let v44: (Adt12, Adt13) = f24(v41, {
                let (_, h109) = v43;
                h109
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f49({
                let (h110, _) = v43;
                h110
            }, v38, v39, {
                let (h111, _) = v42;
                h111
            })), std::rc::Rc::new({
                let (h112, _) = v44;
                h112
            })), {
                let (_, h113) = v44;
                h113
            }))
        }
    }
}

pub fn f25(v0: Str) -> Adt12 {
    Adt12::C2(f27(v0))
}

pub fn f26(v0: Adt7, v1: Adt13) -> R<(Adt12, Adt13)> {
    let v2: (Adt12, Adt13) = f24(match v0.clone() {
        Adt7::C0(_, _, _, h128, _, _) => {
            h128
        }
    }, v1)?;
    let v3: (Adt12, Adt13) = f50(match v0.clone() {
        Adt7::C0(_, _, _, _, h129, _) => {
            h129
        }
    }, {
        let (_, h130) = v2;
        h130
    })?;
    Ok((f51(f8(Str::lit("component"), Adt12::C2(match v0.clone() {
        Adt7::C0(h131, _, _, _, _, _) => {
            h131
        }
    }), f8(Str::lit("lean_name"), Adt12::C2(match v0.clone() {
        Adt7::C0(_, h132, _, _, _, _) => {
            h132
        }
    }), f8(Str::lit("kind"), f52(match v0.clone() {
        Adt7::C0(_, _, h133, _, _, _) => {
            h133
        }
    }), f8(Str::lit("params"), f53({
        let (h134, _) = v2;
        h134
    }), f8(Str::lit("body"), {
        let (h135, _) = v3;
        h135
    }, f8(Str::lit("policy"), f54(match v0 {
        Adt7::C0(_, _, _, _, _, h136) => {
            h136
        }
    }), f55()))))))), {
        let (_, h137) = v3;
        h137
    }))
}

pub fn f27(v0: Str) -> Bytes {
    let a138 = v0;
    utf8_encode(a138)
}

pub fn f28(v0: List<Adt12>) -> Adt12 {
    f21(Fn1::F56, f23(), f57(v0))
}

pub fn f29(v0: Fn2, p142: List<Adt12>, p143: List<Bytes>) -> List<Adt12> {
    let m141 = (p142, p143);
    {
        let mut m141 = m141;
        loop {
            let step = {
                let (v1, v2) = m141;
                let m139 = v2;
                match m139.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Bytes>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Bytes>), List<Adt12>>(({
                            let c140: Fn2 = v0.clone();
                            c140.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<Adt12>, v1: Bytes) -> List<Adt12> {
    List::cons(f9(v1), v0)
}

pub fn f31(v0: List<Bytes>) -> List<Bytes> {
    f58(Fn6::F59, List::<Bytes>::nil(), v0)
}

pub fn f32(v0: Adt0) -> Adt12 {
    f60(Fn7::F61(v0.clone()), f23(), match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h144, _, _, _, _, _) => {
            h144
        }
    })
}

pub fn f33(v0: u64) -> R<Bytes> {
    f62(f63(), 10u64, v0)
}

pub fn f34(v0: Fn3, p149: u64, p150: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m148 = (p149, p150);
    {
        let mut m148 = m148;
        loop {
            let step = {
                let (v1, v2) = m148;
                let m145 = v1;
                if m145 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m145 - 1;
                    let m147 = {
                        let c146: Fn3 = v0.clone();
                        c146.apply(v2.clone())?
                    };
                    match m147 {
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
                Ok(next) => { m148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f64(v0, v1)
}

pub fn f36(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m151 = f65(Fn8::F66(v1), None::<Bytes>, v0);
    match m151 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f37(v0: Fn4, p155: List<(Bytes, Bytes)>, p156: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m154 = (p155, p156);
    {
        let mut m154 = m154;
        loop {
            let step = {
                let (v1, v2) = m154;
                let m152 = v2;
                match m152.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c153: Fn4 = v0.clone();
                            c153.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f67(v0, v1)
}

pub fn f39(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f68(v0)
}

pub fn f40(v0: Fn5, p160: List<Bytes>, p161: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v1, v2) = m159;
                let m157 = v2;
                match m157.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c158: Fn5 = v0.clone();
                            c158.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f69(v1)?, v0))
}

pub fn f42(v0: Adt1, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m162 = v0;
    match m162 {
        Adt1::C0(v2, v3) => {
            Ok((f7(f8(Str::lit("k"), f25(Str::lit("word")), f8(Str::lit("entry"), f9(v2), f8(Str::lit("form"), f9(v3), f23())))), v1))
        }
        Adt1::C1(v4) => {
            let v5: (Adt12, Adt13) = f24(v4, v1)?;
            Ok((f7(f8(Str::lit("k"), f25(Str::lit("math")), f8(Str::lit("t"), {
                let (h163, _) = v5;
                h163
            }, f23()))), {
                let (_, h164) = v5;
                h164
            }))
        }
        Adt1::C2(v6) => {
            Ok((f7(f8(Str::lit("k"), f25(Str::lit("punct")), f8(Str::lit("entry"), f9(v6), f23()))), v1))
        }
    }
}

pub fn f43(v0: Adt3) -> Adt12 {
    let m165 = v0;
    match m165 {
        Adt3::C0(v1) => {
            Adt12::C4(std::rc::Rc::new(f8(Str::lit("num"), f70(v1), Adt12::C5)))
        }
        Adt3::C1(v2) => {
            Adt12::C4(std::rc::Rc::new(f8(Str::lit("var"), Adt12::C2(v2), Adt12::C5)))
        }
        Adt3::C2(r166) => {
            let v3 = (*r166).clone();
            Adt12::C4(std::rc::Rc::new(f8(Str::lit("succ"), f43(v3), Adt12::C5)))
        }
        Adt3::C3(r167) => {
            let v4 = (*r167).clone();
            Adt12::C4(std::rc::Rc::new(f8(Str::lit("max"), Adt12::C3(std::rc::Rc::new(f43(v4))), Adt12::C5)))
        }
        Adt3::C4(r168, r169) => {
            let v5 = (*r168).clone();
            let v6 = (*r169).clone();
            Adt12::C4(std::rc::Rc::new(f8(Str::lit("imax"), Adt12::C3(std::rc::Rc::new(Adt12::C6(std::rc::Rc::new(f43(v5)), std::rc::Rc::new(Adt12::C6(std::rc::Rc::new(f43(v6)), std::rc::Rc::new(Adt12::C5)))))), Adt12::C5)))
        }
        Adt3::C5 => {
            Adt12::C5
        }
        Adt3::C6(r170, r171) => {
            let v7 = (*r170).clone();
            let v8 = (*r171).clone();
            Adt12::C6(std::rc::Rc::new(f43(v7)), std::rc::Rc::new(f43(v8)))
        }
    }
}

pub fn f44(v0: Adt13, v1: u64) -> R<(Adt14, Adt13)> {
    let m173 = f71(match v0.clone() {
        Adt13::C0(h172, _, _) => {
            h172
        }
    }, v1);
    match m173 {
        None => {
            if match v0.clone() {
                Adt13::C0(_, _, h174) => {
                    h174
                }
            } {
                Ok((Adt14::C1(v1), v0))
            } else {
                let v3: (u64, Adt13) = f72(v0, v1)?;
                Ok((Adt14::C0({
                    let (h175, _) = v3;
                    h175
                }), {
                    let (_, h176) = v3;
                    h176
                }))
            }
        }
        Some(v2) => {
            Ok((Adt14::C0(v2), v0))
        }
    }
}

pub fn f45(v0: Adt14) -> Adt12 {
    let m177 = v0;
    match m177 {
        Adt14::C0(v1) => {
            Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("local")), f8(Str::lit("id"), f70(v1), Adt12::C5))))
        }
        Adt14::C1(v2) => {
            Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("local")), f8(Str::lit("free"), f70(v2), Adt12::C5))))
        }
    }
}

pub fn f46(v0: Adt4, v1: Adt3) -> R<Adt12> {
    let v2: Adt12 = if f73(v1.clone()) {
        Adt12::C5
    } else {
        f8(Str::lit("u"), Adt12::C3(std::rc::Rc::new(f43(v1))), Adt12::C5)
    };
    Ok(Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("global")), f8(Str::lit("g"), f74(v0)?, v2)))))
}

pub fn f47(v0: Adt12, v1: Adt12, v2: List<u64>) -> Adt12 {
    let v5: Adt12 = {
        let m178 = v2.clone();
        match m178.uncons() {
            None => {
                Adt12::C5
            }
            Some((_, _)) => {
                f8(Str::lit("i"), Adt12::C3(std::rc::Rc::new(f75(v2))), Adt12::C5)
            }
        }
    };
    Adt12::C4(std::rc::Rc::new(f8(Str::lit("k"), f25(Str::lit("app")), f8(Str::lit("f"), v0, f8(Str::lit("a"), Adt12::C3(std::rc::Rc::new(v1)), v5)))))
}

pub fn f48(v0: Adt13, v1: u64) -> R<(u64, Adt13)> {
    let m180 = f71(match v0.clone() {
        Adt13::C0(h179, _, _) => {
            h179
        }
    }, v1);
    match m180 {
        None => {
            f72(v0, v1)
        }
        Some(v2) => {
            Ok((v2, v0))
        }
    }
}

pub fn f49(v0: u64, v1: u64, v2: Bytes, v3: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(f8(Str::lit("id"), f70(v0), f8(Str::lit("m"), f76(v1), f8(Str::lit("s"), Adt12::C2(v2), f8(Str::lit("t"), v3, Adt12::C5))))))
}

pub fn f50(v0: Adt9, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m181 = v0;
    match m181 {
        Adt9::C0(v2, v3, v4) => {
            let v5: (Adt12, Adt13) = f24(v3, v1)?;
            let v6: (Adt12, Adt13) = f24(v4, {
                let (_, h182) = v5;
                h182
            })?;
            Ok((f77(Str::lit("definition"), f8(Str::lit("entry"), Adt12::C2(v2), f8(Str::lit("t"), {
                let (h183, _) = v5;
                h183
            }, f8(Str::lit("v"), {
                let (h184, _) = v6;
                h184
            }, f55())))), {
                let (_, h185) = v6;
                h185
            }))
        }
        Adt9::C1(v7, v8) => {
            let v9: (Adt12, Adt13) = f24(v7, v1)?;
            let v10: (Adt12, Adt13) = f78(v8, {
                let (_, h186) = v9;
                h186
            })?;
            Ok((f77(Str::lit("theorem"), f8(Str::lit("s"), {
                let (h187, _) = v9;
                h187
            }, f8(Str::lit("p"), {
                let (h188, _) = v10;
                h188
            }, f55()))), {
                let (_, h189) = v10;
                h189
            }))
        }
    }
}

pub fn f51(v0: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(v0))
}

pub fn f52(v0: Adt8) -> Adt12 {
    let m190 = v0;
    match m190 {
        Adt8::C0 => {
            f25(Str::lit("typedefinition"))
        }
        Adt8::C1 => {
            f25(Str::lit("termdefinition"))
        }
        Adt8::C2 => {
            f25(Str::lit("predicatedefinition"))
        }
        Adt8::C3 => {
            f25(Str::lit("theorem"))
        }
        Adt8::C4 => {
            f25(Str::lit("lemma"))
        }
        Adt8::C5 => {
            f25(Str::lit("corollary"))
        }
    }
}

pub fn f53(v0: Adt12) -> Adt12 {
    Adt12::C3(std::rc::Rc::new(v0))
}

pub fn f54(v0: Adt11) -> Adt12 {
    let m191 = v0;
    match m191 {
        Adt11::C0 => {
            f51(f8(Str::lit("kind"), f25(Str::lit("none")), f8(Str::lit("axioms"), f53(f55()), f55())))
        }
        Adt11::C1(v1) => {
            f51(f8(Str::lit("kind"), f25(Str::lit("allow")), f8(Str::lit("axioms"), f53(f79(v1)), f55())))
        }
        Adt11::C2(v2) => {
            f51(f8(Str::lit("kind"), f25(Str::lit("exact")), f8(Str::lit("axioms"), f53(f79(v2)), f55())))
        }
    }
}

pub fn f55() -> Adt12 {
    Adt12::C5
}

pub fn f56(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f57(v0: List<Adt12>) -> List<Adt12> {
    f80(Fn9::F81, List::<Adt12>::nil(), v0)
}

pub fn f58(v0: Fn6, p195: List<Bytes>, p196: List<Bytes>) -> List<Bytes> {
    let m194 = (p195, p196);
    {
        let mut m194 = m194;
        loop {
            let step = {
                let (v1, v2) = m194;
                let m192 = v2;
                match m192.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c193: Fn6 = v0.clone();
                            c193.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f60(v0: Fn7, p200: Adt12, p201: List<(Bytes, Bytes)>) -> Adt12 {
    let m199 = (p200, p201);
    {
        let mut m199 = m199;
        loop {
            let step = {
                let (v1, v2) = m199;
                let m197 = v2;
                match m197.uncons() {
                    None => {
                        Err::<(Adt12, List<(Bytes, Bytes)>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<(Bytes, Bytes)>), Adt12>(({
                            let c198: Fn7 = v0.clone();
                            c198.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt0, v1: Adt12, v2: (Bytes, Bytes)) -> Adt12 {
    let a204 = {
        let (_, h202) = v2.clone();
        h202
    };
    let a205 = f82(v0, {
        let (h203, _) = v2.clone();
        h203
    });
    if equal(a204, a205) {
        v1
    } else {
        Adt12::C7({
            let (h206, _) = v2;
            h206
        }, std::rc::Rc::new(f9({
            let (_, h207) = v2;
            h207
        })), std::rc::Rc::new(v1))
    }
}

pub fn f62(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a208 = v2;
    let a209 = 0u64;
    if nat_eq(a208, a209) {
        Ok(f83(v0, 0u64))
    } else {
        let (_, h210) = {
            let (h211, _) = f34(Fn3::F84(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h211
        };
        Ok(h210)
    }
}

pub fn f63() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f64(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m215 = {
        let a213 = v0.clone();
        let a214 = {
            let (h212, _) = v1.clone();
            h212
        };
        index_bytes(a213, a214)
    };
    match m215 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a217 = {
                    let (h216, _) = v1.clone();
                    h216
                };
                let a218 = 1u64;
                nat_add(a217, a218)?
            }, {
                let a221 = {
                    let (_, h219) = v1;
                    h219
                };
                let a222 = f85(v0, {
                    let (h220, _) = v1;
                    h220
                }, v2)?;
                append_bytes(a221, a222)
            })))
        }
    }
}

pub fn f65(v0: Fn8, p226: Option<Bytes>, p227: List<Bytes>) -> Option<Bytes> {
    let m225 = (p226, p227);
    {
        let mut m225 = m225;
        loop {
            let step = {
                let (v1, v2) = m225;
                let m223 = v2;
                match m223.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c224: Fn8 = v0.clone();
                            c224.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m228 = v1;
    match m228 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a231 = {
                    let a229 = v3;
                    let a230 = v0;
                    append_bytes(a229, a230)
                };
                let a232 = v2;
                append_bytes(a231, a232)
            })
        }
    }
}

pub fn f67(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m233 = v0.clone();
    match m233.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a236 = {
                let (h234, _) = v2;
                h234
            };
            let a237 = {
                let (h235, _) = v1.clone();
                h235
            };
            if equal(a236, a237) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f68(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f86(f87(Fn10::F88, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m242 = {
        let (h238, _) = f89(Fn11::F90, {
            let a240 = {
                let a239 = v0;
                length_list(a239)
            };
            let a241 = 1u64;
            nat_add(a240, a241)?
        }, v1)?;
        h238
    };
    match m242.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f69(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a247 = {
        let a244 = f14({
            let (h243, _) = v0;
            h243
        })?;
        let a245 = Bytes::lit(&[0x3a]);
        append_bytes(a244, a245)
    };
    let a248 = {
        let (_, h246) = v0;
        h246
    };
    Ok(append_bytes(a247, a248))
}

pub fn f70(v0: u64) -> Adt12 {
    Adt12::C1(false, f91(v0))
}

pub fn f71(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f92(Fn12::F93(v1), None::<u64>, v0)
}

pub fn f72(v0: Adt13, v1: u64) -> R<(u64, Adt13)> {
    Ok((match v0.clone() {
        Adt13::C0(_, h249, _) => {
            h249
        }
    }, Adt13::C0(List::cons((v1, match v0.clone() {
        Adt13::C0(_, h250, _) => {
            h250
        }
    }), match v0.clone() {
        Adt13::C0(h251, _, _) => {
            h251
        }
    }), {
        let a253 = match v0.clone() {
            Adt13::C0(_, h252, _) => {
                h252
            }
        };
        let a254 = 1u64;
        nat_add(a253, a254)?
    }, match v0 {
        Adt13::C0(_, _, h255) => {
            h255
        }
    })))
}

pub fn f73(v0: Adt3) -> bool {
    let m256 = v0;
    match m256 {
        Adt3::C0(_) => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_) => {
            false
        }
        Adt3::C4(_, _) => {
            false
        }
        Adt3::C5 => {
            true
        }
        Adt3::C6(_, _) => {
            false
        }
    }
}

pub fn f74(v0: Adt4) -> R<Adt12> {
    let m257 = v0;
    match m257 {
        Adt4::C0(v1) => {
            Ok(Adt12::C4(std::rc::Rc::new(f8(Str::lit("kind"), f25(Str::lit("core")), f8(Str::lit("name"), Adt12::C2(f94(v1)), Adt12::C5)))))
        }
        Adt4::C1(_, v3, v4, v5, v6) => {
            Ok(Adt12::C4(std::rc::Rc::new(f8(Str::lit("kind"), f25(Str::lit("external")), f8(Str::lit("entry"), Adt12::C2(v3), f8(Str::lit("lean_module"), Adt12::C2(v4), f8(Str::lit("lean_name"), Adt12::C2(v5), f8(Str::lit("signature_sha256"), Adt12::C2(f95(v6)?), Adt12::C5))))))))
        }
        Adt4::C2(v7, v8, v9) => {
            Ok(Adt12::C4(std::rc::Rc::new(f8(Str::lit("kind"), f25(Str::lit("document")), f8(Str::lit("module"), Adt12::C2(v7), f8(Str::lit("component"), Adt12::C2(v8), f8(Str::lit("lean_name"), Adt12::C2(v9), Adt12::C5)))))))
        }
        Adt4::C3(_, v11, v12) => {
            Ok(Adt12::C4(std::rc::Rc::new(f8(Str::lit("kind"), f25(Str::lit("defined")), f8(Str::lit("entry"), Adt12::C2(v11), f8(Str::lit("signature_sha256"), Adt12::C2(f95(v12)?), Adt12::C5))))))
        }
    }
}

pub fn f75(v0: List<u64>) -> Adt12 {
    f96(Fn13::F97, Adt12::C5, f98(v0))
}

pub fn f76(v0: u64) -> Adt12 {
    Adt12::C2(f99(v0))
}

pub fn f77(v0: Str, v1: Adt12) -> Adt12 {
    f51(f8(Str::lit("k"), f25(v0), v1))
}

pub fn f78(v0: Adt10, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m258 = v0;
    match m258 {
        Adt10::C0(r321) => {
            let v2 = (*r321).clone();
            let v3: (Adt12, Adt13) = f78(v2, v1)?;
            Ok((f77(Str::lit("seq"), f8(Str::lit("s"), f53({
                let (h259, _) = v3;
                h259
            }), f55())), {
                let (_, h260) = v3;
                h260
            }))
        }
        Adt10::C1(v4) => {
            let v5: (Adt12, Adt13) = f100(v4, v1)?;
            Ok((f77(Str::lit("intro"), f8(Str::lit("l"), f53({
                let (h261, _) = v5;
                h261
            }), f55())), {
                let (_, h262) = v5;
                h262
            }))
        }
        Adt10::C2(v6) => {
            let v7: (Adt12, Adt13) = f101(Str::lit("t"), v6, v1, f55())?;
            Ok((f77(Str::lit("exact"), {
                let (h263, _) = v7;
                h263
            }), {
                let (_, h264) = v7;
                h264
            }))
        }
        Adt10::C3(v8) => {
            let v9: (Adt12, Adt13) = f101(Str::lit("t"), v8, v1, f55())?;
            Ok((f77(Str::lit("apply-one"), {
                let (h265, _) = v9;
                h265
            }), {
                let (_, h266) = v9;
                h266
            }))
        }
        Adt10::C4(v10, r322) => {
            let v11 = (*r322).clone();
            let v12: (Adt12, Adt13) = f24(v10, v1)?;
            let v13: (Adt12, Adt13) = f78(v11, {
                let (_, h267) = v12;
                h267
            })?;
            Ok((f77(Str::lit("apply"), f8(Str::lit("f"), {
                let (h268, _) = v12;
                h268
            }, f8(Str::lit("p"), f53({
                let (h269, _) = v13;
                h269
            }), f55()))), {
                let (_, h270) = v13;
                h270
            }))
        }
        Adt10::C5 => {
            Ok((f77(Str::lit("rfl"), f55()), v1))
        }
        Adt10::C6(v14) => {
            let v15: (Adt12, Adt13) = f101(Str::lit("t"), v14, v1, f55())?;
            Ok((f77(Str::lit("witness"), {
                let (h271, _) = v15;
                h271
            }), {
                let (_, h272) = v15;
                h272
            }))
        }
        Adt10::C7 => {
            Ok((f77(Str::lit("left"), f55()), v1))
        }
        Adt10::C8 => {
            Ok((f77(Str::lit("right"), f55()), v1))
        }
        Adt10::C9(v16, v17, r323) => {
            let v18 = (*r323).clone();
            let v19: (Adt12, Adt13) = f24(v17, v1)?;
            let v20: (u64, Adt13) = f48({
                let (_, h273) = v19;
                h273
            }, v16)?;
            let v21: (Adt12, Adt13) = f78(v18, {
                let (_, h274) = v20;
                h274
            })?;
            Ok((f77(Str::lit("have"), f8(Str::lit("l"), f70({
                let (h275, _) = v20;
                h275
            }), f8(Str::lit("p"), {
                let (h276, _) = v19;
                h276
            }, f8(Str::lit("q"), {
                let (h277, _) = v21;
                h277
            }, f55())))), {
                let (_, h278) = v21;
                h278
            }))
        }
        Adt10::C10(v22, r324) => {
            let v23 = (*r324).clone();
            let v24: (Adt12, Adt13) = f102(v22, v1)?;
            let v25: (Adt12, Adt13) = f78(v23, {
                let (_, h279) = v24;
                h279
            })?;
            Ok((f77(Str::lit("rw"), f8(Str::lit("at"), {
                let (h280, _) = v24;
                h280
            }, f8(Str::lit("r"), f53({
                let (h281, _) = v25;
                h281
            }), f55()))), {
                let (_, h282) = v25;
                h282
            }))
        }
        Adt10::C11(v26, v27) => {
            let v28: (Adt12, Adt13) = f102(v26, v1)?;
            let v29: (Adt12, Adt13) = f24(v27, {
                let (_, h283) = v28;
                h283
            })?;
            Ok((f77(Str::lit("simp-only"), f8(Str::lit("at"), {
                let (h284, _) = v28;
                h284
            }, f8(Str::lit("r"), f53({
                let (h285, _) = v29;
                h285
            }), f55()))), {
                let (_, h286) = v29;
                h286
            }))
        }
        Adt10::C12(r325) => {
            let v30 = (*r325).clone();
            let v31: (Adt12, Adt13) = f78(v30, v1)?;
            Ok((f77(Str::lit("constructor"), f8(Str::lit("b"), f53({
                let (h287, _) = v31;
                h287
            }), f55())), {
                let (_, h288) = v31;
                h288
            }))
        }
        Adt10::C13(v32, r326) => {
            let v33 = (*r326).clone();
            let v34: (Adt12, Adt13) = f24(v32, v1)?;
            let v35: (Adt12, Adt13) = f78(v33, {
                let (_, h289) = v34;
                h289
            })?;
            Ok((f77(Str::lit("cases"), f8(Str::lit("t"), {
                let (h290, _) = v34;
                h290
            }, f8(Str::lit("c"), f53({
                let (h291, _) = v35;
                h291
            }), f55()))), {
                let (_, h292) = v35;
                h292
            }))
        }
        Adt10::C14(v36, r327) => {
            let v37 = (*r327).clone();
            let v38: (Adt12, Adt13) = f24(v36, v1)?;
            let v39: (Adt12, Adt13) = f78(v37, {
                let (_, h293) = v38;
                h293
            })?;
            Ok((f77(Str::lit("induction"), f8(Str::lit("t"), {
                let (h294, _) = v38;
                h294
            }, f8(Str::lit("c"), f53({
                let (h295, _) = v39;
                h295
            }), f55()))), {
                let (_, h296) = v39;
                h296
            }))
        }
        Adt10::C15(v40, v41, r328) => {
            let v42 = (*r328).clone();
            let v43: (Adt12, Adt13) = f24(v41, v1)?;
            let v44: (Adt12, Adt13) = f78(v42, {
                let (_, h297) = v43;
                h297
            })?;
            Ok((f77(Str::lit("calc"), f8(Str::lit("rel"), f46(v40, Adt3::C5)?, f8(Str::lit("s"), {
                let (h298, _) = v43;
                h298
            }, f8(Str::lit("steps"), f53({
                let (h299, _) = v44;
                h299
            }), f55())))), {
                let (_, h300) = v44;
                h300
            }))
        }
        Adt10::C16 => {
            Ok((f55(), v1))
        }
        Adt10::C17(r329, r330) => {
            let v45 = (*r329).clone();
            let v46 = (*r330).clone();
            let v47: (Adt12, Adt13) = f78(v45, v1)?;
            let v48: (Adt12, Adt13) = f78(v46, {
                let (_, h301) = v47;
                h301
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h302, _) = v47;
                h302
            }), std::rc::Rc::new({
                let (h303, _) = v48;
                h303
            })), {
                let (_, h304) = v48;
                h304
            }))
        }
        Adt10::C18(v49, v50, r331) => {
            let v51 = (*r331).clone();
            let v52: (Adt12, Adt13) = f24(v50, v1)?;
            let v53: (Adt12, Adt13) = f78(v51, {
                let (_, h305) = v52;
                h305
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f51(f8(Str::lit("rev"), Adt12::C0(v49), f8(Str::lit("t"), {
                let (h306, _) = v52;
                h306
            }, f55())))), std::rc::Rc::new({
                let (h307, _) = v53;
                h307
            })), {
                let (_, h308) = v53;
                h308
            }))
        }
        Adt10::C19(v54, v55, v56, r332, r333) => {
            let v57 = (*r332).clone();
            let v58 = (*r333).clone();
            let v59: (Adt12, Adt13) = f100(f103(v56), v1)?;
            let v60: (Adt12, Adt13) = f78(v57, {
                let (_, h309) = v59;
                h309
            })?;
            let v61: (Adt12, Adt13) = f78(v58, {
                let (_, h310) = v60;
                h310
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f51(f8(Str::lit("ctor"), Adt12::C2(v54), f8(Str::lit("lean"), Adt12::C2(v55), f8(Str::lit("b"), f53({
                let (h311, _) = v59;
                h311
            }), f8(Str::lit("p"), {
                let (h312, _) = v60;
                h312
            }, f55())))))), std::rc::Rc::new({
                let (h313, _) = v61;
                h313
            })), {
                let (_, h314) = v61;
                h314
            }))
        }
        Adt10::C20(v62, v63, r334) => {
            let v64 = (*r334).clone();
            let v65: (Adt12, Adt13) = f24(v62, v1)?;
            let v66: (Adt12, Adt13) = f24(v63, {
                let (_, h315) = v65;
                h315
            })?;
            let v67: (Adt12, Adt13) = f78(v64, {
                let (_, h316) = v66;
                h316
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f51(f8(Str::lit("t"), {
                let (h317, _) = v65;
                h317
            }, f8(Str::lit("p"), {
                let (h318, _) = v66;
                h318
            }, f55())))), std::rc::Rc::new({
                let (h319, _) = v67;
                h319
            })), {
                let (_, h320) = v67;
                h320
            }))
        }
    }
}

pub fn f79(v0: List<Bytes>) -> Adt12 {
    f104(Fn14::F105, f55(), f31(v0))
}

pub fn f80(v0: Fn9, p338: List<Adt12>, p339: List<Adt12>) -> List<Adt12> {
    let m337 = (p338, p339);
    {
        let mut m337 = m337;
        loop {
            let step = {
                let (v1, v2) = m337;
                let m335 = v2;
                match m335.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c336: Fn9 = v0.clone();
                            c336.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    List::cons(v1, v0)
}

pub fn f82(v0: Adt0, v1: Bytes) -> Bytes {
    let v2: Bytes = match v0.clone() {
        Adt0::C0(_, h340, _, _, _, _, _, _, _, _, _, _, _) => {
            h340
        }
    };
    let v3: u64 = {
        let a342 = match v0.clone() {
            Adt0::C0(h341, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h341
            }
        };
        length_bytes(a342)
    };
    let v4: u64 = {
        let a343 = v2.clone();
        length_bytes(a343)
    };
    let v5: Bytes = if f106(v2.clone(), match v0 {
        Adt0::C0(h344, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h344
        }
    }) {
        f107(v2, 0u64, {
            let a345 = v4;
            let a346 = v3;
            nat_sub(a345, a346)
        })
    } else {
        Bytes::lit(&[])
    };
    let a347 = v5;
    let a348 = v1;
    append_bytes(a347, a348)
}

pub fn f83(v0: Bytes, v1: u64) -> Bytes {
    let m352 = {
        let a349 = v0;
        let a350 = v1;
        let a351 = 1u64;
        slice_bytes(a349, a350, a351)
    };
    match m352 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f84(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f108(v1, v0, v2)
}

pub fn f85(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a397 = {
            let a373 = {
                let a361 = {
                    let a355 = {
                        let a353 = v3;
                        let a354 = 128u8;
                        fixed_u8::bit_and(a353, a354)
                    };
                    let a356 = 128u8;
                    if equal(a355, a356) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a362 = {
                    let a359 = {
                        let a357 = v3;
                        let a358 = 64u8;
                        fixed_u8::bit_and(a357, a358)
                    };
                    let a360 = 64u8;
                    if equal(a359, a360) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a361, a362)?
            };
            let a374 = {
                let a371 = {
                    let a365 = {
                        let a363 = v3;
                        let a364 = 32u8;
                        fixed_u8::bit_and(a363, a364)
                    };
                    let a366 = 32u8;
                    if equal(a365, a366) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a372 = {
                    let a369 = {
                        let a367 = v3;
                        let a368 = 16u8;
                        fixed_u8::bit_and(a367, a368)
                    };
                    let a370 = 16u8;
                    if equal(a369, a370) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a371, a372)?
            };
            nat_add(a373, a374)?
        };
        let a398 = {
            let a395 = {
                let a383 = {
                    let a377 = {
                        let a375 = v3;
                        let a376 = 8u8;
                        fixed_u8::bit_and(a375, a376)
                    };
                    let a378 = 8u8;
                    if equal(a377, a378) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a384 = {
                    let a381 = {
                        let a379 = v3;
                        let a380 = 4u8;
                        fixed_u8::bit_and(a379, a380)
                    };
                    let a382 = 4u8;
                    if equal(a381, a382) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a383, a384)?
            };
            let a396 = {
                let a393 = {
                    let a387 = {
                        let a385 = v3;
                        let a386 = 2u8;
                        fixed_u8::bit_and(a385, a386)
                    };
                    let a388 = 2u8;
                    if equal(a387, a388) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a394 = {
                    let a391 = {
                        let a389 = v3;
                        let a390 = 1u8;
                        fixed_u8::bit_and(a389, a390)
                    };
                    let a392 = 1u8;
                    if equal(a391, a392) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a393, a394)?
            };
            nat_add(a395, a396)?
        };
        nat_add(a397, a398)?
    };
    let a399 = v4;
    let a400 = 34u64;
    if nat_eq(a399, a400) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a401 = v4;
        let a402 = 92u64;
        if nat_eq(a401, a402) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a403 = v4;
            let a404 = 8u64;
            if nat_eq(a403, a404) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a405 = v4;
                let a406 = 12u64;
                if nat_eq(a405, a406) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a407 = v4;
                    let a408 = 10u64;
                    if nat_eq(a407, a408) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a409 = v4;
                        let a410 = 13u64;
                        if nat_eq(a409, a410) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a411 = v4;
                            let a412 = 9u64;
                            if nat_eq(a411, a412) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a413 = v4;
                                let a414 = 32u64;
                                if nat_lt(a413, a414) {
                                    let a415 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a416 = f109(v2)?;
                                    Ok(append_bytes(a415, a416))
                                } else {
                                    Ok(f107(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f86(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f110(Fn15::F111, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f87(v0: Fn10, p420: List<List<(Bytes, Bytes)>>, p421: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m419 = (p420, p421);
    {
        let mut m419 = m419;
        loop {
            let step = {
                let (v1, v2) = m419;
                let m417 = v2;
                match m417.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c418: Fn10 = v0.clone();
                            c418.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f89(v0: Fn11, p426: u64, p427: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m425 = (p426, p427);
    {
        let mut m425 = m425;
        loop {
            let step = {
                let (v1, v2) = m425;
                let m422 = v1;
                if m422 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m422 - 1;
                    let m424 = {
                        let c423: Fn11 = v0.clone();
                        c423.apply(v2.clone())?
                    };
                    match m424 {
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
                Ok(next) => { m425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f112(v0)
}

pub fn f91(v0: u64) -> u64 {
    let a428 = v0;
    let a429 = 9223372036854775807u64;
    if nat_le(a428, a429) {
        v0
    } else {
        9223372036854775807u64
    }
}

pub fn f92(v0: Fn12, p433: Option<u64>, p434: List<(u64, u64)>) -> Option<u64> {
    let m432 = (p433, p434);
    {
        let mut m432 = m432;
        loop {
            let step = {
                let (v1, v2) = m432;
                let m430 = v2;
                match m430.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c431: Fn12 = v0.clone();
                            c431.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let a436 = {
        let (h435, _) = v2;
        h435
    };
    let a437 = v0;
    if nat_eq(a436, a437) {
        Some({
            let (_, h438) = v2;
            h438
        })
    } else {
        v1
    }
}

pub fn f94(v0: Adt5) -> Bytes {
    let m439 = v0;
    match m439 {
        Adt5::C0 => {
            f27(Str::lit("eq"))
        }
        Adt5::C1 => {
            f27(Str::lit("and"))
        }
        Adt5::C2 => {
            f27(Str::lit("or"))
        }
        Adt5::C3 => {
            f27(Str::lit("not"))
        }
        Adt5::C4 => {
            f27(Str::lit("iff"))
        }
        Adt5::C5 => {
            f27(Str::lit("exists"))
        }
        Adt5::C6 => {
            f27(Str::lit("exists-unique"))
        }
    }
}

pub fn f95(v0: Bytes) -> R<Bytes> {
    let (_, h440) = {
        let (h441, _) = f34(Fn3::F113(v0.clone()), {
            let a442 = v0;
            length_bytes(a442)
        }, (0u64, Bytes::lit(&[])))?;
        h441
    };
    Ok(h440)
}

pub fn f96(v0: Fn13, p446: Adt12, p447: List<u64>) -> Adt12 {
    let m445 = (p446, p447);
    {
        let mut m445 = m445;
        loop {
            let step = {
                let (v1, v2) = m445;
                let m443 = v2;
                match m443.uncons() {
                    None => {
                        Err::<(Adt12, List<u64>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<u64>), Adt12>(({
                            let c444: Fn13 = v0.clone();
                            c444.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Adt12, v1: u64) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(f70(v1)), std::rc::Rc::new(v0))
}

pub fn f98(v0: List<u64>) -> List<u64> {
    f114(Fn16::F115, List::<u64>::nil(), v0)
}

pub fn f99(v0: u64) -> Bytes {
    let a448 = v0;
    let a449 = 1u64;
    if nat_eq(a448, a449) {
        f116(Str::lit("implicit"))
    } else {
        let a450 = v0;
        let a451 = 2u64;
        if nat_eq(a450, a451) {
            f116(Str::lit("instance"))
        } else {
            f116(Str::lit("explicit"))
        }
    }
}

pub fn f100(v0: List<u64>, v1: Adt13) -> R<(Adt12, Adt13)> {
    let v2: (List<u64>, Adt13) = f117(Fn17::F118, (List::<u64>::nil(), v1), v0)?;
    Ok((f96(Fn13::F119, f55(), {
        let (h452, _) = v2;
        h452
    }), {
        let (_, h453) = v2;
        h453
    }))
}

pub fn f101(v0: Str, v1: Adt2, v2: Adt13, v3: Adt12) -> R<(Adt12, Adt13)> {
    let v4: (Adt12, Adt13) = f24(v1, v2)?;
    Ok((f8(v0, {
        let (h454, _) = v4;
        h454
    }, v3), {
        let (_, h455) = v4;
        h455
    }))
}

pub fn f102(v0: Option<u64>, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m456 = v0;
    match m456 {
        None => {
            Ok((f25(Str::lit("goal")), v1))
        }
        Some(v2) => {
            let v3: (Adt14, Adt13) = f44(v1, v2)?;
            Ok(({
                let m458 = {
                    let (h457, _) = v3;
                    h457
                };
                match m458 {
                    Adt14::C0(v4) => {
                        f70(v4)
                    }
                    Adt14::C1(v5) => {
                        f70(v5)
                    }
                }
            }, {
                let (_, h459) = v3;
                h459
            }))
        }
    }
}

pub fn f103(v0: List<(u64, Bytes)>) -> List<u64> {
    f98(f120(Fn18::F121, List::<u64>::nil(), v0))
}

pub fn f104(v0: Fn14, p463: Adt12, p464: List<Bytes>) -> Adt12 {
    let m462 = (p463, p464);
    {
        let mut m462 = m462;
        loop {
            let step = {
                let (v1, v2) = m462;
                let m460 = v2;
                match m460.uncons() {
                    None => {
                        Err::<(Adt12, List<Bytes>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Bytes>), Adt12>(({
                            let c461: Fn14 = v0.clone();
                            c461.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Adt12, v1: Bytes) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(Adt12::C2(v1)), std::rc::Rc::new(v0))
}

pub fn f106(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a465 = v0.clone();
        length_bytes(a465)
    };
    let v3: u64 = {
        let a466 = v1.clone();
        length_bytes(a466)
    };
    let a467 = v3;
    let a468 = v2;
    if nat_le(a467, a468) {
        let m474 = {
            let a471 = v0;
            let a472 = {
                let a469 = v2;
                let a470 = v3;
                nat_sub(a469, a470)
            };
            let a473 = v3;
            slice_bytes(a471, a472, a473)
        };
        match m474 {
            None => {
                false
            }
            Some(v4) => {
                let a475 = v4;
                let a476 = v1;
                equal(a475, a476)
            }
        }
    } else {
        false
    }
}

pub fn f107(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m480 = {
        let a477 = v0;
        let a478 = v1;
        let a479 = v2;
        slice_bytes(a477, a478, a479)
    };
    match m480 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f108(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h481, _) = v2;
        h481
    };
    let a482 = v3;
    let a483 = 0u64;
    if nat_eq(a482, a483) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a484 = v3;
            let a485 = v1;
            let a486 = 0u64;
            nat_quot(a484, a485, a486)
        }, {
            let a491 = f83(v0, {
                let a487 = v3;
                let a488 = v1;
                let a489 = 0u64;
                nat_rem(a487, a488, a489)
            });
            let a492 = {
                let (_, h490) = v2;
                h490
            };
            append_bytes(a491, a492)
        }))
    }
}

pub fn f109(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a537 = {
            let a513 = {
                let a501 = {
                    let a495 = {
                        let a493 = v1;
                        let a494 = 128u8;
                        fixed_u8::bit_and(a493, a494)
                    };
                    let a496 = 128u8;
                    if equal(a495, a496) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a502 = {
                    let a499 = {
                        let a497 = v1;
                        let a498 = 64u8;
                        fixed_u8::bit_and(a497, a498)
                    };
                    let a500 = 64u8;
                    if equal(a499, a500) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a501, a502)?
            };
            let a514 = {
                let a511 = {
                    let a505 = {
                        let a503 = v1;
                        let a504 = 32u8;
                        fixed_u8::bit_and(a503, a504)
                    };
                    let a506 = 32u8;
                    if equal(a505, a506) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a512 = {
                    let a509 = {
                        let a507 = v1;
                        let a508 = 16u8;
                        fixed_u8::bit_and(a507, a508)
                    };
                    let a510 = 16u8;
                    if equal(a509, a510) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a511, a512)?
            };
            nat_add(a513, a514)?
        };
        let a538 = {
            let a535 = {
                let a523 = {
                    let a517 = {
                        let a515 = v1;
                        let a516 = 8u8;
                        fixed_u8::bit_and(a515, a516)
                    };
                    let a518 = 8u8;
                    if equal(a517, a518) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a524 = {
                    let a521 = {
                        let a519 = v1;
                        let a520 = 4u8;
                        fixed_u8::bit_and(a519, a520)
                    };
                    let a522 = 4u8;
                    if equal(a521, a522) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a523, a524)?
            };
            let a536 = {
                let a533 = {
                    let a527 = {
                        let a525 = v1;
                        let a526 = 2u8;
                        fixed_u8::bit_and(a525, a526)
                    };
                    let a528 = 2u8;
                    if equal(a527, a528) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a534 = {
                    let a531 = {
                        let a529 = v1;
                        let a530 = 1u8;
                        fixed_u8::bit_and(a529, a530)
                    };
                    let a532 = 1u8;
                    if equal(a531, a532) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a533, a534)?
            };
            nat_add(a535, a536)?
        };
        nat_add(a537, a538)?
    };
    let a545 = f83(f122(), {
        let a539 = v2;
        let a540 = 16u64;
        let a541 = 0u64;
        nat_quot(a539, a540, a541)
    });
    let a546 = f83(f122(), {
        let a542 = v2;
        let a543 = 16u64;
        let a544 = 0u64;
        nat_rem(a542, a543, a544)
    });
    Ok(append_bytes(a545, a546))
}

pub fn f110(v0: Fn15, p550: List<List<(Bytes, Bytes)>>, p551: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m549 = (p550, p551);
    {
        let mut m549 = m549;
        loop {
            let step = {
                let (v1, v2) = m549;
                let m547 = v2;
                match m547.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c548: Fn15 = v0.clone();
                            c548.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f112(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a553 = {
        let a552 = v0.clone();
        length_list(a552)
    };
    let a554 = 1u64;
    if nat_le(a553, a554) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f123(v0)?))
    }
}

pub fn f113(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f124(v0, v1)
}

pub fn f114(v0: Fn16, p558: List<u64>, p559: List<u64>) -> List<u64> {
    let m557 = (p558, p559);
    {
        let mut m557 = m557;
        loop {
            let step = {
                let (v1, v2) = m557;
                let m555 = v2;
                match m555.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c556: Fn16 = v0.clone();
                            c556.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f116(v0: Str) -> Bytes {
    let a560 = v0;
    utf8_encode(a560)
}

pub fn f117(v0: Fn17, p564: (List<u64>, Adt13), p565: List<u64>) -> R<(List<u64>, Adt13)> {
    let m563 = (p564, p565);
    {
        let mut m563 = m563;
        loop {
            let step = {
                let (v1, v2) = m563;
                let m561 = v2;
                match m561.uncons() {
                    None => {
                        Err::<((List<u64>, Adt13), List<u64>), R<(List<u64>, Adt13)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<u64>, Adt13), List<u64>), R<(List<u64>, Adt13)>>(({
                            let c562: Fn17 = v0.clone();
                            c562.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: (List<u64>, Adt13), v1: u64) -> R<(List<u64>, Adt13)> {
    let v2: (u64, Adt13) = f48({
        let (_, h566) = v0;
        h566
    }, v1)?;
    Ok((List::cons({
        let (h567, _) = v2;
        h567
    }, {
        let (h568, _) = v0;
        h568
    }), {
        let (_, h569) = v2;
        h569
    }))
}

pub fn f119(v0: Adt12, v1: u64) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(f70(v1)), std::rc::Rc::new(v0))
}

pub fn f120(v0: Fn18, p573: List<u64>, p574: List<(u64, Bytes)>) -> List<u64> {
    let m572 = (p573, p574);
    {
        let mut m572 = m572;
        loop {
            let step = {
                let (v1, v2) = m572;
                let m570 = v2;
                match m570.uncons() {
                    None => {
                        Err::<(List<u64>, List<(u64, Bytes)>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<(u64, Bytes)>), List<u64>>(({
                            let c571: Fn18 = v0.clone();
                            c571.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: List<u64>, v1: (u64, Bytes)) -> List<u64> {
    List::cons({
        let (h575, _) = v1;
        h575
    }, v0)
}

pub fn f122() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f123(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f86({
        let (_, h576) = {
            let (h577, _) = f125(Fn19::F126, {
                let a579 = {
                    let a578 = v0.clone();
                    length_list(a578)
                };
                let a580 = 1u64;
                nat_add(a579, a580)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h577
        };
        h576
    }))
}

pub fn f124(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m584 = {
        let a582 = v0;
        let a583 = {
            let (h581, _) = v1.clone();
            h581
        };
        index_bytes(a582, a583)
    };
    match m584 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a586 = {
                    let (h585, _) = v1;
                    h585
                };
                let a587 = 1u64;
                nat_add(a586, a587)?
            }, {
                let a589 = {
                    let (_, h588) = v1;
                    h588
                };
                let a590 = f109(v2)?;
                append_bytes(a589, a590)
            })))
        }
    }
}

pub fn f125(v0: Fn19, p595: u64, p596: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m594 = (p595, p596);
    {
        let mut m594 = m594;
        loop {
            let step = {
                let (v1, v2) = m594;
                let m591 = v1;
                if m591 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m591 - 1;
                    let m593 = {
                        let c592: Fn19 = v0.clone();
                        c592.apply(v2.clone())?
                    };
                    match m593 {
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
                Ok(next) => { m594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f127(v0)
}

pub fn f127(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h597, _) = v0;
        h597
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h598) = v0;
        h598
    };
    let m599 = v1;
    match m599.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m600 = v4;
            match m600.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f128(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f128(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a605 = {
            let a603 = {
                let a601 = v0.clone();
                length_list(a601)
            };
            let a604 = {
                let a602 = v1.clone();
                length_list(a602)
            };
            nat_add(a603, a604)?
        };
        let a606 = 1u64;
        nat_add(a605, a606)?
    };
    Ok(f129({
        let (_, h607) = {
            let (_, h608) = {
                let (h609, _) = f130(Fn20::F131, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h609
            };
            h608
        };
        h607
    }))
}

pub fn f129(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f37(Fn4::F132, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f130(v0: Fn20, p614: u64, p615: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m613 = (p614, p615);
    {
        let mut m613 = m613;
        loop {
            let step = {
                let (v1, v2) = m613;
                let m610 = v1;
                if m610 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m610 - 1;
                    let m612 = {
                        let c611: Fn20 = v0.clone();
                        c611.apply(v2.clone())
                    };
                    match m612 {
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
                Ok(next) => { m613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f133(v0)
}

pub fn f132(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f133(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h616, _) = v0.clone();
        h616
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h617, _) = {
            let (_, h618) = v0.clone();
            h618
        };
        h617
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h619) = {
            let (_, h620) = v0;
            h620
        };
        h619
    };
    let m621 = v1.clone();
    match m621.uncons() {
        None => {
            let m622 = v2;
            match m622.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m623 = v2.clone();
            match m623.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f134({
                        let (h624, _) = v8.clone();
                        h624
                    }, {
                        let (h625, _) = v6.clone();
                        h625
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

pub fn f134(v0: Bytes, v1: Bytes) -> bool {
    let a630 = {
        let a626 = v0;
        let a627 = v1;
        compare_bytes(a626, a627)
    };
    let a631 = {
        let a628 = Bytes::lit(&[]);
        let a629 = Bytes::lit(&[0x01]);
        compare_bytes(a628, a629)
    };
    equal(a630, a631)
}
