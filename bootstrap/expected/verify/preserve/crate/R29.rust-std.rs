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
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, List<Adt2>, List<Adt4>, bool, Option<Adt6>, Adt8),
    C1(Bytes, Adt9, Bytes, Adt8),
    C2(List<Bytes>),
    C3(Bytes, bool, bool, Option<Bytes>, List<Bytes>, Adt5, Adt6, Adt11),
    C4(Bytes, List<Adt12>, Bytes),
    C5(Bytes, List<Adt13>),
    C6(Bytes, Adt11),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Adt3),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, Adt5, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(Bytes, Adt6, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, std::rc::Rc<Adt6>),
    C1(Bytes, std::rc::Rc<Adt6>),
    C2(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C3(Bytes),
    C4,
    C5(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes),
    C1(Bytes),
    C2(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C3(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C4(Option<Bytes>, Adt10, std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C5(Option<Bytes>),
    C6(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C7,
    C8(Bytes),
    C9,
    C10(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes),
    C1(Bytes),
    C2(Bytes),
    C3(u64),
    C4(Bytes),
    C5(Bytes, Adt6, std::rc::Rc<Adt11>),
    C6(Bytes, Adt6, std::rc::Rc<Adt11>),
    C7(std::rc::Rc<Adt11>, Bytes),
    C8(Adt6, std::rc::Rc<Adt11>),
    C9(Adt6, std::rc::Rc<Adt11>),
    C10(Bytes, Adt6, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C11(Adt5, std::rc::Rc<Adt11>),
    C12(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C13(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C14(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C15,
    C16(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C17,
    C18(Option<Bytes>, List<Bytes>, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes),
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes),
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Bytes>, List<Adt0>, List<Bytes>, List<Adt15>, List<Adt1>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(Adt14),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt16, p1: Adt1) -> R<Adt16> {
        match self {
            Fn0::F5(k0) => f5(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt1) -> List<Bytes> {
        match self {
            Fn1::F14 => f14(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt15>, p1: Adt1) -> List<Adt15> {
        match self {
            Fn2::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F18,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn3::F18 => f18(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F20,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn4::F20 => f20(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F44,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt2) -> List<Bytes> {
        match self {
            Fn5::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F46,
}

impl Fn6 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn6::F46 => f46(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F51,
}

impl Fn7 {
    pub fn apply(&self, p0: (bool, Adt16), p1: Adt2) -> R<(bool, Adt16)> {
        match self {
            Fn7::F51 => f51(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F53(List<Bytes>),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt16, p1: Adt4) -> R<Adt16> {
        match self {
            Fn8::F53(k0) => f53(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F55(u64, bool, Bytes),
    F104(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
        match self {
            Fn9::F55(k0, k1, k2) => f55(*k0, *k1, k2.clone(), p0),
            Fn9::F104(k0) => f104(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F59(Str),
    F69(List<Bytes>),
}

impl Fn10 {
    pub fn apply(&self, p0: (bool, Adt16), p1: Bytes) -> R<(bool, Adt16)> {
        match self {
            Fn10::F59(k0) => f59(k0.clone(), p0, p1),
            Fn10::F69(k0) => f69(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F66(Adt14),
}

impl Fn11 {
    pub fn apply(&self, p0: (List<Adt13>, Adt16), p1: Adt12) -> R<(List<Adt13>, Adt16)> {
        match self {
            Fn11::F66(k0) => f66(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F78(List<Bytes>),
    F110(List<Bytes>),
}

impl Fn12 {
    pub fn apply(&self, p0: Adt16, p1: Bytes) -> R<Adt16> {
        match self {
            Fn12::F78(k0) => f78(k0.clone(), p0, p1),
            Fn12::F110(k0) => f110(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F88,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt16, p1: Adt12) -> R<Adt16> {
        match self {
            Fn13::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F93(u8, Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn14::F93(k0, k1) => f93(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F99(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn15::F99(k0) => f99(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F101(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<List<Bytes>>, p1: Adt15) -> Option<List<Bytes>> {
        match self {
            Fn16::F101(k0) => f101(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F103(Bytes),
    F118(Bytes),
    F121(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn17::F103(k0) => f103(k0.clone(), p0),
            Fn17::F118(k0) => f118(k0.clone(), p0),
            Fn17::F121(k0) => Ok(f121(k0.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F108(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt0) -> List<Bytes> {
        match self {
            Fn18::F108(k0) => f108(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F113(Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: Option<List<Adt12>>, p1: Adt1) -> Option<List<Adt12>> {
        match self {
            Fn19::F113(k0) => f113(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: u64, v1: List<Bytes>, v2: List<Adt0>, v3: List<Adt1>) -> R<Result<Bytes, Bytes>> {
    let v4: Adt14 = f1(v1, v2, v3.clone());
    Ok(f2(f3(f4(Fn0::F5(v4), f3(f6(v0), Str::lit("\\documentclass[11pt]{article}\u{a}\\usepackage[T1]{fontenc}\u{a}\\usepackage{amsmath,amssymb}\u{a}\\begin{document}\u{a}\\section*{Tooling declarations}\u{a}"))?, v3)?, Str::lit("\\end{document}\u{a}"))?))
}

pub fn f1(v0: List<Bytes>, v1: List<Adt0>, v2: List<Adt1>) -> Adt14 {
    Adt14::C0(v0, v1, f7(v2.clone()), f8(v2.clone()), v2)
}

pub fn f2(v0: Adt16) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt16::C0(_, _, _, _, h1) => {
            h1
        }
    } {
        Err::<Bytes, Bytes>({
            let a2 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a2)
        })
    } else {
        Ok::<Bytes, Bytes>(f9(f10(List::cons(match v0.clone() {
            Adt16::C0(_, h3, _, _, _) => {
                h3
            }
        }, match v0 {
            Adt16::C0(h4, _, _, _, _) => {
                h4
            }
        }))))
    }
}

pub fn f3(v0: Adt16, v1: Str) -> R<Adt16> {
    f11(v0, {
        let a5 = v1;
        utf8_encode(a5)
    })
}

pub fn f4(v0: Fn0, p9: Adt16, p10: List<Adt1>) -> R<Adt16> {
    let m8 = (p9, p10);
    {
        let mut m8 = m8;
        loop {
            let step = {
                let (v1, v2) = m8;
                let m6 = v2;
                match m6.uncons() {
                    None => {
                        Err::<(Adt16, List<Adt1>), R<Adt16>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt16, List<Adt1>), R<Adt16>>(({
                            let c7: Fn0 = v0.clone();
                            c7.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: Adt14, v1: Adt16, v2: Adt1) -> R<Adt16> {
    f12(v1, v0, v2)
}

pub fn f6(v0: u64) -> Adt16 {
    Adt16::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f7(v0: List<Adt1>) -> List<Bytes> {
    f13(Fn1::F14, List::<Bytes>::nil(), v0)
}

pub fn f8(v0: List<Adt1>) -> List<Adt15> {
    f15(Fn2::F16, List::<Adt15>::nil(), v0)
}

pub fn f9(v0: List<Bytes>) -> Bytes {
    let m13 = {
        let (h11, _) = f17(Fn3::F18, {
            let a12 = v0.clone();
            length_list(a12)
        }, v0);
        h11
    };
    match m13.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f10(v0: List<Bytes>) -> List<Bytes> {
    f19(Fn4::F20, List::<Bytes>::nil(), v0)
}

pub fn f11(v0: Adt16, v1: Bytes) -> R<Adt16> {
    if match v0.clone() {
        Adt16::C0(_, _, _, _, h14) => {
            h14
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a17 = match v0.clone() {
                Adt16::C0(_, _, h15, _, _) => {
                    h15
                }
            };
            let a18 = {
                let a16 = v1.clone();
                length_bytes(a16)
            };
            nat_add(a17, a18)?
        };
        let a20 = v2;
        let a21 = match v0.clone() {
            Adt16::C0(_, _, _, h19, _) => {
                h19
            }
        };
        if nat_le(a20, a21) {
            Ok(f21(v0.clone(), {
                let a23 = match v0 {
                    Adt16::C0(_, h22, _, _, _) => {
                        h22
                    }
                };
                let a24 = v1;
                append_bytes(a23, a24)
            }, v2))
        } else {
            Ok(f22(v0))
        }
    }
}

pub fn f12(v0: Adt16, v1: Adt14, v2: Adt1) -> R<Adt16> {
    let m25 = v2.clone();
    match m25 {
        Adt1::C0(v3, v4, v5, v6, v7, v8) => {
            f23(f24(v0, v3, Str::lit("type"))?, v1, v4, v5, v6, v7, v8)
        }
        Adt1::C1(v9, v10, v11, v12) => {
            f25(f24(v0, v9, Str::lit("call"))?, v1, v10, v11, v12)
        }
        Adt1::C2(v13) => {
            f26(f24(v0, {
                let a26 = Str::lit("plumbing");
                utf8_encode(a26)
            }, Str::lit("plumbing"))?, v13)
        }
        Adt1::C3(v14, v15, v16, v17, _, _, _, _) => {
            f27(f28(f24(v0, v14, Str::lit("function"))?, v15, v16, v17)?, v1, v2, true)
        }
        Adt1::C4(v22, _, _) => {
            f27(f24(v0, v22, Str::lit("command"))?, v1, v2, true)
        }
        Adt1::C5(v25, _) => {
            f27(f24(v0, v25, Str::lit("invoke"))?, v1, v2, true)
        }
        Adt1::C6(v27, _) => {
            f27(f24(v0, v27, Str::lit("evaluate"))?, v1, v2, false)
        }
    }
}

pub fn f13(v0: Fn1, p30: List<Bytes>, p31: List<Adt1>) -> List<Bytes> {
    let m29 = (p30, p31);
    {
        let mut m29 = m29;
        loop {
            let step = {
                let (v1, v2) = m29;
                let m27 = v2;
                match m27.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt1>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt1>), List<Bytes>>(({
                            let c28: Fn1 = v0.clone();
                            c28.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m29 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: List<Bytes>, v1: Adt1) -> List<Bytes> {
    let m32 = v1;
    match m32 {
        Adt1::C0(_, _, _, _, _, _) => {
            v0
        }
        Adt1::C1(v8, _, _, _) => {
            List::cons(v8, v0)
        }
        Adt1::C2(_) => {
            v0
        }
        Adt1::C3(_, _, _, _, _, _, _, _) => {
            v0
        }
        Adt1::C4(_, _, _) => {
            v0
        }
        Adt1::C5(_, _) => {
            v0
        }
        Adt1::C6(_, _) => {
            v0
        }
    }
}

pub fn f15(v0: Fn2, p36: List<Adt15>, p37: List<Adt1>) -> List<Adt15> {
    let m35 = (p36, p37);
    {
        let mut m35 = m35;
        loop {
            let step = {
                let (v1, v2) = m35;
                let m33 = v2;
                match m33.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt1>), List<Adt15>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt15>, List<Adt1>), List<Adt15>>(({
                            let c34: Fn2 = v0.clone();
                            c34.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m35 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Adt15>, v1: Adt1) -> List<Adt15> {
    let m38 = v1;
    match m38 {
        Adt1::C0(v2, v3, _, _, _, _) => {
            List::cons(Adt15::C0(v2, f29(v3)), v0)
        }
        Adt1::C1(_, _, _, _) => {
            v0
        }
        Adt1::C2(_) => {
            v0
        }
        Adt1::C3(_, _, _, _, _, _, _, _) => {
            v0
        }
        Adt1::C4(_, _, _) => {
            v0
        }
        Adt1::C5(_, _) => {
            v0
        }
        Adt1::C6(_, _) => {
            v0
        }
    }
}

pub fn f17(v0: Fn3, p43: u64, p44: List<Bytes>) -> (List<Bytes>, bool) {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v1, v2) = m42;
                let m39 = v1;
                if m39 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m39 - 1;
                    let m41 = {
                        let c40: Fn3 = v0.clone();
                        c40.apply(v2.clone())
                    };
                    match m41 {
                        None => {
                            Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), (List<Bytes>, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m42 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<Bytes>) -> Option<List<Bytes>> {
    f30(v0)
}

pub fn f19(v0: Fn4, p48: List<Bytes>, p49: List<Bytes>) -> List<Bytes> {
    let m47 = (p48, p49);
    {
        let mut m47 = m47;
        loop {
            let step = {
                let (v1, v2) = m47;
                let m45 = v2;
                match m45.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c46: Fn4 = v0.clone();
                            c46.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m47 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f21(v0: Adt16, v1: Bytes, v2: u64) -> Adt16 {
    let a51 = 256u64;
    let a52 = {
        let a50 = v1.clone();
        length_bytes(a50)
    };
    if nat_le(a51, a52) {
        Adt16::C0(List::cons(v1, match v0.clone() {
            Adt16::C0(h53, _, _, _, _) => {
                h53
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt16::C0(_, _, _, h54, _) => {
                h54
            }
        }, false)
    } else {
        Adt16::C0(match v0.clone() {
            Adt16::C0(h55, _, _, _, _) => {
                h55
            }
        }, v1, v2, match v0 {
            Adt16::C0(_, _, _, h56, _) => {
                h56
            }
        }, false)
    }
}

pub fn f22(v0: Adt16) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt16::C0(h57, _, _, _, _) => {
            h57
        }
    }, match v0.clone() {
        Adt16::C0(_, h58, _, _, _) => {
            h58
        }
    }, match v0.clone() {
        Adt16::C0(_, _, h59, _, _) => {
            h59
        }
    }, match v0 {
        Adt16::C0(_, _, _, h60, _) => {
            h60
        }
    }, true)
}

pub fn f23(v0: Adt16, v1: Adt14, v2: List<Adt2>, v3: List<Adt4>, v4: bool, v5: Option<Adt6>, v6: Adt8) -> R<Adt16> {
    let v7: List<Bytes> = match v1 {
        Adt14::C0(h61, _, _, _, _) => {
            h61
        }
    };
    let v8: Adt16 = f31(v0, v6)?;
    let v9: Adt16 = if v4 {
        f32(v8.clone(), f3(f33(v8), Str::lit("Constructors are not pinned (opaque)"))?)?
    } else {
        v8
    };
    let v11: Adt16 = {
        let m62 = v5;
        match m62 {
            None => {
                v9
            }
            Some(v10) => {
                f32(v9.clone(), f34(f3(f33(v9), Str::lit("Abbreviation of: "))?, v7.clone(), v10, 0u64, true)?)?
            }
        }
    };
    let v12: Adt16 = {
        let a64 = 0u64;
        let a65 = {
            let a63 = v2.clone();
            length_list(a63)
        };
        if nat_lt(a64, a65) {
            f32(v11.clone(), f35(f3(f33(v11), Str::lit("Parameters: "))?, v2)?)?
        } else {
            v11
        }
    };
    f36(v12, v7, v3)
}

pub fn f24(v0: Adt16, v1: Bytes, v2: Str) -> R<Adt16> {
    f3(f3(f3(f37(f3(v0, Str::lit("\\subsection*{\\texttt{"))?, v1.clone(), {
        let a66 = v1;
        length_bytes(a66)
    }, false)?, Str::lit("}}\u{a}\\noindent Kind: \\texttt{"))?, v2)?, Str::lit("}.\\par\u{a}"))
}

pub fn f25(v0: Adt16, v1: Adt14, v2: Adt9, v3: Bytes, v4: Adt8) -> R<Adt16> {
    f31(f32(f32(v0.clone(), f38(f3(f33(v0.clone()), Str::lit("Signature: "))?, v1, v2, true)?)?, f39(f3(f33(v0), Str::lit("Role: "))?, v3)?)?, v4)
}

pub fn f26(v0: Adt16, v1: List<Bytes>) -> R<Adt16> {
    f32(v0.clone(), f40(f3(f33(v0), Str::lit("Names: "))?, v1, Str::lit(", "))?)
}

pub fn f27(v0: Adt16, v1: Adt14, v2: Adt1, v3: bool) -> R<Adt16> {
    f41(v0.clone(), f42(f33(v0), v1, v2)?, true, v3)
}

pub fn f28(v0: Adt16, v1: bool, v2: bool, v3: Option<Bytes>) -> R<Adt16> {
    let v4: Adt16 = f3(f3(f3(f3(f33(v0.clone()), Str::lit("Visibility: "))?, if v1 {
        Str::lit("public")
    } else {
        Str::lit("private")
    })?, Str::lit("; recursion: "))?, if v2 {
        Str::lit("partial")
    } else {
        Str::lit("total")
    })?;
    f32(v0, {
        let m67 = v3;
        match m67 {
            None => {
                v4
            }
            Some(v5) => {
                f39(f3(v4, Str::lit("; mutual group "))?, v5)?
            }
        }
    })
}

pub fn f29(v0: List<Adt2>) -> List<Bytes> {
    f10(f43(Fn5::F44, List::<Bytes>::nil(), v0))
}

pub fn f30(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m68 = v0.clone();
    match m68.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m69 = v2;
            match m69.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f45(Fn6::F46, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m71 = {
                        let (h70, _) = v5;
                        h70
                    };
                    match m71 {
                        None => {
                            Some(f10({
                                let (_, h73) = v5;
                                h73
                            }))
                        }
                        Some(v6) => {
                            Some(f10(List::cons(v6, {
                                let (_, h72) = v5;
                                h72
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f31(v0: Adt16, v1: Adt8) -> R<Adt16> {
    f32(v0.clone(), f3(f39(f3(f39(f3(f33(v0), Str::lit("Source: "))?, match v1.clone() {
        Adt8::C0(h74, _) => {
            h74
        }
    })?, Str::lit(" ("))?, match v1 {
        Adt8::C0(_, h75) => {
            h75
        }
    })?, Str::lit(")"))?)
}

pub fn f32(v0: Adt16, v1: Adt16) -> R<Adt16> {
    f41(v0, v1, false, false)
}

pub fn f33(v0: Adt16) -> Adt16 {
    f6(match v0 {
        Adt16::C0(_, _, _, h76, _) => {
            h76
        }
    })
}

pub fn f34(v0: Adt16, v1: List<Bytes>, v2: Adt6, v3: u64, v4: bool) -> R<Adt16> {
    let m77 = v2;
    match m77 {
        Adt6::C0(v5, r80) => {
            let v6 = (*r80).clone();
            if f47(v6.clone()) {
                f48(v0, v1, v5)
            } else {
                f3(f34(f3(f48(f3(v0, Str::lit("("))?, v1.clone(), v5)?, Str::lit(" "))?, v1, v6, 1u64, true)?, Str::lit(")"))
            }
        }
        Adt6::C1(v7, r81) => {
            let v8 = (*r81).clone();
            if f47(v8.clone()) {
                f48(v0, v1, v7)
            } else {
                f3(f34(f3(f48(f3(v0, Str::lit("("))?, v1.clone(), v7)?, Str::lit(" "))?, v1, v8, 1u64, true)?, Str::lit(")"))
            }
        }
        Adt6::C2(r82, r83) => {
            let v9 = (*r82).clone();
            let v10 = (*r83).clone();
            f3(f34(f49(f34(f3(v0, Str::lit("("))?, v1.clone(), v9, 2u64, true)?)?, v1, v10, 0u64, true)?, Str::lit(")"))
        }
        Adt6::C3(v11) => {
            f48(v0, v1, v11)
        }
        Adt6::C4 => {
            Ok(v0)
        }
        Adt6::C5(r84, r85) => {
            let v12 = (*r84).clone();
            let v13 = (*r85).clone();
            f34(f34(if v4 {
                v0
            } else {
                let a78 = v3;
                let a79 = 2u64;
                if nat_eq(a78, a79) {
                    f49(v0)?
                } else {
                    f3(v0, Str::lit(" "))?
                }
            }, v1.clone(), v12, 0u64, true)?, v1, v13, v3, false)
        }
    }
}

pub fn f35(v0: Adt16, v1: List<Adt2>) -> R<Adt16> {
    let (_, h86) = f50(Fn7::F51, (true, v0), v1)?;
    Ok(h86)
}

pub fn f36(v0: Adt16, v1: List<Bytes>, v2: List<Adt4>) -> R<Adt16> {
    f52(Fn8::F53(v1), v0, v2)
}

pub fn f37(v0: Adt16, v1: Bytes, v2: u64, v3: bool) -> R<Adt16> {
    let (_, h87) = {
        let (h88, _) = f54(Fn9::F55(v2, v3, v1), v2, (0u64, v0))?;
        h88
    };
    Ok(h87)
}

pub fn f38(v0: Adt16, v1: Adt14, v2: Adt9, v3: bool) -> R<Adt16> {
    let m89 = v2;
    match m89 {
        Adt9::C0(v4) => {
            f48(v0, match v1 {
                Adt14::C0(h90, _, _, _, _) => {
                    h90
                }
            }, v4)
        }
        Adt9::C1(v5) => {
            f48(v0, match v1 {
                Adt14::C0(h91, _, _, _, _) => {
                    h91
                }
            }, v5)
        }
        Adt9::C2(r98, r99) => {
            let v6 = (*r98).clone();
            let v7 = (*r99).clone();
            let v8: Adt16 = if f56(v1.clone(), v6.clone(), f57(v7.clone())?) {
                f3(v0, Str::lit("(@"))?
            } else {
                f3(v0, Str::lit("("))?
            };
            f3(f38(f3(f38(v8, v1.clone(), v6, true)?, Str::lit(" "))?, v1, v7, true)?, Str::lit(")"))
        }
        Adt9::C3(r100, r101) => {
            let v9 = (*r100).clone();
            let v10 = (*r101).clone();
            f3(f38(f49(f38(f3(v0, Str::lit("("))?, v1.clone(), v9, true)?)?, v1, v10, true)?, Str::lit(")"))
        }
        Adt9::C4(v11, v12, r102, r103) => {
            let v13 = (*r102).clone();
            let v14 = (*r103).clone();
            let v15: Adt16 = f3(v0, Str::lit("("))?;
            f3(f38(f49({
                let m92 = v11;
                match m92 {
                    None => {
                        f3(f38(f3(v15, Str::lit("["))?, v1.clone(), v13, true)?, Str::lit("]"))?
                    }
                    Some(v16) => {
                        let m93 = v12;
                        match m93 {
                            Adt10::C0 => {
                                f3(f38(f3(f48(f3(v15, Str::lit("("))?, match v1.clone() {
                                    Adt14::C0(h94, _, _, _, _) => {
                                        h94
                                    }
                                }, v16)?, Str::lit(" : "))?, v1.clone(), v13, true)?, Str::lit(")"))?
                            }
                            Adt10::C1 => {
                                f3(f38(f3(f48(f3(v15, Str::lit("{"))?, match v1.clone() {
                                    Adt14::C0(h95, _, _, _, _) => {
                                        h95
                                    }
                                }, v16)?, Str::lit(" : "))?, v1.clone(), v13, true)?, Str::lit("}"))?
                            }
                            Adt10::C2 => {
                                f3(f38(f3(f48(f3(v15, Str::lit("["))?, match v1.clone() {
                                    Adt14::C0(h96, _, _, _, _) => {
                                        h96
                                    }
                                }, v16)?, Str::lit(" : "))?, v1.clone(), v13, true)?, Str::lit("]"))?
                            }
                        }
                    }
                }
            })?, v1, v14, true)?, Str::lit(")"))
        }
        Adt9::C5(v17) => {
            let m97 = v17;
            match m97 {
                None => {
                    f3(v0, Str::lit("Type"))
                }
                Some(v18) => {
                    f3(f39(f3(v0, Str::lit("(Type "))?, v18)?, Str::lit(")"))
                }
            }
        }
        Adt9::C6(r104, r105) => {
            let v19 = (*r104).clone();
            let v20 = (*r105).clone();
            f3(f38(f3(f38(f3(v0, Str::lit("(optParam "))?, v1.clone(), v19, true)?, Str::lit(" "))?, v1, v20, true)?, Str::lit(")"))
        }
        Adt9::C7 => {
            f3(v0, Str::lit("{ }"))
        }
        Adt9::C8(v21) => {
            f39(v0, v21)
        }
        Adt9::C9 => {
            Ok(v0)
        }
        Adt9::C10(r106, r107) => {
            let v22 = (*r106).clone();
            let v23 = (*r107).clone();
            f38(f38(if v3 {
                v0
            } else {
                f3(v0, Str::lit(" "))?
            }, v1.clone(), v22, true)?, v1, v23, false)
        }
    }
}

pub fn f39(v0: Adt16, v1: Bytes) -> R<Adt16> {
    f11(v0, v1)
}

pub fn f40(v0: Adt16, v1: List<Bytes>, v2: Str) -> R<Adt16> {
    let (_, h108) = f58(Fn10::F59(v2), (true, v0), v1)?;
    Ok(h108)
}

pub fn f41(v0: Adt16, v1: Adt16, v2: bool, v3: bool) -> R<Adt16> {
    let m109 = f2(v1);
    match m109 {
        Ok(v4) => {
            f60(v0, v4, v2, v3)
        }
        Err(_) => {
            Ok(f22(v0))
        }
    }
}

pub fn f42(v0: Adt16, v1: Adt14, v2: Adt1) -> R<Adt16> {
    let v3: List<Bytes> = match v1.clone() {
        Adt14::C0(h110, _, _, _, _) => {
            h110
        }
    };
    let m111 = v2;
    match m111 {
        Adt1::C0(_, _, _, _, _, _) => {
            Ok(v0)
        }
        Adt1::C1(_, _, _, _) => {
            Ok(v0)
        }
        Adt1::C2(_) => {
            Ok(v0)
        }
        Adt1::C3(v15, v16, v17, _, v19, v20, v21, v22) => {
            let v23: Adt16 = if v16 {
                f3(v0, Str::lit("public "))?
            } else {
                v0
            };
            let v24: Adt16 = f3(v23, if v17 {
                Str::lit("meta partial def ")
            } else {
                Str::lit("meta def ")
            })?;
            let v25: Adt16 = f61(f48(v24, v3.clone(), v15)?, v3.clone(), v19)?;
            let v26: Adt16 = f3(f34(f3(f62(v25, v3.clone(), v20, true, true)?, Str::lit(" : "))?, v3, v21, 0u64, true)?, Str::lit(" :=\u{a}  "))?;
            f3(f63(v26, v1, v22, 0u64, true)?, Str::lit("\u{a}"))
        }
        Adt1::C4(v27, v28, v29) => {
            f3(f48(f3(f48(f3(f64(f3(f48(f3(v0, Str::lit("syntax (name := "))?, v3.clone(), v27.clone())?, Str::lit(") "))?, v28)?, Str::lit(": command\u{a}attribute [command_elab "))?, v3.clone(), v27)?, Str::lit("] "))?, v3, v29)?, Str::lit("\u{a}"))
        }
        Adt1::C5(v30, v31) => {
            f3({
                let (_, h112) = f65(Fn11::F66(v1.clone()), (v31, v0), f67(match v1 {
                    Adt14::C0(_, _, _, _, h113) => {
                        h113
                    }
                }, v30))?;
                h112
            }, Str::lit("\u{a}"))
        }
        Adt1::C6(v32, v33) => {
            f3(f63(f48(f3(v0, Str::lit("#eval "))?, v3, v32)?, v1, v33, 1u64, true)?, Str::lit("\u{a}"))
        }
    }
}

pub fn f43(v0: Fn5, p117: List<Bytes>, p118: List<Adt2>) -> List<Bytes> {
    let m116 = (p117, p118);
    {
        let mut m116 = m116;
        loop {
            let step = {
                let (v1, v2) = m116;
                let m114 = v2;
                match m114.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt2>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt2>), List<Bytes>>(({
                            let c115: Fn5 = v0.clone();
                            c115.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: List<Bytes>, v1: Adt2) -> List<Bytes> {
    List::cons(match v1 {
        Adt2::C0(h119, _) => {
            h119
        }
    }, v0)
}

pub fn f45(v0: Fn6, p123: (Option<Bytes>, List<Bytes>), p124: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m122 = (p123, p124);
    {
        let mut m122 = m122;
        loop {
            let step = {
                let (v1, v2) = m122;
                let m120 = v2;
                match m120.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c121: Fn6 = v0.clone();
                            c121.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f68(v0, v1)
}

pub fn f47(v0: Adt6) -> bool {
    let m125 = v0;
    match m125 {
        Adt6::C0(_, _) => {
            false
        }
        Adt6::C1(_, _) => {
            false
        }
        Adt6::C2(_, _) => {
            false
        }
        Adt6::C3(_) => {
            false
        }
        Adt6::C4 => {
            true
        }
        Adt6::C5(_, _) => {
            false
        }
    }
}

pub fn f48(v0: Adt16, v1: List<Bytes>, v2: Bytes) -> R<Adt16> {
    let (_, h126) = f58(Fn10::F69(v1), (true, v0), f70(v2, 46u8)?)?;
    Ok(h126)
}

pub fn f49(v0: Adt16) -> R<Adt16> {
    f39(v0, Bytes::lit(&[0x20, 0xe2, 0x86, 0x92, 0x20]))
}

pub fn f50(v0: Fn7, p130: (bool, Adt16), p131: List<Adt2>) -> R<(bool, Adt16)> {
    let m129 = (p130, p131);
    {
        let mut m129 = m129;
        loop {
            let step = {
                let (v1, v2) = m129;
                let m127 = v2;
                match m127.uncons() {
                    None => {
                        Err::<((bool, Adt16), List<Adt2>), R<(bool, Adt16)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt16), List<Adt2>), R<(bool, Adt16)>>(({
                            let c128: Fn7 = v0.clone();
                            c128.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: (bool, Adt16), v1: Adt2) -> R<(bool, Adt16)> {
    Ok((false, f71({
        let (h132, _) = v0;
        if h132 {
            let (_, h133) = v0;
            h133
        } else {
            f3({
                let (_, h134) = v0;
                h134
            }, Str::lit(", "))?
        }
    }, v1)?))
}

pub fn f52(v0: Fn8, p138: Adt16, p139: List<Adt4>) -> R<Adt16> {
    let m137 = (p138, p139);
    {
        let mut m137 = m137;
        loop {
            let step = {
                let (v1, v2) = m137;
                let m135 = v2;
                match m135.uncons() {
                    None => {
                        Err::<(Adt16, List<Adt4>), R<Adt16>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt16, List<Adt4>), R<Adt16>>(({
                            let c136: Fn8 = v0.clone();
                            c136.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: List<Bytes>, v1: Adt16, v2: Adt4) -> R<Adt16> {
    f32(v1.clone(), f72(f3(f33(v1), Str::lit("Constructor: "))?, v0, v2)?)
}

pub fn f54(v0: Fn9, p144: u64, p145: (u64, Adt16)) -> R<((u64, Adt16), bool)> {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v1, v2) = m143;
                let m140 = v1;
                if m140 == 0 {
                    Err::<(u64, (u64, Adt16)), R<((u64, Adt16), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m140 - 1;
                    let m142 = {
                        let c141: Fn9 = v0.clone();
                        c141.apply(v2.clone())?
                    };
                    match m142 {
                        None => {
                            Err::<(u64, (u64, Adt16)), R<((u64, Adt16), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt16)), R<((u64, Adt16), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: u64, v1: bool, v2: Bytes, v3: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
    f73(v2, v1, v0, v3)
}

pub fn f56(v0: Adt14, v1: Adt9, v2: u64) -> bool {
    let m146 = v1;
    match m146 {
        Adt9::C0(v3) => {
            if f74(match v0.clone() {
                Adt14::C0(_, _, h147, _, _) => {
                    h147
                }
            }, v3.clone()) {
                true
            } else {
                let m149 = f75(match v0 {
                    Adt14::C0(_, _, _, h148, _) => {
                        h148
                    }
                }, v3);
                match m149 {
                    None => {
                        false
                    }
                    Some(v4) => {
                        let a151 = {
                            let a150 = v4;
                            length_list(a150)
                        };
                        let a152 = v2;
                        nat_lt(a151, a152)
                    }
                }
            }
        }
        Adt9::C1(_) => {
            false
        }
        Adt9::C2(_, _) => {
            false
        }
        Adt9::C3(_, _) => {
            false
        }
        Adt9::C4(_, _, _, _) => {
            false
        }
        Adt9::C5(_) => {
            false
        }
        Adt9::C6(_, _) => {
            false
        }
        Adt9::C7 => {
            false
        }
        Adt9::C8(_) => {
            false
        }
        Adt9::C9 => {
            false
        }
        Adt9::C10(_, _) => {
            false
        }
    }
}

pub fn f57(v0: Adt9) -> R<u64> {
    let m153 = v0;
    match m153 {
        Adt9::C0(_) => {
            Ok(0u64)
        }
        Adt9::C1(_) => {
            Ok(0u64)
        }
        Adt9::C2(_, _) => {
            Ok(0u64)
        }
        Adt9::C3(_, _) => {
            Ok(0u64)
        }
        Adt9::C4(_, _, _, _) => {
            Ok(0u64)
        }
        Adt9::C5(_) => {
            Ok(0u64)
        }
        Adt9::C6(_, _) => {
            Ok(0u64)
        }
        Adt9::C7 => {
            Ok(0u64)
        }
        Adt9::C8(_) => {
            Ok(0u64)
        }
        Adt9::C9 => {
            Ok(0u64)
        }
        Adt9::C10(_, r156) => {
            let v16 = (*r156).clone();
            let a154 = f57(v16)?;
            let a155 = 1u64;
            nat_add(a154, a155)
        }
    }
}

pub fn f58(v0: Fn10, p160: (bool, Adt16), p161: List<Bytes>) -> R<(bool, Adt16)> {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v1, v2) = m159;
                let m157 = v2;
                match m157.uncons() {
                    None => {
                        Err::<((bool, Adt16), List<Bytes>), R<(bool, Adt16)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt16), List<Bytes>), R<(bool, Adt16)>>(({
                            let c158: Fn10 = v0.clone();
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

pub fn f59(v0: Str, v1: (bool, Adt16), v2: Bytes) -> R<(bool, Adt16)> {
    Ok((false, f39({
        let (h162, _) = v1;
        if h162 {
            let (_, h163) = v1;
            h163
        } else {
            f3({
                let (_, h164) = v1;
                h164
            }, v0)?
        }
    }, v2)?))
}

pub fn f60(v0: Adt16, v1: Bytes, v2: bool, v3: bool) -> R<Adt16> {
    f3(f37(f3(v0, Str::lit("\\noindent \\texttt{"))?, v1.clone(), if v2 {
        f76(v1)?
    } else {
        let a165 = v1;
        length_bytes(a165)
    }, v3)?, Str::lit("}\\par\u{a}"))
}

pub fn f61(v0: Adt16, v1: List<Bytes>, v2: List<Bytes>) -> R<Adt16> {
    f77(Fn12::F78(v1), v0, v2)
}

pub fn f62(p169: Adt16, v1: List<Bytes>, p170: Adt5, v3: bool, p171: bool) -> R<Adt16> {
    let m168 = (p169, (p170, p171));
    {
        let mut m168 = m168;
        loop {
            let step = {
                let (v0, (v2, v4)) = m168;
                let m166 = v2;
                match m166 {
                    Adt5::C0 => {
                        Err::<(Adt16, (Adt5, bool)), R<Adt16>>(Ok(v0))
                    }
                    Adt5::C1(v5, v6, r167) => {
                        let v7 = (*r167).clone();
                        Ok::<(Adt16, (Adt5, bool)), R<Adt16>>((f3(f34(f3(f48(f3(if v4 {
                            if v3 {
                                f3(v0, Str::lit(" "))?
                            } else {
                                v0
                            }
                        } else {
                            f3(v0, Str::lit(" "))?
                        }, Str::lit("("))?, v1.clone(), v5)?, Str::lit(" : "))?, v1.clone(), v6, 0u64, true)?, Str::lit(")"))?, (v7, false)))
                    }
                }
            };
            match step {
                Ok(next) => { m168 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Adt16, v1: Adt14, v2: Adt11, v3: u64, v4: bool) -> R<Adt16> {
    let m172 = v2;
    match m172 {
        Adt11::C0(v5) => {
            f48(v0, match v1 {
                Adt14::C0(h173, _, _, _, _) => {
                    h173
                }
            }, v5)
        }
        Adt11::C1(v6) => {
            f39(v0, v6)
        }
        Adt11::C2(v7) => {
            f79(v0, v7)
        }
        Adt11::C3(v8) => {
            f80(v0, v8)
        }
        Adt11::C4(v9) => {
            f48(f3(v0, Str::lit("``"))?, match v1 {
                Adt14::C0(h174, _, _, _, _) => {
                    h174
                }
            }, v9)
        }
        Adt11::C5(v10, v11, r192) => {
            let v12 = (*r192).clone();
            f3(f63(f81(f48(f3(v0, Str::lit("("))?, match v1.clone() {
                Adt14::C0(h175, _, _, _, _) => {
                    h175
                }
            }, v10.clone())?, match v1.clone() {
                Adt14::C0(h176, _, _, _, _) => {
                    h176
                }
            }, f82(match v1.clone() {
                Adt14::C0(_, h177, _, _, _) => {
                    h177
                }
            }, v10), v11)?, v1, v12, 1u64, true)?, Str::lit(")"))
        }
        Adt11::C6(v13, v14, r193) => {
            let v15 = (*r193).clone();
            f3(f63(f81(f48(f3(v0, Str::lit("("))?, match v1.clone() {
                Adt14::C0(h178, _, _, _, _) => {
                    h178
                }
            }, v13.clone())?, match v1.clone() {
                Adt14::C0(h179, _, _, _, _) => {
                    h179
                }
            }, f83(match v1.clone() {
                Adt14::C0(_, _, _, h180, _) => {
                    h180
                }
            }, f84(v13)?), v14)?, v1, v15, 1u64, true)?, Str::lit(")"))
        }
        Adt11::C7(r194, v17) => {
            let v16 = (*r194).clone();
            f48(f3(f63(f3(v0, Str::lit("("))?, v1.clone(), v16, 0u64, true)?, Str::lit(")."))?, match v1 {
                Adt14::C0(h181, _, _, _, _) => {
                    h181
                }
            }, v17)
        }
        Adt11::C8(v18, r195) => {
            let v19 = (*r195).clone();
            f3(f34(f3(f63(f3(v0, Str::lit("((["))?, v1.clone(), v19, 2u64, true)?, Str::lit("] : List "))?, match v1 {
                Adt14::C0(h182, _, _, _, _) => {
                    h182
                }
            }, v18, 0u64, true)?, Str::lit("))"))
        }
        Adt11::C9(v20, r196) => {
            let v21 = (*r196).clone();
            f3(f34(f3(f63(f3(v0, Str::lit("((#["))?, v1.clone(), v21, 2u64, true)?, Str::lit("] : Array "))?, match v1 {
                Adt14::C0(h183, _, _, _, _) => {
                    h183
                }
            }, v20, 0u64, true)?, Str::lit("))"))
        }
        Adt11::C10(v22, v23, r197, r198) => {
            let v24 = (*r197).clone();
            let v25 = (*r198).clone();
            f3(f63(f3(f63(f3(f34(f3(f48(f3(v0, Str::lit("(let "))?, match v1.clone() {
                Adt14::C0(h184, _, _, _, _) => {
                    h184
                }
            }, v22)?, Str::lit(" : "))?, match v1.clone() {
                Adt14::C0(h185, _, _, _, _) => {
                    h185
                }
            }, v23, 0u64, true)?, Str::lit(" := "))?, v1.clone(), v24, 0u64, true)?, Str::lit("; "))?, v1, v25, 0u64, true)?, Str::lit(")"))
        }
        Adt11::C11(v26, r199) => {
            let v27 = (*r199).clone();
            f3(f63(f3(f62(f3(v0, Str::lit("(fun "))?, match v1.clone() {
                Adt14::C0(h186, _, _, _, _) => {
                    h186
                }
            }, v26, false, true)?, Str::lit(" => "))?, v1, v27, 0u64, true)?, Str::lit(")"))
        }
        Adt11::C12(r200, r201) => {
            let v28 = (*r200).clone();
            let v29 = (*r201).clone();
            f3(f63(f63(f3(v0, Str::lit("("))?, v1.clone(), v28, 0u64, true)?, v1, v29, 1u64, true)?, Str::lit(")"))
        }
        Adt11::C13(r202, r203, r204) => {
            let v30 = (*r202).clone();
            let v31 = (*r203).clone();
            let v32 = (*r204).clone();
            f3(f63(f3(f63(f3(f63(f3(v0, Str::lit("(if "))?, v1.clone(), v30, 0u64, true)?, Str::lit(" then "))?, v1.clone(), v31, 0u64, true)?, Str::lit(" else "))?, v1, v32, 0u64, true)?, Str::lit(")"))
        }
        Adt11::C14(r205, r206) => {
            let v33 = (*r205).clone();
            let v34 = (*r206).clone();
            f3(f63(f3(f63(f3(v0, Str::lit("(match "))?, v1.clone(), v33, 0u64, true)?, Str::lit(" with"))?, v1, v34, 0u64, true)?, Str::lit(")"))
        }
        Adt11::C15 => {
            Ok(v0)
        }
        Adt11::C16(r207, r208) => {
            let v35 = (*r207).clone();
            let v36 = (*r208).clone();
            f63(f63({
                let a187 = v3;
                let a188 = 1u64;
                if nat_eq(a187, a188) {
                    f3(v0, Str::lit(" "))?
                } else {
                    if v4 {
                        v0
                    } else {
                        f3(v0, Str::lit(", "))?
                    }
                }
            }, v1.clone(), v35, 0u64, true)?, v1, v36, v3, false)
        }
        Adt11::C17 => {
            Ok(v0)
        }
        Adt11::C18(v37, v38, r209, r210) => {
            let v39 = (*r209).clone();
            let v40 = (*r210).clone();
            f63(f63(f3({
                let m189 = v37;
                match m189 {
                    None => {
                        f3(v0, Str::lit(" | _"))?
                    }
                    Some(v41) => {
                        f85(f48(f3(f3(v0, Str::lit(" | "))?, Str::lit("."))?, match v1.clone() {
                            Adt14::C0(h190, _, _, _, _) => {
                                h190
                            }
                        }, f86(v41)?)?, match v1.clone() {
                            Adt14::C0(h191, _, _, _, _) => {
                                h191
                            }
                        }, v38)?
                    }
                }
            }, Str::lit(" => "))?, v1.clone(), v39, 0u64, true)?, v1, v40, 0u64, true)
        }
    }
}

pub fn f64(v0: Adt16, v1: List<Adt12>) -> R<Adt16> {
    f87(Fn13::F88, v0, v1)
}

pub fn f65(v0: Fn11, p214: (List<Adt13>, Adt16), p215: List<Adt12>) -> R<(List<Adt13>, Adt16)> {
    let m213 = (p214, p215);
    {
        let mut m213 = m213;
        loop {
            let step = {
                let (v1, v2) = m213;
                let m211 = v2;
                match m211.uncons() {
                    None => {
                        Err::<((List<Adt13>, Adt16), List<Adt12>), R<(List<Adt13>, Adt16)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt13>, Adt16), List<Adt12>), R<(List<Adt13>, Adt16)>>(({
                            let c212: Fn11 = v0.clone();
                            c212.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Adt14, v1: (List<Adt13>, Adt16), v2: Adt12) -> R<(List<Adt13>, Adt16)> {
    f89(v0, v1, v2)
}

pub fn f67(v0: List<Adt1>, v1: Bytes) -> List<Adt12> {
    let m216 = f90(v0, v1);
    match m216 {
        None => {
            List::<Adt12>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f68(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m218 = {
        let (h217, _) = v0;
        h217
    };
    match m218 {
        None => {
            (Some(v1), {
                let (_, h219) = v0;
                h219
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a220 = v2;
                let a221 = v1;
                append_bytes(a220, a221)
            }, {
                let (_, h222) = v0;
                h222
            }))
        }
    }
}

pub fn f69(v0: List<Bytes>, v1: (bool, Adt16), v2: Bytes) -> R<(bool, Adt16)> {
    Ok((false, f91({
        let (h223, _) = v1;
        if h223 {
            let (_, h224) = v1;
            h224
        } else {
            f3({
                let (_, h225) = v1;
                h225
            }, Str::lit("."))?
        }
    }, v0, v2)?))
}

pub fn f70(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a226 = v0.clone();
        length_bytes(a226)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h227, _) = f92(Fn14::F93(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h227
    };
    let v4: u64 = {
        let (h228, _) = {
            let (_, h229) = v3.clone();
            h229
        };
        h228
    };
    Ok(f10(List::cons(f94(v0, v4, {
        let a230 = v2;
        let a231 = v4;
        nat_sub(a230, a231)
    }), {
        let (_, h232) = {
            let (_, h233) = v3;
            h233
        };
        h232
    })))
}

pub fn f71(v0: Adt16, v1: Adt2) -> R<Adt16> {
    let m235 = match v1.clone() {
        Adt2::C0(_, h234) => {
            h234
        }
    };
    match m235 {
        Adt3::C0 => {
            f3(f39(v0, match v1 {
                Adt2::C0(h236, _) => {
                    h236
                }
            })?, Str::lit(" : Type"))
        }
        Adt3::C1 => {
            f3(f39(v0, match v1 {
                Adt2::C0(h237, _) => {
                    h237
                }
            })?, Str::lit(" : Type -> Type"))
        }
        Adt3::C2(v2) => {
            f39(f3(f39(v0, match v1 {
                Adt2::C0(h238, _) => {
                    h238
                }
            })?, Str::lit(" : "))?, v2)
        }
    }
}

pub fn f72(v0: Adt16, v1: List<Bytes>, v2: Adt4) -> R<Adt16> {
    let m240 = match v2.clone() {
        Adt4::C0(_, _, h239) => {
            h239
        }
    };
    match m240 {
        None => {
            f95(f3(f39(v0, match v2.clone() {
                Adt4::C0(h245, _, _) => {
                    h245
                }
            })?, Str::lit(" "))?, v1, match v2 {
                Adt4::C0(_, h246, _) => {
                    h246
                }
            }, true)
        }
        Some(v3) => {
            f3(f39(f3(f39(f3(f95(f3(f39(v0, match v2.clone() {
                Adt4::C0(h241, _, _) => {
                    h241
                }
            })?, Str::lit(" "))?, v1, match v2 {
                Adt4::C0(_, h242, _) => {
                    h242
                }
            }, true)?, Str::lit(" [only at "))?, match v3.clone() {
                Adt7::C0(h243, _) => {
                    h243
                }
            })?, Str::lit(" = "))?, match v3 {
                Adt7::C0(_, h244) => {
                    h244
                }
            })?, Str::lit("]"))
        }
    }
}

pub fn f73(v0: Bytes, v1: bool, v2: u64, v3: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
    let a248 = {
        let (h247, _) = v3.clone();
        h247
    };
    let a249 = v2;
    if nat_lt(a248, a249) {
        let v4: (u64, u64) = f96(v0.clone(), {
            let (h250, _) = v3.clone();
            h250
        })?;
        let v5: u64 = {
            let (h251, _) = v4;
            h251
        };
        Ok(Some(({
            let a254 = {
                let (h252, _) = v3.clone();
                h252
            };
            let a255 = {
                let (_, h253) = v4;
                h253
            };
            nat_add(a254, a255)?
        }, {
            let a256 = v5;
            let a257 = 92u64;
            if nat_eq(a256, a257) {
                f3({
                    let (_, h258) = v3;
                    h258
                }, Str::lit("\\textbackslash{}"))?
            } else {
                let a259 = v5;
                let a260 = 123u64;
                if nat_eq(a259, a260) {
                    f3({
                        let (_, h261) = v3;
                        h261
                    }, Str::lit("\\{"))?
                } else {
                    let a262 = v5;
                    let a263 = 125u64;
                    if nat_eq(a262, a263) {
                        f3({
                            let (_, h264) = v3;
                            h264
                        }, Str::lit("\\}"))?
                    } else {
                        let a265 = v5;
                        let a266 = 35u64;
                        if nat_eq(a265, a266) {
                            f3({
                                let (_, h267) = v3;
                                h267
                            }, Str::lit("\\#"))?
                        } else {
                            let a268 = v5;
                            let a269 = 36u64;
                            if nat_eq(a268, a269) {
                                f3({
                                    let (_, h270) = v3;
                                    h270
                                }, Str::lit("\\$"))?
                            } else {
                                let a271 = v5;
                                let a272 = 37u64;
                                if nat_eq(a271, a272) {
                                    f39({
                                        let (_, h273) = v3;
                                        h273
                                    }, Bytes::lit(&[0x5c, 0x25]))?
                                } else {
                                    let a274 = v5;
                                    let a275 = 38u64;
                                    if nat_eq(a274, a275) {
                                        f3({
                                            let (_, h276) = v3;
                                            h276
                                        }, Str::lit("\\&"))?
                                    } else {
                                        let a277 = v5;
                                        let a278 = 95u64;
                                        if nat_eq(a277, a278) {
                                            f3({
                                                let (_, h279) = v3;
                                                h279
                                            }, Str::lit("\\_"))?
                                        } else {
                                            let a280 = v5;
                                            let a281 = 94u64;
                                            if nat_eq(a280, a281) {
                                                f3({
                                                    let (_, h282) = v3;
                                                    h282
                                                }, Str::lit("\\^{}"))?
                                            } else {
                                                let a283 = v5;
                                                let a284 = 126u64;
                                                if nat_eq(a283, a284) {
                                                    f3({
                                                        let (_, h285) = v3;
                                                        h285
                                                    }, Str::lit("\\~{}"))?
                                                } else {
                                                    let a286 = v5;
                                                    let a287 = 128u64;
                                                    if nat_lt(a286, a287) {
                                                        if if v1 {
                                                            let a288 = v5;
                                                            let a289 = 10u64;
                                                            nat_eq(a288, a289)
                                                        } else {
                                                            false
                                                        } {
                                                            f3({
                                                                let (_, h290) = v3;
                                                                h290
                                                            }, Str::lit(" "))?
                                                        } else {
                                                            f39({
                                                                let (_, h291) = v3;
                                                                h291
                                                            }, f94(v0, {
                                                                let (h292, _) = v3;
                                                                h292
                                                            }, 1u64))?
                                                        }
                                                    } else {
                                                        f3(f97(f3({
                                                            let (_, h293) = v3;
                                                            h293
                                                        }, Str::lit("<U+"))?, Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46]), v5)?, Str::lit(">"))?
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt16)>)
    }
}

pub fn f74(v0: List<Bytes>, v1: Bytes) -> bool {
    f98(Fn15::F99(v1), false, v0)
}

pub fn f75(v0: List<Adt15>, v1: Bytes) -> Option<List<Bytes>> {
    f100(Fn16::F101(v1), None::<List<Bytes>>, v0)
}

pub fn f76(v0: Bytes) -> R<u64> {
    let (h294, _) = f102(Fn17::F103(v0.clone()), {
        let a295 = v0.clone();
        length_bytes(a295)
    }, {
        let a296 = v0;
        length_bytes(a296)
    })?;
    Ok(h294)
}

pub fn f77(v0: Fn12, p300: Adt16, p301: List<Bytes>) -> R<Adt16> {
    let m299 = (p300, p301);
    {
        let mut m299 = m299;
        loop {
            let step = {
                let (v1, v2) = m299;
                let m297 = v2;
                match m297.uncons() {
                    None => {
                        Err::<(Adt16, List<Bytes>), R<Adt16>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt16, List<Bytes>), R<Adt16>>(({
                            let c298: Fn12 = v0.clone();
                            c298.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<Bytes>, v1: Adt16, v2: Bytes) -> R<Adt16> {
    f3(f48(f3(v1, Str::lit(" {"))?, v0, v2)?, Str::lit(" : Type}"))
}

pub fn f79(v0: Adt16, v1: Bytes) -> R<Adt16> {
    f3({
        let (_, h302) = {
            let (h303, _) = f54(Fn9::F104(v1.clone()), {
                let a304 = v1;
                length_bytes(a304)
            }, (0u64, f3(v0, Str::lit("\""))?))?;
            h303
        };
        h302
    }, Str::lit("\""))
}

pub fn f80(v0: Adt16, v1: u64) -> R<Adt16> {
    let a305 = v1;
    let a306 = 39u64;
    if nat_eq(a305, a306) {
        f3(v0, Str::lit("'\\''"))
    } else {
        let a307 = v1;
        let a308 = 92u64;
        if nat_eq(a307, a308) {
            f3(v0, Str::lit("'\\\\'"))
        } else {
            let a309 = v1;
            let a310 = 10u64;
            if nat_eq(a309, a310) {
                f3(v0, Str::lit("'\\n'"))
            } else {
                let a311 = v1;
                let a312 = 13u64;
                if nat_eq(a311, a312) {
                    f3(v0, Str::lit("'\\r'"))
                } else {
                    let a313 = v1;
                    let a314 = 9u64;
                    if nat_eq(a313, a314) {
                        f3(v0, Str::lit("'\\t'"))
                    } else {
                        let a315 = 32u64;
                        let a316 = v1;
                        if if nat_le(a315, a316) {
                            let a317 = v1;
                            let a318 = 126u64;
                            nat_le(a317, a318)
                        } else {
                            false
                        } {
                            f3(f39(f3(v0, Str::lit("'"))?, f105(v1))?, Str::lit("'"))
                        } else {
                            f3(f97(f3(v0, Str::lit("'\\u"))?, Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), v1)?, Str::lit("'"))
                        }
                    }
                }
            }
        }
    }
}

pub fn f81(p324: Adt16, v1: List<Bytes>, p325: List<Bytes>, p326: Adt6) -> R<Adt16> {
    let m323 = (p324, (p325, p326));
    {
        let mut m323 = m323;
        loop {
            let step = {
                let (v0, (v2, v3)) = m323;
                let m319 = v2;
                match m319.uncons() {
                    None => {
                        Err::<(Adt16, (List<Bytes>, Adt6)), R<Adt16>>(Ok(v0))
                    }
                    Some((v4, v5)) => {
                        let m320 = f106(v3);
                        match m320 {
                            None => {
                                Err::<(Adt16, (List<Bytes>, Adt6)), R<Adt16>>(Ok(v0))
                            }
                            Some(v6) => {
                                Ok::<(Adt16, (List<Bytes>, Adt6)), R<Adt16>>((f3(f34(f3(f48(f3(v0, Str::lit(" ("))?, v1.clone(), v4)?, Str::lit(" := "))?, v1.clone(), {
                                    let (h321, _) = v6;
                                    h321
                                }, 0u64, true)?, Str::lit(")"))?, (v5, {
                                    let (_, h322) = v6;
                                    h322
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: List<Adt0>, v1: Bytes) -> List<Bytes> {
    f107(Fn18::F108(v1), List::<Bytes>::nil(), v0)
}

pub fn f83(v0: List<Adt15>, v1: Bytes) -> List<Bytes> {
    let m327 = f75(v0, v1);
    match m327 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f84(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f109(v0.clone())?;
    let a328 = 0u64;
    let a329 = v1;
    if nat_lt(a328, a329) {
        Ok(f94(v0, 0u64, {
            let a330 = v1;
            let a331 = 1u64;
            nat_sub(a330, a331)
        }))
    } else {
        Ok(Bytes::lit(&[]))
    }
}

pub fn f85(v0: Adt16, v1: List<Bytes>, v2: List<Bytes>) -> R<Adt16> {
    f77(Fn12::F110(v1), v0, v2)
}

pub fn f86(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f109(v0.clone())?;
    let a332 = 0u64;
    let a333 = v1;
    if nat_lt(a332, a333) {
        Ok(f94(v0.clone(), v1, {
            let a335 = {
                let a334 = v0;
                length_bytes(a334)
            };
            let a336 = v1;
            nat_sub(a335, a336)
        }))
    } else {
        Ok(v0)
    }
}

pub fn f87(v0: Fn13, p340: Adt16, p341: List<Adt12>) -> R<Adt16> {
    let m339 = (p340, p341);
    {
        let mut m339 = m339;
        loop {
            let step = {
                let (v1, v2) = m339;
                let m337 = v2;
                match m337.uncons() {
                    None => {
                        Err::<(Adt16, List<Adt12>), R<Adt16>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt16, List<Adt12>), R<Adt16>>(({
                            let c338: Fn13 = v0.clone();
                            c338.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt16, v1: Adt12) -> R<Adt16> {
    f3({
        let m342 = v1;
        match m342 {
            Adt12::C0(v2) => {
                f79(v0, v2)?
            }
            Adt12::C1 => {
                f3(v0, Str::lit("ident"))?
            }
            Adt12::C2 => {
                f3(v0, Str::lit("term"))?
            }
        }
    }, Str::lit(" "))
}

pub fn f89(v0: Adt14, v1: (List<Adt13>, Adt16), v2: Adt12) -> R<(List<Adt13>, Adt16)> {
    let m343 = v2;
    match m343 {
        Adt12::C0(v3) => {
            Ok(({
                let (h344, _) = v1;
                h344
            }, f39({
                let (_, h345) = v1;
                h345
            }, v3)?))
        }
        Adt12::C1 => {
            f111(v0, v1)
        }
        Adt12::C2 => {
            f111(v0, v1)
        }
    }
}

pub fn f90(v0: List<Adt1>, v1: Bytes) -> Option<List<Adt12>> {
    f112(Fn19::F113(v1), None::<List<Adt12>>, v0)
}

pub fn f91(v0: Adt16, v1: List<Bytes>, v2: Bytes) -> R<Adt16> {
    if f74(v1, v2.clone()) {
        f39(f39(f39(v0, Bytes::lit(&[0xc2, 0xab]))?, v2)?, Bytes::lit(&[0xc2, 0xbb]))
    } else {
        f39(v0, v2)
    }
}

pub fn f92(v0: Fn14, p350: u64, p351: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m349 = (p350, p351);
    {
        let mut m349 = m349;
        loop {
            let step = {
                let (v1, v2) = m349;
                let m346 = v1;
                if m346 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m346 - 1;
                    let m348 = {
                        let c347: Fn14 = v0.clone();
                        c347.apply(v2.clone())?
                    };
                    match m348 {
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
                Ok(next) => { m349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f114(v1, v0, v2)
}

pub fn f94(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m355 = {
        let a352 = v0;
        let a353 = v1;
        let a354 = v2;
        slice_bytes(a352, a353, a354)
    };
    match m355 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f95(p359: Adt16, v1: List<Bytes>, p360: Adt5, p361: bool) -> R<Adt16> {
    let m358 = (p359, (p360, p361));
    {
        let mut m358 = m358;
        loop {
            let step = {
                let (v0, (v2, v3)) = m358;
                let m356 = v2;
                match m356 {
                    Adt5::C0 => {
                        Err::<(Adt16, (Adt5, bool)), R<Adt16>>(Ok(v0))
                    }
                    Adt5::C1(v4, v5, r357) => {
                        let v6 = (*r357).clone();
                        Ok::<(Adt16, (Adt5, bool)), R<Adt16>>((f3(f34(f3(f39(f3(if v3 {
                            v0
                        } else {
                            f3(v0, Str::lit(" "))?
                        }, Str::lit("("))?, v4)?, Str::lit(" : "))?, v1.clone(), v5, 0u64, true)?, Str::lit(")"))?, (v6, false)))
                    }
                }
            };
            match step {
                Ok(next) => { m358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m362 = f115(v0.clone(), v1);
    match m362 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a407 = {
                    let a383 = {
                        let a371 = {
                            let a365 = {
                                let a363 = v3;
                                let a364 = 128u8;
                                fixed_u8::bit_and(a363, a364)
                            };
                            let a366 = 128u8;
                            if equal(a365, a366) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a372 = {
                            let a369 = {
                                let a367 = v3;
                                let a368 = 64u8;
                                fixed_u8::bit_and(a367, a368)
                            };
                            let a370 = 64u8;
                            if equal(a369, a370) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a371, a372)?
                    };
                    let a384 = {
                        let a381 = {
                            let a375 = {
                                let a373 = v3;
                                let a374 = 32u8;
                                fixed_u8::bit_and(a373, a374)
                            };
                            let a376 = 32u8;
                            if equal(a375, a376) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a382 = {
                            let a379 = {
                                let a377 = v3;
                                let a378 = 16u8;
                                fixed_u8::bit_and(a377, a378)
                            };
                            let a380 = 16u8;
                            if equal(a379, a380) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a381, a382)?
                    };
                    nat_add(a383, a384)?
                };
                let a408 = {
                    let a405 = {
                        let a393 = {
                            let a387 = {
                                let a385 = v3;
                                let a386 = 8u8;
                                fixed_u8::bit_and(a385, a386)
                            };
                            let a388 = 8u8;
                            if equal(a387, a388) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a394 = {
                            let a391 = {
                                let a389 = v3;
                                let a390 = 4u8;
                                fixed_u8::bit_and(a389, a390)
                            };
                            let a392 = 4u8;
                            if equal(a391, a392) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a393, a394)?
                    };
                    let a406 = {
                        let a403 = {
                            let a397 = {
                                let a395 = v3;
                                let a396 = 2u8;
                                fixed_u8::bit_and(a395, a396)
                            };
                            let a398 = 2u8;
                            if equal(a397, a398) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a404 = {
                            let a401 = {
                                let a399 = v3;
                                let a400 = 1u8;
                                fixed_u8::bit_and(a399, a400)
                            };
                            let a402 = 1u8;
                            if equal(a401, a402) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a403, a404)?
                    };
                    nat_add(a405, a406)?
                };
                nat_add(a407, a408)?
            };
            let a409 = v4;
            let a410 = 128u64;
            if nat_lt(a409, a410) {
                Ok((v4, 1u64))
            } else {
                let a411 = v4;
                let a412 = 224u64;
                if nat_lt(a411, a412) {
                    Ok(({
                        let a420 = {
                            let a416 = {
                                let a413 = v4;
                                let a414 = 32u64;
                                let a415 = 0u64;
                                nat_rem(a413, a414, a415)
                            };
                            let a417 = 64u64;
                            nat_mul(a416, a417)?
                        };
                        let a421 = f116(v0, {
                            let a418 = v1;
                            let a419 = 1u64;
                            nat_add(a418, a419)?
                        })?;
                        nat_add(a420, a421)?
                    }, 2u64))
                } else {
                    let a422 = v4;
                    let a423 = 240u64;
                    if nat_lt(a422, a423) {
                        Ok(({
                            let a437 = {
                                let a427 = {
                                    let a424 = v4;
                                    let a425 = 16u64;
                                    let a426 = 0u64;
                                    nat_rem(a424, a425, a426)
                                };
                                let a428 = 4096u64;
                                nat_mul(a427, a428)?
                            };
                            let a438 = {
                                let a435 = {
                                    let a431 = f116(v0.clone(), {
                                        let a429 = v1;
                                        let a430 = 1u64;
                                        nat_add(a429, a430)?
                                    })?;
                                    let a432 = 64u64;
                                    nat_mul(a431, a432)?
                                };
                                let a436 = f116(v0, {
                                    let a433 = v1;
                                    let a434 = 2u64;
                                    nat_add(a433, a434)?
                                })?;
                                nat_add(a435, a436)?
                            };
                            nat_add(a437, a438)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a458 = {
                                let a442 = {
                                    let a439 = v4;
                                    let a440 = 8u64;
                                    let a441 = 0u64;
                                    nat_rem(a439, a440, a441)
                                };
                                let a443 = 262144u64;
                                nat_mul(a442, a443)?
                            };
                            let a459 = {
                                let a456 = {
                                    let a446 = f116(v0.clone(), {
                                        let a444 = v1;
                                        let a445 = 1u64;
                                        nat_add(a444, a445)?
                                    })?;
                                    let a447 = 4096u64;
                                    nat_mul(a446, a447)?
                                };
                                let a457 = {
                                    let a454 = {
                                        let a450 = f116(v0.clone(), {
                                            let a448 = v1;
                                            let a449 = 2u64;
                                            nat_add(a448, a449)?
                                        })?;
                                        let a451 = 64u64;
                                        nat_mul(a450, a451)?
                                    };
                                    let a455 = f116(v0, {
                                        let a452 = v1;
                                        let a453 = 3u64;
                                        nat_add(a452, a453)?
                                    })?;
                                    nat_add(a454, a455)?
                                };
                                nat_add(a456, a457)?
                            };
                            nat_add(a458, a459)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f97(v0: Adt16, v1: Bytes, v2: u64) -> R<Adt16> {
    let v3: Bytes = {
        let a472 = f117(v1.clone(), v2, 268435456u64);
        let a473 = {
            let a470 = f117(v1.clone(), v2, 16777216u64);
            let a471 = {
                let a468 = f117(v1.clone(), v2, 1048576u64);
                let a469 = {
                    let a466 = f117(v1.clone(), v2, 65536u64);
                    let a467 = {
                        let a464 = f117(v1.clone(), v2, 4096u64);
                        let a465 = {
                            let a462 = f117(v1.clone(), v2, 256u64);
                            let a463 = {
                                let a460 = f117(v1.clone(), v2, 16u64);
                                let a461 = f117(v1, v2, 1u64);
                                append_bytes(a460, a461)
                            };
                            append_bytes(a462, a463)
                        };
                        append_bytes(a464, a465)
                    };
                    append_bytes(a466, a467)
                };
                append_bytes(a468, a469)
            };
            append_bytes(a470, a471)
        };
        append_bytes(a472, a473)
    };
    let v4: u64 = {
        let (h474, _) = f102(Fn17::F118(v3.clone()), 4u64, 0u64)?;
        h474
    };
    f11(v0, f94(v3, v4, {
        let a475 = 8u64;
        let a476 = v4;
        nat_sub(a475, a476)
    }))
}

pub fn f98(v0: Fn15, p480: bool, p481: List<Bytes>) -> bool {
    let m479 = (p480, p481);
    {
        let mut m479 = m479;
        loop {
            let step = {
                let (v1, v2) = m479;
                let m477 = v2;
                match m477.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c478: Fn15 = v0.clone();
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

pub fn f99(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a482 = v2;
        let a483 = v0;
        equal(a482, a483)
    }
}

pub fn f100(v0: Fn16, p487: Option<List<Bytes>>, p488: List<Adt15>) -> Option<List<Bytes>> {
    let m486 = (p487, p488);
    {
        let mut m486 = m486;
        loop {
            let step = {
                let (v1, v2) = m486;
                let m484 = v2;
                match m484.uncons() {
                    None => {
                        Err::<(Option<List<Bytes>>, List<Adt15>), Option<List<Bytes>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Bytes>>, List<Adt15>), Option<List<Bytes>>>(({
                            let c485: Fn16 = v0.clone();
                            c485.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Bytes, v1: Option<List<Bytes>>, v2: Adt15) -> Option<List<Bytes>> {
    let m489 = v1.clone();
    match m489 {
        None => {
            let a491 = match v2.clone() {
                Adt15::C0(h490, _) => {
                    h490
                }
            };
            let a492 = v0;
            if equal(a491, a492) {
                Some(match v2 {
                    Adt15::C0(_, h493) => {
                        h493
                    }
                })
            } else {
                v1
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f102(v0: Fn17, p498: u64, p499: u64) -> R<(u64, bool)> {
    let m497 = (p498, p499);
    {
        let mut m497 = m497;
        loop {
            let step = {
                let (v1, v2) = m497;
                let m494 = v1;
                if m494 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m494 - 1;
                    let m496 = {
                        let c495: Fn17 = v0.clone();
                        c495.apply(v2)?
                    };
                    match m496 {
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
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f119(v0, v1)
}

pub fn f104(v0: Bytes, v1: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
    f120(v0, v1)
}

pub fn f105(v0: u64) -> Bytes {
    f94(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f]), v0, 1u64)
}

pub fn f106(v0: Adt6) -> Option<(Adt6, Adt6)> {
    let m500 = v0;
    match m500 {
        Adt6::C0(_, _) => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C1(_, _) => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C2(_, _) => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C3(_) => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C4 => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C5(r501, r502) => {
            let v8 = (*r501).clone();
            let v9 = (*r502).clone();
            Some((v8, v9))
        }
    }
}

pub fn f107(v0: Fn18, p506: List<Bytes>, p507: List<Adt0>) -> List<Bytes> {
    let m505 = (p506, p507);
    {
        let mut m505 = m505;
        loop {
            let step = {
                let (v1, v2) = m505;
                let m503 = v2;
                match m503.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt0>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt0>), List<Bytes>>(({
                            let c504: Fn18 = v0.clone();
                            c504.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m505 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: Bytes, v1: List<Bytes>, v2: Adt0) -> List<Bytes> {
    let a509 = match v2.clone() {
        Adt0::C0(h508, _) => {
            h508
        }
    };
    let a510 = v0;
    if equal(a509, a510) {
        match v2 {
            Adt0::C0(_, h511) => {
                h511
            }
        }
    } else {
        v1
    }
}

pub fn f109(v0: Bytes) -> R<u64> {
    let (h512, _) = f102(Fn17::F121(v0.clone()), {
        let a513 = v0.clone();
        length_bytes(a513)
    }, {
        let a514 = v0;
        length_bytes(a514)
    })?;
    Ok(h512)
}

pub fn f110(v0: List<Bytes>, v1: Adt16, v2: Bytes) -> R<Adt16> {
    f48(f3(v1, Str::lit(" "))?, v0, v2)
}

pub fn f111(v0: Adt14, v1: (List<Adt13>, Adt16)) -> R<(List<Adt13>, Adt16)> {
    let m516 = {
        let (h515, _) = v1.clone();
        h515
    };
    match m516.uncons() {
        None => {
            Ok(v1)
        }
        Some((v2, v3)) => {
            Ok((v3, {
                let m517 = v2;
                match m517 {
                    Adt13::C0(v4) => {
                        f48({
                            let (_, h518) = v1;
                            h518
                        }, match v0 {
                            Adt14::C0(h519, _, _, _, _) => {
                                h519
                            }
                        }, v4)?
                    }
                    Adt13::C1(v5) => {
                        f38({
                            let (_, h520) = v1;
                            h520
                        }, v0, v5, true)?
                    }
                }
            }))
        }
    }
}

pub fn f112(v0: Fn19, p524: Option<List<Adt12>>, p525: List<Adt1>) -> Option<List<Adt12>> {
    let m523 = (p524, p525);
    {
        let mut m523 = m523;
        loop {
            let step = {
                let (v1, v2) = m523;
                let m521 = v2;
                match m521.uncons() {
                    None => {
                        Err::<(Option<List<Adt12>>, List<Adt1>), Option<List<Adt12>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Adt12>>, List<Adt1>), Option<List<Adt12>>>(({
                            let c522: Fn19 = v0.clone();
                            c522.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Bytes, v1: Option<List<Adt12>>, v2: Adt1) -> Option<List<Adt12>> {
    let m526 = v1.clone();
    match m526 {
        None => {
            let m527 = v2;
            match m527 {
                Adt1::C0(_, _, _, _, _, _) => {
                    v1
                }
                Adt1::C1(_, _, _, _) => {
                    v1
                }
                Adt1::C2(_) => {
                    v1
                }
                Adt1::C3(_, _, _, _, _, _, _, _) => {
                    v1
                }
                Adt1::C4(v23, v24, _) => {
                    let a528 = v23;
                    let a529 = v0;
                    if equal(a528, a529) {
                        Some(v24)
                    } else {
                        v1
                    }
                }
                Adt1::C5(_, _) => {
                    v1
                }
                Adt1::C6(_, _) => {
                    v1
                }
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f114(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h530, _) = v2.clone();
        h530
    };
    let v4: u64 = {
        let (h531, _) = {
            let (_, h532) = v2.clone();
            h532
        };
        h531
    };
    let v5: List<Bytes> = {
        let (_, h533) = {
            let (_, h534) = v2;
            h534
        };
        h533
    };
    let m535 = f115(v0.clone(), v3);
    match m535 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a536 = v6;
            let a537 = v1;
            if equal(a536, a537) {
                Ok(Some(({
                    let a538 = v3;
                    let a539 = 1u64;
                    nat_add(a538, a539)?
                }, ({
                    let a540 = v3;
                    let a541 = 1u64;
                    nat_add(a540, a541)?
                }, List::cons(f94(v0, v4, {
                    let a542 = v3;
                    let a543 = v4;
                    nat_sub(a542, a543)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a544 = v3;
                    let a545 = 1u64;
                    nat_add(a544, a545)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f115(v0: Bytes, v1: u64) -> Option<u8> {
    let a546 = v0;
    let a547 = v1;
    index_bytes(a546, a547)
}

pub fn f116(v0: Bytes, v1: u64) -> R<u64> {
    let m548 = f115(v0, v1);
    match m548 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a595 = {
                let v3: u8 = v2;
                let a593 = {
                    let a569 = {
                        let a557 = {
                            let a551 = {
                                let a549 = v3;
                                let a550 = 128u8;
                                fixed_u8::bit_and(a549, a550)
                            };
                            let a552 = 128u8;
                            if equal(a551, a552) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a558 = {
                            let a555 = {
                                let a553 = v3;
                                let a554 = 64u8;
                                fixed_u8::bit_and(a553, a554)
                            };
                            let a556 = 64u8;
                            if equal(a555, a556) {
                                64u64
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
                                let a560 = 32u8;
                                fixed_u8::bit_and(a559, a560)
                            };
                            let a562 = 32u8;
                            if equal(a561, a562) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a568 = {
                            let a565 = {
                                let a563 = v3;
                                let a564 = 16u8;
                                fixed_u8::bit_and(a563, a564)
                            };
                            let a566 = 16u8;
                            if equal(a565, a566) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a567, a568)?
                    };
                    nat_add(a569, a570)?
                };
                let a594 = {
                    let a591 = {
                        let a579 = {
                            let a573 = {
                                let a571 = v3;
                                let a572 = 8u8;
                                fixed_u8::bit_and(a571, a572)
                            };
                            let a574 = 8u8;
                            if equal(a573, a574) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a580 = {
                            let a577 = {
                                let a575 = v3;
                                let a576 = 4u8;
                                fixed_u8::bit_and(a575, a576)
                            };
                            let a578 = 4u8;
                            if equal(a577, a578) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a579, a580)?
                    };
                    let a592 = {
                        let a589 = {
                            let a583 = {
                                let a581 = v3;
                                let a582 = 2u8;
                                fixed_u8::bit_and(a581, a582)
                            };
                            let a584 = 2u8;
                            if equal(a583, a584) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a590 = {
                            let a587 = {
                                let a585 = v3;
                                let a586 = 1u8;
                                fixed_u8::bit_and(a585, a586)
                            };
                            let a588 = 1u8;
                            if equal(a587, a588) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a589, a590)?
                    };
                    nat_add(a591, a592)?
                };
                nat_add(a593, a594)?
            };
            let a596 = 64u64;
            let a597 = 0u64;
            Ok(nat_rem(a595, a596, a597))
        }
    }
}

pub fn f117(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f94(v0, {
        let a601 = {
            let a598 = v1;
            let a599 = v2;
            let a600 = 0u64;
            nat_quot(a598, a599, a600)
        };
        let a602 = 16u64;
        let a603 = 0u64;
        nat_rem(a601, a602, a603)
    }, 1u64)
}

pub fn f118(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f122(v0, v1)
}

pub fn f119(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a604 = 0u64;
    let a605 = v1;
    if nat_lt(a604, a605) {
        let v2: u64 = f123(v0, v1)?;
        let a606 = 0u64;
        let a607 = v2;
        if nat_lt(a606, a607) {
            Ok(Some({
                let a608 = v1;
                let a609 = v2;
                nat_sub(a608, a609)
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f120(v0: Bytes, v1: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
    let a612 = {
        let (h610, _) = v1.clone();
        h610
    };
    let a613 = {
        let a611 = v0.clone();
        length_bytes(a611)
    };
    if nat_lt(a612, a613) {
        let v2: (u64, u64) = f96(v0.clone(), {
            let (h614, _) = v1.clone();
            h614
        })?;
        let v3: u64 = {
            let (h615, _) = v2;
            h615
        };
        Ok(Some(({
            let a618 = {
                let (h616, _) = v1.clone();
                h616
            };
            let a619 = {
                let (_, h617) = v2;
                h617
            };
            nat_add(a618, a619)?
        }, {
            let a620 = v3;
            let a621 = 92u64;
            if nat_eq(a620, a621) {
                f3({
                    let (_, h622) = v1;
                    h622
                }, Str::lit("\\\\"))?
            } else {
                let a623 = v3;
                let a624 = 34u64;
                if nat_eq(a623, a624) {
                    f3({
                        let (_, h625) = v1;
                        h625
                    }, Str::lit("\\\""))?
                } else {
                    let a626 = v3;
                    let a627 = 10u64;
                    if nat_eq(a626, a627) {
                        f3({
                            let (_, h628) = v1;
                            h628
                        }, Str::lit("\\n"))?
                    } else {
                        let a629 = v3;
                        let a630 = 13u64;
                        if nat_eq(a629, a630) {
                            f3({
                                let (_, h631) = v1;
                                h631
                            }, Str::lit("\\r"))?
                        } else {
                            let a632 = v3;
                            let a633 = 9u64;
                            if nat_eq(a632, a633) {
                                f3({
                                    let (_, h634) = v1;
                                    h634
                                }, Str::lit("\\t"))?
                            } else {
                                let a635 = v3;
                                let a636 = 32u64;
                                if if nat_lt(a635, a636) {
                                    true
                                } else {
                                    let a637 = 127u64;
                                    let a638 = v3;
                                    if nat_le(a637, a638) {
                                        let a639 = v3;
                                        let a640 = 159u64;
                                        nat_le(a639, a640)
                                    } else {
                                        false
                                    }
                                } {
                                    f97(f3({
                                        let (_, h641) = v1;
                                        h641
                                    }, Str::lit("\\u"))?, Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), v3)?
                                } else {
                                    f39({
                                        let (_, h642) = v1;
                                        h642
                                    }, f94(v0, {
                                        let (h643, _) = v1;
                                        h643
                                    }, {
                                        let (_, h644) = v2;
                                        h644
                                    }))?
                                }
                            }
                        }
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt16)>)
    }
}

pub fn f121(v0: Bytes, v1: u64) -> Option<u64> {
    f124(v0, v1)
}

pub fn f122(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m645 = f115(v0, v1);
    match m645 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a646 = v2;
            let a647 = 48u8;
            if equal(a646, a647) {
                Ok(Some({
                    let a648 = v1;
                    let a649 = 1u64;
                    nat_add(a648, a649)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f123(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f125(v0.clone(), {
        let a650 = v1;
        let a651 = 1u64;
        nat_sub(a650, a651)
    })?;
    let v3: u64 = f125(v0.clone(), {
        let a652 = v1;
        let a653 = 2u64;
        nat_sub(a652, a653)
    })?;
    let v4: u64 = f125(v0, {
        let a654 = v1;
        let a655 = 3u64;
        nat_sub(a654, a655)
    })?;
    let a656 = v2;
    let a657 = 32u64;
    if if nat_eq(a656, a657) {
        true
    } else {
        let a658 = 9u64;
        let a659 = v2;
        if nat_le(a658, a659) {
            let a660 = v2;
            let a661 = 13u64;
            nat_le(a660, a661)
        } else {
            false
        }
    } {
        Ok(1u64)
    } else {
        let a662 = 2u64;
        let a663 = v1;
        if if nat_le(a662, a663) {
            let a664 = v3;
            let a665 = 194u64;
            if nat_eq(a664, a665) {
                let a666 = v2;
                let a667 = 133u64;
                if nat_eq(a666, a667) {
                    true
                } else {
                    let a668 = v2;
                    let a669 = 160u64;
                    nat_eq(a668, a669)
                }
            } else {
                false
            }
        } else {
            false
        } {
            Ok(2u64)
        } else {
            let a670 = 3u64;
            let a671 = v1;
            if if nat_le(a670, a671) {
                let a672 = v4;
                let a673 = 225u64;
                if if nat_eq(a672, a673) {
                    let a674 = v3;
                    let a675 = 154u64;
                    if nat_eq(a674, a675) {
                        let a676 = v2;
                        let a677 = 128u64;
                        nat_eq(a676, a677)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    true
                } else {
                    let a678 = v4;
                    let a679 = 226u64;
                    if if nat_eq(a678, a679) {
                        let a680 = v3;
                        let a681 = 128u64;
                        if nat_eq(a680, a681) {
                            let a682 = 128u64;
                            let a683 = v2;
                            if if nat_le(a682, a683) {
                                let a684 = v2;
                                let a685 = 138u64;
                                nat_le(a684, a685)
                            } else {
                                false
                            } {
                                true
                            } else {
                                let a686 = v2;
                                let a687 = 168u64;
                                if nat_eq(a686, a687) {
                                    true
                                } else {
                                    let a688 = v2;
                                    let a689 = 169u64;
                                    if nat_eq(a688, a689) {
                                        true
                                    } else {
                                        let a690 = v2;
                                        let a691 = 175u64;
                                        nat_eq(a690, a691)
                                    }
                                }
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a692 = v4;
                        let a693 = 226u64;
                        if if nat_eq(a692, a693) {
                            let a694 = v3;
                            let a695 = 129u64;
                            if nat_eq(a694, a695) {
                                let a696 = v2;
                                let a697 = 159u64;
                                nat_eq(a696, a697)
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            true
                        } else {
                            let a698 = v4;
                            let a699 = 227u64;
                            if nat_eq(a698, a699) {
                                let a700 = v3;
                                let a701 = 128u64;
                                if nat_eq(a700, a701) {
                                    let a702 = v2;
                                    let a703 = 128u64;
                                    nat_eq(a702, a703)
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                    }
                }
            } else {
                false
            } {
                Ok(3u64)
            } else {
                Ok(0u64)
            }
        }
    }
}

pub fn f124(v0: Bytes, v1: u64) -> Option<u64> {
    let a704 = 0u64;
    let a705 = v1;
    if nat_lt(a704, a705) {
        let m708 = f115(v0, {
            let a706 = v1;
            let a707 = 1u64;
            nat_sub(a706, a707)
        });
        match m708 {
            None => {
                None::<u64>
            }
            Some(v2) => {
                let a709 = v2;
                let a710 = 46u8;
                if equal(a709, a710) {
                    None::<u64>
                } else {
                    Some({
                        let a711 = v1;
                        let a712 = 1u64;
                        nat_sub(a711, a712)
                    })
                }
            }
        }
    } else {
        None::<u64>
    }
}

pub fn f125(v0: Bytes, v1: u64) -> R<u64> {
    let m713 = f115(v0, v1);
    match m713 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a758 = {
                let a734 = {
                    let a722 = {
                        let a716 = {
                            let a714 = v3;
                            let a715 = 128u8;
                            fixed_u8::bit_and(a714, a715)
                        };
                        let a717 = 128u8;
                        if equal(a716, a717) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a723 = {
                        let a720 = {
                            let a718 = v3;
                            let a719 = 64u8;
                            fixed_u8::bit_and(a718, a719)
                        };
                        let a721 = 64u8;
                        if equal(a720, a721) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a722, a723)?
                };
                let a735 = {
                    let a732 = {
                        let a726 = {
                            let a724 = v3;
                            let a725 = 32u8;
                            fixed_u8::bit_and(a724, a725)
                        };
                        let a727 = 32u8;
                        if equal(a726, a727) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a733 = {
                        let a730 = {
                            let a728 = v3;
                            let a729 = 16u8;
                            fixed_u8::bit_and(a728, a729)
                        };
                        let a731 = 16u8;
                        if equal(a730, a731) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a732, a733)?
                };
                nat_add(a734, a735)?
            };
            let a759 = {
                let a756 = {
                    let a744 = {
                        let a738 = {
                            let a736 = v3;
                            let a737 = 8u8;
                            fixed_u8::bit_and(a736, a737)
                        };
                        let a739 = 8u8;
                        if equal(a738, a739) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a745 = {
                        let a742 = {
                            let a740 = v3;
                            let a741 = 4u8;
                            fixed_u8::bit_and(a740, a741)
                        };
                        let a743 = 4u8;
                        if equal(a742, a743) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a744, a745)?
                };
                let a757 = {
                    let a754 = {
                        let a748 = {
                            let a746 = v3;
                            let a747 = 2u8;
                            fixed_u8::bit_and(a746, a747)
                        };
                        let a749 = 2u8;
                        if equal(a748, a749) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a755 = {
                        let a752 = {
                            let a750 = v3;
                            let a751 = 1u8;
                            fixed_u8::bit_and(a750, a751)
                        };
                        let a753 = 1u8;
                        if equal(a752, a753) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a754, a755)?
                };
                nat_add(a756, a757)?
            };
            nat_add(a758, a759)
        }
    }
}
