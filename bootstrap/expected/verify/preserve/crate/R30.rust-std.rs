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
    C0(List<Adt1>, u64, bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2, u64, bool, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Adt3>),
    C1,
    C2,
    C3(Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, bool),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, bool, u64, List<Adt1>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Adt3>, bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F4(List<Bytes>),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn0::F4(k0) => f4(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F6(u64, Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn1::F6(k0, k1) => f6(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F9,
}

impl Fn2 {
    pub fn apply(&self, p0: u64, p1: Adt1) -> R<u64> {
        match self {
            Fn2::F9 => f9(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F13,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn3::F13 => f13(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F33,
}

impl Fn4 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt3) -> Option<Bytes> {
        match self {
            Fn4::F33 => f33(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F36,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt3) -> List<Bytes> {
        match self {
            Fn5::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F39(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn6::F39(k0) => f39(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F45(u64, Bytes),
    F77(Bytes),
    F88(Bytes),
    F94(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F45(k0, k1) => f45(*k0, k1.clone(), p0),
            Fn7::F77(k0) => f77(k0.clone(), p0),
            Fn7::F88(k0) => f88(k0.clone(), p0),
            Fn7::F94(k0) => f94(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F47(Bytes, u64),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn8::F47(k0, k1) => f47(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F49(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn9::F49(k0, k1) => f49(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F63,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn10::F63 => f63(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F65(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn11::F65(k0) => f65(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F70(u64, Bytes),
    F95(u64, u64, Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn12::F70(k0, k1) => f70(*k0, k1.clone(), p0),
            Fn12::F95(k0, k1, k2) => f95(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F85(u64, Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
        match self {
            Fn13::F85(k0, k1) => f85(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F87(u64, u64, Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
        match self {
            Fn14::F87(k0, k1, k2) => f87(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F92(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn15::F92(k0) => f92(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F103,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn16::F103 => f103(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F111(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn17::F111(k0) => f111(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F113(u64, u64, Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn18::F113(k0, k1, k2) => f113(*k0, *k1, k2.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Bytes>) -> R<Result<(), Bytes>> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v3) => {
            let v4: u64 = f2(v3.clone())?;
            let v5: Adt0 = {
                let (h2, _) = f3(Fn0::F4(v1), {
                    let a3 = v4;
                    let a4 = 1u64;
                    nat_add(a3, a4)?
                }, Adt0::C0(v3, 0u64, false, None::<Bytes>))?;
                h2
            };
            let m6 = match v5 {
                Adt0::C0(_, _, _, h5) => {
                    h5
                }
            };
            match m6 {
                None => {
                    Ok(Ok::<(), Bytes>(()))
                }
                Some(v6) => {
                    Ok(Err::<(), Bytes>(v6))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(), Bytes>(v2))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt1>, Bytes>> {
    let v1: u64 = {
        let a7 = v0.clone();
        length_bytes(a7)
    };
    let v2: Adt4 = {
        let (h8, _) = f5(Fn1::F6(v1, v0), {
            let a9 = v1;
            let a10 = 1u64;
            nat_add(a9, a10)?
        }, Adt4::C0(0u64, 0u64, true, 0u64, List::<Adt1>::nil(), None::<Bytes>))?;
        h8
    };
    let m12 = match v2.clone() {
        Adt4::C0(_, _, _, _, _, h11) => {
            h11
        }
    };
    match m12 {
        None => {
            let v4: List<Adt1> = f7(match v2 {
                Adt4::C0(_, _, _, _, h13, _) => {
                    h13
                }
            });
            Ok(Ok::<List<Adt1>, Bytes>(v4))
        }
        Some(v3) => {
            Ok(Err::<List<Adt1>, Bytes>(v3))
        }
    }
}

pub fn f2(v0: List<Adt1>) -> R<u64> {
    f8(Fn2::F9, 0u64, v0)
}

pub fn f3(v0: Fn0, p18: u64, p19: Adt0) -> R<(Adt0, bool)> {
    let m17 = (p18, p19);
    {
        let mut m17 = m17;
        loop {
            let step = {
                let (v1, v2) = m17;
                let m14 = v1;
                if m14 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m14 - 1;
                    let m16 = {
                        let c15: Fn0 = v0.clone();
                        c15.apply(v2.clone())?
                    };
                    match m16 {
                        None => {
                            Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt0), R<(Adt0, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m17 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: List<Bytes>, v1: Adt0) -> R<Option<Adt0>> {
    f10(v0, v1)
}

pub fn f5(v0: Fn1, p24: u64, p25: Adt4) -> R<(Adt4, bool)> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m20 = v1;
                if m20 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m20 - 1;
                    let m22 = {
                        let c21: Fn1 = v0.clone();
                        c21.apply(v2.clone())?
                    };
                    match m22 {
                        None => {
                            Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), R<(Adt4, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f11(v1, v0, v2)
}

pub fn f7(v0: List<Adt1>) -> List<Adt1> {
    f12(Fn3::F13, List::<Adt1>::nil(), v0)
}

pub fn f8(v0: Fn2, p29: u64, p30: List<Adt1>) -> R<u64> {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v1, v2) = m28;
                let m26 = v2;
                match m26.uncons() {
                    None => {
                        Err::<(u64, List<Adt1>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt1>), R<u64>>(({
                            let c27: Fn2 = v0.clone();
                            c27.apply(v1, v3)?
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

pub fn f9(v0: u64, _: Adt1) -> R<u64> {
    let a31 = v0;
    let a32 = 1u64;
    nat_add(a31, a32)
}

pub fn f10(v0: List<Bytes>, v1: Adt0) -> R<Option<Adt0>> {
    let v2: u64 = match v1.clone() {
        Adt0::C0(_, h33, _, _) => {
            h33
        }
    };
    let v3: bool = match v1.clone() {
        Adt0::C0(_, _, h34, _) => {
            h34
        }
    };
    let m36 = match v1.clone() {
        Adt0::C0(_, _, _, h35) => {
            h35
        }
    };
    match m36 {
        None => {
            let m38 = match v1 {
                Adt0::C0(h37, _, _, _) => {
                    h37
                }
            };
            match m38.uncons() {
                None => {
                    Ok(None::<Adt0>)
                }
                Some((v5, v6)) => {
                    let m40 = match v5 {
                        Adt1::C0(h39, _, _, _) => {
                            h39
                        }
                    };
                    match m40 {
                        Adt2::C0(v7) => {
                            let v8: Adt0 = f14(v7, v6, v2, v3, v0);
                            Ok(Some(v8))
                        }
                        Adt2::C1 => {
                            Ok(Some(Adt0::C0(v6, v2, v3, None::<Bytes>)))
                        }
                        Adt2::C2 => {
                            Ok(Some(Adt0::C0(v6, v2, v3, None::<Bytes>)))
                        }
                        Adt2::C3(v9) => {
                            let v10: Bytes = {
                                let a45 = {
                                    let a42 = {
                                        let a41 = Str::lit("the command `");
                                        utf8_encode(a41)
                                    };
                                    let a43 = v9;
                                    append_bytes(a42, a43)
                                };
                                let a46 = {
                                    let a44 = Str::lit("`");
                                    utf8_encode(a44)
                                };
                                append_bytes(a45, a46)
                            };
                            Ok(Some(Adt0::C0(v6, v2, v3, Some(v10))))
                        }
                        Adt2::C4(v11) => {
                            let a48 = v11.clone();
                            let a49 = {
                                let a47 = Str::lit("@");
                                utf8_encode(a47)
                            };
                            if if equal(a48, a49) {
                                f15(v6.clone())
                            } else {
                                false
                            } {
                                Ok(Some(Adt0::C0(v6, v2, true, None::<Bytes>)))
                            } else {
                                let a51 = v11.clone();
                                let a52 = {
                                    let a50 = Str::lit("[");
                                    utf8_encode(a50)
                                };
                                if equal(a51, a52) {
                                    if v3 {
                                        Ok(Some(Adt0::C0(v6, 1u64, false, None::<Bytes>)))
                                    } else {
                                        let a53 = 0u64;
                                        let a54 = v2;
                                        if nat_lt(a53, a54) {
                                            Ok(Some(Adt0::C0(v6, {
                                                let a55 = v2;
                                                let a56 = 1u64;
                                                nat_add(a55, a56)?
                                            }, false, None::<Bytes>)))
                                        } else {
                                            Ok(Some(Adt0::C0(v6, 0u64, false, None::<Bytes>)))
                                        }
                                    }
                                } else {
                                    let a58 = v11;
                                    let a59 = {
                                        let a57 = Str::lit("]");
                                        utf8_encode(a57)
                                    };
                                    if equal(a58, a59) {
                                        let a60 = 0u64;
                                        let a61 = v2;
                                        if nat_lt(a60, a61) {
                                            Ok(Some(Adt0::C0(v6, {
                                                let a62 = v2;
                                                let a63 = 1u64;
                                                nat_sub(a62, a63)
                                            }, v3, None::<Bytes>)))
                                        } else {
                                            Ok(Some(Adt0::C0(v6, v2, v3, None::<Bytes>)))
                                        }
                                    } else {
                                        Ok(Some(Adt0::C0(v6, v2, v3, None::<Bytes>)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m65 = match v2.clone() {
        Adt4::C0(_, _, _, _, _, h64) => {
            h64
        }
    };
    match m65 {
        None => {
            let v4: Option<Adt4> = f16(v0, match v2.clone() {
                Adt4::C0(h66, _, _, _, _, _) => {
                    h66
                }
            }, match v2.clone() {
                Adt4::C0(_, h67, _, _, _, _) => {
                    h67
                }
            }, match v2.clone() {
                Adt4::C0(_, _, h68, _, _, _) => {
                    h68
                }
            }, match v2.clone() {
                Adt4::C0(_, _, _, h69, _, _) => {
                    h69
                }
            }, match v2.clone() {
                Adt4::C0(_, _, _, _, h70, _) => {
                    h70
                }
            })?;
            let a72 = match v2 {
                Adt4::C0(h71, _, _, _, _, _) => {
                    h71
                }
            };
            let a73 = v1;
            if nat_lt(a72, a73) {
                Ok(v4)
            } else {
                Ok(None::<Adt4>)
            }
        }
        Some(_) => {
            Ok(None::<Adt4>)
        }
    }
}

pub fn f12(v0: Fn3, p77: List<Adt1>, p78: List<Adt1>) -> List<Adt1> {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, v2) = m76;
                let m74 = v2;
                match m74.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c75: Fn3 = v0.clone();
                            c75.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m76 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f14(v0: List<Adt3>, v1: List<Adt1>, v2: u64, v3: bool, v4: List<Bytes>) -> Adt0 {
    let m79 = f17(v0.clone());
    match m79 {
        None => {
            let m91 = v0;
            match m91.uncons() {
                None => {
                    Adt0::C0(v1, v2, v3, None::<Bytes>)
                }
                Some((v8, v9)) => {
                    let m92 = v9;
                    match m92.uncons() {
                        None => {
                            let v10: bool = f19(match v8.clone() {
                                Adt3::C0(h93, _) => {
                                    h93
                                }
                            });
                            let v11: bool = {
                                let a95 = match v8.clone() {
                                    Adt3::C0(_, h94) => {
                                        h94
                                    }
                                };
                                if bool_not(a95) {
                                    true
                                } else {
                                    let a96 = 0u64;
                                    let a97 = v2;
                                    nat_lt(a96, a97)
                                }
                            };
                            if if v10 {
                                v11
                            } else {
                                false
                            } {
                                let v12: Bytes = {
                                    let a103 = {
                                        let a100 = {
                                            let a98 = Str::lit("the forbidden token `");
                                            utf8_encode(a98)
                                        };
                                        let a101 = match v8 {
                                            Adt3::C0(h99, _) => {
                                                h99
                                            }
                                        };
                                        append_bytes(a100, a101)
                                    };
                                    let a104 = {
                                        let a102 = Str::lit("`");
                                        utf8_encode(a102)
                                    };
                                    append_bytes(a103, a104)
                                };
                                Adt0::C0(v1, v2, v3, Some(v12))
                            } else {
                                if match v8.clone() {
                                    Adt3::C0(_, h105) => {
                                        h105
                                    }
                                } {
                                    Adt0::C0(v1, v2, v3, None::<Bytes>)
                                } else {
                                    let a108 = match v8.clone() {
                                        Adt3::C0(h106, _) => {
                                            h106
                                        }
                                    };
                                    let a109 = {
                                        let a107 = Str::lit("attribute");
                                        utf8_encode(a107)
                                    };
                                    if if equal(a108, a109) {
                                        f15(v1.clone())
                                    } else {
                                        false
                                    } {
                                        Adt0::C0(v1, v2, true, None::<Bytes>)
                                    } else {
                                        let a112 = match v8.clone() {
                                            Adt3::C0(h110, _) => {
                                                h110
                                            }
                                        };
                                        let a113 = {
                                            let a111 = Str::lit("set_option");
                                            utf8_encode(a111)
                                        };
                                        if equal(a112, a113) {
                                            let v13: Bytes = f20(v1.clone());
                                            let v14: bool = f21(v13.clone());
                                            if v14 {
                                                Adt0::C0(v1, v2, v3, None::<Bytes>)
                                            } else {
                                                let v15: Bytes = {
                                                    let a118 = {
                                                        let a115 = {
                                                            let a114 = Str::lit("the option `");
                                                            utf8_encode(a114)
                                                        };
                                                        let a116 = v13;
                                                        append_bytes(a115, a116)
                                                    };
                                                    let a119 = {
                                                        let a117 = Str::lit("` is not allowed");
                                                        utf8_encode(a117)
                                                    };
                                                    append_bytes(a118, a119)
                                                };
                                                Adt0::C0(v1, v2, v3, Some(v15))
                                            }
                                        } else {
                                            let a122 = match v8 {
                                                Adt3::C0(h120, _) => {
                                                    h120
                                                }
                                            };
                                            let a123 = {
                                                let a121 = Str::lit("import");
                                                utf8_encode(a121)
                                            };
                                            if equal(a122, a123) {
                                                let v16: Bytes = f20(v1.clone());
                                                let v17: bool = f22(v16.clone(), v4);
                                                if v17 {
                                                    Adt0::C0(v1, v2, v3, None::<Bytes>)
                                                } else {
                                                    let v18: Bytes = {
                                                        let a128 = {
                                                            let a125 = {
                                                                let a124 = Str::lit("the import of `");
                                                                utf8_encode(a124)
                                                            };
                                                            let a126 = v16;
                                                            append_bytes(a125, a126)
                                                        };
                                                        let a129 = {
                                                            let a127 = Str::lit("` is not allowed");
                                                            utf8_encode(a127)
                                                        };
                                                        append_bytes(a128, a129)
                                                    };
                                                    Adt0::C0(v1, v2, v3, Some(v18))
                                                }
                                            } else {
                                                Adt0::C0(v1, v2, v3, None::<Bytes>)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some((_, _)) => {
                            Adt0::C0(v1, v2, v3, None::<Bytes>)
                        }
                    }
                }
            }
        }
        Some(v5) => {
            let v6: Bytes = f18(v0);
            let v7: Bytes = {
                let a89 = {
                    let a86 = {
                        let a84 = {
                            let a81 = {
                                let a80 = Str::lit("the forbidden constant `");
                                utf8_encode(a80)
                            };
                            let a82 = v5;
                            append_bytes(a81, a82)
                        };
                        let a85 = {
                            let a83 = Str::lit("` (in `");
                            utf8_encode(a83)
                        };
                        append_bytes(a84, a85)
                    };
                    let a87 = v6;
                    append_bytes(a86, a87)
                };
                let a90 = {
                    let a88 = Str::lit("`)");
                    utf8_encode(a88)
                };
                append_bytes(a89, a90)
            };
            Adt0::C0(v1, v2, v3, Some(v7))
        }
    }
}

pub fn f15(v0: List<Adt1>) -> bool {
    let m130 = v0;
    match m130.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m132 = match v1 {
                Adt1::C0(h131, _, _, _) => {
                    h131
                }
            };
            match m132 {
                Adt2::C0(_) => {
                    false
                }
                Adt2::C1 => {
                    false
                }
                Adt2::C2 => {
                    false
                }
                Adt2::C3(_) => {
                    false
                }
                Adt2::C4(v5) => {
                    let a134 = v5;
                    let a135 = {
                        let a133 = Str::lit("[");
                        utf8_encode(a133)
                    };
                    equal(a134, a135)
                }
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt1>) -> R<Option<Adt4>> {
    let v6: u8 = f23(v0.clone(), v1);
    let v7: bool = f24(v0.clone(), {
        let a136 = v1;
        let a137 = 1u64;
        nat_add(a136, a137)?
    }, 45u8);
    let a138 = v6;
    let a139 = 10u8;
    if equal(a138, a139) {
        Ok(Some(Adt4::C0({
            let a140 = v1;
            let a141 = 1u64;
            nat_add(a140, a141)?
        }, {
            let a142 = v2;
            let a143 = 1u64;
            nat_add(a142, a143)?
        }, true, {
            let a144 = v4;
            let a145 = 1u64;
            nat_add(a144, a145)?
        }, v5, None::<Bytes>)))
    } else {
        let a146 = v6;
        let a147 = 32u8;
        if if equal(a146, a147) {
            true
        } else {
            let a148 = v6;
            let a149 = 9u8;
            if equal(a148, a149) {
                true
            } else {
                let a150 = v6;
                let a151 = 13u8;
                equal(a150, a151)
            }
        } {
            Ok(Some(Adt4::C0({
                let a152 = v1;
                let a153 = 1u64;
                nat_add(a152, a153)?
            }, v2, v3, {
                let a154 = v4;
                let a155 = 1u64;
                nat_add(a154, a155)?
            }, v5, None::<Bytes>)))
        } else {
            let m156 = f25(v0.clone(), v1)?;
            match m156 {
                None => {
                    let a157 = v6;
                    let a158 = 45u8;
                    if if equal(a157, a158) {
                        v7
                    } else {
                        false
                    } {
                        let v10: u64 = f28(v0.clone(), v1)?;
                        Ok(Some(Adt4::C0(v10, v2, v3, {
                            let a159 = v4;
                            let a160 = f29(v0, v1, v10)?;
                            nat_add(a159, a160)?
                        }, v5, None::<Bytes>)))
                    } else {
                        let a161 = v6;
                        let a162 = 47u8;
                        if if equal(a161, a162) {
                            v7
                        } else {
                            false
                        } {
                            let v11: Adt5 = f30(v0.clone(), {
                                let a163 = v1;
                                let a164 = 2u64;
                                nat_add(a163, a164)?
                            }, v2)?;
                            if match v11.clone() {
                                Adt5::C0(_, _, _, h165) => {
                                    h165
                                }
                            } {
                                Ok(Some(f27(v1, v2, v3, v4, v5, {
                                    let a166 = Str::lit("a comment is not closed");
                                    utf8_encode(a166)
                                })))
                            } else {
                                Ok(Some(Adt4::C0(match v11.clone() {
                                    Adt5::C0(h167, _, _, _) => {
                                        h167
                                    }
                                }, match v11.clone() {
                                    Adt5::C0(_, h168, _, _) => {
                                        h168
                                    }
                                }, v3, {
                                    let a170 = v4;
                                    let a171 = f29(v0, v1, match v11 {
                                        Adt5::C0(h169, _, _, _) => {
                                            h169
                                        }
                                    })?;
                                    nat_add(a170, a171)?
                                }, v5, None::<Bytes>)))
                            }
                        } else {
                            Ok(Some(f31(v0, v6, v1, v2, v3, v4, v5)?))
                        }
                    }
                }
                Some(v8) => {
                    let v9: Bytes = f26(v8);
                    Ok(Some(f27(v1, v2, v3, v4, v5, v9)))
                }
            }
        }
    }
}

pub fn f17(v0: List<Adt3>) -> Option<Bytes> {
    f32(Fn4::F33, None::<Bytes>, v0)
}

pub fn f18(v0: List<Adt3>) -> Bytes {
    let v1: List<Bytes> = f34(f35(Fn5::F36, List::<Bytes>::nil(), v0));
    f37(v1, {
        let a172 = Str::lit(".");
        utf8_encode(a172)
    })
}

pub fn f19(v0: Bytes) -> bool {
    let v1: List<Bytes> = List::cons({
        let a173 = Str::lit("sorry");
        utf8_encode(a173)
    }, List::cons({
        let a174 = Str::lit("admit");
        utf8_encode(a174)
    }, List::cons({
        let a175 = Str::lit("axiom");
        utf8_encode(a175)
    }, List::cons({
        let a176 = Str::lit("opaque");
        utf8_encode(a176)
    }, List::cons({
        let a177 = Str::lit("unsafe");
        utf8_encode(a177)
    }, List::cons({
        let a178 = Str::lit("partial");
        utf8_encode(a178)
    }, List::cons({
        let a179 = Str::lit("native_decide");
        utf8_encode(a179)
    }, List::cons({
        let a180 = Str::lit("kernel");
        utf8_encode(a180)
    }, List::cons({
        let a181 = Str::lit("ofReduceBool");
        utf8_encode(a181)
    }, List::cons({
        let a182 = Str::lit("ofReduceNat");
        utf8_encode(a182)
    }, List::cons({
        let a183 = Str::lit("implemented_by");
        utf8_encode(a183)
    }, List::cons({
        let a184 = Str::lit("extern");
        utf8_encode(a184)
    }, List::cons({
        let a185 = Str::lit("macro");
        utf8_encode(a185)
    }, List::cons({
        let a186 = Str::lit("macro_rules");
        utf8_encode(a186)
    }, List::cons({
        let a187 = Str::lit("syntax");
        utf8_encode(a187)
    }, List::cons({
        let a188 = Str::lit("elab");
        utf8_encode(a188)
    }, List::cons({
        let a189 = Str::lit("elab_rules");
        utf8_encode(a189)
    }, List::cons({
        let a190 = Str::lit("notation");
        utf8_encode(a190)
    }, List::cons({
        let a191 = Str::lit("infix");
        utf8_encode(a191)
    }, List::cons({
        let a192 = Str::lit("infixl");
        utf8_encode(a192)
    }, List::cons({
        let a193 = Str::lit("infixr");
        utf8_encode(a193)
    }, List::cons({
        let a194 = Str::lit("prefix");
        utf8_encode(a194)
    }, List::cons({
        let a195 = Str::lit("postfix");
        utf8_encode(a195)
    }, List::cons({
        let a196 = Str::lit("#eval");
        utf8_encode(a196)
    }, List::cons({
        let a197 = Str::lit("run_cmd");
        utf8_encode(a197)
    }, List::cons({
        let a198 = Str::lit("run_tac");
        utf8_encode(a198)
    }, List::cons({
        let a199 = Str::lit("sorryAx");
        utf8_encode(a199)
    }, List::<Bytes>::nil())))))))))))))))))))))))))));
    f22(v0, v1)
}

pub fn f20(v0: List<Adt1>) -> Bytes {
    let m200 = v0;
    match m200.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            let m202 = match v1 {
                Adt1::C0(h201, _, _, _) => {
                    h201
                }
            };
            match m202 {
                Adt2::C0(v3) => {
                    f18(v3)
                }
                Adt2::C1 => {
                    Bytes::lit(&[])
                }
                Adt2::C2 => {
                    Bytes::lit(&[])
                }
                Adt2::C3(_) => {
                    Bytes::lit(&[])
                }
                Adt2::C4(_) => {
                    Bytes::lit(&[])
                }
            }
        }
    }
}

pub fn f21(v0: Bytes) -> bool {
    let v1: List<Bytes> = List::cons({
        let a203 = Str::lit("autoImplicit");
        utf8_encode(a203)
    }, List::cons({
        let a204 = Str::lit("maxRecDepth");
        utf8_encode(a204)
    }, List::cons({
        let a205 = Str::lit("maxHeartbeats");
        utf8_encode(a205)
    }, List::cons({
        let a206 = Str::lit("synthInstance.maxHeartbeats");
        utf8_encode(a206)
    }, List::cons({
        let a207 = Str::lit("linter.unusedVariables");
        utf8_encode(a207)
    }, List::cons({
        let a208 = Str::lit("linter.unusedSimpArgs");
        utf8_encode(a208)
    }, List::<Bytes>::nil()))))));
    f22(v0, v1)
}

pub fn f22(v0: Bytes, v1: List<Bytes>) -> bool {
    f38(Fn6::F39(v0), false, v1)
}

pub fn f23(v0: Bytes, v1: u64) -> u8 {
    let m209 = f40(v0, v1);
    match m209 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m210 = f40(v0, v1);
    match m210 {
        None => {
            false
        }
        Some(v3) => {
            let a211 = v3;
            let a212 = v2;
            equal(a211, a212)
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<Option<u8>> {
    let a214 = v1;
    let a215 = {
        let a213 = v0.clone();
        length_bytes(a213)
    };
    if nat_lt(a214, a215) {
        let v2: u8 = f23(v0.clone(), v1);
        if f41(v2, 0u8, 31u8) {
            Ok(Some(v2))
        } else {
            let a216 = v2;
            let a217 = 127u8;
            if equal(a216, a217) {
                Ok(Some(v2))
            } else {
                let a218 = v2;
                let a219 = 194u8;
                if equal(a218, a219) {
                    if f42(v0.clone(), {
                        let a220 = v1;
                        let a221 = 1u64;
                        nat_add(a220, a221)?
                    }, 128u8, 159u8) {
                        Ok(Some(f23(v0, {
                            let a222 = v1;
                            let a223 = 1u64;
                            nat_add(a222, a223)?
                        })))
                    } else {
                        Ok(None::<u8>)
                    }
                } else {
                    Ok(None::<u8>)
                }
            }
        }
    } else {
        Ok(None::<u8>)
    }
}

pub fn f26(v0: u8) -> Bytes {
    let v2: u8 = {
        let m226 = {
            let a224 = v0;
            let a225 = 4u32;
            fixed_u8::shift_right(a224, a225)
        };
        match m226 {
            None => {
                0u8
            }
            Some(v1) => {
                v1
            }
        }
    };
    let v3: u8 = {
        let a227 = v0;
        let a228 = 15u8;
        fixed_u8::bit_and(a227, a228)
    };
    let v4: Bytes = f43(v2);
    let v5: Bytes = f43(v3);
    let a235 = {
        let a232 = {
            let a229 = Str::lit("a control character U+00");
            utf8_encode(a229)
        };
        let a233 = {
            let a230 = v4;
            let a231 = v5;
            append_bytes(a230, a231)
        };
        append_bytes(a232, a233)
    };
    let a236 = {
        let a234 = Str::lit(" in the text");
        utf8_encode(a234)
    };
    append_bytes(a235, a236)
}

pub fn f27(v0: u64, v1: u64, v2: bool, v3: u64, v4: List<Adt1>, v5: Bytes) -> Adt4 {
    Adt4::C0(v0, v1, v2, v3, v4, Some(v5))
}

pub fn f28(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a237 = v0.clone();
        length_bytes(a237)
    };
    let (h238, _) = f44(Fn7::F45(v2, v0), {
        let a239 = v2;
        let a240 = v1;
        nat_sub(a239, a240)
    }, v1)?;
    Ok(h238)
}

pub fn f29(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h241) = {
        let (h242, _) = f46(Fn8::F47(v0, v2), {
            let a243 = v2;
            let a244 = v1;
            nat_sub(a243, a244)
        }, (v1, 0u64))?;
        h242
    };
    Ok(h241)
}

pub fn f30(v0: Bytes, v1: u64, v2: u64) -> R<Adt5> {
    let v3: u64 = {
        let a245 = v0.clone();
        length_bytes(a245)
    };
    let (h246, _) = f48(Fn9::F49(v3, v0), {
        let a249 = {
            let a247 = v3;
            let a248 = v1;
            nat_sub(a247, a248)
        };
        let a250 = 1u64;
        nat_add(a249, a250)?
    }, Adt5::C0(v1, v2, 1u64, false))?;
    Ok(h246)
}

pub fn f31(v0: Bytes, v1: u8, v2: u64, v3: u64, v4: bool, v5: u64, v6: List<Adt1>) -> R<Adt4> {
    let a251 = v1;
    let a252 = 34u8;
    if equal(a251, a252) {
        let v7: Adt6 = f50(v0.clone(), {
            let a253 = v2;
            let a254 = 1u64;
            nat_add(a253, a254)?
        }, v3)?;
        if match v7.clone() {
            Adt6::C0(_, _, _, h255) => {
                h255
            }
        } {
            Ok(f27(v2, v3, false, v5, v6, {
                let a256 = Str::lit("a string is not closed");
                utf8_encode(a256)
            }))
        } else {
            f51(match v7.clone() {
                Adt6::C0(h257, _, _, _) => {
                    h257
                }
            }, match v7.clone() {
                Adt6::C0(_, h258, _, _) => {
                    h258
                }
            }, v5, f29(v0, v2, match v7 {
                Adt6::C0(h259, _, _, _) => {
                    h259
                }
            })?, Adt2::C2, v3, v4, v6)
        }
    } else {
        let a260 = v1;
        let a261 = 39u8;
        if equal(a260, a261) {
            let m262 = f52(v0.clone(), v2)?;
            match m262 {
                None => {
                    Ok(f27(v2, v3, false, v5, v6, {
                        let a263 = Str::lit("a character literal is not closed");
                        utf8_encode(a263)
                    }))
                }
                Some(v8) => {
                    f51(v8, v3, v5, f29(v0, v2, v8)?, Adt2::C2, v3, v4, v6)
                }
            }
        } else {
            if f53(v1) {
                let m264 = f54(v0.clone(), v2)?;
                match m264 {
                    Ok(v10) => {
                        f51(v10, v3, v5, f29(v0, v2, v10)?, Adt2::C1, v3, v4, v6)
                    }
                    Err(v9) => {
                        Ok(f27(v2, v3, false, v5, v6, v9))
                    }
                }
            } else {
                let v11: u64 = f55(v0.clone(), {
                    let a265 = v2;
                    let a266 = 1u64;
                    nat_add(a265, a266)?
                })?;
                let a267 = v1;
                let a268 = 35u8;
                if if equal(a267, a268) {
                    let a269 = 0u64;
                    let a270 = v11;
                    nat_lt(a269, a270)
                } else {
                    false
                } {
                    let v12: u64 = f56(v0.clone(), {
                        let a273 = {
                            let a271 = v2;
                            let a272 = 1u64;
                            nat_add(a271, a272)?
                        };
                        let a274 = v11;
                        nat_add(a273, a274)?
                    })?;
                    f51(v12, v3, v5, f29(v0.clone(), v2, v12)?, Adt2::C3(f57(v0, v2, {
                        let a275 = v12;
                        let a276 = v2;
                        nat_sub(a275, a276)
                    })), v3, v4, v6)
                } else {
                    if f58(v0.clone(), v2)? {
                        f59(v0, v2, v3, v4, v5, v6)
                    } else {
                        let v13: u64 = f60(v0.clone(), v2);
                        f51({
                            let a277 = v2;
                            let a278 = v13;
                            nat_add(a277, a278)?
                        }, v3, v5, 1u64, Adt2::C4(f57(v0, v2, v13)), v3, v4, v6)
                    }
                }
            }
        }
    }
}

pub fn f32(v0: Fn4, p282: Option<Bytes>, p283: List<Adt3>) -> Option<Bytes> {
    let m281 = (p282, p283);
    {
        let mut m281 = m281;
        loop {
            let step = {
                let (v1, v2) = m281;
                let m279 = v2;
                match m279.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt3>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt3>), Option<Bytes>>(({
                            let c280: Fn4 = v0.clone();
                            c280.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Option<Bytes>, v1: Adt3) -> Option<Bytes> {
    let m284 = v0;
    match m284 {
        None => {
            let v3: bool = f61(match v1.clone() {
                Adt3::C0(h285, _) => {
                    h285
                }
            });
            if v3 {
                Some(match v1 {
                    Adt3::C0(h286, _) => {
                        h286
                    }
                })
            } else {
                None::<Bytes>
            }
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f34(v0: List<Bytes>) -> List<Bytes> {
    f62(Fn10::F63, List::<Bytes>::nil(), v0)
}

pub fn f35(v0: Fn5, p290: List<Bytes>, p291: List<Adt3>) -> List<Bytes> {
    let m289 = (p290, p291);
    {
        let mut m289 = m289;
        loop {
            let step = {
                let (v1, v2) = m289;
                let m287 = v2;
                match m287.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt3>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt3>), List<Bytes>>(({
                            let c288: Fn5 = v0.clone();
                            c288.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: List<Bytes>, v1: Adt3) -> List<Bytes> {
    List::cons(match v1 {
        Adt3::C0(h292, _) => {
            h292
        }
    }, v0)
}

pub fn f37(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m293 = f64(Fn11::F65(v1), None::<Bytes>, v0);
    match m293 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f38(v0: Fn6, p297: bool, p298: List<Bytes>) -> bool {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v1, v2) = m296;
                let m294 = v2;
                match m294.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c295: Fn6 = v0.clone();
                            c295.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a299 = v2;
        let a300 = v0;
        equal(a299, a300)
    }
}

pub fn f40(v0: Bytes, v1: u64) -> Option<u8> {
    let a301 = v0;
    let a302 = v1;
    index_bytes(a301, a302)
}

pub fn f41(v0: u8, v1: u8, v2: u8) -> bool {
    let m305 = {
        let a303 = v0;
        let a304 = v1;
        fixed_u8::checked_sub(a303, a304)
    };
    match m305 {
        None => {
            false
        }
        Some(_) => {
            let m308 = {
                let a306 = v2;
                let a307 = v0;
                fixed_u8::checked_sub(a306, a307)
            };
            match m308 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m309 = f40(v0, v1);
    match m309 {
        None => {
            false
        }
        Some(v4) => {
            f41(v4, v2, v3)
        }
    }
}

pub fn f43(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a310 = Str::lit("0123456789ABCDEF");
        utf8_encode(a310)
    };
    let v2: u64 = {
        let a311 = v0;
        let a312 = 0u8;
        if equal(a311, a312) {
            0u64
        } else {
            let a313 = v0;
            let a314 = 1u8;
            if equal(a313, a314) {
                1u64
            } else {
                let a315 = v0;
                let a316 = 2u8;
                if equal(a315, a316) {
                    2u64
                } else {
                    let a317 = v0;
                    let a318 = 3u8;
                    if equal(a317, a318) {
                        3u64
                    } else {
                        let a319 = v0;
                        let a320 = 4u8;
                        if equal(a319, a320) {
                            4u64
                        } else {
                            let a321 = v0;
                            let a322 = 5u8;
                            if equal(a321, a322) {
                                5u64
                            } else {
                                let a323 = v0;
                                let a324 = 6u8;
                                if equal(a323, a324) {
                                    6u64
                                } else {
                                    let a325 = v0;
                                    let a326 = 7u8;
                                    if equal(a325, a326) {
                                        7u64
                                    } else {
                                        let a327 = v0;
                                        let a328 = 8u8;
                                        if equal(a327, a328) {
                                            8u64
                                        } else {
                                            let a329 = v0;
                                            let a330 = 9u8;
                                            if equal(a329, a330) {
                                                9u64
                                            } else {
                                                let a331 = v0;
                                                let a332 = 10u8;
                                                if equal(a331, a332) {
                                                    10u64
                                                } else {
                                                    let a333 = v0;
                                                    let a334 = 11u8;
                                                    if equal(a333, a334) {
                                                        11u64
                                                    } else {
                                                        let a335 = v0;
                                                        let a336 = 12u8;
                                                        if equal(a335, a336) {
                                                            12u64
                                                        } else {
                                                            let a337 = v0;
                                                            let a338 = 13u8;
                                                            if equal(a337, a338) {
                                                                13u64
                                                            } else {
                                                                let a339 = v0;
                                                                let a340 = 14u8;
                                                                if equal(a339, a340) {
                                                                    14u64
                                                                } else {
                                                                    15u64
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
                    }
                }
            }
        }
    };
    f57(v1, v2, 1u64)
}

pub fn f44(v0: Fn7, p345: u64, p346: u64) -> R<(u64, bool)> {
    let m344 = (p345, p346);
    {
        let mut m344 = m344;
        loop {
            let step = {
                let (v1, v2) = m344;
                let m341 = v1;
                if m341 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m341 - 1;
                    let m343 = {
                        let c342: Fn7 = v0.clone();
                        c342.apply(v2)?
                    };
                    match m343 {
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
                Ok(next) => { m344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f66(v1, v0, v2)
}

pub fn f46(v0: Fn8, p351: u64, p352: (u64, u64)) -> R<((u64, u64), bool)> {
    let m350 = (p351, p352);
    {
        let mut m350 = m350;
        loop {
            let step = {
                let (v1, v2) = m350;
                let m347 = v1;
                if m347 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m347 - 1;
                    let m349 = {
                        let c348: Fn8 = v0.clone();
                        c348.apply(v2)?
                    };
                    match m349 {
                        None => {
                            Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, u64)), R<((u64, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f67(v0, v1, v2)
}

pub fn f48(v0: Fn9, p357: u64, p358: Adt5) -> R<(Adt5, bool)> {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v1, v2) = m356;
                let m353 = v1;
                if m353 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m353 - 1;
                    let m355 = {
                        let c354: Fn9 = v0.clone();
                        c354.apply(v2.clone())?
                    };
                    match m355 {
                        None => {
                            Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), R<(Adt5, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f68(v1, v0, v2)
}

pub fn f50(v0: Bytes, v1: u64, v2: u64) -> R<Adt6> {
    let v3: u64 = {
        let a359 = v0.clone();
        length_bytes(a359)
    };
    let (h360, _) = f69(Fn12::F70(v3, v0), {
        let a363 = {
            let a361 = v3;
            let a362 = v1;
            nat_sub(a361, a362)
        };
        let a364 = 1u64;
        nat_add(a363, a364)?
    }, Adt6::C0(v1, v2, false, false))?;
    Ok(h360)
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt2, v5: u64, v6: bool, v7: List<Adt1>) -> R<Adt4> {
    Ok(Adt4::C0(v0, v1, false, {
        let a365 = v2;
        let a366 = v3;
        nat_add(a365, a366)?
    }, List::cons(Adt1::C0(v4, v5, v6, v2), v7), None::<Bytes>))
}

pub fn f52(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a367 = v0.clone();
        length_bytes(a367)
    };
    let v3: u64 = {
        let a368 = v1;
        let a369 = 1u64;
        nat_add(a368, a369)?
    };
    let v4: u8 = f23(v0.clone(), v3);
    let v5: u64 = {
        let a370 = v3;
        let a371 = 1u64;
        nat_add(a370, a371)?
    };
    let v6: u8 = f23(v0.clone(), v5);
    let v7: u64 = {
        let a372 = v5;
        let a373 = f60(v0.clone(), v5);
        nat_add(a372, a373)?
    };
    let v8: Option<u64> = f71(v0.clone(), v7, 2u64)?;
    let v9: Option<u64> = f71(v0.clone(), v7, 4u64)?;
    let v10: u64 = f60(v0.clone(), v3);
    let v11: Option<u64> = {
        let a374 = v3;
        let a375 = v2;
        if nat_lt(a374, a375) {
            let a376 = v4;
            let a377 = 92u8;
            if equal(a376, a377) {
                let a378 = v5;
                let a379 = v2;
                if nat_lt(a378, a379) {
                    let a380 = v6;
                    let a381 = 120u8;
                    if equal(a380, a381) {
                        v8
                    } else {
                        let a382 = v6;
                        let a383 = 117u8;
                        if equal(a382, a383) {
                            v9
                        } else {
                            Some(v7)
                        }
                    }
                } else {
                    None::<u64>
                }
            } else {
                let a384 = v4;
                let a385 = 39u8;
                if equal(a384, a385) {
                    None::<u64>
                } else {
                    let a386 = v4;
                    let a387 = 10u8;
                    if equal(a386, a387) {
                        None::<u64>
                    } else {
                        Some({
                            let a388 = v3;
                            let a389 = v10;
                            nat_add(a388, a389)?
                        })
                    }
                }
            }
        } else {
            None::<u64>
        }
    };
    let m390 = v11;
    match m390 {
        None => {
            Ok(None::<u64>)
        }
        Some(v12) => {
            let v13: bool = f24(v0, v12, 39u8);
            if v13 {
                Ok(Some({
                    let a391 = v12;
                    let a392 = 1u64;
                    nat_add(a391, a392)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f53(v0: u8) -> bool {
    f41(v0, 48u8, 57u8)
}

pub fn f54(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: u8 = f23(v0.clone(), {
        let a393 = v1;
        let a394 = 1u64;
        nat_add(a393, a394)?
    });
    let v4: u64 = {
        let a395 = v2;
        let a396 = 48u8;
        if equal(a395, a396) {
            let a397 = v3;
            let a398 = 120u8;
            if equal(a397, a398) {
                16u64
            } else {
                let a399 = v3;
                let a400 = 88u8;
                if equal(a399, a400) {
                    16u64
                } else {
                    let a401 = v3;
                    let a402 = 98u8;
                    if equal(a401, a402) {
                        2u64
                    } else {
                        let a403 = v3;
                        let a404 = 66u8;
                        if equal(a403, a404) {
                            2u64
                        } else {
                            let a405 = v3;
                            let a406 = 111u8;
                            if equal(a405, a406) {
                                8u64
                            } else {
                                let a407 = v3;
                                let a408 = 79u8;
                                if equal(a407, a408) {
                                    8u64
                                } else {
                                    10u64
                                }
                            }
                        }
                    }
                }
            }
        } else {
            10u64
        }
    };
    let v5: Result<u64, Bytes> = f72(v0.clone(), {
        let a409 = v1;
        let a410 = 2u64;
        nat_add(a409, a410)?
    }, v4, true)?;
    let v6: u64 = f73(v0.clone(), v1)?;
    let v9: Result<u64, Bytes> = {
        let m413 = f72(v0.clone(), {
            let a411 = v1;
            let a412 = 1u64;
            nat_add(a411, a412)?
        }, 10u64, false)?;
        match m413 {
            Ok(v8) => {
                f74(v0.clone(), v8)?
            }
            Err(v7) => {
                Err::<u64, Bytes>(v7)
            }
        }
    };
    let a416 = {
        let a414 = v4;
        let a415 = 10u64;
        nat_eq(a414, a415)
    };
    if bool_not(a416) {
        Ok(v5)
    } else {
        let a419 = {
            let a417 = v2;
            let a418 = 48u8;
            equal(a417, a418)
        };
        if if bool_not(a419) {
            let a420 = 0u64;
            let a421 = v1;
            if nat_lt(a420, a421) {
                f24(v0.clone(), {
                    let a422 = v1;
                    let a423 = 1u64;
                    nat_sub(a422, a423)
                }, 46u8)
            } else {
                false
            }
        } else {
            false
        } {
            if if f24(v0.clone(), v6, 95u8) {
                true
            } else {
                if f24(v0.clone(), v6, 101u8) {
                    true
                } else {
                    f24(v0, v6, 69u8)
                }
            } {
                Ok(Err::<u64, Bytes>({
                    let a424 = Str::lit("a field index is followed by what could continue a number");
                    utf8_encode(a424)
                }))
            } else {
                Ok(Ok::<u64, Bytes>(v6))
            }
        } else {
            Ok(v9)
        }
    }
}

pub fn f55(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: bool = f75(v2);
    let v4: bool = {
        let a425 = v2;
        let a426 = 95u8;
        equal(a425, a426)
    };
    let v5: u64 = f76(v0.clone(), v1)?;
    let a428 = v1;
    let a429 = {
        let a427 = v0;
        length_bytes(a427)
    };
    if nat_lt(a428, a429) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                Ok(v5)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f56(v0: Bytes, v1: u64) -> R<u64> {
    let (h430, _) = f44(Fn7::F77(v0.clone()), {
        let a432 = {
            let a431 = v0;
            length_bytes(a431)
        };
        let a433 = v1;
        nat_sub(a432, a433)
    }, v1)?;
    Ok(h430)
}

pub fn f57(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m437 = {
        let a434 = v0;
        let a435 = v1;
        let a436 = v2;
        slice_bytes(a434, a435, a436)
    };
    match m437 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f58(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f55(v0.clone(), v1)?;
    let v3: bool = f78(v0, v1)?;
    let a438 = 0u64;
    let a439 = v2;
    if nat_lt(a438, a439) {
        Ok(true)
    } else {
        Ok(v3)
    }
}

pub fn f59(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt1>) -> R<Adt4> {
    let m440 = f79(v0.clone(), v1)?;
    match m440 {
        Ok(v7) => {
            let m442 = {
                let (h441, _) = v7.clone();
                h441
            };
            match m442.uncons() {
                None => {
                    Ok(Adt4::C0({
                        let (_, h443) = v7;
                        h443
                    }, v2, false, v4, v5, None::<Bytes>))
                }
                Some((v8, v9)) => {
                    let m444 = v9;
                    match m444.uncons() {
                        None => {
                            let v10: bool = {
                                let a446 = match v8.clone() {
                                    Adt3::C0(_, h445) => {
                                        h445
                                    }
                                };
                                if bool_not(a446) {
                                    let a449 = match v8.clone() {
                                        Adt3::C0(h447, _) => {
                                            h447
                                        }
                                    };
                                    let a450 = {
                                        let a448 = Str::lit("r");
                                        utf8_encode(a448)
                                    };
                                    equal(a449, a450)
                                } else {
                                    false
                                }
                            };
                            let v11: u64 = if v10 {
                                f80(v0.clone(), {
                                    let (_, h451) = v7.clone();
                                    h451
                                })?
                            } else {
                                0u64
                            };
                            if if v10 {
                                f24(v0.clone(), {
                                    let a453 = {
                                        let (_, h452) = v7.clone();
                                        h452
                                    };
                                    let a454 = v11;
                                    nat_add(a453, a454)?
                                }, 34u8)
                            } else {
                                false
                            } {
                                let v12: Adt6 = f81(v0.clone(), {
                                    let a458 = {
                                        let a456 = {
                                            let (_, h455) = v7;
                                            h455
                                        };
                                        let a457 = v11;
                                        nat_add(a456, a457)?
                                    };
                                    let a459 = 1u64;
                                    nat_add(a458, a459)?
                                }, v2, v11)?;
                                if match v12.clone() {
                                    Adt6::C0(_, _, _, h460) => {
                                        h460
                                    }
                                } {
                                    Ok(f27(v1, v2, false, v4, v5, {
                                        let a461 = Str::lit("a raw string is not closed");
                                        utf8_encode(a461)
                                    }))
                                } else {
                                    f51(match v12.clone() {
                                        Adt6::C0(h462, _, _, _) => {
                                            h462
                                        }
                                    }, match v12.clone() {
                                        Adt6::C0(_, h463, _, _) => {
                                            h463
                                        }
                                    }, v4, f29(v0, v1, match v12 {
                                        Adt6::C0(h464, _, _, _) => {
                                            h464
                                        }
                                    })?, Adt2::C2, v2, v3, v5)
                                }
                            } else {
                                let v13: bool = f82(match v8.clone() {
                                    Adt3::C0(h465, _) => {
                                        h465
                                    }
                                }, {
                                    let a466 = Str::lit("!");
                                    utf8_encode(a466)
                                });
                                let v14: bool = f24(v0.clone(), {
                                    let (_, h467) = v7.clone();
                                    h467
                                }, 34u8);
                                let a469 = match v8 {
                                    Adt3::C0(_, h468) => {
                                        h468
                                    }
                                };
                                if if bool_not(a469) {
                                    if v13 {
                                        v14
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                } {
                                    Ok(f27(v1, v2, false, v4, v5, {
                                        let a470 = Str::lit("an interpolated string holds code");
                                        utf8_encode(a470)
                                    }))
                                } else {
                                    f51({
                                        let (_, h471) = v7.clone();
                                        h471
                                    }, v2, v4, f29(v0, v1, {
                                        let (_, h472) = v7;
                                        h472
                                    })?, Adt2::C0({
                                        let (h473, _) = v7;
                                        h473
                                    }), v2, v3, v5)
                                }
                            }
                        }
                        Some((_, _)) => {
                            f51({
                                let (_, h474) = v7.clone();
                                h474
                            }, v2, v4, f29(v0, v1, {
                                let (_, h475) = v7;
                                h475
                            })?, Adt2::C0({
                                let (h476, _) = v7;
                                h476
                            }), v2, v3, v5)
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(f27(v1, v2, false, v4, v5, v6))
        }
    }
}

pub fn f60(v0: Bytes, v1: u64) -> u64 {
    let v2: u8 = f23(v0, v1);
    let v3: bool = f41(v2, 0u8, 127u8);
    if v3 {
        1u64
    } else {
        let v4: bool = f41(v2, 128u8, 223u8);
        if v4 {
            2u64
        } else {
            let v5: bool = f41(v2, 224u8, 239u8);
            if v5 {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f61(v0: Bytes) -> bool {
    let a478 = v0.clone();
    let a479 = {
        let a477 = Str::lit("ofReduceBool");
        utf8_encode(a477)
    };
    if equal(a478, a479) {
        true
    } else {
        let a481 = v0.clone();
        let a482 = {
            let a480 = Str::lit("ofReduceNat");
            utf8_encode(a480)
        };
        if equal(a481, a482) {
            true
        } else {
            let a484 = v0;
            let a485 = {
                let a483 = Str::lit("sorryAx");
                utf8_encode(a483)
            };
            equal(a484, a485)
        }
    }
}

pub fn f62(v0: Fn10, p489: List<Bytes>, p490: List<Bytes>) -> List<Bytes> {
    let m488 = (p489, p490);
    {
        let mut m488 = m488;
        loop {
            let step = {
                let (v1, v2) = m488;
                let m486 = v2;
                match m486.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c487: Fn10 = v0.clone();
                            c487.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f64(v0: Fn11, p494: Option<Bytes>, p495: List<Bytes>) -> Option<Bytes> {
    let m493 = (p494, p495);
    {
        let mut m493 = m493;
        loop {
            let step = {
                let (v1, v2) = m493;
                let m491 = v2;
                match m491.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c492: Fn11 = v0.clone();
                            c492.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m496 = v1;
    match m496 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a499 = {
                    let a497 = v3;
                    let a498 = v0;
                    append_bytes(a497, a498)
                };
                let a500 = v2;
                append_bytes(a499, a500)
            })
        }
    }
}

pub fn f66(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a501 = v2;
    let a502 = v1;
    if nat_lt(a501, a502) {
        let v3: bool = f24(v0, v2, 10u8);
        if v3 {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a503 = v2;
                let a504 = 1u64;
                nat_add(a503, a504)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f67(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h505, _) = v2;
        h505
    };
    let v4: u64 = {
        let (_, h506) = v2;
        h506
    };
    let a507 = v3;
    let a508 = v1;
    if nat_lt(a507, a508) {
        let v5: u64 = f60(v0, v3);
        Ok(Some(({
            let a509 = v3;
            let a510 = v5;
            nat_add(a509, a510)?
        }, {
            let a511 = v4;
            let a512 = 1u64;
            nat_add(a511, a512)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f68(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h513, _, _, _) => {
            h513
        }
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, h514, _, _) => {
            h514
        }
    };
    let v5: u64 = match v2.clone() {
        Adt5::C0(_, _, h515, _) => {
            h515
        }
    };
    let a516 = v5;
    let a517 = 0u64;
    if nat_eq(a516, a517) {
        Ok(None::<Adt5>)
    } else {
        if match v2 {
            Adt5::C0(_, _, _, h518) => {
                h518
            }
        } {
            Ok(None::<Adt5>)
        } else {
            let a519 = v3;
            let a520 = v1;
            if nat_lt(a519, a520) {
                let v6: u8 = f23(v0.clone(), v3);
                let a521 = v6;
                let a522 = 47u8;
                if if equal(a521, a522) {
                    f24(v0.clone(), {
                        let a523 = v3;
                        let a524 = 1u64;
                        nat_add(a523, a524)?
                    }, 45u8)
                } else {
                    false
                } {
                    Ok(Some(Adt5::C0({
                        let a525 = v3;
                        let a526 = 2u64;
                        nat_add(a525, a526)?
                    }, v4, {
                        let a527 = v5;
                        let a528 = 1u64;
                        nat_add(a527, a528)?
                    }, false)))
                } else {
                    let a529 = v6;
                    let a530 = 45u8;
                    if if equal(a529, a530) {
                        f24(v0.clone(), {
                            let a531 = v3;
                            let a532 = 1u64;
                            nat_add(a531, a532)?
                        }, 47u8)
                    } else {
                        false
                    } {
                        Ok(Some(Adt5::C0({
                            let a533 = v3;
                            let a534 = 2u64;
                            nat_add(a533, a534)?
                        }, v4, {
                            let a535 = v5;
                            let a536 = 1u64;
                            nat_sub(a535, a536)
                        }, false)))
                    } else {
                        let a537 = v6;
                        let a538 = 10u8;
                        if equal(a537, a538) {
                            Ok(Some(Adt5::C0({
                                let a539 = v3;
                                let a540 = 1u64;
                                nat_add(a539, a540)?
                            }, {
                                let a541 = v4;
                                let a542 = 1u64;
                                nat_add(a541, a542)?
                            }, v5, false)))
                        } else {
                            Ok(Some(Adt5::C0({
                                let a543 = v3;
                                let a544 = f60(v0, v3);
                                nat_add(a543, a544)?
                            }, v4, v5, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt5::C0(v3, v4, v5, true)))
            }
        }
    }
}

pub fn f69(v0: Fn12, p549: u64, p550: Adt6) -> R<(Adt6, bool)> {
    let m548 = (p549, p550);
    {
        let mut m548 = m548;
        loop {
            let step = {
                let (v1, v2) = m548;
                let m545 = v1;
                if m545 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m545 - 1;
                    let m547 = {
                        let c546: Fn12 = v0.clone();
                        c546.apply(v2.clone())?
                    };
                    match m547 {
                        None => {
                            Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), R<(Adt6, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m548 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: u64, v1: Bytes, v2: Adt6) -> R<Option<Adt6>> {
    f83(v1, v0, v2)
}

pub fn f71(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: (u64, (u64, bool)) = {
        let (h551, _) = f84(Fn13::F85(v2, v0), v2, (v1, (0u64, true)))?;
        h551
    };
    let (_, h552) = {
        let (_, h553) = v3;
        h553
    };
    if h552 {
        Ok(Some({
            let (h554, _) = v3;
            h554
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f72(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<u64, Bytes>> {
    let v4: u64 = {
        let a555 = v0.clone();
        length_bytes(a555)
    };
    let v5: (u64, (bool, bool)) = {
        let (h556, _) = f86(Fn14::F87(v4, v2, v0), {
            let a559 = {
                let a557 = v4;
                let a558 = v1;
                nat_sub(a557, a558)
            };
            let a560 = 1u64;
            nat_add(a559, a560)?
        }, (v1, (v3, false)))?;
        h556
    };
    let (h561, _) = {
        let (_, h562) = v5;
        h562
    };
    if h561 {
        Ok(Err::<u64, Bytes>({
            let a563 = Str::lit("a number has a digit separator or a radix prefix with no digit after it");
            utf8_encode(a563)
        }))
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h564, _) = v5;
            h564
        }))
    }
}

pub fn f73(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a565 = v0.clone();
        length_bytes(a565)
    };
    let (h566, _) = f44(Fn7::F88(v0), {
        let a567 = v2;
        let a568 = v1;
        nat_sub(a567, a568)
    }, v1)?;
    Ok(h566)
}

pub fn f74(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: bool = f24(v0.clone(), v1, 46u8);
    let v3: bool = f24(v0.clone(), {
        let a569 = v1;
        let a570 = 1u64;
        nat_add(a569, a570)?
    }, 46u8);
    let v4: bool = f42(v0.clone(), {
        let a571 = v1;
        let a572 = 1u64;
        nat_add(a571, a572)?
    }, 48u8, 57u8);
    let v5: bool = f24(v0.clone(), v1, 101u8);
    let v6: bool = f24(v0.clone(), v1, 69u8);
    let v7: Result<u64, Bytes> = f89(v0.clone(), {
        let a573 = v1;
        let a574 = 1u64;
        nat_add(a573, a574)?
    }, true)?;
    let v8: Result<u64, Bytes> = f89(v0.clone(), v1, false)?;
    let v11: Result<u64, Bytes> = {
        let m577 = f72(v0.clone(), {
            let a575 = v1;
            let a576 = 1u64;
            nat_add(a575, a576)?
        }, 10u64, false)?;
        match m577 {
            Ok(v10) => {
                f89(v0, v10, false)?
            }
            Err(v9) => {
                Err::<u64, Bytes>(v9)
            }
        }
    };
    if if v2 {
        v3
    } else {
        false
    } {
        Ok(Ok::<u64, Bytes>(v1))
    } else {
        if v2 {
            if v4 {
                Ok(v11)
            } else {
                Ok(v7)
            }
        } else {
            if if v5 {
                true
            } else {
                v6
            } {
                Ok(v8)
            } else {
                Ok(Ok::<u64, Bytes>(v1))
            }
        }
    }
}

pub fn f75(v0: u8) -> bool {
    let v1: bool = f41(v0, 97u8, 122u8);
    let v2: bool = f41(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn f76(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: u8 = f23(v0.clone(), {
        let a578 = v1;
        let a579 = 1u64;
        nat_add(a578, a579)?
    });
    let v4: u8 = f23(v0.clone(), {
        let a580 = v1;
        let a581 = 2u64;
        nat_add(a580, a581)?
    });
    let v5: u8 = f23(v0, {
        let a582 = v1;
        let a583 = 3u64;
        nat_add(a582, a583)?
    });
    let a584 = v2;
    let a585 = 206u8;
    if equal(a584, a585) {
        if f41(v3, 145u8, 169u8) {
            let a586 = v3;
            let a587 = 160u8;
            if equal(a586, a587) {
                Ok(0u64)
            } else {
                let a588 = v3;
                let a589 = 163u8;
                if equal(a588, a589) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            }
        } else {
            if f41(v3, 177u8, 191u8) {
                let a590 = v3;
                let a591 = 187u8;
                if equal(a590, a591) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            } else {
                Ok(0u64)
            }
        }
    } else {
        let a592 = v2;
        let a593 = 207u8;
        if equal(a592, a593) {
            if f41(v3, 128u8, 187u8) {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            let a594 = v2;
            let a595 = 225u8;
            if equal(a594, a595) {
                if f41(v3, 188u8, 191u8) {
                    if f41(v4, 128u8, 191u8) {
                        let a596 = v3;
                        let a597 = 191u8;
                        if equal(a596, a597) {
                            let a598 = v4;
                            let a599 = 191u8;
                            if equal(a598, a599) {
                                Ok(0u64)
                            } else {
                                Ok(3u64)
                            }
                        } else {
                            Ok(3u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            } else {
                let a600 = v2;
                let a601 = 226u8;
                if equal(a600, a601) {
                    let a602 = v3;
                    let a603 = 132u8;
                    if equal(a602, a603) {
                        if f41(v4, 128u8, 191u8) {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        let a604 = v3;
                        let a605 = 133u8;
                        if equal(a604, a605) {
                            if f41(v4, 128u8, 143u8) {
                                Ok(3u64)
                            } else {
                                Ok(0u64)
                            }
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    let a606 = v2;
                    let a607 = 240u8;
                    if equal(a606, a607) {
                        let a608 = v3;
                        let a609 = 157u8;
                        if equal(a608, a609) {
                            let a610 = v4;
                            let a611 = 146u8;
                            if equal(a610, a611) {
                                if f41(v5, 156u8, 191u8) {
                                    Ok(4u64)
                                } else {
                                    Ok(0u64)
                                }
                            } else {
                                if f41(v4, 147u8, 149u8) {
                                    if f41(v5, 128u8, 191u8) {
                                        Ok(4u64)
                                    } else {
                                        Ok(0u64)
                                    }
                                } else {
                                    let a612 = v4;
                                    let a613 = 150u8;
                                    if equal(a612, a613) {
                                        if f41(v5, 128u8, 159u8) {
                                            Ok(4u64)
                                        } else {
                                            Ok(0u64)
                                        }
                                    } else {
                                        Ok(0u64)
                                    }
                                }
                            }
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
    }
}

pub fn f77(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f90(v0, v1)
}

pub fn f78(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f24(v0.clone(), v1, 194u8);
    let v3: bool = f24(v0, {
        let a614 = v1;
        let a615 = 1u64;
        nat_add(a614, a615)?
    }, 171u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}

pub fn f79(v0: Bytes, v1: u64) -> R<Result<(List<Adt3>, u64), Bytes>> {
    let v2: u64 = {
        let a616 = v0.clone();
        length_bytes(a616)
    };
    let v3: Adt7 = {
        let (h617, _) = f91(Fn15::F92(v0), {
            let a620 = {
                let a618 = v2;
                let a619 = v1;
                nat_sub(a618, a619)
            };
            let a621 = 1u64;
            nat_add(a620, a621)?
        }, Adt7::C0(v1, List::<Adt3>::nil(), false, None::<Bytes>))?;
        h617
    };
    let m623 = match v3.clone() {
        Adt7::C0(_, _, _, h622) => {
            h622
        }
    };
    match m623 {
        None => {
            let v5: List<Adt3> = f93(match v3.clone() {
                Adt7::C0(_, h624, _, _) => {
                    h624
                }
            });
            Ok(Ok::<(List<Adt3>, u64), Bytes>((v5, match v3 {
                Adt7::C0(h625, _, _, _) => {
                    h625
                }
            })))
        }
        Some(v4) => {
            Ok(Err::<(List<Adt3>, u64), Bytes>(v4))
        }
    }
}

pub fn f80(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a626 = v0.clone();
        length_bytes(a626)
    };
    let v3: u64 = {
        let (h627, _) = f44(Fn7::F94(v0), {
            let a628 = v2;
            let a629 = v1;
            nat_sub(a628, a629)
        }, v1)?;
        h627
    };
    let a630 = v3;
    let a631 = v1;
    Ok(nat_sub(a630, a631))
}

pub fn f81(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    let v4: u64 = {
        let a632 = v0.clone();
        length_bytes(a632)
    };
    let (h633, _) = f69(Fn12::F95(v3, v4, v0), {
        let a636 = {
            let a634 = v4;
            let a635 = v1;
            nat_sub(a634, a635)
        };
        let a637 = 1u64;
        nat_add(a636, a637)?
    }, Adt6::C0(v1, v2, false, false))?;
    Ok(h633)
}

pub fn f82(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a638 = v0.clone();
        length_bytes(a638)
    };
    let v3: u64 = {
        let a639 = v1.clone();
        length_bytes(a639)
    };
    let a640 = v3;
    let a641 = v2;
    if nat_le(a640, a641) {
        let m647 = {
            let a644 = v0;
            let a645 = {
                let a642 = v2;
                let a643 = v3;
                nat_sub(a642, a643)
            };
            let a646 = v3;
            slice_bytes(a644, a645, a646)
        };
        match m647 {
            None => {
                false
            }
            Some(v4) => {
                let a648 = v4;
                let a649 = v1;
                equal(a648, a649)
            }
        }
    } else {
        false
    }
}

pub fn f83(v0: Bytes, v1: u64, v2: Adt6) -> R<Option<Adt6>> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h650, _, _, _) => {
            h650
        }
    };
    let v4: u64 = match v2.clone() {
        Adt6::C0(_, h651, _, _) => {
            h651
        }
    };
    if match v2.clone() {
        Adt6::C0(_, _, h652, _) => {
            h652
        }
    } {
        Ok(None::<Adt6>)
    } else {
        if match v2 {
            Adt6::C0(_, _, _, h653) => {
                h653
            }
        } {
            Ok(None::<Adt6>)
        } else {
            let a654 = v3;
            let a655 = v1;
            if nat_lt(a654, a655) {
                let v5: u8 = f23(v0.clone(), v3);
                let a656 = v5;
                let a657 = 92u8;
                if equal(a656, a657) {
                    let a660 = {
                        let a658 = v3;
                        let a659 = 1u64;
                        nat_add(a658, a659)?
                    };
                    let a661 = v1;
                    if nat_lt(a660, a661) {
                        let v6: u64 = f60(v0, {
                            let a662 = v3;
                            let a663 = 1u64;
                            nat_add(a662, a663)?
                        });
                        Ok(Some(Adt6::C0({
                            let a666 = {
                                let a664 = v3;
                                let a665 = 1u64;
                                nat_add(a664, a665)?
                            };
                            let a667 = v6;
                            nat_add(a666, a667)?
                        }, v4, false, false)))
                    } else {
                        Ok(Some(Adt6::C0(v1, v4, false, true)))
                    }
                } else {
                    let a668 = v5;
                    let a669 = 34u8;
                    if equal(a668, a669) {
                        Ok(Some(Adt6::C0({
                            let a670 = v3;
                            let a671 = 1u64;
                            nat_add(a670, a671)?
                        }, v4, true, false)))
                    } else {
                        let a672 = v5;
                        let a673 = 10u8;
                        if equal(a672, a673) {
                            Ok(Some(Adt6::C0({
                                let a674 = v3;
                                let a675 = 1u64;
                                nat_add(a674, a675)?
                            }, {
                                let a676 = v4;
                                let a677 = 1u64;
                                nat_add(a676, a677)?
                            }, false, false)))
                        } else {
                            let v7: u64 = f60(v0, v3);
                            Ok(Some(Adt6::C0({
                                let a678 = v3;
                                let a679 = v7;
                                nat_add(a678, a679)?
                            }, v4, false, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt6::C0(v3, v4, false, true)))
            }
        }
    }
}

pub fn f84(v0: Fn13, p684: u64, p685: (u64, (u64, bool))) -> R<((u64, (u64, bool)), bool)> {
    let m683 = (p684, p685);
    {
        let mut m683 = m683;
        loop {
            let step = {
                let (v1, v2) = m683;
                let m680 = v1;
                if m680 == 0 {
                    Err::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m680 - 1;
                    let m682 = {
                        let c681: Fn13 = v0.clone();
                        c681.apply(v2)?
                    };
                    match m682 {
                        None => {
                            Err::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: u64, v1: Bytes, v2: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    f96(v1.clone(), {
        let a686 = v1;
        length_bytes(a686)
    }, v0, v2)
}

pub fn f86(v0: Fn14, p691: u64, p692: (u64, (bool, bool))) -> R<((u64, (bool, bool)), bool)> {
    let m690 = (p691, p692);
    {
        let mut m690 = m690;
        loop {
            let step = {
                let (v1, v2) = m690;
                let m687 = v1;
                if m687 == 0 {
                    Err::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m687 - 1;
                    let m689 = {
                        let c688: Fn14 = v0.clone();
                        c688.apply(v2)?
                    };
                    match m689 {
                        None => {
                            Err::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: u64, v1: u64, v2: Bytes, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    f97(v2, v0, v1, v3)
}

pub fn f88(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f98(v0, v1)
}

pub fn f89(v0: Bytes, v1: u64, v2: bool) -> R<Result<u64, Bytes>> {
    let v3: bool = f24(v0.clone(), v1, 101u8);
    let v4: bool = f24(v0.clone(), v1, 69u8);
    let v5: Result<u64, Bytes> = f99(v0.clone(), v1)?;
    let v6: bool = f58(v0, v1)?;
    if if v3 {
        true
    } else {
        v4
    } {
        Ok(v5)
    } else {
        if if v2 {
            v6
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>({
                let a693 = Str::lit("an identifier follows a decimal point");
                utf8_encode(a693)
            }))
        } else {
            Ok(Ok::<u64, Bytes>(v1))
        }
    }
}

pub fn f90(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a694 = 0u64;
    let a695 = v2;
    if nat_lt(a694, a695) {
        Ok(Some({
            let a696 = v1;
            let a697 = v2;
            nat_add(a696, a697)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f91(v0: Fn15, p702: u64, p703: Adt7) -> R<(Adt7, bool)> {
    let m701 = (p702, p703);
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v1, v2) = m701;
                let m698 = v1;
                if m698 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m698 - 1;
                    let m700 = {
                        let c699: Fn15 = v0.clone();
                        c699.apply(v2.clone())?
                    };
                    match m700 {
                        None => {
                            Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt7), R<(Adt7, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: Adt7) -> R<Option<Adt7>> {
    f101(v0, v1)
}

pub fn f93(v0: List<Adt3>) -> List<Adt3> {
    f102(Fn16::F103, List::<Adt3>::nil(), v0)
}

pub fn f94(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f104(v0, v1)
}

pub fn f95(v0: u64, v1: u64, v2: Bytes, v3: Adt6) -> R<Option<Adt6>> {
    f105(v2, v1, v0, v3)
}

pub fn f96(v0: Bytes, v1: u64, v2: u64, v3: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    let v4: u64 = {
        let (h704, _) = v3;
        h704
    };
    let v5: u64 = {
        let (h705, _) = {
            let (_, h706) = v3;
            h706
        };
        h705
    };
    let v6: bool = {
        let (_, h707) = {
            let (_, h708) = v3;
            h708
        };
        h707
    };
    if v6 {
        let a709 = v5;
        let a710 = v2;
        if nat_lt(a709, a710) {
            let a711 = v4;
            let a712 = v1;
            if nat_lt(a711, a712) {
                let v7: u64 = f60(v0, v4);
                Ok(Some(({
                    let a713 = v4;
                    let a714 = v7;
                    nat_add(a713, a714)?
                }, ({
                    let a715 = v5;
                    let a716 = 1u64;
                    nat_add(a715, a716)?
                }, true))))
            } else {
                Ok(Some((v4, ({
                    let a717 = v5;
                    let a718 = 1u64;
                    nat_add(a717, a718)?
                }, false))))
            }
        } else {
            Ok(None::<(u64, (u64, bool))>)
        }
    } else {
        Ok(None::<(u64, (u64, bool))>)
    }
}

pub fn f97(v0: Bytes, v1: u64, v2: u64, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    let v4: u64 = {
        let (h719, _) = v3;
        h719
    };
    let v5: bool = {
        let (h720, _) = {
            let (_, h721) = v3;
            h721
        };
        h720
    };
    let v6: bool = {
        let (_, h722) = {
            let (_, h723) = v3;
            h723
        };
        h722
    };
    if v6 {
        Ok(None::<(u64, (bool, bool))>)
    } else {
        let a724 = v4;
        let a725 = v1;
        if nat_lt(a724, a725) {
            let v7: u8 = f23(v0, v4);
            let a726 = v7;
            let a727 = 95u8;
            if equal(a726, a727) {
                Ok(Some(({
                    let a728 = v4;
                    let a729 = 1u64;
                    nat_add(a728, a729)?
                }, (true, false))))
            } else {
                let v8: bool = f106(v7, v2);
                if v8 {
                    Ok(Some(({
                        let a730 = v4;
                        let a731 = 1u64;
                        nat_add(a730, a731)?
                    }, (false, false))))
                } else {
                    Ok(Some((v4, (v5, true))))
                }
            }
        } else {
            Ok(Some((v4, (v5, true))))
        }
    }
}

pub fn f98(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f42(v0, v1, 48u8, 57u8);
    if v2 {
        Ok(Some({
            let a732 = v1;
            let a733 = 1u64;
            nat_add(a732, a733)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f99(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = {
        let a734 = v1;
        let a735 = 1u64;
        nat_add(a734, a735)?
    };
    let v3: bool = f24(v0.clone(), v2, 43u8);
    let v4: bool = f24(v0.clone(), v2, 45u8);
    let v5: u64 = if v3 {
        let a736 = v2;
        let a737 = 1u64;
        nat_add(a736, a737)?
    } else {
        if v4 {
            let a738 = v2;
            let a739 = 1u64;
            nat_add(a738, a739)?
        } else {
            v2
        }
    };
    let v6: bool = f42(v0.clone(), v5, 48u8, 57u8);
    let v7: Result<u64, Bytes> = f72(v0, v5, 10u64, false)?;
    if v6 {
        Ok(v7)
    } else {
        Ok(Err::<u64, Bytes>({
            let a740 = Str::lit("a scientific literal has no exponent digits");
            utf8_encode(a740)
        }))
    }
}

pub fn f100(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: bool = f75(v2);
    let v4: bool = f53(v2);
    let v5: u64 = f76(v0.clone(), v1)?;
    let v6: u64 = f107(v0.clone(), v1)?;
    let a742 = v1;
    let a743 = {
        let a741 = v0;
        length_bytes(a741)
    };
    if nat_lt(a742, a743) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                let a744 = v2;
                let a745 = 95u8;
                if equal(a744, a745) {
                    Ok(1u64)
                } else {
                    let a746 = v2;
                    let a747 = 39u8;
                    if equal(a746, a747) {
                        Ok(1u64)
                    } else {
                        let a748 = v2;
                        let a749 = 33u8;
                        if equal(a748, a749) {
                            Ok(1u64)
                        } else {
                            let a750 = v2;
                            let a751 = 63u8;
                            if equal(a750, a751) {
                                Ok(1u64)
                            } else {
                                let a752 = 0u64;
                                let a753 = v5;
                                if nat_lt(a752, a753) {
                                    Ok(v5)
                                } else {
                                    Ok(v6)
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f101(v0: Bytes, v1: Adt7) -> R<Option<Adt7>> {
    if match v1.clone() {
        Adt7::C0(_, _, h754, _) => {
            h754
        }
    } {
        Ok(None::<Adt7>)
    } else {
        let m765 = if f78(v0.clone(), match v1.clone() {
            Adt7::C0(h755, _, _, _) => {
                h755
            }
        })? {
            f108(v0.clone(), match v1.clone() {
                Adt7::C0(h756, _, _, _) => {
                    h756
                }
            })?
        } else {
            let v2: u64 = f56(v0.clone(), {
                let a759 = match v1.clone() {
                    Adt7::C0(h757, _, _, _) => {
                        h757
                    }
                };
                let a760 = f55(v0.clone(), match v1.clone() {
                    Adt7::C0(h758, _, _, _) => {
                        h758
                    }
                })?;
                nat_add(a759, a760)?
            })?;
            Ok::<(Adt3, u64), Bytes>((Adt3::C0(f57(v0.clone(), match v1.clone() {
                Adt7::C0(h761, _, _, _) => {
                    h761
                }
            }, {
                let a763 = v2;
                let a764 = match v1.clone() {
                    Adt7::C0(h762, _, _, _) => {
                        h762
                    }
                };
                nat_sub(a763, a764)
            }), false), v2))
        };
        match m765 {
            Ok(v4) => {
                let v5: bool = f24(v0.clone(), {
                    let (_, h768) = v4.clone();
                    h768
                }, 46u8);
                let v6: bool = f58(v0, {
                    let a770 = {
                        let (_, h769) = v4.clone();
                        h769
                    };
                    let a771 = 1u64;
                    nat_add(a770, a771)?
                })?;
                if if v5 {
                    v6
                } else {
                    false
                } {
                    Ok(Some(Adt7::C0({
                        let a773 = {
                            let (_, h772) = v4;
                            h772
                        };
                        let a774 = 1u64;
                        nat_add(a773, a774)?
                    }, List::cons({
                        let (h775, _) = v4;
                        h775
                    }, match v1 {
                        Adt7::C0(_, h776, _, _) => {
                            h776
                        }
                    }), false, None::<Bytes>)))
                } else {
                    Ok(Some(Adt7::C0({
                        let (_, h777) = v4;
                        h777
                    }, List::cons({
                        let (h778, _) = v4;
                        h778
                    }, match v1 {
                        Adt7::C0(_, h779, _, _) => {
                            h779
                        }
                    }), true, None::<Bytes>)))
                }
            }
            Err(v3) => {
                Ok(Some(Adt7::C0(match v1.clone() {
                    Adt7::C0(h766, _, _, _) => {
                        h766
                    }
                }, match v1 {
                    Adt7::C0(_, h767, _, _) => {
                        h767
                    }
                }, true, Some(v3))))
            }
        }
    }
}

pub fn f102(v0: Fn16, p783: List<Adt3>, p784: List<Adt3>) -> List<Adt3> {
    let m782 = (p783, p784);
    {
        let mut m782 = m782;
        loop {
            let step = {
                let (v1, v2) = m782;
                let m780 = v2;
                match m780.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c781: Fn16 = v0.clone();
                            c781.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f24(v0, v1, 35u8);
    if v2 {
        Ok(Some({
            let a785 = v1;
            let a786 = 1u64;
            nat_add(a785, a786)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f105(v0: Bytes, v1: u64, v2: u64, v3: Adt6) -> R<Option<Adt6>> {
    let v4: u64 = match v3.clone() {
        Adt6::C0(h787, _, _, _) => {
            h787
        }
    };
    let v5: u64 = match v3.clone() {
        Adt6::C0(_, h788, _, _) => {
            h788
        }
    };
    if match v3.clone() {
        Adt6::C0(_, _, h789, _) => {
            h789
        }
    } {
        Ok(None::<Adt6>)
    } else {
        if match v3 {
            Adt6::C0(_, _, _, h790) => {
                h790
            }
        } {
            Ok(None::<Adt6>)
        } else {
            let a791 = v4;
            let a792 = v1;
            if nat_lt(a791, a792) {
                let v6: u8 = f23(v0.clone(), v4);
                let a793 = v6;
                let a794 = 34u8;
                if equal(a793, a794) {
                    let v7: bool = f109(v0, {
                        let a795 = v4;
                        let a796 = 1u64;
                        nat_add(a795, a796)?
                    }, v2)?;
                    if v7 {
                        Ok(Some(Adt6::C0({
                            let a799 = {
                                let a797 = v4;
                                let a798 = 1u64;
                                nat_add(a797, a798)?
                            };
                            let a800 = v2;
                            nat_add(a799, a800)?
                        }, v5, true, false)))
                    } else {
                        Ok(Some(Adt6::C0({
                            let a801 = v4;
                            let a802 = 1u64;
                            nat_add(a801, a802)?
                        }, v5, false, false)))
                    }
                } else {
                    let a803 = v6;
                    let a804 = 10u8;
                    if equal(a803, a804) {
                        Ok(Some(Adt6::C0({
                            let a805 = v4;
                            let a806 = 1u64;
                            nat_add(a805, a806)?
                        }, {
                            let a807 = v5;
                            let a808 = 1u64;
                            nat_add(a807, a808)?
                        }, false, false)))
                    } else {
                        let v8: u64 = f60(v0, v4);
                        Ok(Some(Adt6::C0({
                            let a809 = v4;
                            let a810 = v8;
                            nat_add(a809, a810)?
                        }, v5, false, false)))
                    }
                }
            } else {
                Ok(Some(Adt6::C0(v4, v5, false, true)))
            }
        }
    }
}

pub fn f106(v0: u8, v1: u64) -> bool {
    let v2: bool = f41(v0, 48u8, 49u8);
    let v3: bool = f41(v0, 48u8, 55u8);
    let v4: bool = f41(v0, 48u8, 57u8);
    let v5: bool = f41(v0, 97u8, 102u8);
    let v6: bool = f41(v0, 65u8, 70u8);
    let a811 = v1;
    let a812 = 2u64;
    if nat_eq(a811, a812) {
        v2
    } else {
        let a813 = v1;
        let a814 = 8u64;
        if nat_eq(a813, a814) {
            v3
        } else {
            let a815 = v1;
            let a816 = 16u64;
            if nat_eq(a815, a816) {
                if v4 {
                    true
                } else {
                    if v5 {
                        true
                    } else {
                        v6
                    }
                }
            } else {
                v4
            }
        }
    }
}

pub fn f107(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: u8 = f23(v0.clone(), {
        let a817 = v1;
        let a818 = 1u64;
        nat_add(a817, a818)?
    });
    let v4: u8 = f23(v0, {
        let a819 = v1;
        let a820 = 2u64;
        nat_add(a819, a820)?
    });
    let a821 = v2;
    let a822 = 225u8;
    if equal(a821, a822) {
        let a823 = v3;
        let a824 = 181u8;
        if equal(a823, a824) {
            if f41(v4, 162u8, 170u8) {
                Ok(3u64)
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a825 = v2;
        let a826 = 226u8;
        if equal(a825, a826) {
            let a827 = v3;
            let a828 = 130u8;
            if equal(a827, a828) {
                if f41(v4, 128u8, 137u8) {
                    Ok(3u64)
                } else {
                    if f41(v4, 144u8, 156u8) {
                        Ok(3u64)
                    } else {
                        Ok(0u64)
                    }
                }
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    }
}

pub fn f108(v0: Bytes, v1: u64) -> R<Result<(Adt3, u64), Bytes>> {
    let v2: u64 = {
        let a829 = v1;
        let a830 = 2u64;
        nat_add(a829, a830)?
    };
    let v3: Adt8 = {
        let (h831, _) = f110(Fn17::F111(v0.clone()), {
            let a835 = {
                let a833 = {
                    let a832 = v0.clone();
                    length_bytes(a832)
                };
                let a834 = v2;
                nat_sub(a833, a834)
            };
            let a836 = 1u64;
            nat_add(a835, a836)?
        }, Adt8::C0(v2, false, false, true))?;
        h831
    };
    let v4: Bytes = f57(v0, v2, {
        let a838 = match v3.clone() {
            Adt8::C0(h837, _, _, _) => {
                h837
            }
        };
        let a839 = v2;
        nat_sub(a838, a839)
    });
    if match v3.clone() {
        Adt8::C0(_, h840, _, _) => {
            h840
        }
    } {
        let a842 = 0u64;
        let a843 = {
            let a841 = v4.clone();
            length_bytes(a841)
        };
        if if nat_lt(a842, a843) {
            match v3.clone() {
                Adt8::C0(_, _, _, h844) => {
                    h844
                }
            }
        } else {
            false
        } {
            Ok(Ok::<(Adt3, u64), Bytes>((Adt3::C0(v4, true), {
                let a846 = match v3 {
                    Adt8::C0(h845, _, _, _) => {
                        h845
                    }
                };
                let a847 = 2u64;
                nat_add(a846, a847)?
            })))
        } else {
            Ok(Err::<(Adt3, u64), Bytes>({
                let a852 = {
                    let a849 = {
                        let a848 = Str::lit("the quoted name `");
                        utf8_encode(a848)
                    };
                    let a850 = v4;
                    append_bytes(a849, a850)
                };
                let a853 = {
                    let a851 = Str::lit("` is not a name");
                    utf8_encode(a851)
                };
                append_bytes(a852, a853)
            }))
        }
    } else {
        Ok(Err::<(Adt3, u64), Bytes>({
            let a854 = Str::lit("a quoted name is not closed");
            utf8_encode(a854)
        }))
    }
}

pub fn f109(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let (_, h855) = {
        let (h856, _) = f112(Fn18::F113(v2, v1, v0), v2, (0u64, true))?;
        h856
    };
    Ok(h855)
}

pub fn f110(v0: Fn17, p861: u64, p862: Adt8) -> R<(Adt8, bool)> {
    let m860 = (p861, p862);
    {
        let mut m860 = m860;
        loop {
            let step = {
                let (v1, v2) = m860;
                let m857 = v1;
                if m857 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m857 - 1;
                    let m859 = {
                        let c858: Fn17 = v0.clone();
                        c858.apply(v2.clone())?
                    };
                    match m859 {
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
                Ok(next) => { m860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: Bytes, v1: Adt8) -> R<Option<Adt8>> {
    f114(v0.clone(), {
        let a863 = v0;
        length_bytes(a863)
    }, v1)
}

pub fn f112(v0: Fn18, p868: u64, p869: (u64, bool)) -> R<((u64, bool), bool)> {
    let m867 = (p868, p869);
    {
        let mut m867 = m867;
        loop {
            let step = {
                let (v1, v2) = m867;
                let m864 = v1;
                if m864 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m864 - 1;
                    let m866 = {
                        let c865: Fn18 = v0.clone();
                        c865.apply(v2)?
                    };
                    match m866 {
                        None => {
                            Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, bool)), R<((u64, bool), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m867 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: u64, v1: u64, v2: Bytes, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    f115(v2, v1, v0, v3)
}

pub fn f114(v0: Bytes, v1: u64, v2: Adt8) -> R<Option<Adt8>> {
    let v3: u64 = match v2.clone() {
        Adt8::C0(h870, _, _, _) => {
            h870
        }
    };
    let v4: bool = match v2.clone() {
        Adt8::C0(_, _, _, h871) => {
            h871
        }
    };
    if match v2.clone() {
        Adt8::C0(_, h872, _, _) => {
            h872
        }
    } {
        Ok(None::<Adt8>)
    } else {
        if match v2 {
            Adt8::C0(_, _, h873, _) => {
                h873
            }
        } {
            Ok(None::<Adt8>)
        } else {
            let a874 = v3;
            let a875 = v1;
            if nat_lt(a874, a875) {
                let v5: bool = f24(v0.clone(), v3, 10u8);
                if v5 {
                    Ok(Some(Adt8::C0(v3, false, true, v4)))
                } else {
                    let v6: bool = f116(v0.clone(), v3)?;
                    if v6 {
                        Ok(Some(Adt8::C0(v3, true, false, v4)))
                    } else {
                        let v7: u64 = f100(v0.clone(), v3)?;
                        let a876 = 0u64;
                        let a877 = v7;
                        if nat_lt(a876, a877) {
                            Ok(Some(Adt8::C0({
                                let a878 = v3;
                                let a879 = v7;
                                nat_add(a878, a879)?
                            }, false, false, v4)))
                        } else {
                            let v8: bool = f24(v0.clone(), v3, 46u8);
                            if v8 {
                                Ok(Some(Adt8::C0({
                                    let a880 = v3;
                                    let a881 = 1u64;
                                    nat_add(a880, a881)?
                                }, false, false, v4)))
                            } else {
                                let v9: u64 = f60(v0, v3);
                                Ok(Some(Adt8::C0({
                                    let a882 = v3;
                                    let a883 = v9;
                                    nat_add(a882, a883)?
                                }, false, false, false)))
                            }
                        }
                    }
                }
            } else {
                Ok(Some(Adt8::C0(v3, false, true, v4)))
            }
        }
    }
}

pub fn f115(v0: Bytes, v1: u64, v2: u64, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v4: u64 = {
        let (h884, _) = v3;
        h884
    };
    let v5: bool = {
        let (_, h885) = v3;
        h885
    };
    if v5 {
        let a886 = v4;
        let a887 = v2;
        if nat_lt(a886, a887) {
            let v6: bool = f24(v0, {
                let a888 = v1;
                let a889 = v4;
                nat_add(a888, a889)?
            }, 35u8);
            Ok(Some(({
                let a890 = v4;
                let a891 = 1u64;
                nat_add(a890, a891)?
            }, v6)))
        } else {
            Ok(None::<(u64, bool)>)
        }
    } else {
        Ok(None::<(u64, bool)>)
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f24(v0.clone(), v1, 194u8);
    let v3: bool = f24(v0, {
        let a892 = v1;
        let a893 = 1u64;
        nat_add(a892, a893)?
    }, 187u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}
