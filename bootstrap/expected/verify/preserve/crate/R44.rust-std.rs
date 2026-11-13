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
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt0, Adt9),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(Adt0, u64, Adt0, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(bool, List<u64>, Adt0, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(bool, List<u64>, Adt0),
}

#[derive(Clone)]
pub enum Fn0 {
    F6(Adt10, Adt8),
}

impl Fn0 {
    pub fn apply(&self, p0: List<bool>) -> R<Option<List<bool>>> {
        match self {
            Fn0::F6(k0, k1) => f6(k0.clone(), k1.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: List<bool>) -> R<List<bool>> {
    let v8: Adt7 = Adt7::C0(v0, v1.clone(), v2, v3.clone(), v4.clone(), v5.clone());
    let v9: u64 = f1(v5.clone())?;
    let v10: Adt0 = f2(f2(f2(Adt0::C14, v3), v4), v1);
    let v11: Adt8 = f3(v8.clone(), Adt8::C0(v10, Adt9::C0), v5.clone(), 0u64, v7, v6.clone())?;
    let v12: Adt10 = f4(v8, v5, 0u64, v6)?;
    let (h1, _) = f5(Fn0::F6(v12, v11), {
        let a2 = 1u64;
        let a3 = v9;
        nat_add(a2, a3)?
    }, f7(v9))?;
    Ok(h1)
}

pub fn f1(v0: Adt2) -> R<u64> {
    let m4 = v0;
    match m4 {
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
        Adt2::C14(_, r7) => {
            let v33 = (*r7).clone();
            let v34: u64 = f1(v33)?;
            let a5 = 1u64;
            let a6 = v34;
            nat_add(a5, a6)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f2(v0: Adt0, v1: Adt0) -> Adt0 {
    let m8 = v1;
    match m8 {
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
        Adt0::C8(r9) => {
            let v3 = (*r9).clone();
            f2(v0, v3)
        }
        Adt0::C9(r10, r11) => {
            let v4 = (*r10).clone();
            let v5 = (*r11).clone();
            f2(f2(v0, v4), v5)
        }
        Adt0::C10(r12) => {
            let v6 = (*r12).clone();
            f2(v0, v6)
        }
        Adt0::C11(r13, r14) => {
            let v7 = (*r13).clone();
            let v8 = (*r14).clone();
            f2(f2(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r15, r16) => {
            let v10 = (*r15).clone();
            let v11 = (*r16).clone();
            let v12: Adt0 = f8(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f2(f2(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r17, r18) => {
            let v13 = (*r17).clone();
            let v14 = (*r18).clone();
            f2(f2(v0, v13), v14)
        }
    }
}

pub fn f3(v0: Adt7, p29: Adt8, p30: Adt2, p31: u64, v4: List<bool>, p32: Adt0) -> R<Adt8> {
    let m28 = (p29, (p30, (p31, p32)));
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m28;
                let m19 = v2;
                match m19 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C14(r26, r27) => {
                        let v34 = (*r26).clone();
                        let v35 = (*r27).clone();
                        let v37: bool = {
                            let m20 = f9(v4.clone(), v3);
                            match m20 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a21 = v36;
                                    bool_not(a21)
                                }
                            }
                        };
                        let v38: Adt8 = Adt8::C0(f10(v0.clone(), match v1.clone() {
                            Adt8::C0(h22, _) => {
                                h22
                            }
                        }, v3), match v1 {
                            Adt8::C0(_, h23) => {
                                h23
                            }
                        });
                        let v39: Adt8 = if v37 {
                            f11(v0.clone(), v38, v34, f12(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>((v39, (v35, ({
                            let a24 = v3;
                            let a25 = 1u64;
                            nat_add(a24, a25)?
                        }, f13(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
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

pub fn f4(v0: Adt7, v1: Adt2, v2: u64, v3: Adt0) -> R<Adt10> {
    let m33 = v1;
    match m33 {
        Adt2::C0(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C1(_) => {
            Ok(Adt10::C0)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C3(_, _, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C4(_, _, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C5(_, _, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C6(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C7(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C8(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C9(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C10(_) => {
            Ok(Adt10::C0)
        }
        Adt2::C11(_) => {
            Ok(Adt10::C0)
        }
        Adt2::C12(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C13 => {
            Ok(Adt10::C0)
        }
        Adt2::C14(r39, r40) => {
            let v32 = (*r39).clone();
            let v33 = (*r40).clone();
            let v34: Adt11 = f14(v0.clone(), Adt11::C0(false, List::<u64>::nil(), Adt0::C14), v32, f12(v3.clone()));
            Ok(Adt10::C1(match v34.clone() {
                Adt11::C0(h34, _, _) => {
                    h34
                }
            }, match v34.clone() {
                Adt11::C0(_, h35, _) => {
                    h35
                }
            }, match v34 {
                Adt11::C0(_, _, h36) => {
                    h36
                }
            }, std::rc::Rc::new(f4(v0, v33, {
                let a37 = v2;
                let a38 = 1u64;
                nat_add(a37, a38)?
            }, f13(v3))?)))
        }
        Adt2::C15(_, _, _) => {
            Ok(Adt10::C0)
        }
    }
}

pub fn f5(v0: Fn0, p45: u64, p46: List<bool>) -> R<(List<bool>, bool)> {
    let m44 = (p45, p46);
    {
        let mut m44 = m44;
        loop {
            let step = {
                let (v1, v2) = m44;
                let m41 = v1;
                if m41 == 0 {
                    Err::<(u64, List<bool>), R<(List<bool>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m41 - 1;
                    let m43 = {
                        let c42: Fn0 = v0.clone();
                        c42.apply(v2.clone())?
                    };
                    match m43 {
                        None => {
                            Err::<(u64, List<bool>), R<(List<bool>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<bool>), R<(List<bool>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m44 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Adt10, v1: Adt8, v2: List<bool>) -> R<Option<List<bool>>> {
    f15(v0, match v1 {
        Adt8::C0(_, h47) => {
            h47
        }
    }, v2)
}

pub fn f7(v0: u64) -> List<bool> {
    let m48 = v0;
    if m48 == 0 {
        List::<bool>::nil()
    } else {
        let v1 = m48 - 1;
        List::cons(false, f7(v1))
    }
}

pub fn f8(v0: Adt0, v1: Adt0) -> Adt0 {
    if f16(v0.clone(), v1.clone()) {
        v0
    } else {
        f17(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f9(p52: List<bool>, p53: u64) -> Option<bool> {
    let m51 = (p52, p53);
    {
        let mut m51 = m51;
        loop {
            let step = {
                let (v0, v1) = m51;
                let m49 = v0;
                match m49.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m50 = v1;
                        if m50 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m50 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m51 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Adt7, v1: Adt0, v2: u64) -> Adt0 {
    f2(f18(v1, f19(v0.clone(), v2)), f20(v0, v2))
}

pub fn f11(v0: Adt7, v1: Adt8, v2: Adt2, v3: Adt0) -> R<Adt8> {
    let m54 = v2;
    match m54 {
        Adt2::C0(v4, _) => {
            Ok(Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h55, _) => {
                    h55
                }
            }, v4), match v1 {
                Adt8::C0(_, h56) => {
                    h56
                }
            }))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, v8, r65, r66) => {
            let v9 = (*r65).clone();
            let v10 = (*r66).clone();
            let v11: Adt8 = Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h57, _) => {
                    h57
                }
            }, v8), match v1 {
                Adt8::C0(_, h58) => {
                    h58
                }
            });
            let v12: Adt8 = f11(v0.clone(), v11, v9, f12(v3.clone()))?;
            f11(v0, v12, v10, f13(v3))
        }
        Adt2::C3(r67, r68, r69) => {
            let v13 = (*r67).clone();
            let v14 = (*r68).clone();
            let v15 = (*r69).clone();
            let v16: Adt8 = f11(v0.clone(), v1, v13, f12(v3.clone()))?;
            let v17: Adt8 = f11(v0.clone(), v16, v14, f13(v3.clone()))?;
            f11(v0, v17, v15, f21(v3))
        }
        Adt2::C4(v18, r70, r71) => {
            let v19 = (*r70).clone();
            let v20 = (*r71).clone();
            let v21: Adt8 = Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h59, _) => {
                    h59
                }
            }, v18), match v1 {
                Adt8::C0(_, h60) => {
                    h60
                }
            });
            let v22: Adt8 = f11(v0.clone(), v21, v19, f12(v3.clone()))?;
            f11(v0, v22, v20, f13(v3))
        }
        Adt2::C5(_, v24, r72) => {
            let v25 = (*r72).clone();
            let v26: Adt8 = Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h61, _) => {
                    h61
                }
            }, v24), match v1 {
                Adt8::C0(_, h62) => {
                    h62
                }
            });
            f11(v0, v26, v25, f12(v3))
        }
        Adt2::C6(_, r73) => {
            let v28 = (*r73).clone();
            f11(v0, v1, v28, f12(v3))
        }
        Adt2::C7(v29, r74) => {
            let v30 = (*r74).clone();
            let v31: Adt0 = f19(v0.clone(), v29);
            let v32: u64 = f1(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f22(v31.clone(), v32)), std::rc::Rc::new(f20(v0.clone(), v29)));
            let v34: Adt8 = Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h63, _) => {
                    h63
                }
            }, v33.clone()), f23(match v1 {
                Adt8::C0(_, h64) => {
                    h64
                }
            }, v33, v29, f24(v31, v32)));
            f11(v0, v34, v30, f12(v3))
        }
        Adt2::C8(r75, r76) => {
            let v35 = (*r75).clone();
            let v36 = (*r76).clone();
            let v37: Adt8 = f11(v0.clone(), v1, v35, f12(v3.clone()))?;
            f11(v0, v37, v36, f13(v3))
        }
        Adt2::C9(_, r77) => {
            let v39 = (*r77).clone();
            f11(v0, v1, v39, f12(v3))
        }
        Adt2::C10(r78) => {
            let v40 = (*r78).clone();
            f11(v0, v1, v40, f12(v3))
        }
        Adt2::C11(r79) => {
            let v41 = (*r79).clone();
            f11(v0, v1, v41, f12(v3))
        }
        Adt2::C12(r80, _) => {
            let v42 = (*r80).clone();
            f11(v0, v1, v42, f12(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r81, r82) => {
            let v44 = (*r81).clone();
            let v45 = (*r82).clone();
            let v46: Adt8 = f11(v0.clone(), v1, v44, f12(v3.clone()))?;
            f11(v0, v46, v45, f13(v3))
        }
        Adt2::C15(_, _, r83) => {
            let v49 = (*r83).clone();
            f11(v0, v1, v49, f12(v3))
        }
    }
}

pub fn f12(v0: Adt0) -> Adt0 {
    f25(f26(v0))
}

pub fn f13(v0: Adt0) -> Adt0 {
    f25(f26(f26(v0)))
}

pub fn f14(v0: Adt7, v1: Adt11, v2: Adt2, v3: Adt0) -> Adt11 {
    let m84 = v2;
    match m84 {
        Adt2::C0(_, _) => {
            v1
        }
        Adt2::C1(_) => {
            v1
        }
        Adt2::C2(_, _, r99, r100) => {
            let v9 = (*r99).clone();
            let v10 = (*r100).clone();
            let v11: Adt11 = f14(v0.clone(), v1, v9, f12(v3.clone()));
            f14(v0, v11, v10, f13(v3))
        }
        Adt2::C3(r101, r102, r103) => {
            let v12 = (*r101).clone();
            let v13 = (*r102).clone();
            let v14 = (*r103).clone();
            let v15: Adt11 = f14(v0.clone(), v1, v12, f12(v3.clone()));
            let v16: Adt11 = f14(v0.clone(), v15, v13, f13(v3.clone()));
            f14(v0, v16, v14, f21(v3))
        }
        Adt2::C4(_, r104, r105) => {
            let v18 = (*r104).clone();
            let v19 = (*r105).clone();
            let v20: Adt11 = f14(v0.clone(), v1, v18, f12(v3.clone()));
            f14(v0, v20, v19, f13(v3))
        }
        Adt2::C5(v21, _, r106) => {
            let v23 = (*r106).clone();
            let v24: Adt11 = f14(v0, v1, v23, f12(v3));
            let m85 = v21;
            match m85 {
                Adt5::C0 => {
                    v24
                }
                Adt5::C1 => {
                    v24
                }
                Adt5::C2 => {
                    v24
                }
                Adt5::C3 => {
                    v24
                }
                Adt5::C4 => {
                    v24
                }
                Adt5::C5 => {
                    v24
                }
                Adt5::C6 => {
                    v24
                }
                Adt5::C7 => {
                    Adt11::C0(true, match v24.clone() {
                        Adt11::C0(_, h86, _) => {
                            h86
                        }
                    }, match v24 {
                        Adt11::C0(_, _, h87) => {
                            h87
                        }
                    })
                }
                Adt5::C8 => {
                    v24
                }
                Adt5::C9 => {
                    v24
                }
                Adt5::C10 => {
                    v24
                }
                Adt5::C11 => {
                    v24
                }
                Adt5::C12 => {
                    v24
                }
                Adt5::C13 => {
                    v24
                }
                Adt5::C14 => {
                    v24
                }
                Adt5::C15(_) => {
                    v24
                }
            }
        }
        Adt2::C6(v26, r107) => {
            let v27 = (*r107).clone();
            let v28: Adt11 = f14(v0, v1, v27, f12(v3));
            Adt11::C0(match v28.clone() {
                Adt11::C0(h88, _, _) => {
                    h88
                }
            }, if f27(match v28.clone() {
                Adt11::C0(_, h89, _) => {
                    h89
                }
            }, v26) {
                match v28.clone() {
                    Adt11::C0(_, h90, _) => {
                        h90
                    }
                }
            } else {
                List::cons(v26, match v28.clone() {
                    Adt11::C0(_, h91, _) => {
                        h91
                    }
                })
            }, match v28 {
                Adt11::C0(_, _, h92) => {
                    h92
                }
            })
        }
        Adt2::C7(_, r108) => {
            let v30 = (*r108).clone();
            f14(v0, v1, v30, f12(v3))
        }
        Adt2::C8(r109, r110) => {
            let v31 = (*r109).clone();
            let v32 = (*r110).clone();
            let v33: Adt11 = f14(v0.clone(), v1, v31, f12(v3.clone()));
            let v34: Adt11 = f14(v0, v33, v32, f13(v3.clone()));
            Adt11::C0(match v34.clone() {
                Adt11::C0(h93, _, _) => {
                    h93
                }
            }, match v34.clone() {
                Adt11::C0(_, h94, _) => {
                    h94
                }
            }, f8(match v34 {
                Adt11::C0(_, _, h95) => {
                    h95
                }
            }, f28(v3)))
        }
        Adt2::C9(v35, r111) => {
            let v36 = (*r111).clone();
            let v37: Adt11 = f14(v0, v1, v36, f12(v3));
            Adt11::C0(if match v37.clone() {
                Adt11::C0(h96, _, _) => {
                    h96
                }
            } {
                true
            } else {
                f29(v35)
            }, match v37.clone() {
                Adt11::C0(_, h97, _) => {
                    h97
                }
            }, match v37 {
                Adt11::C0(_, _, h98) => {
                    h98
                }
            })
        }
        Adt2::C10(r112) => {
            let v38 = (*r112).clone();
            f14(v0, v1, v38, f12(v3))
        }
        Adt2::C11(r113) => {
            let v39 = (*r113).clone();
            f14(v0, v1, v39, f12(v3))
        }
        Adt2::C12(r114, _) => {
            let v40 = (*r114).clone();
            f14(v0, v1, v40, f12(v3))
        }
        Adt2::C13 => {
            v1
        }
        Adt2::C14(r115, r116) => {
            let v42 = (*r115).clone();
            let v43 = (*r116).clone();
            let v44: Adt11 = f14(v0.clone(), v1, v42, f12(v3.clone()));
            f14(v0, v44, v43, f13(v3))
        }
        Adt2::C15(_, _, r117) => {
            let v47 = (*r117).clone();
            f14(v0, v1, v47, f12(v3))
        }
    }
}

pub fn f15(v0: Adt10, v1: Adt9, v2: List<bool>) -> R<Option<List<bool>>> {
    let v3: List<bool> = f30(v0, v1, v2.clone(), 0u64)?;
    if f31(v3.clone(), v2) {
        Ok(None::<List<bool>>)
    } else {
        Ok(Some(v3))
    }
}

pub fn f16(p122: Adt0, v1: Adt0) -> bool {
    let m121 = p122;
    {
        let mut m121 = m121;
        loop {
            let step = {
                let v0 = m121;
                let m118 = v0;
                match m118 {
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
                    Adt0::C15(r119, r120) => {
                        let v12 = (*r119).clone();
                        let v13 = (*r120).clone();
                        if f32(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Adt0, v1: Adt0) -> Adt0 {
    let m123 = v0;
    match m123 {
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
        Adt0::C15(r124, r125) => {
            let v12 = (*r124).clone();
            let v13 = (*r125).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f17(v13, v1)))
        }
    }
}

pub fn f18(p130: Adt0, p131: Adt0) -> Adt0 {
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
                        Ok::<(Adt0, Adt0), Adt0>((f2(v0, v2), v3))
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

pub fn f19(v0: Adt7, v1: u64) -> Adt0 {
    let m132 = f33(v0, v1);
    match m132 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f20(v0: Adt7, v1: u64) -> Adt0 {
    let m134 = f34(match v0 {
        Adt7::C0(_, _, _, _, h133, _) => {
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

pub fn f21(v0: Adt0) -> Adt0 {
    f25(f26(f26(f26(v0))))
}

pub fn f22(p140: Adt0, p141: u64) -> Adt0 {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v0, v1) = m139;
                let m135 = v0;
                match m135 {
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
                    Adt0::C15(r137, r138) => {
                        let v12 = (*r137).clone();
                        let v13 = (*r138).clone();
                        let m136 = v1;
                        if m136 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m136 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Adt9, v1: Adt0, v2: u64, v3: Adt0) -> Adt9 {
    if f35(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt9::C1(v1, v2, v3, std::rc::Rc::new(v0))
    }
}

pub fn f24(v0: Adt0, v1: u64) -> Adt0 {
    let m142 = v0;
    match m142 {
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
        Adt0::C15(r144, r145) => {
            let v12 = (*r144).clone();
            let v13 = (*r145).clone();
            let m143 = v1;
            if m143 == 0 {
                Adt0::C14
            } else {
                let v14 = m143 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f24(v13, v14)))
            }
        }
    }
}

pub fn f25(v0: Adt0) -> Adt0 {
    let m146 = f36(v0);
    match m146 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f26(v0: Adt0) -> Adt0 {
    let m147 = f37(v0);
    match m147 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f27(p152: List<u64>, v1: u64) -> bool {
    let m151 = p152;
    {
        let mut m151 = m151;
        loop {
            let step = {
                let v0 = m151;
                let m148 = v0;
                match m148.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a149 = v2;
                        let a150 = v1;
                        if nat_eq(a149, a150) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
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

pub fn f28(v0: Adt0) -> Adt0 {
    f25(v0)
}

pub fn f29(v0: Adt6) -> bool {
    let m153 = v0;
    match m153 {
        Adt6::C0 => {
            true
        }
        Adt6::C1 => {
            false
        }
        Adt6::C2 => {
            true
        }
        Adt6::C3 => {
            false
        }
        Adt6::C4 => {
            false
        }
        Adt6::C5 => {
            false
        }
        Adt6::C6 => {
            false
        }
        Adt6::C7 => {
            false
        }
        Adt6::C8 => {
            true
        }
        Adt6::C9 => {
            true
        }
        Adt6::C10 => {
            true
        }
        Adt6::C11 => {
            true
        }
        Adt6::C12 => {
            true
        }
        Adt6::C13 => {
            false
        }
        Adt6::C14 => {
            false
        }
        Adt6::C15 => {
            false
        }
        Adt6::C16 => {
            false
        }
        Adt6::C17 => {
            false
        }
        Adt6::C18 => {
            false
        }
        Adt6::C19 => {
            false
        }
        Adt6::C20 => {
            false
        }
        Adt6::C21 => {
            false
        }
        Adt6::C22 => {
            false
        }
        Adt6::C23 => {
            false
        }
        Adt6::C24 => {
            false
        }
        Adt6::C25 => {
            false
        }
        Adt6::C26 => {
            false
        }
        Adt6::C27 => {
            false
        }
        Adt6::C28 => {
            false
        }
        Adt6::C29 => {
            false
        }
        Adt6::C30 => {
            false
        }
        Adt6::C31 => {
            false
        }
        Adt6::C32 => {
            false
        }
        Adt6::C33 => {
            false
        }
        Adt6::C34 => {
            false
        }
        Adt6::C35 => {
            false
        }
        Adt6::C36 => {
            false
        }
        Adt6::C37 => {
            false
        }
        Adt6::C38 => {
            false
        }
        Adt6::C39 => {
            false
        }
        Adt6::C40(_) => {
            false
        }
        Adt6::C41(v2) => {
            let a154 = f38(v2);
            let a155 = 3u64;
            nat_eq(a154, a155)
        }
    }
}

pub fn f30(v0: Adt10, v1: Adt9, v2: List<bool>, v3: u64) -> R<List<bool>> {
    let m156 = v0;
    match m156 {
        Adt10::C0 => {
            Ok(List::<bool>::nil())
        }
        Adt10::C1(v4, v5, v6, r159) => {
            let v7 = (*r159).clone();
            Ok(List::cons(if if v4 {
                true
            } else {
                if f39(v2.clone(), v3) {
                    true
                } else {
                    f40(v5, v2.clone())
                }
            } {
                true
            } else {
                f41(v6, v1.clone(), v2.clone())
            }, f30(v7, v1, v2, {
                let a157 = v3;
                let a158 = 1u64;
                nat_add(a157, a158)?
            })?))
        }
    }
}

pub fn f31(p166: List<bool>, p167: List<bool>) -> bool {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v0, v1) = m165;
                let m160 = v0;
                match m160.uncons() {
                    None => {
                        let m161 = v1;
                        match m161.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m162 = v1;
                        match m162.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a163 = v4;
                                let a164 = v6;
                                if equal(a163, a164) {
                                    Ok::<(List<bool>, List<bool>), bool>((v5, v7))
                                } else {
                                    Err::<(List<bool>, List<bool>), bool>(false)
                                }
                            }
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

pub fn f32(v0: Adt0, v1: Adt0) -> bool {
    let m168 = v0;
    match m168 {
        Adt0::C0 => {
            let a169 = f38(v1);
            let a170 = 0u64;
            nat_eq(a169, a170)
        }
        Adt0::C1 => {
            let a171 = f38(v1);
            let a172 = 1u64;
            nat_eq(a171, a172)
        }
        Adt0::C2 => {
            let a173 = f38(v1);
            let a174 = 2u64;
            nat_eq(a173, a174)
        }
        Adt0::C3 => {
            let a175 = f38(v1);
            let a176 = 3u64;
            nat_eq(a175, a176)
        }
        Adt0::C4(v2) => {
            let a185 = f38(v1.clone());
            let a186 = 4u64;
            if nat_eq(a185, a186) {
                let a187 = f42(v2);
                let a188 = f42(f43(v1));
                nat_eq(a187, a188)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a177 = f38(v1);
            let a178 = 5u64;
            nat_eq(a177, a178)
        }
        Adt0::C6 => {
            let a179 = f38(v1);
            let a180 = 6u64;
            nat_eq(a179, a180)
        }
        Adt0::C7 => {
            let a181 = f38(v1);
            let a182 = 7u64;
            nat_eq(a181, a182)
        }
        Adt0::C8(r205) => {
            let v3 = (*r205).clone();
            let v4: bool = f32(v3, f25(v1.clone()));
            let a189 = f38(v1);
            let a190 = 8u64;
            if nat_eq(a189, a190) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r206, r207) => {
            let v5 = (*r206).clone();
            let v6 = (*r207).clone();
            let v7: bool = f32(v5, f25(v1.clone()));
            let v8: bool = f32(v6, f26(v1.clone()));
            let a191 = f38(v1);
            let a192 = 9u64;
            if nat_eq(a191, a192) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r208) => {
            let v9 = (*r208).clone();
            let v10: bool = f32(v9, f25(v1.clone()));
            let a193 = f38(v1);
            let a194 = 10u64;
            if nat_eq(a193, a194) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r209, r210) => {
            let v11 = (*r209).clone();
            let v12 = (*r210).clone();
            let v13: bool = f32(v11, f25(v1.clone()));
            let v14: bool = f32(v12, f26(v1.clone()));
            let a195 = f38(v1);
            let a196 = 11u64;
            if nat_eq(a195, a196) {
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
            let a197 = f38(v1.clone());
            let a198 = 12u64;
            if nat_eq(a197, a198) {
                let a199 = v15;
                let a200 = f44(v1);
                nat_eq(a199, a200)
            } else {
                false
            }
        }
        Adt0::C13(r211, r212) => {
            let v16 = (*r211).clone();
            let v17 = (*r212).clone();
            let v18: bool = f32(v16, f25(v1.clone()));
            let v19: bool = f32(v17, f26(v1.clone()));
            let a201 = f38(v1);
            let a202 = 13u64;
            if nat_eq(a201, a202) {
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
            let a183 = f38(v1);
            let a184 = 14u64;
            nat_eq(a183, a184)
        }
        Adt0::C15(r213, r214) => {
            let v20 = (*r213).clone();
            let v21 = (*r214).clone();
            let v22: bool = f32(v20, f25(v1.clone()));
            let v23: bool = f32(v21, f26(v1.clone()));
            let a203 = f38(v1);
            let a204 = 15u64;
            if nat_eq(a203, a204) {
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

pub fn f33(v0: Adt7, v1: u64) -> Option<Adt0> {
    f34(match v0 {
        Adt7::C0(_, _, _, h215, _, _) => {
            h215
        }
    }, v1)
}

pub fn f34(p221: Adt0, p222: u64) -> Option<Adt0> {
    let m220 = (p221, p222);
    {
        let mut m220 = m220;
        loop {
            let step = {
                let (v0, v1) = m220;
                let m216 = v0;
                match m216 {
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
                    Adt0::C15(r218, r219) => {
                        let v12 = (*r218).clone();
                        let v13 = (*r219).clone();
                        let m217 = v1;
                        if m217 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m217 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
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

pub fn f35(p228: Adt9, v1: Adt0, v2: u64) -> bool {
    let m227 = p228;
    {
        let mut m227 = m227;
        loop {
            let step = {
                let v0 = m227;
                let m223 = v0;
                match m223 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(v3, v4, _, r226) => {
                        let v6 = (*r226).clone();
                        let a224 = v4;
                        let a225 = v2;
                        if nat_eq(a224, a225) {
                            if f32(v3, v1.clone()) {
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
                Ok(next) => { m227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Adt0) -> Option<Adt0> {
    let m229 = v0;
    match m229 {
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
        Adt0::C8(r230) => {
            let v2 = (*r230).clone();
            Some(v2)
        }
        Adt0::C9(r231, _) => {
            let v3 = (*r231).clone();
            Some(v3)
        }
        Adt0::C10(r232) => {
            let v5 = (*r232).clone();
            Some(v5)
        }
        Adt0::C11(r233, _) => {
            let v6 = (*r233).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r234, _) => {
            let v9 = (*r234).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r235, _) => {
            let v11 = (*r235).clone();
            Some(v11)
        }
    }
}

pub fn f37(v0: Adt0) -> Option<Adt0> {
    let m236 = v0;
    match m236 {
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
        Adt0::C9(_, r237) => {
            let v4 = (*r237).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r238) => {
            let v7 = (*r238).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r239) => {
            let v10 = (*r239).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r240) => {
            let v12 = (*r240).clone();
            Some(v12)
        }
    }
}

pub fn f38(v0: Adt0) -> u64 {
    let m241 = v0;
    match m241 {
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

pub fn f39(v0: List<bool>, v1: u64) -> bool {
    let m242 = f9(v0, v1);
    match m242 {
        None => {
            false
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40(p245: List<u64>, v1: List<bool>) -> bool {
    let m244 = p245;
    {
        let mut m244 = m244;
        loop {
            let step = {
                let v0 = m244;
                let m243 = v0;
                match m243.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f39(v1.clone(), v2) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(p250: Adt0, v1: Adt9, v2: List<bool>) -> bool {
    let m249 = p250;
    {
        let mut m249 = m249;
        loop {
            let step = {
                let v0 = m249;
                let m246 = v0;
                match m246 {
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
                    Adt0::C15(r247, r248) => {
                        let v13 = (*r247).clone();
                        let v14 = (*r248).clone();
                        if f45(v1.clone(), v13, v2.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v14)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Adt1) -> u64 {
    let m251 = v0;
    match m251 {
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

pub fn f43(v0: Adt0) -> Adt1 {
    let m252 = v0;
    match m252 {
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

pub fn f44(v0: Adt0) -> u64 {
    let m253 = v0;
    match m253 {
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

pub fn f45(p257: Adt9, v1: Adt0, v2: List<bool>) -> bool {
    let m256 = p257;
    {
        let mut m256 = m256;
        loop {
            let step = {
                let v0 = m256;
                let m254 = v0;
                match m254 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(v3, v4, _, r255) => {
                        let v6 = (*r255).clone();
                        if f32(v3, v1.clone()) {
                            if f39(v2.clone(), v4) {
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
                Ok(next) => { m256 = next; }
                Err(done) => break done,
            }
        }
    }
}
