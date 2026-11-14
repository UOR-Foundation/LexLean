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
    C0(u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, Adt5),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6, Bytes, Adt2, u64, u64, List<Adt7>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(List<u64>, u64, u64),
    C1(Adt2),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, Bytes, Option<Adt9>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Bytes, List<Adt0>, List<Adt2>, List<List<Adt3>>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Adt11, List<Adt13>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, u64, bool, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt11, List<Adt3>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Fn0 {
    F7(Adt10),
}

impl Fn0 {
    pub fn apply(&self, p0: (List<Adt4>, Adt12), p1: (u64, List<Adt7>)) -> R<(List<Adt4>, Adt12)> {
        match self {
            Fn0::F7(k0) => f7(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F21(Adt10, (u64, List<Adt7>)),
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt13) -> R<List<Adt4>> {
        match self {
            Fn1::F21(k0, k1) => f21(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F29(Adt10, u64, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt13>, p1: Adt3) -> R<List<Adt13>> {
        match self {
            Fn2::F29(k0, k1, k2, k3) => f29(k0.clone(), *k1, *k2, *k3, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F34(Adt10),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn3::F34(k0) => f34(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F38(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn4::F38(k0, k1) => f38(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F44(Adt10, u64),
    F51(u64, Adt10),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F44(k0, k1) => f44(k0.clone(), *k1, p0),
            Fn5::F51(k0, k1) => f51(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F47(Adt10, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: (Bytes, u64)) -> R<Option<(Bytes, u64)>> {
        match self {
            Fn6::F47(k0, k1) => f47(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F50(u64),
}

impl Fn7 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn7::F50(k0) => f50(*k0, p0, p1),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>, v2: List<Adt2>, v3: List<List<Adt3>>, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: bool) -> R<Result<List<Adt4>, Adt8>> {
    let v11: Adt10 = f1(v0, v1, v2, v3, v4, v5, v6);
    let v12: (Adt11, Option<Adt8>) = f2(v11.clone(), f3(v7, v8))?;
    let m2 = {
        let (_, h1) = v12;
        h1
    };
    match m2 {
        None => {
            let v16: List<(u64, List<Adt7>)> = if v10 {
                let a9 = {
                    let m3 = f4(v11.clone(), v9, f5(Str::lit("a")));
                    match m3 {
                        None => {
                            List::<(u64, List<Adt7>)>::nil()
                        }
                        Some(v14) => {
                            List::cons(({
                                let a4 = v9;
                                let a5 = 1u64;
                                nat_add(a4, a5)?
                            }, List::cons(v14, List::<Adt7>::nil())), List::<(u64, List<Adt7>)>::nil())
                        }
                    }
                };
                let a10 = {
                    let m6 = f4(v11.clone(), v9, f5(Str::lit("an")));
                    match m6 {
                        None => {
                            List::<(u64, List<Adt7>)>::nil()
                        }
                        Some(v15) => {
                            List::cons(({
                                let a7 = v9;
                                let a8 = 1u64;
                                nat_add(a7, a8)?
                            }, List::cons(v15, List::<Adt7>::nil())), List::<(u64, List<Adt7>)>::nil())
                        }
                    }
                };
                append_list(a9, a10)
            } else {
                List::cons((v9, List::<Adt7>::nil()), List::<(u64, List<Adt7>)>::nil())
            };
            let v17: (List<Adt4>, Adt12) = f6(Fn0::F7(v11), (List::<Adt4>::nil(), Adt12::C0({
                let (h11, _) = v12;
                h11
            }, List::<Adt13>::nil(), None::<Adt8>)), v16)?;
            let m15 = {
                let m14 = {
                    let (_, h13) = v17;
                    h13
                };
                match m14 {
                    Adt12::C0(_, _, h12) => {
                        h12
                    }
                }
            };
            match m15 {
                None => {
                    Ok(Ok::<List<Adt4>, Adt8>({
                        let (h16, _) = v17;
                        h16
                    }))
                }
                Some(v18) => {
                    Ok(Err::<List<Adt4>, Adt8>(v18))
                }
            }
        }
        Some(v13) => {
            Ok(Err::<List<Adt4>, Adt8>(v13))
        }
    }
}

pub fn f1(v0: Bytes, v1: List<Adt0>, v2: List<Adt2>, v3: List<List<Adt3>>, v4: u64, v5: u64, v6: u64) -> Adt10 {
    Adt10::C0(v0, v1, v2, v3, v5, v6, v4)
}

pub fn f2(v0: Adt10, v1: Adt11) -> R<(Adt11, Option<Adt8>)> {
    let v2: u64 = {
        let a18 = match v1.clone() {
            Adt11::C0(h17, _, _) => {
                h17
            }
        };
        let a19 = 1u64;
        nat_add(a18, a19)?
    };
    let v3: Adt11 = Adt11::C0(v2, match v1.clone() {
        Adt11::C0(_, h20, _) => {
            h20
        }
    }, match v1 {
        Adt11::C0(_, _, h21) => {
            h21
        }
    });
    let a23 = match v0.clone() {
        Adt10::C0(_, _, _, _, h22, _, _) => {
            h22
        }
    };
    let a24 = v2;
    if nat_lt(a23, a24) {
        Ok((v3, Some(f8(f5(Str::lit("LLS8002")), f9(f9(f9(f9(f5(Str::lit("max_parse_states exceeded in phase parse: configured ")), f10(match v0 {
            Adt10::C0(_, _, _, _, h25, _, _) => {
                h25
            }
        })), f5(Str::lit(", observed "))), f10(v2)), f5(Str::lit(" states"))), None::<Adt9>))))
    } else {
        Ok((v3, None::<Adt8>))
    }
}

pub fn f3(v0: u64, v1: u64) -> Adt11 {
    Adt11::C0(v0, v1, List::<u64>::nil())
}

pub fn f4(v0: Adt10, v1: u64, v2: Bytes) -> Option<Adt7> {
    let m26 = f11(v0.clone(), v1);
    match m26 {
        None => {
            None::<Adt7>
        }
        Some(v3) => {
            if f12({
                let m28 = f13(v0, v3);
                match m28 {
                    Adt0::C0(_, _, _, _, _, _, _, h27) => {
                        h27
                    }
                }
            }, v2.clone()) {
                let m29 = f14(v2);
                match m29 {
                    None => {
                        None::<Adt7>
                    }
                    Some(v4) => {
                        Some(Adt7::C0(v3, v4))
                    }
                }
            } else {
                None::<Adt7>
            }
        }
    }
}

pub fn f5(v0: Str) -> Bytes {
    let a30 = v0;
    utf8_encode(a30)
}

pub fn f6(v0: Fn0, p34: (List<Adt4>, Adt12), p35: List<(u64, List<Adt7>)>) -> R<(List<Adt4>, Adt12)> {
    let m33 = (p34, p35);
    {
        let mut m33 = m33;
        loop {
            let step = {
                let (v1, v2) = m33;
                let m31 = v2;
                match m31.uncons() {
                    None => {
                        Err::<((List<Adt4>, Adt12), List<(u64, List<Adt7>)>), R<(List<Adt4>, Adt12)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt4>, Adt12), List<(u64, List<Adt7>)>), R<(List<Adt4>, Adt12)>>(({
                            let c32: Fn0 = v0.clone();
                            c32.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m33 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt10, v1: (List<Adt4>, Adt12), v2: (u64, List<Adt7>)) -> R<(List<Adt4>, Adt12)> {
    f15(v0, v1, v2)
}

pub fn f8(v0: Bytes, v1: Bytes, v2: Option<Adt9>) -> Adt8 {
    Adt8::C0(v0, v1, v2, None::<Bytes>)
}

pub fn f9(v0: Bytes, v1: Bytes) -> Bytes {
    let a36 = v0;
    let a37 = v1;
    append_bytes(a36, a37)
}

pub fn f10(v0: u64) -> Bytes {
    f16(v0)
}

pub fn f11(v0: Adt10, v1: u64) -> Option<u64> {
    let m38 = f17(v0.clone(), v1);
    match m38 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            let m39 = v2;
            match m39 {
                Adt2::C0(v3) => {
                    let a40 = f18(f13(v0, v3));
                    let a41 = 1u64;
                    if nat_eq(a40, a41) {
                        Some(v3)
                    } else {
                        None::<u64>
                    }
                }
                Adt2::C1(_, _, _, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f12(v0: Bytes, v1: Bytes) -> bool {
    let a42 = v0;
    let a43 = v1;
    equal(a42, a43)
}

pub fn f13(v0: Adt10, v1: u64) -> Adt0 {
    let m47 = {
        let a45 = match v0 {
            Adt10::C0(_, h44, _, _, _, _, _) => {
                h44
            }
        };
        let a46 = v1;
        index_list(a45, a46)
    };
    match m47 {
        None => {
            Adt0::C0(Adt1::C6, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64, f5(Str::lit("")))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f14(v0: Bytes) -> Option<Bytes> {
    if if f12(v0.clone(), f5(Str::lit("a"))) {
        true
    } else {
        f12(v0.clone(), f5(Str::lit("A")))
    } {
        Some(f5(Str::lit("a")))
    } else {
        if if f12(v0.clone(), f5(Str::lit("an"))) {
            true
        } else {
            f12(v0.clone(), f5(Str::lit("An")))
        } {
            Some(f5(Str::lit("an")))
        } else {
            if f12(v0.clone(), f5(Str::lit("and"))) {
                Some(f5(Str::lit("and")))
            } else {
                if f12(v0.clone(), f5(Str::lit("as"))) {
                    Some(f5(Str::lit("as")))
                } else {
                    if f12(v0.clone(), f5(Str::lit("defined"))) {
                        Some(f5(Str::lit("defined")))
                    } else {
                        if f12(v0.clone(), f5(Str::lit("every"))) {
                            Some(f5(Str::lit("every")))
                        } else {
                            if f12(v0.clone(), f5(Str::lit("exactly"))) {
                                Some(f5(Str::lit("exactly")))
                            } else {
                                if f12(v0.clone(), f5(Str::lit("exists"))) {
                                    Some(f5(Str::lit("exists")))
                                } else {
                                    if if f12(v0.clone(), f5(Str::lit("for"))) {
                                        true
                                    } else {
                                        f12(v0.clone(), f5(Str::lit("For")))
                                    } {
                                        Some(f5(Str::lit("for")))
                                    } else {
                                        if f12(v0.clone(), f5(Str::lit("holds"))) {
                                            Some(f5(Str::lit("holds")))
                                        } else {
                                            if if f12(v0.clone(), f5(Str::lit("if"))) {
                                                true
                                            } else {
                                                f12(v0.clone(), f5(Str::lit("If")))
                                            } {
                                                Some(f5(Str::lit("if")))
                                            } else {
                                                if f12(v0.clone(), f5(Str::lit("implies"))) {
                                                    Some(f5(Str::lit("implies")))
                                                } else {
                                                    if f12(v0.clone(), f5(Str::lit("is"))) {
                                                        Some(f5(Str::lit("is")))
                                                    } else {
                                                        if if f12(v0.clone(), f5(Str::lit("not"))) {
                                                            true
                                                        } else {
                                                            f12(v0.clone(), f5(Str::lit("Not")))
                                                        } {
                                                            Some(f5(Str::lit("not")))
                                                        } else {
                                                            if f12(v0.clone(), f5(Str::lit("one"))) {
                                                                Some(f5(Str::lit("one")))
                                                            } else {
                                                                if f12(v0.clone(), f5(Str::lit("only"))) {
                                                                    Some(f5(Str::lit("only")))
                                                                } else {
                                                                    if f12(v0.clone(), f5(Str::lit("or"))) {
                                                                        Some(f5(Str::lit("or")))
                                                                    } else {
                                                                        if f12(v0.clone(), f5(Str::lit("such"))) {
                                                                            Some(f5(Str::lit("such")))
                                                                        } else {
                                                                            if f12(v0.clone(), f5(Str::lit("that"))) {
                                                                                Some(f5(Str::lit("that")))
                                                                            } else {
                                                                                if if f12(v0.clone(), f5(Str::lit("the"))) {
                                                                                    true
                                                                                } else {
                                                                                    f12(v0.clone(), f5(Str::lit("The")))
                                                                                } {
                                                                                    Some(f5(Str::lit("the")))
                                                                                } else {
                                                                                    if f12(v0.clone(), f5(Str::lit("then"))) {
                                                                                        Some(f5(Str::lit("then")))
                                                                                    } else {
                                                                                        if if f12(v0.clone(), f5(Str::lit("there"))) {
                                                                                            true
                                                                                        } else {
                                                                                            f12(v0.clone(), f5(Str::lit("There")))
                                                                                        } {
                                                                                            Some(f5(Str::lit("there")))
                                                                                        } else {
                                                                                            if f12(v0.clone(), f5(Str::lit("when"))) {
                                                                                                Some(f5(Str::lit("when")))
                                                                                            } else {
                                                                                                if f12(v0, f5(Str::lit("of"))) {
                                                                                                    Some(f5(Str::lit("of")))
                                                                                                } else {
                                                                                                    None::<Bytes>
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
                    }
                }
            }
        }
    }
}

pub fn f15(v0: Adt10, v1: (List<Adt4>, Adt12), v2: (u64, List<Adt7>)) -> R<(List<Adt4>, Adt12)> {
    let v3: Adt12 = {
        let (_, h48) = v1.clone();
        h48
    };
    let m50 = match v3.clone() {
        Adt12::C0(_, _, h49) => {
            h49
        }
    };
    match m50 {
        None => {
            let v5: Adt12 = f19(v0.clone(), match v3 {
                Adt12::C0(h51, _, _) => {
                    h51
                }
            }, {
                let (h52, _) = v2.clone();
                h52
            }, 0u64, 0u64)?;
            let m54 = match v5.clone() {
                Adt12::C0(_, _, h53) => {
                    h53
                }
            };
            match m54 {
                None => {
                    let v7: List<Adt4> = f20(Fn1::F21(v0.clone(), v2.clone()), {
                        let (h56, _) = v1;
                        h56
                    }, match v5.clone() {
                        Adt12::C0(_, h57, _) => {
                            h57
                        }
                    })?;
                    let v10: List<Adt4> = {
                        let m59 = f22(v0.clone(), {
                            let (h58, _) = v2.clone();
                            h58
                        });
                        match m59 {
                            None => {
                                List::<Adt4>::nil()
                            }
                            Some(v8) => {
                                let m63 = f23(v0, {
                                    let a61 = {
                                        let (h60, _) = v2;
                                        h60
                                    };
                                    let a62 = 1u64;
                                    nat_add(a61, a62)?
                                }, Adt6::C1(v8))?;
                                match m63 {
                                    None => {
                                        List::<Adt4>::nil()
                                    }
                                    Some(v9) => {
                                        List::cons(f24(v9, {
                                            let (_, h64) = v2;
                                            h64
                                        }), List::<Adt4>::nil())
                                    }
                                }
                            }
                        }
                    };
                    Ok(({
                        let a65 = v7;
                        let a66 = v10;
                        append_list(a65, a66)
                    }, v5))
                }
                Some(_) => {
                    Ok(({
                        let (h55, _) = v1;
                        h55
                    }, v5))
                }
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f16(v0: u64) -> Bytes {
    f25(f26(), 10u64, v0)
}

pub fn f17(v0: Adt10, v1: u64) -> Option<Adt2> {
    let a68 = match v0 {
        Adt10::C0(_, _, h67, _, _, _, _) => {
            h67
        }
    };
    let a69 = v1;
    index_list(a68, a69)
}

pub fn f18(v0: Adt0) -> u64 {
    let m71 = match v0 {
        Adt0::C0(h70, _, _, _, _, _, _, _) => {
            h70
        }
    };
    match m71 {
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

pub fn f19(v0: Adt10, v1: Adt11, v2: u64, v3: u64, v4: u64) -> R<Adt12> {
    let m72 = f17(v0.clone(), v2);
    match m72 {
        None => {
            Ok(Adt12::C0(v1, List::<Adt13>::nil(), None::<Adt8>))
        }
        Some(v5) => {
            let m73 = v5;
            match m73 {
                Adt2::C0(v6) => {
                    let v7: Adt14 = f27(v0.clone(), v1, v6)?;
                    let m75 = match v7.clone() {
                        Adt14::C0(_, _, h74) => {
                            h74
                        }
                    };
                    match m75 {
                        None => {
                            Ok(Adt12::C0(match v7.clone() {
                                Adt14::C0(h77, _, _) => {
                                    h77
                                }
                            }, f28(Fn2::F29(v0, v4, v3, v2), List::<Adt13>::nil(), match v7 {
                                Adt14::C0(_, h78, _) => {
                                    h78
                                }
                            })?, None::<Adt8>))
                        }
                        Some(v8) => {
                            Ok(Adt12::C0(match v7 {
                                Adt14::C0(h76, _, _) => {
                                    h76
                                }
                            }, List::<Adt13>::nil(), Some(v8)))
                        }
                    }
                }
                Adt2::C1(_, _, _, _, _) => {
                    Ok(Adt12::C0(v1, List::<Adt13>::nil(), None::<Adt8>))
                }
            }
        }
    }
}

pub fn f20(v0: Fn1, p82: List<Adt4>, p83: List<Adt13>) -> R<List<Adt4>> {
    let m81 = (p82, p83);
    {
        let mut m81 = m81;
        loop {
            let step = {
                let (v1, v2) = m81;
                let m79 = v2;
                match m79.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt13>), R<List<Adt4>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt13>), R<List<Adt4>>>(({
                            let c80: Fn1 = v0.clone();
                            c80.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m81 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt10, v1: (u64, List<Adt7>), v2: List<Adt4>, v3: Adt13) -> R<List<Adt4>> {
    if match v3.clone() {
        Adt13::C0(_, _, h84, _) => {
            h84
        }
    } {
        Ok(v2)
    } else {
        let m93 = f23(v0.clone(), match v3.clone() {
            Adt13::C0(_, _, _, h85) => {
                h85
            }
        }, Adt6::C0(List::cons(match v3.clone() {
            Adt13::C0(h86, _, _, _) => {
                h86
            }
        }, List::<u64>::nil()), {
            let (h87, _) = f30(v0.clone(), {
                let (h88, _) = v1.clone();
                h88
            }, match v3.clone() {
                Adt13::C0(_, _, _, h89) => {
                    h89
                }
            })?;
            h87
        }, {
            let (_, h90) = f30(v0, {
                let (h91, _) = v1;
                h91
            }, match v3 {
                Adt13::C0(_, _, _, h92) => {
                    h92
                }
            })?;
            h90
        }))?;
        match m93 {
            None => {
                Ok(v2)
            }
            Some(v4) => {
                let a95 = v2;
                let a96 = List::cons(f24(v4, {
                    let (_, h94) = v1;
                    h94
                }), List::<Adt4>::nil());
                Ok(append_list(a95, a96))
            }
        }
    }
}

pub fn f22(v0: Adt10, v1: u64) -> Option<Adt2> {
    let m97 = f17(v0, v1);
    match m97 {
        None => {
            None::<Adt2>
        }
        Some(v2) => {
            let m98 = v2.clone();
            match m98 {
                Adt2::C0(_) => {
                    None::<Adt2>
                }
                Adt2::C1(_, _, _, _, _) => {
                    Some(v2)
                }
            }
        }
    }
}

pub fn f23(v0: Adt10, v1: u64, v2: Adt6) -> R<Option<Adt4>> {
    let m99 = f17(v0.clone(), v1);
    match m99 {
        None => {
            Ok(None::<Adt4>)
        }
        Some(v3) => {
            let m100 = v3.clone();
            match m100 {
                Adt2::C0(_) => {
                    Ok(None::<Adt4>)
                }
                Adt2::C1(_, v6, v7, _, _) => {
                    let v10: u64 = f31(v0.clone(), v7, v6)?;
                    let v11: Adt0 = f13(v0.clone(), v10);
                    if f32({
                        let a101 = f18(v11.clone());
                        let a102 = 1u64;
                        nat_eq(a101, a102)
                    }, {
                        let a105 = v10;
                        let a106 = {
                            let a104 = match v0.clone() {
                                Adt10::C0(_, h103, _, _, _, _, _) => {
                                    h103
                                }
                            };
                            length_list(a104)
                        };
                        nat_lt(a105, a106)
                    }) {
                        let v12: u64 = {
                            let (h107, _) = {
                                let (h108, _) = f33(Fn3::F34(v0.clone()), {
                                    let a111 = {
                                        let a110 = match v0.clone() {
                                            Adt10::C0(_, h109, _, _, _, _, _) => {
                                                h109
                                            }
                                        };
                                        length_list(a110)
                                    };
                                    let a112 = 1u64;
                                    nat_add(a111, a112)?
                                }, ({
                                    let a113 = v10;
                                    let a114 = 1u64;
                                    nat_add(a113, a114)?
                                }, match v11 {
                                    Adt0::C0(_, _, h115, _, _, _, _, _) => {
                                        h115
                                    }
                                }))?;
                                h108
                            };
                            h107
                        };
                        let a116 = f31(v0.clone(), v7, v12)?;
                        let a117 = v7;
                        if nat_eq(a116, a117) {
                            Ok(Some(Adt4::C0({
                                let a118 = v1;
                                let a119 = 1u64;
                                nat_add(a118, a119)?
                            }, Adt5::C0(v2, f35(v0, v10, v12)?, v3, v10, v12, List::<Adt7>::nil()))))
                        } else {
                            Ok(None::<Adt4>)
                        }
                    } else {
                        Ok(None::<Adt4>)
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt4, v1: List<Adt7>) -> Adt4 {
    let v2: Adt5 = match v0.clone() {
        Adt4::C0(_, h120) => {
            h120
        }
    };
    Adt4::C0(match v0 {
        Adt4::C0(h121, _) => {
            h121
        }
    }, Adt5::C0(match v2.clone() {
        Adt5::C0(h122, _, _, _, _, _) => {
            h122
        }
    }, match v2.clone() {
        Adt5::C0(_, h123, _, _, _, _) => {
            h123
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h124, _, _, _) => {
            h124
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, h125, _, _) => {
            h125
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, h126, _) => {
            h126
        }
    }, {
        let a128 = v1;
        let a129 = match v2 {
            Adt5::C0(_, _, _, _, _, h127) => {
                h127
            }
        };
        append_list(a128, a129)
    }))
}

pub fn f25(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a130 = v2;
    let a131 = 0u64;
    if nat_eq(a130, a131) {
        f36(v0, 0u64)
    } else {
        let (_, h132) = {
            let (h133, _) = f37(Fn4::F38(v1, v0), v2, (v2, Bytes::lit(&[])));
            h133
        };
        h132
    }
}

pub fn f26() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f27(v0: Adt10, v1: Adt11, v2: u64) -> R<Adt14> {
    let v4: List<Adt3> = {
        let m137 = {
            let a135 = match v0.clone() {
                Adt10::C0(_, _, _, h134, _, _, _) => {
                    h134
                }
            };
            let a136 = v2;
            index_list(a135, a136)
        };
        match m137 {
            None => {
                List::<Adt3>::nil()
            }
            Some(v3) => {
                v3
            }
        }
    };
    if f39(v2, match v1.clone() {
        Adt11::C0(_, _, h138) => {
            h138
        }
    }) {
        Ok(Adt14::C0(v1, v4, None::<Adt8>))
    } else {
        let v5: u64 = {
            let a141 = match v1.clone() {
                Adt11::C0(_, h139, _) => {
                    h139
                }
            };
            let a142 = {
                let a140 = v4.clone();
                length_list(a140)
            };
            nat_add(a141, a142)?
        };
        let a144 = match v0.clone() {
            Adt10::C0(_, _, _, _, _, _, h143) => {
                h143
            }
        };
        let a145 = v5;
        if nat_lt(a144, a145) {
            Ok(Adt14::C0(Adt11::C0(match v1.clone() {
                Adt11::C0(h146, _, _) => {
                    h146
                }
            }, v5, match v1 {
                Adt11::C0(_, _, h147) => {
                    h147
                }
            }), List::<Adt3>::nil(), Some(f8(f5(Str::lit("LLS8002")), f9(f9(f9(f9(f5(Str::lit("max_token_lattice_edges exceeded in phase lexical resolution: configured ")), f10(match v0 {
                Adt10::C0(_, _, _, _, _, _, h148) => {
                    h148
                }
            })), f5(Str::lit(", observed "))), f10(v5)), f5(Str::lit(" distinct lattice edges"))), None::<Adt9>))))
        } else {
            Ok(Adt14::C0(Adt11::C0(match v1.clone() {
                Adt11::C0(h149, _, _) => {
                    h149
                }
            }, v5, List::cons(v2, match v1 {
                Adt11::C0(_, _, h150) => {
                    h150
                }
            })), v4, None::<Adt8>))
        }
    }
}

pub fn f28(v0: Fn2, p154: List<Adt13>, p155: List<Adt3>) -> R<List<Adt13>> {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v1, v2) = m153;
                let m151 = v2;
                match m151.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt3>), R<List<Adt13>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt13>, List<Adt3>), R<List<Adt13>>>(({
                            let c152: Fn2 = v0.clone();
                            c152.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt10, v1: u64, v2: u64, v3: u64, v4: List<Adt13>, v5: Adt3) -> R<List<Adt13>> {
    let a166 = v4;
    let a167 = if f32({
        let a157 = v2;
        let a158 = match v5.clone() {
            Adt3::C0(_, _, h156, _) => {
                h156
            }
        };
        nat_le(a157, a158)
    }, {
        let a160 = match v5.clone() {
            Adt3::C0(_, _, h159, _) => {
                h159
            }
        };
        let a161 = v1;
        nat_le(a160, a161)
    }) {
        List::cons(Adt13::C0(match v5.clone() {
            Adt3::C0(h162, _, _, _) => {
                h162
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h163, _) => {
                h163
            }
        }, match v5.clone() {
            Adt3::C0(_, _, _, h164) => {
                h164
            }
        }, f40(v0, v3, match v5 {
            Adt3::C0(_, h165, _, _) => {
                h165
            }
        })?), List::<Adt13>::nil())
    } else {
        List::<Adt13>::nil()
    };
    Ok(append_list(a166, a167))
}

pub fn f30(v0: Adt10, v1: u64, v2: u64) -> R<(u64, u64)> {
    let v4: Adt2 = {
        let m168 = f17(v0.clone(), v1);
        match m168 {
            None => {
                Adt2::C0(0u64)
            }
            Some(v3) => {
                v3
            }
        }
    };
    let v5: u64 = {
        let a171 = v1;
        let a172 = {
            let a169 = v2;
            let a170 = 1u64;
            nat_sub(a169, a170)
        };
        if nat_lt(a171, a172) {
            let a173 = v2;
            let a174 = 1u64;
            nat_sub(a173, a174)
        } else {
            v1
        }
    };
    let v7: Adt2 = {
        let m175 = f17(v0, v5);
        match m175 {
            None => {
                Adt2::C0(0u64)
            }
            Some(v6) => {
                v6
            }
        }
    };
    Ok((f41(v4), {
        let a176 = f42(v7);
        let a177 = 1u64;
        nat_add(a176, a177)?
    }))
}

pub fn f31(v0: Adt10, v1: u64, v2: u64) -> R<u64> {
    let (h178, _) = f43(Fn5::F44(v0.clone(), v1), {
        let a181 = {
            let a180 = match v0 {
                Adt10::C0(_, h179, _, _, _, _, _) => {
                    h179
                }
            };
            length_list(a180)
        };
        let a182 = 1u64;
        nat_add(a181, a182)?
    }, v2)?;
    Ok(h178)
}

pub fn f32(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f33(v0: Fn3, p187: u64, p188: (u64, u64)) -> R<((u64, u64), bool)> {
    let m186 = (p187, p188);
    {
        let mut m186 = m186;
        loop {
            let step = {
                let (v1, v2) = m186;
                let m183 = v1;
                if m183 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m183 - 1;
                    let m185 = {
                        let c184: Fn3 = v0.clone();
                        c184.apply(v2)?
                    };
                    match m185 {
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
                Ok(next) => { m186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Adt10, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f45(v0, v1)
}

pub fn f35(v0: Adt10, v1: u64, v2: u64) -> R<Bytes> {
    let (h189, _) = {
        let (h190, _) = f46(Fn6::F47(v0, v2), {
            let a193 = {
                let a191 = v2;
                let a192 = v1;
                nat_sub(a191, a192)
            };
            let a194 = 1u64;
            nat_add(a193, a194)?
        }, (f5(Str::lit("")), v1))?;
        h190
    };
    Ok(h189)
}

pub fn f36(v0: Bytes, v1: u64) -> Bytes {
    let m198 = {
        let a195 = v0;
        let a196 = v1;
        let a197 = 1u64;
        slice_bytes(a195, a196, a197)
    };
    match m198 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f37(v0: Fn4, p203: u64, p204: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m202 = (p203, p204);
    {
        let mut m202 = m202;
        loop {
            let step = {
                let (v1, v2) = m202;
                let m199 = v1;
                if m199 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m199 - 1;
                    let m201 = {
                        let c200: Fn4 = v0.clone();
                        c200.apply(v2.clone())
                    };
                    match m201 {
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
                Ok(next) => { m202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f48(v1, v0, v2)
}

pub fn f39(v0: u64, v1: List<u64>) -> bool {
    f49(Fn7::F50(v0), false, v1)
}

pub fn f40(v0: Adt10, v1: u64, v2: u64) -> R<u64> {
    let (h205, _) = f43(Fn5::F51(v2, v0.clone()), {
        let a208 = {
            let a207 = match v0 {
                Adt10::C0(_, _, h206, _, _, _, _) => {
                    h206
                }
            };
            length_list(a207)
        };
        let a209 = 1u64;
        nat_add(a208, a209)?
    }, v1)?;
    Ok(h205)
}

pub fn f41(v0: Adt2) -> u64 {
    let m210 = v0;
    match m210 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f42(v0: Adt2) -> u64 {
    let m211 = v0;
    match m211 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(_, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f43(v0: Fn5, p216: u64, p217: u64) -> R<(u64, bool)> {
    let m215 = (p216, p217);
    {
        let mut m215 = m215;
        loop {
            let step = {
                let (v1, v2) = m215;
                let m212 = v1;
                if m212 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m212 - 1;
                    let m214 = {
                        let c213: Fn5 = v0.clone();
                        c213.apply(v2)?
                    };
                    match m214 {
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
                Ok(next) => { m215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Adt10, v1: u64, v2: u64) -> R<Option<u64>> {
    f52(v0, v1, v2)
}

pub fn f45(v0: Adt10, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m222 = {
        let a220 = match v0 {
            Adt10::C0(_, h218, _, _, _, _, _) => {
                h218
            }
        };
        let a221 = {
            let (h219, _) = v1;
            h219
        };
        index_list(a220, a221)
    };
    match m222 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f18(v2.clone());
            if f32({
                let a225 = match v2.clone() {
                    Adt0::C0(_, h223, _, _, _, _, _, _) => {
                        h223
                    }
                };
                let a226 = {
                    let (_, h224) = v1;
                    h224
                };
                nat_eq(a225, a226)
            }, {
                let a227 = v3;
                let a228 = 1u64;
                if if nat_eq(a227, a228) {
                    true
                } else {
                    let a229 = v3;
                    let a230 = 2u64;
                    nat_eq(a229, a230)
                } {
                    true
                } else {
                    f32({
                        let a231 = v3;
                        let a232 = 4u64;
                        nat_eq(a231, a232)
                    }, if f12(match v2.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, h233) => {
                            h233
                        }
                    }, f5(Str::lit("_"))) {
                        true
                    } else {
                        f12(match v2.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, h234) => {
                                h234
                            }
                        }, f5(Str::lit("'")))
                    })
                }
            }) {
                Ok(Some(({
                    let a236 = {
                        let (h235, _) = v1;
                        h235
                    };
                    let a237 = 1u64;
                    nat_add(a236, a237)?
                }, match v2 {
                    Adt0::C0(_, _, h238, _, _, _, _, _) => {
                        h238
                    }
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f46(v0: Fn6, p243: u64, p244: (Bytes, u64)) -> R<((Bytes, u64), bool)> {
    let m242 = (p243, p244);
    {
        let mut m242 = m242;
        loop {
            let step = {
                let (v1, v2) = m242;
                let m239 = v1;
                if m239 == 0 {
                    Err::<(u64, (Bytes, u64)), R<((Bytes, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m239 - 1;
                    let m241 = {
                        let c240: Fn6 = v0.clone();
                        c240.apply(v2.clone())?
                    };
                    match m241 {
                        None => {
                            Err::<(u64, (Bytes, u64)), R<((Bytes, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Bytes, u64)), R<((Bytes, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Adt10, v1: u64, v2: (Bytes, u64)) -> R<Option<(Bytes, u64)>> {
    let a246 = {
        let (_, h245) = v2.clone();
        h245
    };
    let a247 = v1;
    if nat_lt(a246, a247) {
        Ok(Some((f9({
            let (h248, _) = v2.clone();
            h248
        }, {
            let m251 = f13(v0, {
                let (_, h250) = v2.clone();
                h250
            });
            match m251 {
                Adt0::C0(_, _, _, _, _, _, _, h249) => {
                    h249
                }
            }
        }), {
            let a253 = {
                let (_, h252) = v2;
                h252
            };
            let a254 = 1u64;
            nat_add(a253, a254)?
        })))
    } else {
        Ok(None::<(Bytes, u64)>)
    }
}

pub fn f48(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h255, _) = v2;
        h255
    };
    let a256 = v3;
    let a257 = 0u64;
    if nat_eq(a256, a257) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a258 = v3;
            let a259 = v1;
            let a260 = 0u64;
            nat_quot(a258, a259, a260)
        }, {
            let a265 = f36(v0, {
                let a261 = v3;
                let a262 = v1;
                let a263 = 0u64;
                nat_rem(a261, a262, a263)
            });
            let a266 = {
                let (_, h264) = v2;
                h264
            };
            append_bytes(a265, a266)
        }))
    }
}

pub fn f49(v0: Fn7, p270: bool, p271: List<u64>) -> bool {
    let m269 = (p270, p271);
    {
        let mut m269 = m269;
        loop {
            let step = {
                let (v1, v2) = m269;
                let m267 = v2;
                match m267.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c268: Fn7 = v0.clone();
                            c268.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a272 = v2;
        let a273 = v0;
        nat_eq(a272, a273)
    }
}

pub fn f51(v0: u64, v1: Adt10, v2: u64) -> R<Option<u64>> {
    let m274 = f17(v1, v2);
    match m274 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a275 = f41(v3);
            let a276 = v0;
            if nat_lt(a275, a276) {
                Ok(Some({
                    let a277 = v2;
                    let a278 = 1u64;
                    nat_add(a277, a278)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f52(v0: Adt10, v1: u64, v2: u64) -> R<Option<u64>> {
    if f32({
        let a279 = v2;
        let a280 = v1;
        nat_lt(a279, a280)
    }, {
        let a281 = f18(f13(v0, v2));
        let a282 = 6u64;
        nat_eq(a281, a282)
    }) {
        Ok(Some({
            let a283 = v2;
            let a284 = 1u64;
            nat_add(a283, a284)?
        }))
    } else {
        Ok(None::<u64>)
    }
}
