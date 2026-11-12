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
    C0(Adt1),
    C1(u64),
    C2(Adt2, Adt1),
    C3(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, List<u64>),
    C4(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C5(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C6(u64, u64, Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>),
    C8,
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(u64, u64, Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64),
    C1(Bytes),
    C2(std::rc::Rc<Adt1>),
    C3(std::rc::Rc<Adt1>),
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C5,
    C6(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3),
    C1(Bytes, Bytes, Bytes, Bytes, Bytes),
    C2(Bytes, Bytes, Bytes),
    C3(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt4>),
    C4(std::rc::Rc<Adt4>),
    C5,
    C6(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
    C7(Bytes, std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(List<(u64, u64)>, u64, bool),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F30(Bytes),
    F54(Bytes),
    F59(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F30(k0) => f30(k0.clone(), p0),
            Fn0::F54(k0) => f54(k0.clone(), p0),
            Fn0::F59(k0, k1) => Ok(f59(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F33,
    F82,
}

impl Fn1 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn1::F33 => f33(p0, p1),
            Fn1::F82 => f82(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F36,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn2::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F39(u64),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn3::F39(k0) => f39(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F43,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt4, p1: u64) -> Adt4 {
        match self {
            Fn4::F43 => f43(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F50(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn5::F50(k0) => f50(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F56,
}

impl Fn6 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn6::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F63,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn7::F63 => f63(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F65,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn8::F65 => f65(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F71,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F71 => f71(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F76,
}

impl Fn10 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn10::F76 => f76(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F81,
}

impl Fn11 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn11::F81 => f81(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1({
        let (h1, _) = f2(v0, f3())?;
        h1
    })
}

pub fn f1(v0: Adt4) -> R<Bytes> {
    f4(v0)
}

pub fn f2(v0: Adt0, v1: Adt5) -> R<(Adt4, Adt5)> {
    let m2 = v0;
    match m2 {
        Adt0::C0(v2) => {
            Ok((Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("sort")), f5(Str::lit("u"), f7(v2), Adt4::C5)))), v1))
        }
        Adt0::C1(v3) => {
            let v4: (Adt6, Adt5) = f8(v1, v3)?;
            Ok((f9({
                let (h3, _) = v4;
                h3
            }), {
                let (_, h4) = v4;
                h4
            }))
        }
        Adt0::C2(v5, v6) => {
            Ok((f10(v5, v6)?, v1))
        }
        Adt0::C3(r37, r38, v9) => {
            let v7 = (*r37).clone();
            let v8 = (*r38).clone();
            let v10: (Adt4, Adt5) = f2(v7, v1)?;
            let v11: (Adt4, Adt5) = f2(v8, {
                let (_, h5) = v10;
                h5
            })?;
            Ok((f11({
                let (h6, _) = v10;
                h6
            }, {
                let (h7, _) = v11;
                h7
            }, v9), {
                let (_, h8) = v11;
                h8
            }))
        }
        Adt0::C4(r39, r40) => {
            let v12 = (*r39).clone();
            let v13 = (*r40).clone();
            let v14: (Adt4, Adt5) = f2(v12, v1)?;
            let v15: (Adt4, Adt5) = f2(v13, {
                let (_, h9) = v14;
                h9
            })?;
            Ok((Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("pi")), f5(Str::lit("b"), Adt4::C3(std::rc::Rc::new({
                let (h10, _) = v14;
                h10
            })), f5(Str::lit("v"), {
                let (h11, _) = v15;
                h11
            }, Adt4::C5))))), {
                let (_, h12) = v15;
                h12
            }))
        }
        Adt0::C5(r41, r42) => {
            let v16 = (*r41).clone();
            let v17 = (*r42).clone();
            let v18: (Adt4, Adt5) = f2(v16, v1)?;
            let v19: (Adt4, Adt5) = f2(v17, {
                let (_, h13) = v18;
                h13
            })?;
            Ok((Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("lam")), f5(Str::lit("b"), Adt4::C3(std::rc::Rc::new({
                let (h14, _) = v18;
                h14
            })), f5(Str::lit("v"), {
                let (h15, _) = v19;
                h15
            }, Adt4::C5))))), {
                let (_, h16) = v19;
                h16
            }))
        }
        Adt0::C6(v20, v21, v22, r43, r44, r45) => {
            let v23 = (*r43).clone();
            let v24 = (*r44).clone();
            let v25 = (*r45).clone();
            let v26: (Adt4, Adt5) = f2(v23, v1)?;
            let v27: (Adt4, Adt5) = f2(v24, {
                let (_, h17) = v26;
                h17
            })?;
            let v28: (u64, Adt5) = f12({
                let (_, h18) = v27;
                h18
            }, v20)?;
            let v29: (Adt4, Adt5) = f2(v25, {
                let (_, h19) = v28;
                h19
            })?;
            Ok((Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("let")), f5(Str::lit("b"), f13({
                let (h20, _) = v28;
                h20
            }, v21, v22, {
                let (h21, _) = v26;
                h21
            }), f5(Str::lit("x"), {
                let (h22, _) = v27;
                h22
            }, f5(Str::lit("v"), {
                let (h23, _) = v29;
                h23
            }, Adt4::C5)))))), {
                let (_, h24) = v29;
                h24
            }))
        }
        Adt0::C7(v30, r46) => {
            let v31 = (*r46).clone();
            let v32: (Adt4, Adt5) = f2(v31, v1)?;
            Ok((Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("nat")), f5(Str::lit("d"), Adt4::C2(v30), f5(Str::lit("t"), {
                let (h25, _) = v32;
                h25
            }, Adt4::C5))))), {
                let (_, h26) = v32;
                h26
            }))
        }
        Adt0::C8 => {
            Ok((Adt4::C5, v1))
        }
        Adt0::C9(r47, r48) => {
            let v33 = (*r47).clone();
            let v34 = (*r48).clone();
            let v35: (Adt4, Adt5) = f2(v33, v1)?;
            let v36: (Adt4, Adt5) = f2(v34, {
                let (_, h27) = v35;
                h27
            })?;
            Ok((Adt4::C6(std::rc::Rc::new({
                let (h28, _) = v35;
                h28
            }), std::rc::Rc::new({
                let (h29, _) = v36;
                h29
            })), {
                let (_, h30) = v36;
                h30
            }))
        }
        Adt0::C10(v37, v38, v39, r49, r50) => {
            let v40 = (*r49).clone();
            let v41 = (*r50).clone();
            let v42: (Adt4, Adt5) = f2(v40, v1)?;
            let v43: (u64, Adt5) = f12({
                let (_, h31) = v42;
                h31
            }, v37)?;
            let v44: (Adt4, Adt5) = f2(v41, {
                let (_, h32) = v43;
                h32
            })?;
            Ok((Adt4::C6(std::rc::Rc::new(f13({
                let (h33, _) = v43;
                h33
            }, v38, v39, {
                let (h34, _) = v42;
                h34
            })), std::rc::Rc::new({
                let (h35, _) = v44;
                h35
            })), {
                let (_, h36) = v44;
                h36
            }))
        }
    }
}

pub fn f3() -> Adt5 {
    Adt5::C0(List::<(u64, u64)>::nil(), 0u64, false)
}

pub fn f4(v0: Adt4) -> R<Bytes> {
    let m51 = v0;
    match m51 {
        Adt4::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt4::C1(v2, v3) => {
            f14(v2, v3)
        }
        Adt4::C2(v4) => {
            f15(v4)
        }
        Adt4::C3(r52) => {
            let v5 = (*r52).clone();
            Ok(f16(f17(v5)?))
        }
        Adt4::C4(r53) => {
            let v6 = (*r53).clone();
            f18(f19(v6)?)
        }
        Adt4::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt4::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt4::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f5(v0: Str, v1: Adt4, v2: Adt4) -> Adt4 {
    Adt4::C7(f20(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f6(v0: Str) -> Adt4 {
    Adt4::C2(f20(v0))
}

pub fn f7(v0: Adt1) -> Adt4 {
    let m54 = v0;
    match m54 {
        Adt1::C0(v1) => {
            Adt4::C4(std::rc::Rc::new(f5(Str::lit("num"), f21(v1), Adt4::C5)))
        }
        Adt1::C1(v2) => {
            Adt4::C4(std::rc::Rc::new(f5(Str::lit("var"), Adt4::C2(v2), Adt4::C5)))
        }
        Adt1::C2(r55) => {
            let v3 = (*r55).clone();
            Adt4::C4(std::rc::Rc::new(f5(Str::lit("succ"), f7(v3), Adt4::C5)))
        }
        Adt1::C3(r56) => {
            let v4 = (*r56).clone();
            Adt4::C4(std::rc::Rc::new(f5(Str::lit("max"), Adt4::C3(std::rc::Rc::new(f7(v4))), Adt4::C5)))
        }
        Adt1::C4(r57, r58) => {
            let v5 = (*r57).clone();
            let v6 = (*r58).clone();
            Adt4::C4(std::rc::Rc::new(f5(Str::lit("imax"), Adt4::C3(std::rc::Rc::new(Adt4::C6(std::rc::Rc::new(f7(v5)), std::rc::Rc::new(Adt4::C6(std::rc::Rc::new(f7(v6)), std::rc::Rc::new(Adt4::C5)))))), Adt4::C5)))
        }
        Adt1::C5 => {
            Adt4::C5
        }
        Adt1::C6(r59, r60) => {
            let v7 = (*r59).clone();
            let v8 = (*r60).clone();
            Adt4::C6(std::rc::Rc::new(f7(v7)), std::rc::Rc::new(f7(v8)))
        }
    }
}

pub fn f8(v0: Adt5, v1: u64) -> R<(Adt6, Adt5)> {
    let m62 = f22(match v0.clone() {
        Adt5::C0(h61, _, _) => {
            h61
        }
    }, v1);
    match m62 {
        None => {
            if match v0.clone() {
                Adt5::C0(_, _, h63) => {
                    h63
                }
            } {
                Ok((Adt6::C1(v1), v0))
            } else {
                let v3: (u64, Adt5) = f23(v0, v1)?;
                Ok((Adt6::C0({
                    let (h64, _) = v3;
                    h64
                }), {
                    let (_, h65) = v3;
                    h65
                }))
            }
        }
        Some(v2) => {
            Ok((Adt6::C0(v2), v0))
        }
    }
}

pub fn f9(v0: Adt6) -> Adt4 {
    let m66 = v0;
    match m66 {
        Adt6::C0(v1) => {
            Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("local")), f5(Str::lit("id"), f21(v1), Adt4::C5))))
        }
        Adt6::C1(v2) => {
            Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("local")), f5(Str::lit("free"), f21(v2), Adt4::C5))))
        }
    }
}

pub fn f10(v0: Adt2, v1: Adt1) -> R<Adt4> {
    let v2: Adt4 = if f24(v1.clone()) {
        Adt4::C5
    } else {
        f5(Str::lit("u"), Adt4::C3(std::rc::Rc::new(f7(v1))), Adt4::C5)
    };
    Ok(Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("global")), f5(Str::lit("g"), f25(v0)?, v2)))))
}

pub fn f11(v0: Adt4, v1: Adt4, v2: List<u64>) -> Adt4 {
    let v5: Adt4 = {
        let m67 = v2.clone();
        match m67.uncons() {
            None => {
                Adt4::C5
            }
            Some((_, _)) => {
                f5(Str::lit("i"), Adt4::C3(std::rc::Rc::new(f26(v2))), Adt4::C5)
            }
        }
    };
    Adt4::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("app")), f5(Str::lit("f"), v0, f5(Str::lit("a"), Adt4::C3(std::rc::Rc::new(v1)), v5)))))
}

pub fn f12(v0: Adt5, v1: u64) -> R<(u64, Adt5)> {
    let m69 = f22(match v0.clone() {
        Adt5::C0(h68, _, _) => {
            h68
        }
    }, v1);
    match m69 {
        None => {
            f23(v0, v1)
        }
        Some(v2) => {
            Ok((v2, v0))
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: Bytes, v3: Adt4) -> Adt4 {
    Adt4::C4(std::rc::Rc::new(f5(Str::lit("id"), f21(v0), f5(Str::lit("m"), f27(v1), f5(Str::lit("s"), Adt4::C2(v2), f5(Str::lit("t"), v3, Adt4::C5))))))
}

pub fn f14(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a70 = 0u64;
        let a71 = v1;
        nat_lt(a70, a71)
    } else {
        false
    } {
        let a72 = Bytes::lit(&[0x2d]);
        let a73 = f28(v1)?;
        Ok(append_bytes(a72, a73))
    } else {
        f28(v1)
    }
}

pub fn f15(v0: Bytes) -> R<Bytes> {
    let a79 = {
        let a77 = Bytes::lit(&[0x22]);
        let a78 = {
            let (_, h74) = {
                let (h75, _) = f29(Fn0::F30(v0.clone()), {
                    let a76 = v0;
                    length_bytes(a76)
                }, (0u64, Bytes::lit(&[])))?;
                h75
            };
            h74
        };
        append_bytes(a77, a78)
    };
    let a80 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a79, a80))
}

pub fn f16(v0: List<Bytes>) -> Bytes {
    let a83 = {
        let a81 = Bytes::lit(&[0x5b]);
        let a82 = f31(v0, Bytes::lit(&[0x2c]));
        append_bytes(a81, a82)
    };
    let a84 = Bytes::lit(&[0x5d]);
    append_bytes(a83, a84)
}

pub fn f17(v0: Adt4) -> R<List<Bytes>> {
    let m85 = v0;
    match m85 {
        Adt4::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C6(r86, r87) => {
            let v7 = (*r86).clone();
            let v8 = (*r87).clone();
            Ok(List::cons(f4(v7)?, f17(v8)?))
        }
        Adt4::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f18(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f32(Fn1::F33, List::<(Bytes, Bytes)>::nil(), f34(v0)?);
    let v2: List<Bytes> = f35(Fn2::F36, List::<Bytes>::nil(), v1)?;
    let a90 = {
        let a88 = Bytes::lit(&[0x7b]);
        let a89 = f31(v2, Bytes::lit(&[0x2c]));
        append_bytes(a88, a89)
    };
    let a91 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a90, a91))
}

pub fn f19(v0: Adt4) -> R<List<(Bytes, Bytes)>> {
    let m92 = v0;
    match m92 {
        Adt4::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C7(v9, r93, r94) => {
            let v10 = (*r93).clone();
            let v11 = (*r94).clone();
            Ok(List::cons((v9, f4(v10)?), f19(v11)?))
        }
    }
}

pub fn f20(v0: Str) -> Bytes {
    let a95 = v0;
    utf8_encode(a95)
}

pub fn f21(v0: u64) -> Adt4 {
    Adt4::C1(false, f37(v0))
}

pub fn f22(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f38(Fn3::F39(v1), None::<u64>, v0)
}

pub fn f23(v0: Adt5, v1: u64) -> R<(u64, Adt5)> {
    Ok((match v0.clone() {
        Adt5::C0(_, h96, _) => {
            h96
        }
    }, Adt5::C0(List::cons((v1, match v0.clone() {
        Adt5::C0(_, h97, _) => {
            h97
        }
    }), match v0.clone() {
        Adt5::C0(h98, _, _) => {
            h98
        }
    }), {
        let a100 = match v0.clone() {
            Adt5::C0(_, h99, _) => {
                h99
            }
        };
        let a101 = 1u64;
        nat_add(a100, a101)?
    }, match v0 {
        Adt5::C0(_, _, h102) => {
            h102
        }
    })))
}

pub fn f24(v0: Adt1) -> bool {
    let m103 = v0;
    match m103 {
        Adt1::C0(_) => {
            false
        }
        Adt1::C1(_) => {
            false
        }
        Adt1::C2(_) => {
            false
        }
        Adt1::C3(_) => {
            false
        }
        Adt1::C4(_, _) => {
            false
        }
        Adt1::C5 => {
            true
        }
        Adt1::C6(_, _) => {
            false
        }
    }
}

pub fn f25(v0: Adt2) -> R<Adt4> {
    let m104 = v0;
    match m104 {
        Adt2::C0(v1) => {
            Ok(Adt4::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(Str::lit("core")), f5(Str::lit("name"), Adt4::C2(f40(v1)), Adt4::C5)))))
        }
        Adt2::C1(_, v3, v4, v5, v6) => {
            Ok(Adt4::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(Str::lit("external")), f5(Str::lit("entry"), Adt4::C2(v3), f5(Str::lit("lean_module"), Adt4::C2(v4), f5(Str::lit("lean_name"), Adt4::C2(v5), f5(Str::lit("signature_sha256"), Adt4::C2(f41(v6)?), Adt4::C5))))))))
        }
        Adt2::C2(v7, v8, v9) => {
            Ok(Adt4::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(Str::lit("document")), f5(Str::lit("module"), Adt4::C2(v7), f5(Str::lit("component"), Adt4::C2(v8), f5(Str::lit("lean_name"), Adt4::C2(v9), Adt4::C5)))))))
        }
        Adt2::C3(_, v11, v12) => {
            Ok(Adt4::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(Str::lit("defined")), f5(Str::lit("entry"), Adt4::C2(v11), f5(Str::lit("signature_sha256"), Adt4::C2(f41(v12)?), Adt4::C5))))))
        }
    }
}

pub fn f26(v0: List<u64>) -> Adt4 {
    f42(Fn4::F43, Adt4::C5, f44(v0))
}

pub fn f27(v0: u64) -> Adt4 {
    Adt4::C2(f45(v0))
}

pub fn f28(v0: u64) -> R<Bytes> {
    f46(f47(), 10u64, v0)
}

pub fn f29(v0: Fn0, p109: u64, p110: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m108 = (p109, p110);
    {
        let mut m108 = m108;
        loop {
            let step = {
                let (v1, v2) = m108;
                let m105 = v1;
                if m105 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m105 - 1;
                    let m107 = {
                        let c106: Fn0 = v0.clone();
                        c106.apply(v2.clone())?
                    };
                    match m107 {
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
                Ok(next) => { m108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f48(v0, v1)
}

pub fn f31(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m111 = f49(Fn5::F50(v1), None::<Bytes>, v0);
    match m111 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f32(v0: Fn1, p115: List<(Bytes, Bytes)>, p116: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m114 = (p115, p116);
    {
        let mut m114 = m114;
        loop {
            let step = {
                let (v1, v2) = m114;
                let m112 = v2;
                match m112.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c113: Fn1 = v0.clone();
                            c113.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f51(v0, v1)
}

pub fn f34(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f52(v0)
}

pub fn f35(v0: Fn2, p120: List<Bytes>, p121: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v1, v2) = m119;
                let m117 = v2;
                match m117.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c118: Fn2 = v0.clone();
                            c118.apply(v1, v3)?
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

pub fn f36(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f53(v1)?, v0))
}

pub fn f37(v0: u64) -> u64 {
    let a122 = v0;
    let a123 = 9223372036854775807u64;
    if nat_le(a122, a123) {
        v0
    } else {
        9223372036854775807u64
    }
}

pub fn f38(v0: Fn3, p127: Option<u64>, p128: List<(u64, u64)>) -> Option<u64> {
    let m126 = (p127, p128);
    {
        let mut m126 = m126;
        loop {
            let step = {
                let (v1, v2) = m126;
                let m124 = v2;
                match m124.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c125: Fn3 = v0.clone();
                            c125.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let a130 = {
        let (h129, _) = v2;
        h129
    };
    let a131 = v0;
    if nat_eq(a130, a131) {
        Some({
            let (_, h132) = v2;
            h132
        })
    } else {
        v1
    }
}

pub fn f40(v0: Adt3) -> Bytes {
    let m133 = v0;
    match m133 {
        Adt3::C0 => {
            f20(Str::lit("eq"))
        }
        Adt3::C1 => {
            f20(Str::lit("and"))
        }
        Adt3::C2 => {
            f20(Str::lit("or"))
        }
        Adt3::C3 => {
            f20(Str::lit("not"))
        }
        Adt3::C4 => {
            f20(Str::lit("iff"))
        }
        Adt3::C5 => {
            f20(Str::lit("exists"))
        }
        Adt3::C6 => {
            f20(Str::lit("exists-unique"))
        }
    }
}

pub fn f41(v0: Bytes) -> R<Bytes> {
    let (_, h134) = {
        let (h135, _) = f29(Fn0::F54(v0.clone()), {
            let a136 = v0;
            length_bytes(a136)
        }, (0u64, Bytes::lit(&[])))?;
        h135
    };
    Ok(h134)
}

pub fn f42(v0: Fn4, p140: Adt4, p141: List<u64>) -> Adt4 {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m137 = v2;
                match m137.uncons() {
                    None => {
                        Err::<(Adt4, List<u64>), Adt4>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<u64>), Adt4>(({
                            let c138: Fn4 = v0.clone();
                            c138.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Adt4, v1: u64) -> Adt4 {
    Adt4::C6(std::rc::Rc::new(f21(v1)), std::rc::Rc::new(v0))
}

pub fn f44(v0: List<u64>) -> List<u64> {
    f55(Fn6::F56, List::<u64>::nil(), v0)
}

pub fn f45(v0: u64) -> Bytes {
    let a142 = v0;
    let a143 = 1u64;
    if nat_eq(a142, a143) {
        f57(Str::lit("implicit"))
    } else {
        let a144 = v0;
        let a145 = 2u64;
        if nat_eq(a144, a145) {
            f57(Str::lit("instance"))
        } else {
            f57(Str::lit("explicit"))
        }
    }
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a146 = v2;
    let a147 = 0u64;
    if nat_eq(a146, a147) {
        Ok(f58(v0, 0u64))
    } else {
        let (_, h148) = {
            let (h149, _) = f29(Fn0::F59(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h149
        };
        Ok(h148)
    }
}

pub fn f47() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f48(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m153 = {
        let a151 = v0.clone();
        let a152 = {
            let (h150, _) = v1.clone();
            h150
        };
        index_bytes(a151, a152)
    };
    match m153 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a155 = {
                    let (h154, _) = v1.clone();
                    h154
                };
                let a156 = 1u64;
                nat_add(a155, a156)?
            }, {
                let a159 = {
                    let (_, h157) = v1;
                    h157
                };
                let a160 = f60(v0, {
                    let (h158, _) = v1;
                    h158
                }, v2)?;
                append_bytes(a159, a160)
            })))
        }
    }
}

pub fn f49(v0: Fn5, p164: Option<Bytes>, p165: List<Bytes>) -> Option<Bytes> {
    let m163 = (p164, p165);
    {
        let mut m163 = m163;
        loop {
            let step = {
                let (v1, v2) = m163;
                let m161 = v2;
                match m161.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c162: Fn5 = v0.clone();
                            c162.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m166 = v1;
    match m166 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a169 = {
                    let a167 = v3;
                    let a168 = v0;
                    append_bytes(a167, a168)
                };
                let a170 = v2;
                append_bytes(a169, a170)
            })
        }
    }
}

pub fn f51(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m171 = v0.clone();
    match m171.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a174 = {
                let (h172, _) = v2;
                h172
            };
            let a175 = {
                let (h173, _) = v1.clone();
                h173
            };
            if equal(a174, a175) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f52(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f61(f62(Fn7::F63, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m180 = {
        let (h176, _) = f64(Fn8::F65, {
            let a178 = {
                let a177 = v0;
                length_list(a177)
            };
            let a179 = 1u64;
            nat_add(a178, a179)?
        }, v1)?;
        h176
    };
    match m180.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f53(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a185 = {
        let a182 = f15({
            let (h181, _) = v0;
            h181
        })?;
        let a183 = Bytes::lit(&[0x3a]);
        append_bytes(a182, a183)
    };
    let a186 = {
        let (_, h184) = v0;
        h184
    };
    Ok(append_bytes(a185, a186))
}

pub fn f54(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f66(v0, v1)
}

pub fn f55(v0: Fn6, p190: List<u64>, p191: List<u64>) -> List<u64> {
    let m189 = (p190, p191);
    {
        let mut m189 = m189;
        loop {
            let step = {
                let (v1, v2) = m189;
                let m187 = v2;
                match m187.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c188: Fn6 = v0.clone();
                            c188.apply(v1, v3)
                        }, v4))
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

pub fn f56(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f57(v0: Str) -> Bytes {
    let a192 = v0;
    utf8_encode(a192)
}

pub fn f58(v0: Bytes, v1: u64) -> Bytes {
    let m196 = {
        let a193 = v0;
        let a194 = v1;
        let a195 = 1u64;
        slice_bytes(a193, a194, a195)
    };
    match m196 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f59(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f67(v1, v0, v2)
}

pub fn f60(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a241 = {
            let a217 = {
                let a205 = {
                    let a199 = {
                        let a197 = v3;
                        let a198 = 128u8;
                        fixed_u8::bit_and(a197, a198)
                    };
                    let a200 = 128u8;
                    if equal(a199, a200) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a206 = {
                    let a203 = {
                        let a201 = v3;
                        let a202 = 64u8;
                        fixed_u8::bit_and(a201, a202)
                    };
                    let a204 = 64u8;
                    if equal(a203, a204) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a205, a206)?
            };
            let a218 = {
                let a215 = {
                    let a209 = {
                        let a207 = v3;
                        let a208 = 32u8;
                        fixed_u8::bit_and(a207, a208)
                    };
                    let a210 = 32u8;
                    if equal(a209, a210) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a216 = {
                    let a213 = {
                        let a211 = v3;
                        let a212 = 16u8;
                        fixed_u8::bit_and(a211, a212)
                    };
                    let a214 = 16u8;
                    if equal(a213, a214) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a215, a216)?
            };
            nat_add(a217, a218)?
        };
        let a242 = {
            let a239 = {
                let a227 = {
                    let a221 = {
                        let a219 = v3;
                        let a220 = 8u8;
                        fixed_u8::bit_and(a219, a220)
                    };
                    let a222 = 8u8;
                    if equal(a221, a222) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a228 = {
                    let a225 = {
                        let a223 = v3;
                        let a224 = 4u8;
                        fixed_u8::bit_and(a223, a224)
                    };
                    let a226 = 4u8;
                    if equal(a225, a226) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a227, a228)?
            };
            let a240 = {
                let a237 = {
                    let a231 = {
                        let a229 = v3;
                        let a230 = 2u8;
                        fixed_u8::bit_and(a229, a230)
                    };
                    let a232 = 2u8;
                    if equal(a231, a232) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a238 = {
                    let a235 = {
                        let a233 = v3;
                        let a234 = 1u8;
                        fixed_u8::bit_and(a233, a234)
                    };
                    let a236 = 1u8;
                    if equal(a235, a236) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a237, a238)?
            };
            nat_add(a239, a240)?
        };
        nat_add(a241, a242)?
    };
    let a243 = v4;
    let a244 = 34u64;
    if nat_eq(a243, a244) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a245 = v4;
        let a246 = 92u64;
        if nat_eq(a245, a246) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a247 = v4;
            let a248 = 8u64;
            if nat_eq(a247, a248) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a249 = v4;
                let a250 = 12u64;
                if nat_eq(a249, a250) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a251 = v4;
                    let a252 = 10u64;
                    if nat_eq(a251, a252) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a253 = v4;
                        let a254 = 13u64;
                        if nat_eq(a253, a254) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a255 = v4;
                            let a256 = 9u64;
                            if nat_eq(a255, a256) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a257 = v4;
                                let a258 = 32u64;
                                if nat_lt(a257, a258) {
                                    let a259 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a260 = f68(v2)?;
                                    Ok(append_bytes(a259, a260))
                                } else {
                                    Ok(f69(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f61(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f70(Fn9::F71, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f62(v0: Fn7, p264: List<List<(Bytes, Bytes)>>, p265: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m263 = (p264, p265);
    {
        let mut m263 = m263;
        loop {
            let step = {
                let (v1, v2) = m263;
                let m261 = v2;
                match m261.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c262: Fn7 = v0.clone();
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

pub fn f63(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f64(v0: Fn8, p270: u64, p271: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m269 = (p270, p271);
    {
        let mut m269 = m269;
        loop {
            let step = {
                let (v1, v2) = m269;
                let m266 = v1;
                if m266 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m266 - 1;
                    let m268 = {
                        let c267: Fn8 = v0.clone();
                        c267.apply(v2.clone())?
                    };
                    match m268 {
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
                Ok(next) => { m269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f72(v0)
}

pub fn f66(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m275 = {
        let a273 = v0;
        let a274 = {
            let (h272, _) = v1.clone();
            h272
        };
        index_bytes(a273, a274)
    };
    match m275 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a277 = {
                    let (h276, _) = v1;
                    h276
                };
                let a278 = 1u64;
                nat_add(a277, a278)?
            }, {
                let a280 = {
                    let (_, h279) = v1;
                    h279
                };
                let a281 = f68(v2)?;
                append_bytes(a280, a281)
            })))
        }
    }
}

pub fn f67(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h282, _) = v2;
        h282
    };
    let a283 = v3;
    let a284 = 0u64;
    if nat_eq(a283, a284) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a285 = v3;
            let a286 = v1;
            let a287 = 0u64;
            nat_quot(a285, a286, a287)
        }, {
            let a292 = f58(v0, {
                let a288 = v3;
                let a289 = v1;
                let a290 = 0u64;
                nat_rem(a288, a289, a290)
            });
            let a293 = {
                let (_, h291) = v2;
                h291
            };
            append_bytes(a292, a293)
        }))
    }
}

pub fn f68(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a338 = {
            let a314 = {
                let a302 = {
                    let a296 = {
                        let a294 = v1;
                        let a295 = 128u8;
                        fixed_u8::bit_and(a294, a295)
                    };
                    let a297 = 128u8;
                    if equal(a296, a297) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a303 = {
                    let a300 = {
                        let a298 = v1;
                        let a299 = 64u8;
                        fixed_u8::bit_and(a298, a299)
                    };
                    let a301 = 64u8;
                    if equal(a300, a301) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a302, a303)?
            };
            let a315 = {
                let a312 = {
                    let a306 = {
                        let a304 = v1;
                        let a305 = 32u8;
                        fixed_u8::bit_and(a304, a305)
                    };
                    let a307 = 32u8;
                    if equal(a306, a307) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a313 = {
                    let a310 = {
                        let a308 = v1;
                        let a309 = 16u8;
                        fixed_u8::bit_and(a308, a309)
                    };
                    let a311 = 16u8;
                    if equal(a310, a311) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a312, a313)?
            };
            nat_add(a314, a315)?
        };
        let a339 = {
            let a336 = {
                let a324 = {
                    let a318 = {
                        let a316 = v1;
                        let a317 = 8u8;
                        fixed_u8::bit_and(a316, a317)
                    };
                    let a319 = 8u8;
                    if equal(a318, a319) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a325 = {
                    let a322 = {
                        let a320 = v1;
                        let a321 = 4u8;
                        fixed_u8::bit_and(a320, a321)
                    };
                    let a323 = 4u8;
                    if equal(a322, a323) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a324, a325)?
            };
            let a337 = {
                let a334 = {
                    let a328 = {
                        let a326 = v1;
                        let a327 = 2u8;
                        fixed_u8::bit_and(a326, a327)
                    };
                    let a329 = 2u8;
                    if equal(a328, a329) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a335 = {
                    let a332 = {
                        let a330 = v1;
                        let a331 = 1u8;
                        fixed_u8::bit_and(a330, a331)
                    };
                    let a333 = 1u8;
                    if equal(a332, a333) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a334, a335)?
            };
            nat_add(a336, a337)?
        };
        nat_add(a338, a339)?
    };
    let a346 = f58(f73(), {
        let a340 = v2;
        let a341 = 16u64;
        let a342 = 0u64;
        nat_quot(a340, a341, a342)
    });
    let a347 = f58(f73(), {
        let a343 = v2;
        let a344 = 16u64;
        let a345 = 0u64;
        nat_rem(a343, a344, a345)
    });
    Ok(append_bytes(a346, a347))
}

pub fn f69(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m351 = {
        let a348 = v0;
        let a349 = v1;
        let a350 = v2;
        slice_bytes(a348, a349, a350)
    };
    match m351 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f70(v0: Fn9, p355: List<List<(Bytes, Bytes)>>, p356: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v1, v2) = m354;
                let m352 = v2;
                match m352.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c353: Fn9 = v0.clone();
                            c353.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f72(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a358 = {
        let a357 = v0.clone();
        length_list(a357)
    };
    let a359 = 1u64;
    if nat_le(a358, a359) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f74(v0)?))
    }
}

pub fn f73() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f74(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f61({
        let (_, h360) = {
            let (h361, _) = f75(Fn10::F76, {
                let a363 = {
                    let a362 = v0.clone();
                    length_list(a362)
                };
                let a364 = 1u64;
                nat_add(a363, a364)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h361
        };
        h360
    }))
}

pub fn f75(v0: Fn10, p369: u64, p370: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m368 = (p369, p370);
    {
        let mut m368 = m368;
        loop {
            let step = {
                let (v1, v2) = m368;
                let m365 = v1;
                if m365 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m365 - 1;
                    let m367 = {
                        let c366: Fn10 = v0.clone();
                        c366.apply(v2.clone())?
                    };
                    match m367 {
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
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f77(v0)
}

pub fn f77(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h371, _) = v0;
        h371
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h372) = v0;
        h372
    };
    let m373 = v1;
    match m373.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m374 = v4;
            match m374.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f78(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f78(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a379 = {
            let a377 = {
                let a375 = v0.clone();
                length_list(a375)
            };
            let a378 = {
                let a376 = v1.clone();
                length_list(a376)
            };
            nat_add(a377, a378)?
        };
        let a380 = 1u64;
        nat_add(a379, a380)?
    };
    Ok(f79({
        let (_, h381) = {
            let (_, h382) = {
                let (h383, _) = f80(Fn11::F81, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h383
            };
            h382
        };
        h381
    }))
}

pub fn f79(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f32(Fn1::F82, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f80(v0: Fn11, p388: u64, p389: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m387 = (p388, p389);
    {
        let mut m387 = m387;
        loop {
            let step = {
                let (v1, v2) = m387;
                let m384 = v1;
                if m384 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m384 - 1;
                    let m386 = {
                        let c385: Fn11 = v0.clone();
                        c385.apply(v2.clone())
                    };
                    match m386 {
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
                Ok(next) => { m387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f83(v0)
}

pub fn f82(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f83(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h390, _) = v0.clone();
        h390
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h391, _) = {
            let (_, h392) = v0.clone();
            h392
        };
        h391
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h393) = {
            let (_, h394) = v0;
            h394
        };
        h393
    };
    let m395 = v1.clone();
    match m395.uncons() {
        None => {
            let m396 = v2;
            match m396.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m397 = v2.clone();
            match m397.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f84({
                        let (h398, _) = v8.clone();
                        h398
                    }, {
                        let (h399, _) = v6.clone();
                        h399
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

pub fn f84(v0: Bytes, v1: Bytes) -> bool {
    let a404 = {
        let a400 = v0;
        let a401 = v1;
        compare_bytes(a400, a401)
    };
    let a405 = {
        let a402 = Bytes::lit(&[]);
        let a403 = Bytes::lit(&[0x01]);
        compare_bytes(a402, a403)
    };
    equal(a404, a405)
}
