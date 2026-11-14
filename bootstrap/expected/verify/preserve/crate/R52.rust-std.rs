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
    C0(Bytes, bool, Bytes, List<Adt1>, List<Adt1>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Adt2, bool, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt6, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F8,
    F9,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4, p1: Adt1) -> R<Adt4> {
        match self {
            Fn0::F8 => f8(p0, p1),
            Fn0::F9 => f9(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt4, p1: Bytes) -> R<Adt4> {
        match self {
            Fn1::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F26,
}

impl Fn2 {
    pub fn apply(&self, p0: (bool, Adt4), p1: Adt3) -> R<(bool, Adt4)> {
        match self {
            Fn2::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28,
}

impl Fn3 {
    pub fn apply(&self, p0: (bool, Adt4), p1: Bytes) -> R<(bool, Adt4)> {
        match self {
            Fn3::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F30(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt4)) -> R<Option<(u64, Adt4)>> {
        match self {
            Fn4::F30(k0) => f30(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F39,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn5::F39 => f39(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F41,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F41 => f41(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F47(Bytes, u64, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Adt5)) -> R<(u64, Adt5)> {
        match self {
            Fn7::F47(k0, k1, k2) => f47(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F59(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn8::F59(k0) => f59(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F62,
}

impl Fn9 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn9::F62 => f62(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F65,
}

impl Fn10 {
    pub fn apply(&self, p0: Adt7) -> R<Adt7> {
        match self {
            Fn10::F65 => f65(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F83,
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn11::F83 => f83(p0),
        }
    }
}

pub fn f0(v0: u64, v1: Adt0, v2: Bytes, v3: Bytes, v4: bool, v5: Bytes, v6: Bytes, v7: Bytes, v8: Bytes, v9: List<Bytes>, v10: List<Bytes>, v11: List<Adt3>, v12: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let m1 = f1(v0, v1.clone(), v2.clone(), v3.clone())?;
    match m1 {
        Ok(v14) => {
            let m3 = f2(v0, v2, v3, v4, v5, v6, if v4 {
                f3(v7, v8)
            } else {
                v7
            }, v9, v10, v11, match v1 {
                Adt0::C0(_, _, h2, _, _, _) => {
                    h2
                }
            }, v14.clone(), v12)?;
            match m3 {
                Ok(v16) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v14, v16)))
                }
                Err(v15) => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(v15))
                }
            }
        }
        Err(v13) => {
            Ok(Err::<(Bytes, Bytes), Bytes>(v13))
        }
    }
}

pub fn f1(v0: u64, v1: Adt0, v2: Bytes, v3: Bytes) -> R<Result<Bytes, Bytes>> {
    let v4: Adt4 = f4(f5(f4(if match v1.clone() {
        Adt0::C0(_, h4, _, _, _, _) => {
            h4
        }
    } {
        f4(f5(f4(f5(f4(f5(f4(f6(v0), Str::lit("[package]\u{a}name = \""))?, v2)?, Str::lit("\"\u{a}version = \""))?, v3)?, Str::lit("\"\u{a}edition = \""))?, match v1.clone() {
            Adt0::C0(h5, _, _, _, _, _) => {
                h5
            }
        })?, Str::lit("\"\u{a}publish = true"))?
    } else {
        f4(f5(f4(f5(f4(f5(f4(f6(v0), Str::lit("[package]\u{a}name = \""))?, v2)?, Str::lit("\"\u{a}version = \""))?, v3)?, Str::lit("\"\u{a}edition = \""))?, match v1.clone() {
            Adt0::C0(h6, _, _, _, _, _) => {
                h6
            }
        })?, Str::lit("\"\u{a}publish = false"))?
    }, Str::lit("\u{a}\u{a}[lib]\u{a}path = \""))?, match v1.clone() {
        Adt0::C0(_, _, h7, _, _, _) => {
            h7
        }
    })?, Str::lit("\"\u{a}\u{a}[lints.rust]\u{a}"))?;
    let v5: Adt4 = f7(Fn0::F8, v4, match v1.clone() {
        Adt0::C0(_, _, _, h8, _, _) => {
            h8
        }
    })?;
    let v6: Adt4 = f7(Fn0::F9, f4(v5, Str::lit("\u{a}[lints.clippy]\u{a}"))?, match v1.clone() {
        Adt0::C0(_, _, _, _, h9, _) => {
            h9
        }
    })?;
    Ok(f10(f11(Fn1::F12, v6, match v1 {
        Adt0::C0(_, _, _, _, _, h10) => {
            h10
        }
    })?))
}

pub fn f2(v0: u64, v1: Bytes, v2: Bytes, v3: bool, v4: Bytes, v5: Bytes, v6: Bytes, v7: List<Bytes>, v8: List<Bytes>, v9: List<Adt3>, v10: Bytes, v11: Bytes, v12: Bytes) -> R<Result<Bytes, Bytes>> {
    let v13: Adt4 = f13(f4(f14(f4(f15(f4(f6(v0), Str::lit("{\"compiler_semantics\":"))?, v5)?, Str::lit(",\"effects\":"))?, v8)?, Str::lit(",\"exports\":"))?, v9)?;
    let v14: Adt4 = f16(v13, v10, v11, v12)?;
    let v15: Adt4 = f15(f4(f15(f4(v14, Str::lit(",\"name\":"))?, v1)?, Str::lit(",\"profile\":"))?, if v3 {
        f17(Str::lit("rust-std"))
    } else {
        f17(Str::lit("rust-core"))
    })?;
    Ok(f10(f4(f15(f4(f14(f4(f15(f4(f15(f4(v15, Str::lit(",\"program\":"))?, v4)?, Str::lit(",\"runtime\":"))?, f18(v6)?)?, Str::lit(",\"sources\":"))?, v7)?, Str::lit(",\"spec\":\"lexlean/rust-provenance/1\",\"version\":"))?, v2)?, Str::lit("}\u{a}"))?))
}

pub fn f3(v0: Bytes, v1: Bytes) -> Bytes {
    let a11 = v0;
    let a12 = v1;
    append_bytes(a11, a12)
}

pub fn f4(v0: Adt4, v1: Str) -> R<Adt4> {
    f5(v0, {
        let a13 = v1;
        utf8_encode(a13)
    })
}

pub fn f5(v0: Adt4, v1: Bytes) -> R<Adt4> {
    if match v0.clone() {
        Adt4::C0(_, _, _, _, h14) => {
            h14
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a17 = match v0.clone() {
                Adt4::C0(_, _, h15, _, _) => {
                    h15
                }
            };
            let a18 = {
                let a16 = v1.clone();
                length_bytes(a16)
            };
            nat_add(a17, a18)?
        };
        let a20 = v2;
        let a21 = match v0.clone() {
            Adt4::C0(_, _, _, h19, _) => {
                h19
            }
        };
        if nat_le(a20, a21) {
            Ok(f19(v0.clone(), {
                let a23 = match v0 {
                    Adt4::C0(_, h22, _, _, _) => {
                        h22
                    }
                };
                let a24 = v1;
                append_bytes(a23, a24)
            }, v2))
        } else {
            Ok(f20(v0))
        }
    }
}

pub fn f6(v0: u64) -> Adt4 {
    Adt4::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f7(v0: Fn0, p28: Adt4, p29: List<Adt1>) -> R<Adt4> {
    let m27 = (p28, p29);
    {
        let mut m27 = m27;
        loop {
            let step = {
                let (v1, v2) = m27;
                let m25 = v2;
                match m25.uncons() {
                    None => {
                        Err::<(Adt4, List<Adt1>), R<Adt4>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<Adt1>), R<Adt4>>(({
                            let c26: Fn0 = v0.clone();
                            c26.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m27 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Adt4, v1: Adt1) -> R<Adt4> {
    f21(v0, v1)
}

pub fn f9(v0: Adt4, v1: Adt1) -> R<Adt4> {
    f21(v0, v1)
}

pub fn f10(v0: Adt4) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt4::C0(_, _, _, _, h30) => {
            h30
        }
    } {
        Err::<Bytes, Bytes>({
            let a31 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a31)
        })
    } else {
        Ok::<Bytes, Bytes>(f22(f23(List::cons(match v0.clone() {
            Adt4::C0(_, h32, _, _, _) => {
                h32
            }
        }, match v0 {
            Adt4::C0(h33, _, _, _, _) => {
                h33
            }
        }))))
    }
}

pub fn f11(v0: Fn1, p37: Adt4, p38: List<Bytes>) -> R<Adt4> {
    let m36 = (p37, p38);
    {
        let mut m36 = m36;
        loop {
            let step = {
                let (v1, v2) = m36;
                let m34 = v2;
                match m34.uncons() {
                    None => {
                        Err::<(Adt4, List<Bytes>), R<Adt4>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<Bytes>), R<Adt4>>(({
                            let c35: Fn1 = v0.clone();
                            c35.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m36 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt4, v1: Bytes) -> R<Adt4> {
    f24(v0, v1)
}

pub fn f13(v0: Adt4, v1: List<Adt3>) -> R<Adt4> {
    f4({
        let (_, h39) = f25(Fn2::F26, (true, f4(v0, Str::lit("["))?), v1)?;
        h39
    }, Str::lit("]"))
}

pub fn f14(v0: Adt4, v1: List<Bytes>) -> R<Adt4> {
    f4({
        let (_, h40) = f27(Fn3::F28, (true, f4(v0, Str::lit("["))?), v1)?;
        h40
    }, Str::lit("]"))
}

pub fn f15(v0: Adt4, v1: Bytes) -> R<Adt4> {
    f4({
        let (_, h41) = {
            let (h42, _) = f29(Fn4::F30(v1.clone()), {
                let a43 = v1;
                length_bytes(a43)
            }, (0u64, f4(v0, Str::lit("\""))?))?;
            h42
        };
        h41
    }, Str::lit("\""))
}

pub fn f16(v0: Adt4, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Adt4> {
    let v4: Adt4 = f4(v0, Str::lit(",\"files\":{"))?;
    f4(if f31(v1.clone(), f17(Str::lit("Cargo.toml"))) {
        f32(v4, v1, f18(v3)?)?
    } else {
        if f33(v1.clone(), f17(Str::lit("Cargo.toml"))) {
            f32(f4(f32(v4, v1, f18(v3)?)?, Str::lit(","))?, f17(Str::lit("Cargo.toml")), f18(v2)?)?
        } else {
            f32(f4(f32(v4, f17(Str::lit("Cargo.toml")), f18(v2)?)?, Str::lit(","))?, v1, f18(v3)?)?
        }
    }, Str::lit("}"))
}

pub fn f17(v0: Str) -> Bytes {
    let a44 = v0;
    utf8_encode(a44)
}

pub fn f18(v0: Bytes) -> R<Bytes> {
    Ok(f34(f35(v0)?))
}

pub fn f19(v0: Adt4, v1: Bytes, v2: u64) -> Adt4 {
    let a46 = 256u64;
    let a47 = {
        let a45 = v1.clone();
        length_bytes(a45)
    };
    if nat_le(a46, a47) {
        Adt4::C0(List::cons(v1, match v0.clone() {
            Adt4::C0(h48, _, _, _, _) => {
                h48
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt4::C0(_, _, _, h49, _) => {
                h49
            }
        }, false)
    } else {
        Adt4::C0(match v0.clone() {
            Adt4::C0(h50, _, _, _, _) => {
                h50
            }
        }, v1, v2, match v0 {
            Adt4::C0(_, _, _, h51, _) => {
                h51
            }
        }, false)
    }
}

pub fn f20(v0: Adt4) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(h52, _, _, _, _) => {
            h52
        }
    }, match v0.clone() {
        Adt4::C0(_, h53, _, _, _) => {
            h53
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h54, _, _) => {
            h54
        }
    }, match v0 {
        Adt4::C0(_, _, _, h55, _) => {
            h55
        }
    }, true)
}

pub fn f21(v0: Adt4, v1: Adt1) -> R<Adt4> {
    let v2: Adt4 = f5(v0, match v1.clone() {
        Adt1::C0(h56, _, _, _) => {
            h56
        }
    })?;
    let a58 = match v1.clone() {
        Adt1::C0(_, _, _, h57) => {
            h57
        }
    };
    let a59 = 0u64;
    if nat_eq(a58, a59) {
        f4(f4(f4(v2, Str::lit(" = \""))?, f36(match v1 {
            Adt1::C0(_, h60, _, _) => {
                h60
            }
        }))?, Str::lit("\"\u{a}"))
    } else {
        f4(f37(if match v1.clone() {
            Adt1::C0(_, _, h61, _) => {
                h61
            }
        } {
            f4(f4(f4(v2, Str::lit(" = { level = \""))?, f36(match v1.clone() {
                Adt1::C0(_, h62, _, _) => {
                    h62
                }
            }))?, Str::lit("\", priority = -"))?
        } else {
            f4(f4(f4(v2, Str::lit(" = { level = \""))?, f36(match v1.clone() {
                Adt1::C0(_, h63, _, _) => {
                    h63
                }
            }))?, Str::lit("\", priority = "))?
        }, match v1 {
            Adt1::C0(_, _, _, h64) => {
                h64
            }
        })?, Str::lit(" }\u{a}"))
    }
}

pub fn f22(v0: List<Bytes>) -> Bytes {
    let m67 = {
        let (h65, _) = f38(Fn5::F39, {
            let a66 = v0.clone();
            length_list(a66)
        }, v0);
        h65
    };
    match m67.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f23(v0: List<Bytes>) -> List<Bytes> {
    f40(Fn6::F41, List::<Bytes>::nil(), v0)
}

pub fn f24(v0: Adt4, v1: Bytes) -> R<Adt4> {
    f4(f5(v0, v1)?, Str::lit(" = \"allow\"\u{a}"))
}

pub fn f25(v0: Fn2, p71: (bool, Adt4), p72: List<Adt3>) -> R<(bool, Adt4)> {
    let m70 = (p71, p72);
    {
        let mut m70 = m70;
        loop {
            let step = {
                let (v1, v2) = m70;
                let m68 = v2;
                match m68.uncons() {
                    None => {
                        Err::<((bool, Adt4), List<Adt3>), R<(bool, Adt4)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt4), List<Adt3>), R<(bool, Adt4)>>(({
                            let c69: Fn2 = v0.clone();
                            c69.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m70 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: (bool, Adt4), v1: Adt3) -> R<(bool, Adt4)> {
    f42(v0, v1)
}

pub fn f27(v0: Fn3, p76: (bool, Adt4), p77: List<Bytes>) -> R<(bool, Adt4)> {
    let m75 = (p76, p77);
    {
        let mut m75 = m75;
        loop {
            let step = {
                let (v1, v2) = m75;
                let m73 = v2;
                match m73.uncons() {
                    None => {
                        Err::<((bool, Adt4), List<Bytes>), R<(bool, Adt4)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt4), List<Bytes>), R<(bool, Adt4)>>(({
                            let c74: Fn3 = v0.clone();
                            c74.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m75 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: (bool, Adt4), v1: Bytes) -> R<(bool, Adt4)> {
    f43(v0, v1)
}

pub fn f29(v0: Fn4, p82: u64, p83: (u64, Adt4)) -> R<((u64, Adt4), bool)> {
    let m81 = (p82, p83);
    {
        let mut m81 = m81;
        loop {
            let step = {
                let (v1, v2) = m81;
                let m78 = v1;
                if m78 == 0 {
                    Err::<(u64, (u64, Adt4)), R<((u64, Adt4), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m78 - 1;
                    let m80 = {
                        let c79: Fn4 = v0.clone();
                        c79.apply(v2.clone())?
                    };
                    match m80 {
                        None => {
                            Err::<(u64, (u64, Adt4)), R<((u64, Adt4), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt4)), R<((u64, Adt4), bool)>>((v3, v4))
                        }
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

pub fn f30(v0: Bytes, v1: (u64, Adt4)) -> R<Option<(u64, Adt4)>> {
    f44(v0, v1)
}

pub fn f31(v0: Bytes, v1: Bytes) -> bool {
    let a88 = {
        let a84 = v0;
        let a85 = v1;
        compare_bytes(a84, a85)
    };
    let a89 = {
        let a86 = Bytes::lit(&[]);
        let a87 = Bytes::lit(&[]);
        compare_bytes(a86, a87)
    };
    equal(a88, a89)
}

pub fn f32(v0: Adt4, v1: Bytes, v2: Bytes) -> R<Adt4> {
    f15(f4(f15(v0, v1)?, Str::lit(":"))?, v2)
}

pub fn f33(v0: Bytes, v1: Bytes) -> bool {
    let a94 = {
        let a90 = v0;
        let a91 = v1;
        compare_bytes(a90, a91)
    };
    let a95 = {
        let a92 = Bytes::lit(&[]);
        let a93 = Bytes::lit(&[0x01]);
        compare_bytes(a92, a93)
    };
    equal(a94, a95)
}

pub fn f34(v0: Adt5) -> Bytes {
    let a116 = f45(match v0.clone() {
        Adt5::C0(h96, _, _, _, _, _, _, _) => {
            h96
        }
    });
    let a117 = {
        let a114 = f45(match v0.clone() {
            Adt5::C0(_, h97, _, _, _, _, _, _) => {
                h97
            }
        });
        let a115 = {
            let a112 = f45(match v0.clone() {
                Adt5::C0(_, _, h98, _, _, _, _, _) => {
                    h98
                }
            });
            let a113 = {
                let a110 = f45(match v0.clone() {
                    Adt5::C0(_, _, _, h99, _, _, _, _) => {
                        h99
                    }
                });
                let a111 = {
                    let a108 = f45(match v0.clone() {
                        Adt5::C0(_, _, _, _, h100, _, _, _) => {
                            h100
                        }
                    });
                    let a109 = {
                        let a106 = f45(match v0.clone() {
                            Adt5::C0(_, _, _, _, _, h101, _, _) => {
                                h101
                            }
                        });
                        let a107 = {
                            let a104 = f45(match v0.clone() {
                                Adt5::C0(_, _, _, _, _, _, h102, _) => {
                                    h102
                                }
                            });
                            let a105 = f45(match v0 {
                                Adt5::C0(_, _, _, _, _, _, _, h103) => {
                                    h103
                                }
                            });
                            append_bytes(a104, a105)
                        };
                        append_bytes(a106, a107)
                    };
                    append_bytes(a108, a109)
                };
                append_bytes(a110, a111)
            };
            append_bytes(a112, a113)
        };
        append_bytes(a114, a115)
    };
    append_bytes(a116, a117)
}

pub fn f35(v0: Bytes) -> R<Adt5> {
    let v1: u64 = {
        let a118 = v0.clone();
        length_bytes(a118)
    };
    let v2: u64 = {
        let a124 = {
            let a121 = {
                let a119 = v1;
                let a120 = 72u64;
                nat_add(a119, a120)?
            };
            let a122 = 64u64;
            let a123 = 0u64;
            nat_quot(a121, a122, a123)
        };
        let a125 = 64u64;
        nat_mul(a124, a125)?
    };
    let (_, h126) = f46(Fn7::F47(v0, v1, v2), {
        let a127 = v2;
        let a128 = 64u64;
        let a129 = 0u64;
        nat_quot(a127, a128, a129)
    }, (0u64, f48()))?;
    Ok(h126)
}

pub fn f36(v0: Adt2) -> Str {
    let m130 = v0;
    match m130 {
        Adt2::C0 => {
            Str::lit("allow")
        }
        Adt2::C1 => {
            Str::lit("warn")
        }
        Adt2::C2 => {
            Str::lit("deny")
        }
        Adt2::C3 => {
            Str::lit("forbid")
        }
    }
}

pub fn f37(v0: Adt4, v1: u64) -> R<Adt4> {
    f5(v0, f49(v1)?)
}

pub fn f38(v0: Fn5, p135: u64, p136: List<Bytes>) -> (List<Bytes>, bool) {
    let m134 = (p135, p136);
    {
        let mut m134 = m134;
        loop {
            let step = {
                let (v1, v2) = m134;
                let m131 = v1;
                if m131 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m131 - 1;
                    let m133 = {
                        let c132: Fn5 = v0.clone();
                        c132.apply(v2.clone())
                    };
                    match m133 {
                        None => {
                            Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), (List<Bytes>, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: List<Bytes>) -> Option<List<Bytes>> {
    f50(v0)
}

pub fn f40(v0: Fn6, p140: List<Bytes>, p141: List<Bytes>) -> List<Bytes> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m137 = v2;
                match m137.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c138: Fn6 = v0.clone();
                            c138.apply(v1, v3)
                        }, v4))
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

pub fn f41(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f42(v0: (bool, Adt4), v1: Adt3) -> R<(bool, Adt4)> {
    Ok((false, f4(f15(f4(f37(f4({
        let (h142, _) = v0;
        if h142 {
            let (_, h143) = v0;
            h143
        } else {
            f4({
                let (_, h144) = v0;
                h144
            }, Str::lit(","))?
        }
    }, Str::lit("{\"function\":"))?, match v1.clone() {
        Adt3::C0(_, h145) => {
            h145
        }
    })?, Str::lit(",\"name\":"))?, match v1 {
        Adt3::C0(h146, _) => {
            h146
        }
    })?, Str::lit("}"))?))
}

pub fn f43(v0: (bool, Adt4), v1: Bytes) -> R<(bool, Adt4)> {
    Ok((false, f15({
        let (h147, _) = v0;
        if h147 {
            let (_, h148) = v0;
            h148
        } else {
            f4({
                let (_, h149) = v0;
                h149
            }, Str::lit(","))?
        }
    }, v1)?))
}

pub fn f44(v0: Bytes, v1: (u64, Adt4)) -> R<Option<(u64, Adt4)>> {
    let m151 = f51(v0, {
        let (h150, _) = v1.clone();
        h150
    });
    match m151 {
        None => {
            Ok(None::<(u64, Adt4)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a153 = {
                    let (h152, _) = v1;
                    h152
                };
                let a154 = 1u64;
                nat_add(a153, a154)?
            }, f5({
                let (_, h155) = v1;
                h155
            }, f52(v2)?)?)))
        }
    }
}

pub fn f45(v0: u32) -> Bytes {
    let a184 = f53({
        let a156 = f54(v0, 28u32);
        let a157 = 15u32;
        fixed_u32::bit_and(a156, a157)
    });
    let a185 = {
        let a182 = f53({
            let a158 = f54(v0, 24u32);
            let a159 = 15u32;
            fixed_u32::bit_and(a158, a159)
        });
        let a183 = {
            let a180 = f53({
                let a160 = f54(v0, 20u32);
                let a161 = 15u32;
                fixed_u32::bit_and(a160, a161)
            });
            let a181 = {
                let a178 = f53({
                    let a162 = f54(v0, 16u32);
                    let a163 = 15u32;
                    fixed_u32::bit_and(a162, a163)
                });
                let a179 = {
                    let a176 = f53({
                        let a164 = f54(v0, 12u32);
                        let a165 = 15u32;
                        fixed_u32::bit_and(a164, a165)
                    });
                    let a177 = {
                        let a174 = f53({
                            let a166 = f54(v0, 8u32);
                            let a167 = 15u32;
                            fixed_u32::bit_and(a166, a167)
                        });
                        let a175 = {
                            let a172 = f53({
                                let a168 = f54(v0, 4u32);
                                let a169 = 15u32;
                                fixed_u32::bit_and(a168, a169)
                            });
                            let a173 = f53({
                                let a170 = v0;
                                let a171 = 15u32;
                                fixed_u32::bit_and(a170, a171)
                            });
                            append_bytes(a172, a173)
                        };
                        append_bytes(a174, a175)
                    };
                    append_bytes(a176, a177)
                };
                append_bytes(a178, a179)
            };
            append_bytes(a180, a181)
        };
        append_bytes(a182, a183)
    };
    append_bytes(a184, a185)
}

pub fn f46(v0: Fn7, p189: u64, p190: (u64, Adt5)) -> R<(u64, Adt5)> {
    let m188 = (p189, p190);
    {
        let mut m188 = m188;
        loop {
            let step = {
                let (v1, v2) = m188;
                let m186 = v1;
                if m186 == 0 {
                    Err::<(u64, (u64, Adt5)), R<(u64, Adt5)>>(Ok(v2))
                } else {
                    let v3 = m186 - 1;
                    Ok::<(u64, (u64, Adt5)), R<(u64, Adt5)>>((v3, {
                        let c187: Fn7 = v0.clone();
                        c187.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt5)) -> R<(u64, Adt5)> {
    Ok(({
        let a192 = {
            let (h191, _) = v3.clone();
            h191
        };
        let a193 = 64u64;
        nat_add(a192, a193)?
    }, f55({
        let (_, h194) = v3;
        h194
    }, f56(v0, v1, v2, {
        let (h195, _) = v3;
        h195
    })?)?))
}

pub fn f48() -> Adt5 {
    Adt5::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f49(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a232 = f57(v0, 10000000000000000000u64);
        let a233 = {
            let a230 = f57(v0, 1000000000000000000u64);
            let a231 = {
                let a228 = f57(v0, 100000000000000000u64);
                let a229 = {
                    let a226 = f57(v0, 10000000000000000u64);
                    let a227 = {
                        let a224 = f57(v0, 1000000000000000u64);
                        let a225 = {
                            let a222 = f57(v0, 100000000000000u64);
                            let a223 = {
                                let a220 = f57(v0, 10000000000000u64);
                                let a221 = {
                                    let a218 = f57(v0, 1000000000000u64);
                                    let a219 = {
                                        let a216 = f57(v0, 100000000000u64);
                                        let a217 = {
                                            let a214 = f57(v0, 10000000000u64);
                                            let a215 = {
                                                let a212 = f57(v0, 1000000000u64);
                                                let a213 = {
                                                    let a210 = f57(v0, 100000000u64);
                                                    let a211 = {
                                                        let a208 = f57(v0, 10000000u64);
                                                        let a209 = {
                                                            let a206 = f57(v0, 1000000u64);
                                                            let a207 = {
                                                                let a204 = f57(v0, 100000u64);
                                                                let a205 = {
                                                                    let a202 = f57(v0, 10000u64);
                                                                    let a203 = {
                                                                        let a200 = f57(v0, 1000u64);
                                                                        let a201 = {
                                                                            let a198 = f57(v0, 100u64);
                                                                            let a199 = {
                                                                                let a196 = f57(v0, 10u64);
                                                                                let a197 = f57(v0, 1u64);
                                                                                append_bytes(a196, a197)
                                                                            };
                                                                            append_bytes(a198, a199)
                                                                        };
                                                                        append_bytes(a200, a201)
                                                                    };
                                                                    append_bytes(a202, a203)
                                                                };
                                                                append_bytes(a204, a205)
                                                            };
                                                            append_bytes(a206, a207)
                                                        };
                                                        append_bytes(a208, a209)
                                                    };
                                                    append_bytes(a210, a211)
                                                };
                                                append_bytes(a212, a213)
                                            };
                                            append_bytes(a214, a215)
                                        };
                                        append_bytes(a216, a217)
                                    };
                                    append_bytes(a218, a219)
                                };
                                append_bytes(a220, a221)
                            };
                            append_bytes(a222, a223)
                        };
                        append_bytes(a224, a225)
                    };
                    append_bytes(a226, a227)
                };
                append_bytes(a228, a229)
            };
            append_bytes(a230, a231)
        };
        append_bytes(a232, a233)
    };
    let v2: u64 = {
        let (h234, _) = f58(Fn8::F59(v1.clone()), {
            let a235 = v1.clone();
            length_bytes(a235)
        }, 0u64)?;
        h234
    };
    let a236 = v0;
    let a237 = 0u64;
    if nat_eq(a236, a237) {
        let a238 = Str::lit("0");
        Ok(utf8_encode(a238))
    } else {
        Ok(f60(v1.clone(), v2, {
            let a240 = {
                let a239 = v1;
                length_bytes(a239)
            };
            let a241 = v2;
            nat_sub(a240, a241)
        }))
    }
}

pub fn f50(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m242 = v0.clone();
    match m242.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m243 = v2;
            match m243.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f61(Fn9::F62, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m245 = {
                        let (h244, _) = v5;
                        h244
                    };
                    match m245 {
                        None => {
                            Some(f23({
                                let (_, h247) = v5;
                                h247
                            }))
                        }
                        Some(v6) => {
                            Some(f23(List::cons(v6, {
                                let (_, h246) = v5;
                                h246
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> Option<u8> {
    let a248 = v0;
    let a249 = v1;
    index_bytes(a248, a249)
}

pub fn f52(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a294 = {
            let a270 = {
                let a258 = {
                    let a252 = {
                        let a250 = v1;
                        let a251 = 128u8;
                        fixed_u8::bit_and(a250, a251)
                    };
                    let a253 = 128u8;
                    if equal(a252, a253) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a259 = {
                    let a256 = {
                        let a254 = v1;
                        let a255 = 64u8;
                        fixed_u8::bit_and(a254, a255)
                    };
                    let a257 = 64u8;
                    if equal(a256, a257) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a258, a259)?
            };
            let a271 = {
                let a268 = {
                    let a262 = {
                        let a260 = v1;
                        let a261 = 32u8;
                        fixed_u8::bit_and(a260, a261)
                    };
                    let a263 = 32u8;
                    if equal(a262, a263) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a269 = {
                    let a266 = {
                        let a264 = v1;
                        let a265 = 16u8;
                        fixed_u8::bit_and(a264, a265)
                    };
                    let a267 = 16u8;
                    if equal(a266, a267) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a268, a269)?
            };
            nat_add(a270, a271)?
        };
        let a295 = {
            let a292 = {
                let a280 = {
                    let a274 = {
                        let a272 = v1;
                        let a273 = 8u8;
                        fixed_u8::bit_and(a272, a273)
                    };
                    let a275 = 8u8;
                    if equal(a274, a275) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a281 = {
                    let a278 = {
                        let a276 = v1;
                        let a277 = 4u8;
                        fixed_u8::bit_and(a276, a277)
                    };
                    let a279 = 4u8;
                    if equal(a278, a279) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a280, a281)?
            };
            let a293 = {
                let a290 = {
                    let a284 = {
                        let a282 = v1;
                        let a283 = 2u8;
                        fixed_u8::bit_and(a282, a283)
                    };
                    let a285 = 2u8;
                    if equal(a284, a285) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a291 = {
                    let a288 = {
                        let a286 = v1;
                        let a287 = 1u8;
                        fixed_u8::bit_and(a286, a287)
                    };
                    let a289 = 1u8;
                    if equal(a288, a289) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a290, a291)?
            };
            nat_add(a292, a293)?
        };
        nat_add(a294, a295)?
    };
    let a296 = v2;
    let a297 = 34u64;
    if nat_eq(a296, a297) {
        Ok(f17(Str::lit("\\\"")))
    } else {
        let a298 = v2;
        let a299 = 92u64;
        if nat_eq(a298, a299) {
            Ok(f17(Str::lit("\\\\")))
        } else {
            let a300 = v2;
            let a301 = 8u64;
            if nat_eq(a300, a301) {
                Ok(f17(Str::lit("\\b")))
            } else {
                let a302 = v2;
                let a303 = 12u64;
                if nat_eq(a302, a303) {
                    Ok(f17(Str::lit("\\f")))
                } else {
                    let a304 = v2;
                    let a305 = 10u64;
                    if nat_eq(a304, a305) {
                        Ok(f17(Str::lit("\\n")))
                    } else {
                        let a306 = v2;
                        let a307 = 13u64;
                        if nat_eq(a306, a307) {
                            Ok(f17(Str::lit("\\r")))
                        } else {
                            let a308 = v2;
                            let a309 = 9u64;
                            if nat_eq(a308, a309) {
                                Ok(f17(Str::lit("\\t")))
                            } else {
                                let a310 = v2;
                                let a311 = 32u64;
                                if nat_lt(a310, a311) {
                                    Ok(f3(f17(Str::lit("\\u00")), f63(v2)))
                                } else {
                                    Ok(f60(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v2, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f53(v0: u32) -> Bytes {
    let a314 = {
        let a312 = v0;
        let a313 = 8u32;
        fixed_u32::bit_and(a312, a313)
    };
    let a315 = 0u32;
    if equal(a314, a315) {
        let a318 = {
            let a316 = v0;
            let a317 = 4u32;
            fixed_u32::bit_and(a316, a317)
        };
        let a319 = 0u32;
        if equal(a318, a319) {
            let a322 = {
                let a320 = v0;
                let a321 = 2u32;
                fixed_u32::bit_and(a320, a321)
            };
            let a323 = 0u32;
            if equal(a322, a323) {
                let a326 = {
                    let a324 = v0;
                    let a325 = 1u32;
                    fixed_u32::bit_and(a324, a325)
                };
                let a327 = 0u32;
                if equal(a326, a327) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a330 = {
                    let a328 = v0;
                    let a329 = 1u32;
                    fixed_u32::bit_and(a328, a329)
                };
                let a331 = 0u32;
                if equal(a330, a331) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a334 = {
                let a332 = v0;
                let a333 = 2u32;
                fixed_u32::bit_and(a332, a333)
            };
            let a335 = 0u32;
            if equal(a334, a335) {
                let a338 = {
                    let a336 = v0;
                    let a337 = 1u32;
                    fixed_u32::bit_and(a336, a337)
                };
                let a339 = 0u32;
                if equal(a338, a339) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a342 = {
                    let a340 = v0;
                    let a341 = 1u32;
                    fixed_u32::bit_and(a340, a341)
                };
                let a343 = 0u32;
                if equal(a342, a343) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a346 = {
            let a344 = v0;
            let a345 = 4u32;
            fixed_u32::bit_and(a344, a345)
        };
        let a347 = 0u32;
        if equal(a346, a347) {
            let a350 = {
                let a348 = v0;
                let a349 = 2u32;
                fixed_u32::bit_and(a348, a349)
            };
            let a351 = 0u32;
            if equal(a350, a351) {
                let a354 = {
                    let a352 = v0;
                    let a353 = 1u32;
                    fixed_u32::bit_and(a352, a353)
                };
                let a355 = 0u32;
                if equal(a354, a355) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a358 = {
                    let a356 = v0;
                    let a357 = 1u32;
                    fixed_u32::bit_and(a356, a357)
                };
                let a359 = 0u32;
                if equal(a358, a359) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a362 = {
                let a360 = v0;
                let a361 = 2u32;
                fixed_u32::bit_and(a360, a361)
            };
            let a363 = 0u32;
            if equal(a362, a363) {
                let a366 = {
                    let a364 = v0;
                    let a365 = 1u32;
                    fixed_u32::bit_and(a364, a365)
                };
                let a367 = 0u32;
                if equal(a366, a367) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a370 = {
                    let a368 = v0;
                    let a369 = 1u32;
                    fixed_u32::bit_and(a368, a369)
                };
                let a371 = 0u32;
                if equal(a370, a371) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f54(v0: u32, v1: u32) -> u32 {
    let m374 = {
        let a372 = v0;
        let a373 = v1;
        fixed_u32::shift_right(a372, a373)
    };
    match m374 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f55(v0: Adt5, v1: Adt6) -> R<Adt5> {
    let v2: Adt7 = f64(Fn10::F65, 64u64, Adt7::C0(match v0.clone() {
        Adt5::C0(h375, _, _, _, _, _, _, _) => {
            h375
        }
    }, match v0.clone() {
        Adt5::C0(_, h376, _, _, _, _, _, _) => {
            h376
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h377, _, _, _, _, _) => {
            h377
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, h378, _, _, _, _) => {
            h378
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, h379, _, _, _) => {
            h379
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h380, _, _) => {
            h380
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h381, _) => {
            h381
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h382) => {
            h382
        }
    }, v1, 0u64))?;
    Ok(Adt5::C0(f66(match v0.clone() {
        Adt5::C0(h383, _, _, _, _, _, _, _) => {
            h383
        }
    }, match v2.clone() {
        Adt7::C0(h384, _, _, _, _, _, _, _, _, _) => {
            h384
        }
    }), f66(match v0.clone() {
        Adt5::C0(_, h385, _, _, _, _, _, _) => {
            h385
        }
    }, match v2.clone() {
        Adt7::C0(_, h386, _, _, _, _, _, _, _, _) => {
            h386
        }
    }), f66(match v0.clone() {
        Adt5::C0(_, _, h387, _, _, _, _, _) => {
            h387
        }
    }, match v2.clone() {
        Adt7::C0(_, _, h388, _, _, _, _, _, _, _) => {
            h388
        }
    }), f66(match v0.clone() {
        Adt5::C0(_, _, _, h389, _, _, _, _) => {
            h389
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, h390, _, _, _, _, _, _) => {
            h390
        }
    }), f66(match v0.clone() {
        Adt5::C0(_, _, _, _, h391, _, _, _) => {
            h391
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, h392, _, _, _, _, _) => {
            h392
        }
    }), f66(match v0.clone() {
        Adt5::C0(_, _, _, _, _, h393, _, _) => {
            h393
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, h394, _, _, _, _) => {
            h394
        }
    }), f66(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h395, _) => {
            h395
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, _, h396, _, _, _) => {
            h396
        }
    }), f66(match v0 {
        Adt5::C0(_, _, _, _, _, _, _, h397) => {
            h397
        }
    }, match v2 {
        Adt7::C0(_, _, _, _, _, _, _, h398, _, _) => {
            h398
        }
    })))
}

pub fn f56(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    Ok(Adt6::C0(f67(v0.clone(), v1, v2, v3)?, f67(v0.clone(), v1, v2, {
        let a399 = v3;
        let a400 = 4u64;
        nat_add(a399, a400)?
    })?, f67(v0.clone(), v1, v2, {
        let a401 = v3;
        let a402 = 8u64;
        nat_add(a401, a402)?
    })?, f67(v0.clone(), v1, v2, {
        let a403 = v3;
        let a404 = 12u64;
        nat_add(a403, a404)?
    })?, f67(v0.clone(), v1, v2, {
        let a405 = v3;
        let a406 = 16u64;
        nat_add(a405, a406)?
    })?, f67(v0.clone(), v1, v2, {
        let a407 = v3;
        let a408 = 20u64;
        nat_add(a407, a408)?
    })?, f67(v0.clone(), v1, v2, {
        let a409 = v3;
        let a410 = 24u64;
        nat_add(a409, a410)?
    })?, f67(v0.clone(), v1, v2, {
        let a411 = v3;
        let a412 = 28u64;
        nat_add(a411, a412)?
    })?, f67(v0.clone(), v1, v2, {
        let a413 = v3;
        let a414 = 32u64;
        nat_add(a413, a414)?
    })?, f67(v0.clone(), v1, v2, {
        let a415 = v3;
        let a416 = 36u64;
        nat_add(a415, a416)?
    })?, f67(v0.clone(), v1, v2, {
        let a417 = v3;
        let a418 = 40u64;
        nat_add(a417, a418)?
    })?, f67(v0.clone(), v1, v2, {
        let a419 = v3;
        let a420 = 44u64;
        nat_add(a419, a420)?
    })?, f67(v0.clone(), v1, v2, {
        let a421 = v3;
        let a422 = 48u64;
        nat_add(a421, a422)?
    })?, f67(v0.clone(), v1, v2, {
        let a423 = v3;
        let a424 = 52u64;
        nat_add(a423, a424)?
    })?, f67(v0.clone(), v1, v2, {
        let a425 = v3;
        let a426 = 56u64;
        nat_add(a425, a426)?
    })?, f67(v0, v1, v2, {
        let a427 = v3;
        let a428 = 60u64;
        nat_add(a427, a428)?
    })?))
}

pub fn f57(v0: u64, v1: u64) -> Bytes {
    f60(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a432 = {
            let a429 = v0;
            let a430 = v1;
            let a431 = 0u64;
            nat_quot(a429, a430, a431)
        };
        let a433 = 10u64;
        let a434 = 0u64;
        nat_rem(a432, a433, a434)
    }, 1u64)
}

pub fn f58(v0: Fn8, p439: u64, p440: u64) -> R<(u64, bool)> {
    let m438 = (p439, p440);
    {
        let mut m438 = m438;
        loop {
            let step = {
                let (v1, v2) = m438;
                let m435 = v1;
                if m435 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m435 - 1;
                    let m437 = {
                        let c436: Fn8 = v0.clone();
                        c436.apply(v2)?
                    };
                    match m437 {
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
                Ok(next) => { m438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f68(v0, v1)
}

pub fn f60(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m444 = {
        let a441 = v0;
        let a442 = v1;
        let a443 = v2;
        slice_bytes(a441, a442, a443)
    };
    match m444 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f61(v0: Fn9, p448: (Option<Bytes>, List<Bytes>), p449: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m447 = (p448, p449);
    {
        let mut m447 = m447;
        loop {
            let step = {
                let (v1, v2) = m447;
                let m445 = v2;
                match m445.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c446: Fn9 = v0.clone();
                            c446.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f69(v0, v1)
}

pub fn f63(v0: u64) -> Bytes {
    f3(f60(f17(Str::lit("0123456789abcdef")), {
        let a450 = v0;
        let a451 = 16u64;
        let a452 = 0u64;
        nat_quot(a450, a451, a452)
    }, 1u64), f60(f17(Str::lit("0123456789abcdef")), {
        let a453 = v0;
        let a454 = 16u64;
        let a455 = 0u64;
        nat_rem(a453, a454, a455)
    }, 1u64))
}

pub fn f64(v0: Fn10, p459: u64, p460: Adt7) -> R<Adt7> {
    let m458 = (p459, p460);
    {
        let mut m458 = m458;
        loop {
            let step = {
                let (v1, v2) = m458;
                let m456 = v1;
                if m456 == 0 {
                    Err::<(u64, Adt7), R<Adt7>>(Ok(v2))
                } else {
                    let v3 = m456 - 1;
                    Ok::<(u64, Adt7), R<Adt7>>((v3, {
                        let c457: Fn10 = v0.clone();
                        c457.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt7) -> R<Adt7> {
    f70(v0)
}

pub fn f66(v0: u32, v1: u32) -> u32 {
    let m463 = {
        let a461 = f71(v0);
        let a462 = f71(v1);
        fixed_u64::checked_add(a461, a462)
    };
    match m463 {
        None => {
            0u32
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f67(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a466 = {
        let a464 = v3;
        let a465 = 8u64;
        nat_add(a464, a465)?
    };
    let a467 = v2;
    if nat_eq(a466, a467) {
        Ok(f73({
            let a470 = {
                let a468 = v1;
                let a469 = 8u64;
                nat_mul(a468, a469)?
            };
            let a471 = 4294967296u64;
            let a472 = 0u64;
            nat_quot(a470, a471, a472)
        }))
    } else {
        let a475 = {
            let a473 = v3;
            let a474 = 4u64;
            nat_add(a473, a474)?
        };
        let a476 = v2;
        if nat_eq(a475, a476) {
            Ok(f73({
                let a479 = {
                    let a477 = v1;
                    let a478 = 8u64;
                    nat_mul(a477, a478)?
                };
                let a480 = 4294967296u64;
                let a481 = 0u64;
                nat_rem(a479, a480, a481)
            }))
        } else {
            Ok(f74(f75(v0.clone(), v1, v3), f75(v0.clone(), v1, {
                let a482 = v3;
                let a483 = 1u64;
                nat_add(a482, a483)?
            }), f75(v0.clone(), v1, {
                let a484 = v3;
                let a485 = 2u64;
                nat_add(a484, a485)?
            }), f75(v0, v1, {
                let a486 = v3;
                let a487 = 3u64;
                nat_add(a486, a487)?
            })))
        }
    }
}

pub fn f68(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m488 = f51(v0, v1);
    match m488 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a489 = v2;
            let a490 = 48u8;
            if equal(a489, a490) {
                Ok(Some({
                    let a491 = v1;
                    let a492 = 1u64;
                    nat_add(a491, a492)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f69(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m494 = {
        let (h493, _) = v0;
        h493
    };
    match m494 {
        None => {
            (Some(v1), {
                let (_, h495) = v0;
                h495
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a496 = v2;
                let a497 = v1;
                append_bytes(a496, a497)
            }, {
                let (_, h498) = v0;
                h498
            }))
        }
    }
}

pub fn f70(v0: Adt7) -> R<Adt7> {
    let v1: u32 = f76(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h499) => {
            h499
        }
    });
    let v2: u32 = f66(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h500, _, _) => {
            h500
        }
    }, f66(f77(match v0.clone() {
        Adt7::C0(_, _, _, _, h501, _, _, _, _, _) => {
            h501
        }
    }), f66(f78(match v0.clone() {
        Adt7::C0(_, _, _, _, h502, _, _, _, _, _) => {
            h502
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h503, _, _, _, _) => {
            h503
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h504, _, _, _) => {
            h504
        }
    }), f66(v1, {
        let m507 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h506, _) => {
                h506
            }
        };
        match m507 {
            Adt6::C0(h505, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h505
            }
        }
    }))));
    let v3: u32 = f66(f79(match v0.clone() {
        Adt7::C0(h508, _, _, _, _, _, _, _, _, _) => {
            h508
        }
    }), f80(match v0.clone() {
        Adt7::C0(h509, _, _, _, _, _, _, _, _, _) => {
            h509
        }
    }, match v0.clone() {
        Adt7::C0(_, h510, _, _, _, _, _, _, _, _) => {
            h510
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h511, _, _, _, _, _, _, _) => {
            h511
        }
    }));
    Ok(Adt7::C0(f66(v2, v3), match v0.clone() {
        Adt7::C0(h512, _, _, _, _, _, _, _, _, _) => {
            h512
        }
    }, match v0.clone() {
        Adt7::C0(_, h513, _, _, _, _, _, _, _, _) => {
            h513
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h514, _, _, _, _, _, _, _) => {
            h514
        }
    }, f66(match v0.clone() {
        Adt7::C0(_, _, _, h515, _, _, _, _, _, _) => {
            h515
        }
    }, v2), match v0.clone() {
        Adt7::C0(_, _, _, _, h516, _, _, _, _, _) => {
            h516
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h517, _, _, _, _) => {
            h517
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h518, _, _, _) => {
            h518
        }
    }, f81(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h519, _) => {
            h519
        }
    }), {
        let a521 = match v0 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h520) => {
                h520
            }
        };
        let a522 = 1u64;
        nat_add(a521, a522)?
    }))
}

pub fn f71(v0: u32) -> u64 {
    let m524 = {
        let a523 = v0;
        fixed_u64::convert(i128::from(a523))
    };
    match m524 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f72(v0: u64) -> u32 {
    let m528 = {
        let a527 = {
            let a525 = v0;
            let a526 = 4294967295u64;
            fixed_u64::bit_and(a525, a526)
        };
        fixed_u32::convert(i128::from(a527))
    };
    match m528 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f73(v0: u64) -> u32 {
    let (_, h529) = {
        let (_, h530) = f82(Fn11::F83, 32u64, (v0, (1u32, 0u32)));
        h530
    };
    h529
}

pub fn f74(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a535 = f84(v0, 24u32);
    let a536 = {
        let a533 = f84(v1, 16u32);
        let a534 = {
            let a531 = f84(v2, 8u32);
            let a532 = v3;
            fixed_u32::bit_or(a531, a532)
        };
        fixed_u32::bit_or(a533, a534)
    };
    fixed_u32::bit_or(a535, a536)
}

pub fn f75(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a537 = v2;
    let a538 = v1;
    if nat_lt(a537, a538) {
        let m541 = {
            let a539 = v0;
            let a540 = v2;
            index_bytes(a539, a540)
        };
        match m541 {
            None => {
                0u32
            }
            Some(v3) => {
                let m543 = {
                    let a542 = v3;
                    fixed_u32::convert(i128::from(a542))
                };
                match m543 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a544 = v2;
        let a545 = v1;
        if nat_eq(a544, a545) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f76(v0: u64) -> u32 {
    let a546 = v0;
    let a547 = 32u64;
    if nat_lt(a546, a547) {
        let a548 = v0;
        let a549 = 16u64;
        if nat_lt(a548, a549) {
            let a550 = v0;
            let a551 = 8u64;
            if nat_lt(a550, a551) {
                let a552 = v0;
                let a553 = 4u64;
                if nat_lt(a552, a553) {
                    let a554 = v0;
                    let a555 = 2u64;
                    if nat_lt(a554, a555) {
                        let a556 = v0;
                        let a557 = 1u64;
                        if nat_lt(a556, a557) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a558 = v0;
                        let a559 = 3u64;
                        if nat_lt(a558, a559) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a560 = v0;
                    let a561 = 6u64;
                    if nat_lt(a560, a561) {
                        let a562 = v0;
                        let a563 = 5u64;
                        if nat_lt(a562, a563) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a564 = v0;
                        let a565 = 7u64;
                        if nat_lt(a564, a565) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a566 = v0;
                let a567 = 12u64;
                if nat_lt(a566, a567) {
                    let a568 = v0;
                    let a569 = 10u64;
                    if nat_lt(a568, a569) {
                        let a570 = v0;
                        let a571 = 9u64;
                        if nat_lt(a570, a571) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a572 = v0;
                        let a573 = 11u64;
                        if nat_lt(a572, a573) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a574 = v0;
                    let a575 = 14u64;
                    if nat_lt(a574, a575) {
                        let a576 = v0;
                        let a577 = 13u64;
                        if nat_lt(a576, a577) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a578 = v0;
                        let a579 = 15u64;
                        if nat_lt(a578, a579) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a580 = v0;
            let a581 = 24u64;
            if nat_lt(a580, a581) {
                let a582 = v0;
                let a583 = 20u64;
                if nat_lt(a582, a583) {
                    let a584 = v0;
                    let a585 = 18u64;
                    if nat_lt(a584, a585) {
                        let a586 = v0;
                        let a587 = 17u64;
                        if nat_lt(a586, a587) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a588 = v0;
                        let a589 = 19u64;
                        if nat_lt(a588, a589) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a590 = v0;
                    let a591 = 22u64;
                    if nat_lt(a590, a591) {
                        let a592 = v0;
                        let a593 = 21u64;
                        if nat_lt(a592, a593) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a594 = v0;
                        let a595 = 23u64;
                        if nat_lt(a594, a595) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a596 = v0;
                let a597 = 28u64;
                if nat_lt(a596, a597) {
                    let a598 = v0;
                    let a599 = 26u64;
                    if nat_lt(a598, a599) {
                        let a600 = v0;
                        let a601 = 25u64;
                        if nat_lt(a600, a601) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a602 = v0;
                        let a603 = 27u64;
                        if nat_lt(a602, a603) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a604 = v0;
                    let a605 = 30u64;
                    if nat_lt(a604, a605) {
                        let a606 = v0;
                        let a607 = 29u64;
                        if nat_lt(a606, a607) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a608 = v0;
                        let a609 = 31u64;
                        if nat_lt(a608, a609) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a610 = v0;
        let a611 = 48u64;
        if nat_lt(a610, a611) {
            let a612 = v0;
            let a613 = 40u64;
            if nat_lt(a612, a613) {
                let a614 = v0;
                let a615 = 36u64;
                if nat_lt(a614, a615) {
                    let a616 = v0;
                    let a617 = 34u64;
                    if nat_lt(a616, a617) {
                        let a618 = v0;
                        let a619 = 33u64;
                        if nat_lt(a618, a619) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a620 = v0;
                        let a621 = 35u64;
                        if nat_lt(a620, a621) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a622 = v0;
                    let a623 = 38u64;
                    if nat_lt(a622, a623) {
                        let a624 = v0;
                        let a625 = 37u64;
                        if nat_lt(a624, a625) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a626 = v0;
                        let a627 = 39u64;
                        if nat_lt(a626, a627) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a628 = v0;
                let a629 = 44u64;
                if nat_lt(a628, a629) {
                    let a630 = v0;
                    let a631 = 42u64;
                    if nat_lt(a630, a631) {
                        let a632 = v0;
                        let a633 = 41u64;
                        if nat_lt(a632, a633) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a634 = v0;
                        let a635 = 43u64;
                        if nat_lt(a634, a635) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a636 = v0;
                    let a637 = 46u64;
                    if nat_lt(a636, a637) {
                        let a638 = v0;
                        let a639 = 45u64;
                        if nat_lt(a638, a639) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a640 = v0;
                        let a641 = 47u64;
                        if nat_lt(a640, a641) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a642 = v0;
            let a643 = 56u64;
            if nat_lt(a642, a643) {
                let a644 = v0;
                let a645 = 52u64;
                if nat_lt(a644, a645) {
                    let a646 = v0;
                    let a647 = 50u64;
                    if nat_lt(a646, a647) {
                        let a648 = v0;
                        let a649 = 49u64;
                        if nat_lt(a648, a649) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a650 = v0;
                        let a651 = 51u64;
                        if nat_lt(a650, a651) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a652 = v0;
                    let a653 = 54u64;
                    if nat_lt(a652, a653) {
                        let a654 = v0;
                        let a655 = 53u64;
                        if nat_lt(a654, a655) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a656 = v0;
                        let a657 = 55u64;
                        if nat_lt(a656, a657) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a658 = v0;
                let a659 = 60u64;
                if nat_lt(a658, a659) {
                    let a660 = v0;
                    let a661 = 58u64;
                    if nat_lt(a660, a661) {
                        let a662 = v0;
                        let a663 = 57u64;
                        if nat_lt(a662, a663) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a664 = v0;
                        let a665 = 59u64;
                        if nat_lt(a664, a665) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a666 = v0;
                    let a667 = 62u64;
                    if nat_lt(a666, a667) {
                        let a668 = v0;
                        let a669 = 61u64;
                        if nat_lt(a668, a669) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a670 = v0;
                        let a671 = 63u64;
                        if nat_lt(a670, a671) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f77(v0: u32) -> u32 {
    let a674 = {
        let a672 = f85(v0, 6u32, 26u32);
        let a673 = f85(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a672, a673)
    };
    let a675 = f85(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a674, a675)
}

pub fn f78(v0: u32, v1: u32, v2: u32) -> u32 {
    let a681 = {
        let a676 = v0;
        let a677 = v1;
        fixed_u32::bit_and(a676, a677)
    };
    let a682 = {
        let a679 = {
            let a678 = v0;
            fixed_u32::bit_not(a678)
        };
        let a680 = v2;
        fixed_u32::bit_and(a679, a680)
    };
    fixed_u32::bit_xor(a681, a682)
}

pub fn f79(v0: u32) -> u32 {
    let a685 = {
        let a683 = f85(v0, 2u32, 30u32);
        let a684 = f85(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a683, a684)
    };
    let a686 = f85(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a685, a686)
}

pub fn f80(v0: u32, v1: u32, v2: u32) -> u32 {
    let a695 = {
        let a691 = {
            let a687 = v0;
            let a688 = v1;
            fixed_u32::bit_and(a687, a688)
        };
        let a692 = {
            let a689 = v0;
            let a690 = v2;
            fixed_u32::bit_and(a689, a690)
        };
        fixed_u32::bit_xor(a691, a692)
    };
    let a696 = {
        let a693 = v1;
        let a694 = v2;
        fixed_u32::bit_and(a693, a694)
    };
    fixed_u32::bit_xor(a695, a696)
}

pub fn f81(v0: Adt6) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(_, h697, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h697
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h698, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h698
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h699, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h699
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h700, _, _, _, _, _, _, _, _, _, _, _) => {
            h700
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h701, _, _, _, _, _, _, _, _, _, _) => {
            h701
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h702, _, _, _, _, _, _, _, _, _) => {
            h702
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h703, _, _, _, _, _, _, _, _) => {
            h703
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h704, _, _, _, _, _, _, _) => {
            h704
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h705, _, _, _, _, _, _) => {
            h705
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h706, _, _, _, _, _) => {
            h706
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h707, _, _, _, _) => {
            h707
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h708, _, _, _) => {
            h708
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h709, _, _) => {
            h709
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h710, _) => {
            h710
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h711) => {
            h711
        }
    }, f66(f86(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h712, _) => {
            h712
        }
    }), f66(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h713, _, _, _, _, _, _) => {
            h713
        }
    }, f66(f87(match v0.clone() {
        Adt6::C0(_, h714, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h714
        }
    }), match v0 {
        Adt6::C0(h715, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h715
        }
    }))))
}

pub fn f82(v0: Fn11, p719: u64, p720: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m718 = (p719, p720);
    {
        let mut m718 = m718;
        loop {
            let step = {
                let (v1, v2) = m718;
                let m716 = v1;
                if m716 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m716 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c717: Fn11 = v0.clone();
                        c717.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h721, _) = v0;
        h721
    };
    let v2: u32 = {
        let (h722, _) = {
            let (_, h723) = v0;
            h723
        };
        h722
    };
    let v3: u32 = {
        let (_, h724) = {
            let (_, h725) = v0;
            h725
        };
        h724
    };
    ({
        let a726 = v1;
        let a727 = 2u64;
        let a728 = 0u64;
        nat_quot(a726, a727, a728)
    }, (f84(v2, 1u32), {
        let a732 = {
            let a729 = v1;
            let a730 = 2u64;
            let a731 = 0u64;
            nat_rem(a729, a730, a731)
        };
        let a733 = 1u64;
        if nat_eq(a732, a733) {
            let a734 = v3;
            let a735 = v2;
            fixed_u32::bit_or(a734, a735)
        } else {
            v3
        }
    }))
}

pub fn f84(v0: u32, v1: u32) -> u32 {
    let m738 = {
        let a736 = v0;
        let a737 = v1;
        fixed_u32::shift_left(a736, a737)
    };
    match m738 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f85(v0: u32, v1: u32, v2: u32) -> u32 {
    let a739 = f54(v0, v1);
    let a740 = f84(v0, v2);
    fixed_u32::bit_or(a739, a740)
}

pub fn f86(v0: u32) -> u32 {
    let a743 = {
        let a741 = f85(v0, 17u32, 15u32);
        let a742 = f85(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a741, a742)
    };
    let a744 = f54(v0, 10u32);
    fixed_u32::bit_xor(a743, a744)
}

pub fn f87(v0: u32) -> u32 {
    let a747 = {
        let a745 = f85(v0, 7u32, 25u32);
        let a746 = f85(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a745, a746)
    };
    let a748 = f54(v0, 3u32);
    fixed_u32::bit_xor(a747, a748)
}
