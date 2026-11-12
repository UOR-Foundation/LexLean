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
    F31(Bytes),
    F55(Bytes),
    F60(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F31(k0) => f31(k0.clone(), p0),
            Fn0::F55(k0) => f55(k0.clone(), p0),
            Fn0::F60(k0, k1) => Ok(f60(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F34,
    F83,
}

impl Fn1 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn1::F34 => f34(p0, p1),
            Fn1::F83 => f83(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F37,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn2::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F40(u64),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn3::F40(k0) => f40(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F44,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt4, p1: u64) -> Adt4 {
        match self {
            Fn4::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F51(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn5::F51(k0) => f51(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F57,
}

impl Fn6 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn6::F57 => f57(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F64,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn7::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F66,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn8::F66 => f66(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F72,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F72 => f72(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F77,
}

impl Fn10 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn10::F77 => f77(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F82,
}

impl Fn11 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn11::F82 => f82(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2({
        let (h1, _) = f3(v0, f4())?;
        h1
    }))
}

pub fn f1(v0: Adt4) -> R<Bytes> {
    f5(v0)
}

pub fn f2(v0: Adt4) -> Adt4 {
    let m2 = v0;
    match m2 {
        Adt4::C0(v1) => {
            Adt4::C0(v1)
        }
        Adt4::C1(v2, v3) => {
            Adt4::C1(v2, v3)
        }
        Adt4::C2(v4) => {
            Adt4::C2(v4)
        }
        Adt4::C3(r5) => {
            let v5 = (*r5).clone();
            Adt4::C3(std::rc::Rc::new(f2(v5)))
        }
        Adt4::C4(r6) => {
            let v6 = (*r6).clone();
            Adt4::C4(std::rc::Rc::new(f2(v6)))
        }
        Adt4::C5 => {
            Adt4::C5
        }
        Adt4::C6(r7, r8) => {
            let v7 = (*r7).clone();
            let v8 = (*r8).clone();
            Adt4::C6(std::rc::Rc::new(f2(v7)), std::rc::Rc::new(f2(v8)))
        }
        Adt4::C7(v9, r9, r10) => {
            let v10 = (*r9).clone();
            let v11 = (*r10).clone();
            let a3 = v9.clone();
            let a4 = f6(Str::lit("s"));
            if equal(a3, a4) {
                f2(v11)
            } else {
                Adt4::C7(v9, std::rc::Rc::new(f2(v10)), std::rc::Rc::new(f2(v11)))
            }
        }
    }
}

pub fn f3(v0: Adt0, v1: Adt5) -> R<(Adt4, Adt5)> {
    let m11 = v0;
    match m11 {
        Adt0::C0(v2) => {
            Ok((Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("sort")), f7(Str::lit("u"), f9(v2), Adt4::C5)))), v1))
        }
        Adt0::C1(v3) => {
            let v4: (Adt6, Adt5) = f10(v1, v3)?;
            Ok((f11({
                let (h12, _) = v4;
                h12
            }), {
                let (_, h13) = v4;
                h13
            }))
        }
        Adt0::C2(v5, v6) => {
            Ok((f12(v5, v6)?, v1))
        }
        Adt0::C3(r46, r47, v9) => {
            let v7 = (*r46).clone();
            let v8 = (*r47).clone();
            let v10: (Adt4, Adt5) = f3(v7, v1)?;
            let v11: (Adt4, Adt5) = f3(v8, {
                let (_, h14) = v10;
                h14
            })?;
            Ok((f13({
                let (h15, _) = v10;
                h15
            }, {
                let (h16, _) = v11;
                h16
            }, v9), {
                let (_, h17) = v11;
                h17
            }))
        }
        Adt0::C4(r48, r49) => {
            let v12 = (*r48).clone();
            let v13 = (*r49).clone();
            let v14: (Adt4, Adt5) = f3(v12, v1)?;
            let v15: (Adt4, Adt5) = f3(v13, {
                let (_, h18) = v14;
                h18
            })?;
            Ok((Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("pi")), f7(Str::lit("b"), Adt4::C3(std::rc::Rc::new({
                let (h19, _) = v14;
                h19
            })), f7(Str::lit("v"), {
                let (h20, _) = v15;
                h20
            }, Adt4::C5))))), {
                let (_, h21) = v15;
                h21
            }))
        }
        Adt0::C5(r50, r51) => {
            let v16 = (*r50).clone();
            let v17 = (*r51).clone();
            let v18: (Adt4, Adt5) = f3(v16, v1)?;
            let v19: (Adt4, Adt5) = f3(v17, {
                let (_, h22) = v18;
                h22
            })?;
            Ok((Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("lam")), f7(Str::lit("b"), Adt4::C3(std::rc::Rc::new({
                let (h23, _) = v18;
                h23
            })), f7(Str::lit("v"), {
                let (h24, _) = v19;
                h24
            }, Adt4::C5))))), {
                let (_, h25) = v19;
                h25
            }))
        }
        Adt0::C6(v20, v21, v22, r52, r53, r54) => {
            let v23 = (*r52).clone();
            let v24 = (*r53).clone();
            let v25 = (*r54).clone();
            let v26: (Adt4, Adt5) = f3(v23, v1)?;
            let v27: (Adt4, Adt5) = f3(v24, {
                let (_, h26) = v26;
                h26
            })?;
            let v28: (u64, Adt5) = f14({
                let (_, h27) = v27;
                h27
            }, v20)?;
            let v29: (Adt4, Adt5) = f3(v25, {
                let (_, h28) = v28;
                h28
            })?;
            Ok((Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("let")), f7(Str::lit("b"), f15({
                let (h29, _) = v28;
                h29
            }, v21, v22, {
                let (h30, _) = v26;
                h30
            }), f7(Str::lit("x"), {
                let (h31, _) = v27;
                h31
            }, f7(Str::lit("v"), {
                let (h32, _) = v29;
                h32
            }, Adt4::C5)))))), {
                let (_, h33) = v29;
                h33
            }))
        }
        Adt0::C7(v30, r55) => {
            let v31 = (*r55).clone();
            let v32: (Adt4, Adt5) = f3(v31, v1)?;
            Ok((Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("nat")), f7(Str::lit("d"), Adt4::C2(v30), f7(Str::lit("t"), {
                let (h34, _) = v32;
                h34
            }, Adt4::C5))))), {
                let (_, h35) = v32;
                h35
            }))
        }
        Adt0::C8 => {
            Ok((Adt4::C5, v1))
        }
        Adt0::C9(r56, r57) => {
            let v33 = (*r56).clone();
            let v34 = (*r57).clone();
            let v35: (Adt4, Adt5) = f3(v33, v1)?;
            let v36: (Adt4, Adt5) = f3(v34, {
                let (_, h36) = v35;
                h36
            })?;
            Ok((Adt4::C6(std::rc::Rc::new({
                let (h37, _) = v35;
                h37
            }), std::rc::Rc::new({
                let (h38, _) = v36;
                h38
            })), {
                let (_, h39) = v36;
                h39
            }))
        }
        Adt0::C10(v37, v38, v39, r58, r59) => {
            let v40 = (*r58).clone();
            let v41 = (*r59).clone();
            let v42: (Adt4, Adt5) = f3(v40, v1)?;
            let v43: (u64, Adt5) = f14({
                let (_, h40) = v42;
                h40
            }, v37)?;
            let v44: (Adt4, Adt5) = f3(v41, {
                let (_, h41) = v43;
                h41
            })?;
            Ok((Adt4::C6(std::rc::Rc::new(f15({
                let (h42, _) = v43;
                h42
            }, v38, v39, {
                let (h43, _) = v42;
                h43
            })), std::rc::Rc::new({
                let (h44, _) = v44;
                h44
            })), {
                let (_, h45) = v44;
                h45
            }))
        }
    }
}

pub fn f4() -> Adt5 {
    Adt5::C0(List::<(u64, u64)>::nil(), 0u64, false)
}

pub fn f5(v0: Adt4) -> R<Bytes> {
    let m60 = v0;
    match m60 {
        Adt4::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt4::C1(v2, v3) => {
            f16(v2, v3)
        }
        Adt4::C2(v4) => {
            f17(v4)
        }
        Adt4::C3(r61) => {
            let v5 = (*r61).clone();
            Ok(f18(f19(v5)?))
        }
        Adt4::C4(r62) => {
            let v6 = (*r62).clone();
            f20(f21(v6)?)
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

pub fn f6(v0: Str) -> Bytes {
    let a63 = v0;
    utf8_encode(a63)
}

pub fn f7(v0: Str, v1: Adt4, v2: Adt4) -> Adt4 {
    Adt4::C7(f6(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f8(v0: Str) -> Adt4 {
    Adt4::C2(f6(v0))
}

pub fn f9(v0: Adt1) -> Adt4 {
    let m64 = v0;
    match m64 {
        Adt1::C0(v1) => {
            Adt4::C4(std::rc::Rc::new(f7(Str::lit("num"), f22(v1), Adt4::C5)))
        }
        Adt1::C1(v2) => {
            Adt4::C4(std::rc::Rc::new(f7(Str::lit("var"), Adt4::C2(v2), Adt4::C5)))
        }
        Adt1::C2(r65) => {
            let v3 = (*r65).clone();
            Adt4::C4(std::rc::Rc::new(f7(Str::lit("succ"), f9(v3), Adt4::C5)))
        }
        Adt1::C3(r66) => {
            let v4 = (*r66).clone();
            Adt4::C4(std::rc::Rc::new(f7(Str::lit("max"), Adt4::C3(std::rc::Rc::new(f9(v4))), Adt4::C5)))
        }
        Adt1::C4(r67, r68) => {
            let v5 = (*r67).clone();
            let v6 = (*r68).clone();
            Adt4::C4(std::rc::Rc::new(f7(Str::lit("imax"), Adt4::C3(std::rc::Rc::new(Adt4::C6(std::rc::Rc::new(f9(v5)), std::rc::Rc::new(Adt4::C6(std::rc::Rc::new(f9(v6)), std::rc::Rc::new(Adt4::C5)))))), Adt4::C5)))
        }
        Adt1::C5 => {
            Adt4::C5
        }
        Adt1::C6(r69, r70) => {
            let v7 = (*r69).clone();
            let v8 = (*r70).clone();
            Adt4::C6(std::rc::Rc::new(f9(v7)), std::rc::Rc::new(f9(v8)))
        }
    }
}

pub fn f10(v0: Adt5, v1: u64) -> R<(Adt6, Adt5)> {
    let m72 = f23(match v0.clone() {
        Adt5::C0(h71, _, _) => {
            h71
        }
    }, v1);
    match m72 {
        None => {
            if match v0.clone() {
                Adt5::C0(_, _, h73) => {
                    h73
                }
            } {
                Ok((Adt6::C1(v1), v0))
            } else {
                let v3: (u64, Adt5) = f24(v0, v1)?;
                Ok((Adt6::C0({
                    let (h74, _) = v3;
                    h74
                }), {
                    let (_, h75) = v3;
                    h75
                }))
            }
        }
        Some(v2) => {
            Ok((Adt6::C0(v2), v0))
        }
    }
}

pub fn f11(v0: Adt6) -> Adt4 {
    let m76 = v0;
    match m76 {
        Adt6::C0(v1) => {
            Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("local")), f7(Str::lit("id"), f22(v1), Adt4::C5))))
        }
        Adt6::C1(v2) => {
            Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("local")), f7(Str::lit("free"), f22(v2), Adt4::C5))))
        }
    }
}

pub fn f12(v0: Adt2, v1: Adt1) -> R<Adt4> {
    let v2: Adt4 = if f25(v1.clone()) {
        Adt4::C5
    } else {
        f7(Str::lit("u"), Adt4::C3(std::rc::Rc::new(f9(v1))), Adt4::C5)
    };
    Ok(Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("global")), f7(Str::lit("g"), f26(v0)?, v2)))))
}

pub fn f13(v0: Adt4, v1: Adt4, v2: List<u64>) -> Adt4 {
    let v5: Adt4 = {
        let m77 = v2.clone();
        match m77.uncons() {
            None => {
                Adt4::C5
            }
            Some((_, _)) => {
                f7(Str::lit("i"), Adt4::C3(std::rc::Rc::new(f27(v2))), Adt4::C5)
            }
        }
    };
    Adt4::C4(std::rc::Rc::new(f7(Str::lit("k"), f8(Str::lit("app")), f7(Str::lit("f"), v0, f7(Str::lit("a"), Adt4::C3(std::rc::Rc::new(v1)), v5)))))
}

pub fn f14(v0: Adt5, v1: u64) -> R<(u64, Adt5)> {
    let m79 = f23(match v0.clone() {
        Adt5::C0(h78, _, _) => {
            h78
        }
    }, v1);
    match m79 {
        None => {
            f24(v0, v1)
        }
        Some(v2) => {
            Ok((v2, v0))
        }
    }
}

pub fn f15(v0: u64, v1: u64, v2: Bytes, v3: Adt4) -> Adt4 {
    Adt4::C4(std::rc::Rc::new(f7(Str::lit("id"), f22(v0), f7(Str::lit("m"), f28(v1), f7(Str::lit("s"), Adt4::C2(v2), f7(Str::lit("t"), v3, Adt4::C5))))))
}

pub fn f16(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a80 = 0u64;
        let a81 = v1;
        nat_lt(a80, a81)
    } else {
        false
    } {
        let a82 = Bytes::lit(&[0x2d]);
        let a83 = f29(v1)?;
        Ok(append_bytes(a82, a83))
    } else {
        f29(v1)
    }
}

pub fn f17(v0: Bytes) -> R<Bytes> {
    let a89 = {
        let a87 = Bytes::lit(&[0x22]);
        let a88 = {
            let (_, h84) = {
                let (h85, _) = f30(Fn0::F31(v0.clone()), {
                    let a86 = v0;
                    length_bytes(a86)
                }, (0u64, Bytes::lit(&[])))?;
                h85
            };
            h84
        };
        append_bytes(a87, a88)
    };
    let a90 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a89, a90))
}

pub fn f18(v0: List<Bytes>) -> Bytes {
    let a93 = {
        let a91 = Bytes::lit(&[0x5b]);
        let a92 = f32(v0, Bytes::lit(&[0x2c]));
        append_bytes(a91, a92)
    };
    let a94 = Bytes::lit(&[0x5d]);
    append_bytes(a93, a94)
}

pub fn f19(v0: Adt4) -> R<List<Bytes>> {
    let m95 = v0;
    match m95 {
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
        Adt4::C6(r96, r97) => {
            let v7 = (*r96).clone();
            let v8 = (*r97).clone();
            Ok(List::cons(f5(v7)?, f19(v8)?))
        }
        Adt4::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f20(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f33(Fn1::F34, List::<(Bytes, Bytes)>::nil(), f35(v0)?);
    let v2: List<Bytes> = f36(Fn2::F37, List::<Bytes>::nil(), v1)?;
    let a100 = {
        let a98 = Bytes::lit(&[0x7b]);
        let a99 = f32(v2, Bytes::lit(&[0x2c]));
        append_bytes(a98, a99)
    };
    let a101 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a100, a101))
}

pub fn f21(v0: Adt4) -> R<List<(Bytes, Bytes)>> {
    let m102 = v0;
    match m102 {
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
        Adt4::C7(v9, r103, r104) => {
            let v10 = (*r103).clone();
            let v11 = (*r104).clone();
            Ok(List::cons((v9, f5(v10)?), f21(v11)?))
        }
    }
}

pub fn f22(v0: u64) -> Adt4 {
    Adt4::C1(false, f38(v0))
}

pub fn f23(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f39(Fn3::F40(v1), None::<u64>, v0)
}

pub fn f24(v0: Adt5, v1: u64) -> R<(u64, Adt5)> {
    Ok((match v0.clone() {
        Adt5::C0(_, h105, _) => {
            h105
        }
    }, Adt5::C0(List::cons((v1, match v0.clone() {
        Adt5::C0(_, h106, _) => {
            h106
        }
    }), match v0.clone() {
        Adt5::C0(h107, _, _) => {
            h107
        }
    }), {
        let a109 = match v0.clone() {
            Adt5::C0(_, h108, _) => {
                h108
            }
        };
        let a110 = 1u64;
        nat_add(a109, a110)?
    }, match v0 {
        Adt5::C0(_, _, h111) => {
            h111
        }
    })))
}

pub fn f25(v0: Adt1) -> bool {
    let m112 = v0;
    match m112 {
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

pub fn f26(v0: Adt2) -> R<Adt4> {
    let m113 = v0;
    match m113 {
        Adt2::C0(v1) => {
            Ok(Adt4::C4(std::rc::Rc::new(f7(Str::lit("kind"), f8(Str::lit("core")), f7(Str::lit("name"), Adt4::C2(f41(v1)), Adt4::C5)))))
        }
        Adt2::C1(_, v3, v4, v5, v6) => {
            Ok(Adt4::C4(std::rc::Rc::new(f7(Str::lit("kind"), f8(Str::lit("external")), f7(Str::lit("entry"), Adt4::C2(v3), f7(Str::lit("lean_module"), Adt4::C2(v4), f7(Str::lit("lean_name"), Adt4::C2(v5), f7(Str::lit("signature_sha256"), Adt4::C2(f42(v6)?), Adt4::C5))))))))
        }
        Adt2::C2(v7, v8, v9) => {
            Ok(Adt4::C4(std::rc::Rc::new(f7(Str::lit("kind"), f8(Str::lit("document")), f7(Str::lit("module"), Adt4::C2(v7), f7(Str::lit("component"), Adt4::C2(v8), f7(Str::lit("lean_name"), Adt4::C2(v9), Adt4::C5)))))))
        }
        Adt2::C3(_, v11, v12) => {
            Ok(Adt4::C4(std::rc::Rc::new(f7(Str::lit("kind"), f8(Str::lit("defined")), f7(Str::lit("entry"), Adt4::C2(v11), f7(Str::lit("signature_sha256"), Adt4::C2(f42(v12)?), Adt4::C5))))))
        }
    }
}

pub fn f27(v0: List<u64>) -> Adt4 {
    f43(Fn4::F44, Adt4::C5, f45(v0))
}

pub fn f28(v0: u64) -> Adt4 {
    Adt4::C2(f46(v0))
}

pub fn f29(v0: u64) -> R<Bytes> {
    f47(f48(), 10u64, v0)
}

pub fn f30(v0: Fn0, p118: u64, p119: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m117 = (p118, p119);
    {
        let mut m117 = m117;
        loop {
            let step = {
                let (v1, v2) = m117;
                let m114 = v1;
                if m114 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m114 - 1;
                    let m116 = {
                        let c115: Fn0 = v0.clone();
                        c115.apply(v2.clone())?
                    };
                    match m116 {
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
                Ok(next) => { m117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f49(v0, v1)
}

pub fn f32(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m120 = f50(Fn5::F51(v1), None::<Bytes>, v0);
    match m120 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f33(v0: Fn1, p124: List<(Bytes, Bytes)>, p125: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m123 = (p124, p125);
    {
        let mut m123 = m123;
        loop {
            let step = {
                let (v1, v2) = m123;
                let m121 = v2;
                match m121.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c122: Fn1 = v0.clone();
                            c122.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f52(v0, v1)
}

pub fn f35(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f53(v0)
}

pub fn f36(v0: Fn2, p129: List<Bytes>, p130: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m128 = (p129, p130);
    {
        let mut m128 = m128;
        loop {
            let step = {
                let (v1, v2) = m128;
                let m126 = v2;
                match m126.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c127: Fn2 = v0.clone();
                            c127.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f54(v1)?, v0))
}

pub fn f38(v0: u64) -> u64 {
    let a131 = v0;
    let a132 = 9223372036854775807u64;
    if nat_le(a131, a132) {
        v0
    } else {
        9223372036854775807u64
    }
}

pub fn f39(v0: Fn3, p136: Option<u64>, p137: List<(u64, u64)>) -> Option<u64> {
    let m135 = (p136, p137);
    {
        let mut m135 = m135;
        loop {
            let step = {
                let (v1, v2) = m135;
                let m133 = v2;
                match m133.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c134: Fn3 = v0.clone();
                            c134.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let a139 = {
        let (h138, _) = v2;
        h138
    };
    let a140 = v0;
    if nat_eq(a139, a140) {
        Some({
            let (_, h141) = v2;
            h141
        })
    } else {
        v1
    }
}

pub fn f41(v0: Adt3) -> Bytes {
    let m142 = v0;
    match m142 {
        Adt3::C0 => {
            f6(Str::lit("eq"))
        }
        Adt3::C1 => {
            f6(Str::lit("and"))
        }
        Adt3::C2 => {
            f6(Str::lit("or"))
        }
        Adt3::C3 => {
            f6(Str::lit("not"))
        }
        Adt3::C4 => {
            f6(Str::lit("iff"))
        }
        Adt3::C5 => {
            f6(Str::lit("exists"))
        }
        Adt3::C6 => {
            f6(Str::lit("exists-unique"))
        }
    }
}

pub fn f42(v0: Bytes) -> R<Bytes> {
    let (_, h143) = {
        let (h144, _) = f30(Fn0::F55(v0.clone()), {
            let a145 = v0;
            length_bytes(a145)
        }, (0u64, Bytes::lit(&[])))?;
        h144
    };
    Ok(h143)
}

pub fn f43(v0: Fn4, p149: Adt4, p150: List<u64>) -> Adt4 {
    let m148 = (p149, p150);
    {
        let mut m148 = m148;
        loop {
            let step = {
                let (v1, v2) = m148;
                let m146 = v2;
                match m146.uncons() {
                    None => {
                        Err::<(Adt4, List<u64>), Adt4>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<u64>), Adt4>(({
                            let c147: Fn4 = v0.clone();
                            c147.apply(v1, v3)
                        }, v4))
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

pub fn f44(v0: Adt4, v1: u64) -> Adt4 {
    Adt4::C6(std::rc::Rc::new(f22(v1)), std::rc::Rc::new(v0))
}

pub fn f45(v0: List<u64>) -> List<u64> {
    f56(Fn6::F57, List::<u64>::nil(), v0)
}

pub fn f46(v0: u64) -> Bytes {
    let a151 = v0;
    let a152 = 1u64;
    if nat_eq(a151, a152) {
        f58(Str::lit("implicit"))
    } else {
        let a153 = v0;
        let a154 = 2u64;
        if nat_eq(a153, a154) {
            f58(Str::lit("instance"))
        } else {
            f58(Str::lit("explicit"))
        }
    }
}

pub fn f47(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a155 = v2;
    let a156 = 0u64;
    if nat_eq(a155, a156) {
        Ok(f59(v0, 0u64))
    } else {
        let (_, h157) = {
            let (h158, _) = f30(Fn0::F60(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h158
        };
        Ok(h157)
    }
}

pub fn f48() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f49(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m162 = {
        let a160 = v0.clone();
        let a161 = {
            let (h159, _) = v1.clone();
            h159
        };
        index_bytes(a160, a161)
    };
    match m162 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a164 = {
                    let (h163, _) = v1.clone();
                    h163
                };
                let a165 = 1u64;
                nat_add(a164, a165)?
            }, {
                let a168 = {
                    let (_, h166) = v1;
                    h166
                };
                let a169 = f61(v0, {
                    let (h167, _) = v1;
                    h167
                }, v2)?;
                append_bytes(a168, a169)
            })))
        }
    }
}

pub fn f50(v0: Fn5, p173: Option<Bytes>, p174: List<Bytes>) -> Option<Bytes> {
    let m172 = (p173, p174);
    {
        let mut m172 = m172;
        loop {
            let step = {
                let (v1, v2) = m172;
                let m170 = v2;
                match m170.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c171: Fn5 = v0.clone();
                            c171.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m175 = v1;
    match m175 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a178 = {
                    let a176 = v3;
                    let a177 = v0;
                    append_bytes(a176, a177)
                };
                let a179 = v2;
                append_bytes(a178, a179)
            })
        }
    }
}

pub fn f52(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m180 = v0.clone();
    match m180.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a183 = {
                let (h181, _) = v2;
                h181
            };
            let a184 = {
                let (h182, _) = v1.clone();
                h182
            };
            if equal(a183, a184) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f53(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f62(f63(Fn7::F64, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m189 = {
        let (h185, _) = f65(Fn8::F66, {
            let a187 = {
                let a186 = v0;
                length_list(a186)
            };
            let a188 = 1u64;
            nat_add(a187, a188)?
        }, v1)?;
        h185
    };
    match m189.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f54(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a194 = {
        let a191 = f17({
            let (h190, _) = v0;
            h190
        })?;
        let a192 = Bytes::lit(&[0x3a]);
        append_bytes(a191, a192)
    };
    let a195 = {
        let (_, h193) = v0;
        h193
    };
    Ok(append_bytes(a194, a195))
}

pub fn f55(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f67(v0, v1)
}

pub fn f56(v0: Fn6, p199: List<u64>, p200: List<u64>) -> List<u64> {
    let m198 = (p199, p200);
    {
        let mut m198 = m198;
        loop {
            let step = {
                let (v1, v2) = m198;
                let m196 = v2;
                match m196.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c197: Fn6 = v0.clone();
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

pub fn f57(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f58(v0: Str) -> Bytes {
    let a201 = v0;
    utf8_encode(a201)
}

pub fn f59(v0: Bytes, v1: u64) -> Bytes {
    let m205 = {
        let a202 = v0;
        let a203 = v1;
        let a204 = 1u64;
        slice_bytes(a202, a203, a204)
    };
    match m205 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f60(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f68(v1, v0, v2)
}

pub fn f61(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a250 = {
            let a226 = {
                let a214 = {
                    let a208 = {
                        let a206 = v3;
                        let a207 = 128u8;
                        fixed_u8::bit_and(a206, a207)
                    };
                    let a209 = 128u8;
                    if equal(a208, a209) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a215 = {
                    let a212 = {
                        let a210 = v3;
                        let a211 = 64u8;
                        fixed_u8::bit_and(a210, a211)
                    };
                    let a213 = 64u8;
                    if equal(a212, a213) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a214, a215)?
            };
            let a227 = {
                let a224 = {
                    let a218 = {
                        let a216 = v3;
                        let a217 = 32u8;
                        fixed_u8::bit_and(a216, a217)
                    };
                    let a219 = 32u8;
                    if equal(a218, a219) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a225 = {
                    let a222 = {
                        let a220 = v3;
                        let a221 = 16u8;
                        fixed_u8::bit_and(a220, a221)
                    };
                    let a223 = 16u8;
                    if equal(a222, a223) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a224, a225)?
            };
            nat_add(a226, a227)?
        };
        let a251 = {
            let a248 = {
                let a236 = {
                    let a230 = {
                        let a228 = v3;
                        let a229 = 8u8;
                        fixed_u8::bit_and(a228, a229)
                    };
                    let a231 = 8u8;
                    if equal(a230, a231) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a237 = {
                    let a234 = {
                        let a232 = v3;
                        let a233 = 4u8;
                        fixed_u8::bit_and(a232, a233)
                    };
                    let a235 = 4u8;
                    if equal(a234, a235) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a236, a237)?
            };
            let a249 = {
                let a246 = {
                    let a240 = {
                        let a238 = v3;
                        let a239 = 2u8;
                        fixed_u8::bit_and(a238, a239)
                    };
                    let a241 = 2u8;
                    if equal(a240, a241) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a247 = {
                    let a244 = {
                        let a242 = v3;
                        let a243 = 1u8;
                        fixed_u8::bit_and(a242, a243)
                    };
                    let a245 = 1u8;
                    if equal(a244, a245) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a246, a247)?
            };
            nat_add(a248, a249)?
        };
        nat_add(a250, a251)?
    };
    let a252 = v4;
    let a253 = 34u64;
    if nat_eq(a252, a253) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a254 = v4;
        let a255 = 92u64;
        if nat_eq(a254, a255) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a256 = v4;
            let a257 = 8u64;
            if nat_eq(a256, a257) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a258 = v4;
                let a259 = 12u64;
                if nat_eq(a258, a259) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a260 = v4;
                    let a261 = 10u64;
                    if nat_eq(a260, a261) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a262 = v4;
                        let a263 = 13u64;
                        if nat_eq(a262, a263) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a264 = v4;
                            let a265 = 9u64;
                            if nat_eq(a264, a265) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a266 = v4;
                                let a267 = 32u64;
                                if nat_lt(a266, a267) {
                                    let a268 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a269 = f69(v2)?;
                                    Ok(append_bytes(a268, a269))
                                } else {
                                    Ok(f70(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f62(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f71(Fn9::F72, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f63(v0: Fn7, p273: List<List<(Bytes, Bytes)>>, p274: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m272 = (p273, p274);
    {
        let mut m272 = m272;
        loop {
            let step = {
                let (v1, v2) = m272;
                let m270 = v2;
                match m270.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c271: Fn7 = v0.clone();
                            c271.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f65(v0: Fn8, p279: u64, p280: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m278 = (p279, p280);
    {
        let mut m278 = m278;
        loop {
            let step = {
                let (v1, v2) = m278;
                let m275 = v1;
                if m275 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m275 - 1;
                    let m277 = {
                        let c276: Fn8 = v0.clone();
                        c276.apply(v2.clone())?
                    };
                    match m277 {
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
                Ok(next) => { m278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f73(v0)
}

pub fn f67(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m284 = {
        let a282 = v0;
        let a283 = {
            let (h281, _) = v1.clone();
            h281
        };
        index_bytes(a282, a283)
    };
    match m284 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a286 = {
                    let (h285, _) = v1;
                    h285
                };
                let a287 = 1u64;
                nat_add(a286, a287)?
            }, {
                let a289 = {
                    let (_, h288) = v1;
                    h288
                };
                let a290 = f69(v2)?;
                append_bytes(a289, a290)
            })))
        }
    }
}

pub fn f68(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h291, _) = v2;
        h291
    };
    let a292 = v3;
    let a293 = 0u64;
    if nat_eq(a292, a293) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a294 = v3;
            let a295 = v1;
            let a296 = 0u64;
            nat_quot(a294, a295, a296)
        }, {
            let a301 = f59(v0, {
                let a297 = v3;
                let a298 = v1;
                let a299 = 0u64;
                nat_rem(a297, a298, a299)
            });
            let a302 = {
                let (_, h300) = v2;
                h300
            };
            append_bytes(a301, a302)
        }))
    }
}

pub fn f69(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a347 = {
            let a323 = {
                let a311 = {
                    let a305 = {
                        let a303 = v1;
                        let a304 = 128u8;
                        fixed_u8::bit_and(a303, a304)
                    };
                    let a306 = 128u8;
                    if equal(a305, a306) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a312 = {
                    let a309 = {
                        let a307 = v1;
                        let a308 = 64u8;
                        fixed_u8::bit_and(a307, a308)
                    };
                    let a310 = 64u8;
                    if equal(a309, a310) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a311, a312)?
            };
            let a324 = {
                let a321 = {
                    let a315 = {
                        let a313 = v1;
                        let a314 = 32u8;
                        fixed_u8::bit_and(a313, a314)
                    };
                    let a316 = 32u8;
                    if equal(a315, a316) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a322 = {
                    let a319 = {
                        let a317 = v1;
                        let a318 = 16u8;
                        fixed_u8::bit_and(a317, a318)
                    };
                    let a320 = 16u8;
                    if equal(a319, a320) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a321, a322)?
            };
            nat_add(a323, a324)?
        };
        let a348 = {
            let a345 = {
                let a333 = {
                    let a327 = {
                        let a325 = v1;
                        let a326 = 8u8;
                        fixed_u8::bit_and(a325, a326)
                    };
                    let a328 = 8u8;
                    if equal(a327, a328) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a334 = {
                    let a331 = {
                        let a329 = v1;
                        let a330 = 4u8;
                        fixed_u8::bit_and(a329, a330)
                    };
                    let a332 = 4u8;
                    if equal(a331, a332) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a333, a334)?
            };
            let a346 = {
                let a343 = {
                    let a337 = {
                        let a335 = v1;
                        let a336 = 2u8;
                        fixed_u8::bit_and(a335, a336)
                    };
                    let a338 = 2u8;
                    if equal(a337, a338) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a344 = {
                    let a341 = {
                        let a339 = v1;
                        let a340 = 1u8;
                        fixed_u8::bit_and(a339, a340)
                    };
                    let a342 = 1u8;
                    if equal(a341, a342) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a343, a344)?
            };
            nat_add(a345, a346)?
        };
        nat_add(a347, a348)?
    };
    let a355 = f59(f74(), {
        let a349 = v2;
        let a350 = 16u64;
        let a351 = 0u64;
        nat_quot(a349, a350, a351)
    });
    let a356 = f59(f74(), {
        let a352 = v2;
        let a353 = 16u64;
        let a354 = 0u64;
        nat_rem(a352, a353, a354)
    });
    Ok(append_bytes(a355, a356))
}

pub fn f70(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m360 = {
        let a357 = v0;
        let a358 = v1;
        let a359 = v2;
        slice_bytes(a357, a358, a359)
    };
    match m360 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f71(v0: Fn9, p364: List<List<(Bytes, Bytes)>>, p365: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m363 = (p364, p365);
    {
        let mut m363 = m363;
        loop {
            let step = {
                let (v1, v2) = m363;
                let m361 = v2;
                match m361.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c362: Fn9 = v0.clone();
                            c362.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m363 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f73(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a367 = {
        let a366 = v0.clone();
        length_list(a366)
    };
    let a368 = 1u64;
    if nat_le(a367, a368) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f75(v0)?))
    }
}

pub fn f74() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f75(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f62({
        let (_, h369) = {
            let (h370, _) = f76(Fn10::F77, {
                let a372 = {
                    let a371 = v0.clone();
                    length_list(a371)
                };
                let a373 = 1u64;
                nat_add(a372, a373)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h370
        };
        h369
    }))
}

pub fn f76(v0: Fn10, p378: u64, p379: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m377 = (p378, p379);
    {
        let mut m377 = m377;
        loop {
            let step = {
                let (v1, v2) = m377;
                let m374 = v1;
                if m374 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m374 - 1;
                    let m376 = {
                        let c375: Fn10 = v0.clone();
                        c375.apply(v2.clone())?
                    };
                    match m376 {
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
                Ok(next) => { m377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f78(v0)
}

pub fn f78(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h380, _) = v0;
        h380
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h381) = v0;
        h381
    };
    let m382 = v1;
    match m382.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m383 = v4;
            match m383.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f79(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f79(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a388 = {
            let a386 = {
                let a384 = v0.clone();
                length_list(a384)
            };
            let a387 = {
                let a385 = v1.clone();
                length_list(a385)
            };
            nat_add(a386, a387)?
        };
        let a389 = 1u64;
        nat_add(a388, a389)?
    };
    Ok(f80({
        let (_, h390) = {
            let (_, h391) = {
                let (h392, _) = f81(Fn11::F82, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h392
            };
            h391
        };
        h390
    }))
}

pub fn f80(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f33(Fn1::F83, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f81(v0: Fn11, p397: u64, p398: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m396 = (p397, p398);
    {
        let mut m396 = m396;
        loop {
            let step = {
                let (v1, v2) = m396;
                let m393 = v1;
                if m393 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m393 - 1;
                    let m395 = {
                        let c394: Fn11 = v0.clone();
                        c394.apply(v2.clone())
                    };
                    match m395 {
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
                Ok(next) => { m396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f84(v0)
}

pub fn f83(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f84(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h399, _) = v0.clone();
        h399
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h400, _) = {
            let (_, h401) = v0.clone();
            h401
        };
        h400
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h402) = {
            let (_, h403) = v0;
            h403
        };
        h402
    };
    let m404 = v1.clone();
    match m404.uncons() {
        None => {
            let m405 = v2;
            match m405.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m406 = v2.clone();
            match m406.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f85({
                        let (h407, _) = v8.clone();
                        h407
                    }, {
                        let (h408, _) = v6.clone();
                        h408
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

pub fn f85(v0: Bytes, v1: Bytes) -> bool {
    let a413 = {
        let a409 = v0;
        let a410 = v1;
        compare_bytes(a409, a410)
    };
    let a414 = {
        let a411 = Bytes::lit(&[]);
        let a412 = Bytes::lit(&[0x01]);
        compare_bytes(a411, a412)
    };
    equal(a413, a414)
}
