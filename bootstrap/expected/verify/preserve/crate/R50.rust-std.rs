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
    C0(Adt2),
    C1(List<u64>, u64, u64, std::rc::Rc<Adt5>, List<Adt6>),
    C2,
    C3(std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes, Option<Adt8>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, List<Adt0>, List<Adt2>, List<List<Adt3>>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<Adt11>, List<Adt4>, Option<Adt7>, Adt13),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64),
    C1(u64, u64, Adt6, List<Adt4>, List<Adt12>),
    C2(u64, u64, Adt6, List<Adt4>, List<Adt12>, Adt12, Adt6),
    C3(u64, u64, Adt6, List<Adt4>, List<Adt12>, Adt12, Adt6, List<Adt4>),
    C4(u64, u64, Adt6, List<Adt4>, List<Adt12>, Adt12, Adt6, List<Adt4>, Adt4, Adt6),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, u64, bool, u64),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt13, List<Adt12>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Adt13, List<Adt3>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3(Adt9),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn0::F3(k0) => f3(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F20(Adt6, Adt9, Adt12, Adt4, Adt6, u64, Adt6),
}

impl Fn1 {
    pub fn apply(&self, p0: (Adt13, (List<Adt4>, Option<Adt7>)), p1: Adt4) -> R<(Adt13, (List<Adt4>, Option<Adt7>))> {
        match self {
            Fn1::F20(k0, k1, k2, k3, k4, k5, k6) => f20(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), *k5, k6.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F33(Adt9, u64, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt3) -> R<List<Adt12>> {
        match self {
            Fn2::F33(k0, k1, k2, k3) => f33(k0.clone(), *k1, *k2, *k3, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F36,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt5, p1: Adt5) -> Adt5 {
        match self {
            Fn3::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F38,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn4::F38 => f38(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F49(u64),
}

impl Fn5 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn5::F49(k0) => f49(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F51(u64, Adt9),
}

impl Fn6 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn6::F51(k0, k1) => f51(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F54(u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn7::F54(k0, k1) => f54(*k0, k1.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>, v2: List<Adt2>, v3: List<List<Adt3>>, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Result<List<Adt4>, Adt7>> {
    let v11: Adt9 = f1(v0, v1, v2, v3, v4, v5, v6);
    let v12: Adt10 = {
        let (h1, _) = f2(Fn0::F3(v11), {
            let a6 = {
                let a4 = {
                    let a2 = v5;
                    let a3 = 2u64;
                    nat_add(a2, a3)?
                };
                let a5 = 64u64;
                nat_mul(a4, a5)?
            };
            let a7 = 64u64;
            nat_add(a6, a7)?
        }, Adt10::C0(List::cons(Adt11::C0(v9, v10), List::<Adt11>::nil()), List::<Adt4>::nil(), None::<Adt7>, f4(v7, v8)))?;
        h1
    };
    let m9 = match v12.clone() {
        Adt10::C0(_, _, h8, _) => {
            h8
        }
    };
    match m9 {
        None => {
            Ok(Ok::<List<Adt4>, Adt7>(match v12 {
                Adt10::C0(_, h10, _, _) => {
                    h10
                }
            }))
        }
        Some(v13) => {
            Ok(Err::<List<Adt4>, Adt7>(v13))
        }
    }
}

pub fn f1(v0: Bytes, v1: List<Adt0>, v2: List<Adt2>, v3: List<List<Adt3>>, v4: u64, v5: u64, v6: u64) -> Adt9 {
    Adt9::C0(v0, v1, v2, v3, v5, v6, v4)
}

pub fn f2(v0: Fn0, p15: u64, p16: Adt10) -> R<(Adt10, bool)> {
    let m14 = (p15, p16);
    {
        let mut m14 = m14;
        loop {
            let step = {
                let (v1, v2) = m14;
                let m11 = v1;
                if m11 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m11 - 1;
                    let m13 = {
                        let c12: Fn0 = v0.clone();
                        c12.apply(v2.clone())?
                    };
                    match m13 {
                        None => {
                            Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt10), R<(Adt10, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m14 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt9, v1: Adt10) -> R<Option<Adt10>> {
    f5(v0, v1)
}

pub fn f4(v0: u64, v1: u64) -> Adt13 {
    Adt13::C0(v0, v1, List::<u64>::nil())
}

pub fn f5(v0: Adt9, v1: Adt10) -> R<Option<Adt10>> {
    let m18 = match v1.clone() {
        Adt10::C0(_, _, h17, _) => {
            h17
        }
    };
    match m18 {
        None => {
            let m20 = match v1.clone() {
                Adt10::C0(h19, _, _, _) => {
                    h19
                }
            };
            match m20.uncons() {
                None => {
                    Ok(None::<Adt10>)
                }
                Some((v3, v4)) => {
                    Ok(Some({
                        let m21 = v3;
                        match m21 {
                            Adt11::C0(v5, v6) => {
                                f6(v0, v1, v4, v5, v6)?
                            }
                            Adt11::C1(v7, v8, v9, v10, v11) => {
                                f7(v0, v1, v4, v7, v8, v9, v10, v11)?
                            }
                            Adt11::C2(v12, v13, v14, v15, v16, v17, v18) => {
                                f8(v1.clone(), List::cons(Adt11::C3(v12, v13, v14, v15, v16, v17, v18, match v1 {
                                    Adt10::C0(_, h22, _, _) => {
                                        h22
                                    }
                                }), v4), List::<Adt4>::nil())
                            }
                            Adt11::C3(v19, v20, v21, v22, v23, v24, v25, v26) => {
                                f9(v0, v1, v4, v19, v20, v21, v22, v23, v24, v25, v26)?
                            }
                            Adt11::C4(v27, v28, v29, v30, v31, v32, v33, v34, v35, v36) => {
                                f10(v0, v1, v4, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36)?
                            }
                        }
                    }))
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt10>)
        }
    }
}

pub fn f6(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64, v4: u64) -> R<Adt10> {
    let v5: (Adt13, Option<Adt7>) = f11(v0.clone(), match v1.clone() {
        Adt10::C0(_, _, _, h23) => {
            h23
        }
    })?;
    let m25 = {
        let (_, h24) = v5;
        h24
    };
    match m25 {
        None => {
            let m27 = f13(v0.clone(), v4, v3);
            match m27 {
                None => {
                    let v9: List<Adt4> = {
                        let m29 = f14(v0.clone(), v3);
                        match m29 {
                            None => {
                                List::<Adt4>::nil()
                            }
                            Some(v8) => {
                                List::cons(Adt4::C0({
                                    let a30 = v3;
                                    let a31 = 1u64;
                                    nat_add(a30, a31)?
                                }, Adt5::C0(v8)), List::<Adt4>::nil())
                            }
                        }
                    };
                    let m32 = f15(v0.clone(), v3, f16(Str::lit("the")));
                    match m32 {
                        None => {
                            Ok(Adt10::C0(v2, v9, None::<Adt7>, {
                                let (h33, _) = v5;
                                h33
                            }))
                        }
                        Some(v10) => {
                            let v11: Adt14 = f17(v0, {
                                let (h34, _) = v5;
                                h34
                            }, {
                                let a35 = v3;
                                let a36 = 1u64;
                                nat_add(a35, a36)?
                            }, 1u64, 2u64)?;
                            let m38 = match v11.clone() {
                                Adt14::C0(_, _, h37) => {
                                    h37
                                }
                            };
                            match m38 {
                                None => {
                                    Ok(Adt10::C0(List::cons(Adt11::C1(v3, v4, v10, v9, match v11.clone() {
                                        Adt14::C0(_, h40, _) => {
                                            h40
                                        }
                                    }), v2), List::<Adt4>::nil(), None::<Adt7>, match v11 {
                                        Adt14::C0(h41, _, _) => {
                                            h41
                                        }
                                    }))
                                }
                                Some(v12) => {
                                    Ok(f12(v1, match v11 {
                                        Adt14::C0(h39, _, _) => {
                                            h39
                                        }
                                    }, v12))
                                }
                            }
                        }
                    }
                }
                Some(v7) => {
                    Ok(f12(v1, {
                        let (h28, _) = v5;
                        h28
                    }, v7))
                }
            }
        }
        Some(v6) => {
            Ok(f12(v1, {
                let (h26, _) = v5;
                h26
            }, v6))
        }
    }
}

pub fn f7(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64, v4: u64, v5: Adt6, v6: List<Adt4>, v7: List<Adt12>) -> R<Adt10> {
    let m42 = v7;
    match m42.uncons() {
        None => {
            Ok(f8(v1, v2, v6))
        }
        Some((v8, v9)) => {
            let m44 = f15(v0, match v8.clone() {
                Adt12::C0(_, _, _, h43) => {
                    h43
                }
            }, f16(Str::lit("of")));
            match m44 {
                None => {
                    Ok(f8(v1, List::cons(Adt11::C1(v3, v4, v5, v6, v9), v2), List::<Adt4>::nil()))
                }
                Some(v10) => {
                    Ok(f8(v1, List::cons(Adt11::C0({
                        let a46 = match v8.clone() {
                            Adt12::C0(_, _, _, h45) => {
                                h45
                            }
                        };
                        let a47 = 1u64;
                        nat_add(a46, a47)?
                    }, {
                        let a48 = v4;
                        let a49 = 1u64;
                        nat_add(a48, a49)?
                    }), List::cons(Adt11::C2(v3, v4, v5, v6, v9, v8, v10), v2)), List::<Adt4>::nil()))
                }
            }
        }
    }
}

pub fn f8(v0: Adt10, v1: List<Adt11>, v2: List<Adt4>) -> Adt10 {
    Adt10::C0(v1, v2, match v0.clone() {
        Adt10::C0(_, _, h50, _) => {
            h50
        }
    }, match v0 {
        Adt10::C0(_, _, _, h51) => {
            h51
        }
    })
}

pub fn f9(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64, v4: u64, v5: Adt6, v6: List<Adt4>, v7: List<Adt12>, v8: Adt12, v9: Adt6, v10: List<Adt4>) -> R<Adt10> {
    let m52 = v10;
    match m52.uncons() {
        None => {
            Ok(f8(v1, List::cons(Adt11::C1(v3, v4, v5, v6, v7), v2), List::<Adt4>::nil()))
        }
        Some((v11, v12)) => {
            let a54 = match v8.clone() {
                Adt12::C0(_, h53, _, _) => {
                    h53
                }
            };
            let a55 = 1u64;
            if nat_eq(a54, a55) {
                Ok(f8(v1, List::cons(Adt11::C3(v3, v4, v5.clone(), {
                    let a58 = v6;
                    let a59 = List::cons(Adt4::C0(match v11.clone() {
                        Adt4::C0(h56, _) => {
                            h56
                        }
                    }, f18(v0, v3, v8.clone(), List::cons(match v11 {
                        Adt4::C0(_, h57) => {
                            h57
                        }
                    }, List::<Adt5>::nil()), List::cons(v5, List::cons(v9.clone(), List::<Adt6>::nil())))?), List::<Adt4>::nil());
                    append_list(a58, a59)
                }, v7, v8, v9, v12), v2), List::<Adt4>::nil()))
            } else {
                let m61 = f15(v0, match v11.clone() {
                    Adt4::C0(h60, _) => {
                        h60
                    }
                }, f16(Str::lit("and")));
                match m61 {
                    None => {
                        Ok(f8(v1, List::cons(Adt11::C3(v3, v4, v5, v6, v7, v8, v9, v12), v2), List::<Adt4>::nil()))
                    }
                    Some(v13) => {
                        Ok(f8(v1, List::cons(Adt11::C0({
                            let a63 = match v11.clone() {
                                Adt4::C0(h62, _) => {
                                    h62
                                }
                            };
                            let a64 = 1u64;
                            nat_add(a63, a64)?
                        }, {
                            let a65 = v4;
                            let a66 = 1u64;
                            nat_add(a65, a66)?
                        }), List::cons(Adt11::C4(v3, v4, v5, v6, v7, v8, v9, v12, v11, v13), v2)), List::<Adt4>::nil()))
                    }
                }
            }
        }
    }
}

pub fn f10(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64, v4: u64, v5: Adt6, v6: List<Adt4>, v7: List<Adt12>, v8: Adt12, v9: Adt6, v10: List<Adt4>, v11: Adt4, v12: Adt6) -> R<Adt10> {
    let v13: (Adt13, (List<Adt4>, Option<Adt7>)) = f19(Fn1::F20(v12, v0, v8.clone(), v11, v9.clone(), v3, v5.clone()), (match v1.clone() {
        Adt10::C0(_, _, _, h67) => {
            h67
        }
    }, (v6, None::<Adt7>)), match v1.clone() {
        Adt10::C0(_, h68, _, _) => {
            h68
        }
    })?;
    let m71 = {
        let (_, h69) = {
            let (_, h70) = v13.clone();
            h70
        };
        h69
    };
    match m71 {
        None => {
            Ok(Adt10::C0(List::cons(Adt11::C3(v3, v4, v5, {
                let (h73, _) = {
                    let (_, h74) = v13;
                    h74
                };
                h73
            }, v7, v8, v9, v10), v2), List::<Adt4>::nil(), None::<Adt7>, {
                let (h75, _) = v13;
                h75
            }))
        }
        Some(v14) => {
            Ok(f12(v1, {
                let (h72, _) = v13;
                h72
            }, v14))
        }
    }
}

pub fn f11(v0: Adt9, v1: Adt13) -> R<(Adt13, Option<Adt7>)> {
    let v2: u64 = {
        let a77 = match v1.clone() {
            Adt13::C0(h76, _, _) => {
                h76
            }
        };
        let a78 = 1u64;
        nat_add(a77, a78)?
    };
    let v3: Adt13 = Adt13::C0(v2, match v1.clone() {
        Adt13::C0(_, h79, _) => {
            h79
        }
    }, match v1 {
        Adt13::C0(_, _, h80) => {
            h80
        }
    });
    let a82 = match v0.clone() {
        Adt9::C0(_, _, _, _, h81, _, _) => {
            h81
        }
    };
    let a83 = v2;
    if nat_lt(a82, a83) {
        Ok((v3, Some(f21(f16(Str::lit("LLS8002")), f22(f22(f22(f22(f16(Str::lit("max_parse_states exceeded in phase parse: configured ")), f23(match v0 {
            Adt9::C0(_, _, _, _, h84, _, _) => {
                h84
            }
        })), f16(Str::lit(", observed "))), f23(v2)), f16(Str::lit(" states"))), None::<Adt8>))))
    } else {
        Ok((v3, None::<Adt7>))
    }
}

pub fn f12(v0: Adt10, v1: Adt13, v2: Adt7) -> Adt10 {
    Adt10::C0(match v0.clone() {
        Adt10::C0(h85, _, _, _) => {
            h85
        }
    }, match v0 {
        Adt10::C0(_, h86, _, _) => {
            h86
        }
    }, Some(v2), v1)
}

pub fn f13(v0: Adt9, v1: u64, v2: u64) -> Option<Adt7> {
    let a88 = match v0.clone() {
        Adt9::C0(_, _, _, _, _, h87, _) => {
            h87
        }
    };
    let a89 = v1;
    if nat_lt(a88, a89) {
        Some(f21(f16(Str::lit("LLS8002")), f22(f22(f22(f16(Str::lit("max_scope_depth exceeded in phase parse (proposition nesting): configured ")), f23(match v0.clone() {
            Adt9::C0(_, _, _, _, _, h90, _) => {
                h90
            }
        })), f16(Str::lit(", nesting depth "))), f23(v1)), {
            let m91 = f24(v0.clone(), v2);
            match m91 {
                None => {
                    None::<Adt8>
                }
                Some(v3) => {
                    Some(f25(v0, f26(v3)))
                }
            }
        }))
    } else {
        None::<Adt7>
    }
}

pub fn f14(v0: Adt9, v1: u64) -> Option<Adt2> {
    let m92 = f24(v0, v1);
    match m92 {
        None => {
            None::<Adt2>
        }
        Some(v2) => {
            let m93 = v2.clone();
            match m93 {
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

pub fn f15(v0: Adt9, v1: u64, v2: Bytes) -> Option<Adt6> {
    let m94 = f27(v0.clone(), v1);
    match m94 {
        None => {
            None::<Adt6>
        }
        Some(v3) => {
            if f28({
                let m96 = f29(v0, v3);
                match m96 {
                    Adt0::C0(_, _, _, _, _, _, _, h95) => {
                        h95
                    }
                }
            }, v2.clone()) {
                let m97 = f30(v2);
                match m97 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v4) => {
                        Some(Adt6::C0(v3, v4))
                    }
                }
            } else {
                None::<Adt6>
            }
        }
    }
}

pub fn f16(v0: Str) -> Bytes {
    let a98 = v0;
    utf8_encode(a98)
}

pub fn f17(v0: Adt9, v1: Adt13, v2: u64, v3: u64, v4: u64) -> R<Adt14> {
    let m99 = f24(v0.clone(), v2);
    match m99 {
        None => {
            Ok(Adt14::C0(v1, List::<Adt12>::nil(), None::<Adt7>))
        }
        Some(v5) => {
            let m100 = v5;
            match m100 {
                Adt2::C0(v6) => {
                    let v7: Adt15 = f31(v0.clone(), v1, v6)?;
                    let m102 = match v7.clone() {
                        Adt15::C0(_, _, h101) => {
                            h101
                        }
                    };
                    match m102 {
                        None => {
                            Ok(Adt14::C0(match v7.clone() {
                                Adt15::C0(h104, _, _) => {
                                    h104
                                }
                            }, f32(Fn2::F33(v0, v4, v3, v2), List::<Adt12>::nil(), match v7 {
                                Adt15::C0(_, h105, _) => {
                                    h105
                                }
                            })?, None::<Adt7>))
                        }
                        Some(v8) => {
                            Ok(Adt14::C0(match v7 {
                                Adt15::C0(h103, _, _) => {
                                    h103
                                }
                            }, List::<Adt12>::nil(), Some(v8)))
                        }
                    }
                }
                Adt2::C1(_, _, _, _, _) => {
                    Ok(Adt14::C0(v1, List::<Adt12>::nil(), None::<Adt7>))
                }
            }
        }
    }
}

pub fn f18(v0: Adt9, v1: u64, v2: Adt12, v3: List<Adt5>, v4: List<Adt6>) -> R<Adt5> {
    let v5: (u64, u64) = f34(v0, {
        let a106 = v1;
        let a107 = 1u64;
        nat_add(a106, a107)?
    }, match v2.clone() {
        Adt12::C0(_, _, _, h108) => {
            h108
        }
    })?;
    Ok(Adt5::C1(List::cons(match v2 {
        Adt12::C0(h109, _, _, _) => {
            h109
        }
    }, List::<u64>::nil()), {
        let (h110, _) = v5;
        h110
    }, {
        let (_, h111) = v5;
        h111
    }, std::rc::Rc::new(f35(Fn3::F36, Adt5::C2, f37(Fn4::F38, List::<Adt5>::nil(), v3))), v4))
}

pub fn f19(v0: Fn1, p115: (Adt13, (List<Adt4>, Option<Adt7>)), p116: List<Adt4>) -> R<(Adt13, (List<Adt4>, Option<Adt7>))> {
    let m114 = (p115, p116);
    {
        let mut m114 = m114;
        loop {
            let step = {
                let (v1, v2) = m114;
                let m112 = v2;
                match m112.uncons() {
                    None => {
                        Err::<((Adt13, (List<Adt4>, Option<Adt7>)), List<Adt4>), R<(Adt13, (List<Adt4>, Option<Adt7>))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((Adt13, (List<Adt4>, Option<Adt7>)), List<Adt4>), R<(Adt13, (List<Adt4>, Option<Adt7>))>>(({
                            let c113: Fn1 = v0.clone();
                            c113.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt6, v1: Adt9, v2: Adt12, v3: Adt4, v4: Adt6, v5: u64, v6: Adt6, v7: (Adt13, (List<Adt4>, Option<Adt7>)), v8: Adt4) -> R<(Adt13, (List<Adt4>, Option<Adt7>))> {
    let m119 = {
        let (_, h117) = {
            let (_, h118) = v7.clone();
            h118
        };
        h117
    };
    match m119 {
        None => {
            let v10: (Adt13, Option<Adt7>) = f11(v1.clone(), {
                let (h120, _) = v7;
                h120
            })?;
            let m122 = {
                let (_, h121) = v10;
                h121
            };
            match m122 {
                None => {
                    Ok(({
                        let (h126, _) = v10;
                        h126
                    }, ({
                        let a132 = {
                            let (h127, _) = {
                                let (_, h128) = v7;
                                h128
                            };
                            h127
                        };
                        let a133 = List::cons(Adt4::C0(match v8.clone() {
                            Adt4::C0(h129, _) => {
                                h129
                            }
                        }, f18(v1, v5, v2, List::cons(match v3 {
                            Adt4::C0(_, h130) => {
                                h130
                            }
                        }, List::cons(match v8 {
                            Adt4::C0(_, h131) => {
                                h131
                            }
                        }, List::<Adt5>::nil())), List::cons(v6, List::cons(v4, List::cons(v0, List::<Adt6>::nil()))))?), List::<Adt4>::nil());
                        append_list(a132, a133)
                    }, None::<Adt7>)))
                }
                Some(v11) => {
                    Ok(({
                        let (h123, _) = v10;
                        h123
                    }, ({
                        let (h124, _) = {
                            let (_, h125) = v7;
                            h125
                        };
                        h124
                    }, Some(v11))))
                }
            }
        }
        Some(_) => {
            Ok(v7)
        }
    }
}

pub fn f21(v0: Bytes, v1: Bytes, v2: Option<Adt8>) -> Adt7 {
    Adt7::C0(v0, v1, v2, None::<Bytes>)
}

pub fn f22(v0: Bytes, v1: Bytes) -> Bytes {
    let a134 = v0;
    let a135 = v1;
    append_bytes(a134, a135)
}

pub fn f23(v0: u64) -> Bytes {
    f39(v0)
}

pub fn f24(v0: Adt9, v1: u64) -> Option<Adt2> {
    let a137 = match v0 {
        Adt9::C0(_, _, h136, _, _, _, _) => {
            h136
        }
    };
    let a138 = v1;
    index_list(a137, a138)
}

pub fn f25(v0: Adt9, v1: u64) -> Adt8 {
    f40(match v0.clone() {
        Adt9::C0(h139, _, _, _, _, _, _) => {
            h139
        }
    }, f29(v0, v1))
}

pub fn f26(v0: Adt2) -> u64 {
    let m140 = v0;
    match m140 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f27(v0: Adt9, v1: u64) -> Option<u64> {
    let m141 = f24(v0.clone(), v1);
    match m141 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            let m142 = v2;
            match m142 {
                Adt2::C0(v3) => {
                    let a143 = f41(f29(v0, v3));
                    let a144 = 1u64;
                    if nat_eq(a143, a144) {
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

pub fn f28(v0: Bytes, v1: Bytes) -> bool {
    let a145 = v0;
    let a146 = v1;
    equal(a145, a146)
}

pub fn f29(v0: Adt9, v1: u64) -> Adt0 {
    let m150 = {
        let a148 = match v0 {
            Adt9::C0(_, h147, _, _, _, _, _) => {
                h147
            }
        };
        let a149 = v1;
        index_list(a148, a149)
    };
    match m150 {
        None => {
            Adt0::C0(Adt1::C6, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64, f16(Str::lit("")))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f30(v0: Bytes) -> Option<Bytes> {
    if if f28(v0.clone(), f16(Str::lit("a"))) {
        true
    } else {
        f28(v0.clone(), f16(Str::lit("A")))
    } {
        Some(f16(Str::lit("a")))
    } else {
        if if f28(v0.clone(), f16(Str::lit("an"))) {
            true
        } else {
            f28(v0.clone(), f16(Str::lit("An")))
        } {
            Some(f16(Str::lit("an")))
        } else {
            if f28(v0.clone(), f16(Str::lit("and"))) {
                Some(f16(Str::lit("and")))
            } else {
                if f28(v0.clone(), f16(Str::lit("as"))) {
                    Some(f16(Str::lit("as")))
                } else {
                    if f28(v0.clone(), f16(Str::lit("defined"))) {
                        Some(f16(Str::lit("defined")))
                    } else {
                        if f28(v0.clone(), f16(Str::lit("every"))) {
                            Some(f16(Str::lit("every")))
                        } else {
                            if f28(v0.clone(), f16(Str::lit("exactly"))) {
                                Some(f16(Str::lit("exactly")))
                            } else {
                                if f28(v0.clone(), f16(Str::lit("exists"))) {
                                    Some(f16(Str::lit("exists")))
                                } else {
                                    if if f28(v0.clone(), f16(Str::lit("for"))) {
                                        true
                                    } else {
                                        f28(v0.clone(), f16(Str::lit("For")))
                                    } {
                                        Some(f16(Str::lit("for")))
                                    } else {
                                        if f28(v0.clone(), f16(Str::lit("holds"))) {
                                            Some(f16(Str::lit("holds")))
                                        } else {
                                            if if f28(v0.clone(), f16(Str::lit("if"))) {
                                                true
                                            } else {
                                                f28(v0.clone(), f16(Str::lit("If")))
                                            } {
                                                Some(f16(Str::lit("if")))
                                            } else {
                                                if f28(v0.clone(), f16(Str::lit("implies"))) {
                                                    Some(f16(Str::lit("implies")))
                                                } else {
                                                    if f28(v0.clone(), f16(Str::lit("is"))) {
                                                        Some(f16(Str::lit("is")))
                                                    } else {
                                                        if if f28(v0.clone(), f16(Str::lit("not"))) {
                                                            true
                                                        } else {
                                                            f28(v0.clone(), f16(Str::lit("Not")))
                                                        } {
                                                            Some(f16(Str::lit("not")))
                                                        } else {
                                                            if f28(v0.clone(), f16(Str::lit("one"))) {
                                                                Some(f16(Str::lit("one")))
                                                            } else {
                                                                if f28(v0.clone(), f16(Str::lit("only"))) {
                                                                    Some(f16(Str::lit("only")))
                                                                } else {
                                                                    if f28(v0.clone(), f16(Str::lit("or"))) {
                                                                        Some(f16(Str::lit("or")))
                                                                    } else {
                                                                        if f28(v0.clone(), f16(Str::lit("such"))) {
                                                                            Some(f16(Str::lit("such")))
                                                                        } else {
                                                                            if f28(v0.clone(), f16(Str::lit("that"))) {
                                                                                Some(f16(Str::lit("that")))
                                                                            } else {
                                                                                if if f28(v0.clone(), f16(Str::lit("the"))) {
                                                                                    true
                                                                                } else {
                                                                                    f28(v0.clone(), f16(Str::lit("The")))
                                                                                } {
                                                                                    Some(f16(Str::lit("the")))
                                                                                } else {
                                                                                    if f28(v0.clone(), f16(Str::lit("then"))) {
                                                                                        Some(f16(Str::lit("then")))
                                                                                    } else {
                                                                                        if if f28(v0.clone(), f16(Str::lit("there"))) {
                                                                                            true
                                                                                        } else {
                                                                                            f28(v0.clone(), f16(Str::lit("There")))
                                                                                        } {
                                                                                            Some(f16(Str::lit("there")))
                                                                                        } else {
                                                                                            if f28(v0.clone(), f16(Str::lit("when"))) {
                                                                                                Some(f16(Str::lit("when")))
                                                                                            } else {
                                                                                                if f28(v0, f16(Str::lit("of"))) {
                                                                                                    Some(f16(Str::lit("of")))
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

pub fn f31(v0: Adt9, v1: Adt13, v2: u64) -> R<Adt15> {
    let v4: List<Adt3> = {
        let m154 = {
            let a152 = match v0.clone() {
                Adt9::C0(_, _, _, h151, _, _, _) => {
                    h151
                }
            };
            let a153 = v2;
            index_list(a152, a153)
        };
        match m154 {
            None => {
                List::<Adt3>::nil()
            }
            Some(v3) => {
                v3
            }
        }
    };
    if f42(v2, match v1.clone() {
        Adt13::C0(_, _, h155) => {
            h155
        }
    }) {
        Ok(Adt15::C0(v1, v4, None::<Adt7>))
    } else {
        let v5: u64 = {
            let a158 = match v1.clone() {
                Adt13::C0(_, h156, _) => {
                    h156
                }
            };
            let a159 = {
                let a157 = v4.clone();
                length_list(a157)
            };
            nat_add(a158, a159)?
        };
        let a161 = match v0.clone() {
            Adt9::C0(_, _, _, _, _, _, h160) => {
                h160
            }
        };
        let a162 = v5;
        if nat_lt(a161, a162) {
            Ok(Adt15::C0(Adt13::C0(match v1.clone() {
                Adt13::C0(h163, _, _) => {
                    h163
                }
            }, v5, match v1 {
                Adt13::C0(_, _, h164) => {
                    h164
                }
            }), List::<Adt3>::nil(), Some(f21(f16(Str::lit("LLS8002")), f22(f22(f22(f22(f16(Str::lit("max_token_lattice_edges exceeded in phase lexical resolution: configured ")), f23(match v0 {
                Adt9::C0(_, _, _, _, _, _, h165) => {
                    h165
                }
            })), f16(Str::lit(", observed "))), f23(v5)), f16(Str::lit(" distinct lattice edges"))), None::<Adt8>))))
        } else {
            Ok(Adt15::C0(Adt13::C0(match v1.clone() {
                Adt13::C0(h166, _, _) => {
                    h166
                }
            }, v5, List::cons(v2, match v1 {
                Adt13::C0(_, _, h167) => {
                    h167
                }
            })), v4, None::<Adt7>))
        }
    }
}

pub fn f32(v0: Fn2, p171: List<Adt12>, p172: List<Adt3>) -> R<List<Adt12>> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v1, v2) = m170;
                let m168 = v2;
                match m168.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt3>), R<List<Adt12>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt3>), R<List<Adt12>>>(({
                            let c169: Fn2 = v0.clone();
                            c169.apply(v1, v3)?
                        }, v4))
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

pub fn f33(v0: Adt9, v1: u64, v2: u64, v3: u64, v4: List<Adt12>, v5: Adt3) -> R<List<Adt12>> {
    let a183 = v4;
    let a184 = if f43({
        let a174 = v2;
        let a175 = match v5.clone() {
            Adt3::C0(_, _, h173, _) => {
                h173
            }
        };
        nat_le(a174, a175)
    }, {
        let a177 = match v5.clone() {
            Adt3::C0(_, _, h176, _) => {
                h176
            }
        };
        let a178 = v1;
        nat_le(a177, a178)
    }) {
        List::cons(Adt12::C0(match v5.clone() {
            Adt3::C0(h179, _, _, _) => {
                h179
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h180, _) => {
                h180
            }
        }, match v5.clone() {
            Adt3::C0(_, _, _, h181) => {
                h181
            }
        }, f44(v0, v3, match v5 {
            Adt3::C0(_, h182, _, _) => {
                h182
            }
        })?), List::<Adt12>::nil())
    } else {
        List::<Adt12>::nil()
    };
    Ok(append_list(a183, a184))
}

pub fn f34(v0: Adt9, v1: u64, v2: u64) -> R<(u64, u64)> {
    let v4: Adt2 = {
        let m185 = f24(v0.clone(), v1);
        match m185 {
            None => {
                Adt2::C0(0u64)
            }
            Some(v3) => {
                v3
            }
        }
    };
    let v5: u64 = {
        let a188 = v1;
        let a189 = {
            let a186 = v2;
            let a187 = 1u64;
            nat_sub(a186, a187)
        };
        if nat_lt(a188, a189) {
            let a190 = v2;
            let a191 = 1u64;
            nat_sub(a190, a191)
        } else {
            v1
        }
    };
    let v7: Adt2 = {
        let m192 = f24(v0, v5);
        match m192 {
            None => {
                Adt2::C0(0u64)
            }
            Some(v6) => {
                v6
            }
        }
    };
    Ok((f26(v4), {
        let a193 = f45(v7);
        let a194 = 1u64;
        nat_add(a193, a194)?
    }))
}

pub fn f35(v0: Fn3, p198: Adt5, p199: List<Adt5>) -> Adt5 {
    let m197 = (p198, p199);
    {
        let mut m197 = m197;
        loop {
            let step = {
                let (v1, v2) = m197;
                let m195 = v2;
                match m195.uncons() {
                    None => {
                        Err::<(Adt5, List<Adt5>), Adt5>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt5, List<Adt5>), Adt5>(({
                            let c196: Fn3 = v0.clone();
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

pub fn f36(v0: Adt5, v1: Adt5) -> Adt5 {
    Adt5::C3(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f37(v0: Fn4, p203: List<Adt5>, p204: List<Adt5>) -> List<Adt5> {
    let m202 = (p203, p204);
    {
        let mut m202 = m202;
        loop {
            let step = {
                let (v1, v2) = m202;
                let m200 = v2;
                match m200.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c201: Fn4 = v0.clone();
                            c201.apply(v1, v3)
                        }, v4))
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

pub fn f38(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f39(v0: u64) -> Bytes {
    f46(f47(), 10u64, v0)
}

pub fn f40(v0: Bytes, v1: Adt0) -> Adt8 {
    Adt8::C0(v0, match v1.clone() {
        Adt0::C0(_, h205, _, _, _, _, _, _) => {
            h205
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h206, _, _, _, _, _) => {
            h206
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h207, _, _, _, _) => {
            h207
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h208, _, _, _) => {
            h208
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h209, _, _) => {
            h209
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, h210, _) => {
            h210
        }
    })
}

pub fn f41(v0: Adt0) -> u64 {
    let m212 = match v0 {
        Adt0::C0(h211, _, _, _, _, _, _, _) => {
            h211
        }
    };
    match m212 {
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

pub fn f42(v0: u64, v1: List<u64>) -> bool {
    f48(Fn5::F49(v0), false, v1)
}

pub fn f43(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f44(v0: Adt9, v1: u64, v2: u64) -> R<u64> {
    let (h213, _) = f50(Fn6::F51(v2, v0.clone()), {
        let a216 = {
            let a215 = match v0 {
                Adt9::C0(_, _, h214, _, _, _, _) => {
                    h214
                }
            };
            length_list(a215)
        };
        let a217 = 1u64;
        nat_add(a216, a217)?
    }, v1)?;
    Ok(h213)
}

pub fn f45(v0: Adt2) -> u64 {
    let m218 = v0;
    match m218 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(_, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a219 = v2;
    let a220 = 0u64;
    if nat_eq(a219, a220) {
        f52(v0, 0u64)
    } else {
        let (_, h221) = {
            let (h222, _) = f53(Fn7::F54(v1, v0), v2, (v2, Bytes::lit(&[])));
            h222
        };
        h221
    }
}

pub fn f47() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f48(v0: Fn5, p226: bool, p227: List<u64>) -> bool {
    let m225 = (p226, p227);
    {
        let mut m225 = m225;
        loop {
            let step = {
                let (v1, v2) = m225;
                let m223 = v2;
                match m223.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c224: Fn5 = v0.clone();
                            c224.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a228 = v2;
        let a229 = v0;
        nat_eq(a228, a229)
    }
}

pub fn f50(v0: Fn6, p234: u64, p235: u64) -> R<(u64, bool)> {
    let m233 = (p234, p235);
    {
        let mut m233 = m233;
        loop {
            let step = {
                let (v1, v2) = m233;
                let m230 = v1;
                if m230 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m230 - 1;
                    let m232 = {
                        let c231: Fn6 = v0.clone();
                        c231.apply(v2)?
                    };
                    match m232 {
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
                Ok(next) => { m233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: u64, v1: Adt9, v2: u64) -> R<Option<u64>> {
    let m236 = f24(v1, v2);
    match m236 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a237 = f26(v3);
            let a238 = v0;
            if nat_lt(a237, a238) {
                Ok(Some({
                    let a239 = v2;
                    let a240 = 1u64;
                    nat_add(a239, a240)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f52(v0: Bytes, v1: u64) -> Bytes {
    let m244 = {
        let a241 = v0;
        let a242 = v1;
        let a243 = 1u64;
        slice_bytes(a241, a242, a243)
    };
    match m244 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f53(v0: Fn7, p249: u64, p250: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m248 = (p249, p250);
    {
        let mut m248 = m248;
        loop {
            let step = {
                let (v1, v2) = m248;
                let m245 = v1;
                if m245 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m245 - 1;
                    let m247 = {
                        let c246: Fn7 = v0.clone();
                        c246.apply(v2.clone())
                    };
                    match m247 {
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
                Ok(next) => { m248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f55(v1, v0, v2)
}

pub fn f55(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h251, _) = v2;
        h251
    };
    let a252 = v3;
    let a253 = 0u64;
    if nat_eq(a252, a253) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a254 = v3;
            let a255 = v1;
            let a256 = 0u64;
            nat_quot(a254, a255, a256)
        }, {
            let a261 = f52(v0, {
                let a257 = v3;
                let a258 = v1;
                let a259 = 0u64;
                nat_rem(a257, a258, a259)
            });
            let a262 = {
                let (_, h260) = v2;
                h260
            };
            append_bytes(a261, a262)
        }))
    }
}
