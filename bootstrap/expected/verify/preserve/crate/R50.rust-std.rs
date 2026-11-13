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
    C0(bool, Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2, Adt8, Adt9),
    C1(u64, Adt2, Adt2, bool, Adt10, Adt9),
    C2(Adt12, Adt13, Adt2, Adt17, Adt9),
    C3,
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3),
    C1(Adt4),
    C2(Adt5, std::rc::Rc<Adt2>),
    C3(Adt6, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt7, u64),
    C5,
    C6(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
    C8,
}

#[derive(Clone)]
pub enum Adt4 {
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
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(u64, Adt2, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(u64, List<Adt11>, bool, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt12 {
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
pub enum Adt13 {
    C0,
    C1(Adt14, Adt2, std::rc::Rc<Adt13>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt15),
    C1(Adt12),
    C2(std::rc::Rc<Adt14>),
    C3(Adt16, std::rc::Rc<Adt14>),
    C4(u64, u64, std::rc::Rc<Adt14>),
    C5,
    C6(std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18, Adt9),
    C1(Adt19, Adt12, Adt9),
    C2(Adt20, std::rc::Rc<Adt17>, Adt9),
    C3(Adt21, std::rc::Rc<Adt17>, bool, Adt9),
    C4(Adt12, std::rc::Rc<Adt17>, bool, Adt9),
    C5(Adt23, std::rc::Rc<Adt17>, Adt9),
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C7(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C8(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C9(std::rc::Rc<Adt17>),
    C10(Adt24, Adt12, Adt9),
    C11(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C12,
    C13(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C14(Adt14, bool, Adt2, std::rc::Rc<Adt17>, Adt9, std::rc::Rc<Adt17>),
    C15(Adt14, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C16(Adt12, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt4),
    C5,
    C6,
    C7,
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
    C2(u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt2),
    C1,
    C2(Adt2, Adt2),
    C3(Adt2, Adt2),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt2),
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
    C0(u64, Bytes, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<(u64, List<(u64, u64)>)>, List<(u64, List<(u64, u64)>)>, List<(u64, bool)>, List<(u64, bool)>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(bool, Adt26, bool, List<Adt25>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(List<Adt12>, List<Adt12>, List<Adt12>, List<(Adt12, List<u64>)>, bool),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Bytes, Bytes, Option<Adt4>, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(List<Adt29>, List<Bytes>, List<Adt25>),
}

#[derive(Clone)]
pub enum Fn0 {
    F35(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<List<(u64, u64)>>, p1: (u64, List<(u64, u64)>)) -> Option<List<(u64, u64)>> {
        match self {
            Fn0::F35(k0) => f35(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F37(u64),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<bool>, p1: (u64, bool)) -> Option<bool> {
        match self {
            Fn1::F37(k0) => f37(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F39(Bytes),
    F183(Bytes, u64, Bytes, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F39(k0) => f39(k0.clone(), p0),
            Fn2::F183(k0, k1, k2, k3) => f183(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F56,
}

impl Fn3 {
    pub fn apply(&self, p0: bool, p1: Adt11) -> bool {
        match self {
            Fn3::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F91(Adt12),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Adt12) -> bool {
        match self {
            Fn4::F91(k0) => f91(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F112,
    F116(Adt12),
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn5::F112 => f112(p0, p1),
            Fn5::F116(k0) => f116(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F114,
    F164(Adt12),
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Adt12, List<u64>)>, p1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
        match self {
            Fn6::F114 => f114(p0, p1),
            Fn6::F164(k0) => f164(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F118(List<Adt12>, List<Adt12>, Adt12),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn7::F118(k0, k1, k2) => f118(k0.clone(), k1.clone(), k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F120(List<(Adt12, List<u64>)>, List<Adt12>),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Adt12>, p1: (Adt12, List<u64>)) -> Option<Adt12> {
        match self {
            Fn8::F120(k0, k1) => f120(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F122(Adt12),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<u64>, p1: (Adt12, u64)) -> Option<u64> {
        match self {
            Fn9::F122(k0) => f122(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F126(u64),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn10::F126(k0) => f126(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F129(Adt12),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<List<u64>>, p1: (Adt12, List<u64>)) -> Option<List<u64>> {
        match self {
            Fn11::F129(k0) => f129(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F133,
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt29) -> Option<Bytes> {
        match self {
            Fn12::F133 => f133(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F153(u64),
}

impl Fn13 {
    pub fn apply(&self, p0: Option<Adt25>, p1: Adt25) -> Option<Adt25> {
        match self {
            Fn13::F153(k0) => f153(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F163(u64),
    F188(List<u64>),
}

impl Fn14 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn14::F163(k0) => f163(*k0, p0, p1),
            Fn14::F188(k0) => f188(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F177,
}

impl Fn15 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn15::F177 => f177(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F182(Adt29),
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt29) -> bool {
        match self {
            Fn16::F182(k0) => f182(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F192(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (bool, List<Bytes>), p1: Bytes) -> (bool, List<Bytes>) {
        match self {
            Fn17::F192(k0) => f192(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F196(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn18::F196(k0) => f196(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F199,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn19::F199 => f199(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<(), Bytes>> {
    let v1: List<Adt25> = List::cons(Adt25::C0(0u64, f1(Str::lit("nat_add")), 0u64, true, false), List::cons(Adt25::C0(1u64, f1(Str::lit("nat_sub")), 0u64, false, false), List::cons(Adt25::C0(2u64, f1(Str::lit("nat_mul")), 0u64, true, false), List::cons(Adt25::C0(3u64, f1(Str::lit("nat_quot")), 0u64, false, false), List::cons(Adt25::C0(4u64, f1(Str::lit("nat_rem")), 0u64, false, false), List::cons(Adt25::C0(5u64, f1(Str::lit("nat_eq")), 0u64, false, false), List::cons(Adt25::C0(6u64, f1(Str::lit("nat_le")), 0u64, false, false), List::cons(Adt25::C0(7u64, f1(Str::lit("nat_lt")), 0u64, false, false), List::cons(Adt25::C0(8u64, f1(Str::lit("nat_succ")), 0u64, true, false), List::cons(Adt25::C0(9u64, f1(Str::lit("int_add")), 0u64, true, false), List::<Adt25>::nil()))))))))));
    let v2: List<Adt25> = List::cons(Adt25::C0(10u64, f1(Str::lit("int_sub")), 0u64, true, false), List::cons(Adt25::C0(11u64, f1(Str::lit("int_mul")), 0u64, true, false), List::cons(Adt25::C0(12u64, f1(Str::lit("int_neg")), 0u64, true, false), List::cons(Adt25::C0(13u64, f1(Str::lit("int_quot")), 0u64, true, false), List::cons(Adt25::C0(14u64, f1(Str::lit("int_rem")), 0u64, false, false), List::cons(Adt25::C0(15u64, f1(Str::lit("bool_not")), 0u64, false, false), List::cons(Adt25::C0(16u64, f1(Str::lit("bool_and")), 0u64, false, false), List::cons(Adt25::C0(17u64, f1(Str::lit("bool_or")), 0u64, false, false), List::cons(Adt25::C0(18u64, f1(Str::lit("equal")), 0u64, false, false), List::cons(Adt25::C0(19u64, f1(Str::lit("compare")), 0u64, false, false), v1))))))))));
    let v3: List<Adt25> = List::cons(Adt25::C0(20u64, f1(Str::lit("checked_add")), 1u64, false, false), List::cons(Adt25::C0(21u64, f1(Str::lit("checked_sub")), 1u64, false, false), List::cons(Adt25::C0(22u64, f1(Str::lit("checked_mul")), 1u64, false, false), List::cons(Adt25::C0(23u64, f1(Str::lit("checked_quot")), 1u64, false, false), List::cons(Adt25::C0(24u64, f1(Str::lit("checked_neg_")), 2u64, false, false), List::cons(Adt25::C0(25u64, f1(Str::lit("bit_and")), 1u64, false, false), List::cons(Adt25::C0(26u64, f1(Str::lit("bit_or")), 1u64, false, false), List::cons(Adt25::C0(27u64, f1(Str::lit("bit_xor")), 1u64, false, false), List::cons(Adt25::C0(28u64, f1(Str::lit("bit_not")), 1u64, false, false), List::cons(Adt25::C0(29u64, f1(Str::lit("shift_left")), 1u64, false, false), v2))))))))));
    let v4: List<Adt25> = List::cons(Adt25::C0(30u64, f1(Str::lit("shift_right")), 1u64, false, false), List::cons(Adt25::C0(31u64, f1(Str::lit("convert")), 1u64, false, false), List::cons(Adt25::C0(32u64, f1(Str::lit("append_list")), 0u64, false, true), List::cons(Adt25::C0(33u64, f1(Str::lit("append_bytes")), 0u64, false, true), List::cons(Adt25::C0(34u64, f1(Str::lit("length_list")), 0u64, false, true), List::cons(Adt25::C0(35u64, f1(Str::lit("length_bytes")), 0u64, false, true), List::cons(Adt25::C0(36u64, f1(Str::lit("length_string")), 0u64, false, true), List::cons(Adt25::C0(37u64, f1(Str::lit("index_list")), 0u64, false, true), List::cons(Adt25::C0(38u64, f1(Str::lit("index_bytes")), 0u64, false, true), List::cons(Adt25::C0(39u64, f1(Str::lit("slice_list")), 0u64, false, true), v3))))))))));
    let v5: List<Adt25> = List::cons(Adt25::C0(40u64, f1(Str::lit("slice_bytes")), 0u64, false, true), List::cons(Adt25::C0(41u64, f1(Str::lit("utf8_encode")), 0u64, false, true), List::cons(Adt25::C0(42u64, f1(Str::lit("utf8_decode")), 0u64, false, true), List::cons(Adt25::C0(43u64, f1(Str::lit("compare_bytes")), 0u64, false, true), List::cons(Adt25::C0(44u64, f1(Str::lit("split_exact")), 0u64, false, true), List::cons(Adt25::C0(45u64, f1(Str::lit("join")), 0u64, false, true), List::cons(Adt25::C0(46u64, f1(Str::lit("format_int")), 0u64, false, true), List::cons(Adt25::C0(47u64, f1(Str::lit("format_")), 2u64, false, true), List::cons(Adt25::C0(48u64, f1(Str::lit("parse_int")), 0u64, true, true), List::cons(Adt25::C0(49u64, f1(Str::lit("parse_")), 2u64, false, true), v4))))))))));
    let m2 = f2(Adt26::C0(List::<(u64, List<(u64, u64)>)>::nil(), List::<(u64, List<(u64, u64)>)>::nil(), List::<(u64, bool)>::nil(), List::<(u64, bool)>::nil()), match v0.clone() {
        Adt0::C0(_, h1) => {
            h1
        }
    })?;
    match m2 {
        Ok(v6) => {
            f3(match v0.clone() {
                Adt0::C0(h3, _) => {
                    h3
                }
            }, v6, v5, match v0 {
                Adt0::C0(_, h4) => {
                    h4
                }
            })
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f1(v0: Str) -> Bytes {
    let a5 = v0;
    utf8_encode(a5)
}

pub fn f2(p38: Adt26, p39: Adt1) -> R<Result<Adt26, Bytes>> {
    let m37 = (p38, p39);
    {
        let mut m37 = m37;
        loop {
            let step = {
                let (v0, v1) = m37;
                let m6 = v1;
                match m6 {
                    Adt1::C0(_, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C3 => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C4(r35, r36) => {
                        let v16 = (*r35).clone();
                        let v17 = (*r36).clone();
                        let m7 = v16;
                        match m7 {
                            Adt1::C0(v18, v19, _) => {
                                let m8 = v18.clone();
                                match m8 {
                                    Adt2::C0(_) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C1(_) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C2(_, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C3(_, _, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C4(v28, v29) => {
                                        let m9 = f6(v28);
                                        if m9 {
                                            let m11 = f7(match v0.clone() {
                                                Adt26::C0(_, h10, _, _) => {
                                                    h10
                                                }
                                            }, v29);
                                            if m11 {
                                                Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f5(v18)?, f1(Str::lit(" is declared twice")))))))
                                            } else {
                                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                                    Adt26::C0(h12, _, _, _) => {
                                                        h12
                                                    }
                                                }, List::cons((v29, f8(List::<(u64, u64)>::nil(), v19)?), match v0.clone() {
                                                    Adt26::C0(_, h13, _, _) => {
                                                        h13
                                                    }
                                                }), match v0.clone() {
                                                    Adt26::C0(_, _, h14, _) => {
                                                        h14
                                                    }
                                                }, match v0 {
                                                    Adt26::C0(_, _, _, h15) => {
                                                        h15
                                                    }
                                                }), v17))
                                            }
                                        } else {
                                            let m17 = f7(match v0.clone() {
                                                Adt26::C0(h16, _, _, _) => {
                                                    h16
                                                }
                                            }, v29);
                                            if m17 {
                                                Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f5(v18)?, f1(Str::lit(" is declared twice")))))))
                                            } else {
                                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(List::cons((v29, f8(List::<(u64, u64)>::nil(), v19)?), match v0.clone() {
                                                    Adt26::C0(h18, _, _, _) => {
                                                        h18
                                                    }
                                                }), match v0.clone() {
                                                    Adt26::C0(_, h19, _, _) => {
                                                        h19
                                                    }
                                                }, match v0.clone() {
                                                    Adt26::C0(_, _, h20, _) => {
                                                        h20
                                                    }
                                                }, match v0 {
                                                    Adt26::C0(_, _, _, h21) => {
                                                        h21
                                                    }
                                                }), v17))
                                            }
                                        }
                                    }
                                    Adt2::C5 => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C6(_, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                }
                            }
                            Adt1::C1(v32, _, _, v35, _, _) => {
                                let m23 = f9(match v0.clone() {
                                    Adt26::C0(_, _, h22, _) => {
                                        h22
                                    }
                                }, v32);
                                if m23 {
                                    Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: Fn")), f4(f10(v32)?, f1(Str::lit("::apply is declared twice")))))))
                                } else {
                                    Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                        Adt26::C0(h24, _, _, _) => {
                                            h24
                                        }
                                    }, match v0.clone() {
                                        Adt26::C0(_, h25, _, _) => {
                                            h25
                                        }
                                    }, List::cons((v32, v35), match v0.clone() {
                                        Adt26::C0(_, _, h26, _) => {
                                            h26
                                        }
                                    }), match v0 {
                                        Adt26::C0(_, _, _, h27) => {
                                            h27
                                        }
                                    }), v17))
                                }
                            }
                            Adt1::C2(v38, _, v40, _, _) => {
                                let m28 = v38;
                                match m28 {
                                    Adt12::C0(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C1(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C2(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C3(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C4(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C5(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C6(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C7(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C8(v51) => {
                                        let m30 = f9(match v0.clone() {
                                            Adt26::C0(_, _, _, h29) => {
                                                h29
                                            }
                                        }, v51);
                                        if m30 {
                                            Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: f")), f4(f10(v51)?, f1(Str::lit(" is declared twice")))))))
                                        } else {
                                            Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                                Adt26::C0(h31, _, _, _) => {
                                                    h31
                                                }
                                            }, match v0.clone() {
                                                Adt26::C0(_, h32, _, _) => {
                                                    h32
                                                }
                                            }, match v0.clone() {
                                                Adt26::C0(_, _, h33, _) => {
                                                    h33
                                                }
                                            }, List::cons((v51, f11(v40)), match v0 {
                                                Adt26::C0(_, _, _, h34) => {
                                                    h34
                                                }
                                            })), v17))
                                        }
                                    }
                                    Adt12::C9(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                }
                            }
                            Adt1::C3 => {
                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                            }
                            Adt1::C4(_, _) => {
                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m37 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: bool, v1: Adt26, v2: List<Adt25>, p45: Adt1) -> R<Result<(), Bytes>> {
    let m44 = p45;
    {
        let mut m44 = m44;
        loop {
            let step = {
                let v3 = m44;
                let m40 = v3;
                match m40 {
                    Adt1::C0(_, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C3 => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C4(r42, r43) => {
                        let v18 = (*r42).clone();
                        let v19 = (*r43).clone();
                        let m41 = f12(v0, v1.clone(), v2.clone(), v18)?;
                        match m41 {
                            Ok(_) => {
                                Ok::<Adt1, R<Result<(), Bytes>>>(v19)
                            }
                            Err(v21) => {
                                Err::<Adt1, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v21)))
                            }
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

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a46 = v0;
    let a47 = v1;
    append_bytes(a46, a47)
}

pub fn f5(v0: Adt2) -> R<Bytes> {
    let m48 = v0;
    match m48 {
        Adt2::C0(v1) => {
            Ok(f13(v1))
        }
        Adt2::C1(v2) => {
            Ok(f4(f1(Str::lit("Fixed(")), f4(f14(v2), f1(Str::lit(")")))))
        }
        Adt2::C2(v3, r49) => {
            let v4 = (*r49).clone();
            Ok(f4(f15(v3), f4(f1(Str::lit("(")), f4(f5(v4)?, f1(Str::lit(")"))))))
        }
        Adt2::C3(v5, r50, r51) => {
            let v6 = (*r50).clone();
            let v7 = (*r51).clone();
            Ok(f4(f16(v5), f4(f1(Str::lit("(")), f4(f5(v6)?, f4(f1(Str::lit(", ")), f4(f5(v7)?, f1(Str::lit(")"))))))))
        }
        Adt2::C4(v8, v9) => {
            Ok(f4(f17(v8), f4(f1(Str::lit("(")), f4(f10(v9)?, f1(Str::lit(")"))))))
        }
        Adt2::C5 => {
            Ok(f1(Str::lit("unreachable")))
        }
        Adt2::C6(_, _) => {
            Ok(f1(Str::lit("unreachable")))
        }
    }
}

pub fn f6(v0: Adt7) -> bool {
    let m52 = v0;
    match m52 {
        Adt7::C0 => {
            false
        }
        Adt7::C1 => {
            true
        }
        Adt7::C2 => {
            false
        }
    }
}

pub fn f7(v0: List<(u64, List<(u64, u64)>)>, v1: u64) -> bool {
    let m53 = f18(v0, v1);
    match m53 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f8(p57: List<(u64, u64)>, p58: Adt8) -> R<List<(u64, u64)>> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v0, v1) = m56;
                let m54 = v1;
                match m54 {
                    Adt8::C0 => {
                        Err::<(List<(u64, u64)>, Adt8), R<List<(u64, u64)>>>(Ok(v0))
                    }
                    Adt8::C1(v2, v3, r55) => {
                        let v4 = (*r55).clone();
                        Ok::<(List<(u64, u64)>, Adt8), R<List<(u64, u64)>>>((List::cons((v2, f19(v3)?), v0), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: List<(u64, bool)>, v1: u64) -> bool {
    let m59 = f20(v0, v1);
    match m59 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f10(v0: u64) -> R<Bytes> {
    let v1: Bytes = f1(Str::lit("0"));
    let v2: Bytes = f21({
        let a96 = f22(v0, 10000000000000000000u64);
        let a97 = {
            let a94 = f22(v0, 1000000000000000000u64);
            let a95 = {
                let a92 = f22(v0, 100000000000000000u64);
                let a93 = {
                    let a90 = f22(v0, 10000000000000000u64);
                    let a91 = {
                        let a88 = f22(v0, 1000000000000000u64);
                        let a89 = {
                            let a86 = f22(v0, 100000000000000u64);
                            let a87 = {
                                let a84 = f22(v0, 10000000000000u64);
                                let a85 = {
                                    let a82 = f22(v0, 1000000000000u64);
                                    let a83 = {
                                        let a80 = f22(v0, 100000000000u64);
                                        let a81 = {
                                            let a78 = f22(v0, 10000000000u64);
                                            let a79 = {
                                                let a76 = f22(v0, 1000000000u64);
                                                let a77 = {
                                                    let a74 = f22(v0, 100000000u64);
                                                    let a75 = {
                                                        let a72 = f22(v0, 10000000u64);
                                                        let a73 = {
                                                            let a70 = f22(v0, 1000000u64);
                                                            let a71 = {
                                                                let a68 = f22(v0, 100000u64);
                                                                let a69 = {
                                                                    let a66 = f22(v0, 10000u64);
                                                                    let a67 = {
                                                                        let a64 = f22(v0, 1000u64);
                                                                        let a65 = {
                                                                            let a62 = f22(v0, 100u64);
                                                                            let a63 = {
                                                                                let a60 = f22(v0, 10u64);
                                                                                let a61 = f22(v0, 1u64);
                                                                                append_bytes(a60, a61)
                                                                            };
                                                                            append_bytes(a62, a63)
                                                                        };
                                                                        append_bytes(a64, a65)
                                                                    };
                                                                    append_bytes(a66, a67)
                                                                };
                                                                append_bytes(a68, a69)
                                                            };
                                                            append_bytes(a70, a71)
                                                        };
                                                        append_bytes(a72, a73)
                                                    };
                                                    append_bytes(a74, a75)
                                                };
                                                append_bytes(a76, a77)
                                            };
                                            append_bytes(a78, a79)
                                        };
                                        append_bytes(a80, a81)
                                    };
                                    append_bytes(a82, a83)
                                };
                                append_bytes(a84, a85)
                            };
                            append_bytes(a86, a87)
                        };
                        append_bytes(a88, a89)
                    };
                    append_bytes(a90, a91)
                };
                append_bytes(a92, a93)
            };
            append_bytes(a94, a95)
        };
        append_bytes(a96, a97)
    })?;
    let a98 = v0;
    let a99 = 0u64;
    if nat_eq(a98, a99) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f11(v0: Adt2) -> bool {
    let m100 = v0;
    match m100 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(v3, _) => {
            f23(v3)
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(_, _) => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f12(v0: bool, v1: Adt26, v2: List<Adt25>, v3: Adt1) -> R<Result<(), Bytes>> {
    let m101 = v3;
    match m101 {
        Adt1::C0(_, v5, _) => {
            f24(Adt27::C0(v0, v1, false, v2), v5)
        }
        Adt1::C1(v7, v8, v9, v10, v11, _) => {
            let v13: Adt27 = Adt27::C0(v0, v1, false, v2);
            let m102 = f25(v13.clone(), v8)?;
            match m102 {
                Ok(_) => {
                    let m103 = f26(v13.clone(), v9)?;
                    match m103 {
                        Ok(_) => {
                            f27(v13, v7, v10, v11)
                        }
                        Err(v16) => {
                            Ok(Err::<(), Bytes>(v16))
                        }
                    }
                }
                Err(v17) => {
                    Ok(Err::<(), Bytes>(v17))
                }
            }
        }
        Adt1::C2(v18, v19, v20, v21, _) => {
            let v23: bool = f11(v20.clone());
            let v24: Adt27 = Adt27::C0(v0, v1.clone(), v23, v2);
            let m104 = f26(v24.clone(), v20)?;
            match m104 {
                Ok(_) => {
                    let m105 = f28(v24.clone(), Adt28::C0(List::<Adt12>::nil(), List::<Adt12>::nil(), List::<Adt12>::nil(), List::<(Adt12, List<u64>)>::nil(), false), v19.clone())?;
                    match m105 {
                        Ok(v26) => {
                            let m106 = f29(v26, v24, v21.clone(), true)?;
                            match m106 {
                                Ok(v27) => {
                                    let m109 = if match v27 {
                                        Adt28::C0(_, _, _, _, h107) => {
                                            h107
                                        }
                                    } {
                                        let a108 = v23;
                                        bool_not(a108)
                                    } else {
                                        false
                                    };
                                    if m109 {
                                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: ")), f4(f30(f31(v18)?), f1(Str::lit(" propagates a failure its result type does not carry"))))))
                                    } else {
                                        let m110 = f32(f33(List::<(Adt12, u64)>::nil(), v19), v1, v21, v23)?;
                                        match m110 {
                                            Ok(_) => {
                                                Ok(Ok::<(), Bytes>(()))
                                            }
                                            Err(v29) => {
                                                Ok(Err::<(), Bytes>(v29))
                                            }
                                        }
                                    }
                                }
                                Err(v30) => {
                                    Ok(Err::<(), Bytes>(v30))
                                }
                            }
                        }
                        Err(v31) => {
                            Ok(Err::<(), Bytes>(v31))
                        }
                    }
                }
                Err(v32) => {
                    Ok(Err::<(), Bytes>(v32))
                }
            }
        }
        Adt1::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt1::C4(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f13(v0: Adt3) -> Bytes {
    let m111 = v0;
    match m111 {
        Adt3::C0 => {
            f1(Str::lit("Unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("Bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("Nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("Int"))
        }
        Adt3::C4 => {
            f1(Str::lit("Ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("Str"))
        }
        Adt3::C6 => {
            f1(Str::lit("Bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("Overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("I128"))
        }
    }
}

pub fn f14(v0: Adt4) -> Bytes {
    let m112 = v0;
    match m112 {
        Adt4::C0 => {
            f1(Str::lit("U8"))
        }
        Adt4::C1 => {
            f1(Str::lit("U16"))
        }
        Adt4::C2 => {
            f1(Str::lit("U32"))
        }
        Adt4::C3 => {
            f1(Str::lit("U64"))
        }
        Adt4::C4 => {
            f1(Str::lit("I8"))
        }
        Adt4::C5 => {
            f1(Str::lit("I16"))
        }
        Adt4::C6 => {
            f1(Str::lit("I32"))
        }
        Adt4::C7 => {
            f1(Str::lit("I64"))
        }
    }
}

pub fn f15(v0: Adt5) -> Bytes {
    let m113 = v0;
    match m113 {
        Adt5::C0 => {
            f1(Str::lit("Option"))
        }
        Adt5::C1 => {
            f1(Str::lit("List"))
        }
        Adt5::C2 => {
            f1(Str::lit("Rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("Fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("Ref"))
        }
    }
}

pub fn f16(v0: Adt6) -> Bytes {
    let m114 = v0;
    match m114 {
        Adt6::C0 => {
            f1(Str::lit("Result"))
        }
        Adt6::C1 => {
            f1(Str::lit("Pair"))
        }
    }
}

pub fn f17(v0: Adt7) -> Bytes {
    let m115 = v0;
    match m115 {
        Adt7::C0 => {
            f1(Str::lit("Adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("Fn"))
        }
        Adt7::C2 => {
            f1(Str::lit("Param"))
        }
    }
}

pub fn f18(v0: List<(u64, List<(u64, u64)>)>, v1: u64) -> Option<List<(u64, u64)>> {
    f34(Fn0::F35(v1), None::<List<(u64, u64)>>, v0)
}

pub fn f19(v0: Adt2) -> R<u64> {
    let m116 = v0;
    match m116 {
        Adt2::C0(_) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _) => {
            Ok(0u64)
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(_, r119) => {
            let v11 = (*r119).clone();
            let v12: u64 = f19(v11)?;
            let a117 = 1u64;
            let a118 = v12;
            nat_add(a117, a118)
        }
    }
}

pub fn f20(v0: List<(u64, bool)>, v1: u64) -> Option<bool> {
    f36(Fn1::F37(v1), None::<bool>, v0)
}

pub fn f21(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h120, _) = f38(Fn2::F39(v0.clone()), {
            let a121 = v0.clone();
            length_bytes(a121)
        }, 0u64)?;
        h120
    };
    Ok(f40(v0.clone(), v1, {
        let a123 = {
            let a122 = v0;
            length_bytes(a122)
        };
        let a124 = v1;
        nat_sub(a123, a124)
    }))
}

pub fn f22(v0: u64, v1: u64) -> Bytes {
    f40(f1(Str::lit("0123456789")), {
        let a128 = {
            let a125 = v0;
            let a126 = v1;
            let a127 = 0u64;
            nat_quot(a125, a126, a127)
        };
        let a129 = 10u64;
        let a130 = 0u64;
        nat_rem(a128, a129, a130)
    }, 1u64)
}

pub fn f23(v0: Adt5) -> bool {
    let m131 = v0;
    match m131 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            false
        }
        Adt5::C2 => {
            false
        }
        Adt5::C3 => {
            true
        }
        Adt5::C4 => {
            false
        }
    }
}

pub fn f24(v0: Adt27, p136: Adt8) -> R<Result<(), Bytes>> {
    let m135 = p136;
    {
        let mut m135 = m135;
        loop {
            let step = {
                let v1 = m135;
                let m132 = v1;
                match m132 {
                    Adt8::C0 => {
                        Err::<Adt8, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt8::C1(_, v3, r134) => {
                        let v4 = (*r134).clone();
                        let m133 = f25(v0.clone(), v3)?;
                        match m133 {
                            Ok(_) => {
                                Ok::<Adt8, R<Result<(), Bytes>>>(v4)
                            }
                            Err(v6) => {
                                Err::<Adt8, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt27, p142: Adt2) -> R<Result<(), Bytes>> {
    let m141 = p142;
    {
        let mut m141 = m141;
        loop {
            let step = {
                let v1 = m141;
                let m137 = v1;
                match m137 {
                    Adt2::C0(_) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C1(_) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C2(_, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C4(_, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C5 => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C6(r139, r140) => {
                        let v11 = (*r139).clone();
                        let v12 = (*r140).clone();
                        let m138 = f26(v0.clone(), v11)?;
                        match m138 {
                            Ok(_) => {
                                Ok::<Adt2, R<Result<(), Bytes>>>(v12)
                            }
                            Err(v14) => {
                                Err::<Adt2, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v14)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Adt27, v1: Adt2) -> R<Result<(), Bytes>> {
    let v2: bool = f41(v1.clone());
    let v3: bool = if match v0.clone() {
        Adt27::C0(h143, _, _, _) => {
            h143
        }
    } {
        v2
    } else {
        false
    };
    let m144 = v3;
    if m144 {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders the heap type ")), f4(f5(v1)?, f1(Str::lit(", which rust-core does not provide"))))))
    } else {
        f42(v0, v1)
    }
}

pub fn f27(v0: Adt27, v1: u64, v2: bool, p156: Adt10) -> R<Result<(), Bytes>> {
    let m155 = p156;
    {
        let mut m155 = m155;
        loop {
            let step = {
                let v3 = m155;
                let m145 = v3;
                match m145 {
                    Adt10::C0 => {
                        Err::<Adt10, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt10::C1(v4, v5, v6, r154) => {
                        let v7 = (*r154).clone();
                        let m149 = f20({
                            let m148 = match v0.clone() {
                                Adt27::C0(_, h147, _, _) => {
                                    h147
                                }
                            };
                            match m148 {
                                Adt26::C0(_, _, _, h146) => {
                                    h146
                                }
                            }
                        }, v4);
                        match m149 {
                            None => {
                                Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v1)?, f4(f1(Str::lit(" dispatches to undeclared f")), f10(v4)?))))))
                            }
                            Some(v8) => {
                                let v9: bool = if v8 {
                                    !v6
                                } else {
                                    v6
                                };
                                let v10: bool = if v6 {
                                    !v2
                                } else {
                                    false
                                };
                                let m150 = v9;
                                if m150 {
                                    Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: Fn")), f4(f10(v1)?, f4(f1(Str::lit("::apply misstates the failure of f")), f10(v4)?))))))
                                } else {
                                    let m151 = v10;
                                    if m151 {
                                        Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: Fn")), f4(f10(v1)?, f4(f1(Str::lit("::apply misstates the failure of f")), f10(v4)?))))))
                                    } else {
                                        let m152 = f43(v5);
                                        if m152 {
                                            let m153 = f44(v0.clone(), f1(Str::lit("a boxed capture")));
                                            match m153 {
                                                Ok(_) => {
                                                    Ok::<Adt10, R<Result<(), Bytes>>>(v7)
                                                }
                                                Err(v12) => {
                                                    Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v12)))
                                                }
                                            }
                                        } else {
                                            Ok::<Adt10, R<Result<(), Bytes>>>(v7)
                                        }
                                    }
                                }
                            }
                        }
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

pub fn f28(v0: Adt27, p163: Adt28, p164: Adt13) -> R<Result<Adt28, Bytes>> {
    let m162 = (p163, p164);
    {
        let mut m162 = m162;
        loop {
            let step = {
                let (v1, v2) = m162;
                let m157 = v2;
                match m157 {
                    Adt13::C0 => {
                        Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v1)))
                    }
                    Adt13::C1(v3, v4, r161) => {
                        let v5 = (*r161).clone();
                        let m158 = f26(v0.clone(), v4)?;
                        match m158 {
                            Ok(_) => {
                                let m160 = f45(v1, match v0.clone() {
                                    Adt27::C0(_, h159, _, _) => {
                                        h159
                                    }
                                }, v3)?;
                                match m160 {
                                    Ok(v7) => {
                                        Ok::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>((v7, v5))
                                    }
                                    Err(v8) => {
                                        Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v8)))
                                    }
                                }
                            }
                            Err(v9) => {
                                Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v9)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt28, v1: Adt27, v2: Adt17, v3: bool) -> R<Result<Adt28, Bytes>> {
    let m165 = v2;
    match m165 {
        Adt17::C0(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C9(_) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C11(r173, r174) => {
            let v37 = (*r173).clone();
            let v38 = (*r174).clone();
            let v39: List<Adt12> = match v0.clone() {
                Adt28::C0(_, h166, _, _, _) => {
                    h166
                }
            };
            let m167 = f46(v0, v1.clone(), v37)?;
            match m167 {
                Ok(v40) => {
                    let m168 = f47(v40, v1, v38, v3)?;
                    match m168 {
                        Ok(v41) => {
                            Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v41.clone() {
                                Adt28::C0(h169, _, _, _, _) => {
                                    h169
                                }
                            }, v39, match v41.clone() {
                                Adt28::C0(_, _, h170, _, _) => {
                                    h170
                                }
                            }, match v41.clone() {
                                Adt28::C0(_, _, _, h171, _) => {
                                    h171
                                }
                            }, match v41 {
                                Adt28::C0(_, _, _, _, h172) => {
                                    h172
                                }
                            })))
                        }
                        Err(v42) => {
                            Ok(Err::<Adt28, Bytes>(v42))
                        }
                    }
                }
                Err(v43) => {
                    Ok(Err::<Adt28, Bytes>(v43))
                }
            }
        }
        Adt17::C12 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
    }
}

pub fn f30(v0: Bytes) -> Bytes {
    f4(f1(Str::lit("`")), f4(v0, f1(Str::lit("`"))))
}

pub fn f31(v0: Adt12) -> R<Bytes> {
    let m175 = v0;
    match m175 {
        Adt12::C0(v1) => {
            Ok(f4(f1(Str::lit("v")), f10(v1)?))
        }
        Adt12::C1(v2) => {
            Ok(f4(f1(Str::lit("m")), f10(v2)?))
        }
        Adt12::C2(v3) => {
            Ok(f4(f1(Str::lit("a")), f10(v3)?))
        }
        Adt12::C3(v4) => {
            Ok(f4(f1(Str::lit("c")), f10(v4)?))
        }
        Adt12::C4(v5) => {
            Ok(f4(f1(Str::lit("r")), f10(v5)?))
        }
        Adt12::C5(v6) => {
            Ok(f4(f1(Str::lit("h")), f10(v6)?))
        }
        Adt12::C6(v7) => {
            Ok(f4(f1(Str::lit("k")), f10(v7)?))
        }
        Adt12::C7(v8) => {
            Ok(f4(f1(Str::lit("p")), f10(v8)?))
        }
        Adt12::C8(v9) => {
            Ok(f4(f1(Str::lit("f")), f10(v9)?))
        }
        Adt12::C9(v10) => {
            Ok(v10)
        }
    }
}

pub fn f32(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m176 = v2;
    match m176 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C9(_) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C11(r178, r179) => {
            let v37 = (*r178).clone();
            let v38 = (*r179).clone();
            let m177 = f48(v0, v1.clone(), v37)?;
            match m177 {
                Ok(v39) => {
                    f49(v39, v1, v38, v3)
                }
                Err(v40) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v40))
                }
            }
        }
        Adt17::C12 => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
    }
}

pub fn f33(p183: List<(Adt12, u64)>, p184: Adt13) -> List<(Adt12, u64)> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v0, v1) = m182;
                let m180 = v1;
                match m180 {
                    Adt13::C0 => {
                        Err::<(List<(Adt12, u64)>, Adt13), List<(Adt12, u64)>>(v0)
                    }
                    Adt13::C1(v2, v3, r181) => {
                        let v4 = (*r181).clone();
                        Ok::<(List<(Adt12, u64)>, Adt13), List<(Adt12, u64)>>((f50(v0, v2, v3), v4))
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

pub fn f34(v0: Fn0, p188: Option<List<(u64, u64)>>, p189: List<(u64, List<(u64, u64)>)>) -> Option<List<(u64, u64)>> {
    let m187 = (p188, p189);
    {
        let mut m187 = m187;
        loop {
            let step = {
                let (v1, v2) = m187;
                let m185 = v2;
                match m185.uncons() {
                    None => {
                        Err::<(Option<List<(u64, u64)>>, List<(u64, List<(u64, u64)>)>), Option<List<(u64, u64)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<(u64, u64)>>, List<(u64, List<(u64, u64)>)>), Option<List<(u64, u64)>>>(({
                            let c186: Fn0 = v0.clone();
                            c186.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: u64, v1: Option<List<(u64, u64)>>, v2: (u64, List<(u64, u64)>)) -> Option<List<(u64, u64)>> {
    let m190 = v1;
    match m190 {
        None => {
            let a192 = {
                let (h191, _) = v2;
                h191
            };
            let a193 = v0;
            if nat_eq(a192, a193) {
                Some({
                    let (_, h194) = v2;
                    h194
                })
            } else {
                None::<List<(u64, u64)>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f36(v0: Fn1, p198: Option<bool>, p199: List<(u64, bool)>) -> Option<bool> {
    let m197 = (p198, p199);
    {
        let mut m197 = m197;
        loop {
            let step = {
                let (v1, v2) = m197;
                let m195 = v2;
                match m195.uncons() {
                    None => {
                        Err::<(Option<bool>, List<(u64, bool)>), Option<bool>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<bool>, List<(u64, bool)>), Option<bool>>(({
                            let c196: Fn1 = v0.clone();
                            c196.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: u64, v1: Option<bool>, v2: (u64, bool)) -> Option<bool> {
    let m200 = v1;
    match m200 {
        None => {
            let a202 = {
                let (h201, _) = v2;
                h201
            };
            let a203 = v0;
            if nat_eq(a202, a203) {
                Some({
                    let (_, h204) = v2;
                    h204
                })
            } else {
                None::<bool>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f38(v0: Fn2, p209: u64, p210: u64) -> R<(u64, bool)> {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v1, v2) = m208;
                let m205 = v1;
                if m205 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m205 - 1;
                    let m207 = {
                        let c206: Fn2 = v0.clone();
                        c206.apply(v2)?
                    };
                    match m207 {
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
                Ok(next) => { m208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f51(v0, v1)
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m214 = {
        let a211 = v0;
        let a212 = v1;
        let a213 = v2;
        slice_bytes(a211, a212, a213)
    };
    match m214 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f41(v0: Adt2) -> bool {
    let m215 = v0;
    match m215 {
        Adt2::C0(v1) => {
            f52(v1)
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(v3, r217) => {
            let v4 = (*r217).clone();
            let m216 = f53(v3);
            if m216 {
                true
            } else {
                f41(v4)
            }
        }
        Adt2::C3(_, r218, r219) => {
            let v6 = (*r218).clone();
            let v7 = (*r219).clone();
            let v8: bool = f41(v6);
            let v9: bool = f41(v7);
            if v8 {
                true
            } else {
                v9
            }
        }
        Adt2::C4(v10, _) => {
            f54(v10)
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f42(v0: Adt27, v1: Adt2) -> R<Result<(), Bytes>> {
    let m220 = v1;
    match m220 {
        Adt2::C0(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C1(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C2(_, r232) => {
            let v5 = (*r232).clone();
            f42(v0, v5)
        }
        Adt2::C3(_, r233, r234) => {
            let v7 = (*r233).clone();
            let v8 = (*r234).clone();
            let m221 = f42(v0.clone(), v7)?;
            match m221 {
                Ok(_) => {
                    f42(v0, v8)
                }
                Err(v10) => {
                    Ok(Err::<(), Bytes>(v10))
                }
            }
        }
        Adt2::C4(v11, v12) => {
            let m222 = f54(v11.clone());
            if m222 {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let m223 = f6(v11);
                if m223 {
                    let m227 = f7({
                        let m226 = match v0 {
                            Adt27::C0(_, h225, _, _) => {
                                h225
                            }
                        };
                        match m226 {
                            Adt26::C0(_, h224, _, _) => {
                                h224
                            }
                        }
                    }, v12);
                    if m227 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("type Fn")), f4(f10(v12)?, f1(Str::lit(" is not declared"))))))
                    }
                } else {
                    let m231 = f7({
                        let m230 = match v0 {
                            Adt27::C0(_, h229, _, _) => {
                                h229
                            }
                        };
                        match m230 {
                            Adt26::C0(h228, _, _, _) => {
                                h228
                            }
                        }
                    }, v12);
                    if m231 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("type Adt")), f4(f10(v12)?, f1(Str::lit(" is not declared"))))))
                    }
                }
            }
        }
        Adt2::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C6(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f43(v0: List<Adt11>) -> bool {
    f55(Fn3::F56, false, v0)
}

pub fn f44(v0: Adt27, v1: Bytes) -> Result<(), Bytes> {
    let m236 = match v0 {
        Adt27::C0(h235, _, _, _) => {
            h235
        }
    };
    if m236 {
        Err::<(), Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v1, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<(), Bytes>(())
    }
}

pub fn f45(v0: Adt28, v1: Adt26, v2: Adt14) -> R<Result<Adt28, Bytes>> {
    let m237 = v2;
    match m237 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt14::C1(v4) => {
            let m239 = f57(match v0.clone() {
                Adt28::C0(h238, _, _, _, _) => {
                    h238
                }
            }, v4.clone());
            if m239 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f30(f31(v4)?), f1(Str::lit(" is bound twice in one function"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(v4.clone(), match v0.clone() {
                    Adt28::C0(h240, _, _, _, _) => {
                        h240
                    }
                }), List::cons(v4, match v0.clone() {
                    Adt28::C0(_, h241, _, _, _) => {
                        h241
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, h242, _, _) => {
                        h242
                    }
                }, match v0.clone() {
                    Adt28::C0(_, _, _, h243, _) => {
                        h243
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h244) => {
                        h244
                    }
                })))
            }
        }
        Adt14::C2(r250) => {
            let v5 = (*r250).clone();
            f45(v0, v1, v5)
        }
        Adt14::C3(_, r251) => {
            let v7 = (*r251).clone();
            f45(v0, v1, v7)
        }
        Adt14::C4(v8, v9, r252) => {
            let v10 = (*r252).clone();
            let m245 = f58(v1.clone(), v8, v9);
            match m245 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("pattern Adt")), f4(f10(v8)?, f4(f1(Str::lit("::C")), f4(f10(v9)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v11) => {
                    let m248 = {
                        let a246 = v11;
                        let a247 = f59(v10.clone())?;
                        nat_eq(a246, a247)
                    };
                    if m248 {
                        f45(v0, v1, v10)
                    } else {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("pattern Adt")), f4(f10(v8)?, f4(f1(Str::lit("::C")), f4(f10(v9)?, f4(f1(Str::lit(" binds ")), f4(f10(f59(v10)?)?, f4(f1(Str::lit(" fields of ")), f10(v11)?)))))))))
                    }
                }
            }
        }
        Adt14::C5 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt14::C6(r253, r254) => {
            let v12 = (*r253).clone();
            let v13 = (*r254).clone();
            let m249 = f45(v0, v1.clone(), v12)?;
            match m249 {
                Ok(v14) => {
                    f45(v14, v1, v13)
                }
                Err(v15) => {
                    Ok(Err::<Adt28, Bytes>(v15))
                }
            }
        }
    }
}

pub fn f46(p267: Adt28, v1: Adt27, p268: Adt17) -> R<Result<Adt28, Bytes>> {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v0, v2) = m266;
                let m255 = v2;
                match m255 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C14(v40, v41, v42, r264, _, r265) => {
                        let v43 = (*r264).clone();
                        let v45 = (*r265).clone();
                        let m257 = {
                            let m256 = v41;
                            if m256 {
                                f26(v1.clone(), v42)?
                            } else {
                                Ok::<(), Bytes>(())
                            }
                        };
                        match m257 {
                            Ok(_) => {
                                let m261 = {
                                    let m258 = f60(v40.clone(), v43.clone());
                                    match m258 {
                                        None => {
                                            f47(v0, v1.clone(), v43, false)?
                                        }
                                        Some(v47) => {
                                            let m259 = v40.clone();
                                            match m259 {
                                                Adt14::C0(_) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C1(_) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C2(r260) => {
                                                    let v50 = (*r260).clone();
                                                    f61(v0, v47, v50)?
                                                }
                                                Adt14::C3(_, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C4(_, _, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C5 => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C6(_, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                            }
                                        }
                                    }
                                };
                                match m261 {
                                    Ok(v58) => {
                                        let m263 = f45(v58, match v1.clone() {
                                            Adt27::C0(_, h262, _, _) => {
                                                h262
                                            }
                                        }, v40)?;
                                        match m263 {
                                            Ok(v59) => {
                                                Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v59, v45))
                                            }
                                            Err(v60) => {
                                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v60)))
                                            }
                                        }
                                    }
                                    Err(v61) => {
                                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v61)))
                                    }
                                }
                            }
                            Err(v62) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v62)))
                            }
                        }
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Adt28, v1: Adt27, v2: Adt17, v3: bool) -> R<Result<Adt28, Bytes>> {
    let m269 = v2;
    match m269 {
        Adt17::C0(v4, v5) => {
            let v6: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h270, _) => {
                        h270
                    }
                }
            } else {
                false
            };
            let m271 = v6;
            if m271 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h272) => {
                        h272
                    }
                }, Adt17::C0(v4, v5))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m273 = v4;
                match m273 {
                    Adt18::C0 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C1 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C2 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C3 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C4(_) => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C5 => {
                        Ok(f63(v1, v0, f1(Str::lit("a string literal"))))
                    }
                    Adt18::C6 => {
                        Ok(f63(v1, v0, f1(Str::lit("a byte string literal"))))
                    }
                    Adt18::C7 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C8 => {
                        Ok(Err::<Adt28, Bytes>(f1(Str::lit("overflow: the overflow value stands outside the payload of an `Err`"))))
                    }
                }
            }
        }
        Adt17::C1(v8, v9, v10) => {
            let v11: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h274, _) => {
                        h274
                    }
                }
            } else {
                false
            };
            let m275 = v11;
            if m275 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h276) => {
                        h276
                    }
                }, Adt17::C1(v8, v9, v10))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m277 = v8;
                match m277 {
                    Adt19::C0 => {
                        f64(v0, v9)
                    }
                    Adt19::C1 => {
                        let m278 = f65(v0.clone(), v9)?;
                        match m278 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt28, Bytes>(v13))
                            }
                        }
                    }
                    Adt19::C2 => {
                        let m279 = f65(v0.clone(), v9)?;
                        match m279 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v15) => {
                                Ok(Err::<Adt28, Bytes>(v15))
                            }
                        }
                    }
                    Adt19::C3 => {
                        let m280 = f65(v0.clone(), v9)?;
                        match m280 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v17) => {
                                Ok(Err::<Adt28, Bytes>(v17))
                            }
                        }
                    }
                    Adt19::C4 => {
                        let m281 = f44(v1, f1(Str::lit("a boxed read")));
                        match m281 {
                            Ok(_) => {
                                let m282 = f65(v0.clone(), v9)?;
                                match m282 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt28, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v21) => {
                                Ok(Err::<Adt28, Bytes>(v21))
                            }
                        }
                    }
                }
            }
        }
        Adt17::C2(v22, r415, v24) => {
            let v23 = (*r415).clone();
            let m283 = v22.clone();
            match m283 {
                Adt20::C0 => {
                    let v25: bool = if v3 {
                        match v1.clone() {
                            Adt27::C0(_, _, h284, _) => {
                                h284
                            }
                        }
                    } else {
                        false
                    };
                    let m285 = v25;
                    if m285 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h286) => {
                                h286
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m287 = f66(v23.clone());
                        if m287 {
                            Ok(Err::<Adt28, Bytes>(f1(Str::lit("a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean"))))
                        } else {
                            f47(v0, v1, v23, false)
                        }
                    }
                }
                Adt20::C1 => {
                    let v26: bool = if v3 {
                        match v1.clone() {
                            Adt27::C0(_, _, h288, _) => {
                                h288
                            }
                        }
                    } else {
                        false
                    };
                    let m289 = v26;
                    if m289 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h290) => {
                                h290
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m291 = f44(v1.clone(), f1(Str::lit("a box")));
                        match m291 {
                            Ok(_) => {
                                f47(v0, v1, v23, false)
                            }
                            Err(v28) => {
                                Ok(Err::<Adt28, Bytes>(v28))
                            }
                        }
                    }
                }
                Adt20::C2 => {
                    let v29: bool = if v3 {
                        match v1.clone() {
                            Adt27::C0(_, _, h292, _) => {
                                h292
                            }
                        }
                    } else {
                        false
                    };
                    let m293 = v29;
                    if m293 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h294) => {
                                h294
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        f47(v0, v1, v23, false)
                    }
                }
                Adt20::C3 => {
                    f47(v0, v1, v23, false)
                }
            }
        }
        Adt17::C3(v30, r416, v32, v33) => {
            let v31 = (*r416).clone();
            let m295 = v32;
            if m295 {
                let v34: bool = if v3 {
                    match v1.clone() {
                        Adt27::C0(_, _, h296, _) => {
                            h296
                        }
                    }
                } else {
                    false
                };
                let m297 = v34;
                if m297 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                        Adt27::C0(_, _, _, h298) => {
                            h298
                        }
                    }, Adt17::C3(v30, std::rc::Rc::new(v31), v32, v33))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m299 = f67(v0, v1.clone(), false, v31.clone())?;
                    match m299 {
                        Ok(v35) => {
                            let m300 = v30;
                            match m300 {
                                Adt21::C0(v36) => {
                                    let m304 = f20({
                                        let m303 = match v1.clone() {
                                            Adt27::C0(_, h302, _, _) => {
                                                h302
                                            }
                                        };
                                        match m303 {
                                            Adt26::C0(_, _, _, h301) => {
                                                h301
                                            }
                                        }
                                    }, v36);
                                    match m304 {
                                        None => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v36)?, f1(Str::lit(" is not declared"))))))
                                        }
                                        Some(v37) => {
                                            Ok(f68(v35, v1, v37, v32, v3, f4(f1(Str::lit("f")), f10(v36)?)))
                                        }
                                    }
                                }
                                Adt21::C1(v39) => {
                                    let v40: Bytes = f70(match v1.clone() {
                                        Adt27::C0(_, _, _, h305) => {
                                            h305
                                        }
                                    }, v39.clone());
                                    let m308 = {
                                        let m307 = f71(match v1.clone() {
                                            Adt27::C0(_, _, _, h306) => {
                                                h306
                                            }
                                        }, v39.clone());
                                        if m307 {
                                            f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v40.clone())))
                                        } else {
                                            Ok::<(), Bytes>(())
                                        }
                                    };
                                    match m308 {
                                        Ok(_) => {
                                            Ok(f68(v35, v1.clone(), f72(match v1 {
                                                Adt27::C0(_, _, _, h309) => {
                                                    h309
                                                }
                                            }, v39), v32, v3, f30(v40)))
                                        }
                                        Err(v42) => {
                                            Ok(Err::<Adt28, Bytes>(v42))
                                        }
                                    }
                                }
                                Adt21::C2(v38) => {
                                    f69(v35, v1, v38, v31, v32, v3)
                                }
                            }
                        }
                        Err(v43) => {
                            Ok(Err::<Adt28, Bytes>(v43))
                        }
                    }
                }
            } else {
                let m310 = f67(v0, v1.clone(), false, v31.clone())?;
                match m310 {
                    Ok(v44) => {
                        let m311 = v30;
                        match m311 {
                            Adt21::C0(v45) => {
                                let m315 = f20({
                                    let m314 = match v1.clone() {
                                        Adt27::C0(_, h313, _, _) => {
                                            h313
                                        }
                                    };
                                    match m314 {
                                        Adt26::C0(_, _, _, h312) => {
                                            h312
                                        }
                                    }
                                }, v45);
                                match m315 {
                                    None => {
                                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v45)?, f1(Str::lit(" is not declared"))))))
                                    }
                                    Some(v46) => {
                                        Ok(f68(v44, v1, v46, v32, v3, f4(f1(Str::lit("f")), f10(v45)?)))
                                    }
                                }
                            }
                            Adt21::C1(v48) => {
                                let v49: Bytes = f70(match v1.clone() {
                                    Adt27::C0(_, _, _, h316) => {
                                        h316
                                    }
                                }, v48.clone());
                                let m319 = {
                                    let m318 = f71(match v1.clone() {
                                        Adt27::C0(_, _, _, h317) => {
                                            h317
                                        }
                                    }, v48.clone());
                                    if m318 {
                                        f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v49.clone())))
                                    } else {
                                        Ok::<(), Bytes>(())
                                    }
                                };
                                match m319 {
                                    Ok(_) => {
                                        Ok(f68(v44, v1.clone(), f72(match v1 {
                                            Adt27::C0(_, _, _, h320) => {
                                                h320
                                            }
                                        }, v48), v32, v3, f30(v49)))
                                    }
                                    Err(v51) => {
                                        Ok(Err::<Adt28, Bytes>(v51))
                                    }
                                }
                            }
                            Adt21::C2(v47) => {
                                f69(v44, v1, v47, v31, v32, v3)
                            }
                        }
                    }
                    Err(v52) => {
                        Ok(Err::<Adt28, Bytes>(v52))
                    }
                }
            }
        }
        Adt17::C4(v53, r417, v55, v56) => {
            let v54 = (*r417).clone();
            let m321 = v55;
            if m321 {
                let v57: bool = if v3 {
                    match v1.clone() {
                        Adt27::C0(_, _, h322, _) => {
                            h322
                        }
                    }
                } else {
                    false
                };
                let m323 = v57;
                if m323 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                        Adt27::C0(_, _, _, h324) => {
                            h324
                        }
                    }, Adt17::C4(v53, std::rc::Rc::new(v54), v55, v56))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m325 = f65(v0.clone(), v53)?;
                    match m325 {
                        Ok(_) => {
                            let m326 = f67(v0, v1, false, v54)?;
                            match m326 {
                                Ok(v59) => {
                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v59.clone() {
                                        Adt28::C0(h327, _, _, _, _) => {
                                            h327
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, h328, _, _, _) => {
                                            h328
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, _, h329, _, _) => {
                                            h329
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, _, _, h330, _) => {
                                            h330
                                        }
                                    }, if v55 {
                                        true
                                    } else {
                                        match v59 {
                                            Adt28::C0(_, _, _, _, h331) => {
                                                h331
                                            }
                                        }
                                    })))
                                }
                                Err(v60) => {
                                    Ok(Err::<Adt28, Bytes>(v60))
                                }
                            }
                        }
                        Err(v61) => {
                            Ok(Err::<Adt28, Bytes>(v61))
                        }
                    }
                }
            } else {
                let m332 = f65(v0.clone(), v53)?;
                match m332 {
                    Ok(_) => {
                        let m333 = f67(v0, v1, false, v54)?;
                        match m333 {
                            Ok(v63) => {
                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v63.clone() {
                                    Adt28::C0(h334, _, _, _, _) => {
                                        h334
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, h335, _, _, _) => {
                                        h335
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, _, h336, _, _) => {
                                        h336
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, _, _, h337, _) => {
                                        h337
                                    }
                                }, if v55 {
                                    true
                                } else {
                                    match v63 {
                                        Adt28::C0(_, _, _, _, h338) => {
                                            h338
                                        }
                                    }
                                })))
                            }
                            Err(v64) => {
                                Ok(Err::<Adt28, Bytes>(v64))
                            }
                        }
                    }
                    Err(v65) => {
                        Ok(Err::<Adt28, Bytes>(v65))
                    }
                }
            }
        }
        Adt17::C5(v66, r418, v68) => {
            let v67 = (*r418).clone();
            let v69: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, h339, _) => {
                        h339
                    }
                } {
                    let a340 = f73(v66.clone(), v67.clone())?;
                    bool_not(a340)
                } else {
                    false
                }
            } else {
                false
            };
            let m341 = v69;
            if m341 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h342) => {
                        h342
                    }
                }, Adt17::C5(v66, std::rc::Rc::new(v67), v68))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m343 = f67(v0, v1.clone(), f74(v66.clone()), v67.clone())?;
                match m343 {
                    Ok(v70) => {
                        let m344 = f75(v1, v66, f76(v67)?)?;
                        match m344 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v70))
                            }
                            Err(v72) => {
                                Ok(Err::<Adt28, Bytes>(v72))
                            }
                        }
                    }
                    Err(v73) => {
                        Ok(Err::<Adt28, Bytes>(v73))
                    }
                }
            }
        }
        Adt17::C6(r419, r420, v76) => {
            let v74 = (*r419).clone();
            let v75 = (*r420).clone();
            let v77: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h345, _) => {
                        h345
                    }
                }
            } else {
                false
            };
            let m346 = v77;
            if m346 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h347) => {
                        h347
                    }
                }, Adt17::C6(std::rc::Rc::new(v74), std::rc::Rc::new(v75), v76))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m348 = f47(v0, v1.clone(), v74, false)?;
                match m348 {
                    Ok(v78) => {
                        f47(v78, v1, v75, false)
                    }
                    Err(v79) => {
                        Ok(Err::<Adt28, Bytes>(v79))
                    }
                }
            }
        }
        Adt17::C7(r421, r422, r423, _) => {
            let v80 = (*r421).clone();
            let v81 = (*r422).clone();
            let v82 = (*r423).clone();
            let m349 = f47(v0, v1.clone(), v80, false)?;
            match m349 {
                Ok(v84) => {
                    let v85: Adt28 = Adt28::C0(match v84.clone() {
                        Adt28::C0(h350, _, _, _, _) => {
                            h350
                        }
                    }, match v84.clone() {
                        Adt28::C0(_, h351, _, _, _) => {
                            h351
                        }
                    }, match v84.clone() {
                        Adt28::C0(_, _, h352, _, _) => {
                            h352
                        }
                    }, match v84.clone() {
                        Adt28::C0(_, _, _, h353, _) => {
                            h353
                        }
                    }, match v84.clone() {
                        Adt28::C0(_, _, _, _, h354) => {
                            h354
                        }
                    });
                    let m356 = f45(v85, match v1.clone() {
                        Adt27::C0(_, h355, _, _) => {
                            h355
                        }
                    }, Adt14::C0(Adt15::C0))?;
                    match m356 {
                        Ok(v86) => {
                            let m357 = f29(v86, v1.clone(), v81, v3)?;
                            match m357 {
                                Ok(v87) => {
                                    let v88: Adt28 = Adt28::C0(match v87.clone() {
                                        Adt28::C0(h358, _, _, _, _) => {
                                            h358
                                        }
                                    }, match v84.clone() {
                                        Adt28::C0(_, h359, _, _, _) => {
                                            h359
                                        }
                                    }, f77(match v84.clone() {
                                        Adt28::C0(_, _, h360, _, _) => {
                                            h360
                                        }
                                    }, match v87.clone() {
                                        Adt28::C0(_, _, h361, _, _) => {
                                            h361
                                        }
                                    }), f78(match v84.clone() {
                                        Adt28::C0(_, _, _, h362, _) => {
                                            h362
                                        }
                                    }, match v87.clone() {
                                        Adt28::C0(_, _, _, h363, _) => {
                                            h363
                                        }
                                    }), match v87 {
                                        Adt28::C0(_, _, _, _, h364) => {
                                            h364
                                        }
                                    });
                                    let v89: Adt28 = Adt28::C0(match v88.clone() {
                                        Adt28::C0(h365, _, _, _, _) => {
                                            h365
                                        }
                                    }, match v84.clone() {
                                        Adt28::C0(_, h366, _, _, _) => {
                                            h366
                                        }
                                    }, match v84.clone() {
                                        Adt28::C0(_, _, h367, _, _) => {
                                            h367
                                        }
                                    }, match v84.clone() {
                                        Adt28::C0(_, _, _, h368, _) => {
                                            h368
                                        }
                                    }, match v88.clone() {
                                        Adt28::C0(_, _, _, _, h369) => {
                                            h369
                                        }
                                    });
                                    let m371 = f45(v89, match v1.clone() {
                                        Adt27::C0(_, h370, _, _) => {
                                            h370
                                        }
                                    }, Adt14::C0(Adt15::C0))?;
                                    match m371 {
                                        Ok(v90) => {
                                            let m372 = f29(v90, v1, v82, v3)?;
                                            match m372 {
                                                Ok(v91) => {
                                                    let v92: Adt28 = Adt28::C0(match v91.clone() {
                                                        Adt28::C0(h373, _, _, _, _) => {
                                                            h373
                                                        }
                                                    }, match v84 {
                                                        Adt28::C0(_, h374, _, _, _) => {
                                                            h374
                                                        }
                                                    }, f77(match v88.clone() {
                                                        Adt28::C0(_, _, h375, _, _) => {
                                                            h375
                                                        }
                                                    }, match v91.clone() {
                                                        Adt28::C0(_, _, h376, _, _) => {
                                                            h376
                                                        }
                                                    }), f78(match v88 {
                                                        Adt28::C0(_, _, _, h377, _) => {
                                                            h377
                                                        }
                                                    }, match v91.clone() {
                                                        Adt28::C0(_, _, _, h378, _) => {
                                                            h378
                                                        }
                                                    }), match v91 {
                                                        Adt28::C0(_, _, _, _, h379) => {
                                                            h379
                                                        }
                                                    });
                                                    Ok(Ok::<Adt28, Bytes>(v92))
                                                }
                                                Err(v93) => {
                                                    Ok(Err::<Adt28, Bytes>(v93))
                                                }
                                            }
                                        }
                                        Err(v94) => {
                                            Ok(Err::<Adt28, Bytes>(v94))
                                        }
                                    }
                                }
                                Err(v95) => {
                                    Ok(Err::<Adt28, Bytes>(v95))
                                }
                            }
                        }
                        Err(v96) => {
                            Ok(Err::<Adt28, Bytes>(v96))
                        }
                    }
                }
                Err(v97) => {
                    Ok(Err::<Adt28, Bytes>(v97))
                }
            }
        }
        Adt17::C8(r424, r425, _) => {
            let v98 = (*r424).clone();
            let v99 = (*r425).clone();
            let m380 = f47(v0, v1.clone(), v98, false)?;
            match m380 {
                Ok(v101) => {
                    f79(v101.clone(), v1, v101.clone(), match v101 {
                        Adt28::C0(_, h381, _, _, _) => {
                            h381
                        }
                    }, v99, v3)
                }
                Err(v102) => {
                    Ok(Err::<Adt28, Bytes>(v102))
                }
            }
        }
        Adt17::C9(r426) => {
            let v103 = (*r426).clone();
            f29(v0, v1, v103, v3)
        }
        Adt17::C10(v104, v105, v106) => {
            let v107: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h382, _) => {
                        h382
                    }
                }
            } else {
                false
            };
            let m383 = v107;
            if m383 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h384) => {
                        h384
                    }
                }, Adt17::C10(v104, v105, v106))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m385 = v104;
                match m385 {
                    Adt24::C0 => {
                        let m386 = f65(v0.clone(), v105)?;
                        match m386 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v109) => {
                                Ok(Err::<Adt28, Bytes>(v109))
                            }
                        }
                    }
                    Adt24::C1 => {
                        let m387 = f65(v0.clone(), v105)?;
                        match m387 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v111) => {
                                Ok(Err::<Adt28, Bytes>(v111))
                            }
                        }
                    }
                    Adt24::C2 => {
                        let m388 = f44(v1, f1(Str::lit("a list match")));
                        match m388 {
                            Ok(_) => {
                                let m389 = f65(v0.clone(), v105)?;
                                match m389 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v114) => {
                                        Ok(Err::<Adt28, Bytes>(v114))
                                    }
                                }
                            }
                            Err(v115) => {
                                Ok(Err::<Adt28, Bytes>(v115))
                            }
                        }
                    }
                    Adt24::C3 => {
                        let m390 = f65(v0.clone(), v105)?;
                        match m390 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v117) => {
                                Ok(Err::<Adt28, Bytes>(v117))
                            }
                        }
                    }
                }
            }
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C12 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C16(v131, r427) => {
            let v132 = (*r427).clone();
            let v133: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h391, _) => {
                        h391
                    }
                }
            } else {
                false
            };
            let v134: Result<(), Bytes> = {
                let m392 = v133;
                if m392 {
                    f80(match v1.clone() {
                        Adt27::C0(_, _, _, h393) => {
                            h393
                        }
                    }, v132.clone())?
                } else {
                    Ok::<(), Bytes>(())
                }
            };
            let m394 = v134;
            match m394 {
                Ok(_) => {
                    let m395 = f64(v0, v131.clone())?;
                    match m395 {
                        Ok(v136) => {
                            let v137: Adt28 = Adt28::C0(match v136.clone() {
                                Adt28::C0(h396, _, _, _, _) => {
                                    h396
                                }
                            }, match v136.clone() {
                                Adt28::C0(_, h397, _, _, _) => {
                                    h397
                                }
                            }, f81(match v136.clone() {
                                Adt28::C0(_, _, h398, _, _) => {
                                    h398
                                }
                            }, v131.clone()), match v136.clone() {
                                Adt28::C0(_, _, _, h399, _) => {
                                    h399
                                }
                            }, match v136.clone() {
                                Adt28::C0(_, _, _, _, h400) => {
                                    h400
                                }
                            });
                            let m401 = f29(v137.clone(), v1, v132, false)?;
                            match m401 {
                                Ok(v138) => {
                                    let m405 = f82(match v138.clone() {
                                        Adt28::C0(_, _, h402, _, _) => {
                                            h402
                                        }
                                    }, v131, match v137.clone() {
                                        Adt28::C0(_, _, h403, _, _) => {
                                            h403
                                        }
                                    }, match v136.clone() {
                                        Adt28::C0(_, h404, _, _, _) => {
                                            h404
                                        }
                                    });
                                    match m405 {
                                        None => {
                                            let m409 = f83(match v138.clone() {
                                                Adt28::C0(_, _, _, h406, _) => {
                                                    h406
                                                }
                                            }, match v137 {
                                                Adt28::C0(_, _, _, h407, _) => {
                                                    h407
                                                }
                                            }, match v136.clone() {
                                                Adt28::C0(_, h408, _, _, _) => {
                                                    h408
                                                }
                                            });
                                            match m409 {
                                                None => {
                                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v138.clone() {
                                                        Adt28::C0(h410, _, _, _, _) => {
                                                            h410
                                                        }
                                                    }, match v138.clone() {
                                                        Adt28::C0(_, h411, _, _, _) => {
                                                            h411
                                                        }
                                                    }, match v136.clone() {
                                                        Adt28::C0(_, _, h412, _, _) => {
                                                            h412
                                                        }
                                                    }, match v136 {
                                                        Adt28::C0(_, _, _, h413, _) => {
                                                            h413
                                                        }
                                                    }, match v138 {
                                                        Adt28::C0(_, _, _, _, h414) => {
                                                            h414
                                                        }
                                                    })))
                                                }
                                                Some(v140) => {
                                                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v140)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                                }
                                            }
                                        }
                                        Some(v139) => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v139)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                        }
                                    }
                                }
                                Err(v141) => {
                                    Ok(Err::<Adt28, Bytes>(v141))
                                }
                            }
                        }
                        Err(v142) => {
                            Ok(Err::<Adt28, Bytes>(v142))
                        }
                    }
                }
                Err(v143) => {
                    Ok(Err::<Adt28, Bytes>(v143))
                }
            }
        }
    }
}

pub fn f48(p434: List<(Adt12, u64)>, v1: Adt26, p435: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m433 = (p434, p435);
    {
        let mut m433 = m433;
        loop {
            let step = {
                let (v0, v2) = m433;
                let m428 = v2;
                match m428 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(v40, v41, v42, r431, _, r432) => {
                        let v43 = (*r431).clone();
                        let v45 = (*r432).clone();
                        let v46: List<(Adt12, u64)> = {
                            let m429 = v41;
                            if m429 {
                                f50(v0, v40, v42)
                            } else {
                                v0
                            }
                        };
                        let m430 = f49(v46, v1.clone(), v43, false)?;
                        match m430 {
                            Ok(v47) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v47, v45))
                            }
                            Err(v48) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v48)))
                            }
                        }
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m436 = v2;
    match m436 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, r446, _) => {
            let v10 = (*r446).clone();
            f49(v0, v1, v10, false)
        }
        Adt17::C3(_, r447, _, _) => {
            let v13 = (*r447).clone();
            f84(v0, v1, v13)
        }
        Adt17::C4(v16, r448, v18, _) => {
            let v17 = (*r448).clone();
            let m437 = f85(v0.clone(), v16.clone());
            match m437 {
                None => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f30(f31(v16)?), f1(Str::lit(" is applied without a function type")))))
                }
                Some(v20) => {
                    let m439 = f20(match v1.clone() {
                        Adt26::C0(_, _, h438, _) => {
                            h438
                        }
                    }, v20);
                    match m439 {
                        None => {
                            Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v20)?, f1(Str::lit("::apply is not declared"))))))
                        }
                        Some(v21) => {
                            let v22: bool = if v21 {
                                !v18
                            } else {
                                v18
                            };
                            let v23: bool = if v3 {
                                v21
                            } else {
                                false
                            };
                            let m440 = v22;
                            if m440 {
                                let m441 = v23;
                                if m441 {
                                    f84(v0, v1, v17)
                                } else {
                                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f1(Str::lit("arithmetic: an application of Fn")), f4(f10(v20)?, f4(f1(Str::lit(" ")), f4(if v18 {
                                        f1(Str::lit("propagates"))
                                    } else {
                                        f1(Str::lit("drops"))
                                    }, f4(f1(Str::lit(" a failure its apply ")), if v21 {
                                        f1(Str::lit("can have"))
                                    } else {
                                        f1(Str::lit("cannot have"))
                                    })))))))
                                }
                            } else {
                                f84(v0, v1, v17)
                            }
                        }
                    }
                }
            }
        }
        Adt17::C5(_, r449, _) => {
            let v25 = (*r449).clone();
            f84(v0, v1, v25)
        }
        Adt17::C6(r450, r451, _) => {
            let v27 = (*r450).clone();
            let v28 = (*r451).clone();
            let m442 = f49(v0, v1.clone(), v27, false)?;
            match m442 {
                Ok(v30) => {
                    f49(v30, v1, v28, false)
                }
                Err(v31) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v31))
                }
            }
        }
        Adt17::C7(r452, r453, r454, _) => {
            let v32 = (*r452).clone();
            let v33 = (*r453).clone();
            let v34 = (*r454).clone();
            let m443 = f49(v0, v1.clone(), v32, false)?;
            match m443 {
                Ok(v36) => {
                    let m444 = f32(v36, v1.clone(), v33, v3)?;
                    match m444 {
                        Ok(v37) => {
                            f32(v37, v1, v34, v3)
                        }
                        Err(v38) => {
                            Ok(Err::<List<(Adt12, u64)>, Bytes>(v38))
                        }
                    }
                }
                Err(v39) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v39))
                }
            }
        }
        Adt17::C8(r455, r456, _) => {
            let v40 = (*r455).clone();
            let v41 = (*r456).clone();
            let m445 = f49(v0, v1.clone(), v40, false)?;
            match m445 {
                Ok(v43) => {
                    f86(v43, v1, v41, v3)
                }
                Err(v44) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v44))
                }
            }
        }
        Adt17::C9(r457) => {
            let v45 = (*r457).clone();
            f32(v0, v1, v45, v3)
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C12 => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C16(_, r458) => {
            let v63 = (*r458).clone();
            f32(v0, v1, v63, false)
        }
    }
}

pub fn f50(v0: List<(Adt12, u64)>, v1: Adt14, v2: Adt2) -> List<(Adt12, u64)> {
    let m459 = f87(v1);
    match m459 {
        None => {
            v0
        }
        Some(v3) => {
            let m460 = f88(v2);
            match m460 {
                None => {
                    v0
                }
                Some(v4) => {
                    List::cons((v3, v4), v0)
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f89(v0, v1, 48u8) {
        Ok(Some({
            let a461 = v1;
            let a462 = 1u64;
            nat_add(a461, a462)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f52(v0: Adt3) -> bool {
    let m463 = v0;
    match m463 {
        Adt3::C0 => {
            false
        }
        Adt3::C1 => {
            false
        }
        Adt3::C2 => {
            false
        }
        Adt3::C3 => {
            false
        }
        Adt3::C4 => {
            false
        }
        Adt3::C5 => {
            true
        }
        Adt3::C6 => {
            true
        }
        Adt3::C7 => {
            false
        }
        Adt3::C8 => {
            false
        }
    }
}

pub fn f53(v0: Adt5) -> bool {
    let m464 = v0;
    match m464 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            true
        }
        Adt5::C2 => {
            true
        }
        Adt5::C3 => {
            false
        }
        Adt5::C4 => {
            false
        }
    }
}

pub fn f54(v0: Adt7) -> bool {
    let m465 = v0;
    match m465 {
        Adt7::C0 => {
            false
        }
        Adt7::C1 => {
            false
        }
        Adt7::C2 => {
            true
        }
    }
}

pub fn f55(v0: Fn3, p469: bool, p470: List<Adt11>) -> bool {
    let m468 = (p469, p470);
    {
        let mut m468 = m468;
        loop {
            let step = {
                let (v1, v2) = m468;
                let m466 = v2;
                match m466.uncons() {
                    None => {
                        Err::<(bool, List<Adt11>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt11>), bool>(({
                            let c467: Fn3 = v0.clone();
                            c467.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: bool, v1: Adt11) -> bool {
    let m471 = v1;
    match m471 {
        Adt11::C0 => {
            v0
        }
        Adt11::C1 => {
            v0
        }
        Adt11::C2 => {
            true
        }
        Adt11::C3 => {
            v0
        }
    }
}

pub fn f57(v0: List<Adt12>, v1: Adt12) -> bool {
    f90(Fn4::F91(v1), false, v0)
}

pub fn f58(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m473 = f18(match v0 {
        Adt26::C0(h472, _, _, _) => {
            h472
        }
    }, v1);
    match m473 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f92(v3, v2)
        }
    }
}

pub fn f59(v0: Adt14) -> R<u64> {
    let m474 = v0;
    match m474 {
        Adt14::C0(_) => {
            Ok(0u64)
        }
        Adt14::C1(_) => {
            Ok(0u64)
        }
        Adt14::C2(_) => {
            Ok(0u64)
        }
        Adt14::C3(_, _) => {
            Ok(0u64)
        }
        Adt14::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt14::C5 => {
            Ok(0u64)
        }
        Adt14::C6(_, r477) => {
            let v10 = (*r477).clone();
            let v11: u64 = f59(v10)?;
            let a475 = 1u64;
            let a476 = v11;
            nat_add(a475, a476)
        }
    }
}

pub fn f60(v0: Adt14, v1: Adt17) -> Option<Adt12> {
    let m478 = v0;
    match m478 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(_) => {
            None::<Adt12>
        }
        Adt14::C2(r482) => {
            let v4 = (*r482).clone();
            let m479 = v1;
            match m479 {
                Adt17::C0(_, _) => {
                    None::<Adt12>
                }
                Adt17::C1(v7, v8, _) => {
                    let m480 = f93(v7);
                    if m480 {
                        let m481 = f94(v4);
                        if m481 {
                            Some(v8)
                        } else {
                            None::<Adt12>
                        }
                    } else {
                        None::<Adt12>
                    }
                }
                Adt17::C2(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C3(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C4(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C5(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C6(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C7(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C8(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C9(_) => {
                    None::<Adt12>
                }
                Adt17::C10(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C11(_, _) => {
                    None::<Adt12>
                }
                Adt17::C12 => {
                    None::<Adt12>
                }
                Adt17::C13(_, _) => {
                    None::<Adt12>
                }
                Adt17::C14(_, _, _, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C15(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C16(_, _) => {
                    None::<Adt12>
                }
            }
        }
        Adt14::C3(_, _) => {
            None::<Adt12>
        }
        Adt14::C4(_, _, _) => {
            None::<Adt12>
        }
        Adt14::C5 => {
            None::<Adt12>
        }
        Adt14::C6(_, _) => {
            None::<Adt12>
        }
    }
}

pub fn f61(v0: Adt28, v1: Adt12, v2: Adt14) -> R<Result<Adt28, Bytes>> {
    let m484 = f57(match v0.clone() {
        Adt28::C0(_, h483, _, _, _) => {
            h483
        }
    }, v1.clone());
    if m484 {
        let m486 = f57(match v0.clone() {
            Adt28::C0(_, _, h485, _, _) => {
                h485
            }
        }, v1.clone());
        if m486 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after it was moved"))))))
        } else {
            let v4: List<u64> = {
                let m488 = f95(match v0.clone() {
                    Adt28::C0(_, _, _, h487, _) => {
                        h487
                    }
                }, v1.clone());
                match m488 {
                    None => {
                        List::<u64>::nil()
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            let m489 = f96(v4, 0u64, v2, f30(f31(v1.clone())?))?;
            match m489 {
                Ok(v5) => {
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                        Adt28::C0(h490, _, _, _, _) => {
                            h490
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, h491, _, _, _) => {
                            h491
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, _, h492, _, _) => {
                            h492
                        }
                    }, f97(match v0.clone() {
                        Adt28::C0(_, _, _, h493, _) => {
                            h493
                        }
                    }, v1, v5), match v0 {
                        Adt28::C0(_, _, _, _, h494) => {
                            h494
                        }
                    })))
                }
                Err(v6) => {
                    Ok(Err::<Adt28, Bytes>(v6))
                }
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f62(v0: List<Adt25>, v1: Adt17) -> R<Bytes> {
    Ok(f98({
        let m496 = f99(Adt30::C0(List::<Adt29>::nil(), List::<Bytes>::nil(), v0), v1)?;
        match m496 {
            Adt30::C0(h495, _, _) => {
                h495
            }
        }
    }))
}

pub fn f63(v0: Adt27, v1: Adt28, v2: Bytes) -> Result<Adt28, Bytes> {
    let m498 = match v0 {
        Adt27::C0(h497, _, _, _) => {
            h497
        }
    };
    if m498 {
        Err::<Adt28, Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v2, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<Adt28, Bytes>(v1)
    }
}

pub fn f64(v0: Adt28, v1: Adt12) -> R<Result<Adt28, Bytes>> {
    let m500 = f57(match v0.clone() {
        Adt28::C0(_, h499, _, _, _) => {
            h499
        }
    }, v1.clone());
    if m500 {
        let m502 = f100(match v0.clone() {
            Adt28::C0(_, _, _, h501, _) => {
                h501
            }
        }, v1.clone());
        if m502 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after a component was moved out of it"))))))
        } else {
            let m504 = f57(match v0.clone() {
                Adt28::C0(_, _, h503, _, _) => {
                    h503
                }
            }, v1.clone());
            if m504 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved twice"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                    Adt28::C0(h505, _, _, _, _) => {
                        h505
                    }
                }, match v0.clone() {
                    Adt28::C0(_, h506, _, _, _) => {
                        h506
                    }
                }, List::cons(v1, match v0.clone() {
                    Adt28::C0(_, _, h507, _, _) => {
                        h507
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, _, h508, _) => {
                        h508
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h509) => {
                        h509
                    }
                })))
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f65(v0: Adt28, v1: Adt12) -> R<Result<(), Bytes>> {
    let m511 = f57(match v0.clone() {
        Adt28::C0(_, h510, _, _, _) => {
            h510
        }
    }, v1.clone());
    if m511 {
        let m513 = f57(match v0.clone() {
            Adt28::C0(_, _, h512, _, _) => {
                h512
            }
        }, v1.clone());
        if m513 {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after it was moved"))))))
        } else {
            let m515 = f100(match v0 {
                Adt28::C0(_, _, _, h514, _) => {
                    h514
                }
            }, v1.clone());
            if m515 {
                Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after a component was moved out of it"))))))
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f66(v0: Adt17) -> bool {
    let m516 = v0;
    match m516 {
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
        Adt17::C10(v31, _, _) => {
            f101(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
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

pub fn f67(p523: Adt28, v1: Adt27, v2: bool, p524: Adt17) -> R<Result<Adt28, Bytes>> {
    let m522 = (p523, p524);
    {
        let mut m522 = m522;
        loop {
            let step = {
                let (v0, v3) = m522;
                let m517 = v3;
                match m517 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(r520, r521) => {
                        let v39 = (*r520).clone();
                        let v40 = (*r521).clone();
                        let v41: bool = if v2 {
                            f102(v39.clone())
                        } else {
                            false
                        };
                        let m518 = v41;
                        if m518 {
                            Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v0, v40))
                        } else {
                            let m519 = f47(v0, v1.clone(), v39, false)?;
                            match m519 {
                                Ok(v42) => {
                                    Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v42, v40))
                                }
                                Err(v43) => {
                                    Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v43)))
                                }
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Adt28, v1: Adt27, v2: bool, v3: bool, v4: bool, v5: Bytes) -> Result<Adt28, Bytes> {
    let v6: bool = if v4 {
        match v1 {
            Adt27::C0(_, _, h525, _) => {
                h525
            }
        }
    } else {
        false
    };
    let m526 = v2;
    if m526 {
        let m527 = v3;
        if m527 {
            Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                Adt28::C0(h528, _, _, _, _) => {
                    h528
                }
            }, match v0.clone() {
                Adt28::C0(_, h529, _, _, _) => {
                    h529
                }
            }, match v0.clone() {
                Adt28::C0(_, _, h530, _, _) => {
                    h530
                }
            }, match v0 {
                Adt28::C0(_, _, _, h531, _) => {
                    h531
                }
            }, true))
        } else {
            let m532 = v6;
            if m532 {
                Ok::<Adt28, Bytes>(v0)
            } else {
                Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the fallible call of ")), f4(v5, f1(Str::lit(" does not propagate its overflow")))))
            }
        }
    } else {
        let m533 = v3;
        if m533 {
            Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the infallible call of ")), f4(v5, f1(Str::lit(" propagates a failure it cannot have")))))
        } else {
            Ok::<Adt28, Bytes>(v0)
        }
    }
}

pub fn f69(v0: Adt28, v1: Adt27, v2: u64, v3: Adt17, v4: bool, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m536 = {
        let a534 = f76(v3.clone())?;
        let a535 = f103(v2);
        nat_eq(a534, a535)
    };
    if m536 {
        Ok(f68(v0, v1, false, v4, v5, f30(f104(v2))))
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("machine: ")), f4(f30(f104(v2)), f4(f1(Str::lit(" receives ")), f4(f10(f76(v3)?)?, f4(f1(Str::lit(" operands of ")), f10(f103(v2))?)))))))
    }
}

pub fn f70(v0: List<Adt25>, v1: Adt22) -> Bytes {
    let m538 = f105(v0, match v1.clone() {
        Adt22::C0(h537, _) => {
            h537
        }
    });
    match m538 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let a540 = match v2.clone() {
                Adt25::C0(_, _, h539, _, _) => {
                    h539
                }
            };
            let a541 = 0u64;
            if nat_eq(a540, a541) {
                match v2 {
                    Adt25::C0(_, h542, _, _, _) => {
                        h542
                    }
                }
            } else {
                let a544 = match v2.clone() {
                    Adt25::C0(_, _, h543, _, _) => {
                        h543
                    }
                };
                let a545 = 1u64;
                if nat_eq(a544, a545) {
                    f4(f1(Str::lit("fixed_")), f4(f106(f107(match v1 {
                        Adt22::C0(_, h546) => {
                            h546
                        }
                    })), f4(f1(Str::lit("::")), match v2 {
                        Adt25::C0(_, h547, _, _, _) => {
                            h547
                        }
                    })))
                } else {
                    f4(match v2 {
                        Adt25::C0(_, h548, _, _, _) => {
                            h548
                        }
                    }, f106(f107(match v1 {
                        Adt22::C0(_, h549) => {
                            h549
                        }
                    })))
                }
            }
        }
    }
}

pub fn f71(v0: List<Adt25>, v1: Adt22) -> bool {
    let m551 = f105(v0, match v1 {
        Adt22::C0(h550, _) => {
            h550
        }
    });
    match m551 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, _, h552) => {
                    h552
                }
            }
        }
    }
}

pub fn f72(v0: List<Adt25>, v1: Adt22) -> bool {
    let m554 = f105(v0, match v1 {
        Adt22::C0(h553, _) => {
            h553
        }
    });
    match m554 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, h555, _) => {
                    h555
                }
            }
        }
    }
}

pub fn f73(v0: Adt23, v1: Adt17) -> R<bool> {
    let m556 = v0;
    match m556 {
        Adt23::C0(_) => {
            Ok(false)
        }
        Adt23::C1 => {
            Ok(false)
        }
        Adt23::C2(_, _) => {
            Ok(false)
        }
        Adt23::C3(_, v6) => {
            let v7: bool = f108(v6);
            let v8: bool = f109(v1)?;
            if v7 {
                Ok(v8)
            } else {
                Ok(false)
            }
        }
        Adt23::C4(_, _) => {
            Ok(false)
        }
        Adt23::C5(_, _) => {
            Ok(false)
        }
        Adt23::C6 => {
            Ok(false)
        }
        Adt23::C7(_) => {
            Ok(false)
        }
    }
}

pub fn f74(v0: Adt23) -> bool {
    let m557 = v0;
    match m557 {
        Adt23::C0(_) => {
            false
        }
        Adt23::C1 => {
            false
        }
        Adt23::C2(_, _) => {
            false
        }
        Adt23::C3(_, _) => {
            true
        }
        Adt23::C4(_, _) => {
            false
        }
        Adt23::C5(_, _) => {
            false
        }
        Adt23::C6 => {
            false
        }
        Adt23::C7(_) => {
            false
        }
    }
}

pub fn f75(v0: Adt27, v1: Adt23, v2: u64) -> R<Result<(), Bytes>> {
    let m558 = v1;
    match m558 {
        Adt23::C0(v3) => {
            f26(v0, v3)
        }
        Adt23::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt23::C2(v4, v5) => {
            let m559 = f26(v0.clone(), v4)?;
            match m559 {
                Ok(_) => {
                    f26(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt23::C3(v8, v9) => {
            let m560 = f26(v0.clone(), v8)?;
            match m560 {
                Ok(_) => {
                    f26(v0, v9)
                }
                Err(v11) => {
                    Ok(Err::<(), Bytes>(v11))
                }
            }
        }
        Adt23::C4(v12, v13) => {
            let m562 = f58(match v0 {
                Adt27::C0(_, h561, _, _) => {
                    h561
                }
            }, v12, v13);
            match m562 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v14) => {
                    let m565 = {
                        let a563 = v14;
                        let a564 = v2;
                        nat_eq(a563, a564)
                    };
                    if m565 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f4(f1(Str::lit(" receives ")), f4(f10(v2)?, f4(f1(Str::lit(" fields of ")), f10(v14)?)))))))))
                    }
                }
            }
        }
        Adt23::C5(v15, v16) => {
            let m567 = f110(match v0 {
                Adt27::C0(_, h566, _, _) => {
                    h566
                }
            }, v15, v16);
            match m567 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v17) => {
                    let m570 = {
                        let a568 = v17;
                        let a569 = v2;
                        nat_eq(a568, a569)
                    };
                    if m570 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f4(f1(Str::lit(" captures ")), f4(f10(v2)?, f4(f1(Str::lit(" values of ")), f10(v17)?)))))))))
                    }
                }
            }
        }
        Adt23::C6 => {
            Ok(f44(v0, f1(Str::lit("a list cell"))))
        }
        Adt23::C7(v18) => {
            let m571 = f44(v0.clone(), f1(Str::lit("an empty list")));
            match m571 {
                Ok(_) => {
                    f26(v0, v18)
                }
                Err(v20) => {
                    Ok(Err::<(), Bytes>(v20))
                }
            }
        }
    }
}

pub fn f76(v0: Adt17) -> R<u64> {
    let m572 = v0;
    match m572 {
        Adt17::C0(_, _) => {
            Ok(0u64)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C3(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C4(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C6(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C8(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C9(_) => {
            Ok(0u64)
        }
        Adt17::C10(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C11(_, _) => {
            Ok(0u64)
        }
        Adt17::C12 => {
            Ok(0u64)
        }
        Adt17::C13(_, r575) => {
            let v37 = (*r575).clone();
            let v38: u64 = f76(v37)?;
            let a573 = 1u64;
            let a574 = v38;
            nat_add(a573, a574)
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C15(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C16(_, _) => {
            Ok(0u64)
        }
    }
}

pub fn f77(v0: List<Adt12>, v1: List<Adt12>) -> List<Adt12> {
    f111(Fn5::F112, v0, v1)
}

pub fn f78(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    f113(Fn6::F114, v0, v1)
}

pub fn f79(v0: Adt28, v1: Adt27, p593: Adt28, v3: List<Adt12>, p594: Adt17, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v2, v4) = m592;
                let m576 = v4;
                match m576 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C15(v49, r590, r591) => {
                        let v50 = (*r590).clone();
                        let v51 = (*r591).clone();
                        let v52: Adt28 = Adt28::C0(match v2.clone() {
                            Adt28::C0(h577, _, _, _, _) => {
                                h577
                            }
                        }, v3.clone(), match v0.clone() {
                            Adt28::C0(_, _, h578, _, _) => {
                                h578
                            }
                        }, match v0.clone() {
                            Adt28::C0(_, _, _, h579, _) => {
                                h579
                            }
                        }, match v2.clone() {
                            Adt28::C0(_, _, _, _, h580) => {
                                h580
                            }
                        });
                        let m582 = f45(v52, match v1.clone() {
                            Adt27::C0(_, h581, _, _) => {
                                h581
                            }
                        }, v49)?;
                        match m582 {
                            Ok(v53) => {
                                let m583 = f29(v53, v1.clone(), v50, v5)?;
                                match m583 {
                                    Ok(v54) => {
                                        let v55: Adt28 = Adt28::C0(match v54.clone() {
                                            Adt28::C0(h584, _, _, _, _) => {
                                                h584
                                            }
                                        }, v3.clone(), f77(match v2.clone() {
                                            Adt28::C0(_, _, h585, _, _) => {
                                                h585
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, h586, _, _) => {
                                                h586
                                            }
                                        }), f78(match v2 {
                                            Adt28::C0(_, _, _, h587, _) => {
                                                h587
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, _, h588, _) => {
                                                h588
                                            }
                                        }), match v54 {
                                            Adt28::C0(_, _, _, _, h589) => {
                                                h589
                                            }
                                        });
                                        Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v55, v51))
                                    }
                                    Err(v56) => {
                                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v56)))
                                    }
                                }
                            }
                            Err(v57) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v57)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
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

pub fn f80(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m595 = v1;
    match m595 {
        Adt17::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C9(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C11(_, r596) => {
            let v36 = (*r596).clone();
            f115(v0, v36)
        }
        Adt17::C12 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f81(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    f111(Fn5::F116(v1), List::<Adt12>::nil(), v0)
}

pub fn f82(v0: List<Adt12>, v1: Adt12, v2: List<Adt12>, v3: List<Adt12>) -> Option<Adt12> {
    f117(Fn7::F118(v2, v3, v1), None::<Adt12>, v0)
}

pub fn f83(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>, v2: List<Adt12>) -> Option<Adt12> {
    f119(Fn8::F120(v1, v2), None::<Adt12>, v0)
}

pub fn f84(p602: List<(Adt12, u64)>, v1: Adt26, p603: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m601 = (p602, p603);
    {
        let mut m601 = m601;
        loop {
            let step = {
                let (v0, v2) = m601;
                let m597 = v2;
                match m597 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(r599, r600) => {
                        let v38 = (*r599).clone();
                        let v39 = (*r600).clone();
                        let m598 = f49(v0, v1.clone(), v38, false)?;
                        match m598 {
                            Ok(v40) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v40, v39))
                            }
                            Err(v41) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v41)))
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<(Adt12, u64)>, v1: Adt12) -> Option<u64> {
    f121(Fn9::F122(v1), None::<u64>, v0)
}

pub fn f86(p609: List<(Adt12, u64)>, v1: Adt26, p610: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m608 = (p609, p610);
    {
        let mut m608 = m608;
        loop {
            let step = {
                let (v0, v2) = m608;
                let m604 = v2;
                match m604 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, r606, r607) => {
                        let v48 = (*r606).clone();
                        let v49 = (*r607).clone();
                        let m605 = f32(v0, v1.clone(), v48, v3)?;
                        match m605 {
                            Ok(v50) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v50, v49))
                            }
                            Err(v51) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v51)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Adt14) -> Option<Adt12> {
    let m611 = v0;
    match m611 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(v2) => {
            Some(v2)
        }
        Adt14::C2(_) => {
            None::<Adt12>
        }
        Adt14::C3(_, _) => {
            None::<Adt12>
        }
        Adt14::C4(_, _, _) => {
            None::<Adt12>
        }
        Adt14::C5 => {
            None::<Adt12>
        }
        Adt14::C6(_, _) => {
            None::<Adt12>
        }
    }
}

pub fn f88(v0: Adt2) -> Option<u64> {
    let m612 = v0;
    match m612 {
        Adt2::C0(_) => {
            None::<u64>
        }
        Adt2::C1(_) => {
            None::<u64>
        }
        Adt2::C2(_, _) => {
            None::<u64>
        }
        Adt2::C3(_, _, _) => {
            None::<u64>
        }
        Adt2::C4(v8, v9) => {
            let m613 = f6(v8);
            if m613 {
                Some(v9)
            } else {
                None::<u64>
            }
        }
        Adt2::C5 => {
            None::<u64>
        }
        Adt2::C6(_, _) => {
            None::<u64>
        }
    }
}

pub fn f89(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m614 = f123(v0, v1);
    match m614 {
        None => {
            false
        }
        Some(v3) => {
            let a615 = v3;
            let a616 = v2;
            equal(a615, a616)
        }
    }
}

pub fn f90(v0: Fn4, p620: bool, p621: List<Adt12>) -> bool {
    let m619 = (p620, p621);
    {
        let mut m619 = m619;
        loop {
            let step = {
                let (v1, v2) = m619;
                let m617 = v2;
                match m617.uncons() {
                    None => {
                        Err::<(bool, List<Adt12>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt12>), bool>(({
                            let c618: Fn4 = v0.clone();
                            c618.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m619 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: Adt12, v1: bool, v2: Adt12) -> bool {
    let v3: bool = f124(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f92(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f125(Fn10::F126(v1), None::<u64>, v0)
}

pub fn f93(v0: Adt19) -> bool {
    let m622 = v0;
    match m622 {
        Adt19::C0 => {
            true
        }
        Adt19::C1 => {
            false
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

pub fn f94(v0: Adt14) -> bool {
    let m623 = v0;
    match m623 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            false
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(r624, r625) => {
            let v9 = (*r624).clone();
            let v10 = (*r625).clone();
            let v11: bool = f127(v9);
            let v12: bool = f94(v10);
            if v11 {
                true
            } else {
                v12
            }
        }
    }
}

pub fn f95(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> Option<List<u64>> {
    f128(Fn11::F129(v1), None::<List<u64>>, v0)
}

pub fn f96(p636: List<u64>, p637: u64, p638: Adt14, v3: Bytes) -> R<Result<List<u64>, Bytes>> {
    let m635 = (p636, (p637, p638));
    {
        let mut m635 = m635;
        loop {
            let step = {
                let (v0, (v1, v2)) = m635;
                let m626 = v2;
                match m626 {
                    Adt14::C0(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C1(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C2(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C3(_, _) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C4(_, _, _) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C5 => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C6(r633, r634) => {
                        let v12 = (*r633).clone();
                        let v13 = (*r634).clone();
                        let v14: bool = f127(v12);
                        let m627 = v14;
                        if m627 {
                            Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((v0, ({
                                let a628 = v1;
                                let a629 = 1u64;
                                nat_add(a628, a629)?
                            }, v13)))
                        } else {
                            let m630 = f130(v0.clone(), v1);
                            if m630 {
                                Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Err::<List<u64>, Bytes>(f4(f1(Str::lit("ownership: component ")), f4(f10(v1)?, f4(f1(Str::lit(" of ")), f4(v3.clone(), f1(Str::lit(" is moved twice")))))))))
                            } else {
                                Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((List::cons(v1, v0), ({
                                    let a631 = v1;
                                    let a632 = 1u64;
                                    nat_add(a631, a632)?
                                }, v13)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> List<(Adt12, List<u64>)> {
    List::cons((v1.clone(), v2), f131(v0, v1))
}

pub fn f98(v0: List<Adt29>) -> Bytes {
    let m639 = f132(Fn12::F133, None::<Bytes>, v0);
    match m639 {
        None => {
            f1(Str::lit("a read"))
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f99(v0: Adt30, v1: Adt17) -> R<Adt30> {
    let m640 = v1;
    match m640 {
        Adt17::C0(v2, v3) => {
            let m641 = f134(v2.clone());
            if m641 {
                Ok(v0)
            } else {
                Ok(f135(v0, f136(v2.clone()), v3, f137(v2)))
            }
        }
        Adt17::C1(v4, _, v6) => {
            Ok(f138(v0, f139(v4), v6))
        }
        Adt17::C2(v7, r650, v9) => {
            let v8 = (*r650).clone();
            let v10: Adt30 = f138(v0, f140(v7), v9);
            f99(v10, v8)
        }
        Adt17::C3(v11, r651, _, v14) => {
            let v12 = (*r651).clone();
            let v18: Adt30 = {
                let m642 = v11;
                match m642 {
                    Adt21::C0(_) => {
                        f138(v0, f1(Str::lit("call:function")), v14)
                    }
                    Adt21::C1(v17) => {
                        f135(v0.clone(), f4(f1(Str::lit("call:runtime:")), f141(f142(f70(match v0.clone() {
                            Adt30::C0(_, _, h643) => {
                                h643
                            }
                        }, v17.clone()))?)), v14, f143(match v0 {
                            Adt30::C0(_, _, h644) => {
                                h644
                            }
                        }, v17))
                    }
                    Adt21::C2(_) => {
                        f138(v0, f1(Str::lit("call:machine")), v14)
                    }
                }
            };
            f99(v18, v12)
        }
        Adt17::C4(_, r652, _, v22) => {
            let v20 = (*r652).clone();
            let v23: Adt30 = f138(v0, f1(Str::lit("apply")), v22);
            f99(v23, v20)
        }
        Adt17::C5(v24, r653, v26) => {
            let v25 = (*r653).clone();
            let v27: Adt30 = f138(f144(v0, v24.clone()), f4(f1(Str::lit("construct:")), f145(v24)), v26);
            f99(v27, v25)
        }
        Adt17::C6(r654, r655, v30) => {
            let v28 = (*r654).clone();
            let v29 = (*r655).clone();
            let v31: Adt30 = f138(v0, f1(Str::lit("construct:pair")), v30);
            let v32: Adt30 = f99(v31, v28)?;
            f99(v32, v29)
        }
        Adt17::C7(r656, r657, r658, v36) => {
            let v33 = (*r656).clone();
            let v34 = (*r657).clone();
            let v35 = (*r658).clone();
            let v37: Adt30 = f138(v0, f1(Str::lit("if")), v36);
            let v38: Adt30 = f99(v37, v33)?;
            let v39: Adt30 = f99(v38, v34)?;
            f99(v39, v35)
        }
        Adt17::C8(r659, r660, v42) => {
            let v40 = (*r659).clone();
            let v41 = (*r660).clone();
            let v43: Adt30 = {
                let m645 = f146(v41.clone());
                if m645 {
                    f138(v0, f1(Str::lit("match:empty")), v42)
                } else {
                    let m646 = f147(v40.clone());
                    if m646 {
                        f138(v0, f1(Str::lit("match:list")), v42)
                    } else {
                        f148(v0, v41.clone(), v42)
                    }
                }
            };
            let v44: Adt30 = f99(v43, v40)?;
            f99(v44, v41)
        }
        Adt17::C9(r661) => {
            let v45 = (*r661).clone();
            f99(v0, v45)
        }
        Adt17::C10(v46, _, v48) => {
            let m647 = f149(v46);
            if m647 {
                Ok(f138(v0, f1(Str::lit("match:nat")), v48))
            } else {
                Ok(v0)
            }
        }
        Adt17::C11(r662, r663) => {
            let v49 = (*r662).clone();
            let v50 = (*r663).clone();
            let v51: Adt30 = f99(v0, v49)?;
            f99(v51, v50)
        }
        Adt17::C12 => {
            Ok(v0)
        }
        Adt17::C13(r664, r665) => {
            let v52 = (*r664).clone();
            let v53 = (*r665).clone();
            let v54: Adt30 = f99(v0, v52)?;
            f99(v54, v53)
        }
        Adt17::C14(v55, v56, v57, r666, v59, r667) => {
            let v58 = (*r666).clone();
            let v60 = (*r667).clone();
            let v61: Bytes = {
                let m648 = f150(v55);
                if m648 {
                    f1(Str::lit("destructure:pair"))
                } else {
                    f1(Str::lit("let"))
                }
            };
            let v62: Adt30 = {
                let m649 = v56;
                if m649 {
                    f151(v0, v57)
                } else {
                    v0
                }
            };
            let v63: Adt30 = f138(v62, v61, v59);
            let v64: Adt30 = f99(v63, v58)?;
            f99(v64, v60)
        }
        Adt17::C15(_, r668, r669) => {
            let v66 = (*r668).clone();
            let v67 = (*r669).clone();
            let v68: Adt30 = f99(v0, v66)?;
            f99(v68, v67)
        }
        Adt17::C16(_, r670) => {
            let v70 = (*r670).clone();
            f99(v0, v70)
        }
    }
}

pub fn f100(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> bool {
    let m671 = f95(v0, v1);
    match m671 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f101(v0: Adt24) -> bool {
    let m672 = v0;
    match m672 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            true
        }
    }
}

pub fn f102(v0: Adt17) -> bool {
    let m673 = v0;
    match m673 {
        Adt17::C0(v1, _) => {
            f134(v1)
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
            false
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

pub fn f103(v0: u64) -> u64 {
    let a674 = v0;
    let a675 = 0u64;
    if nat_eq(a674, a675) {
        2u64
    } else {
        let a676 = v0;
        let a677 = 1u64;
        if nat_eq(a676, a677) {
            2u64
        } else {
            let a678 = v0;
            let a679 = 2u64;
            if nat_eq(a678, a679) {
                2u64
            } else {
                let a680 = v0;
                let a681 = 3u64;
                if nat_eq(a680, a681) {
                    2u64
                } else {
                    let a682 = v0;
                    let a683 = 4u64;
                    if nat_eq(a682, a683) {
                        1u64
                    } else {
                        let a684 = v0;
                        let a685 = 5u64;
                        if nat_eq(a684, a685) {
                            2u64
                        } else {
                            let a686 = v0;
                            let a687 = 6u64;
                            if nat_eq(a686, a687) {
                                2u64
                            } else {
                                let a688 = v0;
                                let a689 = 7u64;
                                if nat_eq(a688, a689) {
                                    1u64
                                } else {
                                    let a690 = v0;
                                    let a691 = 8u64;
                                    if nat_eq(a690, a691) {
                                        2u64
                                    } else {
                                        let a692 = v0;
                                        let a693 = 9u64;
                                        if nat_eq(a692, a693) {
                                            2u64
                                        } else {
                                            let a694 = v0;
                                            let a695 = 10u64;
                                            if nat_eq(a694, a695) {
                                                2u64
                                            } else {
                                                let a696 = v0;
                                                let a697 = 11u64;
                                                if nat_eq(a696, a697) {
                                                    1u64
                                                } else {
                                                    let a698 = v0;
                                                    let a699 = 12u64;
                                                    if nat_eq(a698, a699) {
                                                        2u64
                                                    } else {
                                                        let a700 = v0;
                                                        let a701 = 13u64;
                                                        if nat_eq(a700, a701) {
                                                            2u64
                                                        } else {
                                                            let a702 = v0;
                                                            let a703 = 14u64;
                                                            if nat_eq(a702, a703) {
                                                                2u64
                                                            } else {
                                                                let a704 = v0;
                                                                let a705 = 15u64;
                                                                if nat_eq(a704, a705) {
                                                                    3u64
                                                                } else {
                                                                    let a706 = v0;
                                                                    let a707 = 16u64;
                                                                    if nat_eq(a706, a707) {
                                                                        3u64
                                                                    } else {
                                                                        let a708 = v0;
                                                                        let a709 = 17u64;
                                                                        if nat_eq(a708, a709) {
                                                                            2u64
                                                                        } else {
                                                                            let a710 = v0;
                                                                            let a711 = 18u64;
                                                                            if nat_eq(a710, a711) {
                                                                                2u64
                                                                            } else {
                                                                                let a712 = v0;
                                                                                let a713 = 19u64;
                                                                                if nat_eq(a712, a713) {
                                                                                    2u64
                                                                                } else {
                                                                                    let a714 = v0;
                                                                                    let a715 = 20u64;
                                                                                    if nat_eq(a714, a715) {
                                                                                        3u64
                                                                                    } else {
                                                                                        let a716 = v0;
                                                                                        let a717 = 21u64;
                                                                                        if nat_eq(a716, a717) {
                                                                                            2u64
                                                                                        } else {
                                                                                            let a718 = v0;
                                                                                            let a719 = 22u64;
                                                                                            if nat_eq(a718, a719) {
                                                                                                2u64
                                                                                            } else {
                                                                                                let a720 = v0;
                                                                                                let a721 = 23u64;
                                                                                                if nat_eq(a720, a721) {
                                                                                                    2u64
                                                                                                } else {
                                                                                                    let a722 = v0;
                                                                                                    let a723 = 24u64;
                                                                                                    if nat_eq(a722, a723) {
                                                                                                        1u64
                                                                                                    } else {
                                                                                                        let a724 = v0;
                                                                                                        let a725 = 25u64;
                                                                                                        if nat_eq(a724, a725) {
                                                                                                            3u64
                                                                                                        } else {
                                                                                                            let a726 = v0;
                                                                                                            let a727 = 26u64;
                                                                                                            if nat_eq(a726, a727) {
                                                                                                                1u64
                                                                                                            } else {
                                                                                                                let a728 = v0;
                                                                                                                let a729 = 27u64;
                                                                                                                if nat_eq(a728, a729) {
                                                                                                                    2u64
                                                                                                                } else {
                                                                                                                    let a730 = v0;
                                                                                                                    let a731 = 28u64;
                                                                                                                    if nat_eq(a730, a731) {
                                                                                                                        2u64
                                                                                                                    } else {
                                                                                                                        let a732 = v0;
                                                                                                                        let a733 = 29u64;
                                                                                                                        if nat_eq(a732, a733) {
                                                                                                                            3u64
                                                                                                                        } else {
                                                                                                                            let a734 = v0;
                                                                                                                            let a735 = 30u64;
                                                                                                                            if nat_eq(a734, a735) {
                                                                                                                                2u64
                                                                                                                            } else {
                                                                                                                                let a736 = v0;
                                                                                                                                let a737 = 31u64;
                                                                                                                                if nat_eq(a736, a737) {
                                                                                                                                    1u64
                                                                                                                                } else {
                                                                                                                                    let a738 = v0;
                                                                                                                                    let a739 = 32u64;
                                                                                                                                    if nat_eq(a738, a739) {
                                                                                                                                        1u64
                                                                                                                                    } else {
                                                                                                                                        let a740 = v0;
                                                                                                                                        let a741 = 33u64;
                                                                                                                                        if nat_eq(a740, a741) {
                                                                                                                                            2u64
                                                                                                                                        } else {
                                                                                                                                            let a742 = v0;
                                                                                                                                            let a743 = 34u64;
                                                                                                                                            if nat_eq(a742, a743) {
                                                                                                                                                3u64
                                                                                                                                            } else {
                                                                                                                                                let a744 = v0;
                                                                                                                                                let a745 = 35u64;
                                                                                                                                                if nat_eq(a744, a745) {
                                                                                                                                                    1u64
                                                                                                                                                } else {
                                                                                                                                                    let a746 = v0;
                                                                                                                                                    let a747 = 36u64;
                                                                                                                                                    if nat_eq(a746, a747) {
                                                                                                                                                        1u64
                                                                                                                                                    } else {
                                                                                                                                                        let a748 = v0;
                                                                                                                                                        let a749 = 37u64;
                                                                                                                                                        if nat_eq(a748, a749) {
                                                                                                                                                            2u64
                                                                                                                                                        } else {
                                                                                                                                                            let a750 = v0;
                                                                                                                                                            let a751 = 38u64;
                                                                                                                                                            if nat_eq(a750, a751) {
                                                                                                                                                                3u64
                                                                                                                                                            } else {
                                                                                                                                                                let a752 = v0;
                                                                                                                                                                let a753 = 39u64;
                                                                                                                                                                if nat_eq(a752, a753) {
                                                                                                                                                                    2u64
                                                                                                                                                                } else {
                                                                                                                                                                    let a754 = v0;
                                                                                                                                                                    let a755 = 40u64;
                                                                                                                                                                    if nat_eq(a754, a755) {
                                                                                                                                                                        1u64
                                                                                                                                                                    } else {
                                                                                                                                                                        let a756 = v0;
                                                                                                                                                                        let a757 = 41u64;
                                                                                                                                                                        if nat_eq(a756, a757) {
                                                                                                                                                                            1u64
                                                                                                                                                                        } else {
                                                                                                                                                                            let a758 = v0;
                                                                                                                                                                            let a759 = 42u64;
                                                                                                                                                                            if nat_eq(a758, a759) {
                                                                                                                                                                                1u64
                                                                                                                                                                            } else {
                                                                                                                                                                                let a760 = v0;
                                                                                                                                                                                let a761 = 43u64;
                                                                                                                                                                                if nat_eq(a760, a761) {
                                                                                                                                                                                    1u64
                                                                                                                                                                                } else {
                                                                                                                                                                                    0u64
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

pub fn f104(v0: u64) -> Bytes {
    let a762 = v0;
    let a763 = 0u64;
    if nat_eq(a762, a763) {
        f1(Str::lit("checked_add"))
    } else {
        let a764 = v0;
        let a765 = 1u64;
        if nat_eq(a764, a765) {
            f1(Str::lit("checked_sub"))
        } else {
            let a766 = v0;
            let a767 = 2u64;
            if nat_eq(a766, a767) {
                f1(Str::lit("checked_mul"))
            } else {
                let a768 = v0;
                let a769 = 3u64;
                if nat_eq(a768, a769) {
                    f1(Str::lit("checked_div"))
                } else {
                    let a770 = v0;
                    let a771 = 4u64;
                    if nat_eq(a770, a771) {
                        f1(Str::lit("checked_neg"))
                    } else {
                        let a772 = v0;
                        let a773 = 5u64;
                        if nat_eq(a772, a773) {
                            f1(Str::lit("checked_shl"))
                        } else {
                            let a774 = v0;
                            let a775 = 6u64;
                            if nat_eq(a774, a775) {
                                f1(Str::lit("checked_shr"))
                            } else {
                                let a776 = v0;
                                let a777 = 7u64;
                                if nat_eq(a776, a777) {
                                    f1(Str::lit("convert"))
                                } else {
                                    let a778 = v0;
                                    let a779 = 8u64;
                                    if nat_eq(a778, a779) {
                                        f1(Str::lit("bit_and"))
                                    } else {
                                        let a780 = v0;
                                        let a781 = 9u64;
                                        if nat_eq(a780, a781) {
                                            f1(Str::lit("bit_or"))
                                        } else {
                                            let a782 = v0;
                                            let a783 = 10u64;
                                            if nat_eq(a782, a783) {
                                                f1(Str::lit("bit_xor"))
                                            } else {
                                                let a784 = v0;
                                                let a785 = 11u64;
                                                if nat_eq(a784, a785) {
                                                    f1(Str::lit("bit_not"))
                                                } else {
                                                    let a786 = v0;
                                                    let a787 = 12u64;
                                                    if nat_eq(a786, a787) {
                                                        f1(Str::lit("nat_eq"))
                                                    } else {
                                                        let a788 = v0;
                                                        let a789 = 13u64;
                                                        if nat_eq(a788, a789) {
                                                            f1(Str::lit("nat_le"))
                                                        } else {
                                                            let a790 = v0;
                                                            let a791 = 14u64;
                                                            if nat_eq(a790, a791) {
                                                                f1(Str::lit("nat_lt"))
                                                            } else {
                                                                let a792 = v0;
                                                                let a793 = 15u64;
                                                                if nat_eq(a792, a793) {
                                                                    f1(Str::lit("nat_rem"))
                                                                } else {
                                                                    let a794 = v0;
                                                                    let a795 = 16u64;
                                                                    if nat_eq(a794, a795) {
                                                                        f1(Str::lit("int_rem"))
                                                                    } else {
                                                                        let a796 = v0;
                                                                        let a797 = 17u64;
                                                                        if nat_eq(a796, a797) {
                                                                            f1(Str::lit("nat_add"))
                                                                        } else {
                                                                            let a798 = v0;
                                                                            let a799 = 18u64;
                                                                            if nat_eq(a798, a799) {
                                                                                f1(Str::lit("nat_sub"))
                                                                            } else {
                                                                                let a800 = v0;
                                                                                let a801 = 19u64;
                                                                                if nat_eq(a800, a801) {
                                                                                    f1(Str::lit("nat_mul"))
                                                                                } else {
                                                                                    let a802 = v0;
                                                                                    let a803 = 20u64;
                                                                                    if nat_eq(a802, a803) {
                                                                                        f1(Str::lit("nat_quot"))
                                                                                    } else {
                                                                                        let a804 = v0;
                                                                                        let a805 = 21u64;
                                                                                        if nat_eq(a804, a805) {
                                                                                            f1(Str::lit("int_add"))
                                                                                        } else {
                                                                                            let a806 = v0;
                                                                                            let a807 = 22u64;
                                                                                            if nat_eq(a806, a807) {
                                                                                                f1(Str::lit("int_sub"))
                                                                                            } else {
                                                                                                let a808 = v0;
                                                                                                let a809 = 23u64;
                                                                                                if nat_eq(a808, a809) {
                                                                                                    f1(Str::lit("int_mul"))
                                                                                                } else {
                                                                                                    let a810 = v0;
                                                                                                    let a811 = 24u64;
                                                                                                    if nat_eq(a810, a811) {
                                                                                                        f1(Str::lit("int_neg"))
                                                                                                    } else {
                                                                                                        let a812 = v0;
                                                                                                        let a813 = 25u64;
                                                                                                        if nat_eq(a812, a813) {
                                                                                                            f1(Str::lit("int_quot"))
                                                                                                        } else {
                                                                                                            let a814 = v0;
                                                                                                            let a815 = 26u64;
                                                                                                            if nat_eq(a814, a815) {
                                                                                                                f1(Str::lit("list_length"))
                                                                                                            } else {
                                                                                                                let a816 = v0;
                                                                                                                let a817 = 27u64;
                                                                                                                if nat_eq(a816, a817) {
                                                                                                                    f1(Str::lit("list_append"))
                                                                                                                } else {
                                                                                                                    let a818 = v0;
                                                                                                                    let a819 = 28u64;
                                                                                                                    if nat_eq(a818, a819) {
                                                                                                                        f1(Str::lit("list_index"))
                                                                                                                    } else {
                                                                                                                        let a820 = v0;
                                                                                                                        let a821 = 29u64;
                                                                                                                        if nat_eq(a820, a821) {
                                                                                                                            f1(Str::lit("list_slice"))
                                                                                                                        } else {
                                                                                                                            let a822 = v0;
                                                                                                                            let a823 = 30u64;
                                                                                                                            if nat_eq(a822, a823) {
                                                                                                                                f1(Str::lit("bytes_append"))
                                                                                                                            } else {
                                                                                                                                let a824 = v0;
                                                                                                                                let a825 = 31u64;
                                                                                                                                if nat_eq(a824, a825) {
                                                                                                                                    f1(Str::lit("bytes_length"))
                                                                                                                                } else {
                                                                                                                                    let a826 = v0;
                                                                                                                                    let a827 = 32u64;
                                                                                                                                    if nat_eq(a826, a827) {
                                                                                                                                        f1(Str::lit("string_length"))
                                                                                                                                    } else {
                                                                                                                                        let a828 = v0;
                                                                                                                                        let a829 = 33u64;
                                                                                                                                        if nat_eq(a828, a829) {
                                                                                                                                            f1(Str::lit("bytes_index"))
                                                                                                                                        } else {
                                                                                                                                            let a830 = v0;
                                                                                                                                            let a831 = 34u64;
                                                                                                                                            if nat_eq(a830, a831) {
                                                                                                                                                f1(Str::lit("bytes_slice"))
                                                                                                                                            } else {
                                                                                                                                                let a832 = v0;
                                                                                                                                                let a833 = 35u64;
                                                                                                                                                if nat_eq(a832, a833) {
                                                                                                                                                    f1(Str::lit("utf8_encode"))
                                                                                                                                                } else {
                                                                                                                                                    let a834 = v0;
                                                                                                                                                    let a835 = 36u64;
                                                                                                                                                    if nat_eq(a834, a835) {
                                                                                                                                                        f1(Str::lit("utf8_decode"))
                                                                                                                                                    } else {
                                                                                                                                                        let a836 = v0;
                                                                                                                                                        let a837 = 37u64;
                                                                                                                                                        if nat_eq(a836, a837) {
                                                                                                                                                            f1(Str::lit("bytes_compare"))
                                                                                                                                                        } else {
                                                                                                                                                            let a838 = v0;
                                                                                                                                                            let a839 = 38u64;
                                                                                                                                                            if nat_eq(a838, a839) {
                                                                                                                                                                f1(Str::lit("string_split"))
                                                                                                                                                            } else {
                                                                                                                                                                let a840 = v0;
                                                                                                                                                                let a841 = 39u64;
                                                                                                                                                                if nat_eq(a840, a841) {
                                                                                                                                                                    f1(Str::lit("string_join"))
                                                                                                                                                                } else {
                                                                                                                                                                    let a842 = v0;
                                                                                                                                                                    let a843 = 40u64;
                                                                                                                                                                    if nat_eq(a842, a843) {
                                                                                                                                                                        f1(Str::lit("format_decimal"))
                                                                                                                                                                    } else {
                                                                                                                                                                        let a844 = v0;
                                                                                                                                                                        let a845 = 41u64;
                                                                                                                                                                        if nat_eq(a844, a845) {
                                                                                                                                                                            f1(Str::lit("parse_decimal"))
                                                                                                                                                                        } else {
                                                                                                                                                                            let a846 = v0;
                                                                                                                                                                            let a847 = 42u64;
                                                                                                                                                                            if nat_eq(a846, a847) {
                                                                                                                                                                                f1(Str::lit("parse_decimal_int"))
                                                                                                                                                                            } else {
                                                                                                                                                                                let a848 = v0;
                                                                                                                                                                                let a849 = 43u64;
                                                                                                                                                                                if nat_eq(a848, a849) {
                                                                                                                                                                                    f1(Str::lit("format_decimal_int"))
                                                                                                                                                                                } else {
                                                                                                                                                                                    Bytes::lit(&[])
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

pub fn f105(v0: List<Adt25>, v1: u64) -> Option<Adt25> {
    f152(Fn13::F153(v1), None::<Adt25>, v0)
}

pub fn f106(v0: Adt4) -> Bytes {
    let m850 = v0;
    match m850 {
        Adt4::C0 => {
            f1(Str::lit("u8"))
        }
        Adt4::C1 => {
            f1(Str::lit("u16"))
        }
        Adt4::C2 => {
            f1(Str::lit("u32"))
        }
        Adt4::C3 => {
            f1(Str::lit("u64"))
        }
        Adt4::C4 => {
            f1(Str::lit("i8"))
        }
        Adt4::C5 => {
            f1(Str::lit("i16"))
        }
        Adt4::C6 => {
            f1(Str::lit("i32"))
        }
        Adt4::C7 => {
            f1(Str::lit("i64"))
        }
    }
}

pub fn f107(v0: Option<Adt4>) -> Adt4 {
    let m851 = v0;
    match m851 {
        None => {
            Adt4::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f108(v0: Adt2) -> bool {
    let m852 = v0;
    match m852 {
        Adt2::C0(v1) => {
            let m853 = v1;
            match m853 {
                Adt3::C0 => {
                    false
                }
                Adt3::C1 => {
                    false
                }
                Adt3::C2 => {
                    false
                }
                Adt3::C3 => {
                    false
                }
                Adt3::C4 => {
                    false
                }
                Adt3::C5 => {
                    false
                }
                Adt3::C6 => {
                    false
                }
                Adt3::C7 => {
                    true
                }
                Adt3::C8 => {
                    false
                }
            }
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, _) => {
            false
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(_, _) => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f109(v0: Adt17) -> R<bool> {
    let m854 = v0;
    match m854 {
        Adt17::C0(_, _) => {
            Ok(false)
        }
        Adt17::C1(_, _, _) => {
            Ok(false)
        }
        Adt17::C2(_, _, _) => {
            Ok(false)
        }
        Adt17::C3(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C4(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C5(_, _, _) => {
            Ok(false)
        }
        Adt17::C6(_, _, _) => {
            Ok(false)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C8(_, _, _) => {
            Ok(false)
        }
        Adt17::C9(_) => {
            Ok(false)
        }
        Adt17::C10(_, _, _) => {
            Ok(false)
        }
        Adt17::C11(_, _) => {
            Ok(false)
        }
        Adt17::C12 => {
            Ok(false)
        }
        Adt17::C13(r857, r858) => {
            let v36 = (*r857).clone();
            let v37 = (*r858).clone();
            let v38: bool = {
                let a855 = f76(v37)?;
                let a856 = 0u64;
                nat_eq(a855, a856)
            };
            let v39: bool = f102(v36);
            if v38 {
                Ok(v39)
            } else {
                Ok(false)
            }
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(false)
        }
        Adt17::C15(_, _, _) => {
            Ok(false)
        }
        Adt17::C16(_, _) => {
            Ok(false)
        }
    }
}

pub fn f110(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m860 = f18(match v0 {
        Adt26::C0(_, h859, _, _) => {
            h859
        }
    }, v1);
    match m860 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f92(v3, v2)
        }
    }
}

pub fn f111(v0: Fn5, p864: List<Adt12>, p865: List<Adt12>) -> List<Adt12> {
    let m863 = (p864, p865);
    {
        let mut m863 = m863;
        loop {
            let step = {
                let (v1, v2) = m863;
                let m861 = v2;
                match m861.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c862: Fn5 = v0.clone();
                            c862.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    let v2: bool = f57(v0.clone(), v1.clone());
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f113(v0: Fn6, p869: List<(Adt12, List<u64>)>, p870: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    let m868 = (p869, p870);
    {
        let mut m868 = m868;
        loop {
            let step = {
                let (v1, v2) = m868;
                let m866 = v2;
                match m866.uncons() {
                    None => {
                        Err::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(({
                            let c867: Fn6 = v0.clone();
                            c867.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<(Adt12, List<u64>)>, v1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let m872 = f95(v0.clone(), {
        let (h871, _) = v1.clone();
        h871
    });
    match m872 {
        None => {
            f97(v0, {
                let (h875, _) = v1;
                h875
            }, {
                let (_, h876) = v1;
                h876
            })
        }
        Some(v2) => {
            f97(v0, {
                let (h873, _) = v1;
                h873
            }, f154(v2, {
                let (_, h874) = v1;
                h874
            }))
        }
    }
}

pub fn f115(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m877 = v1;
    match m877 {
        Adt17::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C5(v18, r880, _) => {
            let v19 = (*r880).clone();
            let m878 = f74(v18);
            if m878 {
                f155(v0, v19)
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, r881, r882, _) => {
            let v25 = (*r881).clone();
            let v26 = (*r882).clone();
            let m879 = f80(v0.clone(), v25)?;
            match m879 {
                Ok(_) => {
                    f80(v0, v26)
                }
                Err(v29) => {
                    Ok(Err::<(), Bytes>(v29))
                }
            }
        }
        Adt17::C8(_, r883, _) => {
            let v31 = (*r883).clone();
            f156(v0, v31)
        }
        Adt17::C9(r884) => {
            let v33 = (*r884).clone();
            f80(v0, v33)
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C12 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f116(v0: Adt12, v1: List<Adt12>, v2: Adt12) -> List<Adt12> {
    let v3: bool = f124(v2.clone(), v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f117(v0: Fn7, p888: Option<Adt12>, p889: List<Adt12>) -> Option<Adt12> {
    let m887 = (p888, p889);
    {
        let mut m887 = m887;
        loop {
            let step = {
                let (v1, v2) = m887;
                let m885 = v2;
                match m885.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c886: Fn7 = v0.clone();
                            c886.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: List<Adt12>, v1: List<Adt12>, v2: Adt12, v3: Option<Adt12>, v4: Adt12) -> Option<Adt12> {
    let v5: bool = f124(v4.clone(), v2);
    let v6: bool = f57(v0, v4.clone());
    let v7: bool = f57(v1, v4.clone());
    let v8: bool = if v5 {
        false
    } else {
        if v6 {
            false
        } else {
            v7
        }
    };
    if v8 {
        f157(v3, v4)
    } else {
        v3
    }
}

pub fn f119(v0: Fn8, p893: Option<Adt12>, p894: List<(Adt12, List<u64>)>) -> Option<Adt12> {
    let m892 = (p893, p894);
    {
        let mut m892 = m892;
        loop {
            let step = {
                let (v1, v2) = m892;
                let m890 = v2;
                match m890.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(({
                            let c891: Fn8 = v0.clone();
                            c891.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m892 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: List<(Adt12, List<u64>)>, v1: List<Adt12>, v2: Option<Adt12>, v3: (Adt12, List<u64>)) -> Option<Adt12> {
    let v4: bool = f57(v1, {
        let (h895, _) = v3.clone();
        h895
    });
    let v5: bool = f158(v0, {
        let (h896, _) = v3.clone();
        h896
    }, {
        let (_, h897) = v3;
        h897
    });
    let v6: bool = if v4 {
        !v5
    } else {
        false
    };
    if v6 {
        f157(v2, {
            let (h898, _) = v3;
            h898
        })
    } else {
        v2
    }
}

pub fn f121(v0: Fn9, p902: Option<u64>, p903: List<(Adt12, u64)>) -> Option<u64> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v1, v2) = m901;
                let m899 = v2;
                match m899.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(({
                            let c900: Fn9 = v0.clone();
                            c900.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt12, v1: Option<u64>, v2: (Adt12, u64)) -> Option<u64> {
    let m904 = v1;
    match m904 {
        None => {
            let v4: bool = f124({
                let (h905, _) = v2;
                h905
            }, v0);
            if v4 {
                Some({
                    let (_, h906) = v2;
                    h906
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

pub fn f123(v0: Bytes, v1: u64) -> Option<u8> {
    let a907 = v0;
    let a908 = v1;
    index_bytes(a907, a908)
}

pub fn f124(v0: Adt12, v1: Adt12) -> bool {
    let a909 = f159(v0.clone());
    let a910 = f159(v1.clone());
    if nat_eq(a909, a910) {
        let a911 = f160(v0.clone());
        let a912 = f160(v1.clone());
        if nat_eq(a911, a912) {
            let a913 = f161(v0);
            let a914 = f161(v1);
            equal(a913, a914)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f125(v0: Fn10, p918: Option<u64>, p919: List<(u64, u64)>) -> Option<u64> {
    let m917 = (p918, p919);
    {
        let mut m917 = m917;
        loop {
            let step = {
                let (v1, v2) = m917;
                let m915 = v2;
                match m915.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c916: Fn10 = v0.clone();
                            c916.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let m920 = v1;
    match m920 {
        None => {
            let a922 = {
                let (h921, _) = v2;
                h921
            };
            let a923 = v0;
            if nat_eq(a922, a923) {
                Some({
                    let (_, h924) = v2;
                    h924
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

pub fn f127(v0: Adt14) -> bool {
    let m925 = v0;
    match m925 {
        Adt14::C0(v1) => {
            let m926 = v1;
            match m926 {
                Adt15::C0 => {
                    true
                }
                Adt15::C1 => {
                    false
                }
                Adt15::C2 => {
                    false
                }
                Adt15::C3 => {
                    false
                }
            }
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            false
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f128(v0: Fn11, p930: Option<List<u64>>, p931: List<(Adt12, List<u64>)>) -> Option<List<u64>> {
    let m929 = (p930, p931);
    {
        let mut m929 = m929;
        loop {
            let step = {
                let (v1, v2) = m929;
                let m927 = v2;
                match m927.uncons() {
                    None => {
                        Err::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(({
                            let c928: Fn11 = v0.clone();
                            c928.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Adt12, v1: Option<List<u64>>, v2: (Adt12, List<u64>)) -> Option<List<u64>> {
    let m932 = v1;
    match m932 {
        None => {
            let v4: bool = f124({
                let (h933, _) = v2;
                h933
            }, v0);
            if v4 {
                Some({
                    let (_, h934) = v2;
                    h934
                })
            } else {
                None::<List<u64>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f130(v0: List<u64>, v1: u64) -> bool {
    f162(Fn14::F163(v1), false, v0)
}

pub fn f131(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> List<(Adt12, List<u64>)> {
    f113(Fn6::F164(v1), List::<(Adt12, List<u64>)>::nil(), v0)
}

pub fn f132(v0: Fn12, p938: Option<Bytes>, p939: List<Adt29>) -> Option<Bytes> {
    let m937 = (p938, p939);
    {
        let mut m937 = m937;
        loop {
            let step = {
                let (v1, v2) = m937;
                let m935 = v2;
                match m935.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(({
                            let c936: Fn12 = v0.clone();
                            c936.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m937 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Option<Bytes>, v1: Adt29) -> Option<Bytes> {
    let m940 = v0;
    match m940 {
        None => {
            Some(match v1 {
                Adt29::C0(h941, _, _, _) => {
                    h941
                }
            })
        }
        Some(v2) => {
            let m943 = f165(match v1.clone() {
                Adt29::C0(h942, _, _, _) => {
                    h942
                }
            }, v2.clone());
            if m943 {
                Some(match v1 {
                    Adt29::C0(h944, _, _, _) => {
                        h944
                    }
                })
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f134(v0: Adt18) -> bool {
    let m945 = v0;
    match m945 {
        Adt18::C0 => {
            false
        }
        Adt18::C1 => {
            false
        }
        Adt18::C2 => {
            false
        }
        Adt18::C3 => {
            false
        }
        Adt18::C4(_) => {
            false
        }
        Adt18::C5 => {
            false
        }
        Adt18::C6 => {
            false
        }
        Adt18::C7 => {
            false
        }
        Adt18::C8 => {
            true
        }
    }
}

pub fn f135(v0: Adt30, v1: Bytes, v2: Adt9, v3: Option<Adt4>) -> Adt30 {
    let v4: Adt29 = Adt29::C0(v1, match v2.clone() {
        Adt9::C0(h946, _) => {
            h946
        }
    }, match v2 {
        Adt9::C0(_, h947) => {
            h947
        }
    }, v3);
    let m949 = f166(match v0.clone() {
        Adt30::C0(h948, _, _) => {
            h948
        }
    }, v4.clone());
    if m949 {
        v0
    } else {
        Adt30::C0(List::cons(v4, match v0.clone() {
            Adt30::C0(h950, _, _) => {
                h950
            }
        }), match v0.clone() {
            Adt30::C0(_, h951, _) => {
                h951
            }
        }, match v0 {
            Adt30::C0(_, _, h952) => {
                h952
            }
        })
    }
}

pub fn f136(v0: Adt18) -> Bytes {
    let m953 = v0;
    match m953 {
        Adt18::C0 => {
            f1(Str::lit("lit:unit"))
        }
        Adt18::C1 => {
            f1(Str::lit("lit:bool"))
        }
        Adt18::C2 => {
            f1(Str::lit("lit:nat"))
        }
        Adt18::C3 => {
            f1(Str::lit("lit:int"))
        }
        Adt18::C4(_) => {
            f1(Str::lit("lit:fixed"))
        }
        Adt18::C5 => {
            f1(Str::lit("lit:string"))
        }
        Adt18::C6 => {
            f1(Str::lit("lit:bytes"))
        }
        Adt18::C7 => {
            f1(Str::lit("lit:ordering"))
        }
        Adt18::C8 => {
            f1(Str::lit("lit:overflow"))
        }
    }
}

pub fn f137(v0: Adt18) -> Option<Adt4> {
    let m954 = v0;
    match m954 {
        Adt18::C0 => {
            None::<Adt4>
        }
        Adt18::C1 => {
            None::<Adt4>
        }
        Adt18::C2 => {
            None::<Adt4>
        }
        Adt18::C3 => {
            None::<Adt4>
        }
        Adt18::C4(v1) => {
            Some(v1)
        }
        Adt18::C5 => {
            None::<Adt4>
        }
        Adt18::C6 => {
            None::<Adt4>
        }
        Adt18::C7 => {
            None::<Adt4>
        }
        Adt18::C8 => {
            None::<Adt4>
        }
    }
}

pub fn f138(v0: Adt30, v1: Bytes, v2: Adt9) -> Adt30 {
    f135(v0, v1, v2, None::<Adt4>)
}

pub fn f139(v0: Adt19) -> Bytes {
    let m955 = v0;
    match m955 {
        Adt19::C0 => {
            f1(Str::lit("read"))
        }
        Adt19::C1 => {
            f1(Str::lit("read"))
        }
        Adt19::C2 => {
            f1(Str::lit("read"))
        }
        Adt19::C3 => {
            f1(Str::lit("read"))
        }
        Adt19::C4 => {
            f1(Str::lit("box"))
        }
    }
}

pub fn f140(v0: Adt20) -> Bytes {
    let m956 = v0;
    match m956 {
        Adt20::C0 => {
            f1(Str::lit("if"))
        }
        Adt20::C1 => {
            f1(Str::lit("box"))
        }
        Adt20::C2 => {
            f1(Str::lit("widen"))
        }
        Adt20::C3 => {
            f1(Str::lit("succeed"))
        }
    }
}

pub fn f141(v0: Bytes) -> Bytes {
    let m957 = f167(v0.clone(), f1(Str::lit("checked_neg_")));
    if m957 {
        f1(Str::lit("checked_neg"))
    } else {
        let m958 = f167(v0.clone(), f1(Str::lit("format_")));
        if m958 {
            f1(Str::lit("format_decimal"))
        } else {
            let m959 = f167(v0.clone(), f1(Str::lit("parse_")));
            if m959 {
                f1(Str::lit("parse_decimal"))
            } else {
                let m960 = f167(v0.clone(), f1(Str::lit("append_")));
                if m960 {
                    f1(Str::lit("append"))
                } else {
                    let m961 = f167(v0.clone(), f1(Str::lit("length_")));
                    if m961 {
                        f1(Str::lit("length"))
                    } else {
                        let m962 = f167(v0.clone(), f1(Str::lit("index_")));
                        if m962 {
                            f1(Str::lit("index"))
                        } else {
                            let m963 = f167(v0.clone(), f1(Str::lit("slice_")));
                            if m963 {
                                f1(Str::lit("slice"))
                            } else {
                                let m966 = {
                                    let a964 = v0.clone();
                                    let a965 = f1(Str::lit("nat_succ"));
                                    equal(a964, a965)
                                };
                                if m966 {
                                    f1(Str::lit("succ"))
                                } else {
                                    v0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f142(v0: Bytes) -> R<Bytes> {
    let m967 = f168(v0.clone(), f1(Str::lit("::")))?;
    match m967 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f40(v0.clone(), {
                let a968 = v1;
                let a969 = 2u64;
                nat_add(a968, a969)?
            }, {
                let a973 = {
                    let a970 = v0;
                    length_bytes(a970)
                };
                let a974 = {
                    let a971 = v1;
                    let a972 = 2u64;
                    nat_add(a971, a972)?
                };
                nat_sub(a973, a974)
            }))
        }
    }
}

pub fn f143(v0: List<Adt25>, v1: Adt22) -> Option<Adt4> {
    let m976 = f105(v0, match v1.clone() {
        Adt22::C0(h975, _) => {
            h975
        }
    });
    match m976 {
        None => {
            None::<Adt4>
        }
        Some(v2) => {
            let a978 = match v2 {
                Adt25::C0(_, _, h977, _, _) => {
                    h977
                }
            };
            let a979 = 0u64;
            if nat_eq(a978, a979) {
                None::<Adt4>
            } else {
                Some(f107(match v1 {
                    Adt22::C0(_, h980) => {
                        h980
                    }
                }))
            }
        }
    }
}

pub fn f144(v0: Adt30, v1: Adt23) -> Adt30 {
    let m981 = v1;
    match m981 {
        Adt23::C0(v2) => {
            f151(v0, v2)
        }
        Adt23::C1 => {
            v0
        }
        Adt23::C2(v3, v4) => {
            f151(f151(v0, v3), v4)
        }
        Adt23::C3(v5, v6) => {
            f151(f151(v0, v5), v6)
        }
        Adt23::C4(_, _) => {
            v0
        }
        Adt23::C5(_, _) => {
            v0
        }
        Adt23::C6 => {
            v0
        }
        Adt23::C7(v11) => {
            f151(v0, v11)
        }
    }
}

pub fn f145(v0: Adt23) -> Bytes {
    let m982 = v0;
    match m982 {
        Adt23::C0(_) => {
            f1(Str::lit("none"))
        }
        Adt23::C1 => {
            f1(Str::lit("some"))
        }
        Adt23::C2(_, _) => {
            f1(Str::lit("ok"))
        }
        Adt23::C3(_, _) => {
            f1(Str::lit("err"))
        }
        Adt23::C4(_, _) => {
            f1(Str::lit("adt"))
        }
        Adt23::C5(_, _) => {
            f1(Str::lit("closure"))
        }
        Adt23::C6 => {
            f1(Str::lit("cons"))
        }
        Adt23::C7(_) => {
            f1(Str::lit("nil"))
        }
    }
}

pub fn f146(v0: Adt17) -> bool {
    let m983 = v0;
    match m983 {
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

pub fn f147(v0: Adt17) -> bool {
    let m984 = v0;
    match m984 {
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
        Adt17::C10(v31, _, _) => {
            f169(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
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

pub fn f148(p989: Adt30, p990: Adt17, v2: Adt9) -> Adt30 {
    let m988 = (p989, p990);
    {
        let mut m988 = m988;
        loop {
            let step = {
                let (v0, v1) = m988;
                let m985 = v1;
                match m985 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C12 => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C15(v46, _, r987) => {
                        let v48 = (*r987).clone();
                        let m986 = f170(v46);
                        match m986 {
                            None => {
                                Ok::<(Adt30, Adt17), Adt30>((v0, v48))
                            }
                            Some(v49) => {
                                Ok::<(Adt30, Adt17), Adt30>((f138(v0, v49, v2.clone()), v48))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                }
            };
            match step {
                Ok(next) => { m988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Adt24) -> bool {
    let m991 = v0;
    match m991 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f150(v0: Adt14) -> bool {
    let m992 = v0;
    match m992 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            true
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f151(v0: Adt30, v1: Adt2) -> Adt30 {
    let m993 = v1;
    match m993 {
        Adt2::C0(v2) => {
            f171(v0, f4(f1(Str::lit("type:")), f172(v2)))
        }
        Adt2::C1(_) => {
            f171(v0, f1(Str::lit("type:fixed")))
        }
        Adt2::C2(v4, r994) => {
            let v5 = (*r994).clone();
            let v6: Adt30 = f171(v0, f4(f1(Str::lit("type:")), f173(v4)));
            f151(v6, v5)
        }
        Adt2::C3(v7, r995, r996) => {
            let v8 = (*r995).clone();
            let v9 = (*r996).clone();
            let v10: Adt30 = f171(v0, f4(f1(Str::lit("type:")), f174(v7)));
            let v11: Adt30 = f151(v10, v8);
            f151(v11, v9)
        }
        Adt2::C4(v12, _) => {
            f171(v0, f4(f1(Str::lit("type:")), f175(v12)))
        }
        Adt2::C5 => {
            v0
        }
        Adt2::C6(r997, r998) => {
            let v14 = (*r997).clone();
            let v15 = (*r998).clone();
            let v16: Adt30 = f151(v0, v14);
            f151(v16, v15)
        }
    }
}

pub fn f152(v0: Fn13, p1002: Option<Adt25>, p1003: List<Adt25>) -> Option<Adt25> {
    let m1001 = (p1002, p1003);
    {
        let mut m1001 = m1001;
        loop {
            let step = {
                let (v1, v2) = m1001;
                let m999 = v2;
                match m999.uncons() {
                    None => {
                        Err::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(({
                            let c1000: Fn13 = v0.clone();
                            c1000.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: u64, v1: Option<Adt25>, v2: Adt25) -> Option<Adt25> {
    let m1004 = v1;
    match m1004 {
        None => {
            let a1006 = match v2.clone() {
                Adt25::C0(h1005, _, _, _, _) => {
                    h1005
                }
            };
            let a1007 = v0;
            if nat_eq(a1006, a1007) {
                Some(v2)
            } else {
                None::<Adt25>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f154(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f176(Fn15::F177, v0, v1)
}

pub fn f155(v0: List<Adt25>, p1013: Adt17) -> R<Result<(), Bytes>> {
    let m1012 = p1013;
    {
        let mut m1012 = m1012;
        loop {
            let step = {
                let v1 = m1012;
                let m1008 = v1;
                match m1008 {
                    Adt17::C0(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C9(_) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C11(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C12 => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C13(r1010, r1011) => {
                        let v37 = (*r1010).clone();
                        let v38 = (*r1011).clone();
                        let m1009 = f178(v37.clone())?;
                        if m1009 {
                            Ok::<Adt17, R<Result<(), Bytes>>>(v38)
                        } else {
                            Err::<Adt17, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible loop ends with, ")), f4(f62(v0.clone(), v37)?, f1(Str::lit(", is not a fallible value")))))))
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C16(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m1012 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: List<Adt25>, p1019: Adt17) -> R<Result<(), Bytes>> {
    let m1018 = p1019;
    {
        let mut m1018 = m1018;
        loop {
            let step = {
                let v1 = m1018;
                let m1014 = v1;
                match m1014 {
                    Adt17::C0(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C9(_) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C11(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C12 => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C13(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C15(_, r1016, r1017) => {
                        let v46 = (*r1016).clone();
                        let v47 = (*r1017).clone();
                        let m1015 = f80(v0.clone(), v46)?;
                        match m1015 {
                            Ok(_) => {
                                Ok::<Adt17, R<Result<(), Bytes>>>(v47)
                            }
                            Err(v49) => {
                                Err::<Adt17, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v49)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m1018 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: Option<Adt12>, v1: Adt12) -> Option<Adt12> {
    let m1020 = v0;
    match m1020 {
        None => {
            Some(v1)
        }
        Some(v2) => {
            let m1021 = f179(v1.clone(), v2.clone());
            if m1021 {
                Some(v1)
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f158(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> bool {
    let m1022 = f95(v0, v1);
    match m1022 {
        None => {
            false
        }
        Some(v3) => {
            let v4: bool = f180(v2.clone(), v3.clone());
            let v5: bool = f180(v3, v2);
            if v4 {
                v5
            } else {
                false
            }
        }
    }
}

pub fn f159(v0: Adt12) -> u64 {
    let m1023 = v0;
    match m1023 {
        Adt12::C0(_) => {
            0u64
        }
        Adt12::C1(_) => {
            1u64
        }
        Adt12::C2(_) => {
            2u64
        }
        Adt12::C3(_) => {
            3u64
        }
        Adt12::C4(_) => {
            4u64
        }
        Adt12::C5(_) => {
            5u64
        }
        Adt12::C6(_) => {
            6u64
        }
        Adt12::C7(_) => {
            7u64
        }
        Adt12::C8(_) => {
            8u64
        }
        Adt12::C9(_) => {
            9u64
        }
    }
}

pub fn f160(v0: Adt12) -> u64 {
    let m1024 = v0;
    match m1024 {
        Adt12::C0(v1) => {
            v1
        }
        Adt12::C1(v2) => {
            v2
        }
        Adt12::C2(v3) => {
            v3
        }
        Adt12::C3(v4) => {
            v4
        }
        Adt12::C4(v5) => {
            v5
        }
        Adt12::C5(v6) => {
            v6
        }
        Adt12::C6(v7) => {
            v7
        }
        Adt12::C7(v8) => {
            v8
        }
        Adt12::C8(v9) => {
            v9
        }
        Adt12::C9(_) => {
            0u64
        }
    }
}

pub fn f161(v0: Adt12) -> Bytes {
    let m1025 = v0;
    match m1025 {
        Adt12::C0(_) => {
            Bytes::lit(&[])
        }
        Adt12::C1(_) => {
            Bytes::lit(&[])
        }
        Adt12::C2(_) => {
            Bytes::lit(&[])
        }
        Adt12::C3(_) => {
            Bytes::lit(&[])
        }
        Adt12::C4(_) => {
            Bytes::lit(&[])
        }
        Adt12::C5(_) => {
            Bytes::lit(&[])
        }
        Adt12::C6(_) => {
            Bytes::lit(&[])
        }
        Adt12::C7(_) => {
            Bytes::lit(&[])
        }
        Adt12::C8(_) => {
            Bytes::lit(&[])
        }
        Adt12::C9(v10) => {
            v10
        }
    }
}

pub fn f162(v0: Fn14, p1029: bool, p1030: List<u64>) -> bool {
    let m1028 = (p1029, p1030);
    {
        let mut m1028 = m1028;
        loop {
            let step = {
                let (v1, v2) = m1028;
                let m1026 = v2;
                match m1026.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1027: Fn14 = v0.clone();
                            c1027.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1031 = v2;
        let a1032 = v0;
        nat_eq(a1031, a1032)
    }
}

pub fn f164(v0: Adt12, v1: List<(Adt12, List<u64>)>, v2: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let v3: bool = f124({
        let (h1033, _) = v2.clone();
        h1033
    }, v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f165(v0: Bytes, v1: Bytes) -> bool {
    let a1038 = {
        let a1034 = v0;
        let a1035 = v1;
        compare_bytes(a1034, a1035)
    };
    let a1039 = {
        let a1036 = Bytes::lit(&[]);
        let a1037 = Bytes::lit(&[0x01]);
        compare_bytes(a1036, a1037)
    };
    equal(a1038, a1039)
}

pub fn f166(v0: List<Adt29>, v1: Adt29) -> bool {
    f181(Fn16::F182(v1), false, v0)
}

pub fn f167(v0: Bytes, v1: Bytes) -> bool {
    let m1044 = {
        let a1041 = v0;
        let a1042 = 0u64;
        let a1043 = {
            let a1040 = v1.clone();
            length_bytes(a1040)
        };
        slice_bytes(a1041, a1042, a1043)
    };
    match m1044 {
        None => {
            false
        }
        Some(v2) => {
            let a1045 = v2;
            let a1046 = v1;
            equal(a1045, a1046)
        }
    }
}

pub fn f168(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1047 = v0.clone();
        length_bytes(a1047)
    };
    let v3: u64 = {
        let a1048 = v1.clone();
        length_bytes(a1048)
    };
    let v4: u64 = {
        let (h1049, _) = f38(Fn2::F183(v1.clone(), v3, v0.clone(), v2), {
            let a1050 = v2;
            let a1051 = 1u64;
            nat_add(a1050, a1051)?
        }, 0u64)?;
        h1049
    };
    if f184(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f169(v0: Adt24) -> bool {
    let m1052 = v0;
    match m1052 {
        Adt24::C0 => {
            false
        }
        Adt24::C1 => {
            false
        }
        Adt24::C2 => {
            true
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f170(v0: Adt14) -> Option<Bytes> {
    let m1053 = v0;
    match m1053 {
        Adt14::C0(v1) => {
            f185(v1)
        }
        Adt14::C1(_) => {
            None::<Bytes>
        }
        Adt14::C2(_) => {
            None::<Bytes>
        }
        Adt14::C3(v4, _) => {
            Some(f186(v4))
        }
        Adt14::C4(_, _, _) => {
            Some(f1(Str::lit("match:adt")))
        }
        Adt14::C5 => {
            None::<Bytes>
        }
        Adt14::C6(_, _) => {
            None::<Bytes>
        }
    }
}

pub fn f171(v0: Adt30, v1: Bytes) -> Adt30 {
    Adt30::C0(match v0.clone() {
        Adt30::C0(h1054, _, _) => {
            h1054
        }
    }, f187(match v0.clone() {
        Adt30::C0(_, h1055, _) => {
            h1055
        }
    }, v1), match v0 {
        Adt30::C0(_, _, h1056) => {
            h1056
        }
    })
}

pub fn f172(v0: Adt3) -> Bytes {
    let m1057 = v0;
    match m1057 {
        Adt3::C0 => {
            f1(Str::lit("unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("int"))
        }
        Adt3::C4 => {
            f1(Str::lit("ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("string"))
        }
        Adt3::C6 => {
            f1(Str::lit("bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("i128"))
        }
    }
}

pub fn f173(v0: Adt5) -> Bytes {
    let m1058 = v0;
    match m1058 {
        Adt5::C0 => {
            f1(Str::lit("option"))
        }
        Adt5::C1 => {
            f1(Str::lit("list"))
        }
        Adt5::C2 => {
            f1(Str::lit("rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("ref"))
        }
    }
}

pub fn f174(v0: Adt6) -> Bytes {
    let m1059 = v0;
    match m1059 {
        Adt6::C0 => {
            f1(Str::lit("result"))
        }
        Adt6::C1 => {
            f1(Str::lit("pair"))
        }
    }
}

pub fn f175(v0: Adt7) -> Bytes {
    let m1060 = v0;
    match m1060 {
        Adt7::C0 => {
            f1(Str::lit("adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("fn"))
        }
        Adt7::C2 => {
            f1(Str::lit("param"))
        }
    }
}

pub fn f176(v0: Fn15, p1064: List<u64>, p1065: List<u64>) -> List<u64> {
    let m1063 = (p1064, p1065);
    {
        let mut m1063 = m1063;
        loop {
            let step = {
                let (v1, v2) = m1063;
                let m1061 = v2;
                match m1061.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c1062: Fn15 = v0.clone();
                            c1062.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: bool = f130(v0.clone(), v1);
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f178(v0: Adt17) -> R<bool> {
    let m1066 = v0;
    match m1066 {
        Adt17::C0(_, _) => {
            Ok(false)
        }
        Adt17::C1(_, _, _) => {
            Ok(false)
        }
        Adt17::C2(v6, _, _) => {
            let m1067 = v6;
            match m1067 {
                Adt20::C0 => {
                    Ok(false)
                }
                Adt20::C1 => {
                    Ok(false)
                }
                Adt20::C2 => {
                    Ok(false)
                }
                Adt20::C3 => {
                    Ok(true)
                }
            }
        }
        Adt17::C3(_, _, v11, _) => {
            Ok(!v11)
        }
        Adt17::C4(_, _, v15, _) => {
            Ok(!v15)
        }
        Adt17::C5(v17, r1068, _) => {
            let v18 = (*r1068).clone();
            f73(v17, v18)
        }
        Adt17::C6(_, _, _) => {
            Ok(false)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(true)
        }
        Adt17::C8(_, _, _) => {
            Ok(true)
        }
        Adt17::C9(_) => {
            Ok(true)
        }
        Adt17::C10(_, _, _) => {
            Ok(false)
        }
        Adt17::C11(_, _) => {
            Ok(false)
        }
        Adt17::C12 => {
            Ok(false)
        }
        Adt17::C13(_, _) => {
            Ok(false)
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(false)
        }
        Adt17::C15(_, _, _) => {
            Ok(false)
        }
        Adt17::C16(_, _) => {
            Ok(true)
        }
    }
}

pub fn f179(v0: Adt12, v1: Adt12) -> bool {
    let a1069 = f159(v0.clone());
    let a1070 = f159(v1.clone());
    if nat_eq(a1069, a1070) {
        let a1071 = f159(v0.clone());
        let a1072 = 9u64;
        if nat_eq(a1071, a1072) {
            f165(f161(v0), f161(v1))
        } else {
            let a1073 = f160(v0);
            let a1074 = f160(v1);
            nat_lt(a1073, a1074)
        }
    } else {
        let a1075 = f159(v0);
        let a1076 = f159(v1);
        nat_lt(a1075, a1076)
    }
}

pub fn f180(v0: List<u64>, v1: List<u64>) -> bool {
    f162(Fn14::F188(v1), true, v0)
}

pub fn f181(v0: Fn16, p1080: bool, p1081: List<Adt29>) -> bool {
    let m1079 = (p1080, p1081);
    {
        let mut m1079 = m1079;
        loop {
            let step = {
                let (v1, v2) = m1079;
                let m1077 = v2;
                match m1077.uncons() {
                    None => {
                        Err::<(bool, List<Adt29>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt29>), bool>(({
                            let c1078: Fn16 = v0.clone();
                            c1078.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1079 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Adt29, v1: bool, v2: Adt29) -> bool {
    let v3: bool = f189(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f183(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f184(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1084 = {
            let a1082 = v4;
            let a1083 = v1;
            nat_add(a1082, a1083)?
        };
        let a1085 = v3;
        if nat_lt(a1084, a1085) {
            Ok(Some({
                let a1086 = v4;
                let a1087 = 1u64;
                nat_add(a1086, a1087)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f184(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1092 = {
        let a1089 = v0;
        let a1090 = v2;
        let a1091 = {
            let a1088 = v1.clone();
            length_bytes(a1088)
        };
        slice_bytes(a1089, a1090, a1091)
    };
    match m1092 {
        None => {
            false
        }
        Some(v3) => {
            let a1093 = v3;
            let a1094 = v1;
            equal(a1093, a1094)
        }
    }
}

pub fn f185(v0: Adt15) -> Option<Bytes> {
    let m1095 = v0;
    match m1095 {
        Adt15::C0 => {
            None::<Bytes>
        }
        Adt15::C1 => {
            None::<Bytes>
        }
        Adt15::C2 => {
            Some(f1(Str::lit("match:option")))
        }
        Adt15::C3 => {
            Some(f1(Str::lit("match:ordering")))
        }
    }
}

pub fn f186(v0: Adt16) -> Bytes {
    let m1096 = v0;
    match m1096 {
        Adt16::C0 => {
            f1(Str::lit("match:option"))
        }
        Adt16::C1 => {
            f1(Str::lit("match:result"))
        }
        Adt16::C2 => {
            f1(Str::lit("match:result"))
        }
    }
}

pub fn f187(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1097 = f190(v0.clone(), v1.clone());
    if m1097 {
        v0
    } else {
        let v2: (bool, List<Bytes>) = f191(Fn17::F192(v1.clone()), (false, List::<Bytes>::nil()), v0);
        let m1099 = {
            let (h1098, _) = v2;
            h1098
        };
        if m1099 {
            f193({
                let (_, h1100) = v2;
                h1100
            })
        } else {
            f193(List::cons(v1, {
                let (_, h1101) = v2;
                h1101
            }))
        }
    }
}

pub fn f188(v0: List<u64>, v1: bool, v2: u64) -> bool {
    let v3: bool = f130(v0, v2);
    if v1 {
        v3
    } else {
        false
    }
}

pub fn f189(v0: Adt29, v1: Adt29) -> bool {
    let a1104 = match v0.clone() {
        Adt29::C0(h1102, _, _, _) => {
            h1102
        }
    };
    let a1105 = match v1.clone() {
        Adt29::C0(h1103, _, _, _) => {
            h1103
        }
    };
    if equal(a1104, a1105) {
        let a1108 = match v0.clone() {
            Adt29::C0(_, h1106, _, _) => {
                h1106
            }
        };
        let a1109 = match v1.clone() {
            Adt29::C0(_, h1107, _, _) => {
                h1107
            }
        };
        if equal(a1108, a1109) {
            if f194(match v0.clone() {
                Adt29::C0(_, _, h1110, _) => {
                    h1110
                }
            }, match v1.clone() {
                Adt29::C0(_, _, h1111, _) => {
                    h1111
                }
            }) {
                f194(match v0 {
                    Adt29::C0(_, _, _, h1112) => {
                        h1112
                    }
                }, match v1 {
                    Adt29::C0(_, _, _, h1113) => {
                        h1113
                    }
                })
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f190(v0: List<Bytes>, v1: Bytes) -> bool {
    f195(Fn18::F196(v1), false, v0)
}

pub fn f191(v0: Fn17, p1117: (bool, List<Bytes>), p1118: List<Bytes>) -> (bool, List<Bytes>) {
    let m1116 = (p1117, p1118);
    {
        let mut m1116 = m1116;
        loop {
            let step = {
                let (v1, v2) = m1116;
                let m1114 = v2;
                match m1114.uncons() {
                    None => {
                        Err::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(({
                            let c1115: Fn17 = v0.clone();
                            c1115.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    f197(v0, v1, v2)
}

pub fn f193(v0: List<Bytes>) -> List<Bytes> {
    f198(Fn19::F199, List::<Bytes>::nil(), v0)
}

pub fn f194(v0: Option<Adt4>, v1: Option<Adt4>) -> bool {
    let a1119 = f200(v0);
    let a1120 = f200(v1);
    equal(a1119, a1120)
}

pub fn f195(v0: Fn18, p1124: bool, p1125: List<Bytes>) -> bool {
    let m1123 = (p1124, p1125);
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let (v1, v2) = m1123;
                let m1121 = v2;
                match m1121.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1122: Fn18 = v0.clone();
                            c1122.apply(v1, v3)
                        }, v4))
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

pub fn f196(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1126 = v2;
        let a1127 = v0;
        equal(a1126, a1127)
    }
}

pub fn f197(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    let v3: bool = {
        let (h1128, _) = v1;
        h1128
    };
    let v4: List<Bytes> = {
        let (_, h1129) = v1;
        h1129
    };
    let m1130 = v3;
    if m1130 {
        (true, List::cons(v2, v4))
    } else {
        let m1131 = f165(v0.clone(), v2.clone());
        if m1131 {
            (true, List::cons(v2, List::cons(v0, v4)))
        } else {
            (false, List::cons(v2, v4))
        }
    }
}

pub fn f198(v0: Fn19, p1135: List<Bytes>, p1136: List<Bytes>) -> List<Bytes> {
    let m1134 = (p1135, p1136);
    {
        let mut m1134 = m1134;
        loop {
            let step = {
                let (v1, v2) = m1134;
                let m1132 = v2;
                match m1132.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1133: Fn19 = v0.clone();
                            c1133.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f200(v0: Option<Adt4>) -> Bytes {
    let m1137 = v0;
    match m1137 {
        None => {
            f1(Str::lit("None"))
        }
        Some(v1) => {
            f4(f1(Str::lit("Some(")), f4(f14(v1), f1(Str::lit(")"))))
        }
    }
}
