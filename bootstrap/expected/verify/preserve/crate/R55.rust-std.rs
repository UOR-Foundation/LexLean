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
    C1(Bytes),
    C2(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C3(u64, std::rc::Rc<Adt0>),
    C4,
    C5,
    C6(std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Bytes>, u64, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, bool, Adt0),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, bool, List<Adt2>, List<Adt2>, Option<bool>),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F2(k0, k1) => f2(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn1::F9 => f9(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F11,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F11 => f11(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F17(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn3::F17(k0) => f17(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F20,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn4::F20 => f20(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F22,
}

impl Fn5 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn5::F22 => f22(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F25(u64),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn6::F25(k0) => f25(*k0, p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt1 = {
        let (h1, _) = f1(Fn0::F2(v2, v1), {
            let a2 = v2;
            let a3 = 1u64;
            nat_add(a2, a3)?
        }, Adt1::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt2::C0(0u64, false, v0), List::<Adt2>::nil())))?;
        h1
    };
    Ok(f3(match v3 {
        Adt1::C0(h4, _, _) => {
            h4
        }
    }))
}

pub fn f1(v0: Fn0, p9: u64, p10: Adt1) -> R<(Adt1, bool)> {
    let m8 = (p9, p10);
    {
        let mut m8 = m8;
        loop {
            let step = {
                let (v1, v2) = m8;
                let m5 = v1;
                if m5 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m5 - 1;
                    let m7 = {
                        let c6: Fn0 = v0.clone();
                        c6.apply(v2.clone())?
                    };
                    match m7 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: Adt1) -> R<Option<Adt1>> {
    f4(v1, v0, v2)
}

pub fn f3(v0: List<Bytes>) -> Bytes {
    f5(f6(v0))
}

pub fn f4(v0: u64, v1: u64, v2: Adt1) -> R<Option<Adt1>> {
    let m12 = match v2.clone() {
        Adt1::C0(_, _, h11) => {
            h11
        }
    };
    match m12.uncons() {
        None => {
            Ok(None::<Adt1>)
        }
        Some((v3, v4)) => {
            Ok(Some(f7(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f5(v0: List<Bytes>) -> Bytes {
    let m15 = {
        let (h13, _) = f8(Fn1::F9, {
            let a14 = v0.clone();
            length_list(a14)
        }, v0);
        h13
    };
    match m15.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f6(v0: List<Bytes>) -> List<Bytes> {
    f10(Fn2::F11, List::<Bytes>::nil(), v0)
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt2, v4: List<Adt2>) -> R<Adt1> {
    let v5: u64 = match v3.clone() {
        Adt2::C0(h16, _, _) => {
            h16
        }
    };
    let v6: bool = match v3.clone() {
        Adt2::C0(_, h17, _) => {
            h17
        }
    };
    let m19 = match v3 {
        Adt2::C0(_, _, h18) => {
            h18
        }
    };
    match m19 {
        Adt0::C0 => {
            Ok(Adt1::C0(match v2.clone() {
                Adt1::C0(h20, _, _) => {
                    h20
                }
            }, match v2 {
                Adt1::C0(_, h21, _) => {
                    h21
                }
            }, v4))
        }
        Adt0::C1(v7) => {
            Ok(Adt1::C0(List::cons(v7.clone(), match v2.clone() {
                Adt1::C0(h22, _, _) => {
                    h22
                }
            }), {
                let a24 = match v2 {
                    Adt1::C0(_, h23, _) => {
                        h23
                    }
                };
                let a25 = f12(v7)?;
                nat_add(a24, a25)?
            }, v4))
        }
        Adt0::C2(r42, r43) => {
            let v8 = (*r42).clone();
            let v9 = (*r43).clone();
            Ok(Adt1::C0(match v2.clone() {
                Adt1::C0(h26, _, _) => {
                    h26
                }
            }, match v2 {
                Adt1::C0(_, h27, _) => {
                    h27
                }
            }, List::cons(Adt2::C0(v5, v6, v8), List::cons(Adt2::C0(v5, v6, v9), v4))))
        }
        Adt0::C3(v10, r44) => {
            let v11 = (*r44).clone();
            Ok(Adt1::C0(match v2.clone() {
                Adt1::C0(h28, _, _) => {
                    h28
                }
            }, match v2 {
                Adt1::C0(_, h29, _) => {
                    h29
                }
            }, List::cons(Adt2::C0({
                let a30 = v5;
                let a31 = v10;
                nat_add(a30, a31)?
            }, v6, v11), v4)))
        }
        Adt0::C4 => {
            if v6 {
                Ok(Adt1::C0(List::cons({
                    let a32 = Str::lit(" ");
                    utf8_encode(a32)
                }, match v2.clone() {
                    Adt1::C0(h33, _, _) => {
                        h33
                    }
                }), {
                    let a35 = match v2 {
                        Adt1::C0(_, h34, _) => {
                            h34
                        }
                    };
                    let a36 = 1u64;
                    nat_add(a35, a36)?
                }, v4))
            } else {
                f13(v2, v5, v4)
            }
        }
        Adt0::C5 => {
            if v6 {
                Ok(Adt1::C0(match v2.clone() {
                    Adt1::C0(h37, _, _) => {
                        h37
                    }
                }, match v2 {
                    Adt1::C0(_, h38, _) => {
                        h38
                    }
                }, v4))
            } else {
                f13(v2, v5, v4)
            }
        }
        Adt0::C6(r45) => {
            let v12 = (*r45).clone();
            let v13: bool = f14(v1, v0, match v2.clone() {
                Adt1::C0(_, h39, _) => {
                    h39
                }
            }, v4.clone(), Adt2::C0(v5, true, v12.clone()))?;
            Ok(Adt1::C0(match v2.clone() {
                Adt1::C0(h40, _, _) => {
                    h40
                }
            }, match v2 {
                Adt1::C0(_, h41, _) => {
                    h41
                }
            }, List::cons(Adt2::C0(v5, v13, v12), v4)))
        }
    }
}

pub fn f8(v0: Fn1, p50: u64, p51: List<Bytes>) -> (List<Bytes>, bool) {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v1, v2) = m49;
                let m46 = v1;
                if m46 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m46 - 1;
                    let m48 = {
                        let c47: Fn1 = v0.clone();
                        c47.apply(v2.clone())
                    };
                    match m48 {
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
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: List<Bytes>) -> Option<List<Bytes>> {
    f15(v0)
}

pub fn f10(v0: Fn2, p55: List<Bytes>, p56: List<Bytes>) -> List<Bytes> {
    let m54 = (p55, p56);
    {
        let mut m54 = m54;
        loop {
            let step = {
                let (v1, v2) = m54;
                let m52 = v2;
                match m52.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c53: Fn2 = v0.clone();
                            c53.apply(v1, v3)
                        }, v4))
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

pub fn f11(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f12(v0: Bytes) -> R<u64> {
    let (_, h57) = {
        let (h58, _) = f16(Fn3::F17(v0.clone()), {
            let a59 = v0;
            length_bytes(a59)
        }, (0u64, 0u64))?;
        h58
    };
    Ok(h57)
}

pub fn f13(v0: Adt1, v1: u64, v2: List<Adt2>) -> R<Adt1> {
    Ok(Adt1::C0(f18(match v0 {
        Adt1::C0(h60, _, _) => {
            h60
        }
    }, v1)?, v1, v2))
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: List<Adt2>, v4: Adt2) -> R<bool> {
    let v5: Adt3 = {
        let (h61, _) = f19(Fn4::F20, {
            let a62 = v0;
            let a63 = 1u64;
            nat_add(a62, a63)?
        }, Adt3::C0(0u64, {
            let a64 = v1;
            let a65 = v2;
            nat_sub(a64, a65)
        }, {
            let a66 = v1;
            let a67 = v2;
            nat_lt(a66, a67)
        }, List::cons(v4, List::<Adt2>::nil()), v3, None::<bool>))?;
        h61
    };
    let m69 = match v5 {
        Adt3::C0(_, _, _, _, _, h68) => {
            h68
        }
    };
    match m69 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f15(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m70 = v0.clone();
    match m70.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m71 = v2;
            match m71.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f21(Fn5::F22, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m73 = {
                        let (h72, _) = v5;
                        h72
                    };
                    match m73 {
                        None => {
                            Some(f6({
                                let (_, h75) = v5;
                                h75
                            }))
                        }
                        Some(v6) => {
                            Some(f6(List::cons(v6, {
                                let (_, h74) = v5;
                                h74
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f16(v0: Fn3, p80: u64, p81: (u64, u64)) -> R<((u64, u64), bool)> {
    let m79 = (p80, p81);
    {
        let mut m79 = m79;
        loop {
            let step = {
                let (v1, v2) = m79;
                let m76 = v1;
                if m76 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m76 - 1;
                    let m78 = {
                        let c77: Fn3 = v0.clone();
                        c77.apply(v2)?
                    };
                    match m78 {
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
                Ok(next) => { m79 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f23(v0, v1)
}

pub fn f18(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a82 = v1;
        let a83 = 256u64;
        let a84 = 0u64;
        nat_quot(a82, a83, a84)
    };
    let v3: List<Bytes> = {
        let (_, h85) = {
            let (h86, _) = f24(Fn6::F25(v2), {
                let a87 = v2;
                let a88 = 1u64;
                nat_add(a87, a88)?
            }, (0u64, List::cons({
                let a89 = Str::lit("\u{a}");
                utf8_encode(a89)
            }, v0)))?;
            h86
        };
        h85
    };
    Ok(List::cons(f26(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a90 = v1;
        let a91 = 256u64;
        let a92 = 0u64;
        nat_rem(a90, a91, a92)
    }), v3))
}

pub fn f19(v0: Fn4, p97: u64, p98: Adt3) -> R<(Adt3, bool)> {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v1, v2) = m96;
                let m93 = v1;
                if m93 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m93 - 1;
                    let m95 = {
                        let c94: Fn4 = v0.clone();
                        c94.apply(v2.clone())?
                    };
                    match m95 {
                        None => {
                            Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), R<(Adt3, bool)>>((v3, v4))
                        }
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

pub fn f20(v0: Adt3) -> R<Option<Adt3>> {
    f27(v0)
}

pub fn f21(v0: Fn5, p102: (Option<Bytes>, List<Bytes>), p103: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m101 = (p102, p103);
    {
        let mut m101 = m101;
        loop {
            let step = {
                let (v1, v2) = m101;
                let m99 = v2;
                match m99.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c100: Fn5 = v0.clone();
                            c100.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m101 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f28(v0, v1)
}

pub fn f23(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m105 = f29(v0, {
        let (h104, _) = v1;
        h104
    });
    match m105 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a150 = {
                    let a126 = {
                        let a114 = {
                            let a108 = {
                                let a106 = v3;
                                let a107 = 128u8;
                                fixed_u8::bit_and(a106, a107)
                            };
                            let a109 = 128u8;
                            if equal(a108, a109) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a115 = {
                            let a112 = {
                                let a110 = v3;
                                let a111 = 64u8;
                                fixed_u8::bit_and(a110, a111)
                            };
                            let a113 = 64u8;
                            if equal(a112, a113) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a114, a115)?
                    };
                    let a127 = {
                        let a124 = {
                            let a118 = {
                                let a116 = v3;
                                let a117 = 32u8;
                                fixed_u8::bit_and(a116, a117)
                            };
                            let a119 = 32u8;
                            if equal(a118, a119) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a125 = {
                            let a122 = {
                                let a120 = v3;
                                let a121 = 16u8;
                                fixed_u8::bit_and(a120, a121)
                            };
                            let a123 = 16u8;
                            if equal(a122, a123) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a124, a125)?
                    };
                    nat_add(a126, a127)?
                };
                let a151 = {
                    let a148 = {
                        let a136 = {
                            let a130 = {
                                let a128 = v3;
                                let a129 = 8u8;
                                fixed_u8::bit_and(a128, a129)
                            };
                            let a131 = 8u8;
                            if equal(a130, a131) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a137 = {
                            let a134 = {
                                let a132 = v3;
                                let a133 = 4u8;
                                fixed_u8::bit_and(a132, a133)
                            };
                            let a135 = 4u8;
                            if equal(a134, a135) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a136, a137)?
                    };
                    let a149 = {
                        let a146 = {
                            let a140 = {
                                let a138 = v3;
                                let a139 = 2u8;
                                fixed_u8::bit_and(a138, a139)
                            };
                            let a141 = 2u8;
                            if equal(a140, a141) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a147 = {
                            let a144 = {
                                let a142 = v3;
                                let a143 = 1u8;
                                fixed_u8::bit_and(a142, a143)
                            };
                            let a145 = 1u8;
                            if equal(a144, a145) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a146, a147)?
                    };
                    nat_add(a148, a149)?
                };
                nat_add(a150, a151)?
            };
            let a152 = v4;
            let a153 = 128u64;
            if nat_lt(a152, a153) {
                Ok(Some(({
                    let a155 = {
                        let (h154, _) = v1;
                        h154
                    };
                    let a156 = 1u64;
                    nat_add(a155, a156)?
                }, {
                    let a158 = {
                        let (_, h157) = v1;
                        h157
                    };
                    let a159 = 1u64;
                    nat_add(a158, a159)?
                })))
            } else {
                let a160 = v4;
                let a161 = 192u64;
                if nat_lt(a160, a161) {
                    Ok(Some(({
                        let a163 = {
                            let (h162, _) = v1;
                            h162
                        };
                        let a164 = 1u64;
                        nat_add(a163, a164)?
                    }, {
                        let (_, h165) = v1;
                        h165
                    })))
                } else {
                    Ok(Some(({
                        let a167 = {
                            let (h166, _) = v1;
                            h166
                        };
                        let a168 = 1u64;
                        nat_add(a167, a168)?
                    }, {
                        let a170 = {
                            let (_, h169) = v1;
                            h169
                        };
                        let a171 = 1u64;
                        nat_add(a170, a171)?
                    })))
                }
            }
        }
    }
}

pub fn f24(v0: Fn6, p176: u64, p177: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m175 = (p176, p177);
    {
        let mut m175 = m175;
        loop {
            let step = {
                let (v1, v2) = m175;
                let m172 = v1;
                if m172 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m172 - 1;
                    let m174 = {
                        let c173: Fn6 = v0.clone();
                        c173.apply(v2.clone())?
                    };
                    match m174 {
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
                Ok(next) => { m175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f30(v0, v1)
}

pub fn f26(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m181 = {
        let a178 = v0;
        let a179 = v1;
        let a180 = v2;
        slice_bytes(a178, a179, a180)
    };
    match m181 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f27(v0: Adt3) -> R<Option<Adt3>> {
    let m183 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, h182) => {
            h182
        }
    };
    match m183 {
        None => {
            if if match v0.clone() {
                Adt3::C0(_, _, h184, _, _, _) => {
                    h184
                }
            } {
                true
            } else {
                let a187 = match v0.clone() {
                    Adt3::C0(_, h185, _, _, _, _) => {
                        h185
                    }
                };
                let a188 = match v0.clone() {
                    Adt3::C0(h186, _, _, _, _, _) => {
                        h186
                    }
                };
                nat_lt(a187, a188)
            } {
                Ok(Some(f31(v0.clone(), match v0.clone() {
                    Adt3::C0(h189, _, _, _, _, _) => {
                        h189
                    }
                }, match v0.clone() {
                    Adt3::C0(_, _, _, h190, _, _) => {
                        h190
                    }
                }, match v0 {
                    Adt3::C0(_, _, _, _, h191, _) => {
                        h191
                    }
                }, Some(false))))
            } else {
                let m193 = match v0.clone() {
                    Adt3::C0(_, _, _, h192, _, _) => {
                        h192
                    }
                };
                match m193.uncons() {
                    None => {
                        let m196 = match v0.clone() {
                            Adt3::C0(_, _, _, _, h195, _) => {
                                h195
                            }
                        };
                        match m196.uncons() {
                            None => {
                                Ok(Some(f31(v0.clone(), match v0 {
                                    Adt3::C0(h197, _, _, _, _, _) => {
                                        h197
                                    }
                                }, List::<Adt2>::nil(), List::<Adt2>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f32(v0, v4, List::<Adt2>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f32(v0.clone(), v2, v3, match v0 {
                            Adt3::C0(_, _, _, _, h194, _) => {
                                h194
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt3>)
        }
    }
}

pub fn f28(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m199 = {
        let (h198, _) = v0;
        h198
    };
    match m199 {
        None => {
            (Some(v1), {
                let (_, h200) = v0;
                h200
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a201 = v2;
                let a202 = v1;
                append_bytes(a201, a202)
            }, {
                let (_, h203) = v0;
                h203
            }))
        }
    }
}

pub fn f29(v0: Bytes, v1: u64) -> Option<u8> {
    let a204 = v0;
    let a205 = v1;
    index_bytes(a204, a205)
}

pub fn f30(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a207 = {
        let (h206, _) = v1.clone();
        h206
    };
    let a208 = v0;
    if nat_lt(a207, a208) {
        Ok(Some(({
            let a210 = {
                let (h209, _) = v1;
                h209
            };
            let a211 = 1u64;
            nat_add(a210, a211)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h212) = v1;
            h212
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f31(v0: Adt3, v1: u64, v2: List<Adt2>, v3: List<Adt2>, v4: Option<bool>) -> Adt3 {
    Adt3::C0(v1, match v0.clone() {
        Adt3::C0(_, h213, _, _, _, _) => {
            h213
        }
    }, match v0 {
        Adt3::C0(_, _, h214, _, _, _) => {
            h214
        }
    }, v2, v3, v4)
}

pub fn f32(v0: Adt3, v1: Adt2, v2: List<Adt2>, v3: List<Adt2>) -> R<Adt3> {
    let v4: u64 = match v1.clone() {
        Adt2::C0(h215, _, _) => {
            h215
        }
    };
    let v5: bool = match v1.clone() {
        Adt2::C0(_, h216, _) => {
            h216
        }
    };
    let v6: u64 = match v0.clone() {
        Adt3::C0(h217, _, _, _, _, _) => {
            h217
        }
    };
    let m219 = match v1 {
        Adt2::C0(_, _, h218) => {
            h218
        }
    };
    match m219 {
        Adt0::C0 => {
            Ok(f31(v0, v6, v2, v3, None::<bool>))
        }
        Adt0::C1(v7) => {
            Ok(f31(v0, {
                let a220 = v6;
                let a221 = f12(v7)?;
                nat_add(a220, a221)?
            }, v2, v3, None::<bool>))
        }
        Adt0::C2(r226, r227) => {
            let v8 = (*r226).clone();
            let v9 = (*r227).clone();
            Ok(f31(v0, v6, List::cons(Adt2::C0(v4, v5, v8), List::cons(Adt2::C0(v4, v5, v9), v2)), v3, None::<bool>))
        }
        Adt0::C3(v10, r228) => {
            let v11 = (*r228).clone();
            Ok(f31(v0, v6, List::cons(Adt2::C0({
                let a222 = v4;
                let a223 = v10;
                nat_add(a222, a223)?
            }, v5, v11), v2), v3, None::<bool>))
        }
        Adt0::C4 => {
            if v5 {
                Ok(f31(v0, {
                    let a224 = v6;
                    let a225 = 1u64;
                    nat_add(a224, a225)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f31(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt0::C5 => {
            if v5 {
                Ok(f31(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f31(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt0::C6(r229) => {
            let v12 = (*r229).clone();
            Ok(f31(v0, v6, List::cons(Adt2::C0(v4, true, v12), v2), v3, None::<bool>))
        }
    }
}
