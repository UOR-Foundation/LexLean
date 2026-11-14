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
    C0(List<u64>, List<Bytes>, List<(Bytes, (Bytes, Bytes))>, List<Bytes>, List<Bytes>, Option<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Adt0>, Adt1),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn0::F2(k0) => f2(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F8,
}

impl Fn1 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn1::F8 => f8(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F10,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F10 => f10(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F12,
}

impl Fn3 {
    pub fn apply(&self, p0: List<(Bytes, (Bytes, Bytes))>, p1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
        match self {
            Fn3::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F15,
    F21,
    F34,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn4::F15 => f15(p0, p1),
            Fn4::F21 => f21(p0, p1),
            Fn4::F34 => f34(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F23,
}

impl Fn5 {
    pub fn apply(&self, p0: (Adt0, List<Adt0>)) -> Option<(Adt0, List<Adt0>)> {
        match self {
            Fn5::F23 => f23(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F32(u64),
}

impl Fn6 {
    pub fn apply(&self, p0: (List<Adt0>, u64)) -> R<Option<(List<Adt0>, u64)>> {
        match self {
            Fn6::F32(k0) => f32(*k0, p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: u64) -> R<Adt1> {
    let v2: Adt2 = {
        let (h1, _) = f1(Fn0::F2(v1), {
            let a2 = v1;
            let a3 = 1u64;
            nat_add(a2, a3)?
        }, Adt2::C0(List::cons(v0, List::<Adt0>::nil()), Adt1::C0(List::<u64>::nil(), List::<Bytes>::nil(), List::<(Bytes, (Bytes, Bytes))>::nil(), List::<Bytes>::nil(), List::<Bytes>::nil(), None::<Bytes>, None::<Bytes>)))?;
        h1
    };
    let v3: Adt1 = match v2 {
        Adt2::C0(_, h4) => {
            h4
        }
    };
    Ok(Adt1::C0(f3(match v3.clone() {
        Adt1::C0(h5, _, _, _, _, _, _) => {
            h5
        }
    }), f4(match v3.clone() {
        Adt1::C0(_, h6, _, _, _, _, _) => {
            h6
        }
    }), f5(match v3.clone() {
        Adt1::C0(_, _, h7, _, _, _, _) => {
            h7
        }
    }), f4(match v3.clone() {
        Adt1::C0(_, _, _, h8, _, _, _) => {
            h8
        }
    }), f4(match v3.clone() {
        Adt1::C0(_, _, _, _, h9, _, _) => {
            h9
        }
    }), match v3.clone() {
        Adt1::C0(_, _, _, _, _, h10, _) => {
            h10
        }
    }, match v3 {
        Adt1::C0(_, _, _, _, _, _, h11) => {
            h11
        }
    }))
}

pub fn f1(v0: Fn0, p16: u64, p17: Adt2) -> R<(Adt2, bool)> {
    let m15 = (p16, p17);
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, v2) = m15;
                let m12 = v1;
                if m12 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m12 - 1;
                    let m14 = {
                        let c13: Fn0 = v0.clone();
                        c13.apply(v2.clone())?
                    };
                    match m14 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m15 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt2) -> R<Option<Adt2>> {
    f6(v0, v1)
}

pub fn f3(v0: List<u64>) -> List<u64> {
    f7(Fn1::F8, List::<u64>::nil(), v0)
}

pub fn f4(v0: List<Bytes>) -> List<Bytes> {
    f9(Fn2::F10, List::<Bytes>::nil(), v0)
}

pub fn f5(v0: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    f11(Fn3::F12, List::<(Bytes, (Bytes, Bytes))>::nil(), v0)
}

pub fn f6(v0: u64, v1: Adt2) -> R<Option<Adt2>> {
    let m19 = match v1.clone() {
        Adt2::C0(h18, _) => {
            h18
        }
    };
    match m19.uncons() {
        None => {
            Ok(None::<Adt2>)
        }
        Some((v2, v3)) => {
            let v4: Adt1 = match v1 {
                Adt2::C0(_, h20) => {
                    h20
                }
            };
            let m21 = v2;
            match m21 {
                Adt0::C0(v5, v6, v7) => {
                    Ok(f13(v3, Adt1::C0(match v4.clone() {
                        Adt1::C0(h22, _, _, _, _, _, _) => {
                            h22
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h23, _, _, _, _, _) => {
                            h23
                        }
                    }, List::cons((v5, (v6, v7)), match v4.clone() {
                        Adt1::C0(_, _, h24, _, _, _, _) => {
                            h24
                        }
                    }), match v4.clone() {
                        Adt1::C0(_, _, _, h25, _, _, _) => {
                            h25
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, h26, _, _) => {
                            h26
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h27, _) => {
                            h27
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h28) => {
                            h28
                        }
                    })))
                }
                Adt0::C1(v8) => {
                    Ok(f13(v3, Adt1::C0(match v4.clone() {
                        Adt1::C0(h29, _, _, _, _, _, _) => {
                            h29
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h30, _, _, _, _, _) => {
                            h30
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h31, _, _, _, _) => {
                            h31
                        }
                    }, List::cons(v8, match v4.clone() {
                        Adt1::C0(_, _, _, h32, _, _, _) => {
                            h32
                        }
                    }), match v4.clone() {
                        Adt1::C0(_, _, _, _, h33, _, _) => {
                            h33
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h34, _) => {
                            h34
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h35) => {
                            h35
                        }
                    })))
                }
                Adt0::C2(v9) => {
                    Ok(f13(v3, Adt1::C0(List::cons(v9, match v4.clone() {
                        Adt1::C0(h36, _, _, _, _, _, _) => {
                            h36
                        }
                    }), match v4.clone() {
                        Adt1::C0(_, h37, _, _, _, _, _) => {
                            h37
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h38, _, _, _, _) => {
                            h38
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, h39, _, _, _) => {
                            h39
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, h40, _, _) => {
                            h40
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h41, _) => {
                            h41
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h42) => {
                            h42
                        }
                    })))
                }
                Adt0::C3(r57) => {
                    let v10 = (*r57).clone();
                    Ok(f13(f14(Fn4::F15, v3, f16(f17(v10, v0)?)), v4))
                }
                Adt0::C4 => {
                    Ok(f13(v3, v4))
                }
                Adt0::C5(v11) => {
                    Ok(f13(v3, Adt1::C0(match v4.clone() {
                        Adt1::C0(h43, _, _, _, _, _, _) => {
                            h43
                        }
                    }, List::cons(v11, match v4.clone() {
                        Adt1::C0(_, h44, _, _, _, _, _) => {
                            h44
                        }
                    }), match v4.clone() {
                        Adt1::C0(_, _, h45, _, _, _, _) => {
                            h45
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, h46, _, _, _) => {
                            h46
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, h47, _, _) => {
                            h47
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h48, _) => {
                            h48
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h49) => {
                            h49
                        }
                    })))
                }
                Adt0::C6(r58) => {
                    let v12 = (*r58).clone();
                    Ok(f13(List::cons(v12, v3), v4))
                }
                Adt0::C7(r59) => {
                    let v13 = (*r59).clone();
                    Ok(f13(List::cons(v13, v3), v4))
                }
                Adt0::C8(r60) => {
                    let v14 = (*r60).clone();
                    Ok(f13(List::cons(v14, v3), v4))
                }
                Adt0::C9(r61, r62) => {
                    let v15 = (*r61).clone();
                    let v16 = (*r62).clone();
                    Ok(f13(List::cons(v15.clone(), List::cons(v16.clone(), v3)), f18(v4, f19(Str::lit("sub")), v15, v16, v0)?))
                }
                Adt0::C10(r63, r64) => {
                    let v17 = (*r63).clone();
                    let v18 = (*r64).clone();
                    Ok(f13(List::cons(v17.clone(), List::cons(v18.clone(), v3)), f18(v4, f19(Str::lit("sup")), v17, v18, v0)?))
                }
                Adt0::C11(r65, r66) => {
                    let v19 = (*r65).clone();
                    let v20 = (*r66).clone();
                    Ok(f13(List::cons(v19.clone(), List::cons(v20.clone(), v3)), f18(v4, f19(Str::lit("frac")), v19, v20, v0)?))
                }
                Adt0::C12(v21) => {
                    Ok(f13(v3, Adt1::C0(match v4.clone() {
                        Adt1::C0(h50, _, _, _, _, _, _) => {
                            h50
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h51, _, _, _, _, _) => {
                            h51
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h52, _, _, _, _) => {
                            h52
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, h53, _, _, _) => {
                            h53
                        }
                    }, List::cons(v21, match v4.clone() {
                        Adt1::C0(_, _, _, _, h54, _, _) => {
                            h54
                        }
                    }), f20(match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h55, _) => {
                            h55
                        }
                    }, f19(Str::lit("operator-name"))), match v4 {
                        Adt1::C0(_, _, _, _, _, _, h56) => {
                            h56
                        }
                    })))
                }
                Adt0::C13 => {
                    Ok(f13(v3, v4))
                }
                Adt0::C14(_, _) => {
                    Ok(f13(v3, v4))
                }
            }
        }
    }
}

pub fn f7(v0: Fn1, p70: List<u64>, p71: List<u64>) -> List<u64> {
    let m69 = (p70, p71);
    {
        let mut m69 = m69;
        loop {
            let step = {
                let (v1, v2) = m69;
                let m67 = v2;
                match m67.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
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

pub fn f8(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f9(v0: Fn2, p75: List<Bytes>, p76: List<Bytes>) -> List<Bytes> {
    let m74 = (p75, p76);
    {
        let mut m74 = m74;
        loop {
            let step = {
                let (v1, v2) = m74;
                let m72 = v2;
                match m72.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c73: Fn2 = v0.clone();
                            c73.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m74 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f11(v0: Fn3, p80: List<(Bytes, (Bytes, Bytes))>, p81: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m79 = (p80, p81);
    {
        let mut m79 = m79;
        loop {
            let step = {
                let (v1, v2) = m79;
                let m77 = v2;
                match m77.uncons() {
                    None => {
                        Err::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(({
                            let c78: Fn3 = v0.clone();
                            c78.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m79 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: List<(Bytes, (Bytes, Bytes))>, v1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
    List::cons(v1, v0)
}

pub fn f13(v0: List<Adt0>, v1: Adt1) -> Option<Adt2> {
    Some(Adt2::C0(v0, v1))
}

pub fn f14(v0: Fn4, p85: List<Adt0>, p86: List<Adt0>) -> List<Adt0> {
    let m84 = (p85, p86);
    {
        let mut m84 = m84;
        loop {
            let step = {
                let (v1, v2) = m84;
                let m82 = v2;
                match m82.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c83: Fn4 = v0.clone();
                            c83.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m84 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f16(v0: List<Adt0>) -> List<Adt0> {
    f14(Fn4::F21, List::<Adt0>::nil(), v0)
}

pub fn f17(v0: Adt0, v1: u64) -> R<List<Adt0>> {
    Ok(f16({
        let (_, h87) = {
            let (h88, _) = f22(Fn5::F23, {
                let a89 = v1;
                let a90 = 1u64;
                nat_add(a89, a90)?
            }, (v0, List::<Adt0>::nil()));
            h88
        };
        h87
    }))
}

pub fn f18(v0: Adt1, v1: Bytes, v2: Adt0, v3: Adt0, v4: u64) -> R<Adt1> {
    Ok(Adt1::C0(match v0.clone() {
        Adt1::C0(h91, _, _, _, _, _, _) => {
            h91
        }
    }, match v0.clone() {
        Adt1::C0(_, h92, _, _, _, _, _) => {
            h92
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h93, _, _, _, _) => {
            h93
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h94, _, _, _) => {
            h94
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h95, _, _) => {
            h95
        }
    }, f20(match v0.clone() {
        Adt1::C0(_, _, _, _, _, h96, _) => {
            h96
        }
    }, v1.clone()), f24(match v0 {
        Adt1::C0(_, _, _, _, _, _, h97) => {
            h97
        }
    }, f25(v1, v2, v3, v4)?)))
}

pub fn f19(v0: Str) -> Bytes {
    let a98 = v0;
    utf8_encode(a98)
}

pub fn f20(v0: Option<Bytes>, v1: Bytes) -> Option<Bytes> {
    let m99 = v0.clone();
    match m99 {
        None => {
            Some(v1)
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f21(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f22(v0: Fn5, p104: u64, p105: (Adt0, List<Adt0>)) -> ((Adt0, List<Adt0>), bool) {
    let m103 = (p104, p105);
    {
        let mut m103 = m103;
        loop {
            let step = {
                let (v1, v2) = m103;
                let m100 = v1;
                if m100 == 0 {
                    Err::<(u64, (Adt0, List<Adt0>)), ((Adt0, List<Adt0>), bool)>((v2, false))
                } else {
                    let v3 = m100 - 1;
                    let m102 = {
                        let c101: Fn5 = v0.clone();
                        c101.apply(v2.clone())
                    };
                    match m102 {
                        None => {
                            Err::<(u64, (Adt0, List<Adt0>)), ((Adt0, List<Adt0>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt0, List<Adt0>)), ((Adt0, List<Adt0>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: (Adt0, List<Adt0>)) -> Option<(Adt0, List<Adt0>)> {
    let m107 = {
        let (h106, _) = v0;
        h106
    };
    match m107 {
        Adt0::C0(_, _, _) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C1(_) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C2(_) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C3(_) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C4 => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C5(_) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C6(_) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C7(_) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C8(_) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C9(_, _) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C10(_, _) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C11(_, _) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C12(_) => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C13 => {
            None::<(Adt0, List<Adt0>)>
        }
        Adt0::C14(r109, r110) => {
            let v1 = (*r109).clone();
            let v2 = (*r110).clone();
            Some((v2, List::cons(v1, {
                let (_, h108) = v0;
                h108
            })))
        }
    }
}

pub fn f24(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m111 = v0.clone();
    match m111 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f25(v0: Bytes, v1: Adt0, v2: Adt0, v3: u64) -> R<Option<Bytes>> {
    let m112 = f26(v0.clone(), v1, v3)?;
    match m112 {
        None => {
            f26(v0, v2, v3)
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f26(v0: Bytes, v1: Adt0, v2: u64) -> R<Option<Bytes>> {
    if f27(v1.clone(), v2)? {
        Ok(Some(f28(f29(v0), f19(Str::lit(" operand renders nothing")))))
    } else {
        let a113 = v0.clone();
        let a114 = f19(Str::lit("frac"));
        if if equal(a113, a114) {
            false
        } else {
            f30(v1)
        } {
            Ok(Some(f28(f29(v0), f19(Str::lit(" operand is a bare script; wrap nested scripts in `(group ...)`")))))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f27(v0: Adt0, v1: u64) -> R<bool> {
    let a119 = {
        let (_, h115) = {
            let (h116, _) = f31(Fn6::F32(v1), {
                let a117 = v1;
                let a118 = 1u64;
                nat_add(a117, a118)?
            }, (List::cons(v0, List::<Adt0>::nil()), 0u64))?;
            h116
        };
        h115
    };
    let a120 = 0u64;
    Ok(nat_eq(a119, a120))
}

pub fn f28(v0: Bytes, v1: Bytes) -> Bytes {
    let a121 = v0;
    let a122 = v1;
    append_bytes(a121, a122)
}

pub fn f29(v0: Bytes) -> Bytes {
    f28(f28(f19(Str::lit("`")), v0), f19(Str::lit("`")))
}

pub fn f30(v0: Adt0) -> bool {
    let m123 = v0;
    match m123 {
        Adt0::C0(_, _, _) => {
            false
        }
        Adt0::C1(_) => {
            false
        }
        Adt0::C2(_) => {
            false
        }
        Adt0::C3(_) => {
            false
        }
        Adt0::C4 => {
            false
        }
        Adt0::C5(_) => {
            false
        }
        Adt0::C6(_) => {
            false
        }
        Adt0::C7(_) => {
            false
        }
        Adt0::C8(_) => {
            false
        }
        Adt0::C9(_, _) => {
            true
        }
        Adt0::C10(_, _) => {
            true
        }
        Adt0::C11(_, _) => {
            false
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13 => {
            false
        }
        Adt0::C14(_, _) => {
            false
        }
    }
}

pub fn f31(v0: Fn6, p128: u64, p129: (List<Adt0>, u64)) -> R<((List<Adt0>, u64), bool)> {
    let m127 = (p128, p129);
    {
        let mut m127 = m127;
        loop {
            let step = {
                let (v1, v2) = m127;
                let m124 = v1;
                if m124 == 0 {
                    Err::<(u64, (List<Adt0>, u64)), R<((List<Adt0>, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m124 - 1;
                    let m126 = {
                        let c125: Fn6 = v0.clone();
                        c125.apply(v2.clone())?
                    };
                    match m126 {
                        None => {
                            Err::<(u64, (List<Adt0>, u64)), R<((List<Adt0>, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt0>, u64)), R<((List<Adt0>, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: u64, v1: (List<Adt0>, u64)) -> R<Option<(List<Adt0>, u64)>> {
    f33(v0, v1)
}

pub fn f33(v0: u64, v1: (List<Adt0>, u64)) -> R<Option<(List<Adt0>, u64)>> {
    let a131 = {
        let (_, h130) = v1;
        h130
    };
    let a132 = 1u64;
    if nat_eq(a131, a132) {
        Ok(None::<(List<Adt0>, u64)>)
    } else {
        let m134 = {
            let (h133, _) = v1;
            h133
        };
        match m134.uncons() {
            None => {
                Ok(None::<(List<Adt0>, u64)>)
            }
            Some((v2, v3)) => {
                let m135 = v2;
                match m135 {
                    Adt0::C0(_, _, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C1(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C2(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C3(r136) => {
                        let v4 = (*r136).clone();
                        Ok(Some((f14(Fn4::F34, v3, f17(v4, v0)?), 0u64)))
                    }
                    Adt0::C4 => {
                        Ok(Some((v3, 0u64)))
                    }
                    Adt0::C5(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C6(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C7(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C8(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C9(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C10(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C11(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C12(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C13 => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt0::C14(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                }
            }
        }
    }
}

pub fn f34(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}
