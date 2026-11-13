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
    C0(List<Adt3>, Bytes, Adt5, List<Adt5>, List<Adt5>, u64, u64, Adt6, Option<Adt14>, Option<Adt14>, Option<Adt14>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, u64, u64, Adt1, Adt4),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, Bytes),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C2(Adt5, u64, u64, Option<(u64, u64)>, std::rc::Rc<Adt6>, u64),
    C3(Adt7),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt8, Adt5, Option<Adt5>, Adt9, Adt11, Option<Adt12>, u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Adt10, List<Bytes>, u64, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C2(std::rc::Rc<Adt12>, u64),
    C3(Adt11),
    C4(Adt5, Adt11, std::rc::Rc<Adt12>, u64),
    C5(Adt5, List<(bool, Adt5)>, u64),
    C6(Adt5, List<Adt5>, u64),
    C7(Adt5, std::rc::Rc<Adt12>, u64),
    C8(std::rc::Rc<Adt12>, u64),
    C9(bool, Adt5, std::rc::Rc<Adt12>, u64),
    C10(Adt5, List<Adt13>, u64),
    C11(u64, std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C12(Adt5, List<Bytes>, Adt5, std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Adt5, Adt5, Adt5),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt5, bool, u64),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Bytes, Option<Adt16>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, List<Adt0>, u64, u64),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, Bytes, Adt19),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64, List<Adt3>, Option<Adt15>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Adt5, Adt19),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, u64, List<Bytes>, bool, bool, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Adt19, List<Adt23>, Adt25),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt24, Adt5, u64, u64, Option<(u64, u64)>, u64),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, u64, List<Adt6>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt24),
    C1(Adt26, Adt27, List<Adt28>),
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(u64, Bytes, u64, List<Adt12>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Adt24, Bytes, Adt8, Adt5, Option<Adt5>, Adt9, Adt11, u64),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Adt26, Adt5, Adt11, u64),
    C1(Adt29, u64),
    C2(Adt30, Adt5, List<Bytes>, Adt5),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Adt26, Bytes, Bytes, Option<Adt5>, List<(u64, Adt12)>, u64),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Adt26, bool, Bytes, Adt5, List<Adt31>, u64),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Adt5, List<Bytes>, Adt5, Adt12),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt11, Adt19),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt9, Adt19),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Adt12, Adt19),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F20(Bytes, Adt17, Bytes),
    F124(Adt17),
}

impl Fn0 {
    pub fn apply(&self, p0: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
        match self {
            Fn0::F20(k0, k1, k2) => f20(k0.clone(), k1.clone(), k2.clone(), p0),
            Fn0::F124(k0) => f124(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F31,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn1::F31 => f31(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F33(Adt4, Adt17, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, List<Adt3>)) -> R<Option<(u64, List<Adt3>)>> {
        match self {
            Fn2::F33(k0, k1, k2) => f33(k0.clone(), k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F37(Adt17, bool),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt21) -> R<Option<Adt21>> {
        match self {
            Fn3::F37(k0, k1) => f37(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F45(Adt17),
    F134(Bytes),
    F157(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F45(k0) => f45(k0.clone(), p0),
            Fn4::F134(k0) => f134(k0.clone(), p0),
            Fn4::F157(k0) => f157(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F53(Adt17),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt22) -> R<Option<Adt22>> {
        match self {
            Fn5::F53(k0) => f53(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F64(Adt17),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn6::F64(k0) => f64(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F66,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn7::F66 => f66(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F70,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>) -> R<Option<List<Bytes>>> {
        match self {
            Fn8::F70 => f70(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F72,
    F137,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn9::F72 => Ok(f72(p0, p1)),
            Fn9::F137 => f137(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F88(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt32) -> R<Option<Adt32>> {
        match self {
            Fn10::F88(k0) => f88(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F90,
}

impl Fn11 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn11::F90 => f90(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F102,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt12, p1: Adt12) -> Adt12 {
        match self {
            Fn12::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F109(Adt17),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt35) -> R<Option<Adt35>> {
        match self {
            Fn13::F109(k0) => f109(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F113,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt6, p1: Adt6) -> Adt6 {
        match self {
            Fn14::F113 => f113(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F122(Adt17),
}

impl Fn15 {
    pub fn apply(&self, p0: (Adt19, List<(bool, Adt5)>)) -> R<Option<(Adt19, List<(bool, Adt5)>)>> {
        match self {
            Fn15::F122(k0) => f122(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F126(Adt17),
}

impl Fn16 {
    pub fn apply(&self, p0: (Adt19, List<Adt13>)) -> R<Option<(Adt19, List<Adt13>)>> {
        match self {
            Fn16::F126(k0) => f126(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F136,
    F167,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<Bytes>, Option<Bytes>), p1: Bytes) -> R<(List<Bytes>, Option<Bytes>)> {
        match self {
            Fn17::F136 => f136(p0, p1),
            Fn17::F167 => Ok(f167(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F140(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt37) -> R<Option<Adt37>> {
        match self {
            Fn18::F140(k0) => f140(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F142,
}

impl Fn19 {
    pub fn apply(&self, p0: Adt12, p1: (u64, Adt12)) -> Adt12 {
        match self {
            Fn19::F142 => f142(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F144(u8, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn20::F144(k0, k1) => f144(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F146,
}

impl Fn21 {
    pub fn apply(&self, p0: Adt12, p1: Adt31) -> Adt12 {
        match self {
            Fn21::F146 => f146(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F149,
}

impl Fn22 {
    pub fn apply(&self, p0: List<(bool, Adt5)>, p1: (bool, Adt5)) -> List<(bool, Adt5)> {
        match self {
            Fn22::F149 => f149(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F153,
}

impl Fn23 {
    pub fn apply(&self, p0: List<Adt13>, p1: Adt13) -> List<Adt13> {
        match self {
            Fn23::F153 => f153(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F171(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn24::F171(k0) => f171(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F173(Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn25::F173(k0) => f173(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>, v2: Bytes, v3: u64) -> R<Result<Adt2, Adt15>> {
    let v4: Adt17 = Adt17::C0(v0, v1.clone(), {
        let a1 = v1;
        length_list(a1)
    }, v3);
    let v5: Adt18 = f1(v4.clone(), Adt19::C0(0u64, List::<Adt3>::nil(), None::<Adt15>))?;
    let v6: Adt19 = if f2(match v5.clone() {
        Adt18::C0(_, h2, _) => {
            h2
        }
    }, f3(Str::lit("lexlean"))) {
        match v5 {
            Adt18::C0(_, _, h3) => {
                h3
            }
        }
    } else {
        f4(v4.clone(), match v5 {
            Adt18::C0(_, _, h4) => {
                h4
            }
        }, f3(Str::lit("a module begins with \\begin{lexlean}{module-name}")))?
    };
    let v7: Adt20 = f5(v4.clone(), v6)?;
    let v8: Bytes = {
        let m7 = match v7.clone() {
            Adt20::C0(h6, _) => {
                h6
            }
        };
        match m7 {
            Adt5::C0(_, _, h5) => {
                h5
            }
        }
    };
    let v9: Adt19 = f6(v4.clone(), match v7.clone() {
        Adt20::C0(_, h8) => {
            h8
        }
    }, match v7.clone() {
        Adt20::C0(h9, _) => {
            h9
        }
    }, f3(Str::lit("env.lexlean")))?;
    let v10: Adt19 = if f2(v8.clone(), v2.clone()) {
        v9
    } else {
        f4(v4.clone(), v9, f7(f7(f7(f3(Str::lit("the declared module ")), f8(v8)), f3(Str::lit(" does not match the source path module "))), f8(v2)))?
    };
    let v11: (Adt19, List<Adt5>) = f9(v4.clone(), v10, f3(Str::lit("\\useglossary")), f3(Str::lit("useglossary")))?;
    let v12: (Adt19, List<Adt5>) = f9(v4.clone(), {
        let (h10, _) = v11;
        h10
    }, f3(Str::lit("\\importmodule")), f3(Str::lit("importmodule")))?;
    let v13: Adt19 = if f10(v4.clone(), {
        let m13 = {
            let (h12, _) = v12.clone();
            h12
        };
        match m13 {
            Adt19::C0(h11, _, _) => {
                h11
            }
        }
    }, f3(Str::lit("\\useglossary")))? {
        f4(v4.clone(), {
            let (h14, _) = v12;
            h14
        }, f3(Str::lit("glossary imports occur before module imports in the header")))?
    } else {
        let (h15, _) = v12;
        h15
    };
    let v14: Adt20 = f5(v4.clone(), f11(v4.clone(), v13, f3(Str::lit("\\title")))?)?;
    let v15: Adt19 = if f10(v4.clone(), {
        let m18 = match v14.clone() {
            Adt20::C0(_, h17) => {
                h17
            }
        };
        match m18 {
            Adt19::C0(h16, _, _) => {
                h16
            }
        }
    }, f3(Str::lit("\\title")))? {
        f4(v4.clone(), match v14.clone() {
            Adt20::C0(_, h19) => {
                h19
            }
        }, f3(Str::lit("each module has exactly one title")))?
    } else {
        match v14.clone() {
            Adt20::C0(_, h20) => {
                h20
            }
        }
    };
    f12(v4, v15, match v7 {
        Adt20::C0(h21, _) => {
            h21
        }
    }, f13({
        let (_, h22) = v11;
        h22
    }), f13({
        let (_, h23) = v12;
        h23
    }), match v14 {
        Adt20::C0(h24, _) => {
            h24
        }
    })
}

pub fn f1(v0: Adt17, v1: Adt19) -> R<Adt18> {
    let v2: Adt19 = f11(v0.clone(), v1, f3(Str::lit("\\begin")))?;
    let v3: Adt20 = f5(v0.clone(), v2.clone())?;
    Ok(Adt18::C0({
        let a26 = match v2 {
            Adt19::C0(h25, _, _) => {
                h25
            }
        };
        let a27 = 1u64;
        nat_sub(a26, a27)
    }, {
        let m30 = match v3.clone() {
            Adt20::C0(h29, _) => {
                h29
            }
        };
        match m30 {
            Adt5::C0(_, _, h28) => {
                h28
            }
        }
    }, f14(v0, match v3.clone() {
        Adt20::C0(_, h31) => {
            h31
        }
    }, match v3.clone() {
        Adt20::C0(h32, _) => {
            h32
        }
    }, f15({
        let m35 = match v3 {
            Adt20::C0(h34, _) => {
                h34
            }
        };
        match m35 {
            Adt5::C0(_, _, h33) => {
                h33
            }
        }
    }))?))
}

pub fn f2(v0: Bytes, v1: Bytes) -> bool {
    let a36 = v0;
    let a37 = v1;
    equal(a36, a37)
}

pub fn f3(v0: Str) -> Bytes {
    let a38 = v0;
    utf8_encode(a38)
}

pub fn f4(v0: Adt17, v1: Adt19, v2: Bytes) -> R<Adt19> {
    f16(v0, v1, f3(Str::lit("LLP2003")), v2)
}

pub fn f5(v0: Adt17, v1: Adt19) -> R<Adt20> {
    f17(v0, v1, false)
}

pub fn f6(v0: Adt17, v1: Adt19, v2: Adt5, v3: Bytes) -> R<Adt19> {
    f14(v0, v1, v2, f18(v3))
}

pub fn f7(v0: Bytes, v1: Bytes) -> Bytes {
    let a39 = v0;
    let a40 = v1;
    append_bytes(a39, a40)
}

pub fn f8(v0: Bytes) -> Bytes {
    f7(f7(f3(Str::lit("`")), v0), f3(Str::lit("`")))
}

pub fn f9(v0: Adt17, v1: Adt19, v2: Bytes, v3: Bytes) -> R<(Adt19, List<Adt5>)> {
    let (h41, _) = f19(Fn0::F20(v2, v0.clone(), v3), {
        let a43 = match v0 {
            Adt17::C0(_, _, h42, _) => {
                h42
            }
        };
        let a44 = 1u64;
        nat_add(a43, a44)?
    }, (v1, List::<Adt5>::nil()))?;
    Ok(h41)
}

pub fn f10(v0: Adt17, v1: u64, v2: Bytes) -> R<bool> {
    Ok(f21(v0.clone(), f22(v0, v1)?, 0u64, v2))
}

pub fn f11(v0: Adt17, v1: Adt19, v2: Bytes) -> R<Adt19> {
    f23(v0, v1, 0u64, v2.clone(), f24(v2.clone(), 1u64, {
        let a46 = {
            let a45 = v2;
            length_bytes(a45)
        };
        let a47 = 1u64;
        nat_sub(a46, a47)
    }))
}

pub fn f12(v0: Adt17, v1: Adt19, v2: Adt5, v3: List<Adt5>, v4: List<Adt5>, v5: Adt5) -> R<Result<Adt2, Adt15>> {
    let v6: u64 = match v1.clone() {
        Adt19::C0(h48, _, _) => {
            h48
        }
    };
    if f25(v0.clone(), v6, f3(Str::lit("coremodule")))? {
        let v7: (Adt14, Adt19) = f26(v0.clone(), v1, f3(Str::lit("coremodule")), f3(Str::lit("\\coredata")), f3(Str::lit("coredata")), f3(Str::lit("coresource")))?;
        f27(v0, {
            let (_, h49) = v7;
            h49
        }, v2, v3, v4, v5, Adt6::C0, Some({
            let (h50, _) = v7;
            h50
        }), f28(0u64), f28(0u64))
    } else {
        if f25(v0.clone(), v6, f3(Str::lit("semanticmodule")))? {
            let v8: (Adt14, Adt19) = f26(v0.clone(), v1, f3(Str::lit("semanticmodule")), f3(Str::lit("\\semanticdata")), f3(Str::lit("semanticdata")), f3(Str::lit("semanticsource")))?;
            f27(v0, {
                let (_, h51) = v8;
                h51
            }, v2, v3, v4, v5, Adt6::C0, f28(0u64), Some({
                let (h52, _) = v8;
                h52
            }), f28(0u64))
        } else {
            if f25(v0.clone(), v6, f3(Str::lit("toolingmodule")))? {
                let v9: (Adt14, Adt19) = f26(v0.clone(), v1, f3(Str::lit("toolingmodule")), f3(Str::lit("\\toolingdata")), f3(Str::lit("toolingdata")), f3(Str::lit("toolingsource")))?;
                f27(v0, {
                    let (_, h53) = v9;
                    h53
                }, v2, v3, v4, v5, Adt6::C0, f28(0u64), f28(0u64), Some({
                    let (h54, _) = v9;
                    h54
                }))
            } else {
                let v10: (Adt6, Adt19) = f29(v0.clone(), v1)?;
                f27(v0, {
                    let (_, h55) = v10;
                    h55
                }, v2, v3, v4, v5, {
                    let (h56, _) = v10;
                    h56
                }, f28(0u64), f28(0u64), f28(0u64))
            }
        }
    }
}

pub fn f13(v0: List<Adt5>) -> List<Adt5> {
    f30(Fn1::F31, List::<Adt5>::nil(), v0)
}

pub fn f14(v0: Adt17, v1: Adt19, v2: Adt5, v3: Adt4) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, h57, _) => {
            h57
        }
    };
    Ok(Adt19::C0(match v1.clone() {
        Adt19::C0(h58, _, _) => {
            h58
        }
    }, {
        let (_, h59) = {
            let (h60, _) = f32(Fn2::F33(v3, v0, v4), {
                let a62 = v4;
                let a63 = match v2.clone() {
                    Adt5::C0(h61, _, _) => {
                        h61
                    }
                };
                nat_sub(a62, a63)
            }, (match v2 {
                Adt5::C0(h64, _, _) => {
                    h64
                }
            }, match v1.clone() {
                Adt19::C0(_, h65, _) => {
                    h65
                }
            }))?;
            h60
        };
        h59
    }, match v1 {
        Adt19::C0(_, _, h66) => {
            h66
        }
    }))
}

pub fn f15(v0: Bytes) -> Adt4 {
    f34(f7(f3(Str::lit("env.")), v0))
}

pub fn f16(v0: Adt17, v1: Adt19, v2: Bytes, v3: Bytes) -> R<Adt19> {
    let m68 = match v1.clone() {
        Adt19::C0(_, _, h67) => {
            h67
        }
    };
    match m68 {
        None => {
            let v5: u64 = f22(v0.clone(), match v1.clone() {
                Adt19::C0(h69, _, _) => {
                    h69
                }
            })?;
            Ok(Adt19::C0(v5, match v1 {
                Adt19::C0(_, h70, _) => {
                    h70
                }
            }, Some(Adt15::C0(v2, v3, Some(f35(v0, v5)), None::<Bytes>))))
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f17(v0: Adt17, v1: Adt19, v2: bool) -> R<Adt20> {
    let v3: Adt19 = f23(v0.clone(), v1, 3u64, f3(Str::lit("{")), f3(Str::lit("brace-open")))?;
    let v4: u64 = match v3.clone() {
        Adt19::C0(h71, _, _) => {
            h71
        }
    };
    let v5: Adt21 = {
        let (h72, _) = f36(Fn3::F37(v0.clone(), v2), {
            let a74 = match v0.clone() {
                Adt17::C0(_, _, h73, _) => {
                    h73
                }
            };
            let a75 = 1u64;
            nat_add(a74, a75)?
        }, Adt21::C0(v4, 0u64, List::<Bytes>::nil(), false, false, None::<Adt0>))?;
        h72
    };
    let v6: u64 = match v5.clone() {
        Adt21::C0(h76, _, _, _, _, _) => {
            h76
        }
    };
    let m78 = match v5.clone() {
        Adt21::C0(_, _, _, _, _, h77) => {
            h77
        }
    };
    match m78 {
        None => {
            Ok(Adt20::C0(Adt5::C0(v4, v6, Bytes::lit(&[])), f4(v0, f40(v3, v6), f3(Str::lit("unclosed `{` argument")))?))
        }
        Some(v7) => {
            Ok(Adt20::C0(Adt5::C0(v4, v6, f38(match v5 {
                Adt21::C0(_, _, h79, _, _, _) => {
                    h79
                }
            })?), f39(v0, f40(v3, {
                let a80 = v6;
                let a81 = 1u64;
                nat_add(a80, a81)?
            }), v7, f34(f3(Str::lit("brace-close"))))))
        }
    }
}

pub fn f18(v0: Bytes) -> Adt4 {
    Adt4::C1(f7(f3(Str::lit("lexlean.core::")), v0))
}

pub fn f19(v0: Fn0, p86: u64, p87: (Adt19, List<Adt5>)) -> R<((Adt19, List<Adt5>), bool)> {
    let m85 = (p86, p87);
    {
        let mut m85 = m85;
        loop {
            let step = {
                let (v1, v2) = m85;
                let m82 = v1;
                if m82 == 0 {
                    Err::<(u64, (Adt19, List<Adt5>)), R<((Adt19, List<Adt5>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m82 - 1;
                    let m84 = {
                        let c83: Fn0 = v0.clone();
                        c83.apply(v2.clone())?
                    };
                    match m84 {
                        None => {
                            Err::<(u64, (Adt19, List<Adt5>)), R<((Adt19, List<Adt5>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt19, List<Adt5>)), R<((Adt19, List<Adt5>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m85 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Bytes, v1: Adt17, v2: Bytes, v3: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
    f41(v1, v0, v2, v3)
}

pub fn f21(v0: Adt17, v1: u64, v2: u64, v3: Bytes) -> bool {
    let m88 = f42(v0, v1);
    match m88 {
        None => {
            false
        }
        Some(v4) => {
            f43(v4, v2, v3)
        }
    }
}

pub fn f22(v0: Adt17, v1: u64) -> R<u64> {
    let (h89, _) = f44(Fn4::F45(v0.clone()), match v0 {
        Adt17::C0(_, _, h90, _) => {
            h90
        }
    }, v1)?;
    Ok(h89)
}

pub fn f23(v0: Adt17, v1: Adt19, v2: u64, v3: Bytes, v4: Bytes) -> R<Adt19> {
    let v5: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h91, _, _) => {
            h91
        }
    })?;
    let m92 = f42(v0.clone(), v5);
    match m92 {
        None => {
            f4(v0, f40(v1, v5), f47(v3))
        }
        Some(v6) => {
            if f43(v6.clone(), v2, v3.clone()) {
                Ok(f39(v0, f40(v1, {
                    let a93 = v5;
                    let a94 = 1u64;
                    nat_add(a93, a94)?
                }), v6, f34(v4)))
            } else {
                f4(v0, f40(v1, v5), f46(v3, match v6 {
                    Adt0::C0(_, _, _, _, _, _, _, h95) => {
                        h95
                    }
                }))
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m99 = {
        let a96 = v0;
        let a97 = v1;
        let a98 = v2;
        slice_bytes(a96, a97, a98)
    };
    match m99 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f25(v0: Adt17, v1: u64, v2: Bytes) -> R<bool> {
    if f10(v0.clone(), v1, f3(Str::lit("\\begin")))? {
        let v3: u64 = f22(v0.clone(), {
            let a100 = f22(v0.clone(), v1)?;
            let a101 = 1u64;
            nat_add(a100, a101)?
        })?;
        if f21(v0.clone(), v3, 3u64, f3(Str::lit("{"))) {
            let m104 = f42(v0.clone(), {
                let a102 = v3;
                let a103 = 1u64;
                nat_add(a102, a103)?
            });
            match m104 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if f2(match v4 {
                        Adt0::C0(_, _, _, _, _, _, _, h105) => {
                            h105
                        }
                    }, v2) {
                        Ok(f21(v0, {
                            let a106 = v3;
                            let a107 = 2u64;
                            nat_add(a106, a107)?
                        }, 3u64, f3(Str::lit("}"))))
                    } else {
                        Ok(false)
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

pub fn f26(v0: Adt17, v1: Adt19, v2: Bytes, v3: Bytes, v4: Bytes, v5: Bytes) -> R<(Adt14, Adt19)> {
    let v6: Adt18 = f1(v0.clone(), v1)?;
    let v7: Adt19 = match v6.clone() {
        Adt18::C0(_, _, h108) => {
            h108
        }
    };
    let v8: bool = {
        let a110 = f10(v0.clone(), match v7.clone() {
            Adt19::C0(h109, _, _) => {
                h109
            }
        }, v3.clone())?;
        bool_not(a110)
    };
    let v9: Adt20 = if v8 {
        f48(v0.clone(), f40(v7.clone(), f22(v0.clone(), match v7 {
            Adt19::C0(h111, _, _) => {
                h111
            }
        })?))?
    } else {
        f17(v0.clone(), f11(v0.clone(), v7, v3)?, true)?
    };
    let v10: Adt19 = f49(v0.clone(), match v9.clone() {
        Adt20::C0(_, h112) => {
            h112
        }
    }, match v9.clone() {
        Adt20::C0(h113, _) => {
            h113
        }
    }, if v8 {
        v5
    } else {
        v4
    });
    let v11: Adt19 = f50(v0.clone(), f50(v0, v10, v2)?, f3(Str::lit("lexlean")))?;
    Ok((Adt14::C0(match v9 {
        Adt20::C0(h114, _) => {
            h114
        }
    }, v8, match v6 {
        Adt18::C0(h115, _, _) => {
            h115
        }
    }), v11))
}

pub fn f27(v0: Adt17, v1: Adt19, v2: Adt5, v3: List<Adt5>, v4: List<Adt5>, v5: Adt5, v6: Adt6, v7: Option<Adt14>, v8: Option<Adt14>, v9: Option<Adt14>) -> R<Result<Adt2, Adt15>> {
    let v10: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h116, _, _) => {
            h116
        }
    })?;
    let v12: Adt19 = {
        let m117 = f42(v0.clone(), v10);
        match m117 {
            None => {
                v1
            }
            Some(v11) => {
                f4(v0, f40(v1, v10), f7(f7(f3(Str::lit("unexpected ")), f8(match v11 {
                    Adt0::C0(_, _, _, _, _, _, _, h118) => {
                        h118
                    }
                })), f3(Str::lit(" after \\end{lexlean}"))))?
            }
        }
    };
    let m120 = match v12.clone() {
        Adt19::C0(_, _, h119) => {
            h119
        }
    };
    match m120 {
        None => {
            Ok(Ok::<Adt2, Adt15>(Adt2::C0(f51(match v12 {
                Adt19::C0(_, h121, _) => {
                    h121
                }
            }), match v2.clone() {
                Adt5::C0(_, _, h122) => {
                    h122
                }
            }, v2, v3, v4, match v5.clone() {
                Adt5::C0(h123, _, _) => {
                    h123
                }
            }, match v5 {
                Adt5::C0(_, h124, _) => {
                    h124
                }
            }, v6, v7, v8, v9)))
        }
        Some(v13) => {
            Ok(Err::<Adt2, Adt15>(v13))
        }
    }
}

pub fn f28(_: u64) -> Option<Adt14> {
    None::<Adt14>
}

pub fn f29(v0: Adt17, v1: Adt19) -> R<(Adt6, Adt19)> {
    let v2: Adt22 = {
        let (h125, _) = f52(Fn5::F53(v0.clone()), {
            let a127 = match v0 {
                Adt17::C0(_, _, h126, _) => {
                    h126
                }
            };
            let a128 = 1u64;
            nat_add(a127, a128)?
        }, Adt22::C0(v1, List::<Adt23>::nil(), Adt25::C0(Adt24::C0(f3(Str::lit("lexlean")), 0u64, List::<Adt6>::nil()))))?;
        h125
    };
    let m130 = match v2.clone() {
        Adt22::C0(_, _, h129) => {
            h129
        }
    };
    match m130 {
        Adt25::C0(_) => {
            Ok((Adt6::C0, match v2 {
                Adt22::C0(h132, _, _) => {
                    h132
                }
            }))
        }
        Adt25::C1(_, _, _) => {
            Ok((Adt6::C0, match v2 {
                Adt22::C0(h133, _, _) => {
                    h133
                }
            }))
        }
        Adt25::C2(v3) => {
            Ok((v3, match v2 {
                Adt22::C0(h131, _, _) => {
                    h131
                }
            }))
        }
    }
}

pub fn f30(v0: Fn1, p137: List<Adt5>, p138: List<Adt5>) -> List<Adt5> {
    let m136 = (p137, p138);
    {
        let mut m136 = m136;
        loop {
            let step = {
                let (v1, v2) = m136;
                let m134 = v2;
                match m134.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c135: Fn1 = v0.clone();
                            c135.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f32(v0: Fn2, p143: u64, p144: (u64, List<Adt3>)) -> R<((u64, List<Adt3>), bool)> {
    let m142 = (p143, p144);
    {
        let mut m142 = m142;
        loop {
            let step = {
                let (v1, v2) = m142;
                let m139 = v1;
                if m139 == 0 {
                    Err::<(u64, (u64, List<Adt3>)), R<((u64, List<Adt3>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m139 - 1;
                    let m141 = {
                        let c140: Fn2 = v0.clone();
                        c140.apply(v2.clone())?
                    };
                    match m141 {
                        None => {
                            Err::<(u64, (u64, List<Adt3>)), R<((u64, List<Adt3>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<Adt3>)), R<((u64, List<Adt3>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Adt4, v1: Adt17, v2: u64, v3: (u64, List<Adt3>)) -> R<Option<(u64, List<Adt3>)>> {
    f54(v1, v0, v2, v3)
}

pub fn f34(v0: Bytes) -> Adt4 {
    Adt4::C0(f3(Str::lit("lexlean.core")), v0)
}

pub fn f35(v0: Adt17, v1: u64) -> Adt16 {
    let v2: u64 = {
        let a146 = match v0.clone() {
            Adt17::C0(_, _, h145, _) => {
                h145
            }
        };
        let a147 = 1u64;
        nat_sub(a146, a147)
    };
    let m150 = f42(v0.clone(), {
        let a148 = v1;
        let a149 = v2;
        if nat_lt(a148, a149) {
            v1
        } else {
            v2
        }
    });
    match m150 {
        None => {
            f56(match v0 {
                Adt17::C0(h152, _, _, _) => {
                    h152
                }
            })
        }
        Some(v3) => {
            f55(match v0 {
                Adt17::C0(h151, _, _, _) => {
                    h151
                }
            }, v3)
        }
    }
}

pub fn f36(v0: Fn3, p157: u64, p158: Adt21) -> R<(Adt21, bool)> {
    let m156 = (p157, p158);
    {
        let mut m156 = m156;
        loop {
            let step = {
                let (v1, v2) = m156;
                let m153 = v1;
                if m153 == 0 {
                    Err::<(u64, Adt21), R<(Adt21, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m153 - 1;
                    let m155 = {
                        let c154: Fn3 = v0.clone();
                        c154.apply(v2.clone())?
                    };
                    match m155 {
                        None => {
                            Err::<(u64, Adt21), R<(Adt21, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt21), R<(Adt21, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: Adt17, v1: bool, v2: Adt21) -> R<Option<Adt21>> {
    f57(v0, v1, v2)
}

pub fn f38(v0: List<Bytes>) -> R<Bytes> {
    f58(f59(v0)?)
}

pub fn f39(v0: Adt17, v1: Adt19, v2: Adt0, v3: Adt4) -> Adt19 {
    Adt19::C0(match v1.clone() {
        Adt19::C0(h159, _, _) => {
            h159
        }
    }, List::cons(f60(v0, v2, v3), match v1.clone() {
        Adt19::C0(_, h160, _) => {
            h160
        }
    }), match v1 {
        Adt19::C0(_, _, h161) => {
            h161
        }
    })
}

pub fn f40(v0: Adt19, v1: u64) -> Adt19 {
    Adt19::C0(v1, match v0.clone() {
        Adt19::C0(_, h162, _) => {
            h162
        }
    }, match v0 {
        Adt19::C0(_, _, h163) => {
            h163
        }
    })
}

pub fn f41(v0: Adt17, v1: Bytes, v2: Bytes, v3: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
    let v4: Adt19 = {
        let (h164, _) = v3;
        h164
    };
    let m166 = match v4.clone() {
        Adt19::C0(_, _, h165) => {
            h165
        }
    };
    match m166 {
        None => {
            if f10(v0.clone(), match v4.clone() {
                Adt19::C0(h167, _, _) => {
                    h167
                }
            }, v1.clone())? {
                let v6: Adt20 = f5(v0.clone(), f11(v0.clone(), v4, v1)?)?;
                Ok(Some((f6(v0, match v6.clone() {
                    Adt20::C0(_, h168) => {
                        h168
                    }
                }, match v6.clone() {
                    Adt20::C0(h169, _) => {
                        h169
                    }
                }, v2)?, List::cons(match v6 {
                    Adt20::C0(h170, _) => {
                        h170
                    }
                }, {
                    let (_, h171) = v3;
                    h171
                }))))
            } else {
                Ok(None::<(Adt19, List<Adt5>)>)
            }
        }
        Some(_) => {
            Ok(None::<(Adt19, List<Adt5>)>)
        }
    }
}

pub fn f42(v0: Adt17, v1: u64) -> Option<Adt0> {
    let a173 = match v0 {
        Adt17::C0(_, h172, _, _) => {
            h172
        }
    };
    let a174 = v1;
    index_list(a173, a174)
}

pub fn f43(v0: Adt0, v1: u64, v2: Bytes) -> bool {
    let a175 = f61(v0.clone());
    let a176 = v1;
    if nat_eq(a175, a176) {
        f2(match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h177) => {
                h177
            }
        }, v2)
    } else {
        false
    }
}

pub fn f44(v0: Fn4, p182: u64, p183: u64) -> R<(u64, bool)> {
    let m181 = (p182, p183);
    {
        let mut m181 = m181;
        loop {
            let step = {
                let (v1, v2) = m181;
                let m178 = v1;
                if m178 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m178 - 1;
                    let m180 = {
                        let c179: Fn4 = v0.clone();
                        c179.apply(v2)?
                    };
                    match m180 {
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
                Ok(next) => { m181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Adt17, v1: u64) -> R<Option<u64>> {
    f62(v0, v1)
}

pub fn f46(v0: Bytes, v1: Bytes) -> Bytes {
    f7(f7(f7(f3(Str::lit("expected ")), f8(v0)), f3(Str::lit(", found "))), f8(v1))
}

pub fn f47(v0: Bytes) -> Bytes {
    f7(f7(f3(Str::lit("expected ")), f8(v0)), f3(Str::lit(", found end of file")))
}

pub fn f48(v0: Adt17, v1: Adt19) -> R<Adt20> {
    let v2: u64 = match v1.clone() {
        Adt19::C0(h184, _, _) => {
            h184
        }
    };
    let v3: (u64, List<Bytes>) = {
        let (h185, _) = f63(Fn6::F64(v0.clone()), {
            let a187 = match v0.clone() {
                Adt17::C0(_, _, h186, _) => {
                    h186
                }
            };
            let a188 = 1u64;
            nat_add(a187, a188)?
        }, (v2, List::<Bytes>::nil()))?;
        h185
    };
    let v4: u64 = {
        let (h189, _) = v3;
        h189
    };
    Ok(Adt20::C0(Adt5::C0(v2, v4, f38({
        let (_, h190) = v3;
        h190
    })?), {
        let m191 = f42(v0.clone(), v4);
        match m191 {
            None => {
                f4(v0, f40(v1, v4), f3(Str::lit("unclosed environment")))?
            }
            Some(_) => {
                f40(v1, v4)
            }
        }
    }))
}

pub fn f49(v0: Adt17, v1: Adt19, v2: Adt5, v3: Bytes) -> Adt19 {
    let m193 = f42(v0.clone(), match v2.clone() {
        Adt5::C0(h192, _, _) => {
            h192
        }
    });
    match m193 {
        None => {
            v1
        }
        Some(v4) => {
            let v6: Adt0 = {
                let m197 = f42(v0.clone(), {
                    let a195 = match v2 {
                        Adt5::C0(_, h194, _) => {
                            h194
                        }
                    };
                    let a196 = 1u64;
                    nat_sub(a195, a196)
                });
                match m197 {
                    None => {
                        v4.clone()
                    }
                    Some(v5) => {
                        v5
                    }
                }
            };
            Adt19::C0(match v1.clone() {
                Adt19::C0(h198, _, _) => {
                    h198
                }
            }, List::cons(Adt3::C0(match v0 {
                Adt17::C0(h199, _, _, _) => {
                    h199
                }
            }, match v4.clone() {
                Adt0::C0(_, h200, _, _, _, _, _, _) => {
                    h200
                }
            }, match v6 {
                Adt0::C0(_, _, h201, _, _, _, _, _) => {
                    h201
                }
            }, match v4 {
                Adt0::C0(h202, _, _, _, _, _, _, _) => {
                    h202
                }
            }, f18(v3)), match v1.clone() {
                Adt19::C0(_, h203, _) => {
                    h203
                }
            }), match v1 {
                Adt19::C0(_, _, h204) => {
                    h204
                }
            })
        }
    }
}

pub fn f50(v0: Adt17, v1: Adt19, v2: Bytes) -> R<Adt19> {
    let v3: Adt19 = f11(v0.clone(), v1, f3(Str::lit("\\end")))?;
    let v4: Adt20 = f5(v0.clone(), v3)?;
    if f2({
        let m207 = match v4.clone() {
            Adt20::C0(h206, _) => {
                h206
            }
        };
        match m207 {
            Adt5::C0(_, _, h205) => {
                h205
            }
        }
    }, v2.clone()) {
        f14(v0, match v4.clone() {
            Adt20::C0(_, h208) => {
                h208
            }
        }, match v4 {
            Adt20::C0(h209, _) => {
                h209
            }
        }, f15(v2))
    } else {
        f4(v0, match v4.clone() {
            Adt20::C0(_, h210) => {
                h210
            }
        }, f7(f7(f7(f7(f3(Str::lit("expected `\\end{")), v2), f3(Str::lit("}`, found `\\end{"))), {
            let m213 = match v4 {
                Adt20::C0(h212, _) => {
                    h212
                }
            };
            match m213 {
                Adt5::C0(_, _, h211) => {
                    h211
                }
            }
        }), f3(Str::lit("}`"))))
    }
}

pub fn f51(v0: List<Adt3>) -> List<Adt3> {
    f65(Fn7::F66, List::<Adt3>::nil(), v0)
}

pub fn f52(v0: Fn5, p218: u64, p219: Adt22) -> R<(Adt22, bool)> {
    let m217 = (p218, p219);
    {
        let mut m217 = m217;
        loop {
            let step = {
                let (v1, v2) = m217;
                let m214 = v1;
                if m214 == 0 {
                    Err::<(u64, Adt22), R<(Adt22, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m214 - 1;
                    let m216 = {
                        let c215: Fn5 = v0.clone();
                        c215.apply(v2.clone())?
                    };
                    match m216 {
                        None => {
                            Err::<(u64, Adt22), R<(Adt22, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt22), R<(Adt22, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: Adt17, v1: Adt22) -> R<Option<Adt22>> {
    f67(v0, v1)
}

pub fn f54(v0: Adt17, v1: Adt4, v2: u64, v3: (u64, List<Adt3>)) -> R<Option<(u64, List<Adt3>)>> {
    let a221 = {
        let (h220, _) = v3.clone();
        h220
    };
    let a222 = v2;
    if nat_lt(a221, a222) {
        let m224 = f42(v0.clone(), {
            let (h223, _) = v3.clone();
            h223
        });
        match m224 {
            None => {
                Ok(None::<(u64, List<Adt3>)>)
            }
            Some(v4) => {
                let a225 = f61(v4.clone());
                let a226 = 6u64;
                if nat_eq(a225, a226) {
                    Ok(Some(({
                        let a228 = {
                            let (h227, _) = v3;
                            h227
                        };
                        let a229 = 1u64;
                        nat_add(a228, a229)?
                    }, {
                        let (_, h230) = v3;
                        h230
                    })))
                } else {
                    Ok(Some(({
                        let a232 = {
                            let (h231, _) = v3;
                            h231
                        };
                        let a233 = 1u64;
                        nat_add(a232, a233)?
                    }, List::cons(f60(v0, v4, v1), {
                        let (_, h234) = v3;
                        h234
                    }))))
                }
            }
        }
    } else {
        Ok(None::<(u64, List<Adt3>)>)
    }
}

pub fn f55(v0: Bytes, v1: Adt0) -> Adt16 {
    Adt16::C0(v0, match v1.clone() {
        Adt0::C0(_, h235, _, _, _, _, _, _) => {
            h235
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h236, _, _, _, _, _) => {
            h236
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h237, _, _, _, _) => {
            h237
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h238, _, _, _) => {
            h238
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h239, _, _) => {
            h239
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, h240, _) => {
            h240
        }
    })
}

pub fn f56(v0: Bytes) -> Adt16 {
    Adt16::C0(v0, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64)
}

pub fn f57(v0: Adt17, v1: bool, v2: Adt21) -> R<Option<Adt21>> {
    let m242 = match v2.clone() {
        Adt21::C0(_, _, _, _, _, h241) => {
            h241
        }
    };
    match m242 {
        None => {
            let m244 = f42(v0, match v2.clone() {
                Adt21::C0(h243, _, _, _, _, _) => {
                    h243
                }
            });
            match m244 {
                None => {
                    Ok(None::<Adt21>)
                }
                Some(v4) => {
                    let v5: bool = match v2.clone() {
                        Adt21::C0(_, _, _, h245, _, _) => {
                            h245
                        }
                    };
                    let v6: u64 = match v2.clone() {
                        Adt21::C0(_, h246, _, _, _, _) => {
                            h246
                        }
                    };
                    let v7: List<Bytes> = match v2.clone() {
                        Adt21::C0(_, _, h247, _, _, _) => {
                            h247
                        }
                    };
                    if if v5 {
                        false
                    } else {
                        f43(v4.clone(), 3u64, f3(Str::lit("{")))
                    } {
                        f68(v1, v2, v4, {
                            let a248 = v6;
                            let a249 = 1u64;
                            nat_add(a248, a249)?
                        }, List::cons(f3(Str::lit("{")), v7))
                    } else {
                        if if v5 {
                            false
                        } else {
                            f43(v4.clone(), 3u64, f3(Str::lit("}")))
                        } {
                            let a250 = v6;
                            let a251 = 0u64;
                            if nat_eq(a250, a251) {
                                Ok(Some(Adt21::C0(match v2.clone() {
                                    Adt21::C0(h252, _, _, _, _, _) => {
                                        h252
                                    }
                                }, v6, v7, v5, match v2 {
                                    Adt21::C0(_, _, _, _, h253, _) => {
                                        h253
                                    }
                                }, Some(v4))))
                            } else {
                                f68(v1, v2, v4, {
                                    let a254 = v6;
                                    let a255 = 1u64;
                                    nat_sub(a254, a255)
                                }, List::cons(f3(Str::lit("}")), v7))
                            }
                        } else {
                            if if v5 {
                                false
                            } else {
                                let a256 = f61(v4.clone());
                                let a257 = 6u64;
                                nat_eq(a256, a257)
                            } {
                                f68(v1, v2, v4, v6, v7)
                            } else {
                                f68(v1, v2, v4.clone(), v6, List::cons(match v4 {
                                    Adt0::C0(_, _, _, _, _, _, _, h258) => {
                                        h258
                                    }
                                }, v7))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt21>)
        }
    }
}

pub fn f58(v0: List<Bytes>) -> R<Bytes> {
    let m261 = {
        let (h259, _) = f69(Fn8::F70, {
            let a260 = v0.clone();
            length_list(a260)
        }, v0)?;
        h259
    };
    match m261.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, _)) => {
            Ok(v1)
        }
    }
}

pub fn f59(v0: List<Bytes>) -> R<List<Bytes>> {
    f71(Fn9::F72, List::<Bytes>::nil(), v0)
}

pub fn f60(v0: Adt17, v1: Adt0, v2: Adt4) -> Adt3 {
    Adt3::C0(match v0 {
        Adt17::C0(h262, _, _, _) => {
            h262
        }
    }, match v1.clone() {
        Adt0::C0(_, h263, _, _, _, _, _, _) => {
            h263
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h264, _, _, _, _, _) => {
            h264
        }
    }, match v1 {
        Adt0::C0(h265, _, _, _, _, _, _, _) => {
            h265
        }
    }, v2)
}

pub fn f61(v0: Adt0) -> u64 {
    let m267 = match v0 {
        Adt0::C0(h266, _, _, _, _, _, _, _) => {
            h266
        }
    };
    match m267 {
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

pub fn f62(v0: Adt17, v1: u64) -> R<Option<u64>> {
    let m268 = f42(v0, v1);
    match m268 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a269 = f61(v2);
            let a270 = 6u64;
            if nat_eq(a269, a270) {
                Ok(Some({
                    let a271 = v1;
                    let a272 = 1u64;
                    nat_add(a271, a272)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f63(v0: Fn6, p277: u64, p278: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m273 = v1;
                if m273 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m273 - 1;
                    let m275 = {
                        let c274: Fn6 = v0.clone();
                        c274.apply(v2.clone())?
                    };
                    match m275 {
                        None => {
                            Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt17, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f73(v0, v1)
}

pub fn f65(v0: Fn7, p282: List<Adt3>, p283: List<Adt3>) -> List<Adt3> {
    let m281 = (p282, p283);
    {
        let mut m281 = m281;
        loop {
            let step = {
                let (v1, v2) = m281;
                let m279 = v2;
                match m279.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c280: Fn7 = v0.clone();
                            c280.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f67(v0: Adt17, v1: Adt22) -> R<Option<Adt22>> {
    let v2: Adt19 = match v1.clone() {
        Adt22::C0(h284, _, _) => {
            h284
        }
    };
    let m286 = match v2.clone() {
        Adt19::C0(_, _, h285) => {
            h285
        }
    };
    match m286 {
        None => {
            let m288 = match v1.clone() {
                Adt22::C0(_, _, h287) => {
                    h287
                }
            };
            match m288 {
                Adt25::C0(v4) => {
                    Ok(Some(f74(v0, v2, match v1 {
                        Adt22::C0(_, h289, _) => {
                            h289
                        }
                    }, v4)?))
                }
                Adt25::C1(v5, v6, v7) => {
                    Ok(Some(f75(v0, v2, match v1 {
                        Adt22::C0(_, h290, _) => {
                            h290
                        }
                    }, v5, v6, v7)?))
                }
                Adt25::C2(_) => {
                    Ok(None::<Adt22>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt22>)
        }
    }
}

pub fn f68(v0: bool, v1: Adt21, v2: Adt0, v3: u64, v4: List<Bytes>) -> R<Option<Adt21>> {
    let v5: Adt32 = if v0 {
        f76(match v2 {
            Adt0::C0(_, _, _, _, _, _, _, h291) => {
                h291
            }
        }, match v1.clone() {
            Adt21::C0(_, _, _, h292, _, _) => {
                h292
            }
        }, match v1.clone() {
            Adt21::C0(_, _, _, _, h293, _) => {
                h293
            }
        })?
    } else {
        Adt32::C0(0u64, match v1.clone() {
            Adt21::C0(_, _, _, h294, _, _) => {
                h294
            }
        }, match v1.clone() {
            Adt21::C0(_, _, _, _, h295, _) => {
                h295
            }
        })
    };
    Ok(Some(Adt21::C0({
        let a297 = match v1 {
            Adt21::C0(h296, _, _, _, _, _) => {
                h296
            }
        };
        let a298 = 1u64;
        nat_add(a297, a298)?
    }, v3, v4, match v5.clone() {
        Adt32::C0(_, h299, _) => {
            h299
        }
    }, match v5 {
        Adt32::C0(_, _, h300) => {
            h300
        }
    }, None::<Adt0>)))
}

pub fn f69(v0: Fn8, p305: u64, p306: List<Bytes>) -> R<(List<Bytes>, bool)> {
    let m304 = (p305, p306);
    {
        let mut m304 = m304;
        loop {
            let step = {
                let (v1, v2) = m304;
                let m301 = v1;
                if m301 == 0 {
                    Err::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m301 - 1;
                    let m303 = {
                        let c302: Fn8 = v0.clone();
                        c302.apply(v2.clone())?
                    };
                    match m303 {
                        None => {
                            Err::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Bytes>) -> R<Option<List<Bytes>>> {
    f77(v0)
}

pub fn f71(v0: Fn9, p310: List<Bytes>, p311: List<Bytes>) -> R<List<Bytes>> {
    let m309 = (p310, p311);
    {
        let mut m309 = m309;
        loop {
            let step = {
                let (v1, v2) = m309;
                let m307 = v2;
                match m307.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c308: Fn9 = v0.clone();
                            c308.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f73(v0: Adt17, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let m313 = f42(v0, {
        let (h312, _) = v1.clone();
        h312
    });
    match m313 {
        None => {
            Ok(None::<(u64, List<Bytes>)>)
        }
        Some(v2) => {
            if f43(v2.clone(), 0u64, f3(Str::lit("\\end"))) {
                Ok(None::<(u64, List<Bytes>)>)
            } else {
                Ok(Some(({
                    let a315 = {
                        let (h314, _) = v1;
                        h314
                    };
                    let a316 = 1u64;
                    nat_add(a315, a316)?
                }, List::cons(match v2 {
                    Adt0::C0(_, _, _, _, _, _, _, h317) => {
                        h317
                    }
                }, {
                    let (_, h318) = v1;
                    h318
                }))))
            }
        }
    }
}

pub fn f74(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt24) -> R<Adt22> {
    let v4: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h319, _, _) => {
            h319
        }
    })?;
    let v5: Bytes = match v3.clone() {
        Adt24::C0(h320, _, _) => {
            h320
        }
    };
    let m321 = f42(v0.clone(), v4);
    match m321 {
        None => {
            Ok(Adt22::C0(f4(v0, f40(v1, v4), f7(f7(f3(Str::lit("unclosed ")), f8(v5)), f3(Str::lit(" environment"))))?, v2, Adt25::C0(v3)))
        }
        Some(v6) => {
            if f43(v6.clone(), 0u64, f3(Str::lit("\\end"))) {
                f78(v0, v1, v2, v3)
            } else {
                if f43(v6.clone(), 0u64, f3(Str::lit("\\begin"))) {
                    let v7: Adt18 = f1(v0.clone(), v1)?;
                    let v8: Bytes = match v7.clone() {
                        Adt18::C0(_, h322, _) => {
                            h322
                        }
                    };
                    if f2(v8.clone(), f3(Str::lit("section"))) {
                        f79(v0, match v7.clone() {
                            Adt18::C0(_, _, h323) => {
                                h323
                            }
                        }, v2, v3, match v7 {
                            Adt18::C0(h324, _, _) => {
                                h324
                            }
                        })
                    } else {
                        let m325 = f80(v8.clone());
                        match m325 {
                            None => {
                                Ok(Adt22::C0(f16(v0, match v7 {
                                    Adt18::C0(_, _, h328) => {
                                        h328
                                    }
                                }, f3(Str::lit("LLL1004")), f7(f8(v8), f3(Str::lit(" is not a block environment"))))?, v2, Adt25::C0(v3)))
                            }
                            Some(v9) => {
                                f81(v0, match v7.clone() {
                                    Adt18::C0(_, _, h326) => {
                                        h326
                                    }
                                }, v2, v3, match v7 {
                                    Adt18::C0(h327, _, _) => {
                                        h327
                                    }
                                }, v8, v9)
                            }
                        }
                    }
                } else {
                    Ok(Adt22::C0(f4(v0, f40(v1, v4), f7(f7(f7(f7(f3(Str::lit("expected a block or `\\end{")), v5), f3(Str::lit("}`, found `"))), match v6 {
                        Adt0::C0(_, _, _, _, _, _, _, h329) => {
                            h329
                        }
                    }), f3(Str::lit("`"))))?, v2, Adt25::C0(v3)))
                }
            }
        }
    }
}

pub fn f75(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt26, v4: Adt27, v5: List<Adt28>) -> R<Adt22> {
    let v6: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h330, _, _) => {
            h330
        }
    })?;
    let m331 = f42(v0.clone(), v6);
    match m331 {
        None => {
            Ok(Adt22::C0(f4(v0, f40(v1, v6), f7(f7(f3(Str::lit("unclosed ")), f8(match v3.clone() {
                Adt26::C0(_, h332, _, _) => {
                    h332
                }
            })), f3(Str::lit(" environment"))))?, v2, Adt25::C1(v3, v4, v5)))
        }
        Some(v7) => {
            if f43(v7.clone(), 0u64, f3(Str::lit("\\end"))) {
                let v8: Adt19 = f50(v0.clone(), v1, match v3.clone() {
                    Adt26::C0(_, h333, _, _) => {
                        h333
                    }
                })?;
                f82(v0, v8, v2, v4, v5, Adt12::C2(std::rc::Rc::new(f83(match v3.clone() {
                    Adt26::C0(_, _, _, h334) => {
                        h334
                    }
                })), match v3 {
                    Adt26::C0(h335, _, _, _) => {
                        h335
                    }
                }))
            } else {
                if f43(v7, 0u64, f3(Str::lit("\\begin"))) {
                    f84(v0, v1, v2, v3, v4, v5)
                } else {
                    let v9: Adt33 = f85(v0, v1)?;
                    Ok(Adt22::C0(match v9.clone() {
                        Adt33::C0(_, h336) => {
                            h336
                        }
                    }, v2, Adt25::C1(f86(v3, Adt12::C3(match v9 {
                        Adt33::C0(h337, _) => {
                            h337
                        }
                    })), v4, v5)))
                }
            }
        }
    }
}

pub fn f76(v0: Bytes, v1: bool, v2: bool) -> R<Adt32> {
    let (h338, _) = f87(Fn10::F88(v0.clone()), {
        let a339 = v0;
        length_bytes(a339)
    }, Adt32::C0(0u64, v1, v2))?;
    Ok(h338)
}

pub fn f77(v0: List<Bytes>) -> R<Option<List<Bytes>>> {
    let m340 = v0.clone();
    match m340.uncons() {
        None => {
            Ok(None::<List<Bytes>>)
        }
        Some((_, v2)) => {
            let m341 = v2;
            match m341.uncons() {
                None => {
                    Ok(None::<List<Bytes>>)
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f89(Fn11::F90, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m343 = {
                        let (h342, _) = v5;
                        h342
                    };
                    match m343 {
                        None => {
                            Ok(Some(f59({
                                let (_, h345) = v5;
                                h345
                            })?))
                        }
                        Some(v6) => {
                            Ok(Some(f59(List::cons(v6, {
                                let (_, h344) = v5;
                                h344
                            }))?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f78(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt24) -> R<Adt22> {
    let v4: Adt19 = f50(v0, v1, match v3.clone() {
        Adt24::C0(h346, _, _) => {
            h346
        }
    })?;
    let v5: Adt6 = f91(match v3 {
        Adt24::C0(_, _, h347) => {
            h347
        }
    });
    let m348 = v2.clone();
    match m348.uncons() {
        None => {
            Ok(Adt22::C0(v4, v2, Adt25::C2(v5)))
        }
        Some((v6, v7)) => {
            Ok(Adt22::C0(v4, v7, Adt25::C0(f92(match v6.clone() {
                Adt23::C0(h349, _, _, _, _, _) => {
                    h349
                }
            }, Adt6::C2(match v6.clone() {
                Adt23::C0(_, h350, _, _, _, _) => {
                    h350
                }
            }, match v6.clone() {
                Adt23::C0(_, _, h351, _, _, _) => {
                    h351
                }
            }, match v6.clone() {
                Adt23::C0(_, _, _, h352, _, _) => {
                    h352
                }
            }, match v6.clone() {
                Adt23::C0(_, _, _, _, h353, _) => {
                    h353
                }
            }, std::rc::Rc::new(v5), match v6 {
                Adt23::C0(_, _, _, _, _, h354) => {
                    h354
                }
            })))))
        }
    }
}

pub fn f79(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt24, v4: u64) -> R<Adt22> {
    let v5: u64 = {
        let a356 = match v3.clone() {
            Adt24::C0(_, h355, _) => {
                h355
            }
        };
        let a357 = 1u64;
        nat_add(a356, a357)?
    };
    let a359 = match v0.clone() {
        Adt17::C0(_, _, _, h358) => {
            h358
        }
    };
    let a360 = v5;
    if nat_lt(a359, a360) {
        Ok(Adt22::C0(f16(v0.clone(), v1, f3(Str::lit("LLS8002")), f93(v0, f3(Str::lit("section nesting")), v5)?)?, v2, Adt25::C0(v3)))
    } else {
        let v6: Adt20 = f94(v0.clone(), v1, f3(Str::lit("env.section")))?;
        let v7: Adt20 = f5(v0.clone(), f11(v0.clone(), match v6.clone() {
            Adt20::C0(_, h361) => {
                h361
            }
        }, f3(Str::lit("\\heading")))?)?;
        let v8: Option<Adt20> = if f10(v0.clone(), {
            let m364 = match v7.clone() {
                Adt20::C0(_, h363) => {
                    h363
                }
            };
            match m364 {
                Adt19::C0(h362, _, _) => {
                    h362
                }
            }
        }, f3(Str::lit("\\parameters")))? {
            Some(f5(v0.clone(), f11(v0, match v7.clone() {
                Adt20::C0(_, h365) => {
                    h365
                }
            }, f3(Str::lit("\\parameters")))?)?)
        } else {
            None::<Adt20>
        };
        let v10: Adt19 = {
            let m366 = v8.clone();
            match m366 {
                None => {
                    match v7.clone() {
                        Adt20::C0(_, h368) => {
                            h368
                        }
                    }
                }
                Some(v9) => {
                    match v9 {
                        Adt20::C0(_, h367) => {
                            h367
                        }
                    }
                }
            }
        };
        Ok(Adt22::C0(v10, List::cons(Adt23::C0(v3, match v6 {
            Adt20::C0(h369, _) => {
                h369
            }
        }, {
            let m372 = match v7.clone() {
                Adt20::C0(h371, _) => {
                    h371
                }
            };
            match m372 {
                Adt5::C0(h370, _, _) => {
                    h370
                }
            }
        }, {
            let m375 = match v7 {
                Adt20::C0(h374, _) => {
                    h374
                }
            };
            match m375 {
                Adt5::C0(_, h373, _) => {
                    h373
                }
            }
        }, {
            let m376 = v8;
            match m376 {
                None => {
                    None::<(u64, u64)>
                }
                Some(v11) => {
                    Some(({
                        let m379 = match v11.clone() {
                            Adt20::C0(h378, _) => {
                                h378
                            }
                        };
                        match m379 {
                            Adt5::C0(h377, _, _) => {
                                h377
                            }
                        }
                    }, {
                        let m382 = match v11 {
                            Adt20::C0(h381, _) => {
                                h381
                            }
                        };
                        match m382 {
                            Adt5::C0(_, h380, _) => {
                                h380
                            }
                        }
                    }))
                }
            }
        }, v4), v2), Adt25::C0(Adt24::C0(f3(Str::lit("section")), v5, List::<Adt6>::nil()))))
    }
}

pub fn f80(v0: Bytes) -> Option<Adt8> {
    if f2(v0.clone(), f3(Str::lit("typedefinition"))) {
        Some(Adt8::C0)
    } else {
        if f2(v0.clone(), f3(Str::lit("termdefinition"))) {
            Some(Adt8::C1)
        } else {
            if f2(v0.clone(), f3(Str::lit("predicatedefinition"))) {
                Some(Adt8::C2)
            } else {
                if f2(v0.clone(), f3(Str::lit("theorem"))) {
                    Some(Adt8::C3)
                } else {
                    if f2(v0.clone(), f3(Str::lit("lemma"))) {
                        Some(Adt8::C4)
                    } else {
                        if f2(v0, f3(Str::lit("corollary"))) {
                            Some(Adt8::C5)
                        } else {
                            None::<Adt8>
                        }
                    }
                }
            }
        }
    }
}

pub fn f81(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt24, v4: u64, v5: Bytes, v6: Adt8) -> R<Adt22> {
    let v7: Bytes = f7(f3(Str::lit("env.")), v5.clone());
    let v8: Adt20 = f94(v0.clone(), v1, v7.clone())?;
    let v9: bool = f95(v6.clone());
    let v11: Option<Adt20> = if v9 {
        None::<Adt20>
    } else {
        let v10: Adt20 = f5(v0.clone(), match v8.clone() {
            Adt20::C0(_, h383) => {
                h383
            }
        })?;
        Some(Adt20::C0(match v10.clone() {
            Adt20::C0(h384, _) => {
                h384
            }
        }, f6(v0.clone(), match v10.clone() {
            Adt20::C0(_, h385) => {
                h385
            }
        }, match v10 {
            Adt20::C0(h386, _) => {
                h386
            }
        }, v7)?))
    };
    let v13: Adt34 = f96(v0.clone(), {
        let m387 = v11.clone();
        match m387 {
            None => {
                match v8.clone() {
                    Adt20::C0(_, h389) => {
                        h389
                    }
                }
            }
            Some(v12) => {
                match v12 {
                    Adt20::C0(_, h388) => {
                        h388
                    }
                }
            }
        }
    })?;
    let v14: Adt33 = f85(v0.clone(), match v13.clone() {
        Adt34::C0(_, h390) => {
            h390
        }
    })?;
    let v15: Adt19 = match v14.clone() {
        Adt33::C0(_, h391) => {
            h391
        }
    };
    let v17: Adt27 = Adt27::C0(v3.clone(), v5, v6, match v8 {
        Adt20::C0(h392, _) => {
            h392
        }
    }, {
        let m393 = v11;
        match m393 {
            None => {
                None::<Adt5>
            }
            Some(v16) => {
                Some(match v16 {
                    Adt20::C0(h394, _) => {
                        h394
                    }
                })
            }
        }
    }, match v13 {
        Adt34::C0(h395, _) => {
            h395
        }
    }, match v14 {
        Adt33::C0(h396, _) => {
            h396
        }
    }, v4);
    let v18: bool = f25(v0.clone(), match v15.clone() {
        Adt19::C0(h397, _, _) => {
            h397
        }
    }, f3(Str::lit("proof")))?;
    if v9 {
        if v18 {
            let v19: Adt18 = f1(v0.clone(), v15)?;
            f97(v0, match v19.clone() {
                Adt18::C0(_, _, h398) => {
                    h398
                }
            }, v2, match v19 {
                Adt18::C0(h399, _, _) => {
                    h399
                }
            }, f3(Str::lit("proof")), 1u64, v17, List::<Adt28>::nil())
        } else {
            Ok(Adt22::C0(f16(v0, v15, f3(Str::lit("LLF5005")), f3(Str::lit("a theorem-like declaration without a proof is not valid")))?, v2, Adt25::C0(v3)))
        }
    } else {
        if v18 {
            Ok(Adt22::C0(f4(v0, v15, f3(Str::lit("no definitional component accepts a proof environment")))?, v2, Adt25::C0(v3)))
        } else {
            f98(v0, v15, v2, v17, None::<Adt12>)
        }
    }
}

pub fn f82(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt27, v4: List<Adt28>, v5: Adt12) -> R<Adt22> {
    let m400 = v4;
    match m400.uncons() {
        None => {
            f98(v0, v1, v2, v3, Some(v5))
        }
        Some((v6, v7)) => {
            let m401 = v6;
            match m401 {
                Adt28::C0(v8, v9, v10, v11) => {
                    let v12: Adt19 = f50(v0, v1, f3(Str::lit("have")))?;
                    Ok(Adt22::C0(v12, v2, Adt25::C1(f86(v8, Adt12::C4(v9, v10, std::rc::Rc::new(v5), v11)), v3, v7)))
                }
                Adt28::C1(v13, v14) => {
                    f99(v0, v1, v2, v3, v7, Adt29::C0(match v13.clone() {
                        Adt29::C0(h402, _, _, _, _, _) => {
                            h402
                        }
                    }, match v13.clone() {
                        Adt29::C0(_, h403, _, _, _, _) => {
                            h403
                        }
                    }, match v13.clone() {
                        Adt29::C0(_, _, h404, _, _, _) => {
                            h404
                        }
                    }, match v13.clone() {
                        Adt29::C0(_, _, _, h405, _, _) => {
                            h405
                        }
                    }, List::cons((v14, v5), match v13.clone() {
                        Adt29::C0(_, _, _, _, h406, _) => {
                            h406
                        }
                    }), match v13 {
                        Adt29::C0(_, _, _, _, _, h407) => {
                            h407
                        }
                    }))
                }
                Adt28::C2(v15, v16, v17, v18) => {
                    f100(v0, v1, v2, v3, v7, Adt30::C0(match v15.clone() {
                        Adt30::C0(h408, _, _, _, _, _) => {
                            h408
                        }
                    }, match v15.clone() {
                        Adt30::C0(_, h409, _, _, _, _) => {
                            h409
                        }
                    }, match v15.clone() {
                        Adt30::C0(_, _, h410, _, _, _) => {
                            h410
                        }
                    }, match v15.clone() {
                        Adt30::C0(_, _, _, h411, _, _) => {
                            h411
                        }
                    }, List::cons(Adt31::C0(v16, v17, v18, v5), match v15.clone() {
                        Adt30::C0(_, _, _, _, h412, _) => {
                            h412
                        }
                    }), match v15 {
                        Adt30::C0(_, _, _, _, _, h413) => {
                            h413
                        }
                    }))
                }
            }
        }
    }
}

pub fn f83(v0: List<Adt12>) -> Adt12 {
    f101(Fn12::F102, Adt12::C0, v0)
}

pub fn f84(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt26, v4: Adt27, v5: List<Adt28>) -> R<Adt22> {
    let v6: Adt18 = f1(v0.clone(), v1)?;
    let v7: Bytes = match v6.clone() {
        Adt18::C0(_, h414, _) => {
            h414
        }
    };
    let v8: u64 = match v6.clone() {
        Adt18::C0(h415, _, _) => {
            h415
        }
    };
    let v9: Adt19 = match v6 {
        Adt18::C0(_, _, h416) => {
            h416
        }
    };
    if f2(v7.clone(), f3(Str::lit("have"))) {
        f103(v0, v9, v2, v3, v4, v5, v8)
    } else {
        if f2(v7.clone(), f3(Str::lit("rewrite"))) {
            Ok(f104(v9.clone(), v2, v3, v4, v5, f105(v0, v9, v8)?))
        } else {
            if f2(v7.clone(), f3(Str::lit("simplify"))) {
                Ok(f104(v9.clone(), v2, v3, v4, v5, f106(v0, v9, v8)?))
            } else {
                if f2(v7.clone(), f3(Str::lit("apply"))) {
                    let v10: Adt20 = f5(v0.clone(), v9)?;
                    f99(v0, match v10.clone() {
                        Adt20::C0(_, h417) => {
                            h417
                        }
                    }, v2, v4, v5, Adt29::C0(v3, f3(Str::lit("premise")), f3(Str::lit("apply")), Some(match v10 {
                        Adt20::C0(h418, _) => {
                            h418
                        }
                    }), List::<(u64, Adt12)>::nil(), v8))
                } else {
                    if f2(v7.clone(), f3(Str::lit("constructor"))) {
                        f99(v0, v9, v2, v4, v5, Adt29::C0(v3, f3(Str::lit("branch")), f3(Str::lit("constructor")), None::<Adt5>, List::<(u64, Adt12)>::nil(), v8))
                    } else {
                        if if f2(v7.clone(), f3(Str::lit("cases"))) {
                            true
                        } else {
                            f2(v7.clone(), f3(Str::lit("induction")))
                        } {
                            let v11: Adt20 = f5(v0.clone(), v9)?;
                            f100(v0, match v11.clone() {
                                Adt20::C0(_, h419) => {
                                    h419
                                }
                            }, v2, v4, v5, Adt30::C0(v3, f2(v7.clone(), f3(Str::lit("induction"))), v7, match v11 {
                                Adt20::C0(h420, _) => {
                                    h420
                                }
                            }, List::<Adt31>::nil(), v8))
                        } else {
                            if f2(v7.clone(), f3(Str::lit("calculate"))) {
                                Ok(f104(v9.clone(), v2, v3, v4, v5, f107(v0, v9, v8)?))
                            } else {
                                Ok(Adt22::C0(f16(v0, v9, f3(Str::lit("LLL1004")), f7(f8(v7), f3(Str::lit(" is not a proof environment"))))?, v2, Adt25::C1(v3, v4, v5)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f85(v0: Adt17, v1: Adt19) -> R<Adt33> {
    let v2: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h421, _, _) => {
            h421
        }
    })?;
    let v3: Adt35 = {
        let (h422, _) = f108(Fn13::F109(v0.clone()), {
            let a424 = match v0.clone() {
                Adt17::C0(_, _, h423, _) => {
                    h423
                }
            };
            let a425 = 1u64;
            nat_add(a424, a425)?
        }, Adt35::C0(v2, 0u64, 0u64, 0u64))?;
        h422
    };
    let v4: u64 = match v3.clone() {
        Adt35::C0(h426, _, _, _) => {
            h426
        }
    };
    let a428 = match v3 {
        Adt35::C0(_, _, _, h427) => {
            h427
        }
    };
    let a429 = 1u64;
    if nat_eq(a428, a429) {
        Ok(Adt33::C0(Adt11::C0(v2, v4), f40(v1, {
            let a430 = v4;
            let a431 = 1u64;
            nat_add(a430, a431)?
        })))
    } else {
        Ok(Adt33::C0(Adt11::C0(v2, v4), f16(v0, f40(v1, v4), f3(Str::lit("LLP2001")), f3(Str::lit("a sentence must end with a period")))?))
    }
}

pub fn f86(v0: Adt26, v1: Adt12) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(h432, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt26::C0(_, h433, _, _) => {
            h433
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h434, _) => {
            h434
        }
    }, List::cons(v1, match v0 {
        Adt26::C0(_, _, _, h435) => {
            h435
        }
    }))
}

pub fn f87(v0: Fn10, p440: u64, p441: Adt32) -> R<(Adt32, bool)> {
    let m439 = (p440, p441);
    {
        let mut m439 = m439;
        loop {
            let step = {
                let (v1, v2) = m439;
                let m436 = v1;
                if m436 == 0 {
                    Err::<(u64, Adt32), R<(Adt32, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m436 - 1;
                    let m438 = {
                        let c437: Fn10 = v0.clone();
                        c437.apply(v2.clone())?
                    };
                    match m438 {
                        None => {
                            Err::<(u64, Adt32), R<(Adt32, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt32), R<(Adt32, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Bytes, v1: Adt32) -> R<Option<Adt32>> {
    f110(v0, v1)
}

pub fn f89(v0: Fn11, p445: (Option<Bytes>, List<Bytes>), p446: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m444 = (p445, p446);
    {
        let mut m444 = m444;
        loop {
            let step = {
                let (v1, v2) = m444;
                let m442 = v2;
                match m442.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c443: Fn11 = v0.clone();
                            c443.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f111(v0, v1)
}

pub fn f91(v0: List<Adt6>) -> Adt6 {
    f112(Fn14::F113, Adt6::C0, v0)
}

pub fn f92(v0: Adt24, v1: Adt6) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(h447, _, _) => {
            h447
        }
    }, match v0.clone() {
        Adt24::C0(_, h448, _) => {
            h448
        }
    }, List::cons(v1, match v0 {
        Adt24::C0(_, _, h449) => {
            h449
        }
    }))
}

pub fn f93(v0: Adt17, v1: Bytes, v2: u64) -> R<Bytes> {
    Ok(f7(f7(f7(f7(f7(f3(Str::lit("max_scope_depth exceeded in phase parse (")), v1), f3(Str::lit("): configured "))), f114(match v0 {
        Adt17::C0(_, _, _, h450) => {
            h450
        }
    })?), f3(Str::lit(", nesting depth "))), f114(v2)?))
}

pub fn f94(v0: Adt17, v1: Adt19, v2: Bytes) -> R<Adt20> {
    let v3: Adt20 = f5(v0.clone(), v1)?;
    let v4: Bytes = {
        let m453 = match v3.clone() {
            Adt20::C0(h452, _) => {
                h452
            }
        };
        match m453 {
            Adt5::C0(_, _, h451) => {
                h451
            }
        }
    };
    Ok(Adt20::C0(match v3.clone() {
        Adt20::C0(h454, _) => {
            h454
        }
    }, if f115(v4.clone())? {
        f6(v0, match v3.clone() {
            Adt20::C0(_, h455) => {
                h455
            }
        }, match v3 {
            Adt20::C0(h456, _) => {
                h456
            }
        }, v2)?
    } else {
        f4(v0, match v3 {
            Adt20::C0(_, h457) => {
                h457
            }
        }, f7(f8(v4), f3(Str::lit(" is not a component ID"))))?
    }))
}

pub fn f95(v0: Adt8) -> bool {
    let m458 = v0;
    match m458 {
        Adt8::C0 => {
            false
        }
        Adt8::C1 => {
            false
        }
        Adt8::C2 => {
            false
        }
        Adt8::C3 => {
            true
        }
        Adt8::C4 => {
            true
        }
        Adt8::C5 => {
            true
        }
    }
}

pub fn f96(v0: Adt17, v1: Adt19) -> R<Adt34> {
    let v2: u64 = match v1.clone() {
        Adt19::C0(h459, _, _) => {
            h459
        }
    };
    if f10(v0.clone(), v2, f3(Str::lit("\\noaxioms")))? {
        let v3: Adt19 = f11(v0, v1, f3(Str::lit("\\noaxioms")))?;
        Ok(Adt34::C0(Adt9::C0(Adt10::C0, List::<Bytes>::nil(), {
            let a461 = match v3.clone() {
                Adt19::C0(h460, _, _) => {
                    h460
                }
            };
            let a462 = 1u64;
            nat_sub(a461, a462)
        }, None::<Adt5>), v3))
    } else {
        if f10(v0.clone(), v2, f3(Str::lit("\\allowaxioms")))? {
            f116(v0, v1, Adt10::C1, f3(Str::lit("\\allowaxioms")), f3(Str::lit("allowaxioms")))
        } else {
            if f10(v0.clone(), v2, f3(Str::lit("\\exactaxioms")))? {
                f116(v0, v1, Adt10::C2, f3(Str::lit("\\exactaxioms")), f3(Str::lit("exactaxioms")))
            } else {
                Ok(Adt34::C0(Adt9::C0(Adt10::C0, List::<Bytes>::nil(), 0u64, None::<Adt5>), f4(v0, v1, f3(Str::lit("every definition and theorem-like declaration requires exactly one axiom policy")))?))
            }
        }
    }
}

pub fn f97(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: u64, v4: Bytes, v5: u64, v6: Adt27, v7: List<Adt28>) -> R<Adt22> {
    Ok(Adt22::C0({
        let a464 = match v0.clone() {
            Adt17::C0(_, _, _, h463) => {
                h463
            }
        };
        let a465 = v5;
        if nat_lt(a464, a465) {
            f16(v0.clone(), v1, f3(Str::lit("LLS8002")), f93(v0, f3(Str::lit("proof environment nesting")), v5)?)?
        } else {
            v1
        }
    }, v2, Adt25::C1(Adt26::C0(v3, v4, v5, List::<Adt12>::nil()), v6, v7)))
}

pub fn f98(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt27, v4: Option<Adt12>) -> R<Adt22> {
    let v5: Adt19 = f50(v0, v1, match v3.clone() {
        Adt27::C0(_, h466, _, _, _, _, _, _) => {
            h466
        }
    })?;
    Ok(Adt22::C0(v5.clone(), v2, Adt25::C0(f92(match v3.clone() {
        Adt27::C0(h467, _, _, _, _, _, _, _) => {
            h467
        }
    }, Adt6::C3(Adt7::C0(match v3.clone() {
        Adt27::C0(_, _, h468, _, _, _, _, _) => {
            h468
        }
    }, match v3.clone() {
        Adt27::C0(_, _, _, h469, _, _, _, _) => {
            h469
        }
    }, match v3.clone() {
        Adt27::C0(_, _, _, _, h470, _, _, _) => {
            h470
        }
    }, match v3.clone() {
        Adt27::C0(_, _, _, _, _, h471, _, _) => {
            h471
        }
    }, match v3.clone() {
        Adt27::C0(_, _, _, _, _, _, h472, _) => {
            h472
        }
    }, v4, match v3 {
        Adt27::C0(_, _, _, _, _, _, _, h473) => {
            h473
        }
    }, match v5 {
        Adt19::C0(h474, _, _) => {
            h474
        }
    }))))))
}

pub fn f99(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt27, v4: List<Adt28>, v5: Adt29) -> R<Adt22> {
    let v6: Bytes = match v5.clone() {
        Adt29::C0(_, h475, _, _, _, _) => {
            h475
        }
    };
    if f25(v0.clone(), match v1.clone() {
        Adt19::C0(h476, _, _) => {
            h476
        }
    }, v6.clone())? {
        let v7: Adt18 = f1(v0.clone(), v1)?;
        let v8: Adt20 = f5(v0.clone(), match v7.clone() {
            Adt18::C0(_, _, h477) => {
                h477
            }
        })?;
        let v9: Adt19 = f6(v0.clone(), match v8.clone() {
            Adt20::C0(_, h478) => {
                h478
            }
        }, match v8.clone() {
            Adt20::C0(h479, _) => {
                h479
            }
        }, f7(f3(Str::lit("env.")), v6.clone()))?;
        let m483 = f117({
            let m482 = match v8.clone() {
                Adt20::C0(h481, _) => {
                    h481
                }
            };
            match m482 {
                Adt5::C0(_, _, h480) => {
                    h480
                }
            }
        })?;
        match m483 {
            None => {
                Ok(Adt22::C0(f4(v0, v9, f7(f8({
                    let m492 = match v8 {
                        Adt20::C0(h491, _) => {
                            h491
                        }
                    };
                    match m492 {
                        Adt5::C0(_, _, h490) => {
                            h490
                        }
                    }
                }), f3(Str::lit(" is not a decimal label"))))?, v2, Adt25::C1(match v5 {
                    Adt29::C0(h493, _, _, _, _, _) => {
                        h493
                    }
                }, v3, v4)))
            }
            Some(v10) => {
                f97(v0, v9, v2, match v7 {
                    Adt18::C0(h484, _, _) => {
                        h484
                    }
                }, v6, {
                    let a488 = {
                        let m487 = match v5.clone() {
                            Adt29::C0(h486, _, _, _, _, _) => {
                                h486
                            }
                        };
                        match m487 {
                            Adt26::C0(_, _, h485, _) => {
                                h485
                            }
                        }
                    };
                    let a489 = 1u64;
                    nat_add(a488, a489)?
                }, v3, List::cons(Adt28::C1(v5, v10), v4))
            }
        }
    } else {
        let v11: Adt19 = f50(v0, v1, match v5.clone() {
            Adt29::C0(_, _, h494, _, _, _) => {
                h494
            }
        })?;
        let v12: Adt12 = f118(match v5.clone() {
            Adt29::C0(_, _, _, _, h495, _) => {
                h495
            }
        });
        Ok(Adt22::C0(v11, v2, Adt25::C1(f86(match v5.clone() {
            Adt29::C0(h496, _, _, _, _, _) => {
                h496
            }
        }, {
            let m498 = match v5.clone() {
                Adt29::C0(_, _, _, h497, _, _) => {
                    h497
                }
            };
            match m498 {
                None => {
                    Adt12::C8(std::rc::Rc::new(v12), match v5 {
                        Adt29::C0(_, _, _, _, _, h500) => {
                            h500
                        }
                    })
                }
                Some(v13) => {
                    Adt12::C7(v13, std::rc::Rc::new(v12), match v5 {
                        Adt29::C0(_, _, _, _, _, h499) => {
                            h499
                        }
                    })
                }
            }
        }), v3, v4)))
    }
}

pub fn f100(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt27, v4: List<Adt28>, v5: Adt30) -> R<Adt22> {
    if f25(v0.clone(), match v1.clone() {
        Adt19::C0(h501, _, _) => {
            h501
        }
    }, f3(Str::lit("case")))? {
        let v6: Adt18 = f1(v0.clone(), v1)?;
        let v7: Adt20 = f5(v0.clone(), match v6.clone() {
            Adt18::C0(_, _, h502) => {
                h502
            }
        })?;
        let v8: Adt20 = f5(v0.clone(), f11(v0.clone(), f6(v0.clone(), match v7.clone() {
            Adt20::C0(_, h503) => {
                h503
            }
        }, match v7.clone() {
            Adt20::C0(h504, _) => {
                h504
            }
        }, f3(Str::lit("env.case")))?, f3(Str::lit("\\bind")))?)?;
        let v9: Adt19 = f6(v0.clone(), match v8.clone() {
            Adt20::C0(_, h505) => {
                h505
            }
        }, match v8.clone() {
            Adt20::C0(h506, _) => {
                h506
            }
        }, f3(Str::lit("bind")))?;
        let v10: Bytes = {
            let m509 = match v8.clone() {
                Adt20::C0(h508, _) => {
                    h508
                }
            };
            match m509 {
                Adt5::C0(_, _, h507) => {
                    h507
                }
            }
        };
        f97(v0, v9, v2, match v6 {
            Adt18::C0(h510, _, _) => {
                h510
            }
        }, f3(Str::lit("case")), {
            let a514 = {
                let m513 = match v5.clone() {
                    Adt30::C0(h512, _, _, _, _, _) => {
                        h512
                    }
                };
                match m513 {
                    Adt26::C0(_, _, h511, _) => {
                        h511
                    }
                }
            };
            let a515 = 1u64;
            nat_add(a514, a515)?
        }, v3, List::cons(Adt28::C2(v5, match v7 {
            Adt20::C0(h516, _) => {
                h516
            }
        }, {
            let a518 = {
                let a517 = v10.clone();
                length_bytes(a517)
            };
            let a519 = 0u64;
            if nat_eq(a518, a519) {
                List::<Bytes>::nil()
            } else {
                f119(v10, 59u8)?
            }
        }, match v8 {
            Adt20::C0(h520, _) => {
                h520
            }
        }), v4))
    } else {
        let v11: Adt19 = f50(v0, v1, match v5.clone() {
            Adt30::C0(_, _, h521, _, _, _) => {
                h521
            }
        })?;
        Ok(Adt22::C0(v11, v2, Adt25::C1(f86(match v5.clone() {
            Adt30::C0(h522, _, _, _, _, _) => {
                h522
            }
        }, Adt12::C9(match v5.clone() {
            Adt30::C0(_, h523, _, _, _, _) => {
                h523
            }
        }, match v5.clone() {
            Adt30::C0(_, _, _, h524, _, _) => {
                h524
            }
        }, std::rc::Rc::new(f120(match v5.clone() {
            Adt30::C0(_, _, _, _, h525, _) => {
                h525
            }
        })), match v5 {
            Adt30::C0(_, _, _, _, _, h526) => {
                h526
            }
        })), v3, v4)))
    }
}

pub fn f101(v0: Fn12, p530: Adt12, p531: List<Adt12>) -> Adt12 {
    let m529 = (p530, p531);
    {
        let mut m529 = m529;
        loop {
            let step = {
                let (v1, v2) = m529;
                let m527 = v2;
                match m527.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt12>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt12>), Adt12>(({
                            let c528: Fn12 = v0.clone();
                            c528.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m529 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f103(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt26, v4: Adt27, v5: List<Adt28>, v6: u64) -> R<Adt22> {
    let v7: Adt20 = f5(v0.clone(), v1)?;
    let v8: Adt33 = f85(v0.clone(), f6(v0.clone(), match v7.clone() {
        Adt20::C0(_, h532) => {
            h532
        }
    }, match v7.clone() {
        Adt20::C0(h533, _) => {
            h533
        }
    }, f3(Str::lit("env.have")))?)?;
    let v9: Adt18 = f1(v0.clone(), match v8.clone() {
        Adt33::C0(_, h534) => {
            h534
        }
    })?;
    if f2(match v9.clone() {
        Adt18::C0(_, h535, _) => {
            h535
        }
    }, f3(Str::lit("proof"))) {
        f97(v0, match v9.clone() {
            Adt18::C0(_, _, h536) => {
                h536
            }
        }, v2, match v9 {
            Adt18::C0(h537, _, _) => {
                h537
            }
        }, f3(Str::lit("proof")), {
            let a539 = match v3.clone() {
                Adt26::C0(_, _, h538, _) => {
                    h538
                }
            };
            let a540 = 1u64;
            nat_add(a539, a540)?
        }, v4, List::cons(Adt28::C0(v3, match v7 {
            Adt20::C0(h541, _) => {
                h541
            }
        }, match v8 {
            Adt33::C0(h542, _) => {
                h542
            }
        }, v6), v5))
    } else {
        Ok(Adt22::C0(f4(v0, match v9 {
            Adt18::C0(_, _, h543) => {
                h543
            }
        }, f3(Str::lit("a have establishes its proposition with a nested proof")))?, v2, Adt25::C1(v3, v4, v5)))
    }
}

pub fn f104(_: Adt19, v1: List<Adt23>, v2: Adt26, v3: Adt27, v4: List<Adt28>, v5: Adt36) -> Adt22 {
    Adt22::C0(match v5.clone() {
        Adt36::C0(_, h544) => {
            h544
        }
    }, v1, Adt25::C1(f86(v2, match v5 {
        Adt36::C0(h545, _) => {
            h545
        }
    }), v3, v4))
}

pub fn f105(v0: Adt17, v1: Adt19, v2: u64) -> R<Adt36> {
    let v3: Adt20 = f5(v0.clone(), v1)?;
    let v4: Adt19 = f6(v0.clone(), match v3.clone() {
        Adt20::C0(_, h546) => {
            h546
        }
    }, match v3.clone() {
        Adt20::C0(h547, _) => {
            h547
        }
    }, f3(Str::lit("env.rewrite")))?;
    let v5: (Adt19, List<(bool, Adt5)>) = {
        let (h548, _) = f121(Fn15::F122(v0.clone()), {
            let a550 = match v0.clone() {
                Adt17::C0(_, _, h549, _) => {
                    h549
                }
            };
            let a551 = 1u64;
            nat_add(a550, a551)?
        }, (v4, List::<(bool, Adt5)>::nil()))?;
        h548
    };
    let v6: Adt19 = f50(v0.clone(), {
        let (h552, _) = v5.clone();
        h552
    }, f3(Str::lit("rewrite")))?;
    Ok(Adt36::C0(Adt12::C5(match v3 {
        Adt20::C0(h553, _) => {
            h553
        }
    }, f123({
        let (_, h554) = v5.clone();
        h554
    }), v2), {
        let a557 = {
            let a556 = {
                let (_, h555) = v5;
                h555
            };
            length_list(a556)
        };
        let a558 = 0u64;
        if nat_eq(a557, a558) {
            f16(v0, v6, f3(Str::lit("LLF5003")), f3(Str::lit("a rewrite requires at least one rule")))?
        } else {
            v6
        }
    }))
}

pub fn f106(v0: Adt17, v1: Adt19, v2: u64) -> R<Adt36> {
    let v3: Adt20 = f5(v0.clone(), v1)?;
    let v4: Adt19 = f6(v0.clone(), match v3.clone() {
        Adt20::C0(_, h559) => {
            h559
        }
    }, match v3.clone() {
        Adt20::C0(h560, _) => {
            h560
        }
    }, f3(Str::lit("env.simplify")))?;
    let v5: (Adt19, List<Adt5>) = {
        let (h561, _) = f19(Fn0::F124(v0.clone()), {
            let a563 = match v0.clone() {
                Adt17::C0(_, _, h562, _) => {
                    h562
                }
            };
            let a564 = 1u64;
            nat_add(a563, a564)?
        }, (v4, List::<Adt5>::nil()))?;
        h561
    };
    let v6: Adt19 = f50(v0.clone(), {
        let (h565, _) = v5.clone();
        h565
    }, f3(Str::lit("simplify")))?;
    Ok(Adt36::C0(Adt12::C6(match v3 {
        Adt20::C0(h566, _) => {
            h566
        }
    }, f13({
        let (_, h567) = v5.clone();
        h567
    }), v2), {
        let a570 = {
            let a569 = {
                let (_, h568) = v5;
                h568
            };
            length_list(a569)
        };
        let a571 = 0u64;
        if nat_eq(a570, a571) {
            f16(v0, v6, f3(Str::lit("LLF5003")), f3(Str::lit("a simplify requires at least one rule")))?
        } else {
            v6
        }
    }))
}

pub fn f107(v0: Adt17, v1: Adt19, v2: u64) -> R<Adt36> {
    let v3: Adt20 = f5(v0.clone(), f11(v0.clone(), v1, f3(Str::lit("\\start")))?)?;
    let v4: (Adt19, List<Adt13>) = {
        let (h572, _) = f125(Fn16::F126(v0.clone()), {
            let a574 = match v0.clone() {
                Adt17::C0(_, _, h573, _) => {
                    h573
                }
            };
            let a575 = 1u64;
            nat_add(a574, a575)?
        }, (match v3.clone() {
            Adt20::C0(_, h576) => {
                h576
            }
        }, List::<Adt13>::nil()))?;
        h572
    };
    let v5: Adt19 = f50(v0.clone(), {
        let (h577, _) = v4.clone();
        h577
    }, f3(Str::lit("calculate")))?;
    Ok(Adt36::C0(Adt12::C10(match v3 {
        Adt20::C0(h578, _) => {
            h578
        }
    }, f127({
        let (_, h579) = v4.clone();
        h579
    }), v2), {
        let a582 = {
            let a581 = {
                let (_, h580) = v4;
                h580
            };
            length_list(a581)
        };
        let a583 = 0u64;
        if nat_eq(a582, a583) {
            f16(v0, v5, f3(Str::lit("LLF5003")), f3(Str::lit("a calculation requires at least one step")))?
        } else {
            v5
        }
    }))
}

pub fn f108(v0: Fn13, p588: u64, p589: Adt35) -> R<(Adt35, bool)> {
    let m587 = (p588, p589);
    {
        let mut m587 = m587;
        loop {
            let step = {
                let (v1, v2) = m587;
                let m584 = v1;
                if m584 == 0 {
                    Err::<(u64, Adt35), R<(Adt35, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m584 - 1;
                    let m586 = {
                        let c585: Fn13 = v0.clone();
                        c585.apply(v2.clone())?
                    };
                    match m586 {
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
                Ok(next) => { m587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: Adt17, v1: Adt35) -> R<Option<Adt35>> {
    f128(v0, v1)
}

pub fn f110(v0: Bytes, v1: Adt32) -> R<Option<Adt32>> {
    let m591 = f129(v0, match v1.clone() {
        Adt32::C0(h590, _, _) => {
            h590
        }
    });
    match m591 {
        None => {
            Ok(None::<Adt32>)
        }
        Some(v2) => {
            let v3: u64 = {
                let a593 = match v1.clone() {
                    Adt32::C0(h592, _, _) => {
                        h592
                    }
                };
                let a594 = 1u64;
                nat_add(a593, a594)?
            };
            let v4: bool = match v1.clone() {
                Adt32::C0(_, h595, _) => {
                    h595
                }
            };
            if match v1 {
                Adt32::C0(_, _, h596) => {
                    h596
                }
            } {
                Ok(Some(Adt32::C0(v3, v4, false)))
            } else {
                if if v4 {
                    let a597 = v2;
                    let a598 = 92u8;
                    equal(a597, a598)
                } else {
                    false
                } {
                    Ok(Some(Adt32::C0(v3, v4, true)))
                } else {
                    let a599 = v2;
                    let a600 = 34u8;
                    if equal(a599, a600) {
                        Ok(Some(Adt32::C0(v3, {
                            let a601 = v4;
                            bool_not(a601)
                        }, false)))
                    } else {
                        Ok(Some(Adt32::C0(v3, v4, false)))
                    }
                }
            }
        }
    }
}

pub fn f111(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m603 = {
        let (h602, _) = v0;
        h602
    };
    match m603 {
        None => {
            (Some(v1), {
                let (_, h604) = v0;
                h604
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a605 = v2;
                let a606 = v1;
                append_bytes(a605, a606)
            }, {
                let (_, h607) = v0;
                h607
            }))
        }
    }
}

pub fn f112(v0: Fn14, p611: Adt6, p612: List<Adt6>) -> Adt6 {
    let m610 = (p611, p612);
    {
        let mut m610 = m610;
        loop {
            let step = {
                let (v1, v2) = m610;
                let m608 = v2;
                match m608.uncons() {
                    None => {
                        Err::<(Adt6, List<Adt6>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<Adt6>), Adt6>(({
                            let c609: Fn14 = v0.clone();
                            c609.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Adt6, v1: Adt6) -> Adt6 {
    Adt6::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f114(v0: u64) -> R<Bytes> {
    let v1: Bytes = f130(Str::lit("0"));
    let v2: Bytes = f131({
        let a649 = f132(v0, 10000000000000000000u64);
        let a650 = {
            let a647 = f132(v0, 1000000000000000000u64);
            let a648 = {
                let a645 = f132(v0, 100000000000000000u64);
                let a646 = {
                    let a643 = f132(v0, 10000000000000000u64);
                    let a644 = {
                        let a641 = f132(v0, 1000000000000000u64);
                        let a642 = {
                            let a639 = f132(v0, 100000000000000u64);
                            let a640 = {
                                let a637 = f132(v0, 10000000000000u64);
                                let a638 = {
                                    let a635 = f132(v0, 1000000000000u64);
                                    let a636 = {
                                        let a633 = f132(v0, 100000000000u64);
                                        let a634 = {
                                            let a631 = f132(v0, 10000000000u64);
                                            let a632 = {
                                                let a629 = f132(v0, 1000000000u64);
                                                let a630 = {
                                                    let a627 = f132(v0, 100000000u64);
                                                    let a628 = {
                                                        let a625 = f132(v0, 10000000u64);
                                                        let a626 = {
                                                            let a623 = f132(v0, 1000000u64);
                                                            let a624 = {
                                                                let a621 = f132(v0, 100000u64);
                                                                let a622 = {
                                                                    let a619 = f132(v0, 10000u64);
                                                                    let a620 = {
                                                                        let a617 = f132(v0, 1000u64);
                                                                        let a618 = {
                                                                            let a615 = f132(v0, 100u64);
                                                                            let a616 = {
                                                                                let a613 = f132(v0, 10u64);
                                                                                let a614 = f132(v0, 1u64);
                                                                                append_bytes(a613, a614)
                                                                            };
                                                                            append_bytes(a615, a616)
                                                                        };
                                                                        append_bytes(a617, a618)
                                                                    };
                                                                    append_bytes(a619, a620)
                                                                };
                                                                append_bytes(a621, a622)
                                                            };
                                                            append_bytes(a623, a624)
                                                        };
                                                        append_bytes(a625, a626)
                                                    };
                                                    append_bytes(a627, a628)
                                                };
                                                append_bytes(a629, a630)
                                            };
                                            append_bytes(a631, a632)
                                        };
                                        append_bytes(a633, a634)
                                    };
                                    append_bytes(a635, a636)
                                };
                                append_bytes(a637, a638)
                            };
                            append_bytes(a639, a640)
                        };
                        append_bytes(a641, a642)
                    };
                    append_bytes(a643, a644)
                };
                append_bytes(a645, a646)
            };
            append_bytes(a647, a648)
        };
        append_bytes(a649, a650)
    })?;
    let a651 = v0;
    let a652 = 0u64;
    if nat_eq(a651, a652) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f115(v0: Bytes) -> R<bool> {
    let m653 = f129(v0.clone(), 0u64);
    match m653 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f133(v1, 97u8, 122u8) {
                let a657 = {
                    let (h654, _) = f44(Fn4::F134(v0.clone()), {
                        let a655 = v0.clone();
                        length_bytes(a655)
                    }, 1u64)?;
                    h654
                };
                let a658 = {
                    let a656 = v0;
                    length_bytes(a656)
                };
                Ok(nat_eq(a657, a658))
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f116(v0: Adt17, v1: Adt19, v2: Adt10, v3: Bytes, v4: Bytes) -> R<Adt34> {
    let v5: Adt19 = f11(v0.clone(), v1, v3)?;
    let v6: u64 = {
        let a660 = match v5.clone() {
            Adt19::C0(h659, _, _) => {
                h659
            }
        };
        let a661 = 1u64;
        nat_sub(a660, a661)
    };
    let v7: Adt20 = f5(v0.clone(), v5)?;
    let v8: Adt19 = f6(v0.clone(), match v7.clone() {
        Adt20::C0(_, h662) => {
            h662
        }
    }, match v7.clone() {
        Adt20::C0(h663, _) => {
            h663
        }
    }, v4)?;
    let v9: (List<Bytes>, Option<Bytes>) = f135(Fn17::F136, (List::<Bytes>::nil(), None::<Bytes>), f119({
        let m666 = match v7.clone() {
            Adt20::C0(h665, _) => {
                h665
            }
        };
        match m666 {
            Adt5::C0(_, _, h664) => {
                h664
            }
        }
    }, 59u8)?)?;
    Ok(Adt34::C0(Adt9::C0(v2, f71(Fn9::F137, List::<Bytes>::nil(), {
        let (h667, _) = v9;
        h667
    })?, v6, Some(match v7 {
        Adt20::C0(h668, _) => {
            h668
        }
    })), {
        let m670 = {
            let (_, h669) = v9;
            h669
        };
        match m670 {
            None => {
                v8
            }
            Some(v10) => {
                f4(v0, v8, v10)?
            }
        }
    }))
}

pub fn f117(v0: Bytes) -> R<Option<u64>> {
    let v1: u64 = if f138(v0.clone(), 0u64, 43u8) {
        1u64
    } else {
        0u64
    };
    let a672 = v1;
    let a673 = {
        let a671 = v0.clone();
        length_bytes(a671)
    };
    if nat_lt(a672, a673) {
        let v2: Adt37 = {
            let (h674, _) = f139(Fn18::F140(v0.clone()), {
                let a675 = v0;
                length_bytes(a675)
            }, Adt37::C0(v1, 0u64, true))?;
            h674
        };
        if match v2.clone() {
            Adt37::C0(_, _, h676) => {
                h676
            }
        } {
            Ok(Some(match v2 {
                Adt37::C0(_, h677, _) => {
                    h677
                }
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f118(v0: List<(u64, Adt12)>) -> Adt12 {
    f141(Fn19::F142, Adt12::C0, v0)
}

pub fn f119(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a678 = v0.clone();
        length_bytes(a678)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h679, _) = f143(Fn20::F144(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h679
    };
    let v4: u64 = {
        let (h680, _) = {
            let (_, h681) = v3.clone();
            h681
        };
        h680
    };
    f59(List::cons(f24(v0, v4, {
        let a682 = v2;
        let a683 = v4;
        nat_sub(a682, a683)
    }), {
        let (_, h684) = {
            let (_, h685) = v3;
            h685
        };
        h684
    }))
}

pub fn f120(v0: List<Adt31>) -> Adt12 {
    f145(Fn21::F146, Adt12::C0, v0)
}

pub fn f121(v0: Fn15, p690: u64, p691: (Adt19, List<(bool, Adt5)>)) -> R<((Adt19, List<(bool, Adt5)>), bool)> {
    let m689 = (p690, p691);
    {
        let mut m689 = m689;
        loop {
            let step = {
                let (v1, v2) = m689;
                let m686 = v1;
                if m686 == 0 {
                    Err::<(u64, (Adt19, List<(bool, Adt5)>)), R<((Adt19, List<(bool, Adt5)>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m686 - 1;
                    let m688 = {
                        let c687: Fn15 = v0.clone();
                        c687.apply(v2.clone())?
                    };
                    match m688 {
                        None => {
                            Err::<(u64, (Adt19, List<(bool, Adt5)>)), R<((Adt19, List<(bool, Adt5)>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt19, List<(bool, Adt5)>)), R<((Adt19, List<(bool, Adt5)>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt17, v1: (Adt19, List<(bool, Adt5)>)) -> R<Option<(Adt19, List<(bool, Adt5)>)>> {
    f147(v0, v1)
}

pub fn f123(v0: List<(bool, Adt5)>) -> List<(bool, Adt5)> {
    f148(Fn22::F149, List::<(bool, Adt5)>::nil(), v0)
}

pub fn f124(v0: Adt17, v1: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
    f150(v0, v1)
}

pub fn f125(v0: Fn16, p696: u64, p697: (Adt19, List<Adt13>)) -> R<((Adt19, List<Adt13>), bool)> {
    let m695 = (p696, p697);
    {
        let mut m695 = m695;
        loop {
            let step = {
                let (v1, v2) = m695;
                let m692 = v1;
                if m692 == 0 {
                    Err::<(u64, (Adt19, List<Adt13>)), R<((Adt19, List<Adt13>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m692 - 1;
                    let m694 = {
                        let c693: Fn16 = v0.clone();
                        c693.apply(v2.clone())?
                    };
                    match m694 {
                        None => {
                            Err::<(u64, (Adt19, List<Adt13>)), R<((Adt19, List<Adt13>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt19, List<Adt13>)), R<((Adt19, List<Adt13>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m695 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: Adt17, v1: (Adt19, List<Adt13>)) -> R<Option<(Adt19, List<Adt13>)>> {
    f151(v0, v1)
}

pub fn f127(v0: List<Adt13>) -> List<Adt13> {
    f152(Fn23::F153, List::<Adt13>::nil(), v0)
}

pub fn f128(v0: Adt17, v1: Adt35) -> R<Option<Adt35>> {
    let a699 = match v1.clone() {
        Adt35::C0(_, _, _, h698) => {
            h698
        }
    };
    let a700 = 0u64;
    if nat_eq(a699, a700) {
        let m702 = f42(v0, match v1.clone() {
            Adt35::C0(h701, _, _, _) => {
                h701
            }
        });
        match m702 {
            None => {
                Ok(f154(v1, 2u64))
            }
            Some(v2) => {
                let v3: u64 = f61(v2.clone());
                let v4: u64 = match v1.clone() {
                    Adt35::C0(_, h703, _, _) => {
                        h703
                    }
                };
                let v5: u64 = match v1.clone() {
                    Adt35::C0(_, _, h704, _) => {
                        h704
                    }
                };
                let a705 = v3;
                let a706 = 0u64;
                if nat_eq(a705, a706) {
                    f155(v1, match v2 {
                        Adt0::C0(_, _, _, _, _, _, _, h707) => {
                            h707
                        }
                    })
                } else {
                    let a708 = v3;
                    let a709 = 3u64;
                    if nat_eq(a708, a709) {
                        let a710 = v5;
                        let a711 = 0u64;
                        if nat_eq(a710, a711) {
                            if f2(match v2.clone() {
                                Adt0::C0(_, _, _, _, _, _, _, h712) => {
                                    h712
                                }
                            }, f3(Str::lit("{"))) {
                                f156(v1, {
                                    let a713 = v4;
                                    let a714 = 1u64;
                                    nat_add(a713, a714)?
                                }, v5)
                            } else {
                                if f2(match v2 {
                                    Adt0::C0(_, _, _, _, _, _, _, h715) => {
                                        h715
                                    }
                                }, f3(Str::lit("}"))) {
                                    f156(v1, {
                                        let a716 = v4;
                                        let a717 = 1u64;
                                        nat_sub(a716, a717)
                                    }, v5)
                                } else {
                                    f156(v1, v4, v5)
                                }
                            }
                        } else {
                            f156(v1, v4, v5)
                        }
                    } else {
                        let a718 = v3;
                        let a719 = 4u64;
                        if if nat_eq(a718, a719) {
                            let a720 = v5;
                            let a721 = 0u64;
                            if nat_eq(a720, a721) {
                                let a722 = v4;
                                let a723 = 0u64;
                                if nat_eq(a722, a723) {
                                    f2(match v2 {
                                        Adt0::C0(_, _, _, _, _, _, _, h724) => {
                                            h724
                                        }
                                    }, f3(Str::lit(".")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Ok(f154(v1, 1u64))
                        } else {
                            f156(v1, v4, v5)
                        }
                    }
                }
            }
        }
    } else {
        Ok(None::<Adt35>)
    }
}

pub fn f129(v0: Bytes, v1: u64) -> Option<u8> {
    let a725 = v0;
    let a726 = v1;
    index_bytes(a725, a726)
}

pub fn f130(v0: Str) -> Bytes {
    let a727 = v0;
    utf8_encode(a727)
}

pub fn f131(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h728, _) = f44(Fn4::F157(v0.clone()), {
            let a729 = v0.clone();
            length_bytes(a729)
        }, 0u64)?;
        h728
    };
    Ok(f24(v0.clone(), v1, {
        let a731 = {
            let a730 = v0;
            length_bytes(a730)
        };
        let a732 = v1;
        nat_sub(a731, a732)
    }))
}

pub fn f132(v0: u64, v1: u64) -> Bytes {
    f24(f130(Str::lit("0123456789")), {
        let a736 = {
            let a733 = v0;
            let a734 = v1;
            let a735 = 0u64;
            nat_quot(a733, a734, a735)
        };
        let a737 = 10u64;
        let a738 = 0u64;
        nat_rem(a736, a737, a738)
    }, 1u64)
}

pub fn f133(v0: u8, v1: u8, v2: u8) -> bool {
    let m741 = {
        let a739 = v0;
        let a740 = v1;
        fixed_u8::checked_sub(a739, a740)
    };
    match m741 {
        None => {
            false
        }
        Some(_) => {
            let m744 = {
                let a742 = v2;
                let a743 = v0;
                fixed_u8::checked_sub(a742, a743)
            };
            match m744 {
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

pub fn f134(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f158(v0, v1)
}

pub fn f135(v0: Fn17, p748: (List<Bytes>, Option<Bytes>), p749: List<Bytes>) -> R<(List<Bytes>, Option<Bytes>)> {
    let m747 = (p748, p749);
    {
        let mut m747 = m747;
        loop {
            let step = {
                let (v1, v2) = m747;
                let m745 = v2;
                match m745.uncons() {
                    None => {
                        Err::<((List<Bytes>, Option<Bytes>), List<Bytes>), R<(List<Bytes>, Option<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Bytes>, Option<Bytes>), List<Bytes>), R<(List<Bytes>, Option<Bytes>)>>(({
                            let c746: Fn17 = v0.clone();
                            c746.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m747 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: (List<Bytes>, Option<Bytes>), v1: Bytes) -> R<(List<Bytes>, Option<Bytes>)> {
    f159(v0, v1)
}

pub fn f137(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    f160(v0, v1)
}

pub fn f138(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m750 = f129(v0, v1);
    match m750 {
        None => {
            false
        }
        Some(v3) => {
            let a751 = v3;
            let a752 = v2;
            equal(a751, a752)
        }
    }
}

pub fn f139(v0: Fn18, p757: u64, p758: Adt37) -> R<(Adt37, bool)> {
    let m756 = (p757, p758);
    {
        let mut m756 = m756;
        loop {
            let step = {
                let (v1, v2) = m756;
                let m753 = v1;
                if m753 == 0 {
                    Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m753 - 1;
                    let m755 = {
                        let c754: Fn18 = v0.clone();
                        c754.apply(v2.clone())?
                    };
                    match m755 {
                        None => {
                            Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt37), R<(Adt37, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: Bytes, v1: Adt37) -> R<Option<Adt37>> {
    f161(v0, v1)
}

pub fn f141(v0: Fn19, p762: Adt12, p763: List<(u64, Adt12)>) -> Adt12 {
    let m761 = (p762, p763);
    {
        let mut m761 = m761;
        loop {
            let step = {
                let (v1, v2) = m761;
                let m759 = v2;
                match m759.uncons() {
                    None => {
                        Err::<(Adt12, List<(u64, Adt12)>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<(u64, Adt12)>), Adt12>(({
                            let c760: Fn19 = v0.clone();
                            c760.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: Adt12, v1: (u64, Adt12)) -> Adt12 {
    Adt12::C11({
        let (h764, _) = v1;
        h764
    }, std::rc::Rc::new({
        let (_, h765) = v1;
        h765
    }), std::rc::Rc::new(v0))
}

pub fn f143(v0: Fn20, p770: u64, p771: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m769 = (p770, p771);
    {
        let mut m769 = m769;
        loop {
            let step = {
                let (v1, v2) = m769;
                let m766 = v1;
                if m766 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m766 - 1;
                    let m768 = {
                        let c767: Fn20 = v0.clone();
                        c767.apply(v2.clone())?
                    };
                    match m768 {
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
                Ok(next) => { m769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f162(v1, v0, v2)
}

pub fn f145(v0: Fn21, p775: Adt12, p776: List<Adt31>) -> Adt12 {
    let m774 = (p775, p776);
    {
        let mut m774 = m774;
        loop {
            let step = {
                let (v1, v2) = m774;
                let m772 = v2;
                match m772.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt31>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt31>), Adt12>(({
                            let c773: Fn21 = v0.clone();
                            c773.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: Adt12, v1: Adt31) -> Adt12 {
    Adt12::C12(match v1.clone() {
        Adt31::C0(h777, _, _, _) => {
            h777
        }
    }, match v1.clone() {
        Adt31::C0(_, h778, _, _) => {
            h778
        }
    }, match v1.clone() {
        Adt31::C0(_, _, h779, _) => {
            h779
        }
    }, std::rc::Rc::new(match v1 {
        Adt31::C0(_, _, _, h780) => {
            h780
        }
    }), std::rc::Rc::new(v0))
}

pub fn f147(v0: Adt17, v1: (Adt19, List<(bool, Adt5)>)) -> R<Option<(Adt19, List<(bool, Adt5)>)>> {
    let v2: Adt19 = {
        let (h781, _) = v1;
        h781
    };
    let m783 = match v2.clone() {
        Adt19::C0(_, _, h782) => {
            h782
        }
    };
    match m783 {
        None => {
            if f10(v0.clone(), match v2.clone() {
                Adt19::C0(h784, _, _) => {
                    h784
                }
            }, f3(Str::lit("\\forward")))? {
                let v4: Adt20 = f5(v0.clone(), f11(v0, v2, f3(Str::lit("\\forward")))?)?;
                Ok(Some((match v4.clone() {
                    Adt20::C0(_, h785) => {
                        h785
                    }
                }, List::cons((false, match v4 {
                    Adt20::C0(h786, _) => {
                        h786
                    }
                }), {
                    let (_, h787) = v1;
                    h787
                }))))
            } else {
                if f10(v0.clone(), match v2.clone() {
                    Adt19::C0(h788, _, _) => {
                        h788
                    }
                }, f3(Str::lit("\\backward")))? {
                    let v5: Adt20 = f5(v0.clone(), f11(v0, v2, f3(Str::lit("\\backward")))?)?;
                    Ok(Some((match v5.clone() {
                        Adt20::C0(_, h789) => {
                            h789
                        }
                    }, List::cons((true, match v5 {
                        Adt20::C0(h790, _) => {
                            h790
                        }
                    }), {
                        let (_, h791) = v1;
                        h791
                    }))))
                } else {
                    Ok(None::<(Adt19, List<(bool, Adt5)>)>)
                }
            }
        }
        Some(_) => {
            Ok(None::<(Adt19, List<(bool, Adt5)>)>)
        }
    }
}

pub fn f148(v0: Fn22, p795: List<(bool, Adt5)>, p796: List<(bool, Adt5)>) -> List<(bool, Adt5)> {
    let m794 = (p795, p796);
    {
        let mut m794 = m794;
        loop {
            let step = {
                let (v1, v2) = m794;
                let m792 = v2;
                match m792.uncons() {
                    None => {
                        Err::<(List<(bool, Adt5)>, List<(bool, Adt5)>), List<(bool, Adt5)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(bool, Adt5)>, List<(bool, Adt5)>), List<(bool, Adt5)>>(({
                            let c793: Fn22 = v0.clone();
                            c793.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m794 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: List<(bool, Adt5)>, v1: (bool, Adt5)) -> List<(bool, Adt5)> {
    List::cons(v1, v0)
}

pub fn f150(v0: Adt17, v1: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
    let v2: Adt19 = {
        let (h797, _) = v1;
        h797
    };
    let m799 = match v2.clone() {
        Adt19::C0(_, _, h798) => {
            h798
        }
    };
    match m799 {
        None => {
            if f10(v0.clone(), match v2.clone() {
                Adt19::C0(h800, _, _) => {
                    h800
                }
            }, f3(Str::lit("\\rule")))? {
                let v4: Adt20 = f5(v0.clone(), f11(v0, v2, f3(Str::lit("\\rule")))?)?;
                Ok(Some((match v4.clone() {
                    Adt20::C0(_, h801) => {
                        h801
                    }
                }, List::cons(match v4 {
                    Adt20::C0(h802, _) => {
                        h802
                    }
                }, {
                    let (_, h803) = v1;
                    h803
                }))))
            } else {
                Ok(None::<(Adt19, List<Adt5>)>)
            }
        }
        Some(_) => {
            Ok(None::<(Adt19, List<Adt5>)>)
        }
    }
}

pub fn f151(v0: Adt17, v1: (Adt19, List<Adt13>)) -> R<Option<(Adt19, List<Adt13>)>> {
    let v2: Adt19 = {
        let (h804, _) = v1;
        h804
    };
    let m806 = match v2.clone() {
        Adt19::C0(_, _, h805) => {
            h805
        }
    };
    match m806 {
        None => {
            if f10(v0.clone(), match v2.clone() {
                Adt19::C0(h807, _, _) => {
                    h807
                }
            }, f3(Str::lit("\\step")))? {
                let v4: Adt20 = f5(v0.clone(), f11(v0.clone(), v2, f3(Str::lit("\\step")))?)?;
                let v5: Adt20 = f5(v0.clone(), f6(v0.clone(), match v4.clone() {
                    Adt20::C0(_, h808) => {
                        h808
                    }
                }, match v4.clone() {
                    Adt20::C0(h809, _) => {
                        h809
                    }
                }, f3(Str::lit("step")))?)?;
                let v6: Adt20 = f5(v0, match v5.clone() {
                    Adt20::C0(_, h810) => {
                        h810
                    }
                })?;
                Ok(Some((match v6.clone() {
                    Adt20::C0(_, h811) => {
                        h811
                    }
                }, List::cons(Adt13::C0(match v4 {
                    Adt20::C0(h812, _) => {
                        h812
                    }
                }, match v5 {
                    Adt20::C0(h813, _) => {
                        h813
                    }
                }, match v6 {
                    Adt20::C0(h814, _) => {
                        h814
                    }
                }), {
                    let (_, h815) = v1;
                    h815
                }))))
            } else {
                Ok(None::<(Adt19, List<Adt13>)>)
            }
        }
        Some(_) => {
            Ok(None::<(Adt19, List<Adt13>)>)
        }
    }
}

pub fn f152(v0: Fn23, p819: List<Adt13>, p820: List<Adt13>) -> List<Adt13> {
    let m818 = (p819, p820);
    {
        let mut m818 = m818;
        loop {
            let step = {
                let (v1, v2) = m818;
                let m816 = v2;
                match m816.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>(({
                            let c817: Fn23 = v0.clone();
                            c817.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m818 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: List<Adt13>, v1: Adt13) -> List<Adt13> {
    List::cons(v1, v0)
}

pub fn f154(v0: Adt35, v1: u64) -> Option<Adt35> {
    Some(Adt35::C0(match v0.clone() {
        Adt35::C0(h821, _, _, _) => {
            h821
        }
    }, match v0.clone() {
        Adt35::C0(_, h822, _, _) => {
            h822
        }
    }, match v0 {
        Adt35::C0(_, _, h823, _) => {
            h823
        }
    }, v1))
}

pub fn f155(v0: Adt35, v1: Bytes) -> R<Option<Adt35>> {
    let v2: u64 = match v0.clone() {
        Adt35::C0(_, h824, _, _) => {
            h824
        }
    };
    let v3: u64 = match v0.clone() {
        Adt35::C0(_, _, h825, _) => {
            h825
        }
    };
    let a826 = v3;
    let a827 = 0u64;
    if nat_eq(a826, a827) {
        if f2(v1.clone(), f3(Str::lit("\\("))) {
            f156(v0, v2, 1u64)
        } else {
            if f2(v1.clone(), f3(Str::lit("\\["))) {
                f156(v0, v2, 2u64)
            } else {
                let a828 = v2;
                let a829 = 0u64;
                if if nat_eq(a828, a829) {
                    if f2(v1.clone(), f3(Str::lit("\\begin"))) {
                        true
                    } else {
                        f2(v1, f3(Str::lit("\\end")))
                    }
                } else {
                    false
                } {
                    Ok(f154(v0, 2u64))
                } else {
                    f156(v0, v2, 0u64)
                }
            }
        }
    } else {
        let a830 = v3;
        let a831 = 1u64;
        if if nat_eq(a830, a831) {
            f2(v1, f3(Str::lit("\\)")))
        } else {
            f2(v1, f3(Str::lit("\\]")))
        } {
            f156(v0, v2, 0u64)
        } else {
            f156(v0, v2, v3)
        }
    }
}

pub fn f156(v0: Adt35, v1: u64, v2: u64) -> R<Option<Adt35>> {
    Ok(Some(Adt35::C0({
        let a833 = match v0 {
            Adt35::C0(h832, _, _, _) => {
                h832
            }
        };
        let a834 = 1u64;
        nat_add(a833, a834)?
    }, v1, v2, 0u64)))
}

pub fn f157(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f163(v0, v1)
}

pub fn f158(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m835 = f129(v0, v1);
    match m835 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f164(v2) {
                Ok(Some({
                    let a836 = v1;
                    let a837 = 1u64;
                    nat_add(a836, a837)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f159(v0: (List<Bytes>, Option<Bytes>), v1: Bytes) -> R<(List<Bytes>, Option<Bytes>)> {
    let m839 = {
        let (_, h838) = v0.clone();
        h838
    };
    match m839 {
        None => {
            let a841 = {
                let a840 = v1.clone();
                length_bytes(a840)
            };
            let a842 = 0u64;
            if nat_eq(a841, a842) {
                Ok(({
                    let (h843, _) = v0;
                    h843
                }, Some(f3(Str::lit("empty entry in an axiom name list")))))
            } else {
                if f165(v1.clone())? {
                    if f166(v1.clone(), {
                        let (h844, _) = v0.clone();
                        h844
                    }) {
                        Ok(({
                            let (h845, _) = v0;
                            h845
                        }, Some(f7(f3(Str::lit("duplicate axiom name ")), f8(v1)))))
                    } else {
                        Ok((List::cons(v1, {
                            let (h846, _) = v0;
                            h846
                        }), None::<Bytes>))
                    }
                } else {
                    Ok(({
                        let (h847, _) = v0;
                        h847
                    }, Some(f7(f8(v1), f3(Str::lit(" is not a conservative ASCII Lean name"))))))
                }
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f160(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let v2: (List<Bytes>, Option<Bytes>) = f135(Fn17::F167, (List::<Bytes>::nil(), Some(v1)), v0)?;
    f59({
        let m849 = {
            let (_, h848) = v2;
            h848
        };
        match m849 {
            None => {
                let (h851, _) = v2;
                h851
            }
            Some(v3) => {
                List::cons(v3, {
                    let (h850, _) = v2;
                    h850
                })
            }
        }
    })
}

pub fn f161(v0: Bytes, v1: Adt37) -> R<Option<Adt37>> {
    if match v1.clone() {
        Adt37::C0(_, _, h852) => {
            h852
        }
    } {
        let m854 = f129(v0, match v1.clone() {
            Adt37::C0(h853, _, _) => {
                h853
            }
        });
        match m854 {
            None => {
                Ok(None::<Adt37>)
            }
            Some(v2) => {
                let v3: u64 = {
                    let a856 = match v1.clone() {
                        Adt37::C0(h855, _, _) => {
                            h855
                        }
                    };
                    let a857 = 1u64;
                    nat_add(a856, a857)?
                };
                if f168(v2) {
                    let v4: u64 = f169(v2)?;
                    let a864 = match v1.clone() {
                        Adt37::C0(_, h858, _) => {
                            h858
                        }
                    };
                    let a865 = {
                        let a861 = {
                            let a859 = 18446744073709551615u64;
                            let a860 = v4;
                            nat_sub(a859, a860)
                        };
                        let a862 = 10u64;
                        let a863 = 0u64;
                        nat_quot(a861, a862, a863)
                    };
                    if nat_le(a864, a865) {
                        Ok(Some(Adt37::C0(v3, {
                            let a869 = {
                                let a867 = match v1 {
                                    Adt37::C0(_, h866, _) => {
                                        h866
                                    }
                                };
                                let a868 = 10u64;
                                nat_mul(a867, a868)?
                            };
                            let a870 = v4;
                            nat_add(a869, a870)?
                        }, true)))
                    } else {
                        Ok(Some(Adt37::C0(v3, 0u64, false)))
                    }
                } else {
                    Ok(Some(Adt37::C0(v3, 0u64, false)))
                }
            }
        }
    } else {
        Ok(None::<Adt37>)
    }
}

pub fn f162(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h871, _) = v2.clone();
        h871
    };
    let v4: u64 = {
        let (h872, _) = {
            let (_, h873) = v2.clone();
            h873
        };
        h872
    };
    let v5: List<Bytes> = {
        let (_, h874) = {
            let (_, h875) = v2;
            h875
        };
        h874
    };
    let m876 = f129(v0.clone(), v3);
    match m876 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a877 = v6;
            let a878 = v1;
            if equal(a877, a878) {
                Ok(Some(({
                    let a879 = v3;
                    let a880 = 1u64;
                    nat_add(a879, a880)?
                }, ({
                    let a881 = v3;
                    let a882 = 1u64;
                    nat_add(a881, a882)?
                }, List::cons(f24(v0, v4, {
                    let a883 = v3;
                    let a884 = v4;
                    nat_sub(a883, a884)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a885 = v3;
                    let a886 = 1u64;
                    nat_add(a885, a886)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f138(v0, v1, 48u8) {
        Ok(Some({
            let a887 = v1;
            let a888 = 1u64;
            nat_add(a887, a888)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f164(v0: u8) -> bool {
    if f133(v0, 97u8, 122u8) {
        true
    } else {
        if f168(v0) {
            true
        } else {
            let a889 = v0;
            let a890 = 45u8;
            equal(a889, a890)
        }
    }
}

pub fn f165(v0: Bytes) -> R<bool> {
    let v1: (u64, bool) = {
        let (h891, _) = f170(Fn24::F171(v0.clone()), {
            let a892 = v0.clone();
            length_bytes(a892)
        }, (0u64, true))?;
        h891
    };
    let a895 = {
        let (h893, _) = v1;
        h893
    };
    let a896 = {
        let a894 = v0;
        length_bytes(a894)
    };
    if nat_eq(a895, a896) {
        let a898 = {
            let (_, h897) = v1;
            h897
        };
        Ok(bool_not(a898))
    } else {
        Ok(false)
    }
}

pub fn f166(v0: Bytes, v1: List<Bytes>) -> bool {
    f172(Fn25::F173(v0), false, v1)
}

pub fn f167(v0: (List<Bytes>, Option<Bytes>), v1: Bytes) -> (List<Bytes>, Option<Bytes>) {
    f174(v0, v1)
}

pub fn f168(v0: u8) -> bool {
    f133(v0, 48u8, 57u8)
}

pub fn f169(v0: u8) -> R<u64> {
    let a945 = {
        let v1: u8 = v0;
        let a943 = {
            let a919 = {
                let a907 = {
                    let a901 = {
                        let a899 = v1;
                        let a900 = 128u8;
                        fixed_u8::bit_and(a899, a900)
                    };
                    let a902 = 128u8;
                    if equal(a901, a902) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a908 = {
                    let a905 = {
                        let a903 = v1;
                        let a904 = 64u8;
                        fixed_u8::bit_and(a903, a904)
                    };
                    let a906 = 64u8;
                    if equal(a905, a906) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a907, a908)?
            };
            let a920 = {
                let a917 = {
                    let a911 = {
                        let a909 = v1;
                        let a910 = 32u8;
                        fixed_u8::bit_and(a909, a910)
                    };
                    let a912 = 32u8;
                    if equal(a911, a912) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a918 = {
                    let a915 = {
                        let a913 = v1;
                        let a914 = 16u8;
                        fixed_u8::bit_and(a913, a914)
                    };
                    let a916 = 16u8;
                    if equal(a915, a916) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a917, a918)?
            };
            nat_add(a919, a920)?
        };
        let a944 = {
            let a941 = {
                let a929 = {
                    let a923 = {
                        let a921 = v1;
                        let a922 = 8u8;
                        fixed_u8::bit_and(a921, a922)
                    };
                    let a924 = 8u8;
                    if equal(a923, a924) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a930 = {
                    let a927 = {
                        let a925 = v1;
                        let a926 = 4u8;
                        fixed_u8::bit_and(a925, a926)
                    };
                    let a928 = 4u8;
                    if equal(a927, a928) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a929, a930)?
            };
            let a942 = {
                let a939 = {
                    let a933 = {
                        let a931 = v1;
                        let a932 = 2u8;
                        fixed_u8::bit_and(a931, a932)
                    };
                    let a934 = 2u8;
                    if equal(a933, a934) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a940 = {
                    let a937 = {
                        let a935 = v1;
                        let a936 = 1u8;
                        fixed_u8::bit_and(a935, a936)
                    };
                    let a938 = 1u8;
                    if equal(a937, a938) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a939, a940)?
            };
            nat_add(a941, a942)?
        };
        nat_add(a943, a944)?
    };
    let a946 = 48u64;
    Ok(nat_sub(a945, a946))
}

pub fn f170(v0: Fn24, p951: u64, p952: (u64, bool)) -> R<((u64, bool), bool)> {
    let m950 = (p951, p952);
    {
        let mut m950 = m950;
        loop {
            let step = {
                let (v1, v2) = m950;
                let m947 = v1;
                if m947 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m947 - 1;
                    let m949 = {
                        let c948: Fn24 = v0.clone();
                        c948.apply(v2)?
                    };
                    match m949 {
                        None => {
                            Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, bool)), R<((u64, bool), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m950 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: Bytes, v1: (u64, bool)) -> R<Option<(u64, bool)>> {
    f175(v0, v1)
}

pub fn f172(v0: Fn25, p956: bool, p957: List<Bytes>) -> bool {
    let m955 = (p956, p957);
    {
        let mut m955 = m955;
        loop {
            let step = {
                let (v1, v2) = m955;
                let m953 = v2;
                match m953.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c954: Fn25 = v0.clone();
                            c954.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a958 = v2;
        let a959 = v0;
        equal(a958, a959)
    }
}

pub fn f174(v0: (List<Bytes>, Option<Bytes>), v1: Bytes) -> (List<Bytes>, Option<Bytes>) {
    let m961 = {
        let (_, h960) = v0;
        h960
    };
    match m961 {
        None => {
            (List::cons(v1, {
                let (h964, _) = v0;
                h964
            }), None::<Bytes>)
        }
        Some(v2) => {
            if f176(v2.clone(), v1.clone()) {
                (List::cons(v1, List::cons(v2, {
                    let (h962, _) = v0;
                    h962
                })), None::<Bytes>)
            } else {
                (List::cons(v1, {
                    let (h963, _) = v0;
                    h963
                }), Some(v2))
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: (u64, bool)) -> R<Option<(u64, bool)>> {
    let m966 = f129(v0, {
        let (h965, _) = v1;
        h965
    });
    match m966 {
        None => {
            Ok(None::<(u64, bool)>)
        }
        Some(v2) => {
            let v3: u64 = {
                let a968 = {
                    let (h967, _) = v1;
                    h967
                };
                let a969 = 1u64;
                nat_add(a968, a969)?
            };
            let v4: bool = if f177(v2) {
                true
            } else {
                let a970 = v2;
                let a971 = 95u8;
                equal(a970, a971)
            };
            let (_, h972) = v1;
            if h972 {
                if v4 {
                    Ok(Some((v3, false)))
                } else {
                    Ok(None::<(u64, bool)>)
                }
            } else {
                let a973 = v2;
                let a974 = 46u8;
                if equal(a973, a974) {
                    Ok(Some((v3, true)))
                } else {
                    if if v4 {
                        true
                    } else {
                        f168(v2)
                    } {
                        Ok(Some((v3, false)))
                    } else {
                        Ok(None::<(u64, bool)>)
                    }
                }
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: Bytes) -> bool {
    let a979 = {
        let a975 = v0;
        let a976 = v1;
        compare_bytes(a975, a976)
    };
    let a980 = {
        let a977 = Bytes::lit(&[]);
        let a978 = Bytes::lit(&[0x01]);
        compare_bytes(a977, a978)
    };
    equal(a979, a980)
}

pub fn f177(v0: u8) -> bool {
    let v1: bool = f133(v0, 97u8, 122u8);
    let v2: bool = f133(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}
