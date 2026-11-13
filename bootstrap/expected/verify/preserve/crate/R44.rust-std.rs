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
    C0(Adt0, Adt8, Adt0, Adt9),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(Adt0, Adt0, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(Adt0, u64, Adt0, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt0, Adt9),
}

#[derive(Clone)]
pub enum Fn0 {
    F19,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn0::F19 => f19(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F21(Adt9, Adt8, Adt0, Adt0, Adt10),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn1::F21(k0, k1, k2, k3, k4) => f21(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: List<bool>) -> R<Adt7> {
    let v8: Adt10 = Adt10::C0(v0, v1.clone(), v2, v3, v4, v5.clone());
    let v9: Adt11 = f1(v8.clone(), Adt11::C0(Adt0::C14, Adt9::C0), v5.clone(), 0u64, List::<bool>::nil(), v6.clone())?;
    let v10: Adt0 = f2(match v9.clone() {
        Adt11::C0(h1, _) => {
            h1
        }
    }, v1.clone());
    let v11: Adt11 = f1(v8.clone(), Adt11::C0(v10.clone(), Adt9::C0), v5, 0u64, v7, v6)?;
    let v12: Adt0 = f3(f4(f5(v1)?), v10.clone());
    let v13: Adt8 = f6(v8.clone(), v10.clone(), match v9.clone() {
        Adt11::C0(_, h2) => {
            h2
        }
    }, v12.clone(), Adt8::C0);
    let v14: Adt8 = f7(v13, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f5(v12.clone())?;
            let a4 = f5(v12.clone())?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?;
    Ok(Adt7::C0(v10.clone(), v14.clone(), f8(v8, v10, match v9 {
        Adt11::C0(_, h7) => {
            h7
        }
    }, v14, v12)?, f9(match v11 {
        Adt11::C0(_, h8) => {
            h8
        }
    }, Adt9::C0)))
}

pub fn f1(v0: Adt10, p19: Adt11, p20: Adt2, p21: u64, v4: List<bool>, p22: Adt0) -> R<Adt11> {
    let m18 = (p19, (p20, (p21, p22)));
    {
        let mut m18 = m18;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m18;
                let m9 = v2;
                match m9 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C14(r16, r17) => {
                        let v34 = (*r16).clone();
                        let v35 = (*r17).clone();
                        let v37: bool = {
                            let m10 = f10(v4.clone(), v3);
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
                        let v38: Adt11 = Adt11::C0(f11(v0.clone(), match v1.clone() {
                            Adt11::C0(h12, _) => {
                                h12
                            }
                        }, v3), match v1 {
                            Adt11::C0(_, h13) => {
                                h13
                            }
                        });
                        let v39: Adt11 = if v37 {
                            f12(v0.clone(), v38, v34, f13(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>((v39, (v35, ({
                            let a14 = v3;
                            let a15 = 1u64;
                            nat_add(a14, a15)?
                        }, f14(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
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
                        Ok::<(Adt0, Adt0), Adt0>((f15(v0, v12), v13))
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

pub fn f6(v0: Adt10, v1: Adt0, v2: Adt9, p41: Adt0, p42: Adt8) -> Adt8 {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v3, v4) = m40;
                let m37 = v3;
                match m37 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C15(r38, r39) => {
                        let v15 = (*r38).clone();
                        let v16 = (*r39).clone();
                        Ok::<(Adt0, Adt8), Adt8>((v16, f16(v1.clone(), v15.clone(), f17(v0.clone(), v2.clone(), v15), v4)))
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

pub fn f7(v0: Adt8, v1: u64) -> R<Adt8> {
    let (h43, _) = f18(Fn0::F19, v1, v0)?;
    Ok(h43)
}

pub fn f8(v0: Adt10, v1: Adt0, v2: Adt9, v3: Adt8, v4: Adt0) -> R<Adt0> {
    let (h44, _) = f20(Fn1::F21(v2, v3, v1, v4.clone(), v0), {
        let a45 = 1u64;
        let a46 = f5(v4)?;
        nat_add(a45, a46)?
    }, Adt0::C14)?;
    Ok(h44)
}

pub fn f9(p50: Adt9, p51: Adt9) -> Adt9 {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v0, v1) = m49;
                let m47 = v0;
                match m47 {
                    Adt9::C0 => {
                        Err::<(Adt9, Adt9), Adt9>(v1)
                    }
                    Adt9::C1(v2, v3, v4, r48) => {
                        let v5 = (*r48).clone();
                        Ok::<(Adt9, Adt9), Adt9>((v5, Adt9::C1(v2, v3, v4, std::rc::Rc::new(v1))))
                    }
                }
            };
            match step {
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(p55: List<bool>, p56: u64) -> Option<bool> {
    let m54 = (p55, p56);
    {
        let mut m54 = m54;
        loop {
            let step = {
                let (v0, v1) = m54;
                let m52 = v0;
                match m52.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m53 = v1;
                        if m53 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m53 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m54 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Adt10, v1: Adt0, v2: u64) -> Adt0 {
    f22(f23(v1, f24(v0.clone(), v2)), f25(v0, v2))
}

pub fn f12(v0: Adt10, v1: Adt11, v2: Adt2, v3: Adt0) -> R<Adt11> {
    let m57 = v2;
    match m57 {
        Adt2::C0(v4, _) => {
            Ok(Adt11::C0(f22(match v1.clone() {
                Adt11::C0(h58, _) => {
                    h58
                }
            }, v4), match v1 {
                Adt11::C0(_, h59) => {
                    h59
                }
            }))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, v8, r68, r69) => {
            let v9 = (*r68).clone();
            let v10 = (*r69).clone();
            let v11: Adt11 = Adt11::C0(f22(match v1.clone() {
                Adt11::C0(h60, _) => {
                    h60
                }
            }, v8), match v1 {
                Adt11::C0(_, h61) => {
                    h61
                }
            });
            let v12: Adt11 = f12(v0.clone(), v11, v9, f13(v3.clone()))?;
            f12(v0, v12, v10, f14(v3))
        }
        Adt2::C3(r70, r71, r72) => {
            let v13 = (*r70).clone();
            let v14 = (*r71).clone();
            let v15 = (*r72).clone();
            let v16: Adt11 = f12(v0.clone(), v1, v13, f13(v3.clone()))?;
            let v17: Adt11 = f12(v0.clone(), v16, v14, f14(v3.clone()))?;
            f12(v0, v17, v15, f26(v3))
        }
        Adt2::C4(v18, r73, r74) => {
            let v19 = (*r73).clone();
            let v20 = (*r74).clone();
            let v21: Adt11 = Adt11::C0(f22(match v1.clone() {
                Adt11::C0(h62, _) => {
                    h62
                }
            }, v18), match v1 {
                Adt11::C0(_, h63) => {
                    h63
                }
            });
            let v22: Adt11 = f12(v0.clone(), v21, v19, f13(v3.clone()))?;
            f12(v0, v22, v20, f14(v3))
        }
        Adt2::C5(_, v24, r75) => {
            let v25 = (*r75).clone();
            let v26: Adt11 = Adt11::C0(f22(match v1.clone() {
                Adt11::C0(h64, _) => {
                    h64
                }
            }, v24), match v1 {
                Adt11::C0(_, h65) => {
                    h65
                }
            });
            f12(v0, v26, v25, f13(v3))
        }
        Adt2::C6(_, r76) => {
            let v28 = (*r76).clone();
            f12(v0, v1, v28, f13(v3))
        }
        Adt2::C7(v29, r77) => {
            let v30 = (*r77).clone();
            let v31: Adt0 = f24(v0.clone(), v29);
            let v32: u64 = f27(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f28(v31.clone(), v32)), std::rc::Rc::new(f25(v0.clone(), v29)));
            let v34: Adt11 = Adt11::C0(f22(match v1.clone() {
                Adt11::C0(h66, _) => {
                    h66
                }
            }, v33.clone()), f29(match v1 {
                Adt11::C0(_, h67) => {
                    h67
                }
            }, v33, v29, f30(v31, v32)));
            f12(v0, v34, v30, f13(v3))
        }
        Adt2::C8(r78, r79) => {
            let v35 = (*r78).clone();
            let v36 = (*r79).clone();
            let v37: Adt11 = f12(v0.clone(), v1, v35, f13(v3.clone()))?;
            f12(v0, v37, v36, f14(v3))
        }
        Adt2::C9(_, r80) => {
            let v39 = (*r80).clone();
            f12(v0, v1, v39, f13(v3))
        }
        Adt2::C10(r81) => {
            let v40 = (*r81).clone();
            f12(v0, v1, v40, f13(v3))
        }
        Adt2::C11(r82) => {
            let v41 = (*r82).clone();
            f12(v0, v1, v41, f13(v3))
        }
        Adt2::C12(r83, _) => {
            let v42 = (*r83).clone();
            f12(v0, v1, v42, f13(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r84, r85) => {
            let v44 = (*r84).clone();
            let v45 = (*r85).clone();
            let v46: Adt11 = f12(v0.clone(), v1, v44, f13(v3.clone()))?;
            f12(v0, v46, v45, f14(v3))
        }
        Adt2::C15(_, _, r86) => {
            let v49 = (*r86).clone();
            f12(v0, v1, v49, f13(v3))
        }
    }
}

pub fn f13(v0: Adt0) -> Adt0 {
    f31(f32(v0))
}

pub fn f14(v0: Adt0) -> Adt0 {
    f31(f32(f32(v0)))
}

pub fn f15(p91: Adt0, p92: Adt0) -> Adt0 {
    let m90 = (p91, p92);
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v0, v1) = m90;
                let m87 = v1;
                match m87 {
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
                    Adt0::C15(r88, r89) => {
                        let v12 = (*r88).clone();
                        let v13 = (*r89).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f23(v0, v12), v13))
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

pub fn f16(v0: Adt0, v1: Adt0, p97: Adt0, p98: Adt8) -> Adt8 {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v2, v3) = m96;
                let m93 = v2;
                match m93 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C15(r94, r95) => {
                        let v14 = (*r94).clone();
                        let v15 = (*r95).clone();
                        Ok::<(Adt0, Adt8), Adt8>((v15, f33(v0.clone(), v1.clone(), v14, v3)))
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

pub fn f17(v0: Adt10, v1: Adt9, v2: Adt0) -> Adt0 {
    let m99 = v2;
    match m99 {
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
            let m100 = f34(v0, v10);
            match m100 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r101, r102) => {
            let v12 = (*r101).clone();
            let v13 = (*r102).clone();
            f35(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f18(v0: Fn0, p107: u64, p108: Adt8) -> R<(Adt8, bool)> {
    let m106 = (p107, p108);
    {
        let mut m106 = m106;
        loop {
            let step = {
                let (v1, v2) = m106;
                let m103 = v1;
                if m103 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m103 - 1;
                    let m105 = {
                        let c104: Fn0 = v0.clone();
                        c104.apply(v2.clone())?
                    };
                    match m105 {
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
                Ok(next) => { m106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt8) -> R<Option<Adt8>> {
    f36(v0)
}

pub fn f20(v0: Fn1, p113: u64, p114: Adt0) -> R<(Adt0, bool)> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m109 = v1;
                if m109 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m109 - 1;
                    let m111 = {
                        let c110: Fn1 = v0.clone();
                        c110.apply(v2.clone())?
                    };
                    match m111 {
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
                Ok(next) => { m112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt9, v1: Adt8, v2: Adt0, v3: Adt0, v4: Adt10, v5: Adt0) -> R<Option<Adt0>> {
    f37(v4, v2, v0, v1, v3, v5)
}

pub fn f22(v0: Adt0, v1: Adt0) -> Adt0 {
    let m115 = v1;
    match m115 {
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
        Adt0::C8(r116) => {
            let v3 = (*r116).clone();
            f22(v0, v3)
        }
        Adt0::C9(r117, r118) => {
            let v4 = (*r117).clone();
            let v5 = (*r118).clone();
            f22(f22(v0, v4), v5)
        }
        Adt0::C10(r119) => {
            let v6 = (*r119).clone();
            f22(v0, v6)
        }
        Adt0::C11(r120, r121) => {
            let v7 = (*r120).clone();
            let v8 = (*r121).clone();
            f22(f22(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r122, r123) => {
            let v10 = (*r122).clone();
            let v11 = (*r123).clone();
            let v12: Adt0 = f38(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f22(f22(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r124, r125) => {
            let v13 = (*r124).clone();
            let v14 = (*r125).clone();
            f22(f22(v0, v13), v14)
        }
    }
}

pub fn f23(p130: Adt0, p131: Adt0) -> Adt0 {
    let m129 = (p130, p131);
    {
        let mut m129 = m129;
        loop {
            let step = {
                let (v0, v1) = m129;
                let m126 = v1;
                match m126 {
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
                    Adt0::C15(r127, r128) => {
                        let v2 = (*r127).clone();
                        let v3 = (*r128).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f22(v0, v2), v3))
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

pub fn f24(v0: Adt10, v1: u64) -> Adt0 {
    let m132 = f39(v0, v1);
    match m132 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f25(v0: Adt10, v1: u64) -> Adt0 {
    let m134 = f40(match v0 {
        Adt10::C0(_, _, _, _, h133, _) => {
            h133
        }
    }, v1);
    match m134 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f26(v0: Adt0) -> Adt0 {
    f31(f32(f32(f32(v0))))
}

pub fn f27(v0: Adt2) -> R<u64> {
    let m135 = v0;
    match m135 {
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
        Adt2::C14(_, r138) => {
            let v33 = (*r138).clone();
            let v34: u64 = f27(v33)?;
            let a136 = 1u64;
            let a137 = v34;
            nat_add(a136, a137)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f28(p144: Adt0, p145: u64) -> Adt0 {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v0, v1) = m143;
                let m139 = v0;
                match m139 {
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
                    Adt0::C15(r141, r142) => {
                        let v12 = (*r141).clone();
                        let v13 = (*r142).clone();
                        let m140 = v1;
                        if m140 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m140 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
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

pub fn f29(v0: Adt9, v1: Adt0, v2: u64, v3: Adt0) -> Adt9 {
    if f41(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt9::C1(v1, v2, v3, std::rc::Rc::new(v0))
    }
}

pub fn f30(v0: Adt0, v1: u64) -> Adt0 {
    let m146 = v0;
    match m146 {
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
        Adt0::C15(r148, r149) => {
            let v12 = (*r148).clone();
            let v13 = (*r149).clone();
            let m147 = v1;
            if m147 == 0 {
                Adt0::C14
            } else {
                let v14 = m147 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f30(v13, v14)))
            }
        }
    }
}

pub fn f31(v0: Adt0) -> Adt0 {
    let m150 = f42(v0);
    match m150 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f32(v0: Adt0) -> Adt0 {
    let m151 = f43(v0);
    match m151 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt0, p156: Adt0, p157: Adt8) -> Adt8 {
    let m155 = (p156, p157);
    {
        let mut m155 = m155;
        loop {
            let step = {
                let (v2, v3) = m155;
                let m152 = v2;
                match m152 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C15(r153, r154) => {
                        let v14 = (*r153).clone();
                        let v15 = (*r154).clone();
                        Ok::<(Adt0, Adt8), Adt8>((v15, f44(v1.clone(), f45(v0.clone(), Adt0::C14, v14), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Adt10, v1: u64) -> Option<Adt0> {
    f40(match v0 {
        Adt10::C0(_, h158, _, _, _, _) => {
            h158
        }
    }, v1)
}

pub fn f35(v0: Adt9, v1: Adt0) -> Adt0 {
    let m159 = v0;
    match m159 {
        Adt9::C0 => {
            Adt0::C14
        }
        Adt9::C1(v2, _, v4, r160) => {
            let v5 = (*r160).clone();
            if f46(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f35(v5, v1)))
            } else {
                f35(v5, v1)
            }
        }
    }
}

pub fn f36(v0: Adt8) -> R<Option<Adt8>> {
    let v1: Adt8 = f47(v0.clone(), v0.clone(), v0.clone());
    let a161 = f48(v1.clone())?;
    let a162 = f48(v0)?;
    if nat_eq(a161, a162) {
        Ok(None::<Adt8>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f37(v0: Adt10, v1: Adt0, v2: Adt9, v3: Adt8, v4: Adt0, v5: Adt0) -> R<Option<Adt0>> {
    let v6: Adt0 = f49(v0, v1, v2, v3, v4, v5.clone());
    let a163 = f5(v6.clone())?;
    let a164 = f5(v5)?;
    if nat_eq(a163, a164) {
        Ok(None::<Adt0>)
    } else {
        Ok(Some(v6))
    }
}

pub fn f38(v0: Adt0, v1: Adt0) -> Adt0 {
    if f50(v0.clone(), v1.clone()) {
        v0
    } else {
        f3(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f39(v0: Adt10, v1: u64) -> Option<Adt0> {
    f40(match v0 {
        Adt10::C0(_, _, _, h165, _, _) => {
            h165
        }
    }, v1)
}

pub fn f40(p171: Adt0, p172: u64) -> Option<Adt0> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v0, v1) = m170;
                let m166 = v0;
                match m166 {
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
                    Adt0::C15(r168, r169) => {
                        let v12 = (*r168).clone();
                        let v13 = (*r169).clone();
                        let m167 = v1;
                        if m167 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m167 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(p178: Adt9, v1: Adt0, v2: u64) -> bool {
    let m177 = p178;
    {
        let mut m177 = m177;
        loop {
            let step = {
                let v0 = m177;
                let m173 = v0;
                match m173 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(v3, v4, _, r176) => {
                        let v6 = (*r176).clone();
                        let a174 = v4;
                        let a175 = v2;
                        if nat_eq(a174, a175) {
                            if f46(v3, v1.clone()) {
                                Err::<Adt9, bool>(true)
                            } else {
                                Ok::<Adt9, bool>(v6)
                            }
                        } else {
                            Ok::<Adt9, bool>(v6)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Adt0) -> Option<Adt0> {
    let m179 = v0;
    match m179 {
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
        Adt0::C8(r180) => {
            let v2 = (*r180).clone();
            Some(v2)
        }
        Adt0::C9(r181, _) => {
            let v3 = (*r181).clone();
            Some(v3)
        }
        Adt0::C10(r182) => {
            let v5 = (*r182).clone();
            Some(v5)
        }
        Adt0::C11(r183, _) => {
            let v6 = (*r183).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r184, _) => {
            let v9 = (*r184).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r185, _) => {
            let v11 = (*r185).clone();
            Some(v11)
        }
    }
}

pub fn f43(v0: Adt0) -> Option<Adt0> {
    let m186 = v0;
    match m186 {
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
        Adt0::C9(_, r187) => {
            let v4 = (*r187).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r188) => {
            let v7 = (*r188).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r189) => {
            let v10 = (*r189).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r190) => {
            let v12 = (*r190).clone();
            Some(v12)
        }
    }
}

pub fn f44(v0: Adt0, p195: Adt0, p196: Adt8) -> Adt8 {
    let m194 = (p195, p196);
    {
        let mut m194 = m194;
        loop {
            let step = {
                let (v1, v2) = m194;
                let m191 = v1;
                match m191 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C15(r192, r193) => {
                        let v13 = (*r192).clone();
                        let v14 = (*r193).clone();
                        Ok::<(Adt0, Adt8), Adt8>((v14, f51(v2, v0.clone(), v13)))
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

pub fn f45(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m197 = v2;
    match m197 {
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
        Adt0::C8(r198) => {
            let v4 = (*r198).clone();
            f45(v0, v1, v4)
        }
        Adt0::C9(r199, r200) => {
            let v5 = (*r199).clone();
            let v6 = (*r200).clone();
            f45(v0.clone(), f45(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r201, r202) => {
            let v8 = (*r201).clone();
            let v9 = (*r202).clone();
            f45(v0.clone(), f45(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f38(v1, Adt0::C12(v10))
        }
        Adt0::C13(r203, r204) => {
            let v11 = (*r203).clone();
            let v12 = (*r204).clone();
            if f50(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f38(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
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

pub fn f46(v0: Adt0, v1: Adt0) -> bool {
    let m205 = v0;
    match m205 {
        Adt0::C0 => {
            let a206 = f52(v1);
            let a207 = 0u64;
            nat_eq(a206, a207)
        }
        Adt0::C1 => {
            let a208 = f52(v1);
            let a209 = 1u64;
            nat_eq(a208, a209)
        }
        Adt0::C2 => {
            let a210 = f52(v1);
            let a211 = 2u64;
            nat_eq(a210, a211)
        }
        Adt0::C3 => {
            let a212 = f52(v1);
            let a213 = 3u64;
            nat_eq(a212, a213)
        }
        Adt0::C4(v2) => {
            let a222 = f52(v1.clone());
            let a223 = 4u64;
            if nat_eq(a222, a223) {
                let a224 = f53(v2);
                let a225 = f53(f54(v1));
                nat_eq(a224, a225)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a214 = f52(v1);
            let a215 = 5u64;
            nat_eq(a214, a215)
        }
        Adt0::C6 => {
            let a216 = f52(v1);
            let a217 = 6u64;
            nat_eq(a216, a217)
        }
        Adt0::C7 => {
            let a218 = f52(v1);
            let a219 = 7u64;
            nat_eq(a218, a219)
        }
        Adt0::C8(r242) => {
            let v3 = (*r242).clone();
            let v4: bool = f46(v3, f31(v1.clone()));
            let a226 = f52(v1);
            let a227 = 8u64;
            if nat_eq(a226, a227) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r243, r244) => {
            let v5 = (*r243).clone();
            let v6 = (*r244).clone();
            let v7: bool = f46(v5, f31(v1.clone()));
            let v8: bool = f46(v6, f32(v1.clone()));
            let a228 = f52(v1);
            let a229 = 9u64;
            if nat_eq(a228, a229) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r245) => {
            let v9 = (*r245).clone();
            let v10: bool = f46(v9, f31(v1.clone()));
            let a230 = f52(v1);
            let a231 = 10u64;
            if nat_eq(a230, a231) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r246, r247) => {
            let v11 = (*r246).clone();
            let v12 = (*r247).clone();
            let v13: bool = f46(v11, f31(v1.clone()));
            let v14: bool = f46(v12, f32(v1.clone()));
            let a232 = f52(v1);
            let a233 = 11u64;
            if nat_eq(a232, a233) {
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
            let a234 = f52(v1.clone());
            let a235 = 12u64;
            if nat_eq(a234, a235) {
                let a236 = v15;
                let a237 = f55(v1);
                nat_eq(a236, a237)
            } else {
                false
            }
        }
        Adt0::C13(r248, r249) => {
            let v16 = (*r248).clone();
            let v17 = (*r249).clone();
            let v18: bool = f46(v16, f31(v1.clone()));
            let v19: bool = f46(v17, f32(v1.clone()));
            let a238 = f52(v1);
            let a239 = 13u64;
            if nat_eq(a238, a239) {
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
            let a220 = f52(v1);
            let a221 = 14u64;
            nat_eq(a220, a221)
        }
        Adt0::C15(r250, r251) => {
            let v20 = (*r250).clone();
            let v21 = (*r251).clone();
            let v22: bool = f46(v20, f31(v1.clone()));
            let v23: bool = f46(v21, f32(v1.clone()));
            let a240 = f52(v1);
            let a241 = 15u64;
            if nat_eq(a240, a241) {
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

pub fn f47(v0: Adt8, p255: Adt8, p256: Adt8) -> Adt8 {
    let m254 = (p255, p256);
    {
        let mut m254 = m254;
        loop {
            let step = {
                let (v1, v2) = m254;
                let m252 = v1;
                match m252 {
                    Adt8::C0 => {
                        Err::<(Adt8, Adt8), Adt8>(v2)
                    }
                    Adt8::C1(v3, v4, r253) => {
                        let v5 = (*r253).clone();
                        Ok::<(Adt8, Adt8), Adt8>((v5, f56(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Adt8) -> R<u64> {
    let m257 = v0;
    match m257 {
        Adt8::C0 => {
            Ok(0u64)
        }
        Adt8::C1(_, _, r260) => {
            let v3 = (*r260).clone();
            let a258 = 1u64;
            let a259 = f48(v3)?;
            nat_add(a258, a259)
        }
    }
}

pub fn f49(v0: Adt10, v1: Adt0, v2: Adt9, v3: Adt8, p265: Adt0, p266: Adt0) -> Adt0 {
    let m264 = (p265, p266);
    {
        let mut m264 = m264;
        loop {
            let step = {
                let (v4, v5) = m264;
                let m261 = v4;
                match m261 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C15(r262, r263) => {
                        let v16 = (*r262).clone();
                        let v17 = (*r263).clone();
                        Ok::<(Adt0, Adt0), Adt0>((v17, if f50(v5.clone(), v16.clone()) {
                            v5
                        } else {
                            if f57(v1.clone(), v3.clone(), v5.clone(), v16.clone(), f17(v0.clone(), v2.clone(), v16.clone())) {
                                f38(v5, v16)
                            } else {
                                v5
                            }
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(p271: Adt0, v1: Adt0) -> bool {
    let m270 = p271;
    {
        let mut m270 = m270;
        loop {
            let step = {
                let v0 = m270;
                let m267 = v0;
                match m267 {
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
                    Adt0::C15(r268, r269) => {
                        let v12 = (*r268).clone();
                        let v13 = (*r269).clone();
                        if f46(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m270 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt8, v1: Adt0, v2: Adt0) -> Adt8 {
    if f58(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt8::C1(v1, v2, std::rc::Rc::new(v0))
    }
}

pub fn f52(v0: Adt0) -> u64 {
    let m272 = v0;
    match m272 {
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

pub fn f53(v0: Adt1) -> u64 {
    let m273 = v0;
    match m273 {
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

pub fn f54(v0: Adt0) -> Adt1 {
    let m274 = v0;
    match m274 {
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

pub fn f55(v0: Adt0) -> u64 {
    let m275 = v0;
    match m275 {
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

pub fn f56(p279: Adt8, v1: Adt0, v2: Adt0, p280: Adt8) -> Adt8 {
    let m278 = (p279, p280);
    {
        let mut m278 = m278;
        loop {
            let step = {
                let (v0, v3) = m278;
                let m276 = v0;
                match m276 {
                    Adt8::C0 => {
                        Err::<(Adt8, Adt8), Adt8>(v3)
                    }
                    Adt8::C1(v4, v5, r277) => {
                        let v6 = (*r277).clone();
                        Ok::<(Adt8, Adt8), Adt8>((v6, if f46(v4, v1.clone()) {
                            f51(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0, p285: Adt0) -> bool {
    let m284 = p285;
    {
        let mut m284 = m284;
        loop {
            let step = {
                let v4 = m284;
                let m281 = v4;
                match m281 {
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
                    Adt0::C15(r282, r283) => {
                        let v15 = (*r282).clone();
                        let v16 = (*r283).clone();
                        if f59(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v15) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v16)
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

pub fn f58(p289: Adt8, v1: Adt0, v2: Adt0) -> bool {
    let m288 = p289;
    {
        let mut m288 = m288;
        loop {
            let step = {
                let v0 = m288;
                let m286 = v0;
                match m286 {
                    Adt8::C0 => {
                        Err::<Adt8, bool>(false)
                    }
                    Adt8::C1(v3, v4, r287) => {
                        let v5 = (*r287).clone();
                        if f46(v3, v1.clone()) {
                            if f46(v4, v2.clone()) {
                                Err::<Adt8, bool>(true)
                            } else {
                                Ok::<Adt8, bool>(v5)
                            }
                        } else {
                            Ok::<Adt8, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0, p294: Adt0) -> bool {
    let m293 = p294;
    {
        let mut m293 = m293;
        loop {
            let step = {
                let v4 = m293;
                let m290 = v4;
                match m290 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C15(r291, r292) => {
                        let v15 = (*r291).clone();
                        let v16 = (*r292).clone();
                        if if f60(v0.clone(), v1.clone(), v3.clone(), v15.clone()) {
                            true
                        } else {
                            f61(v15, v2.clone())
                        } {
                            Ok::<Adt0, bool>(v16)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m293 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0) -> bool {
    f62(v2, v1, f45(v0, Adt0::C14, v3))
}

pub fn f61(v0: Adt0, v1: Adt0) -> bool {
    let m295 = v0;
    match m295 {
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
            true
        }
        Adt0::C6 => {
            true
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(_) => {
            true
        }
        Adt0::C9(r296, r297) => {
            let v4 = (*r296).clone();
            let v5 = (*r297).clone();
            if f61(v4, v1.clone()) {
                true
            } else {
                f61(v5, v1)
            }
        }
        Adt0::C10(_) => {
            true
        }
        Adt0::C11(r298, r299) => {
            let v7 = (*r298).clone();
            let v8 = (*r299).clone();
            if f61(v7, v1.clone()) {
                f61(v8, v1)
            } else {
                false
            }
        }
        Adt0::C12(v9) => {
            f50(v1, Adt0::C12(v9))
        }
        Adt0::C13(r300, r301) => {
            let v10 = (*r300).clone();
            let v11 = (*r301).clone();
            f50(v1, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)))
        }
        Adt0::C14 => {
            true
        }
        Adt0::C15(_, _) => {
            true
        }
    }
}

pub fn f62(v0: Adt0, v1: Adt8, p306: Adt0) -> bool {
    let m305 = p306;
    {
        let mut m305 = m305;
        loop {
            let step = {
                let v2 = m305;
                let m302 = v2;
                match m302 {
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
                    Adt0::C15(r303, r304) => {
                        let v13 = (*r303).clone();
                        let v14 = (*r304).clone();
                        if f46(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f58(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m305 = next; }
                Err(done) => break done,
            }
        }
    }
}
