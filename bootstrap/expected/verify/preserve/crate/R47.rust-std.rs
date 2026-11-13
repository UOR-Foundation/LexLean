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
pub enum Fn0 {
    F20(Bytes),
    F24,
}

impl Fn0 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn0::F20(k0) => Ok(f20(k0.clone(), p0, p1)),
            Fn0::F24 => f24(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F23,
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, bool), p1: Bytes) -> R<(u64, bool)> {
        match self {
            Fn1::F23 => f23(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F26,
}

impl Fn2 {
    pub fn apply(&self, p0: (Option<Bytes>, bool), p1: Bytes) -> (Option<Bytes>, bool) {
        match self {
            Fn2::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F29(u64, Bytes),
    F30(Bytes, u64, Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F29(k0, k1) => f29(*k0, k1.clone(), p0),
            Fn3::F30(k0, k1, k2, k3) => f30(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F33(u8, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn4::F33(k0, k1) => f33(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F40(List<Bytes>),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn5::F40(k0) => f40(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F44,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F44 => f44(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>, v6: List<Bytes>) -> R<Result<bool, Bytes>> {
    if f1(v0.clone(), f2(Str::lit("lexlean/rust-package/1"))) {
        let v7: bool = if f3(v1.clone())? {
            if f4(f5(0u64)?, v1.clone())? {
                false
            } else {
                let a1 = f4(f6(0u64)?, v1.clone())?;
                bool_not(a1)
            }
        } else {
            false
        };
        if v7 {
            if f7(v2.clone())? {
                let v8: bool = f1(v3.clone(), f2(Str::lit("rust-std")));
                let v9: bool = f1(v3.clone(), f2(Str::lit("rust-core")));
                if if v8 {
                    true
                } else {
                    v9
                } {
                    if f8(v4.clone())? {
                        if f9(v4) {
                            f10(v9, v5, v6)
                        } else {
                            Ok(f11(f2(Str::lit("sources are strictly ascending"))))
                        }
                    } else {
                        Ok(f11(f2(Str::lit("every source is a lowercase SHA-256 hex identity"))))
                    }
                } else {
                    Ok(f11(f12(f13(v3), f2(Str::lit(" is not a Rust profile")))))
                }
            } else {
                Ok(f11(f12(f13(v2), f2(Str::lit(" is not a version of three canonical decimals each at most 18446744073709551615")))))
            }
        } else {
            Ok(f11(f12(f2(Str::lit("identifier collision: ")), f12(f13(v1), f2(Str::lit(" is not an available crate name"))))))
        }
    } else {
        Ok(f11(f12(f2(Str::lit("package manifest has spec ")), f12(f13(v0), f2(Str::lit(", expected `lexlean/rust-package/1`"))))))
    }
}

pub fn f1(v0: Bytes, v1: Bytes) -> bool {
    let a6 = {
        let a2 = v0;
        let a3 = v1;
        compare_bytes(a2, a3)
    };
    let a7 = {
        let a4 = Bytes::lit(&[]);
        let a5 = Bytes::lit(&[]);
        compare_bytes(a4, a5)
    };
    equal(a6, a7)
}

pub fn f2(v0: Str) -> Bytes {
    let a8 = v0;
    utf8_encode(a8)
}

pub fn f3(v0: Bytes) -> R<bool> {
    let v2: bool = {
        let m9 = f14(v0.clone(), 0u64);
        match m9 {
            None => {
                false
            }
            Some(v1) => {
                f15(v1, 97u64, 122u64)?
            }
        }
    };
    if v2 {
        if f16(v0.clone(), 2u64)? {
            let m10 = f17(v0.clone(), f2(Str::lit("__")))?;
            match m10 {
                None => {
                    if f18(v0.clone(), f2(Str::lit("_"))) {
                        Ok(false)
                    } else {
                        let a12 = {
                            let a11 = v0;
                            length_bytes(a11)
                        };
                        let a13 = 64u64;
                        Ok(nat_le(a12, a13))
                    }
                }
                Some(_) => {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f4(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f19(Fn0::F20(v1), false, v0)
}

pub fn f5(_: u64) -> R<List<Bytes>> {
    f21(f2(Str::lit("as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true try type unsafe use where while abstract become box do final macro override priv typeof unsized virtual yield union macro_rules gen raw safe")), 32u8)
}

pub fn f6(_: u64) -> R<List<Bytes>> {
    f21(f2(Str::lit("std core alloc proc_macro test harness")), 32u8)
}

pub fn f7(v0: Bytes) -> R<bool> {
    let v1: List<Bytes> = f21(v0, 46u8)?;
    let v2: (u64, bool) = f22(Fn1::F23, (0u64, true), v1)?;
    let a15 = {
        let (h14, _) = v2;
        h14
    };
    let a16 = 3u64;
    if nat_eq(a15, a16) {
        let (_, h17) = v2;
        Ok(h17)
    } else {
        Ok(false)
    }
}

pub fn f8(v0: List<Bytes>) -> R<bool> {
    f19(Fn0::F24, true, v0)
}

pub fn f9(v0: List<Bytes>) -> bool {
    let (_, h18) = f25(Fn2::F26, (None::<Bytes>, true), v0);
    h18
}

pub fn f10(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<bool, Bytes>> {
    let m19 = f27(v1.clone(), v2)?;
    match m19 {
        None => {
            if f9(v1.clone()) {
                if if v0 {
                    f4(v1, f2(Str::lit("allocation")))?
                } else {
                    false
                } {
                    Ok(f11(f2(Str::lit("hidden allocation: a rust-core package states the effect `allocation`"))))
                } else {
                    Ok(Ok::<bool, Bytes>({
                        let a20 = v0;
                        bool_not(a20)
                    }))
                }
            } else {
                Ok(f11(f2(Str::lit("effects are strictly ascending"))))
            }
        }
        Some(v3) => {
            Ok(f11(f12(f13(v3), f2(Str::lit(" is not a registered effect")))))
        }
    }
}

pub fn f11(v0: Bytes) -> Result<bool, Bytes> {
    Err::<bool, Bytes>(v0)
}

pub fn f12(v0: Bytes, v1: Bytes) -> Bytes {
    let a21 = v0;
    let a22 = v1;
    append_bytes(a21, a22)
}

pub fn f13(v0: Bytes) -> Bytes {
    f12(f2(Str::lit("`")), f12(v0, f2(Str::lit("`"))))
}

pub fn f14(v0: Bytes, v1: u64) -> Option<u8> {
    let a23 = v0;
    let a24 = v1;
    index_bytes(a23, a24)
}

pub fn f15(v0: u8, v1: u64, v2: u64) -> R<bool> {
    let v4: u64 = {
        let v3: u8 = v0;
        let a69 = {
            let a45 = {
                let a33 = {
                    let a27 = {
                        let a25 = v3;
                        let a26 = 128u8;
                        fixed_u8::bit_and(a25, a26)
                    };
                    let a28 = 128u8;
                    if equal(a27, a28) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a34 = {
                    let a31 = {
                        let a29 = v3;
                        let a30 = 64u8;
                        fixed_u8::bit_and(a29, a30)
                    };
                    let a32 = 64u8;
                    if equal(a31, a32) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a33, a34)?
            };
            let a46 = {
                let a43 = {
                    let a37 = {
                        let a35 = v3;
                        let a36 = 32u8;
                        fixed_u8::bit_and(a35, a36)
                    };
                    let a38 = 32u8;
                    if equal(a37, a38) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a44 = {
                    let a41 = {
                        let a39 = v3;
                        let a40 = 16u8;
                        fixed_u8::bit_and(a39, a40)
                    };
                    let a42 = 16u8;
                    if equal(a41, a42) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a43, a44)?
            };
            nat_add(a45, a46)?
        };
        let a70 = {
            let a67 = {
                let a55 = {
                    let a49 = {
                        let a47 = v3;
                        let a48 = 8u8;
                        fixed_u8::bit_and(a47, a48)
                    };
                    let a50 = 8u8;
                    if equal(a49, a50) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a56 = {
                    let a53 = {
                        let a51 = v3;
                        let a52 = 4u8;
                        fixed_u8::bit_and(a51, a52)
                    };
                    let a54 = 4u8;
                    if equal(a53, a54) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a55, a56)?
            };
            let a68 = {
                let a65 = {
                    let a59 = {
                        let a57 = v3;
                        let a58 = 2u8;
                        fixed_u8::bit_and(a57, a58)
                    };
                    let a60 = 2u8;
                    if equal(a59, a60) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a66 = {
                    let a63 = {
                        let a61 = v3;
                        let a62 = 1u8;
                        fixed_u8::bit_and(a61, a62)
                    };
                    let a64 = 1u8;
                    if equal(a63, a64) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a65, a66)?
            };
            nat_add(a67, a68)?
        };
        nat_add(a69, a70)?
    };
    let a71 = v1;
    let a72 = v4;
    if nat_le(a71, a72) {
        let a73 = v4;
        let a74 = v2;
        Ok(nat_le(a73, a74))
    } else {
        Ok(false)
    }
}

pub fn f16(v0: Bytes, v1: u64) -> R<bool> {
    let a78 = {
        let (h75, _) = f28(Fn3::F29(v1, v0.clone()), {
            let a76 = v0.clone();
            length_bytes(a76)
        }, 0u64)?;
        h75
    };
    let a79 = {
        let a77 = v0;
        length_bytes(a77)
    };
    Ok(nat_eq(a78, a79))
}

pub fn f17(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a80 = v0.clone();
        length_bytes(a80)
    };
    let v3: u64 = {
        let a81 = v1.clone();
        length_bytes(a81)
    };
    let v4: u64 = {
        let (h82, _) = f28(Fn3::F30(v1.clone(), v3, v0.clone(), v2), {
            let a83 = v2;
            let a84 = 1u64;
            nat_add(a83, a84)?
        }, 0u64)?;
        h82
    };
    if f31(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f18(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a85 = v0.clone();
        length_bytes(a85)
    };
    let v3: u64 = {
        let a86 = v1.clone();
        length_bytes(a86)
    };
    let a87 = v3;
    let a88 = v2;
    if nat_le(a87, a88) {
        let m94 = {
            let a91 = v0;
            let a92 = {
                let a89 = v2;
                let a90 = v3;
                nat_sub(a89, a90)
            };
            let a93 = v3;
            slice_bytes(a91, a92, a93)
        };
        match m94 {
            None => {
                false
            }
            Some(v4) => {
                let a95 = v4;
                let a96 = v1;
                equal(a95, a96)
            }
        }
    } else {
        false
    }
}

pub fn f19(v0: Fn0, p100: bool, p101: List<Bytes>) -> R<bool> {
    let m99 = (p100, p101);
    {
        let mut m99 = m99;
        loop {
            let step = {
                let (v1, v2) = m99;
                let m97 = v2;
                match m97.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c98: Fn0 = v0.clone();
                            c98.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m99 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    let v3: bool = f1(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f21(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a102 = v0.clone();
        length_bytes(a102)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h103, _) = f32(Fn4::F33(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h103
    };
    let v4: u64 = {
        let (h104, _) = {
            let (_, h105) = v3.clone();
            h105
        };
        h104
    };
    Ok(f34(List::cons(f35(v0, v4, {
        let a106 = v2;
        let a107 = v4;
        nat_sub(a106, a107)
    }), {
        let (_, h108) = {
            let (_, h109) = v3;
            h109
        };
        h108
    })))
}

pub fn f22(v0: Fn1, p113: (u64, bool), p114: List<Bytes>) -> R<(u64, bool)> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m110 = v2;
                match m110.uncons() {
                    None => {
                        Err::<((u64, bool), List<Bytes>), R<(u64, bool)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, bool), List<Bytes>), R<(u64, bool)>>(({
                            let c111: Fn1 = v0.clone();
                            c111.apply(v1, v3)?
                        }, v4))
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

pub fn f23(v0: (u64, bool), v1: Bytes) -> R<(u64, bool)> {
    let v2: bool = f36(v1)?;
    Ok(({
        let a116 = {
            let (h115, _) = v0;
            h115
        };
        let a117 = 1u64;
        nat_add(a116, a117)?
    }, {
        let (_, h118) = v0;
        if h118 {
            v2
        } else {
            false
        }
    }))
}

pub fn f24(v0: bool, v1: Bytes) -> R<bool> {
    let v2: bool = f37(v1)?;
    if v0 {
        Ok(v2)
    } else {
        Ok(false)
    }
}

pub fn f25(v0: Fn2, p122: (Option<Bytes>, bool), p123: List<Bytes>) -> (Option<Bytes>, bool) {
    let m121 = (p122, p123);
    {
        let mut m121 = m121;
        loop {
            let step = {
                let (v1, v2) = m121;
                let m119 = v2;
                match m119.uncons() {
                    None => {
                        Err::<((Option<Bytes>, bool), List<Bytes>), (Option<Bytes>, bool)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, bool), List<Bytes>), (Option<Bytes>, bool)>(({
                            let c120: Fn2 = v0.clone();
                            c120.apply(v1, v3)
                        }, v4))
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

pub fn f26(v0: (Option<Bytes>, bool), v1: Bytes) -> (Option<Bytes>, bool) {
    f38(v0, v1)
}

pub fn f27(v0: List<Bytes>, v1: List<Bytes>) -> R<Option<Bytes>> {
    f39(Fn5::F40(v1), None::<Bytes>, v0)
}

pub fn f28(v0: Fn3, p128: u64, p129: u64) -> R<(u64, bool)> {
    let m127 = (p128, p129);
    {
        let mut m127 = m127;
        loop {
            let step = {
                let (v1, v2) = m127;
                let m124 = v1;
                if m124 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m124 - 1;
                    let m126 = {
                        let c125: Fn3 = v0.clone();
                        c125.apply(v2)?
                    };
                    match m126 {
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
                Ok(next) => { m127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f41(v1, v0, v2)
}

pub fn f30(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f31(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a132 = {
            let a130 = v4;
            let a131 = v1;
            nat_add(a130, a131)?
        };
        let a133 = v3;
        if nat_lt(a132, a133) {
            Ok(Some({
                let a134 = v4;
                let a135 = 1u64;
                nat_add(a134, a135)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f31(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m140 = {
        let a137 = v0;
        let a138 = v2;
        let a139 = {
            let a136 = v1.clone();
            length_bytes(a136)
        };
        slice_bytes(a137, a138, a139)
    };
    match m140 {
        None => {
            false
        }
        Some(v3) => {
            let a141 = v3;
            let a142 = v1;
            equal(a141, a142)
        }
    }
}

pub fn f32(v0: Fn4, p147: u64, p148: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m146 = (p147, p148);
    {
        let mut m146 = m146;
        loop {
            let step = {
                let (v1, v2) = m146;
                let m143 = v1;
                if m143 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m143 - 1;
                    let m145 = {
                        let c144: Fn4 = v0.clone();
                        c144.apply(v2.clone())?
                    };
                    match m145 {
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
                Ok(next) => { m146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f42(v1, v0, v2)
}

pub fn f34(v0: List<Bytes>) -> List<Bytes> {
    f43(Fn6::F44, List::<Bytes>::nil(), v0)
}

pub fn f35(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m152 = {
        let a149 = v0;
        let a150 = v1;
        let a151 = v2;
        slice_bytes(a149, a150, a151)
    };
    match m152 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f36(v0: Bytes) -> R<bool> {
    let m153 = f14(v0.clone(), 0u64);
    match m153 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f16(v0.clone(), 0u64)? {
                let a154 = v1;
                let a155 = 48u8;
                if if equal(a154, a155) {
                    let a157 = 1u64;
                    let a158 = {
                        let a156 = v0.clone();
                        length_bytes(a156)
                    };
                    nat_lt(a157, a158)
                } else {
                    false
                } {
                    Ok(false)
                } else {
                    Ok(f45(v0))
                }
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f37(v0: Bytes) -> R<bool> {
    let a160 = {
        let a159 = v0.clone();
        length_bytes(a159)
    };
    let a161 = 64u64;
    if nat_eq(a160, a161) {
        f16(v0, 1u64)
    } else {
        Ok(false)
    }
}

pub fn f38(v0: (Option<Bytes>, bool), v1: Bytes) -> (Option<Bytes>, bool) {
    let m163 = {
        let (h162, _) = v0;
        h162
    };
    match m163 {
        None => {
            (Some(v1), {
                let (_, h164) = v0;
                h164
            })
        }
        Some(v2) => {
            let v3: bool = f46(v2, v1.clone());
            (Some(v1), {
                let (_, h165) = v0;
                if h165 {
                    v3
                } else {
                    false
                }
            })
        }
    }
}

pub fn f39(v0: Fn5, p169: Option<Bytes>, p170: List<Bytes>) -> R<Option<Bytes>> {
    let m168 = (p169, p170);
    {
        let mut m168 = m168;
        loop {
            let step = {
                let (v1, v2) = m168;
                let m166 = v2;
                match m166.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c167: Fn5 = v0.clone();
                            c167.apply(v1, v3)?
                        }, v4))
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

pub fn f40(v0: List<Bytes>, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    f47(v0, v1, v2)
}

pub fn f41(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m171 = f14(v0, v2);
    match m171 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            if f48(v1, v3)? {
                Ok(Some({
                    let a172 = v2;
                    let a173 = 1u64;
                    nat_add(a172, a173)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h174, _) = v2.clone();
        h174
    };
    let v4: u64 = {
        let (h175, _) = {
            let (_, h176) = v2.clone();
            h176
        };
        h175
    };
    let v5: List<Bytes> = {
        let (_, h177) = {
            let (_, h178) = v2;
            h178
        };
        h177
    };
    let m179 = f14(v0.clone(), v3);
    match m179 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a180 = v6;
            let a181 = v1;
            if equal(a180, a181) {
                Ok(Some(({
                    let a182 = v3;
                    let a183 = 1u64;
                    nat_add(a182, a183)?
                }, ({
                    let a184 = v3;
                    let a185 = 1u64;
                    nat_add(a184, a185)?
                }, List::cons(f35(v0, v4, {
                    let a186 = v3;
                    let a187 = v4;
                    nat_sub(a186, a187)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a188 = v3;
                    let a189 = 1u64;
                    nat_add(a188, a189)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f43(v0: Fn6, p193: List<Bytes>, p194: List<Bytes>) -> List<Bytes> {
    let m192 = (p193, p194);
    {
        let mut m192 = m192;
        loop {
            let step = {
                let (v1, v2) = m192;
                let m190 = v2;
                match m190.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c191: Fn6 = v0.clone();
                            c191.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m192 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f45(v0: Bytes) -> bool {
    let a196 = {
        let a195 = v0.clone();
        length_bytes(a195)
    };
    let a197 = 20u64;
    if nat_lt(a196, a197) {
        true
    } else {
        let a199 = {
            let a198 = v0.clone();
            length_bytes(a198)
        };
        let a200 = 20u64;
        if nat_eq(a199, a200) {
            let a201 = f46(f2(Str::lit("18446744073709551615")), v0);
            bool_not(a201)
        } else {
            false
        }
    }
}

pub fn f46(v0: Bytes, v1: Bytes) -> bool {
    let a206 = {
        let a202 = v0;
        let a203 = v1;
        compare_bytes(a202, a203)
    };
    let a207 = {
        let a204 = Bytes::lit(&[]);
        let a205 = Bytes::lit(&[0x01]);
        compare_bytes(a204, a205)
    };
    equal(a206, a207)
}

pub fn f47(v0: List<Bytes>, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m208 = v1.clone();
    match m208 {
        None => {
            if f4(v0, v2.clone())? {
                Ok(v1)
            } else {
                Ok(Some(v2))
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f48(v0: u64, v1: u8) -> R<bool> {
    let v2: bool = f15(v1, 48u64, 57u64)?;
    let a209 = v0;
    let a210 = 0u64;
    if nat_eq(a209, a210) {
        Ok(v2)
    } else {
        let v3: bool = f15(v1, 97u64, 102u64)?;
        let a211 = v0;
        let a212 = 1u64;
        if nat_eq(a211, a212) {
            if v2 {
                Ok(true)
            } else {
                Ok(v3)
            }
        } else {
            let v4: bool = f15(v1, 97u64, 122u64)?;
            let v5: bool = {
                let a213 = v1;
                let a214 = 95u8;
                equal(a213, a214)
            };
            let v6: bool = if v4 {
                true
            } else {
                v5
            };
            if v2 {
                Ok(true)
            } else {
                Ok(v6)
            }
        }
    }
}
