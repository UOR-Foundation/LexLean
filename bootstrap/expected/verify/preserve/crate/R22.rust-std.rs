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
    C0(std::rc::Rc<Adt0>),
    C1(List<u64>),
    C2(Adt1),
    C3(Adt1),
    C4(Adt1, std::rc::Rc<Adt0>),
    C5,
    C6(Adt1),
    C7,
    C8,
    C9(u64, Adt1, std::rc::Rc<Adt0>),
    C10(Option<u64>, std::rc::Rc<Adt0>),
    C11(Option<u64>, Adt1),
    C12(std::rc::Rc<Adt0>),
    C13(Adt1, std::rc::Rc<Adt0>),
    C14(Adt1, std::rc::Rc<Adt0>),
    C15(Adt3, Adt1, std::rc::Rc<Adt0>),
    C16,
    C17(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C18(bool, Adt1, std::rc::Rc<Adt0>),
    C19(Bytes, Bytes, List<(u64, Bytes)>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C20(Adt1, Adt1, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2),
    C1(u64),
    C2(Adt3, Adt2),
    C3(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>, List<u64>),
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C5(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C6(u64, u64, Bytes, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C7(Bytes, std::rc::Rc<Adt1>),
    C8,
    C9(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C10(u64, u64, Bytes, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64),
    C1(Bytes),
    C2(std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>),
    C4(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5,
    C6(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt4),
    C1(Bytes, Bytes, Bytes, Bytes, Bytes),
    C2(Bytes, Bytes, Bytes),
    C3(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt5 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt5>),
    C4(std::rc::Rc<Adt5>),
    C5,
    C6(std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
    C7(Bytes, std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(List<(u64, u64)>, u64, bool),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F27,
}

impl Fn0 {
    pub fn apply(&self, p0: (List<u64>, Adt6), p1: u64) -> R<(List<u64>, Adt6)> {
        match self {
            Fn0::F27 => f27(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F29,
    F68,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt5, p1: u64) -> Adt5 {
        match self {
            Fn1::F29 => f29(p0, p1),
            Fn1::F68 => f68(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F42,
}

impl Fn2 {
    pub fn apply(&self, p0: List<u64>, p1: (u64, Bytes)) -> List<u64> {
        match self {
            Fn2::F42 => f42(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F45(Bytes),
    F70(Bytes),
    F72(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F45(k0) => f45(k0.clone(), p0),
            Fn3::F70(k0) => f70(k0.clone(), p0),
            Fn3::F72(k0, k1) => Ok(f72(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F48,
    F96,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn4::F48 => f48(p0, p1),
            Fn4::F96 => f96(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F51,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn5::F51 => f51(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F55(u64),
}

impl Fn6 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn6::F55(k0) => f55(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F59,
}

impl Fn7 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn7::F59 => f59(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F64(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn8::F64(k0) => f64(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F76,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F76 => f76(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F78,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F78 => f78(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F85,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F85 => f85(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F90,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F90 => f90(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F95,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F95 => f95(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1({
        let (h1, _) = f2(v0, f3())?;
        h1
    })
}

pub fn f1(v0: Adt5) -> R<Bytes> {
    f4(v0)
}

pub fn f2(v0: Adt0, v1: Adt6) -> R<(Adt5, Adt6)> {
    let m2 = v0;
    match m2 {
        Adt0::C0(r65) => {
            let v2 = (*r65).clone();
            let v3: (Adt5, Adt6) = f2(v2, v1)?;
            Ok((f5(Str::lit("seq"), f6(Str::lit("s"), f7({
                let (h3, _) = v3;
                h3
            }), f8())), {
                let (_, h4) = v3;
                h4
            }))
        }
        Adt0::C1(v4) => {
            let v5: (Adt5, Adt6) = f9(v4, v1)?;
            Ok((f5(Str::lit("intro"), f6(Str::lit("l"), f7({
                let (h5, _) = v5;
                h5
            }), f8())), {
                let (_, h6) = v5;
                h6
            }))
        }
        Adt0::C2(v6) => {
            let v7: (Adt5, Adt6) = f10(Str::lit("t"), v6, v1, f8())?;
            Ok((f5(Str::lit("exact"), {
                let (h7, _) = v7;
                h7
            }), {
                let (_, h8) = v7;
                h8
            }))
        }
        Adt0::C3(v8) => {
            let v9: (Adt5, Adt6) = f10(Str::lit("t"), v8, v1, f8())?;
            Ok((f5(Str::lit("apply-one"), {
                let (h9, _) = v9;
                h9
            }), {
                let (_, h10) = v9;
                h10
            }))
        }
        Adt0::C4(v10, r66) => {
            let v11 = (*r66).clone();
            let v12: (Adt5, Adt6) = f11(v10, v1)?;
            let v13: (Adt5, Adt6) = f2(v11, {
                let (_, h11) = v12;
                h11
            })?;
            Ok((f5(Str::lit("apply"), f6(Str::lit("f"), {
                let (h12, _) = v12;
                h12
            }, f6(Str::lit("p"), f7({
                let (h13, _) = v13;
                h13
            }), f8()))), {
                let (_, h14) = v13;
                h14
            }))
        }
        Adt0::C5 => {
            Ok((f5(Str::lit("rfl"), f8()), v1))
        }
        Adt0::C6(v14) => {
            let v15: (Adt5, Adt6) = f10(Str::lit("t"), v14, v1, f8())?;
            Ok((f5(Str::lit("witness"), {
                let (h15, _) = v15;
                h15
            }), {
                let (_, h16) = v15;
                h16
            }))
        }
        Adt0::C7 => {
            Ok((f5(Str::lit("left"), f8()), v1))
        }
        Adt0::C8 => {
            Ok((f5(Str::lit("right"), f8()), v1))
        }
        Adt0::C9(v16, v17, r67) => {
            let v18 = (*r67).clone();
            let v19: (Adt5, Adt6) = f11(v17, v1)?;
            let v20: (u64, Adt6) = f12({
                let (_, h17) = v19;
                h17
            }, v16)?;
            let v21: (Adt5, Adt6) = f2(v18, {
                let (_, h18) = v20;
                h18
            })?;
            Ok((f5(Str::lit("have"), f6(Str::lit("l"), f13({
                let (h19, _) = v20;
                h19
            }), f6(Str::lit("p"), {
                let (h20, _) = v19;
                h20
            }, f6(Str::lit("q"), {
                let (h21, _) = v21;
                h21
            }, f8())))), {
                let (_, h22) = v21;
                h22
            }))
        }
        Adt0::C10(v22, r68) => {
            let v23 = (*r68).clone();
            let v24: (Adt5, Adt6) = f14(v22, v1)?;
            let v25: (Adt5, Adt6) = f2(v23, {
                let (_, h23) = v24;
                h23
            })?;
            Ok((f5(Str::lit("rw"), f6(Str::lit("at"), {
                let (h24, _) = v24;
                h24
            }, f6(Str::lit("r"), f7({
                let (h25, _) = v25;
                h25
            }), f8()))), {
                let (_, h26) = v25;
                h26
            }))
        }
        Adt0::C11(v26, v27) => {
            let v28: (Adt5, Adt6) = f14(v26, v1)?;
            let v29: (Adt5, Adt6) = f11(v27, {
                let (_, h27) = v28;
                h27
            })?;
            Ok((f5(Str::lit("simp-only"), f6(Str::lit("at"), {
                let (h28, _) = v28;
                h28
            }, f6(Str::lit("r"), f7({
                let (h29, _) = v29;
                h29
            }), f8()))), {
                let (_, h30) = v29;
                h30
            }))
        }
        Adt0::C12(r69) => {
            let v30 = (*r69).clone();
            let v31: (Adt5, Adt6) = f2(v30, v1)?;
            Ok((f5(Str::lit("constructor"), f6(Str::lit("b"), f7({
                let (h31, _) = v31;
                h31
            }), f8())), {
                let (_, h32) = v31;
                h32
            }))
        }
        Adt0::C13(v32, r70) => {
            let v33 = (*r70).clone();
            let v34: (Adt5, Adt6) = f11(v32, v1)?;
            let v35: (Adt5, Adt6) = f2(v33, {
                let (_, h33) = v34;
                h33
            })?;
            Ok((f5(Str::lit("cases"), f6(Str::lit("t"), {
                let (h34, _) = v34;
                h34
            }, f6(Str::lit("c"), f7({
                let (h35, _) = v35;
                h35
            }), f8()))), {
                let (_, h36) = v35;
                h36
            }))
        }
        Adt0::C14(v36, r71) => {
            let v37 = (*r71).clone();
            let v38: (Adt5, Adt6) = f11(v36, v1)?;
            let v39: (Adt5, Adt6) = f2(v37, {
                let (_, h37) = v38;
                h37
            })?;
            Ok((f5(Str::lit("induction"), f6(Str::lit("t"), {
                let (h38, _) = v38;
                h38
            }, f6(Str::lit("c"), f7({
                let (h39, _) = v39;
                h39
            }), f8()))), {
                let (_, h40) = v39;
                h40
            }))
        }
        Adt0::C15(v40, v41, r72) => {
            let v42 = (*r72).clone();
            let v43: (Adt5, Adt6) = f11(v41, v1)?;
            let v44: (Adt5, Adt6) = f2(v42, {
                let (_, h41) = v43;
                h41
            })?;
            Ok((f5(Str::lit("calc"), f6(Str::lit("rel"), f15(v40, Adt2::C5)?, f6(Str::lit("s"), {
                let (h42, _) = v43;
                h42
            }, f6(Str::lit("steps"), f7({
                let (h43, _) = v44;
                h43
            }), f8())))), {
                let (_, h44) = v44;
                h44
            }))
        }
        Adt0::C16 => {
            Ok((f8(), v1))
        }
        Adt0::C17(r73, r74) => {
            let v45 = (*r73).clone();
            let v46 = (*r74).clone();
            let v47: (Adt5, Adt6) = f2(v45, v1)?;
            let v48: (Adt5, Adt6) = f2(v46, {
                let (_, h45) = v47;
                h45
            })?;
            Ok((Adt5::C6(std::rc::Rc::new({
                let (h46, _) = v47;
                h46
            }), std::rc::Rc::new({
                let (h47, _) = v48;
                h47
            })), {
                let (_, h48) = v48;
                h48
            }))
        }
        Adt0::C18(v49, v50, r75) => {
            let v51 = (*r75).clone();
            let v52: (Adt5, Adt6) = f11(v50, v1)?;
            let v53: (Adt5, Adt6) = f2(v51, {
                let (_, h49) = v52;
                h49
            })?;
            Ok((Adt5::C6(std::rc::Rc::new(f16(f6(Str::lit("rev"), Adt5::C0(v49), f6(Str::lit("t"), {
                let (h50, _) = v52;
                h50
            }, f8())))), std::rc::Rc::new({
                let (h51, _) = v53;
                h51
            })), {
                let (_, h52) = v53;
                h52
            }))
        }
        Adt0::C19(v54, v55, v56, r76, r77) => {
            let v57 = (*r76).clone();
            let v58 = (*r77).clone();
            let v59: (Adt5, Adt6) = f9(f17(v56), v1)?;
            let v60: (Adt5, Adt6) = f2(v57, {
                let (_, h53) = v59;
                h53
            })?;
            let v61: (Adt5, Adt6) = f2(v58, {
                let (_, h54) = v60;
                h54
            })?;
            Ok((Adt5::C6(std::rc::Rc::new(f16(f6(Str::lit("ctor"), Adt5::C2(v54), f6(Str::lit("lean"), Adt5::C2(v55), f6(Str::lit("b"), f7({
                let (h55, _) = v59;
                h55
            }), f6(Str::lit("p"), {
                let (h56, _) = v60;
                h56
            }, f8())))))), std::rc::Rc::new({
                let (h57, _) = v61;
                h57
            })), {
                let (_, h58) = v61;
                h58
            }))
        }
        Adt0::C20(v62, v63, r78) => {
            let v64 = (*r78).clone();
            let v65: (Adt5, Adt6) = f11(v62, v1)?;
            let v66: (Adt5, Adt6) = f11(v63, {
                let (_, h59) = v65;
                h59
            })?;
            let v67: (Adt5, Adt6) = f2(v64, {
                let (_, h60) = v66;
                h60
            })?;
            Ok((Adt5::C6(std::rc::Rc::new(f16(f6(Str::lit("t"), {
                let (h61, _) = v65;
                h61
            }, f6(Str::lit("p"), {
                let (h62, _) = v66;
                h62
            }, f8())))), std::rc::Rc::new({
                let (h63, _) = v67;
                h63
            })), {
                let (_, h64) = v67;
                h64
            }))
        }
    }
}

pub fn f3() -> Adt6 {
    Adt6::C0(List::<(u64, u64)>::nil(), 0u64, false)
}

pub fn f4(v0: Adt5) -> R<Bytes> {
    let m79 = v0;
    match m79 {
        Adt5::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt5::C1(v2, v3) => {
            f18(v2, v3)
        }
        Adt5::C2(v4) => {
            f19(v4)
        }
        Adt5::C3(r80) => {
            let v5 = (*r80).clone();
            Ok(f20(f21(v5)?))
        }
        Adt5::C4(r81) => {
            let v6 = (*r81).clone();
            f22(f23(v6)?)
        }
        Adt5::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt5::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt5::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f5(v0: Str, v1: Adt5) -> Adt5 {
    f16(f6(Str::lit("k"), f24(v0), v1))
}

pub fn f6(v0: Str, v1: Adt5, v2: Adt5) -> Adt5 {
    Adt5::C7(f25(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f7(v0: Adt5) -> Adt5 {
    Adt5::C3(std::rc::Rc::new(v0))
}

pub fn f8() -> Adt5 {
    Adt5::C5
}

pub fn f9(v0: List<u64>, v1: Adt6) -> R<(Adt5, Adt6)> {
    let v2: (List<u64>, Adt6) = f26(Fn0::F27, (List::<u64>::nil(), v1), v0)?;
    Ok((f28(Fn1::F29, f8(), {
        let (h82, _) = v2;
        h82
    }), {
        let (_, h83) = v2;
        h83
    }))
}

pub fn f10(v0: Str, v1: Adt1, v2: Adt6, v3: Adt5) -> R<(Adt5, Adt6)> {
    let v4: (Adt5, Adt6) = f11(v1, v2)?;
    Ok((f6(v0, {
        let (h84, _) = v4;
        h84
    }, v3), {
        let (_, h85) = v4;
        h85
    }))
}

pub fn f11(v0: Adt1, v1: Adt6) -> R<(Adt5, Adt6)> {
    let m86 = v0;
    match m86 {
        Adt1::C0(v2) => {
            Ok((Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("sort")), f6(Str::lit("u"), f30(v2), Adt5::C5)))), v1))
        }
        Adt1::C1(v3) => {
            let v4: (Adt7, Adt6) = f31(v1, v3)?;
            Ok((f32({
                let (h87, _) = v4;
                h87
            }), {
                let (_, h88) = v4;
                h88
            }))
        }
        Adt1::C2(v5, v6) => {
            Ok((f15(v5, v6)?, v1))
        }
        Adt1::C3(r121, r122, v9) => {
            let v7 = (*r121).clone();
            let v8 = (*r122).clone();
            let v10: (Adt5, Adt6) = f11(v7, v1)?;
            let v11: (Adt5, Adt6) = f11(v8, {
                let (_, h89) = v10;
                h89
            })?;
            Ok((f33({
                let (h90, _) = v10;
                h90
            }, {
                let (h91, _) = v11;
                h91
            }, v9), {
                let (_, h92) = v11;
                h92
            }))
        }
        Adt1::C4(r123, r124) => {
            let v12 = (*r123).clone();
            let v13 = (*r124).clone();
            let v14: (Adt5, Adt6) = f11(v12, v1)?;
            let v15: (Adt5, Adt6) = f11(v13, {
                let (_, h93) = v14;
                h93
            })?;
            Ok((Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("pi")), f6(Str::lit("b"), Adt5::C3(std::rc::Rc::new({
                let (h94, _) = v14;
                h94
            })), f6(Str::lit("v"), {
                let (h95, _) = v15;
                h95
            }, Adt5::C5))))), {
                let (_, h96) = v15;
                h96
            }))
        }
        Adt1::C5(r125, r126) => {
            let v16 = (*r125).clone();
            let v17 = (*r126).clone();
            let v18: (Adt5, Adt6) = f11(v16, v1)?;
            let v19: (Adt5, Adt6) = f11(v17, {
                let (_, h97) = v18;
                h97
            })?;
            Ok((Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("lam")), f6(Str::lit("b"), Adt5::C3(std::rc::Rc::new({
                let (h98, _) = v18;
                h98
            })), f6(Str::lit("v"), {
                let (h99, _) = v19;
                h99
            }, Adt5::C5))))), {
                let (_, h100) = v19;
                h100
            }))
        }
        Adt1::C6(v20, v21, v22, r127, r128, r129) => {
            let v23 = (*r127).clone();
            let v24 = (*r128).clone();
            let v25 = (*r129).clone();
            let v26: (Adt5, Adt6) = f11(v23, v1)?;
            let v27: (Adt5, Adt6) = f11(v24, {
                let (_, h101) = v26;
                h101
            })?;
            let v28: (u64, Adt6) = f12({
                let (_, h102) = v27;
                h102
            }, v20)?;
            let v29: (Adt5, Adt6) = f11(v25, {
                let (_, h103) = v28;
                h103
            })?;
            Ok((Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("let")), f6(Str::lit("b"), f34({
                let (h104, _) = v28;
                h104
            }, v21, v22, {
                let (h105, _) = v26;
                h105
            }), f6(Str::lit("x"), {
                let (h106, _) = v27;
                h106
            }, f6(Str::lit("v"), {
                let (h107, _) = v29;
                h107
            }, Adt5::C5)))))), {
                let (_, h108) = v29;
                h108
            }))
        }
        Adt1::C7(v30, r130) => {
            let v31 = (*r130).clone();
            let v32: (Adt5, Adt6) = f11(v31, v1)?;
            Ok((Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("nat")), f6(Str::lit("d"), Adt5::C2(v30), f6(Str::lit("t"), {
                let (h109, _) = v32;
                h109
            }, Adt5::C5))))), {
                let (_, h110) = v32;
                h110
            }))
        }
        Adt1::C8 => {
            Ok((Adt5::C5, v1))
        }
        Adt1::C9(r131, r132) => {
            let v33 = (*r131).clone();
            let v34 = (*r132).clone();
            let v35: (Adt5, Adt6) = f11(v33, v1)?;
            let v36: (Adt5, Adt6) = f11(v34, {
                let (_, h111) = v35;
                h111
            })?;
            Ok((Adt5::C6(std::rc::Rc::new({
                let (h112, _) = v35;
                h112
            }), std::rc::Rc::new({
                let (h113, _) = v36;
                h113
            })), {
                let (_, h114) = v36;
                h114
            }))
        }
        Adt1::C10(v37, v38, v39, r133, r134) => {
            let v40 = (*r133).clone();
            let v41 = (*r134).clone();
            let v42: (Adt5, Adt6) = f11(v40, v1)?;
            let v43: (u64, Adt6) = f12({
                let (_, h115) = v42;
                h115
            }, v37)?;
            let v44: (Adt5, Adt6) = f11(v41, {
                let (_, h116) = v43;
                h116
            })?;
            Ok((Adt5::C6(std::rc::Rc::new(f34({
                let (h117, _) = v43;
                h117
            }, v38, v39, {
                let (h118, _) = v42;
                h118
            })), std::rc::Rc::new({
                let (h119, _) = v44;
                h119
            })), {
                let (_, h120) = v44;
                h120
            }))
        }
    }
}

pub fn f12(v0: Adt6, v1: u64) -> R<(u64, Adt6)> {
    let m136 = f35(match v0.clone() {
        Adt6::C0(h135, _, _) => {
            h135
        }
    }, v1);
    match m136 {
        None => {
            f36(v0, v1)
        }
        Some(v2) => {
            Ok((v2, v0))
        }
    }
}

pub fn f13(v0: u64) -> Adt5 {
    Adt5::C1(false, f37(v0))
}

pub fn f14(v0: Option<u64>, v1: Adt6) -> R<(Adt5, Adt6)> {
    let m137 = v0;
    match m137 {
        None => {
            Ok((f24(Str::lit("goal")), v1))
        }
        Some(v2) => {
            let v3: (Adt7, Adt6) = f31(v1, v2)?;
            Ok(({
                let m139 = {
                    let (h138, _) = v3;
                    h138
                };
                match m139 {
                    Adt7::C0(v4) => {
                        f13(v4)
                    }
                    Adt7::C1(v5) => {
                        f13(v5)
                    }
                }
            }, {
                let (_, h140) = v3;
                h140
            }))
        }
    }
}

pub fn f15(v0: Adt3, v1: Adt2) -> R<Adt5> {
    let v2: Adt5 = if f38(v1.clone()) {
        Adt5::C5
    } else {
        f6(Str::lit("u"), Adt5::C3(std::rc::Rc::new(f30(v1))), Adt5::C5)
    };
    Ok(Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("global")), f6(Str::lit("g"), f39(v0)?, v2)))))
}

pub fn f16(v0: Adt5) -> Adt5 {
    Adt5::C4(std::rc::Rc::new(v0))
}

pub fn f17(v0: List<(u64, Bytes)>) -> List<u64> {
    f40(f41(Fn2::F42, List::<u64>::nil(), v0))
}

pub fn f18(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a141 = 0u64;
        let a142 = v1;
        nat_lt(a141, a142)
    } else {
        false
    } {
        let a143 = Bytes::lit(&[0x2d]);
        let a144 = f43(v1)?;
        Ok(append_bytes(a143, a144))
    } else {
        f43(v1)
    }
}

pub fn f19(v0: Bytes) -> R<Bytes> {
    let a150 = {
        let a148 = Bytes::lit(&[0x22]);
        let a149 = {
            let (_, h145) = {
                let (h146, _) = f44(Fn3::F45(v0.clone()), {
                    let a147 = v0;
                    length_bytes(a147)
                }, (0u64, Bytes::lit(&[])))?;
                h146
            };
            h145
        };
        append_bytes(a148, a149)
    };
    let a151 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a150, a151))
}

pub fn f20(v0: List<Bytes>) -> Bytes {
    let a154 = {
        let a152 = Bytes::lit(&[0x5b]);
        let a153 = f46(v0, Bytes::lit(&[0x2c]));
        append_bytes(a152, a153)
    };
    let a155 = Bytes::lit(&[0x5d]);
    append_bytes(a154, a155)
}

pub fn f21(v0: Adt5) -> R<List<Bytes>> {
    let m156 = v0;
    match m156 {
        Adt5::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt5::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt5::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt5::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt5::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt5::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt5::C6(r157, r158) => {
            let v7 = (*r157).clone();
            let v8 = (*r158).clone();
            Ok(List::cons(f4(v7)?, f21(v8)?))
        }
        Adt5::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f22(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f47(Fn4::F48, List::<(Bytes, Bytes)>::nil(), f49(v0)?);
    let v2: List<Bytes> = f50(Fn5::F51, List::<Bytes>::nil(), v1)?;
    let a161 = {
        let a159 = Bytes::lit(&[0x7b]);
        let a160 = f46(v2, Bytes::lit(&[0x2c]));
        append_bytes(a159, a160)
    };
    let a162 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a161, a162))
}

pub fn f23(v0: Adt5) -> R<List<(Bytes, Bytes)>> {
    let m163 = v0;
    match m163 {
        Adt5::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt5::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt5::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt5::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt5::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt5::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt5::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt5::C7(v9, r164, r165) => {
            let v10 = (*r164).clone();
            let v11 = (*r165).clone();
            Ok(List::cons((v9, f4(v10)?), f23(v11)?))
        }
    }
}

pub fn f24(v0: Str) -> Adt5 {
    Adt5::C2(f25(v0))
}

pub fn f25(v0: Str) -> Bytes {
    let a166 = v0;
    utf8_encode(a166)
}

pub fn f26(v0: Fn0, p170: (List<u64>, Adt6), p171: List<u64>) -> R<(List<u64>, Adt6)> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v1, v2) = m169;
                let m167 = v2;
                match m167.uncons() {
                    None => {
                        Err::<((List<u64>, Adt6), List<u64>), R<(List<u64>, Adt6)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<u64>, Adt6), List<u64>), R<(List<u64>, Adt6)>>(({
                            let c168: Fn0 = v0.clone();
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

pub fn f27(v0: (List<u64>, Adt6), v1: u64) -> R<(List<u64>, Adt6)> {
    let v2: (u64, Adt6) = f12({
        let (_, h172) = v0;
        h172
    }, v1)?;
    Ok((List::cons({
        let (h173, _) = v2;
        h173
    }, {
        let (h174, _) = v0;
        h174
    }), {
        let (_, h175) = v2;
        h175
    }))
}

pub fn f28(v0: Fn1, p179: Adt5, p180: List<u64>) -> Adt5 {
    let m178 = (p179, p180);
    {
        let mut m178 = m178;
        loop {
            let step = {
                let (v1, v2) = m178;
                let m176 = v2;
                match m176.uncons() {
                    None => {
                        Err::<(Adt5, List<u64>), Adt5>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt5, List<u64>), Adt5>(({
                            let c177: Fn1 = v0.clone();
                            c177.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt5, v1: u64) -> Adt5 {
    Adt5::C6(std::rc::Rc::new(f13(v1)), std::rc::Rc::new(v0))
}

pub fn f30(v0: Adt2) -> Adt5 {
    let m181 = v0;
    match m181 {
        Adt2::C0(v1) => {
            Adt5::C4(std::rc::Rc::new(f6(Str::lit("num"), f13(v1), Adt5::C5)))
        }
        Adt2::C1(v2) => {
            Adt5::C4(std::rc::Rc::new(f6(Str::lit("var"), Adt5::C2(v2), Adt5::C5)))
        }
        Adt2::C2(r182) => {
            let v3 = (*r182).clone();
            Adt5::C4(std::rc::Rc::new(f6(Str::lit("succ"), f30(v3), Adt5::C5)))
        }
        Adt2::C3(r183) => {
            let v4 = (*r183).clone();
            Adt5::C4(std::rc::Rc::new(f6(Str::lit("max"), Adt5::C3(std::rc::Rc::new(f30(v4))), Adt5::C5)))
        }
        Adt2::C4(r184, r185) => {
            let v5 = (*r184).clone();
            let v6 = (*r185).clone();
            Adt5::C4(std::rc::Rc::new(f6(Str::lit("imax"), Adt5::C3(std::rc::Rc::new(Adt5::C6(std::rc::Rc::new(f30(v5)), std::rc::Rc::new(Adt5::C6(std::rc::Rc::new(f30(v6)), std::rc::Rc::new(Adt5::C5)))))), Adt5::C5)))
        }
        Adt2::C5 => {
            Adt5::C5
        }
        Adt2::C6(r186, r187) => {
            let v7 = (*r186).clone();
            let v8 = (*r187).clone();
            Adt5::C6(std::rc::Rc::new(f30(v7)), std::rc::Rc::new(f30(v8)))
        }
    }
}

pub fn f31(v0: Adt6, v1: u64) -> R<(Adt7, Adt6)> {
    let m189 = f35(match v0.clone() {
        Adt6::C0(h188, _, _) => {
            h188
        }
    }, v1);
    match m189 {
        None => {
            if match v0.clone() {
                Adt6::C0(_, _, h190) => {
                    h190
                }
            } {
                Ok((Adt7::C1(v1), v0))
            } else {
                let v3: (u64, Adt6) = f36(v0, v1)?;
                Ok((Adt7::C0({
                    let (h191, _) = v3;
                    h191
                }), {
                    let (_, h192) = v3;
                    h192
                }))
            }
        }
        Some(v2) => {
            Ok((Adt7::C0(v2), v0))
        }
    }
}

pub fn f32(v0: Adt7) -> Adt5 {
    let m193 = v0;
    match m193 {
        Adt7::C0(v1) => {
            Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("local")), f6(Str::lit("id"), f13(v1), Adt5::C5))))
        }
        Adt7::C1(v2) => {
            Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("local")), f6(Str::lit("free"), f13(v2), Adt5::C5))))
        }
    }
}

pub fn f33(v0: Adt5, v1: Adt5, v2: List<u64>) -> Adt5 {
    let v5: Adt5 = {
        let m194 = v2.clone();
        match m194.uncons() {
            None => {
                Adt5::C5
            }
            Some((_, _)) => {
                f6(Str::lit("i"), Adt5::C3(std::rc::Rc::new(f52(v2))), Adt5::C5)
            }
        }
    };
    Adt5::C4(std::rc::Rc::new(f6(Str::lit("k"), f24(Str::lit("app")), f6(Str::lit("f"), v0, f6(Str::lit("a"), Adt5::C3(std::rc::Rc::new(v1)), v5)))))
}

pub fn f34(v0: u64, v1: u64, v2: Bytes, v3: Adt5) -> Adt5 {
    Adt5::C4(std::rc::Rc::new(f6(Str::lit("id"), f13(v0), f6(Str::lit("m"), f53(v1), f6(Str::lit("s"), Adt5::C2(v2), f6(Str::lit("t"), v3, Adt5::C5))))))
}

pub fn f35(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f54(Fn6::F55(v1), None::<u64>, v0)
}

pub fn f36(v0: Adt6, v1: u64) -> R<(u64, Adt6)> {
    Ok((match v0.clone() {
        Adt6::C0(_, h195, _) => {
            h195
        }
    }, Adt6::C0(List::cons((v1, match v0.clone() {
        Adt6::C0(_, h196, _) => {
            h196
        }
    }), match v0.clone() {
        Adt6::C0(h197, _, _) => {
            h197
        }
    }), {
        let a199 = match v0.clone() {
            Adt6::C0(_, h198, _) => {
                h198
            }
        };
        let a200 = 1u64;
        nat_add(a199, a200)?
    }, match v0 {
        Adt6::C0(_, _, h201) => {
            h201
        }
    })))
}

pub fn f37(v0: u64) -> u64 {
    let a202 = v0;
    let a203 = 9223372036854775807u64;
    if nat_le(a202, a203) {
        v0
    } else {
        9223372036854775807u64
    }
}

pub fn f38(v0: Adt2) -> bool {
    let m204 = v0;
    match m204 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_) => {
            false
        }
        Adt2::C3(_) => {
            false
        }
        Adt2::C4(_, _) => {
            false
        }
        Adt2::C5 => {
            true
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f39(v0: Adt3) -> R<Adt5> {
    let m205 = v0;
    match m205 {
        Adt3::C0(v1) => {
            Ok(Adt5::C4(std::rc::Rc::new(f6(Str::lit("kind"), f24(Str::lit("core")), f6(Str::lit("name"), Adt5::C2(f56(v1)), Adt5::C5)))))
        }
        Adt3::C1(_, v3, v4, v5, v6) => {
            Ok(Adt5::C4(std::rc::Rc::new(f6(Str::lit("kind"), f24(Str::lit("external")), f6(Str::lit("entry"), Adt5::C2(v3), f6(Str::lit("lean_module"), Adt5::C2(v4), f6(Str::lit("lean_name"), Adt5::C2(v5), f6(Str::lit("signature_sha256"), Adt5::C2(f57(v6)?), Adt5::C5))))))))
        }
        Adt3::C2(v7, v8, v9) => {
            Ok(Adt5::C4(std::rc::Rc::new(f6(Str::lit("kind"), f24(Str::lit("document")), f6(Str::lit("module"), Adt5::C2(v7), f6(Str::lit("component"), Adt5::C2(v8), f6(Str::lit("lean_name"), Adt5::C2(v9), Adt5::C5)))))))
        }
        Adt3::C3(_, v11, v12) => {
            Ok(Adt5::C4(std::rc::Rc::new(f6(Str::lit("kind"), f24(Str::lit("defined")), f6(Str::lit("entry"), Adt5::C2(v11), f6(Str::lit("signature_sha256"), Adt5::C2(f57(v12)?), Adt5::C5))))))
        }
    }
}

pub fn f40(v0: List<u64>) -> List<u64> {
    f58(Fn7::F59, List::<u64>::nil(), v0)
}

pub fn f41(v0: Fn2, p209: List<u64>, p210: List<(u64, Bytes)>) -> List<u64> {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v1, v2) = m208;
                let m206 = v2;
                match m206.uncons() {
                    None => {
                        Err::<(List<u64>, List<(u64, Bytes)>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<(u64, Bytes)>), List<u64>>(({
                            let c207: Fn2 = v0.clone();
                            c207.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: List<u64>, v1: (u64, Bytes)) -> List<u64> {
    List::cons({
        let (h211, _) = v1;
        h211
    }, v0)
}

pub fn f43(v0: u64) -> R<Bytes> {
    f60(f61(), 10u64, v0)
}

pub fn f44(v0: Fn3, p216: u64, p217: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m215 = (p216, p217);
    {
        let mut m215 = m215;
        loop {
            let step = {
                let (v1, v2) = m215;
                let m212 = v1;
                if m212 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m212 - 1;
                    let m214 = {
                        let c213: Fn3 = v0.clone();
                        c213.apply(v2.clone())?
                    };
                    match m214 {
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
                Ok(next) => { m215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f62(v0, v1)
}

pub fn f46(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m218 = f63(Fn8::F64(v1), None::<Bytes>, v0);
    match m218 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f47(v0: Fn4, p222: List<(Bytes, Bytes)>, p223: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m221 = (p222, p223);
    {
        let mut m221 = m221;
        loop {
            let step = {
                let (v1, v2) = m221;
                let m219 = v2;
                match m219.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c220: Fn4 = v0.clone();
                            c220.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f65(v0, v1)
}

pub fn f49(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f66(v0)
}

pub fn f50(v0: Fn5, p227: List<Bytes>, p228: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m226 = (p227, p228);
    {
        let mut m226 = m226;
        loop {
            let step = {
                let (v1, v2) = m226;
                let m224 = v2;
                match m224.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c225: Fn5 = v0.clone();
                            c225.apply(v1, v3)?
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

pub fn f51(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f67(v1)?, v0))
}

pub fn f52(v0: List<u64>) -> Adt5 {
    f28(Fn1::F68, Adt5::C5, f40(v0))
}

pub fn f53(v0: u64) -> Adt5 {
    Adt5::C2(f69(v0))
}

pub fn f54(v0: Fn6, p232: Option<u64>, p233: List<(u64, u64)>) -> Option<u64> {
    let m231 = (p232, p233);
    {
        let mut m231 = m231;
        loop {
            let step = {
                let (v1, v2) = m231;
                let m229 = v2;
                match m229.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c230: Fn6 = v0.clone();
                            c230.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m231 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let a235 = {
        let (h234, _) = v2;
        h234
    };
    let a236 = v0;
    if nat_eq(a235, a236) {
        Some({
            let (_, h237) = v2;
            h237
        })
    } else {
        v1
    }
}

pub fn f56(v0: Adt4) -> Bytes {
    let m238 = v0;
    match m238 {
        Adt4::C0 => {
            f25(Str::lit("eq"))
        }
        Adt4::C1 => {
            f25(Str::lit("and"))
        }
        Adt4::C2 => {
            f25(Str::lit("or"))
        }
        Adt4::C3 => {
            f25(Str::lit("not"))
        }
        Adt4::C4 => {
            f25(Str::lit("iff"))
        }
        Adt4::C5 => {
            f25(Str::lit("exists"))
        }
        Adt4::C6 => {
            f25(Str::lit("exists-unique"))
        }
    }
}

pub fn f57(v0: Bytes) -> R<Bytes> {
    let (_, h239) = {
        let (h240, _) = f44(Fn3::F70(v0.clone()), {
            let a241 = v0;
            length_bytes(a241)
        }, (0u64, Bytes::lit(&[])))?;
        h240
    };
    Ok(h239)
}

pub fn f58(v0: Fn7, p245: List<u64>, p246: List<u64>) -> List<u64> {
    let m244 = (p245, p246);
    {
        let mut m244 = m244;
        loop {
            let step = {
                let (v1, v2) = m244;
                let m242 = v2;
                match m242.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c243: Fn7 = v0.clone();
                            c243.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f60(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a247 = v2;
    let a248 = 0u64;
    if nat_eq(a247, a248) {
        Ok(f71(v0, 0u64))
    } else {
        let (_, h249) = {
            let (h250, _) = f44(Fn3::F72(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h250
        };
        Ok(h249)
    }
}

pub fn f61() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f62(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m254 = {
        let a252 = v0.clone();
        let a253 = {
            let (h251, _) = v1.clone();
            h251
        };
        index_bytes(a252, a253)
    };
    match m254 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a256 = {
                    let (h255, _) = v1.clone();
                    h255
                };
                let a257 = 1u64;
                nat_add(a256, a257)?
            }, {
                let a260 = {
                    let (_, h258) = v1;
                    h258
                };
                let a261 = f73(v0, {
                    let (h259, _) = v1;
                    h259
                }, v2)?;
                append_bytes(a260, a261)
            })))
        }
    }
}

pub fn f63(v0: Fn8, p265: Option<Bytes>, p266: List<Bytes>) -> Option<Bytes> {
    let m264 = (p265, p266);
    {
        let mut m264 = m264;
        loop {
            let step = {
                let (v1, v2) = m264;
                let m262 = v2;
                match m262.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c263: Fn8 = v0.clone();
                            c263.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m267 = v1;
    match m267 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a270 = {
                    let a268 = v3;
                    let a269 = v0;
                    append_bytes(a268, a269)
                };
                let a271 = v2;
                append_bytes(a270, a271)
            })
        }
    }
}

pub fn f65(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m272 = v0.clone();
    match m272.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a275 = {
                let (h273, _) = v2;
                h273
            };
            let a276 = {
                let (h274, _) = v1.clone();
                h274
            };
            if equal(a275, a276) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f66(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f74(f75(Fn9::F76, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m281 = {
        let (h277, _) = f77(Fn10::F78, {
            let a279 = {
                let a278 = v0;
                length_list(a278)
            };
            let a280 = 1u64;
            nat_add(a279, a280)?
        }, v1)?;
        h277
    };
    match m281.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f67(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a286 = {
        let a283 = f19({
            let (h282, _) = v0;
            h282
        })?;
        let a284 = Bytes::lit(&[0x3a]);
        append_bytes(a283, a284)
    };
    let a287 = {
        let (_, h285) = v0;
        h285
    };
    Ok(append_bytes(a286, a287))
}

pub fn f68(v0: Adt5, v1: u64) -> Adt5 {
    Adt5::C6(std::rc::Rc::new(f13(v1)), std::rc::Rc::new(v0))
}

pub fn f69(v0: u64) -> Bytes {
    let a288 = v0;
    let a289 = 1u64;
    if nat_eq(a288, a289) {
        f79(Str::lit("implicit"))
    } else {
        let a290 = v0;
        let a291 = 2u64;
        if nat_eq(a290, a291) {
            f79(Str::lit("instance"))
        } else {
            f79(Str::lit("explicit"))
        }
    }
}

pub fn f70(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f80(v0, v1)
}

pub fn f71(v0: Bytes, v1: u64) -> Bytes {
    let m295 = {
        let a292 = v0;
        let a293 = v1;
        let a294 = 1u64;
        slice_bytes(a292, a293, a294)
    };
    match m295 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f72(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f81(v1, v0, v2)
}

pub fn f73(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a340 = {
            let a316 = {
                let a304 = {
                    let a298 = {
                        let a296 = v3;
                        let a297 = 128u8;
                        fixed_u8::bit_and(a296, a297)
                    };
                    let a299 = 128u8;
                    if equal(a298, a299) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a305 = {
                    let a302 = {
                        let a300 = v3;
                        let a301 = 64u8;
                        fixed_u8::bit_and(a300, a301)
                    };
                    let a303 = 64u8;
                    if equal(a302, a303) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a304, a305)?
            };
            let a317 = {
                let a314 = {
                    let a308 = {
                        let a306 = v3;
                        let a307 = 32u8;
                        fixed_u8::bit_and(a306, a307)
                    };
                    let a309 = 32u8;
                    if equal(a308, a309) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a315 = {
                    let a312 = {
                        let a310 = v3;
                        let a311 = 16u8;
                        fixed_u8::bit_and(a310, a311)
                    };
                    let a313 = 16u8;
                    if equal(a312, a313) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a314, a315)?
            };
            nat_add(a316, a317)?
        };
        let a341 = {
            let a338 = {
                let a326 = {
                    let a320 = {
                        let a318 = v3;
                        let a319 = 8u8;
                        fixed_u8::bit_and(a318, a319)
                    };
                    let a321 = 8u8;
                    if equal(a320, a321) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a327 = {
                    let a324 = {
                        let a322 = v3;
                        let a323 = 4u8;
                        fixed_u8::bit_and(a322, a323)
                    };
                    let a325 = 4u8;
                    if equal(a324, a325) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a326, a327)?
            };
            let a339 = {
                let a336 = {
                    let a330 = {
                        let a328 = v3;
                        let a329 = 2u8;
                        fixed_u8::bit_and(a328, a329)
                    };
                    let a331 = 2u8;
                    if equal(a330, a331) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a337 = {
                    let a334 = {
                        let a332 = v3;
                        let a333 = 1u8;
                        fixed_u8::bit_and(a332, a333)
                    };
                    let a335 = 1u8;
                    if equal(a334, a335) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a336, a337)?
            };
            nat_add(a338, a339)?
        };
        nat_add(a340, a341)?
    };
    let a342 = v4;
    let a343 = 34u64;
    if nat_eq(a342, a343) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a344 = v4;
        let a345 = 92u64;
        if nat_eq(a344, a345) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a346 = v4;
            let a347 = 8u64;
            if nat_eq(a346, a347) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a348 = v4;
                let a349 = 12u64;
                if nat_eq(a348, a349) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a350 = v4;
                    let a351 = 10u64;
                    if nat_eq(a350, a351) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a352 = v4;
                        let a353 = 13u64;
                        if nat_eq(a352, a353) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a354 = v4;
                            let a355 = 9u64;
                            if nat_eq(a354, a355) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a356 = v4;
                                let a357 = 32u64;
                                if nat_lt(a356, a357) {
                                    let a358 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a359 = f82(v2)?;
                                    Ok(append_bytes(a358, a359))
                                } else {
                                    Ok(f83(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f84(Fn11::F85, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f75(v0: Fn9, p363: List<List<(Bytes, Bytes)>>, p364: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m362 = (p363, p364);
    {
        let mut m362 = m362;
        loop {
            let step = {
                let (v1, v2) = m362;
                let m360 = v2;
                match m360.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c361: Fn9 = v0.clone();
                            c361.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m362 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f77(v0: Fn10, p369: u64, p370: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m368 = (p369, p370);
    {
        let mut m368 = m368;
        loop {
            let step = {
                let (v1, v2) = m368;
                let m365 = v1;
                if m365 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m365 - 1;
                    let m367 = {
                        let c366: Fn10 = v0.clone();
                        c366.apply(v2.clone())?
                    };
                    match m367 {
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
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f86(v0)
}

pub fn f79(v0: Str) -> Bytes {
    let a371 = v0;
    utf8_encode(a371)
}

pub fn f80(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m375 = {
        let a373 = v0;
        let a374 = {
            let (h372, _) = v1.clone();
            h372
        };
        index_bytes(a373, a374)
    };
    match m375 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a377 = {
                    let (h376, _) = v1;
                    h376
                };
                let a378 = 1u64;
                nat_add(a377, a378)?
            }, {
                let a380 = {
                    let (_, h379) = v1;
                    h379
                };
                let a381 = f82(v2)?;
                append_bytes(a380, a381)
            })))
        }
    }
}

pub fn f81(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h382, _) = v2;
        h382
    };
    let a383 = v3;
    let a384 = 0u64;
    if nat_eq(a383, a384) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a385 = v3;
            let a386 = v1;
            let a387 = 0u64;
            nat_quot(a385, a386, a387)
        }, {
            let a392 = f71(v0, {
                let a388 = v3;
                let a389 = v1;
                let a390 = 0u64;
                nat_rem(a388, a389, a390)
            });
            let a393 = {
                let (_, h391) = v2;
                h391
            };
            append_bytes(a392, a393)
        }))
    }
}

pub fn f82(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a438 = {
            let a414 = {
                let a402 = {
                    let a396 = {
                        let a394 = v1;
                        let a395 = 128u8;
                        fixed_u8::bit_and(a394, a395)
                    };
                    let a397 = 128u8;
                    if equal(a396, a397) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a403 = {
                    let a400 = {
                        let a398 = v1;
                        let a399 = 64u8;
                        fixed_u8::bit_and(a398, a399)
                    };
                    let a401 = 64u8;
                    if equal(a400, a401) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a402, a403)?
            };
            let a415 = {
                let a412 = {
                    let a406 = {
                        let a404 = v1;
                        let a405 = 32u8;
                        fixed_u8::bit_and(a404, a405)
                    };
                    let a407 = 32u8;
                    if equal(a406, a407) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a413 = {
                    let a410 = {
                        let a408 = v1;
                        let a409 = 16u8;
                        fixed_u8::bit_and(a408, a409)
                    };
                    let a411 = 16u8;
                    if equal(a410, a411) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a412, a413)?
            };
            nat_add(a414, a415)?
        };
        let a439 = {
            let a436 = {
                let a424 = {
                    let a418 = {
                        let a416 = v1;
                        let a417 = 8u8;
                        fixed_u8::bit_and(a416, a417)
                    };
                    let a419 = 8u8;
                    if equal(a418, a419) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a425 = {
                    let a422 = {
                        let a420 = v1;
                        let a421 = 4u8;
                        fixed_u8::bit_and(a420, a421)
                    };
                    let a423 = 4u8;
                    if equal(a422, a423) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a424, a425)?
            };
            let a437 = {
                let a434 = {
                    let a428 = {
                        let a426 = v1;
                        let a427 = 2u8;
                        fixed_u8::bit_and(a426, a427)
                    };
                    let a429 = 2u8;
                    if equal(a428, a429) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a435 = {
                    let a432 = {
                        let a430 = v1;
                        let a431 = 1u8;
                        fixed_u8::bit_and(a430, a431)
                    };
                    let a433 = 1u8;
                    if equal(a432, a433) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a434, a435)?
            };
            nat_add(a436, a437)?
        };
        nat_add(a438, a439)?
    };
    let a446 = f71(f87(), {
        let a440 = v2;
        let a441 = 16u64;
        let a442 = 0u64;
        nat_quot(a440, a441, a442)
    });
    let a447 = f71(f87(), {
        let a443 = v2;
        let a444 = 16u64;
        let a445 = 0u64;
        nat_rem(a443, a444, a445)
    });
    Ok(append_bytes(a446, a447))
}

pub fn f83(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m451 = {
        let a448 = v0;
        let a449 = v1;
        let a450 = v2;
        slice_bytes(a448, a449, a450)
    };
    match m451 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f84(v0: Fn11, p455: List<List<(Bytes, Bytes)>>, p456: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m454 = (p455, p456);
    {
        let mut m454 = m454;
        loop {
            let step = {
                let (v1, v2) = m454;
                let m452 = v2;
                match m452.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c453: Fn11 = v0.clone();
                            c453.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f86(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a458 = {
        let a457 = v0.clone();
        length_list(a457)
    };
    let a459 = 1u64;
    if nat_le(a458, a459) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f88(v0)?))
    }
}

pub fn f87() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f88(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f74({
        let (_, h460) = {
            let (h461, _) = f89(Fn12::F90, {
                let a463 = {
                    let a462 = v0.clone();
                    length_list(a462)
                };
                let a464 = 1u64;
                nat_add(a463, a464)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h461
        };
        h460
    }))
}

pub fn f89(v0: Fn12, p469: u64, p470: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m468 = (p469, p470);
    {
        let mut m468 = m468;
        loop {
            let step = {
                let (v1, v2) = m468;
                let m465 = v1;
                if m465 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m465 - 1;
                    let m467 = {
                        let c466: Fn12 = v0.clone();
                        c466.apply(v2.clone())?
                    };
                    match m467 {
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
                Ok(next) => { m468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f91(v0)
}

pub fn f91(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h471, _) = v0;
        h471
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h472) = v0;
        h472
    };
    let m473 = v1;
    match m473.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m474 = v4;
            match m474.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f92(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f92(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a479 = {
            let a477 = {
                let a475 = v0.clone();
                length_list(a475)
            };
            let a478 = {
                let a476 = v1.clone();
                length_list(a476)
            };
            nat_add(a477, a478)?
        };
        let a480 = 1u64;
        nat_add(a479, a480)?
    };
    Ok(f93({
        let (_, h481) = {
            let (_, h482) = {
                let (h483, _) = f94(Fn13::F95, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h483
            };
            h482
        };
        h481
    }))
}

pub fn f93(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f47(Fn4::F96, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f94(v0: Fn13, p488: u64, p489: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m487 = (p488, p489);
    {
        let mut m487 = m487;
        loop {
            let step = {
                let (v1, v2) = m487;
                let m484 = v1;
                if m484 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m484 - 1;
                    let m486 = {
                        let c485: Fn13 = v0.clone();
                        c485.apply(v2.clone())
                    };
                    match m486 {
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
                Ok(next) => { m487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f97(v0)
}

pub fn f96(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f97(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h490, _) = v0.clone();
        h490
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h491, _) = {
            let (_, h492) = v0.clone();
            h492
        };
        h491
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h493) = {
            let (_, h494) = v0;
            h494
        };
        h493
    };
    let m495 = v1.clone();
    match m495.uncons() {
        None => {
            let m496 = v2;
            match m496.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m497 = v2.clone();
            match m497.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f98({
                        let (h498, _) = v8.clone();
                        h498
                    }, {
                        let (h499, _) = v6.clone();
                        h499
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

pub fn f98(v0: Bytes, v1: Bytes) -> bool {
    let a504 = {
        let a500 = v0;
        let a501 = v1;
        compare_bytes(a500, a501)
    };
    let a505 = {
        let a502 = Bytes::lit(&[]);
        let a503 = Bytes::lit(&[0x01]);
        compare_bytes(a502, a503)
    };
    equal(a504, a505)
}
