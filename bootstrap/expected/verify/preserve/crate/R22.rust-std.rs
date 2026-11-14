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
    C0(Adt1, u64, u64, u64, u64, u64, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, u64, u64),
    C1(List<Adt4>, u64, u64),
    C2(Bytes, u64, u64),
    C3(Bytes, Bytes, u64, u64),
    C4(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, u64, u64, u64, u64, List<u64>),
    C5(List<u64>, u64, u64, std::rc::Rc<Adt3>),
    C6(List<u64>, u64, u64, std::rc::Rc<Adt3>),
    C7(List<u64>, u64, u64, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C8(std::rc::Rc<Adt3>, u64, u64),
    C9,
    C10(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, Bytes, Option<Adt6>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, List<Adt0>, u64, List<List<Adt2>>, u64, u64, u64, List<Adt8>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(List<List<Adt8>>, List<Bytes>, List<Adt10>, Option<Adt5>, Adt11),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Adt3, u64, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Result<Adt10, Adt5>, Adt11, List<Adt13>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, List<Adt15>, Option<Adt10>, Option<Adt5>, Adt11, List<Adt13>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, u64),
    C1(u64, u64, Option<u64>),
    C2(u64, u64, Adt10, List<u64>, u64, u64, Option<u64>),
    C3(u64),
    C4(u64, u64),
    C5(List<u64>, u64, u64, u64),
    C6(u64),
    C7(Adt10, u64, List<u64>, List<Adt10>, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Adt11, List<Adt2>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64, Bytes, bool),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(Adt7, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn0::F2(k0, k1) => f2(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F6,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt10) -> List<Adt3> {
        match self {
            Fn1::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F12(Adt10),
}

impl Fn2 {
    pub fn apply(&self, p0: bool, p1: Adt10) -> bool {
        match self {
            Fn2::F12(k0) => f12(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F14(Adt7),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt14) -> R<Option<Adt14>> {
        match self {
            Fn3::F14(k0) => f14(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F25(List<Adt8>),
}

impl Fn4 {
    pub fn apply(&self, p0: (List<List<Adt8>>, List<Bytes>), p1: Adt13) -> (List<List<Adt8>>, List<Bytes>) {
        match self {
            Fn4::F25(k0) => f25(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F28,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt13>, p1: Adt13) -> List<Adt13> {
        match self {
            Fn5::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F30(Adt7),
    F96(Bytes, u64, Bytes, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn6::F30(k0) => f30(k0.clone(), p0),
            Fn6::F96(k0, k1, k2, k3) => f96(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F38(List<Adt8>, Adt13),
}

impl Fn7 {
    pub fn apply(&self, p0: (bool, (List<List<Adt8>>, List<Bytes>)), p1: u64) -> (bool, (List<List<Adt8>>, List<Bytes>)) {
        match self {
            Fn7::F38(k0, k1) => f38(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F42(u64),
}

impl Fn8 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn8::F42(k0) => f42(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F45(u64),
}

impl Fn9 {
    pub fn apply(&self, p0: bool, p1: Adt8) -> bool {
        match self {
            Fn9::F45(k0) => f45(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F68(u64, u64),
}

impl Fn10 {
    pub fn apply(&self, p0: (bool, List<Adt8>), p1: Adt8) -> (bool, List<Adt8>) {
        match self {
            Fn10::F68(k0, k1) => f68(*k0, *k1, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F70,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt8>, p1: Adt8) -> List<Adt8> {
        match self {
            Fn11::F70 => f70(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F72,
}

impl Fn12 {
    pub fn apply(&self, p0: Bytes, p1: Adt8) -> Bytes {
        match self {
            Fn12::F72 => f72(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F74(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn13::F74(k0) => f74(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F86,
}

impl Fn14 {
    pub fn apply(&self, p0: Bytes, p1: Adt10) -> Bytes {
        match self {
            Fn14::F86 => f86(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F88,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Adt10>, p1: Adt10) -> List<Adt10> {
        match self {
            Fn15::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F90,
}

impl Fn16 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn16::F90 => f90(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F93(u64, Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn17::F93(k0, k1) => f93(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F95(Adt7),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt17) -> R<Option<Adt17>> {
        match self {
            Fn18::F95(k0) => f95(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F100(u64, u64, u64),
}

impl Fn19 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn19::F100(k0, k1, k2) => f100(*k0, *k1, *k2, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F102(Adt2, bool),
}

impl Fn20 {
    pub fn apply(&self, p0: bool, p1: Adt2) -> bool {
        match self {
            Fn20::F102(k0, k1) => f102(k0.clone(), *k1, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F104,
    F112,
}

impl Fn21 {
    pub fn apply(&self, p0: List<u64>, p1: Adt2) -> List<u64> {
        match self {
            Fn21::F104 => f104(p0, p1),
            Fn21::F112 => f112(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F106,
}

impl Fn22 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> Bytes {
        match self {
            Fn22::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F108,
}

impl Fn23 {
    pub fn apply(&self, p0: Adt3, p1: Adt10) -> Adt3 {
        match self {
            Fn23::F108 => f108(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F116(u64),
}

impl Fn24 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt2) -> List<Adt4> {
        match self {
            Fn24::F116(k0) => f116(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F118,
}

impl Fn25 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn25::F118 => f118(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F121(Adt7),
}

impl Fn26 {
    pub fn apply(&self, p0: Adt18) -> R<Option<Adt18>> {
        match self {
            Fn26::F121(k0) => f121(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F123(u64),
}

impl Fn27 {
    pub fn apply(&self, p0: (bool, List<u64>), p1: u64) -> (bool, List<u64>) {
        match self {
            Fn27::F123(k0) => f123(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F125(u64),
}

impl Fn28 {
    pub fn apply(&self, p0: Option<u64>, p1: Adt8) -> Option<u64> {
        match self {
            Fn28::F125(k0) => f125(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F127,
}

impl Fn29 {
    pub fn apply(&self, p0: Bytes, p1: Adt4) -> Bytes {
        match self {
            Fn29::F127 => f127(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>, v2: u64, v3: u64, v4: List<List<Adt2>>, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Result<List<Adt3>, Adt5>> {
    let v10: Adt7 = Adt7::C0(v0.clone(), v1, v3, v4, v6, v7, v5, List::<Adt8>::nil());
    let v11: Adt9 = {
        let (h1, _) = f1(Fn0::F2(v10, v2), {
            let a10 = {
                let a8 = {
                    let a2 = v6;
                    let a3 = 2u64;
                    nat_add(a2, a3)?
                };
                let a9 = {
                    let a6 = {
                        let a4 = v3;
                        let a5 = v2;
                        nat_sub(a4, a5)
                    };
                    let a7 = 2u64;
                    nat_add(a6, a7)?
                };
                nat_mul(a8, a9)?
            };
            let a11 = 2u64;
            nat_add(a10, a11)?
        }, Adt9::C0(List::cons(List::<Adt8>::nil(), List::<List<Adt8>>::nil()), List::cons(f3(Str::lit("")), List::<Bytes>::nil()), List::<Adt10>::nil(), None::<Adt5>, Adt11::C0(v8, v9, List::<u64>::nil())))?;
        h1
    };
    let a14 = {
        let a13 = match v11.clone() {
            Adt9::C0(_, _, h12, _, _) => {
                h12
            }
        };
        length_list(a13)
    };
    let a15 = 0u64;
    if nat_eq(a14, a15) {
        let m17 = match v11 {
            Adt9::C0(_, _, _, h16, _) => {
                h16
            }
        };
        match m17 {
            None => {
                Ok(Err::<List<Adt3>, Adt5>(f4(f3(Str::lit("LLI9001")), f3(Str::lit("phase parse: the token lattice produced neither a cover nor a failure")), Adt6::C0(v0, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64))))
            }
            Some(v12) => {
                Ok(Err::<List<Adt3>, Adt5>(v12))
            }
        }
    } else {
        Ok(Ok::<List<Adt3>, Adt5>(f5(Fn1::F6, List::<Adt3>::nil(), match v11 {
            Adt9::C0(_, _, h18, _, _) => {
                h18
            }
        })))
    }
}

pub fn f1(v0: Fn0, p23: u64, p24: Adt9) -> R<(Adt9, bool)> {
    let m22 = (p23, p24);
    {
        let mut m22 = m22;
        loop {
            let step = {
                let (v1, v2) = m22;
                let m19 = v1;
                if m19 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m19 - 1;
                    let m21 = {
                        let c20: Fn0 = v0.clone();
                        c20.apply(v2.clone())?
                    };
                    match m21 {
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
                Ok(next) => { m22 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: Adt7, v1: u64, v2: Adt9) -> R<Option<Adt9>> {
    f7(v0, v1, v2)
}

pub fn f3(v0: Str) -> Bytes {
    let a25 = v0;
    utf8_encode(a25)
}

pub fn f4(v0: Bytes, v1: Bytes, v2: Adt6) -> Adt5 {
    f8(v0, v1, Some(v2))
}

pub fn f5(v0: Fn1, p29: List<Adt3>, p30: List<Adt10>) -> List<Adt3> {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v1, v2) = m28;
                let m26 = v2;
                match m26.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt10>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt10>), List<Adt3>>(({
                            let c27: Fn1 = v0.clone();
                            c27.apply(v1, v3)
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

pub fn f6(v0: List<Adt3>, v1: Adt10) -> List<Adt3> {
    let a32 = v0;
    let a33 = List::cons(match v1 {
        Adt10::C0(h31, _, _, _) => {
            h31
        }
    }, List::<Adt3>::nil());
    append_list(a32, a33)
}

pub fn f7(v0: Adt7, v1: u64, v2: Adt9) -> R<Option<Adt9>> {
    let m35 = match v2.clone() {
        Adt9::C0(h34, _, _, _, _) => {
            h34
        }
    };
    match m35.uncons() {
        None => {
            Ok(None::<Adt9>)
        }
        Some((v3, v4)) => {
            let v5: Adt12 = f9(Adt7::C0(match v0.clone() {
                Adt7::C0(h36, _, _, _, _, _, _, _) => {
                    h36
                }
            }, match v0.clone() {
                Adt7::C0(_, h37, _, _, _, _, _, _) => {
                    h37
                }
            }, match v0.clone() {
                Adt7::C0(_, _, h38, _, _, _, _, _) => {
                    h38
                }
            }, match v0.clone() {
                Adt7::C0(_, _, _, h39, _, _, _, _) => {
                    h39
                }
            }, match v0.clone() {
                Adt7::C0(_, _, _, _, h40, _, _, _) => {
                    h40
                }
            }, match v0.clone() {
                Adt7::C0(_, _, _, _, _, h41, _, _) => {
                    h41
                }
            }, match v0 {
                Adt7::C0(_, _, _, _, _, _, h42, _) => {
                    h42
                }
            }, v3.clone()), v1, match v2.clone() {
                Adt9::C0(_, _, _, _, h43) => {
                    h43
                }
            })?;
            let v6: (List<List<Adt8>>, List<Bytes>) = f10(v3, match v5.clone() {
                Adt12::C0(_, _, h44) => {
                    h44
                }
            }, v4, match v2.clone() {
                Adt9::C0(_, h45, _, _, _) => {
                    h45
                }
            });
            Ok(Some({
                let m47 = match v5.clone() {
                    Adt12::C0(h46, _, _) => {
                        h46
                    }
                };
                match m47 {
                    Ok(v7) => {
                        Adt9::C0({
                            let (h48, _) = v6;
                            h48
                        }, {
                            let (_, h49) = v6;
                            h49
                        }, if f11(Fn2::F12(v7.clone()), false, match v2.clone() {
                            Adt9::C0(_, _, h50, _, _) => {
                                h50
                            }
                        }) {
                            match v2.clone() {
                                Adt9::C0(_, _, h51, _, _) => {
                                    h51
                                }
                            }
                        } else {
                            let a53 = match v2.clone() {
                                Adt9::C0(_, _, h52, _, _) => {
                                    h52
                                }
                            };
                            let a54 = List::cons(v7, List::<Adt10>::nil());
                            append_list(a53, a54)
                        }, match v2 {
                            Adt9::C0(_, _, _, h55, _) => {
                                h55
                            }
                        }, match v5 {
                            Adt12::C0(_, h56, _) => {
                                h56
                            }
                        })
                    }
                    Err(v8) => {
                        Adt9::C0({
                            let (h57, _) = v6;
                            h57
                        }, {
                            let (_, h58) = v6;
                            h58
                        }, match v2.clone() {
                            Adt9::C0(_, _, h59, _, _) => {
                                h59
                            }
                        }, {
                            let m61 = match v2.clone() {
                                Adt9::C0(_, _, _, h60, _) => {
                                    h60
                                }
                            };
                            match m61 {
                                None => {
                                    Some(v8)
                                }
                                Some(_) => {
                                    match v2 {
                                        Adt9::C0(_, _, _, h62, _) => {
                                            h62
                                        }
                                    }
                                }
                            }
                        }, match v5 {
                            Adt12::C0(_, h63, _) => {
                                h63
                            }
                        })
                    }
                }
            }))
        }
    }
}

pub fn f8(v0: Bytes, v1: Bytes, v2: Option<Adt6>) -> Adt5 {
    Adt5::C0(v0, v1, v2, None::<Bytes>)
}

pub fn f9(v0: Adt7, v1: u64, v2: Adt11) -> R<Adt12> {
    let v3: Adt14 = {
        let (h64, _) = f13(Fn3::F14(v0.clone()), {
            let a70 = {
                let a68 = {
                    let a66 = match v0.clone() {
                        Adt7::C0(_, _, h65, _, _, _, _, _) => {
                            h65
                        }
                    };
                    let a67 = v1;
                    nat_sub(a66, a67)
                };
                let a69 = 4u64;
                nat_add(a68, a69)?
            };
            let a71 = 16u64;
            nat_mul(a70, a71)?
        }, Adt14::C0(v1, List::cons(Adt15::C0(0u64, 1u64), List::<Adt15>::nil()), None::<Adt10>, None::<Adt5>, v2, List::<Adt13>::nil()))?;
        h64
    };
    let v4: List<Adt13> = f15(match v3.clone() {
        Adt14::C0(_, _, _, _, _, h72) => {
            h72
        }
    });
    let m74 = match v3.clone() {
        Adt14::C0(_, _, _, h73, _, _) => {
            h73
        }
    };
    match m74 {
        None => {
            let v6: u64 = f16(v0.clone(), match v3.clone() {
                Adt14::C0(h76, _, _, _, _, _) => {
                    h76
                }
            })?;
            let a78 = v6;
            let a79 = match v0.clone() {
                Adt7::C0(_, _, h77, _, _, _, _, _) => {
                    h77
                }
            };
            if nat_lt(a78, a79) {
                let v7: Adt0 = f17(v0.clone(), v6);
                let v8: u64 = f18(v7.clone());
                let v9: bool = {
                    let a80 = v8;
                    let a81 = 1u64;
                    if if nat_eq(a80, a81) {
                        true
                    } else {
                        let a82 = v8;
                        let a83 = 2u64;
                        if nat_eq(a82, a83) {
                            true
                        } else {
                            let a84 = v8;
                            let a85 = 3u64;
                            if nat_eq(a84, a85) {
                                true
                            } else {
                                let a86 = v8;
                                let a87 = 6u64;
                                nat_eq(a86, a87)
                            }
                        }
                    } {
                        true
                    } else {
                        f19(match v7.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, h88) => {
                                h88
                            }
                        }, f3(Str::lit(",")))
                    }
                };
                let v10: Adt16 = if v9 {
                    Adt16::C0(match v3 {
                        Adt14::C0(_, _, _, _, h89, _) => {
                            h89
                        }
                    }, List::<Adt2>::nil(), None::<Adt5>)
                } else {
                    f20(v0.clone(), match v3 {
                        Adt14::C0(_, _, _, _, h90, _) => {
                            h90
                        }
                    }, v6)?
                };
                let v11: Adt5 = f4(f3(Str::lit("LLP2004")), f21(f21(f3(Str::lit("unexpected `")), match v7.clone() {
                    Adt0::C0(_, _, _, _, _, _, _, h91) => {
                        h91
                    }
                }), f3(Str::lit("` after the expression"))), f22(v0.clone(), v6));
                let m93 = match v10.clone() {
                    Adt16::C0(_, _, h92) => {
                        h92
                    }
                };
                match m93 {
                    None => {
                        Ok(Adt12::C0(Err::<Adt10, Adt5>(if if v9 {
                            true
                        } else {
                            let a97 = 0u64;
                            let a98 = {
                                let a96 = match v10.clone() {
                                    Adt16::C0(_, h95, _) => {
                                        h95
                                    }
                                };
                                length_list(a96)
                            };
                            nat_lt(a97, a98)
                        } {
                            v11
                        } else {
                            f4(f3(Str::lit("LLL1004")), f21(f21(f3(Str::lit("unknown atom `")), match v7 {
                                Adt0::C0(_, _, _, _, _, _, _, h99) => {
                                    h99
                                }
                            }), f3(Str::lit("` in a mathematical island"))), f22(v0, v6))
                        }), match v10 {
                            Adt16::C0(h100, _, _) => {
                                h100
                            }
                        }, v4))
                    }
                    Some(v12) => {
                        Ok(Adt12::C0(Err::<Adt10, Adt5>(v12), match v10 {
                            Adt16::C0(h94, _, _) => {
                                h94
                            }
                        }, v4))
                    }
                }
            } else {
                Ok(Adt12::C0(Ok::<Adt10, Adt5>(f23(v3.clone())), match v3 {
                    Adt14::C0(_, _, _, _, h101, _) => {
                        h101
                    }
                }, v4))
            }
        }
        Some(v5) => {
            Ok(Adt12::C0(Err::<Adt10, Adt5>(v5), match v3 {
                Adt14::C0(_, _, _, _, h75, _) => {
                    h75
                }
            }, v4))
        }
    }
}

pub fn f10(v0: List<Adt8>, v1: List<Adt13>, v2: List<List<Adt8>>, v3: List<Bytes>) -> (List<List<Adt8>>, List<Bytes>) {
    f24(Fn4::F25(v0), (v2, v3), v1)
}

pub fn f11(v0: Fn2, p105: bool, p106: List<Adt10>) -> bool {
    let m104 = (p105, p106);
    {
        let mut m104 = m104;
        loop {
            let step = {
                let (v1, v2) = m104;
                let m102 = v2;
                match m102.uncons() {
                    None => {
                        Err::<(bool, List<Adt10>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt10>), bool>(({
                            let c103: Fn2 = v0.clone();
                            c103.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt10, v1: bool, v2: Adt10) -> bool {
    if v1 {
        true
    } else {
        f19(match v2 {
            Adt10::C0(_, _, _, h107) => {
                h107
            }
        }, match v0 {
            Adt10::C0(_, _, _, h108) => {
                h108
            }
        })
    }
}

pub fn f13(v0: Fn3, p113: u64, p114: Adt14) -> R<(Adt14, bool)> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m109 = v1;
                if m109 == 0 {
                    Err::<(u64, Adt14), R<(Adt14, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m109 - 1;
                    let m111 = {
                        let c110: Fn3 = v0.clone();
                        c110.apply(v2.clone())?
                    };
                    match m111 {
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
                Ok(next) => { m112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Adt7, v1: Adt14) -> R<Option<Adt14>> {
    f26(v0, v1)
}

pub fn f15(v0: List<Adt13>) -> List<Adt13> {
    f27(Fn5::F28, List::<Adt13>::nil(), v0)
}

pub fn f16(v0: Adt7, v1: u64) -> R<u64> {
    let (h115, _) = f29(Fn6::F30(v0.clone()), {
        let a118 = {
            let a117 = match v0 {
                Adt7::C0(_, h116, _, _, _, _, _, _) => {
                    h116
                }
            };
            length_list(a117)
        };
        let a119 = 1u64;
        nat_add(a118, a119)?
    }, v1)?;
    Ok(h115)
}

pub fn f17(v0: Adt7, v1: u64) -> Adt0 {
    let m120 = f31(v0, v1);
    match m120 {
        None => {
            Adt0::C0(Adt1::C6, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64, f3(Str::lit("")))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f18(v0: Adt0) -> u64 {
    let m122 = match v0 {
        Adt0::C0(h121, _, _, _, _, _, _, _) => {
            h121
        }
    };
    match m122 {
        Adt1::C0 => {
            0u64
        }
        Adt1::C1 => {
            1u64
        }
        Adt1::C2 => {
            2u64
        }
        Adt1::C3 => {
            3u64
        }
        Adt1::C4 => {
            4u64
        }
        Adt1::C5 => {
            5u64
        }
        Adt1::C6 => {
            6u64
        }
    }
}

pub fn f19(v0: Bytes, v1: Bytes) -> bool {
    let a123 = v0;
    let a124 = v1;
    equal(a123, a124)
}

pub fn f20(v0: Adt7, v1: Adt11, v2: u64) -> R<Adt16> {
    let v4: List<Adt2> = {
        let m128 = {
            let a126 = match v0.clone() {
                Adt7::C0(_, _, _, h125, _, _, _, _) => {
                    h125
                }
            };
            let a127 = v2;
            index_list(a126, a127)
        };
        match m128 {
            None => {
                List::<Adt2>::nil()
            }
            Some(v3) => {
                v3
            }
        }
    };
    if f32(v2, match v1.clone() {
        Adt11::C0(_, _, h129) => {
            h129
        }
    }) {
        Ok(Adt16::C0(v1, v4, None::<Adt5>))
    } else {
        let v5: u64 = {
            let a132 = match v1.clone() {
                Adt11::C0(_, h130, _) => {
                    h130
                }
            };
            let a133 = {
                let a131 = v4.clone();
                length_list(a131)
            };
            nat_add(a132, a133)?
        };
        let a135 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, h134, _) => {
                h134
            }
        };
        let a136 = v5;
        if nat_lt(a135, a136) {
            Ok(Adt16::C0(Adt11::C0(match v1.clone() {
                Adt11::C0(h137, _, _) => {
                    h137
                }
            }, v5, match v1 {
                Adt11::C0(_, _, h138) => {
                    h138
                }
            }), List::<Adt2>::nil(), Some(f8(f3(Str::lit("LLS8002")), f21(f21(f21(f21(f3(Str::lit("max_token_lattice_edges exceeded in phase lexical resolution: configured ")), f33(match v0 {
                Adt7::C0(_, _, _, _, _, _, h139, _) => {
                    h139
                }
            })), f3(Str::lit(", observed "))), f33(v5)), f3(Str::lit(" distinct lattice edges"))), None::<Adt6>))))
        } else {
            Ok(Adt16::C0(Adt11::C0(match v1.clone() {
                Adt11::C0(h140, _, _) => {
                    h140
                }
            }, v5, List::cons(v2, match v1 {
                Adt11::C0(_, _, h141) => {
                    h141
                }
            })), v4, None::<Adt5>))
        }
    }
}

pub fn f21(v0: Bytes, v1: Bytes) -> Bytes {
    let a142 = v0;
    let a143 = v1;
    append_bytes(a142, a143)
}

pub fn f22(v0: Adt7, v1: u64) -> Adt6 {
    f34(match v0.clone() {
        Adt7::C0(h144, _, _, _, _, _, _, _) => {
            h144
        }
    }, f17(v0, v1))
}

pub fn f23(v0: Adt14) -> Adt10 {
    let m146 = match v0 {
        Adt14::C0(_, _, h145, _, _, _) => {
            h145
        }
    };
    match m146 {
        None => {
            f35(Adt3::C9, 0u64, 0u64, f3(Str::lit("")))
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f24(v0: Fn4, p150: (List<List<Adt8>>, List<Bytes>), p151: List<Adt13>) -> (List<List<Adt8>>, List<Bytes>) {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v1, v2) = m149;
                let m147 = v2;
                match m147.uncons() {
                    None => {
                        Err::<((List<List<Adt8>>, List<Bytes>), List<Adt13>), (List<List<Adt8>>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<List<Adt8>>, List<Bytes>), List<Adt13>), (List<List<Adt8>>, List<Bytes>)>(({
                            let c148: Fn4 = v0.clone();
                            c148.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: List<Adt8>, v1: (List<List<Adt8>>, List<Bytes>), v2: Adt13) -> (List<List<Adt8>>, List<Bytes>) {
    if f36(v0.clone(), match v2.clone() {
        Adt13::C0(h152, _) => {
            h152
        }
    }) {
        v1
    } else {
        let (_, h153) = f37(Fn7::F38(v0, v2.clone()), (true, v1), match v2 {
            Adt13::C0(_, h154) => {
                h154
            }
        });
        h153
    }
}

pub fn f26(v0: Adt7, v1: Adt14) -> R<Option<Adt14>> {
    let m156 = match v1.clone() {
        Adt14::C0(_, _, _, h155, _, _) => {
            h155
        }
    };
    match m156 {
        None => {
            let m158 = match v1.clone() {
                Adt14::C0(_, h157, _, _, _, _) => {
                    h157
                }
            };
            match m158.uncons() {
                None => {
                    Ok(None::<Adt14>)
                }
                Some((v3, v4)) => {
                    Ok(Some(f39(v0, v1, v4, v3)?))
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt14>)
        }
    }
}

pub fn f27(v0: Fn5, p162: List<Adt13>, p163: List<Adt13>) -> List<Adt13> {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v1, v2) = m161;
                let m159 = v2;
                match m159.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>(({
                            let c160: Fn5 = v0.clone();
                            c160.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Adt13>, v1: Adt13) -> List<Adt13> {
    List::cons(v1, v0)
}

pub fn f29(v0: Fn6, p168: u64, p169: u64) -> R<(u64, bool)> {
    let m167 = (p168, p169);
    {
        let mut m167 = m167;
        loop {
            let step = {
                let (v1, v2) = m167;
                let m164 = v1;
                if m164 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m164 - 1;
                    let m166 = {
                        let c165: Fn6 = v0.clone();
                        c165.apply(v2)?
                    };
                    match m166 {
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
                Ok(next) => { m167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Adt7, v1: u64) -> R<Option<u64>> {
    f40(v0, v1)
}

pub fn f31(v0: Adt7, v1: u64) -> Option<Adt0> {
    let a171 = match v0 {
        Adt7::C0(_, h170, _, _, _, _, _, _) => {
            h170
        }
    };
    let a172 = v1;
    index_list(a171, a172)
}

pub fn f32(v0: u64, v1: List<u64>) -> bool {
    f41(Fn8::F42(v0), false, v1)
}

pub fn f33(v0: u64) -> Bytes {
    f43(v0)
}

pub fn f34(v0: Bytes, v1: Adt0) -> Adt6 {
    Adt6::C0(v0, match v1.clone() {
        Adt0::C0(_, h173, _, _, _, _, _, _) => {
            h173
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h174, _, _, _, _, _) => {
            h174
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h175, _, _, _, _) => {
            h175
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h176, _, _, _) => {
            h176
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h177, _, _) => {
            h177
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, h178, _) => {
            h178
        }
    })
}

pub fn f35(v0: Adt3, v1: u64, v2: u64, v3: Bytes) -> Adt10 {
    Adt10::C0(v0, v1, v2, v3)
}

pub fn f36(v0: List<Adt8>, v1: u64) -> bool {
    f44(Fn9::F45(v1), false, v0)
}

pub fn f37(v0: Fn7, p182: (bool, (List<List<Adt8>>, List<Bytes>)), p183: List<u64>) -> (bool, (List<List<Adt8>>, List<Bytes>)) {
    let m181 = (p182, p183);
    {
        let mut m181 = m181;
        loop {
            let step = {
                let (v1, v2) = m181;
                let m179 = v2;
                match m179.uncons() {
                    None => {
                        Err::<((bool, (List<List<Adt8>>, List<Bytes>)), List<u64>), (bool, (List<List<Adt8>>, List<Bytes>))>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, (List<List<Adt8>>, List<Bytes>)), List<u64>), (bool, (List<List<Adt8>>, List<Bytes>))>(({
                            let c180: Fn7 = v0.clone();
                            c180.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<Adt8>, v1: Adt13, v2: (bool, (List<List<Adt8>>, List<Bytes>)), v3: u64) -> (bool, (List<List<Adt8>>, List<Bytes>)) {
    let (h184, _) = v2.clone();
    if h184 {
        (false, {
            let (_, h185) = v2;
            h185
        })
    } else {
        let v4: List<Adt8> = f46(v0, match v1 {
            Adt13::C0(h186, _) => {
                h186
            }
        }, v3);
        let v5: Bytes = f47(v4.clone());
        if f48(v5.clone(), {
            let (_, h187) = {
                let (_, h188) = v2.clone();
                h188
            };
            h187
        }) {
            (false, {
                let (_, h189) = v2;
                h189
            })
        } else {
            (false, (List::cons(v4, {
                let (h190, _) = {
                    let (_, h191) = v2.clone();
                    h191
                };
                h190
            }), List::cons(v5, {
                let (_, h192) = {
                    let (_, h193) = v2;
                    h193
                };
                h192
            })))
        }
    }
}

pub fn f39(v0: Adt7, v1: Adt14, v2: List<Adt15>, v3: Adt15) -> R<Adt14> {
    let v4: Adt10 = f23(v1.clone());
    let m194 = v3;
    match m194 {
        Adt15::C0(v5, v6) => {
            let v7: (Adt11, Option<Adt5>) = f49(v0, match v1.clone() {
                Adt14::C0(_, _, _, _, h195, _) => {
                    h195
                }
            })?;
            let v8: Adt14 = f50(v1.clone(), {
                let (h196, _) = v7;
                h196
            });
            let m198 = {
                let (_, h197) = v7;
                h197
            };
            match m198 {
                None => {
                    Ok(f52(v8, match v1 {
                        Adt14::C0(h199, _, _, _, _, _) => {
                            h199
                        }
                    }, List::cons(Adt15::C3(v6), List::cons(Adt15::C1(v5, v6, None::<u64>), v2)), None::<Adt10>))
                }
                Some(v9) => {
                    Ok(f51(v8, v9))
                }
            }
        }
        Adt15::C1(v11, v12, v13) => {
            f54(v0, v1, v2, v11, v12, v13, v4)
        }
        Adt15::C2(v14, v15, v16, v17, v18, v19, v20) => {
            Ok(f52(v1.clone(), match v1 {
                Adt14::C0(h200, _, _, _, _, _) => {
                    h200
                }
            }, List::cons(Adt15::C1(v14, v15, v20), v2), Some(f55(v17, v18, v19, v16, v4))))
        }
        Adt15::C3(v10) => {
            f53(v0, v1, v2, v10)
        }
        Adt15::C4(v21, v22) => {
            let v23: u64 = f16(v0.clone(), match v1.clone() {
                Adt14::C0(h201, _, _, _, _, _) => {
                    h201
                }
            })?;
            if f56({
                let a203 = v23;
                let a204 = match v0.clone() {
                    Adt7::C0(_, _, h202, _, _, _, _, _) => {
                        h202
                    }
                };
                nat_lt(a203, a204)
            }, f57(f17(v0.clone(), v23), 3u64, f3(Str::lit(")")))) {
                Ok(f52(v1, {
                    let a205 = v23;
                    let a206 = 1u64;
                    nat_add(a205, a206)?
                }, List::cons(Adt15::C6(v22), v2), Some(f35(Adt3::C8(std::rc::Rc::new(match v4.clone() {
                    Adt10::C0(h207, _, _, _) => {
                        h207
                    }
                }), v21, {
                    let a208 = v23;
                    let a209 = 1u64;
                    nat_add(a208, a209)?
                }), v21, {
                    let a210 = v23;
                    let a211 = 1u64;
                    nat_add(a210, a211)?
                }, f21(f21(f21(f3(Str::lit("G")), match v4 {
                    Adt10::C0(_, _, _, h212) => {
                        h212
                    }
                }), f58(v21)), f58({
                    let a213 = v23;
                    let a214 = 1u64;
                    nat_add(a213, a214)?
                }))))))
            } else {
                Ok(f51(f52(v1, v23, v2, None::<Adt10>), f4(f3(Str::lit("LLP2004")), f3(Str::lit("unclosed grouping parenthesis")), f59(v0, v23)?)))
            }
        }
        Adt15::C5(v24, v25, v26, v27) => {
            Ok(f52(v1.clone(), match v1 {
                Adt14::C0(h215, _, _, _, _, _) => {
                    h215
                }
            }, List::cons(Adt15::C6(v27), v2), Some(f60(f3(Str::lit("Q")), v24, v25, v26, v4, true))))
        }
        Adt15::C6(v28) => {
            let v29: u64 = f16(v0.clone(), match v1.clone() {
                Adt14::C0(h216, _, _, _, _, _) => {
                    h216
                }
            })?;
            if f56({
                let a218 = v29;
                let a219 = match v0.clone() {
                    Adt7::C0(_, _, h217, _, _, _, _, _) => {
                        h217
                    }
                };
                nat_lt(a218, a219)
            }, f57(f17(v0, v29), 3u64, f3(Str::lit("(")))) {
                Ok(f52(v1, {
                    let a220 = v29;
                    let a221 = 1u64;
                    nat_add(a220, a221)?
                }, List::cons(Adt15::C0(0u64, {
                    let a222 = v28;
                    let a223 = 1u64;
                    nat_add(a222, a223)?
                }), List::cons(Adt15::C7(v4, v29, List::<u64>::nil(), List::<Adt10>::nil(), v28), v2)), None::<Adt10>))
            } else {
                Ok(f52(v1, v29, v2, Some(v4)))
            }
        }
        Adt15::C7(v30, v31, v32, v33, v34) => {
            let v35: List<Adt10> = List::cons(v4, v33);
            let v36: u64 = f16(v0.clone(), match v1.clone() {
                Adt14::C0(h224, _, _, _, _, _) => {
                    h224
                }
            })?;
            let v37: Adt0 = f17(v0.clone(), v36);
            if f56({
                let a226 = v36;
                let a227 = match v0.clone() {
                    Adt7::C0(_, _, h225, _, _, _, _, _) => {
                        h225
                    }
                };
                nat_lt(a226, a227)
            }, f57(v37.clone(), 4u64, f3(Str::lit(",")))) {
                Ok(f52(v1, {
                    let a228 = v36;
                    let a229 = 1u64;
                    nat_add(a228, a229)?
                }, List::cons(Adt15::C0(0u64, {
                    let a230 = v34;
                    let a231 = 1u64;
                    nat_add(a230, a231)?
                }), List::cons(Adt15::C7(v30, v31, List::cons(v36, v32), v35, v34), v2)), None::<Adt10>))
            } else {
                if f56({
                    let a233 = v36;
                    let a234 = match v0.clone() {
                        Adt7::C0(_, _, h232, _, _, _, _, _) => {
                            h232
                        }
                    };
                    nat_lt(a233, a234)
                }, f57(v37, 3u64, f3(Str::lit(")")))) {
                    Ok(f52(v1, {
                        let a235 = v36;
                        let a236 = 1u64;
                        nat_add(a235, a236)?
                    }, List::cons(Adt15::C6(v34), v2), Some(f61(v30, f62(v35), {
                        let a237 = v36;
                        let a238 = 1u64;
                        nat_add(a237, a238)?
                    }, v31, v36, f63(v32)))))
                } else {
                    Ok(f51(f52(v1, v36, v2, None::<Adt10>), f4(f3(Str::lit("LLP2004")), f3(Str::lit("expected `,` or `)` in a call argument list")), f59(v0, v36)?)))
                }
            }
        }
    }
}

pub fn f40(v0: Adt7, v1: u64) -> R<Option<u64>> {
    let a240 = v1;
    let a241 = match v0.clone() {
        Adt7::C0(_, _, h239, _, _, _, _, _) => {
            h239
        }
    };
    if nat_lt(a240, a241) {
        let a242 = f64(v0, v1);
        let a243 = 6u64;
        if nat_eq(a242, a243) {
            Ok(Some({
                let a244 = v1;
                let a245 = 1u64;
                nat_add(a244, a245)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f41(v0: Fn8, p249: bool, p250: List<u64>) -> bool {
    let m248 = (p249, p250);
    {
        let mut m248 = m248;
        loop {
            let step = {
                let (v1, v2) = m248;
                let m246 = v2;
                match m246.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c247: Fn8 = v0.clone();
                            c247.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a251 = v2;
        let a252 = v0;
        nat_eq(a251, a252)
    }
}

pub fn f43(v0: u64) -> Bytes {
    f65(f66(), 10u64, v0)
}

pub fn f44(v0: Fn9, p256: bool, p257: List<Adt8>) -> bool {
    let m255 = (p256, p257);
    {
        let mut m255 = m255;
        loop {
            let step = {
                let (v1, v2) = m255;
                let m253 = v2;
                match m253.uncons() {
                    None => {
                        Err::<(bool, List<Adt8>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt8>), bool>(({
                            let c254: Fn9 = v0.clone();
                            c254.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, v1: bool, v2: Adt8) -> bool {
    if v1 {
        true
    } else {
        let a259 = match v2 {
            Adt8::C0(h258, _) => {
                h258
            }
        };
        let a260 = v0;
        nat_eq(a259, a260)
    }
}

pub fn f46(v0: List<Adt8>, v1: u64, v2: u64) -> List<Adt8> {
    let v3: (bool, List<Adt8>) = f67(Fn10::F68(v1, v2), (false, List::<Adt8>::nil()), v0);
    f69(Fn11::F70, List::<Adt8>::nil(), {
        let (h261, _) = v3;
        if h261 {
            let (_, h262) = v3;
            h262
        } else {
            List::cons(Adt8::C0(v1, v2), {
                let (_, h263) = v3;
                h263
            })
        }
    })
}

pub fn f47(v0: List<Adt8>) -> Bytes {
    f71(Fn12::F72, f3(Str::lit("")), v0)
}

pub fn f48(v0: Bytes, v1: List<Bytes>) -> bool {
    f73(Fn13::F74(v0), false, v1)
}

pub fn f49(v0: Adt7, v1: Adt11) -> R<(Adt11, Option<Adt5>)> {
    let v2: u64 = {
        let a265 = match v1.clone() {
            Adt11::C0(h264, _, _) => {
                h264
            }
        };
        let a266 = 1u64;
        nat_add(a265, a266)?
    };
    let v3: Adt11 = Adt11::C0(v2, match v1.clone() {
        Adt11::C0(_, h267, _) => {
            h267
        }
    }, match v1 {
        Adt11::C0(_, _, h268) => {
            h268
        }
    });
    let a270 = match v0.clone() {
        Adt7::C0(_, _, _, _, h269, _, _, _) => {
            h269
        }
    };
    let a271 = v2;
    if nat_lt(a270, a271) {
        Ok((v3, Some(f8(f3(Str::lit("LLS8002")), f21(f21(f21(f21(f3(Str::lit("max_parse_states exceeded in phase parse: configured ")), f33(match v0 {
            Adt7::C0(_, _, _, _, h272, _, _, _) => {
                h272
            }
        })), f3(Str::lit(", observed "))), f33(v2)), f3(Str::lit(" states"))), None::<Adt6>))))
    } else {
        Ok((v3, None::<Adt5>))
    }
}

pub fn f50(v0: Adt14, v1: Adt11) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h273, _, _, _, _, _) => {
            h273
        }
    }, match v0.clone() {
        Adt14::C0(_, h274, _, _, _, _) => {
            h274
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h275, _, _, _) => {
            h275
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h276, _, _) => {
            h276
        }
    }, v1, match v0 {
        Adt14::C0(_, _, _, _, _, h277) => {
            h277
        }
    })
}

pub fn f51(v0: Adt14, v1: Adt5) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h278, _, _, _, _, _) => {
            h278
        }
    }, match v0.clone() {
        Adt14::C0(_, h279, _, _, _, _) => {
            h279
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h280, _, _, _) => {
            h280
        }
    }, Some(v1), match v0.clone() {
        Adt14::C0(_, _, _, _, h281, _) => {
            h281
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, h282) => {
            h282
        }
    })
}

pub fn f52(v0: Adt14, v1: u64, v2: List<Adt15>, v3: Option<Adt10>) -> Adt14 {
    Adt14::C0(v1, v2, v3, match v0.clone() {
        Adt14::C0(_, _, _, h283, _, _) => {
            h283
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h284, _) => {
            h284
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, h285) => {
            h285
        }
    })
}

pub fn f53(v0: Adt7, v1: Adt14, v2: List<Adt15>, v3: u64) -> R<Adt14> {
    let v4: (Adt11, Option<Adt5>) = f49(v0.clone(), match v1.clone() {
        Adt14::C0(_, _, _, _, h286, _) => {
            h286
        }
    })?;
    let v5: Adt14 = f50(v1.clone(), {
        let (h287, _) = v4;
        h287
    });
    let m289 = {
        let (_, h288) = v4;
        h288
    };
    match m289 {
        None => {
            let a291 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, h290, _, _) => {
                    h290
                }
            };
            let a292 = v3;
            if nat_lt(a291, a292) {
                Ok(f51(v5, f4(f3(Str::lit("LLS8002")), f21(f21(f21(f21(f3(Str::lit("max_scope_depth exceeded in phase parse (mathematical grouping): configured ")), f33(match v0.clone() {
                    Adt7::C0(_, _, _, _, _, h293, _, _) => {
                        h293
                    }
                })), f3(Str::lit(", nesting depth "))), f33(v3)), f3(Str::lit(""))), f59(v0, match v1 {
                    Adt14::C0(h294, _, _, _, _, _) => {
                        h294
                    }
                })?)))
            } else {
                let v7: u64 = f16(v0.clone(), match v1 {
                    Adt14::C0(h295, _, _, _, _, _) => {
                        h295
                    }
                })?;
                let a297 = v7;
                let a298 = match v0.clone() {
                    Adt7::C0(_, _, h296, _, _, _, _, _) => {
                        h296
                    }
                };
                if nat_lt(a297, a298) {
                    let v8: Adt0 = f17(v0.clone(), v7);
                    let v9: u64 = f18(v8.clone());
                    let a299 = v9;
                    let a300 = 2u64;
                    if nat_eq(a299, a300) {
                        Ok(f52(v5, {
                            let a301 = v7;
                            let a302 = 1u64;
                            nat_add(a301, a302)?
                        }, List::cons(Adt15::C6(v3), v2), Some(f35(Adt3::C0(match v8.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, h303) => {
                                h303
                            }
                        }, v7, {
                            let a304 = v7;
                            let a305 = 1u64;
                            nat_add(a304, a305)?
                        }), v7, {
                            let a306 = v7;
                            let a307 = 1u64;
                            nat_add(a306, a307)?
                        }, f21(f21(f21(f3(Str::lit("N")), f75(match v8 {
                            Adt0::C0(_, _, _, _, _, _, _, h308) => {
                                h308
                            }
                        })), f58(v7)), f58({
                            let a309 = v7;
                            let a310 = 1u64;
                            nat_add(a309, a310)?
                        }))))))
                    } else {
                        if f57(v8.clone(), 3u64, f3(Str::lit("("))) {
                            Ok(f52(v5, {
                                let a311 = v7;
                                let a312 = 1u64;
                                nat_add(a311, a312)?
                            }, List::cons(Adt15::C0(0u64, {
                                let a313 = v3;
                                let a314 = 1u64;
                                nat_add(a313, a314)?
                            }), List::cons(Adt15::C4(v7, v3), v2)), None::<Adt10>))
                        } else {
                            if if f57(v8.clone(), 3u64, f3(Str::lit("{"))) {
                                true
                            } else {
                                f57(v8.clone(), 3u64, f3(Str::lit("}")))
                            } {
                                Ok(f51(v5, f4(f3(Str::lit("LLP2004")), f3(Str::lit("braces do not group mathematical terms")), f22(v0, v7))))
                            } else {
                                if f57(v8.clone(), 0u64, f3(Str::lit("\\lexeme"))) {
                                    let m317 = f76(v0, {
                                        let a315 = v7;
                                        let a316 = 1u64;
                                        nat_add(a315, a316)?
                                    })?;
                                    match m317 {
                                        Ok(v10) => {
                                            Ok(f52(v5, match v10.clone() {
                                                Adt17::C0(h318, _, _) => {
                                                    h318
                                                }
                                            }, List::cons(Adt15::C6(v3), v2), Some(f35(Adt3::C2(match v10.clone() {
                                                Adt17::C0(_, h319, _) => {
                                                    h319
                                                }
                                            }, v7, match v10.clone() {
                                                Adt17::C0(h320, _, _) => {
                                                    h320
                                                }
                                            }), v7, match v10.clone() {
                                                Adt17::C0(h321, _, _) => {
                                                    h321
                                                }
                                            }, f21(f21(f21(f3(Str::lit("E")), f75(match v10.clone() {
                                                Adt17::C0(_, h322, _) => {
                                                    h322
                                                }
                                            })), f58(v7)), f58(match v10 {
                                                Adt17::C0(h323, _, _) => {
                                                    h323
                                                }
                                            }))))))
                                        }
                                        Err(v11) => {
                                            Ok(f51(v5, v11))
                                        }
                                    }
                                } else {
                                    if f57(v8.clone(), 0u64, f3(Str::lit("\\reference"))) {
                                        let m326 = f76(v0.clone(), {
                                            let a324 = v7;
                                            let a325 = 1u64;
                                            nat_add(a324, a325)?
                                        })?;
                                        match m326 {
                                            Ok(v12) => {
                                                let v13: Bytes = match v12.clone() {
                                                    Adt17::C0(_, h327, _) => {
                                                        h327
                                                    }
                                                };
                                                let m328 = f77(v13.clone(), f3(Str::lit("::")))?;
                                                match m328 {
                                                    None => {
                                                        Ok(f51(v5, f4(f3(Str::lit("LLR3005")), f21(f21(f3(Str::lit("`")), v13), f3(Str::lit("` is not a `Module::component` reference"))), f22(v0, v7))))
                                                    }
                                                    Some(v14) => {
                                                        let v15: Bytes = f78(v13.clone(), 0u64, v14);
                                                        let v16: Bytes = f78(v13.clone(), {
                                                            let a329 = v14;
                                                            let a330 = 2u64;
                                                            nat_add(a329, a330)?
                                                        }, {
                                                            let a334 = {
                                                                let a331 = v13;
                                                                length_bytes(a331)
                                                            };
                                                            let a335 = {
                                                                let a332 = v14;
                                                                let a333 = 2u64;
                                                                nat_add(a332, a333)?
                                                            };
                                                            nat_sub(a334, a335)
                                                        });
                                                        Ok(f52(v5, match v12.clone() {
                                                            Adt17::C0(h336, _, _) => {
                                                                h336
                                                            }
                                                        }, List::cons(Adt15::C6(v3), v2), Some(f35(Adt3::C3(v15.clone(), v16.clone(), v7, match v12.clone() {
                                                            Adt17::C0(h337, _, _) => {
                                                                h337
                                                            }
                                                        }), v7, match v12.clone() {
                                                            Adt17::C0(h338, _, _) => {
                                                                h338
                                                            }
                                                        }, f21(f21(f21(f21(f3(Str::lit("R")), f75(v15)), f75(v16)), f58(v7)), f58(match v12 {
                                                            Adt17::C0(h339, _, _) => {
                                                                h339
                                                            }
                                                        }))))))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(f51(v5, v17))
                                            }
                                        }
                                    } else {
                                        f79(v0, f52(v5, v7, v2.clone(), None::<Adt10>), v2, v3, v7, v8)
                                    }
                                }
                            }
                        }
                    }
                } else {
                    Ok(f51(f52(v5, v7, v2, None::<Adt10>), f4(f3(Str::lit("LLP2001")), f3(Str::lit("expected a mathematical expression")), f59(v0, v7)?)))
                }
            }
        }
        Some(v6) => {
            Ok(f51(v5, v6))
        }
    }
}

pub fn f54(v0: Adt7, v1: Adt14, v2: List<Adt15>, v3: u64, v4: u64, v5: Option<u64>, v6: Adt10) -> R<Adt14> {
    let v7: u64 = f16(v0.clone(), match v1.clone() {
        Adt14::C0(h340, _, _, _, _, _) => {
            h340
        }
    })?;
    let v8: Adt14 = f52(v1.clone(), v7, v2.clone(), Some(v6.clone()));
    let a342 = v7;
    let a343 = match v0.clone() {
        Adt7::C0(_, _, h341, _, _, _, _, _) => {
            h341
        }
    };
    if nat_lt(a342, a343) {
        let v9: Adt16 = f20(v0.clone(), match v1 {
            Adt14::C0(_, _, _, _, h344, _) => {
                h344
            }
        }, v7)?;
        let v10: Adt14 = f50(v8, match v9.clone() {
            Adt16::C0(h345, _, _) => {
                h345
            }
        });
        let m347 = match v9.clone() {
            Adt16::C0(_, _, h346) => {
                h346
            }
        };
        match m347 {
            None => {
                let v12: List<Adt2> = f80(match v9 {
                    Adt16::C0(_, h348, _) => {
                        h348
                    }
                }, match v0.clone() {
                    Adt7::C0(_, _, h349, _, _, _, _, _) => {
                        h349
                    }
                }, 3u64, 4u64);
                let m352 = {
                    let a350 = v12.clone();
                    let a351 = 0u64;
                    index_list(a350, a351)
                };
                match m352 {
                    None => {
                        Ok(v10)
                    }
                    Some(v13) => {
                        if f81(v13.clone(), v12.clone(), true) {
                            let v14: u64 = match v13.clone() {
                                Adt2::C0(_, _, _, h353, _) => {
                                    h353
                                }
                            };
                            let a354 = v14;
                            let a355 = v3;
                            if nat_lt(a354, a355) {
                                Ok(v10)
                            } else {
                                let a357 = match v13.clone() {
                                    Adt2::C0(_, _, h356, _, _) => {
                                        h356
                                    }
                                };
                                let a358 = 3u64;
                                if nat_eq(a357, a358) {
                                    Ok(f52(v10, match v13.clone() {
                                        Adt2::C0(_, h359, _, _, _) => {
                                            h359
                                        }
                                    }, List::cons(Adt15::C1(v3, v4, v5), v2), Some(f60(f3(Str::lit("P")), f82(v12), v7, match v13 {
                                        Adt2::C0(_, h360, _, _, _) => {
                                            h360
                                        }
                                    }, v6, false))))
                                } else {
                                    let m361 = v5;
                                    if match m361 {
                                        None => {
                                            false
                                        }
                                        Some(v15) => {
                                            let a362 = v15;
                                            let a363 = v14;
                                            nat_eq(a362, a363)
                                        }
                                    } {
                                        Ok(f51(v10, f4(f3(Str::lit("LLP2004")), f3(Str::lit("a nonassociative chain requires explicit parentheses")), f59(v0, v7)?)))
                                    } else {
                                        let v16: u64 = match v13.clone() {
                                            Adt2::C0(_, _, _, _, h364) => {
                                                h364
                                            }
                                        };
                                        let v17: u64 = {
                                            let a365 = v16;
                                            let a366 = 2u64;
                                            if nat_eq(a365, a366) {
                                                v14
                                            } else {
                                                let a367 = v14;
                                                let a368 = 1u64;
                                                nat_add(a367, a368)?
                                            }
                                        };
                                        let v18: Option<u64> = {
                                            let a369 = v16;
                                            let a370 = 1u64;
                                            if if nat_eq(a369, a370) {
                                                true
                                            } else {
                                                let a371 = v16;
                                                let a372 = 2u64;
                                                nat_eq(a371, a372)
                                            } {
                                                None::<u64>
                                            } else {
                                                Some(v14)
                                            }
                                        };
                                        Ok(f52(v10, match v13.clone() {
                                            Adt2::C0(_, h373, _, _, _) => {
                                                h373
                                            }
                                        }, List::cons(Adt15::C0(v17, {
                                            let a374 = v4;
                                            let a375 = 1u64;
                                            nat_add(a374, a375)?
                                        }), List::cons(Adt15::C2(v3, v4, v6, f82(v12), v7, match v13 {
                                            Adt2::C0(_, h376, _, _, _) => {
                                                h376
                                            }
                                        }, v18), v2)), None::<Adt10>))
                                    }
                                }
                            }
                        } else {
                            Ok(f51(v10, f4(f3(Str::lit("LLP2002")), f3(Str::lit("operator candidates at this surface disagree on precedence, associativity, or frame; LexLean does not guess")), f59(v0, v7)?)))
                        }
                    }
                }
            }
            Some(v11) => {
                Ok(f51(v10, v11))
            }
        }
    } else {
        Ok(v8)
    }
}

pub fn f55(v0: List<u64>, v1: u64, v2: u64, v3: Adt10, v4: Adt10) -> Adt10 {
    f35(Adt3::C7(v0.clone(), v1, v2, std::rc::Rc::new(match v3.clone() {
        Adt10::C0(h377, _, _, _) => {
            h377
        }
    }), std::rc::Rc::new(match v4.clone() {
        Adt10::C0(h378, _, _, _) => {
            h378
        }
    })), match v3.clone() {
        Adt10::C0(_, h379, _, _) => {
            h379
        }
    }, match v4.clone() {
        Adt10::C0(_, _, h380, _) => {
            h380
        }
    }, f21(f21(f21(f21(f21(f3(Str::lit("X")), f83(v0)), f58(v1)), f58(v2)), match v3 {
        Adt10::C0(_, _, _, h381) => {
            h381
        }
    }), match v4 {
        Adt10::C0(_, _, _, h382) => {
            h382
        }
    }))
}

pub fn f56(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f57(v0: Adt0, v1: u64, v2: Bytes) -> bool {
    f56({
        let a383 = f18(v0.clone());
        let a384 = v1;
        nat_eq(a383, a384)
    }, f19(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h385) => {
            h385
        }
    }, v2))
}

pub fn f58(v0: u64) -> Bytes {
    f21(f33(v0), f3(Str::lit(",")))
}

pub fn f59(v0: Adt7, v1: u64) -> R<Adt6> {
    let v2: u64 = f16(v0.clone(), v1)?;
    let a387 = v2;
    let a388 = match v0.clone() {
        Adt7::C0(_, _, h386, _, _, _, _, _) => {
            h386
        }
    };
    if nat_lt(a387, a388) {
        Ok(f22(v0, v2))
    } else {
        Ok(f22(v0.clone(), {
            let a390 = match v0 {
                Adt7::C0(_, _, h389, _, _, _, _, _) => {
                    h389
                }
            };
            let a391 = 1u64;
            nat_sub(a390, a391)
        }))
    }
}

pub fn f60(v0: Bytes, v1: List<u64>, v2: u64, v3: u64, v4: Adt10, v5: bool) -> Adt10 {
    let v6: Bytes = f21(f21(f21(f21(v0, f83(v1.clone())), f58(v2)), f58(v3)), match v4.clone() {
        Adt10::C0(_, _, _, h392) => {
            h392
        }
    });
    if v5 {
        f35(Adt3::C5(v1, v2, v3, std::rc::Rc::new(match v4.clone() {
            Adt10::C0(h393, _, _, _) => {
                h393
            }
        })), v2, match v4 {
            Adt10::C0(_, _, h394, _) => {
                h394
            }
        }, v6)
    } else {
        f35(Adt3::C6(v1, v2, v3, std::rc::Rc::new(match v4.clone() {
            Adt10::C0(h395, _, _, _) => {
                h395
            }
        })), match v4 {
            Adt10::C0(_, h396, _, _) => {
                h396
            }
        }, v3, v6)
    }
}

pub fn f61(v0: Adt10, v1: List<Adt10>, v2: u64, v3: u64, v4: u64, v5: List<u64>) -> Adt10 {
    f35(Adt3::C4(std::rc::Rc::new(match v0.clone() {
        Adt10::C0(h397, _, _, _) => {
            h397
        }
    }), std::rc::Rc::new(f84(v1.clone())), match v0.clone() {
        Adt10::C0(_, h398, _, _) => {
            h398
        }
    }, v2, v3, v4, v5.clone()), match v0.clone() {
        Adt10::C0(_, h399, _, _) => {
            h399
        }
    }, v2, f21(f21(f21(f21(f21(f3(Str::lit("C")), match v0 {
        Adt10::C0(_, _, _, h400) => {
            h400
        }
    }), f85(Fn14::F86, f58({
        let a401 = v1.clone();
        length_list(a401)
    }), v1)), f58(v2)), f21(f58(v3), f58(v4))), f83(v5)))
}

pub fn f62(v0: List<Adt10>) -> List<Adt10> {
    f87(Fn15::F88, List::<Adt10>::nil(), v0)
}

pub fn f63(v0: List<u64>) -> List<u64> {
    f89(Fn16::F90, List::<u64>::nil(), v0)
}

pub fn f64(v0: Adt7, v1: u64) -> u64 {
    let m402 = f31(v0, v1);
    match m402 {
        None => {
            7u64
        }
        Some(v2) => {
            f18(v2)
        }
    }
}

pub fn f65(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a403 = v2;
    let a404 = 0u64;
    if nat_eq(a403, a404) {
        f91(v0, 0u64)
    } else {
        let (_, h405) = {
            let (h406, _) = f92(Fn17::F93(v1, v0), v2, (v2, Bytes::lit(&[])));
            h406
        };
        h405
    }
}

pub fn f66() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f67(v0: Fn10, p410: (bool, List<Adt8>), p411: List<Adt8>) -> (bool, List<Adt8>) {
    let m409 = (p410, p411);
    {
        let mut m409 = m409;
        loop {
            let step = {
                let (v1, v2) = m409;
                let m407 = v2;
                match m407.uncons() {
                    None => {
                        Err::<((bool, List<Adt8>), List<Adt8>), (bool, List<Adt8>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt8>), List<Adt8>), (bool, List<Adt8>)>(({
                            let c408: Fn10 = v0.clone();
                            c408.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: u64, v1: u64, v2: (bool, List<Adt8>), v3: Adt8) -> (bool, List<Adt8>) {
    let (h412, _) = v2;
    if h412 {
        (true, List::cons(v3, {
            let (_, h413) = v2;
            h413
        }))
    } else {
        let a415 = v0;
        let a416 = match v3.clone() {
            Adt8::C0(h414, _) => {
                h414
            }
        };
        if nat_lt(a415, a416) {
            (true, List::cons(v3, List::cons(Adt8::C0(v0, v1), {
                let (_, h417) = v2;
                h417
            })))
        } else {
            (false, List::cons(v3, {
                let (_, h418) = v2;
                h418
            }))
        }
    }
}

pub fn f69(v0: Fn11, p422: List<Adt8>, p423: List<Adt8>) -> List<Adt8> {
    let m421 = (p422, p423);
    {
        let mut m421 = m421;
        loop {
            let step = {
                let (v1, v2) = m421;
                let m419 = v2;
                match m419.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<Adt8>), List<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<Adt8>), List<Adt8>>(({
                            let c420: Fn11 = v0.clone();
                            c420.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Adt8>, v1: Adt8) -> List<Adt8> {
    List::cons(v1, v0)
}

pub fn f71(v0: Fn12, p427: Bytes, p428: List<Adt8>) -> Bytes {
    let m426 = (p427, p428);
    {
        let mut m426 = m426;
        loop {
            let step = {
                let (v1, v2) = m426;
                let m424 = v2;
                match m424.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt8>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt8>), Bytes>(({
                            let c425: Fn12 = v0.clone();
                            c425.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Bytes, v1: Adt8) -> Bytes {
    f21(f21(v0, f58(match v1.clone() {
        Adt8::C0(h429, _) => {
            h429
        }
    })), f58(match v1 {
        Adt8::C0(_, h430) => {
            h430
        }
    }))
}

pub fn f73(v0: Fn13, p434: bool, p435: List<Bytes>) -> bool {
    let m433 = (p434, p435);
    {
        let mut m433 = m433;
        loop {
            let step = {
                let (v1, v2) = m433;
                let m431 = v2;
                match m431.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c432: Fn13 = v0.clone();
                            c432.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        f19(v2, v0)
    }
}

pub fn f75(v0: Bytes) -> Bytes {
    f21(f21(f33({
        let a436 = v0.clone();
        length_bytes(a436)
    }), f3(Str::lit(":"))), v0)
}

pub fn f76(v0: Adt7, v1: u64) -> R<Result<Adt17, Adt5>> {
    let v2: u64 = f16(v0.clone(), v1)?;
    if f56({
        let a438 = v2;
        let a439 = match v0.clone() {
            Adt7::C0(_, _, h437, _, _, _, _, _) => {
                h437
            }
        };
        nat_lt(a438, a439)
    }, f57(f17(v0.clone(), v2), 3u64, f3(Str::lit("{")))) {
        let v3: Adt17 = {
            let (h440, _) = f94(Fn18::F95(v0.clone()), {
                let a443 = {
                    let a442 = match v0.clone() {
                        Adt7::C0(_, h441, _, _, _, _, _, _) => {
                            h441
                        }
                    };
                    length_list(a442)
                };
                let a444 = 1u64;
                nat_add(a443, a444)?
            }, Adt17::C0({
                let a445 = v2;
                let a446 = 1u64;
                nat_add(a445, a446)?
            }, f3(Str::lit("")), false))?;
            h440
        };
        if match v3.clone() {
            Adt17::C0(_, _, h447) => {
                h447
            }
        } {
            Ok(Ok::<Adt17, Adt5>(v3))
        } else {
            Ok(Err::<Adt17, Adt5>(f4(f3(Str::lit("LLP2003")), f3(Str::lit("unclosed `{` argument")), f59(v0, match v3 {
                Adt17::C0(h448, _, _) => {
                    h448
                }
            })?)))
        }
    } else {
        Ok(Err::<Adt17, Adt5>(f4(f3(Str::lit("LLP2003")), f3(Str::lit("expected `{`")), f59(v0, v2)?)))
    }
}

pub fn f77(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a449 = v0.clone();
        length_bytes(a449)
    };
    let v3: u64 = {
        let a450 = v1.clone();
        length_bytes(a450)
    };
    let v4: u64 = {
        let (h451, _) = f29(Fn6::F96(v1.clone(), v3, v0.clone(), v2), {
            let a452 = v2;
            let a453 = 1u64;
            nat_add(a452, a453)?
        }, 0u64)?;
        h451
    };
    if f97(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f78(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m457 = {
        let a454 = v0;
        let a455 = v1;
        let a456 = v2;
        slice_bytes(a454, a455, a456)
    };
    match m457 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f79(v0: Adt7, v1: Adt14, v2: List<Adt15>, v3: u64, v4: u64, v5: Adt0) -> R<Adt14> {
    let v6: Adt16 = f20(v0.clone(), match v1.clone() {
        Adt14::C0(_, _, _, _, h458, _) => {
            h458
        }
    }, v4)?;
    let v7: Adt14 = f50(v1, match v6.clone() {
        Adt16::C0(h459, _, _) => {
            h459
        }
    });
    let m461 = match v6.clone() {
        Adt16::C0(_, _, h460) => {
            h460
        }
    };
    match m461 {
        None => {
            let v9: u64 = match v0.clone() {
                Adt7::C0(_, _, h462, _, _, _, _, _) => {
                    h462
                }
            };
            let v10: List<Adt2> = f80(match v6.clone() {
                Adt16::C0(_, h463, _) => {
                    h463
                }
            }, v9, 2u64, 2u64);
            let v11: List<Adt2> = f80(match v6 {
                Adt16::C0(_, h464, _) => {
                    h464
                }
            }, v9, 0u64, 1u64);
            let m467 = {
                let a465 = v10.clone();
                let a466 = 0u64;
                index_list(a465, a466)
            };
            match m467 {
                None => {
                    f98(v0, v7, v2, v3, v4, v5, v11)
                }
                Some(v12) => {
                    if f81(v12.clone(), v10.clone(), false) {
                        let a469 = {
                            let a468 = v11;
                            length_list(a468)
                        };
                        let a470 = 0u64;
                        if nat_eq(a469, a470) {
                            Ok(f52(v7, match v12.clone() {
                                Adt2::C0(_, h471, _, _, _) => {
                                    h471
                                }
                            }, List::cons(Adt15::C0(match v12.clone() {
                                Adt2::C0(_, _, _, h472, _) => {
                                    h472
                                }
                            }, {
                                let a473 = v3;
                                let a474 = 1u64;
                                nat_add(a473, a474)?
                            }), List::cons(Adt15::C5(f82(v10), v4, match v12 {
                                Adt2::C0(_, h475, _, _, _) => {
                                    h475
                                }
                            }, v3), v2)), None::<Adt10>))
                        } else {
                            Ok(f51(v7, f4(f3(Str::lit("LLP2002")), f3(Str::lit("this surface is both a prefix operator and a leaf; LexLean does not guess")), f22(v0, v4))))
                        }
                    } else {
                        Ok(f51(v7, f4(f3(Str::lit("LLP2002")), f3(Str::lit("prefix candidates disagree on their profile; LexLean does not guess")), f22(v0, v4))))
                    }
                }
            }
        }
        Some(v8) => {
            Ok(f51(v7, v8))
        }
    }
}

pub fn f80(v0: List<Adt2>, v1: u64, v2: u64, v3: u64) -> List<Adt2> {
    f99(Fn19::F100(v3, v2, v1), List::<Adt2>::nil(), v0)
}

pub fn f81(v0: Adt2, v1: List<Adt2>, v2: bool) -> bool {
    f101(Fn20::F102(v0, v2), true, v1)
}

pub fn f82(v0: List<Adt2>) -> List<u64> {
    f63(f103(Fn21::F104, List::<u64>::nil(), v0))
}

pub fn f83(v0: List<u64>) -> Bytes {
    f105(Fn22::F106, f58({
        let a476 = v0.clone();
        length_list(a476)
    }), v0)
}

pub fn f84(v0: List<Adt10>) -> Adt3 {
    f107(Fn23::F108, Adt3::C9, f62(v0))
}

pub fn f85(v0: Fn14, p480: Bytes, p481: List<Adt10>) -> Bytes {
    let m479 = (p480, p481);
    {
        let mut m479 = m479;
        loop {
            let step = {
                let (v1, v2) = m479;
                let m477 = v2;
                match m477.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt10>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt10>), Bytes>(({
                            let c478: Fn14 = v0.clone();
                            c478.apply(v1, v3)
                        }, v4))
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

pub fn f86(v0: Bytes, v1: Adt10) -> Bytes {
    f21(v0, match v1 {
        Adt10::C0(_, _, _, h482) => {
            h482
        }
    })
}

pub fn f87(v0: Fn15, p486: List<Adt10>, p487: List<Adt10>) -> List<Adt10> {
    let m485 = (p486, p487);
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v1, v2) = m485;
                let m483 = v2;
                match m483.uncons() {
                    None => {
                        Err::<(List<Adt10>, List<Adt10>), List<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt10>, List<Adt10>), List<Adt10>>(({
                            let c484: Fn15 = v0.clone();
                            c484.apply(v1, v3)
                        }, v4))
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

pub fn f88(v0: List<Adt10>, v1: Adt10) -> List<Adt10> {
    List::cons(v1, v0)
}

pub fn f89(v0: Fn16, p491: List<u64>, p492: List<u64>) -> List<u64> {
    let m490 = (p491, p492);
    {
        let mut m490 = m490;
        loop {
            let step = {
                let (v1, v2) = m490;
                let m488 = v2;
                match m488.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c489: Fn16 = v0.clone();
                            c489.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f91(v0: Bytes, v1: u64) -> Bytes {
    let m496 = {
        let a493 = v0;
        let a494 = v1;
        let a495 = 1u64;
        slice_bytes(a493, a494, a495)
    };
    match m496 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: Fn17, p501: u64, p502: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m500 = (p501, p502);
    {
        let mut m500 = m500;
        loop {
            let step = {
                let (v1, v2) = m500;
                let m497 = v1;
                if m497 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m497 - 1;
                    let m499 = {
                        let c498: Fn17 = v0.clone();
                        c498.apply(v2.clone())
                    };
                    match m499 {
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
                Ok(next) => { m500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f109(v1, v0, v2)
}

pub fn f94(v0: Fn18, p507: u64, p508: Adt17) -> R<(Adt17, bool)> {
    let m506 = (p507, p508);
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v1, v2) = m506;
                let m503 = v1;
                if m503 == 0 {
                    Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m503 - 1;
                    let m505 = {
                        let c504: Fn18 = v0.clone();
                        c504.apply(v2.clone())?
                    };
                    match m505 {
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
                Ok(next) => { m506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: Adt7, v1: Adt17) -> R<Option<Adt17>> {
    f110(v0, v1)
}

pub fn f96(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f97(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a511 = {
            let a509 = v4;
            let a510 = v1;
            nat_add(a509, a510)?
        };
        let a512 = v3;
        if nat_lt(a511, a512) {
            Ok(Some({
                let a513 = v4;
                let a514 = 1u64;
                nat_add(a513, a514)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f97(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m519 = {
        let a516 = v0;
        let a517 = v2;
        let a518 = {
            let a515 = v1.clone();
            length_bytes(a515)
        };
        slice_bytes(a516, a517, a518)
    };
    match m519 {
        None => {
            false
        }
        Some(v3) => {
            let a520 = v3;
            let a521 = v1;
            equal(a520, a521)
        }
    }
}

pub fn f98(v0: Adt7, v1: Adt14, v2: List<Adt15>, v3: u64, v4: u64, v5: Adt0, v6: List<Adt2>) -> R<Adt14> {
    let v7: u64 = match v0.clone() {
        Adt7::C0(_, _, h522, _, _, _, _, _) => {
            h522
        }
    };
    let v9: Option<Adt18> = {
        let a523 = f18(v5.clone());
        let a524 = 1u64;
        if nat_eq(a523, a524) {
            let v8: Adt18 = f111(v0.clone(), v5.clone(), v4)?;
            let a526 = match v8.clone() {
                Adt18::C0(_, h525, _) => {
                    h525
                }
            };
            let a527 = v7;
            if nat_le(a526, a527) {
                Some(v8)
            } else {
                None::<Adt18>
            }
        } else {
            None::<Adt18>
        }
    };
    let v10: List<u64> = f103(Fn21::F112, List::<u64>::nil(), v6.clone());
    let v12: List<u64> = {
        let m528 = v9.clone();
        match m528 {
            None => {
                v10
            }
            Some(v11) => {
                f113(v10, match v11 {
                    Adt18::C0(_, h529, _) => {
                        h529
                    }
                })
            }
        }
    };
    let m532 = {
        let a530 = v12.clone();
        let a531 = 0u64;
        index_list(a530, a531)
    };
    match m532 {
        None => {
            Ok(f51(v1, f4(f3(Str::lit("LLL1004")), f21(f21(f3(Str::lit("unknown atom `")), match v5 {
                Adt0::C0(_, _, _, _, _, _, _, h533) => {
                    h533
                }
            }), f3(Str::lit("` in a mathematical island"))), f22(v0, v4))))
        }
        Some(v13) => {
            let v15: u64 = {
                let m534 = f114(v0, v4);
                match m534 {
                    None => {
                        v13
                    }
                    Some(v14) => {
                        if f32(v14, v12.clone()) {
                            v14
                        } else {
                            v13
                        }
                    }
                }
            };
            let v16: List<Adt13> = {
                let a536 = 1u64;
                let a537 = {
                    let a535 = v12.clone();
                    length_list(a535)
                };
                if nat_lt(a536, a537) {
                    List::cons(Adt13::C0(v4, v12), match v1.clone() {
                        Adt14::C0(_, _, _, _, _, h538) => {
                            h538
                        }
                    })
                } else {
                    match v1.clone() {
                        Adt14::C0(_, _, _, _, _, h539) => {
                            h539
                        }
                    }
                }
            };
            let v17: List<Adt4> = f115(Fn24::F116(v15), List::<Adt4>::nil(), v6);
            let v19: List<Adt4> = f117(Fn25::F118, {
                let m540 = v9;
                match m540 {
                    None => {
                        List::<Adt4>::nil()
                    }
                    Some(v18) => {
                        let a542 = match v18.clone() {
                            Adt18::C0(_, h541, _) => {
                                h541
                            }
                        };
                        let a543 = v15;
                        if nat_eq(a542, a543) {
                            List::cons(Adt4::C1(match v18 {
                                Adt18::C0(h544, _, _) => {
                                    h544
                                }
                            }), List::<Adt4>::nil())
                        } else {
                            List::<Adt4>::nil()
                        }
                    }
                }
            }, v17);
            let v20: Adt10 = f35(Adt3::C1(v19.clone(), v4, v15), v4, v15, f21(f21(f21(f3(Str::lit("L")), f119(v19)), f58(v4)), f58(v15)));
            Ok(Adt14::C0(v15, List::cons(Adt15::C6(v3), v2), Some(v20), match v1.clone() {
                Adt14::C0(_, _, _, h545, _, _) => {
                    h545
                }
            }, match v1 {
                Adt14::C0(_, _, _, _, h546, _) => {
                    h546
                }
            }, v16))
        }
    }
}

pub fn f99(v0: Fn19, p550: List<Adt2>, p551: List<Adt2>) -> List<Adt2> {
    let m549 = (p550, p551);
    {
        let mut m549 = m549;
        loop {
            let step = {
                let (v1, v2) = m549;
                let m547 = v2;
                match m547.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c548: Fn19 = v0.clone();
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

pub fn f100(v0: u64, v1: u64, v2: u64, v3: List<Adt2>, v4: Adt2) -> List<Adt2> {
    let a561 = v3;
    let a562 = if f56({
        let a553 = match v4.clone() {
            Adt2::C0(_, h552, _, _, _) => {
                h552
            }
        };
        let a554 = v2;
        nat_le(a553, a554)
    }, f56({
        let a556 = v1;
        let a557 = match v4.clone() {
            Adt2::C0(_, _, h555, _, _) => {
                h555
            }
        };
        nat_le(a556, a557)
    }, {
        let a559 = match v4.clone() {
            Adt2::C0(_, _, h558, _, _) => {
                h558
            }
        };
        let a560 = v0;
        nat_le(a559, a560)
    })) {
        List::cons(v4, List::<Adt2>::nil())
    } else {
        List::<Adt2>::nil()
    };
    append_list(a561, a562)
}

pub fn f101(v0: Fn20, p566: bool, p567: List<Adt2>) -> bool {
    let m565 = (p566, p567);
    {
        let mut m565 = m565;
        loop {
            let step = {
                let (v1, v2) = m565;
                let m563 = v2;
                match m563.uncons() {
                    None => {
                        Err::<(bool, List<Adt2>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt2>), bool>(({
                            let c564: Fn20 = v0.clone();
                            c564.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Adt2, v1: bool, v2: bool, v3: Adt2) -> bool {
    if v2 {
        f56({
            let a570 = match v3.clone() {
                Adt2::C0(_, h568, _, _, _) => {
                    h568
                }
            };
            let a571 = match v0.clone() {
                Adt2::C0(_, h569, _, _, _) => {
                    h569
                }
            };
            nat_eq(a570, a571)
        }, f56({
            let a574 = match v3.clone() {
                Adt2::C0(_, _, _, h572, _) => {
                    h572
                }
            };
            let a575 = match v0.clone() {
                Adt2::C0(_, _, _, h573, _) => {
                    h573
                }
            };
            nat_eq(a574, a575)
        }, if v1 {
            f56({
                let a578 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h576) => {
                        h576
                    }
                };
                let a579 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h577) => {
                        h577
                    }
                };
                nat_eq(a578, a579)
            }, {
                let a582 = match v3 {
                    Adt2::C0(_, _, h580, _, _) => {
                        h580
                    }
                };
                let a583 = match v0 {
                    Adt2::C0(_, _, h581, _, _) => {
                        h581
                    }
                };
                nat_eq(a582, a583)
            })
        } else {
            true
        }))
    } else {
        false
    }
}

pub fn f103(v0: Fn21, p587: List<u64>, p588: List<Adt2>) -> List<u64> {
    let m586 = (p587, p588);
    {
        let mut m586 = m586;
        loop {
            let step = {
                let (v1, v2) = m586;
                let m584 = v2;
                match m584.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt2>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt2>), List<u64>>(({
                            let c585: Fn21 = v0.clone();
                            c585.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m586 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: List<u64>, v1: Adt2) -> List<u64> {
    List::cons(match v1 {
        Adt2::C0(h589, _, _, _, _) => {
            h589
        }
    }, v0)
}

pub fn f105(v0: Fn22, p593: Bytes, p594: List<u64>) -> Bytes {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v1, v2) = m592;
                let m590 = v2;
                match m590.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), Bytes>(({
                            let c591: Fn22 = v0.clone();
                            c591.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: Bytes, v1: u64) -> Bytes {
    f21(v0, f58(v1))
}

pub fn f107(v0: Fn23, p598: Adt3, p599: List<Adt10>) -> Adt3 {
    let m597 = (p598, p599);
    {
        let mut m597 = m597;
        loop {
            let step = {
                let (v1, v2) = m597;
                let m595 = v2;
                match m595.uncons() {
                    None => {
                        Err::<(Adt3, List<Adt10>), Adt3>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Adt10>), Adt3>(({
                            let c596: Fn23 = v0.clone();
                            c596.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: Adt3, v1: Adt10) -> Adt3 {
    Adt3::C10(std::rc::Rc::new(match v1 {
        Adt10::C0(h600, _, _, _) => {
            h600
        }
    }), std::rc::Rc::new(v0))
}

pub fn f109(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h601, _) = v2;
        h601
    };
    let a602 = v3;
    let a603 = 0u64;
    if nat_eq(a602, a603) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a604 = v3;
            let a605 = v1;
            let a606 = 0u64;
            nat_quot(a604, a605, a606)
        }, {
            let a611 = f91(v0, {
                let a607 = v3;
                let a608 = v1;
                let a609 = 0u64;
                nat_rem(a607, a608, a609)
            });
            let a612 = {
                let (_, h610) = v2;
                h610
            };
            append_bytes(a611, a612)
        }))
    }
}

pub fn f110(v0: Adt7, v1: Adt17) -> R<Option<Adt17>> {
    if match v1.clone() {
        Adt17::C0(_, _, h613) => {
            h613
        }
    } {
        Ok(None::<Adt17>)
    } else {
        let v2: u64 = match v1.clone() {
            Adt17::C0(h614, _, _) => {
                h614
            }
        };
        let a616 = v2;
        let a617 = match v0.clone() {
            Adt7::C0(_, _, h615, _, _, _, _, _) => {
                h615
            }
        };
        if nat_lt(a616, a617) {
            let m618 = f31(v0, v2);
            match m618 {
                None => {
                    Ok(None::<Adt17>)
                }
                Some(v3) => {
                    if f57(v3.clone(), 3u64, f3(Str::lit("}"))) {
                        Ok(Some(Adt17::C0({
                            let a619 = v2;
                            let a620 = 1u64;
                            nat_add(a619, a620)?
                        }, match v1 {
                            Adt17::C0(_, h621, _) => {
                                h621
                            }
                        }, true)))
                    } else {
                        let a622 = f18(v3.clone());
                        let a623 = 6u64;
                        if nat_eq(a622, a623) {
                            Ok(Some(Adt17::C0({
                                let a624 = v2;
                                let a625 = 1u64;
                                nat_add(a624, a625)?
                            }, match v1 {
                                Adt17::C0(_, h626, _) => {
                                    h626
                                }
                            }, false)))
                        } else {
                            Ok(Some(Adt17::C0({
                                let a627 = v2;
                                let a628 = 1u64;
                                nat_add(a627, a628)?
                            }, f21(match v1 {
                                Adt17::C0(_, h629, _) => {
                                    h629
                                }
                            }, match v3 {
                                Adt0::C0(_, _, _, _, _, _, _, h630) => {
                                    h630
                                }
                            }), false)))
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt17>)
        }
    }
}

pub fn f111(v0: Adt7, v1: Adt0, v2: u64) -> R<Adt18> {
    let (h631, _) = f120(Fn26::F121(v0.clone()), {
        let a634 = {
            let a633 = match v0 {
                Adt7::C0(_, h632, _, _, _, _, _, _) => {
                    h632
                }
            };
            length_list(a633)
        };
        let a635 = 1u64;
        nat_add(a634, a635)?
    }, Adt18::C0(match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h636) => {
            h636
        }
    }, {
        let a637 = v2;
        let a638 = 1u64;
        nat_add(a637, a638)?
    }, match v1 {
        Adt0::C0(_, _, h639, _, _, _, _, _) => {
            h639
        }
    }))?;
    Ok(h631)
}

pub fn f112(v0: List<u64>, v1: Adt2) -> List<u64> {
    f113(v0, match v1 {
        Adt2::C0(_, h640, _, _, _) => {
            h640
        }
    })
}

pub fn f113(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: (bool, List<u64>) = f122(Fn27::F123(v1), (false, List::<u64>::nil()), v0);
    f63({
        let (h641, _) = v2;
        if h641 {
            let (_, h642) = v2;
            h642
        } else {
            List::cons(v1, {
                let (_, h643) = v2;
                h643
            })
        }
    })
}

pub fn f114(v0: Adt7, v1: u64) -> Option<u64> {
    f124(Fn28::F125(v1), None::<u64>, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, h644) => {
            h644
        }
    })
}

pub fn f115(v0: Fn24, p648: List<Adt4>, p649: List<Adt2>) -> List<Adt4> {
    let m647 = (p648, p649);
    {
        let mut m647 = m647;
        loop {
            let step = {
                let (v1, v2) = m647;
                let m645 = v2;
                match m645.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt2>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt2>), List<Adt4>>(({
                            let c646: Fn24 = v0.clone();
                            c646.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: u64, v1: List<Adt4>, v2: Adt2) -> List<Adt4> {
    let a651 = match v2.clone() {
        Adt2::C0(_, h650, _, _, _) => {
            h650
        }
    };
    let a652 = v0;
    if nat_eq(a651, a652) {
        List::cons(Adt4::C0(match v2 {
            Adt2::C0(h653, _, _, _, _) => {
                h653
            }
        }), v1)
    } else {
        v1
    }
}

pub fn f117(v0: Fn25, p657: List<Adt4>, p658: List<Adt4>) -> List<Adt4> {
    let m656 = (p657, p658);
    {
        let mut m656 = m656;
        loop {
            let step = {
                let (v1, v2) = m656;
                let m654 = v2;
                match m654.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c655: Fn25 = v0.clone();
                            c655.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f119(v0: List<Adt4>) -> Bytes {
    f126(Fn29::F127, f58({
        let a659 = v0.clone();
        length_list(a659)
    }), v0)
}

pub fn f120(v0: Fn26, p664: u64, p665: Adt18) -> R<(Adt18, bool)> {
    let m663 = (p664, p665);
    {
        let mut m663 = m663;
        loop {
            let step = {
                let (v1, v2) = m663;
                let m660 = v1;
                if m660 == 0 {
                    Err::<(u64, Adt18), R<(Adt18, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m660 - 1;
                    let m662 = {
                        let c661: Fn26 = v0.clone();
                        c661.apply(v2.clone())?
                    };
                    match m662 {
                        None => {
                            Err::<(u64, Adt18), R<(Adt18, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt18), R<(Adt18, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m663 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt7, v1: Adt18) -> R<Option<Adt18>> {
    f128(v0, v1)
}

pub fn f122(v0: Fn27, p669: (bool, List<u64>), p670: List<u64>) -> (bool, List<u64>) {
    let m668 = (p669, p670);
    {
        let mut m668 = m668;
        loop {
            let step = {
                let (v1, v2) = m668;
                let m666 = v2;
                match m666.uncons() {
                    None => {
                        Err::<((bool, List<u64>), List<u64>), (bool, List<u64>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<u64>), List<u64>), (bool, List<u64>)>(({
                            let c667: Fn27 = v0.clone();
                            c667.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: u64, v1: (bool, List<u64>), v2: u64) -> (bool, List<u64>) {
    let (h671, _) = v1;
    if h671 {
        (true, List::cons(v2, {
            let (_, h672) = v1;
            h672
        }))
    } else {
        let a673 = v0;
        let a674 = v2;
        if nat_lt(a673, a674) {
            (true, List::cons(v2, List::cons(v0, {
                let (_, h675) = v1;
                h675
            })))
        } else {
            let a676 = v0;
            let a677 = v2;
            if nat_eq(a676, a677) {
                (true, List::cons(v2, {
                    let (_, h678) = v1;
                    h678
                }))
            } else {
                (false, List::cons(v2, {
                    let (_, h679) = v1;
                    h679
                }))
            }
        }
    }
}

pub fn f124(v0: Fn28, p683: Option<u64>, p684: List<Adt8>) -> Option<u64> {
    let m682 = (p683, p684);
    {
        let mut m682 = m682;
        loop {
            let step = {
                let (v1, v2) = m682;
                let m680 = v2;
                match m680.uncons() {
                    None => {
                        Err::<(Option<u64>, List<Adt8>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<Adt8>), Option<u64>>(({
                            let c681: Fn28 = v0.clone();
                            c681.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: u64, v1: Option<u64>, v2: Adt8) -> Option<u64> {
    let a686 = match v2.clone() {
        Adt8::C0(h685, _) => {
            h685
        }
    };
    let a687 = v0;
    if nat_eq(a686, a687) {
        Some(match v2 {
            Adt8::C0(_, h688) => {
                h688
            }
        })
    } else {
        v1
    }
}

pub fn f126(v0: Fn29, p692: Bytes, p693: List<Adt4>) -> Bytes {
    let m691 = (p692, p693);
    {
        let mut m691 = m691;
        loop {
            let step = {
                let (v1, v2) = m691;
                let m689 = v2;
                match m689.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt4>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt4>), Bytes>(({
                            let c690: Fn29 = v0.clone();
                            c690.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: Bytes, v1: Adt4) -> Bytes {
    f21(v0, {
        let m694 = v1;
        match m694 {
            Adt4::C0(v2) => {
                f21(f3(Str::lit("F")), f58(v2))
            }
            Adt4::C1(v3) => {
                f21(f3(Str::lit("I")), f75(v3))
            }
        }
    })
}

pub fn f128(v0: Adt7, v1: Adt18) -> R<Option<Adt18>> {
    let m696 = f31(v0, match v1.clone() {
        Adt18::C0(_, h695, _) => {
            h695
        }
    });
    match m696 {
        None => {
            Ok(None::<Adt18>)
        }
        Some(v2) => {
            let v3: u64 = f18(v2.clone());
            if f56({
                let a699 = match v2.clone() {
                    Adt0::C0(_, h697, _, _, _, _, _, _) => {
                        h697
                    }
                };
                let a700 = match v1.clone() {
                    Adt18::C0(_, _, h698) => {
                        h698
                    }
                };
                nat_eq(a699, a700)
            }, {
                let a701 = v3;
                let a702 = 1u64;
                if if nat_eq(a701, a702) {
                    true
                } else {
                    let a703 = v3;
                    let a704 = 2u64;
                    nat_eq(a703, a704)
                } {
                    true
                } else {
                    f56({
                        let a705 = v3;
                        let a706 = 4u64;
                        nat_eq(a705, a706)
                    }, if f19(match v2.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, h707) => {
                            h707
                        }
                    }, f3(Str::lit("_"))) {
                        true
                    } else {
                        f19(match v2.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, h708) => {
                                h708
                            }
                        }, f3(Str::lit("'")))
                    })
                }
            }) {
                Ok(Some(Adt18::C0(f21(match v1.clone() {
                    Adt18::C0(h709, _, _) => {
                        h709
                    }
                }, match v2.clone() {
                    Adt0::C0(_, _, _, _, _, _, _, h710) => {
                        h710
                    }
                }), {
                    let a712 = match v1 {
                        Adt18::C0(_, h711, _) => {
                            h711
                        }
                    };
                    let a713 = 1u64;
                    nat_add(a712, a713)?
                }, match v2 {
                    Adt0::C0(_, _, h714, _, _, _, _, _) => {
                        h714
                    }
                })))
            } else {
                Ok(None::<Adt18>)
            }
        }
    }
}
