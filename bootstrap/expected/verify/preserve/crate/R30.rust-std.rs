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
    C0(bool, List<Adt1>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Bytes>, List<Bytes>),
    C1(Bytes, List<Bytes>),
    C2(Bytes, List<Bytes>, Adt2),
    C3(Bytes, Adt2, Adt3),
    C4(Bytes, List<Adt5>),
    C5(List<Adt7>, List<Bytes>, Adt2, List<Adt8>),
    C6(Adt8),
    C7(Bytes, List<Bytes>, List<Bytes>, Adt2),
    C8(Bytes, List<Bytes>, List<Bytes>, List<Bytes>, Adt2),
    C9(Bytes, List<Bytes>, List<Bytes>, List<Adt9>),
    C10(Bytes),
    C11(Bytes, List<Adt1>),
    C12(Bytes, List<Bytes>),
    C13(Bytes, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(u64),
    C7(std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>),
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(Bytes, std::rc::Rc<Adt2>),
    C11,
    C12,
    C13,
    C14,
    C15(std::rc::Rc<Adt2>),
    C16,
    C17,
    C18,
    C19(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes),
    C1(u64),
    C2(List<Bytes>),
    C3(List<Bytes>, std::rc::Rc<Adt3>),
    C4(std::rc::Rc<Adt3>, Bytes, std::rc::Rc<Adt3>),
    C5(std::rc::Rc<Adt3>, u64),
    C6(std::rc::Rc<Adt3>, Bytes),
    C7(std::rc::Rc<Adt3>),
    C8(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C9(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C10(Adt4, Adt2, std::rc::Rc<Adt3>),
    C11(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C12(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C13(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C14(Adt4, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C15(std::rc::Rc<Adt3>),
    C16(Adt4, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C17,
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(List<Bytes>, List<Bytes>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C22(std::rc::Rc<Adt3>),
    C23(std::rc::Rc<Adt3>, Adt2),
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C25(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C26(std::rc::Rc<Adt3>),
    C27(std::rc::Rc<Adt3>),
    C28(std::rc::Rc<Adt3>, Bytes, Adt2, std::rc::Rc<Adt3>),
    C29(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C30,
    C31,
    C32(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C33(Adt4, Adt2, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C34(Adt4, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(Bytes),
    C2(Bytes),
    C3(List<Bytes>),
    C4(std::rc::Rc<Adt4>),
    C5(List<Bytes>, std::rc::Rc<Adt4>),
    C6(List<Bytes>, List<Bytes>, std::rc::Rc<Adt4>),
    C7(u64),
    C8,
    C9(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, bool, bool, List<Adt6>, Adt2),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, bool, Adt2),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt5, List<Adt7>, bool, Adt3),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes),
    C1(Bytes, Adt2),
    C2(Bytes, List<Bytes>, Adt2),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Bytes, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, List<Bytes>, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes, List<Adt9>, u64),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(bool, List<Bytes>, List<Adt10>, List<Adt11>, List<Adt12>, List<Adt13>, List<Bytes>, List<Adt13>, List<Adt13>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<Adt7>, List<Adt17>, Adt2, bool),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, bool, Adt2),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(Adt14),
    F8,
    F36(Bytes, Adt14),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt1) -> R<Option<Bytes>> {
        match self {
            Fn0::F5(k0) => f5(k0.clone(), p0, p1),
            Fn0::F8 => Ok(f8(p0, p1)),
            Fn0::F36(k0, k1) => f36(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt15, p1: Adt1) -> R<Adt15> {
        match self {
            Fn1::F7 => f7(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F11,
}

impl Fn2 {
    pub fn apply(&self, p0: Result<Adt14, Bytes>, p1: Adt1) -> R<Result<Adt14, Bytes>> {
        match self {
            Fn2::F11 => f11(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F18(List<Adt13>, List<Adt13>),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt13) -> R<Option<Bytes>> {
        match self {
            Fn3::F18(k0, k1) => f18(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F27(Adt14),
    F96(List<Adt8>, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt5) -> R<Option<Bytes>> {
        match self {
            Fn4::F27(k0) => f27(k0.clone(), p0, p1),
            Fn4::F96(k0, k1) => f96(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F31(List<Adt7>, Adt14, Adt2),
    F97(List<Adt7>, Adt14, List<Adt5>, Bytes, Adt2),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt8) -> R<Option<Bytes>> {
        match self {
            Fn5::F31(k0, k1, k2) => f31(k0.clone(), k1.clone(), k2.clone(), p0, p1),
            Fn5::F97(k0, k1, k2, k3, k4) => f97(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F34(List<Bytes>, Adt14),
}

impl Fn6 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt9) -> R<Option<Bytes>> {
        match self {
            Fn6::F34(k0, k1) => f34(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F43(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt13>, p1: Adt1) -> List<Adt13> {
        match self {
            Fn7::F43(k0) => f43(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F91,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt18, p1: Adt5) -> R<Adt18> {
        match self {
            Fn8::F91 => f91(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F94(Adt14),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt7) -> R<Option<Bytes>> {
        match self {
            Fn9::F94(k0) => f94(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F102,
    F131(Bytes),
    F150(List<Bytes>, List<Bytes>),
    F241(Bytes, Adt14),
}

impl Fn10 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn10::F102 => Ok(f102(p0, p1)),
            Fn10::F131(k0) => Ok(f131(k0.clone(), p0, p1)),
            Fn10::F150(k0, k1) => f150(k0.clone(), k1.clone(), p0, p1),
            Fn10::F241(k0, k1) => Ok(f241(k0.clone(), k1.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F111(Str),
    F112(Str),
    F152(Adt7, Adt14),
    F171(Str),
    F209(List<Bytes>, List<Bytes>),
    F210(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn11::F111(k0) => Ok(f111(k0.clone(), p0, p1)),
            Fn11::F112(k0) => f112(k0.clone(), p0, p1),
            Fn11::F152(k0, k1) => Ok(f152(k0.clone(), k1.clone(), p0, p1)),
            Fn11::F171(k0) => f171(k0.clone(), p0, p1),
            Fn11::F209(k0, k1) => f209(k0.clone(), k1.clone(), p0, p1),
            Fn11::F210(k0) => Ok(f210(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F114,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt14, p1: Adt5) -> Adt14 {
        match self {
            Fn12::F114 => f114(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F118,
    F174(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt14, p1: Adt8) -> Adt14 {
        match self {
            Fn13::F118 => f118(p0, p1),
            Fn13::F174(k0) => f174(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F121(Bytes),
    F122,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt18, p1: Adt9) -> R<Adt18> {
        match self {
            Fn14::F121(k0) => f121(k0.clone(), p0, p1),
            Fn14::F122 => f122(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F130(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: Option<Adt17>, p1: Adt17) -> Option<Adt17> {
        match self {
            Fn15::F130(k0) => f130(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F162(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn16::F162(k0) => f162(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F166(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: u64, p1: Adt13) -> R<u64> {
        match self {
            Fn17::F166(k0) => f166(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F168(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Option<u64>, p1: Adt13) -> Option<u64> {
        match self {
            Fn18::F168(k0) => f168(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F170,
}

impl Fn19 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> Bytes {
        match self {
            Fn19::F170 => f170(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F173(Bytes),
    F231(Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn20::F173(k0) => f173(k0.clone(), p0),
            Fn20::F231(k0) => f231(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F180(Bytes),
}

impl Fn21 {
    pub fn apply(&self, p0: Option<Adt11>, p1: Adt11) -> Option<Adt11> {
        match self {
            Fn21::F180(k0) => f180(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F182(Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn22::F182(k0) => f182(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F184(Bytes),
    F234(Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: Option<Adt13>, p1: Adt13) -> Option<Adt13> {
        match self {
            Fn23::F184(k0) => f184(k0.clone(), p0, p1),
            Fn23::F234(k0) => f234(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F196(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: List<u64>, p1: Adt13) -> List<u64> {
        match self {
            Fn24::F196(k0) => f196(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F198(u64),
}

impl Fn25 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn25::F198(k0) => f198(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F204,
}

impl Fn26 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn26::F204 => f204(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F213(bool, u64, Adt14),
}

impl Fn27 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt6) -> R<Option<Bytes>> {
        match self {
            Fn27::F213(k0, k1, k2) => f213(*k0, *k1, k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F215(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: Option<Adt10>, p1: Adt10) -> Option<Adt10> {
        match self {
            Fn28::F215(k0) => f215(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F217(Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: u64, p1: Adt8) -> R<u64> {
        match self {
            Fn29::F217(k0) => f217(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F219(Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: Option<Adt8>, p1: Adt8) -> Option<Adt8> {
        match self {
            Fn30::F219(k0) => f219(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F221,
}

impl Fn31 {
    pub fn apply(&self, p0: Bytes, p1: Adt6) -> R<Bytes> {
        match self {
            Fn31::F221 => f221(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F224(Bytes),
    F248(Bytes),
}

impl Fn32 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn32::F224(k0) => f224(k0.clone(), p0, p1),
            Fn32::F248(k0) => f248(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F226(Bytes),
}

impl Fn33 {
    pub fn apply(&self, p0: Result<List<Adt17>, Bytes>, p1: Adt6) -> Result<List<Adt17>, Bytes> {
        match self {
            Fn33::F226(k0) => f226(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F229,
}

impl Fn34 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn34::F229 => f229(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F233(Bytes),
}

impl Fn35 {
    pub fn apply(&self, p0: Option<Adt9>, p1: Adt9) -> Option<Adt9> {
        match self {
            Fn35::F233(k0) => f233(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F244(Bytes),
}

impl Fn36 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn36::F244(k0) => f244(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<(), Bytes>> {
    let m2 = f1(match v0.clone() {
        Adt0::C0(_, h1) => {
            h1
        }
    })?;
    match m2 {
        None => {
            let m4 = f2(match v0.clone() {
                Adt0::C0(_, h3) => {
                    h3
                }
            })?;
            match m4 {
                None => {
                    let m7 = f3(Adt14::C0(match v0.clone() {
                        Adt0::C0(h5, _) => {
                            h5
                        }
                    }, List::<Bytes>::nil(), List::<Adt10>::nil(), List::<Adt11>::nil(), List::<Adt12>::nil(), List::<Adt13>::nil(), List::<Bytes>::nil(), List::<Adt13>::nil(), List::<Adt13>::nil(), List::<Bytes>::nil()), match v0.clone() {
                        Adt0::C0(_, h6) => {
                            h6
                        }
                    })?;
                    match m7 {
                        Ok(v4) => {
                            let m9 = f4(Fn0::F5(v4), None::<Bytes>, match v0 {
                                Adt0::C0(_, h8) => {
                                    h8
                                }
                            })?;
                            match m9 {
                                None => {
                                    Ok(Ok::<(), Bytes>(()))
                                }
                                Some(v5) => {
                                    Ok(Err::<(), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(), Bytes>(v3))
                        }
                    }
                }
                Some(v2) => {
                    Ok(Err::<(), Bytes>(v2))
                }
            }
        }
        Some(v1) => {
            Ok(Err::<(), Bytes>(v1))
        }
    }
}

pub fn f1(v0: List<Adt1>) -> R<Option<Bytes>> {
    let m11 = f6(Fn1::F7, Adt15::C0(false, None::<Bytes>), v0)?;
    match m11 {
        Adt15::C0(_, h10) => {
            Ok(h10)
        }
    }
}

pub fn f2(v0: List<Adt1>) -> R<Option<Bytes>> {
    let m12 = f4(Fn0::F8, None::<Bytes>, v0.clone())?;
    match m12 {
        None => {
            f9(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f3(v0: Adt14, v1: List<Adt1>) -> R<Result<Adt14, Bytes>> {
    f10(Fn2::F11, Ok::<Adt14, Bytes>(v0), v1)
}

pub fn f4(v0: Fn0, p16: Option<Bytes>, p17: List<Adt1>) -> R<Option<Bytes>> {
    let m15 = (p16, p17);
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, v2) = m15;
                let m13 = v2;
                match m13.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt1>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt1>), R<Option<Bytes>>>(({
                            let c14: Fn0 = v0.clone();
                            c14.apply(v1, v3)?
                        }, v4))
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

pub fn f5(v0: Adt14, v1: Option<Bytes>, v2: Adt1) -> R<Option<Bytes>> {
    let m18 = v1;
    match m18 {
        None => {
            f12(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f6(v0: Fn1, p22: Adt15, p23: List<Adt1>) -> R<Adt15> {
    let m21 = (p22, p23);
    {
        let mut m21 = m21;
        loop {
            let step = {
                let (v1, v2) = m21;
                let m19 = v2;
                match m19.uncons() {
                    None => {
                        Err::<(Adt15, List<Adt1>), R<Adt15>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt15, List<Adt1>), R<Adt15>>(({
                            let c20: Fn1 = v0.clone();
                            c20.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m21 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt15, v1: Adt1) -> R<Adt15> {
    let m25 = match v0.clone() {
        Adt15::C0(_, h24) => {
            h24
        }
    };
    match m25 {
        None => {
            f13(v0, v1)
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f8(v0: Option<Bytes>, v1: Adt1) -> Option<Bytes> {
    let m26 = v0;
    match m26 {
        None => {
            f14(v1)
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f9(v0: List<Adt1>) -> R<Option<Bytes>> {
    let v1: List<Adt13> = f15(v0.clone(), f16(Str::lit("unix")));
    let v2: List<Adt13> = f15(v0, f16(Str::lit("not_unix")));
    let m29 = f17(Fn3::F18(v2.clone(), v1.clone()), None::<Bytes>, {
        let a27 = v1;
        let a28 = v2;
        append_list(a27, a28)
    })?;
    match m29 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            Ok(Some(f19(f16(Str::lit("runtime support: cfg-pair: ")), f19(f20(v3), f16(Str::lit(" is not stated once under `unix` and once under `not_unix` with one arity"))))))
        }
    }
}

pub fn f10(v0: Fn2, p33: Result<Adt14, Bytes>, p34: List<Adt1>) -> R<Result<Adt14, Bytes>> {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v1, v2) = m32;
                let m30 = v2;
                match m30.uncons() {
                    None => {
                        Err::<(Result<Adt14, Bytes>, List<Adt1>), R<Result<Adt14, Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<Adt14, Bytes>, List<Adt1>), R<Result<Adt14, Bytes>>>(({
                            let c31: Fn2 = v0.clone();
                            c31.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m32 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Result<Adt14, Bytes>, v1: Adt1) -> R<Result<Adt14, Bytes>> {
    let m35 = v0;
    match m35 {
        Ok(v3) => {
            f21(v3, v1)
        }
        Err(v2) => {
            Ok(Err::<Adt14, Bytes>(v2))
        }
    }
}

pub fn f12(v0: Adt14, v1: Adt1) -> R<Option<Bytes>> {
    let m36 = v1;
    match m36 {
        Adt1::C0(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C2(_, v7, v8) => {
            f22(v0, v8, {
                let a37 = v7;
                length_list(a37)
            }, false)
        }
        Adt1::C3(_, v10, v11) => {
            let m38 = f22(v0.clone(), v10, 0u64, false)?;
            match m38 {
                None => {
                    f23(v0, f24(()), v11)
                }
                Some(v12) => {
                    Ok(Some(v12))
                }
            }
        }
        Adt1::C4(v13, v14) => {
            let m40 = {
                let m39 = f25(v14.clone())?;
                match m39 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v15) => {
                        Some(f19(f16(Str::lit("runtime support: trait-method: ")), f19(f20(v13), f19(f16(Str::lit(" declares ")), f19(f20(v15), f16(Str::lit(" twice")))))))
                    }
                }
            };
            match m40 {
                None => {
                    f26(Fn4::F27(v0), None::<Bytes>, v14)
                }
                Some(v16) => {
                    Ok(Some(v16))
                }
            }
        }
        Adt1::C5(v17, v18, v19, v20) => {
            let m41 = f28(v0.clone(), v17.clone())?;
            match m41 {
                None => {
                    let m43 = f22(v0.clone(), v19.clone(), {
                        let a42 = v17.clone();
                        length_list(a42)
                    }, false)?;
                    match m43 {
                        None => {
                            let m44 = v18;
                            match m44.uncons() {
                                None => {
                                    f30(Fn5::F31(v17, v0, v19), None::<Bytes>, v20)
                                }
                                Some((v23, _)) => {
                                    f29(v0, v23, v17, v19, v20)
                                }
                            }
                        }
                        Some(v22) => {
                            Ok(Some(v22))
                        }
                    }
                }
                Some(v21) => {
                    Ok(Some(v21))
                }
            }
        }
        Adt1::C6(v25) => {
            f32(v0, v25)
        }
        Adt1::C7(_, v27, _, v29) => {
            f22(v0, v29, {
                let a45 = v27;
                length_list(a45)
            }, false)
        }
        Adt1::C8(_, v31, _, _, v34) => {
            f22(v0, v34, {
                let a46 = v31;
                length_list(a46)
            }, false)
        }
        Adt1::C9(_, v36, _, v38) => {
            f33(Fn6::F34(v36, v0), None::<Bytes>, v38)
        }
        Adt1::C10(v39) => {
            f35(v39)
        }
        Adt1::C11(v40, v41) => {
            f4(Fn0::F36(v40, v0), None::<Bytes>, v41)
        }
        Adt1::C12(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C13(_, r48) => {
            let v45 = (*r48).clone();
            let m47 = f37(v45);
            match m47 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v46) => {
                    f32(v0, v46)
                }
            }
        }
    }
}

pub fn f13(v0: Adt15, v1: Adt1) -> R<Adt15> {
    let m49 = v1;
    match m49 {
        Adt1::C0(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C1(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C2(_, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C3(_, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C4(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C5(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C6(_) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C7(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C8(_, _, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C9(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C10(_) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C11(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C12(v37, v38) => {
            if match v0 {
                Adt15::C0(h50, _) => {
                    h50
                }
            } {
                Ok(Adt15::C0(true, Some(f19(f16(Str::lit("runtime support: attribute-position: `#![")), f19(v37, f16(Str::lit("]` follows an item")))))))
            } else {
                Ok(Adt15::C0(false, f38(v37, v38)?))
            }
        }
        Adt1::C13(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
    }
}

pub fn f14(v0: Adt1) -> Option<Bytes> {
    let m51 = f39(v0);
    match m51 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            if if f40({
                let (h52, _) = v1.clone();
                h52
            }, f16(Str::lit("unix"))) {
                true
            } else {
                f40({
                    let (h53, _) = v1.clone();
                    h53
                }, f16(Str::lit("not_unix")))
            } {
                let m55 = f37({
                    let (_, h54) = v1.clone();
                    h54
                });
                match m55 {
                    None => {
                        let m57 = f41({
                            let (_, h56) = v1;
                            h56
                        });
                        match m57 {
                            None => {
                                Some(f16(Str::lit("runtime support: cfg-item: a `cfg` item that is neither a `use` nor a function")))
                            }
                            Some(_) => {
                                None::<Bytes>
                            }
                        }
                    }
                    Some(_) => {
                        None::<Bytes>
                    }
                }
            } else {
                Some(f19(f16(Str::lit("runtime support: cfg-condition: ")), f19(f20({
                    let (h58, _) = v1;
                    h58
                }), f16(Str::lit(" is neither `unix` nor `not_unix`")))))
            }
        }
    }
}

pub fn f15(v0: List<Adt1>, v1: Bytes) -> List<Adt13> {
    f42(Fn7::F43(v1), List::<Adt13>::nil(), v0)
}

pub fn f16(v0: Str) -> Bytes {
    let a59 = v0;
    utf8_encode(a59)
}

pub fn f17(v0: Fn3, p63: Option<Bytes>, p64: List<Adt13>) -> R<Option<Bytes>> {
    let m62 = (p63, p64);
    {
        let mut m62 = m62;
        loop {
            let step = {
                let (v1, v2) = m62;
                let m60 = v2;
                match m60.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt13>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt13>), R<Option<Bytes>>>(({
                            let c61: Fn3 = v0.clone();
                            c61.apply(v1, v3)?
                        }, v4))
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

pub fn f18(v0: List<Adt13>, v1: List<Adt13>, v2: Option<Bytes>, v3: Adt13) -> R<Option<Bytes>> {
    if f44(v1, v0, match v3.clone() {
        Adt13::C0(h65, _) => {
            h65
        }
    })? {
        Ok(v2)
    } else {
        let m66 = v2;
        match m66 {
            None => {
                Ok(Some(match v3 {
                    Adt13::C0(h69, _) => {
                        h69
                    }
                }))
            }
            Some(v4) => {
                if f45(match v3.clone() {
                    Adt13::C0(h67, _) => {
                        h67
                    }
                }, v4.clone()) {
                    Ok(Some(match v3 {
                        Adt13::C0(h68, _) => {
                            h68
                        }
                    }))
                } else {
                    Ok(Some(v4))
                }
            }
        }
    }
}

pub fn f19(v0: Bytes, v1: Bytes) -> Bytes {
    let a70 = v0;
    let a71 = v1;
    append_bytes(a70, a71)
}

pub fn f20(v0: Bytes) -> Bytes {
    f19(f16(Str::lit("`")), f19(v0, f16(Str::lit("`"))))
}

pub fn f21(v0: Adt14, v1: Adt1) -> R<Result<Adt14, Bytes>> {
    let m72 = v1;
    match m72 {
        Adt1::C0(v2, v3) => {
            f46(v0, v2, v3)
        }
        Adt1::C1(v4, v5) => {
            Ok(f47(f48(Str::lit("a unit struct name"), v4.clone())?, f49({
                let m73 = f50(v0, v4.clone())?;
                match m73 {
                    Ok(v7) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v7.clone() {
                            Adt14::C0(h74, _, _, _, _, _, _, _, _, _) => {
                                h74
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, h75, _, _, _, _, _, _, _, _) => {
                                h75
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, h76, _, _, _, _, _, _, _) => {
                                h76
                            }
                        }, {
                            let a78 = match v7.clone() {
                                Adt14::C0(_, _, _, h77, _, _, _, _, _, _) => {
                                    h77
                                }
                            };
                            let a79 = List::cons(Adt11::C0(v4, List::<Bytes>::nil(), 0u64), List::<Adt11>::nil());
                            append_list(a78, a79)
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, h80, _, _, _, _, _) => {
                                h80
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, _, h81, _, _, _, _) => {
                                h81
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, _, _, h82, _, _, _) => {
                                h82
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h83, _, _) => {
                                h83
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h84, _) => {
                                h84
                            }
                        }, match v7 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h85) => {
                                h85
                            }
                        }))
                    }
                    Err(v6) => {
                        Err::<Adt14, Bytes>(v6)
                    }
                }
            }, f51(Str::lit("a unit struct"), v5)?)))
        }
        Adt1::C2(v8, v9, _) => {
            Ok(f47(f48(Str::lit("an alias name"), v8.clone())?, f49({
                let m86 = f50(v0, v8.clone())?;
                match m86 {
                    Ok(v12) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v12.clone() {
                            Adt14::C0(h87, _, _, _, _, _, _, _, _, _) => {
                                h87
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, h88, _, _, _, _, _, _, _, _) => {
                                h88
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, h89, _, _, _, _, _, _, _) => {
                                h89
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, h90, _, _, _, _, _, _) => {
                                h90
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, _, h91, _, _, _, _, _) => {
                                h91
                            }
                        }, {
                            let a94 = match v12.clone() {
                                Adt14::C0(_, _, _, _, _, h92, _, _, _, _) => {
                                    h92
                                }
                            };
                            let a95 = List::cons(Adt13::C0(v8, {
                                let a93 = v9.clone();
                                length_list(a93)
                            }), List::<Adt13>::nil());
                            append_list(a94, a95)
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, _, _, _, h96, _, _, _) => {
                                h96
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h97, _, _) => {
                                h97
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h98, _) => {
                                h98
                            }
                        }, match v12 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h99) => {
                                h99
                            }
                        }))
                    }
                    Err(v11) => {
                        Err::<Adt14, Bytes>(v11)
                    }
                }
            }, f52(Str::lit("an alias parameter"), v9)?)))
        }
        Adt1::C3(v13, _, _) => {
            Ok(f47(f48(Str::lit("a static name"), v13.clone())?, {
                let m100 = f50(v0, v13.clone())?;
                match m100 {
                    Ok(v17) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v17.clone() {
                            Adt14::C0(h101, _, _, _, _, _, _, _, _, _) => {
                                h101
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, h102, _, _, _, _, _, _, _, _) => {
                                h102
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, h103, _, _, _, _, _, _, _) => {
                                h103
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, h104, _, _, _, _, _, _) => {
                                h104
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, _, h105, _, _, _, _, _) => {
                                h105
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, _, _, h106, _, _, _, _) => {
                                h106
                            }
                        }, {
                            let a108 = match v17.clone() {
                                Adt14::C0(_, _, _, _, _, _, h107, _, _, _) => {
                                    h107
                                }
                            };
                            let a109 = List::cons(v13, List::<Bytes>::nil());
                            append_list(a108, a109)
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h110, _, _) => {
                                h110
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h111, _) => {
                                h111
                            }
                        }, match v17 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h112) => {
                                h112
                            }
                        }))
                    }
                    Err(v16) => {
                        Err::<Adt14, Bytes>(v16)
                    }
                }
            }))
        }
        Adt1::C4(v18, v19) => {
            Ok(f47(f48(Str::lit("a trait name"), v18.clone())?, {
                let m113 = f50(v0, v18.clone())?;
                match m113 {
                    Ok(v21) => {
                        Ok::<Adt14, Bytes>(f53(Adt14::C0(match v21.clone() {
                            Adt14::C0(h114, _, _, _, _, _, _, _, _, _) => {
                                h114
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, h115, _, _, _, _, _, _, _, _) => {
                                h115
                            }
                        }, {
                            let a117 = match v21.clone() {
                                Adt14::C0(_, _, h116, _, _, _, _, _, _, _) => {
                                    h116
                                }
                            };
                            let a118 = List::cons(Adt10::C0(v18, v19.clone()), List::<Adt10>::nil());
                            append_list(a117, a118)
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, h119, _, _, _, _, _, _) => {
                                h119
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, h120, _, _, _, _, _) => {
                                h120
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, h121, _, _, _, _) => {
                                h121
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, h122, _, _, _) => {
                                h122
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h123, _, _) => {
                                h123
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h124, _) => {
                                h124
                            }
                        }, match v21 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h125) => {
                                h125
                            }
                        }), v19))
                    }
                    Err(v20) => {
                        Err::<Adt14, Bytes>(v20)
                    }
                }
            }))
        }
        Adt1::C5(_, _, v24, v25) => {
            Ok(Ok::<Adt14, Bytes>(f54(f55(v0, v25.clone()), v24, v25)))
        }
        Adt1::C6(v26) => {
            Ok(f47(f48(Str::lit("a function name"), {
                let m128 = match v26.clone() {
                    Adt8::C0(h127, _, _, _) => {
                        h127
                    }
                };
                match m128 {
                    Adt5::C0(h126, _, _, _, _) => {
                        h126
                    }
                }
            })?, {
                let m132 = f50(v0, {
                    let m131 = match v26.clone() {
                        Adt8::C0(h130, _, _, _) => {
                            h130
                        }
                    };
                    match m131 {
                        Adt5::C0(h129, _, _, _, _) => {
                            h129
                        }
                    }
                })?;
                match m132 {
                    Ok(v28) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v28.clone() {
                            Adt14::C0(h133, _, _, _, _, _, _, _, _, _) => {
                                h133
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, h134, _, _, _, _, _, _, _, _) => {
                                h134
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, h135, _, _, _, _, _, _, _) => {
                                h135
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, _, h136, _, _, _, _, _, _) => {
                                h136
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, _, _, h137, _, _, _, _, _) => {
                                h137
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, _, _, _, h138, _, _, _, _) => {
                                h138
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, _, _, _, _, h139, _, _, _) => {
                                h139
                            }
                        }, List::cons(Adt13::C0({
                            let m142 = match v26.clone() {
                                Adt8::C0(h141, _, _, _) => {
                                    h141
                                }
                            };
                            match m142 {
                                Adt5::C0(h140, _, _, _, _) => {
                                    h140
                                }
                            }
                        }, {
                            let a146 = {
                                let m145 = match v26 {
                                    Adt8::C0(h144, _, _, _) => {
                                        h144
                                    }
                                };
                                match m145 {
                                    Adt5::C0(_, _, _, h143, _) => {
                                        h143
                                    }
                                }
                            };
                            length_list(a146)
                        }), match v28.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h147, _, _) => {
                                h147
                            }
                        }), match v28.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h148, _) => {
                                h148
                            }
                        }, match v28 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h149) => {
                                h149
                            }
                        }))
                    }
                    Err(v27) => {
                        Err::<Adt14, Bytes>(v27)
                    }
                }
            }))
        }
        Adt1::C7(v29, v30, v31, v32) => {
            Ok(f47(f48(Str::lit("a tuple struct name"), v29.clone())?, f49({
                let m150 = f50(v0, v29.clone())?;
                match m150 {
                    Ok(v34) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v34.clone() {
                            Adt14::C0(h151, _, _, _, _, _, _, _, _, _) => {
                                h151
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, h152, _, _, _, _, _, _, _, _) => {
                                h152
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, h153, _, _, _, _, _, _, _) => {
                                h153
                            }
                        }, {
                            let a156 = match v34.clone() {
                                Adt14::C0(_, _, _, h154, _, _, _, _, _, _) => {
                                    h154
                                }
                            };
                            let a157 = List::cons(Adt11::C0(v29, f56(v32, 0u64)?, {
                                let a155 = v30.clone();
                                length_list(a155)
                            }), List::<Adt11>::nil());
                            append_list(a156, a157)
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, h158, _, _, _, _, _) => {
                                h158
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, _, h159, _, _, _, _) => {
                                h159
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, _, _, h160, _, _, _) => {
                                h160
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h161, _, _) => {
                                h161
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h162, _) => {
                                h162
                            }
                        }, match v34 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h163) => {
                                h163
                            }
                        }))
                    }
                    Err(v33) => {
                        Err::<Adt14, Bytes>(v33)
                    }
                }
            }, f57(f52(Str::lit("a tuple struct generic"), v30)?, f51(Str::lit("a tuple struct"), v31)?))))
        }
        Adt1::C8(v35, v36, v37, v38, v39) => {
            Ok(f47(f48(Str::lit("a record struct name"), v35.clone())?, {
                let m164 = f50(v0, v35.clone())?;
                match m164 {
                    Ok(v41) => {
                        let m165 = f57(f52(Str::lit("a record struct generic"), v36.clone())?, f57(f51(Str::lit("a record struct"), v37)?, f52(Str::lit("a record field name"), v38.clone())?));
                        match m165 {
                            None => {
                                let a167 = {
                                    let a166 = v38.clone();
                                    length_list(a166)
                                };
                                let a168 = f58(v39.clone())?;
                                if nat_eq(a167, a168) {
                                    Ok::<Adt14, Bytes>(Adt14::C0(match v41.clone() {
                                        Adt14::C0(h169, _, _, _, _, _, _, _, _, _) => {
                                            h169
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, h170, _, _, _, _, _, _, _, _) => {
                                            h170
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, h171, _, _, _, _, _, _, _) => {
                                            h171
                                        }
                                    }, {
                                        let a174 = match v41.clone() {
                                            Adt14::C0(_, _, _, h172, _, _, _, _, _, _) => {
                                                h172
                                            }
                                        };
                                        let a175 = List::cons(Adt11::C0(v35, v38, {
                                            let a173 = v36;
                                            length_list(a173)
                                        }), List::<Adt11>::nil());
                                        append_list(a174, a175)
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, h176, _, _, _, _, _) => {
                                            h176
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, _, h177, _, _, _, _) => {
                                            h177
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, _, _, h178, _, _, _) => {
                                            h178
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, _, _, _, h179, _, _) => {
                                            h179
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, _, _, _, _, h180, _) => {
                                            h180
                                        }
                                    }, match v41 {
                                        Adt14::C0(_, _, _, _, _, _, _, _, _, h181) => {
                                            h181
                                        }
                                    }))
                                } else {
                                    Err::<Adt14, Bytes>(f19(f16(Str::lit("runtime support: struct-fields: ")), f19(f20(v35), f19(f16(Str::lit(" has ")), f19(f59({
                                        let a182 = v38;
                                        length_list(a182)
                                    })?, f19(f16(Str::lit(" field names and ")), f19(f59(f58(v39)?)?, f16(Str::lit(" types")))))))))
                                }
                            }
                            Some(v42) => {
                                Err::<Adt14, Bytes>(v42)
                            }
                        }
                    }
                    Err(v40) => {
                        Err::<Adt14, Bytes>(v40)
                    }
                }
            }))
        }
        Adt1::C9(v52, v53, v54, v55) => {
            Ok(f47(f48(Str::lit("an enum name"), v52.clone())?, {
                let m207 = f50(v0, v52.clone())?;
                match m207 {
                    Ok(v57) => {
                        let m208 = f57(f52(Str::lit("an enum generic"), v53.clone())?, f57(f51(Str::lit("an enum"), v54)?, f60(v52.clone(), v55.clone())?));
                        match m208 {
                            None => {
                                let m209 = f61(v55.clone())?;
                                match m209 {
                                    None => {
                                        Ok::<Adt14, Bytes>(Adt14::C0(match v57.clone() {
                                            Adt14::C0(h210, _, _, _, _, _, _, _, _, _) => {
                                                h210
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, h211, _, _, _, _, _, _, _, _) => {
                                                h211
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, h212, _, _, _, _, _, _, _) => {
                                                h212
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, h213, _, _, _, _, _, _) => {
                                                h213
                                            }
                                        }, {
                                            let a216 = match v57.clone() {
                                                Adt14::C0(_, _, _, _, h214, _, _, _, _, _) => {
                                                    h214
                                                }
                                            };
                                            let a217 = List::cons(Adt12::C0(v52, v55, {
                                                let a215 = v53;
                                                length_list(a215)
                                            }), List::<Adt12>::nil());
                                            append_list(a216, a217)
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, h218, _, _, _, _) => {
                                                h218
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, h219, _, _, _) => {
                                                h219
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, _, h220, _, _) => {
                                                h220
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, _, _, h221, _) => {
                                                h221
                                            }
                                        }, match v57 {
                                            Adt14::C0(_, _, _, _, _, _, _, _, _, h222) => {
                                                h222
                                            }
                                        }))
                                    }
                                    Some(v59) => {
                                        Err::<Adt14, Bytes>(f19(f16(Str::lit("runtime support: variant-name: ")), f19(f20(v52), f19(f16(Str::lit(" declares ")), f19(f20(v59), f16(Str::lit(" twice")))))))
                                    }
                                }
                            }
                            Some(v58) => {
                                Err::<Adt14, Bytes>(v58)
                            }
                        }
                    }
                    Err(v56) => {
                        Err::<Adt14, Bytes>(v56)
                    }
                }
            }))
        }
        Adt1::C10(_) => {
            Ok(Ok::<Adt14, Bytes>(v0))
        }
        Adt1::C11(v44, _) => {
            Ok(f47(f48(Str::lit("a module name"), v44.clone())?, f50(v0, v44)?))
        }
        Adt1::C12(_, _) => {
            Ok(Ok::<Adt14, Bytes>(v0))
        }
        Adt1::C13(_, r223) => {
            let v49 = (*r223).clone();
            let m183 = f37(v49.clone());
            match m183 {
                None => {
                    let m204 = f41(v49);
                    match m204 {
                        None => {
                            Ok(Ok::<Adt14, Bytes>(v0))
                        }
                        Some(v51) => {
                            f46(v0, {
                                let (h205, _) = v51;
                                h205
                            }, {
                                let (_, h206) = v51;
                                h206
                            })
                        }
                    }
                }
                Some(v50) => {
                    Ok(f47(f48(Str::lit("a function name"), {
                        let m186 = match v50.clone() {
                            Adt8::C0(h185, _, _, _) => {
                                h185
                            }
                        };
                        match m186 {
                            Adt5::C0(h184, _, _, _, _) => {
                                h184
                            }
                        }
                    })?, Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
                        Adt14::C0(h187, _, _, _, _, _, _, _, _, _) => {
                            h187
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, h188, _, _, _, _, _, _, _, _) => {
                            h188
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, h189, _, _, _, _, _, _, _) => {
                            h189
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, h190, _, _, _, _, _, _) => {
                            h190
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, h191, _, _, _, _, _) => {
                            h191
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h192, _, _, _, _) => {
                            h192
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, h193, _, _, _) => {
                            h193
                        }
                    }, List::cons(Adt13::C0({
                        let m196 = match v50.clone() {
                            Adt8::C0(h195, _, _, _) => {
                                h195
                            }
                        };
                        match m196 {
                            Adt5::C0(h194, _, _, _, _) => {
                                h194
                            }
                        }
                    }, {
                        let a200 = {
                            let m199 = match v50 {
                                Adt8::C0(h198, _, _, _) => {
                                    h198
                                }
                            };
                            match m199 {
                                Adt5::C0(_, _, _, h197, _) => {
                                    h197
                                }
                            }
                        };
                        length_list(a200)
                    }), match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, h201, _, _) => {
                            h201
                        }
                    }), match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, h202, _) => {
                            h202
                        }
                    }, match v0 {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, h203) => {
                            h203
                        }
                    }))))
                }
            }
        }
    }
}

pub fn f22(v0: Adt14, v1: Adt2, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m224 = v1;
    match m224 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1 => {
            Ok(None::<Bytes>)
        }
        Adt2::C2 => {
            Ok(None::<Bytes>)
        }
        Adt2::C3 => {
            Ok(None::<Bytes>)
        }
        Adt2::C4 => {
            Ok(None::<Bytes>)
        }
        Adt2::C5 => {
            if v3 {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: self-type: ")), f16(Str::lit("`Self` outside a trait or an impl")))))
            }
        }
        Adt2::C6(v4) => {
            let a225 = v4;
            let a226 = v2;
            if nat_lt(a225, a226) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: type-parameter: ")), f19(f16(Str::lit("parameter ")), f19(f59(v4)?, f19(f16(Str::lit(" of ")), f19(f59(v2)?, f16(Str::lit(" in scope")))))))))
            }
        }
        Adt2::C7(r234) => {
            let v5 = (*r234).clone();
            f22(v0, v5, v2, v3)
        }
        Adt2::C8(r235) => {
            let v6 = (*r235).clone();
            f22(v0, v6, v2, v3)
        }
        Adt2::C9(r236, r237) => {
            let v7 = (*r236).clone();
            let v8 = (*r237).clone();
            let m227 = f22(v0.clone(), v7, v2, v3)?;
            match m227 {
                None => {
                    f22(v0, v8, v2, v3)
                }
                Some(v9) => {
                    Ok(Some(v9))
                }
            }
        }
        Adt2::C10(v10, r238) => {
            let v11 = (*r238).clone();
            let m228 = f62(v10.clone())?;
            match m228 {
                None => {
                    let m229 = f63(v0.clone(), v10.clone())?;
                    match m229 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: type-name: ")), f19(f20(v10), f16(Str::lit(" is neither declared nor in the closed table"))))))
                        }
                        Some(v13) => {
                            if if match v0.clone() {
                                Adt14::C0(h230, _, _, _, _, _, _, _, _, _) => {
                                    h230
                                }
                            } {
                                f64(v0.clone(), v10.clone())
                            } else {
                                false
                            } {
                                Ok(Some(f19(f16(Str::lit("runtime support: hidden-allocation: ")), f19(f16(Str::lit("rust-core renders ")), f19(f20(v10), f16(Str::lit(", which requires heap allocation")))))))
                            } else {
                                let a231 = v13;
                                let a232 = f58(v11.clone())?;
                                if nat_eq(a231, a232) {
                                    f22(v0, v11, v2, v3)
                                } else {
                                    Ok(Some(f19(f16(Str::lit("runtime support: type-arity: ")), f19(f20(v10), f19(f16(Str::lit(" takes ")), f19(f59(v13)?, f19(f16(Str::lit(" arguments, given ")), f59(f58(v11)?)?)))))))
                                }
                            }
                        }
                    }
                }
                Some(v12) => {
                    Ok(Some(v12))
                }
            }
        }
        Adt2::C11 => {
            Ok(None::<Bytes>)
        }
        Adt2::C12 => {
            Ok(None::<Bytes>)
        }
        Adt2::C13 => {
            Ok(None::<Bytes>)
        }
        Adt2::C14 => {
            Ok(None::<Bytes>)
        }
        Adt2::C15(r239) => {
            let v14 = (*r239).clone();
            f22(v0, v14, v2, v3)
        }
        Adt2::C16 => {
            Ok(None::<Bytes>)
        }
        Adt2::C17 => {
            Ok(None::<Bytes>)
        }
        Adt2::C18 => {
            Ok(None::<Bytes>)
        }
        Adt2::C19(r240, r241) => {
            let v15 = (*r240).clone();
            let v16 = (*r241).clone();
            let m233 = f22(v0.clone(), v15, v2, v3)?;
            match m233 {
                None => {
                    f22(v0, v16, v2, v3)
                }
                Some(v17) => {
                    Ok(Some(v17))
                }
            }
        }
    }
}

pub fn f23(v0: Adt14, v1: Adt16, v2: Adt3) -> R<Option<Bytes>> {
    let m242 = v2;
    match m242 {
        Adt3::C0(v3) => {
            let m244 = f65(match v1 {
                Adt16::C0(_, h243, _, _) => {
                    h243
                }
            }, v3.clone());
            match m244 {
                None => {
                    if f66(match v0 {
                        Adt14::C0(_, _, _, _, _, _, h245, _, _, _) => {
                            h245
                        }
                    }, v3.clone())? {
                        Ok(None::<Bytes>)
                    } else {
                        Ok(Some(f19(f16(Str::lit("runtime support: variable-scope: ")), f19(f20(v3), f16(Str::lit(" is not in scope"))))))
                    }
                }
                Some(_) => {
                    Ok(None::<Bytes>)
                }
            }
        }
        Adt3::C1(v5) => {
            f67(v5)
        }
        Adt3::C2(v6) => {
            f68(v0, v6)
        }
        Adt3::C3(v7, r290) => {
            let v8 = (*r290).clone();
            let m246 = f69(v0.clone(), v7, f70(v8.clone())?)?;
            match m246 {
                None => {
                    f71(v0, v1, v8)
                }
                Some(v9) => {
                    Ok(Some(v9))
                }
            }
        }
        Adt3::C4(r291, v11, r292) => {
            let v10 = (*r291).clone();
            let v12 = (*r292).clone();
            let m247 = f23(v0.clone(), v1.clone(), v10.clone())?;
            match m247 {
                None => {
                    let m248 = f72(v0.clone(), v1.clone(), v10, v11, f70(v12.clone())?)?;
                    match m248 {
                        None => {
                            f71(v0, v1, v12)
                        }
                        Some(v14) => {
                            Ok(Some(v14))
                        }
                    }
                }
                Some(v13) => {
                    Ok(Some(v13))
                }
            }
        }
        Adt3::C5(r293, _) => {
            let v15 = (*r293).clone();
            f23(v0, v1, v15)
        }
        Adt3::C6(r294, v18) => {
            let v17 = (*r294).clone();
            let m249 = f48(Str::lit("a field access"), v18)?;
            match m249 {
                None => {
                    f23(v0, v1, v17)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
        Adt3::C7(r295) => {
            let v20 = (*r295).clone();
            f23(v0, v1, v20)
        }
        Adt3::C8(r296, r297) => {
            let v21 = (*r296).clone();
            let v22 = (*r297).clone();
            let m250 = f23(v0.clone(), v1.clone(), v21)?;
            match m250 {
                None => {
                    f23(v0, v1, v22)
                }
                Some(v23) => {
                    Ok(Some(v23))
                }
            }
        }
        Adt3::C9(r298, r299) => {
            let v24 = (*r298).clone();
            let v25 = (*r299).clone();
            let m251 = f57(f73(v24.clone())?, f23(v0.clone(), v1.clone(), v24)?);
            match m251 {
                None => {
                    if f74(v25.clone()) {
                        Ok(Some(f19(f16(Str::lit("runtime support: match-arms: ")), f16(Str::lit("a match with no arm")))))
                    } else {
                        f75(v0, v1, v25)
                    }
                }
                Some(v26) => {
                    Ok(Some(v26))
                }
            }
        }
        Adt3::C10(_, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: let-position: ")), f16(Str::lit("a `let` that is not a statement of a block")))))
        }
        Adt3::C11(r300, r301) => {
            let v30 = (*r300).clone();
            let v31 = (*r301).clone();
            let m252 = f76(v1.clone(), v30.clone());
            match m252 {
                None => {
                    let m253 = f23(v0.clone(), v1.clone(), v30)?;
                    match m253 {
                        None => {
                            f23(v0, v1, v31)
                        }
                        Some(v33) => {
                            Ok(Some(v33))
                        }
                    }
                }
                Some(v32) => {
                    Ok(Some(v32))
                }
            }
        }
        Adt3::C12(r302, r303, r304) => {
            let v34 = (*r302).clone();
            let v35 = (*r303).clone();
            let v36 = (*r304).clone();
            let m254 = f57(f73(v34.clone())?, f23(v0.clone(), v1.clone(), v34)?);
            match m254 {
                None => {
                    let m255 = f77(v0.clone(), v1.clone(), v35)?;
                    match m255 {
                        None => {
                            f77(v0, v1, v36)
                        }
                        Some(v38) => {
                            Ok(Some(v38))
                        }
                    }
                }
                Some(v37) => {
                    Ok(Some(v37))
                }
            }
        }
        Adt3::C13(r305, r306) => {
            let v39 = (*r305).clone();
            let v40 = (*r306).clone();
            let m256 = f57(f73(v39.clone())?, f23(v0.clone(), v1.clone(), v39)?);
            match m256 {
                None => {
                    f77(v0, f78(v1, List::<Adt17>::nil(), true), v40)
                }
                Some(v41) => {
                    Ok(Some(v41))
                }
            }
        }
        Adt3::C14(v42, r307, r308) => {
            let v43 = (*r307).clone();
            let v44 = (*r308).clone();
            let m257 = f57(f73(v43.clone())?, f23(v0.clone(), v1.clone(), v43)?);
            match m257 {
                None => {
                    let m258 = f79(v0.clone(), v42.clone())?;
                    match m258 {
                        None => {
                            let m259 = f80(List::<Adt17>::nil(), v42)?;
                            match m259 {
                                Ok(v48) => {
                                    f77(v0, f78(v1, v48, true), v44)
                                }
                                Err(v47) => {
                                    Ok(Some(v47))
                                }
                            }
                        }
                        Some(v46) => {
                            Ok(Some(v46))
                        }
                    }
                }
                Some(v45) => {
                    Ok(Some(v45))
                }
            }
        }
        Adt3::C15(r309) => {
            let v49 = (*r309).clone();
            f77(v0, f78(v1, List::<Adt17>::nil(), true), v49)
        }
        Adt3::C16(v50, r310, r311) => {
            let v51 = (*r310).clone();
            let v52 = (*r311).clone();
            let m260 = f57(f73(v51.clone())?, f23(v0.clone(), v1.clone(), v51)?);
            match m260 {
                None => {
                    let m261 = f79(v0.clone(), v50.clone())?;
                    match m261 {
                        None => {
                            let m262 = f80(List::<Adt17>::nil(), v50)?;
                            match m262 {
                                Ok(v56) => {
                                    f77(v0, f78(v1, v56, true), v52)
                                }
                                Err(v55) => {
                                    Ok(Some(v55))
                                }
                            }
                        }
                        Some(v54) => {
                            Ok(Some(v54))
                        }
                    }
                }
                Some(v53) => {
                    Ok(Some(v53))
                }
            }
        }
        Adt3::C17 => {
            if match v1 {
                Adt16::C0(_, _, _, h263) => {
                    h263
                }
            } {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: break-position: ")), f16(Str::lit("a `break` outside a loop")))))
            }
        }
        Adt3::C18(r312) => {
            let v57 = (*r312).clone();
            if f81(v57.clone()) {
                Ok(None::<Bytes>)
            } else {
                f23(v0, v1, v57)
            }
        }
        Adt3::C19(r313) => {
            let v58 = (*r313).clone();
            let a264 = f70(v58.clone())?;
            let a265 = 2u64;
            if nat_lt(a264, a265) {
                Ok(Some(f19(f16(Str::lit("runtime support: tuple-arity: ")), f19(f16(Str::lit("a tuple of ")), f59(f70(v58)?)?))))
            } else {
                f71(v0, v1, v58)
            }
        }
        Adt3::C20(v59, v60, r314) => {
            let v61 = (*r314).clone();
            let m266 = f68(v0.clone(), v59.clone())?;
            match m266 {
                None => {
                    let m267 = f82(v0.clone(), v59.clone());
                    match m267 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: struct-fields: ")), f19(f20(f83(v59, Str::lit("::"))?), f16(Str::lit(" is no record struct or record variant"))))))
                        }
                        Some(v63) => {
                            if f84(v60.clone(), v63.clone(), f70(v61.clone())?)? {
                                f71(v0, v1, v61)
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: struct-fields: ")), f19(f20(f83(v59, Str::lit("::"))?), f19(f16(Str::lit(" is given ")), f19(f83(v60, Str::lit(", "))?, f19(f16(Str::lit(", not ")), f83(v63, Str::lit(", "))?)))))))
                            }
                        }
                    }
                }
                Some(v62) => {
                    Ok(Some(v62))
                }
            }
        }
        Adt3::C21(_, r315, r316) => {
            let v65 = (*r315).clone();
            let v66 = (*r316).clone();
            let m268 = f23(v0.clone(), v1.clone(), v65)?;
            match m268 {
                None => {
                    f23(v0, v1, v66)
                }
                Some(v67) => {
                    Ok(Some(v67))
                }
            }
        }
        Adt3::C22(r317) => {
            let v68 = (*r317).clone();
            f23(v0, v1, v68)
        }
        Adt3::C23(r318, v70) => {
            let v69 = (*r318).clone();
            let m273 = f57(f22(v0.clone(), v70.clone(), {
                let a270 = match v1.clone() {
                    Adt16::C0(h269, _, _, _) => {
                        h269
                    }
                };
                length_list(a270)
            }, {
                let a272 = f85(match v1.clone() {
                    Adt16::C0(_, _, h271, _) => {
                        h271
                    }
                });
                bool_not(a272)
            })?, f86(Str::lit("a cast type"), v70));
            match m273 {
                None => {
                    f23(v0, v1, v69)
                }
                Some(v71) => {
                    Ok(Some(v71))
                }
            }
        }
        Adt3::C24(r319, r320) => {
            let v72 = (*r319).clone();
            let v73 = (*r320).clone();
            let m274 = f23(v0.clone(), v1.clone(), v72)?;
            match m274 {
                None => {
                    f23(v0, v1, v73)
                }
                Some(v74) => {
                    Ok(Some(v74))
                }
            }
        }
        Adt3::C25(r321, r322, r323) => {
            let v75 = (*r321).clone();
            let v76 = (*r322).clone();
            let v77 = (*r323).clone();
            let m275 = f23(v0.clone(), v1.clone(), v75)?;
            match m275 {
                None => {
                    let m276 = if f81(v76.clone()) {
                        None::<Bytes>
                    } else {
                        f23(v0.clone(), v1.clone(), v76)?
                    };
                    match m276 {
                        None => {
                            if f81(v77.clone()) {
                                Ok(None::<Bytes>)
                            } else {
                                f23(v0, v1, v77)
                            }
                        }
                        Some(v79) => {
                            Ok(Some(v79))
                        }
                    }
                }
                Some(v78) => {
                    Ok(Some(v78))
                }
            }
        }
        Adt3::C26(r324) => {
            let v80 = (*r324).clone();
            f23(v0, v1, v80)
        }
        Adt3::C27(r325) => {
            let v81 = (*r325).clone();
            f23(v0, v1, v81)
        }
        Adt3::C28(r326, v83, v84, r327) => {
            let v82 = (*r326).clone();
            let v85 = (*r327).clone();
            let m277 = f23(v0.clone(), v1.clone(), v82)?;
            match m277 {
                None => {
                    let m278 = f87(v83.clone());
                    match m278 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: method-name: ")), f19(f20(v83), f16(Str::lit(" takes no type arguments in the closed table"))))))
                        }
                        Some(v87) => {
                            let a279 = v87;
                            let a280 = f58(v84.clone())?;
                            if if nat_eq(a279, a280) {
                                let a281 = f88(v83.clone());
                                let a282 = f70(v85.clone())?;
                                nat_eq(a281, a282)
                            } else {
                                false
                            } {
                                if if match v0.clone() {
                                    Adt14::C0(h283, _, _, _, _, _, _, _, _, _) => {
                                        h283
                                    }
                                } {
                                    f40(v83.clone(), f16(Str::lit("collect")))
                                } else {
                                    false
                                } {
                                    Ok(Some(f19(f16(Str::lit("runtime support: hidden-allocation: ")), f19(f16(Str::lit("rust-core renders ")), f19(f20(v83), f16(Str::lit(", which requires heap allocation")))))))
                                } else {
                                    let m288 = f57(f22(v0.clone(), v84.clone(), {
                                        let a285 = match v1.clone() {
                                            Adt16::C0(h284, _, _, _) => {
                                                h284
                                            }
                                        };
                                        length_list(a285)
                                    }, {
                                        let a287 = f85(match v1.clone() {
                                            Adt16::C0(_, _, h286, _) => {
                                                h286
                                            }
                                        });
                                        bool_not(a287)
                                    })?, f86(Str::lit("a type argument"), v84));
                                    match m288 {
                                        None => {
                                            f71(v0, v1, v85)
                                        }
                                        Some(v88) => {
                                            Ok(Some(v88))
                                        }
                                    }
                                }
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: method-arity: ")), f19(f20(v83.clone()), f19(f16(Str::lit(" takes ")), f19(f59(v87)?, f19(f16(Str::lit(" type arguments and ")), f19(f59(f88(v83))?, f16(Str::lit(" arguments"))))))))))
                            }
                        }
                    }
                }
                Some(v86) => {
                    Ok(Some(v86))
                }
            }
        }
        Adt3::C29(r328, r329) => {
            let v89 = (*r328).clone();
            let v90 = (*r329).clone();
            let m289 = f89(v0.clone(), v1, v89)?;
            match m289 {
                Ok(v92) => {
                    f23(v0, v92, v90)
                }
                Err(v91) => {
                    Ok(Some(v91))
                }
            }
        }
        Adt3::C30 => {
            Ok(None::<Bytes>)
        }
        Adt3::C31 => {
            Ok(None::<Bytes>)
        }
        Adt3::C32(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f24(_: ()) -> Adt16 {
    Adt16::C0(List::<Adt7>::nil(), List::<Adt17>::nil(), Adt2::C17, false)
}

pub fn f25(v0: List<Adt5>) -> R<Option<Bytes>> {
    let m331 = f90(Fn8::F91, Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m331 {
        Adt18::C0(_, h330) => {
            Ok(h330)
        }
    }
}

pub fn f26(v0: Fn4, p335: Option<Bytes>, p336: List<Adt5>) -> R<Option<Bytes>> {
    let m334 = (p335, p336);
    {
        let mut m334 = m334;
        loop {
            let step = {
                let (v1, v2) = m334;
                let m332 = v2;
                match m332.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt5>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt5>), R<Option<Bytes>>>(({
                            let c333: Fn4 = v0.clone();
                            c333.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt14, v1: Option<Bytes>, v2: Adt5) -> R<Option<Bytes>> {
    let m337 = v1;
    match m337 {
        None => {
            f92(v0, v2, 0u64, true)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f28(v0: Adt14, v1: List<Adt7>) -> R<Option<Bytes>> {
    f93(Fn9::F94(v0), None::<Bytes>, v1)
}

pub fn f29(v0: Adt14, v1: Bytes, v2: List<Adt7>, v3: Adt2, v4: List<Adt8>) -> R<Option<Bytes>> {
    let m338 = f28(v0.clone(), v2.clone())?;
    match m338 {
        None => {
            let m339 = f95(v0.clone(), v1.clone());
            match m339 {
                None => {
                    Ok(Some(f19(f16(Str::lit("runtime support: impl-trait: ")), f19(f16(Str::lit("`impl ")), f19(v1, f16(Str::lit("` names no declared trait")))))))
                }
                Some(v6) => {
                    let m340 = f26(Fn4::F96(v4.clone(), v1.clone()), None::<Bytes>, v6.clone())?;
                    match m340 {
                        None => {
                            f30(Fn5::F97(v2, v0, v6, v1, v3), None::<Bytes>, v4)
                        }
                        Some(v7) => {
                            Ok(Some(v7))
                        }
                    }
                }
            }
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f30(v0: Fn5, p344: Option<Bytes>, p345: List<Adt8>) -> R<Option<Bytes>> {
    let m343 = (p344, p345);
    {
        let mut m343 = m343;
        loop {
            let step = {
                let (v1, v2) = m343;
                let m341 = v2;
                match m341.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt8>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt8>), R<Option<Bytes>>>(({
                            let c342: Fn5 = v0.clone();
                            c342.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: List<Adt7>, v1: Adt14, v2: Adt2, v3: Option<Bytes>, v4: Adt8) -> R<Option<Bytes>> {
    let m346 = v3;
    match m346 {
        None => {
            f98(v1, v0, v2, v4)
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f32(v0: Adt14, v1: Adt8) -> R<Option<Bytes>> {
    let a348 = match v1.clone() {
        Adt8::C0(_, _, h347, _) => {
            h347
        }
    };
    if bool_not(a348) {
        Ok(Some(f19(f16(Str::lit("runtime support: function-visibility: ")), f19(f20({
            let m351 = match v1 {
                Adt8::C0(h350, _, _, _) => {
                    h350
                }
            };
            match m351 {
                Adt5::C0(h349, _, _, _, _) => {
                    h349
                }
            }
        }), f16(Str::lit(" is a free function that is not public"))))))
    } else {
        let m354 = match v1.clone() {
            Adt8::C0(h353, _, _, _) => {
                h353
            }
        };
        if match m354 {
            Adt5::C0(_, h352, _, _, _) => {
                h352
            }
        } {
            Ok(Some(f19(f16(Str::lit("runtime support: function-receiver: ")), f19(f16(Str::lit("free function ")), f19(f20({
                let m357 = match v1 {
                    Adt8::C0(h356, _, _, _) => {
                        h356
                    }
                };
                match m357 {
                    Adt5::C0(h355, _, _, _, _) => {
                        h355
                    }
                }
            }), f16(Str::lit(" takes `&self`")))))))
        } else {
            f98(v0, List::<Adt7>::nil(), Adt2::C17, v1)
        }
    }
}

pub fn f33(v0: Fn6, p361: Option<Bytes>, p362: List<Adt9>) -> R<Option<Bytes>> {
    let m360 = (p361, p362);
    {
        let mut m360 = m360;
        loop {
            let step = {
                let (v1, v2) = m360;
                let m358 = v2;
                match m358.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt9>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt9>), R<Option<Bytes>>>(({
                            let c359: Fn6 = v0.clone();
                            c359.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<Bytes>, v1: Adt14, v2: Option<Bytes>, v3: Adt9) -> R<Option<Bytes>> {
    let m363 = v2;
    match m363 {
        None => {
            let m364 = v3;
            match m364 {
                Adt9::C0(_) => {
                    Ok(None::<Bytes>)
                }
                Adt9::C1(_, v7) => {
                    f22(v1, v7, {
                        let a365 = v0;
                        length_list(a365)
                    }, false)
                }
                Adt9::C2(_, _, v10) => {
                    f22(v1, v10, {
                        let a366 = v0;
                        length_list(a366)
                    }, false)
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f35(v0: Bytes) -> R<Option<Bytes>> {
    let a368 = {
        let a367 = v0.clone();
        length_bytes(a367)
    };
    let a369 = 0u64;
    if nat_eq(a368, a369) {
        Ok(Some(f16(Str::lit("runtime support: defined-path: empty"))))
    } else {
        f52(Str::lit("a defined path segment"), f99(v0)?)
    }
}

pub fn f36(v0: Bytes, v1: Adt14, v2: Option<Bytes>, v3: Adt1) -> R<Option<Bytes>> {
    let m370 = v2;
    match m370 {
        None => {
            f100(v1, v0, v3)
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f37(v0: Adt1) -> Option<Adt8> {
    let m371 = v0;
    match m371 {
        Adt1::C0(_, _) => {
            None::<Adt8>
        }
        Adt1::C1(_, _) => {
            None::<Adt8>
        }
        Adt1::C2(_, _, _) => {
            None::<Adt8>
        }
        Adt1::C3(_, _, _) => {
            None::<Adt8>
        }
        Adt1::C4(_, _) => {
            None::<Adt8>
        }
        Adt1::C5(_, _, _, _) => {
            None::<Adt8>
        }
        Adt1::C6(v17) => {
            Some(v17)
        }
        Adt1::C7(_, _, _, _) => {
            None::<Adt8>
        }
        Adt1::C8(_, _, _, _, _) => {
            None::<Adt8>
        }
        Adt1::C9(_, _, _, _) => {
            None::<Adt8>
        }
        Adt1::C10(_) => {
            None::<Adt8>
        }
        Adt1::C11(_, _) => {
            None::<Adt8>
        }
        Adt1::C12(_, _) => {
            None::<Adt8>
        }
        Adt1::C13(_, _) => {
            None::<Adt8>
        }
    }
}

pub fn f38(v0: Bytes, v1: List<Bytes>) -> R<Option<Bytes>> {
    if f40(v0.clone(), f16(Str::lit("no_std"))) {
        let m372 = v1;
        match m372.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, _)) => {
                Ok(Some(f19(f16(Str::lit("runtime support: attribute-args: ")), f19(f20(v0), f16(Str::lit(" takes no arguments"))))))
            }
        }
    } else {
        if f40(v0.clone(), f16(Str::lit("forbid"))) {
            let m373 = v1.clone();
            match m373.uncons() {
                None => {
                    Ok(Some(f19(f16(Str::lit("runtime support: attribute-args: ")), f19(f20(v0), f16(Str::lit(" takes a lint"))))))
                }
                Some((_, _)) => {
                    if f101(Fn10::F102, false, v1.clone())? {
                        Ok(Some(f19(f16(Str::lit("runtime support: attribute-args: ")), f19(f20(v0), f16(Str::lit(" takes a lint with a name"))))))
                    } else {
                        f52(Str::lit("a lint name"), v1)
                    }
                }
            }
        } else {
            Ok(Some(f19(f16(Str::lit("runtime support: attribute-name: ")), f19(f20(v0), f16(Str::lit(" is not an inner attribute of the closed set"))))))
        }
    }
}

pub fn f39(v0: Adt1) -> Option<(Bytes, Adt1)> {
    let m374 = v0;
    match m374 {
        Adt1::C0(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C1(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C2(_, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C3(_, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C4(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C5(_, _, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C6(_) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C7(_, _, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C8(_, _, _, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C9(_, _, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C10(_) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C11(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C12(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C13(v36, r375) => {
            let v37 = (*r375).clone();
            Some((v36, v37))
        }
    }
}

pub fn f40(v0: Bytes, v1: Bytes) -> bool {
    let a376 = v0;
    let a377 = v1;
    equal(a376, a377)
}

pub fn f41(v0: Adt1) -> Option<(List<Bytes>, List<Bytes>)> {
    let m378 = v0;
    match m378 {
        Adt1::C0(v1, v2) => {
            Some((v1, v2))
        }
        Adt1::C1(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C2(_, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C3(_, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C4(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C5(_, _, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C6(_) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C7(_, _, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C8(_, _, _, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C9(_, _, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C10(_) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C11(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C12(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C13(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
    }
}

pub fn f42(v0: Fn7, p382: List<Adt13>, p383: List<Adt1>) -> List<Adt13> {
    let m381 = (p382, p383);
    {
        let mut m381 = m381;
        loop {
            let step = {
                let (v1, v2) = m381;
                let m379 = v2;
                match m379.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt1>), List<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt13>, List<Adt1>), List<Adt13>>(({
                            let c380: Fn7 = v0.clone();
                            c380.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Bytes, v1: List<Adt13>, v2: Adt1) -> List<Adt13> {
    let m384 = f103(v2, v0);
    match m384 {
        None => {
            v1
        }
        Some(v3) => {
            let a385 = v1;
            let a386 = List::cons(v3, List::<Adt13>::nil());
            append_list(a385, a386)
        }
    }
}

pub fn f44(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> R<bool> {
    let a387 = f104(v0.clone(), v2.clone())?;
    let a388 = 1u64;
    if nat_eq(a387, a388) {
        let a389 = f104(v1.clone(), v2.clone())?;
        let a390 = 1u64;
        if nat_eq(a389, a390) {
            let m391 = f105(v0, v2.clone());
            match m391 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m392 = f105(v1, v2);
                    match m392 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            let a393 = v3;
                            let a394 = v4;
                            Ok(nat_eq(a393, a394))
                        }
                    }
                }
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f45(v0: Bytes, v1: Bytes) -> bool {
    let a399 = {
        let a395 = v0;
        let a396 = v1;
        compare_bytes(a395, a396)
    };
    let a400 = {
        let a397 = Bytes::lit(&[]);
        let a398 = Bytes::lit(&[0x01]);
        compare_bytes(a397, a398)
    };
    equal(a399, a400)
}

pub fn f46(v0: Adt14, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<Adt14, Bytes>> {
    Ok(f49({
        let m401 = v1.clone();
        match m401.uncons() {
            None => {
                Err::<Adt14, Bytes>(f16(Str::lit("runtime support: use-path: empty")))
            }
            Some((v3, _)) => {
                if if f40(v3.clone(), f16(Str::lit("core"))) {
                    true
                } else {
                    if match v0.clone() {
                        Adt14::C0(h402, _, _, _, _, _, _, _, _, _) => {
                            h402
                        }
                    } {
                        false
                    } else {
                        f40(v3.clone(), f16(Str::lit("std")))
                    }
                } {
                    Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
                        Adt14::C0(h403, _, _, _, _, _, _, _, _, _) => {
                            h403
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, h404, _, _, _, _, _, _, _, _) => {
                            h404
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, h405, _, _, _, _, _, _, _) => {
                            h405
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, h406, _, _, _, _, _, _) => {
                            h406
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, h407, _, _, _, _, _) => {
                            h407
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h408, _, _, _, _) => {
                            h408
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, h409, _, _, _) => {
                            h409
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, h410, _, _) => {
                            h410
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, h411, _) => {
                            h411
                        }
                    }, {
                        let a414 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h412) => {
                                h412
                            }
                        };
                        let a415 = List::cons({
                            let m413 = v2.clone();
                            match m413.uncons() {
                                None => {
                                    f106(v1.clone())
                                }
                                Some((v5, _)) => {
                                    v5
                                }
                            }
                        }, List::<Bytes>::nil());
                        append_list(a414, a415)
                    }))
                } else {
                    Err::<Adt14, Bytes>(f19(f16(Str::lit("runtime support: profile-root: ")), f19(f20(f19(f16(Str::lit("use ")), f83(v1.clone(), Str::lit("::"))?)), f19(f16(Str::lit(" starts at ")), f19(f20(v3), f16(Str::lit(", which the profile does not carry")))))))
                }
            }
        }
    }, f57(f107(Str::lit("a `use` segment"), v1.clone())?, f57(f52(Str::lit("a `use` alias"), v2)?, f62(f106(v1))?))))
}

pub fn f47(v0: Option<Bytes>, v1: Result<Adt14, Bytes>) -> Result<Adt14, Bytes> {
    let m416 = v0;
    match m416 {
        None => {
            v1
        }
        Some(v2) => {
            Err::<Adt14, Bytes>(v2)
        }
    }
}

pub fn f48(v0: Str, v1: Bytes) -> R<Option<Bytes>> {
    if if f108(v1.clone())? {
        let a417 = f66(f109(), v1.clone())?;
        bool_not(a417)
    } else {
        false
    } {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f19(f16(Str::lit("runtime support: identifier: ")), f19(f16(v0), f19(f16(Str::lit(" ")), f19(f20(v1), f16(Str::lit(" is not an identifier"))))))))
    }
}

pub fn f49(v0: Result<Adt14, Bytes>, v1: Option<Bytes>) -> Result<Adt14, Bytes> {
    let m418 = v0.clone();
    match m418 {
        Ok(_) => {
            let m419 = v1;
            match m419 {
                None => {
                    v0
                }
                Some(v4) => {
                    Err::<Adt14, Bytes>(v4)
                }
            }
        }
        Err(v2) => {
            Err::<Adt14, Bytes>(v2)
        }
    }
}

pub fn f50(v0: Adt14, v1: Bytes) -> R<Result<Adt14, Bytes>> {
    if f66(match v0.clone() {
        Adt14::C0(_, h420, _, _, _, _, _, _, _, _) => {
            h420
        }
    }, v1.clone())? {
        Ok(Err::<Adt14, Bytes>(f19(f16(Str::lit("runtime support: duplicate-name: ")), f19(f20(v1), f16(Str::lit(" is declared twice"))))))
    } else {
        Ok(Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
            Adt14::C0(h421, _, _, _, _, _, _, _, _, _) => {
                h421
            }
        }, {
            let a423 = match v0.clone() {
                Adt14::C0(_, h422, _, _, _, _, _, _, _, _) => {
                    h422
                }
            };
            let a424 = List::cons(v1, List::<Bytes>::nil());
            append_list(a423, a424)
        }, match v0.clone() {
            Adt14::C0(_, _, h425, _, _, _, _, _, _, _) => {
                h425
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h426, _, _, _, _, _, _) => {
                h426
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h427, _, _, _, _, _) => {
                h427
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h428, _, _, _, _) => {
                h428
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h429, _, _, _) => {
                h429
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h430, _, _) => {
                h430
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h431, _) => {
                h431
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h432) => {
                h432
            }
        })))
    }
}

pub fn f51(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f110(Fn11::F111(v0), None::<Bytes>, v1)
}

pub fn f52(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f110(Fn11::F112(v0), None::<Bytes>, v1)
}

pub fn f53(v0: Adt14, v1: List<Adt5>) -> Adt14 {
    f113(Fn12::F114, v0, v1)
}

pub fn f54(v0: Adt14, v1: Adt2, v2: List<Adt8>) -> Adt14 {
    let m433 = f115(v1);
    match m433 {
        None => {
            v0
        }
        Some(v3) => {
            f116(v0, v3, v2)
        }
    }
}

pub fn f55(v0: Adt14, v1: List<Adt8>) -> Adt14 {
    f117(Fn13::F118, v0, v1)
}

pub fn f56(v0: Adt2, v1: u64) -> R<List<Bytes>> {
    let m434 = v0;
    match m434 {
        Adt2::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C1 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C2 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C3 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C4 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C8(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C9(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C10(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C11 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C12 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C13 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C14 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C15(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C16 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C17 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C18 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C19(_, r437) => {
            let v11 = (*r437).clone();
            Ok(List::cons(f59(v1)?, f56(v11, {
                let a435 = v1;
                let a436 = 1u64;
                nat_add(a435, a436)?
            })?))
        }
    }
}

pub fn f57(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m438 = v0;
    match m438 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f58(v0: Adt2) -> R<u64> {
    let m439 = v0;
    match m439 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1 => {
            Ok(0u64)
        }
        Adt2::C2 => {
            Ok(0u64)
        }
        Adt2::C3 => {
            Ok(0u64)
        }
        Adt2::C4 => {
            Ok(0u64)
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(_) => {
            Ok(0u64)
        }
        Adt2::C7(_) => {
            Ok(0u64)
        }
        Adt2::C8(_) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_, _) => {
            Ok(0u64)
        }
        Adt2::C11 => {
            Ok(0u64)
        }
        Adt2::C12 => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14 => {
            Ok(0u64)
        }
        Adt2::C15(_) => {
            Ok(0u64)
        }
        Adt2::C16 => {
            Ok(0u64)
        }
        Adt2::C17 => {
            Ok(0u64)
        }
        Adt2::C18 => {
            Ok(0u64)
        }
        Adt2::C19(_, r442) => {
            let v10 = (*r442).clone();
            let a440 = f58(v10)?;
            let a441 = 1u64;
            nat_add(a440, a441)
        }
    }
}

pub fn f59(v0: u64) -> R<Bytes> {
    f119(v0)
}

pub fn f60(v0: Bytes, v1: List<Adt9>) -> R<Option<Bytes>> {
    let m444 = f120(Fn14::F121(v0), Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v1)?;
    match m444 {
        Adt18::C0(_, h443) => {
            Ok(h443)
        }
    }
}

pub fn f61(v0: List<Adt9>) -> R<Option<Bytes>> {
    let m446 = f120(Fn14::F122, Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m446 {
        Adt18::C0(_, h445) => {
            Ok(h445)
        }
    }
}

pub fn f62(v0: Bytes) -> R<Option<Bytes>> {
    let m447 = f123(v0.clone())?;
    match m447 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f19(f16(Str::lit("runtime support: hidden-effect: ")), f19(f20(v0), f19(f16(Str::lit(" realizes `")), f19(v1, f16(Str::lit("`, which the root does not admit"))))))))
        }
    }
}

pub fn f63(v0: Adt14, v1: Bytes) -> R<Option<u64>> {
    let m448 = f124(v0.clone(), v1.clone());
    match m448 {
        None => {
            let m450 = f125(v0.clone(), v1.clone());
            match m450 {
                None => {
                    let m452 = f126(v0, v1.clone());
                    match m452 {
                        None => {
                            f127(v1)
                        }
                        Some(v4) => {
                            Ok(Some(match v4 {
                                Adt13::C0(_, h453) => {
                                    h453
                                }
                            }))
                        }
                    }
                }
                Some(v3) => {
                    Ok(Some(match v3 {
                        Adt12::C0(_, _, h451) => {
                            h451
                        }
                    }))
                }
            }
        }
        Some(v2) => {
            Ok(Some(match v2 {
                Adt11::C0(_, _, h449) => {
                    h449
                }
            }))
        }
    }
}

pub fn f64(v0: Adt14, v1: Bytes) -> bool {
    let m454 = f124(v0.clone(), v1.clone());
    match m454 {
        None => {
            let m455 = f125(v0.clone(), v1.clone());
            match m455 {
                None => {
                    let m456 = f126(v0, v1.clone());
                    match m456 {
                        None => {
                            f128(v1)
                        }
                        Some(_) => {
                            false
                        }
                    }
                }
                Some(_) => {
                    false
                }
            }
        }
        Some(_) => {
            false
        }
    }
}

pub fn f65(v0: List<Adt17>, v1: Bytes) -> Option<Adt17> {
    f129(Fn15::F130(v1), None::<Adt17>, v0)
}

pub fn f66(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f101(Fn10::F131(v1), false, v0)
}

pub fn f67(v0: u64) -> R<Option<Bytes>> {
    let a457 = v0;
    let a458 = 1u64;
    if nat_eq(a457, a458) {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f19(f16(Str::lit("runtime support: char-literal: ")), f19(f16(Str::lit("a character literal of ")), f19(f59(v0)?, f16(Str::lit(" scalar values")))))))
    }
}

pub fn f68(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let m459 = f107(Str::lit("a path segment"), v1.clone())?;
    match m459 {
        None => {
            let m460 = f62(f132(v1.clone()))?;
            match m460 {
                None => {
                    f133(v0, v1)
                }
                Some(v3) => {
                    Ok(Some(v3))
                }
            }
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f69(v0: Adt14, v1: List<Bytes>, v2: u64) -> R<Option<Bytes>> {
    let m461 = f62(f132(v1.clone()))?;
    match m461 {
        None => {
            if f134(v0.clone(), v1.clone())? {
                Ok(Some(f19(f16(Str::lit("runtime support: hidden-allocation: ")), f19(f16(Str::lit("rust-core renders ")), f19(f20(f83(v1, Str::lit("::"))?), f16(Str::lit(", which requires heap allocation")))))))
            } else {
                let m462 = f135(v0, v1.clone())?;
                match m462 {
                    None => {
                        Ok(Some(f19(f16(Str::lit("runtime support: call-name: ")), f19(f20(f83(v1, Str::lit("::"))?), f16(Str::lit(" is neither declared nor in the closed table"))))))
                    }
                    Some(v4) => {
                        let a463 = v4;
                        let a464 = v2;
                        if nat_eq(a463, a464) {
                            Ok(None::<Bytes>)
                        } else {
                            Ok(Some(f19(f16(Str::lit("runtime support: call-arity: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" takes ")), f19(f59(v4)?, f19(f16(Str::lit(" arguments, given ")), f59(v2)?)))))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f70(v0: Adt3) -> R<u64> {
    let m465 = v0;
    match m465 {
        Adt3::C0(_) => {
            Ok(0u64)
        }
        Adt3::C1(_) => {
            Ok(0u64)
        }
        Adt3::C2(_) => {
            Ok(0u64)
        }
        Adt3::C3(_, _) => {
            Ok(0u64)
        }
        Adt3::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C5(_, _) => {
            Ok(0u64)
        }
        Adt3::C6(_, _) => {
            Ok(0u64)
        }
        Adt3::C7(_) => {
            Ok(0u64)
        }
        Adt3::C8(_, _) => {
            Ok(0u64)
        }
        Adt3::C9(_, _) => {
            Ok(0u64)
        }
        Adt3::C10(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C11(_, _) => {
            Ok(0u64)
        }
        Adt3::C12(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C13(_, _) => {
            Ok(0u64)
        }
        Adt3::C14(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C15(_) => {
            Ok(0u64)
        }
        Adt3::C16(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C17 => {
            Ok(0u64)
        }
        Adt3::C18(_) => {
            Ok(0u64)
        }
        Adt3::C19(_) => {
            Ok(0u64)
        }
        Adt3::C20(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C21(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C22(_) => {
            Ok(0u64)
        }
        Adt3::C23(_, _) => {
            Ok(0u64)
        }
        Adt3::C24(_, _) => {
            Ok(0u64)
        }
        Adt3::C25(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C26(_) => {
            Ok(0u64)
        }
        Adt3::C27(_) => {
            Ok(0u64)
        }
        Adt3::C28(_, _, _, _) => {
            Ok(0u64)
        }
        Adt3::C29(_, _) => {
            Ok(0u64)
        }
        Adt3::C30 => {
            Ok(0u64)
        }
        Adt3::C31 => {
            Ok(0u64)
        }
        Adt3::C32(_, r468) => {
            let v60 = (*r468).clone();
            let a466 = f70(v60)?;
            let a467 = 1u64;
            nat_add(a466, a467)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(0u64)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f71(v0: Adt14, v1: Adt16, p474: Adt3) -> R<Option<Bytes>> {
    let m473 = p474;
    {
        let mut m473 = m473;
        loop {
            let step = {
                let v2 = m473;
                let m469 = v2;
                match m469 {
                    Adt3::C0(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C32(r471, r472) => {
                        let v61 = (*r471).clone();
                        let v62 = (*r472).clone();
                        let m470 = f23(v0.clone(), v1.clone(), v61)?;
                        match m470 {
                            None => {
                                Ok::<Adt3, R<Option<Bytes>>>(v62)
                            }
                            Some(v63) => {
                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v63)))
                            }
                        }
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                }
            };
            match step {
                Ok(next) => { m473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt14, v1: Adt16, v2: Adt3, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m475 = f136(v3.clone())?;
    match m475 {
        None => {
            let v6: List<u64> = f138(v0.clone(), v3.clone());
            let m479 = v6.clone();
            match m479.uncons() {
                None => {
                    Ok(Some(f19(f16(Str::lit("runtime support: method-name: ")), f19(f20(v3), f16(Str::lit(" is a method of no declared trait or impl and not in the closed table"))))))
                }
                Some((_, _)) => {
                    if f139(v6.clone(), v4) {
                        f140(v0, v1, v2, v3)
                    } else {
                        Ok(Some(f19(f16(Str::lit("runtime support: method-arity: ")), f19(f20(v3), f19(f16(Str::lit(" takes ")), f19(f141(v6)?, f19(f16(Str::lit(" arguments, given ")), f59(v4)?)))))))
                    }
                }
            }
        }
        Some(v5) => {
            if if f137(v3.clone())? {
                match v0 {
                    Adt14::C0(h476, _, _, _, _, _, _, _, _, _) => {
                        h476
                    }
                }
            } else {
                false
            } {
                Ok(Some(f19(f16(Str::lit("runtime support: hidden-allocation: ")), f19(f16(Str::lit("rust-core renders ")), f19(f20(v3), f16(Str::lit(", which requires heap allocation")))))))
            } else {
                let a477 = v5;
                let a478 = v4;
                if nat_eq(a477, a478) {
                    Ok(None::<Bytes>)
                } else {
                    Ok(Some(f19(f16(Str::lit("runtime support: method-arity: ")), f19(f20(v3), f19(f16(Str::lit(" takes ")), f19(f59(v5)?, f19(f16(Str::lit(" arguments, given ")), f59(v4)?)))))))
                }
            }
        }
    }
}

pub fn f73(v0: Adt3) -> R<Option<Bytes>> {
    if f142(v0)? {
        Ok(Some(f19(f16(Str::lit("runtime support: struct-head: ")), f16(Str::lit("a struct literal outside parentheses in the head of a condition, a loop, or a match")))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f74(v0: Adt3) -> bool {
    let m480 = v0;
    match m480 {
        Adt3::C0(_) => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_, _) => {
            false
        }
        Adt3::C4(_, _, _) => {
            false
        }
        Adt3::C5(_, _) => {
            false
        }
        Adt3::C6(_, _) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_, _) => {
            false
        }
        Adt3::C9(_, _) => {
            false
        }
        Adt3::C10(_, _, _) => {
            false
        }
        Adt3::C11(_, _) => {
            false
        }
        Adt3::C12(_, _, _) => {
            false
        }
        Adt3::C13(_, _) => {
            false
        }
        Adt3::C14(_, _, _) => {
            false
        }
        Adt3::C15(_) => {
            false
        }
        Adt3::C16(_, _, _) => {
            false
        }
        Adt3::C17 => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _, _) => {
            false
        }
        Adt3::C21(_, _, _) => {
            false
        }
        Adt3::C22(_) => {
            false
        }
        Adt3::C23(_, _) => {
            false
        }
        Adt3::C24(_, _) => {
            false
        }
        Adt3::C25(_, _, _) => {
            false
        }
        Adt3::C26(_) => {
            false
        }
        Adt3::C27(_) => {
            false
        }
        Adt3::C28(_, _, _, _) => {
            false
        }
        Adt3::C29(_, _) => {
            false
        }
        Adt3::C30 => {
            false
        }
        Adt3::C31 => {
            true
        }
        Adt3::C32(_, _) => {
            false
        }
        Adt3::C33(_, _, _, _) => {
            false
        }
        Adt3::C34(_, _, _, _) => {
            false
        }
    }
}

pub fn f75(v0: Adt14, v1: Adt16, p490: Adt3) -> R<Option<Bytes>> {
    let m489 = p490;
    {
        let mut m489 = m489;
        loop {
            let step = {
                let v2 = m489;
                let m481 = v2;
                match m481 {
                    Adt3::C0(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C34(v67, r486, r487, r488) => {
                        let v68 = (*r486).clone();
                        let v69 = (*r487).clone();
                        let v70 = (*r488).clone();
                        let m482 = f79(v0.clone(), v67.clone())?;
                        match m482 {
                            None => {
                                let m483 = f80(List::<Adt17>::nil(), v67)?;
                                match m483 {
                                    Ok(v73) => {
                                        let v74: Adt16 = f78(v1.clone(), v73, false);
                                        let m484 = if f81(v68.clone()) {
                                            None::<Bytes>
                                        } else {
                                            f23(v0.clone(), v74.clone(), v68)?
                                        };
                                        match m484 {
                                            None => {
                                                let m485 = f23(v0.clone(), v74, v69)?;
                                                match m485 {
                                                    None => {
                                                        Ok::<Adt3, R<Option<Bytes>>>(v70)
                                                    }
                                                    Some(v76) => {
                                                        Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v76)))
                                                    }
                                                }
                                            }
                                            Some(v75) => {
                                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v75)))
                                            }
                                        }
                                    }
                                    Err(v72) => {
                                        Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v72)))
                                    }
                                }
                            }
                            Some(v71) => {
                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v71)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m489 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: Adt16, p501: Adt3) -> Option<Bytes> {
    let m500 = p501;
    {
        let mut m500 = m500;
        loop {
            let step = {
                let v1 = m500;
                let m491 = v1;
                match m491 {
                    Adt3::C0(v2) => {
                        let m493 = f65(match v0.clone() {
                            Adt16::C0(_, h492, _, _) => {
                                h492
                            }
                        }, v2.clone());
                        match m493 {
                            None => {
                                Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: variable-scope: ")), f19(f20(v2), f16(Str::lit(" is not in scope"))))))
                            }
                            Some(v3) => {
                                if match v3.clone() {
                                    Adt17::C0(_, h494, _) => {
                                        h494
                                    }
                                } {
                                    Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                } else {
                                    if f143(match v3 {
                                        Adt17::C0(_, _, h495) => {
                                            h495
                                        }
                                    }) {
                                        Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                    } else {
                                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-immutable: ")), f19(f20(v2), f16(Str::lit(" is assigned and is not `mut` or `&mut`"))))))
                                    }
                                }
                            }
                        }
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C5(r496, _) => {
                        let v11 = (*r496).clone();
                        Ok::<Adt3, Option<Bytes>>(v11)
                    }
                    Adt3::C6(r497, _) => {
                        let v13 = (*r497).clone();
                        Ok::<Adt3, Option<Bytes>>(v13)
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C24(r498, _) => {
                        let v48 = (*r498).clone();
                        Ok::<Adt3, Option<Bytes>>(v48)
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C26(r499) => {
                        let v53 = (*r499).clone();
                        Ok::<Adt3, Option<Bytes>>(v53)
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
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

pub fn f77(v0: Adt14, v1: Adt16, v2: Adt3) -> R<Option<Bytes>> {
    let m502 = v2;
    match m502 {
        Adt3::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C1(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C3(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C4(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C5(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C6(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C8(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C11(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C12(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C13(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C14(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C15(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C16(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C17 => {
            Ok(None::<Bytes>)
        }
        Adt3::C18(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C19(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C20(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C21(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C22(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C23(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C24(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C25(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C26(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C27(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C28(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C29(r504, r505) => {
            let v59 = (*r504).clone();
            let v60 = (*r505).clone();
            let m503 = f89(v0.clone(), v1, v59)?;
            match m503 {
                Ok(v62) => {
                    f23(v0, v62, v60)
                }
                Err(v61) => {
                    Ok(Some(v61))
                }
            }
        }
        Adt3::C30 => {
            Ok(None::<Bytes>)
        }
        Adt3::C31 => {
            Ok(None::<Bytes>)
        }
        Adt3::C32(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f78(v0: Adt16, v1: List<Adt17>, v2: bool) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt16::C0(h506, _, _, _) => {
            h506
        }
    }, {
        let a508 = v1;
        let a509 = match v0.clone() {
            Adt16::C0(_, h507, _, _) => {
                h507
            }
        };
        append_list(a508, a509)
    }, match v0.clone() {
        Adt16::C0(_, _, h510, _) => {
            h510
        }
    }, if match v0 {
        Adt16::C0(_, _, _, h511) => {
            h511
        }
    } {
        true
    } else {
        v2
    })
}

pub fn f79(v0: Adt14, v1: Adt4) -> R<Option<Bytes>> {
    let m512 = v1;
    match m512 {
        Adt4::C0 => {
            Ok(None::<Bytes>)
        }
        Adt4::C1(_) => {
            Ok(None::<Bytes>)
        }
        Adt4::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt4::C3(v4) => {
            f68(v0, v4)
        }
        Adt4::C4(r522) => {
            let v5 = (*r522).clone();
            f79(v0, v5)
        }
        Adt4::C5(v6, r523) => {
            let v7 = (*r523).clone();
            if f144(v6.clone()) {
                f79(v0, v7)
            } else {
                let m513 = f68(v0.clone(), v6.clone())?;
                match m513 {
                    None => {
                        let m514 = f145(v0.clone(), v6, v7.clone())?;
                        match m514 {
                            None => {
                                f79(v0, v7)
                            }
                            Some(v9) => {
                                Ok(Some(v9))
                            }
                        }
                    }
                    Some(v8) => {
                        Ok(Some(v8))
                    }
                }
            }
        }
        Adt4::C6(v10, v11, r524) => {
            let v12 = (*r524).clone();
            let m515 = f68(v0.clone(), v10.clone())?;
            match m515 {
                None => {
                    let m516 = f52(Str::lit("a pattern field name"), v11.clone())?;
                    match m516 {
                        None => {
                            let a518 = {
                                let a517 = v11.clone();
                                length_list(a517)
                            };
                            let a519 = f146(v12.clone())?;
                            if nat_eq(a518, a519) {
                                let m520 = f147(v0.clone(), v10, v11)?;
                                match m520 {
                                    None => {
                                        f79(v0, v12)
                                    }
                                    Some(v15) => {
                                        Ok(Some(v15))
                                    }
                                }
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: pattern-fields: ")), f16(Str::lit("a struct pattern with unequal names and patterns")))))
                            }
                        }
                        Some(v14) => {
                            Ok(Some(v14))
                        }
                    }
                }
                Some(v13) => {
                    Ok(Some(v13))
                }
            }
        }
        Adt4::C7(v16) => {
            f67(v16)
        }
        Adt4::C8 => {
            Ok(None::<Bytes>)
        }
        Adt4::C9(r525, r526) => {
            let v17 = (*r525).clone();
            let v18 = (*r526).clone();
            let m521 = f79(v0.clone(), v17)?;
            match m521 {
                None => {
                    f79(v0, v18)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
    }
}

pub fn f80(v0: List<Adt17>, v1: Adt4) -> R<Result<List<Adt17>, Bytes>> {
    let m527 = v1;
    match m527 {
        Adt4::C0 => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C1(v2) => {
            let m528 = f48(Str::lit("a pattern binding"), v2.clone())?;
            match m528 {
                None => {
                    let m529 = f65(v0.clone(), v2.clone());
                    match m529 {
                        None => {
                            Ok(Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(v2, false, Adt2::C16), v0)))
                        }
                        Some(_) => {
                            Ok(Err::<List<Adt17>, Bytes>(f19(f16(Str::lit("runtime support: pattern-duplicate: ")), f19(f20(v2), f16(Str::lit(" is bound twice in one pattern"))))))
                        }
                    }
                }
                Some(v3) => {
                    Ok(Err::<List<Adt17>, Bytes>(v3))
                }
            }
        }
        Adt4::C2(v5) => {
            let m530 = f48(Str::lit("a pattern binding"), v5.clone())?;
            match m530 {
                None => {
                    let m531 = f65(v0.clone(), v5.clone());
                    match m531 {
                        None => {
                            Ok(Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(v5, true, Adt2::C16), v0)))
                        }
                        Some(_) => {
                            Ok(Err::<List<Adt17>, Bytes>(f19(f16(Str::lit("runtime support: pattern-duplicate: ")), f19(f20(v5), f16(Str::lit(" is bound twice in one pattern"))))))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Err::<List<Adt17>, Bytes>(v6))
                }
            }
        }
        Adt4::C3(_) => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C4(r533) => {
            let v9 = (*r533).clone();
            f80(v0, v9)
        }
        Adt4::C5(_, r534) => {
            let v11 = (*r534).clone();
            f80(v0, v11)
        }
        Adt4::C6(_, _, r535) => {
            let v14 = (*r535).clone();
            f80(v0, v14)
        }
        Adt4::C7(_) => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C8 => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C9(r536, r537) => {
            let v16 = (*r536).clone();
            let v17 = (*r537).clone();
            let m532 = f80(v0, v16)?;
            match m532 {
                Ok(v18) => {
                    f80(v18, v17)
                }
                Err(v19) => {
                    Ok(Err::<List<Adt17>, Bytes>(v19))
                }
            }
        }
    }
}

pub fn f81(v0: Adt3) -> bool {
    let m538 = v0;
    match m538 {
        Adt3::C0(_) => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_, _) => {
            false
        }
        Adt3::C4(_, _, _) => {
            false
        }
        Adt3::C5(_, _) => {
            false
        }
        Adt3::C6(_, _) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_, _) => {
            false
        }
        Adt3::C9(_, _) => {
            false
        }
        Adt3::C10(_, _, _) => {
            false
        }
        Adt3::C11(_, _) => {
            false
        }
        Adt3::C12(_, _, _) => {
            false
        }
        Adt3::C13(_, _) => {
            false
        }
        Adt3::C14(_, _, _) => {
            false
        }
        Adt3::C15(_) => {
            false
        }
        Adt3::C16(_, _, _) => {
            false
        }
        Adt3::C17 => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _, _) => {
            false
        }
        Adt3::C21(_, _, _) => {
            false
        }
        Adt3::C22(_) => {
            false
        }
        Adt3::C23(_, _) => {
            false
        }
        Adt3::C24(_, _) => {
            false
        }
        Adt3::C25(_, _, _) => {
            false
        }
        Adt3::C26(_) => {
            false
        }
        Adt3::C27(_) => {
            false
        }
        Adt3::C28(_, _, _, _) => {
            false
        }
        Adt3::C29(_, _) => {
            false
        }
        Adt3::C30 => {
            true
        }
        Adt3::C31 => {
            false
        }
        Adt3::C32(_, _) => {
            false
        }
        Adt3::C33(_, _, _, _) => {
            false
        }
        Adt3::C34(_, _, _, _) => {
            false
        }
    }
}

pub fn f82(v0: Adt14, v1: List<Bytes>) -> Option<List<Bytes>> {
    let m539 = v1;
    match m539.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((v2, v3)) => {
            let m540 = v3;
            match m540.uncons() {
                None => {
                    let m541 = f124(v0, v2);
                    match m541 {
                        None => {
                            None::<List<Bytes>>
                        }
                        Some(v4) => {
                            Some(match v4 {
                                Adt11::C0(_, h542, _) => {
                                    h542
                                }
                            })
                        }
                    }
                }
                Some((v5, v6)) => {
                    let m543 = v6;
                    match m543.uncons() {
                        None => {
                            let m544 = f125(v0, v2);
                            match m544 {
                                None => {
                                    None::<List<Bytes>>
                                }
                                Some(v7) => {
                                    f148(match v7 {
                                        Adt12::C0(_, h545, _) => {
                                            h545
                                        }
                                    }, v5)
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<List<Bytes>>
                        }
                    }
                }
            }
        }
    }
}

pub fn f83(v0: List<Bytes>, v1: Str) -> R<Bytes> {
    f149(v0, f16(v1))
}

pub fn f84(v0: List<Bytes>, v1: List<Bytes>, v2: u64) -> R<bool> {
    let a548 = {
        let a546 = v0.clone();
        length_list(a546)
    };
    let a549 = {
        let a547 = v1.clone();
        length_list(a547)
    };
    if nat_eq(a548, a549) {
        let a551 = {
            let a550 = v0.clone();
            length_list(a550)
        };
        let a552 = v2;
        if nat_eq(a551, a552) {
            f101(Fn10::F150(v1.clone(), v0), true, v1)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f85(v0: Adt2) -> bool {
    let m553 = v0;
    match m553 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            false
        }
        Adt2::C17 => {
            true
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f86(v0: Str, v1: Adt2) -> Option<Bytes> {
    let m554 = v1;
    match m554 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1 => {
            None::<Bytes>
        }
        Adt2::C2 => {
            None::<Bytes>
        }
        Adt2::C3 => {
            None::<Bytes>
        }
        Adt2::C4 => {
            None::<Bytes>
        }
        Adt2::C5 => {
            None::<Bytes>
        }
        Adt2::C6(_) => {
            Some(f19(f16(Str::lit("runtime support: type-scope: ")), f19(f16(v0), f16(Str::lit(" names a generic parameter, which the printer writes without the generics in scope")))))
        }
        Adt2::C7(r557) => {
            let v3 = (*r557).clone();
            f86(v0, v3)
        }
        Adt2::C8(r558) => {
            let v4 = (*r558).clone();
            f86(v0, v4)
        }
        Adt2::C9(r559, r560) => {
            let v5 = (*r559).clone();
            let v6 = (*r560).clone();
            let m555 = f86(v0.clone(), v5);
            match m555 {
                None => {
                    f86(v0, v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r561) => {
            let v9 = (*r561).clone();
            f86(v0, v9)
        }
        Adt2::C11 => {
            None::<Bytes>
        }
        Adt2::C12 => {
            None::<Bytes>
        }
        Adt2::C13 => {
            None::<Bytes>
        }
        Adt2::C14 => {
            None::<Bytes>
        }
        Adt2::C15(r562) => {
            let v10 = (*r562).clone();
            f86(v0, v10)
        }
        Adt2::C16 => {
            None::<Bytes>
        }
        Adt2::C17 => {
            None::<Bytes>
        }
        Adt2::C18 => {
            None::<Bytes>
        }
        Adt2::C19(r563, r564) => {
            let v11 = (*r563).clone();
            let v12 = (*r564).clone();
            let m556 = f86(v0.clone(), v11);
            match m556 {
                None => {
                    f86(v0, v12)
                }
                Some(v13) => {
                    Some(v13)
                }
            }
        }
    }
}

pub fn f87(v0: Bytes) -> Option<u64> {
    let a565 = v0.clone();
    let a566 = f16(Str::lit("parse"));
    if equal(a565, a566) {
        Some(1u64)
    } else {
        let a567 = v0;
        let a568 = f16(Str::lit("collect"));
        if equal(a567, a568) {
            Some(1u64)
        } else {
            None::<u64>
        }
    }
}

pub fn f88(v0: Bytes) -> u64 {
    let a569 = v0.clone();
    let a570 = f16(Str::lit("parse"));
    if equal(a569, a570) {
        0u64
    } else {
        let a571 = v0;
        let a572 = f16(Str::lit("collect"));
        let _ = equal(a571, a572);
        0u64
    }
}

pub fn f89(v0: Adt14, p588: Adt16, p589: Adt3) -> R<Result<Adt16, Bytes>> {
    let m587 = (p588, p589);
    {
        let mut m587 = m587;
        loop {
            let step = {
                let (v1, v2) = m587;
                let m573 = v2;
                match m573 {
                    Adt3::C0(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C1(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C2(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C3(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C5(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C6(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C7(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C8(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C9(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C11(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C13(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C15(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C17 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C18(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C19(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C22(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C23(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C24(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C26(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C27(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C29(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C30 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C31 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C32(r583, r584) => {
                        let v61 = (*r583).clone();
                        let v62 = (*r584).clone();
                        let m574 = f23(v0.clone(), v1.clone(), v61)?;
                        match m574 {
                            None => {
                                Ok::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>((v1, v62))
                            }
                            Some(v63) => {
                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v63)))
                            }
                        }
                    }
                    Adt3::C33(v64, v65, r585, r586) => {
                        let v66 = (*r585).clone();
                        let v67 = (*r586).clone();
                        let m579 = if f85(v65.clone()) {
                            None::<Bytes>
                        } else {
                            f57(f22(v0.clone(), v65.clone(), {
                                let a576 = match v1.clone() {
                                    Adt16::C0(h575, _, _, _) => {
                                        h575
                                    }
                                };
                                length_list(a576)
                            }, {
                                let a578 = f85(match v1.clone() {
                                    Adt16::C0(_, _, h577, _) => {
                                        h577
                                    }
                                });
                                bool_not(a578)
                            })?, f86(Str::lit("a `let` type"), v65))
                        };
                        match m579 {
                            None => {
                                let m580 = f23(v0.clone(), v1.clone(), v66)?;
                                match m580 {
                                    None => {
                                        let m581 = f80(List::<Adt17>::nil(), v64.clone())?;
                                        match m581 {
                                            Ok(v71) => {
                                                let m582 = f79(v0.clone(), v64)?;
                                                match m582 {
                                                    None => {
                                                        Ok::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>((f78(v1, v71, false), v67))
                                                    }
                                                    Some(v72) => {
                                                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v72)))
                                                    }
                                                }
                                            }
                                            Err(v70) => {
                                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v70)))
                                            }
                                        }
                                    }
                                    Some(v69) => {
                                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v69)))
                                    }
                                }
                            }
                            Some(v68) => {
                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v68)))
                            }
                        }
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Fn8, p593: Adt18, p594: List<Adt5>) -> R<Adt18> {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v1, v2) = m592;
                let m590 = v2;
                match m590.uncons() {
                    None => {
                        Err::<(Adt18, List<Adt5>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Adt5>), R<Adt18>>(({
                            let c591: Fn8 = v0.clone();
                            c591.apply(v1, v3)?
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

pub fn f91(v0: Adt18, v1: Adt5) -> R<Adt18> {
    let m596 = match v0.clone() {
        Adt18::C0(_, h595) => {
            h595
        }
    };
    match m596 {
        None => {
            if f66(match v0.clone() {
                Adt18::C0(h597, _) => {
                    h597
                }
            }, match v1.clone() {
                Adt5::C0(h598, _, _, _, _) => {
                    h598
                }
            })? {
                Ok(Adt18::C0(match v0 {
                    Adt18::C0(h599, _) => {
                        h599
                    }
                }, Some(match v1 {
                    Adt5::C0(h600, _, _, _, _) => {
                        h600
                    }
                })))
            } else {
                Ok(Adt18::C0(List::cons(match v1 {
                    Adt5::C0(h601, _, _, _, _) => {
                        h601
                    }
                }, match v0 {
                    Adt18::C0(h602, _) => {
                        h602
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f92(v0: Adt14, v1: Adt5, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m604 = f48(Str::lit("a method name"), match v1.clone() {
        Adt5::C0(h603, _, _, _, _) => {
            h603
        }
    })?;
    match m604 {
        None => {
            if if match v1.clone() {
                Adt5::C0(_, _, h605, _, _) => {
                    h605
                }
            } {
                let a607 = match v1.clone() {
                    Adt5::C0(_, h606, _, _, _) => {
                        h606
                    }
                };
                bool_not(a607)
            } else {
                false
            } {
                Ok(Some(f19(f16(Str::lit("runtime support: function-receiver: ")), f19(f20(match v1 {
                    Adt5::C0(h608, _, _, _, _) => {
                        h608
                    }
                }), f16(Str::lit(" is exclusive without a receiver"))))))
            } else {
                let m610 = f151(v0.clone(), match v1.clone() {
                    Adt5::C0(_, _, _, h609, _) => {
                        h609
                    }
                }, v2, v3)?;
                match m610 {
                    None => {
                        f22(v0, match v1 {
                            Adt5::C0(_, _, _, _, h611) => {
                                h611
                            }
                        }, v2, v3)
                    }
                    Some(v5) => {
                        Ok(Some(v5))
                    }
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f93(v0: Fn9, p615: Option<Bytes>, p616: List<Adt7>) -> R<Option<Bytes>> {
    let m614 = (p615, p616);
    {
        let mut m614 = m614;
        loop {
            let step = {
                let (v1, v2) = m614;
                let m612 = v2;
                match m612.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(({
                            let c613: Fn9 = v0.clone();
                            c613.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt14, v1: Option<Bytes>, v2: Adt7) -> R<Option<Bytes>> {
    let m617 = v1;
    match m617 {
        None => {
            let m619 = f48(Str::lit("a generic parameter"), match v2.clone() {
                Adt7::C0(h618, _) => {
                    h618
                }
            })?;
            match m619 {
                None => {
                    f110(Fn11::F152(v2.clone(), v0), None::<Bytes>, match v2 {
                        Adt7::C0(_, h620) => {
                            h620
                        }
                    })
                }
                Some(v4) => {
                    Ok(Some(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f95(v0: Adt14, v1: Bytes) -> Option<List<Adt5>> {
    let m621 = f153(v0, v1.clone());
    match m621 {
        None => {
            let m623 = f154(v1.clone());
            match m623 {
                None => {
                    None::<List<Adt5>>
                }
                Some(v3) => {
                    Some(List::cons(Adt5::C0(v3, true, f155(v1.clone()), List::<Adt6>::nil(), if f155(v1) {
                        Adt2::C0
                    } else {
                        Adt2::C5
                    }), List::<Adt5>::nil()))
                }
            }
        }
        Some(v2) => {
            Some(match v2 {
                Adt10::C0(_, h622) => {
                    h622
                }
            })
        }
    }
}

pub fn f96(v0: List<Adt8>, v1: Bytes, v2: Option<Bytes>, v3: Adt5) -> R<Option<Bytes>> {
    let m624 = v2;
    match m624 {
        None => {
            let v5: u64 = f156(v0.clone(), match v3.clone() {
                Adt5::C0(h625, _, _, _, _) => {
                    h625
                }
            })?;
            let a626 = v5;
            let a627 = 0u64;
            if nat_eq(a626, a627) {
                Ok(Some(f19(f16(Str::lit("runtime support: impl-complete: ")), f19(f16(Str::lit("`impl ")), f19(v1, f19(f16(Str::lit("` does not state ")), f20(match v3 {
                    Adt5::C0(h628, _, _, _, _) => {
                        h628
                    }
                })))))))
            } else {
                let a629 = v5;
                let a630 = 1u64;
                if nat_eq(a629, a630) {
                    let m632 = f157(v0, match v3.clone() {
                        Adt5::C0(h631, _, _, _, _) => {
                            h631
                        }
                    });
                    match m632 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v6) => {
                            let a634 = f158(match v6 {
                                Adt8::C0(h633, _, _, _) => {
                                    h633
                                }
                            })?;
                            let a635 = f158(v3.clone())?;
                            if equal(a634, a635) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: impl-signature: ")), f19(f16(Str::lit("`impl ")), f19(v1, f19(f16(Str::lit("` states ")), f19(f20(match v3 {
                                    Adt5::C0(h636, _, _, _, _) => {
                                        h636
                                    }
                                }), f16(Str::lit(" with a signature other than the trait's")))))))))
                            }
                        }
                    }
                } else {
                    Ok(Some(f19(f16(Str::lit("runtime support: impl-complete: ")), f19(f16(Str::lit("`impl ")), f19(v1, f19(f16(Str::lit("` states ")), f19(f20(match v3 {
                        Adt5::C0(h637, _, _, _, _) => {
                            h637
                        }
                    }), f16(Str::lit(" twice")))))))))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f97(v0: List<Adt7>, v1: Adt14, v2: List<Adt5>, v3: Bytes, v4: Adt2, v5: Option<Bytes>, v6: Adt8) -> R<Option<Bytes>> {
    let m638 = v5;
    match m638 {
        None => {
            let a642 = f159(v2, {
                let m641 = match v6.clone() {
                    Adt8::C0(h640, _, _, _) => {
                        h640
                    }
                };
                match m641 {
                    Adt5::C0(h639, _, _, _, _) => {
                        h639
                    }
                }
            });
            if bool_not(a642) {
                Ok(Some(f19(f16(Str::lit("runtime support: impl-extra: ")), f19(f16(Str::lit("`impl ")), f19(v3, f19(f16(Str::lit("` states ")), f19(f20({
                    let m645 = match v6 {
                        Adt8::C0(h644, _, _, _) => {
                            h644
                        }
                    };
                    match m645 {
                        Adt5::C0(h643, _, _, _, _) => {
                            h643
                        }
                    }
                }), f16(Str::lit(", which the trait does not")))))))))
            } else {
                if match v6.clone() {
                    Adt8::C0(_, _, h646, _) => {
                        h646
                    }
                } {
                    Ok(Some(f19(f16(Str::lit("runtime support: impl-visibility: ")), f19(f20({
                        let m649 = match v6 {
                            Adt8::C0(h648, _, _, _) => {
                                h648
                            }
                        };
                        match m649 {
                            Adt5::C0(h647, _, _, _, _) => {
                                h647
                            }
                        }
                    }), f19(f16(Str::lit(" of `impl ")), f19(v3, f16(Str::lit("` is public; a trait method has the trait's visibility"))))))))
                } else {
                    f98(v1, v0, v4, v6)
                }
            }
        }
        Some(v7) => {
            Ok(Some(v7))
        }
    }
}

pub fn f98(v0: Adt14, v1: List<Adt7>, v2: Adt2, v3: Adt8) -> R<Option<Bytes>> {
    let m651 = f28(v0.clone(), match v3.clone() {
        Adt8::C0(_, h650, _, _) => {
            h650
        }
    })?;
    match m651 {
        None => {
            let v5: List<Adt7> = {
                let a653 = v1;
                let a654 = match v3.clone() {
                    Adt8::C0(_, h652, _, _) => {
                        h652
                    }
                };
                append_list(a653, a654)
            };
            let v6: u64 = {
                let a655 = v5.clone();
                length_list(a655)
            };
            let v7: Adt5 = match v3.clone() {
                Adt8::C0(h656, _, _, _) => {
                    h656
                }
            };
            let m658 = f92(v0.clone(), v7.clone(), v6, {
                let a657 = f85(v2.clone());
                bool_not(a657)
            })?;
            match m658 {
                None => {
                    let m663 = if match v7.clone() {
                        Adt5::C0(_, h659, _, _, _) => {
                            h659
                        }
                    } {
                        if f85(v2.clone()) {
                            Err::<List<Adt17>, Bytes>(f19(f16(Str::lit("runtime support: function-receiver: ")), f19(f20(match v7.clone() {
                                Adt5::C0(h660, _, _, _, _) => {
                                    h660
                                }
                            }), f16(Str::lit(" takes `&self` outside an impl")))))
                        } else {
                            Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(f16(Str::lit("self")), match v7.clone() {
                                Adt5::C0(_, _, h661, _, _) => {
                                    h661
                                }
                            }, if match v7.clone() {
                                Adt5::C0(_, _, h662, _, _) => {
                                    h662
                                }
                            } {
                                Adt2::C8(std::rc::Rc::new(v2.clone()))
                            } else {
                                Adt2::C7(std::rc::Rc::new(v2.clone()))
                            }), List::<Adt17>::nil()))
                        }
                    } else {
                        Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil())
                    };
                    match m663 {
                        Ok(v10) => {
                            let m666 = f160(Ok::<List<Adt17>, Bytes>(v10), match v7.clone() {
                                Adt5::C0(_, _, _, h664, _) => {
                                    h664
                                }
                            }, match v7 {
                                Adt5::C0(h665, _, _, _, _) => {
                                    h665
                                }
                            });
                            match m666 {
                                Ok(v12) => {
                                    f77(v0, Adt16::C0(v5, v12, v2, false), match v3 {
                                        Adt8::C0(_, _, _, h667) => {
                                            h667
                                        }
                                    })
                                }
                                Err(v11) => {
                                    Ok(Some(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Some(v9))
                        }
                    }
                }
                Some(v8) => {
                    Ok(Some(v8))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f99(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a668 = v0.clone();
        length_bytes(a668)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h669, _) = f161(Fn16::F162(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h669
    };
    let v3: u64 = {
        let (h670, _) = {
            let (_, h671) = v2.clone();
            h671
        };
        h670
    };
    Ok(f163(List::cons(f164(v0, v3, {
        let a672 = v1;
        let a673 = v3;
        nat_sub(a672, a673)
    }), {
        let (_, h674) = {
            let (_, h675) = v2;
            h675
        };
        h674
    })))
}

pub fn f100(v0: Adt14, v1: Bytes, v2: Adt1) -> R<Option<Bytes>> {
    let m676 = v2;
    match m676 {
        Adt1::C0(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C1(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C2(_, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C3(_, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C4(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C5(_, _, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C6(v4) => {
            f32(v0, v4)
        }
        Adt1::C7(_, _, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C8(_, _, _, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C9(_, _, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C10(v3) => {
            f35(v3)
        }
        Adt1::C11(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C12(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C13(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
    }
}

pub fn f101(v0: Fn10, p680: bool, p681: List<Bytes>) -> R<bool> {
    let m679 = (p680, p681);
    {
        let mut m679 = m679;
        loop {
            let step = {
                let (v1, v2) = m679;
                let m677 = v2;
                match m677.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c678: Fn10 = v0.clone();
                            c678.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: bool, v1: Bytes) -> bool {
    if v0 {
        true
    } else {
        let a683 = {
            let a682 = v1;
            length_bytes(a682)
        };
        let a684 = 0u64;
        nat_eq(a683, a684)
    }
}

pub fn f103(v0: Adt1, v1: Bytes) -> Option<Adt13> {
    let m685 = f39(v0);
    match m685 {
        None => {
            None::<Adt13>
        }
        Some(v2) => {
            if f40({
                let (h686, _) = v2;
                h686
            }, v1) {
                let m688 = f37({
                    let (_, h687) = v2;
                    h687
                });
                match m688 {
                    None => {
                        None::<Adt13>
                    }
                    Some(v3) => {
                        Some(Adt13::C0({
                            let m691 = match v3.clone() {
                                Adt8::C0(h690, _, _, _) => {
                                    h690
                                }
                            };
                            match m691 {
                                Adt5::C0(h689, _, _, _, _) => {
                                    h689
                                }
                            }
                        }, {
                            let a695 = {
                                let m694 = match v3 {
                                    Adt8::C0(h693, _, _, _) => {
                                        h693
                                    }
                                };
                                match m694 {
                                    Adt5::C0(_, _, _, h692, _) => {
                                        h692
                                    }
                                }
                            };
                            length_list(a695)
                        }))
                    }
                }
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f104(v0: List<Adt13>, v1: Bytes) -> R<u64> {
    f165(Fn17::F166(v1), 0u64, v0)
}

pub fn f105(v0: List<Adt13>, v1: Bytes) -> Option<u64> {
    f167(Fn18::F168(v1), None::<u64>, v0)
}

pub fn f106(v0: List<Bytes>) -> Bytes {
    f169(Fn19::F170, f16(Str::lit("")), v0)
}

pub fn f107(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f110(Fn11::F171(v0), None::<Bytes>, v1)
}

pub fn f108(v0: Bytes) -> R<bool> {
    let a697 = 0u64;
    let a698 = {
        let a696 = v0.clone();
        length_bytes(a696)
    };
    if nat_lt(a697, a698) {
        let a702 = {
            let (h699, _) = f172(Fn20::F173(v0.clone()), {
                let a700 = v0.clone();
                length_bytes(a700)
            }, 0u64)?;
            h699
        };
        let a703 = {
            let a701 = v0;
            length_bytes(a701)
        };
        Ok(nat_eq(a702, a703))
    } else {
        Ok(false)
    }
}

pub fn f109() -> List<Bytes> {
    List::cons(f16(Str::lit("abstract")), List::cons(f16(Str::lit("as")), List::cons(f16(Str::lit("async")), List::cons(f16(Str::lit("await")), List::cons(f16(Str::lit("become")), List::cons(f16(Str::lit("box")), List::cons(f16(Str::lit("break")), List::cons(f16(Str::lit("const")), List::cons(f16(Str::lit("continue")), List::cons(f16(Str::lit("crate")), List::cons(f16(Str::lit("do")), List::cons(f16(Str::lit("dyn")), List::cons(f16(Str::lit("else")), List::cons(f16(Str::lit("enum")), List::cons(f16(Str::lit("extern")), List::cons(f16(Str::lit("false")), List::cons(f16(Str::lit("final")), List::cons(f16(Str::lit("fn")), List::cons(f16(Str::lit("for")), List::cons(f16(Str::lit("if")), List::cons(f16(Str::lit("impl")), List::cons(f16(Str::lit("in")), List::cons(f16(Str::lit("let")), List::cons(f16(Str::lit("loop")), List::cons(f16(Str::lit("macro")), List::cons(f16(Str::lit("match")), List::cons(f16(Str::lit("mod")), List::cons(f16(Str::lit("move")), List::cons(f16(Str::lit("mut")), List::cons(f16(Str::lit("override")), List::cons(f16(Str::lit("priv")), List::cons(f16(Str::lit("pub")), List::cons(f16(Str::lit("ref")), List::cons(f16(Str::lit("return")), List::cons(f16(Str::lit("self")), List::cons(f16(Str::lit("Self")), List::cons(f16(Str::lit("static")), List::cons(f16(Str::lit("struct")), List::cons(f16(Str::lit("super")), List::cons(f16(Str::lit("trait")), List::cons(f16(Str::lit("true")), List::cons(f16(Str::lit("try")), List::cons(f16(Str::lit("type")), List::cons(f16(Str::lit("typeof")), List::cons(f16(Str::lit("unsafe")), List::cons(f16(Str::lit("unsized")), List::cons(f16(Str::lit("use")), List::cons(f16(Str::lit("virtual")), List::cons(f16(Str::lit("where")), List::cons(f16(Str::lit("while")), List::cons(f16(Str::lit("yield")), List::cons(f16(Str::lit("_")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f110(v0: Fn11, p707: Option<Bytes>, p708: List<Bytes>) -> R<Option<Bytes>> {
    let m706 = (p707, p708);
    {
        let mut m706 = m706;
        loop {
            let step = {
                let (v1, v2) = m706;
                let m704 = v2;
                match m704.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c705: Fn11 = v0.clone();
                            c705.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: Str, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m709 = v1;
    match m709 {
        None => {
            if if f40(v2.clone(), f16(Str::lit("Clone"))) {
                true
            } else {
                if f40(v2.clone(), f16(Str::lit("Copy"))) {
                    true
                } else {
                    if f40(v2.clone(), f16(Str::lit("Debug"))) {
                        true
                    } else {
                        if f40(v2.clone(), f16(Str::lit("Eq"))) {
                            true
                        } else {
                            f40(v2.clone(), f16(Str::lit("PartialEq")))
                        }
                    }
                }
            } {
                None::<Bytes>
            } else {
                Some(f19(f16(Str::lit("runtime support: derive: ")), f19(f16(v0), f19(f16(Str::lit(" derives ")), f19(f20(v2), f16(Str::lit(", which is not in the closed table")))))))
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f112(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m710 = v1;
    match m710 {
        None => {
            f48(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f113(v0: Fn12, p714: Adt14, p715: List<Adt5>) -> Adt14 {
    let m713 = (p714, p715);
    {
        let mut m713 = m713;
        loop {
            let step = {
                let (v1, v2) = m713;
                let m711 = v2;
                match m711.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt5>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt5>), Adt14>(({
                            let c712: Fn12 = v0.clone();
                            c712.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: Adt14, v1: Adt5) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h716, _, _, _, _, _, _, _, _, _) => {
            h716
        }
    }, match v0.clone() {
        Adt14::C0(_, h717, _, _, _, _, _, _, _, _) => {
            h717
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h718, _, _, _, _, _, _, _) => {
            h718
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h719, _, _, _, _, _, _) => {
            h719
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h720, _, _, _, _, _) => {
            h720
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h721, _, _, _, _) => {
            h721
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h722, _, _, _) => {
            h722
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h723, _, _) => {
            h723
        }
    }, {
        let a728 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h724, _) => {
                h724
            }
        };
        let a729 = List::cons(Adt13::C0(match v1.clone() {
            Adt5::C0(h725, _, _, _, _) => {
                h725
            }
        }, {
            let a727 = match v1 {
                Adt5::C0(_, _, _, h726, _) => {
                    h726
                }
            };
            length_list(a727)
        }), List::<Adt13>::nil());
        append_list(a728, a729)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h730) => {
            h730
        }
    })
}

pub fn f115(v0: Adt2) -> Option<Bytes> {
    let m731 = v0;
    match m731 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1 => {
            None::<Bytes>
        }
        Adt2::C2 => {
            None::<Bytes>
        }
        Adt2::C3 => {
            None::<Bytes>
        }
        Adt2::C4 => {
            None::<Bytes>
        }
        Adt2::C5 => {
            None::<Bytes>
        }
        Adt2::C6(_) => {
            None::<Bytes>
        }
        Adt2::C7(_) => {
            None::<Bytes>
        }
        Adt2::C8(_) => {
            None::<Bytes>
        }
        Adt2::C9(_, _) => {
            None::<Bytes>
        }
        Adt2::C10(v6, _) => {
            Some(v6)
        }
        Adt2::C11 => {
            None::<Bytes>
        }
        Adt2::C12 => {
            None::<Bytes>
        }
        Adt2::C13 => {
            None::<Bytes>
        }
        Adt2::C14 => {
            None::<Bytes>
        }
        Adt2::C15(_) => {
            None::<Bytes>
        }
        Adt2::C16 => {
            None::<Bytes>
        }
        Adt2::C17 => {
            None::<Bytes>
        }
        Adt2::C18 => {
            None::<Bytes>
        }
        Adt2::C19(_, _) => {
            None::<Bytes>
        }
    }
}

pub fn f116(v0: Adt14, v1: Bytes, v2: List<Adt8>) -> Adt14 {
    f117(Fn13::F174(v1), v0, v2)
}

pub fn f117(v0: Fn13, p735: Adt14, p736: List<Adt8>) -> Adt14 {
    let m734 = (p735, p736);
    {
        let mut m734 = m734;
        loop {
            let step = {
                let (v1, v2) = m734;
                let m732 = v2;
                match m732.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt8>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt8>), Adt14>(({
                            let c733: Fn13 = v0.clone();
                            c733.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Adt14, v1: Adt8) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h737, _, _, _, _, _, _, _, _, _) => {
            h737
        }
    }, match v0.clone() {
        Adt14::C0(_, h738, _, _, _, _, _, _, _, _) => {
            h738
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h739, _, _, _, _, _, _, _) => {
            h739
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h740, _, _, _, _, _, _) => {
            h740
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h741, _, _, _, _, _) => {
            h741
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h742, _, _, _, _) => {
            h742
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h743, _, _, _) => {
            h743
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h744, _, _) => {
            h744
        }
    }, {
        let a753 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h745, _) => {
                h745
            }
        };
        let a754 = List::cons(Adt13::C0({
            let m748 = match v1.clone() {
                Adt8::C0(h747, _, _, _) => {
                    h747
                }
            };
            match m748 {
                Adt5::C0(h746, _, _, _, _) => {
                    h746
                }
            }
        }, {
            let a752 = {
                let m751 = match v1 {
                    Adt8::C0(h750, _, _, _) => {
                        h750
                    }
                };
                match m751 {
                    Adt5::C0(_, _, _, h749, _) => {
                        h749
                    }
                }
            };
            length_list(a752)
        }), List::<Adt13>::nil());
        append_list(a753, a754)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h755) => {
            h755
        }
    })
}

pub fn f119(v0: u64) -> R<Bytes> {
    let v1: Bytes = f175(Str::lit("0"));
    let v2: Bytes = f176({
        let a792 = f177(v0, 10000000000000000000u64);
        let a793 = {
            let a790 = f177(v0, 1000000000000000000u64);
            let a791 = {
                let a788 = f177(v0, 100000000000000000u64);
                let a789 = {
                    let a786 = f177(v0, 10000000000000000u64);
                    let a787 = {
                        let a784 = f177(v0, 1000000000000000u64);
                        let a785 = {
                            let a782 = f177(v0, 100000000000000u64);
                            let a783 = {
                                let a780 = f177(v0, 10000000000000u64);
                                let a781 = {
                                    let a778 = f177(v0, 1000000000000u64);
                                    let a779 = {
                                        let a776 = f177(v0, 100000000000u64);
                                        let a777 = {
                                            let a774 = f177(v0, 10000000000u64);
                                            let a775 = {
                                                let a772 = f177(v0, 1000000000u64);
                                                let a773 = {
                                                    let a770 = f177(v0, 100000000u64);
                                                    let a771 = {
                                                        let a768 = f177(v0, 10000000u64);
                                                        let a769 = {
                                                            let a766 = f177(v0, 1000000u64);
                                                            let a767 = {
                                                                let a764 = f177(v0, 100000u64);
                                                                let a765 = {
                                                                    let a762 = f177(v0, 10000u64);
                                                                    let a763 = {
                                                                        let a760 = f177(v0, 1000u64);
                                                                        let a761 = {
                                                                            let a758 = f177(v0, 100u64);
                                                                            let a759 = {
                                                                                let a756 = f177(v0, 10u64);
                                                                                let a757 = f177(v0, 1u64);
                                                                                append_bytes(a756, a757)
                                                                            };
                                                                            append_bytes(a758, a759)
                                                                        };
                                                                        append_bytes(a760, a761)
                                                                    };
                                                                    append_bytes(a762, a763)
                                                                };
                                                                append_bytes(a764, a765)
                                                            };
                                                            append_bytes(a766, a767)
                                                        };
                                                        append_bytes(a768, a769)
                                                    };
                                                    append_bytes(a770, a771)
                                                };
                                                append_bytes(a772, a773)
                                            };
                                            append_bytes(a774, a775)
                                        };
                                        append_bytes(a776, a777)
                                    };
                                    append_bytes(a778, a779)
                                };
                                append_bytes(a780, a781)
                            };
                            append_bytes(a782, a783)
                        };
                        append_bytes(a784, a785)
                    };
                    append_bytes(a786, a787)
                };
                append_bytes(a788, a789)
            };
            append_bytes(a790, a791)
        };
        append_bytes(a792, a793)
    })?;
    let a794 = v0;
    let a795 = 0u64;
    if nat_eq(a794, a795) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f120(v0: Fn14, p799: Adt18, p800: List<Adt9>) -> R<Adt18> {
    let m798 = (p799, p800);
    {
        let mut m798 = m798;
        loop {
            let step = {
                let (v1, v2) = m798;
                let m796 = v2;
                match m796.uncons() {
                    None => {
                        Err::<(Adt18, List<Adt9>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Adt9>), R<Adt18>>(({
                            let c797: Fn14 = v0.clone();
                            c797.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m798 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Bytes, v1: Adt18, v2: Adt9) -> R<Adt18> {
    let m802 = match v1.clone() {
        Adt18::C0(_, h801) => {
            h801
        }
    };
    match m802 {
        None => {
            let m803 = f48(Str::lit("a variant name"), f178(v2.clone()))?;
            match m803 {
                None => {
                    let m806 = {
                        let m805 = v2.clone();
                        match m805 {
                            Adt9::C0(_) => {
                                None::<Bytes>
                            }
                            Adt9::C1(_, _) => {
                                None::<Bytes>
                            }
                            Adt9::C2(_, v9, _) => {
                                f52(Str::lit("a record variant field name"), v9)?
                            }
                        }
                    };
                    match m806 {
                        None => {
                            if f66(match v1.clone() {
                                Adt18::C0(h808, _) => {
                                    h808
                                }
                            }, f178(v2.clone()))? {
                                Ok(Adt18::C0(match v1 {
                                    Adt18::C0(h809, _) => {
                                        h809
                                    }
                                }, Some(f19(f16(Str::lit("runtime support: variant-name: ")), f19(f20(v0), f19(f16(Str::lit(" declares ")), f19(f20(f178(v2)), f16(Str::lit(" twice")))))))))
                            } else {
                                Ok(Adt18::C0(List::cons(f178(v2), match v1 {
                                    Adt18::C0(h810, _) => {
                                        h810
                                    }
                                }), None::<Bytes>))
                            }
                        }
                        Some(v11) => {
                            Ok(Adt18::C0(match v1 {
                                Adt18::C0(h807, _) => {
                                    h807
                                }
                            }, Some(v11)))
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt18::C0(match v1 {
                        Adt18::C0(h804, _) => {
                            h804
                        }
                    }, Some(v4)))
                }
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f122(v0: Adt18, v1: Adt9) -> R<Adt18> {
    let m812 = match v0.clone() {
        Adt18::C0(_, h811) => {
            h811
        }
    };
    match m812 {
        None => {
            if f66(match v0.clone() {
                Adt18::C0(h813, _) => {
                    h813
                }
            }, f178(v1.clone()))? {
                Ok(Adt18::C0(match v0 {
                    Adt18::C0(h814, _) => {
                        h814
                    }
                }, Some(f178(v1))))
            } else {
                Ok(Adt18::C0(List::cons(f178(v1), match v0 {
                    Adt18::C0(h815, _) => {
                        h815
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f123(v0: Bytes) -> R<Option<Bytes>> {
    if f66(List::cons(f16(Str::lit("fs")), List::cons(f16(Str::lit("Metadata")), List::cons(f16(Str::lit("MetadataExt")), List::<Bytes>::nil()))), v0.clone())? {
        Ok(Some(f16(Str::lit("files"))))
    } else {
        let a816 = v0.clone();
        let a817 = f16(Str::lit("File"));
        if equal(a816, a817) {
            Ok(Some(f16(Str::lit("files` or `process"))))
        } else {
            if f66(List::cons(f16(Str::lit("Command")), List::cons(f16(Str::lit("CommandExt")), List::cons(f16(Str::lit("Stdio")), List::cons(f16(Str::lit("Child")), List::cons(f16(Str::lit("Instant")), List::cons(f16(Str::lit("Duration")), List::cons(f16(Str::lit("thread")), List::<Bytes>::nil()))))))), v0)? {
                Ok(Some(f16(Str::lit("process"))))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f124(v0: Adt14, v1: Bytes) -> Option<Adt11> {
    f179(Fn21::F180(v1), None::<Adt11>, match v0 {
        Adt14::C0(_, _, _, h818, _, _, _, _, _, _) => {
            h818
        }
    })
}

pub fn f125(v0: Adt14, v1: Bytes) -> Option<Adt12> {
    f181(Fn22::F182(v1), None::<Adt12>, match v0 {
        Adt14::C0(_, _, _, _, h819, _, _, _, _, _) => {
            h819
        }
    })
}

pub fn f126(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f183(Fn23::F184(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, h820, _, _, _, _) => {
            h820
        }
    })
}

pub fn f127(v0: Bytes) -> R<Option<u64>> {
    let a821 = v0.clone();
    let a822 = f16(Str::lit("Result"));
    if equal(a821, a822) {
        Ok(Some(2u64))
    } else {
        if f66(List::cons(f16(Str::lit("Option")), List::cons(f16(Str::lit("Rc")), List::cons(f16(Str::lit("Vec")), List::<Bytes>::nil()))), v0.clone())? {
            Ok(Some(1u64))
        } else {
            if f66(List::cons(f16(Str::lit("AtomicU64")), List::cons(f16(Str::lit("File")), List::cons(f16(Str::lit("Metadata")), List::cons(f16(Str::lit("Command")), List::cons(f16(Str::lit("Child")), List::cons(f16(Str::lit("Instant")), List::cons(f16(Str::lit("Error")), List::cons(f16(Str::lit("String")), List::<Bytes>::nil())))))))), v0)? {
                Ok(Some(0u64))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f128(v0: Bytes) -> bool {
    let a823 = v0.clone();
    let a824 = f16(Str::lit("Rc"));
    if equal(a823, a824) {
        true
    } else {
        let a825 = v0.clone();
        let a826 = f16(Str::lit("Vec"));
        if equal(a825, a826) {
            true
        } else {
            let a827 = v0;
            let a828 = f16(Str::lit("String"));
            equal(a827, a828)
        }
    }
}

pub fn f129(v0: Fn15, p832: Option<Adt17>, p833: List<Adt17>) -> Option<Adt17> {
    let m831 = (p832, p833);
    {
        let mut m831 = m831;
        loop {
            let step = {
                let (v1, v2) = m831;
                let m829 = v2;
                match m829.uncons() {
                    None => {
                        Err::<(Option<Adt17>, List<Adt17>), Option<Adt17>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt17>, List<Adt17>), Option<Adt17>>(({
                            let c830: Fn15 = v0.clone();
                            c830.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m831 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: Bytes, v1: Option<Adt17>, v2: Adt17) -> Option<Adt17> {
    let m834 = v1;
    match m834 {
        None => {
            if f40(match v2.clone() {
                Adt17::C0(h835, _, _) => {
                    h835
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt17>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f131(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        f40(v2, v0)
    }
}

pub fn f132(v0: List<Bytes>) -> Bytes {
    let m836 = v0;
    match m836.uncons() {
        None => {
            f16(Str::lit(""))
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f133(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let v2: Bytes = f132(v1.clone());
    let m837 = f125(v0.clone(), v2.clone());
    match m837 {
        None => {
            if if f66(match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h841) => {
                    h841
                }
            }, v2.clone())? {
                true
            } else {
                let a843 = {
                    let a842 = v1.clone();
                    length_list(a842)
                };
                let a844 = 1u64;
                if if nat_eq(a843, a844) {
                    let m845 = f187(v0.clone(), v2.clone());
                    match m845 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                } else {
                    false
                } {
                    true
                } else {
                    let m846 = f124(v0.clone(), v2.clone());
                    if match m846 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    } {
                        true
                    } else {
                        if f66(match v0 {
                            Adt14::C0(_, _, _, _, _, _, h847, _, _, _) => {
                                h847
                            }
                        }, v2.clone())? {
                            true
                        } else {
                            if f188(v2.clone()) {
                                true
                            } else {
                                if f40(v2.clone(), f16(Str::lit("None"))) {
                                    true
                                } else {
                                    if f40(v2.clone(), f16(Str::lit("Self"))) {
                                        true
                                    } else {
                                        f40(v2.clone(), f16(Str::lit("usize")))
                                    }
                                }
                            }
                        }
                    }
                }
            } {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: path-name: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" starts at ")), f19(f20(v2), f16(Str::lit(", which no `use` or item declares"))))))))
            }
        }
        Some(v3) => {
            let m838 = f185(v1.clone(), 1u64);
            match m838 {
                None => {
                    Ok(Some(f19(f16(Str::lit("runtime support: variant-name: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" is not a variant of ")), f20(v2))))))
                }
                Some(v4) => {
                    let m840 = f186(match v3 {
                        Adt12::C0(_, h839, _) => {
                            h839
                        }
                    }, v4);
                    match m840 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: variant-name: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" is not a variant of ")), f20(v2))))))
                        }
                        Some(_) => {
                            Ok(None::<Bytes>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f134(v0: Adt14, v1: List<Bytes>) -> R<bool> {
    if match v0 {
        Adt14::C0(h848, _, _, _, _, _, _, _, _, _) => {
            h848
        }
    } {
        let m849 = v1;
        match m849.uncons() {
            None => {
                Ok(false)
            }
            Some((v2, v3)) => {
                let m850 = v3;
                match m850.uncons() {
                    None => {
                        Ok(false)
                    }
                    Some((v4, v5)) => {
                        let m851 = v5;
                        match m851.uncons() {
                            None => {
                                f189(v2, v4)
                            }
                            Some((_, _)) => {
                                Ok(false)
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f135(v0: Adt14, v1: List<Bytes>) -> R<Option<u64>> {
    let m852 = v1;
    match m852.uncons() {
        None => {
            Ok(None::<u64>)
        }
        Some((v2, v3)) => {
            let m853 = v3;
            match m853.uncons() {
                None => {
                    let m854 = f187(v0.clone(), v2.clone());
                    match m854 {
                        None => {
                            let m856 = f190(v2.clone());
                            match m856 {
                                None => {
                                    Ok(f191(v0, v2))
                                }
                                Some(v5) => {
                                    Ok(Some(v5))
                                }
                            }
                        }
                        Some(v4) => {
                            Ok(Some(match v4 {
                                Adt13::C0(_, h855) => {
                                    h855
                                }
                            }))
                        }
                    }
                }
                Some((v6, v7)) => {
                    let m857 = v7;
                    match m857.uncons() {
                        None => {
                            let m858 = f192(v2.clone(), v6.clone())?;
                            match m858 {
                                None => {
                                    let m859 = f193(v0.clone(), v2.clone(), v6.clone())?;
                                    match m859 {
                                        None => {
                                            Ok(f194(v0, v2, v6))
                                        }
                                        Some(v9) => {
                                            Ok(Some(v9))
                                        }
                                    }
                                }
                                Some(v8) => {
                                    Ok(Some(v8))
                                }
                            }
                        }
                        Some((_, _)) => {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f136(v0: Bytes) -> R<Option<u64>> {
    if f66(List::cons(f16(Str::lit("fetch_add")), List::cons(f16(Str::lit("env")), List::<Bytes>::nil())), v0.clone())? {
        Ok(Some(2u64))
    } else {
        if f66(List::cons(f16(Str::lit("load")), List::cons(f16(Str::lit("cmp")), List::cons(f16(Str::lit("min")), List::cons(f16(Str::lit("max")), List::cons(f16(Str::lit("push")), List::cons(f16(Str::lit("truncate")), List::cons(f16(Str::lit("push_str")), List::cons(f16(Str::lit("extend_from_slice")), List::cons(f16(Str::lit("get")), List::cons(f16(Str::lit("strip_prefix")), List::cons(f16(Str::lit("starts_with")), List::cons(f16(Str::lit("split")), List::cons(f16(Str::lit("contains")), List::cons(f16(Str::lit("skip")), List::cons(f16(Str::lit("saturating_add")), List::cons(f16(Str::lit("arg")), List::cons(f16(Str::lit("current_dir")), List::cons(f16(Str::lit("stdin")), List::cons(f16(Str::lit("stdout")), List::cons(f16(Str::lit("stderr")), List::cons(f16(Str::lit("process_group")), List::cons(f16(Str::lit("stack_size")), List::cons(f16(Str::lit("spawn")), List::cons(f16(Str::lit("write_all")), List::<Bytes>::nil())))))))))))))))))))))))), v0.clone())? {
            Ok(Some(1u64))
        } else {
            if f66(List::cons(f16(Str::lit("len")), List::cons(f16(Str::lit("is_empty")), List::cons(f16(Str::lit("chars")), List::cons(f16(Str::lit("next")), List::cons(f16(Str::lit("pop")), List::cons(f16(Str::lit("iter")), List::cons(f16(Str::lit("rev")), List::cons(f16(Str::lit("copied")), List::cons(f16(Str::lit("ok")), List::cons(f16(Str::lit("clone")), List::cons(f16(Str::lit("to_string")), List::cons(f16(Str::lit("as_bytes")), List::cons(f16(Str::lit("collect")), List::cons(f16(Str::lit("is_ascii_digit")), List::cons(f16(Str::lit("kind")), List::cons(f16(Str::lit("file_name")), List::cons(f16(Str::lit("into_encoded_bytes")), List::cons(f16(Str::lit("into_os_string")), List::cons(f16(Str::lit("is_symlink")), List::cons(f16(Str::lit("is_file")), List::cons(f16(Str::lit("is_dir")), List::cons(f16(Str::lit("sort")), List::cons(f16(Str::lit("elapsed")), List::cons(f16(Str::lit("as_millis")), List::cons(f16(Str::lit("try_wait")), List::cons(f16(Str::lit("kill")), List::cons(f16(Str::lit("wait")), List::cons(f16(Str::lit("id")), List::cons(f16(Str::lit("code")), List::cons(f16(Str::lit("env_clear")), List::cons(f16(Str::lit("status")), List::cons(f16(Str::lit("join")), List::cons(f16(Str::lit("flush")), List::cons(f16(Str::lit("is_terminal")), List::cons(f16(Str::lit("lock")), List::cons(f16(Str::lit("dev")), List::cons(f16(Str::lit("ino")), List::cons(f16(Str::lit("text")), List::cons(f16(Str::lit("octets")), List::cons(f16(Str::lit("uncons")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))), v0)? {
                Ok(Some(0u64))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f137(v0: Bytes) -> R<bool> {
    f66(List::cons(f16(Str::lit("push")), List::cons(f16(Str::lit("pop")), List::cons(f16(Str::lit("truncate")), List::cons(f16(Str::lit("push_str")), List::cons(f16(Str::lit("extend_from_slice")), List::cons(f16(Str::lit("to_string")), List::cons(f16(Str::lit("collect")), List::cons(f16(Str::lit("sort")), List::<Bytes>::nil())))))))), v0)
}

pub fn f138(v0: Adt14, v1: Bytes) -> List<u64> {
    f195(Fn24::F196(v1), List::<u64>::nil(), match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, h860, _) => {
            h860
        }
    })
}

pub fn f139(v0: List<u64>, v1: u64) -> bool {
    f197(Fn25::F198(v1), false, v0)
}

pub fn f140(v0: Adt14, v1: Adt16, v2: Adt3, v3: Bytes) -> R<Option<Bytes>> {
    let m861 = f199(f200(v1.clone(), v2));
    match m861 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1 => {
            Ok(None::<Bytes>)
        }
        Adt2::C2 => {
            Ok(None::<Bytes>)
        }
        Adt2::C3 => {
            Ok(None::<Bytes>)
        }
        Adt2::C4 => {
            Ok(None::<Bytes>)
        }
        Adt2::C5 => {
            Ok(None::<Bytes>)
        }
        Adt2::C6(v4) => {
            if f201(v0, f202(match v1 {
                Adt16::C0(h862, _, _, _) => {
                    h862
                }
            }, v4), v3.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: method-bound: ")), f19(f20(v3), f16(Str::lit(" is called on a parameter whose bounds do not provide it"))))))
            }
        }
        Adt2::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C11 => {
            Ok(None::<Bytes>)
        }
        Adt2::C12 => {
            Ok(None::<Bytes>)
        }
        Adt2::C13 => {
            Ok(None::<Bytes>)
        }
        Adt2::C14 => {
            Ok(None::<Bytes>)
        }
        Adt2::C15(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C16 => {
            Ok(None::<Bytes>)
        }
        Adt2::C17 => {
            Ok(None::<Bytes>)
        }
        Adt2::C18 => {
            Ok(None::<Bytes>)
        }
        Adt2::C19(_, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f141(v0: List<u64>) -> R<Bytes> {
    f203(Fn26::F204, f16(Str::lit("")), v0)
}

pub fn f142(v0: Adt3) -> R<bool> {
    let m863 = v0;
    match m863 {
        Adt3::C0(_) => {
            Ok(false)
        }
        Adt3::C1(_) => {
            Ok(false)
        }
        Adt3::C2(_) => {
            Ok(false)
        }
        Adt3::C3(_, _) => {
            Ok(false)
        }
        Adt3::C4(r894, _, _) => {
            let v6 = (*r894).clone();
            let a864 = 13u64;
            let a865 = f205(v6.clone());
            if nat_le(a864, a865) {
                f142(v6)
            } else {
                Ok(false)
            }
        }
        Adt3::C5(r895, _) => {
            let v9 = (*r895).clone();
            let a866 = 13u64;
            let a867 = f205(v9.clone());
            if nat_le(a866, a867) {
                f142(v9)
            } else {
                Ok(false)
            }
        }
        Adt3::C6(r896, _) => {
            let v11 = (*r896).clone();
            let a868 = 13u64;
            let a869 = f205(v11.clone());
            if nat_le(a868, a869) {
                f142(v11)
            } else {
                Ok(false)
            }
        }
        Adt3::C7(r897) => {
            let v13 = (*r897).clone();
            let a870 = 12u64;
            let a871 = f205(v13.clone());
            if nat_le(a870, a871) {
                f142(v13)
            } else {
                Ok(false)
            }
        }
        Adt3::C8(r898, r899) => {
            let v14 = (*r898).clone();
            let v15 = (*r899).clone();
            let a872 = 6u64;
            let a873 = f205(v14.clone());
            if if nat_le(a872, a873) {
                f142(v14)?
            } else {
                false
            } {
                Ok(true)
            } else {
                let a874 = 6u64;
                let a875 = f205(v15.clone());
                if nat_le(a874, a875) {
                    f142(v15)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C9(_, _) => {
            Ok(false)
        }
        Adt3::C10(_, _, _) => {
            Ok(false)
        }
        Adt3::C11(r900, r901) => {
            let v21 = (*r900).clone();
            let v22 = (*r901).clone();
            if f142(v21)? {
                Ok(true)
            } else {
                f142(v22)
            }
        }
        Adt3::C12(_, _, _) => {
            Ok(false)
        }
        Adt3::C13(_, _) => {
            Ok(false)
        }
        Adt3::C14(_, _, _) => {
            Ok(false)
        }
        Adt3::C15(_) => {
            Ok(false)
        }
        Adt3::C16(_, _, _) => {
            Ok(false)
        }
        Adt3::C17 => {
            Ok(false)
        }
        Adt3::C18(r902) => {
            let v35 = (*r902).clone();
            f142(v35)
        }
        Adt3::C19(_) => {
            Ok(false)
        }
        Adt3::C20(_, _, _) => {
            Ok(true)
        }
        Adt3::C21(v40, r903, r904) => {
            let v41 = (*r903).clone();
            let v42 = (*r904).clone();
            if if f206(v40, v41.clone()) {
                false
            } else {
                f142(v41)?
            } {
                Ok(true)
            } else {
                let a878 = {
                    let a876 = f207(v40);
                    let a877 = 1u64;
                    nat_add(a876, a877)?
                };
                let a879 = f205(v42.clone());
                if nat_le(a878, a879) {
                    f142(v42)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C22(r905) => {
            let v43 = (*r905).clone();
            let a880 = 12u64;
            let a881 = f205(v43.clone());
            if nat_le(a880, a881) {
                f142(v43)
            } else {
                Ok(false)
            }
        }
        Adt3::C23(r906, _) => {
            let v44 = (*r906).clone();
            let a882 = 11u64;
            let a883 = f205(v44.clone());
            if nat_le(a882, a883) {
                f142(v44)
            } else {
                Ok(false)
            }
        }
        Adt3::C24(r907, _) => {
            let v46 = (*r907).clone();
            let a884 = 13u64;
            let a885 = f205(v46.clone());
            if nat_le(a884, a885) {
                f142(v46)
            } else {
                Ok(false)
            }
        }
        Adt3::C25(r908, _, _) => {
            let v48 = (*r908).clone();
            let a886 = 13u64;
            let a887 = f205(v48.clone());
            if nat_le(a886, a887) {
                f142(v48)
            } else {
                Ok(false)
            }
        }
        Adt3::C26(r909) => {
            let v51 = (*r909).clone();
            let a888 = 12u64;
            let a889 = f205(v51.clone());
            if nat_le(a888, a889) {
                f142(v51)
            } else {
                Ok(false)
            }
        }
        Adt3::C27(r910) => {
            let v52 = (*r910).clone();
            let a890 = 12u64;
            let a891 = f205(v52.clone());
            if nat_le(a890, a891) {
                f142(v52)
            } else {
                Ok(false)
            }
        }
        Adt3::C28(r911, _, _, _) => {
            let v53 = (*r911).clone();
            let a892 = 13u64;
            let a893 = f205(v53.clone());
            if nat_le(a892, a893) {
                f142(v53)
            } else {
                Ok(false)
            }
        }
        Adt3::C29(_, _) => {
            Ok(false)
        }
        Adt3::C30 => {
            Ok(false)
        }
        Adt3::C31 => {
            Ok(false)
        }
        Adt3::C32(_, _) => {
            Ok(false)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(false)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(false)
        }
    }
}

pub fn f143(v0: Adt2) -> bool {
    let m912 = v0;
    match m912 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            true
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            false
        }
        Adt2::C17 => {
            false
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f144(v0: List<Bytes>) -> bool {
    let m913 = v0;
    match m913.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let m914 = v2;
            match m914.uncons() {
                None => {
                    let m915 = f190(v1);
                    match m915 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }
                Some((_, _)) => {
                    false
                }
            }
        }
    }
}

pub fn f145(v0: Adt14, v1: List<Bytes>, v2: Adt4) -> R<Option<Bytes>> {
    let m916 = f185(v1.clone(), 1u64);
    match m916 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            let m917 = f125(v0, f132(v1.clone()));
            match m917 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v4) => {
                    let m919 = f186(match v4 {
                        Adt12::C0(_, h918, _) => {
                            h918
                        }
                    }, v3);
                    match m919 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: pattern-fields: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" is matched with ")), f19(f59(f146(v2)?)?, f16(Str::lit(" patterns"))))))))
                        }
                        Some(v5) => {
                            let a920 = f208(v5)?;
                            let a921 = f146(v2.clone())?;
                            if nat_eq(a920, a921) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: pattern-fields: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" is matched with ")), f19(f59(f146(v2)?)?, f16(Str::lit(" patterns"))))))))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f146(v0: Adt4) -> R<u64> {
    let m922 = v0;
    match m922 {
        Adt4::C0 => {
            Ok(0u64)
        }
        Adt4::C1(_) => {
            Ok(0u64)
        }
        Adt4::C2(_) => {
            Ok(0u64)
        }
        Adt4::C3(_) => {
            Ok(0u64)
        }
        Adt4::C4(_) => {
            Ok(0u64)
        }
        Adt4::C5(_, _) => {
            Ok(0u64)
        }
        Adt4::C6(_, _, _) => {
            Ok(0u64)
        }
        Adt4::C7(_) => {
            Ok(0u64)
        }
        Adt4::C8 => {
            Ok(0u64)
        }
        Adt4::C9(_, r925) => {
            let v12 = (*r925).clone();
            let a923 = f146(v12)?;
            let a924 = 1u64;
            nat_add(a923, a924)
        }
    }
}

pub fn f147(v0: Adt14, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m926 = f82(v0, v1.clone());
    match m926 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            f110(Fn11::F209(v3, v1), None::<Bytes>, v2)
        }
    }
}

pub fn f148(v0: List<Adt9>, v1: Bytes) -> Option<List<Bytes>> {
    let m927 = f186(v0, v1);
    match m927 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m928 = v2;
            match m928 {
                Adt9::C0(_) => {
                    None::<List<Bytes>>
                }
                Adt9::C1(_, _) => {
                    None::<List<Bytes>>
                }
                Adt9::C2(_, v7, _) => {
                    Some(v7)
                }
            }
        }
    }
}

pub fn f149(v0: List<Bytes>, v1: Bytes) -> R<Bytes> {
    let m929 = f110(Fn11::F210(v1), None::<Bytes>, v0)?;
    match m929 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f150(v0: List<Bytes>, v1: List<Bytes>, v2: bool, v3: Bytes) -> R<bool> {
    if v2 {
        let a930 = f211(v1, v3.clone())?;
        let a931 = f211(v0, v3)?;
        Ok(nat_eq(a930, a931))
    } else {
        Ok(false)
    }
}

pub fn f151(v0: Adt14, v1: List<Adt6>, v2: u64, v3: bool) -> R<Option<Bytes>> {
    f212(Fn27::F213(v3, v2, v0), None::<Bytes>, v1)
}

pub fn f152(v0: Adt7, v1: Adt14, v2: Option<Bytes>, v3: Bytes) -> Option<Bytes> {
    let m932 = v2;
    match m932 {
        None => {
            let m933 = f153(v1, v3.clone());
            if match m933 {
                None => {
                    let m934 = f154(v3.clone());
                    match m934 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }
                Some(_) => {
                    true
                }
            } {
                None::<Bytes>
            } else {
                Some(f19(f16(Str::lit("runtime support: bound-trait: ")), f19(f20(f19(match v0 {
                    Adt7::C0(h935, _) => {
                        h935
                    }
                }, f19(f16(Str::lit(": ")), v3))), f16(Str::lit(" names no declared trait")))))
            }
        }
        Some(v4) => {
            Some(v4)
        }
    }
}

pub fn f153(v0: Adt14, v1: Bytes) -> Option<Adt10> {
    f214(Fn28::F215(v1), None::<Adt10>, match v0 {
        Adt14::C0(_, _, h936, _, _, _, _, _, _, _) => {
            h936
        }
    })
}

pub fn f154(v0: Bytes) -> Option<Bytes> {
    let a937 = v0.clone();
    let a938 = f16(Str::lit("Clone"));
    if equal(a937, a938) {
        Some(f16(Str::lit("clone")))
    } else {
        let a939 = v0;
        let a940 = f16(Str::lit("Drop"));
        if equal(a939, a940) {
            Some(f16(Str::lit("drop")))
        } else {
            None::<Bytes>
        }
    }
}

pub fn f155(v0: Bytes) -> bool {
    let a941 = v0.clone();
    let a942 = f16(Str::lit("Clone"));
    if equal(a941, a942) {
        false
    } else {
        let a943 = v0;
        let a944 = f16(Str::lit("Drop"));
        equal(a943, a944)
    }
}

pub fn f156(v0: List<Adt8>, v1: Bytes) -> R<u64> {
    f216(Fn29::F217(v1), 0u64, v0)
}

pub fn f157(v0: List<Adt8>, v1: Bytes) -> Option<Adt8> {
    f218(Fn30::F219(v1), None::<Adt8>, v0)
}

pub fn f158(v0: Adt5) -> R<Bytes> {
    Ok(f19(match v0.clone() {
        Adt5::C0(h945, _, _, _, _) => {
            h945
        }
    }, f19(f16(Str::lit("|")), f19(if match v0.clone() {
        Adt5::C0(_, h946, _, _, _) => {
            h946
        }
    } {
        f16(Str::lit("r"))
    } else {
        f16(Str::lit("-"))
    }, f19(if match v0.clone() {
        Adt5::C0(_, _, h947, _, _) => {
            h947
        }
    } {
        f16(Str::lit("x"))
    } else {
        f16(Str::lit("-"))
    }, f19(f16(Str::lit("|")), f19(f220(Fn31::F221, f16(Str::lit("")), match v0.clone() {
        Adt5::C0(_, _, _, h948, _) => {
            h948
        }
    })?, f19(f16(Str::lit("|")), f222(match v0 {
        Adt5::C0(_, _, _, _, h949) => {
            h949
        }
    })?))))))))
}

pub fn f159(v0: List<Adt5>, v1: Bytes) -> bool {
    f223(Fn32::F224(v1), false, v0)
}

pub fn f160(v0: Result<List<Adt17>, Bytes>, v1: List<Adt6>, v2: Bytes) -> Result<List<Adt17>, Bytes> {
    f225(Fn33::F226(v2), v0, v1)
}

pub fn f161(v0: Fn16, p954: u64, p955: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m953 = (p954, p955);
    {
        let mut m953 = m953;
        loop {
            let step = {
                let (v1, v2) = m953;
                let m950 = v1;
                if m950 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m950 - 1;
                    let m952 = {
                        let c951: Fn16 = v0.clone();
                        c951.apply(v2.clone())?
                    };
                    match m952 {
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
                Ok(next) => { m953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f227(v0, v1)
}

pub fn f163(v0: List<Bytes>) -> List<Bytes> {
    f228(Fn34::F229, List::<Bytes>::nil(), v0)
}

pub fn f164(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m959 = {
        let a956 = v0;
        let a957 = v1;
        let a958 = v2;
        slice_bytes(a956, a957, a958)
    };
    match m959 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f165(v0: Fn17, p963: u64, p964: List<Adt13>) -> R<u64> {
    let m962 = (p963, p964);
    {
        let mut m962 = m962;
        loop {
            let step = {
                let (v1, v2) = m962;
                let m960 = v2;
                match m960.uncons() {
                    None => {
                        Err::<(u64, List<Adt13>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt13>), R<u64>>(({
                            let c961: Fn17 = v0.clone();
                            c961.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m962 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Bytes, v1: u64, v2: Adt13) -> R<u64> {
    if f40(match v2 {
        Adt13::C0(h965, _) => {
            h965
        }
    }, v0) {
        let a966 = v1;
        let a967 = 1u64;
        nat_add(a966, a967)
    } else {
        Ok(v1)
    }
}

pub fn f167(v0: Fn18, p971: Option<u64>, p972: List<Adt13>) -> Option<u64> {
    let m970 = (p971, p972);
    {
        let mut m970 = m970;
        loop {
            let step = {
                let (v1, v2) = m970;
                let m968 = v2;
                match m968.uncons() {
                    None => {
                        Err::<(Option<u64>, List<Adt13>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<Adt13>), Option<u64>>(({
                            let c969: Fn18 = v0.clone();
                            c969.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m970 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: Option<u64>, v2: Adt13) -> Option<u64> {
    let m973 = v1;
    match m973 {
        None => {
            if f40(match v2.clone() {
                Adt13::C0(h974, _) => {
                    h974
                }
            }, v0) {
                Some(match v2 {
                    Adt13::C0(_, h975) => {
                        h975
                    }
                })
            } else {
                None::<u64>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f169(v0: Fn19, p979: Bytes, p980: List<Bytes>) -> Bytes {
    let m978 = (p979, p980);
    {
        let mut m978 = m978;
        loop {
            let step = {
                let (v1, v2) = m978;
                let m976 = v2;
                match m976.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let c977: Fn19 = v0.clone();
                            c977.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(_: Bytes, v1: Bytes) -> Bytes {
    v1
}

pub fn f171(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m981 = v1;
    match m981 {
        None => {
            if if f40(v2.clone(), f16(Str::lit("Self"))) {
                true
            } else {
                if f40(v2.clone(), f16(Str::lit("self"))) {
                    true
                } else {
                    if f40(v2.clone(), f16(Str::lit("crate"))) {
                        true
                    } else {
                        f40(v2.clone(), f16(Str::lit("super")))
                    }
                }
            } {
                Ok(None::<Bytes>)
            } else {
                f48(v0, v2)
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f172(v0: Fn20, p986: u64, p987: u64) -> R<(u64, bool)> {
    let m985 = (p986, p987);
    {
        let mut m985 = m985;
        loop {
            let step = {
                let (v1, v2) = m985;
                let m982 = v1;
                if m982 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m982 - 1;
                    let m984 = {
                        let c983: Fn20 = v0.clone();
                        c983.apply(v2)?
                    };
                    match m984 {
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
                Ok(next) => { m985 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f230(v0, v1) {
        Ok(Some({
            let a988 = v1;
            let a989 = 1u64;
            nat_add(a988, a989)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f174(v0: Bytes, v1: Adt14, v2: Adt8) -> Adt14 {
    let m992 = match v2.clone() {
        Adt8::C0(h991, _, _, _) => {
            h991
        }
    };
    if match m992 {
        Adt5::C0(_, h990, _, _, _) => {
            h990
        }
    } {
        v1
    } else {
        Adt14::C0(match v1.clone() {
            Adt14::C0(h993, _, _, _, _, _, _, _, _, _) => {
                h993
            }
        }, match v1.clone() {
            Adt14::C0(_, h994, _, _, _, _, _, _, _, _) => {
                h994
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h995, _, _, _, _, _, _, _) => {
                h995
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h996, _, _, _, _, _, _) => {
                h996
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h997, _, _, _, _, _) => {
                h997
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, h998, _, _, _, _) => {
                h998
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h999, _, _, _) => {
                h999
            }
        }, {
            let a1008 = match v1.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h1000, _, _) => {
                    h1000
                }
            };
            let a1009 = List::cons(Adt13::C0(f19(v0, f19(f16(Str::lit("::")), {
                let m1003 = match v2.clone() {
                    Adt8::C0(h1002, _, _, _) => {
                        h1002
                    }
                };
                match m1003 {
                    Adt5::C0(h1001, _, _, _, _) => {
                        h1001
                    }
                }
            })), {
                let a1007 = {
                    let m1006 = match v2 {
                        Adt8::C0(h1005, _, _, _) => {
                            h1005
                        }
                    };
                    match m1006 {
                        Adt5::C0(_, _, _, h1004, _) => {
                            h1004
                        }
                    }
                };
                length_list(a1007)
            }), List::<Adt13>::nil());
            append_list(a1008, a1009)
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h1010, _) => {
                h1010
            }
        }, match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h1011) => {
                h1011
            }
        })
    }
}

pub fn f175(v0: Str) -> Bytes {
    let a1012 = v0;
    utf8_encode(a1012)
}

pub fn f176(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h1013, _) = f172(Fn20::F231(v0.clone()), {
            let a1014 = v0.clone();
            length_bytes(a1014)
        }, 0u64)?;
        h1013
    };
    Ok(f164(v0.clone(), v1, {
        let a1016 = {
            let a1015 = v0;
            length_bytes(a1015)
        };
        let a1017 = v1;
        nat_sub(a1016, a1017)
    }))
}

pub fn f177(v0: u64, v1: u64) -> Bytes {
    f164(f175(Str::lit("0123456789")), {
        let a1021 = {
            let a1018 = v0;
            let a1019 = v1;
            let a1020 = 0u64;
            nat_quot(a1018, a1019, a1020)
        };
        let a1022 = 10u64;
        let a1023 = 0u64;
        nat_rem(a1021, a1022, a1023)
    }, 1u64)
}

pub fn f178(v0: Adt9) -> Bytes {
    let m1024 = v0;
    match m1024 {
        Adt9::C0(v1) => {
            v1
        }
        Adt9::C1(v2, _) => {
            v2
        }
        Adt9::C2(v4, _, _) => {
            v4
        }
    }
}

pub fn f179(v0: Fn21, p1028: Option<Adt11>, p1029: List<Adt11>) -> Option<Adt11> {
    let m1027 = (p1028, p1029);
    {
        let mut m1027 = m1027;
        loop {
            let step = {
                let (v1, v2) = m1027;
                let m1025 = v2;
                match m1025.uncons() {
                    None => {
                        Err::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(({
                            let c1026: Fn21 = v0.clone();
                            c1026.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Bytes, v1: Option<Adt11>, v2: Adt11) -> Option<Adt11> {
    let m1030 = v1;
    match m1030 {
        None => {
            if f40(match v2.clone() {
                Adt11::C0(h1031, _, _) => {
                    h1031
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt11>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f181(v0: Fn22, p1035: Option<Adt12>, p1036: List<Adt12>) -> Option<Adt12> {
    let m1034 = (p1035, p1036);
    {
        let mut m1034 = m1034;
        loop {
            let step = {
                let (v1, v2) = m1034;
                let m1032 = v2;
                match m1032.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c1033: Fn22 = v0.clone();
                            c1033.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, v1: Option<Adt12>, v2: Adt12) -> Option<Adt12> {
    let m1037 = v1;
    match m1037 {
        None => {
            if f40(match v2.clone() {
                Adt12::C0(h1038, _, _) => {
                    h1038
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt12>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f183(v0: Fn23, p1042: Option<Adt13>, p1043: List<Adt13>) -> Option<Adt13> {
    let m1041 = (p1042, p1043);
    {
        let mut m1041 = m1041;
        loop {
            let step = {
                let (v1, v2) = m1041;
                let m1039 = v2;
                match m1039.uncons() {
                    None => {
                        Err::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(({
                            let c1040: Fn23 = v0.clone();
                            c1040.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1044 = v1;
    match m1044 {
        None => {
            if f40(match v2.clone() {
                Adt13::C0(h1045, _) => {
                    h1045
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt13>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f185(p1049: List<Bytes>, p1050: u64) -> Option<Bytes> {
    let m1048 = (p1049, p1050);
    {
        let mut m1048 = m1048;
        loop {
            let step = {
                let (v0, v1) = m1048;
                let m1046 = v0;
                match m1046.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1047 = v1;
                        if m1047 == 0 {
                            Err::<(List<Bytes>, u64), Option<Bytes>>(Some(v2))
                        } else {
                            let v4 = m1047 - 1;
                            Ok::<(List<Bytes>, u64), Option<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1048 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: List<Adt9>, v1: Bytes) -> Option<Adt9> {
    f232(Fn35::F233(v1), None::<Adt9>, v0)
}

pub fn f187(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f183(Fn23::F234(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, h1051, _, _) => {
            h1051
        }
    })
}

pub fn f188(v0: Bytes) -> bool {
    let a1052 = v0.clone();
    let a1053 = f16(Str::lit("u8"));
    if equal(a1052, a1053) {
        true
    } else {
        let a1054 = v0.clone();
        let a1055 = f16(Str::lit("u16"));
        if equal(a1054, a1055) {
            true
        } else {
            let a1056 = v0.clone();
            let a1057 = f16(Str::lit("u32"));
            if equal(a1056, a1057) {
                true
            } else {
                let a1058 = v0.clone();
                let a1059 = f16(Str::lit("u64"));
                if equal(a1058, a1059) {
                    true
                } else {
                    let a1060 = v0.clone();
                    let a1061 = f16(Str::lit("u128"));
                    if equal(a1060, a1061) {
                        true
                    } else {
                        let a1062 = v0.clone();
                        let a1063 = f16(Str::lit("i8"));
                        if equal(a1062, a1063) {
                            true
                        } else {
                            let a1064 = v0.clone();
                            let a1065 = f16(Str::lit("i16"));
                            if equal(a1064, a1065) {
                                true
                            } else {
                                let a1066 = v0.clone();
                                let a1067 = f16(Str::lit("i32"));
                                if equal(a1066, a1067) {
                                    true
                                } else {
                                    let a1068 = v0.clone();
                                    let a1069 = f16(Str::lit("i64"));
                                    if equal(a1068, a1069) {
                                        true
                                    } else {
                                        let a1070 = v0;
                                        let a1071 = f16(Str::lit("i128"));
                                        equal(a1070, a1071)
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

pub fn f189(v0: Bytes, v1: Bytes) -> R<bool> {
    let m1072 = f235(v0.clone(), v1.clone())?;
    match m1072 {
        None => {
            if f188(v0) {
                f236(f16(Str::lit("usize")), v1)
            } else {
                Ok(false)
            }
        }
        Some(_) => {
            f236(v0, v1)
        }
    }
}

pub fn f190(v0: Bytes) -> Option<u64> {
    let a1073 = v0.clone();
    let a1074 = f16(Str::lit("Some"));
    if equal(a1073, a1074) {
        Some(1u64)
    } else {
        let a1075 = v0.clone();
        let a1076 = f16(Str::lit("Ok"));
        if equal(a1075, a1076) {
            Some(1u64)
        } else {
            let a1077 = v0;
            let a1078 = f16(Str::lit("Err"));
            if equal(a1077, a1078) {
                Some(1u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f191(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m1079 = f124(v0, v1);
    match m1079 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            Some({
                let a1081 = match v2 {
                    Adt11::C0(_, h1080, _) => {
                        h1080
                    }
                };
                length_list(a1081)
            })
        }
    }
}

pub fn f192(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let m1082 = f235(v0.clone(), v1.clone())?;
    match m1082 {
        None => {
            if f188(v0) {
                f235(f16(Str::lit("usize")), v1)
            } else {
                Ok(None::<u64>)
            }
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f193(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Option<u64>> {
    let m1083 = f125(v0, v1);
    match m1083 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            f237(match v3 {
                Adt12::C0(_, h1084, _) => {
                    h1084
                }
            }, v2)
        }
    }
}

pub fn f194(v0: Adt14, v1: Bytes, v2: Bytes) -> Option<u64> {
    let m1085 = f187(v0, f19(v1, f19(f16(Str::lit("::")), v2)));
    match m1085 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            Some(match v3 {
                Adt13::C0(_, h1086) => {
                    h1086
                }
            })
        }
    }
}

pub fn f195(v0: Fn24, p1090: List<u64>, p1091: List<Adt13>) -> List<u64> {
    let m1089 = (p1090, p1091);
    {
        let mut m1089 = m1089;
        loop {
            let step = {
                let (v1, v2) = m1089;
                let m1087 = v2;
                match m1087.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt13>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt13>), List<u64>>(({
                            let c1088: Fn24 = v0.clone();
                            c1088.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1089 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Bytes, v1: List<u64>, v2: Adt13) -> List<u64> {
    if f40(match v2.clone() {
        Adt13::C0(h1092, _) => {
            h1092
        }
    }, v0) {
        let a1094 = v1;
        let a1095 = List::cons(match v2 {
            Adt13::C0(_, h1093) => {
                h1093
            }
        }, List::<u64>::nil());
        append_list(a1094, a1095)
    } else {
        v1
    }
}

pub fn f197(v0: Fn25, p1099: bool, p1100: List<u64>) -> bool {
    let m1098 = (p1099, p1100);
    {
        let mut m1098 = m1098;
        loop {
            let step = {
                let (v1, v2) = m1098;
                let m1096 = v2;
                match m1096.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1097: Fn25 = v0.clone();
                            c1097.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1098 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1101 = v2;
        let a1102 = v0;
        nat_eq(a1101, a1102)
    }
}

pub fn f199(p1113: Adt2) -> Adt2 {
    let m1112 = p1113;
    {
        let mut m1112 = m1112;
        loop {
            let step = {
                let v0 = m1112;
                let m1103 = v0;
                match m1103 {
                    Adt2::C0 => {
                        Err::<Adt2, Adt2>(Adt2::C0)
                    }
                    Adt2::C1 => {
                        Err::<Adt2, Adt2>(Adt2::C1)
                    }
                    Adt2::C2 => {
                        Err::<Adt2, Adt2>(Adt2::C2)
                    }
                    Adt2::C3 => {
                        Err::<Adt2, Adt2>(Adt2::C3)
                    }
                    Adt2::C4 => {
                        Err::<Adt2, Adt2>(Adt2::C4)
                    }
                    Adt2::C5 => {
                        Err::<Adt2, Adt2>(Adt2::C5)
                    }
                    Adt2::C6(v1) => {
                        Err::<Adt2, Adt2>(Adt2::C6(v1))
                    }
                    Adt2::C7(r1104) => {
                        let v2 = (*r1104).clone();
                        Ok::<Adt2, Adt2>(v2)
                    }
                    Adt2::C8(r1105) => {
                        let v3 = (*r1105).clone();
                        Ok::<Adt2, Adt2>(v3)
                    }
                    Adt2::C9(r1106, r1107) => {
                        let v4 = (*r1106).clone();
                        let v5 = (*r1107).clone();
                        Err::<Adt2, Adt2>(Adt2::C9(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))
                    }
                    Adt2::C10(v6, r1108) => {
                        let v7 = (*r1108).clone();
                        Err::<Adt2, Adt2>(Adt2::C10(v6, std::rc::Rc::new(v7)))
                    }
                    Adt2::C11 => {
                        Err::<Adt2, Adt2>(Adt2::C11)
                    }
                    Adt2::C12 => {
                        Err::<Adt2, Adt2>(Adt2::C12)
                    }
                    Adt2::C13 => {
                        Err::<Adt2, Adt2>(Adt2::C13)
                    }
                    Adt2::C14 => {
                        Err::<Adt2, Adt2>(Adt2::C14)
                    }
                    Adt2::C15(r1109) => {
                        let v8 = (*r1109).clone();
                        Err::<Adt2, Adt2>(Adt2::C15(std::rc::Rc::new(v8)))
                    }
                    Adt2::C16 => {
                        Err::<Adt2, Adt2>(Adt2::C16)
                    }
                    Adt2::C17 => {
                        Err::<Adt2, Adt2>(Adt2::C17)
                    }
                    Adt2::C18 => {
                        Err::<Adt2, Adt2>(Adt2::C18)
                    }
                    Adt2::C19(r1110, r1111) => {
                        let v9 = (*r1110).clone();
                        let v10 = (*r1111).clone();
                        Err::<Adt2, Adt2>(Adt2::C19(std::rc::Rc::new(v9), std::rc::Rc::new(v10)))
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

pub fn f200(v0: Adt16, v1: Adt3) -> Adt2 {
    let m1114 = v1;
    match m1114 {
        Adt3::C0(v2) => {
            let m1116 = f65(match v0 {
                Adt16::C0(_, h1115, _, _) => {
                    h1115
                }
            }, v2);
            match m1116 {
                None => {
                    Adt2::C16
                }
                Some(v3) => {
                    match v3 {
                        Adt17::C0(_, _, h1117) => {
                            h1117
                        }
                    }
                }
            }
        }
        Adt3::C1(_) => {
            Adt2::C16
        }
        Adt3::C2(_) => {
            Adt2::C16
        }
        Adt3::C3(_, _) => {
            Adt2::C16
        }
        Adt3::C4(_, _, _) => {
            Adt2::C16
        }
        Adt3::C5(r1118, v12) => {
            let v11 = (*r1118).clone();
            f238(f239(v0.clone(), f199(f200(v0, v11))), v12)
        }
        Adt3::C6(_, _) => {
            Adt2::C16
        }
        Adt3::C7(r1119) => {
            let v15 = (*r1119).clone();
            let v16: Adt2 = f200(v0, v15);
            if f240(v16.clone()) {
                Adt2::C16
            } else {
                Adt2::C7(std::rc::Rc::new(v16))
            }
        }
        Adt3::C8(_, _) => {
            Adt2::C16
        }
        Adt3::C9(_, _) => {
            Adt2::C16
        }
        Adt3::C10(_, _, _) => {
            Adt2::C16
        }
        Adt3::C11(_, _) => {
            Adt2::C16
        }
        Adt3::C12(_, _, _) => {
            Adt2::C16
        }
        Adt3::C13(_, _) => {
            Adt2::C16
        }
        Adt3::C14(_, _, _) => {
            Adt2::C16
        }
        Adt3::C15(_) => {
            Adt2::C16
        }
        Adt3::C16(_, _, _) => {
            Adt2::C16
        }
        Adt3::C17 => {
            Adt2::C16
        }
        Adt3::C18(_) => {
            Adt2::C16
        }
        Adt3::C19(_) => {
            Adt2::C16
        }
        Adt3::C20(_, _, _) => {
            Adt2::C16
        }
        Adt3::C21(_, _, _) => {
            Adt2::C16
        }
        Adt3::C22(_) => {
            Adt2::C16
        }
        Adt3::C23(_, _) => {
            Adt2::C16
        }
        Adt3::C24(_, _) => {
            Adt2::C16
        }
        Adt3::C25(_, _, _) => {
            Adt2::C16
        }
        Adt3::C26(_) => {
            Adt2::C16
        }
        Adt3::C27(_) => {
            Adt2::C16
        }
        Adt3::C28(_, _, _, _) => {
            Adt2::C16
        }
        Adt3::C29(_, _) => {
            Adt2::C16
        }
        Adt3::C30 => {
            Adt2::C16
        }
        Adt3::C31 => {
            Adt2::C16
        }
        Adt3::C32(_, _) => {
            Adt2::C16
        }
        Adt3::C33(_, _, _, _) => {
            Adt2::C16
        }
        Adt3::C34(_, _, _, _) => {
            Adt2::C16
        }
    }
}

pub fn f201(v0: Adt14, v1: List<Bytes>, v2: Bytes) -> R<bool> {
    f101(Fn10::F241(v2, v0), false, v1)
}

pub fn f202(p1124: List<Adt7>, p1125: u64) -> List<Bytes> {
    let m1123 = (p1124, p1125);
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let (v0, v1) = m1123;
                let m1120 = v0;
                match m1120.uncons() {
                    None => {
                        Err::<(List<Adt7>, u64), List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        let m1121 = v1;
                        if m1121 == 0 {
                            match v2 {
                                Adt7::C0(_, h1122) => {
                                    Err::<(List<Adt7>, u64), List<Bytes>>(h1122)
                                }
                            }
                        } else {
                            let v4 = m1121 - 1;
                            Ok::<(List<Adt7>, u64), List<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Fn26, p1129: Bytes, p1130: List<u64>) -> R<Bytes> {
    let m1128 = (p1129, p1130);
    {
        let mut m1128 = m1128;
        loop {
            let step = {
                let (v1, v2) = m1128;
                let m1126 = v2;
                match m1126.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c1127: Fn26 = v0.clone();
                            c1127.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1131 = v0.clone();
    let a1132 = f16(Str::lit(""));
    if equal(a1131, a1132) {
        f59(v1)
    } else {
        Ok(f19(v0, f19(f16(Str::lit("/")), f59(v1)?)))
    }
}

pub fn f205(v0: Adt3) -> u64 {
    let m1133 = v0;
    match m1133 {
        Adt3::C0(_) => {
            14u64
        }
        Adt3::C1(_) => {
            14u64
        }
        Adt3::C2(_) => {
            14u64
        }
        Adt3::C3(_, _) => {
            13u64
        }
        Adt3::C4(_, _, _) => {
            13u64
        }
        Adt3::C5(_, _) => {
            13u64
        }
        Adt3::C6(_, _) => {
            13u64
        }
        Adt3::C7(_) => {
            12u64
        }
        Adt3::C8(_, _) => {
            5u64
        }
        Adt3::C9(_, _) => {
            14u64
        }
        Adt3::C10(_, _, _) => {
            1u64
        }
        Adt3::C11(_, _) => {
            1u64
        }
        Adt3::C12(_, _, _) => {
            14u64
        }
        Adt3::C13(_, _) => {
            14u64
        }
        Adt3::C14(_, _, _) => {
            14u64
        }
        Adt3::C15(_) => {
            14u64
        }
        Adt3::C16(_, _, _) => {
            14u64
        }
        Adt3::C17 => {
            1u64
        }
        Adt3::C18(_) => {
            1u64
        }
        Adt3::C19(_) => {
            14u64
        }
        Adt3::C20(_, _, _) => {
            14u64
        }
        Adt3::C21(v40, _, _) => {
            f207(v40)
        }
        Adt3::C22(_) => {
            12u64
        }
        Adt3::C23(_, _) => {
            11u64
        }
        Adt3::C24(_, _) => {
            13u64
        }
        Adt3::C25(_, _, _) => {
            14u64
        }
        Adt3::C26(_) => {
            12u64
        }
        Adt3::C27(_) => {
            12u64
        }
        Adt3::C28(_, _, _, _) => {
            13u64
        }
        Adt3::C29(_, _) => {
            14u64
        }
        Adt3::C30 => {
            14u64
        }
        Adt3::C31 => {
            14u64
        }
        Adt3::C32(_, _) => {
            14u64
        }
        Adt3::C33(_, _, _, _) => {
            14u64
        }
        Adt3::C34(_, _, _, _) => {
            14u64
        }
    }
}

pub fn f206(v0: u64, v1: Adt3) -> bool {
    let a1134 = v0;
    let a1135 = 8u64;
    if if nat_eq(a1134, a1135) {
        f242(v1.clone())
    } else {
        false
    } {
        true
    } else {
        let a1138 = f205(v1);
        let a1139 = {
            let a1136 = f207(v0);
            let a1137 = 5u64;
            if nat_eq(a1136, a1137) {
                6u64
            } else {
                f207(v0)
            }
        };
        nat_lt(a1138, a1139)
    }
}

pub fn f207(v0: u64) -> u64 {
    let a1140 = v0;
    let a1141 = 6u64;
    if nat_eq(a1140, a1141) {
        3u64
    } else {
        let a1142 = v0;
        let a1143 = 5u64;
        if nat_eq(a1142, a1143) {
            4u64
        } else {
            let a1144 = v0;
            let a1145 = 7u64;
            if nat_eq(a1144, a1145) {
                5u64
            } else {
                let a1146 = v0;
                let a1147 = 8u64;
                if nat_eq(a1146, a1147) {
                    5u64
                } else {
                    let a1148 = v0;
                    let a1149 = 9u64;
                    if nat_eq(a1148, a1149) {
                        5u64
                    } else {
                        let a1150 = v0;
                        let a1151 = 10u64;
                        if nat_eq(a1150, a1151) {
                            5u64
                        } else {
                            let a1152 = v0;
                            let a1153 = 11u64;
                            if nat_eq(a1152, a1153) {
                                5u64
                            } else {
                                let a1154 = v0;
                                let a1155 = 0u64;
                                if nat_eq(a1154, a1155) {
                                    9u64
                                } else {
                                    let a1156 = v0;
                                    let a1157 = 1u64;
                                    if nat_eq(a1156, a1157) {
                                        9u64
                                    } else {
                                        10u64
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

pub fn f208(v0: Adt9) -> R<u64> {
    let m1158 = v0;
    match m1158 {
        Adt9::C0(_) => {
            Ok(0u64)
        }
        Adt9::C1(_, v3) => {
            f58(v3)
        }
        Adt9::C2(_, _, v6) => {
            f58(v6)
        }
    }
}

pub fn f209(v0: List<Bytes>, v1: List<Bytes>, v2: Option<Bytes>, v3: Bytes) -> R<Option<Bytes>> {
    let m1159 = v2;
    match m1159 {
        None => {
            if f66(v0, v3.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: pattern-fields: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" has no field ")), f20(v3))))))
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f210(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1160 = v1;
    match m1160 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1163 = {
                    let a1161 = v3;
                    let a1162 = v0;
                    append_bytes(a1161, a1162)
                };
                let a1164 = v2;
                append_bytes(a1163, a1164)
            })
        }
    }
}

pub fn f211(v0: List<Bytes>, v1: Bytes) -> R<u64> {
    f243(Fn36::F244(v1), 0u64, v0)
}

pub fn f212(v0: Fn27, p1168: Option<Bytes>, p1169: List<Adt6>) -> R<Option<Bytes>> {
    let m1167 = (p1168, p1169);
    {
        let mut m1167 = m1167;
        loop {
            let step = {
                let (v1, v2) = m1167;
                let m1165 = v2;
                match m1165.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(({
                            let c1166: Fn27 = v0.clone();
                            c1166.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: bool, v1: u64, v2: Adt14, v3: Option<Bytes>, v4: Adt6) -> R<Option<Bytes>> {
    let m1170 = v3;
    match m1170 {
        None => {
            let m1172 = f48(Str::lit("a parameter name"), match v4.clone() {
                Adt6::C0(h1171, _, _) => {
                    h1171
                }
            })?;
            match m1172 {
                None => {
                    f22(v2, match v4 {
                        Adt6::C0(_, _, h1173) => {
                            h1173
                        }
                    }, v1, v0)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f214(v0: Fn28, p1177: Option<Adt10>, p1178: List<Adt10>) -> Option<Adt10> {
    let m1176 = (p1177, p1178);
    {
        let mut m1176 = m1176;
        loop {
            let step = {
                let (v1, v2) = m1176;
                let m1174 = v2;
                match m1174.uncons() {
                    None => {
                        Err::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(({
                            let c1175: Fn28 = v0.clone();
                            c1175.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: Bytes, v1: Option<Adt10>, v2: Adt10) -> Option<Adt10> {
    let m1179 = v1;
    match m1179 {
        None => {
            if f40(match v2.clone() {
                Adt10::C0(h1180, _) => {
                    h1180
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt10>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f216(v0: Fn29, p1184: u64, p1185: List<Adt8>) -> R<u64> {
    let m1183 = (p1184, p1185);
    {
        let mut m1183 = m1183;
        loop {
            let step = {
                let (v1, v2) = m1183;
                let m1181 = v2;
                match m1181.uncons() {
                    None => {
                        Err::<(u64, List<Adt8>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt8>), R<u64>>(({
                            let c1182: Fn29 = v0.clone();
                            c1182.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Bytes, v1: u64, v2: Adt8) -> R<u64> {
    if f40({
        let m1188 = match v2 {
            Adt8::C0(h1187, _, _, _) => {
                h1187
            }
        };
        match m1188 {
            Adt5::C0(h1186, _, _, _, _) => {
                h1186
            }
        }
    }, v0) {
        let a1189 = v1;
        let a1190 = 1u64;
        nat_add(a1189, a1190)
    } else {
        Ok(v1)
    }
}

pub fn f218(v0: Fn30, p1194: Option<Adt8>, p1195: List<Adt8>) -> Option<Adt8> {
    let m1193 = (p1194, p1195);
    {
        let mut m1193 = m1193;
        loop {
            let step = {
                let (v1, v2) = m1193;
                let m1191 = v2;
                match m1191.uncons() {
                    None => {
                        Err::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(({
                            let c1192: Fn30 = v0.clone();
                            c1192.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Bytes, v1: Option<Adt8>, v2: Adt8) -> Option<Adt8> {
    let m1196 = v1;
    match m1196 {
        None => {
            if f40({
                let m1199 = match v2.clone() {
                    Adt8::C0(h1198, _, _, _) => {
                        h1198
                    }
                };
                match m1199 {
                    Adt5::C0(h1197, _, _, _, _) => {
                        h1197
                    }
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt8>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f220(v0: Fn31, p1203: Bytes, p1204: List<Adt6>) -> R<Bytes> {
    let m1202 = (p1203, p1204);
    {
        let mut m1202 = m1202;
        loop {
            let step = {
                let (v1, v2) = m1202;
                let m1200 = v2;
                match m1200.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt6>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt6>), R<Bytes>>(({
                            let c1201: Fn31 = v0.clone();
                            c1201.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: Adt6) -> R<Bytes> {
    Ok(f19(v0, f19(match v1.clone() {
        Adt6::C0(h1205, _, _) => {
            h1205
        }
    }, f19(if match v1.clone() {
        Adt6::C0(_, h1206, _) => {
            h1206
        }
    } {
        f16(Str::lit(":m:"))
    } else {
        f16(Str::lit(":-:"))
    }, f19(f222(match v1 {
        Adt6::C0(_, _, h1207) => {
            h1207
        }
    })?, f16(Str::lit(",")))))))
}

pub fn f222(v0: Adt2) -> R<Bytes> {
    let m1208 = v0;
    match m1208 {
        Adt2::C0 => {
            Ok(f16(Str::lit("U")))
        }
        Adt2::C1 => {
            Ok(f16(Str::lit("B")))
        }
        Adt2::C2 => {
            Ok(f16(Str::lit("N")))
        }
        Adt2::C3 => {
            Ok(f16(Str::lit("F")))
        }
        Adt2::C4 => {
            Ok(f16(Str::lit("O")))
        }
        Adt2::C5 => {
            Ok(f16(Str::lit("S")))
        }
        Adt2::C6(v1) => {
            Ok(f19(f16(Str::lit("P")), f19(f59(v1)?, f16(Str::lit(";")))))
        }
        Adt2::C7(r1209) => {
            let v2 = (*r1209).clone();
            Ok(f19(f16(Str::lit("R(")), f19(f222(v2)?, f16(Str::lit(")")))))
        }
        Adt2::C8(r1210) => {
            let v3 = (*r1210).clone();
            Ok(f19(f16(Str::lit("M(")), f19(f222(v3)?, f16(Str::lit(")")))))
        }
        Adt2::C9(r1211, r1212) => {
            let v4 = (*r1211).clone();
            let v5 = (*r1212).clone();
            Ok(f19(f16(Str::lit("T(")), f19(f222(v4)?, f19(f16(Str::lit(",")), f19(f222(v5)?, f16(Str::lit(")")))))))
        }
        Adt2::C10(v6, r1213) => {
            let v7 = (*r1213).clone();
            Ok(f19(f16(Str::lit("D")), f19(f20(v6), f19(f16(Str::lit("(")), f19(f222(v7)?, f16(Str::lit(")")))))))
        }
        Adt2::C11 => {
            Ok(f16(Str::lit("W")))
        }
        Adt2::C12 => {
            Ok(f16(Str::lit("Z")))
        }
        Adt2::C13 => {
            Ok(f16(Str::lit("C")))
        }
        Adt2::C14 => {
            Ok(f16(Str::lit("Q")))
        }
        Adt2::C15(r1214) => {
            let v8 = (*r1214).clone();
            Ok(f19(f16(Str::lit("L(")), f19(f222(v8)?, f16(Str::lit(")")))))
        }
        Adt2::C16 => {
            Ok(f16(Str::lit("?")))
        }
        Adt2::C17 => {
            Ok(f16(Str::lit("-")))
        }
        Adt2::C18 => {
            Ok(f16(Str::lit("")))
        }
        Adt2::C19(r1215, r1216) => {
            let v9 = (*r1215).clone();
            let v10 = (*r1216).clone();
            Ok(f19(f222(v9)?, f19(f16(Str::lit(";")), f222(v10)?)))
        }
    }
}

pub fn f223(v0: Fn32, p1220: bool, p1221: List<Adt5>) -> bool {
    let m1219 = (p1220, p1221);
    {
        let mut m1219 = m1219;
        loop {
            let step = {
                let (v1, v2) = m1219;
                let m1217 = v2;
                match m1217.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c1218: Fn32 = v0.clone();
                            c1218.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f40(match v2 {
            Adt5::C0(h1222, _, _, _, _) => {
                h1222
            }
        }, v0)
    }
}

pub fn f225(v0: Fn33, p1226: Result<List<Adt17>, Bytes>, p1227: List<Adt6>) -> Result<List<Adt17>, Bytes> {
    let m1225 = (p1226, p1227);
    {
        let mut m1225 = m1225;
        loop {
            let step = {
                let (v1, v2) = m1225;
                let m1223 = v2;
                match m1223.uncons() {
                    None => {
                        Err::<(Result<List<Adt17>, Bytes>, List<Adt6>), Result<List<Adt17>, Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt17>, Bytes>, List<Adt6>), Result<List<Adt17>, Bytes>>(({
                            let c1224: Fn33 = v0.clone();
                            c1224.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: Bytes, v1: Result<List<Adt17>, Bytes>, v2: Adt6) -> Result<List<Adt17>, Bytes> {
    let m1228 = v1;
    match m1228 {
        Ok(v4) => {
            let m1230 = f65(v4.clone(), match v2.clone() {
                Adt6::C0(h1229, _, _) => {
                    h1229
                }
            });
            match m1230 {
                None => {
                    Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(match v2.clone() {
                        Adt6::C0(h1232, _, _) => {
                            h1232
                        }
                    }, match v2.clone() {
                        Adt6::C0(_, h1233, _) => {
                            h1233
                        }
                    }, match v2 {
                        Adt6::C0(_, _, h1234) => {
                            h1234
                        }
                    }), v4))
                }
                Some(_) => {
                    Err::<List<Adt17>, Bytes>(f19(f16(Str::lit("runtime support: param-duplicate: ")), f19(f20(match v2 {
                        Adt6::C0(h1231, _, _) => {
                            h1231
                        }
                    }), f19(f16(Str::lit(" names two parameters of ")), f20(v0)))))
                }
            }
        }
        Err(v3) => {
            Err::<List<Adt17>, Bytes>(v3)
        }
    }
}

pub fn f227(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1235, _) = v1.clone();
        h1235
    };
    let v3: u64 = {
        let (h1236, _) = {
            let (_, h1237) = v1.clone();
            h1237
        };
        h1236
    };
    let v4: List<Bytes> = {
        let (_, h1238) = {
            let (_, h1239) = v1;
            h1239
        };
        h1238
    };
    let m1240 = f245(v0.clone(), v2);
    match m1240 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1241 = v5;
            let a1242 = 58u8;
            if equal(a1241, a1242) {
                let m1245 = f245(v0.clone(), {
                    let a1243 = v2;
                    let a1244 = 1u64;
                    nat_add(a1243, a1244)?
                });
                match m1245 {
                    None => {
                        Ok(Some(({
                            let a1256 = v2;
                            let a1257 = 1u64;
                            nat_add(a1256, a1257)?
                        }, (v3, v4))))
                    }
                    Some(v6) => {
                        let a1246 = v6;
                        let a1247 = 58u8;
                        if equal(a1246, a1247) {
                            Ok(Some(({
                                let a1248 = v2;
                                let a1249 = 2u64;
                                nat_add(a1248, a1249)?
                            }, ({
                                let a1250 = v2;
                                let a1251 = 2u64;
                                nat_add(a1250, a1251)?
                            }, List::cons(f164(v0, v3, {
                                let a1252 = v2;
                                let a1253 = v3;
                                nat_sub(a1252, a1253)
                            }), v4)))))
                        } else {
                            Ok(Some(({
                                let a1254 = v2;
                                let a1255 = 1u64;
                                nat_add(a1254, a1255)?
                            }, (v3, v4))))
                        }
                    }
                }
            } else {
                Ok(Some(({
                    let a1258 = v2;
                    let a1259 = 1u64;
                    nat_add(a1258, a1259)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f228(v0: Fn34, p1263: List<Bytes>, p1264: List<Bytes>) -> List<Bytes> {
    let m1262 = (p1263, p1264);
    {
        let mut m1262 = m1262;
        loop {
            let step = {
                let (v1, v2) = m1262;
                let m1260 = v2;
                match m1260.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1261: Fn34 = v0.clone();
                            c1261.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f230(v0: Bytes, v1: u64) -> bool {
    let m1265 = f245(v0, v1);
    match m1265 {
        None => {
            false
        }
        Some(v2) => {
            if f246(v2, 97u8, 122u8) {
                true
            } else {
                if f246(v2, 65u8, 90u8) {
                    true
                } else {
                    let a1266 = v2;
                    let a1267 = 95u8;
                    if equal(a1266, a1267) {
                        true
                    } else {
                        let a1268 = 0u64;
                        let a1269 = v1;
                        if nat_lt(a1268, a1269) {
                            f246(v2, 48u8, 57u8)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f231(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f247(v0, v1)
}

pub fn f232(v0: Fn35, p1273: Option<Adt9>, p1274: List<Adt9>) -> Option<Adt9> {
    let m1272 = (p1273, p1274);
    {
        let mut m1272 = m1272;
        loop {
            let step = {
                let (v1, v2) = m1272;
                let m1270 = v2;
                match m1270.uncons() {
                    None => {
                        Err::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(({
                            let c1271: Fn35 = v0.clone();
                            c1271.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233(v0: Bytes, v1: Option<Adt9>, v2: Adt9) -> Option<Adt9> {
    let m1275 = v1;
    match m1275 {
        None => {
            if f40(f178(v2.clone()), v0) {
                Some(v2)
            } else {
                None::<Adt9>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f234(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1276 = v1;
    match m1276 {
        None => {
            if f40(match v2.clone() {
                Adt13::C0(h1277, _) => {
                    h1277
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt13>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f235(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let a1278 = v0.clone();
    let a1279 = f16(Str::lit("AtomicU64"));
    if equal(a1278, a1279) {
        let a1280 = v1;
        let a1281 = f16(Str::lit("new"));
        if equal(a1280, a1281) {
            Ok(Some(1u64))
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a1282 = v0.clone();
        let a1283 = f16(Str::lit("Rc"));
        if equal(a1282, a1283) {
            if f66(List::cons(f16(Str::lit("new")), List::cons(f16(Str::lit("from")), List::cons(f16(Str::lit("try_unwrap")), List::cons(f16(Str::lit("get_mut")), List::<Bytes>::nil())))), v1)? {
                Ok(Some(1u64))
            } else {
                Ok(None::<u64>)
            }
        } else {
            let a1284 = v0.clone();
            let a1285 = f16(Str::lit("Vec"));
            if equal(a1284, a1285) {
                let a1286 = v1.clone();
                let a1287 = f16(Str::lit("new"));
                if equal(a1286, a1287) {
                    Ok(Some(0u64))
                } else {
                    let a1288 = v1;
                    let a1289 = f16(Str::lit("with_capacity"));
                    if equal(a1288, a1289) {
                        Ok(Some(1u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            } else {
                let a1290 = v0.clone();
                let a1291 = f16(Str::lit("String"));
                if equal(a1290, a1291) {
                    let a1292 = v1;
                    let a1293 = f16(Str::lit("new"));
                    if equal(a1292, a1293) {
                        Ok(Some(0u64))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a1294 = v0.clone();
                    let a1295 = f16(Str::lit("mem"));
                    if equal(a1294, a1295) {
                        let a1296 = v1.clone();
                        let a1297 = f16(Str::lit("replace"));
                        if equal(a1296, a1297) {
                            Ok(Some(2u64))
                        } else {
                            let a1298 = v1;
                            let a1299 = f16(Str::lit("take"));
                            if equal(a1298, a1299) {
                                Ok(Some(1u64))
                            } else {
                                Ok(None::<u64>)
                            }
                        }
                    } else {
                        let a1300 = v0.clone();
                        let a1301 = f16(Str::lit("str"));
                        if equal(a1300, a1301) {
                            let a1302 = v1;
                            let a1303 = f16(Str::lit("from_utf8"));
                            if equal(a1302, a1303) {
                                Ok(Some(1u64))
                            } else {
                                Ok(None::<u64>)
                            }
                        } else {
                            let a1304 = v0.clone();
                            let a1305 = f16(Str::lit("usize"));
                            if equal(a1304, a1305) {
                                if f66(List::cons(f16(Str::lit("from")), List::cons(f16(Str::lit("try_from")), List::<Bytes>::nil())), v1)? {
                                    Ok(Some(1u64))
                                } else {
                                    Ok(None::<u64>)
                                }
                            } else {
                                let a1306 = v0.clone();
                                let a1307 = f16(Str::lit("fs"));
                                if equal(a1306, a1307) {
                                    if f66(List::cons(f16(Str::lit("read")), List::cons(f16(Str::lit("read_dir")), List::cons(f16(Str::lit("symlink_metadata")), List::cons(f16(Str::lit("create_dir_all")), List::cons(f16(Str::lit("remove_file")), List::cons(f16(Str::lit("remove_dir_all")), List::cons(f16(Str::lit("canonicalize")), List::<Bytes>::nil()))))))), v1.clone())? {
                                        Ok(Some(1u64))
                                    } else {
                                        if f66(List::cons(f16(Str::lit("write")), List::cons(f16(Str::lit("rename")), List::<Bytes>::nil())), v1)? {
                                            Ok(Some(2u64))
                                        } else {
                                            Ok(None::<u64>)
                                        }
                                    }
                                } else {
                                    let a1308 = v0.clone();
                                    let a1309 = f16(Str::lit("File"));
                                    if equal(a1308, a1309) {
                                        let a1310 = v1;
                                        let a1311 = f16(Str::lit("create"));
                                        if equal(a1310, a1311) {
                                            Ok(Some(1u64))
                                        } else {
                                            Ok(None::<u64>)
                                        }
                                    } else {
                                        let a1312 = v0.clone();
                                        let a1313 = f16(Str::lit("env"));
                                        if equal(a1312, a1313) {
                                            if f66(List::cons(f16(Str::lit("args_os")), List::cons(f16(Str::lit("current_dir")), List::cons(f16(Str::lit("current_exe")), List::cons(f16(Str::lit("vars_os")), List::<Bytes>::nil())))), v1)? {
                                                Ok(Some(0u64))
                                            } else {
                                                Ok(None::<u64>)
                                            }
                                        } else {
                                            let a1314 = v0.clone();
                                            let a1315 = f16(Str::lit("io"));
                                            if equal(a1314, a1315) {
                                                if f66(List::cons(f16(Str::lit("stdout")), List::cons(f16(Str::lit("stderr")), List::<Bytes>::nil())), v1)? {
                                                    Ok(Some(0u64))
                                                } else {
                                                    Ok(None::<u64>)
                                                }
                                            } else {
                                                let a1316 = v0.clone();
                                                let a1317 = f16(Str::lit("process"));
                                                if equal(a1316, a1317) {
                                                    let a1318 = v1;
                                                    let a1319 = f16(Str::lit("exit"));
                                                    if equal(a1318, a1319) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        Ok(None::<u64>)
                                                    }
                                                } else {
                                                    let a1320 = v0.clone();
                                                    let a1321 = f16(Str::lit("Command"));
                                                    if equal(a1320, a1321) {
                                                        if f66(List::cons(f16(Str::lit("new")), List::cons(f16(Str::lit("spawn")), List::<Bytes>::nil())), v1)? {
                                                            Ok(Some(1u64))
                                                        } else {
                                                            Ok(None::<u64>)
                                                        }
                                                    } else {
                                                        let a1322 = v0.clone();
                                                        let a1323 = f16(Str::lit("Stdio"));
                                                        if equal(a1322, a1323) {
                                                            let a1324 = v1;
                                                            let a1325 = f16(Str::lit("null"));
                                                            if equal(a1324, a1325) {
                                                                Ok(Some(0u64))
                                                            } else {
                                                                Ok(None::<u64>)
                                                            }
                                                        } else {
                                                            let a1326 = v0.clone();
                                                            let a1327 = f16(Str::lit("Instant"));
                                                            if equal(a1326, a1327) {
                                                                let a1328 = v1;
                                                                let a1329 = f16(Str::lit("now"));
                                                                if equal(a1328, a1329) {
                                                                    Ok(Some(0u64))
                                                                } else {
                                                                    Ok(None::<u64>)
                                                                }
                                                            } else {
                                                                let a1330 = v0.clone();
                                                                let a1331 = f16(Str::lit("Duration"));
                                                                if equal(a1330, a1331) {
                                                                    let a1332 = v1;
                                                                    let a1333 = f16(Str::lit("from_millis"));
                                                                    if equal(a1332, a1333) {
                                                                        Ok(Some(1u64))
                                                                    } else {
                                                                        Ok(None::<u64>)
                                                                    }
                                                                } else {
                                                                    let a1334 = v0.clone();
                                                                    let a1335 = f16(Str::lit("thread"));
                                                                    if equal(a1334, a1335) {
                                                                        let a1336 = v1;
                                                                        let a1337 = f16(Str::lit("sleep"));
                                                                        if equal(a1336, a1337) {
                                                                            Ok(Some(1u64))
                                                                        } else {
                                                                            Ok(None::<u64>)
                                                                        }
                                                                    } else {
                                                                        let a1338 = v0;
                                                                        let a1339 = f16(Str::lit("Builder"));
                                                                        if equal(a1338, a1339) {
                                                                            let a1340 = v1;
                                                                            let a1341 = f16(Str::lit("new"));
                                                                            if equal(a1340, a1341) {
                                                                                Ok(Some(0u64))
                                                                            } else {
                                                                                Ok(None::<u64>)
                                                                            }
                                                                        } else {
                                                                            Ok(None::<u64>)
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
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1342 = v0.clone();
    let a1343 = f16(Str::lit("Rc"));
    if equal(a1342, a1343) {
        f66(List::cons(f16(Str::lit("new")), List::cons(f16(Str::lit("from")), List::cons(f16(Str::lit("try_unwrap")), List::cons(f16(Str::lit("get_mut")), List::<Bytes>::nil())))), v1)
    } else {
        let a1344 = v0.clone();
        let a1345 = f16(Str::lit("Vec"));
        if equal(a1344, a1345) {
            f66(List::cons(f16(Str::lit("new")), List::cons(f16(Str::lit("with_capacity")), List::<Bytes>::nil())), v1)
        } else {
            let a1346 = v0;
            let a1347 = f16(Str::lit("String"));
            if equal(a1346, a1347) {
                let a1348 = v1;
                let a1349 = f16(Str::lit("new"));
                Ok(equal(a1348, a1349))
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f237(v0: List<Adt9>, v1: Bytes) -> R<Option<u64>> {
    let m1350 = f186(v0, v1);
    match m1350 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1351 = v2;
            match m1351 {
                Adt9::C0(_) => {
                    Ok(None::<u64>)
                }
                Adt9::C1(_, v5) => {
                    Ok(Some(f58(v5)?))
                }
                Adt9::C2(_, _, _) => {
                    Ok(None::<u64>)
                }
            }
        }
    }
}

pub fn f238(v0: Adt2, v1: u64) -> Adt2 {
    let m1352 = v0;
    match m1352 {
        Adt2::C0 => {
            Adt2::C16
        }
        Adt2::C1 => {
            Adt2::C16
        }
        Adt2::C2 => {
            Adt2::C16
        }
        Adt2::C3 => {
            Adt2::C16
        }
        Adt2::C4 => {
            Adt2::C16
        }
        Adt2::C5 => {
            Adt2::C16
        }
        Adt2::C6(_) => {
            Adt2::C16
        }
        Adt2::C7(_) => {
            Adt2::C16
        }
        Adt2::C8(_) => {
            Adt2::C16
        }
        Adt2::C9(r1357, r1358) => {
            let v5 = (*r1357).clone();
            let v6 = (*r1358).clone();
            let a1353 = v1;
            let a1354 = 0u64;
            if nat_eq(a1353, a1354) {
                v5
            } else {
                let a1355 = v1;
                let a1356 = 1u64;
                if nat_eq(a1355, a1356) {
                    v6
                } else {
                    Adt2::C16
                }
            }
        }
        Adt2::C10(_, _) => {
            Adt2::C16
        }
        Adt2::C11 => {
            Adt2::C16
        }
        Adt2::C12 => {
            Adt2::C16
        }
        Adt2::C13 => {
            Adt2::C16
        }
        Adt2::C14 => {
            Adt2::C16
        }
        Adt2::C15(_) => {
            Adt2::C16
        }
        Adt2::C16 => {
            Adt2::C16
        }
        Adt2::C17 => {
            Adt2::C16
        }
        Adt2::C18 => {
            Adt2::C16
        }
        Adt2::C19(_, _) => {
            Adt2::C16
        }
    }
}

pub fn f239(v0: Adt16, v1: Adt2) -> Adt2 {
    let m1359 = v1.clone();
    match m1359 {
        Adt2::C0 => {
            v1
        }
        Adt2::C1 => {
            v1
        }
        Adt2::C2 => {
            v1
        }
        Adt2::C3 => {
            v1
        }
        Adt2::C4 => {
            v1
        }
        Adt2::C5 => {
            match v0 {
                Adt16::C0(_, _, h1360, _) => {
                    h1360
                }
            }
        }
        Adt2::C6(_) => {
            v1
        }
        Adt2::C7(_) => {
            v1
        }
        Adt2::C8(_) => {
            v1
        }
        Adt2::C9(_, _) => {
            v1
        }
        Adt2::C10(_, _) => {
            v1
        }
        Adt2::C11 => {
            v1
        }
        Adt2::C12 => {
            v1
        }
        Adt2::C13 => {
            v1
        }
        Adt2::C14 => {
            v1
        }
        Adt2::C15(_) => {
            v1
        }
        Adt2::C16 => {
            v1
        }
        Adt2::C17 => {
            v1
        }
        Adt2::C18 => {
            v1
        }
        Adt2::C19(_, _) => {
            v1
        }
    }
}

pub fn f240(v0: Adt2) -> bool {
    let m1361 = v0;
    match m1361 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            true
        }
        Adt2::C17 => {
            false
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f241(v0: Bytes, v1: Adt14, v2: bool, v3: Bytes) -> bool {
    if v2 {
        true
    } else {
        let m1362 = f153(v1, v3.clone());
        match m1362 {
            None => {
                let m1364 = f154(v3);
                match m1364 {
                    None => {
                        false
                    }
                    Some(v5) => {
                        f40(v5, v0)
                    }
                }
            }
            Some(v4) => {
                f223(Fn32::F248(v0), false, match v4 {
                    Adt10::C0(_, h1363) => {
                        h1363
                    }
                })
            }
        }
    }
}

pub fn f242(p1370: Adt3) -> bool {
    let m1369 = p1370;
    {
        let mut m1369 = m1369;
        loop {
            let step = {
                let v0 = m1369;
                let m1365 = v0;
                match m1365 {
                    Adt3::C0(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C17 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C21(v40, _, r1368) => {
                        let v42 = (*r1368).clone();
                        let a1366 = f207(v40);
                        let a1367 = f205(v42.clone());
                        if nat_lt(a1366, a1367) {
                            Ok::<Adt3, bool>(v42)
                        } else {
                            Err::<Adt3, bool>(false)
                        }
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, bool>(true)
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C30 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C31 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                }
            };
            match step {
                Ok(next) => { m1369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: Fn36, p1374: u64, p1375: List<Bytes>) -> R<u64> {
    let m1373 = (p1374, p1375);
    {
        let mut m1373 = m1373;
        loop {
            let step = {
                let (v1, v2) = m1373;
                let m1371 = v2;
                match m1371.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c1372: Fn36 = v0.clone();
                            c1372.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: Bytes, v1: u64, v2: Bytes) -> R<u64> {
    if f40(v2, v0) {
        let a1376 = v1;
        let a1377 = 1u64;
        nat_add(a1376, a1377)
    } else {
        Ok(v1)
    }
}

pub fn f245(v0: Bytes, v1: u64) -> Option<u8> {
    let a1378 = v0;
    let a1379 = v1;
    index_bytes(a1378, a1379)
}

pub fn f246(v0: u8, v1: u8, v2: u8) -> bool {
    let m1382 = {
        let a1380 = v0;
        let a1381 = v1;
        fixed_u8::checked_sub(a1380, a1381)
    };
    match m1382 {
        None => {
            false
        }
        Some(_) => {
            let m1385 = {
                let a1383 = v2;
                let a1384 = v0;
                fixed_u8::checked_sub(a1383, a1384)
            };
            match m1385 {
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

pub fn f247(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f249(v0, v1, 48u8) {
        Ok(Some({
            let a1386 = v1;
            let a1387 = 1u64;
            nat_add(a1386, a1387)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f248(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f40(match v2 {
            Adt5::C0(h1388, _, _, _, _) => {
                h1388
            }
        }, v0)
    }
}

pub fn f249(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m1389 = f245(v0, v1);
    match m1389 {
        None => {
            false
        }
        Some(v3) => {
            let a1390 = v3;
            let a1391 = v2;
            equal(a1390, a1391)
        }
    }
}
