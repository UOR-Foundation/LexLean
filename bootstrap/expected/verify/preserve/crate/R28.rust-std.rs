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
    C0(Bytes, Bytes, Bytes, List<Bytes>, List<Bytes>, Bytes, Bytes, Bytes, List<Adt1>, List<Adt2>, List<Adt3>, Adt4, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes),
    C1(Bytes, Bytes),
    C2(Bytes, Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, Bytes, List<Bytes>, Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F7,
}

impl Fn0 {
    pub fn apply(&self, p0: Bytes, p1: Adt2) -> R<Bytes> {
        match self {
            Fn0::F7 => f7(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9,
}

impl Fn1 {
    pub fn apply(&self, p0: Bytes, p1: Adt3) -> R<Bytes> {
        match self {
            Fn1::F9 => f9(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F22,
}

impl Fn2 {
    pub fn apply(&self, p0: Bytes, p1: Adt1) -> R<Bytes> {
        match self {
            Fn2::F22 => f22(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F30,
    F45,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn3::F30 => f30(p0, p1),
            Fn3::F45 => Ok(f45(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F35,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt1>, p1: (Bytes, Adt1)) -> List<Adt1> {
        match self {
            Fn4::F35 => f35(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F39,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: Adt1) -> List<(Bytes, Adt1)> {
        match self {
            Fn5::F39 => f39(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F42(Bytes),
    F58(Bytes),
    F63(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn6::F42(k0) => f42(k0.clone(), p0),
            Fn6::F58(k0) => f58(k0.clone(), p0),
            Fn6::F63(k0, k1) => Ok(f63(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F44(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn7::F44(k0) => f44(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F48,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
        match self {
            Fn8::F48 => f48(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F51,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn9::F51 => f51(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F53,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
        match self {
            Fn10::F53 => f53(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F60,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn11::F60 => f60(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F73,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
        match self {
            Fn12::F73 => f73(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F78,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
        match self {
            Fn13::F78 => f78(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    let v1: Bytes = f1(f1(f1(f1(f1(f1(f1(f1(f1(f2(Str::lit("spec"), f3(Str::lit("lexlean/project/1")))?, f2(Str::lit("name"), match v0.clone() {
        Adt0::C0(h1, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1
        }
    })?), f2(Str::lit("language"), match v0.clone() {
        Adt0::C0(_, h2, _, _, _, _, _, _, _, _, _, _, _) => {
            h2
        }
    })?), f2(Str::lit("module_prefix"), match v0.clone() {
        Adt0::C0(_, _, h3, _, _, _, _, _, _, _, _, _, _) => {
            h3
        }
    })?), f4(Str::lit("source_roots"), match v0.clone() {
        Adt0::C0(_, _, _, h4, _, _, _, _, _, _, _, _, _) => {
            h4
        }
    })?), f4(Str::lit("entrypoints"), match v0.clone() {
        Adt0::C0(_, _, _, _, h5, _, _, _, _, _, _, _, _) => {
            h5
        }
    })?), f2(Str::lit("build_root"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, h6, _, _, _, _, _, _, _) => {
            h6
        }
    })?), f2(Str::lit("lockfile"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h7, _, _, _, _, _, _) => {
            h7
        }
    })?), f2(Str::lit("lean_workspace"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h8, _, _, _, _, _) => {
            h8
        }
    })?), f2(Str::lit("lean_toolchain"), f5())?);
    let v2: Bytes = f6(Fn0::F7, Bytes::lit(&[]), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h9, _, _, _) => {
            h9
        }
    })?;
    let v3: Bytes = f8(Fn1::F9, Bytes::lit(&[]), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h10, _, _) => {
            h10
        }
    })?;
    let v5: Bytes = {
        let m12 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, _, h11) => {
                h11
            }
        };
        match m12 {
            None => {
                Bytes::lit(&[])
            }
            Some(v4) => {
                f10(v4)?
            }
        }
    };
    Ok(f1(f1(f1(f1(f1(v1, f11(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h13, _, _, _, _) => {
            h13
        }
    })?), v2), v3), f12(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, h14, _) => {
            h14
        }
    })?), v5))
}

pub fn f1(v0: Bytes, v1: Bytes) -> Bytes {
    let a15 = v0;
    let a16 = v1;
    append_bytes(a15, a16)
}

pub fn f2(v0: Str, v1: Bytes) -> R<Bytes> {
    f13(f3(v0), v1)
}

pub fn f3(v0: Str) -> Bytes {
    let a17 = v0;
    utf8_encode(a17)
}

pub fn f4(v0: Str, v1: List<Bytes>) -> R<Bytes> {
    Ok(f14(f3(v0), f15(v1)?))
}

pub fn f5() -> Bytes {
    f16(Str::lit("leanprover/lean4:v4.32.2"))
}

pub fn f6(v0: Fn0, p21: Bytes, p22: List<Adt2>) -> R<Bytes> {
    let m20 = (p21, p22);
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, v2) = m20;
                let m18 = v2;
                match m18.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt2>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt2>), R<Bytes>>(({
                            let c19: Fn0 = v0.clone();
                            c19.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Bytes, v1: Adt2) -> R<Bytes> {
    Ok(f1(v0, f17(v1)?))
}

pub fn f8(v0: Fn1, p26: Bytes, p27: List<Adt3>) -> R<Bytes> {
    let m25 = (p26, p27);
    {
        let mut m25 = m25;
        loop {
            let step = {
                let (v1, v2) = m25;
                let m23 = v2;
                match m23.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt3>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt3>), R<Bytes>>(({
                            let c24: Fn1 = v0.clone();
                            c24.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m25 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Bytes, v1: Adt3) -> R<Bytes> {
    Ok(f1(v0, f18(v1)?))
}

pub fn f10(v0: Adt5) -> R<Bytes> {
    Ok(f1(f1(f1(f1(f1(f1(f1(f1(f19(Str::lit("[pdf]")), f2(Str::lit("mode"), f3(Str::lit("external")))?), f2(Str::lit("program"), match v0.clone() {
        Adt5::C0(h28, _, _, _, _, _, _) => {
            h28
        }
    })?), f20(Str::lit("program_sha256"), match v0.clone() {
        Adt5::C0(_, h29, _, _, _, _, _) => {
            h29
        }
    })?), f4(Str::lit("version_argv"), match v0.clone() {
        Adt5::C0(_, _, h30, _, _, _, _) => {
            h30
        }
    })?), f20(Str::lit("version_stdout_sha256"), match v0.clone() {
        Adt5::C0(_, _, _, h31, _, _, _) => {
            h31
        }
    })?), f4(Str::lit("compile_argv"), match v0.clone() {
        Adt5::C0(_, _, _, _, h32, _, _) => {
            h32
        }
    })?), f2(Str::lit("output"), match v0.clone() {
        Adt5::C0(_, _, _, _, _, h33, _) => {
            h33
        }
    })?), f4(Str::lit("resources"), match v0 {
        Adt5::C0(_, _, _, _, _, _, h34) => {
            h34
        }
    })?))
}

pub fn f11(v0: List<Adt1>) -> R<Bytes> {
    let m35 = v0.clone();
    match m35.uncons() {
        None => {
            Ok(f1(f3(Str::lit("lexicon_source = []")), Bytes::lit(&[0x0a])))
        }
        Some((_, _)) => {
            f21(Fn2::F22, Bytes::lit(&[]), f23(v0)?)
        }
    }
}

pub fn f12(v0: Adt4) -> R<Bytes> {
    Ok(f1(f1(f1(f1(f1(f1(f1(f1(f1(f1(f1(f19(Str::lit("[limits]")), f24(Str::lit("max_file_bytes"), match v0.clone() {
        Adt4::C0(h36, _, _, _, _, _, _, _, _, _, _) => {
            h36
        }
    })?), f24(Str::lit("max_total_source_bytes"), match v0.clone() {
        Adt4::C0(_, h37, _, _, _, _, _, _, _, _, _) => {
            h37
        }
    })?), f24(Str::lit("max_primitive_atoms"), match v0.clone() {
        Adt4::C0(_, _, h38, _, _, _, _, _, _, _, _) => {
            h38
        }
    })?), f24(Str::lit("max_token_lattice_edges"), match v0.clone() {
        Adt4::C0(_, _, _, h39, _, _, _, _, _, _, _) => {
            h39
        }
    })?), f24(Str::lit("max_parse_states"), match v0.clone() {
        Adt4::C0(_, _, _, _, h40, _, _, _, _, _, _) => {
            h40
        }
    })?), f24(Str::lit("max_ir_nodes"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, h41, _, _, _, _, _) => {
            h41
        }
    })?), f24(Str::lit("max_scope_depth"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h42, _, _, _, _) => {
            h42
        }
    })?), f24(Str::lit("max_import_depth"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h43, _, _, _) => {
            h43
        }
    })?), f24(Str::lit("max_diagnostics"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h44, _, _) => {
            h44
        }
    })?), f24(Str::lit("max_child_output_bytes"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h45, _) => {
            h45
        }
    })?), f24(Str::lit("child_timeout_ms"), match v0 {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h46) => {
            h46
        }
    })?))
}

pub fn f13(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f14(v0, f25(v1)?))
}

pub fn f14(v0: Bytes, v1: Bytes) -> Bytes {
    f26(f26(f26(v0, f16(Str::lit(" = "))), v1), Bytes::lit(&[0x0a]))
}

pub fn f15(v0: List<Bytes>) -> R<Bytes> {
    Ok(f26(f26(f16(Str::lit("[")), f27(f28(f29(Fn3::F30, List::<Bytes>::nil(), v0)?)?, f16(Str::lit(", ")))), f16(Str::lit("]"))))
}

pub fn f16(v0: Str) -> Bytes {
    let a47 = v0;
    utf8_encode(a47)
}

pub fn f17(v0: Adt2) -> R<Bytes> {
    Ok(f1(f1(f19(Str::lit("[[artifact_source]]")), f20(Str::lit("sha256"), match v0.clone() {
        Adt2::C0(h48, _) => {
            h48
        }
    })?), f2(Str::lit("path"), match v0 {
        Adt2::C0(_, h49) => {
            h49
        }
    })?))
}

pub fn f18(v0: Adt3) -> R<Bytes> {
    Ok(f1(f1(f1(f1(f19(Str::lit("[[source_package]]")), f2(Str::lit("package"), match v0.clone() {
        Adt3::C0(h50, _, _) => {
            h50
        }
    })?), f2(Str::lit("version"), match v0.clone() {
        Adt3::C0(_, h51, _) => {
            h51
        }
    })?), f2(Str::lit("kind"), f3(Str::lit("path")))?), f2(Str::lit("path"), match v0 {
        Adt3::C0(_, _, h52) => {
            h52
        }
    })?))
}

pub fn f19(v0: Str) -> Bytes {
    f31(f3(v0))
}

pub fn f20(v0: Str, v1: Bytes) -> R<Bytes> {
    f32(f3(v0), v1)
}

pub fn f21(v0: Fn2, p56: Bytes, p57: List<Adt1>) -> R<Bytes> {
    let m55 = (p56, p57);
    {
        let mut m55 = m55;
        loop {
            let step = {
                let (v1, v2) = m55;
                let m53 = v2;
                match m53.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt1>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt1>), R<Bytes>>(({
                            let c54: Fn2 = v0.clone();
                            c54.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m55 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Bytes, v1: Adt1) -> R<Bytes> {
    Ok(f1(v0, f33(v1)?))
}

pub fn f23(v0: List<Adt1>) -> R<List<Adt1>> {
    Ok(f34(Fn4::F35, List::<Adt1>::nil(), f36(f37(f36(f38(Fn5::F39, List::<(Bytes, Adt1)>::nil(), v0)))?)))
}

pub fn f24(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f14(f3(v0), f40(v1)?))
}

pub fn f25(v0: Bytes) -> R<Bytes> {
    Ok(f26(f26(Bytes::lit(&[0x22]), {
        let (_, h58) = {
            let (h59, _) = f41(Fn6::F42(v0.clone()), {
                let a61 = {
                    let a60 = v0;
                    length_bytes(a60)
                };
                let a62 = 1u64;
                nat_add(a61, a62)?
            }, (0u64, Bytes::lit(&[])))?;
            h59
        };
        h58
    }), Bytes::lit(&[0x22])))
}

pub fn f26(v0: Bytes, v1: Bytes) -> Bytes {
    let a63 = v0;
    let a64 = v1;
    append_bytes(a63, a64)
}

pub fn f27(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m65 = f43(Fn7::F44(v1), None::<Bytes>, v0);
    match m65 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f28(v0: List<Bytes>) -> R<List<Bytes>> {
    f29(Fn3::F45, List::<Bytes>::nil(), v0)
}

pub fn f29(v0: Fn3, p69: List<Bytes>, p70: List<Bytes>) -> R<List<Bytes>> {
    let m68 = (p69, p70);
    {
        let mut m68 = m68;
        loop {
            let step = {
                let (v1, v2) = m68;
                let m66 = v2;
                match m66.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c67: Fn3 = v0.clone();
                            c67.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m68 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f25(v1)?, v0))
}

pub fn f31(v0: Bytes) -> Bytes {
    f26(f26(Bytes::lit(&[0x0a]), v0), Bytes::lit(&[0x0a]))
}

pub fn f32(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f14(v0, f25(f46(v1)?)?))
}

pub fn f33(v0: Adt1) -> R<Bytes> {
    Ok(f1(f19(Str::lit("[[lexicon_source]]")), {
        let m71 = v0;
        match m71 {
            Adt1::C0(v1) => {
                f1(f2(Str::lit("package"), v1)?, f2(Str::lit("kind"), f3(Str::lit("builtin")))?)
            }
            Adt1::C1(v2, v3) => {
                f1(f1(f2(Str::lit("package"), v2)?, f2(Str::lit("kind"), f3(Str::lit("path")))?), f2(Str::lit("path"), v3)?)
            }
            Adt1::C2(v4, v5, v6, v7) => {
                f1(f1(f1(f1(f2(Str::lit("package"), v4)?, f2(Str::lit("kind"), f3(Str::lit("git")))?), f2(Str::lit("url"), v5)?), f2(Str::lit("revision"), v6)?), f2(Str::lit("subdirectory"), v7)?)
            }
        }
    }))
}

pub fn f34(v0: Fn4, p75: List<Adt1>, p76: List<(Bytes, Adt1)>) -> List<Adt1> {
    let m74 = (p75, p76);
    {
        let mut m74 = m74;
        loop {
            let step = {
                let (v1, v2) = m74;
                let m72 = v2;
                match m72.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(({
                            let c73: Fn4 = v0.clone();
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

pub fn f35(v0: List<Adt1>, v1: (Bytes, Adt1)) -> List<Adt1> {
    List::cons({
        let (_, h77) = v1;
        h77
    }, v0)
}

pub fn f36(v0: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    f47(Fn8::F48, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f37(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v1: List<List<(Bytes, Adt1)>> = f49(f50(Fn9::F51, List::<List<(Bytes, Adt1)>>::nil(), v0.clone()));
    let m82 = {
        let (h78, _) = f52(Fn10::F53, {
            let a80 = {
                let a79 = v0;
                length_list(a79)
            };
            let a81 = 1u64;
            nat_add(a80, a81)?
        }, v1)?;
        h78
    };
    match m82.uncons() {
        None => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f38(v0: Fn5, p86: List<(Bytes, Adt1)>, p87: List<Adt1>) -> List<(Bytes, Adt1)> {
    let m85 = (p86, p87);
    {
        let mut m85 = m85;
        loop {
            let step = {
                let (v1, v2) = m85;
                let m83 = v2;
                match m83.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<Adt1>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<Adt1>), List<(Bytes, Adt1)>>(({
                            let c84: Fn5 = v0.clone();
                            c84.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m85 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: List<(Bytes, Adt1)>, v1: Adt1) -> List<(Bytes, Adt1)> {
    List::cons((f54(v1.clone()), v1), v0)
}

pub fn f40(v0: u64) -> R<Bytes> {
    f55(f56(), 10u64, v0)
}

pub fn f41(v0: Fn6, p92: u64, p93: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m91 = (p92, p93);
    {
        let mut m91 = m91;
        loop {
            let step = {
                let (v1, v2) = m91;
                let m88 = v1;
                if m88 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m88 - 1;
                    let m90 = {
                        let c89: Fn6 = v0.clone();
                        c89.apply(v2.clone())?
                    };
                    match m90 {
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
                Ok(next) => { m91 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f57(v0, v1)
}

pub fn f43(v0: Fn7, p97: Option<Bytes>, p98: List<Bytes>) -> Option<Bytes> {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v1, v2) = m96;
                let m94 = v2;
                match m94.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c95: Fn7 = v0.clone();
                            c95.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m96 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m99 = v1;
    match m99 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a102 = {
                    let a100 = v3;
                    let a101 = v0;
                    append_bytes(a100, a101)
                };
                let a103 = v2;
                append_bytes(a102, a103)
            })
        }
    }
}

pub fn f45(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f46(v0: Bytes) -> R<Bytes> {
    let (_, h104) = {
        let (h105, _) = f41(Fn6::F58(v0.clone()), {
            let a106 = v0;
            length_bytes(a106)
        }, (0u64, Bytes::lit(&[])))?;
        h105
    };
    Ok(h104)
}

pub fn f47(v0: Fn8, p110: List<(Bytes, Adt1)>, p111: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    let m109 = (p110, p111);
    {
        let mut m109 = m109;
        loop {
            let step = {
                let (v1, v2) = m109;
                let m107 = v2;
                match m107.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(({
                            let c108: Fn8 = v0.clone();
                            c108.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f49(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f59(Fn11::F60, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f50(v0: Fn9, p115: List<List<(Bytes, Adt1)>>, p116: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    let m114 = (p115, p116);
    {
        let mut m114 = m114;
        loop {
            let step = {
                let (v1, v2) = m114;
                let m112 = v2;
                match m112.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(({
                            let c113: Fn9 = v0.clone();
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

pub fn f51(v0: List<List<(Bytes, Adt1)>>, v1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt1)>::nil()), v0)
}

pub fn f52(v0: Fn10, p121: u64, p122: List<List<(Bytes, Adt1)>>) -> R<(List<List<(Bytes, Adt1)>>, bool)> {
    let m120 = (p121, p122);
    {
        let mut m120 = m120;
        loop {
            let step = {
                let (v1, v2) = m120;
                let m117 = v1;
                if m117 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m117 - 1;
                    let m119 = {
                        let c118: Fn10 = v0.clone();
                        c118.apply(v2.clone())?
                    };
                    match m119 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    f61(v0)
}

pub fn f54(v0: Adt1) -> Bytes {
    let m123 = v0;
    match m123 {
        Adt1::C0(v1) => {
            v1
        }
        Adt1::C1(v2, _) => {
            v2
        }
        Adt1::C2(v4, _, _, _) => {
            v4
        }
    }
}

pub fn f55(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a124 = v2;
    let a125 = 0u64;
    if nat_eq(a124, a125) {
        Ok(f62(v0, 0u64))
    } else {
        let (_, h126) = {
            let (h127, _) = f41(Fn6::F63(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h127
        };
        Ok(h126)
    }
}

pub fn f56() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f57(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m129 = f64(v0, {
        let (h128, _) = v1.clone();
        h128
    });
    match m129 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a131 = {
                    let (h130, _) = v1;
                    h130
                };
                let a132 = 1u64;
                nat_add(a131, a132)?
            }, f26({
                let (_, h133) = v1;
                h133
            }, f65({
                let v3: u8 = v2;
                let a178 = {
                    let a154 = {
                        let a142 = {
                            let a136 = {
                                let a134 = v3;
                                let a135 = 128u8;
                                fixed_u8::bit_and(a134, a135)
                            };
                            let a137 = 128u8;
                            if equal(a136, a137) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a143 = {
                            let a140 = {
                                let a138 = v3;
                                let a139 = 64u8;
                                fixed_u8::bit_and(a138, a139)
                            };
                            let a141 = 64u8;
                            if equal(a140, a141) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a142, a143)?
                    };
                    let a155 = {
                        let a152 = {
                            let a146 = {
                                let a144 = v3;
                                let a145 = 32u8;
                                fixed_u8::bit_and(a144, a145)
                            };
                            let a147 = 32u8;
                            if equal(a146, a147) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a153 = {
                            let a150 = {
                                let a148 = v3;
                                let a149 = 16u8;
                                fixed_u8::bit_and(a148, a149)
                            };
                            let a151 = 16u8;
                            if equal(a150, a151) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a152, a153)?
                    };
                    nat_add(a154, a155)?
                };
                let a179 = {
                    let a176 = {
                        let a164 = {
                            let a158 = {
                                let a156 = v3;
                                let a157 = 8u8;
                                fixed_u8::bit_and(a156, a157)
                            };
                            let a159 = 8u8;
                            if equal(a158, a159) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a165 = {
                            let a162 = {
                                let a160 = v3;
                                let a161 = 4u8;
                                fixed_u8::bit_and(a160, a161)
                            };
                            let a163 = 4u8;
                            if equal(a162, a163) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a164, a165)?
                    };
                    let a177 = {
                        let a174 = {
                            let a168 = {
                                let a166 = v3;
                                let a167 = 2u8;
                                fixed_u8::bit_and(a166, a167)
                            };
                            let a169 = 2u8;
                            if equal(a168, a169) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a175 = {
                            let a172 = {
                                let a170 = v3;
                                let a171 = 1u8;
                                fixed_u8::bit_and(a170, a171)
                            };
                            let a173 = 1u8;
                            if equal(a172, a173) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a174, a175)?
                    };
                    nat_add(a176, a177)?
                };
                nat_add(a178, a179)?
            })))))
        }
    }
}

pub fn f58(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f66(v0, v1)
}

pub fn f59(v0: Fn11, p183: List<List<(Bytes, Adt1)>>, p184: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v1, v2) = m182;
                let m180 = v2;
                match m180.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c181: Fn11 = v0.clone();
                            c181.apply(v1, v3)
                        }, v4))
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

pub fn f60(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f61(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    let a186 = {
        let a185 = v0.clone();
        length_list(a185)
    };
    let a187 = 1u64;
    if nat_le(a186, a187) {
        Ok(None::<List<List<(Bytes, Adt1)>>>)
    } else {
        Ok(Some(f67(v0)?))
    }
}

pub fn f62(v0: Bytes, v1: u64) -> Bytes {
    let m191 = {
        let a188 = v0;
        let a189 = v1;
        let a190 = 1u64;
        slice_bytes(a188, a189, a190)
    };
    match m191 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f63(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f68(v1, v0, v2)
}

pub fn f64(v0: Bytes, v1: u64) -> Option<u8> {
    let a192 = v0;
    let a193 = v1;
    index_bytes(a192, a193)
}

pub fn f65(v0: u64) -> Bytes {
    let a194 = v0;
    let a195 = 34u64;
    if nat_eq(a194, a195) {
        Bytes::lit(&[0x5c, 0x22])
    } else {
        let a196 = v0;
        let a197 = 92u64;
        if nat_eq(a196, a197) {
            Bytes::lit(&[0x5c, 0x5c])
        } else {
            let a198 = v0;
            let a199 = 32u64;
            if if nat_lt(a198, a199) {
                true
            } else {
                let a200 = v0;
                let a201 = 127u64;
                nat_eq(a200, a201)
            } {
                f26(f26(Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]), f69({
                    let a202 = v0;
                    let a203 = 16u64;
                    let a204 = 0u64;
                    nat_quot(a202, a203, a204)
                })), f69({
                    let a205 = v0;
                    let a206 = 16u64;
                    let a207 = 0u64;
                    nat_rem(a205, a206, a207)
                }))
            } else {
                f62(f70(), v0)
            }
        }
    }
}

pub fn f66(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m211 = {
        let a209 = v0;
        let a210 = {
            let (h208, _) = v1.clone();
            h208
        };
        index_bytes(a209, a210)
    };
    match m211 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a213 = {
                    let (h212, _) = v1;
                    h212
                };
                let a214 = 1u64;
                nat_add(a213, a214)?
            }, {
                let a216 = {
                    let (_, h215) = v1;
                    h215
                };
                let a217 = f71(v2)?;
                append_bytes(a216, a217)
            })))
        }
    }
}

pub fn f67(v0: List<List<(Bytes, Adt1)>>) -> R<List<List<(Bytes, Adt1)>>> {
    Ok(f49({
        let (_, h218) = {
            let (h219, _) = f72(Fn12::F73, {
                let a221 = {
                    let a220 = v0.clone();
                    length_list(a220)
                };
                let a222 = 1u64;
                nat_add(a221, a222)?
            }, (v0, List::<List<(Bytes, Adt1)>>::nil()))?;
            h219
        };
        h218
    }))
}

pub fn f68(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h223, _) = v2;
        h223
    };
    let a224 = v3;
    let a225 = 0u64;
    if nat_eq(a224, a225) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a226 = v3;
            let a227 = v1;
            let a228 = 0u64;
            nat_quot(a226, a227, a228)
        }, {
            let a233 = f62(v0, {
                let a229 = v3;
                let a230 = v1;
                let a231 = 0u64;
                nat_rem(a229, a230, a231)
            });
            let a234 = {
                let (_, h232) = v2;
                h232
            };
            append_bytes(a233, a234)
        }))
    }
}

pub fn f69(v0: u64) -> Bytes {
    f62(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46]), v0)
}

pub fn f70() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f71(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a279 = {
            let a255 = {
                let a243 = {
                    let a237 = {
                        let a235 = v1;
                        let a236 = 128u8;
                        fixed_u8::bit_and(a235, a236)
                    };
                    let a238 = 128u8;
                    if equal(a237, a238) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a244 = {
                    let a241 = {
                        let a239 = v1;
                        let a240 = 64u8;
                        fixed_u8::bit_and(a239, a240)
                    };
                    let a242 = 64u8;
                    if equal(a241, a242) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a243, a244)?
            };
            let a256 = {
                let a253 = {
                    let a247 = {
                        let a245 = v1;
                        let a246 = 32u8;
                        fixed_u8::bit_and(a245, a246)
                    };
                    let a248 = 32u8;
                    if equal(a247, a248) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a254 = {
                    let a251 = {
                        let a249 = v1;
                        let a250 = 16u8;
                        fixed_u8::bit_and(a249, a250)
                    };
                    let a252 = 16u8;
                    if equal(a251, a252) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a253, a254)?
            };
            nat_add(a255, a256)?
        };
        let a280 = {
            let a277 = {
                let a265 = {
                    let a259 = {
                        let a257 = v1;
                        let a258 = 8u8;
                        fixed_u8::bit_and(a257, a258)
                    };
                    let a260 = 8u8;
                    if equal(a259, a260) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a266 = {
                    let a263 = {
                        let a261 = v1;
                        let a262 = 4u8;
                        fixed_u8::bit_and(a261, a262)
                    };
                    let a264 = 4u8;
                    if equal(a263, a264) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a265, a266)?
            };
            let a278 = {
                let a275 = {
                    let a269 = {
                        let a267 = v1;
                        let a268 = 2u8;
                        fixed_u8::bit_and(a267, a268)
                    };
                    let a270 = 2u8;
                    if equal(a269, a270) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a276 = {
                    let a273 = {
                        let a271 = v1;
                        let a272 = 1u8;
                        fixed_u8::bit_and(a271, a272)
                    };
                    let a274 = 1u8;
                    if equal(a273, a274) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a275, a276)?
            };
            nat_add(a277, a278)?
        };
        nat_add(a279, a280)?
    };
    let a287 = f62(f74(), {
        let a281 = v2;
        let a282 = 16u64;
        let a283 = 0u64;
        nat_quot(a281, a282, a283)
    });
    let a288 = f62(f74(), {
        let a284 = v2;
        let a285 = 16u64;
        let a286 = 0u64;
        nat_rem(a284, a285, a286)
    });
    Ok(append_bytes(a287, a288))
}

pub fn f72(v0: Fn12, p293: u64, p294: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)> {
    let m292 = (p293, p294);
    {
        let mut m292 = m292;
        loop {
            let step = {
                let (v1, v2) = m292;
                let m289 = v1;
                if m289 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m289 - 1;
                    let m291 = {
                        let c290: Fn12 = v0.clone();
                        c290.apply(v2.clone())?
                    };
                    match m291 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    f75(v0)
}

pub fn f74() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f75(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    let v1: List<List<(Bytes, Adt1)>> = {
        let (h295, _) = v0;
        h295
    };
    let v2: List<List<(Bytes, Adt1)>> = {
        let (_, h296) = v0;
        h296
    };
    let m297 = v1;
    match m297.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>)
        }
        Some((v3, v4)) => {
            let m298 = v4;
            match m298.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt1)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f76(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f76(v0: List<(Bytes, Adt1)>, v1: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v2: u64 = {
        let a303 = {
            let a301 = {
                let a299 = v0.clone();
                length_list(a299)
            };
            let a302 = {
                let a300 = v1.clone();
                length_list(a300)
            };
            nat_add(a301, a302)?
        };
        let a304 = 1u64;
        nat_add(a303, a304)?
    };
    Ok(f36({
        let (_, h305) = {
            let (_, h306) = {
                let (h307, _) = f77(Fn13::F78, v2, (v0, (v1, List::<(Bytes, Adt1)>::nil())));
                h307
            };
            h306
        };
        h305
    }))
}

pub fn f77(v0: Fn13, p312: u64, p313: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool) {
    let m311 = (p312, p313);
    {
        let mut m311 = m311;
        loop {
            let step = {
                let (v1, v2) = m311;
                let m308 = v1;
                if m308 == 0 {
                    Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, false))
                } else {
                    let v3 = m308 - 1;
                    let m310 = {
                        let c309: Fn13 = v0.clone();
                        c309.apply(v2.clone())
                    };
                    match m310 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    f79(v0)
}

pub fn f79(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    let v1: List<(Bytes, Adt1)> = {
        let (h314, _) = v0.clone();
        h314
    };
    let v2: List<(Bytes, Adt1)> = {
        let (h315, _) = {
            let (_, h316) = v0.clone();
            h316
        };
        h315
    };
    let v3: List<(Bytes, Adt1)> = {
        let (_, h317) = {
            let (_, h318) = v0;
            h318
        };
        h317
    };
    let m319 = v1.clone();
    match m319.uncons() {
        None => {
            let m320 = v2;
            match m320.uncons() {
                None => {
                    None::<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt1)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m321 = v2.clone();
            match m321.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt1)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f80({
                        let (h322, _) = v8.clone();
                        h322
                    }, {
                        let (h323, _) = v6.clone();
                        h323
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

pub fn f80(v0: Bytes, v1: Bytes) -> bool {
    let a328 = {
        let a324 = v0;
        let a325 = v1;
        compare_bytes(a324, a325)
    };
    let a329 = {
        let a326 = Bytes::lit(&[]);
        let a327 = Bytes::lit(&[0x01]);
        compare_bytes(a326, a327)
    };
    equal(a328, a329)
}
