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
    C0(Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, List<Bytes>, List<Adt1>, List<Adt2>, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Bytes, Bytes, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt3>),
    C4(std::rc::Rc<Adt3>),
    C5,
    C6(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C7(Bytes, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Fn0 {
    F19,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3, p1: Bytes) -> Adt3 {
        match self {
            Fn0::F19 => f19(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F22,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt3, p1: Adt1) -> Adt3 {
        match self {
            Fn1::F22 => f22(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F25,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt3, p1: Adt2) -> R<Adt3> {
        match self {
            Fn2::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F30(Bytes),
    F37(Bytes),
    F70(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F30(k0) => f30(k0.clone(), p0),
            Fn3::F37(k0) => f37(k0.clone(), p0),
            Fn3::F70(k0, k1) => Ok(f70(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F33,
    F116,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn4::F33 => f33(p0, p1),
            Fn4::F116 => f116(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F36,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn5::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F39,
    F105,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F39 => f39(p0, p1),
            Fn6::F105 => f105(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F42,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn7::F42 => f42(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F45,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn8::F45 => f45(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F47,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt2>, p1: (Bytes, Adt2)) -> List<Adt2> {
        match self {
            Fn9::F47 => f47(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F51,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: Adt2) -> R<List<(Bytes, Adt2)>> {
        match self {
            Fn10::F51 => f51(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F56(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn11::F56(k0) => f56(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F62,
}

impl Fn12 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
        match self {
            Fn12::F62 => f62(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F65,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn13::F65 => f65(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F67,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
        match self {
            Fn14::F67 => f67(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F74,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F74 => f74(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F76,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F76 => f76(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F79,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn17::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F87,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn18::F87 => f87(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F93,
}

impl Fn19 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn19::F93 => f93(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F96,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
        match self {
            Fn20::F96 => f96(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F98(u8, Bytes),
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn21::F98(k0, k1) => f98(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F102,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F102 => f102(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F111,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
        match self {
            Fn23::F111 => f111(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F114,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F114 => f114(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0)?)
}

pub fn f1(v0: Adt3) -> R<Bytes> {
    let a1 = f3(v0)?;
    let a2 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1, a2))
}

pub fn f2(v0: Adt0) -> R<Adt3> {
    Ok(Adt3::C4(std::rc::Rc::new(f4(Str::lit("spec"), f5(f6(Str::lit("lexlean/build-manifest/1"))), f4(Str::lit("compiler"), Adt3::C4(std::rc::Rc::new(f4(Str::lit("version"), f5(match v0.clone() {
        Adt0::C0(_, h3, _, _, _, _, _, _, _, _, _) => {
            h3
        }
    }), f4(Str::lit("semantics_id"), f7(match v0.clone() {
        Adt0::C0(_, _, h4, _, _, _, _, _, _, _, _) => {
            h4
        }
    })?, Adt3::C5)))), f4(Str::lit("language"), f5(match v0.clone() {
        Adt0::C0(h5, _, _, _, _, _, _, _, _, _, _) => {
            h5
        }
    }), f4(Str::lit("project"), f5(match v0.clone() {
        Adt0::C0(_, _, _, h6, _, _, _, _, _, _, _) => {
            h6
        }
    }), f4(Str::lit("source_id"), f7(match v0.clone() {
        Adt0::C0(_, _, _, _, h7, _, _, _, _, _, _) => {
            h7
        }
    })?, f4(Str::lit("semantic_id"), f7(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h8, _, _, _, _, _) => {
            h8
        }
    })?, f4(Str::lit("build_id"), f7(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h9, _, _, _, _) => {
            h9
        }
    })?, f4(Str::lit("lean_toolchain"), f5(f6(Str::lit("leanprover/lean4:v4.32.2"))), f4(Str::lit("selection"), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h10, _, _, _) => {
            h10
        }
    }), f4(Str::lit("modules"), f9(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h11, _, _) => {
            h11
        }
    }), f4(Str::lit("inputs"), f10(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h12, _) => {
            h12
        }
    })?, f4(Str::lit("outputs"), f10(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h13) => {
            h13
        }
    })?, Adt3::C5)))))))))))))))
}

pub fn f3(v0: Adt3) -> R<Bytes> {
    let m14 = v0;
    match m14 {
        Adt3::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt3::C1(v2, v3) => {
            f11(v2, v3)
        }
        Adt3::C2(v4) => {
            f12(v4)
        }
        Adt3::C3(r15) => {
            let v5 = (*r15).clone();
            Ok(f13(f14(v5)?))
        }
        Adt3::C4(r16) => {
            let v6 = (*r16).clone();
            f15(f16(v6)?)
        }
        Adt3::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt3::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt3::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Str, v1: Adt3, v2: Adt3) -> Adt3 {
    Adt3::C7(f6(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f5(v0: Bytes) -> Adt3 {
    Adt3::C2(v0)
}

pub fn f6(v0: Str) -> Bytes {
    let a17 = v0;
    utf8_encode(a17)
}

pub fn f7(v0: Bytes) -> R<Adt3> {
    Ok(f5(f17(v0)?))
}

pub fn f8(v0: List<Bytes>) -> Adt3 {
    Adt3::C3(std::rc::Rc::new(f18(Fn0::F19, Adt3::C5, f20(v0))))
}

pub fn f9(v0: List<Adt1>) -> Adt3 {
    Adt3::C3(std::rc::Rc::new(f21(Fn1::F22, Adt3::C5, f23(v0))))
}

pub fn f10(v0: List<Adt2>) -> R<Adt3> {
    Ok(Adt3::C3(std::rc::Rc::new(f24(Fn2::F25, Adt3::C5, f26(f27(v0)?))?)))
}

pub fn f11(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a18 = 0u64;
        let a19 = v1;
        nat_lt(a18, a19)
    } else {
        false
    } {
        let a20 = Bytes::lit(&[0x2d]);
        let a21 = f28(v1)?;
        Ok(append_bytes(a20, a21))
    } else {
        f28(v1)
    }
}

pub fn f12(v0: Bytes) -> R<Bytes> {
    let a27 = {
        let a25 = Bytes::lit(&[0x22]);
        let a26 = {
            let (_, h22) = {
                let (h23, _) = f29(Fn3::F30(v0.clone()), {
                    let a24 = v0;
                    length_bytes(a24)
                }, (0u64, Bytes::lit(&[])))?;
                h23
            };
            h22
        };
        append_bytes(a25, a26)
    };
    let a28 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a27, a28))
}

pub fn f13(v0: List<Bytes>) -> Bytes {
    let a31 = {
        let a29 = Bytes::lit(&[0x5b]);
        let a30 = f31(v0, Bytes::lit(&[0x2c]));
        append_bytes(a29, a30)
    };
    let a32 = Bytes::lit(&[0x5d]);
    append_bytes(a31, a32)
}

pub fn f14(v0: Adt3) -> R<List<Bytes>> {
    let m33 = v0;
    match m33 {
        Adt3::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C6(r34, r35) => {
            let v7 = (*r34).clone();
            let v8 = (*r35).clone();
            Ok(List::cons(f3(v7)?, f14(v8)?))
        }
        Adt3::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f15(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f32(Fn4::F33, List::<(Bytes, Bytes)>::nil(), f34(v0)?);
    let v2: List<Bytes> = f35(Fn5::F36, List::<Bytes>::nil(), v1)?;
    let a38 = {
        let a36 = Bytes::lit(&[0x7b]);
        let a37 = f31(v2, Bytes::lit(&[0x2c]));
        append_bytes(a36, a37)
    };
    let a39 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a38, a39))
}

pub fn f16(v0: Adt3) -> R<List<(Bytes, Bytes)>> {
    let m40 = v0;
    match m40 {
        Adt3::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C7(v9, r41, r42) => {
            let v10 = (*r41).clone();
            let v11 = (*r42).clone();
            Ok(List::cons((v9, f3(v10)?), f16(v11)?))
        }
    }
}

pub fn f17(v0: Bytes) -> R<Bytes> {
    let (_, h43) = {
        let (h44, _) = f29(Fn3::F37(v0.clone()), {
            let a45 = v0;
            length_bytes(a45)
        }, (0u64, Bytes::lit(&[])))?;
        h44
    };
    Ok(h43)
}

pub fn f18(v0: Fn0, p49: Adt3, p50: List<Bytes>) -> Adt3 {
    let m48 = (p49, p50);
    {
        let mut m48 = m48;
        loop {
            let step = {
                let (v1, v2) = m48;
                let m46 = v2;
                match m46.uncons() {
                    None => {
                        Err::<(Adt3, List<Bytes>), Adt3>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Bytes>), Adt3>(({
                            let c47: Fn0 = v0.clone();
                            c47.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m48 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt3, v1: Bytes) -> Adt3 {
    Adt3::C6(std::rc::Rc::new(f5(v1)), std::rc::Rc::new(v0))
}

pub fn f20(v0: List<Bytes>) -> List<Bytes> {
    f38(Fn6::F39, List::<Bytes>::nil(), v0)
}

pub fn f21(v0: Fn1, p54: Adt3, p55: List<Adt1>) -> Adt3 {
    let m53 = (p54, p55);
    {
        let mut m53 = m53;
        loop {
            let step = {
                let (v1, v2) = m53;
                let m51 = v2;
                match m51.uncons() {
                    None => {
                        Err::<(Adt3, List<Adt1>), Adt3>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Adt1>), Adt3>(({
                            let c52: Fn1 = v0.clone();
                            c52.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m53 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Adt3, v1: Adt1) -> Adt3 {
    Adt3::C6(std::rc::Rc::new(f40(v1)), std::rc::Rc::new(v0))
}

pub fn f23(v0: List<Adt1>) -> List<Adt1> {
    f41(Fn7::F42, List::<Adt1>::nil(), v0)
}

pub fn f24(v0: Fn2, p59: Adt3, p60: List<Adt2>) -> R<Adt3> {
    let m58 = (p59, p60);
    {
        let mut m58 = m58;
        loop {
            let step = {
                let (v1, v2) = m58;
                let m56 = v2;
                match m56.uncons() {
                    None => {
                        Err::<(Adt3, List<Adt2>), R<Adt3>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Adt2>), R<Adt3>>(({
                            let c57: Fn2 = v0.clone();
                            c57.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m58 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt3, v1: Adt2) -> R<Adt3> {
    Ok(Adt3::C6(std::rc::Rc::new(f43(v1)?), std::rc::Rc::new(v0)))
}

pub fn f26(v0: List<Adt2>) -> List<Adt2> {
    f44(Fn8::F45, List::<Adt2>::nil(), v0)
}

pub fn f27(v0: List<Adt2>) -> R<List<Adt2>> {
    Ok(f46(Fn9::F47, List::<Adt2>::nil(), f48(f49(f48(f50(Fn10::F51, List::<(Bytes, Adt2)>::nil(), v0)?))?)))
}

pub fn f28(v0: u64) -> R<Bytes> {
    f52(f53(), 10u64, v0)
}

pub fn f29(v0: Fn3, p65: u64, p66: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m64 = (p65, p66);
    {
        let mut m64 = m64;
        loop {
            let step = {
                let (v1, v2) = m64;
                let m61 = v1;
                if m61 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m61 - 1;
                    let m63 = {
                        let c62: Fn3 = v0.clone();
                        c62.apply(v2.clone())?
                    };
                    match m63 {
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
                Ok(next) => { m64 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f54(v0, v1)
}

pub fn f31(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m67 = f55(Fn11::F56(v1), None::<Bytes>, v0);
    match m67 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f32(v0: Fn4, p71: List<(Bytes, Bytes)>, p72: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m70 = (p71, p72);
    {
        let mut m70 = m70;
        loop {
            let step = {
                let (v1, v2) = m70;
                let m68 = v2;
                match m68.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c69: Fn4 = v0.clone();
                            c69.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m70 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f57(v0, v1)
}

pub fn f34(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f58(v0)
}

pub fn f35(v0: Fn5, p76: List<Bytes>, p77: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m75 = (p76, p77);
    {
        let mut m75 = m75;
        loop {
            let step = {
                let (v1, v2) = m75;
                let m73 = v2;
                match m73.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c74: Fn5 = v0.clone();
                            c74.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m75 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f59(v1)?, v0))
}

pub fn f37(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f60(v0, v1)
}

pub fn f38(v0: Fn6, p81: List<Bytes>, p82: List<Bytes>) -> List<Bytes> {
    let m80 = (p81, p82);
    {
        let mut m80 = m80;
        loop {
            let step = {
                let (v1, v2) = m80;
                let m78 = v2;
                match m78.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c79: Fn6 = v0.clone();
                            c79.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m80 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f40(v0: Adt1) -> Adt3 {
    let v2: Adt3 = {
        let m84 = match v0.clone() {
            Adt1::C0(_, _, _, h83) => {
                h83
            }
        };
        match m84 {
            None => {
                Adt3::C5
            }
            Some(v1) => {
                f4(Str::lit("origin"), f5(v1), Adt3::C5)
            }
        }
    };
    Adt3::C4(std::rc::Rc::new(f4(Str::lit("module"), f5(match v0.clone() {
        Adt1::C0(h85, _, _, _) => {
            h85
        }
    }), f4(Str::lit("lean_module"), f5(match v0.clone() {
        Adt1::C0(_, h86, _, _) => {
            h86
        }
    }), f4(Str::lit("source_path"), f5(match v0 {
        Adt1::C0(_, _, h87, _) => {
            h87
        }
    }), v2)))))
}

pub fn f41(v0: Fn7, p91: List<Adt1>, p92: List<Adt1>) -> List<Adt1> {
    let m90 = (p91, p92);
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v1, v2) = m90;
                let m88 = v2;
                match m88.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c89: Fn7 = v0.clone();
                            c89.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m90 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f43(v0: Adt2) -> R<Adt3> {
    Ok(Adt3::C4(std::rc::Rc::new(f4(Str::lit("kind"), f5(match v0.clone() {
        Adt2::C0(h93, _, _, _) => {
            h93
        }
    }), f4(Str::lit("path"), f5(match v0.clone() {
        Adt2::C0(_, h94, _, _) => {
            h94
        }
    }), f4(Str::lit("byte_length"), Adt3::C1(false, match v0.clone() {
        Adt2::C0(_, _, h95, _) => {
            h95
        }
    }), f4(Str::lit("sha256"), f7(match v0 {
        Adt2::C0(_, _, _, h96) => {
            h96
        }
    })?, Adt3::C5)))))))
}

pub fn f44(v0: Fn8, p100: List<Adt2>, p101: List<Adt2>) -> List<Adt2> {
    let m99 = (p100, p101);
    {
        let mut m99 = m99;
        loop {
            let step = {
                let (v1, v2) = m99;
                let m97 = v2;
                match m97.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c98: Fn8 = v0.clone();
                            c98.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m99 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f46(v0: Fn9, p105: List<Adt2>, p106: List<(Bytes, Adt2)>) -> List<Adt2> {
    let m104 = (p105, p106);
    {
        let mut m104 = m104;
        loop {
            let step = {
                let (v1, v2) = m104;
                let m102 = v2;
                match m102.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(({
                            let c103: Fn9 = v0.clone();
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

pub fn f47(v0: List<Adt2>, v1: (Bytes, Adt2)) -> List<Adt2> {
    List::cons({
        let (_, h107) = v1;
        h107
    }, v0)
}

pub fn f48(v0: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    f61(Fn12::F62, List::<(Bytes, Adt2)>::nil(), v0)
}

pub fn f49(v0: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v1: List<List<(Bytes, Adt2)>> = f63(f64(Fn13::F65, List::<List<(Bytes, Adt2)>>::nil(), v0.clone()));
    let m112 = {
        let (h108, _) = f66(Fn14::F67, {
            let a110 = {
                let a109 = v0;
                length_list(a109)
            };
            let a111 = 1u64;
            nat_add(a110, a111)?
        }, v1)?;
        h108
    };
    match m112.uncons() {
        None => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f50(v0: Fn10, p116: List<(Bytes, Adt2)>, p117: List<Adt2>) -> R<List<(Bytes, Adt2)>> {
    let m115 = (p116, p117);
    {
        let mut m115 = m115;
        loop {
            let step = {
                let (v1, v2) = m115;
                let m113 = v2;
                match m113.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<Adt2>), R<List<(Bytes, Adt2)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<Adt2>), R<List<(Bytes, Adt2)>>>(({
                            let c114: Fn10 = v0.clone();
                            c114.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: List<(Bytes, Adt2)>, v1: Adt2) -> R<List<(Bytes, Adt2)>> {
    Ok(List::cons((f68(v1.clone())?, v1), v0))
}

pub fn f52(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a118 = v2;
    let a119 = 0u64;
    if nat_eq(a118, a119) {
        Ok(f69(v0, 0u64))
    } else {
        let (_, h120) = {
            let (h121, _) = f29(Fn3::F70(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h121
        };
        Ok(h120)
    }
}

pub fn f53() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f54(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m125 = {
        let a123 = v0.clone();
        let a124 = {
            let (h122, _) = v1.clone();
            h122
        };
        index_bytes(a123, a124)
    };
    match m125 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a127 = {
                    let (h126, _) = v1.clone();
                    h126
                };
                let a128 = 1u64;
                nat_add(a127, a128)?
            }, {
                let a131 = {
                    let (_, h129) = v1;
                    h129
                };
                let a132 = f71(v0, {
                    let (h130, _) = v1;
                    h130
                }, v2)?;
                append_bytes(a131, a132)
            })))
        }
    }
}

pub fn f55(v0: Fn11, p136: Option<Bytes>, p137: List<Bytes>) -> Option<Bytes> {
    let m135 = (p136, p137);
    {
        let mut m135 = m135;
        loop {
            let step = {
                let (v1, v2) = m135;
                let m133 = v2;
                match m133.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c134: Fn11 = v0.clone();
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

pub fn f56(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m138 = v1;
    match m138 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a141 = {
                    let a139 = v3;
                    let a140 = v0;
                    append_bytes(a139, a140)
                };
                let a142 = v2;
                append_bytes(a141, a142)
            })
        }
    }
}

pub fn f57(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m143 = v0.clone();
    match m143.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a146 = {
                let (h144, _) = v2;
                h144
            };
            let a147 = {
                let (h145, _) = v1.clone();
                h145
            };
            if equal(a146, a147) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f58(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f72(f73(Fn15::F74, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m152 = {
        let (h148, _) = f75(Fn16::F76, {
            let a150 = {
                let a149 = v0;
                length_list(a149)
            };
            let a151 = 1u64;
            nat_add(a150, a151)?
        }, v1)?;
        h148
    };
    match m152.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f59(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a157 = {
        let a154 = f12({
            let (h153, _) = v0;
            h153
        })?;
        let a155 = Bytes::lit(&[0x3a]);
        append_bytes(a154, a155)
    };
    let a158 = {
        let (_, h156) = v0;
        h156
    };
    Ok(append_bytes(a157, a158))
}

pub fn f60(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m162 = {
        let a160 = v0;
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
                    let (h163, _) = v1;
                    h163
                };
                let a165 = 1u64;
                nat_add(a164, a165)?
            }, {
                let a167 = {
                    let (_, h166) = v1;
                    h166
                };
                let a168 = f77(v2)?;
                append_bytes(a167, a168)
            })))
        }
    }
}

pub fn f61(v0: Fn12, p172: List<(Bytes, Adt2)>, p173: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m171 = (p172, p173);
    {
        let mut m171 = m171;
        loop {
            let step = {
                let (v1, v2) = m171;
                let m169 = v2;
                match m169.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(({
                            let c170: Fn12 = v0.clone();
                            c170.apply(v1, v3)
                        }, v4))
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

pub fn f62(v0: List<(Bytes, Adt2)>, v1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
    List::cons(v1, v0)
}

pub fn f63(v0: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    f78(Fn17::F79, List::<List<(Bytes, Adt2)>>::nil(), v0)
}

pub fn f64(v0: Fn13, p177: List<List<(Bytes, Adt2)>>, p178: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    let m176 = (p177, p178);
    {
        let mut m176 = m176;
        loop {
            let step = {
                let (v1, v2) = m176;
                let m174 = v2;
                match m174.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(({
                            let c175: Fn13 = v0.clone();
                            c175.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: List<List<(Bytes, Adt2)>>, v1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt2)>::nil()), v0)
}

pub fn f66(v0: Fn14, p183: u64, p184: List<List<(Bytes, Adt2)>>) -> R<(List<List<(Bytes, Adt2)>>, bool)> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v1, v2) = m182;
                let m179 = v1;
                if m179 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m179 - 1;
                    let m181 = {
                        let c180: Fn14 = v0.clone();
                        c180.apply(v2.clone())?
                    };
                    match m181 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    f80(v0)
}

pub fn f68(v0: Adt2) -> R<Bytes> {
    Ok(f81(f81(f81(f82(match v0.clone() {
        Adt2::C0(h185, _, _, _) => {
            h185
        }
    })?, f82(match v0.clone() {
        Adt2::C0(_, h186, _, _) => {
            h186
        }
    })?), f83(match v0.clone() {
        Adt2::C0(_, _, h187, _) => {
            h187
        }
    }, 8u64)), match v0 {
        Adt2::C0(_, _, _, h188) => {
            h188
        }
    }))
}

pub fn f69(v0: Bytes, v1: u64) -> Bytes {
    let m192 = {
        let a189 = v0;
        let a190 = v1;
        let a191 = 1u64;
        slice_bytes(a189, a190, a191)
    };
    match m192 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f70(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f84(v1, v0, v2)
}

pub fn f71(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a237 = {
            let a213 = {
                let a201 = {
                    let a195 = {
                        let a193 = v3;
                        let a194 = 128u8;
                        fixed_u8::bit_and(a193, a194)
                    };
                    let a196 = 128u8;
                    if equal(a195, a196) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a202 = {
                    let a199 = {
                        let a197 = v3;
                        let a198 = 64u8;
                        fixed_u8::bit_and(a197, a198)
                    };
                    let a200 = 64u8;
                    if equal(a199, a200) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a201, a202)?
            };
            let a214 = {
                let a211 = {
                    let a205 = {
                        let a203 = v3;
                        let a204 = 32u8;
                        fixed_u8::bit_and(a203, a204)
                    };
                    let a206 = 32u8;
                    if equal(a205, a206) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a212 = {
                    let a209 = {
                        let a207 = v3;
                        let a208 = 16u8;
                        fixed_u8::bit_and(a207, a208)
                    };
                    let a210 = 16u8;
                    if equal(a209, a210) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a211, a212)?
            };
            nat_add(a213, a214)?
        };
        let a238 = {
            let a235 = {
                let a223 = {
                    let a217 = {
                        let a215 = v3;
                        let a216 = 8u8;
                        fixed_u8::bit_and(a215, a216)
                    };
                    let a218 = 8u8;
                    if equal(a217, a218) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a224 = {
                    let a221 = {
                        let a219 = v3;
                        let a220 = 4u8;
                        fixed_u8::bit_and(a219, a220)
                    };
                    let a222 = 4u8;
                    if equal(a221, a222) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a223, a224)?
            };
            let a236 = {
                let a233 = {
                    let a227 = {
                        let a225 = v3;
                        let a226 = 2u8;
                        fixed_u8::bit_and(a225, a226)
                    };
                    let a228 = 2u8;
                    if equal(a227, a228) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a234 = {
                    let a231 = {
                        let a229 = v3;
                        let a230 = 1u8;
                        fixed_u8::bit_and(a229, a230)
                    };
                    let a232 = 1u8;
                    if equal(a231, a232) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a233, a234)?
            };
            nat_add(a235, a236)?
        };
        nat_add(a237, a238)?
    };
    let a239 = v4;
    let a240 = 34u64;
    if nat_eq(a239, a240) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a241 = v4;
        let a242 = 92u64;
        if nat_eq(a241, a242) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a243 = v4;
            let a244 = 8u64;
            if nat_eq(a243, a244) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a245 = v4;
                let a246 = 12u64;
                if nat_eq(a245, a246) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a247 = v4;
                    let a248 = 10u64;
                    if nat_eq(a247, a248) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a249 = v4;
                        let a250 = 13u64;
                        if nat_eq(a249, a250) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a251 = v4;
                            let a252 = 9u64;
                            if nat_eq(a251, a252) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a253 = v4;
                                let a254 = 32u64;
                                if nat_lt(a253, a254) {
                                    let a255 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a256 = f77(v2)?;
                                    Ok(append_bytes(a255, a256))
                                } else {
                                    Ok(f85(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f72(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f86(Fn18::F87, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f73(v0: Fn15, p260: List<List<(Bytes, Bytes)>>, p261: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m259 = (p260, p261);
    {
        let mut m259 = m259;
        loop {
            let step = {
                let (v1, v2) = m259;
                let m257 = v2;
                match m257.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c258: Fn15 = v0.clone();
                            c258.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m259 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f75(v0: Fn16, p266: u64, p267: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v1, v2) = m265;
                let m262 = v1;
                if m262 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m262 - 1;
                    let m264 = {
                        let c263: Fn16 = v0.clone();
                        c263.apply(v2.clone())?
                    };
                    match m264 {
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
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f88(v0)
}

pub fn f77(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a312 = {
            let a288 = {
                let a276 = {
                    let a270 = {
                        let a268 = v1;
                        let a269 = 128u8;
                        fixed_u8::bit_and(a268, a269)
                    };
                    let a271 = 128u8;
                    if equal(a270, a271) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a277 = {
                    let a274 = {
                        let a272 = v1;
                        let a273 = 64u8;
                        fixed_u8::bit_and(a272, a273)
                    };
                    let a275 = 64u8;
                    if equal(a274, a275) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a276, a277)?
            };
            let a289 = {
                let a286 = {
                    let a280 = {
                        let a278 = v1;
                        let a279 = 32u8;
                        fixed_u8::bit_and(a278, a279)
                    };
                    let a281 = 32u8;
                    if equal(a280, a281) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a287 = {
                    let a284 = {
                        let a282 = v1;
                        let a283 = 16u8;
                        fixed_u8::bit_and(a282, a283)
                    };
                    let a285 = 16u8;
                    if equal(a284, a285) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a286, a287)?
            };
            nat_add(a288, a289)?
        };
        let a313 = {
            let a310 = {
                let a298 = {
                    let a292 = {
                        let a290 = v1;
                        let a291 = 8u8;
                        fixed_u8::bit_and(a290, a291)
                    };
                    let a293 = 8u8;
                    if equal(a292, a293) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a299 = {
                    let a296 = {
                        let a294 = v1;
                        let a295 = 4u8;
                        fixed_u8::bit_and(a294, a295)
                    };
                    let a297 = 4u8;
                    if equal(a296, a297) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a298, a299)?
            };
            let a311 = {
                let a308 = {
                    let a302 = {
                        let a300 = v1;
                        let a301 = 2u8;
                        fixed_u8::bit_and(a300, a301)
                    };
                    let a303 = 2u8;
                    if equal(a302, a303) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a309 = {
                    let a306 = {
                        let a304 = v1;
                        let a305 = 1u8;
                        fixed_u8::bit_and(a304, a305)
                    };
                    let a307 = 1u8;
                    if equal(a306, a307) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a308, a309)?
            };
            nat_add(a310, a311)?
        };
        nat_add(a312, a313)?
    };
    let a320 = f69(f89(), {
        let a314 = v2;
        let a315 = 16u64;
        let a316 = 0u64;
        nat_quot(a314, a315, a316)
    });
    let a321 = f69(f89(), {
        let a317 = v2;
        let a318 = 16u64;
        let a319 = 0u64;
        nat_rem(a317, a318, a319)
    });
    Ok(append_bytes(a320, a321))
}

pub fn f78(v0: Fn17, p325: List<List<(Bytes, Adt2)>>, p326: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v1, v2) = m324;
                let m322 = v2;
                match m322.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(({
                            let c323: Fn17 = v0.clone();
                            c323.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: List<List<(Bytes, Adt2)>>, v1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    List::cons(v1, v0)
}

pub fn f80(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    let a328 = {
        let a327 = v0.clone();
        length_list(a327)
    };
    let a329 = 1u64;
    if nat_le(a328, a329) {
        Ok(None::<List<List<(Bytes, Adt2)>>>)
    } else {
        Ok(Some(f90(v0)?))
    }
}

pub fn f81(v0: Bytes, v1: Bytes) -> Bytes {
    let a330 = v0;
    let a331 = v1;
    append_bytes(a330, a331)
}

pub fn f82(v0: Bytes) -> R<Bytes> {
    Ok(f81(f31(f91(v0, 0u8)?, Bytes::lit(&[0x00, 0x01])), Bytes::lit(&[0x00, 0x00])))
}

pub fn f83(v0: u64, v1: u64) -> Bytes {
    let (_, h332) = f92(Fn19::F93, v1, (v0, Bytes::lit(&[])));
    h332
}

pub fn f84(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h333, _) = v2;
        h333
    };
    let a334 = v3;
    let a335 = 0u64;
    if nat_eq(a334, a335) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a336 = v3;
            let a337 = v1;
            let a338 = 0u64;
            nat_quot(a336, a337, a338)
        }, {
            let a343 = f69(v0, {
                let a339 = v3;
                let a340 = v1;
                let a341 = 0u64;
                nat_rem(a339, a340, a341)
            });
            let a344 = {
                let (_, h342) = v2;
                h342
            };
            append_bytes(a343, a344)
        }))
    }
}

pub fn f85(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m348 = {
        let a345 = v0;
        let a346 = v1;
        let a347 = v2;
        slice_bytes(a345, a346, a347)
    };
    match m348 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f86(v0: Fn18, p352: List<List<(Bytes, Bytes)>>, p353: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m351 = (p352, p353);
    {
        let mut m351 = m351;
        loop {
            let step = {
                let (v1, v2) = m351;
                let m349 = v2;
                match m349.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c350: Fn18 = v0.clone();
                            c350.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f88(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a355 = {
        let a354 = v0.clone();
        length_list(a354)
    };
    let a356 = 1u64;
    if nat_le(a355, a356) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f94(v0)?))
    }
}

pub fn f89() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f90(v0: List<List<(Bytes, Adt2)>>) -> R<List<List<(Bytes, Adt2)>>> {
    Ok(f63({
        let (_, h357) = {
            let (h358, _) = f95(Fn20::F96, {
                let a360 = {
                    let a359 = v0.clone();
                    length_list(a359)
                };
                let a361 = 1u64;
                nat_add(a360, a361)?
            }, (v0, List::<List<(Bytes, Adt2)>>::nil()))?;
            h358
        };
        h357
    }))
}

pub fn f91(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a362 = v0.clone();
        length_bytes(a362)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h363, _) = f97(Fn21::F98(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h363
    };
    let v4: u64 = {
        let (h364, _) = {
            let (_, h365) = v3.clone();
            h365
        };
        h364
    };
    Ok(f99(List::cons(f85(v0, v4, {
        let a366 = v2;
        let a367 = v4;
        nat_sub(a366, a367)
    }), {
        let (_, h368) = {
            let (_, h369) = v3;
            h369
        };
        h368
    })))
}

pub fn f92(v0: Fn19, p373: u64, p374: (u64, Bytes)) -> (u64, Bytes) {
    let m372 = (p373, p374);
    {
        let mut m372 = m372;
        loop {
            let step = {
                let (v1, v2) = m372;
                let m370 = v1;
                if m370 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m370 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c371: Fn19 = v0.clone();
                        c371.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a376 = {
            let (h375, _) = v0.clone();
            h375
        };
        let a377 = 256u64;
        let a378 = 0u64;
        nat_quot(a376, a377, a378)
    }, f81(f69(f100(), {
        let a380 = {
            let (h379, _) = v0;
            h379
        };
        let a381 = 256u64;
        let a382 = 0u64;
        nat_rem(a380, a381, a382)
    }), {
        let (_, h383) = v0;
        h383
    }))
}

pub fn f94(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f72({
        let (_, h384) = {
            let (h385, _) = f101(Fn22::F102, {
                let a387 = {
                    let a386 = v0.clone();
                    length_list(a386)
                };
                let a388 = 1u64;
                nat_add(a387, a388)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h385
        };
        h384
    }))
}

pub fn f95(v0: Fn20, p393: u64, p394: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)> {
    let m392 = (p393, p394);
    {
        let mut m392 = m392;
        loop {
            let step = {
                let (v1, v2) = m392;
                let m389 = v1;
                if m389 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m389 - 1;
                    let m391 = {
                        let c390: Fn20 = v0.clone();
                        c390.apply(v2.clone())?
                    };
                    match m391 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    f103(v0)
}

pub fn f97(v0: Fn21, p399: u64, p400: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m398 = (p399, p400);
    {
        let mut m398 = m398;
        loop {
            let step = {
                let (v1, v2) = m398;
                let m395 = v1;
                if m395 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m395 - 1;
                    let m397 = {
                        let c396: Fn21 = v0.clone();
                        c396.apply(v2.clone())?
                    };
                    match m397 {
                        None => {
                            Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f104(v1, v0, v2)
}

pub fn f99(v0: List<Bytes>) -> List<Bytes> {
    f38(Fn6::F105, List::<Bytes>::nil(), v0)
}

pub fn f100() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f101(v0: Fn22, p405: u64, p406: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m404 = (p405, p406);
    {
        let mut m404 = m404;
        loop {
            let step = {
                let (v1, v2) = m404;
                let m401 = v1;
                if m401 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m401 - 1;
                    let m403 = {
                        let c402: Fn22 = v0.clone();
                        c402.apply(v2.clone())?
                    };
                    match m403 {
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
                Ok(next) => { m404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f106(v0)
}

pub fn f103(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    let v1: List<List<(Bytes, Adt2)>> = {
        let (h407, _) = v0;
        h407
    };
    let v2: List<List<(Bytes, Adt2)>> = {
        let (_, h408) = v0;
        h408
    };
    let m409 = v1;
    match m409.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>)
        }
        Some((v3, v4)) => {
            let m410 = v4;
            match m410.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt2)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f107(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f104(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h411, _) = v2.clone();
        h411
    };
    let v4: u64 = {
        let (h412, _) = {
            let (_, h413) = v2.clone();
            h413
        };
        h412
    };
    let v5: List<Bytes> = {
        let (_, h414) = {
            let (_, h415) = v2;
            h415
        };
        h414
    };
    let m416 = f108(v0.clone(), v3);
    match m416 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a417 = v6;
            let a418 = v1;
            if equal(a417, a418) {
                Ok(Some(({
                    let a419 = v3;
                    let a420 = 1u64;
                    nat_add(a419, a420)?
                }, ({
                    let a421 = v3;
                    let a422 = 1u64;
                    nat_add(a421, a422)?
                }, List::cons(f85(v0, v4, {
                    let a423 = v3;
                    let a424 = v4;
                    nat_sub(a423, a424)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a425 = v3;
                    let a426 = 1u64;
                    nat_add(a425, a426)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f105(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f106(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h427, _) = v0;
        h427
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h428) = v0;
        h428
    };
    let m429 = v1;
    match m429.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m430 = v4;
            match m430.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f109(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f107(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v2: u64 = {
        let a435 = {
            let a433 = {
                let a431 = v0.clone();
                length_list(a431)
            };
            let a434 = {
                let a432 = v1.clone();
                length_list(a432)
            };
            nat_add(a433, a434)?
        };
        let a436 = 1u64;
        nat_add(a435, a436)?
    };
    Ok(f48({
        let (_, h437) = {
            let (_, h438) = {
                let (h439, _) = f110(Fn23::F111, v2, (v0, (v1, List::<(Bytes, Adt2)>::nil())));
                h439
            };
            h438
        };
        h437
    }))
}

pub fn f108(v0: Bytes, v1: u64) -> Option<u8> {
    let a440 = v0;
    let a441 = v1;
    index_bytes(a440, a441)
}

pub fn f109(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a446 = {
            let a444 = {
                let a442 = v0.clone();
                length_list(a442)
            };
            let a445 = {
                let a443 = v1.clone();
                length_list(a443)
            };
            nat_add(a444, a445)?
        };
        let a447 = 1u64;
        nat_add(a446, a447)?
    };
    Ok(f112({
        let (_, h448) = {
            let (_, h449) = {
                let (h450, _) = f113(Fn24::F114, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h450
            };
            h449
        };
        h448
    }))
}

pub fn f110(v0: Fn23, p455: u64, p456: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool) {
    let m454 = (p455, p456);
    {
        let mut m454 = m454;
        loop {
            let step = {
                let (v1, v2) = m454;
                let m451 = v1;
                if m451 == 0 {
                    Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, false))
                } else {
                    let v3 = m451 - 1;
                    let m453 = {
                        let c452: Fn23 = v0.clone();
                        c452.apply(v2.clone())
                    };
                    match m453 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v3, v4))
                        }
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

pub fn f111(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    f115(v0)
}

pub fn f112(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f32(Fn4::F116, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f113(v0: Fn24, p461: u64, p462: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m460 = (p461, p462);
    {
        let mut m460 = m460;
        loop {
            let step = {
                let (v1, v2) = m460;
                let m457 = v1;
                if m457 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m457 - 1;
                    let m459 = {
                        let c458: Fn24 = v0.clone();
                        c458.apply(v2.clone())
                    };
                    match m459 {
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
                Ok(next) => { m460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f117(v0)
}

pub fn f115(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    let v1: List<(Bytes, Adt2)> = {
        let (h463, _) = v0.clone();
        h463
    };
    let v2: List<(Bytes, Adt2)> = {
        let (h464, _) = {
            let (_, h465) = v0.clone();
            h465
        };
        h464
    };
    let v3: List<(Bytes, Adt2)> = {
        let (_, h466) = {
            let (_, h467) = v0;
            h467
        };
        h466
    };
    let m468 = v1.clone();
    match m468.uncons() {
        None => {
            let m469 = v2;
            match m469.uncons() {
                None => {
                    None::<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt2)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m470 = v2.clone();
            match m470.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt2)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f118({
                        let (h471, _) = v8.clone();
                        h471
                    }, {
                        let (h472, _) = v6.clone();
                        h472
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

pub fn f116(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f117(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h473, _) = v0.clone();
        h473
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h474, _) = {
            let (_, h475) = v0.clone();
            h475
        };
        h474
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h476) = {
            let (_, h477) = v0;
            h477
        };
        h476
    };
    let m478 = v1.clone();
    match m478.uncons() {
        None => {
            let m479 = v2;
            match m479.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m480 = v2.clone();
            match m480.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f118({
                        let (h481, _) = v8.clone();
                        h481
                    }, {
                        let (h482, _) = v6.clone();
                        h482
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

pub fn f118(v0: Bytes, v1: Bytes) -> bool {
    let a487 = {
        let a483 = v0;
        let a484 = v1;
        compare_bytes(a483, a484)
    };
    let a488 = {
        let a485 = Bytes::lit(&[]);
        let a486 = Bytes::lit(&[0x01]);
        compare_bytes(a485, a486)
    };
    equal(a487, a488)
}
