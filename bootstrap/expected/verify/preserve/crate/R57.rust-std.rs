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
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt0>),
    C4(std::rc::Rc<Adt0>),
    C5,
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3, u64, u64),
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
    C9,
    C10,
    C11(Bytes),
    C12(Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, List<Adt2>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, u64, List<Adt6>, Adt0, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(List<Adt0>),
    C1(Bytes, u64, List<(Bytes, Adt0)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Bytes>, bool, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F13,
}

impl Fn0 {
    pub fn apply(&self, p0: (Adt0, List<(Bytes, Adt0)>)) -> Option<(Adt0, List<(Bytes, Adt0)>)> {
        match self {
            Fn0::F13 => f13(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt0)) -> bool {
        match self {
            Fn1::F15(k0) => f15(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F18,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt0, p1: (Bytes, Adt0)) -> Adt0 {
        match self {
            Fn2::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F22,
}

impl Fn3 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn3::F22 => f22(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F24,
}

impl Fn4 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn4::F24 => f24(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F26(u64, Bytes),
    F35(Bytes, u64, Bytes, u64),
    F75(u64, Bytes),
    F120(u64, Bytes),
    F141(Bytes),
    F142(Bytes),
    F146(Bytes),
    F159(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F26(k0, k1) => f26(*k0, k1.clone(), p0),
            Fn5::F35(k0, k1, k2, k3) => f35(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn5::F75(k0, k1) => f75(*k0, k1.clone(), p0),
            Fn5::F120(k0, k1) => f120(*k0, k1.clone(), p0),
            Fn5::F141(k0) => f141(k0.clone(), p0),
            Fn5::F142(k0) => f142(k0.clone(), p0),
            Fn5::F146(k0) => f146(k0.clone(), p0),
            Fn5::F159(k0) => f159(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F28(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn6::F28(k0, k1) => f28(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F38,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn7::F38 => f38(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F40,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn8::F40 => f40(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F45,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn9::F45 => f45(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F47(u64, List<Adt2>),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn10::F47(k0, k1) => f47(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F58,
}

impl Fn11 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn11::F58 => f58(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F72(u64, Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn12::F72(k0, k1) => f72(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F84,
}

impl Fn13 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn13::F84 => f84(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F86,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn14::F86 => f86(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F100,
}

impl Fn15 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn15::F100 => f100(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F122,
}

impl Fn16 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn16::F122 => f122(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F134,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn17::F134 => f134(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F145(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn18::F145(k0) => f145(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F148(u64, Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn19::F148(k0, k1) => f148(*k0, k1.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Result<Adt0, Adt1>> {
    let m1 = f1(v0.clone())?;
    match m1 {
        Ok(v3) => {
            let m2 = v3;
            match m2 {
                Adt0::C0(_) => {
                    Ok(Err::<Adt0, Adt1>(f5(f9(v0)?, f4(Str::lit("a module is an object")))))
                }
                Adt0::C1(_, _) => {
                    Ok(Err::<Adt0, Adt1>(f5(f9(v0)?, f4(Str::lit("a module is an object")))))
                }
                Adt0::C2(_) => {
                    Ok(Err::<Adt0, Adt1>(f5(f9(v0)?, f4(Str::lit("a module is an object")))))
                }
                Adt0::C3(_) => {
                    Ok(Err::<Adt0, Adt1>(f5(f9(v0)?, f4(Str::lit("a module is an object")))))
                }
                Adt0::C4(r4) => {
                    let v4 = (*r4).clone();
                    let v5: List<(Bytes, Adt0)> = f2(v4, {
                        let a3 = v0.clone();
                        length_bytes(a3)
                    })?;
                    if f3(v5.clone(), f4(Str::lit("spec"))) {
                        Ok(Err::<Adt0, Adt1>(f5(f6(v0)?, f4(Str::lit("the `spec` of a module is implied by its language and is not written")))))
                    } else {
                        Ok(Ok::<Adt0, Adt1>(Adt0::C4(std::rc::Rc::new(f7(f8(List::cons((f4(Str::lit("spec")), Adt0::C2(v1)), v5))?)))))
                    }
                }
                Adt0::C5 => {
                    Ok(Err::<Adt0, Adt1>(f5(f9(v0)?, f4(Str::lit("a module is an object")))))
                }
                Adt0::C6(_, _) => {
                    Ok(Err::<Adt0, Adt1>(f5(f9(v0)?, f4(Str::lit("a module is an object")))))
                }
                Adt0::C7(_, _, _) => {
                    Ok(Err::<Adt0, Adt1>(f5(f9(v0)?, f4(Str::lit("a module is an object")))))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt0, Adt1>(v2))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<Adt0, Adt1>> {
    let m5 = f10(v0.clone())?;
    match m5 {
        Ok(v1) => {
            f11(v1, {
                let a6 = v0;
                length_bytes(a6)
            })
        }
        Err(v2) => {
            Ok(Err::<Adt0, Adt1>(v2))
        }
    }
}

pub fn f2(v0: Adt0, v1: u64) -> R<List<(Bytes, Adt0)>> {
    let (_, h7) = {
        let (h8, _) = f12(Fn0::F13, {
            let a9 = v1;
            let a10 = 1u64;
            nat_add(a9, a10)?
        }, (v0, List::<(Bytes, Adt0)>::nil()));
        h8
    };
    Ok(h7)
}

pub fn f3(v0: List<(Bytes, Adt0)>, v1: Bytes) -> bool {
    f14(Fn1::F15(v1), false, v0)
}

pub fn f4(v0: Str) -> Bytes {
    let a11 = v0;
    utf8_encode(a11)
}

pub fn f5(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f6(v0: Bytes) -> R<u64> {
    let m12 = f16(v0, f4(Str::lit("spec")))?;
    match m12 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f7(v0: List<(Bytes, Adt0)>) -> Adt0 {
    f17(Fn2::F18, Adt0::C5, f19(v0))
}

pub fn f8(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f20(f21(Fn3::F22, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m17 = {
        let (h13, _) = f23(Fn4::F24, {
            let a15 = {
                let a14 = v0;
                length_list(a14)
            };
            let a16 = 1u64;
            nat_add(a15, a16)?
        }, v1)?;
        h13
    };
    match m17.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f9(v0: Bytes) -> R<u64> {
    let v1: u64 = {
        let a18 = v0.clone();
        length_bytes(a18)
    };
    let v2: u64 = {
        let (h19, _) = f25(Fn5::F26(v1, v0), {
            let a20 = v1;
            let a21 = 1u64;
            nat_add(a20, a21)?
        }, 0u64)?;
        h19
    };
    let a22 = v2;
    let a23 = v1;
    if nat_lt(a22, a23) {
        Ok(v2)
    } else {
        Ok(0u64)
    }
}

pub fn f10(v0: Bytes) -> R<Result<List<Adt2>, Adt1>> {
    let v1: u64 = {
        let a24 = v0.clone();
        length_bytes(a24)
    };
    let v2: Adt4 = {
        let (h25, _) = f27(Fn6::F28(v1, v0), {
            let a26 = v1;
            let a27 = 1u64;
            nat_add(a26, a27)?
        }, Adt4::C0(0u64, List::<Adt2>::nil(), None::<Adt1>))?;
        h25
    };
    let m29 = match v2.clone() {
        Adt4::C0(_, _, h28) => {
            h28
        }
    };
    match m29 {
        None => {
            Ok(Ok::<List<Adt2>, Adt1>(f29(match v2 {
                Adt4::C0(_, h30, _) => {
                    h30
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt2>, Adt1>(v3))
        }
    }
}

pub fn f11(v0: List<Adt2>, v1: u64) -> R<Result<Adt0, Adt1>> {
    let v2: Adt5 = f30(v0.clone(), v1, 0u64, 0u64)?;
    let m32 = match v2.clone() {
        Adt5::C0(_, _, _, _, _, h31) => {
            h31
        }
    };
    match m32 {
        None => {
            let a34 = match v2.clone() {
                Adt5::C0(_, _, h33, _, _, _) => {
                    h33
                }
            };
            let a35 = 9u64;
            if nat_eq(a34, a35) {
                let a38 = match v2.clone() {
                    Adt5::C0(h36, _, _, _, _, _) => {
                        h36
                    }
                };
                let a39 = {
                    let a37 = v0.clone();
                    length_list(a37)
                };
                if nat_lt(a38, a39) {
                    Ok(Err::<Adt0, Adt1>(f31(f32(v0, v1, match v2 {
                        Adt5::C0(h40, _, _, _, _, _) => {
                            h40
                        }
                    }), f33(Str::lit("unexpected text after the value")))))
                } else {
                    Ok(Ok::<Adt0, Adt1>(match v2 {
                        Adt5::C0(_, _, _, _, h41, _) => {
                            h41
                        }
                    }))
                }
            } else {
                Ok(Err::<Adt0, Adt1>(f31(v1, f33(Str::lit("the reader ran out of steps")))))
            }
        }
        Some(v3) => {
            Ok(Err::<Adt0, Adt1>(v3))
        }
    }
}

pub fn f12(v0: Fn0, p46: u64, p47: (Adt0, List<(Bytes, Adt0)>)) -> ((Adt0, List<(Bytes, Adt0)>), bool) {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m42 = v1;
                if m42 == 0 {
                    Err::<(u64, (Adt0, List<(Bytes, Adt0)>)), ((Adt0, List<(Bytes, Adt0)>), bool)>((v2, false))
                } else {
                    let v3 = m42 - 1;
                    let m44 = {
                        let c43: Fn0 = v0.clone();
                        c43.apply(v2.clone())
                    };
                    match m44 {
                        None => {
                            Err::<(u64, (Adt0, List<(Bytes, Adt0)>)), ((Adt0, List<(Bytes, Adt0)>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt0, List<(Bytes, Adt0)>)), ((Adt0, List<(Bytes, Adt0)>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: (Adt0, List<(Bytes, Adt0)>)) -> Option<(Adt0, List<(Bytes, Adt0)>)> {
    f34(v0)
}

pub fn f14(v0: Fn1, p51: bool, p52: List<(Bytes, Adt0)>) -> bool {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v1, v2) = m50;
                let m48 = v2;
                match m48.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt0)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt0)>), bool>(({
                            let c49: Fn1 = v0.clone();
                            c49.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m50 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: bool, v2: (Bytes, Adt0)) -> bool {
    if v1 {
        true
    } else {
        let a54 = {
            let (h53, _) = v2;
            h53
        };
        let a55 = v0;
        equal(a54, a55)
    }
}

pub fn f16(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a56 = v0.clone();
        length_bytes(a56)
    };
    let v3: u64 = {
        let a57 = v1.clone();
        length_bytes(a57)
    };
    let v4: u64 = {
        let (h58, _) = f25(Fn5::F35(v1.clone(), v3, v0.clone(), v2), {
            let a59 = v2;
            let a60 = 1u64;
            nat_add(a59, a60)?
        }, 0u64)?;
        h58
    };
    if f36(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f17(v0: Fn2, p64: Adt0, p65: List<(Bytes, Adt0)>) -> Adt0 {
    let m63 = (p64, p65);
    {
        let mut m63 = m63;
        loop {
            let step = {
                let (v1, v2) = m63;
                let m61 = v2;
                match m61.uncons() {
                    None => {
                        Err::<(Adt0, List<(Bytes, Adt0)>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<(Bytes, Adt0)>), Adt0>(({
                            let c62: Fn2 = v0.clone();
                            c62.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m63 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Adt0, v1: (Bytes, Adt0)) -> Adt0 {
    Adt0::C7({
        let (h66, _) = v1;
        h66
    }, std::rc::Rc::new({
        let (_, h67) = v1;
        h67
    }), std::rc::Rc::new(v0))
}

pub fn f19(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f37(Fn7::F38, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f20(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f39(Fn8::F40, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f21(v0: Fn3, p71: List<List<(Bytes, Adt0)>>, p72: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m70 = (p71, p72);
    {
        let mut m70 = m70;
        loop {
            let step = {
                let (v1, v2) = m70;
                let m68 = v2;
                match m68.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c69: Fn3 = v0.clone();
                            c69.apply(v1, v3)
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

pub fn f22(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f23(v0: Fn4, p77: u64, p78: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, v2) = m76;
                let m73 = v1;
                if m73 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m73 - 1;
                    let m75 = {
                        let c74: Fn4 = v0.clone();
                        c74.apply(v2.clone())?
                    };
                    match m75 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m76 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f41(v0)
}

pub fn f25(v0: Fn5, p83: u64, p84: u64) -> R<(u64, bool)> {
    let m82 = (p83, p84);
    {
        let mut m82 = m82;
        loop {
            let step = {
                let (v1, v2) = m82;
                let m79 = v1;
                if m79 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m79 - 1;
                    let m81 = {
                        let c80: Fn5 = v0.clone();
                        c80.apply(v2)?
                    };
                    match m81 {
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
                Ok(next) => { m82 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f42(v1, v0, v2)
}

pub fn f27(v0: Fn6, p89: u64, p90: Adt4) -> R<(Adt4, bool)> {
    let m88 = (p89, p90);
    {
        let mut m88 = m88;
        loop {
            let step = {
                let (v1, v2) = m88;
                let m85 = v1;
                if m85 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m85 - 1;
                    let m87 = {
                        let c86: Fn6 = v0.clone();
                        c86.apply(v2.clone())?
                    };
                    match m87 {
                        None => {
                            Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), R<(Adt4, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m88 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f43(v1, v0, v2)
}

pub fn f29(v0: List<Adt2>) -> List<Adt2> {
    f44(Fn9::F45, List::<Adt2>::nil(), v0)
}

pub fn f30(v0: List<Adt2>, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let (h91, _) = f46(Fn10::F47(v1, v0.clone()), {
        let a95 = {
            let a93 = {
                let a92 = v0;
                length_list(a92)
            };
            let a94 = 4u64;
            nat_mul(a93, a94)?
        };
        let a96 = 8u64;
        nat_add(a95, a96)?
    }, Adt5::C0(v2, v3, 0u64, List::<Adt6>::nil(), Adt0::C5, None::<Adt1>))?;
    Ok(h91)
}

pub fn f31(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f32(v0: List<Adt2>, v1: u64, v2: u64) -> u64 {
    let m97 = f48(v0, v2);
    match m97 {
        None => {
            v1
        }
        Some(v3) => {
            match v3 {
                Adt2::C0(_, h98, _) => {
                    h98
                }
            }
        }
    }
}

pub fn f33(v0: Str) -> Bytes {
    let a99 = v0;
    utf8_encode(a99)
}

pub fn f34(v0: (Adt0, List<(Bytes, Adt0)>)) -> Option<(Adt0, List<(Bytes, Adt0)>)> {
    let m101 = {
        let (h100, _) = v0;
        h100
    };
    match m101 {
        Adt0::C0(_) => {
            None::<(Adt0, List<(Bytes, Adt0)>)>
        }
        Adt0::C1(_, _) => {
            None::<(Adt0, List<(Bytes, Adt0)>)>
        }
        Adt0::C2(_) => {
            None::<(Adt0, List<(Bytes, Adt0)>)>
        }
        Adt0::C3(_) => {
            None::<(Adt0, List<(Bytes, Adt0)>)>
        }
        Adt0::C4(_) => {
            None::<(Adt0, List<(Bytes, Adt0)>)>
        }
        Adt0::C5 => {
            None::<(Adt0, List<(Bytes, Adt0)>)>
        }
        Adt0::C6(_, _) => {
            None::<(Adt0, List<(Bytes, Adt0)>)>
        }
        Adt0::C7(v1, r103, r104) => {
            let v2 = (*r103).clone();
            let v3 = (*r104).clone();
            Some((v3, List::cons((v1, v2), {
                let (_, h102) = v0;
                h102
            })))
        }
    }
}

pub fn f35(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f36(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a107 = {
            let a105 = v4;
            let a106 = v1;
            nat_add(a105, a106)?
        };
        let a108 = v3;
        if nat_lt(a107, a108) {
            Ok(Some({
                let a109 = v4;
                let a110 = 1u64;
                nat_add(a109, a110)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f36(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m115 = {
        let a112 = v0;
        let a113 = v2;
        let a114 = {
            let a111 = v1.clone();
            length_bytes(a111)
        };
        slice_bytes(a112, a113, a114)
    };
    match m115 {
        None => {
            false
        }
        Some(v3) => {
            let a116 = v3;
            let a117 = v1;
            equal(a116, a117)
        }
    }
}

pub fn f37(v0: Fn7, p121: List<(Bytes, Adt0)>, p122: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m120 = (p121, p122);
    {
        let mut m120 = m120;
        loop {
            let step = {
                let (v1, v2) = m120;
                let m118 = v2;
                match m118.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c119: Fn7 = v0.clone();
                            c119.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f39(v0: Fn8, p126: List<List<(Bytes, Adt0)>>, p127: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m125 = (p126, p127);
    {
        let mut m125 = m125;
        loop {
            let step = {
                let (v1, v2) = m125;
                let m123 = v2;
                match m123.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c124: Fn8 = v0.clone();
                            c124.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f41(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a129 = {
        let a128 = v0.clone();
        length_list(a128)
    };
    let a130 = 1u64;
    if nat_le(a129, a130) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f49(v0)?))
    }
}

pub fn f42(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a131 = v2;
    let a132 = v1;
    if nat_lt(a131, a132) {
        let v3: u64 = f50(v0, v2)?;
        let a133 = 0u64;
        let a134 = v3;
        if nat_lt(a133, a134) {
            Ok(Some({
                let a135 = v2;
                let a136 = v3;
                nat_add(a135, a136)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m138 = match v2.clone() {
        Adt4::C0(_, _, h137) => {
            h137
        }
    };
    match m138 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt4::C0(h139, _, _) => {
                    h139
                }
            };
            let a140 = v4;
            let a141 = v1;
            if nat_lt(a140, a141) {
                let v5: u64 = f50(v0.clone(), v4)?;
                let a142 = 0u64;
                let a143 = v5;
                if nat_lt(a142, a143) {
                    Ok(Some(Adt4::C0({
                        let a144 = v4;
                        let a145 = v5;
                        nat_add(a144, a145)?
                    }, match v2 {
                        Adt4::C0(_, h146, _) => {
                            h146
                        }
                    }, None::<Adt1>)))
                } else {
                    if f51(v0.clone(), v4, 123u8) {
                        Ok(f52(v2, Adt3::C0, {
                            let a147 = v4;
                            let a148 = 1u64;
                            nat_add(a147, a148)?
                        }))
                    } else {
                        if f51(v0.clone(), v4, 125u8) {
                            Ok(f52(v2, Adt3::C1, {
                                let a149 = v4;
                                let a150 = 1u64;
                                nat_add(a149, a150)?
                            }))
                        } else {
                            if f51(v0.clone(), v4, 91u8) {
                                Ok(f52(v2, Adt3::C2, {
                                    let a151 = v4;
                                    let a152 = 1u64;
                                    nat_add(a151, a152)?
                                }))
                            } else {
                                if f51(v0.clone(), v4, 93u8) {
                                    Ok(f52(v2, Adt3::C3, {
                                        let a153 = v4;
                                        let a154 = 1u64;
                                        nat_add(a153, a154)?
                                    }))
                                } else {
                                    if f51(v0.clone(), v4, 40u8) {
                                        Ok(f52(v2, Adt3::C4, {
                                            let a155 = v4;
                                            let a156 = 1u64;
                                            nat_add(a155, a156)?
                                        }))
                                    } else {
                                        if f51(v0.clone(), v4, 41u8) {
                                            Ok(f52(v2, Adt3::C5, {
                                                let a157 = v4;
                                                let a158 = 1u64;
                                                nat_add(a157, a158)?
                                            }))
                                        } else {
                                            if f51(v0.clone(), v4, 44u8) {
                                                Ok(f52(v2, Adt3::C6, {
                                                    let a159 = v4;
                                                    let a160 = 1u64;
                                                    nat_add(a159, a160)?
                                                }))
                                            } else {
                                                if f51(v0.clone(), v4, 59u8) {
                                                    Ok(f52(v2, Adt3::C10, {
                                                        let a161 = v4;
                                                        let a162 = 1u64;
                                                        nat_add(a161, a162)?
                                                    }))
                                                } else {
                                                    if f51(v0.clone(), v4, 58u8) {
                                                        f53(v0, v2)
                                                    } else {
                                                        if f51(v0.clone(), v4, 34u8) {
                                                            f54(v0, v1, v2)
                                                        } else {
                                                            f55(v0, v1, v2)
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
            } else {
                Ok(None::<Adt4>)
            }
        }
        Some(_) => {
            Ok(None::<Adt4>)
        }
    }
}

pub fn f44(v0: Fn9, p166: List<Adt2>, p167: List<Adt2>) -> List<Adt2> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v1, v2) = m165;
                let m163 = v2;
                match m163.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c164: Fn9 = v0.clone();
                            c164.apply(v1, v3)
                        }, v4))
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

pub fn f45(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f46(v0: Fn10, p172: u64, p173: Adt5) -> R<(Adt5, bool)> {
    let m171 = (p172, p173);
    {
        let mut m171 = m171;
        loop {
            let step = {
                let (v1, v2) = m171;
                let m168 = v1;
                if m168 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m168 - 1;
                    let m170 = {
                        let c169: Fn10 = v0.clone();
                        c169.apply(v2.clone())?
                    };
                    match m170 {
                        None => {
                            Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), R<(Adt5, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: u64, v1: List<Adt2>, v2: Adt5) -> R<Option<Adt5>> {
    f56(v1, v0, v2)
}

pub fn f48(v0: List<Adt2>, v1: u64) -> Option<Adt2> {
    let a174 = v0;
    let a175 = v1;
    index_list(a174, a175)
}

pub fn f49(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f20({
        let (_, h176) = {
            let (h177, _) = f57(Fn11::F58, {
                let a179 = {
                    let a178 = v0.clone();
                    length_list(a178)
                };
                let a180 = 1u64;
                nat_add(a179, a180)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h177
        };
        h176
    }))
}

pub fn f50(v0: Bytes, v1: u64) -> R<u64> {
    if if f59(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f60(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f60(v0.clone(), v1, 194u8) {
            if if f60(v0.clone(), {
                let a181 = v1;
                let a182 = 1u64;
                nat_add(a181, a182)?
            }, 133u8) {
                true
            } else {
                f60(v0, {
                    let a183 = v1;
                    let a184 = 1u64;
                    nat_add(a183, a184)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f60(v0.clone(), v1, 225u8) {
                if if f60(v0.clone(), {
                    let a185 = v1;
                    let a186 = 1u64;
                    nat_add(a185, a186)?
                }, 154u8) {
                    f60(v0, {
                        let a187 = v1;
                        let a188 = 2u64;
                        nat_add(a187, a188)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f60(v0.clone(), v1, 226u8) {
                    if f60(v0.clone(), {
                        let a189 = v1;
                        let a190 = 1u64;
                        nat_add(a189, a190)?
                    }, 128u8) {
                        if if f59(v0.clone(), {
                            let a191 = v1;
                            let a192 = 2u64;
                            nat_add(a191, a192)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f60(v0.clone(), {
                                let a193 = v1;
                                let a194 = 2u64;
                                nat_add(a193, a194)?
                            }, 168u8) {
                                true
                            } else {
                                if f60(v0.clone(), {
                                    let a195 = v1;
                                    let a196 = 2u64;
                                    nat_add(a195, a196)?
                                }, 169u8) {
                                    true
                                } else {
                                    f60(v0, {
                                        let a197 = v1;
                                        let a198 = 2u64;
                                        nat_add(a197, a198)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f60(v0.clone(), {
                            let a199 = v1;
                            let a200 = 1u64;
                            nat_add(a199, a200)?
                        }, 129u8) {
                            f60(v0, {
                                let a201 = v1;
                                let a202 = 2u64;
                                nat_add(a201, a202)?
                            }, 159u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    if f60(v0.clone(), v1, 227u8) {
                        if if f60(v0.clone(), {
                            let a203 = v1;
                            let a204 = 1u64;
                            nat_add(a203, a204)?
                        }, 128u8) {
                            f60(v0, {
                                let a205 = v1;
                                let a206 = 2u64;
                                nat_add(a205, a206)?
                            }, 128u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m207 = f61(v0, v1);
    match m207 {
        None => {
            false
        }
        Some(v3) => {
            let a208 = v3;
            let a209 = v2;
            equal(a208, a209)
        }
    }
}

pub fn f52(v0: Adt4, v1: Adt3, v2: u64) -> Option<Adt4> {
    Some(Adt4::C0(v2, List::cons(Adt2::C0(v1, match v0.clone() {
        Adt4::C0(h210, _, _) => {
            h210
        }
    }, v2), match v0 {
        Adt4::C0(_, h211, _) => {
            h211
        }
    }), None::<Adt1>))
}

pub fn f53(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    let v2: u64 = match v1.clone() {
        Adt4::C0(h212, _, _) => {
            h212
        }
    };
    if f51(v0.clone(), {
        let a213 = v2;
        let a214 = 1u64;
        nat_add(a213, a214)?
    }, 61u8) {
        Ok(f52(v1, Adt3::C8, {
            let a215 = v2;
            let a216 = 2u64;
            nat_add(a215, a216)?
        }))
    } else {
        if f51(v0, {
            let a217 = v2;
            let a218 = 1u64;
            nat_add(a217, a218)?
        }, 58u8) {
            Ok(f52(v1, Adt3::C9, {
                let a219 = v2;
                let a220 = 2u64;
                nat_add(a219, a220)?
            }))
        } else {
            Ok(f52(v1, Adt3::C7, {
                let a221 = v2;
                let a222 = 1u64;
                nat_add(a221, a222)?
            }))
        }
    }
}

pub fn f54(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h223, _, _) => {
            h223
        }
    };
    let v4: Adt7 = f62(v0, v1, v3)?;
    let m225 = match v4.clone() {
        Adt7::C0(_, _, _, h224) => {
            h224
        }
    };
    match m225 {
        None => {
            if match v4.clone() {
                Adt7::C0(_, _, h227, _) => {
                    h227
                }
            } {
                Ok(f52(v2, Adt3::C11(f63(match v4.clone() {
                    Adt7::C0(_, h228, _, _) => {
                        h228
                    }
                })), match v4 {
                    Adt7::C0(h229, _, _, _) => {
                        h229
                    }
                }))
            } else {
                Ok(Some(Adt4::C0(v3, match v2 {
                    Adt4::C0(_, h230, _) => {
                        h230
                    }
                }, Some(f64(v3, f65(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt4::C0(v3, match v2 {
                Adt4::C0(_, h226, _) => {
                    h226
                }
            }, Some(v5))))
        }
    }
}

pub fn f55(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h231, _, _) => {
            h231
        }
    };
    let v4: u64 = f66(v0.clone(), v1, v3)?;
    Ok(f52(v2, Adt3::C12(f67(v0, v3, {
        let a232 = v4;
        let a233 = v3;
        nat_sub(a232, a233)
    })), v4))
}

pub fn f56(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let m235 = match v2.clone() {
        Adt5::C0(_, _, _, _, _, h234) => {
            h234
        }
    };
    match m235 {
        None => {
            let a237 = match v2.clone() {
                Adt5::C0(_, _, h236, _, _, _) => {
                    h236
                }
            };
            let a238 = 9u64;
            if nat_eq(a237, a238) {
                Ok(None::<Adt5>)
            } else {
                Ok(Some(f68(v0, v1, v2)?))
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f57(v0: Fn11, p243: u64, p244: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m242 = (p243, p244);
    {
        let mut m242 = m242;
        loop {
            let step = {
                let (v1, v2) = m242;
                let m239 = v1;
                if m239 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m239 - 1;
                    let m241 = {
                        let c240: Fn11 = v0.clone();
                        c240.apply(v2.clone())?
                    };
                    match m241 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f69(v0)
}

pub fn f59(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m245 = f61(v0, v1);
    match m245 {
        None => {
            false
        }
        Some(v4) => {
            f70(v4, v2, v3)
        }
    }
}

pub fn f60(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m246 = f61(v0, v1);
    match m246 {
        None => {
            false
        }
        Some(v3) => {
            let a247 = v3;
            let a248 = v2;
            equal(a247, a248)
        }
    }
}

pub fn f61(v0: Bytes, v1: u64) -> Option<u8> {
    let a249 = v0;
    let a250 = v1;
    index_bytes(a249, a250)
}

pub fn f62(v0: Bytes, v1: u64, v2: u64) -> R<Adt7> {
    let (h251, _) = f71(Fn12::F72(v1, v0), {
        let a254 = {
            let a252 = v1;
            let a253 = v2;
            nat_sub(a252, a253)
        };
        let a255 = 1u64;
        nat_add(a254, a255)?
    }, Adt7::C0({
        let a256 = v2;
        let a257 = 1u64;
        nat_add(a256, a257)?
    }, List::<Bytes>::nil(), false, None::<Adt1>))?;
    Ok(h251)
}

pub fn f63(v0: List<Bytes>) -> Bytes {
    f73(f74(v0))
}

pub fn f64(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f65(v0: Str) -> Bytes {
    let a258 = v0;
    utf8_encode(a258)
}

pub fn f66(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h259, _) = f25(Fn5::F75(v1, v0), {
        let a260 = v1;
        let a261 = v2;
        nat_sub(a260, a261)
    }, v2)?;
    Ok(h259)
}

pub fn f67(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m265 = {
        let a262 = v0;
        let a263 = v1;
        let a264 = v2;
        slice_bytes(a262, a263, a264)
    };
    match m265 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f68(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(_, _, h266, _, _, _) => {
            h266
        }
    };
    let a267 = v3;
    let a268 = 0u64;
    if nat_eq(a267, a268) {
        f76(v0, v1, v2)
    } else {
        let a269 = v3;
        let a270 = 1u64;
        if nat_eq(a269, a270) {
            f77(v0, v2)
        } else {
            let a271 = v3;
            let a272 = 2u64;
            if nat_eq(a271, a272) {
                f78(v0, v1, v2)
            } else {
                let a273 = v3;
                let a274 = 3u64;
                if nat_eq(a273, a274) {
                    f79(v0, v2)
                } else {
                    let a275 = v3;
                    let a276 = 4u64;
                    if nat_eq(a275, a276) {
                        f80(v0, v1, v2, 3u64, f33(Str::lit("expected `,` or `]`")))
                    } else {
                        f80(v0, v1, v2, 1u64, f33(Str::lit("expected `,` or `}`")))
                    }
                }
            }
        }
    }
}

pub fn f69(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h277, _) = v0;
        h277
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h278) = v0;
        h278
    };
    let m279 = v1;
    match m279.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m280 = v4;
            match m280.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f81(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f70(v0: u8, v1: u8, v2: u8) -> bool {
    let m283 = {
        let a281 = v0;
        let a282 = v1;
        fixed_u8::checked_sub(a281, a282)
    };
    match m283 {
        None => {
            false
        }
        Some(_) => {
            let m286 = {
                let a284 = v2;
                let a285 = v0;
                fixed_u8::checked_sub(a284, a285)
            };
            match m286 {
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

pub fn f71(v0: Fn12, p291: u64, p292: Adt7) -> R<(Adt7, bool)> {
    let m290 = (p291, p292);
    {
        let mut m290 = m290;
        loop {
            let step = {
                let (v1, v2) = m290;
                let m287 = v1;
                if m287 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m287 - 1;
                    let m289 = {
                        let c288: Fn12 = v0.clone();
                        c288.apply(v2.clone())?
                    };
                    match m289 {
                        None => {
                            Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt7), R<(Adt7, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m290 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: u64, v1: Bytes, v2: Adt7) -> R<Option<Adt7>> {
    f82(v1, v0, v2)
}

pub fn f73(v0: List<Bytes>) -> Bytes {
    let m295 = {
        let (h293, _) = f83(Fn13::F84, {
            let a294 = v0.clone();
            length_list(a294)
        }, v0);
        h293
    };
    match m295.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f74(v0: List<Bytes>) -> List<Bytes> {
    f85(Fn14::F86, List::<Bytes>::nil(), v0)
}

pub fn f75(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f87(v1, v0, v2)
}

pub fn f76(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h296, _, _, _, _, _) => {
            h296
        }
    };
    let a298 = 256u64;
    let a299 = match v2.clone() {
        Adt5::C0(_, h297, _, _, _, _) => {
            h297
        }
    };
    if nat_lt(a298, a299) {
        Ok(f88(v2, f32(v0, v1, v3), f33(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v4: u64 = f89(v0.clone(), v3);
        let a300 = v4;
        let a301 = 99u64;
        if nat_eq(a300, a301) {
            Ok(f88(v2, v1, f33(Str::lit("expected a value, found the end of the text"))))
        } else {
            let a302 = v4;
            let a303 = 11u64;
            if nat_eq(a302, a303) {
                Ok(f90(v2, {
                    let a304 = v3;
                    let a305 = 1u64;
                    nat_add(a304, a305)?
                }, Adt0::C2(f91(v0, v3))))
            } else {
                let a306 = v4;
                let a307 = 2u64;
                if nat_eq(a306, a307) {
                    f92(v2, {
                        let a308 = v3;
                        let a309 = 1u64;
                        nat_add(a308, a309)?
                    }, Adt6::C0(List::<Adt0>::nil()), 1u64)
                } else {
                    let a310 = v4;
                    let a311 = 0u64;
                    if nat_eq(a310, a311) {
                        f92(v2, {
                            let a312 = v3;
                            let a313 = 1u64;
                            nat_add(a312, a313)?
                        }, Adt6::C1(Bytes::lit(&[]), 0u64, List::<(Bytes, Adt0)>::nil()), 2u64)
                    } else {
                        let a314 = v4;
                        let a315 = 12u64;
                        if nat_eq(a314, a315) {
                            f93(v0.clone(), v1, v2, f32(v0, v1, v3))
                        } else {
                            Ok(f88(v2, f32(v0.clone(), v1, v3), f94(f33(Str::lit("expected a value, found ")), f95(v0, v3)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f77(v0: List<Adt2>, v1: Adt5) -> R<Adt5> {
    let a317 = f89(v0, match v1.clone() {
        Adt5::C0(h316, _, _, _, _, _) => {
            h316
        }
    });
    let a318 = 3u64;
    if nat_eq(a317, a318) {
        f96(v1)
    } else {
        Ok(Adt5::C0(match v1.clone() {
            Adt5::C0(h319, _, _, _, _, _) => {
                h319
            }
        }, match v1.clone() {
            Adt5::C0(_, h320, _, _, _, _) => {
                h320
            }
        }, 0u64, match v1 {
            Adt5::C0(_, _, _, h321, _, _) => {
                h321
            }
        }, Adt0::C5, None::<Adt1>))
    }
}

pub fn f78(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let m323 = match v2.clone() {
        Adt5::C0(_, _, _, h322, _, _) => {
            h322
        }
    };
    match m323.uncons() {
        None => {
            Ok(f88(v2, 0u64, f33(Str::lit("no object is open"))))
        }
        Some((v3, v4)) => {
            let m324 = v3;
            match m324 {
                Adt6::C0(_) => {
                    Ok(f88(v2, 0u64, f33(Str::lit("no object is open"))))
                }
                Adt6::C1(_, _, v8) => {
                    let a326 = f89(v0.clone(), match v2.clone() {
                        Adt5::C0(h325, _, _, _, _, _) => {
                            h325
                        }
                    });
                    let a327 = 1u64;
                    if nat_eq(a326, a327) {
                        f96(v2)
                    } else {
                        f97(v0, v1, v2, v8, v4)
                    }
                }
            }
        }
    }
}

pub fn f79(v0: List<Adt2>, v1: Adt5) -> R<Adt5> {
    let m329 = match v1.clone() {
        Adt5::C0(_, _, _, h328, _, _) => {
            h328
        }
    };
    match m329.uncons() {
        None => {
            Ok(Adt5::C0(match v1.clone() {
                Adt5::C0(h330, _, _, _, _, _) => {
                    h330
                }
            }, match v1.clone() {
                Adt5::C0(_, h331, _, _, _, _) => {
                    h331
                }
            }, 9u64, List::<Adt6>::nil(), match v1 {
                Adt5::C0(_, _, _, _, h332, _) => {
                    h332
                }
            }, None::<Adt1>))
        }
        Some((v2, v3)) => {
            let m333 = v2;
            match m333 {
                Adt6::C0(v4) => {
                    f98(v0, v1.clone(), List::cons(Adt6::C0(List::cons(match v1 {
                        Adt5::C0(_, _, _, _, h334, _) => {
                            h334
                        }
                    }, v4)), v3), 1u64, 4u64)
                }
                Adt6::C1(v5, v6, v7) => {
                    if f3(v7.clone(), v5.clone()) {
                        Ok(f88(v1, v6, f94(f94(f33(Str::lit("the member `")), v5), f33(Str::lit("` is written twice")))))
                    } else {
                        f98(v0, v1.clone(), List::cons(Adt6::C1(Bytes::lit(&[]), 0u64, List::cons((v5, match v1 {
                            Adt5::C0(_, _, _, _, h335, _) => {
                                h335
                            }
                        }), v7)), v3), 2u64, 5u64)
                    }
                }
            }
        }
    }
}

pub fn f80(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: Bytes) -> R<Adt5> {
    let a337 = f89(v0.clone(), match v2.clone() {
        Adt5::C0(h336, _, _, _, _, _) => {
            h336
        }
    });
    let a338 = v3;
    if nat_eq(a337, a338) {
        f96(v2)
    } else {
        Ok(f88(v2.clone(), f32(v0, v1, match v2 {
            Adt5::C0(h339, _, _, _, _, _) => {
                h339
            }
        }), v4))
    }
}

pub fn f81(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a344 = {
            let a342 = {
                let a340 = v0.clone();
                length_list(a340)
            };
            let a343 = {
                let a341 = v1.clone();
                length_list(a341)
            };
            nat_add(a342, a343)?
        };
        let a345 = 1u64;
        nat_add(a344, a345)?
    };
    Ok(f19({
        let (_, h346) = {
            let (_, h347) = {
                let (h348, _) = f99(Fn15::F100, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h348
            };
            h347
        };
        h346
    }))
}

pub fn f82(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let m350 = match v2.clone() {
        Adt7::C0(_, _, _, h349) => {
            h349
        }
    };
    match m350 {
        None => {
            if match v2.clone() {
                Adt7::C0(_, _, h351, _) => {
                    h351
                }
            } {
                Ok(None::<Adt7>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt7::C0(h352, _, _, _) => {
                        h352
                    }
                };
                let a353 = v4;
                let a354 = v1;
                if nat_lt(a353, a354) {
                    if f51(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt7::C0({
                            let a355 = v4;
                            let a356 = 1u64;
                            nat_add(a355, a356)?
                        }, match v2 {
                            Adt7::C0(_, h357, _, _) => {
                                h357
                            }
                        }, true, None::<Adt1>)))
                    } else {
                        if f51(v0.clone(), v4, 92u8) {
                            Ok(Some(f101(v0, v1, v2)?))
                        } else {
                            let a358 = f102(v0.clone(), v4)?;
                            let a359 = 32u64;
                            if nat_lt(a358, a359) {
                                Ok(Some(f103(v2, v4, f65(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f104(v0.clone(), v1, v4)?;
                                Ok(Some(f105(v2, v5, f67(v0, v4, {
                                    let a360 = v5;
                                    let a361 = v4;
                                    nat_sub(a360, a361)
                                }))))
                            }
                        }
                    }
                } else {
                    Ok(None::<Adt7>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt7>)
        }
    }
}

pub fn f83(v0: Fn13, p366: u64, p367: List<Bytes>) -> (List<Bytes>, bool) {
    let m365 = (p366, p367);
    {
        let mut m365 = m365;
        loop {
            let step = {
                let (v1, v2) = m365;
                let m362 = v1;
                if m362 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m362 - 1;
                    let m364 = {
                        let c363: Fn13 = v0.clone();
                        c363.apply(v2.clone())
                    };
                    match m364 {
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
                Ok(next) => { m365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: List<Bytes>) -> Option<List<Bytes>> {
    f106(v0)
}

pub fn f85(v0: Fn14, p371: List<Bytes>, p372: List<Bytes>) -> List<Bytes> {
    let m370 = (p371, p372);
    {
        let mut m370 = m370;
        loop {
            let step = {
                let (v1, v2) = m370;
                let m368 = v2;
                match m368.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c369: Fn14 = v0.clone();
                            c369.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f87(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a373 = v2;
    let a374 = v1;
    if nat_lt(a373, a374) {
        if f107(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a375 = v2;
                let a376 = 1u64;
                nat_add(a375, a376)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f88(v0: Adt5, v1: u64, v2: Bytes) -> Adt5 {
    Adt5::C0(match v0.clone() {
        Adt5::C0(h377, _, _, _, _, _) => {
            h377
        }
    }, match v0.clone() {
        Adt5::C0(_, h378, _, _, _, _) => {
            h378
        }
    }, 9u64, match v0.clone() {
        Adt5::C0(_, _, _, h379, _, _) => {
            h379
        }
    }, match v0 {
        Adt5::C0(_, _, _, _, h380, _) => {
            h380
        }
    }, Some(f31(v1, v2)))
}

pub fn f89(v0: List<Adt2>, v1: u64) -> u64 {
    let m381 = f48(v0, v1);
    match m381 {
        None => {
            99u64
        }
        Some(v2) => {
            f108(match v2 {
                Adt2::C0(h382, _, _) => {
                    h382
                }
            })
        }
    }
}

pub fn f90(v0: Adt5, v1: u64, v2: Adt0) -> Adt5 {
    Adt5::C0(v1, match v0.clone() {
        Adt5::C0(_, h383, _, _, _, _) => {
            h383
        }
    }, 3u64, match v0 {
        Adt5::C0(_, _, _, h384, _, _) => {
            h384
        }
    }, v2, None::<Adt1>)
}

pub fn f91(v0: List<Adt2>, v1: u64) -> Bytes {
    let m385 = f48(v0, v1);
    match m385 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f109(match v2 {
                Adt2::C0(h386, _, _) => {
                    h386
                }
            })
        }
    }
}

pub fn f92(v0: Adt5, v1: u64, v2: Adt6, v3: u64) -> R<Adt5> {
    Ok(Adt5::C0(v1, {
        let a388 = match v0.clone() {
            Adt5::C0(_, h387, _, _, _, _) => {
                h387
            }
        };
        let a389 = 1u64;
        nat_add(a388, a389)?
    }, v3, List::cons(v2, match v0 {
        Adt5::C0(_, _, _, h390, _, _) => {
            h390
        }
    }), Adt0::C5, None::<Adt1>))
}

pub fn f93(v0: List<Adt2>, _: u64, v2: Adt5, v3: u64) -> R<Adt5> {
    let v4: u64 = match v2.clone() {
        Adt5::C0(h391, _, _, _, _, _) => {
            h391
        }
    };
    let v5: Bytes = f91(v0.clone(), v4);
    let a392 = v5.clone();
    let a393 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a392, a393) {
        Ok(f90(v2, {
            let a394 = v4;
            let a395 = 1u64;
            nat_add(a394, a395)?
        }, Adt0::C0(true)))
    } else {
        let a396 = v5.clone();
        let a397 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        if equal(a396, a397) {
            Ok(f90(v2, {
                let a398 = v4;
                let a399 = 1u64;
                nat_add(a398, a399)?
            }, Adt0::C0(false)))
        } else {
            if f110(v5.clone())? {
                let m400 = f111(v5.clone())?;
                match m400 {
                    None => {
                        Ok(f88(v2, v3, f94(f94(f33(Str::lit("`")), v5), f33(Str::lit("` does not fit a 64-bit integer")))))
                    }
                    Some(v6) => {
                        Ok(f90(v2, {
                            let a401 = v4;
                            let a402 = 1u64;
                            nat_add(a401, a402)?
                        }, v6))
                    }
                }
            } else {
                if f112(v5.clone())? {
                    let a405 = f89(v0, {
                        let a403 = v4;
                        let a404 = 1u64;
                        nat_add(a403, a404)?
                    });
                    let a406 = 0u64;
                    if nat_eq(a405, a406) {
                        f92(v2, {
                            let a407 = v4;
                            let a408 = 2u64;
                            nat_add(a407, a408)?
                        }, Adt6::C1(Bytes::lit(&[]), 0u64, List::cons((f33(Str::lit("kind")), Adt0::C2(v5)), List::<(Bytes, Adt0)>::nil())), 2u64)
                    } else {
                        Ok(f90(v2, {
                            let a409 = v4;
                            let a410 = 1u64;
                            nat_add(a409, a410)?
                        }, Adt0::C2(v5)))
                    }
                } else {
                    Ok(f88(v2, v3, f94(f94(f33(Str::lit("`")), v5), f33(Str::lit("` is not a name; quote it to write a string")))))
                }
            }
        }
    }
}

pub fn f94(v0: Bytes, v1: Bytes) -> Bytes {
    let a411 = v0;
    let a412 = v1;
    append_bytes(a411, a412)
}

pub fn f95(v0: List<Adt2>, v1: u64) -> R<Bytes> {
    let m413 = f48(v0, v1);
    match m413 {
        None => {
            Ok(f33(Str::lit("the end of the text")))
        }
        Some(v2) => {
            f113(match v2 {
                Adt2::C0(h414, _, _) => {
                    h414
                }
            })
        }
    }
}

pub fn f96(v0: Adt5) -> R<Adt5> {
    let m416 = match v0.clone() {
        Adt5::C0(_, _, _, h415, _, _) => {
            h415
        }
    };
    match m416.uncons() {
        None => {
            Ok(f88(v0, 0u64, f33(Str::lit("nothing to close"))))
        }
        Some((v1, v2)) => {
            let m417 = v1;
            match m417 {
                Adt6::C0(v3) => {
                    Ok(Adt5::C0({
                        let a419 = match v0.clone() {
                            Adt5::C0(h418, _, _, _, _, _) => {
                                h418
                            }
                        };
                        let a420 = 1u64;
                        nat_add(a419, a420)?
                    }, {
                        let a422 = match v0 {
                            Adt5::C0(_, h421, _, _, _, _) => {
                                h421
                            }
                        };
                        let a423 = 1u64;
                        nat_sub(a422, a423)
                    }, 3u64, v2, Adt0::C3(std::rc::Rc::new(f114(v3))), None::<Adt1>))
                }
                Adt6::C1(_, _, v6) => {
                    Ok(Adt5::C0({
                        let a425 = match v0.clone() {
                            Adt5::C0(h424, _, _, _, _, _) => {
                                h424
                            }
                        };
                        let a426 = 1u64;
                        nat_add(a425, a426)?
                    }, {
                        let a428 = match v0 {
                            Adt5::C0(_, h427, _, _, _, _) => {
                                h427
                            }
                        };
                        let a429 = 1u64;
                        nat_sub(a428, a429)
                    }, 3u64, v2, Adt0::C4(std::rc::Rc::new(f7(f8(v6)?))), None::<Adt1>))
                }
            }
        }
    }
}

pub fn f97(v0: List<Adt2>, v1: u64, v2: Adt5, v3: List<(Bytes, Adt0)>, v4: List<Adt6>) -> R<Adt5> {
    let v5: u64 = match v2.clone() {
        Adt5::C0(h430, _, _, _, _, _) => {
            h430
        }
    };
    let v6: u64 = f89(v0.clone(), v5);
    let v7: Bytes = f91(v0.clone(), v5);
    let a431 = v6;
    let a432 = 11u64;
    if if nat_eq(a431, a432) {
        true
    } else {
        let a433 = v6;
        let a434 = 12u64;
        if nat_eq(a433, a434) {
            f112(v7.clone())?
        } else {
            false
        }
    } {
        let a437 = f89(v0.clone(), {
            let a435 = v5;
            let a436 = 1u64;
            nat_add(a435, a436)?
        });
        let a438 = 7u64;
        if nat_eq(a437, a438) {
            Ok(Adt5::C0({
                let a439 = v5;
                let a440 = 2u64;
                nat_add(a439, a440)?
            }, match v2 {
                Adt5::C0(_, h441, _, _, _, _) => {
                    h441
                }
            }, 0u64, List::cons(Adt6::C1(v7, f32(v0, v1, v5), v3), v4), Adt0::C5, None::<Adt1>))
        } else {
            Ok(f88(v2, f32(v0, v1, {
                let a442 = v5;
                let a443 = 1u64;
                nat_add(a442, a443)?
            }), f33(Str::lit("expected `:` after a member name"))))
        }
    } else {
        Ok(f88(v2, f32(v0, v1, v5), f33(Str::lit("expected a member name"))))
    }
}

pub fn f98(v0: List<Adt2>, v1: Adt5, v2: List<Adt6>, v3: u64, v4: u64) -> R<Adt5> {
    let a445 = f89(v0, match v1.clone() {
        Adt5::C0(h444, _, _, _, _, _) => {
            h444
        }
    });
    let a446 = 6u64;
    if nat_eq(a445, a446) {
        Ok(Adt5::C0({
            let a448 = match v1.clone() {
                Adt5::C0(h447, _, _, _, _, _) => {
                    h447
                }
            };
            let a449 = 1u64;
            nat_add(a448, a449)?
        }, match v1 {
            Adt5::C0(_, h450, _, _, _, _) => {
                h450
            }
        }, v3, v2, Adt0::C5, None::<Adt1>))
    } else {
        Ok(Adt5::C0(match v1.clone() {
            Adt5::C0(h451, _, _, _, _, _) => {
                h451
            }
        }, match v1 {
            Adt5::C0(_, h452, _, _, _, _) => {
                h452
            }
        }, v4, v2, Adt0::C5, None::<Adt1>))
    }
}

pub fn f99(v0: Fn15, p457: u64, p458: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m456 = (p457, p458);
    {
        let mut m456 = m456;
        loop {
            let step = {
                let (v1, v2) = m456;
                let m453 = v1;
                if m453 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m453 - 1;
                    let m455 = {
                        let c454: Fn15 = v0.clone();
                        c454.apply(v2.clone())
                    };
                    match m455 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f115(v0)
}

pub fn f101(v0: Bytes, v1: u64, v2: Adt7) -> R<Adt7> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h459, _, _, _) => {
            h459
        }
    };
    let m462 = f61(v0.clone(), {
        let a460 = v3;
        let a461 = 1u64;
        nat_add(a460, a461)?
    });
    match m462 {
        None => {
            Ok(Adt7::C0(v1, match v2 {
                Adt7::C0(_, h463, _, _) => {
                    h463
                }
            }, false, None::<Adt1>))
        }
        Some(v4) => {
            let a464 = v4;
            let a465 = 34u8;
            if equal(a464, a465) {
                Ok(f105(v2, {
                    let a466 = v3;
                    let a467 = 2u64;
                    nat_add(a466, a467)?
                }, f65(Str::lit("\""))))
            } else {
                let a468 = v4;
                let a469 = 92u8;
                if equal(a468, a469) {
                    Ok(f105(v2, {
                        let a470 = v3;
                        let a471 = 2u64;
                        nat_add(a470, a471)?
                    }, f65(Str::lit("\\"))))
                } else {
                    let a472 = v4;
                    let a473 = 47u8;
                    if equal(a472, a473) {
                        Ok(f105(v2, {
                            let a474 = v3;
                            let a475 = 2u64;
                            nat_add(a474, a475)?
                        }, f65(Str::lit("/"))))
                    } else {
                        let a476 = v4;
                        let a477 = 98u8;
                        if equal(a476, a477) {
                            Ok(f105(v2, {
                                let a478 = v3;
                                let a479 = 2u64;
                                nat_add(a478, a479)?
                            }, f116(8u64)))
                        } else {
                            let a480 = v4;
                            let a481 = 102u8;
                            if equal(a480, a481) {
                                Ok(f105(v2, {
                                    let a482 = v3;
                                    let a483 = 2u64;
                                    nat_add(a482, a483)?
                                }, f116(12u64)))
                            } else {
                                let a484 = v4;
                                let a485 = 110u8;
                                if equal(a484, a485) {
                                    Ok(f105(v2, {
                                        let a486 = v3;
                                        let a487 = 2u64;
                                        nat_add(a486, a487)?
                                    }, f116(10u64)))
                                } else {
                                    let a488 = v4;
                                    let a489 = 114u8;
                                    if equal(a488, a489) {
                                        Ok(f105(v2, {
                                            let a490 = v3;
                                            let a491 = 2u64;
                                            nat_add(a490, a491)?
                                        }, f116(13u64)))
                                    } else {
                                        let a492 = v4;
                                        let a493 = 116u8;
                                        if equal(a492, a493) {
                                            Ok(f105(v2, {
                                                let a494 = v3;
                                                let a495 = 2u64;
                                                nat_add(a494, a495)?
                                            }, f116(9u64)))
                                        } else {
                                            let a496 = v4;
                                            let a497 = 117u8;
                                            if equal(a496, a497) {
                                                f117(v0, v2)
                                            } else {
                                                Ok(f103(v2, v3, f118(f118(f65(Str::lit("`\\")), f67(v0.clone(), {
                                                    let a498 = v3;
                                                    let a499 = 1u64;
                                                    nat_add(a498, a499)?
                                                }, f119(v0, {
                                                    let a500 = v3;
                                                    let a501 = 1u64;
                                                    nat_add(a500, a501)?
                                                })?)), f65(Str::lit("` is not an escape")))))
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

pub fn f102(v0: Bytes, v1: u64) -> R<u64> {
    let m502 = f61(v0, v1);
    match m502 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a547 = {
                let a523 = {
                    let a511 = {
                        let a505 = {
                            let a503 = v3;
                            let a504 = 128u8;
                            fixed_u8::bit_and(a503, a504)
                        };
                        let a506 = 128u8;
                        if equal(a505, a506) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a512 = {
                        let a509 = {
                            let a507 = v3;
                            let a508 = 64u8;
                            fixed_u8::bit_and(a507, a508)
                        };
                        let a510 = 64u8;
                        if equal(a509, a510) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a511, a512)?
                };
                let a524 = {
                    let a521 = {
                        let a515 = {
                            let a513 = v3;
                            let a514 = 32u8;
                            fixed_u8::bit_and(a513, a514)
                        };
                        let a516 = 32u8;
                        if equal(a515, a516) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a522 = {
                        let a519 = {
                            let a517 = v3;
                            let a518 = 16u8;
                            fixed_u8::bit_and(a517, a518)
                        };
                        let a520 = 16u8;
                        if equal(a519, a520) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a521, a522)?
                };
                nat_add(a523, a524)?
            };
            let a548 = {
                let a545 = {
                    let a533 = {
                        let a527 = {
                            let a525 = v3;
                            let a526 = 8u8;
                            fixed_u8::bit_and(a525, a526)
                        };
                        let a528 = 8u8;
                        if equal(a527, a528) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a534 = {
                        let a531 = {
                            let a529 = v3;
                            let a530 = 4u8;
                            fixed_u8::bit_and(a529, a530)
                        };
                        let a532 = 4u8;
                        if equal(a531, a532) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a533, a534)?
                };
                let a546 = {
                    let a543 = {
                        let a537 = {
                            let a535 = v3;
                            let a536 = 2u8;
                            fixed_u8::bit_and(a535, a536)
                        };
                        let a538 = 2u8;
                        if equal(a537, a538) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a544 = {
                        let a541 = {
                            let a539 = v3;
                            let a540 = 1u8;
                            fixed_u8::bit_and(a539, a540)
                        };
                        let a542 = 1u8;
                        if equal(a541, a542) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a543, a544)?
                };
                nat_add(a545, a546)?
            };
            nat_add(a547, a548)
        }
    }
}

pub fn f103(v0: Adt7, v1: u64, v2: Bytes) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h549, _, _, _) => {
            h549
        }
    }, match v0 {
        Adt7::C0(_, h550, _, _) => {
            h550
        }
    }, false, Some(f64(v1, v2)))
}

pub fn f104(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h551, _) = f25(Fn5::F120(v1, v0), {
        let a552 = v1;
        let a553 = v2;
        nat_sub(a552, a553)
    }, v2)?;
    Ok(h551)
}

pub fn f105(v0: Adt7, v1: u64, v2: Bytes) -> Adt7 {
    Adt7::C0(v1, List::cons(v2, match v0 {
        Adt7::C0(_, h554, _, _) => {
            h554
        }
    }), false, None::<Adt1>)
}

pub fn f106(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m555 = v0.clone();
    match m555.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m556 = v2;
            match m556.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f121(Fn16::F122, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m558 = {
                        let (h557, _) = v5;
                        h557
                    };
                    match m558 {
                        None => {
                            Some(f74({
                                let (_, h560) = v5;
                                h560
                            }))
                        }
                        Some(v6) => {
                            Some(f74(List::cons(v6, {
                                let (_, h559) = v5;
                                h559
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f107(v0: Bytes, v1: u64) -> R<bool> {
    let a561 = 0u64;
    let a562 = f50(v0.clone(), v1)?;
    if nat_lt(a561, a562) {
        Ok(true)
    } else {
        Ok(f123(v0, v1))
    }
}

pub fn f108(v0: Adt3) -> u64 {
    let m563 = v0;
    match m563 {
        Adt3::C0 => {
            0u64
        }
        Adt3::C1 => {
            1u64
        }
        Adt3::C2 => {
            2u64
        }
        Adt3::C3 => {
            3u64
        }
        Adt3::C4 => {
            4u64
        }
        Adt3::C5 => {
            5u64
        }
        Adt3::C6 => {
            6u64
        }
        Adt3::C7 => {
            7u64
        }
        Adt3::C8 => {
            8u64
        }
        Adt3::C9 => {
            9u64
        }
        Adt3::C10 => {
            10u64
        }
        Adt3::C11(_) => {
            11u64
        }
        Adt3::C12(_) => {
            12u64
        }
    }
}

pub fn f109(v0: Adt3) -> Bytes {
    let m564 = v0;
    match m564 {
        Adt3::C0 => {
            Bytes::lit(&[])
        }
        Adt3::C1 => {
            Bytes::lit(&[])
        }
        Adt3::C2 => {
            Bytes::lit(&[])
        }
        Adt3::C3 => {
            Bytes::lit(&[])
        }
        Adt3::C4 => {
            Bytes::lit(&[])
        }
        Adt3::C5 => {
            Bytes::lit(&[])
        }
        Adt3::C6 => {
            Bytes::lit(&[])
        }
        Adt3::C7 => {
            Bytes::lit(&[])
        }
        Adt3::C8 => {
            Bytes::lit(&[])
        }
        Adt3::C9 => {
            Bytes::lit(&[])
        }
        Adt3::C10 => {
            Bytes::lit(&[])
        }
        Adt3::C11(v1) => {
            v1
        }
        Adt3::C12(v2) => {
            v2
        }
    }
}

pub fn f110(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a565 = f124(v0.clone(), 0u64)?;
        let a566 = 45u64;
        if nat_eq(a565, a566) {
            1u64
        } else {
            0u64
        }
    };
    let a568 = v1;
    let a569 = {
        let a567 = v0.clone();
        length_bytes(a567)
    };
    if nat_lt(a568, a569) {
        let a571 = f125(v0.clone(), v1)?;
        let a572 = {
            let a570 = v0;
            length_bytes(a570)
        };
        Ok(nat_eq(a571, a572))
    } else {
        Ok(false)
    }
}

pub fn f111(v0: Bytes) -> R<Option<Adt0>> {
    let v1: bool = {
        let a573 = f124(v0.clone(), 0u64)?;
        let a574 = 45u64;
        nat_eq(a573, a574)
    };
    let v2: u64 = if v1 {
        1u64
    } else {
        0u64
    };
    let v3: u64 = f126(v0.clone(), v2)?;
    let v4: Bytes = f67(v0.clone(), v3, {
        let a576 = {
            let a575 = v0;
            length_bytes(a575)
        };
        let a577 = v3;
        nat_sub(a576, a577)
    });
    let a579 = {
        let a578 = v4.clone();
        length_bytes(a578)
    };
    let a580 = 0u64;
    if nat_eq(a579, a580) {
        Ok(Some(Adt0::C1(false, 0u64)))
    } else {
        let a582 = 19u64;
        let a583 = {
            let a581 = v4.clone();
            length_bytes(a581)
        };
        if nat_lt(a582, a583) {
            Ok(None::<Adt0>)
        } else {
            let m584 = f127(v4)?;
            match m584 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v5) => {
                    if f128(v1, v5) {
                        Ok(Some(Adt0::C1(v1, v5)))
                    } else {
                        Ok(None::<Adt0>)
                    }
                }
            }
        }
    }
}

pub fn f112(v0: Bytes) -> R<bool> {
    let a586 = {
        let a585 = v0.clone();
        length_bytes(a585)
    };
    let a587 = 0u64;
    if nat_eq(a586, a587) {
        Ok(false)
    } else {
        if f129(f124(v0.clone(), 0u64)?) {
            let a589 = f130(v0.clone(), 1u64)?;
            let a590 = {
                let a588 = v0.clone();
                length_bytes(a588)
            };
            if nat_eq(a589, a590) {
                let a591 = f131(v0);
                Ok(bool_not(a591))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f113(v0: Adt3) -> R<Bytes> {
    let m592 = v0;
    match m592 {
        Adt3::C0 => {
            Ok(f33(Str::lit("`{`")))
        }
        Adt3::C1 => {
            Ok(f33(Str::lit("`}`")))
        }
        Adt3::C2 => {
            Ok(f33(Str::lit("`[`")))
        }
        Adt3::C3 => {
            Ok(f33(Str::lit("`]`")))
        }
        Adt3::C4 => {
            Ok(f33(Str::lit("`(`")))
        }
        Adt3::C5 => {
            Ok(f33(Str::lit("`)`")))
        }
        Adt3::C6 => {
            Ok(f33(Str::lit("`,`")))
        }
        Adt3::C7 => {
            Ok(f33(Str::lit("`:`")))
        }
        Adt3::C8 => {
            Ok(f33(Str::lit("`:=`")))
        }
        Adt3::C9 => {
            Ok(f33(Str::lit("`::`")))
        }
        Adt3::C10 => {
            Ok(f33(Str::lit("`;`")))
        }
        Adt3::C11(v1) => {
            Ok(f94(f33(Str::lit("the string ")), f132(v1)?))
        }
        Adt3::C12(v2) => {
            Ok(f94(f94(f33(Str::lit("`")), v2), f33(Str::lit("`"))))
        }
    }
}

pub fn f114(v0: List<Adt0>) -> Adt0 {
    f133(Fn17::F134, Adt0::C5, v0)
}

pub fn f115(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h593, _) = v0.clone();
        h593
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h594, _) = {
            let (_, h595) = v0.clone();
            h595
        };
        h594
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h596) = {
            let (_, h597) = v0;
            h597
        };
        h596
    };
    let m598 = v1.clone();
    match m598.uncons() {
        None => {
            let m599 = v2;
            match m599.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m600 = v2.clone();
            match m600.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f135({
                        let (h601, _) = v8.clone();
                        h601
                    }, {
                        let (h602, _) = v6.clone();
                        h602
                    }) {
                        Some((v1, (v9, List::cons(v8, v3))))
                    } else {
                        Some((v7, (v2, List::cons(v6, v3))))
                    }
                }
            }
        }
    }
}

pub fn f116(v0: u64) -> Bytes {
    f67(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f117(v0: Bytes, v1: Adt7) -> R<Adt7> {
    let v2: u64 = match v1.clone() {
        Adt7::C0(h603, _, _, _) => {
            h603
        }
    };
    let m606 = f136(v0, {
        let a604 = v2;
        let a605 = 2u64;
        nat_add(a604, a605)?
    })?;
    match m606 {
        None => {
            Ok(f103(v1, v2, f65(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a607 = 55296u64;
            let a608 = v3;
            if nat_le(a607, a608) {
                let a609 = v3;
                let a610 = 57343u64;
                if nat_le(a609, a610) {
                    Ok(f103(v1, v2, f118(f118(f65(Str::lit("`\\u")), f137(v3)), f65(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f105(v1, {
                        let a611 = v2;
                        let a612 = 6u64;
                        nat_add(a611, a612)?
                    }, f138(v3)?))
                }
            } else {
                Ok(f105(v1, {
                    let a613 = v2;
                    let a614 = 6u64;
                    nat_add(a613, a614)?
                }, f138(v3)?))
            }
        }
    }
}

pub fn f118(v0: Bytes, v1: Bytes) -> Bytes {
    let a615 = v0;
    let a616 = v1;
    append_bytes(a615, a616)
}

pub fn f119(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f102(v0, v1)?;
    let a617 = v2;
    let a618 = 128u64;
    if nat_lt(a617, a618) {
        Ok(1u64)
    } else {
        let a619 = v2;
        let a620 = 224u64;
        if nat_lt(a619, a620) {
            Ok(2u64)
        } else {
            let a621 = v2;
            let a622 = 240u64;
            if nat_lt(a621, a622) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f120(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f139(v1, v0, v2)
}

pub fn f121(v0: Fn16, p626: (Option<Bytes>, List<Bytes>), p627: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m625 = (p626, p627);
    {
        let mut m625 = m625;
        loop {
            let step = {
                let (v1, v2) = m625;
                let m623 = v2;
                match m623.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c624: Fn16 = v0.clone();
                            c624.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f140(v0, v1)
}

pub fn f123(v0: Bytes, v1: u64) -> bool {
    if f51(v0.clone(), v1, 123u8) {
        true
    } else {
        if f51(v0.clone(), v1, 125u8) {
            true
        } else {
            if f51(v0.clone(), v1, 91u8) {
                true
            } else {
                if f51(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f51(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f51(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f51(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f51(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f51(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f51(v0, v1, 34u8)
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

pub fn f124(v0: Bytes, v1: u64) -> R<u64> {
    let m628 = f61(v0, v1);
    match m628 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a673 = {
                let a649 = {
                    let a637 = {
                        let a631 = {
                            let a629 = v3;
                            let a630 = 128u8;
                            fixed_u8::bit_and(a629, a630)
                        };
                        let a632 = 128u8;
                        if equal(a631, a632) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a638 = {
                        let a635 = {
                            let a633 = v3;
                            let a634 = 64u8;
                            fixed_u8::bit_and(a633, a634)
                        };
                        let a636 = 64u8;
                        if equal(a635, a636) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a637, a638)?
                };
                let a650 = {
                    let a647 = {
                        let a641 = {
                            let a639 = v3;
                            let a640 = 32u8;
                            fixed_u8::bit_and(a639, a640)
                        };
                        let a642 = 32u8;
                        if equal(a641, a642) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a648 = {
                        let a645 = {
                            let a643 = v3;
                            let a644 = 16u8;
                            fixed_u8::bit_and(a643, a644)
                        };
                        let a646 = 16u8;
                        if equal(a645, a646) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a647, a648)?
                };
                nat_add(a649, a650)?
            };
            let a674 = {
                let a671 = {
                    let a659 = {
                        let a653 = {
                            let a651 = v3;
                            let a652 = 8u8;
                            fixed_u8::bit_and(a651, a652)
                        };
                        let a654 = 8u8;
                        if equal(a653, a654) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a660 = {
                        let a657 = {
                            let a655 = v3;
                            let a656 = 4u8;
                            fixed_u8::bit_and(a655, a656)
                        };
                        let a658 = 4u8;
                        if equal(a657, a658) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a659, a660)?
                };
                let a672 = {
                    let a669 = {
                        let a663 = {
                            let a661 = v3;
                            let a662 = 2u8;
                            fixed_u8::bit_and(a661, a662)
                        };
                        let a664 = 2u8;
                        if equal(a663, a664) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a670 = {
                        let a667 = {
                            let a665 = v3;
                            let a666 = 1u8;
                            fixed_u8::bit_and(a665, a666)
                        };
                        let a668 = 1u8;
                        if equal(a667, a668) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a669, a670)?
                };
                nat_add(a671, a672)?
            };
            nat_add(a673, a674)
        }
    }
}

pub fn f125(v0: Bytes, v1: u64) -> R<u64> {
    let (h675, _) = f25(Fn5::F141(v0.clone()), {
        let a677 = {
            let a676 = v0;
            length_bytes(a676)
        };
        let a678 = 1u64;
        nat_add(a677, a678)?
    }, v1)?;
    Ok(h675)
}

pub fn f126(v0: Bytes, v1: u64) -> R<u64> {
    let (h679, _) = f25(Fn5::F142(v0.clone()), {
        let a681 = {
            let a680 = v0;
            length_bytes(a680)
        };
        let a682 = 1u64;
        nat_add(a681, a682)?
    }, v1)?;
    Ok(h679)
}

pub fn f127(v0: Bytes) -> R<Option<u64>> {
    if f143(v0.clone())? {
        Ok(Some({
            let (_, h683) = {
                let (h684, _) = f144(Fn18::F145(v0.clone()), {
                    let a685 = v0;
                    length_bytes(a685)
                }, (0u64, 0u64))?;
                h684
            };
            h683
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f128(v0: bool, v1: u64) -> bool {
    if v0 {
        let a686 = 0u64;
        let a687 = v1;
        if nat_lt(a686, a687) {
            let a688 = v1;
            let a689 = 9223372036854775808u64;
            nat_le(a688, a689)
        } else {
            false
        }
    } else {
        let a690 = v1;
        let a691 = 9223372036854775807u64;
        nat_le(a690, a691)
    }
}

pub fn f129(v0: u64) -> bool {
    let a692 = 64u64;
    let a693 = v0;
    if if nat_lt(a692, a693) {
        let a694 = v0;
        let a695 = 91u64;
        nat_lt(a694, a695)
    } else {
        false
    } {
        true
    } else {
        let a696 = 96u64;
        let a697 = v0;
        if if nat_lt(a696, a697) {
            let a698 = v0;
            let a699 = 123u64;
            nat_lt(a698, a699)
        } else {
            false
        } {
            true
        } else {
            let a700 = v0;
            let a701 = 95u64;
            nat_eq(a700, a701)
        }
    }
}

pub fn f130(v0: Bytes, v1: u64) -> R<u64> {
    let (h702, _) = f25(Fn5::F146(v0.clone()), {
        let a704 = {
            let a703 = v0;
            length_bytes(a703)
        };
        let a705 = 1u64;
        nat_add(a704, a705)?
    }, v1)?;
    Ok(h702)
}

pub fn f131(v0: Bytes) -> bool {
    let a706 = v0.clone();
    let a707 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a706, a707) {
        true
    } else {
        let a708 = v0;
        let a709 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a708, a709)
    }
}

pub fn f132(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a710 = v0.clone();
        length_bytes(a710)
    };
    let v2: Adt8 = {
        let (h711, _) = f147(Fn19::F148(v1, v0), {
            let a712 = v1;
            let a713 = 1u64;
            nat_add(a712, a713)?
        }, Adt8::C0(0u64, List::<Bytes>::nil()))?;
        h711
    };
    Ok(f118(f118(f65(Str::lit("\"")), f63(match v2 {
        Adt8::C0(_, h714) => {
            h714
        }
    })), f65(Str::lit("\""))))
}

pub fn f133(v0: Fn17, p718: Adt0, p719: List<Adt0>) -> Adt0 {
    let m717 = (p718, p719);
    {
        let mut m717 = m717;
        loop {
            let step = {
                let (v1, v2) = m717;
                let m715 = v2;
                match m715.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c716: Fn17 = v0.clone();
                            c716.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m717 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f135(v0: Bytes, v1: Bytes) -> bool {
    let a724 = {
        let a720 = v0;
        let a721 = v1;
        compare_bytes(a720, a721)
    };
    let a725 = {
        let a722 = Bytes::lit(&[]);
        let a723 = Bytes::lit(&[0x01]);
        compare_bytes(a722, a723)
    };
    equal(a724, a725)
}

pub fn f136(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m726 = f149(v0.clone(), v1)?;
    match m726 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m729 = f149(v0.clone(), {
                let a727 = v1;
                let a728 = 1u64;
                nat_add(a727, a728)?
            })?;
            match m729 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m732 = f149(v0.clone(), {
                        let a730 = v1;
                        let a731 = 2u64;
                        nat_add(a730, a731)?
                    })?;
                    match m732 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m735 = f149(v0, {
                                let a733 = v1;
                                let a734 = 3u64;
                                nat_add(a733, a734)?
                            })?;
                            match m735 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a746 = {
                                            let a744 = {
                                                let a742 = {
                                                    let a740 = {
                                                        let a738 = {
                                                            let a736 = v2;
                                                            let a737 = 16u64;
                                                            nat_mul(a736, a737)?
                                                        };
                                                        let a739 = v3;
                                                        nat_add(a738, a739)?
                                                    };
                                                    let a741 = 16u64;
                                                    nat_mul(a740, a741)?
                                                };
                                                let a743 = v4;
                                                nat_add(a742, a743)?
                                            };
                                            let a745 = 16u64;
                                            nat_mul(a744, a745)?
                                        };
                                        let a747 = v5;
                                        nat_add(a746, a747)?
                                    }))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f137(v0: u64) -> Bytes {
    f118(f118(f150(v0, 4096u64), f150(v0, 256u64)), f118(f150(v0, 16u64), f151(v0)))
}

pub fn f138(v0: u64) -> R<Bytes> {
    let a748 = v0;
    let a749 = 128u64;
    if nat_lt(a748, a749) {
        Ok(f116(v0))
    } else {
        let a750 = v0;
        let a751 = 2048u64;
        if nat_lt(a750, a751) {
            Ok(f118(f116({
                let a755 = 192u64;
                let a756 = {
                    let a752 = v0;
                    let a753 = 64u64;
                    let a754 = 0u64;
                    nat_quot(a752, a753, a754)
                };
                nat_add(a755, a756)?
            }), f116({
                let a760 = 128u64;
                let a761 = {
                    let a757 = v0;
                    let a758 = 64u64;
                    let a759 = 0u64;
                    nat_rem(a757, a758, a759)
                };
                nat_add(a760, a761)?
            })))
        } else {
            Ok(f118(f116({
                let a765 = 224u64;
                let a766 = {
                    let a762 = v0;
                    let a763 = 4096u64;
                    let a764 = 0u64;
                    nat_quot(a762, a763, a764)
                };
                nat_add(a765, a766)?
            }), f118(f116({
                let a773 = 128u64;
                let a774 = {
                    let a770 = {
                        let a767 = v0;
                        let a768 = 64u64;
                        let a769 = 0u64;
                        nat_quot(a767, a768, a769)
                    };
                    let a771 = 64u64;
                    let a772 = 0u64;
                    nat_rem(a770, a771, a772)
                };
                nat_add(a773, a774)?
            }), f116({
                let a778 = 128u64;
                let a779 = {
                    let a775 = v0;
                    let a776 = 64u64;
                    let a777 = 0u64;
                    nat_rem(a775, a776, a777)
                };
                nat_add(a778, a779)?
            }))))
        }
    }
}

pub fn f139(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a780 = v2;
    let a781 = v1;
    if nat_lt(a780, a781) {
        let v3: u64 = f102(v0, v2)?;
        let a782 = v3;
        let a783 = 32u64;
        if nat_lt(a782, a783) {
            Ok(None::<u64>)
        } else {
            let a784 = v3;
            let a785 = 34u64;
            if nat_eq(a784, a785) {
                Ok(None::<u64>)
            } else {
                let a786 = v3;
                let a787 = 92u64;
                if nat_eq(a786, a787) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a788 = v2;
                        let a789 = 1u64;
                        nat_add(a788, a789)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f140(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m791 = {
        let (h790, _) = v0;
        h790
    };
    match m791 {
        None => {
            (Some(v1), {
                let (_, h792) = v0;
                h792
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a793 = v2;
                let a794 = v1;
                append_bytes(a793, a794)
            }, {
                let (_, h795) = v0;
                h795
            }))
        }
    }
}

pub fn f141(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f152(v0, v1)
}

pub fn f142(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f153(v0, v1)
}

pub fn f143(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a796 = v0.clone();
        length_bytes(a796)
    };
    let a797 = 0u64;
    let a798 = v1;
    if nat_lt(a797, a798) {
        let a799 = f154(v0.clone())?;
        let a800 = v1;
        if nat_eq(a799, a800) {
            let a801 = 1u64;
            let a802 = v1;
            if nat_lt(a801, a802) {
                let m805 = {
                    let a803 = v0;
                    let a804 = 0u64;
                    index_bytes(a803, a804)
                };
                match m805 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a806 = v2;
                        let a807 = 48u8;
                        Ok(!equal(a806, a807))
                    }
                }
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f144(v0: Fn18, p812: u64, p813: (u64, u64)) -> R<((u64, u64), bool)> {
    let m811 = (p812, p813);
    {
        let mut m811 = m811;
        loop {
            let step = {
                let (v1, v2) = m811;
                let m808 = v1;
                if m808 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m808 - 1;
                    let m810 = {
                        let c809: Fn18 = v0.clone();
                        c809.apply(v2)?
                    };
                    match m810 {
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
                Ok(next) => { m811 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f155(v0, v1)
}

pub fn f146(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f156(v0, v1)
}

pub fn f147(v0: Fn19, p818: u64, p819: Adt8) -> R<(Adt8, bool)> {
    let m817 = (p818, p819);
    {
        let mut m817 = m817;
        loop {
            let step = {
                let (v1, v2) = m817;
                let m814 = v1;
                if m814 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m814 - 1;
                    let m816 = {
                        let c815: Fn19 = v0.clone();
                        c815.apply(v2.clone())?
                    };
                    match m816 {
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
                Ok(next) => { m817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: u64, v1: Bytes, v2: Adt8) -> R<Option<Adt8>> {
    f157(v1, v0, v2)
}

pub fn f149(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m820 = f61(v0, v1);
    match m820 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f158(v2)
        }
    }
}

pub fn f150(v0: u64, v1: u64) -> Bytes {
    f151({
        let a821 = v0;
        let a822 = v1;
        let a823 = 0u64;
        nat_quot(a821, a822, a823)
    })
}

pub fn f151(v0: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a824 = v0;
        let a825 = 16u64;
        let a826 = 0u64;
        nat_rem(a824, a825, a826)
    }, 1u64)
}

pub fn f152(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f124(v0, v1)?;
    let a827 = 47u64;
    let a828 = v2;
    if nat_lt(a827, a828) {
        let a829 = v2;
        let a830 = 58u64;
        if nat_lt(a829, a830) {
            Ok(Some({
                let a831 = v1;
                let a832 = 1u64;
                nat_add(a831, a832)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f153(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a833 = f124(v0, v1)?;
    let a834 = 48u64;
    if nat_eq(a833, a834) {
        Ok(Some({
            let a835 = v1;
            let a836 = 1u64;
            nat_add(a835, a836)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f154(v0: Bytes) -> R<u64> {
    let (h837, _) = f25(Fn5::F159(v0.clone()), {
        let a838 = v0;
        length_bytes(a838)
    }, 0u64)?;
    Ok(h837)
}

pub fn f155(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m842 = {
        let a840 = v0;
        let a841 = {
            let (h839, _) = v1;
            h839
        };
        index_bytes(a840, a841)
    };
    match m842 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a844 = {
                    let (h843, _) = v1;
                    h843
                };
                let a845 = 1u64;
                nat_add(a844, a845)?
            }, {
                let a849 = {
                    let a847 = {
                        let (_, h846) = v1;
                        h846
                    };
                    let a848 = 10u64;
                    nat_mul(a847, a848)?
                };
                let a850 = f160(v2)?;
                nat_add(a849, a850)?
            })))
        }
    }
}

pub fn f156(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f161(f124(v0, v1)?) {
        Ok(Some({
            let a851 = v1;
            let a852 = 1u64;
            nat_add(a851, a852)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f157(v0: Bytes, v1: u64, v2: Adt8) -> R<Option<Adt8>> {
    let v3: u64 = match v2.clone() {
        Adt8::C0(h853, _) => {
            h853
        }
    };
    let a854 = v3;
    let a855 = v1;
    if nat_lt(a854, a855) {
        let v4: u64 = f102(v0.clone(), v3)?;
        let a856 = v4;
        let a857 = 32u64;
        if if nat_lt(a856, a857) {
            true
        } else {
            let a858 = v4;
            let a859 = 34u64;
            if nat_eq(a858, a859) {
                true
            } else {
                let a860 = v4;
                let a861 = 92u64;
                nat_eq(a860, a861)
            }
        } {
            Ok(Some(Adt8::C0({
                let a862 = v3;
                let a863 = 1u64;
                nat_add(a862, a863)?
            }, List::cons(f162(v4), match v2 {
                Adt8::C0(_, h864) => {
                    h864
                }
            }))))
        } else {
            let v5: u64 = f104(v0.clone(), v1, v3)?;
            Ok(Some(Adt8::C0(v5, List::cons(f67(v0, v3, {
                let a865 = v5;
                let a866 = v3;
                nat_sub(a865, a866)
            }), match v2 {
                Adt8::C0(_, h867) => {
                    h867
                }
            }))))
        }
    } else {
        Ok(None::<Adt8>)
    }
}

pub fn f158(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a912 = {
            let a888 = {
                let a876 = {
                    let a870 = {
                        let a868 = v1;
                        let a869 = 128u8;
                        fixed_u8::bit_and(a868, a869)
                    };
                    let a871 = 128u8;
                    if equal(a870, a871) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a877 = {
                    let a874 = {
                        let a872 = v1;
                        let a873 = 64u8;
                        fixed_u8::bit_and(a872, a873)
                    };
                    let a875 = 64u8;
                    if equal(a874, a875) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a876, a877)?
            };
            let a889 = {
                let a886 = {
                    let a880 = {
                        let a878 = v1;
                        let a879 = 32u8;
                        fixed_u8::bit_and(a878, a879)
                    };
                    let a881 = 32u8;
                    if equal(a880, a881) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a887 = {
                    let a884 = {
                        let a882 = v1;
                        let a883 = 16u8;
                        fixed_u8::bit_and(a882, a883)
                    };
                    let a885 = 16u8;
                    if equal(a884, a885) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a886, a887)?
            };
            nat_add(a888, a889)?
        };
        let a913 = {
            let a910 = {
                let a898 = {
                    let a892 = {
                        let a890 = v1;
                        let a891 = 8u8;
                        fixed_u8::bit_and(a890, a891)
                    };
                    let a893 = 8u8;
                    if equal(a892, a893) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a899 = {
                    let a896 = {
                        let a894 = v1;
                        let a895 = 4u8;
                        fixed_u8::bit_and(a894, a895)
                    };
                    let a897 = 4u8;
                    if equal(a896, a897) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a898, a899)?
            };
            let a911 = {
                let a908 = {
                    let a902 = {
                        let a900 = v1;
                        let a901 = 2u8;
                        fixed_u8::bit_and(a900, a901)
                    };
                    let a903 = 2u8;
                    if equal(a902, a903) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a909 = {
                    let a906 = {
                        let a904 = v1;
                        let a905 = 1u8;
                        fixed_u8::bit_and(a904, a905)
                    };
                    let a907 = 1u8;
                    if equal(a906, a907) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a908, a909)?
            };
            nat_add(a910, a911)?
        };
        nat_add(a912, a913)?
    };
    let a914 = v2;
    let a915 = 48u64;
    if nat_lt(a914, a915) {
        Ok(None::<u64>)
    } else {
        let a916 = v2;
        let a917 = 57u64;
        if nat_le(a916, a917) {
            Ok(Some({
                let a918 = v2;
                let a919 = 48u64;
                nat_sub(a918, a919)
            }))
        } else {
            let a920 = v2;
            let a921 = 65u64;
            if nat_lt(a920, a921) {
                Ok(None::<u64>)
            } else {
                let a922 = v2;
                let a923 = 70u64;
                if nat_le(a922, a923) {
                    Ok(Some({
                        let a924 = v2;
                        let a925 = 55u64;
                        nat_sub(a924, a925)
                    }))
                } else {
                    let a926 = v2;
                    let a927 = 97u64;
                    if nat_lt(a926, a927) {
                        Ok(None::<u64>)
                    } else {
                        let a928 = v2;
                        let a929 = 102u64;
                        if nat_le(a928, a929) {
                            Ok(Some({
                                let a930 = v2;
                                let a931 = 87u64;
                                nat_sub(a930, a931)
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f159(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m934 = {
        let a932 = v0;
        let a933 = v1;
        index_bytes(a932, a933)
    };
    match m934 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f163(v2)? {
                Ok(Some({
                    let a935 = v1;
                    let a936 = 1u64;
                    nat_add(a935, a936)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f160(v0: u8) -> R<u64> {
    let a983 = {
        let v1: u8 = v0;
        let a981 = {
            let a957 = {
                let a945 = {
                    let a939 = {
                        let a937 = v1;
                        let a938 = 128u8;
                        fixed_u8::bit_and(a937, a938)
                    };
                    let a940 = 128u8;
                    if equal(a939, a940) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a946 = {
                    let a943 = {
                        let a941 = v1;
                        let a942 = 64u8;
                        fixed_u8::bit_and(a941, a942)
                    };
                    let a944 = 64u8;
                    if equal(a943, a944) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a945, a946)?
            };
            let a958 = {
                let a955 = {
                    let a949 = {
                        let a947 = v1;
                        let a948 = 32u8;
                        fixed_u8::bit_and(a947, a948)
                    };
                    let a950 = 32u8;
                    if equal(a949, a950) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a956 = {
                    let a953 = {
                        let a951 = v1;
                        let a952 = 16u8;
                        fixed_u8::bit_and(a951, a952)
                    };
                    let a954 = 16u8;
                    if equal(a953, a954) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a955, a956)?
            };
            nat_add(a957, a958)?
        };
        let a982 = {
            let a979 = {
                let a967 = {
                    let a961 = {
                        let a959 = v1;
                        let a960 = 8u8;
                        fixed_u8::bit_and(a959, a960)
                    };
                    let a962 = 8u8;
                    if equal(a961, a962) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a968 = {
                    let a965 = {
                        let a963 = v1;
                        let a964 = 4u8;
                        fixed_u8::bit_and(a963, a964)
                    };
                    let a966 = 4u8;
                    if equal(a965, a966) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a967, a968)?
            };
            let a980 = {
                let a977 = {
                    let a971 = {
                        let a969 = v1;
                        let a970 = 2u8;
                        fixed_u8::bit_and(a969, a970)
                    };
                    let a972 = 2u8;
                    if equal(a971, a972) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a978 = {
                    let a975 = {
                        let a973 = v1;
                        let a974 = 1u8;
                        fixed_u8::bit_and(a973, a974)
                    };
                    let a976 = 1u8;
                    if equal(a975, a976) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a977, a978)?
            };
            nat_add(a979, a980)?
        };
        nat_add(a981, a982)?
    };
    let a984 = 48u64;
    Ok(nat_sub(a983, a984))
}

pub fn f161(v0: u64) -> bool {
    if f129(v0) {
        true
    } else {
        let a985 = 47u64;
        let a986 = v0;
        if if nat_lt(a985, a986) {
            let a987 = v0;
            let a988 = 58u64;
            nat_lt(a987, a988)
        } else {
            false
        } {
            true
        } else {
            let a989 = v0;
            let a990 = 46u64;
            if nat_eq(a989, a990) {
                true
            } else {
                let a991 = v0;
                let a992 = 39u64;
                if nat_eq(a991, a992) {
                    true
                } else {
                    let a993 = v0;
                    let a994 = 47u64;
                    if nat_eq(a993, a994) {
                        true
                    } else {
                        let a995 = v0;
                        let a996 = 45u64;
                        nat_eq(a995, a996)
                    }
                }
            }
        }
    }
}

pub fn f162(v0: u64) -> Bytes {
    let a997 = v0;
    let a998 = 34u64;
    if nat_eq(a997, a998) {
        f65(Str::lit("\\\""))
    } else {
        let a999 = v0;
        let a1000 = 92u64;
        if nat_eq(a999, a1000) {
            f65(Str::lit("\\\\"))
        } else {
            let a1001 = v0;
            let a1002 = 8u64;
            if nat_eq(a1001, a1002) {
                f65(Str::lit("\\b"))
            } else {
                let a1003 = v0;
                let a1004 = 12u64;
                if nat_eq(a1003, a1004) {
                    f65(Str::lit("\\f"))
                } else {
                    let a1005 = v0;
                    let a1006 = 10u64;
                    if nat_eq(a1005, a1006) {
                        f65(Str::lit("\\n"))
                    } else {
                        let a1007 = v0;
                        let a1008 = 13u64;
                        if nat_eq(a1007, a1008) {
                            f65(Str::lit("\\r"))
                        } else {
                            let a1009 = v0;
                            let a1010 = 9u64;
                            if nat_eq(a1009, a1010) {
                                f65(Str::lit("\\t"))
                            } else {
                                f118(f65(Str::lit("\\u00")), f164(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f163(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1055 = {
            let a1031 = {
                let a1019 = {
                    let a1013 = {
                        let a1011 = v1;
                        let a1012 = 128u8;
                        fixed_u8::bit_and(a1011, a1012)
                    };
                    let a1014 = 128u8;
                    if equal(a1013, a1014) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1020 = {
                    let a1017 = {
                        let a1015 = v1;
                        let a1016 = 64u8;
                        fixed_u8::bit_and(a1015, a1016)
                    };
                    let a1018 = 64u8;
                    if equal(a1017, a1018) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1019, a1020)?
            };
            let a1032 = {
                let a1029 = {
                    let a1023 = {
                        let a1021 = v1;
                        let a1022 = 32u8;
                        fixed_u8::bit_and(a1021, a1022)
                    };
                    let a1024 = 32u8;
                    if equal(a1023, a1024) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1030 = {
                    let a1027 = {
                        let a1025 = v1;
                        let a1026 = 16u8;
                        fixed_u8::bit_and(a1025, a1026)
                    };
                    let a1028 = 16u8;
                    if equal(a1027, a1028) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1029, a1030)?
            };
            nat_add(a1031, a1032)?
        };
        let a1056 = {
            let a1053 = {
                let a1041 = {
                    let a1035 = {
                        let a1033 = v1;
                        let a1034 = 8u8;
                        fixed_u8::bit_and(a1033, a1034)
                    };
                    let a1036 = 8u8;
                    if equal(a1035, a1036) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1042 = {
                    let a1039 = {
                        let a1037 = v1;
                        let a1038 = 4u8;
                        fixed_u8::bit_and(a1037, a1038)
                    };
                    let a1040 = 4u8;
                    if equal(a1039, a1040) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1041, a1042)?
            };
            let a1054 = {
                let a1051 = {
                    let a1045 = {
                        let a1043 = v1;
                        let a1044 = 2u8;
                        fixed_u8::bit_and(a1043, a1044)
                    };
                    let a1046 = 2u8;
                    if equal(a1045, a1046) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1052 = {
                    let a1049 = {
                        let a1047 = v1;
                        let a1048 = 1u8;
                        fixed_u8::bit_and(a1047, a1048)
                    };
                    let a1050 = 1u8;
                    if equal(a1049, a1050) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1051, a1052)?
            };
            nat_add(a1053, a1054)?
        };
        nat_add(a1055, a1056)?
    };
    let a1057 = 48u64;
    let a1058 = v2;
    if nat_le(a1057, a1058) {
        let a1059 = v2;
        let a1060 = 57u64;
        Ok(nat_le(a1059, a1060))
    } else {
        Ok(false)
    }
}

pub fn f164(v0: u64) -> Bytes {
    f118(f150(v0, 16u64), f151(v0))
}
