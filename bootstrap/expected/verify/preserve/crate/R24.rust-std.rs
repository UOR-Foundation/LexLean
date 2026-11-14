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
    C0(u64),
    C1(u64, u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt4, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(List<Adt2>),
    C1(Adt2),
    C2(Adt2),
    C3,
    C4(Adt2),
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, Bytes, Option<Adt7>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F19(List<Adt0>, List<Adt2>),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn0::F19(k0, k1) => f19(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F22(List<Adt0>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt10, p1: Adt2) -> R<Adt10> {
        match self {
            Fn1::F22(k0) => f22(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F26,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn2::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F36(List<Adt0>),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt11) -> R<Option<Adt11>> {
        match self {
            Fn3::F36(k0) => f36(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F38(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn4::F38(k0) => f38(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F40(Bytes, u64, Bytes, u64),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F40(k0, k1, k2, k3) => f40(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>, v2: List<Adt2>) -> R<Result<Adt3, Adt6>> {
    let v3: u64 = {
        let a1 = v2.clone();
        length_list(a1)
    };
    let v4: Result<Adt3, Adt6> = Err::<Adt3, Adt6>(f1(v0, v1.clone(), v2.clone())?);
    if f2(v1.clone(), v2.clone(), 0u64, f3(Str::lit("Assume"))) {
        f4(v1, v2, v3, v4)
    } else {
        if f2(v1.clone(), v2.clone(), 0u64, f3(Str::lit("Apply"))) {
            if f5(f6(v2.clone(), 1u64), {
                let a2 = v3;
                let a3 = 2u64;
                nat_eq(a2, a3)
            }) {
                Ok(f7(Adt4::C1(f8(v2.clone(), 1u64)), List::cons(f9(v2, 0u64, f3(Str::lit("apply-verb"))), List::<Adt5>::nil())))
            } else {
                Ok(v4)
            }
        } else {
            if f2(v1.clone(), v2.clone(), 0u64, f3(Str::lit("Close"))) {
                Ok(f10(v1, v2, v3, v4))
            } else {
                if f2(v1.clone(), v2.clone(), 0u64, f3(Str::lit("Use"))) {
                    if f5(f6(v2.clone(), 1u64), f5(f2(v1.clone(), v2.clone(), 2u64, f3(Str::lit("as"))), f5(f2(v1.clone(), v2.clone(), 3u64, f3(Str::lit("the"))), f5(f2(v1, v2.clone(), 4u64, f3(Str::lit("witness"))), {
                        let a4 = v3;
                        let a5 = 5u64;
                        nat_eq(a4, a5)
                    })))) {
                        Ok(f7(Adt4::C4(f8(v2.clone(), 1u64)), List::cons(f9(v2.clone(), 0u64, f3(Str::lit("use"))), List::cons(f9(v2.clone(), 2u64, f3(Str::lit("as"))), List::cons(f9(v2.clone(), 3u64, f3(Str::lit("the"))), List::cons(f9(v2, 4u64, f3(Str::lit("witness"))), List::<Adt5>::nil()))))))
                    } else {
                        Ok(v4)
                    }
                } else {
                    if f2(v1.clone(), v2.clone(), 0u64, f3(Str::lit("Select"))) {
                        Ok(f11(v1, v2, v3, v4))
                    } else {
                        Ok(v4)
                    }
                }
            }
        }
    }
}

pub fn f1(v0: Bytes, v1: List<Adt0>, v2: List<Adt2>) -> R<Adt6> {
    let m6 = f12(v1.clone(), v2.clone())?;
    match m6 {
        None => {
            let v4: Bytes = f3(Str::lit("not a registered proof sentence; the exact simple sentences are fixed (\u{a7}16.2)"));
            let m11 = {
                let a9 = v2;
                let a10 = 0u64;
                index_list(a9, a10)
            };
            match m11 {
                None => {
                    Ok(Adt6::C0(f3(Str::lit("LLF5005")), v4, Some(Adt7::C0(v0, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64)), None::<Bytes>))
                }
                Some(v5) => {
                    Ok(f13(v0, f3(Str::lit("LLF5005")), v4, f15(v1, f16(v5))))
                }
            }
        }
        Some(v3) => {
            Ok(f13(v0, f3(Str::lit("LLF5005")), f14(f14(f3(Str::lit("`")), match v3.clone() {
                Adt9::C0(h7, _) => {
                    h7
                }
            }), f3(Str::lit("` is a forbidden proof form (\u{a7}16.12): raw tactics, automation, tactic combinators, and proof holes are never accepted"))), f15(v1, match v3 {
                Adt9::C0(_, h8) => {
                    h8
                }
            })))
        }
    }
}

pub fn f2(v0: List<Adt0>, v1: List<Adt2>, v2: u64, v3: Bytes) -> bool {
    let m14 = {
        let a12 = v1;
        let a13 = v2;
        index_list(a12, a13)
    };
    match m14 {
        None => {
            false
        }
        Some(v4) => {
            let m15 = v4;
            match m15 {
                Adt2::C0(v5) => {
                    let m18 = {
                        let a16 = v0;
                        let a17 = v5;
                        index_list(a16, a17)
                    };
                    match m18 {
                        None => {
                            false
                        }
                        Some(v6) => {
                            f5({
                                let a19 = f17(v6.clone());
                                let a20 = 1u64;
                                nat_eq(a19, a20)
                            }, {
                                let a22 = match v6 {
                                    Adt0::C0(_, _, _, _, _, _, _, h21) => {
                                        h21
                                    }
                                };
                                let a23 = v3;
                                equal(a22, a23)
                            })
                        }
                    }
                }
                Adt2::C1(_, _, _, _, _) => {
                    false
                }
            }
        }
    }
}

pub fn f3(v0: Str) -> Bytes {
    let a24 = v0;
    utf8_encode(a24)
}

pub fn f4(v0: List<Adt0>, v1: List<Adt2>, v2: u64, v3: Result<Adt3, Adt6>) -> R<Result<Adt3, Adt6>> {
    if f6(v1.clone(), 1u64) {
        let v4: Adt8 = {
            let (h25, _) = f18(Fn0::F19(v0, v1.clone()), {
                let a26 = v2;
                let a27 = 1u64;
                nat_add(a26, a27)?
            }, Adt8::C0(2u64, List::cons(f8(v1.clone(), 1u64), List::<Adt2>::nil())))?;
            h25
        };
        let a29 = match v4.clone() {
            Adt8::C0(h28, _) => {
                h28
            }
        };
        let a30 = v2;
        if nat_eq(a29, a30) {
            Ok(f7(Adt4::C0(f20(match v4 {
                Adt8::C0(_, h31) => {
                    h31
                }
            })), List::cons(f9(v1, 0u64, f3(Str::lit("assume"))), List::<Adt5>::nil())))
        } else {
            Ok(v3)
        }
    } else {
        Ok(v3)
    }
}

pub fn f5(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f6(v0: List<Adt2>, v1: u64) -> bool {
    let m34 = {
        let a32 = v0;
        let a33 = v1;
        index_list(a32, a33)
    };
    match m34 {
        None => {
            false
        }
        Some(v2) => {
            let m35 = v2;
            match m35 {
                Adt2::C0(_) => {
                    false
                }
                Adt2::C1(_, _, _, _, _) => {
                    true
                }
            }
        }
    }
}

pub fn f7(v0: Adt4, v1: List<Adt5>) -> Result<Adt3, Adt6> {
    Ok::<Adt3, Adt6>(Adt3::C0(v0, v1))
}

pub fn f8(v0: List<Adt2>, v1: u64) -> Adt2 {
    let m38 = {
        let a36 = v0;
        let a37 = v1;
        index_list(a36, a37)
    };
    match m38 {
        None => {
            Adt2::C0(0u64)
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f9(v0: List<Adt2>, v1: u64, v2: Bytes) -> Adt5 {
    Adt5::C0(f16(f8(v0, v1)), v2)
}

pub fn f10(v0: List<Adt0>, v1: List<Adt2>, v2: u64, v3: Result<Adt3, Adt6>) -> Result<Adt3, Adt6> {
    if f5(f2(v0.clone(), v1.clone(), 1u64, f3(Str::lit("the"))), f2(v0.clone(), v1.clone(), 2u64, f3(Str::lit("goal")))) {
        let v4: List<Adt5> = List::cons(f9(v1.clone(), 0u64, f3(Str::lit("close"))), List::cons(f9(v1.clone(), 1u64, f3(Str::lit("the"))), List::cons(f9(v1.clone(), 2u64, f3(Str::lit("goal"))), List::<Adt5>::nil())));
        if f2(v0.clone(), v1.clone(), 3u64, f3(Str::lit("with"))) {
            if f5(f6(v1.clone(), 4u64), {
                let a39 = v2;
                let a40 = 5u64;
                nat_eq(a39, a40)
            }) {
                f7(Adt4::C2(f8(v1.clone(), 4u64)), {
                    let a41 = v4;
                    let a42 = List::cons(f9(v1, 3u64, f3(Str::lit("with"))), List::<Adt5>::nil());
                    append_list(a41, a42)
                })
            } else {
                v3
            }
        } else {
            if f5(f2(v0.clone(), v1.clone(), 3u64, f3(Str::lit("by"))), f5(f2(v0, v1.clone(), 4u64, f3(Str::lit("reflexivity"))), {
                let a43 = v2;
                let a44 = 5u64;
                nat_eq(a43, a44)
            })) {
                f7(Adt4::C3, {
                    let a45 = v4;
                    let a46 = List::cons(f9(v1.clone(), 3u64, f3(Str::lit("by"))), List::cons(f9(v1, 4u64, f3(Str::lit("reflexivity"))), List::<Adt5>::nil()));
                    append_list(a45, a46)
                })
            } else {
                v3
            }
        }
    } else {
        v3
    }
}

pub fn f11(v0: List<Adt0>, v1: List<Adt2>, v2: u64, v3: Result<Adt3, Adt6>) -> Result<Adt3, Adt6> {
    if f2(v0.clone(), v1.clone(), 1u64, f3(Str::lit("the"))) {
        let v4: bool = f2(v0.clone(), v1.clone(), 2u64, f3(Str::lit("left")));
        let v5: bool = if v4 {
            false
        } else {
            f2(v0.clone(), v1.clone(), 2u64, f3(Str::lit("right")))
        };
        if f5(if v4 {
            true
        } else {
            v5
        }, f5(f2(v0, v1.clone(), 3u64, f3(Str::lit("alternative"))), {
            let a47 = v2;
            let a48 = 4u64;
            nat_eq(a47, a48)
        })) {
            f7(if v4 {
                Adt4::C5
            } else {
                Adt4::C6
            }, List::cons(f9(v1.clone(), 0u64, f3(Str::lit("select"))), List::cons(f9(v1.clone(), 1u64, f3(Str::lit("the"))), List::cons(f9(v1.clone(), 2u64, if v4 {
                f3(Str::lit("left"))
            } else {
                f3(Str::lit("right"))
            }), List::cons(f9(v1, 3u64, f3(Str::lit("alternative"))), List::<Adt5>::nil())))))
        } else {
            v3
        }
    } else {
        v3
    }
}

pub fn f12(v0: List<Adt0>, v1: List<Adt2>) -> R<Option<Adt9>> {
    let m50 = f21(Fn1::F22(v0), Adt10::C0(0u64, None::<Adt9>), v1)?;
    match m50 {
        Adt10::C0(_, h49) => {
            Ok(h49)
        }
    }
}

pub fn f13(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt0) -> Adt6 {
    Adt6::C0(v1, v2, Some(f23(v0, v3)), None::<Bytes>)
}

pub fn f14(v0: Bytes, v1: Bytes) -> Bytes {
    let a51 = v0;
    let a52 = v1;
    append_bytes(a51, a52)
}

pub fn f15(v0: List<Adt0>, v1: u64) -> Adt0 {
    let m55 = {
        let a53 = v0;
        let a54 = v1;
        index_list(a53, a54)
    };
    match m55 {
        None => {
            Adt0::C0(Adt1::C6, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64, f3(Str::lit("")))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f16(v0: Adt2) -> u64 {
    let m56 = v0;
    match m56 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f17(v0: Adt0) -> u64 {
    let m58 = match v0 {
        Adt0::C0(h57, _, _, _, _, _, _, _) => {
            h57
        }
    };
    match m58 {
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

pub fn f18(v0: Fn0, p63: u64, p64: Adt8) -> R<(Adt8, bool)> {
    let m62 = (p63, p64);
    {
        let mut m62 = m62;
        loop {
            let step = {
                let (v1, v2) = m62;
                let m59 = v1;
                if m59 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m59 - 1;
                    let m61 = {
                        let c60: Fn0 = v0.clone();
                        c60.apply(v2.clone())?
                    };
                    match m61 {
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
                Ok(next) => { m62 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: List<Adt0>, v1: List<Adt2>, v2: Adt8) -> R<Option<Adt8>> {
    f24(v0, v1, v2)
}

pub fn f20(v0: List<Adt2>) -> List<Adt2> {
    f25(Fn2::F26, List::<Adt2>::nil(), v0)
}

pub fn f21(v0: Fn1, p68: Adt10, p69: List<Adt2>) -> R<Adt10> {
    let m67 = (p68, p69);
    {
        let mut m67 = m67;
        loop {
            let step = {
                let (v1, v2) = m67;
                let m65 = v2;
                match m65.uncons() {
                    None => {
                        Err::<(Adt10, List<Adt2>), R<Adt10>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt10, List<Adt2>), R<Adt10>>(({
                            let c66: Fn1 = v0.clone();
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

pub fn f22(v0: List<Adt0>, v1: Adt10, v2: Adt2) -> R<Adt10> {
    f27(v0, v1, v2)
}

pub fn f23(v0: Bytes, v1: Adt0) -> Adt7 {
    Adt7::C0(v0, match v1.clone() {
        Adt0::C0(_, h70, _, _, _, _, _, _) => {
            h70
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h71, _, _, _, _, _) => {
            h71
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h72, _, _, _, _) => {
            h72
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h73, _, _, _) => {
            h73
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h74, _, _) => {
            h74
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, h75, _) => {
            h75
        }
    })
}

pub fn f24(v0: List<Adt0>, v1: List<Adt2>, v2: Adt8) -> R<Option<Adt8>> {
    let v3: u64 = match v2.clone() {
        Adt8::C0(h76, _) => {
            h76
        }
    };
    if f5(f28(v0, v1.clone(), v3), f6(v1.clone(), {
        let a77 = v3;
        let a78 = 1u64;
        nat_add(a77, a78)?
    })) {
        Ok(Some(Adt8::C0({
            let a79 = v3;
            let a80 = 2u64;
            nat_add(a79, a80)?
        }, List::cons(f8(v1, {
            let a81 = v3;
            let a82 = 1u64;
            nat_add(a81, a82)?
        }), match v2 {
            Adt8::C0(_, h83) => {
                h83
            }
        }))))
    } else {
        Ok(None::<Adt8>)
    }
}

pub fn f25(v0: Fn2, p87: List<Adt2>, p88: List<Adt2>) -> List<Adt2> {
    let m86 = (p87, p88);
    {
        let mut m86 = m86;
        loop {
            let step = {
                let (v1, v2) = m86;
                let m84 = v2;
                match m84.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c85: Fn2 = v0.clone();
                            c85.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m86 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f27(v0: List<Adt0>, v1: Adt10, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt10::C0(h89, _) => {
            h89
        }
    };
    let m91 = match v1.clone() {
        Adt10::C0(_, h90) => {
            h90
        }
    };
    match m91 {
        None => {
            let m95 = v2;
            match m95 {
                Adt2::C0(v5) => {
                    Ok(Adt10::C0({
                        let a96 = v3;
                        let a97 = 1u64;
                        nat_add(a96, a97)?
                    }, f29(v0, v3, v5)?))
                }
                Adt2::C1(_, _, _, _, _) => {
                    Ok(Adt10::C0({
                        let a98 = v3;
                        let a99 = 1u64;
                        nat_add(a98, a99)?
                    }, None::<Adt9>))
                }
            }
        }
        Some(_) => {
            Ok(Adt10::C0({
                let a92 = v3;
                let a93 = 1u64;
                nat_add(a92, a93)?
            }, match v1 {
                Adt10::C0(_, h94) => {
                    h94
                }
            }))
        }
    }
}

pub fn f28(v0: List<Adt0>, v1: List<Adt2>, v2: u64) -> bool {
    let m102 = {
        let a100 = v1;
        let a101 = v2;
        index_list(a100, a101)
    };
    match m102 {
        None => {
            false
        }
        Some(v3) => {
            let m103 = v3;
            match m103 {
                Adt2::C0(v4) => {
                    let m106 = {
                        let a104 = v0;
                        let a105 = v4;
                        index_list(a104, a105)
                    };
                    match m106 {
                        None => {
                            false
                        }
                        Some(v5) => {
                            f5({
                                let a107 = f17(v5.clone());
                                let a108 = 4u64;
                                nat_eq(a107, a108)
                            }, {
                                let a110 = match v5 {
                                    Adt0::C0(_, _, _, _, _, _, _, h109) => {
                                        h109
                                    }
                                };
                                let a111 = f3(Str::lit(","));
                                equal(a110, a111)
                            })
                        }
                    }
                }
                Adt2::C1(_, _, _, _, _) => {
                    false
                }
            }
        }
    }
}

pub fn f29(v0: List<Adt0>, v1: u64, v2: u64) -> R<Option<Adt9>> {
    let v3: Adt0 = f15(v0.clone(), v2);
    let v4: Bytes = f30(v0, v3.clone(), v2)?;
    let v5: Adt9 = Adt9::C0(v4.clone(), v2);
    if f31(v4.clone(), f32()) {
        Ok(Some(v5))
    } else {
        if f5({
            let a112 = v1;
            let a113 = 0u64;
            nat_eq(a112, a113)
        }, {
            let m114 = f33(v4.clone(), f3(Str::lit("_")))?;
            match m114 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }) {
            Ok(Some(v5))
        } else {
            if f5({
                let a115 = f17(v3.clone());
                let a116 = 4u64;
                nat_eq(a115, a116)
            }, if f34(v4, f3(Str::lit("<;>"))) {
                true
            } else {
                let a118 = match v3 {
                    Adt0::C0(_, _, _, _, _, _, _, h117) => {
                        h117
                    }
                };
                let a119 = f3(Str::lit("\u{b7}"));
                equal(a118, a119)
            }) {
                Ok(Some(v5))
            } else {
                Ok(None::<Adt9>)
            }
        }
    }
}

pub fn f30(v0: List<Adt0>, v1: Adt0, v2: u64) -> R<Bytes> {
    let m129 = {
        let (h121, _) = f35(Fn3::F36(v0.clone()), {
            let a123 = {
                let a122 = v0;
                length_list(a122)
            };
            let a124 = 1u64;
            nat_add(a123, a124)?
        }, Adt11::C0(match v1.clone() {
            Adt0::C0(_, _, _, _, _, _, _, h125) => {
                h125
            }
        }, match v1 {
            Adt0::C0(_, _, h126, _, _, _, _, _) => {
                h126
            }
        }, {
            let a127 = v2;
            let a128 = 1u64;
            nat_add(a127, a128)?
        }))?;
        h121
    };
    match m129 {
        Adt11::C0(h120, _, _) => {
            Ok(h120)
        }
    }
}

pub fn f31(v0: Bytes, v1: List<Bytes>) -> bool {
    f37(Fn4::F38(v0), false, v1)
}

pub fn f32() -> List<Bytes> {
    List::cons(f3(Str::lit("aesop")), List::cons(f3(Str::lit("admit")), List::cons(f3(Str::lit("cases")), List::cons(f3(Str::lit("constructor")), List::cons(f3(Str::lit("induction")), List::cons(f3(Str::lit("all_goals")), List::cons(f3(Str::lit("any_goals")), List::cons(f3(Str::lit("apply?")), List::cons(f3(Str::lit("assumption")), List::cons(f3(Str::lit("case")), List::cons(f3(Str::lit("decide")), List::cons(f3(Str::lit("exact")), List::cons(f3(Str::lit("exact?")), List::cons(f3(Str::lit("first")), List::cons(f3(Str::lit("intro")), List::cons(f3(Str::lit("linarith")), List::cons(f3(Str::lit("native_decide")), List::cons(f3(Str::lit("next")), List::cons(f3(Str::lit("omega")), List::cons(f3(Str::lit("refine")), List::cons(f3(Str::lit("repeat")), List::cons(f3(Str::lit("rfl")), List::cons(f3(Str::lit("rw")), List::cons(f3(Str::lit("rwa")), List::cons(f3(Str::lit("simp")), List::cons(f3(Str::lit("simp_all")), List::cons(f3(Str::lit("simpa")), List::cons(f3(Str::lit("sorry")), List::cons(f3(Str::lit("tauto")), List::cons(f3(Str::lit("trivial")), List::cons(f3(Str::lit("try")), List::<Bytes>::nil())))))))))))))))))))))))))))))))
}

pub fn f33(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a130 = v0.clone();
        length_bytes(a130)
    };
    let v3: u64 = {
        let a131 = v1.clone();
        length_bytes(a131)
    };
    let v4: u64 = {
        let (h132, _) = f39(Fn5::F40(v1.clone(), v3, v0.clone(), v2), {
            let a133 = v2;
            let a134 = 1u64;
            nat_add(a133, a134)?
        }, 0u64)?;
        h132
    };
    if f41(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f34(v0: Bytes, v1: Bytes) -> bool {
    let m139 = {
        let a136 = v0;
        let a137 = 0u64;
        let a138 = {
            let a135 = v1.clone();
            length_bytes(a135)
        };
        slice_bytes(a136, a137, a138)
    };
    match m139 {
        None => {
            false
        }
        Some(v2) => {
            let a140 = v2;
            let a141 = v1;
            equal(a140, a141)
        }
    }
}

pub fn f35(v0: Fn3, p146: u64, p147: Adt11) -> R<(Adt11, bool)> {
    let m145 = (p146, p147);
    {
        let mut m145 = m145;
        loop {
            let step = {
                let (v1, v2) = m145;
                let m142 = v1;
                if m142 == 0 {
                    Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m142 - 1;
                    let m144 = {
                        let c143: Fn3 = v0.clone();
                        c143.apply(v2.clone())?
                    };
                    match m144 {
                        None => {
                            Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt11), R<(Adt11, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: List<Adt0>, v1: Adt11) -> R<Option<Adt11>> {
    f42(v0, v1)
}

pub fn f37(v0: Fn4, p151: bool, p152: List<Bytes>) -> bool {
    let m150 = (p151, p152);
    {
        let mut m150 = m150;
        loop {
            let step = {
                let (v1, v2) = m150;
                let m148 = v2;
                match m148.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c149: Fn4 = v0.clone();
                            c149.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a153 = v2;
        let a154 = v0;
        equal(a153, a154)
    }
}

pub fn f39(v0: Fn5, p159: u64, p160: u64) -> R<(u64, bool)> {
    let m158 = (p159, p160);
    {
        let mut m158 = m158;
        loop {
            let step = {
                let (v1, v2) = m158;
                let m155 = v1;
                if m155 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m155 - 1;
                    let m157 = {
                        let c156: Fn5 = v0.clone();
                        c156.apply(v2)?
                    };
                    match m157 {
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
                Ok(next) => { m158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f41(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a163 = {
            let a161 = v4;
            let a162 = v1;
            nat_add(a161, a162)?
        };
        let a164 = v3;
        if nat_lt(a163, a164) {
            Ok(Some({
                let a165 = v4;
                let a166 = 1u64;
                nat_add(a165, a166)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f41(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m171 = {
        let a168 = v0;
        let a169 = v2;
        let a170 = {
            let a167 = v1.clone();
            length_bytes(a167)
        };
        slice_bytes(a168, a169, a170)
    };
    match m171 {
        None => {
            false
        }
        Some(v3) => {
            let a172 = v3;
            let a173 = v1;
            equal(a172, a173)
        }
    }
}

pub fn f42(v0: List<Adt0>, v1: Adt11) -> R<Option<Adt11>> {
    let m177 = {
        let a175 = v0;
        let a176 = match v1.clone() {
            Adt11::C0(_, _, h174) => {
                h174
            }
        };
        index_list(a175, a176)
    };
    match m177 {
        None => {
            Ok(None::<Adt11>)
        }
        Some(v2) => {
            let v3: u64 = f17(v2.clone());
            if f5({
                let a180 = match v2.clone() {
                    Adt0::C0(_, h178, _, _, _, _, _, _) => {
                        h178
                    }
                };
                let a181 = match v1.clone() {
                    Adt11::C0(_, h179, _) => {
                        h179
                    }
                };
                nat_eq(a180, a181)
            }, f5({
                let a182 = v3;
                let a183 = 1u64;
                if nat_eq(a182, a183) {
                    true
                } else {
                    let a184 = v3;
                    let a185 = 2u64;
                    if nat_eq(a184, a185) {
                        true
                    } else {
                        let a186 = v3;
                        let a187 = 4u64;
                        nat_eq(a186, a187)
                    }
                }
            }, !f43(match v2.clone() {
                Adt0::C0(_, _, _, _, _, _, _, h188) => {
                    h188
                }
            }))) {
                Ok(Some(Adt11::C0(f14(match v1.clone() {
                    Adt11::C0(h189, _, _) => {
                        h189
                    }
                }, match v2.clone() {
                    Adt0::C0(_, _, _, _, _, _, _, h190) => {
                        h190
                    }
                }), match v2 {
                    Adt0::C0(_, _, h191, _, _, _, _, _) => {
                        h191
                    }
                }, {
                    let a193 = match v1 {
                        Adt11::C0(_, _, h192) => {
                            h192
                        }
                    };
                    let a194 = 1u64;
                    nat_add(a193, a194)?
                })))
            } else {
                Ok(None::<Adt11>)
            }
        }
    }
}

pub fn f43(v0: Bytes) -> bool {
    f31(v0, List::cons(f3(Str::lit(",")), List::cons(f3(Str::lit(".")), List::cons(f3(Str::lit(";")), List::cons(f3(Str::lit(":")), List::cons(f3(Str::lit("(")), List::cons(f3(Str::lit(")")), List::<Bytes>::nil())))))))
}
