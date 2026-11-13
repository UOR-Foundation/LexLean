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
    C0,
    C1,
    C2,
    C3,
    C4(Adt1),
    C5,
    C6,
    C7,
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(u64),
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14,
    C15(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
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
    C7,
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt0, Adt3),
    C1(u64),
    C2(u64, Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(Adt5, Adt0, std::rc::Rc<Adt2>),
    C6(u64, std::rc::Rc<Adt2>),
    C7(u64, std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C9(Adt6, std::rc::Rc<Adt2>),
    C10(std::rc::Rc<Adt2>),
    C11(std::rc::Rc<Adt2>),
    C12(std::rc::Rc<Adt2>, u64),
    C13,
    C14(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C15(Adt5, List<u64>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Bytes),
    C6(Bytes),
    C7(Bytes),
    C8(Bytes),
    C9(Bytes),
    C10(Bytes),
    C11(Bytes),
    C12(Bytes),
    C13(Bytes),
    C14(Adt4),
    C15,
    C16(std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>),
    C22(u64, std::rc::Rc<Adt3>),
    C23,
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15(u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    C28,
    C29,
    C30,
    C31,
    C32,
    C33,
    C34,
    C35,
    C36,
    C37,
    C38,
    C39,
    C40(Adt1),
    C41(Adt0),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt8, Adt14, Adt15),
    C1(u64, Adt8, Adt8, bool, Adt16, Adt15),
    C2(Adt18, Adt19, Adt8, Adt24, Adt15),
    C3,
    C4(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt9),
    C1(Adt10),
    C2(Adt11, std::rc::Rc<Adt8>),
    C3(Adt12, std::rc::Rc<Adt8>, std::rc::Rc<Adt8>),
    C4(Adt13, u64),
    C5,
    C6(std::rc::Rc<Adt8>, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt13 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1(u64, Adt8, std::rc::Rc<Adt14>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Option<Adt10>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1(u64, List<Adt17>, bool, std::rc::Rc<Adt16>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4(u64),
    C5(u64),
    C6(u64),
    C7(u64),
    C8(u64),
    C9(Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1(Adt20, Adt8, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Adt21),
    C1(Adt18),
    C2(std::rc::Rc<Adt20>),
    C3(Adt22, std::rc::Rc<Adt20>),
    C4(u64, u64, std::rc::Rc<Adt20>),
    C5,
    C6(std::rc::Rc<Adt20>, std::rc::Rc<Adt20>),
    C7(Adt23),
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt22 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt23 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt25, Adt15),
    C1(Adt26, Adt18, Adt15),
    C2(Adt27, std::rc::Rc<Adt24>, Adt15),
    C3(Adt28, std::rc::Rc<Adt24>, bool, Adt15),
    C4(Adt18, std::rc::Rc<Adt24>, bool, Adt15),
    C5(Adt30, std::rc::Rc<Adt24>, Adt15),
    C6(std::rc::Rc<Adt24>, std::rc::Rc<Adt24>, Adt15),
    C7(std::rc::Rc<Adt24>, std::rc::Rc<Adt24>, std::rc::Rc<Adt24>, Adt15),
    C8(std::rc::Rc<Adt24>, std::rc::Rc<Adt24>, Adt15),
    C9(std::rc::Rc<Adt24>),
    C10(Adt31, Adt18, Adt15),
    C11(std::rc::Rc<Adt24>, std::rc::Rc<Adt24>),
    C12,
    C13(std::rc::Rc<Adt24>, std::rc::Rc<Adt24>),
    C14(Adt20, bool, Adt8, std::rc::Rc<Adt24>, Adt15, std::rc::Rc<Adt24>),
    C15(Adt20, std::rc::Rc<Adt24>, std::rc::Rc<Adt24>),
    C16(Adt18, std::rc::Rc<Adt24>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt10, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt23),
    C8,
}

#[derive(Clone)]
pub enum Adt26 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt27 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt28 {
    C0(u64),
    C1(Adt29),
    C2(u64, Option<Adt10>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(u64, Option<Adt10>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Adt8),
    C1,
    C2(Adt8, Adt8),
    C3(Adt8, Adt8),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt8),
}

#[derive(Clone)]
pub enum Adt31 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt0, Adt34),
}

#[derive(Clone)]
pub enum Adt34 {
    C0,
    C1(Adt0, u64, Adt0, std::rc::Rc<Adt34>),
}

#[derive(Clone)]
pub enum Adt35 {
    C0,
    C1(Adt0, Adt0, std::rc::Rc<Adt35>),
}

#[derive(Clone)]
pub enum Fn0 {
    F20,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt35) -> R<Option<Adt35>> {
        match self {
            Fn0::F20 => f20(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: List<bool>, v8: List<bool>) -> R<Adt7> {
    let v9: Adt32 = Adt32::C0(v0, v1.clone(), v2, v3, v4, v5.clone());
    let v10: Adt33 = f1(v9.clone(), Adt33::C0(Adt0::C14, Adt34::C0), v5.clone(), 0u64, List::<bool>::nil(), v6.clone())?;
    let v11: Adt0 = f2(match v10.clone() {
        Adt33::C0(h1, _) => {
            h1
        }
    }, v1.clone());
    let v12: Adt33 = f1(v9.clone(), Adt33::C0(v11.clone(), Adt34::C0), v5, 0u64, v7, v6)?;
    let v13: Adt0 = f3(f4(f5(v1.clone())?), v11.clone());
    let v14: Adt35 = f6(v9, v11.clone(), match v10.clone() {
        Adt33::C0(_, h2) => {
            h2
        }
    }, v13.clone(), Adt35::C0);
    let v15: Adt35 = f7(v14, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f5(v13.clone())?;
            let a4 = f5(v13)?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?;
    f8(v11.clone(), v15.clone(), v1, 0u64, f9(v11.clone(), v15, f10(match v12 {
        Adt33::C0(_, h7) => {
            h7
        }
    }, Adt34::C0), match v10 {
        Adt33::C0(_, h8) => {
            h8
        }
    }, v8, v11, 0u64, Adt7::C3)?)
}

pub fn f1(v0: Adt32, p19: Adt33, p20: Adt2, p21: u64, v4: List<bool>, p22: Adt0) -> R<Adt33> {
    let m18 = (p19, (p20, (p21, p22)));
    {
        let mut m18 = m18;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m18;
                let m9 = v2;
                match m9 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                    Adt2::C14(r16, r17) => {
                        let v34 = (*r16).clone();
                        let v35 = (*r17).clone();
                        let v37: bool = {
                            let m10 = f11(v4.clone(), v3);
                            match m10 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a11 = v36;
                                    bool_not(a11)
                                }
                            }
                        };
                        let v38: Adt33 = Adt33::C0(f12(v0.clone(), match v1.clone() {
                            Adt33::C0(h12, _) => {
                                h12
                            }
                        }, v3), match v1 {
                            Adt33::C0(_, h13) => {
                                h13
                            }
                        });
                        let v39: Adt33 = if v37 {
                            f13(v0.clone(), v38, v34, f14(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>((v39, (v35, ({
                            let a14 = v3;
                            let a15 = 1u64;
                            nat_add(a14, a15)?
                        }, f15(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt33, (Adt2, (u64, Adt0))), R<Adt33>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m18 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(p27: Adt0, p28: Adt0) -> Adt0 {
    let m26 = (p27, p28);
    {
        let mut m26 = m26;
        loop {
            let step = {
                let (v0, v1) = m26;
                let m23 = v1;
                match m23 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C15(r24, r25) => {
                        let v12 = (*r24).clone();
                        let v13 = (*r25).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f16(v0, v12), v13))
                    }
                }
            };
            match step {
                Ok(next) => { m26 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt0, v1: Adt0) -> Adt0 {
    let m29 = v0;
    match m29 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(_) => {
            v1
        }
        Adt0::C9(_, _) => {
            v1
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(_, _) => {
            v1
        }
        Adt0::C12(_) => {
            v1
        }
        Adt0::C13(_, _) => {
            v1
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(r30, r31) => {
            let v12 = (*r30).clone();
            let v13 = (*r31).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f3(v13, v1)))
        }
    }
}

pub fn f4(v0: u64) -> Adt0 {
    let m32 = v0;
    if m32 == 0 {
        Adt0::C14
    } else {
        let v1 = m32 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f4(v1)))
    }
}

pub fn f5(v0: Adt0) -> R<u64> {
    let m33 = v0;
    match m33 {
        Adt0::C0 => {
            Ok(0u64)
        }
        Adt0::C1 => {
            Ok(0u64)
        }
        Adt0::C2 => {
            Ok(0u64)
        }
        Adt0::C3 => {
            Ok(0u64)
        }
        Adt0::C4(_) => {
            Ok(0u64)
        }
        Adt0::C5 => {
            Ok(0u64)
        }
        Adt0::C6 => {
            Ok(0u64)
        }
        Adt0::C7 => {
            Ok(0u64)
        }
        Adt0::C8(_) => {
            Ok(0u64)
        }
        Adt0::C9(_, _) => {
            Ok(0u64)
        }
        Adt0::C10(_) => {
            Ok(0u64)
        }
        Adt0::C11(_, _) => {
            Ok(0u64)
        }
        Adt0::C12(_) => {
            Ok(0u64)
        }
        Adt0::C13(_, _) => {
            Ok(0u64)
        }
        Adt0::C14 => {
            Ok(0u64)
        }
        Adt0::C15(_, r36) => {
            let v12 = (*r36).clone();
            let v13: u64 = f5(v12)?;
            let a34 = 1u64;
            let a35 = v13;
            nat_add(a34, a35)
        }
    }
}

pub fn f6(v0: Adt32, v1: Adt0, v2: Adt34, p41: Adt0, p42: Adt35) -> Adt35 {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v3, v4) = m40;
                let m37 = v3;
                match m37 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt35), Adt35>(v4)
                    }
                    Adt0::C15(r38, r39) => {
                        let v15 = (*r38).clone();
                        let v16 = (*r39).clone();
                        Ok::<(Adt0, Adt35), Adt35>((v16, f17(v1.clone(), v15.clone(), f18(v0.clone(), v2.clone(), v15), v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m40 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt35, v1: u64) -> R<Adt35> {
    let (h43, _) = f19(Fn0::F20, v1, v0)?;
    Ok(h43)
}

pub fn f8(v0: Adt0, v1: Adt35, v2: Adt0, v3: u64, v4: Adt7) -> R<Adt7> {
    let m44 = v2;
    match m44 {
        Adt0::C0 => {
            Ok(v4)
        }
        Adt0::C1 => {
            Ok(v4)
        }
        Adt0::C2 => {
            Ok(v4)
        }
        Adt0::C3 => {
            Ok(v4)
        }
        Adt0::C4(_) => {
            Ok(v4)
        }
        Adt0::C5 => {
            Ok(v4)
        }
        Adt0::C6 => {
            Ok(v4)
        }
        Adt0::C7 => {
            Ok(v4)
        }
        Adt0::C8(_) => {
            Ok(v4)
        }
        Adt0::C9(_, _) => {
            Ok(v4)
        }
        Adt0::C10(_) => {
            Ok(v4)
        }
        Adt0::C11(_, _) => {
            Ok(v4)
        }
        Adt0::C12(_) => {
            Ok(v4)
        }
        Adt0::C13(_, _) => {
            Ok(v4)
        }
        Adt0::C14 => {
            Ok(v4)
        }
        Adt0::C15(r48, r49) => {
            let v15 = (*r48).clone();
            let v16 = (*r49).clone();
            Ok(Adt7::C4(std::rc::Rc::new(Adt7::C0(Adt8::C4(Adt13::C0, v3), f21(v0.clone(), v1.clone(), Adt0::C12(v3), v15, 0u64)?, Adt15::C0({
                let a45 = Str::lit("type:adt");
                utf8_encode(a45)
            }, None::<Adt10>))), std::rc::Rc::new(f8(v0, v1, v16, {
                let a46 = v3;
                let a47 = 1u64;
                nat_add(a46, a47)?
            }, v4)?)))
        }
    }
}

pub fn f9(v0: Adt0, v1: Adt35, v2: Adt34, v3: Adt34, v4: List<bool>, v5: Adt0, v6: u64, v7: Adt7) -> R<Adt7> {
    let m50 = v5;
    match m50 {
        Adt0::C0 => {
            Ok(v7)
        }
        Adt0::C1 => {
            Ok(v7)
        }
        Adt0::C2 => {
            Ok(v7)
        }
        Adt0::C3 => {
            Ok(v7)
        }
        Adt0::C4(_) => {
            Ok(v7)
        }
        Adt0::C5 => {
            Ok(v7)
        }
        Adt0::C6 => {
            Ok(v7)
        }
        Adt0::C7 => {
            Ok(v7)
        }
        Adt0::C8(_) => {
            Ok(v7)
        }
        Adt0::C9(_, _) => {
            Ok(v7)
        }
        Adt0::C10(_) => {
            Ok(v7)
        }
        Adt0::C11(_, _) => {
            Ok(v7)
        }
        Adt0::C12(_) => {
            Ok(v7)
        }
        Adt0::C13(_, _) => {
            Ok(v7)
        }
        Adt0::C14 => {
            Ok(v7)
        }
        Adt0::C15(r86, r87) => {
            let v18 = (*r86).clone();
            let v19 = (*r87).clone();
            let m51 = v18;
            match m51 {
                Adt0::C0 => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a52 = v6;
                        let a53 = 1u64;
                        nat_add(a52, a53)?
                    }, v7)
                }
                Adt0::C1 => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a54 = v6;
                        let a55 = 1u64;
                        nat_add(a54, a55)?
                    }, v7)
                }
                Adt0::C2 => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a56 = v6;
                        let a57 = 1u64;
                        nat_add(a56, a57)?
                    }, v7)
                }
                Adt0::C3 => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a58 = v6;
                        let a59 = 1u64;
                        nat_add(a58, a59)?
                    }, v7)
                }
                Adt0::C4(_) => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a60 = v6;
                        let a61 = 1u64;
                        nat_add(a60, a61)?
                    }, v7)
                }
                Adt0::C5 => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a62 = v6;
                        let a63 = 1u64;
                        nat_add(a62, a63)?
                    }, v7)
                }
                Adt0::C6 => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a64 = v6;
                        let a65 = 1u64;
                        nat_add(a64, a65)?
                    }, v7)
                }
                Adt0::C7 => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a66 = v6;
                        let a67 = 1u64;
                        nat_add(a66, a67)?
                    }, v7)
                }
                Adt0::C8(_) => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a68 = v6;
                        let a69 = 1u64;
                        nat_add(a68, a69)?
                    }, v7)
                }
                Adt0::C9(_, _) => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a70 = v6;
                        let a71 = 1u64;
                        nat_add(a70, a71)?
                    }, v7)
                }
                Adt0::C10(_) => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a72 = v6;
                        let a73 = 1u64;
                        nat_add(a72, a73)?
                    }, v7)
                }
                Adt0::C11(_, _) => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a74 = v6;
                        let a75 = 1u64;
                        nat_add(a74, a75)?
                    }, v7)
                }
                Adt0::C12(_) => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a76 = v6;
                        let a77 = 1u64;
                        nat_add(a76, a77)?
                    }, v7)
                }
                Adt0::C13(r84, r85) => {
                    let v28 = (*r84).clone();
                    let v29 = (*r85).clone();
                    let v30: Adt0 = Adt0::C13(std::rc::Rc::new(v28.clone()), std::rc::Rc::new(v29.clone()));
                    Ok(Adt7::C4(std::rc::Rc::new(Adt7::C0(Adt8::C4(Adt13::C1, v6), f22(v0.clone(), v1.clone(), v30.clone(), v30.clone(), v2.clone())?, f23(f24(v30.clone(), v3.clone())))), std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(Adt7::C1(v6, f25(v0.clone(), v28)?, f25(v0.clone(), v29)?, f26(v30.clone(), v4.clone(), v2.clone()), f27(v0.clone(), v1.clone(), v30.clone(), v30.clone(), v4.clone(), v2.clone()), f23(f24(v30, v3.clone())))), std::rc::Rc::new(f9(v0, v1, v2, v3, v4, v19, {
                        let a78 = v6;
                        let a79 = 1u64;
                        nat_add(a78, a79)?
                    }, v7)?)))))
                }
                Adt0::C14 => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a80 = v6;
                        let a81 = 1u64;
                        nat_add(a80, a81)?
                    }, v7)
                }
                Adt0::C15(_, _) => {
                    f9(v0, v1, v2, v3, v4, v19, {
                        let a82 = v6;
                        let a83 = 1u64;
                        nat_add(a82, a83)?
                    }, v7)
                }
            }
        }
    }
}

pub fn f10(p91: Adt34, p92: Adt34) -> Adt34 {
    let m90 = (p91, p92);
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v0, v1) = m90;
                let m88 = v0;
                match m88 {
                    Adt34::C0 => {
                        Err::<(Adt34, Adt34), Adt34>(v1)
                    }
                    Adt34::C1(v2, v3, v4, r89) => {
                        let v5 = (*r89).clone();
                        Ok::<(Adt34, Adt34), Adt34>((v5, Adt34::C1(v2, v3, v4, std::rc::Rc::new(v1))))
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

pub fn f11(p96: List<bool>, p97: u64) -> Option<bool> {
    let m95 = (p96, p97);
    {
        let mut m95 = m95;
        loop {
            let step = {
                let (v0, v1) = m95;
                let m93 = v0;
                match m93.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m94 = v1;
                        if m94 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m94 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m95 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt32, v1: Adt0, v2: u64) -> Adt0 {
    f28(f29(v1, f30(v0.clone(), v2)), f31(v0, v2))
}

pub fn f13(v0: Adt32, v1: Adt33, v2: Adt2, v3: Adt0) -> R<Adt33> {
    let m98 = v2;
    match m98 {
        Adt2::C0(v4, _) => {
            Ok(Adt33::C0(f28(match v1.clone() {
                Adt33::C0(h99, _) => {
                    h99
                }
            }, v4), match v1 {
                Adt33::C0(_, h100) => {
                    h100
                }
            }))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, v8, r109, r110) => {
            let v9 = (*r109).clone();
            let v10 = (*r110).clone();
            let v11: Adt33 = Adt33::C0(f28(match v1.clone() {
                Adt33::C0(h101, _) => {
                    h101
                }
            }, v8), match v1 {
                Adt33::C0(_, h102) => {
                    h102
                }
            });
            let v12: Adt33 = f13(v0.clone(), v11, v9, f14(v3.clone()))?;
            f13(v0, v12, v10, f15(v3))
        }
        Adt2::C3(r111, r112, r113) => {
            let v13 = (*r111).clone();
            let v14 = (*r112).clone();
            let v15 = (*r113).clone();
            let v16: Adt33 = f13(v0.clone(), v1, v13, f14(v3.clone()))?;
            let v17: Adt33 = f13(v0.clone(), v16, v14, f15(v3.clone()))?;
            f13(v0, v17, v15, f32(v3))
        }
        Adt2::C4(v18, r114, r115) => {
            let v19 = (*r114).clone();
            let v20 = (*r115).clone();
            let v21: Adt33 = Adt33::C0(f28(match v1.clone() {
                Adt33::C0(h103, _) => {
                    h103
                }
            }, v18), match v1 {
                Adt33::C0(_, h104) => {
                    h104
                }
            });
            let v22: Adt33 = f13(v0.clone(), v21, v19, f14(v3.clone()))?;
            f13(v0, v22, v20, f15(v3))
        }
        Adt2::C5(_, v24, r116) => {
            let v25 = (*r116).clone();
            let v26: Adt33 = Adt33::C0(f28(match v1.clone() {
                Adt33::C0(h105, _) => {
                    h105
                }
            }, v24), match v1 {
                Adt33::C0(_, h106) => {
                    h106
                }
            });
            f13(v0, v26, v25, f14(v3))
        }
        Adt2::C6(_, r117) => {
            let v28 = (*r117).clone();
            f13(v0, v1, v28, f14(v3))
        }
        Adt2::C7(v29, r118) => {
            let v30 = (*r118).clone();
            let v31: Adt0 = f30(v0.clone(), v29);
            let v32: u64 = f33(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f34(v31.clone(), v32)), std::rc::Rc::new(f31(v0.clone(), v29)));
            let v34: Adt33 = Adt33::C0(f28(match v1.clone() {
                Adt33::C0(h107, _) => {
                    h107
                }
            }, v33.clone()), f35(match v1 {
                Adt33::C0(_, h108) => {
                    h108
                }
            }, v33, v29, f36(v31, v32)));
            f13(v0, v34, v30, f14(v3))
        }
        Adt2::C8(r119, r120) => {
            let v35 = (*r119).clone();
            let v36 = (*r120).clone();
            let v37: Adt33 = f13(v0.clone(), v1, v35, f14(v3.clone()))?;
            f13(v0, v37, v36, f15(v3))
        }
        Adt2::C9(_, r121) => {
            let v39 = (*r121).clone();
            f13(v0, v1, v39, f14(v3))
        }
        Adt2::C10(r122) => {
            let v40 = (*r122).clone();
            f13(v0, v1, v40, f14(v3))
        }
        Adt2::C11(r123) => {
            let v41 = (*r123).clone();
            f13(v0, v1, v41, f14(v3))
        }
        Adt2::C12(r124, _) => {
            let v42 = (*r124).clone();
            f13(v0, v1, v42, f14(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r125, r126) => {
            let v44 = (*r125).clone();
            let v45 = (*r126).clone();
            let v46: Adt33 = f13(v0.clone(), v1, v44, f14(v3.clone()))?;
            f13(v0, v46, v45, f15(v3))
        }
        Adt2::C15(_, _, r127) => {
            let v49 = (*r127).clone();
            f13(v0, v1, v49, f14(v3))
        }
    }
}

pub fn f14(v0: Adt0) -> Adt0 {
    f37(f38(v0))
}

pub fn f15(v0: Adt0) -> Adt0 {
    f37(f38(f38(v0)))
}

pub fn f16(p132: Adt0, p133: Adt0) -> Adt0 {
    let m131 = (p132, p133);
    {
        let mut m131 = m131;
        loop {
            let step = {
                let (v0, v1) = m131;
                let m128 = v1;
                match m128 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C15(r129, r130) => {
                        let v12 = (*r129).clone();
                        let v13 = (*r130).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f29(v0, v12), v13))
                    }
                }
            };
            match step {
                Ok(next) => { m131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Adt0, v1: Adt0, p138: Adt0, p139: Adt35) -> Adt35 {
    let m137 = (p138, p139);
    {
        let mut m137 = m137;
        loop {
            let step = {
                let (v2, v3) = m137;
                let m134 = v2;
                match m134 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C15(r135, r136) => {
                        let v14 = (*r135).clone();
                        let v15 = (*r136).clone();
                        Ok::<(Adt0, Adt35), Adt35>((v15, f39(v0.clone(), v1.clone(), v14, v3)))
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

pub fn f18(v0: Adt32, v1: Adt34, v2: Adt0) -> Adt0 {
    let m140 = v2;
    match m140 {
        Adt0::C0 => {
            Adt0::C14
        }
        Adt0::C1 => {
            Adt0::C14
        }
        Adt0::C2 => {
            Adt0::C14
        }
        Adt0::C3 => {
            Adt0::C14
        }
        Adt0::C4(_) => {
            Adt0::C14
        }
        Adt0::C5 => {
            Adt0::C14
        }
        Adt0::C6 => {
            Adt0::C14
        }
        Adt0::C7 => {
            Adt0::C14
        }
        Adt0::C8(_) => {
            Adt0::C14
        }
        Adt0::C9(_, _) => {
            Adt0::C14
        }
        Adt0::C10(_) => {
            Adt0::C14
        }
        Adt0::C11(_, _) => {
            Adt0::C14
        }
        Adt0::C12(v10) => {
            let m141 = f40(v0, v10);
            match m141 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r142, r143) => {
            let v12 = (*r142).clone();
            let v13 = (*r143).clone();
            f41(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f19(v0: Fn0, p148: u64, p149: Adt35) -> R<(Adt35, bool)> {
    let m147 = (p148, p149);
    {
        let mut m147 = m147;
        loop {
            let step = {
                let (v1, v2) = m147;
                let m144 = v1;
                if m144 == 0 {
                    Err::<(u64, Adt35), R<(Adt35, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m144 - 1;
                    let m146 = {
                        let c145: Fn0 = v0.clone();
                        c145.apply(v2.clone())?
                    };
                    match m146 {
                        None => {
                            Err::<(u64, Adt35), R<(Adt35, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt35), R<(Adt35, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt35) -> R<Option<Adt35>> {
    f42(v0)
}

pub fn f21(v0: Adt0, v1: Adt35, v2: Adt0, v3: Adt0, v4: u64) -> R<Adt14> {
    let m150 = v3;
    match m150 {
        Adt0::C0 => {
            Ok(Adt14::C0)
        }
        Adt0::C1 => {
            Ok(Adt14::C0)
        }
        Adt0::C2 => {
            Ok(Adt14::C0)
        }
        Adt0::C3 => {
            Ok(Adt14::C0)
        }
        Adt0::C4(_) => {
            Ok(Adt14::C0)
        }
        Adt0::C5 => {
            Ok(Adt14::C0)
        }
        Adt0::C6 => {
            Ok(Adt14::C0)
        }
        Adt0::C7 => {
            Ok(Adt14::C0)
        }
        Adt0::C8(_) => {
            Ok(Adt14::C0)
        }
        Adt0::C9(_, _) => {
            Ok(Adt14::C0)
        }
        Adt0::C10(_) => {
            Ok(Adt14::C0)
        }
        Adt0::C11(_, _) => {
            Ok(Adt14::C0)
        }
        Adt0::C12(_) => {
            Ok(Adt14::C0)
        }
        Adt0::C13(_, _) => {
            Ok(Adt14::C0)
        }
        Adt0::C14 => {
            Ok(Adt14::C0)
        }
        Adt0::C15(r153, r154) => {
            let v15 = (*r153).clone();
            let v16 = (*r154).clone();
            Ok(Adt14::C1(v4, f43(v0.clone(), v1.clone(), v2.clone(), v15)?, std::rc::Rc::new(f21(v0, v1, v2, v16, {
                let a151 = v4;
                let a152 = 1u64;
                nat_add(a151, a152)?
            })?)))
        }
    }
}

pub fn f22(v0: Adt0, v1: Adt35, v2: Adt0, v3: Adt0, v4: Adt34) -> R<Adt14> {
    let m155 = v4;
    match m155 {
        Adt34::C0 => {
            Ok(Adt14::C0)
        }
        Adt34::C1(v5, v6, v7, r156) => {
            let v8 = (*r156).clone();
            if f44(v5, v3.clone()) {
                Ok(Adt14::C1(v6, f43(v0.clone(), v1.clone(), v2.clone(), v7)?, std::rc::Rc::new(f22(v0, v1, v2, v3, v8)?)))
            } else {
                f22(v0, v1, v2, v3, v8)
            }
        }
    }
}

pub fn f23(v0: bool) -> Adt15 {
    if v0 {
        Adt15::C0({
            let a157 = Str::lit("expr:closure");
            utf8_encode(a157)
        }, None::<Adt10>)
    } else {
        Adt15::C0({
            let a158 = Str::lit("type:fn");
            utf8_encode(a158)
        }, None::<Adt10>)
    }
}

pub fn f24(v0: Adt0, p162: Adt34) -> bool {
    let m161 = p162;
    {
        let mut m161 = m161;
        loop {
            let step = {
                let v1 = m161;
                let m159 = v1;
                match m159 {
                    Adt34::C0 => {
                        Err::<Adt34, bool>(false)
                    }
                    Adt34::C1(v2, _, _, r160) => {
                        let v5 = (*r160).clone();
                        if f44(v2, v0.clone()) {
                            Err::<Adt34, bool>(true)
                        } else {
                            Ok::<Adt34, bool>(v5)
                        }
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

pub fn f25(v0: Adt0, v1: Adt0) -> R<Adt8> {
    let m163 = v1;
    match m163 {
        Adt0::C0 => {
            Ok(Adt8::C0(Adt9::C0))
        }
        Adt0::C1 => {
            Ok(Adt8::C0(Adt9::C1))
        }
        Adt0::C2 => {
            Ok(Adt8::C0(Adt9::C2))
        }
        Adt0::C3 => {
            Ok(Adt8::C0(Adt9::C3))
        }
        Adt0::C4(v2) => {
            Ok(Adt8::C1(f45(v2)))
        }
        Adt0::C5 => {
            Ok(Adt8::C0(Adt9::C5))
        }
        Adt0::C6 => {
            Ok(Adt8::C0(Adt9::C6))
        }
        Adt0::C7 => {
            Ok(Adt8::C0(Adt9::C4))
        }
        Adt0::C8(r164) => {
            let v3 = (*r164).clone();
            Ok(Adt8::C2(Adt11::C0, std::rc::Rc::new(f25(v0, v3)?)))
        }
        Adt0::C9(r165, r166) => {
            let v4 = (*r165).clone();
            let v5 = (*r166).clone();
            Ok(Adt8::C3(Adt12::C0, std::rc::Rc::new(f25(v0.clone(), v4)?), std::rc::Rc::new(f25(v0, v5)?)))
        }
        Adt0::C10(r167) => {
            let v6 = (*r167).clone();
            Ok(Adt8::C2(Adt11::C1, std::rc::Rc::new(f25(v0, v6)?)))
        }
        Adt0::C11(r168, r169) => {
            let v7 = (*r168).clone();
            let v8 = (*r169).clone();
            Ok(Adt8::C3(Adt12::C1, std::rc::Rc::new(f25(v0.clone(), v7)?), std::rc::Rc::new(f25(v0, v8)?)))
        }
        Adt0::C12(v9) => {
            Ok(Adt8::C4(Adt13::C0, v9))
        }
        Adt0::C13(r170, r171) => {
            let v10 = (*r170).clone();
            let v11 = (*r171).clone();
            Ok(Adt8::C4(Adt13::C1, f46(v0, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)), 0u64)?))
        }
        Adt0::C14 => {
            Ok(Adt8::C5)
        }
        Adt0::C15(r172, r173) => {
            let v12 = (*r172).clone();
            let v13 = (*r173).clone();
            Ok(Adt8::C6(std::rc::Rc::new(f25(v0.clone(), v12)?), std::rc::Rc::new(f25(v0, v13)?)))
        }
    }
}

pub fn f26(v0: Adt0, v1: List<bool>, p177: Adt34) -> bool {
    let m176 = p177;
    {
        let mut m176 = m176;
        loop {
            let step = {
                let v2 = m176;
                let m174 = v2;
                match m174 {
                    Adt34::C0 => {
                        Err::<Adt34, bool>(false)
                    }
                    Adt34::C1(v3, v4, _, r175) => {
                        let v6 = (*r175).clone();
                        if f44(v3, v0.clone()) {
                            if f47(v1.clone(), v4) {
                                Err::<Adt34, bool>(true)
                            } else {
                                Ok::<Adt34, bool>(v6)
                            }
                        } else {
                            Ok::<Adt34, bool>(v6)
                        }
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

pub fn f27(v0: Adt0, v1: Adt35, v2: Adt0, v3: Adt0, v4: List<bool>, v5: Adt34) -> Adt16 {
    let m178 = v5;
    match m178 {
        Adt34::C0 => {
            Adt16::C0
        }
        Adt34::C1(v6, v7, v8, r179) => {
            let v9 = (*r179).clone();
            if f44(v6, v3.clone()) {
                Adt16::C1(v7, f48(v0.clone(), v1.clone(), v2.clone(), v8), f47(v4.clone(), v7), std::rc::Rc::new(f27(v0, v1, v2, v3, v4, v9)))
            } else {
                f27(v0, v1, v2, v3, v4, v9)
            }
        }
    }
}

pub fn f28(v0: Adt0, v1: Adt0) -> Adt0 {
    let m180 = v1;
    match m180 {
        Adt0::C0 => {
            v0
        }
        Adt0::C1 => {
            v0
        }
        Adt0::C2 => {
            v0
        }
        Adt0::C3 => {
            v0
        }
        Adt0::C4(_) => {
            v0
        }
        Adt0::C5 => {
            v0
        }
        Adt0::C6 => {
            v0
        }
        Adt0::C7 => {
            v0
        }
        Adt0::C8(r181) => {
            let v3 = (*r181).clone();
            f28(v0, v3)
        }
        Adt0::C9(r182, r183) => {
            let v4 = (*r182).clone();
            let v5 = (*r183).clone();
            f28(f28(v0, v4), v5)
        }
        Adt0::C10(r184) => {
            let v6 = (*r184).clone();
            f28(v0, v6)
        }
        Adt0::C11(r185, r186) => {
            let v7 = (*r185).clone();
            let v8 = (*r186).clone();
            f28(f28(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r187, r188) => {
            let v10 = (*r187).clone();
            let v11 = (*r188).clone();
            let v12: Adt0 = f49(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f28(f28(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r189, r190) => {
            let v13 = (*r189).clone();
            let v14 = (*r190).clone();
            f28(f28(v0, v13), v14)
        }
    }
}

pub fn f29(p195: Adt0, p196: Adt0) -> Adt0 {
    let m194 = (p195, p196);
    {
        let mut m194 = m194;
        loop {
            let step = {
                let (v0, v1) = m194;
                let m191 = v1;
                match m191 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C15(r192, r193) => {
                        let v2 = (*r192).clone();
                        let v3 = (*r193).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f28(v0, v2), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Adt32, v1: u64) -> Adt0 {
    let m197 = f50(v0, v1);
    match m197 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f31(v0: Adt32, v1: u64) -> Adt0 {
    let m199 = f51(match v0 {
        Adt32::C0(_, _, _, _, h198, _) => {
            h198
        }
    }, v1);
    match m199 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f32(v0: Adt0) -> Adt0 {
    f37(f38(f38(f38(v0))))
}

pub fn f33(v0: Adt2) -> R<u64> {
    let m200 = v0;
    match m200 {
        Adt2::C0(_, _) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C6(_, _) => {
            Ok(0u64)
        }
        Adt2::C7(_, _) => {
            Ok(0u64)
        }
        Adt2::C8(_, _) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_) => {
            Ok(0u64)
        }
        Adt2::C11(_) => {
            Ok(0u64)
        }
        Adt2::C12(_, _) => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14(_, r203) => {
            let v33 = (*r203).clone();
            let v34: u64 = f33(v33)?;
            let a201 = 1u64;
            let a202 = v34;
            nat_add(a201, a202)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f34(p209: Adt0, p210: u64) -> Adt0 {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v0, v1) = m208;
                let m204 = v0;
                match m204 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C15(r206, r207) => {
                        let v12 = (*r206).clone();
                        let v13 = (*r207).clone();
                        let m205 = v1;
                        if m205 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m205 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
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

pub fn f35(v0: Adt34, v1: Adt0, v2: u64, v3: Adt0) -> Adt34 {
    if f52(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt34::C1(v1, v2, v3, std::rc::Rc::new(v0))
    }
}

pub fn f36(v0: Adt0, v1: u64) -> Adt0 {
    let m211 = v0;
    match m211 {
        Adt0::C0 => {
            Adt0::C14
        }
        Adt0::C1 => {
            Adt0::C14
        }
        Adt0::C2 => {
            Adt0::C14
        }
        Adt0::C3 => {
            Adt0::C14
        }
        Adt0::C4(_) => {
            Adt0::C14
        }
        Adt0::C5 => {
            Adt0::C14
        }
        Adt0::C6 => {
            Adt0::C14
        }
        Adt0::C7 => {
            Adt0::C14
        }
        Adt0::C8(_) => {
            Adt0::C14
        }
        Adt0::C9(_, _) => {
            Adt0::C14
        }
        Adt0::C10(_) => {
            Adt0::C14
        }
        Adt0::C11(_, _) => {
            Adt0::C14
        }
        Adt0::C12(_) => {
            Adt0::C14
        }
        Adt0::C13(_, _) => {
            Adt0::C14
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(r213, r214) => {
            let v12 = (*r213).clone();
            let v13 = (*r214).clone();
            let m212 = v1;
            if m212 == 0 {
                Adt0::C14
            } else {
                let v14 = m212 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f36(v13, v14)))
            }
        }
    }
}

pub fn f37(v0: Adt0) -> Adt0 {
    let m215 = f53(v0);
    match m215 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f38(v0: Adt0) -> Adt0 {
    let m216 = f54(v0);
    match m216 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f39(v0: Adt0, v1: Adt0, p221: Adt0, p222: Adt35) -> Adt35 {
    let m220 = (p221, p222);
    {
        let mut m220 = m220;
        loop {
            let step = {
                let (v2, v3) = m220;
                let m217 = v2;
                match m217 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt35), Adt35>(v3)
                    }
                    Adt0::C15(r218, r219) => {
                        let v14 = (*r218).clone();
                        let v15 = (*r219).clone();
                        Ok::<(Adt0, Adt35), Adt35>((v15, f55(v1.clone(), f56(v0.clone(), Adt0::C14, v14), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Adt32, v1: u64) -> Option<Adt0> {
    f51(match v0 {
        Adt32::C0(_, h223, _, _, _, _) => {
            h223
        }
    }, v1)
}

pub fn f41(v0: Adt34, v1: Adt0) -> Adt0 {
    let m224 = v0;
    match m224 {
        Adt34::C0 => {
            Adt0::C14
        }
        Adt34::C1(v2, _, v4, r225) => {
            let v5 = (*r225).clone();
            if f44(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f41(v5, v1)))
            } else {
                f41(v5, v1)
            }
        }
    }
}

pub fn f42(v0: Adt35) -> R<Option<Adt35>> {
    let v1: Adt35 = f57(v0.clone(), v0.clone(), v0.clone());
    let a226 = f58(v1.clone())?;
    let a227 = f58(v0)?;
    if nat_eq(a226, a227) {
        Ok(None::<Adt35>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f43(v0: Adt0, v1: Adt35, v2: Adt0, v3: Adt0) -> R<Adt8> {
    let m228 = v3;
    match m228 {
        Adt0::C0 => {
            Ok(Adt8::C5)
        }
        Adt0::C1 => {
            Ok(Adt8::C5)
        }
        Adt0::C2 => {
            Ok(Adt8::C5)
        }
        Adt0::C3 => {
            Ok(Adt8::C5)
        }
        Adt0::C4(_) => {
            Ok(Adt8::C5)
        }
        Adt0::C5 => {
            Ok(Adt8::C5)
        }
        Adt0::C6 => {
            Ok(Adt8::C5)
        }
        Adt0::C7 => {
            Ok(Adt8::C5)
        }
        Adt0::C8(_) => {
            Ok(Adt8::C5)
        }
        Adt0::C9(_, _) => {
            Ok(Adt8::C5)
        }
        Adt0::C10(_) => {
            Ok(Adt8::C5)
        }
        Adt0::C11(_, _) => {
            Ok(Adt8::C5)
        }
        Adt0::C12(_) => {
            Ok(Adt8::C5)
        }
        Adt0::C13(_, _) => {
            Ok(Adt8::C5)
        }
        Adt0::C14 => {
            Ok(Adt8::C5)
        }
        Adt0::C15(r229, r230) => {
            let v14 = (*r229).clone();
            let v15 = (*r230).clone();
            Ok(Adt8::C6(std::rc::Rc::new(f59(v0.clone(), v1.clone(), v2.clone(), v14)?), std::rc::Rc::new(f43(v0, v1, v2, v15)?)))
        }
    }
}

pub fn f44(v0: Adt0, v1: Adt0) -> bool {
    let m231 = v0;
    match m231 {
        Adt0::C0 => {
            let a232 = f60(v1);
            let a233 = 0u64;
            nat_eq(a232, a233)
        }
        Adt0::C1 => {
            let a234 = f60(v1);
            let a235 = 1u64;
            nat_eq(a234, a235)
        }
        Adt0::C2 => {
            let a236 = f60(v1);
            let a237 = 2u64;
            nat_eq(a236, a237)
        }
        Adt0::C3 => {
            let a238 = f60(v1);
            let a239 = 3u64;
            nat_eq(a238, a239)
        }
        Adt0::C4(v2) => {
            let a248 = f60(v1.clone());
            let a249 = 4u64;
            if nat_eq(a248, a249) {
                let a250 = f61(v2);
                let a251 = f61(f62(v1));
                nat_eq(a250, a251)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a240 = f60(v1);
            let a241 = 5u64;
            nat_eq(a240, a241)
        }
        Adt0::C6 => {
            let a242 = f60(v1);
            let a243 = 6u64;
            nat_eq(a242, a243)
        }
        Adt0::C7 => {
            let a244 = f60(v1);
            let a245 = 7u64;
            nat_eq(a244, a245)
        }
        Adt0::C8(r268) => {
            let v3 = (*r268).clone();
            let v4: bool = f44(v3, f37(v1.clone()));
            let a252 = f60(v1);
            let a253 = 8u64;
            if nat_eq(a252, a253) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r269, r270) => {
            let v5 = (*r269).clone();
            let v6 = (*r270).clone();
            let v7: bool = f44(v5, f37(v1.clone()));
            let v8: bool = f44(v6, f38(v1.clone()));
            let a254 = f60(v1);
            let a255 = 9u64;
            if nat_eq(a254, a255) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r271) => {
            let v9 = (*r271).clone();
            let v10: bool = f44(v9, f37(v1.clone()));
            let a256 = f60(v1);
            let a257 = 10u64;
            if nat_eq(a256, a257) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r272, r273) => {
            let v11 = (*r272).clone();
            let v12 = (*r273).clone();
            let v13: bool = f44(v11, f37(v1.clone()));
            let v14: bool = f44(v12, f38(v1.clone()));
            let a258 = f60(v1);
            let a259 = 11u64;
            if nat_eq(a258, a259) {
                if v13 {
                    v14
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C12(v15) => {
            let a260 = f60(v1.clone());
            let a261 = 12u64;
            if nat_eq(a260, a261) {
                let a262 = v15;
                let a263 = f63(v1);
                nat_eq(a262, a263)
            } else {
                false
            }
        }
        Adt0::C13(r274, r275) => {
            let v16 = (*r274).clone();
            let v17 = (*r275).clone();
            let v18: bool = f44(v16, f37(v1.clone()));
            let v19: bool = f44(v17, f38(v1.clone()));
            let a264 = f60(v1);
            let a265 = 13u64;
            if nat_eq(a264, a265) {
                if v18 {
                    v19
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C14 => {
            let a246 = f60(v1);
            let a247 = 14u64;
            nat_eq(a246, a247)
        }
        Adt0::C15(r276, r277) => {
            let v20 = (*r276).clone();
            let v21 = (*r277).clone();
            let v22: bool = f44(v20, f37(v1.clone()));
            let v23: bool = f44(v21, f38(v1.clone()));
            let a266 = f60(v1);
            let a267 = 15u64;
            if nat_eq(a266, a267) {
                if v22 {
                    v23
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f45(v0: Adt1) -> Adt10 {
    let m278 = v0;
    match m278 {
        Adt1::C0 => {
            Adt10::C0
        }
        Adt1::C1 => {
            Adt10::C1
        }
        Adt1::C2 => {
            Adt10::C2
        }
        Adt1::C3 => {
            Adt10::C3
        }
        Adt1::C4 => {
            Adt10::C4
        }
        Adt1::C5 => {
            Adt10::C5
        }
        Adt1::C6 => {
            Adt10::C6
        }
        Adt1::C7 => {
            Adt10::C7
        }
    }
}

pub fn f46(p285: Adt0, v1: Adt0, p286: u64) -> R<u64> {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v0, v2) = m284;
                let m279 = v0;
                match m279 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C15(r282, r283) => {
                        let v13 = (*r282).clone();
                        let v14 = (*r283).clone();
                        if f44(v13, v1.clone()) {
                            Err::<(Adt0, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(Adt0, u64), R<u64>>((v14, {
                                let a280 = v2;
                                let a281 = 1u64;
                                nat_add(a280, a281)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: List<bool>, v1: u64) -> bool {
    let m287 = f11(v0, v1);
    match m287 {
        None => {
            false
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f48(v0: Adt0, v1: Adt35, v2: Adt0, v3: Adt0) -> List<Adt17> {
    let m288 = v3;
    match m288 {
        Adt0::C0 => {
            List::<Adt17>::nil()
        }
        Adt0::C1 => {
            List::<Adt17>::nil()
        }
        Adt0::C2 => {
            List::<Adt17>::nil()
        }
        Adt0::C3 => {
            List::<Adt17>::nil()
        }
        Adt0::C4(_) => {
            List::<Adt17>::nil()
        }
        Adt0::C5 => {
            List::<Adt17>::nil()
        }
        Adt0::C6 => {
            List::<Adt17>::nil()
        }
        Adt0::C7 => {
            List::<Adt17>::nil()
        }
        Adt0::C8(_) => {
            List::<Adt17>::nil()
        }
        Adt0::C9(_, _) => {
            List::<Adt17>::nil()
        }
        Adt0::C10(_) => {
            List::<Adt17>::nil()
        }
        Adt0::C11(_, _) => {
            List::<Adt17>::nil()
        }
        Adt0::C12(_) => {
            List::<Adt17>::nil()
        }
        Adt0::C13(_, _) => {
            List::<Adt17>::nil()
        }
        Adt0::C14 => {
            List::<Adt17>::nil()
        }
        Adt0::C15(r289, r290) => {
            let v14 = (*r289).clone();
            let v15 = (*r290).clone();
            List::cons(f64(v0.clone(), v1.clone(), v2.clone(), v14), f48(v0, v1, v2, v15))
        }
    }
}

pub fn f49(v0: Adt0, v1: Adt0) -> Adt0 {
    if f65(v0.clone(), v1.clone()) {
        v0
    } else {
        f3(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f50(v0: Adt32, v1: u64) -> Option<Adt0> {
    f51(match v0 {
        Adt32::C0(_, _, _, h291, _, _) => {
            h291
        }
    }, v1)
}

pub fn f51(p297: Adt0, p298: u64) -> Option<Adt0> {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v0, v1) = m296;
                let m292 = v0;
                match m292 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C15(r294, r295) => {
                        let v12 = (*r294).clone();
                        let v13 = (*r295).clone();
                        let m293 = v1;
                        if m293 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m293 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
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

pub fn f52(p304: Adt34, v1: Adt0, v2: u64) -> bool {
    let m303 = p304;
    {
        let mut m303 = m303;
        loop {
            let step = {
                let v0 = m303;
                let m299 = v0;
                match m299 {
                    Adt34::C0 => {
                        Err::<Adt34, bool>(false)
                    }
                    Adt34::C1(v3, v4, _, r302) => {
                        let v6 = (*r302).clone();
                        let a300 = v4;
                        let a301 = v2;
                        if nat_eq(a300, a301) {
                            if f44(v3, v1.clone()) {
                                Err::<Adt34, bool>(true)
                            } else {
                                Ok::<Adt34, bool>(v6)
                            }
                        } else {
                            Ok::<Adt34, bool>(v6)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: Adt0) -> Option<Adt0> {
    let m305 = v0;
    match m305 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(r306) => {
            let v2 = (*r306).clone();
            Some(v2)
        }
        Adt0::C9(r307, _) => {
            let v3 = (*r307).clone();
            Some(v3)
        }
        Adt0::C10(r308) => {
            let v5 = (*r308).clone();
            Some(v5)
        }
        Adt0::C11(r309, _) => {
            let v6 = (*r309).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r310, _) => {
            let v9 = (*r310).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r311, _) => {
            let v11 = (*r311).clone();
            Some(v11)
        }
    }
}

pub fn f54(v0: Adt0) -> Option<Adt0> {
    let m312 = v0;
    match m312 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(_) => {
            None::<Adt0>
        }
        Adt0::C9(_, r313) => {
            let v4 = (*r313).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r314) => {
            let v7 = (*r314).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r315) => {
            let v10 = (*r315).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r316) => {
            let v12 = (*r316).clone();
            Some(v12)
        }
    }
}

pub fn f55(v0: Adt0, p321: Adt0, p322: Adt35) -> Adt35 {
    let m320 = (p321, p322);
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v1, v2) = m320;
                let m317 = v1;
                match m317 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt35), Adt35>(v2)
                    }
                    Adt0::C15(r318, r319) => {
                        let v13 = (*r318).clone();
                        let v14 = (*r319).clone();
                        Ok::<(Adt0, Adt35), Adt35>((v14, f66(v2, v0.clone(), v13)))
                    }
                }
            };
            match step {
                Ok(next) => { m320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m323 = v2;
    match m323 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(r324) => {
            let v4 = (*r324).clone();
            f56(v0, v1, v4)
        }
        Adt0::C9(r325, r326) => {
            let v5 = (*r325).clone();
            let v6 = (*r326).clone();
            f56(v0.clone(), f56(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r327, r328) => {
            let v8 = (*r327).clone();
            let v9 = (*r328).clone();
            f56(v0.clone(), f56(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f49(v1, Adt0::C12(v10))
        }
        Adt0::C13(r329, r330) => {
            let v11 = (*r329).clone();
            let v12 = (*r330).clone();
            if f65(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f49(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
            } else {
                v1
            }
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(_, _) => {
            v1
        }
    }
}

pub fn f57(v0: Adt35, p334: Adt35, p335: Adt35) -> Adt35 {
    let m333 = (p334, p335);
    {
        let mut m333 = m333;
        loop {
            let step = {
                let (v1, v2) = m333;
                let m331 = v1;
                match m331 {
                    Adt35::C0 => {
                        Err::<(Adt35, Adt35), Adt35>(v2)
                    }
                    Adt35::C1(v3, v4, r332) => {
                        let v5 = (*r332).clone();
                        Ok::<(Adt35, Adt35), Adt35>((v5, f67(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Adt35) -> R<u64> {
    let m336 = v0;
    match m336 {
        Adt35::C0 => {
            Ok(0u64)
        }
        Adt35::C1(_, _, r339) => {
            let v3 = (*r339).clone();
            let a337 = 1u64;
            let a338 = f58(v3)?;
            nat_add(a337, a338)
        }
    }
}

pub fn f59(v0: Adt0, v1: Adt35, v2: Adt0, v3: Adt0) -> R<Adt8> {
    if f68(v0.clone(), v1, v2, v3.clone()) {
        Ok(Adt8::C2(Adt11::C2, std::rc::Rc::new(f25(v0, v3)?)))
    } else {
        f25(v0, v3)
    }
}

pub fn f60(v0: Adt0) -> u64 {
    let m340 = v0;
    match m340 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            1u64
        }
        Adt0::C2 => {
            2u64
        }
        Adt0::C3 => {
            3u64
        }
        Adt0::C4(_) => {
            4u64
        }
        Adt0::C5 => {
            5u64
        }
        Adt0::C6 => {
            6u64
        }
        Adt0::C7 => {
            7u64
        }
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9(_, _) => {
            9u64
        }
        Adt0::C10(_) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12(_) => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14 => {
            14u64
        }
        Adt0::C15(_, _) => {
            15u64
        }
    }
}

pub fn f61(v0: Adt1) -> u64 {
    let m341 = v0;
    match m341 {
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
        Adt1::C7 => {
            7u64
        }
    }
}

pub fn f62(v0: Adt0) -> Adt1 {
    let m342 = v0;
    match m342 {
        Adt0::C0 => {
            Adt1::C0
        }
        Adt0::C1 => {
            Adt1::C0
        }
        Adt0::C2 => {
            Adt1::C0
        }
        Adt0::C3 => {
            Adt1::C0
        }
        Adt0::C4(v1) => {
            v1
        }
        Adt0::C5 => {
            Adt1::C0
        }
        Adt0::C6 => {
            Adt1::C0
        }
        Adt0::C7 => {
            Adt1::C0
        }
        Adt0::C8(_) => {
            Adt1::C0
        }
        Adt0::C9(_, _) => {
            Adt1::C0
        }
        Adt0::C10(_) => {
            Adt1::C0
        }
        Adt0::C11(_, _) => {
            Adt1::C0
        }
        Adt0::C12(_) => {
            Adt1::C0
        }
        Adt0::C13(_, _) => {
            Adt1::C0
        }
        Adt0::C14 => {
            Adt1::C0
        }
        Adt0::C15(_, _) => {
            Adt1::C0
        }
    }
}

pub fn f63(v0: Adt0) -> u64 {
    let m343 = v0;
    match m343 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            0u64
        }
        Adt0::C2 => {
            0u64
        }
        Adt0::C3 => {
            0u64
        }
        Adt0::C4(_) => {
            0u64
        }
        Adt0::C5 => {
            0u64
        }
        Adt0::C6 => {
            0u64
        }
        Adt0::C7 => {
            0u64
        }
        Adt0::C8(_) => {
            0u64
        }
        Adt0::C9(_, _) => {
            0u64
        }
        Adt0::C10(_) => {
            0u64
        }
        Adt0::C11(_, _) => {
            0u64
        }
        Adt0::C12(v8) => {
            v8
        }
        Adt0::C13(_, _) => {
            0u64
        }
        Adt0::C14 => {
            0u64
        }
        Adt0::C15(_, _) => {
            0u64
        }
    }
}

pub fn f64(v0: Adt0, v1: Adt35, v2: Adt0, v3: Adt0) -> Adt17 {
    if f44(v3.clone(), Adt0::C0) {
        Adt17::C3
    } else {
        if f68(v0, v1, v2, v3.clone()) {
            Adt17::C2
        } else {
            if f69(v3) {
                Adt17::C0
            } else {
                Adt17::C1
            }
        }
    }
}

pub fn f65(p348: Adt0, v1: Adt0) -> bool {
    let m347 = p348;
    {
        let mut m347 = m347;
        loop {
            let step = {
                let v0 = m347;
                let m344 = v0;
                match m344 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r345, r346) => {
                        let v12 = (*r345).clone();
                        let v13 = (*r346).clone();
                        if f44(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Adt35, v1: Adt0, v2: Adt0) -> Adt35 {
    if f70(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt35::C1(v1, v2, std::rc::Rc::new(v0))
    }
}

pub fn f67(p352: Adt35, v1: Adt0, v2: Adt0, p353: Adt35) -> Adt35 {
    let m351 = (p352, p353);
    {
        let mut m351 = m351;
        loop {
            let step = {
                let (v0, v3) = m351;
                let m349 = v0;
                match m349 {
                    Adt35::C0 => {
                        Err::<(Adt35, Adt35), Adt35>(v3)
                    }
                    Adt35::C1(v4, v5, r350) => {
                        let v6 = (*r350).clone();
                        Ok::<(Adt35, Adt35), Adt35>((v6, if f44(v4, v1.clone()) {
                            f66(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
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

pub fn f68(v0: Adt0, v1: Adt35, v2: Adt0, v3: Adt0) -> bool {
    f71(v2, v1, f56(v0, Adt0::C14, v3))
}

pub fn f69(v0: Adt0) -> bool {
    let m354 = v0;
    match m354 {
        Adt0::C0 => {
            true
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            false
        }
        Adt0::C6 => {
            false
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(r355) => {
            let v2 = (*r355).clone();
            f69(v2)
        }
        Adt0::C9(r356, r357) => {
            let v3 = (*r356).clone();
            let v4 = (*r357).clone();
            if f69(v3) {
                f69(v4)
            } else {
                false
            }
        }
        Adt0::C10(_) => {
            false
        }
        Adt0::C11(r358, r359) => {
            let v6 = (*r358).clone();
            let v7 = (*r359).clone();
            if f69(v6) {
                f69(v7)
            } else {
                false
            }
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14 => {
            false
        }
        Adt0::C15(_, _) => {
            false
        }
    }
}

pub fn f70(p363: Adt35, v1: Adt0, v2: Adt0) -> bool {
    let m362 = p363;
    {
        let mut m362 = m362;
        loop {
            let step = {
                let v0 = m362;
                let m360 = v0;
                match m360 {
                    Adt35::C0 => {
                        Err::<Adt35, bool>(false)
                    }
                    Adt35::C1(v3, v4, r361) => {
                        let v5 = (*r361).clone();
                        if f44(v3, v1.clone()) {
                            if f44(v4, v2.clone()) {
                                Err::<Adt35, bool>(true)
                            } else {
                                Ok::<Adt35, bool>(v5)
                            }
                        } else {
                            Ok::<Adt35, bool>(v5)
                        }
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

pub fn f71(v0: Adt0, v1: Adt35, p368: Adt0) -> bool {
    let m367 = p368;
    {
        let mut m367 = m367;
        loop {
            let step = {
                let v2 = m367;
                let m364 = v2;
                match m364 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r365, r366) => {
                        let v13 = (*r365).clone();
                        let v14 = (*r366).clone();
                        if f44(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f70(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m367 = next; }
                Err(done) => break done,
            }
        }
    }
}
