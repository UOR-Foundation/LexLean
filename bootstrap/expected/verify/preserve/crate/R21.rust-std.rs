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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<(u64, u64)>, u64, bool),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt15, u64),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F15,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt12>, p1: (Bytes, Adt0)) -> R<List<Adt12>> {
        match self {
            Fn0::F15 => f15(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F30,
    F80,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt12, p1: Adt12) -> Adt12 {
        match self {
            Fn1::F30 => f30(p0, p1),
            Fn1::F80 => f80(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F34,
}

impl Fn2 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn2::F34 => f34(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F37,
}

impl Fn3 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn3::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F39,
}

impl Fn4 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn4::F39 => f39(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F42(Bytes, u64, u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Adt13)) -> R<(u64, Adt13)> {
        match self {
            Fn5::F42(k0, k1, k2) => f42(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F45,
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn6::F45 => f45(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F48(Bytes),
    F94(u64, Bytes),
    F178(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn7::F48(k0) => f48(k0.clone(), p0),
            Fn7::F94(k0, k1) => Ok(f94(*k0, k1.clone(), p0)),
            Fn7::F178(k0) => f178(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F51,
    F197,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F51 => f51(p0, p1),
            Fn8::F197 => f197(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F54,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn9::F54 => f54(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F56,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn10::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F64,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn11::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F74(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn12::F74(k0) => f74(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F79,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<Adt12>, Adt14), p1: Adt1) -> R<(List<Adt12>, Adt14)> {
        match self {
            Fn13::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F84,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt12>, p1: Bytes) -> List<Adt12> {
        match self {
            Fn14::F84 => f84(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F91,
}

impl Fn15 {
    pub fn apply(&self, p0: Adt16) -> R<Adt16> {
        match self {
            Fn15::F91 => f91(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F98,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F98 => f98(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F100,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F100 => f100(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F116,
}

impl Fn18 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn18::F116 => f116(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F118(Adt0),
}

impl Fn19 {
    pub fn apply(&self, p0: Adt12, p1: (Bytes, Bytes)) -> Adt12 {
        match self {
            Fn19::F118(k0) => f118(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F120,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn20::F120 => f120(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F131,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F131 => f131(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F152,
}

impl Fn22 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn22::F152 => f152(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F158(u64),
}

impl Fn23 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn23::F158(k0) => f158(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F162,
    F184,
}

impl Fn24 {
    pub fn apply(&self, p0: Adt12, p1: u64) -> Adt12 {
        match self {
            Fn24::F162 => f162(p0, p1),
            Fn24::F184 => f184(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F170,
}

impl Fn25 {
    pub fn apply(&self, p0: Adt12, p1: Bytes) -> Adt12 {
        match self {
            Fn25::F170 => f170(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F177,
}

impl Fn26 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn26::F177 => f177(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F180,
}

impl Fn27 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn27::F180 => f180(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F183,
}

impl Fn28 {
    pub fn apply(&self, p0: (List<u64>, Adt14), p1: u64) -> R<(List<u64>, Adt14)> {
        match self {
            Fn28::F183 => f183(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F186,
}

impl Fn29 {
    pub fn apply(&self, p0: List<u64>, p1: (u64, Bytes)) -> List<u64> {
        match self {
            Fn29::F186 => f186(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F188,
}

impl Fn30 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn30::F188 => f188(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F196,
}

impl Fn31 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn31::F196 => f196(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes, v2: List<(Bytes, Adt0)>, v3: Bytes) -> R<Bytes> {
    f1(v0, v1, f2(f3(v2)?)?, v3)
}

pub fn f1(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f4(f5(Str::lit("lexlean-semantic-v1")), f6(f6(f6(f6(f7(f5(Str::lit("compiler-semantics")), v0), f7(f5(Str::lit("language")), v1)), f7(f5(Str::lit("toolchain")), f5(Str::lit("leanprover/lean4:v4.32.1")))), f7(f5(Str::lit("linked-ir")), v2)), f7(f5(Str::lit("lexicon-closure")), v3)))
}

pub fn f2(v0: Adt12) -> R<Bytes> {
    f8(v0)
}

pub fn f3(v0: List<(Bytes, Adt0)>) -> R<Adt12> {
    Ok(f9(f10(Str::lit("spec"), f11(Str::lit("lexlean/linked-ir/1")), f10(Str::lit("modules"), f12(f13(f14(Fn0::F15, List::<Adt12>::nil(), f16(f17(v0)?))?)), f18()))))
}

pub fn f4(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f19(f20(f6(f6(v0, Bytes::lit(&[0x00])), v1))?)
}

pub fn f5(v0: Str) -> Bytes {
    let a1 = v0;
    utf8_encode(a1)
}

pub fn f6(v0: Bytes, v1: Bytes) -> Bytes {
    let a2 = v0;
    let a3 = v1;
    append_bytes(a2, a3)
}

pub fn f7(v0: Bytes, v1: Bytes) -> Bytes {
    f6(f6(f6(f21({
        let a4 = v0.clone();
        length_bytes(a4)
    }, 4u64), v0), f21({
        let a5 = v1.clone();
        length_bytes(a5)
    }, 8u64)), v1)
}

pub fn f8(v0: Adt12) -> R<Bytes> {
    let m6 = v0;
    match m6 {
        Adt12::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt12::C1(v2, v3) => {
            f22(v2, v3)
        }
        Adt12::C2(v4) => {
            f23(v4)
        }
        Adt12::C3(r7) => {
            let v5 = (*r7).clone();
            Ok(f24(f25(v5)?))
        }
        Adt12::C4(r8) => {
            let v6 = (*r8).clone();
            f26(f27(v6)?)
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

pub fn f9(v0: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(v0))
}

pub fn f10(v0: Str, v1: Adt12, v2: Adt12) -> Adt12 {
    Adt12::C7(f28(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f11(v0: Str) -> Adt12 {
    Adt12::C2(f28(v0))
}

pub fn f12(v0: Adt12) -> Adt12 {
    Adt12::C3(std::rc::Rc::new(v0))
}

pub fn f13(v0: List<Adt12>) -> Adt12 {
    f29(Fn1::F30, f18(), f31(v0))
}

pub fn f14(v0: Fn0, p12: List<Adt12>, p13: List<(Bytes, Adt0)>) -> R<List<Adt12>> {
    let m11 = (p12, p13);
    {
        let mut m11 = m11;
        loop {
            let step = {
                let (v1, v2) = m11;
                let m9 = v2;
                match m9.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<(Bytes, Adt0)>), R<List<Adt12>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<(Bytes, Adt0)>), R<List<Adt12>>>(({
                            let c10: Fn0 = v0.clone();
                            c10.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m11 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<Adt12>, v1: (Bytes, Adt0)) -> R<List<Adt12>> {
    Ok(List::cons(f32({
        let (_, h14) = v1;
        h14
    })?, v0))
}

pub fn f16(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f33(Fn2::F34, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f17(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f35(f36(Fn3::F37, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m19 = {
        let (h15, _) = f38(Fn4::F39, {
            let a17 = {
                let a16 = v0;
                length_list(a16)
            };
            let a18 = 1u64;
            nat_add(a17, a18)?
        }, v1)?;
        h15
    };
    match m19.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f18() -> Adt12 {
    Adt12::C5
}

pub fn f19(v0: Adt13) -> R<Bytes> {
    Ok(f6(f6(f6(f6(f6(f6(f6(f40(match v0.clone() {
        Adt13::C0(h20, _, _, _, _, _, _, _) => {
            h20
        }
    })?, f40(match v0.clone() {
        Adt13::C0(_, h21, _, _, _, _, _, _) => {
            h21
        }
    })?), f40(match v0.clone() {
        Adt13::C0(_, _, h22, _, _, _, _, _) => {
            h22
        }
    })?), f40(match v0.clone() {
        Adt13::C0(_, _, _, h23, _, _, _, _) => {
            h23
        }
    })?), f40(match v0.clone() {
        Adt13::C0(_, _, _, _, h24, _, _, _) => {
            h24
        }
    })?), f40(match v0.clone() {
        Adt13::C0(_, _, _, _, _, h25, _, _) => {
            h25
        }
    })?), f40(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h26, _) => {
            h26
        }
    })?), f40(match v0 {
        Adt13::C0(_, _, _, _, _, _, _, h27) => {
            h27
        }
    })?))
}

pub fn f20(v0: Bytes) -> R<Adt13> {
    let v1: u64 = {
        let a28 = v0.clone();
        length_bytes(a28)
    };
    let v2: u64 = {
        let a34 = {
            let a31 = {
                let a29 = v1;
                let a30 = 72u64;
                nat_add(a29, a30)?
            };
            let a32 = 64u64;
            let a33 = 0u64;
            nat_quot(a31, a32, a33)
        };
        let a35 = 64u64;
        nat_mul(a34, a35)?
    };
    let (_, h36) = f41(Fn5::F42(v0, v1, v2), {
        let a37 = v2;
        let a38 = 64u64;
        let a39 = 0u64;
        nat_quot(a37, a38, a39)
    }, (0u64, f43()))?;
    Ok(h36)
}

pub fn f21(v0: u64, v1: u64) -> Bytes {
    let (_, h40) = f44(Fn6::F45, v1, (v0, Bytes::lit(&[])));
    h40
}

pub fn f22(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a41 = 0u64;
        let a42 = v1;
        nat_lt(a41, a42)
    } else {
        false
    } {
        let a43 = Bytes::lit(&[0x2d]);
        let a44 = f46(v1)?;
        Ok(append_bytes(a43, a44))
    } else {
        f46(v1)
    }
}

pub fn f23(v0: Bytes) -> R<Bytes> {
    let a50 = {
        let a48 = Bytes::lit(&[0x22]);
        let a49 = {
            let (_, h45) = {
                let (h46, _) = f47(Fn7::F48(v0.clone()), {
                    let a47 = v0;
                    length_bytes(a47)
                }, (0u64, Bytes::lit(&[])))?;
                h46
            };
            h45
        };
        append_bytes(a48, a49)
    };
    let a51 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a50, a51))
}

pub fn f24(v0: List<Bytes>) -> Bytes {
    let a54 = {
        let a52 = Bytes::lit(&[0x5b]);
        let a53 = f49(v0, Bytes::lit(&[0x2c]));
        append_bytes(a52, a53)
    };
    let a55 = Bytes::lit(&[0x5d]);
    append_bytes(a54, a55)
}

pub fn f25(v0: Adt12) -> R<List<Bytes>> {
    let m56 = v0;
    match m56 {
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
        Adt12::C6(r57, r58) => {
            let v7 = (*r57).clone();
            let v8 = (*r58).clone();
            Ok(List::cons(f8(v7)?, f25(v8)?))
        }
        Adt12::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f26(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f50(Fn8::F51, List::<(Bytes, Bytes)>::nil(), f52(v0)?);
    let v2: List<Bytes> = f53(Fn9::F54, List::<Bytes>::nil(), v1)?;
    let a61 = {
        let a59 = Bytes::lit(&[0x7b]);
        let a60 = f49(v2, Bytes::lit(&[0x2c]));
        append_bytes(a59, a60)
    };
    let a62 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a61, a62))
}

pub fn f27(v0: Adt12) -> R<List<(Bytes, Bytes)>> {
    let m63 = v0;
    match m63 {
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
        Adt12::C7(v9, r64, r65) => {
            let v10 = (*r64).clone();
            let v11 = (*r65).clone();
            Ok(List::cons((v9, f8(v10)?), f27(v11)?))
        }
    }
}

pub fn f28(v0: Str) -> Bytes {
    let a66 = v0;
    utf8_encode(a66)
}

pub fn f29(v0: Fn1, p70: Adt12, p71: List<Adt12>) -> Adt12 {
    let m69 = (p70, p71);
    {
        let mut m69 = m69;
        loop {
            let step = {
                let (v1, v2) = m69;
                let m67 = v2;
                match m67.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt12>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt12>), Adt12>(({
                            let c68: Fn1 = v0.clone();
                            c68.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m69 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f31(v0: List<Adt12>) -> List<Adt12> {
    f55(Fn10::F56, List::<Adt12>::nil(), v0)
}

pub fn f32(v0: Adt0) -> R<Adt12> {
    let v1: (Adt12, Adt14) = f57(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h72, _, _, _, _) => {
            h72
        }
    }, f58())?;
    let v2: (Adt12, Adt14) = f59(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h73, _, _, _) => {
            h73
        }
    }, {
        let (_, h74) = v1;
        h74
    })?;
    Ok(f9(f10(Str::lit("name"), f60(match v0.clone() {
        Adt0::C0(h75, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h75
        }
    }), f10(Str::lit("lean_module"), f60(match v0.clone() {
        Adt0::C0(_, h76, _, _, _, _, _, _, _, _, _, _, _) => {
            h76
        }
    }), f10(Str::lit("glossary"), f61(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h77, _, _, _, _, _, _, _) => {
            h77
        }
    }), f10(Str::lit("imports"), f61(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h78, _, _, _, _, _, _) => {
            h78
        }
    }), f10(Str::lit("title"), {
        let (h79, _) = v1;
        h79
    }, f10(Str::lit("blocks"), f12({
        let (h80, _) = v2;
        h80
    }), f62(v0)))))))))
}

pub fn f33(v0: Fn2, p84: List<(Bytes, Adt0)>, p85: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m83 = (p84, p85);
    {
        let mut m83 = m83;
        loop {
            let step = {
                let (v1, v2) = m83;
                let m81 = v2;
                match m81.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c82: Fn2 = v0.clone();
                            c82.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m83 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f35(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f63(Fn11::F64, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f36(v0: Fn3, p89: List<List<(Bytes, Adt0)>>, p90: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m88 = (p89, p90);
    {
        let mut m88 = m88;
        loop {
            let step = {
                let (v1, v2) = m88;
                let m86 = v2;
                match m86.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c87: Fn3 = v0.clone();
                            c87.apply(v1, v3)
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

pub fn f37(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f38(v0: Fn4, p95: u64, p96: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m94 = (p95, p96);
    {
        let mut m94 = m94;
        loop {
            let step = {
                let (v1, v2) = m94;
                let m91 = v1;
                if m91 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m91 - 1;
                    let m93 = {
                        let c92: Fn4 = v0.clone();
                        c92.apply(v2.clone())?
                    };
                    match m93 {
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
                Ok(next) => { m94 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f65(v0)
}

pub fn f40(v0: u32) -> R<Bytes> {
    Ok(f6(f6(f6(f66({
        let v1: u32 = f67(v0, 24u32);
        let a285 = {
            let a189 = {
                let a141 = {
                    let a117 = {
                        let a105 = {
                            let a99 = {
                                let a97 = v1;
                                let a98 = 2147483648u32;
                                fixed_u32::bit_and(a97, a98)
                            };
                            let a100 = 2147483648u32;
                            if equal(a99, a100) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a106 = {
                            let a103 = {
                                let a101 = v1;
                                let a102 = 1073741824u32;
                                fixed_u32::bit_and(a101, a102)
                            };
                            let a104 = 1073741824u32;
                            if equal(a103, a104) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a105, a106)?
                    };
                    let a118 = {
                        let a115 = {
                            let a109 = {
                                let a107 = v1;
                                let a108 = 536870912u32;
                                fixed_u32::bit_and(a107, a108)
                            };
                            let a110 = 536870912u32;
                            if equal(a109, a110) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a116 = {
                            let a113 = {
                                let a111 = v1;
                                let a112 = 268435456u32;
                                fixed_u32::bit_and(a111, a112)
                            };
                            let a114 = 268435456u32;
                            if equal(a113, a114) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a115, a116)?
                    };
                    nat_add(a117, a118)?
                };
                let a142 = {
                    let a139 = {
                        let a127 = {
                            let a121 = {
                                let a119 = v1;
                                let a120 = 134217728u32;
                                fixed_u32::bit_and(a119, a120)
                            };
                            let a122 = 134217728u32;
                            if equal(a121, a122) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a128 = {
                            let a125 = {
                                let a123 = v1;
                                let a124 = 67108864u32;
                                fixed_u32::bit_and(a123, a124)
                            };
                            let a126 = 67108864u32;
                            if equal(a125, a126) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a127, a128)?
                    };
                    let a140 = {
                        let a137 = {
                            let a131 = {
                                let a129 = v1;
                                let a130 = 33554432u32;
                                fixed_u32::bit_and(a129, a130)
                            };
                            let a132 = 33554432u32;
                            if equal(a131, a132) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a138 = {
                            let a135 = {
                                let a133 = v1;
                                let a134 = 16777216u32;
                                fixed_u32::bit_and(a133, a134)
                            };
                            let a136 = 16777216u32;
                            if equal(a135, a136) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a137, a138)?
                    };
                    nat_add(a139, a140)?
                };
                nat_add(a141, a142)?
            };
            let a190 = {
                let a187 = {
                    let a163 = {
                        let a151 = {
                            let a145 = {
                                let a143 = v1;
                                let a144 = 8388608u32;
                                fixed_u32::bit_and(a143, a144)
                            };
                            let a146 = 8388608u32;
                            if equal(a145, a146) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a152 = {
                            let a149 = {
                                let a147 = v1;
                                let a148 = 4194304u32;
                                fixed_u32::bit_and(a147, a148)
                            };
                            let a150 = 4194304u32;
                            if equal(a149, a150) {
                                4194304u64
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
                                let a154 = 2097152u32;
                                fixed_u32::bit_and(a153, a154)
                            };
                            let a156 = 2097152u32;
                            if equal(a155, a156) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a162 = {
                            let a159 = {
                                let a157 = v1;
                                let a158 = 1048576u32;
                                fixed_u32::bit_and(a157, a158)
                            };
                            let a160 = 1048576u32;
                            if equal(a159, a160) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a161, a162)?
                    };
                    nat_add(a163, a164)?
                };
                let a188 = {
                    let a185 = {
                        let a173 = {
                            let a167 = {
                                let a165 = v1;
                                let a166 = 524288u32;
                                fixed_u32::bit_and(a165, a166)
                            };
                            let a168 = 524288u32;
                            if equal(a167, a168) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a174 = {
                            let a171 = {
                                let a169 = v1;
                                let a170 = 262144u32;
                                fixed_u32::bit_and(a169, a170)
                            };
                            let a172 = 262144u32;
                            if equal(a171, a172) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a173, a174)?
                    };
                    let a186 = {
                        let a183 = {
                            let a177 = {
                                let a175 = v1;
                                let a176 = 131072u32;
                                fixed_u32::bit_and(a175, a176)
                            };
                            let a178 = 131072u32;
                            if equal(a177, a178) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a184 = {
                            let a181 = {
                                let a179 = v1;
                                let a180 = 65536u32;
                                fixed_u32::bit_and(a179, a180)
                            };
                            let a182 = 65536u32;
                            if equal(a181, a182) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a183, a184)?
                    };
                    nat_add(a185, a186)?
                };
                nat_add(a187, a188)?
            };
            nat_add(a189, a190)?
        };
        let a286 = {
            let a283 = {
                let a235 = {
                    let a211 = {
                        let a199 = {
                            let a193 = {
                                let a191 = v1;
                                let a192 = 32768u32;
                                fixed_u32::bit_and(a191, a192)
                            };
                            let a194 = 32768u32;
                            if equal(a193, a194) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a200 = {
                            let a197 = {
                                let a195 = v1;
                                let a196 = 16384u32;
                                fixed_u32::bit_and(a195, a196)
                            };
                            let a198 = 16384u32;
                            if equal(a197, a198) {
                                16384u64
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
                                let a202 = 8192u32;
                                fixed_u32::bit_and(a201, a202)
                            };
                            let a204 = 8192u32;
                            if equal(a203, a204) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a210 = {
                            let a207 = {
                                let a205 = v1;
                                let a206 = 4096u32;
                                fixed_u32::bit_and(a205, a206)
                            };
                            let a208 = 4096u32;
                            if equal(a207, a208) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a209, a210)?
                    };
                    nat_add(a211, a212)?
                };
                let a236 = {
                    let a233 = {
                        let a221 = {
                            let a215 = {
                                let a213 = v1;
                                let a214 = 2048u32;
                                fixed_u32::bit_and(a213, a214)
                            };
                            let a216 = 2048u32;
                            if equal(a215, a216) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a222 = {
                            let a219 = {
                                let a217 = v1;
                                let a218 = 1024u32;
                                fixed_u32::bit_and(a217, a218)
                            };
                            let a220 = 1024u32;
                            if equal(a219, a220) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a221, a222)?
                    };
                    let a234 = {
                        let a231 = {
                            let a225 = {
                                let a223 = v1;
                                let a224 = 512u32;
                                fixed_u32::bit_and(a223, a224)
                            };
                            let a226 = 512u32;
                            if equal(a225, a226) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a232 = {
                            let a229 = {
                                let a227 = v1;
                                let a228 = 256u32;
                                fixed_u32::bit_and(a227, a228)
                            };
                            let a230 = 256u32;
                            if equal(a229, a230) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a231, a232)?
                    };
                    nat_add(a233, a234)?
                };
                nat_add(a235, a236)?
            };
            let a284 = {
                let a281 = {
                    let a257 = {
                        let a245 = {
                            let a239 = {
                                let a237 = v1;
                                let a238 = 128u32;
                                fixed_u32::bit_and(a237, a238)
                            };
                            let a240 = 128u32;
                            if equal(a239, a240) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a246 = {
                            let a243 = {
                                let a241 = v1;
                                let a242 = 64u32;
                                fixed_u32::bit_and(a241, a242)
                            };
                            let a244 = 64u32;
                            if equal(a243, a244) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a245, a246)?
                    };
                    let a258 = {
                        let a255 = {
                            let a249 = {
                                let a247 = v1;
                                let a248 = 32u32;
                                fixed_u32::bit_and(a247, a248)
                            };
                            let a250 = 32u32;
                            if equal(a249, a250) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a256 = {
                            let a253 = {
                                let a251 = v1;
                                let a252 = 16u32;
                                fixed_u32::bit_and(a251, a252)
                            };
                            let a254 = 16u32;
                            if equal(a253, a254) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a255, a256)?
                    };
                    nat_add(a257, a258)?
                };
                let a282 = {
                    let a279 = {
                        let a267 = {
                            let a261 = {
                                let a259 = v1;
                                let a260 = 8u32;
                                fixed_u32::bit_and(a259, a260)
                            };
                            let a262 = 8u32;
                            if equal(a261, a262) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a268 = {
                            let a265 = {
                                let a263 = v1;
                                let a264 = 4u32;
                                fixed_u32::bit_and(a263, a264)
                            };
                            let a266 = 4u32;
                            if equal(a265, a266) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a267, a268)?
                    };
                    let a280 = {
                        let a277 = {
                            let a271 = {
                                let a269 = v1;
                                let a270 = 2u32;
                                fixed_u32::bit_and(a269, a270)
                            };
                            let a272 = 2u32;
                            if equal(a271, a272) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a278 = {
                            let a275 = {
                                let a273 = v1;
                                let a274 = 1u32;
                                fixed_u32::bit_and(a273, a274)
                            };
                            let a276 = 1u32;
                            if equal(a275, a276) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a277, a278)?
                    };
                    nat_add(a279, a280)?
                };
                nat_add(a281, a282)?
            };
            nat_add(a283, a284)?
        };
        nat_add(a285, a286)?
    }), f66({
        let v2: u32 = {
            let a287 = f67(v0, 16u32);
            let a288 = 255u32;
            fixed_u32::bit_and(a287, a288)
        };
        let a477 = {
            let a381 = {
                let a333 = {
                    let a309 = {
                        let a297 = {
                            let a291 = {
                                let a289 = v2;
                                let a290 = 2147483648u32;
                                fixed_u32::bit_and(a289, a290)
                            };
                            let a292 = 2147483648u32;
                            if equal(a291, a292) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a298 = {
                            let a295 = {
                                let a293 = v2;
                                let a294 = 1073741824u32;
                                fixed_u32::bit_and(a293, a294)
                            };
                            let a296 = 1073741824u32;
                            if equal(a295, a296) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a297, a298)?
                    };
                    let a310 = {
                        let a307 = {
                            let a301 = {
                                let a299 = v2;
                                let a300 = 536870912u32;
                                fixed_u32::bit_and(a299, a300)
                            };
                            let a302 = 536870912u32;
                            if equal(a301, a302) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a308 = {
                            let a305 = {
                                let a303 = v2;
                                let a304 = 268435456u32;
                                fixed_u32::bit_and(a303, a304)
                            };
                            let a306 = 268435456u32;
                            if equal(a305, a306) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a307, a308)?
                    };
                    nat_add(a309, a310)?
                };
                let a334 = {
                    let a331 = {
                        let a319 = {
                            let a313 = {
                                let a311 = v2;
                                let a312 = 134217728u32;
                                fixed_u32::bit_and(a311, a312)
                            };
                            let a314 = 134217728u32;
                            if equal(a313, a314) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a320 = {
                            let a317 = {
                                let a315 = v2;
                                let a316 = 67108864u32;
                                fixed_u32::bit_and(a315, a316)
                            };
                            let a318 = 67108864u32;
                            if equal(a317, a318) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a319, a320)?
                    };
                    let a332 = {
                        let a329 = {
                            let a323 = {
                                let a321 = v2;
                                let a322 = 33554432u32;
                                fixed_u32::bit_and(a321, a322)
                            };
                            let a324 = 33554432u32;
                            if equal(a323, a324) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a330 = {
                            let a327 = {
                                let a325 = v2;
                                let a326 = 16777216u32;
                                fixed_u32::bit_and(a325, a326)
                            };
                            let a328 = 16777216u32;
                            if equal(a327, a328) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a329, a330)?
                    };
                    nat_add(a331, a332)?
                };
                nat_add(a333, a334)?
            };
            let a382 = {
                let a379 = {
                    let a355 = {
                        let a343 = {
                            let a337 = {
                                let a335 = v2;
                                let a336 = 8388608u32;
                                fixed_u32::bit_and(a335, a336)
                            };
                            let a338 = 8388608u32;
                            if equal(a337, a338) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a344 = {
                            let a341 = {
                                let a339 = v2;
                                let a340 = 4194304u32;
                                fixed_u32::bit_and(a339, a340)
                            };
                            let a342 = 4194304u32;
                            if equal(a341, a342) {
                                4194304u64
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
                                let a346 = 2097152u32;
                                fixed_u32::bit_and(a345, a346)
                            };
                            let a348 = 2097152u32;
                            if equal(a347, a348) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a354 = {
                            let a351 = {
                                let a349 = v2;
                                let a350 = 1048576u32;
                                fixed_u32::bit_and(a349, a350)
                            };
                            let a352 = 1048576u32;
                            if equal(a351, a352) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a353, a354)?
                    };
                    nat_add(a355, a356)?
                };
                let a380 = {
                    let a377 = {
                        let a365 = {
                            let a359 = {
                                let a357 = v2;
                                let a358 = 524288u32;
                                fixed_u32::bit_and(a357, a358)
                            };
                            let a360 = 524288u32;
                            if equal(a359, a360) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a366 = {
                            let a363 = {
                                let a361 = v2;
                                let a362 = 262144u32;
                                fixed_u32::bit_and(a361, a362)
                            };
                            let a364 = 262144u32;
                            if equal(a363, a364) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a365, a366)?
                    };
                    let a378 = {
                        let a375 = {
                            let a369 = {
                                let a367 = v2;
                                let a368 = 131072u32;
                                fixed_u32::bit_and(a367, a368)
                            };
                            let a370 = 131072u32;
                            if equal(a369, a370) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a376 = {
                            let a373 = {
                                let a371 = v2;
                                let a372 = 65536u32;
                                fixed_u32::bit_and(a371, a372)
                            };
                            let a374 = 65536u32;
                            if equal(a373, a374) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a375, a376)?
                    };
                    nat_add(a377, a378)?
                };
                nat_add(a379, a380)?
            };
            nat_add(a381, a382)?
        };
        let a478 = {
            let a475 = {
                let a427 = {
                    let a403 = {
                        let a391 = {
                            let a385 = {
                                let a383 = v2;
                                let a384 = 32768u32;
                                fixed_u32::bit_and(a383, a384)
                            };
                            let a386 = 32768u32;
                            if equal(a385, a386) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a392 = {
                            let a389 = {
                                let a387 = v2;
                                let a388 = 16384u32;
                                fixed_u32::bit_and(a387, a388)
                            };
                            let a390 = 16384u32;
                            if equal(a389, a390) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a391, a392)?
                    };
                    let a404 = {
                        let a401 = {
                            let a395 = {
                                let a393 = v2;
                                let a394 = 8192u32;
                                fixed_u32::bit_and(a393, a394)
                            };
                            let a396 = 8192u32;
                            if equal(a395, a396) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a402 = {
                            let a399 = {
                                let a397 = v2;
                                let a398 = 4096u32;
                                fixed_u32::bit_and(a397, a398)
                            };
                            let a400 = 4096u32;
                            if equal(a399, a400) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a401, a402)?
                    };
                    nat_add(a403, a404)?
                };
                let a428 = {
                    let a425 = {
                        let a413 = {
                            let a407 = {
                                let a405 = v2;
                                let a406 = 2048u32;
                                fixed_u32::bit_and(a405, a406)
                            };
                            let a408 = 2048u32;
                            if equal(a407, a408) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a414 = {
                            let a411 = {
                                let a409 = v2;
                                let a410 = 1024u32;
                                fixed_u32::bit_and(a409, a410)
                            };
                            let a412 = 1024u32;
                            if equal(a411, a412) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a413, a414)?
                    };
                    let a426 = {
                        let a423 = {
                            let a417 = {
                                let a415 = v2;
                                let a416 = 512u32;
                                fixed_u32::bit_and(a415, a416)
                            };
                            let a418 = 512u32;
                            if equal(a417, a418) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a424 = {
                            let a421 = {
                                let a419 = v2;
                                let a420 = 256u32;
                                fixed_u32::bit_and(a419, a420)
                            };
                            let a422 = 256u32;
                            if equal(a421, a422) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a423, a424)?
                    };
                    nat_add(a425, a426)?
                };
                nat_add(a427, a428)?
            };
            let a476 = {
                let a473 = {
                    let a449 = {
                        let a437 = {
                            let a431 = {
                                let a429 = v2;
                                let a430 = 128u32;
                                fixed_u32::bit_and(a429, a430)
                            };
                            let a432 = 128u32;
                            if equal(a431, a432) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a438 = {
                            let a435 = {
                                let a433 = v2;
                                let a434 = 64u32;
                                fixed_u32::bit_and(a433, a434)
                            };
                            let a436 = 64u32;
                            if equal(a435, a436) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a437, a438)?
                    };
                    let a450 = {
                        let a447 = {
                            let a441 = {
                                let a439 = v2;
                                let a440 = 32u32;
                                fixed_u32::bit_and(a439, a440)
                            };
                            let a442 = 32u32;
                            if equal(a441, a442) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a448 = {
                            let a445 = {
                                let a443 = v2;
                                let a444 = 16u32;
                                fixed_u32::bit_and(a443, a444)
                            };
                            let a446 = 16u32;
                            if equal(a445, a446) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a447, a448)?
                    };
                    nat_add(a449, a450)?
                };
                let a474 = {
                    let a471 = {
                        let a459 = {
                            let a453 = {
                                let a451 = v2;
                                let a452 = 8u32;
                                fixed_u32::bit_and(a451, a452)
                            };
                            let a454 = 8u32;
                            if equal(a453, a454) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a460 = {
                            let a457 = {
                                let a455 = v2;
                                let a456 = 4u32;
                                fixed_u32::bit_and(a455, a456)
                            };
                            let a458 = 4u32;
                            if equal(a457, a458) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a459, a460)?
                    };
                    let a472 = {
                        let a469 = {
                            let a463 = {
                                let a461 = v2;
                                let a462 = 2u32;
                                fixed_u32::bit_and(a461, a462)
                            };
                            let a464 = 2u32;
                            if equal(a463, a464) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a470 = {
                            let a467 = {
                                let a465 = v2;
                                let a466 = 1u32;
                                fixed_u32::bit_and(a465, a466)
                            };
                            let a468 = 1u32;
                            if equal(a467, a468) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a469, a470)?
                    };
                    nat_add(a471, a472)?
                };
                nat_add(a473, a474)?
            };
            nat_add(a475, a476)?
        };
        nat_add(a477, a478)?
    })), f66({
        let v3: u32 = {
            let a479 = f67(v0, 8u32);
            let a480 = 255u32;
            fixed_u32::bit_and(a479, a480)
        };
        let a669 = {
            let a573 = {
                let a525 = {
                    let a501 = {
                        let a489 = {
                            let a483 = {
                                let a481 = v3;
                                let a482 = 2147483648u32;
                                fixed_u32::bit_and(a481, a482)
                            };
                            let a484 = 2147483648u32;
                            if equal(a483, a484) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a490 = {
                            let a487 = {
                                let a485 = v3;
                                let a486 = 1073741824u32;
                                fixed_u32::bit_and(a485, a486)
                            };
                            let a488 = 1073741824u32;
                            if equal(a487, a488) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a489, a490)?
                    };
                    let a502 = {
                        let a499 = {
                            let a493 = {
                                let a491 = v3;
                                let a492 = 536870912u32;
                                fixed_u32::bit_and(a491, a492)
                            };
                            let a494 = 536870912u32;
                            if equal(a493, a494) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a500 = {
                            let a497 = {
                                let a495 = v3;
                                let a496 = 268435456u32;
                                fixed_u32::bit_and(a495, a496)
                            };
                            let a498 = 268435456u32;
                            if equal(a497, a498) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a499, a500)?
                    };
                    nat_add(a501, a502)?
                };
                let a526 = {
                    let a523 = {
                        let a511 = {
                            let a505 = {
                                let a503 = v3;
                                let a504 = 134217728u32;
                                fixed_u32::bit_and(a503, a504)
                            };
                            let a506 = 134217728u32;
                            if equal(a505, a506) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a512 = {
                            let a509 = {
                                let a507 = v3;
                                let a508 = 67108864u32;
                                fixed_u32::bit_and(a507, a508)
                            };
                            let a510 = 67108864u32;
                            if equal(a509, a510) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a511, a512)?
                    };
                    let a524 = {
                        let a521 = {
                            let a515 = {
                                let a513 = v3;
                                let a514 = 33554432u32;
                                fixed_u32::bit_and(a513, a514)
                            };
                            let a516 = 33554432u32;
                            if equal(a515, a516) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a522 = {
                            let a519 = {
                                let a517 = v3;
                                let a518 = 16777216u32;
                                fixed_u32::bit_and(a517, a518)
                            };
                            let a520 = 16777216u32;
                            if equal(a519, a520) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a521, a522)?
                    };
                    nat_add(a523, a524)?
                };
                nat_add(a525, a526)?
            };
            let a574 = {
                let a571 = {
                    let a547 = {
                        let a535 = {
                            let a529 = {
                                let a527 = v3;
                                let a528 = 8388608u32;
                                fixed_u32::bit_and(a527, a528)
                            };
                            let a530 = 8388608u32;
                            if equal(a529, a530) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a536 = {
                            let a533 = {
                                let a531 = v3;
                                let a532 = 4194304u32;
                                fixed_u32::bit_and(a531, a532)
                            };
                            let a534 = 4194304u32;
                            if equal(a533, a534) {
                                4194304u64
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
                                let a538 = 2097152u32;
                                fixed_u32::bit_and(a537, a538)
                            };
                            let a540 = 2097152u32;
                            if equal(a539, a540) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a546 = {
                            let a543 = {
                                let a541 = v3;
                                let a542 = 1048576u32;
                                fixed_u32::bit_and(a541, a542)
                            };
                            let a544 = 1048576u32;
                            if equal(a543, a544) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a545, a546)?
                    };
                    nat_add(a547, a548)?
                };
                let a572 = {
                    let a569 = {
                        let a557 = {
                            let a551 = {
                                let a549 = v3;
                                let a550 = 524288u32;
                                fixed_u32::bit_and(a549, a550)
                            };
                            let a552 = 524288u32;
                            if equal(a551, a552) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a558 = {
                            let a555 = {
                                let a553 = v3;
                                let a554 = 262144u32;
                                fixed_u32::bit_and(a553, a554)
                            };
                            let a556 = 262144u32;
                            if equal(a555, a556) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a557, a558)?
                    };
                    let a570 = {
                        let a567 = {
                            let a561 = {
                                let a559 = v3;
                                let a560 = 131072u32;
                                fixed_u32::bit_and(a559, a560)
                            };
                            let a562 = 131072u32;
                            if equal(a561, a562) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a568 = {
                            let a565 = {
                                let a563 = v3;
                                let a564 = 65536u32;
                                fixed_u32::bit_and(a563, a564)
                            };
                            let a566 = 65536u32;
                            if equal(a565, a566) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a567, a568)?
                    };
                    nat_add(a569, a570)?
                };
                nat_add(a571, a572)?
            };
            nat_add(a573, a574)?
        };
        let a670 = {
            let a667 = {
                let a619 = {
                    let a595 = {
                        let a583 = {
                            let a577 = {
                                let a575 = v3;
                                let a576 = 32768u32;
                                fixed_u32::bit_and(a575, a576)
                            };
                            let a578 = 32768u32;
                            if equal(a577, a578) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a584 = {
                            let a581 = {
                                let a579 = v3;
                                let a580 = 16384u32;
                                fixed_u32::bit_and(a579, a580)
                            };
                            let a582 = 16384u32;
                            if equal(a581, a582) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a583, a584)?
                    };
                    let a596 = {
                        let a593 = {
                            let a587 = {
                                let a585 = v3;
                                let a586 = 8192u32;
                                fixed_u32::bit_and(a585, a586)
                            };
                            let a588 = 8192u32;
                            if equal(a587, a588) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a594 = {
                            let a591 = {
                                let a589 = v3;
                                let a590 = 4096u32;
                                fixed_u32::bit_and(a589, a590)
                            };
                            let a592 = 4096u32;
                            if equal(a591, a592) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a593, a594)?
                    };
                    nat_add(a595, a596)?
                };
                let a620 = {
                    let a617 = {
                        let a605 = {
                            let a599 = {
                                let a597 = v3;
                                let a598 = 2048u32;
                                fixed_u32::bit_and(a597, a598)
                            };
                            let a600 = 2048u32;
                            if equal(a599, a600) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a606 = {
                            let a603 = {
                                let a601 = v3;
                                let a602 = 1024u32;
                                fixed_u32::bit_and(a601, a602)
                            };
                            let a604 = 1024u32;
                            if equal(a603, a604) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a605, a606)?
                    };
                    let a618 = {
                        let a615 = {
                            let a609 = {
                                let a607 = v3;
                                let a608 = 512u32;
                                fixed_u32::bit_and(a607, a608)
                            };
                            let a610 = 512u32;
                            if equal(a609, a610) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a616 = {
                            let a613 = {
                                let a611 = v3;
                                let a612 = 256u32;
                                fixed_u32::bit_and(a611, a612)
                            };
                            let a614 = 256u32;
                            if equal(a613, a614) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a615, a616)?
                    };
                    nat_add(a617, a618)?
                };
                nat_add(a619, a620)?
            };
            let a668 = {
                let a665 = {
                    let a641 = {
                        let a629 = {
                            let a623 = {
                                let a621 = v3;
                                let a622 = 128u32;
                                fixed_u32::bit_and(a621, a622)
                            };
                            let a624 = 128u32;
                            if equal(a623, a624) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a630 = {
                            let a627 = {
                                let a625 = v3;
                                let a626 = 64u32;
                                fixed_u32::bit_and(a625, a626)
                            };
                            let a628 = 64u32;
                            if equal(a627, a628) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a629, a630)?
                    };
                    let a642 = {
                        let a639 = {
                            let a633 = {
                                let a631 = v3;
                                let a632 = 32u32;
                                fixed_u32::bit_and(a631, a632)
                            };
                            let a634 = 32u32;
                            if equal(a633, a634) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a640 = {
                            let a637 = {
                                let a635 = v3;
                                let a636 = 16u32;
                                fixed_u32::bit_and(a635, a636)
                            };
                            let a638 = 16u32;
                            if equal(a637, a638) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a639, a640)?
                    };
                    nat_add(a641, a642)?
                };
                let a666 = {
                    let a663 = {
                        let a651 = {
                            let a645 = {
                                let a643 = v3;
                                let a644 = 8u32;
                                fixed_u32::bit_and(a643, a644)
                            };
                            let a646 = 8u32;
                            if equal(a645, a646) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a652 = {
                            let a649 = {
                                let a647 = v3;
                                let a648 = 4u32;
                                fixed_u32::bit_and(a647, a648)
                            };
                            let a650 = 4u32;
                            if equal(a649, a650) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a651, a652)?
                    };
                    let a664 = {
                        let a661 = {
                            let a655 = {
                                let a653 = v3;
                                let a654 = 2u32;
                                fixed_u32::bit_and(a653, a654)
                            };
                            let a656 = 2u32;
                            if equal(a655, a656) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a662 = {
                            let a659 = {
                                let a657 = v3;
                                let a658 = 1u32;
                                fixed_u32::bit_and(a657, a658)
                            };
                            let a660 = 1u32;
                            if equal(a659, a660) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a661, a662)?
                    };
                    nat_add(a663, a664)?
                };
                nat_add(a665, a666)?
            };
            nat_add(a667, a668)?
        };
        nat_add(a669, a670)?
    })), f66({
        let v4: u32 = {
            let a671 = v0;
            let a672 = 255u32;
            fixed_u32::bit_and(a671, a672)
        };
        let a861 = {
            let a765 = {
                let a717 = {
                    let a693 = {
                        let a681 = {
                            let a675 = {
                                let a673 = v4;
                                let a674 = 2147483648u32;
                                fixed_u32::bit_and(a673, a674)
                            };
                            let a676 = 2147483648u32;
                            if equal(a675, a676) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a682 = {
                            let a679 = {
                                let a677 = v4;
                                let a678 = 1073741824u32;
                                fixed_u32::bit_and(a677, a678)
                            };
                            let a680 = 1073741824u32;
                            if equal(a679, a680) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a681, a682)?
                    };
                    let a694 = {
                        let a691 = {
                            let a685 = {
                                let a683 = v4;
                                let a684 = 536870912u32;
                                fixed_u32::bit_and(a683, a684)
                            };
                            let a686 = 536870912u32;
                            if equal(a685, a686) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a692 = {
                            let a689 = {
                                let a687 = v4;
                                let a688 = 268435456u32;
                                fixed_u32::bit_and(a687, a688)
                            };
                            let a690 = 268435456u32;
                            if equal(a689, a690) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a691, a692)?
                    };
                    nat_add(a693, a694)?
                };
                let a718 = {
                    let a715 = {
                        let a703 = {
                            let a697 = {
                                let a695 = v4;
                                let a696 = 134217728u32;
                                fixed_u32::bit_and(a695, a696)
                            };
                            let a698 = 134217728u32;
                            if equal(a697, a698) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a704 = {
                            let a701 = {
                                let a699 = v4;
                                let a700 = 67108864u32;
                                fixed_u32::bit_and(a699, a700)
                            };
                            let a702 = 67108864u32;
                            if equal(a701, a702) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a703, a704)?
                    };
                    let a716 = {
                        let a713 = {
                            let a707 = {
                                let a705 = v4;
                                let a706 = 33554432u32;
                                fixed_u32::bit_and(a705, a706)
                            };
                            let a708 = 33554432u32;
                            if equal(a707, a708) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a714 = {
                            let a711 = {
                                let a709 = v4;
                                let a710 = 16777216u32;
                                fixed_u32::bit_and(a709, a710)
                            };
                            let a712 = 16777216u32;
                            if equal(a711, a712) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a713, a714)?
                    };
                    nat_add(a715, a716)?
                };
                nat_add(a717, a718)?
            };
            let a766 = {
                let a763 = {
                    let a739 = {
                        let a727 = {
                            let a721 = {
                                let a719 = v4;
                                let a720 = 8388608u32;
                                fixed_u32::bit_and(a719, a720)
                            };
                            let a722 = 8388608u32;
                            if equal(a721, a722) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a728 = {
                            let a725 = {
                                let a723 = v4;
                                let a724 = 4194304u32;
                                fixed_u32::bit_and(a723, a724)
                            };
                            let a726 = 4194304u32;
                            if equal(a725, a726) {
                                4194304u64
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
                                let a730 = 2097152u32;
                                fixed_u32::bit_and(a729, a730)
                            };
                            let a732 = 2097152u32;
                            if equal(a731, a732) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a738 = {
                            let a735 = {
                                let a733 = v4;
                                let a734 = 1048576u32;
                                fixed_u32::bit_and(a733, a734)
                            };
                            let a736 = 1048576u32;
                            if equal(a735, a736) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a737, a738)?
                    };
                    nat_add(a739, a740)?
                };
                let a764 = {
                    let a761 = {
                        let a749 = {
                            let a743 = {
                                let a741 = v4;
                                let a742 = 524288u32;
                                fixed_u32::bit_and(a741, a742)
                            };
                            let a744 = 524288u32;
                            if equal(a743, a744) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a750 = {
                            let a747 = {
                                let a745 = v4;
                                let a746 = 262144u32;
                                fixed_u32::bit_and(a745, a746)
                            };
                            let a748 = 262144u32;
                            if equal(a747, a748) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a749, a750)?
                    };
                    let a762 = {
                        let a759 = {
                            let a753 = {
                                let a751 = v4;
                                let a752 = 131072u32;
                                fixed_u32::bit_and(a751, a752)
                            };
                            let a754 = 131072u32;
                            if equal(a753, a754) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a760 = {
                            let a757 = {
                                let a755 = v4;
                                let a756 = 65536u32;
                                fixed_u32::bit_and(a755, a756)
                            };
                            let a758 = 65536u32;
                            if equal(a757, a758) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a759, a760)?
                    };
                    nat_add(a761, a762)?
                };
                nat_add(a763, a764)?
            };
            nat_add(a765, a766)?
        };
        let a862 = {
            let a859 = {
                let a811 = {
                    let a787 = {
                        let a775 = {
                            let a769 = {
                                let a767 = v4;
                                let a768 = 32768u32;
                                fixed_u32::bit_and(a767, a768)
                            };
                            let a770 = 32768u32;
                            if equal(a769, a770) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a776 = {
                            let a773 = {
                                let a771 = v4;
                                let a772 = 16384u32;
                                fixed_u32::bit_and(a771, a772)
                            };
                            let a774 = 16384u32;
                            if equal(a773, a774) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a775, a776)?
                    };
                    let a788 = {
                        let a785 = {
                            let a779 = {
                                let a777 = v4;
                                let a778 = 8192u32;
                                fixed_u32::bit_and(a777, a778)
                            };
                            let a780 = 8192u32;
                            if equal(a779, a780) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a786 = {
                            let a783 = {
                                let a781 = v4;
                                let a782 = 4096u32;
                                fixed_u32::bit_and(a781, a782)
                            };
                            let a784 = 4096u32;
                            if equal(a783, a784) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a785, a786)?
                    };
                    nat_add(a787, a788)?
                };
                let a812 = {
                    let a809 = {
                        let a797 = {
                            let a791 = {
                                let a789 = v4;
                                let a790 = 2048u32;
                                fixed_u32::bit_and(a789, a790)
                            };
                            let a792 = 2048u32;
                            if equal(a791, a792) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a798 = {
                            let a795 = {
                                let a793 = v4;
                                let a794 = 1024u32;
                                fixed_u32::bit_and(a793, a794)
                            };
                            let a796 = 1024u32;
                            if equal(a795, a796) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a797, a798)?
                    };
                    let a810 = {
                        let a807 = {
                            let a801 = {
                                let a799 = v4;
                                let a800 = 512u32;
                                fixed_u32::bit_and(a799, a800)
                            };
                            let a802 = 512u32;
                            if equal(a801, a802) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a808 = {
                            let a805 = {
                                let a803 = v4;
                                let a804 = 256u32;
                                fixed_u32::bit_and(a803, a804)
                            };
                            let a806 = 256u32;
                            if equal(a805, a806) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a807, a808)?
                    };
                    nat_add(a809, a810)?
                };
                nat_add(a811, a812)?
            };
            let a860 = {
                let a857 = {
                    let a833 = {
                        let a821 = {
                            let a815 = {
                                let a813 = v4;
                                let a814 = 128u32;
                                fixed_u32::bit_and(a813, a814)
                            };
                            let a816 = 128u32;
                            if equal(a815, a816) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a822 = {
                            let a819 = {
                                let a817 = v4;
                                let a818 = 64u32;
                                fixed_u32::bit_and(a817, a818)
                            };
                            let a820 = 64u32;
                            if equal(a819, a820) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a821, a822)?
                    };
                    let a834 = {
                        let a831 = {
                            let a825 = {
                                let a823 = v4;
                                let a824 = 32u32;
                                fixed_u32::bit_and(a823, a824)
                            };
                            let a826 = 32u32;
                            if equal(a825, a826) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a832 = {
                            let a829 = {
                                let a827 = v4;
                                let a828 = 16u32;
                                fixed_u32::bit_and(a827, a828)
                            };
                            let a830 = 16u32;
                            if equal(a829, a830) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a831, a832)?
                    };
                    nat_add(a833, a834)?
                };
                let a858 = {
                    let a855 = {
                        let a843 = {
                            let a837 = {
                                let a835 = v4;
                                let a836 = 8u32;
                                fixed_u32::bit_and(a835, a836)
                            };
                            let a838 = 8u32;
                            if equal(a837, a838) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a844 = {
                            let a841 = {
                                let a839 = v4;
                                let a840 = 4u32;
                                fixed_u32::bit_and(a839, a840)
                            };
                            let a842 = 4u32;
                            if equal(a841, a842) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a843, a844)?
                    };
                    let a856 = {
                        let a853 = {
                            let a847 = {
                                let a845 = v4;
                                let a846 = 2u32;
                                fixed_u32::bit_and(a845, a846)
                            };
                            let a848 = 2u32;
                            if equal(a847, a848) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a854 = {
                            let a851 = {
                                let a849 = v4;
                                let a850 = 1u32;
                                fixed_u32::bit_and(a849, a850)
                            };
                            let a852 = 1u32;
                            if equal(a851, a852) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a853, a854)?
                    };
                    nat_add(a855, a856)?
                };
                nat_add(a857, a858)?
            };
            nat_add(a859, a860)?
        };
        nat_add(a861, a862)?
    })))
}

pub fn f41(v0: Fn5, p866: u64, p867: (u64, Adt13)) -> R<(u64, Adt13)> {
    let m865 = (p866, p867);
    {
        let mut m865 = m865;
        loop {
            let step = {
                let (v1, v2) = m865;
                let m863 = v1;
                if m863 == 0 {
                    Err::<(u64, (u64, Adt13)), R<(u64, Adt13)>>(Ok(v2))
                } else {
                    let v3 = m863 - 1;
                    Ok::<(u64, (u64, Adt13)), R<(u64, Adt13)>>((v3, {
                        let c864: Fn5 = v0.clone();
                        c864.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt13)) -> R<(u64, Adt13)> {
    Ok(({
        let a869 = {
            let (h868, _) = v3.clone();
            h868
        };
        let a870 = 64u64;
        nat_add(a869, a870)?
    }, f68({
        let (_, h871) = v3;
        h871
    }, f69(v0, v1, v2, {
        let (h872, _) = v3;
        h872
    })?)?))
}

pub fn f43() -> Adt13 {
    Adt13::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f44(v0: Fn6, p876: u64, p877: (u64, Bytes)) -> (u64, Bytes) {
    let m875 = (p876, p877);
    {
        let mut m875 = m875;
        loop {
            let step = {
                let (v1, v2) = m875;
                let m873 = v1;
                if m873 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m873 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c874: Fn6 = v0.clone();
                        c874.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m875 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a879 = {
            let (h878, _) = v0.clone();
            h878
        };
        let a880 = 256u64;
        let a881 = 0u64;
        nat_quot(a879, a880, a881)
    }, f6(f66({
        let a883 = {
            let (h882, _) = v0;
            h882
        };
        let a884 = 256u64;
        let a885 = 0u64;
        nat_rem(a883, a884, a885)
    }), {
        let (_, h886) = v0;
        h886
    }))
}

pub fn f46(v0: u64) -> R<Bytes> {
    f70(f71(), 10u64, v0)
}

pub fn f47(v0: Fn7, p891: u64, p892: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m890 = (p891, p892);
    {
        let mut m890 = m890;
        loop {
            let step = {
                let (v1, v2) = m890;
                let m887 = v1;
                if m887 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m887 - 1;
                    let m889 = {
                        let c888: Fn7 = v0.clone();
                        c888.apply(v2.clone())?
                    };
                    match m889 {
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
                Ok(next) => { m890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f72(v0, v1)
}

pub fn f49(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m893 = f73(Fn12::F74(v1), None::<Bytes>, v0);
    match m893 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f50(v0: Fn8, p897: List<(Bytes, Bytes)>, p898: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m896 = (p897, p898);
    {
        let mut m896 = m896;
        loop {
            let step = {
                let (v1, v2) = m896;
                let m894 = v2;
                match m894.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c895: Fn8 = v0.clone();
                            c895.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f75(v0, v1)
}

pub fn f52(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f76(v0)
}

pub fn f53(v0: Fn9, p902: List<Bytes>, p903: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v1, v2) = m901;
                let m899 = v2;
                match m899.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c900: Fn9 = v0.clone();
                            c900.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f77(v1)?, v0))
}

pub fn f55(v0: Fn10, p907: List<Adt12>, p908: List<Adt12>) -> List<Adt12> {
    let m906 = (p907, p908);
    {
        let mut m906 = m906;
        loop {
            let step = {
                let (v1, v2) = m906;
                let m904 = v2;
                match m904.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c905: Fn10 = v0.clone();
                            c905.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    List::cons(v1, v0)
}

pub fn f57(v0: List<Adt1>, v1: Adt14) -> R<(Adt12, Adt14)> {
    let v2: (List<Adt12>, Adt14) = f78(Fn13::F79, (List::<Adt12>::nil(), v1), v0)?;
    Ok((f12(f29(Fn1::F80, f18(), {
        let (h909, _) = v2;
        h909
    })), {
        let (_, h910) = v2;
        h910
    }))
}

pub fn f58() -> Adt14 {
    Adt14::C0(List::<(u64, u64)>::nil(), 0u64, false)
}

pub fn f59(v0: Adt6, v1: Adt14) -> R<(Adt12, Adt14)> {
    let m911 = v0;
    match m911 {
        Adt6::C0(v2, v3, v4, r924) => {
            let v5 = (*r924).clone();
            let v6: (Adt12, Adt14) = f57(v3, v1)?;
            let v7: (Adt12, Adt14) = f81(v4, {
                let (_, h912) = v6;
                h912
            })?;
            let v8: (Adt12, Adt14) = f59(v5, {
                let (_, h913) = v7;
                h913
            })?;
            Ok((f9(f10(Str::lit("k"), f11(Str::lit("section")), f10(Str::lit("component"), f60(v2), f10(Str::lit("heading"), {
                let (h914, _) = v6;
                h914
            }, f10(Str::lit("params"), f12({
                let (h915, _) = v7;
                h915
            }), f10(Str::lit("blocks"), f12({
                let (h916, _) = v8;
                h916
            }), f18())))))), {
                let (_, h917) = v8;
                h917
            }))
        }
        Adt6::C1(v9) => {
            let v10: (Adt12, Adt14) = f82(v9, v1)?;
            Ok((f9(f10(Str::lit("k"), f11(Str::lit("declaration")), f10(Str::lit("d"), {
                let (h918, _) = v10;
                h918
            }, f18()))), {
                let (_, h919) = v10;
                h919
            }))
        }
        Adt6::C2 => {
            Ok((f18(), v1))
        }
        Adt6::C3(r925, r926) => {
            let v11 = (*r925).clone();
            let v12 = (*r926).clone();
            let v13: (Adt12, Adt14) = f59(v11, v1)?;
            let v14: (Adt12, Adt14) = f59(v12, {
                let (_, h920) = v13;
                h920
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h921, _) = v13;
                h921
            }), std::rc::Rc::new({
                let (h922, _) = v14;
                h922
            })), {
                let (_, h923) = v14;
                h923
            }))
        }
    }
}

pub fn f60(v0: Bytes) -> Adt12 {
    Adt12::C2(v0)
}

pub fn f61(v0: List<Bytes>) -> Adt12 {
    f12(f13(f83(Fn14::F84, List::<Adt12>::nil(), f85(v0))))
}

pub fn f62(v0: Adt0) -> Adt12 {
    let v2: Adt12 = {
        let m928 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, _, h927) => {
                h927
            }
        };
        match m928 {
            None => {
                f18()
            }
            Some(v1) => {
                f10(Str::lit("tooling"), v1, f18())
            }
        }
    };
    let v4: Adt12 = {
        let m930 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, h929, _) => {
                h929
            }
        };
        match m930 {
            None => {
                v2
            }
            Some(v3) => {
                f10(Str::lit("semantic"), v3, v2)
            }
        }
    };
    let v6: Adt12 = {
        let m932 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, h931, _, _) => {
                h931
            }
        };
        match m932 {
            None => {
                v4
            }
            Some(v5) => {
                f10(Str::lit("core"), f60(v5), v4)
            }
        }
    };
    let v7: Adt12 = f86(v0.clone());
    let v19: Adt12 = {
        let m933 = v7.clone();
        match m933 {
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
                f10(Str::lit("import_lean"), f9(v7), v6)
            }
        }
    };
    let m935 = match v0 {
        Adt0::C0(_, _, _, _, h934, _, _, _, _, _, _, _, _) => {
            h934
        }
    };
    match m935 {
        None => {
            v19
        }
        Some(v20) => {
            f10(Str::lit("origin"), f60(v20), v19)
        }
    }
}

pub fn f63(v0: Fn11, p939: List<List<(Bytes, Adt0)>>, p940: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m938 = (p939, p940);
    {
        let mut m938 = m938;
        loop {
            let step = {
                let (v1, v2) = m938;
                let m936 = v2;
                match m936.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c937: Fn11 = v0.clone();
                            c937.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m938 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f65(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a942 = {
        let a941 = v0.clone();
        length_list(a941)
    };
    let a943 = 1u64;
    if nat_le(a942, a943) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f87(v0)?))
    }
}

pub fn f66(v0: u64) -> Bytes {
    f88(f89(), v0)
}

pub fn f67(v0: u32, v1: u32) -> u32 {
    let m946 = {
        let a944 = v0;
        let a945 = v1;
        fixed_u32::shift_right(a944, a945)
    };
    match m946 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f68(v0: Adt13, v1: Adt15) -> R<Adt13> {
    let v2: Adt16 = f90(Fn15::F91, 64u64, Adt16::C0(match v0.clone() {
        Adt13::C0(h947, _, _, _, _, _, _, _) => {
            h947
        }
    }, match v0.clone() {
        Adt13::C0(_, h948, _, _, _, _, _, _) => {
            h948
        }
    }, match v0.clone() {
        Adt13::C0(_, _, h949, _, _, _, _, _) => {
            h949
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, h950, _, _, _, _) => {
            h950
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, h951, _, _, _) => {
            h951
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, h952, _, _) => {
            h952
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h953, _) => {
            h953
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, h954) => {
            h954
        }
    }, v1, 0u64))?;
    Ok(Adt13::C0(f92(match v0.clone() {
        Adt13::C0(h955, _, _, _, _, _, _, _) => {
            h955
        }
    }, match v2.clone() {
        Adt16::C0(h956, _, _, _, _, _, _, _, _, _) => {
            h956
        }
    }), f92(match v0.clone() {
        Adt13::C0(_, h957, _, _, _, _, _, _) => {
            h957
        }
    }, match v2.clone() {
        Adt16::C0(_, h958, _, _, _, _, _, _, _, _) => {
            h958
        }
    }), f92(match v0.clone() {
        Adt13::C0(_, _, h959, _, _, _, _, _) => {
            h959
        }
    }, match v2.clone() {
        Adt16::C0(_, _, h960, _, _, _, _, _, _, _) => {
            h960
        }
    }), f92(match v0.clone() {
        Adt13::C0(_, _, _, h961, _, _, _, _) => {
            h961
        }
    }, match v2.clone() {
        Adt16::C0(_, _, _, h962, _, _, _, _, _, _) => {
            h962
        }
    }), f92(match v0.clone() {
        Adt13::C0(_, _, _, _, h963, _, _, _) => {
            h963
        }
    }, match v2.clone() {
        Adt16::C0(_, _, _, _, h964, _, _, _, _, _) => {
            h964
        }
    }), f92(match v0.clone() {
        Adt13::C0(_, _, _, _, _, h965, _, _) => {
            h965
        }
    }, match v2.clone() {
        Adt16::C0(_, _, _, _, _, h966, _, _, _, _) => {
            h966
        }
    }), f92(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h967, _) => {
            h967
        }
    }, match v2.clone() {
        Adt16::C0(_, _, _, _, _, _, h968, _, _, _) => {
            h968
        }
    }), f92(match v0 {
        Adt13::C0(_, _, _, _, _, _, _, h969) => {
            h969
        }
    }, match v2 {
        Adt16::C0(_, _, _, _, _, _, _, h970, _, _) => {
            h970
        }
    })))
}

pub fn f69(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt15> {
    Ok(Adt15::C0(f93(v0.clone(), v1, v2, v3)?, f93(v0.clone(), v1, v2, {
        let a971 = v3;
        let a972 = 4u64;
        nat_add(a971, a972)?
    })?, f93(v0.clone(), v1, v2, {
        let a973 = v3;
        let a974 = 8u64;
        nat_add(a973, a974)?
    })?, f93(v0.clone(), v1, v2, {
        let a975 = v3;
        let a976 = 12u64;
        nat_add(a975, a976)?
    })?, f93(v0.clone(), v1, v2, {
        let a977 = v3;
        let a978 = 16u64;
        nat_add(a977, a978)?
    })?, f93(v0.clone(), v1, v2, {
        let a979 = v3;
        let a980 = 20u64;
        nat_add(a979, a980)?
    })?, f93(v0.clone(), v1, v2, {
        let a981 = v3;
        let a982 = 24u64;
        nat_add(a981, a982)?
    })?, f93(v0.clone(), v1, v2, {
        let a983 = v3;
        let a984 = 28u64;
        nat_add(a983, a984)?
    })?, f93(v0.clone(), v1, v2, {
        let a985 = v3;
        let a986 = 32u64;
        nat_add(a985, a986)?
    })?, f93(v0.clone(), v1, v2, {
        let a987 = v3;
        let a988 = 36u64;
        nat_add(a987, a988)?
    })?, f93(v0.clone(), v1, v2, {
        let a989 = v3;
        let a990 = 40u64;
        nat_add(a989, a990)?
    })?, f93(v0.clone(), v1, v2, {
        let a991 = v3;
        let a992 = 44u64;
        nat_add(a991, a992)?
    })?, f93(v0.clone(), v1, v2, {
        let a993 = v3;
        let a994 = 48u64;
        nat_add(a993, a994)?
    })?, f93(v0.clone(), v1, v2, {
        let a995 = v3;
        let a996 = 52u64;
        nat_add(a995, a996)?
    })?, f93(v0.clone(), v1, v2, {
        let a997 = v3;
        let a998 = 56u64;
        nat_add(a997, a998)?
    })?, f93(v0, v1, v2, {
        let a999 = v3;
        let a1000 = 60u64;
        nat_add(a999, a1000)?
    })?))
}

pub fn f70(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1001 = v2;
    let a1002 = 0u64;
    if nat_eq(a1001, a1002) {
        Ok(f88(v0, 0u64))
    } else {
        let (_, h1003) = {
            let (h1004, _) = f47(Fn7::F94(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1004
        };
        Ok(h1003)
    }
}

pub fn f71() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f72(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1008 = {
        let a1006 = v0.clone();
        let a1007 = {
            let (h1005, _) = v1.clone();
            h1005
        };
        index_bytes(a1006, a1007)
    };
    match m1008 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1010 = {
                    let (h1009, _) = v1.clone();
                    h1009
                };
                let a1011 = 1u64;
                nat_add(a1010, a1011)?
            }, {
                let a1014 = {
                    let (_, h1012) = v1;
                    h1012
                };
                let a1015 = f95(v0, {
                    let (h1013, _) = v1;
                    h1013
                }, v2)?;
                append_bytes(a1014, a1015)
            })))
        }
    }
}

pub fn f73(v0: Fn12, p1019: Option<Bytes>, p1020: List<Bytes>) -> Option<Bytes> {
    let m1018 = (p1019, p1020);
    {
        let mut m1018 = m1018;
        loop {
            let step = {
                let (v1, v2) = m1018;
                let m1016 = v2;
                match m1016.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1017: Fn12 = v0.clone();
                            c1017.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1018 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1021 = v1;
    match m1021 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1024 = {
                    let a1022 = v3;
                    let a1023 = v0;
                    append_bytes(a1022, a1023)
                };
                let a1025 = v2;
                append_bytes(a1024, a1025)
            })
        }
    }
}

pub fn f75(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1026 = v0.clone();
    match m1026.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1029 = {
                let (h1027, _) = v2;
                h1027
            };
            let a1030 = {
                let (h1028, _) = v1.clone();
                h1028
            };
            if equal(a1029, a1030) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f76(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f96(f97(Fn16::F98, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1035 = {
        let (h1031, _) = f99(Fn17::F100, {
            let a1033 = {
                let a1032 = v0;
                length_list(a1032)
            };
            let a1034 = 1u64;
            nat_add(a1033, a1034)?
        }, v1)?;
        h1031
    };
    match m1035.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f77(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1040 = {
        let a1037 = f23({
            let (h1036, _) = v0;
            h1036
        })?;
        let a1038 = Bytes::lit(&[0x3a]);
        append_bytes(a1037, a1038)
    };
    let a1041 = {
        let (_, h1039) = v0;
        h1039
    };
    Ok(append_bytes(a1040, a1041))
}

pub fn f78(v0: Fn13, p1045: (List<Adt12>, Adt14), p1046: List<Adt1>) -> R<(List<Adt12>, Adt14)> {
    let m1044 = (p1045, p1046);
    {
        let mut m1044 = m1044;
        loop {
            let step = {
                let (v1, v2) = m1044;
                let m1042 = v2;
                match m1042.uncons() {
                    None => {
                        Err::<((List<Adt12>, Adt14), List<Adt1>), R<(List<Adt12>, Adt14)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt12>, Adt14), List<Adt1>), R<(List<Adt12>, Adt14)>>(({
                            let c1043: Fn13 = v0.clone();
                            c1043.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1044 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: (List<Adt12>, Adt14), v1: Adt1) -> R<(List<Adt12>, Adt14)> {
    let v2: (Adt12, Adt14) = f101(v1, {
        let (_, h1047) = v0;
        h1047
    })?;
    Ok((List::cons({
        let (h1048, _) = v2;
        h1048
    }, {
        let (h1049, _) = v0;
        h1049
    }), {
        let (_, h1050) = v2;
        h1050
    }))
}

pub fn f80(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f81(v0: Adt2, v1: Adt14) -> R<(Adt12, Adt14)> {
    let m1051 = v0;
    match m1051 {
        Adt2::C0(v2) => {
            Ok((Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("sort")), f10(Str::lit("u"), f102(v2), Adt12::C5)))), v1))
        }
        Adt2::C1(v3) => {
            let v4: (Adt17, Adt14) = f103(v1, v3)?;
            Ok((f104({
                let (h1052, _) = v4;
                h1052
            }), {
                let (_, h1053) = v4;
                h1053
            }))
        }
        Adt2::C2(v5, v6) => {
            Ok((f105(v5, v6)?, v1))
        }
        Adt2::C3(r1086, r1087, v9) => {
            let v7 = (*r1086).clone();
            let v8 = (*r1087).clone();
            let v10: (Adt12, Adt14) = f81(v7, v1)?;
            let v11: (Adt12, Adt14) = f81(v8, {
                let (_, h1054) = v10;
                h1054
            })?;
            Ok((f106({
                let (h1055, _) = v10;
                h1055
            }, {
                let (h1056, _) = v11;
                h1056
            }, v9), {
                let (_, h1057) = v11;
                h1057
            }))
        }
        Adt2::C4(r1088, r1089) => {
            let v12 = (*r1088).clone();
            let v13 = (*r1089).clone();
            let v14: (Adt12, Adt14) = f81(v12, v1)?;
            let v15: (Adt12, Adt14) = f81(v13, {
                let (_, h1058) = v14;
                h1058
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("pi")), f10(Str::lit("b"), Adt12::C3(std::rc::Rc::new({
                let (h1059, _) = v14;
                h1059
            })), f10(Str::lit("v"), {
                let (h1060, _) = v15;
                h1060
            }, Adt12::C5))))), {
                let (_, h1061) = v15;
                h1061
            }))
        }
        Adt2::C5(r1090, r1091) => {
            let v16 = (*r1090).clone();
            let v17 = (*r1091).clone();
            let v18: (Adt12, Adt14) = f81(v16, v1)?;
            let v19: (Adt12, Adt14) = f81(v17, {
                let (_, h1062) = v18;
                h1062
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("lam")), f10(Str::lit("b"), Adt12::C3(std::rc::Rc::new({
                let (h1063, _) = v18;
                h1063
            })), f10(Str::lit("v"), {
                let (h1064, _) = v19;
                h1064
            }, Adt12::C5))))), {
                let (_, h1065) = v19;
                h1065
            }))
        }
        Adt2::C6(v20, v21, v22, r1092, r1093, r1094) => {
            let v23 = (*r1092).clone();
            let v24 = (*r1093).clone();
            let v25 = (*r1094).clone();
            let v26: (Adt12, Adt14) = f81(v23, v1)?;
            let v27: (Adt12, Adt14) = f81(v24, {
                let (_, h1066) = v26;
                h1066
            })?;
            let v28: (u64, Adt14) = f107({
                let (_, h1067) = v27;
                h1067
            }, v20)?;
            let v29: (Adt12, Adt14) = f81(v25, {
                let (_, h1068) = v28;
                h1068
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("let")), f10(Str::lit("b"), f108({
                let (h1069, _) = v28;
                h1069
            }, v21, v22, {
                let (h1070, _) = v26;
                h1070
            }), f10(Str::lit("x"), {
                let (h1071, _) = v27;
                h1071
            }, f10(Str::lit("v"), {
                let (h1072, _) = v29;
                h1072
            }, Adt12::C5)))))), {
                let (_, h1073) = v29;
                h1073
            }))
        }
        Adt2::C7(v30, r1095) => {
            let v31 = (*r1095).clone();
            let v32: (Adt12, Adt14) = f81(v31, v1)?;
            Ok((Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("nat")), f10(Str::lit("d"), Adt12::C2(v30), f10(Str::lit("t"), {
                let (h1074, _) = v32;
                h1074
            }, Adt12::C5))))), {
                let (_, h1075) = v32;
                h1075
            }))
        }
        Adt2::C8 => {
            Ok((Adt12::C5, v1))
        }
        Adt2::C9(r1096, r1097) => {
            let v33 = (*r1096).clone();
            let v34 = (*r1097).clone();
            let v35: (Adt12, Adt14) = f81(v33, v1)?;
            let v36: (Adt12, Adt14) = f81(v34, {
                let (_, h1076) = v35;
                h1076
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h1077, _) = v35;
                h1077
            }), std::rc::Rc::new({
                let (h1078, _) = v36;
                h1078
            })), {
                let (_, h1079) = v36;
                h1079
            }))
        }
        Adt2::C10(v37, v38, v39, r1098, r1099) => {
            let v40 = (*r1098).clone();
            let v41 = (*r1099).clone();
            let v42: (Adt12, Adt14) = f81(v40, v1)?;
            let v43: (u64, Adt14) = f107({
                let (_, h1080) = v42;
                h1080
            }, v37)?;
            let v44: (Adt12, Adt14) = f81(v41, {
                let (_, h1081) = v43;
                h1081
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f108({
                let (h1082, _) = v43;
                h1082
            }, v38, v39, {
                let (h1083, _) = v42;
                h1083
            })), std::rc::Rc::new({
                let (h1084, _) = v44;
                h1084
            })), {
                let (_, h1085) = v44;
                h1085
            }))
        }
    }
}

pub fn f82(v0: Adt7, v1: Adt14) -> R<(Adt12, Adt14)> {
    let v2: (Adt12, Adt14) = f81(match v0.clone() {
        Adt7::C0(_, _, _, h1100, _, _) => {
            h1100
        }
    }, v1)?;
    let v3: (Adt12, Adt14) = f109(match v0.clone() {
        Adt7::C0(_, _, _, _, h1101, _) => {
            h1101
        }
    }, {
        let (_, h1102) = v2;
        h1102
    })?;
    Ok((f110(f10(Str::lit("component"), Adt12::C2(match v0.clone() {
        Adt7::C0(h1103, _, _, _, _, _) => {
            h1103
        }
    }), f10(Str::lit("lean_name"), Adt12::C2(match v0.clone() {
        Adt7::C0(_, h1104, _, _, _, _) => {
            h1104
        }
    }), f10(Str::lit("kind"), f111(match v0.clone() {
        Adt7::C0(_, _, h1105, _, _, _) => {
            h1105
        }
    }), f10(Str::lit("params"), f112({
        let (h1106, _) = v2;
        h1106
    }), f10(Str::lit("body"), {
        let (h1107, _) = v3;
        h1107
    }, f10(Str::lit("policy"), f113(match v0 {
        Adt7::C0(_, _, _, _, _, h1108) => {
            h1108
        }
    }), f114()))))))), {
        let (_, h1109) = v3;
        h1109
    }))
}

pub fn f83(v0: Fn14, p1113: List<Adt12>, p1114: List<Bytes>) -> List<Adt12> {
    let m1112 = (p1113, p1114);
    {
        let mut m1112 = m1112;
        loop {
            let step = {
                let (v1, v2) = m1112;
                let m1110 = v2;
                match m1110.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Bytes>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Bytes>), List<Adt12>>(({
                            let c1111: Fn14 = v0.clone();
                            c1111.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: List<Adt12>, v1: Bytes) -> List<Adt12> {
    List::cons(f60(v1), v0)
}

pub fn f85(v0: List<Bytes>) -> List<Bytes> {
    f115(Fn18::F116, List::<Bytes>::nil(), v0)
}

pub fn f86(v0: Adt0) -> Adt12 {
    f117(Fn19::F118(v0.clone()), f18(), match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h1115, _, _, _, _, _) => {
            h1115
        }
    })
}

pub fn f87(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f35({
        let (_, h1116) = {
            let (h1117, _) = f119(Fn20::F120, {
                let a1119 = {
                    let a1118 = v0.clone();
                    length_list(a1118)
                };
                let a1120 = 1u64;
                nat_add(a1119, a1120)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h1117
        };
        h1116
    }))
}

pub fn f88(v0: Bytes, v1: u64) -> Bytes {
    let m1124 = {
        let a1121 = v0;
        let a1122 = v1;
        let a1123 = 1u64;
        slice_bytes(a1121, a1122, a1123)
    };
    match m1124 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f89() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f90(v0: Fn15, p1128: u64, p1129: Adt16) -> R<Adt16> {
    let m1127 = (p1128, p1129);
    {
        let mut m1127 = m1127;
        loop {
            let step = {
                let (v1, v2) = m1127;
                let m1125 = v1;
                if m1125 == 0 {
                    Err::<(u64, Adt16), R<Adt16>>(Ok(v2))
                } else {
                    let v3 = m1125 - 1;
                    Ok::<(u64, Adt16), R<Adt16>>((v3, {
                        let c1126: Fn15 = v0.clone();
                        c1126.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: Adt16) -> R<Adt16> {
    f121(v0)
}

pub fn f92(v0: u32, v1: u32) -> u32 {
    let m1132 = {
        let a1130 = f122(v0);
        let a1131 = f122(v1);
        fixed_u64::checked_add(a1130, a1131)
    };
    match m1132 {
        None => {
            0u32
        }
        Some(v2) => {
            f123(v2)
        }
    }
}

pub fn f93(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1135 = {
        let a1133 = v3;
        let a1134 = 8u64;
        nat_add(a1133, a1134)?
    };
    let a1136 = v2;
    if nat_eq(a1135, a1136) {
        Ok(f124({
            let a1139 = {
                let a1137 = v1;
                let a1138 = 8u64;
                nat_mul(a1137, a1138)?
            };
            let a1140 = 4294967296u64;
            let a1141 = 0u64;
            nat_quot(a1139, a1140, a1141)
        }))
    } else {
        let a1144 = {
            let a1142 = v3;
            let a1143 = 4u64;
            nat_add(a1142, a1143)?
        };
        let a1145 = v2;
        if nat_eq(a1144, a1145) {
            Ok(f124({
                let a1148 = {
                    let a1146 = v1;
                    let a1147 = 8u64;
                    nat_mul(a1146, a1147)?
                };
                let a1149 = 4294967296u64;
                let a1150 = 0u64;
                nat_rem(a1148, a1149, a1150)
            }))
        } else {
            Ok(f125(f126(v0.clone(), v1, v3), f126(v0.clone(), v1, {
                let a1151 = v3;
                let a1152 = 1u64;
                nat_add(a1151, a1152)?
            }), f126(v0.clone(), v1, {
                let a1153 = v3;
                let a1154 = 2u64;
                nat_add(a1153, a1154)?
            }), f126(v0, v1, {
                let a1155 = v3;
                let a1156 = 3u64;
                nat_add(a1155, a1156)?
            })))
        }
    }
}

pub fn f94(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f127(v1, v0, v2)
}

pub fn f95(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1201 = {
            let a1177 = {
                let a1165 = {
                    let a1159 = {
                        let a1157 = v3;
                        let a1158 = 128u8;
                        fixed_u8::bit_and(a1157, a1158)
                    };
                    let a1160 = 128u8;
                    if equal(a1159, a1160) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1166 = {
                    let a1163 = {
                        let a1161 = v3;
                        let a1162 = 64u8;
                        fixed_u8::bit_and(a1161, a1162)
                    };
                    let a1164 = 64u8;
                    if equal(a1163, a1164) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1165, a1166)?
            };
            let a1178 = {
                let a1175 = {
                    let a1169 = {
                        let a1167 = v3;
                        let a1168 = 32u8;
                        fixed_u8::bit_and(a1167, a1168)
                    };
                    let a1170 = 32u8;
                    if equal(a1169, a1170) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1176 = {
                    let a1173 = {
                        let a1171 = v3;
                        let a1172 = 16u8;
                        fixed_u8::bit_and(a1171, a1172)
                    };
                    let a1174 = 16u8;
                    if equal(a1173, a1174) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1175, a1176)?
            };
            nat_add(a1177, a1178)?
        };
        let a1202 = {
            let a1199 = {
                let a1187 = {
                    let a1181 = {
                        let a1179 = v3;
                        let a1180 = 8u8;
                        fixed_u8::bit_and(a1179, a1180)
                    };
                    let a1182 = 8u8;
                    if equal(a1181, a1182) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1188 = {
                    let a1185 = {
                        let a1183 = v3;
                        let a1184 = 4u8;
                        fixed_u8::bit_and(a1183, a1184)
                    };
                    let a1186 = 4u8;
                    if equal(a1185, a1186) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1187, a1188)?
            };
            let a1200 = {
                let a1197 = {
                    let a1191 = {
                        let a1189 = v3;
                        let a1190 = 2u8;
                        fixed_u8::bit_and(a1189, a1190)
                    };
                    let a1192 = 2u8;
                    if equal(a1191, a1192) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1198 = {
                    let a1195 = {
                        let a1193 = v3;
                        let a1194 = 1u8;
                        fixed_u8::bit_and(a1193, a1194)
                    };
                    let a1196 = 1u8;
                    if equal(a1195, a1196) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1197, a1198)?
            };
            nat_add(a1199, a1200)?
        };
        nat_add(a1201, a1202)?
    };
    let a1203 = v4;
    let a1204 = 34u64;
    if nat_eq(a1203, a1204) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1205 = v4;
        let a1206 = 92u64;
        if nat_eq(a1205, a1206) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1207 = v4;
            let a1208 = 8u64;
            if nat_eq(a1207, a1208) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1209 = v4;
                let a1210 = 12u64;
                if nat_eq(a1209, a1210) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1211 = v4;
                    let a1212 = 10u64;
                    if nat_eq(a1211, a1212) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1213 = v4;
                        let a1214 = 13u64;
                        if nat_eq(a1213, a1214) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1215 = v4;
                            let a1216 = 9u64;
                            if nat_eq(a1215, a1216) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1217 = v4;
                                let a1218 = 32u64;
                                if nat_lt(a1217, a1218) {
                                    let a1219 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1220 = f128(v2)?;
                                    Ok(append_bytes(a1219, a1220))
                                } else {
                                    Ok(f129(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f96(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f130(Fn21::F131, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f97(v0: Fn16, p1224: List<List<(Bytes, Bytes)>>, p1225: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m1223 = (p1224, p1225);
    {
        let mut m1223 = m1223;
        loop {
            let step = {
                let (v1, v2) = m1223;
                let m1221 = v2;
                match m1221.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c1222: Fn16 = v0.clone();
                            c1222.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f99(v0: Fn17, p1230: u64, p1231: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m1229 = (p1230, p1231);
    {
        let mut m1229 = m1229;
        loop {
            let step = {
                let (v1, v2) = m1229;
                let m1226 = v1;
                if m1226 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1226 - 1;
                    let m1228 = {
                        let c1227: Fn17 = v0.clone();
                        c1227.apply(v2.clone())?
                    };
                    match m1228 {
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
                Ok(next) => { m1229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f132(v0)
}

pub fn f101(v0: Adt1, v1: Adt14) -> R<(Adt12, Adt14)> {
    let m1232 = v0;
    match m1232 {
        Adt1::C0(v2, v3) => {
            Ok((f9(f10(Str::lit("k"), f11(Str::lit("word")), f10(Str::lit("entry"), f60(v2), f10(Str::lit("form"), f60(v3), f18())))), v1))
        }
        Adt1::C1(v4) => {
            let v5: (Adt12, Adt14) = f81(v4, v1)?;
            Ok((f9(f10(Str::lit("k"), f11(Str::lit("math")), f10(Str::lit("t"), {
                let (h1233, _) = v5;
                h1233
            }, f18()))), {
                let (_, h1234) = v5;
                h1234
            }))
        }
        Adt1::C2(v6) => {
            Ok((f9(f10(Str::lit("k"), f11(Str::lit("punct")), f10(Str::lit("entry"), f60(v6), f18()))), v1))
        }
    }
}

pub fn f102(v0: Adt3) -> Adt12 {
    let m1235 = v0;
    match m1235 {
        Adt3::C0(v1) => {
            Adt12::C4(std::rc::Rc::new(f10(Str::lit("num"), f133(v1), Adt12::C5)))
        }
        Adt3::C1(v2) => {
            Adt12::C4(std::rc::Rc::new(f10(Str::lit("var"), Adt12::C2(v2), Adt12::C5)))
        }
        Adt3::C2(r1236) => {
            let v3 = (*r1236).clone();
            Adt12::C4(std::rc::Rc::new(f10(Str::lit("succ"), f102(v3), Adt12::C5)))
        }
        Adt3::C3(r1237) => {
            let v4 = (*r1237).clone();
            Adt12::C4(std::rc::Rc::new(f10(Str::lit("max"), Adt12::C3(std::rc::Rc::new(f102(v4))), Adt12::C5)))
        }
        Adt3::C4(r1238, r1239) => {
            let v5 = (*r1238).clone();
            let v6 = (*r1239).clone();
            Adt12::C4(std::rc::Rc::new(f10(Str::lit("imax"), Adt12::C3(std::rc::Rc::new(Adt12::C6(std::rc::Rc::new(f102(v5)), std::rc::Rc::new(Adt12::C6(std::rc::Rc::new(f102(v6)), std::rc::Rc::new(Adt12::C5)))))), Adt12::C5)))
        }
        Adt3::C5 => {
            Adt12::C5
        }
        Adt3::C6(r1240, r1241) => {
            let v7 = (*r1240).clone();
            let v8 = (*r1241).clone();
            Adt12::C6(std::rc::Rc::new(f102(v7)), std::rc::Rc::new(f102(v8)))
        }
    }
}

pub fn f103(v0: Adt14, v1: u64) -> R<(Adt17, Adt14)> {
    let m1243 = f134(match v0.clone() {
        Adt14::C0(h1242, _, _) => {
            h1242
        }
    }, v1);
    match m1243 {
        None => {
            if match v0.clone() {
                Adt14::C0(_, _, h1244) => {
                    h1244
                }
            } {
                Ok((Adt17::C1(v1), v0))
            } else {
                let v3: (u64, Adt14) = f135(v0, v1)?;
                Ok((Adt17::C0({
                    let (h1245, _) = v3;
                    h1245
                }), {
                    let (_, h1246) = v3;
                    h1246
                }))
            }
        }
        Some(v2) => {
            Ok((Adt17::C0(v2), v0))
        }
    }
}

pub fn f104(v0: Adt17) -> Adt12 {
    let m1247 = v0;
    match m1247 {
        Adt17::C0(v1) => {
            Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("local")), f10(Str::lit("id"), f133(v1), Adt12::C5))))
        }
        Adt17::C1(v2) => {
            Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("local")), f10(Str::lit("free"), f133(v2), Adt12::C5))))
        }
    }
}

pub fn f105(v0: Adt4, v1: Adt3) -> R<Adt12> {
    let v2: Adt12 = if f136(v1.clone()) {
        Adt12::C5
    } else {
        f10(Str::lit("u"), Adt12::C3(std::rc::Rc::new(f102(v1))), Adt12::C5)
    };
    Ok(Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("global")), f10(Str::lit("g"), f137(v0)?, v2)))))
}

pub fn f106(v0: Adt12, v1: Adt12, v2: List<u64>) -> Adt12 {
    let v5: Adt12 = {
        let m1248 = v2.clone();
        match m1248.uncons() {
            None => {
                Adt12::C5
            }
            Some((_, _)) => {
                f10(Str::lit("i"), Adt12::C3(std::rc::Rc::new(f138(v2))), Adt12::C5)
            }
        }
    };
    Adt12::C4(std::rc::Rc::new(f10(Str::lit("k"), f11(Str::lit("app")), f10(Str::lit("f"), v0, f10(Str::lit("a"), Adt12::C3(std::rc::Rc::new(v1)), v5)))))
}

pub fn f107(v0: Adt14, v1: u64) -> R<(u64, Adt14)> {
    let m1250 = f134(match v0.clone() {
        Adt14::C0(h1249, _, _) => {
            h1249
        }
    }, v1);
    match m1250 {
        None => {
            f135(v0, v1)
        }
        Some(v2) => {
            Ok((v2, v0))
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: Bytes, v3: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(f10(Str::lit("id"), f133(v0), f10(Str::lit("m"), f139(v1), f10(Str::lit("s"), Adt12::C2(v2), f10(Str::lit("t"), v3, Adt12::C5))))))
}

pub fn f109(v0: Adt9, v1: Adt14) -> R<(Adt12, Adt14)> {
    let m1251 = v0;
    match m1251 {
        Adt9::C0(v2, v3, v4) => {
            let v5: (Adt12, Adt14) = f81(v3, v1)?;
            let v6: (Adt12, Adt14) = f81(v4, {
                let (_, h1252) = v5;
                h1252
            })?;
            Ok((f140(Str::lit("definition"), f10(Str::lit("entry"), Adt12::C2(v2), f10(Str::lit("t"), {
                let (h1253, _) = v5;
                h1253
            }, f10(Str::lit("v"), {
                let (h1254, _) = v6;
                h1254
            }, f114())))), {
                let (_, h1255) = v6;
                h1255
            }))
        }
        Adt9::C1(v7, v8) => {
            let v9: (Adt12, Adt14) = f81(v7, v1)?;
            let v10: (Adt12, Adt14) = f141(v8, {
                let (_, h1256) = v9;
                h1256
            })?;
            Ok((f140(Str::lit("theorem"), f10(Str::lit("s"), {
                let (h1257, _) = v9;
                h1257
            }, f10(Str::lit("p"), {
                let (h1258, _) = v10;
                h1258
            }, f114()))), {
                let (_, h1259) = v10;
                h1259
            }))
        }
    }
}

pub fn f110(v0: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(v0))
}

pub fn f111(v0: Adt8) -> Adt12 {
    let m1260 = v0;
    match m1260 {
        Adt8::C0 => {
            f11(Str::lit("typedefinition"))
        }
        Adt8::C1 => {
            f11(Str::lit("termdefinition"))
        }
        Adt8::C2 => {
            f11(Str::lit("predicatedefinition"))
        }
        Adt8::C3 => {
            f11(Str::lit("theorem"))
        }
        Adt8::C4 => {
            f11(Str::lit("lemma"))
        }
        Adt8::C5 => {
            f11(Str::lit("corollary"))
        }
    }
}

pub fn f112(v0: Adt12) -> Adt12 {
    Adt12::C3(std::rc::Rc::new(v0))
}

pub fn f113(v0: Adt11) -> Adt12 {
    let m1261 = v0;
    match m1261 {
        Adt11::C0 => {
            f110(f10(Str::lit("kind"), f11(Str::lit("none")), f10(Str::lit("axioms"), f112(f114()), f114())))
        }
        Adt11::C1(v1) => {
            f110(f10(Str::lit("kind"), f11(Str::lit("allow")), f10(Str::lit("axioms"), f112(f142(v1)), f114())))
        }
        Adt11::C2(v2) => {
            f110(f10(Str::lit("kind"), f11(Str::lit("exact")), f10(Str::lit("axioms"), f112(f142(v2)), f114())))
        }
    }
}

pub fn f114() -> Adt12 {
    Adt12::C5
}

pub fn f115(v0: Fn18, p1265: List<Bytes>, p1266: List<Bytes>) -> List<Bytes> {
    let m1264 = (p1265, p1266);
    {
        let mut m1264 = m1264;
        loop {
            let step = {
                let (v1, v2) = m1264;
                let m1262 = v2;
                match m1262.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1263: Fn18 = v0.clone();
                            c1263.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f117(v0: Fn19, p1270: Adt12, p1271: List<(Bytes, Bytes)>) -> Adt12 {
    let m1269 = (p1270, p1271);
    {
        let mut m1269 = m1269;
        loop {
            let step = {
                let (v1, v2) = m1269;
                let m1267 = v2;
                match m1267.uncons() {
                    None => {
                        Err::<(Adt12, List<(Bytes, Bytes)>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<(Bytes, Bytes)>), Adt12>(({
                            let c1268: Fn19 = v0.clone();
                            c1268.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Adt0, v1: Adt12, v2: (Bytes, Bytes)) -> Adt12 {
    let a1274 = {
        let (_, h1272) = v2.clone();
        h1272
    };
    let a1275 = f143(v0, {
        let (h1273, _) = v2.clone();
        h1273
    });
    if equal(a1274, a1275) {
        v1
    } else {
        Adt12::C7({
            let (h1276, _) = v2;
            h1276
        }, std::rc::Rc::new(f60({
            let (_, h1277) = v2;
            h1277
        })), std::rc::Rc::new(v1))
    }
}

pub fn f119(v0: Fn20, p1282: u64, p1283: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m1281 = (p1282, p1283);
    {
        let mut m1281 = m1281;
        loop {
            let step = {
                let (v1, v2) = m1281;
                let m1278 = v1;
                if m1278 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1278 - 1;
                    let m1280 = {
                        let c1279: Fn20 = v0.clone();
                        c1279.apply(v2.clone())?
                    };
                    match m1280 {
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
                Ok(next) => { m1281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f144(v0)
}

pub fn f121(v0: Adt16) -> R<Adt16> {
    let v1: u32 = f145(match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, h1284) => {
            h1284
        }
    });
    let v2: u32 = f92(match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, h1285, _, _) => {
            h1285
        }
    }, f92(f146(match v0.clone() {
        Adt16::C0(_, _, _, _, h1286, _, _, _, _, _) => {
            h1286
        }
    }), f92(f147(match v0.clone() {
        Adt16::C0(_, _, _, _, h1287, _, _, _, _, _) => {
            h1287
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, h1288, _, _, _, _) => {
            h1288
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, h1289, _, _, _) => {
            h1289
        }
    }), f92(v1, {
        let m1292 = match v0.clone() {
            Adt16::C0(_, _, _, _, _, _, _, _, h1291, _) => {
                h1291
            }
        };
        match m1292 {
            Adt15::C0(h1290, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1290
            }
        }
    }))));
    let v3: u32 = f92(f148(match v0.clone() {
        Adt16::C0(h1293, _, _, _, _, _, _, _, _, _) => {
            h1293
        }
    }), f149(match v0.clone() {
        Adt16::C0(h1294, _, _, _, _, _, _, _, _, _) => {
            h1294
        }
    }, match v0.clone() {
        Adt16::C0(_, h1295, _, _, _, _, _, _, _, _) => {
            h1295
        }
    }, match v0.clone() {
        Adt16::C0(_, _, h1296, _, _, _, _, _, _, _) => {
            h1296
        }
    }));
    Ok(Adt16::C0(f92(v2, v3), match v0.clone() {
        Adt16::C0(h1297, _, _, _, _, _, _, _, _, _) => {
            h1297
        }
    }, match v0.clone() {
        Adt16::C0(_, h1298, _, _, _, _, _, _, _, _) => {
            h1298
        }
    }, match v0.clone() {
        Adt16::C0(_, _, h1299, _, _, _, _, _, _, _) => {
            h1299
        }
    }, f92(match v0.clone() {
        Adt16::C0(_, _, _, h1300, _, _, _, _, _, _) => {
            h1300
        }
    }, v2), match v0.clone() {
        Adt16::C0(_, _, _, _, h1301, _, _, _, _, _) => {
            h1301
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, h1302, _, _, _, _) => {
            h1302
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, h1303, _, _, _) => {
            h1303
        }
    }, f150(match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, h1304, _) => {
            h1304
        }
    }), {
        let a1306 = match v0 {
            Adt16::C0(_, _, _, _, _, _, _, _, _, h1305) => {
                h1305
            }
        };
        let a1307 = 1u64;
        nat_add(a1306, a1307)?
    }))
}

pub fn f122(v0: u32) -> u64 {
    let m1309 = {
        let a1308 = v0;
        fixed_u64::convert(i128::from(a1308))
    };
    match m1309 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f123(v0: u64) -> u32 {
    let m1313 = {
        let a1312 = {
            let a1310 = v0;
            let a1311 = 4294967295u64;
            fixed_u64::bit_and(a1310, a1311)
        };
        fixed_u32::convert(i128::from(a1312))
    };
    match m1313 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f124(v0: u64) -> u32 {
    let (_, h1314) = {
        let (_, h1315) = f151(Fn22::F152, 32u64, (v0, (1u32, 0u32)));
        h1315
    };
    h1314
}

pub fn f125(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1320 = f153(v0, 24u32);
    let a1321 = {
        let a1318 = f153(v1, 16u32);
        let a1319 = {
            let a1316 = f153(v2, 8u32);
            let a1317 = v3;
            fixed_u32::bit_or(a1316, a1317)
        };
        fixed_u32::bit_or(a1318, a1319)
    };
    fixed_u32::bit_or(a1320, a1321)
}

pub fn f126(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1322 = v2;
    let a1323 = v1;
    if nat_lt(a1322, a1323) {
        let m1326 = {
            let a1324 = v0;
            let a1325 = v2;
            index_bytes(a1324, a1325)
        };
        match m1326 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1328 = {
                    let a1327 = v3;
                    fixed_u32::convert(i128::from(a1327))
                };
                match m1328 {
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
        let a1329 = v2;
        let a1330 = v1;
        if nat_eq(a1329, a1330) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f127(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1331, _) = v2;
        h1331
    };
    let a1332 = v3;
    let a1333 = 0u64;
    if nat_eq(a1332, a1333) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1334 = v3;
            let a1335 = v1;
            let a1336 = 0u64;
            nat_quot(a1334, a1335, a1336)
        }, {
            let a1341 = f88(v0, {
                let a1337 = v3;
                let a1338 = v1;
                let a1339 = 0u64;
                nat_rem(a1337, a1338, a1339)
            });
            let a1342 = {
                let (_, h1340) = v2;
                h1340
            };
            append_bytes(a1341, a1342)
        }))
    }
}

pub fn f128(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1387 = {
            let a1363 = {
                let a1351 = {
                    let a1345 = {
                        let a1343 = v1;
                        let a1344 = 128u8;
                        fixed_u8::bit_and(a1343, a1344)
                    };
                    let a1346 = 128u8;
                    if equal(a1345, a1346) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1352 = {
                    let a1349 = {
                        let a1347 = v1;
                        let a1348 = 64u8;
                        fixed_u8::bit_and(a1347, a1348)
                    };
                    let a1350 = 64u8;
                    if equal(a1349, a1350) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1351, a1352)?
            };
            let a1364 = {
                let a1361 = {
                    let a1355 = {
                        let a1353 = v1;
                        let a1354 = 32u8;
                        fixed_u8::bit_and(a1353, a1354)
                    };
                    let a1356 = 32u8;
                    if equal(a1355, a1356) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1362 = {
                    let a1359 = {
                        let a1357 = v1;
                        let a1358 = 16u8;
                        fixed_u8::bit_and(a1357, a1358)
                    };
                    let a1360 = 16u8;
                    if equal(a1359, a1360) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1361, a1362)?
            };
            nat_add(a1363, a1364)?
        };
        let a1388 = {
            let a1385 = {
                let a1373 = {
                    let a1367 = {
                        let a1365 = v1;
                        let a1366 = 8u8;
                        fixed_u8::bit_and(a1365, a1366)
                    };
                    let a1368 = 8u8;
                    if equal(a1367, a1368) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1374 = {
                    let a1371 = {
                        let a1369 = v1;
                        let a1370 = 4u8;
                        fixed_u8::bit_and(a1369, a1370)
                    };
                    let a1372 = 4u8;
                    if equal(a1371, a1372) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1373, a1374)?
            };
            let a1386 = {
                let a1383 = {
                    let a1377 = {
                        let a1375 = v1;
                        let a1376 = 2u8;
                        fixed_u8::bit_and(a1375, a1376)
                    };
                    let a1378 = 2u8;
                    if equal(a1377, a1378) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1384 = {
                    let a1381 = {
                        let a1379 = v1;
                        let a1380 = 1u8;
                        fixed_u8::bit_and(a1379, a1380)
                    };
                    let a1382 = 1u8;
                    if equal(a1381, a1382) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1383, a1384)?
            };
            nat_add(a1385, a1386)?
        };
        nat_add(a1387, a1388)?
    };
    let a1395 = f88(f154(), {
        let a1389 = v2;
        let a1390 = 16u64;
        let a1391 = 0u64;
        nat_quot(a1389, a1390, a1391)
    });
    let a1396 = f88(f154(), {
        let a1392 = v2;
        let a1393 = 16u64;
        let a1394 = 0u64;
        nat_rem(a1392, a1393, a1394)
    });
    Ok(append_bytes(a1395, a1396))
}

pub fn f129(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m1400 = {
        let a1397 = v0;
        let a1398 = v1;
        let a1399 = v2;
        slice_bytes(a1397, a1398, a1399)
    };
    match m1400 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f130(v0: Fn21, p1404: List<List<(Bytes, Bytes)>>, p1405: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m1403 = (p1404, p1405);
    {
        let mut m1403 = m1403;
        loop {
            let step = {
                let (v1, v2) = m1403;
                let m1401 = v2;
                match m1401.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c1402: Fn21 = v0.clone();
                            c1402.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f132(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a1407 = {
        let a1406 = v0.clone();
        length_list(a1406)
    };
    let a1408 = 1u64;
    if nat_le(a1407, a1408) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f155(v0)?))
    }
}

pub fn f133(v0: u64) -> Adt12 {
    Adt12::C1(false, f156(v0))
}

pub fn f134(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f157(Fn23::F158(v1), None::<u64>, v0)
}

pub fn f135(v0: Adt14, v1: u64) -> R<(u64, Adt14)> {
    Ok((match v0.clone() {
        Adt14::C0(_, h1409, _) => {
            h1409
        }
    }, Adt14::C0(List::cons((v1, match v0.clone() {
        Adt14::C0(_, h1410, _) => {
            h1410
        }
    }), match v0.clone() {
        Adt14::C0(h1411, _, _) => {
            h1411
        }
    }), {
        let a1413 = match v0.clone() {
            Adt14::C0(_, h1412, _) => {
                h1412
            }
        };
        let a1414 = 1u64;
        nat_add(a1413, a1414)?
    }, match v0 {
        Adt14::C0(_, _, h1415) => {
            h1415
        }
    })))
}

pub fn f136(v0: Adt3) -> bool {
    let m1416 = v0;
    match m1416 {
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

pub fn f137(v0: Adt4) -> R<Adt12> {
    let m1417 = v0;
    match m1417 {
        Adt4::C0(v1) => {
            Ok(Adt12::C4(std::rc::Rc::new(f10(Str::lit("kind"), f11(Str::lit("core")), f10(Str::lit("name"), Adt12::C2(f159(v1)), Adt12::C5)))))
        }
        Adt4::C1(_, v3, v4, v5, v6) => {
            Ok(Adt12::C4(std::rc::Rc::new(f10(Str::lit("kind"), f11(Str::lit("external")), f10(Str::lit("entry"), Adt12::C2(v3), f10(Str::lit("lean_module"), Adt12::C2(v4), f10(Str::lit("lean_name"), Adt12::C2(v5), f10(Str::lit("signature_sha256"), Adt12::C2(f160(v6)?), Adt12::C5))))))))
        }
        Adt4::C2(v7, v8, v9) => {
            Ok(Adt12::C4(std::rc::Rc::new(f10(Str::lit("kind"), f11(Str::lit("document")), f10(Str::lit("module"), Adt12::C2(v7), f10(Str::lit("component"), Adt12::C2(v8), f10(Str::lit("lean_name"), Adt12::C2(v9), Adt12::C5)))))))
        }
        Adt4::C3(_, v11, v12) => {
            Ok(Adt12::C4(std::rc::Rc::new(f10(Str::lit("kind"), f11(Str::lit("defined")), f10(Str::lit("entry"), Adt12::C2(v11), f10(Str::lit("signature_sha256"), Adt12::C2(f160(v12)?), Adt12::C5))))))
        }
    }
}

pub fn f138(v0: List<u64>) -> Adt12 {
    f161(Fn24::F162, Adt12::C5, f163(v0))
}

pub fn f139(v0: u64) -> Adt12 {
    Adt12::C2(f164(v0))
}

pub fn f140(v0: Str, v1: Adt12) -> Adt12 {
    f110(f10(Str::lit("k"), f11(v0), v1))
}

pub fn f141(v0: Adt10, v1: Adt14) -> R<(Adt12, Adt14)> {
    let m1418 = v0;
    match m1418 {
        Adt10::C0(r1481) => {
            let v2 = (*r1481).clone();
            let v3: (Adt12, Adt14) = f141(v2, v1)?;
            Ok((f140(Str::lit("seq"), f10(Str::lit("s"), f112({
                let (h1419, _) = v3;
                h1419
            }), f114())), {
                let (_, h1420) = v3;
                h1420
            }))
        }
        Adt10::C1(v4) => {
            let v5: (Adt12, Adt14) = f165(v4, v1)?;
            Ok((f140(Str::lit("intro"), f10(Str::lit("l"), f112({
                let (h1421, _) = v5;
                h1421
            }), f114())), {
                let (_, h1422) = v5;
                h1422
            }))
        }
        Adt10::C2(v6) => {
            let v7: (Adt12, Adt14) = f166(Str::lit("t"), v6, v1, f114())?;
            Ok((f140(Str::lit("exact"), {
                let (h1423, _) = v7;
                h1423
            }), {
                let (_, h1424) = v7;
                h1424
            }))
        }
        Adt10::C3(v8) => {
            let v9: (Adt12, Adt14) = f166(Str::lit("t"), v8, v1, f114())?;
            Ok((f140(Str::lit("apply-one"), {
                let (h1425, _) = v9;
                h1425
            }), {
                let (_, h1426) = v9;
                h1426
            }))
        }
        Adt10::C4(v10, r1482) => {
            let v11 = (*r1482).clone();
            let v12: (Adt12, Adt14) = f81(v10, v1)?;
            let v13: (Adt12, Adt14) = f141(v11, {
                let (_, h1427) = v12;
                h1427
            })?;
            Ok((f140(Str::lit("apply"), f10(Str::lit("f"), {
                let (h1428, _) = v12;
                h1428
            }, f10(Str::lit("p"), f112({
                let (h1429, _) = v13;
                h1429
            }), f114()))), {
                let (_, h1430) = v13;
                h1430
            }))
        }
        Adt10::C5 => {
            Ok((f140(Str::lit("rfl"), f114()), v1))
        }
        Adt10::C6(v14) => {
            let v15: (Adt12, Adt14) = f166(Str::lit("t"), v14, v1, f114())?;
            Ok((f140(Str::lit("witness"), {
                let (h1431, _) = v15;
                h1431
            }), {
                let (_, h1432) = v15;
                h1432
            }))
        }
        Adt10::C7 => {
            Ok((f140(Str::lit("left"), f114()), v1))
        }
        Adt10::C8 => {
            Ok((f140(Str::lit("right"), f114()), v1))
        }
        Adt10::C9(v16, v17, r1483) => {
            let v18 = (*r1483).clone();
            let v19: (Adt12, Adt14) = f81(v17, v1)?;
            let v20: (u64, Adt14) = f107({
                let (_, h1433) = v19;
                h1433
            }, v16)?;
            let v21: (Adt12, Adt14) = f141(v18, {
                let (_, h1434) = v20;
                h1434
            })?;
            Ok((f140(Str::lit("have"), f10(Str::lit("l"), f133({
                let (h1435, _) = v20;
                h1435
            }), f10(Str::lit("p"), {
                let (h1436, _) = v19;
                h1436
            }, f10(Str::lit("q"), {
                let (h1437, _) = v21;
                h1437
            }, f114())))), {
                let (_, h1438) = v21;
                h1438
            }))
        }
        Adt10::C10(v22, r1484) => {
            let v23 = (*r1484).clone();
            let v24: (Adt12, Adt14) = f167(v22, v1)?;
            let v25: (Adt12, Adt14) = f141(v23, {
                let (_, h1439) = v24;
                h1439
            })?;
            Ok((f140(Str::lit("rw"), f10(Str::lit("at"), {
                let (h1440, _) = v24;
                h1440
            }, f10(Str::lit("r"), f112({
                let (h1441, _) = v25;
                h1441
            }), f114()))), {
                let (_, h1442) = v25;
                h1442
            }))
        }
        Adt10::C11(v26, v27) => {
            let v28: (Adt12, Adt14) = f167(v26, v1)?;
            let v29: (Adt12, Adt14) = f81(v27, {
                let (_, h1443) = v28;
                h1443
            })?;
            Ok((f140(Str::lit("simp-only"), f10(Str::lit("at"), {
                let (h1444, _) = v28;
                h1444
            }, f10(Str::lit("r"), f112({
                let (h1445, _) = v29;
                h1445
            }), f114()))), {
                let (_, h1446) = v29;
                h1446
            }))
        }
        Adt10::C12(r1485) => {
            let v30 = (*r1485).clone();
            let v31: (Adt12, Adt14) = f141(v30, v1)?;
            Ok((f140(Str::lit("constructor"), f10(Str::lit("b"), f112({
                let (h1447, _) = v31;
                h1447
            }), f114())), {
                let (_, h1448) = v31;
                h1448
            }))
        }
        Adt10::C13(v32, r1486) => {
            let v33 = (*r1486).clone();
            let v34: (Adt12, Adt14) = f81(v32, v1)?;
            let v35: (Adt12, Adt14) = f141(v33, {
                let (_, h1449) = v34;
                h1449
            })?;
            Ok((f140(Str::lit("cases"), f10(Str::lit("t"), {
                let (h1450, _) = v34;
                h1450
            }, f10(Str::lit("c"), f112({
                let (h1451, _) = v35;
                h1451
            }), f114()))), {
                let (_, h1452) = v35;
                h1452
            }))
        }
        Adt10::C14(v36, r1487) => {
            let v37 = (*r1487).clone();
            let v38: (Adt12, Adt14) = f81(v36, v1)?;
            let v39: (Adt12, Adt14) = f141(v37, {
                let (_, h1453) = v38;
                h1453
            })?;
            Ok((f140(Str::lit("induction"), f10(Str::lit("t"), {
                let (h1454, _) = v38;
                h1454
            }, f10(Str::lit("c"), f112({
                let (h1455, _) = v39;
                h1455
            }), f114()))), {
                let (_, h1456) = v39;
                h1456
            }))
        }
        Adt10::C15(v40, v41, r1488) => {
            let v42 = (*r1488).clone();
            let v43: (Adt12, Adt14) = f81(v41, v1)?;
            let v44: (Adt12, Adt14) = f141(v42, {
                let (_, h1457) = v43;
                h1457
            })?;
            Ok((f140(Str::lit("calc"), f10(Str::lit("rel"), f105(v40, Adt3::C5)?, f10(Str::lit("s"), {
                let (h1458, _) = v43;
                h1458
            }, f10(Str::lit("steps"), f112({
                let (h1459, _) = v44;
                h1459
            }), f114())))), {
                let (_, h1460) = v44;
                h1460
            }))
        }
        Adt10::C16 => {
            Ok((f114(), v1))
        }
        Adt10::C17(r1489, r1490) => {
            let v45 = (*r1489).clone();
            let v46 = (*r1490).clone();
            let v47: (Adt12, Adt14) = f141(v45, v1)?;
            let v48: (Adt12, Adt14) = f141(v46, {
                let (_, h1461) = v47;
                h1461
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h1462, _) = v47;
                h1462
            }), std::rc::Rc::new({
                let (h1463, _) = v48;
                h1463
            })), {
                let (_, h1464) = v48;
                h1464
            }))
        }
        Adt10::C18(v49, v50, r1491) => {
            let v51 = (*r1491).clone();
            let v52: (Adt12, Adt14) = f81(v50, v1)?;
            let v53: (Adt12, Adt14) = f141(v51, {
                let (_, h1465) = v52;
                h1465
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f110(f10(Str::lit("rev"), Adt12::C0(v49), f10(Str::lit("t"), {
                let (h1466, _) = v52;
                h1466
            }, f114())))), std::rc::Rc::new({
                let (h1467, _) = v53;
                h1467
            })), {
                let (_, h1468) = v53;
                h1468
            }))
        }
        Adt10::C19(v54, v55, v56, r1492, r1493) => {
            let v57 = (*r1492).clone();
            let v58 = (*r1493).clone();
            let v59: (Adt12, Adt14) = f165(f168(v56), v1)?;
            let v60: (Adt12, Adt14) = f141(v57, {
                let (_, h1469) = v59;
                h1469
            })?;
            let v61: (Adt12, Adt14) = f141(v58, {
                let (_, h1470) = v60;
                h1470
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f110(f10(Str::lit("ctor"), Adt12::C2(v54), f10(Str::lit("lean"), Adt12::C2(v55), f10(Str::lit("b"), f112({
                let (h1471, _) = v59;
                h1471
            }), f10(Str::lit("p"), {
                let (h1472, _) = v60;
                h1472
            }, f114())))))), std::rc::Rc::new({
                let (h1473, _) = v61;
                h1473
            })), {
                let (_, h1474) = v61;
                h1474
            }))
        }
        Adt10::C20(v62, v63, r1494) => {
            let v64 = (*r1494).clone();
            let v65: (Adt12, Adt14) = f81(v62, v1)?;
            let v66: (Adt12, Adt14) = f81(v63, {
                let (_, h1475) = v65;
                h1475
            })?;
            let v67: (Adt12, Adt14) = f141(v64, {
                let (_, h1476) = v66;
                h1476
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f110(f10(Str::lit("t"), {
                let (h1477, _) = v65;
                h1477
            }, f10(Str::lit("p"), {
                let (h1478, _) = v66;
                h1478
            }, f114())))), std::rc::Rc::new({
                let (h1479, _) = v67;
                h1479
            })), {
                let (_, h1480) = v67;
                h1480
            }))
        }
    }
}

pub fn f142(v0: List<Bytes>) -> Adt12 {
    f169(Fn25::F170, f114(), f85(v0))
}

pub fn f143(v0: Adt0, v1: Bytes) -> Bytes {
    let v2: Bytes = match v0.clone() {
        Adt0::C0(_, h1495, _, _, _, _, _, _, _, _, _, _, _) => {
            h1495
        }
    };
    let v3: u64 = {
        let a1497 = match v0.clone() {
            Adt0::C0(h1496, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1496
            }
        };
        length_bytes(a1497)
    };
    let v4: u64 = {
        let a1498 = v2.clone();
        length_bytes(a1498)
    };
    let v5: Bytes = if f171(v2.clone(), match v0 {
        Adt0::C0(h1499, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1499
        }
    }) {
        f129(v2, 0u64, {
            let a1500 = v4;
            let a1501 = v3;
            nat_sub(a1500, a1501)
        })
    } else {
        Bytes::lit(&[])
    };
    let a1502 = v5;
    let a1503 = v1;
    append_bytes(a1502, a1503)
}

pub fn f144(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h1504, _) = v0;
        h1504
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h1505) = v0;
        h1505
    };
    let m1506 = v1;
    match m1506.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m1507 = v4;
            match m1507.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f172(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f145(v0: u64) -> u32 {
    let a1508 = v0;
    let a1509 = 32u64;
    if nat_lt(a1508, a1509) {
        let a1510 = v0;
        let a1511 = 16u64;
        if nat_lt(a1510, a1511) {
            let a1512 = v0;
            let a1513 = 8u64;
            if nat_lt(a1512, a1513) {
                let a1514 = v0;
                let a1515 = 4u64;
                if nat_lt(a1514, a1515) {
                    let a1516 = v0;
                    let a1517 = 2u64;
                    if nat_lt(a1516, a1517) {
                        let a1518 = v0;
                        let a1519 = 1u64;
                        if nat_lt(a1518, a1519) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1520 = v0;
                        let a1521 = 3u64;
                        if nat_lt(a1520, a1521) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1522 = v0;
                    let a1523 = 6u64;
                    if nat_lt(a1522, a1523) {
                        let a1524 = v0;
                        let a1525 = 5u64;
                        if nat_lt(a1524, a1525) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1526 = v0;
                        let a1527 = 7u64;
                        if nat_lt(a1526, a1527) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1528 = v0;
                let a1529 = 12u64;
                if nat_lt(a1528, a1529) {
                    let a1530 = v0;
                    let a1531 = 10u64;
                    if nat_lt(a1530, a1531) {
                        let a1532 = v0;
                        let a1533 = 9u64;
                        if nat_lt(a1532, a1533) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1534 = v0;
                        let a1535 = 11u64;
                        if nat_lt(a1534, a1535) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1536 = v0;
                    let a1537 = 14u64;
                    if nat_lt(a1536, a1537) {
                        let a1538 = v0;
                        let a1539 = 13u64;
                        if nat_lt(a1538, a1539) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1540 = v0;
                        let a1541 = 15u64;
                        if nat_lt(a1540, a1541) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1542 = v0;
            let a1543 = 24u64;
            if nat_lt(a1542, a1543) {
                let a1544 = v0;
                let a1545 = 20u64;
                if nat_lt(a1544, a1545) {
                    let a1546 = v0;
                    let a1547 = 18u64;
                    if nat_lt(a1546, a1547) {
                        let a1548 = v0;
                        let a1549 = 17u64;
                        if nat_lt(a1548, a1549) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1550 = v0;
                        let a1551 = 19u64;
                        if nat_lt(a1550, a1551) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1552 = v0;
                    let a1553 = 22u64;
                    if nat_lt(a1552, a1553) {
                        let a1554 = v0;
                        let a1555 = 21u64;
                        if nat_lt(a1554, a1555) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1556 = v0;
                        let a1557 = 23u64;
                        if nat_lt(a1556, a1557) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1558 = v0;
                let a1559 = 28u64;
                if nat_lt(a1558, a1559) {
                    let a1560 = v0;
                    let a1561 = 26u64;
                    if nat_lt(a1560, a1561) {
                        let a1562 = v0;
                        let a1563 = 25u64;
                        if nat_lt(a1562, a1563) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1564 = v0;
                        let a1565 = 27u64;
                        if nat_lt(a1564, a1565) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1566 = v0;
                    let a1567 = 30u64;
                    if nat_lt(a1566, a1567) {
                        let a1568 = v0;
                        let a1569 = 29u64;
                        if nat_lt(a1568, a1569) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1570 = v0;
                        let a1571 = 31u64;
                        if nat_lt(a1570, a1571) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1572 = v0;
        let a1573 = 48u64;
        if nat_lt(a1572, a1573) {
            let a1574 = v0;
            let a1575 = 40u64;
            if nat_lt(a1574, a1575) {
                let a1576 = v0;
                let a1577 = 36u64;
                if nat_lt(a1576, a1577) {
                    let a1578 = v0;
                    let a1579 = 34u64;
                    if nat_lt(a1578, a1579) {
                        let a1580 = v0;
                        let a1581 = 33u64;
                        if nat_lt(a1580, a1581) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1582 = v0;
                        let a1583 = 35u64;
                        if nat_lt(a1582, a1583) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1584 = v0;
                    let a1585 = 38u64;
                    if nat_lt(a1584, a1585) {
                        let a1586 = v0;
                        let a1587 = 37u64;
                        if nat_lt(a1586, a1587) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1588 = v0;
                        let a1589 = 39u64;
                        if nat_lt(a1588, a1589) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1590 = v0;
                let a1591 = 44u64;
                if nat_lt(a1590, a1591) {
                    let a1592 = v0;
                    let a1593 = 42u64;
                    if nat_lt(a1592, a1593) {
                        let a1594 = v0;
                        let a1595 = 41u64;
                        if nat_lt(a1594, a1595) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1596 = v0;
                        let a1597 = 43u64;
                        if nat_lt(a1596, a1597) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1598 = v0;
                    let a1599 = 46u64;
                    if nat_lt(a1598, a1599) {
                        let a1600 = v0;
                        let a1601 = 45u64;
                        if nat_lt(a1600, a1601) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1602 = v0;
                        let a1603 = 47u64;
                        if nat_lt(a1602, a1603) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1604 = v0;
            let a1605 = 56u64;
            if nat_lt(a1604, a1605) {
                let a1606 = v0;
                let a1607 = 52u64;
                if nat_lt(a1606, a1607) {
                    let a1608 = v0;
                    let a1609 = 50u64;
                    if nat_lt(a1608, a1609) {
                        let a1610 = v0;
                        let a1611 = 49u64;
                        if nat_lt(a1610, a1611) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1612 = v0;
                        let a1613 = 51u64;
                        if nat_lt(a1612, a1613) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1614 = v0;
                    let a1615 = 54u64;
                    if nat_lt(a1614, a1615) {
                        let a1616 = v0;
                        let a1617 = 53u64;
                        if nat_lt(a1616, a1617) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1618 = v0;
                        let a1619 = 55u64;
                        if nat_lt(a1618, a1619) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1620 = v0;
                let a1621 = 60u64;
                if nat_lt(a1620, a1621) {
                    let a1622 = v0;
                    let a1623 = 58u64;
                    if nat_lt(a1622, a1623) {
                        let a1624 = v0;
                        let a1625 = 57u64;
                        if nat_lt(a1624, a1625) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1626 = v0;
                        let a1627 = 59u64;
                        if nat_lt(a1626, a1627) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1628 = v0;
                    let a1629 = 62u64;
                    if nat_lt(a1628, a1629) {
                        let a1630 = v0;
                        let a1631 = 61u64;
                        if nat_lt(a1630, a1631) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1632 = v0;
                        let a1633 = 63u64;
                        if nat_lt(a1632, a1633) {
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

pub fn f146(v0: u32) -> u32 {
    let a1636 = {
        let a1634 = f173(v0, 6u32, 26u32);
        let a1635 = f173(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1634, a1635)
    };
    let a1637 = f173(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1636, a1637)
}

pub fn f147(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1643 = {
        let a1638 = v0;
        let a1639 = v1;
        fixed_u32::bit_and(a1638, a1639)
    };
    let a1644 = {
        let a1641 = {
            let a1640 = v0;
            fixed_u32::bit_not(a1640)
        };
        let a1642 = v2;
        fixed_u32::bit_and(a1641, a1642)
    };
    fixed_u32::bit_xor(a1643, a1644)
}

pub fn f148(v0: u32) -> u32 {
    let a1647 = {
        let a1645 = f173(v0, 2u32, 30u32);
        let a1646 = f173(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1645, a1646)
    };
    let a1648 = f173(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1647, a1648)
}

pub fn f149(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1657 = {
        let a1653 = {
            let a1649 = v0;
            let a1650 = v1;
            fixed_u32::bit_and(a1649, a1650)
        };
        let a1654 = {
            let a1651 = v0;
            let a1652 = v2;
            fixed_u32::bit_and(a1651, a1652)
        };
        fixed_u32::bit_xor(a1653, a1654)
    };
    let a1658 = {
        let a1655 = v1;
        let a1656 = v2;
        fixed_u32::bit_and(a1655, a1656)
    };
    fixed_u32::bit_xor(a1657, a1658)
}

pub fn f150(v0: Adt15) -> Adt15 {
    Adt15::C0(match v0.clone() {
        Adt15::C0(_, h1659, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1659
        }
    }, match v0.clone() {
        Adt15::C0(_, _, h1660, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1660
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, h1661, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1661
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, h1662, _, _, _, _, _, _, _, _, _, _, _) => {
            h1662
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, h1663, _, _, _, _, _, _, _, _, _, _) => {
            h1663
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, h1664, _, _, _, _, _, _, _, _, _) => {
            h1664
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, h1665, _, _, _, _, _, _, _, _) => {
            h1665
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, h1666, _, _, _, _, _, _, _) => {
            h1666
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, h1667, _, _, _, _, _, _) => {
            h1667
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, h1668, _, _, _, _, _) => {
            h1668
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, h1669, _, _, _, _) => {
            h1669
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1670, _, _, _) => {
            h1670
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1671, _, _) => {
            h1671
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1672, _) => {
            h1672
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1673) => {
            h1673
        }
    }, f92(f174(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1674, _) => {
            h1674
        }
    }), f92(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, h1675, _, _, _, _, _, _) => {
            h1675
        }
    }, f92(f175(match v0.clone() {
        Adt15::C0(_, h1676, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1676
        }
    }), match v0 {
        Adt15::C0(h1677, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1677
        }
    }))))
}

pub fn f151(v0: Fn22, p1681: u64, p1682: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1680 = (p1681, p1682);
    {
        let mut m1680 = m1680;
        loop {
            let step = {
                let (v1, v2) = m1680;
                let m1678 = v1;
                if m1678 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1678 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1679: Fn22 = v0.clone();
                        c1679.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1683, _) = v0;
        h1683
    };
    let v2: u32 = {
        let (h1684, _) = {
            let (_, h1685) = v0;
            h1685
        };
        h1684
    };
    let v3: u32 = {
        let (_, h1686) = {
            let (_, h1687) = v0;
            h1687
        };
        h1686
    };
    ({
        let a1688 = v1;
        let a1689 = 2u64;
        let a1690 = 0u64;
        nat_quot(a1688, a1689, a1690)
    }, (f153(v2, 1u32), {
        let a1694 = {
            let a1691 = v1;
            let a1692 = 2u64;
            let a1693 = 0u64;
            nat_rem(a1691, a1692, a1693)
        };
        let a1695 = 1u64;
        if nat_eq(a1694, a1695) {
            let a1696 = v3;
            let a1697 = v2;
            fixed_u32::bit_or(a1696, a1697)
        } else {
            v3
        }
    }))
}

pub fn f153(v0: u32, v1: u32) -> u32 {
    let m1700 = {
        let a1698 = v0;
        let a1699 = v1;
        fixed_u32::shift_left(a1698, a1699)
    };
    match m1700 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f154() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f155(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f96({
        let (_, h1701) = {
            let (h1702, _) = f176(Fn26::F177, {
                let a1704 = {
                    let a1703 = v0.clone();
                    length_list(a1703)
                };
                let a1705 = 1u64;
                nat_add(a1704, a1705)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h1702
        };
        h1701
    }))
}

pub fn f156(v0: u64) -> u64 {
    let a1706 = v0;
    let a1707 = 9223372036854775807u64;
    if nat_le(a1706, a1707) {
        v0
    } else {
        9223372036854775807u64
    }
}

pub fn f157(v0: Fn23, p1711: Option<u64>, p1712: List<(u64, u64)>) -> Option<u64> {
    let m1710 = (p1711, p1712);
    {
        let mut m1710 = m1710;
        loop {
            let step = {
                let (v1, v2) = m1710;
                let m1708 = v2;
                match m1708.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c1709: Fn23 = v0.clone();
                            c1709.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let a1714 = {
        let (h1713, _) = v2;
        h1713
    };
    let a1715 = v0;
    if nat_eq(a1714, a1715) {
        Some({
            let (_, h1716) = v2;
            h1716
        })
    } else {
        v1
    }
}

pub fn f159(v0: Adt5) -> Bytes {
    let m1717 = v0;
    match m1717 {
        Adt5::C0 => {
            f28(Str::lit("eq"))
        }
        Adt5::C1 => {
            f28(Str::lit("and"))
        }
        Adt5::C2 => {
            f28(Str::lit("or"))
        }
        Adt5::C3 => {
            f28(Str::lit("not"))
        }
        Adt5::C4 => {
            f28(Str::lit("iff"))
        }
        Adt5::C5 => {
            f28(Str::lit("exists"))
        }
        Adt5::C6 => {
            f28(Str::lit("exists-unique"))
        }
    }
}

pub fn f160(v0: Bytes) -> R<Bytes> {
    let (_, h1718) = {
        let (h1719, _) = f47(Fn7::F178(v0.clone()), {
            let a1720 = v0;
            length_bytes(a1720)
        }, (0u64, Bytes::lit(&[])))?;
        h1719
    };
    Ok(h1718)
}

pub fn f161(v0: Fn24, p1724: Adt12, p1725: List<u64>) -> Adt12 {
    let m1723 = (p1724, p1725);
    {
        let mut m1723 = m1723;
        loop {
            let step = {
                let (v1, v2) = m1723;
                let m1721 = v2;
                match m1721.uncons() {
                    None => {
                        Err::<(Adt12, List<u64>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<u64>), Adt12>(({
                            let c1722: Fn24 = v0.clone();
                            c1722.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Adt12, v1: u64) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(f133(v1)), std::rc::Rc::new(v0))
}

pub fn f163(v0: List<u64>) -> List<u64> {
    f179(Fn27::F180, List::<u64>::nil(), v0)
}

pub fn f164(v0: u64) -> Bytes {
    let a1726 = v0;
    let a1727 = 1u64;
    if nat_eq(a1726, a1727) {
        f181(Str::lit("implicit"))
    } else {
        let a1728 = v0;
        let a1729 = 2u64;
        if nat_eq(a1728, a1729) {
            f181(Str::lit("instance"))
        } else {
            f181(Str::lit("explicit"))
        }
    }
}

pub fn f165(v0: List<u64>, v1: Adt14) -> R<(Adt12, Adt14)> {
    let v2: (List<u64>, Adt14) = f182(Fn28::F183, (List::<u64>::nil(), v1), v0)?;
    Ok((f161(Fn24::F184, f114(), {
        let (h1730, _) = v2;
        h1730
    }), {
        let (_, h1731) = v2;
        h1731
    }))
}

pub fn f166(v0: Str, v1: Adt2, v2: Adt14, v3: Adt12) -> R<(Adt12, Adt14)> {
    let v4: (Adt12, Adt14) = f81(v1, v2)?;
    Ok((f10(v0, {
        let (h1732, _) = v4;
        h1732
    }, v3), {
        let (_, h1733) = v4;
        h1733
    }))
}

pub fn f167(v0: Option<u64>, v1: Adt14) -> R<(Adt12, Adt14)> {
    let m1734 = v0;
    match m1734 {
        None => {
            Ok((f11(Str::lit("goal")), v1))
        }
        Some(v2) => {
            let v3: (Adt17, Adt14) = f103(v1, v2)?;
            Ok(({
                let m1736 = {
                    let (h1735, _) = v3;
                    h1735
                };
                match m1736 {
                    Adt17::C0(v4) => {
                        f133(v4)
                    }
                    Adt17::C1(v5) => {
                        f133(v5)
                    }
                }
            }, {
                let (_, h1737) = v3;
                h1737
            }))
        }
    }
}

pub fn f168(v0: List<(u64, Bytes)>) -> List<u64> {
    f163(f185(Fn29::F186, List::<u64>::nil(), v0))
}

pub fn f169(v0: Fn25, p1741: Adt12, p1742: List<Bytes>) -> Adt12 {
    let m1740 = (p1741, p1742);
    {
        let mut m1740 = m1740;
        loop {
            let step = {
                let (v1, v2) = m1740;
                let m1738 = v2;
                match m1738.uncons() {
                    None => {
                        Err::<(Adt12, List<Bytes>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Bytes>), Adt12>(({
                            let c1739: Fn25 = v0.clone();
                            c1739.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: Adt12, v1: Bytes) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(Adt12::C2(v1)), std::rc::Rc::new(v0))
}

pub fn f171(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a1743 = v0.clone();
        length_bytes(a1743)
    };
    let v3: u64 = {
        let a1744 = v1.clone();
        length_bytes(a1744)
    };
    let a1745 = v3;
    let a1746 = v2;
    if nat_le(a1745, a1746) {
        let m1752 = {
            let a1749 = v0;
            let a1750 = {
                let a1747 = v2;
                let a1748 = v3;
                nat_sub(a1747, a1748)
            };
            let a1751 = v3;
            slice_bytes(a1749, a1750, a1751)
        };
        match m1752 {
            None => {
                false
            }
            Some(v4) => {
                let a1753 = v4;
                let a1754 = v1;
                equal(a1753, a1754)
            }
        }
    } else {
        false
    }
}

pub fn f172(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a1759 = {
            let a1757 = {
                let a1755 = v0.clone();
                length_list(a1755)
            };
            let a1758 = {
                let a1756 = v1.clone();
                length_list(a1756)
            };
            nat_add(a1757, a1758)?
        };
        let a1760 = 1u64;
        nat_add(a1759, a1760)?
    };
    Ok(f16({
        let (_, h1761) = {
            let (_, h1762) = {
                let (h1763, _) = f187(Fn30::F188, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h1763
            };
            h1762
        };
        h1761
    }))
}

pub fn f173(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1764 = f67(v0, v1);
    let a1765 = f153(v0, v2);
    fixed_u32::bit_or(a1764, a1765)
}

pub fn f174(v0: u32) -> u32 {
    let a1768 = {
        let a1766 = f173(v0, 17u32, 15u32);
        let a1767 = f173(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1766, a1767)
    };
    let a1769 = f67(v0, 10u32);
    fixed_u32::bit_xor(a1768, a1769)
}

pub fn f175(v0: u32) -> u32 {
    let a1772 = {
        let a1770 = f173(v0, 7u32, 25u32);
        let a1771 = f173(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1770, a1771)
    };
    let a1773 = f67(v0, 3u32);
    fixed_u32::bit_xor(a1772, a1773)
}

pub fn f176(v0: Fn26, p1778: u64, p1779: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m1777 = (p1778, p1779);
    {
        let mut m1777 = m1777;
        loop {
            let step = {
                let (v1, v2) = m1777;
                let m1774 = v1;
                if m1774 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1774 - 1;
                    let m1776 = {
                        let c1775: Fn26 = v0.clone();
                        c1775.apply(v2.clone())?
                    };
                    match m1776 {
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
                Ok(next) => { m1777 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f189(v0)
}

pub fn f178(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f190(v0, v1)
}

pub fn f179(v0: Fn27, p1783: List<u64>, p1784: List<u64>) -> List<u64> {
    let m1782 = (p1783, p1784);
    {
        let mut m1782 = m1782;
        loop {
            let step = {
                let (v1, v2) = m1782;
                let m1780 = v2;
                match m1780.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c1781: Fn27 = v0.clone();
                            c1781.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f181(v0: Str) -> Bytes {
    let a1785 = v0;
    utf8_encode(a1785)
}

pub fn f182(v0: Fn28, p1789: (List<u64>, Adt14), p1790: List<u64>) -> R<(List<u64>, Adt14)> {
    let m1788 = (p1789, p1790);
    {
        let mut m1788 = m1788;
        loop {
            let step = {
                let (v1, v2) = m1788;
                let m1786 = v2;
                match m1786.uncons() {
                    None => {
                        Err::<((List<u64>, Adt14), List<u64>), R<(List<u64>, Adt14)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<u64>, Adt14), List<u64>), R<(List<u64>, Adt14)>>(({
                            let c1787: Fn28 = v0.clone();
                            c1787.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1788 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: (List<u64>, Adt14), v1: u64) -> R<(List<u64>, Adt14)> {
    let v2: (u64, Adt14) = f107({
        let (_, h1791) = v0;
        h1791
    }, v1)?;
    Ok((List::cons({
        let (h1792, _) = v2;
        h1792
    }, {
        let (h1793, _) = v0;
        h1793
    }), {
        let (_, h1794) = v2;
        h1794
    }))
}

pub fn f184(v0: Adt12, v1: u64) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(f133(v1)), std::rc::Rc::new(v0))
}

pub fn f185(v0: Fn29, p1798: List<u64>, p1799: List<(u64, Bytes)>) -> List<u64> {
    let m1797 = (p1798, p1799);
    {
        let mut m1797 = m1797;
        loop {
            let step = {
                let (v1, v2) = m1797;
                let m1795 = v2;
                match m1795.uncons() {
                    None => {
                        Err::<(List<u64>, List<(u64, Bytes)>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<(u64, Bytes)>), List<u64>>(({
                            let c1796: Fn29 = v0.clone();
                            c1796.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1797 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: List<u64>, v1: (u64, Bytes)) -> List<u64> {
    List::cons({
        let (h1800, _) = v1;
        h1800
    }, v0)
}

pub fn f187(v0: Fn30, p1805: u64, p1806: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m1804 = (p1805, p1806);
    {
        let mut m1804 = m1804;
        loop {
            let step = {
                let (v1, v2) = m1804;
                let m1801 = v1;
                if m1801 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m1801 - 1;
                    let m1803 = {
                        let c1802: Fn30 = v0.clone();
                        c1802.apply(v2.clone())
                    };
                    match m1803 {
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
                Ok(next) => { m1804 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f191(v0)
}

pub fn f189(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h1807, _) = v0;
        h1807
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h1808) = v0;
        h1808
    };
    let m1809 = v1;
    match m1809.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m1810 = v4;
            match m1810.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f192(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1814 = {
        let a1812 = v0;
        let a1813 = {
            let (h1811, _) = v1.clone();
            h1811
        };
        index_bytes(a1812, a1813)
    };
    match m1814 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1816 = {
                    let (h1815, _) = v1;
                    h1815
                };
                let a1817 = 1u64;
                nat_add(a1816, a1817)?
            }, {
                let a1819 = {
                    let (_, h1818) = v1;
                    h1818
                };
                let a1820 = f128(v2)?;
                append_bytes(a1819, a1820)
            })))
        }
    }
}

pub fn f191(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h1821, _) = v0.clone();
        h1821
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h1822, _) = {
            let (_, h1823) = v0.clone();
            h1823
        };
        h1822
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h1824) = {
            let (_, h1825) = v0;
            h1825
        };
        h1824
    };
    let m1826 = v1.clone();
    match m1826.uncons() {
        None => {
            let m1827 = v2;
            match m1827.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1828 = v2.clone();
            match m1828.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f193({
                        let (h1829, _) = v8.clone();
                        h1829
                    }, {
                        let (h1830, _) = v6.clone();
                        h1830
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

pub fn f192(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a1835 = {
            let a1833 = {
                let a1831 = v0.clone();
                length_list(a1831)
            };
            let a1834 = {
                let a1832 = v1.clone();
                length_list(a1832)
            };
            nat_add(a1833, a1834)?
        };
        let a1836 = 1u64;
        nat_add(a1835, a1836)?
    };
    Ok(f194({
        let (_, h1837) = {
            let (_, h1838) = {
                let (h1839, _) = f195(Fn31::F196, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h1839
            };
            h1838
        };
        h1837
    }))
}

pub fn f193(v0: Bytes, v1: Bytes) -> bool {
    let a1844 = {
        let a1840 = v0;
        let a1841 = v1;
        compare_bytes(a1840, a1841)
    };
    let a1845 = {
        let a1842 = Bytes::lit(&[]);
        let a1843 = Bytes::lit(&[0x01]);
        compare_bytes(a1842, a1843)
    };
    equal(a1844, a1845)
}

pub fn f194(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f50(Fn8::F197, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f195(v0: Fn31, p1850: u64, p1851: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m1849 = (p1850, p1851);
    {
        let mut m1849 = m1849;
        loop {
            let step = {
                let (v1, v2) = m1849;
                let m1846 = v1;
                if m1846 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m1846 - 1;
                    let m1848 = {
                        let c1847: Fn31 = v0.clone();
                        c1847.apply(v2.clone())
                    };
                    match m1848 {
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
                Ok(next) => { m1849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f198(v0)
}

pub fn f197(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f198(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h1852, _) = v0.clone();
        h1852
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h1853, _) = {
            let (_, h1854) = v0.clone();
            h1854
        };
        h1853
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h1855) = {
            let (_, h1856) = v0;
            h1856
        };
        h1855
    };
    let m1857 = v1.clone();
    match m1857.uncons() {
        None => {
            let m1858 = v2;
            match m1858.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1859 = v2.clone();
            match m1859.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f193({
                        let (h1860, _) = v8.clone();
                        h1860
                    }, {
                        let (h1861, _) = v6.clone();
                        h1861
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
