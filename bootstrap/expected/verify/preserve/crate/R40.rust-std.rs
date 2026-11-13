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
    C0(Adt1, Adt7, Adt8),
    C1(u64, Adt1, Adt1, bool, Adt9, Adt8),
    C2(Adt11, Adt12, Adt1, Adt17, Adt8),
    C3,
    C4(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2),
    C1(Adt3),
    C2(Adt4, std::rc::Rc<Adt1>),
    C3(Adt5, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C4(Adt6, u64),
    C5,
    C6(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
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
pub enum Adt3 {
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
pub enum Adt4 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1(u64, Adt1, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, Option<Adt3>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(u64, List<Adt10>, bool, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt11 {
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
pub enum Adt12 {
    C0,
    C1(Adt13, Adt1, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Adt14),
    C1(Adt11),
    C2(std::rc::Rc<Adt13>),
    C3(Adt15, std::rc::Rc<Adt13>),
    C4(u64, u64, std::rc::Rc<Adt13>),
    C5,
    C6(std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C7(Adt16),
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18, Adt8),
    C1(Adt19, Adt11, Adt8),
    C2(Adt20, std::rc::Rc<Adt17>, Adt8),
    C3(Adt21, std::rc::Rc<Adt17>, bool, Adt8),
    C4(Adt11, std::rc::Rc<Adt17>, bool, Adt8),
    C5(Adt23, std::rc::Rc<Adt17>, Adt8),
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt8),
    C7(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt8),
    C8(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt8),
    C9(std::rc::Rc<Adt17>),
    C10(Adt24, Adt11, Adt8),
    C11(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C12,
    C13(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C14(Adt13, bool, Adt1, std::rc::Rc<Adt17>, Adt8, std::rc::Rc<Adt17>),
    C15(Adt13, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C16(Adt11, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt3, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt16),
    C8,
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64),
    C1(Adt22),
    C2(u64, Option<Adt3>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Option<Adt3>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt1),
    C1,
    C2(Adt1, Adt1),
    C3(Adt1, Adt1),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt1),
}

#[derive(Clone)]
pub enum Adt24 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u64, List<(u64, u64)>),
}

pub fn f0(v0: Adt0) -> R<Adt0> {
    f1(v0)
}

pub fn f1(v0: Adt0) -> R<Adt0> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3) => {
            Ok(Adt0::C0(v1, v2, v3))
        }
        Adt0::C1(v4, v5, v6, v7, v8, v9) => {
            Ok(Adt0::C1(v4, v5, v6, v7, v8, v9))
        }
        Adt0::C2(v10, v11, v12, v13, v14) => {
            Ok(Adt0::C2(v10, v11, v12, f2(v13, List::<(Adt11, Adt25)>::nil())?, v14))
        }
        Adt0::C3 => {
            Ok(Adt0::C3)
        }
        Adt0::C4(r2, r3) => {
            let v15 = (*r2).clone();
            let v16 = (*r3).clone();
            Ok(Adt0::C4(std::rc::Rc::new(f1(v15)?), std::rc::Rc::new(f1(v16)?)))
        }
    }
}

pub fn f2(v0: Adt17, v1: List<(Adt11, Adt25)>) -> R<Adt17> {
    let m4 = v0;
    match m4 {
        Adt17::C0(v2, v3) => {
            Ok(Adt17::C0(v2, v3))
        }
        Adt17::C1(v4, v5, v6) => {
            if f3(v4.clone()) {
                if f4(v1, v5.clone()) {
                    Ok(Adt17::C1(Adt19::C0, v5, v6))
                } else {
                    Ok(Adt17::C1(v4, v5, v6))
                }
            } else {
                Ok(Adt17::C1(v4, v5, v6))
            }
        }
        Adt17::C2(v7, r13, v9) => {
            let v8 = (*r13).clone();
            Ok(Adt17::C2(v7, std::rc::Rc::new(f2(v8, v1)?), v9))
        }
        Adt17::C3(v10, r14, v12, v13) => {
            let v11 = (*r14).clone();
            Ok(Adt17::C3(v10, std::rc::Rc::new(f2(v11, v1)?), v12, v13))
        }
        Adt17::C4(v14, r15, v16, v17) => {
            let v15 = (*r15).clone();
            let v18: List<(Adt11, Adt25)> = f5(v1, v14.clone())?;
            Ok(Adt17::C4(v14, std::rc::Rc::new(f2(v15, v18)?), v16, v17))
        }
        Adt17::C5(v19, r16, v21) => {
            let v20 = (*r16).clone();
            Ok(Adt17::C5(v19, std::rc::Rc::new(f2(v20, v1)?), v21))
        }
        Adt17::C6(r17, r18, v24) => {
            let v22 = (*r17).clone();
            let v23 = (*r18).clone();
            let v25: List<(Adt11, Adt25)> = f6(v23.clone(), v1.clone(), 0u64)?;
            Ok(Adt17::C6(std::rc::Rc::new(f2(v22, v25)?), std::rc::Rc::new(f2(v23, v1)?), v24))
        }
        Adt17::C7(r19, r20, r21, v29) => {
            let v26 = (*r19).clone();
            let v27 = (*r20).clone();
            let v28 = (*r21).clone();
            let v30: List<(Adt11, Adt25)> = f6(v27.clone(), v1.clone(), 0u64)?;
            let v31: List<(Adt11, Adt25)> = f6(v28.clone(), v1.clone(), 0u64)?;
            Ok(Adt17::C7(std::rc::Rc::new(f2(v26, f7(v30, v31))?), std::rc::Rc::new(f2(v27, v1.clone())?), std::rc::Rc::new(f2(v28, v1)?), v29))
        }
        Adt17::C8(r22, r23, v34) => {
            let v32 = (*r22).clone();
            let v33 = (*r23).clone();
            let v35: List<(Adt11, Adt25)> = if f8(v33.clone()) {
                v1.clone()
            } else {
                f6(v33.clone(), v1.clone(), 1u64)?
            };
            Ok(Adt17::C8(std::rc::Rc::new(f2(v32, v35)?), std::rc::Rc::new(f2(v33, v1)?), v34))
        }
        Adt17::C9(r24) => {
            let v36 = (*r24).clone();
            Ok(Adt17::C9(std::rc::Rc::new(f2(v36, v1)?)))
        }
        Adt17::C10(v37, v38, v39) => {
            Ok(Adt17::C10(v37, v38, v39))
        }
        Adt17::C11(r25, r26) => {
            let v40 = (*r25).clone();
            let v41 = (*r26).clone();
            let v42: List<(Adt11, Adt25)> = f6(v41.clone(), v1.clone(), 0u64)?;
            Ok(Adt17::C11(std::rc::Rc::new(f2(v40, v42)?), std::rc::Rc::new(f2(v41, v1)?)))
        }
        Adt17::C12 => {
            Ok(Adt17::C12)
        }
        Adt17::C13(r27, r28) => {
            let v43 = (*r27).clone();
            let v44 = (*r28).clone();
            let v45: List<(Adt11, Adt25)> = f6(v44.clone(), v1.clone(), 0u64)?;
            Ok(Adt17::C13(std::rc::Rc::new(f2(v43, v45)?), std::rc::Rc::new(f2(v44, v1)?)))
        }
        Adt17::C14(v46, v47, v48, r29, v50, r30) => {
            let v49 = (*r29).clone();
            let v51 = (*r30).clone();
            let v52: List<(Adt11, Adt25)> = f6(v51.clone(), v1.clone(), 0u64)?;
            let v53: List<(Adt11, Adt25)> = f9(v46.clone(), v52);
            Ok(Adt17::C14(v46.clone(), v47, v48, std::rc::Rc::new({
                let m5 = f10(v46, v49.clone())?;
                match m5 {
                    None => {
                        f2(v49, v53)?
                    }
                    Some(v54) => {
                        if f11(v53, {
                            let (h6, _) = {
                                let (_, h7) = v54.clone();
                                h7
                            };
                            h6
                        }, {
                            let (h8, _) = v54.clone();
                            h8
                        }) {
                            Adt17::C1(Adt19::C0, {
                                let (h9, _) = {
                                    let (_, h10) = v54.clone();
                                    h10
                                };
                                h9
                            }, {
                                let (_, h11) = {
                                    let (_, h12) = v54;
                                    h12
                                };
                                h11
                            })
                        } else {
                            v49
                        }
                    }
                }
            }), v50, std::rc::Rc::new(f2(v51, v1)?)))
        }
        Adt17::C15(v55, r31, r32) => {
            let v56 = (*r31).clone();
            let v57 = (*r32).clone();
            Ok(Adt17::C15(v55, std::rc::Rc::new(f2(v56, v1.clone())?), std::rc::Rc::new(f2(v57, v1)?)))
        }
        Adt17::C16(v58, r33) => {
            let v59 = (*r33).clone();
            let v60: List<(Adt11, Adt25)> = f6(v59.clone(), List::<(Adt11, Adt25)>::nil(), 0u64)?;
            let v61: List<(Adt11, Adt25)> = f12(v60, v58.clone());
            let v62: List<(Adt11, Adt25)> = f12(f7(v1, v61), v58.clone());
            Ok(Adt17::C16(v58, std::rc::Rc::new(f2(v59, v62)?)))
        }
    }
}

pub fn f3(v0: Adt19) -> bool {
    let m34 = v0;
    match m34 {
        Adt19::C0 => {
            false
        }
        Adt19::C1 => {
            true
        }
        Adt19::C2 => {
            false
        }
        Adt19::C3 => {
            false
        }
        Adt19::C4 => {
            false
        }
    }
}

pub fn f4(v0: List<(Adt11, Adt25)>, v1: Adt11) -> bool {
    let m35 = f13(v0, v1);
    match m35 {
        None => {
            true
        }
        Some(v2) => {
            let a37 = match v2.clone() {
                Adt25::C0(h36, _) => {
                    h36
                }
            };
            let a38 = 0u64;
            if nat_eq(a37, a38) {
                let a41 = {
                    let a40 = match v2 {
                        Adt25::C0(_, h39) => {
                            h39
                        }
                    };
                    length_list(a40)
                };
                let a42 = 0u64;
                nat_eq(a41, a42)
            } else {
                false
            }
        }
    }
}

pub fn f5(v0: List<(Adt11, Adt25)>, v1: Adt11) -> R<List<(Adt11, Adt25)>> {
    let v2: Adt25 = f14(v0.clone(), v1.clone());
    Ok(f15(v0, v1, Adt25::C0({
        let a44 = match v2.clone() {
            Adt25::C0(h43, _) => {
                h43
            }
        };
        let a45 = 1u64;
        nat_add(a44, a45)?
    }, match v2 {
        Adt25::C0(_, h46) => {
            h46
        }
    })))
}

pub fn f6(v0: Adt17, v1: List<(Adt11, Adt25)>, v2: u64) -> R<List<(Adt11, Adt25)>> {
    let m47 = v0;
    match m47 {
        Adt17::C0(_, _) => {
            Ok(v1)
        }
        Adt17::C1(_, v6, _) => {
            f5(v1, v6)
        }
        Adt17::C2(_, r54, _) => {
            let v9 = (*r54).clone();
            f6(v9, v1, 0u64)
        }
        Adt17::C3(_, r55, _, _) => {
            let v12 = (*r55).clone();
            f6(v12, v1, 0u64)
        }
        Adt17::C4(v15, r56, _, _) => {
            let v16 = (*r56).clone();
            let v19: List<(Adt11, Adt25)> = f5(v1, v15)?;
            f6(v16, v19, 0u64)
        }
        Adt17::C5(_, r57, _) => {
            let v21 = (*r57).clone();
            f6(v21, v1, 0u64)
        }
        Adt17::C6(r58, r59, _) => {
            let v23 = (*r58).clone();
            let v24 = (*r59).clone();
            let v26: List<(Adt11, Adt25)> = f6(v24, v1, 0u64)?;
            f6(v23, v26, 0u64)
        }
        Adt17::C7(r60, r61, r62, _) => {
            let v27 = (*r60).clone();
            let v28 = (*r61).clone();
            let v29 = (*r62).clone();
            let v31: List<(Adt11, Adt25)> = f6(v28, v1.clone(), 0u64)?;
            let v32: List<(Adt11, Adt25)> = f6(v29, v1, 0u64)?;
            f6(v27, f7(v31, v32), 0u64)
        }
        Adt17::C8(r63, r64, _) => {
            let v33 = (*r63).clone();
            let v34 = (*r64).clone();
            let v36: List<(Adt11, Adt25)> = if f8(v34.clone()) {
                v1
            } else {
                f6(v34, v1, 1u64)?
            };
            f6(v33, v36, 0u64)
        }
        Adt17::C9(r65) => {
            let v37 = (*r65).clone();
            f6(v37, v1, 0u64)
        }
        Adt17::C10(_, v39, _) => {
            f5(v1, v39)
        }
        Adt17::C11(r66, r67) => {
            let v41 = (*r66).clone();
            let v42 = (*r67).clone();
            let v43: List<(Adt11, Adt25)> = f6(v42, v1, 0u64)?;
            f6(v41, v43, 0u64)
        }
        Adt17::C12 => {
            let a48 = v2;
            let a49 = 1u64;
            if nat_eq(a48, a49) {
                Ok(List::<(Adt11, Adt25)>::nil())
            } else {
                Ok(v1)
            }
        }
        Adt17::C13(r68, r69) => {
            let v44 = (*r68).clone();
            let v45 = (*r69).clone();
            let v46: List<(Adt11, Adt25)> = f6(v45, v1, 0u64)?;
            f6(v44, v46, 0u64)
        }
        Adt17::C14(v47, _, _, r70, _, r71) => {
            let v50 = (*r70).clone();
            let v52 = (*r71).clone();
            let v53: List<(Adt11, Adt25)> = f6(v52, v1, 0u64)?;
            let v54: List<(Adt11, Adt25)> = f9(v47.clone(), v53);
            let m50 = f10(v47, v50.clone())?;
            match m50 {
                None => {
                    f6(v50, v54, 0u64)
                }
                Some(v55) => {
                    f16(v54, {
                        let (h51, _) = {
                            let (_, h52) = v55;
                            h52
                        };
                        h51
                    }, {
                        let (h53, _) = v55;
                        h53
                    })
                }
            }
        }
        Adt17::C15(v56, r72, r73) => {
            let v57 = (*r72).clone();
            let v58 = (*r73).clone();
            let v59: List<(Adt11, Adt25)> = f6(v57, v1.clone(), 0u64)?;
            let v60: List<(Adt11, Adt25)> = f6(v58, v1, 1u64)?;
            Ok(f7(f9(v56, v59), v60))
        }
        Adt17::C16(v61, r74) => {
            let v62 = (*r74).clone();
            let v63: List<(Adt11, Adt25)> = f6(v62.clone(), List::<(Adt11, Adt25)>::nil(), 0u64)?;
            let v64: List<(Adt11, Adt25)> = f12(v63, v61.clone());
            let v65: List<(Adt11, Adt25)> = f12(f7(v1, v64), v61.clone());
            let v66: List<(Adt11, Adt25)> = f6(v62, v65, 0u64)?;
            f5(v66, v61)
        }
    }
}

pub fn f7(p87: List<(Adt11, Adt25)>, p88: List<(Adt11, Adt25)>) -> List<(Adt11, Adt25)> {
    let m86 = (p87, p88);
    {
        let mut m86 = m86;
        loop {
            let step = {
                let (v0, v1) = m86;
                let m75 = v1;
                match m75.uncons() {
                    None => {
                        Err::<(List<(Adt11, Adt25)>, List<(Adt11, Adt25)>), List<(Adt11, Adt25)>>(v0)
                    }
                    Some((v2, v3)) => {
                        let v4: Adt25 = f14(v0.clone(), {
                            let (h76, _) = v2.clone();
                            h76
                        });
                        Ok::<(List<(Adt11, Adt25)>, List<(Adt11, Adt25)>), List<(Adt11, Adt25)>>((f15(v0, {
                            let (h77, _) = v2.clone();
                            h77
                        }, Adt25::C0(f17(match v4.clone() {
                            Adt25::C0(h78, _) => {
                                h78
                            }
                        }, {
                            let m81 = {
                                let (_, h80) = v2.clone();
                                h80
                            };
                            match m81 {
                                Adt25::C0(h79, _) => {
                                    h79
                                }
                            }
                        }), f18(match v4 {
                            Adt25::C0(_, h82) => {
                                h82
                            }
                        }, {
                            let m85 = {
                                let (_, h84) = v2;
                                h84
                            };
                            match m85 {
                                Adt25::C0(_, h83) => {
                                    h83
                                }
                            }
                        }))), v3))
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

pub fn f8(v0: Adt17) -> bool {
    let m89 = v0;
    match m89 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            true
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f9(v0: Adt13, v1: List<(Adt11, Adt25)>) -> List<(Adt11, Adt25)> {
    let m90 = v0;
    match m90 {
        Adt13::C0(_) => {
            v1
        }
        Adt13::C1(v3) => {
            f12(v1, v3)
        }
        Adt13::C2(r91) => {
            let v4 = (*r91).clone();
            f9(v4, v1)
        }
        Adt13::C3(_, r92) => {
            let v6 = (*r92).clone();
            f9(v6, v1)
        }
        Adt13::C4(_, _, r93) => {
            let v9 = (*r93).clone();
            f9(v9, v1)
        }
        Adt13::C5 => {
            v1
        }
        Adt13::C6(r94, r95) => {
            let v10 = (*r94).clone();
            let v11 = (*r95).clone();
            let v12: List<(Adt11, Adt25)> = f9(v10, v1);
            f9(v11, v12)
        }
        Adt13::C7(_) => {
            v1
        }
    }
}

pub fn f10(v0: Adt13, v1: Adt17) -> R<Option<(u64, (Adt11, Adt8))>> {
    let m96 = v0;
    match m96 {
        Adt13::C0(_) => {
            Ok(None::<(u64, (Adt11, Adt8))>)
        }
        Adt13::C1(_) => {
            Ok(None::<(u64, (Adt11, Adt8))>)
        }
        Adt13::C2(r102) => {
            let v4 = (*r102).clone();
            let m97 = f19(v1);
            match m97 {
                None => {
                    Ok(None::<(u64, (Adt11, Adt8))>)
                }
                Some(v5) => {
                    let a98 = f20(v4.clone())?;
                    let a99 = 1u64;
                    if if nat_eq(a98, a99) {
                        let a100 = 1u64;
                        let a101 = f21(v4.clone())?;
                        nat_lt(a100, a101)
                    } else {
                        false
                    } {
                        Ok(Some((f22(v4, 0u64)?, v5)))
                    } else {
                        Ok(None::<(u64, (Adt11, Adt8))>)
                    }
                }
            }
        }
        Adt13::C3(_, _) => {
            Ok(None::<(u64, (Adt11, Adt8))>)
        }
        Adt13::C4(_, _, _) => {
            Ok(None::<(u64, (Adt11, Adt8))>)
        }
        Adt13::C5 => {
            Ok(None::<(u64, (Adt11, Adt8))>)
        }
        Adt13::C6(_, _) => {
            Ok(None::<(u64, (Adt11, Adt8))>)
        }
        Adt13::C7(_) => {
            Ok(None::<(u64, (Adt11, Adt8))>)
        }
    }
}

pub fn f11(v0: List<(Adt11, Adt25)>, v1: Adt11, v2: u64) -> bool {
    let m103 = f13(v0, v1);
    match m103 {
        None => {
            true
        }
        Some(v3) => {
            let a105 = match v3.clone() {
                Adt25::C0(h104, _) => {
                    h104
                }
            };
            let a106 = 0u64;
            if nat_eq(a105, a106) {
                let a108 = f23(match v3.clone() {
                    Adt25::C0(_, h107) => {
                        h107
                    }
                }, v2);
                let a109 = 0u64;
                if nat_eq(a108, a109) {
                    f24(match v3 {
                        Adt25::C0(_, h110) => {
                            h110
                        }
                    })
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f12(v0: List<(Adt11, Adt25)>, v1: Adt11) -> List<(Adt11, Adt25)> {
    let m111 = v0;
    match m111.uncons() {
        None => {
            List::<(Adt11, Adt25)>::nil()
        }
        Some((v2, v3)) => {
            if f25({
                let (h112, _) = v2.clone();
                h112
            }, v1.clone()) {
                v3
            } else {
                List::cons(v2, f12(v3, v1))
            }
        }
    }
}

pub fn f13(p117: List<(Adt11, Adt25)>, v1: Adt11) -> Option<Adt25> {
    let m116 = p117;
    {
        let mut m116 = m116;
        loop {
            let step = {
                let v0 = m116;
                let m113 = v0;
                match m113.uncons() {
                    None => {
                        Err::<List<(Adt11, Adt25)>, Option<Adt25>>(None::<Adt25>)
                    }
                    Some((v2, v3)) => {
                        if f25({
                            let (h114, _) = v2;
                            h114
                        }, v1.clone()) {
                            Err::<List<(Adt11, Adt25)>, Option<Adt25>>(Some({
                                let (_, h115) = v2;
                                h115
                            }))
                        } else {
                            Ok::<List<(Adt11, Adt25)>, Option<Adt25>>(v3)
                        }
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

pub fn f14(v0: List<(Adt11, Adt25)>, v1: Adt11) -> Adt25 {
    let m118 = f13(v0, v1);
    match m118 {
        None => {
            Adt25::C0(0u64, List::<(u64, u64)>::nil())
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f15(v0: List<(Adt11, Adt25)>, v1: Adt11, v2: Adt25) -> List<(Adt11, Adt25)> {
    let m119 = v0;
    match m119.uncons() {
        None => {
            List::cons((v1, v2), List::<(Adt11, Adt25)>::nil())
        }
        Some((v3, v4)) => {
            if f25({
                let (h120, _) = v3.clone();
                h120
            }, v1.clone()) {
                List::cons((v1, v2), v4)
            } else {
                List::cons(v3, f15(v4, v1, v2))
            }
        }
    }
}

pub fn f16(v0: List<(Adt11, Adt25)>, v1: Adt11, v2: u64) -> R<List<(Adt11, Adt25)>> {
    let v3: Adt25 = f14(v0.clone(), v1.clone());
    Ok(f15(v0, v1, Adt25::C0(match v3.clone() {
        Adt25::C0(h121, _) => {
            h121
        }
    }, f26(match v3.clone() {
        Adt25::C0(_, h122) => {
            h122
        }
    }, v2, {
        let a124 = f23(match v3 {
            Adt25::C0(_, h123) => {
                h123
            }
        }, v2);
        let a125 = 1u64;
        nat_add(a124, a125)?
    }))))
}

pub fn f17(v0: u64, v1: u64) -> u64 {
    let a126 = v0;
    let a127 = v1;
    if nat_lt(a126, a127) {
        v1
    } else {
        v0
    }
}

pub fn f18(p133: List<(u64, u64)>, p134: List<(u64, u64)>) -> List<(u64, u64)> {
    let m132 = (p133, p134);
    {
        let mut m132 = m132;
        loop {
            let step = {
                let (v0, v1) = m132;
                let m128 = v1;
                match m128.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>((f26(v0.clone(), {
                            let (h129, _) = v2;
                            h129
                        }, f17({
                            let (_, h130) = v2;
                            h130
                        }, f23(v0, {
                            let (h131, _) = v2;
                            h131
                        }))), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt17) -> Option<(Adt11, Adt8)> {
    let m135 = v0;
    match m135 {
        Adt17::C0(_, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C1(v3, v4, v5) => {
            let m136 = v3;
            match m136 {
                Adt19::C0 => {
                    None::<(Adt11, Adt8)>
                }
                Adt19::C1 => {
                    Some((v4, v5))
                }
                Adt19::C2 => {
                    None::<(Adt11, Adt8)>
                }
                Adt19::C3 => {
                    None::<(Adt11, Adt8)>
                }
                Adt19::C4 => {
                    None::<(Adt11, Adt8)>
                }
            }
        }
        Adt17::C2(_, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C3(_, _, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C4(_, _, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C5(_, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C6(_, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C7(_, _, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C8(_, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C9(_) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C10(_, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C11(_, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C12 => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C13(_, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C14(_, _, _, _, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C15(_, _, _) => {
            None::<(Adt11, Adt8)>
        }
        Adt17::C16(_, _) => {
            None::<(Adt11, Adt8)>
        }
    }
}

pub fn f20(v0: Adt13) -> R<u64> {
    let m137 = v0;
    match m137 {
        Adt13::C0(_) => {
            Ok(0u64)
        }
        Adt13::C1(_) => {
            Ok(0u64)
        }
        Adt13::C2(_) => {
            Ok(0u64)
        }
        Adt13::C3(_, _) => {
            Ok(0u64)
        }
        Adt13::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt13::C5 => {
            Ok(0u64)
        }
        Adt13::C6(r140, r141) => {
            let v9 = (*r140).clone();
            let v10 = (*r141).clone();
            if f27(v9) {
                f20(v10)
            } else {
                let a138 = 1u64;
                let a139 = f20(v10)?;
                nat_add(a138, a139)
            }
        }
        Adt13::C7(_) => {
            Ok(0u64)
        }
    }
}

pub fn f21(v0: Adt13) -> R<u64> {
    let m142 = v0;
    match m142 {
        Adt13::C0(_) => {
            Ok(0u64)
        }
        Adt13::C1(_) => {
            Ok(0u64)
        }
        Adt13::C2(_) => {
            Ok(0u64)
        }
        Adt13::C3(_, _) => {
            Ok(0u64)
        }
        Adt13::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt13::C5 => {
            Ok(0u64)
        }
        Adt13::C6(_, r145) => {
            let v10 = (*r145).clone();
            let a143 = 1u64;
            let a144 = f21(v10)?;
            nat_add(a143, a144)
        }
        Adt13::C7(_) => {
            Ok(0u64)
        }
    }
}

pub fn f22(p152: Adt13, p153: u64) -> R<u64> {
    let m151 = (p152, p153);
    {
        let mut m151 = m151;
        loop {
            let step = {
                let (v0, v1) = m151;
                let m146 = v0;
                match m146 {
                    Adt13::C0(_) => {
                        Err::<(Adt13, u64), R<u64>>(Ok(v1))
                    }
                    Adt13::C1(_) => {
                        Err::<(Adt13, u64), R<u64>>(Ok(v1))
                    }
                    Adt13::C2(_) => {
                        Err::<(Adt13, u64), R<u64>>(Ok(v1))
                    }
                    Adt13::C3(_, _) => {
                        Err::<(Adt13, u64), R<u64>>(Ok(v1))
                    }
                    Adt13::C4(_, _, _) => {
                        Err::<(Adt13, u64), R<u64>>(Ok(v1))
                    }
                    Adt13::C5 => {
                        Err::<(Adt13, u64), R<u64>>(Ok(v1))
                    }
                    Adt13::C6(r149, r150) => {
                        let v10 = (*r149).clone();
                        let v11 = (*r150).clone();
                        if f27(v10) {
                            Ok::<(Adt13, u64), R<u64>>((v11, {
                                let a147 = v1;
                                let a148 = 1u64;
                                nat_add(a147, a148)?
                            }))
                        } else {
                            Err::<(Adt13, u64), R<u64>>(Ok(v1))
                        }
                    }
                    Adt13::C7(_) => {
                        Err::<(Adt13, u64), R<u64>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m151 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(p160: List<(u64, u64)>, v1: u64) -> u64 {
    let m159 = p160;
    {
        let mut m159 = m159;
        loop {
            let step = {
                let v0 = m159;
                let m154 = v0;
                match m154.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, u64>(0u64)
                    }
                    Some((v2, v3)) => {
                        let a156 = {
                            let (h155, _) = v2;
                            h155
                        };
                        let a157 = v1;
                        if nat_eq(a156, a157) {
                            let (_, h158) = v2;
                            Err::<List<(u64, u64)>, u64>(h158)
                        } else {
                            Ok::<List<(u64, u64)>, u64>(v3)
                        }
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

pub fn f24(p166: List<(u64, u64)>) -> bool {
    let m165 = p166;
    {
        let mut m165 = m165;
        loop {
            let step = {
                let v0 = m165;
                let m161 = v0;
                match m161.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a163 = {
                            let (_, h162) = v1;
                            h162
                        };
                        let a164 = 2u64;
                        if nat_lt(a163, a164) {
                            Ok::<List<(u64, u64)>, bool>(v2)
                        } else {
                            Err::<List<(u64, u64)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt11, v1: Adt11) -> bool {
    let v2: (u64, u64) = f28(v0);
    let v3: (u64, u64) = f28(v1);
    let a169 = {
        let (h167, _) = v2;
        h167
    };
    let a170 = {
        let (h168, _) = v3;
        h168
    };
    if nat_eq(a169, a170) {
        let a173 = {
            let (_, h171) = v2;
            h171
        };
        let a174 = {
            let (_, h172) = v3;
            h172
        };
        nat_eq(a173, a174)
    } else {
        false
    }
}

pub fn f26(v0: List<(u64, u64)>, v1: u64, v2: u64) -> List<(u64, u64)> {
    let m175 = v0;
    match m175.uncons() {
        None => {
            List::cons((v1, v2), List::<(u64, u64)>::nil())
        }
        Some((v3, v4)) => {
            let a177 = {
                let (h176, _) = v3;
                h176
            };
            let a178 = v1;
            if nat_eq(a177, a178) {
                List::cons((v1, v2), v4)
            } else {
                List::cons(v3, f26(v4, v1, v2))
            }
        }
    }
}

pub fn f27(v0: Adt13) -> bool {
    let m179 = v0;
    match m179 {
        Adt13::C0(v1) => {
            let m180 = v1;
            match m180 {
                Adt14::C0 => {
                    true
                }
                Adt14::C1 => {
                    false
                }
                Adt14::C2 => {
                    false
                }
            }
        }
        Adt13::C1(_) => {
            false
        }
        Adt13::C2(_) => {
            false
        }
        Adt13::C3(_, _) => {
            false
        }
        Adt13::C4(_, _, _) => {
            false
        }
        Adt13::C5 => {
            false
        }
        Adt13::C6(_, _) => {
            false
        }
        Adt13::C7(_) => {
            false
        }
    }
}

pub fn f28(v0: Adt11) -> (u64, u64) {
    let m181 = v0;
    match m181 {
        Adt11::C0(v1) => {
            (0u64, v1)
        }
        Adt11::C1(v2) => {
            (1u64, v2)
        }
        Adt11::C2(v3) => {
            (2u64, v3)
        }
        Adt11::C3(v4) => {
            (3u64, v4)
        }
        Adt11::C4(v5) => {
            (4u64, v5)
        }
        Adt11::C5(v6) => {
            (5u64, v6)
        }
        Adt11::C6(v7) => {
            (6u64, v7)
        }
        Adt11::C7(v8) => {
            (7u64, v8)
        }
        Adt11::C8(v9) => {
            (8u64, v9)
        }
        Adt11::C9(_) => {
            (9u64, 0u64)
        }
    }
}
