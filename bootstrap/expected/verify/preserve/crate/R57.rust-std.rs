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
    C0(Adt8, Adt11),
    C1(Adt12, Adt13, Adt11),
    C2(Adt14, std::rc::Rc<Adt7>, Adt11),
    C3(Adt15, std::rc::Rc<Adt7>, bool, Adt11),
    C4(Adt13, std::rc::Rc<Adt7>, bool, Adt11),
    C5(Adt17, std::rc::Rc<Adt7>, Adt11),
    C6(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, Adt11),
    C7(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, Adt11),
    C8(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, Adt11),
    C9(std::rc::Rc<Adt7>),
    C10(Adt23, Adt13, Adt11),
    C11(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C12,
    C13(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C14(Adt24, bool, Adt18, std::rc::Rc<Adt7>, Adt11, std::rc::Rc<Adt7>),
    C15(Adt24, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C16(Adt13, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt9, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt10),
    C8,
}

#[derive(Clone)]
pub enum Adt9 {
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
pub enum Adt10 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt13 {
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
pub enum Adt14 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64),
    C1(Adt16),
    C2(u64, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u64, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18),
    C1,
    C2(Adt18, Adt18),
    C3(Adt18, Adt18),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt18),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt19),
    C1(Adt9),
    C2(Adt20, std::rc::Rc<Adt18>),
    C3(Adt21, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4(Adt22, u64),
    C5,
    C6(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
}

#[derive(Clone)]
pub enum Adt19 {
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
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt22 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt23 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt25),
    C1(Adt13),
    C2(std::rc::Rc<Adt24>),
    C3(Adt26, std::rc::Rc<Adt24>),
    C4(u64, u64, std::rc::Rc<Adt24>),
    C5,
    C6(std::rc::Rc<Adt24>, std::rc::Rc<Adt24>),
    C7(Adt10),
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt26 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Adt0, Adt29),
}

#[derive(Clone)]
pub enum Adt29 {
    C0,
    C1(Adt0, u64, Adt0, std::rc::Rc<Adt29>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0,
    C1(Adt0, Adt0, std::rc::Rc<Adt30>),
}

#[derive(Clone)]
pub enum Fn0 {
    F21,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt30) -> R<Option<Adt30>> {
        match self {
            Fn0::F21 => f21(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F53(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn1::F53(k0) => f53(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F79(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn2::F79(k0) => f79(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: u64) -> R<Option<Adt7>> {
    let v8: Adt27 = Adt27::C0(v0, v1.clone(), v2, v3, v4, v5.clone());
    let v9: Adt28 = f1(v8.clone(), Adt28::C0(Adt0::C14, Adt29::C0), v5.clone(), 0u64, List::<bool>::nil(), v6)?;
    let v10: Adt0 = f2(match v9.clone() {
        Adt28::C0(h1, _) => {
            h1
        }
    }, v1.clone());
    let v11: Adt0 = f3(f4(f5(v1.clone())?), v10.clone());
    let v12: Adt30 = f6(v8, v10.clone(), match v9 {
        Adt28::C0(_, h2) => {
            h2
        }
    }, v11.clone(), Adt30::C0);
    let v13: Adt30 = f7(v12, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f5(v11.clone())?;
            let a4 = f5(v11)?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?;
    let m7 = f8(v5, v7);
    match m7 {
        None => {
            Ok(None::<Adt7>)
        }
        Some(v14) => {
            let m8 = f9(v14);
            match m8 {
                None => {
                    Ok(None::<Adt7>)
                }
                Some(v15) => {
                    let v16: Adt7 = f10(v10, v13, v1, Adt0::C14, 0u64, {
                        let (_, h9) = v15;
                        h9
                    }, {
                        let (h10, _) = v15;
                        h10
                    })?;
                    if f11(v16.clone()) {
                        Ok(None::<Adt7>)
                    } else {
                        Ok(Some(v16))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: Adt27, p21: Adt28, p22: Adt2, p23: u64, v4: List<bool>, p24: Adt0) -> R<Adt28> {
    let m20 = (p21, (p22, (p23, p24)));
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m20;
                let m11 = v2;
                match m11 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C14(r18, r19) => {
                        let v34 = (*r18).clone();
                        let v35 = (*r19).clone();
                        let v37: bool = {
                            let m12 = f12(v4.clone(), v3);
                            match m12 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a13 = v36;
                                    bool_not(a13)
                                }
                            }
                        };
                        let v38: Adt28 = Adt28::C0(f13(v0.clone(), match v1.clone() {
                            Adt28::C0(h14, _) => {
                                h14
                            }
                        }, v3), match v1 {
                            Adt28::C0(_, h15) => {
                                h15
                            }
                        });
                        let v39: Adt28 = if v37 {
                            f14(v0.clone(), v38, v34, f15(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>((v39, (v35, ({
                            let a16 = v3;
                            let a17 = 1u64;
                            nat_add(a16, a17)?
                        }, f16(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(p29: Adt0, p30: Adt0) -> Adt0 {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v0, v1) = m28;
                let m25 = v1;
                match m25 {
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
                    Adt0::C15(r26, r27) => {
                        let v12 = (*r26).clone();
                        let v13 = (*r27).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f17(v0, v12), v13))
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

pub fn f3(v0: Adt0, v1: Adt0) -> Adt0 {
    let m31 = v0;
    match m31 {
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
        Adt0::C15(r32, r33) => {
            let v12 = (*r32).clone();
            let v13 = (*r33).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f3(v13, v1)))
        }
    }
}

pub fn f4(v0: u64) -> Adt0 {
    let m34 = v0;
    if m34 == 0 {
        Adt0::C14
    } else {
        let v1 = m34 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f4(v1)))
    }
}

pub fn f5(v0: Adt0) -> R<u64> {
    let m35 = v0;
    match m35 {
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
        Adt0::C15(_, r38) => {
            let v12 = (*r38).clone();
            let v13: u64 = f5(v12)?;
            let a36 = 1u64;
            let a37 = v13;
            nat_add(a36, a37)
        }
    }
}

pub fn f6(v0: Adt27, v1: Adt0, v2: Adt29, p43: Adt0, p44: Adt30) -> Adt30 {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v3, v4) = m42;
                let m39 = v3;
                match m39 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C15(r40, r41) => {
                        let v15 = (*r40).clone();
                        let v16 = (*r41).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v16, f18(v1.clone(), v15.clone(), f19(v0.clone(), v2.clone(), v15), v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m42 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt30, v1: u64) -> R<Adt30> {
    let (h45, _) = f20(Fn0::F21, v1, v0)?;
    Ok(h45)
}

pub fn f8(p51: Adt2, p52: u64) -> Option<Adt2> {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v0, v1) = m50;
                let m46 = v0;
                match m46 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C13 => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C14(r48, r49) => {
                        let v30 = (*r48).clone();
                        let v31 = (*r49).clone();
                        let m47 = v1;
                        if m47 == 0 {
                            Err::<(Adt2, u64), Option<Adt2>>(Some(v30))
                        } else {
                            let v32 = m47 - 1;
                            Ok::<(Adt2, u64), Option<Adt2>>((v31, v32))
                        }
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
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

pub fn f9(v0: Adt2) -> Option<(Adt0, Adt3)> {
    let m53 = v0;
    match m53 {
        Adt2::C0(v1, v2) => {
            Some((v1, v2))
        }
        Adt2::C1(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C2(_, _, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C3(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C4(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C5(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C6(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C7(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C8(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C9(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C10(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C11(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C12(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C13 => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C14(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C15(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
    }
}

pub fn f10(v0: Adt0, v1: Adt30, v2: Adt0, v3: Adt0, v4: u64, v5: Adt3, v6: Adt0) -> R<Adt7> {
    let m54 = v5;
    match m54 {
        Adt3::C0 => {
            Ok(Adt7::C0(Adt8::C0, f22({
                let a55 = Str::lit("value:unit");
                utf8_encode(a55)
            }, v6)))
        }
        Adt3::C1(v7) => {
            Ok(Adt7::C0(Adt8::C1(v7), f22({
                let a56 = Str::lit("value:bool");
                utf8_encode(a56)
            }, v6)))
        }
        Adt3::C2(v8) => {
            let m57 = f23(v8)?;
            match m57 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v9) => {
                    Ok(Adt7::C0(Adt8::C2(v9), f22({
                        let a58 = Str::lit("value:nat");
                        utf8_encode(a58)
                    }, v6)))
                }
            }
        }
        Adt3::C3(v10) => {
            let m59 = f25(v10)?;
            match m59 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v11) => {
                    Ok(Adt7::C0(Adt8::C3({
                        let (h60, _) = v11;
                        h60
                    }, {
                        let (_, h61) = v11;
                        h61
                    }), f22({
                        let a62 = Str::lit("value:int");
                        utf8_encode(a62)
                    }, v6)))
                }
            }
        }
        Adt3::C4(v12) => {
            f26(v12, v6, {
                let a63 = Str::lit("value:u8");
                utf8_encode(a63)
            })
        }
        Adt3::C5(v13) => {
            f26(v13, v6, {
                let a64 = Str::lit("value:u16");
                utf8_encode(a64)
            })
        }
        Adt3::C6(v14) => {
            f26(v14, v6, {
                let a65 = Str::lit("value:u32");
                utf8_encode(a65)
            })
        }
        Adt3::C7(v15) => {
            f26(v15, v6, {
                let a66 = Str::lit("value:u64");
                utf8_encode(a66)
            })
        }
        Adt3::C8(v16) => {
            f26(v16, v6, {
                let a67 = Str::lit("value:i8");
                utf8_encode(a67)
            })
        }
        Adt3::C9(v17) => {
            f26(v17, v6, {
                let a68 = Str::lit("value:i16");
                utf8_encode(a68)
            })
        }
        Adt3::C10(v18) => {
            f26(v18, v6, {
                let a69 = Str::lit("value:i32");
                utf8_encode(a69)
            })
        }
        Adt3::C11(v19) => {
            f26(v19, v6, {
                let a70 = Str::lit("value:i64");
                utf8_encode(a70)
            })
        }
        Adt3::C12(v20) => {
            Ok(Adt7::C0(Adt8::C5(v20), f22({
                let a71 = Str::lit("value:string");
                utf8_encode(a71)
            }, v6)))
        }
        Adt3::C13(v21) => {
            let m72 = f27(v21)?;
            match m72 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v22) => {
                    Ok(Adt7::C0(Adt8::C6(v22), f22({
                        let a73 = Str::lit("value:bytes");
                        utf8_encode(a73)
                    }, v6)))
                }
            }
        }
        Adt3::C14(v23) => {
            Ok(Adt7::C0(Adt8::C7({
                let m74 = v23;
                match m74 {
                    Adt4::C0 => {
                        Adt10::C0
                    }
                    Adt4::C1 => {
                        Adt10::C1
                    }
                    Adt4::C2 => {
                        Adt10::C2
                    }
                }
            }), f22({
                let a75 = Str::lit("value:ordering");
                utf8_encode(a75)
            }, v6)))
        }
        Adt3::C15 => {
            let v24: Adt0 = f28(v6.clone());
            Ok(Adt7::C5(Adt17::C0(f29(v0, v24)?), std::rc::Rc::new(Adt7::C12), f22({
                let a76 = Str::lit("value:none");
                utf8_encode(a76)
            }, v6)))
        }
        Adt3::C16(r97) => {
            let v25 = (*r97).clone();
            let v26: Adt0 = f28(v6.clone());
            let v27: Adt7 = f10(v0, v1, v2, v3, 0u64, v25, v26)?;
            if f11(v27.clone()) {
                Ok(f24(0u64))
            } else {
                Ok(Adt7::C5(Adt17::C1, std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v27), std::rc::Rc::new(Adt7::C12))), f22({
                    let a77 = Str::lit("value:some");
                    utf8_encode(a77)
                }, v6)))
            }
        }
        Adt3::C17(r98) => {
            let v28 = (*r98).clone();
            let v29: Adt7 = f10(v0.clone(), v1, v2, v3, 0u64, v28, f28(v6.clone()))?;
            if f11(v29.clone()) {
                Ok(f24(0u64))
            } else {
                Ok(Adt7::C5(Adt17::C2(f29(v0.clone(), f28(v6.clone()))?, f29(v0, f30(v6.clone()))?), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v29), std::rc::Rc::new(Adt7::C12))), f22({
                    let a78 = Str::lit("value:ok");
                    utf8_encode(a78)
                }, v6)))
            }
        }
        Adt3::C18(r99) => {
            let v30 = (*r99).clone();
            let v31: Adt7 = f10(v0.clone(), v1, v2, v3, 0u64, v30, f30(v6.clone()))?;
            if f11(v31.clone()) {
                Ok(f24(0u64))
            } else {
                Ok(Adt7::C5(Adt17::C3(f29(v0.clone(), f28(v6.clone()))?, f29(v0, f30(v6.clone()))?), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v31), std::rc::Rc::new(Adt7::C12))), f22({
                    let a79 = Str::lit("value:error");
                    utf8_encode(a79)
                }, v6)))
            }
        }
        Adt3::C19(r100) => {
            let v32 = (*r100).clone();
            f10(v0, v1, v2, v3, 1u64, v32, f28(v6))
        }
        Adt3::C20(r101, r102) => {
            let v33 = (*r101).clone();
            let v34 = (*r102).clone();
            let v35: Adt7 = f10(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v33, f28(v6.clone()))?;
            if f11(v35.clone()) {
                Ok(f24(0u64))
            } else {
                let v36: Adt7 = f10(v0, v1, v2, v3, 0u64, v34, f30(v6.clone()))?;
                if f11(v36.clone()) {
                    Ok(f24(0u64))
                } else {
                    Ok(Adt7::C6(std::rc::Rc::new(v35), std::rc::Rc::new(v36), f22({
                        let a80 = Str::lit("value:pair");
                        utf8_encode(a80)
                    }, v6)))
                }
            }
        }
        Adt3::C21(v37, r103) => {
            let v38 = (*r103).clone();
            let v39: u64 = f31(v6.clone());
            let m81 = f32(v2.clone(), v39, v37);
            match m81 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v40) => {
                    let v41: Adt7 = f10(v0, v1, v2, Adt0::C12(v39), 2u64, v38, v40)?;
                    if f11(v41.clone()) {
                        Ok(f24(0u64))
                    } else {
                        Ok(Adt7::C5(Adt17::C4(v39, v37), std::rc::Rc::new(v41), f22({
                            let a82 = Str::lit("value:adt");
                            utf8_encode(a82)
                        }, v6)))
                    }
                }
            }
        }
        Adt3::C22(_, _) => {
            Ok(f24(0u64))
        }
        Adt3::C23 => {
            let a83 = v4;
            let a84 = 1u64;
            if nat_eq(a83, a84) {
                Ok(Adt7::C5(Adt17::C7(f29(v0, v6)?), std::rc::Rc::new(Adt7::C12), Adt11::C0({
                    let a85 = Str::lit("value:list");
                    utf8_encode(a85)
                }, None::<Adt9>)))
            } else {
                let a86 = v4;
                let a87 = 2u64;
                if nat_eq(a86, a87) {
                    Ok(Adt7::C12)
                } else {
                    Ok(f24(0u64))
                }
            }
        }
        Adt3::C24(r104, r105) => {
            let v44 = (*r104).clone();
            let v45 = (*r105).clone();
            let a88 = v4;
            let a89 = 1u64;
            if nat_eq(a88, a89) {
                let v46: Adt7 = f10(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v6.clone())?;
                if f11(v46.clone()) {
                    Ok(f24(0u64))
                } else {
                    let v47: Adt7 = f10(v0, v1, v2, v3, 1u64, v45, v6)?;
                    if f11(v47.clone()) {
                        Ok(f24(0u64))
                    } else {
                        Ok(Adt7::C5(Adt17::C6, std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v46), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v47), std::rc::Rc::new(Adt7::C12))))), Adt11::C0({
                            let a90 = Str::lit("value:list");
                            utf8_encode(a90)
                        }, None::<Adt9>)))
                    }
                }
            } else {
                let a91 = v4;
                let a92 = 2u64;
                if nat_eq(a91, a92) {
                    let m93 = v6;
                    match m93 {
                        Adt0::C0 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C1 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C2 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C3 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C4(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C5 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C6 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C7 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C8(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C9(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C10(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C11(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C12(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C13(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C14 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C15(r95, r96) => {
                            let v58 = (*r95).clone();
                            let v59 = (*r96).clone();
                            let v60: Adt7 = f10(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v58.clone())?;
                            if f11(v60.clone()) {
                                Ok(f24(0u64))
                            } else {
                                let v61: Adt7 = f10(v0.clone(), v1.clone(), v2, v3.clone(), 2u64, v45, v59)?;
                                if f11(v61.clone()) {
                                    Ok(f24(0u64))
                                } else {
                                    Ok(Adt7::C13(std::rc::Rc::new(if f33(v0, v1, v3, v58) {
                                        Adt7::C2(Adt14::C1, std::rc::Rc::new(v60), Adt11::C0({
                                            let a94 = Str::lit("indirection");
                                            utf8_encode(a94)
                                        }, None::<Adt9>))
                                    } else {
                                        v60
                                    }), std::rc::Rc::new(v61)))
                                }
                            }
                        }
                    }
                } else {
                    Ok(f24(0u64))
                }
            }
        }
    }
}

pub fn f11(v0: Adt7) -> bool {
    let m106 = v0;
    match m106 {
        Adt7::C0(v1, _) => {
            let m107 = v1;
            match m107 {
                Adt8::C0 => {
                    false
                }
                Adt8::C1(_) => {
                    false
                }
                Adt8::C2(_) => {
                    false
                }
                Adt8::C3(_, _) => {
                    false
                }
                Adt8::C4(_, _, _) => {
                    false
                }
                Adt8::C5(_) => {
                    false
                }
                Adt8::C6(_) => {
                    false
                }
                Adt8::C7(_) => {
                    false
                }
                Adt8::C8 => {
                    true
                }
            }
        }
        Adt7::C1(_, _, _) => {
            false
        }
        Adt7::C2(_, _, _) => {
            false
        }
        Adt7::C3(_, _, _, _) => {
            false
        }
        Adt7::C4(_, _, _, _) => {
            false
        }
        Adt7::C5(_, _, _) => {
            false
        }
        Adt7::C6(_, _, _) => {
            false
        }
        Adt7::C7(_, _, _, _) => {
            false
        }
        Adt7::C8(_, _, _) => {
            false
        }
        Adt7::C9(_) => {
            false
        }
        Adt7::C10(_, _, _) => {
            false
        }
        Adt7::C11(_, _) => {
            false
        }
        Adt7::C12 => {
            false
        }
        Adt7::C13(_, _) => {
            false
        }
        Adt7::C14(_, _, _, _, _, _) => {
            false
        }
        Adt7::C15(_, _, _) => {
            false
        }
        Adt7::C16(_, _) => {
            false
        }
    }
}

pub fn f12(p111: List<bool>, p112: u64) -> Option<bool> {
    let m110 = (p111, p112);
    {
        let mut m110 = m110;
        loop {
            let step = {
                let (v0, v1) = m110;
                let m108 = v0;
                match m108.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m109 = v1;
                        if m109 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m109 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: Adt27, v1: Adt0, v2: u64) -> Adt0 {
    f34(f35(v1, f36(v0.clone(), v2)), f37(v0, v2))
}

pub fn f14(v0: Adt27, v1: Adt28, v2: Adt2, v3: Adt0) -> R<Adt28> {
    let m113 = v2;
    match m113 {
        Adt2::C0(v4, _) => {
            Ok(Adt28::C0(f34(match v1.clone() {
                Adt28::C0(h114, _) => {
                    h114
                }
            }, v4), match v1 {
                Adt28::C0(_, h115) => {
                    h115
                }
            }))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, v8, r124, r125) => {
            let v9 = (*r124).clone();
            let v10 = (*r125).clone();
            let v11: Adt28 = Adt28::C0(f34(match v1.clone() {
                Adt28::C0(h116, _) => {
                    h116
                }
            }, v8), match v1 {
                Adt28::C0(_, h117) => {
                    h117
                }
            });
            let v12: Adt28 = f14(v0.clone(), v11, v9, f15(v3.clone()))?;
            f14(v0, v12, v10, f16(v3))
        }
        Adt2::C3(r126, r127, r128) => {
            let v13 = (*r126).clone();
            let v14 = (*r127).clone();
            let v15 = (*r128).clone();
            let v16: Adt28 = f14(v0.clone(), v1, v13, f15(v3.clone()))?;
            let v17: Adt28 = f14(v0.clone(), v16, v14, f16(v3.clone()))?;
            f14(v0, v17, v15, f38(v3))
        }
        Adt2::C4(v18, r129, r130) => {
            let v19 = (*r129).clone();
            let v20 = (*r130).clone();
            let v21: Adt28 = Adt28::C0(f34(match v1.clone() {
                Adt28::C0(h118, _) => {
                    h118
                }
            }, v18), match v1 {
                Adt28::C0(_, h119) => {
                    h119
                }
            });
            let v22: Adt28 = f14(v0.clone(), v21, v19, f15(v3.clone()))?;
            f14(v0, v22, v20, f16(v3))
        }
        Adt2::C5(_, v24, r131) => {
            let v25 = (*r131).clone();
            let v26: Adt28 = Adt28::C0(f34(match v1.clone() {
                Adt28::C0(h120, _) => {
                    h120
                }
            }, v24), match v1 {
                Adt28::C0(_, h121) => {
                    h121
                }
            });
            f14(v0, v26, v25, f15(v3))
        }
        Adt2::C6(_, r132) => {
            let v28 = (*r132).clone();
            f14(v0, v1, v28, f15(v3))
        }
        Adt2::C7(v29, r133) => {
            let v30 = (*r133).clone();
            let v31: Adt0 = f36(v0.clone(), v29);
            let v32: u64 = f39(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f40(v31.clone(), v32)), std::rc::Rc::new(f37(v0.clone(), v29)));
            let v34: Adt28 = Adt28::C0(f34(match v1.clone() {
                Adt28::C0(h122, _) => {
                    h122
                }
            }, v33.clone()), f41(match v1 {
                Adt28::C0(_, h123) => {
                    h123
                }
            }, v33, v29, f42(v31, v32)));
            f14(v0, v34, v30, f15(v3))
        }
        Adt2::C8(r134, r135) => {
            let v35 = (*r134).clone();
            let v36 = (*r135).clone();
            let v37: Adt28 = f14(v0.clone(), v1, v35, f15(v3.clone()))?;
            f14(v0, v37, v36, f16(v3))
        }
        Adt2::C9(_, r136) => {
            let v39 = (*r136).clone();
            f14(v0, v1, v39, f15(v3))
        }
        Adt2::C10(r137) => {
            let v40 = (*r137).clone();
            f14(v0, v1, v40, f15(v3))
        }
        Adt2::C11(r138) => {
            let v41 = (*r138).clone();
            f14(v0, v1, v41, f15(v3))
        }
        Adt2::C12(r139, _) => {
            let v42 = (*r139).clone();
            f14(v0, v1, v42, f15(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r140, r141) => {
            let v44 = (*r140).clone();
            let v45 = (*r141).clone();
            let v46: Adt28 = f14(v0.clone(), v1, v44, f15(v3.clone()))?;
            f14(v0, v46, v45, f16(v3))
        }
        Adt2::C15(_, _, r142) => {
            let v49 = (*r142).clone();
            f14(v0, v1, v49, f15(v3))
        }
    }
}

pub fn f15(v0: Adt0) -> Adt0 {
    f28(f30(v0))
}

pub fn f16(v0: Adt0) -> Adt0 {
    f28(f30(f30(v0)))
}

pub fn f17(p147: Adt0, p148: Adt0) -> Adt0 {
    let m146 = (p147, p148);
    {
        let mut m146 = m146;
        loop {
            let step = {
                let (v0, v1) = m146;
                let m143 = v1;
                match m143 {
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
                    Adt0::C15(r144, r145) => {
                        let v12 = (*r144).clone();
                        let v13 = (*r145).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f35(v0, v12), v13))
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

pub fn f18(v0: Adt0, v1: Adt0, p153: Adt0, p154: Adt30) -> Adt30 {
    let m152 = (p153, p154);
    {
        let mut m152 = m152;
        loop {
            let step = {
                let (v2, v3) = m152;
                let m149 = v2;
                match m149 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C15(r150, r151) => {
                        let v14 = (*r150).clone();
                        let v15 = (*r151).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v15, f43(v0.clone(), v1.clone(), v14, v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt27, v1: Adt29, v2: Adt0) -> Adt0 {
    let m155 = v2;
    match m155 {
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
            let m156 = f44(v0, v10);
            match m156 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r157, r158) => {
            let v12 = (*r157).clone();
            let v13 = (*r158).clone();
            f45(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f20(v0: Fn0, p163: u64, p164: Adt30) -> R<(Adt30, bool)> {
    let m162 = (p163, p164);
    {
        let mut m162 = m162;
        loop {
            let step = {
                let (v1, v2) = m162;
                let m159 = v1;
                if m159 == 0 {
                    Err::<(u64, Adt30), R<(Adt30, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m159 - 1;
                    let m161 = {
                        let c160: Fn0 = v0.clone();
                        c160.apply(v2.clone())?
                    };
                    match m161 {
                        None => {
                            Err::<(u64, Adt30), R<(Adt30, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt30), R<(Adt30, bool)>>((v3, v4))
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

pub fn f21(v0: Adt30) -> R<Option<Adt30>> {
    f46(v0)
}

pub fn f22(v0: Bytes, v1: Adt0) -> Adt11 {
    Adt11::C0(v0, f47(v1))
}

pub fn f23(v0: Bytes) -> R<Option<u64>> {
    let m165 = f48(v0, false)?;
    match m165 {
        None => {
            Ok(None::<u64>)
        }
        Some(v1) => {
            Ok(Some({
                let (_, h166) = v1;
                h166
            }))
        }
    }
}

pub fn f24(_: u64) -> Adt7 {
    Adt7::C0(Adt8::C8, Adt11::C0({
        let a167 = Str::lit("");
        utf8_encode(a167)
    }, None::<Adt9>))
}

pub fn f25(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m168 = f48(v0, true)?;
    match m168 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            let (h169, _) = v1;
            if h169 {
                let a171 = {
                    let (_, h170) = v1;
                    h170
                };
                let a172 = 9223372036854775809u64;
                if nat_lt(a171, a172) {
                    Ok(Some(({
                        let a174 = 0u64;
                        let a175 = {
                            let (_, h173) = v1;
                            h173
                        };
                        nat_lt(a174, a175)
                    }, {
                        let (_, h176) = v1;
                        h176
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            } else {
                let a178 = {
                    let (_, h177) = v1;
                    h177
                };
                let a179 = 9223372036854775808u64;
                if nat_lt(a178, a179) {
                    Ok(Some((false, {
                        let (_, h180) = v1;
                        h180
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            }
        }
    }
}

pub fn f26(v0: Bytes, v1: Adt0, v2: Bytes) -> R<Adt7> {
    let m181 = f49(v1.clone());
    match m181 {
        None => {
            Ok(f24(0u64))
        }
        Some(v3) => {
            let m182 = f50(v0)?;
            match m182 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v4) => {
                    Ok(Adt7::C0(Adt8::C4(f51(v3), {
                        let (h183, _) = v4;
                        h183
                    }, {
                        let (_, h184) = v4;
                        h184
                    }), f22(v2, v1)))
                }
            }
        }
    }
}

pub fn f27(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a185 = v0.clone();
        length_bytes(a185)
    };
    let v2: ((u64, Bytes), bool) = f52(Fn1::F53(v0), v1, (0u64, {
        let a186 = Str::lit("");
        utf8_encode(a186)
    }))?;
    let a189 = {
        let (h187, _) = {
            let (h188, _) = v2.clone();
            h188
        };
        h187
    };
    let a190 = v1;
    if nat_eq(a189, a190) {
        Ok(Some({
            let (_, h191) = {
                let (h192, _) = v2;
                h192
            };
            h191
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f28(v0: Adt0) -> Adt0 {
    let m193 = f54(v0);
    match m193 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f29(v0: Adt0, v1: Adt0) -> R<Adt18> {
    let m194 = v1;
    match m194 {
        Adt0::C0 => {
            Ok(Adt18::C0(Adt19::C0))
        }
        Adt0::C1 => {
            Ok(Adt18::C0(Adt19::C1))
        }
        Adt0::C2 => {
            Ok(Adt18::C0(Adt19::C2))
        }
        Adt0::C3 => {
            Ok(Adt18::C0(Adt19::C3))
        }
        Adt0::C4(v2) => {
            Ok(Adt18::C1(f51(v2)))
        }
        Adt0::C5 => {
            Ok(Adt18::C0(Adt19::C5))
        }
        Adt0::C6 => {
            Ok(Adt18::C0(Adt19::C6))
        }
        Adt0::C7 => {
            Ok(Adt18::C0(Adt19::C4))
        }
        Adt0::C8(r195) => {
            let v3 = (*r195).clone();
            Ok(Adt18::C2(Adt20::C0, std::rc::Rc::new(f29(v0, v3)?)))
        }
        Adt0::C9(r196, r197) => {
            let v4 = (*r196).clone();
            let v5 = (*r197).clone();
            Ok(Adt18::C3(Adt21::C0, std::rc::Rc::new(f29(v0.clone(), v4)?), std::rc::Rc::new(f29(v0, v5)?)))
        }
        Adt0::C10(r198) => {
            let v6 = (*r198).clone();
            Ok(Adt18::C2(Adt20::C1, std::rc::Rc::new(f29(v0, v6)?)))
        }
        Adt0::C11(r199, r200) => {
            let v7 = (*r199).clone();
            let v8 = (*r200).clone();
            Ok(Adt18::C3(Adt21::C1, std::rc::Rc::new(f29(v0.clone(), v7)?), std::rc::Rc::new(f29(v0, v8)?)))
        }
        Adt0::C12(v9) => {
            Ok(Adt18::C4(Adt22::C0, v9))
        }
        Adt0::C13(r201, r202) => {
            let v10 = (*r201).clone();
            let v11 = (*r202).clone();
            Ok(Adt18::C4(Adt22::C1, f55(v0, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)), 0u64)?))
        }
        Adt0::C14 => {
            Ok(Adt18::C5)
        }
        Adt0::C15(r203, r204) => {
            let v12 = (*r203).clone();
            let v13 = (*r204).clone();
            Ok(Adt18::C6(std::rc::Rc::new(f29(v0.clone(), v12)?), std::rc::Rc::new(f29(v0, v13)?)))
        }
    }
}

pub fn f30(v0: Adt0) -> Adt0 {
    let m205 = f56(v0);
    match m205 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f31(v0: Adt0) -> u64 {
    let m206 = v0;
    match m206 {
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

pub fn f32(v0: Adt0, v1: u64, v2: u64) -> Option<Adt0> {
    let m207 = f57(v0, v1);
    match m207 {
        None => {
            None::<Adt0>
        }
        Some(v3) => {
            f57(v3, v2)
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt30, v2: Adt0, v3: Adt0) -> bool {
    f58(v2, v1, f59(v0, Adt0::C14, v3))
}

pub fn f34(v0: Adt0, v1: Adt0) -> Adt0 {
    let m208 = v1;
    match m208 {
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
        Adt0::C8(r209) => {
            let v3 = (*r209).clone();
            f34(v0, v3)
        }
        Adt0::C9(r210, r211) => {
            let v4 = (*r210).clone();
            let v5 = (*r211).clone();
            f34(f34(v0, v4), v5)
        }
        Adt0::C10(r212) => {
            let v6 = (*r212).clone();
            f34(v0, v6)
        }
        Adt0::C11(r213, r214) => {
            let v7 = (*r213).clone();
            let v8 = (*r214).clone();
            f34(f34(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r215, r216) => {
            let v10 = (*r215).clone();
            let v11 = (*r216).clone();
            let v12: Adt0 = f60(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f34(f34(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r217, r218) => {
            let v13 = (*r217).clone();
            let v14 = (*r218).clone();
            f34(f34(v0, v13), v14)
        }
    }
}

pub fn f35(p223: Adt0, p224: Adt0) -> Adt0 {
    let m222 = (p223, p224);
    {
        let mut m222 = m222;
        loop {
            let step = {
                let (v0, v1) = m222;
                let m219 = v1;
                match m219 {
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
                    Adt0::C15(r220, r221) => {
                        let v2 = (*r220).clone();
                        let v3 = (*r221).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f34(v0, v2), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Adt27, v1: u64) -> Adt0 {
    let m225 = f61(v0, v1);
    match m225 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f37(v0: Adt27, v1: u64) -> Adt0 {
    let m227 = f62(match v0 {
        Adt27::C0(_, _, _, _, h226, _) => {
            h226
        }
    }, v1);
    match m227 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f38(v0: Adt0) -> Adt0 {
    f28(f30(f30(f30(v0))))
}

pub fn f39(v0: Adt2) -> R<u64> {
    let m228 = v0;
    match m228 {
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
        Adt2::C14(_, r231) => {
            let v33 = (*r231).clone();
            let v34: u64 = f39(v33)?;
            let a229 = 1u64;
            let a230 = v34;
            nat_add(a229, a230)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f40(p237: Adt0, p238: u64) -> Adt0 {
    let m236 = (p237, p238);
    {
        let mut m236 = m236;
        loop {
            let step = {
                let (v0, v1) = m236;
                let m232 = v0;
                match m232 {
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
                    Adt0::C15(r234, r235) => {
                        let v12 = (*r234).clone();
                        let v13 = (*r235).clone();
                        let m233 = v1;
                        if m233 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m233 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Adt29, v1: Adt0, v2: u64, v3: Adt0) -> Adt29 {
    if f63(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt29::C1(v1, v2, v3, std::rc::Rc::new(v0))
    }
}

pub fn f42(v0: Adt0, v1: u64) -> Adt0 {
    let m239 = v0;
    match m239 {
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
        Adt0::C15(r241, r242) => {
            let v12 = (*r241).clone();
            let v13 = (*r242).clone();
            let m240 = v1;
            if m240 == 0 {
                Adt0::C14
            } else {
                let v14 = m240 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f42(v13, v14)))
            }
        }
    }
}

pub fn f43(v0: Adt0, v1: Adt0, p247: Adt0, p248: Adt30) -> Adt30 {
    let m246 = (p247, p248);
    {
        let mut m246 = m246;
        loop {
            let step = {
                let (v2, v3) = m246;
                let m243 = v2;
                match m243 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C15(r244, r245) => {
                        let v14 = (*r244).clone();
                        let v15 = (*r245).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v15, f64(v1.clone(), f59(v0.clone(), Adt0::C14, v14), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Adt27, v1: u64) -> Option<Adt0> {
    f62(match v0 {
        Adt27::C0(_, h249, _, _, _, _) => {
            h249
        }
    }, v1)
}

pub fn f45(v0: Adt29, v1: Adt0) -> Adt0 {
    let m250 = v0;
    match m250 {
        Adt29::C0 => {
            Adt0::C14
        }
        Adt29::C1(v2, _, v4, r251) => {
            let v5 = (*r251).clone();
            if f65(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f45(v5, v1)))
            } else {
                f45(v5, v1)
            }
        }
    }
}

pub fn f46(v0: Adt30) -> R<Option<Adt30>> {
    let v1: Adt30 = f66(v0.clone(), v0.clone(), v0.clone());
    let a252 = f67(v1.clone())?;
    let a253 = f67(v0)?;
    if nat_eq(a252, a253) {
        Ok(None::<Adt30>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f47(v0: Adt0) -> Option<Adt9> {
    let m254 = v0;
    match m254 {
        Adt0::C0 => {
            None::<Adt9>
        }
        Adt0::C1 => {
            None::<Adt9>
        }
        Adt0::C2 => {
            None::<Adt9>
        }
        Adt0::C3 => {
            None::<Adt9>
        }
        Adt0::C4(v1) => {
            Some(f51(v1))
        }
        Adt0::C5 => {
            None::<Adt9>
        }
        Adt0::C6 => {
            None::<Adt9>
        }
        Adt0::C7 => {
            None::<Adt9>
        }
        Adt0::C8(_) => {
            None::<Adt9>
        }
        Adt0::C9(_, _) => {
            None::<Adt9>
        }
        Adt0::C10(_) => {
            None::<Adt9>
        }
        Adt0::C11(_, _) => {
            None::<Adt9>
        }
        Adt0::C12(_) => {
            None::<Adt9>
        }
        Adt0::C13(_, _) => {
            None::<Adt9>
        }
        Adt0::C14 => {
            None::<Adt9>
        }
        Adt0::C15(_, _) => {
            None::<Adt9>
        }
    }
}

pub fn f48(v0: Bytes, v1: bool) -> R<Option<(bool, u64)>> {
    let m255 = f68(v0.clone(), 0u64);
    match m255 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v2) => {
            if if v1 {
                let a256 = v2;
                let a257 = 45u8;
                equal(a256, a257)
            } else {
                false
            } {
                let m258 = f69(v0, 1u64)?;
                match m258 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v3) => {
                        Ok(Some((true, v3)))
                    }
                }
            } else {
                let m259 = f69(v0, 0u64)?;
                match m259 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v4) => {
                        Ok(Some((false, v4)))
                    }
                }
            }
        }
    }
}

pub fn f49(v0: Adt0) -> Option<Adt1> {
    let m260 = v0;
    match m260 {
        Adt0::C0 => {
            None::<Adt1>
        }
        Adt0::C1 => {
            None::<Adt1>
        }
        Adt0::C2 => {
            None::<Adt1>
        }
        Adt0::C3 => {
            None::<Adt1>
        }
        Adt0::C4(v1) => {
            Some(v1)
        }
        Adt0::C5 => {
            None::<Adt1>
        }
        Adt0::C6 => {
            None::<Adt1>
        }
        Adt0::C7 => {
            None::<Adt1>
        }
        Adt0::C8(_) => {
            None::<Adt1>
        }
        Adt0::C9(_, _) => {
            None::<Adt1>
        }
        Adt0::C10(_) => {
            None::<Adt1>
        }
        Adt0::C11(_, _) => {
            None::<Adt1>
        }
        Adt0::C12(_) => {
            None::<Adt1>
        }
        Adt0::C13(_, _) => {
            None::<Adt1>
        }
        Adt0::C14 => {
            None::<Adt1>
        }
        Adt0::C15(_, _) => {
            None::<Adt1>
        }
    }
}

pub fn f50(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m261 = f48(v0, true)?;
    match m261 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            Ok(Some(({
                let (h262, _) = v1;
                if h262 {
                    let a264 = 0u64;
                    let a265 = {
                        let (_, h263) = v1;
                        h263
                    };
                    nat_lt(a264, a265)
                } else {
                    false
                }
            }, {
                let (_, h266) = v1;
                h266
            })))
        }
    }
}

pub fn f51(v0: Adt1) -> Adt9 {
    let m267 = v0;
    match m267 {
        Adt1::C0 => {
            Adt9::C0
        }
        Adt1::C1 => {
            Adt9::C1
        }
        Adt1::C2 => {
            Adt9::C2
        }
        Adt1::C3 => {
            Adt9::C3
        }
        Adt1::C4 => {
            Adt9::C4
        }
        Adt1::C5 => {
            Adt9::C5
        }
        Adt1::C6 => {
            Adt9::C6
        }
        Adt1::C7 => {
            Adt9::C7
        }
    }
}

pub fn f52(v0: Fn1, p272: u64, p273: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m271 = (p272, p273);
    {
        let mut m271 = m271;
        loop {
            let step = {
                let (v1, v2) = m271;
                let m268 = v1;
                if m268 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m268 - 1;
                    let m270 = {
                        let c269: Fn1 = v0.clone();
                        c269.apply(v2.clone())?
                    };
                    match m270 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f70(v0, v1)
}

pub fn f54(v0: Adt0) -> Option<Adt0> {
    let m274 = v0;
    match m274 {
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
        Adt0::C8(r275) => {
            let v2 = (*r275).clone();
            Some(v2)
        }
        Adt0::C9(r276, _) => {
            let v3 = (*r276).clone();
            Some(v3)
        }
        Adt0::C10(r277) => {
            let v5 = (*r277).clone();
            Some(v5)
        }
        Adt0::C11(r278, _) => {
            let v6 = (*r278).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r279, _) => {
            let v9 = (*r279).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r280, _) => {
            let v11 = (*r280).clone();
            Some(v11)
        }
    }
}

pub fn f55(p287: Adt0, v1: Adt0, p288: u64) -> R<u64> {
    let m286 = (p287, p288);
    {
        let mut m286 = m286;
        loop {
            let step = {
                let (v0, v2) = m286;
                let m281 = v0;
                match m281 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C15(r284, r285) => {
                        let v13 = (*r284).clone();
                        let v14 = (*r285).clone();
                        if f65(v13, v1.clone()) {
                            Err::<(Adt0, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(Adt0, u64), R<u64>>((v14, {
                                let a282 = v2;
                                let a283 = 1u64;
                                nat_add(a282, a283)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Adt0) -> Option<Adt0> {
    let m289 = v0;
    match m289 {
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
        Adt0::C9(_, r290) => {
            let v4 = (*r290).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r291) => {
            let v7 = (*r291).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r292) => {
            let v10 = (*r292).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r293) => {
            let v12 = (*r293).clone();
            Some(v12)
        }
    }
}

pub fn f57(p299: Adt0, p300: u64) -> Option<Adt0> {
    let m298 = (p299, p300);
    {
        let mut m298 = m298;
        loop {
            let step = {
                let (v0, v1) = m298;
                let m294 = v0;
                match m294 {
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
                    Adt0::C15(r296, r297) => {
                        let v12 = (*r296).clone();
                        let v13 = (*r297).clone();
                        let m295 = v1;
                        if m295 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m295 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Adt0, v1: Adt30, p305: Adt0) -> bool {
    let m304 = p305;
    {
        let mut m304 = m304;
        loop {
            let step = {
                let v2 = m304;
                let m301 = v2;
                match m301 {
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
                    Adt0::C15(r302, r303) => {
                        let v13 = (*r302).clone();
                        let v14 = (*r303).clone();
                        if f65(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f71(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
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

pub fn f59(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m306 = v2;
    match m306 {
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
        Adt0::C8(r307) => {
            let v4 = (*r307).clone();
            f59(v0, v1, v4)
        }
        Adt0::C9(r308, r309) => {
            let v5 = (*r308).clone();
            let v6 = (*r309).clone();
            f59(v0.clone(), f59(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r310, r311) => {
            let v8 = (*r310).clone();
            let v9 = (*r311).clone();
            f59(v0.clone(), f59(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f60(v1, Adt0::C12(v10))
        }
        Adt0::C13(r312, r313) => {
            let v11 = (*r312).clone();
            let v12 = (*r313).clone();
            if f72(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f60(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
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

pub fn f60(v0: Adt0, v1: Adt0) -> Adt0 {
    if f72(v0.clone(), v1.clone()) {
        v0
    } else {
        f3(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f61(v0: Adt27, v1: u64) -> Option<Adt0> {
    f62(match v0 {
        Adt27::C0(_, _, _, h314, _, _) => {
            h314
        }
    }, v1)
}

pub fn f62(p320: Adt0, p321: u64) -> Option<Adt0> {
    let m319 = (p320, p321);
    {
        let mut m319 = m319;
        loop {
            let step = {
                let (v0, v1) = m319;
                let m315 = v0;
                match m315 {
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
                    Adt0::C15(r317, r318) => {
                        let v12 = (*r317).clone();
                        let v13 = (*r318).clone();
                        let m316 = v1;
                        if m316 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m316 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(p327: Adt29, v1: Adt0, v2: u64) -> bool {
    let m326 = p327;
    {
        let mut m326 = m326;
        loop {
            let step = {
                let v0 = m326;
                let m322 = v0;
                match m322 {
                    Adt29::C0 => {
                        Err::<Adt29, bool>(false)
                    }
                    Adt29::C1(v3, v4, _, r325) => {
                        let v6 = (*r325).clone();
                        let a323 = v4;
                        let a324 = v2;
                        if nat_eq(a323, a324) {
                            if f65(v3, v1.clone()) {
                                Err::<Adt29, bool>(true)
                            } else {
                                Ok::<Adt29, bool>(v6)
                            }
                        } else {
                            Ok::<Adt29, bool>(v6)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m326 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt0, p332: Adt0, p333: Adt30) -> Adt30 {
    let m331 = (p332, p333);
    {
        let mut m331 = m331;
        loop {
            let step = {
                let (v1, v2) = m331;
                let m328 = v1;
                match m328 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C15(r329, r330) => {
                        let v13 = (*r329).clone();
                        let v14 = (*r330).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v14, f73(v2, v0.clone(), v13)))
                    }
                }
            };
            match step {
                Ok(next) => { m331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt0, v1: Adt0) -> bool {
    let m334 = v0;
    match m334 {
        Adt0::C0 => {
            let a335 = f74(v1);
            let a336 = 0u64;
            nat_eq(a335, a336)
        }
        Adt0::C1 => {
            let a337 = f74(v1);
            let a338 = 1u64;
            nat_eq(a337, a338)
        }
        Adt0::C2 => {
            let a339 = f74(v1);
            let a340 = 2u64;
            nat_eq(a339, a340)
        }
        Adt0::C3 => {
            let a341 = f74(v1);
            let a342 = 3u64;
            nat_eq(a341, a342)
        }
        Adt0::C4(v2) => {
            let a351 = f74(v1.clone());
            let a352 = 4u64;
            if nat_eq(a351, a352) {
                let a353 = f75(v2);
                let a354 = f75(f76(v1));
                nat_eq(a353, a354)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a343 = f74(v1);
            let a344 = 5u64;
            nat_eq(a343, a344)
        }
        Adt0::C6 => {
            let a345 = f74(v1);
            let a346 = 6u64;
            nat_eq(a345, a346)
        }
        Adt0::C7 => {
            let a347 = f74(v1);
            let a348 = 7u64;
            nat_eq(a347, a348)
        }
        Adt0::C8(r371) => {
            let v3 = (*r371).clone();
            let v4: bool = f65(v3, f28(v1.clone()));
            let a355 = f74(v1);
            let a356 = 8u64;
            if nat_eq(a355, a356) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r372, r373) => {
            let v5 = (*r372).clone();
            let v6 = (*r373).clone();
            let v7: bool = f65(v5, f28(v1.clone()));
            let v8: bool = f65(v6, f30(v1.clone()));
            let a357 = f74(v1);
            let a358 = 9u64;
            if nat_eq(a357, a358) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r374) => {
            let v9 = (*r374).clone();
            let v10: bool = f65(v9, f28(v1.clone()));
            let a359 = f74(v1);
            let a360 = 10u64;
            if nat_eq(a359, a360) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r375, r376) => {
            let v11 = (*r375).clone();
            let v12 = (*r376).clone();
            let v13: bool = f65(v11, f28(v1.clone()));
            let v14: bool = f65(v12, f30(v1.clone()));
            let a361 = f74(v1);
            let a362 = 11u64;
            if nat_eq(a361, a362) {
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
            let a363 = f74(v1.clone());
            let a364 = 12u64;
            if nat_eq(a363, a364) {
                let a365 = v15;
                let a366 = f31(v1);
                nat_eq(a365, a366)
            } else {
                false
            }
        }
        Adt0::C13(r377, r378) => {
            let v16 = (*r377).clone();
            let v17 = (*r378).clone();
            let v18: bool = f65(v16, f28(v1.clone()));
            let v19: bool = f65(v17, f30(v1.clone()));
            let a367 = f74(v1);
            let a368 = 13u64;
            if nat_eq(a367, a368) {
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
            let a349 = f74(v1);
            let a350 = 14u64;
            nat_eq(a349, a350)
        }
        Adt0::C15(r379, r380) => {
            let v20 = (*r379).clone();
            let v21 = (*r380).clone();
            let v22: bool = f65(v20, f28(v1.clone()));
            let v23: bool = f65(v21, f30(v1.clone()));
            let a369 = f74(v1);
            let a370 = 15u64;
            if nat_eq(a369, a370) {
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

pub fn f66(v0: Adt30, p384: Adt30, p385: Adt30) -> Adt30 {
    let m383 = (p384, p385);
    {
        let mut m383 = m383;
        loop {
            let step = {
                let (v1, v2) = m383;
                let m381 = v1;
                match m381 {
                    Adt30::C0 => {
                        Err::<(Adt30, Adt30), Adt30>(v2)
                    }
                    Adt30::C1(v3, v4, r382) => {
                        let v5 = (*r382).clone();
                        Ok::<(Adt30, Adt30), Adt30>((v5, f77(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt30) -> R<u64> {
    let m386 = v0;
    match m386 {
        Adt30::C0 => {
            Ok(0u64)
        }
        Adt30::C1(_, _, r389) => {
            let v3 = (*r389).clone();
            let a387 = 1u64;
            let a388 = f67(v3)?;
            nat_add(a387, a388)
        }
    }
}

pub fn f68(v0: Bytes, v1: u64) -> Option<u8> {
    let a390 = v0;
    let a391 = v1;
    index_bytes(a390, a391)
}

pub fn f69(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a392 = v0.clone();
        length_bytes(a392)
    };
    let a393 = v1;
    let a394 = v2;
    if nat_lt(a393, a394) {
        let v3: ((u64, u64), bool) = f78(Fn2::F79(v0), {
            let a395 = v2;
            let a396 = v1;
            nat_sub(a395, a396)
        }, (v1, 0u64))?;
        let a399 = {
            let (h397, _) = {
                let (h398, _) = v3;
                h398
            };
            h397
        };
        let a400 = v2;
        if nat_eq(a399, a400) {
            Ok(Some({
                let (_, h401) = {
                    let (h402, _) = v3;
                    h402
                };
                h401
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f70(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m404 = f68(v0.clone(), {
        let (h403, _) = v1.clone();
        h403
    });
    match m404 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            let m408 = f68(v0, {
                let a406 = {
                    let (h405, _) = v1.clone();
                    h405
                };
                let a407 = 1u64;
                nat_add(a406, a407)?
            });
            match m408 {
                None => {
                    Ok(None::<(u64, Bytes)>)
                }
                Some(v3) => {
                    let m409 = f80(v2, v3)?;
                    match m409 {
                        None => {
                            Ok(None::<(u64, Bytes)>)
                        }
                        Some(v4) => {
                            Ok(Some(({
                                let a411 = {
                                    let (h410, _) = v1;
                                    h410
                                };
                                let a412 = 2u64;
                                nat_add(a411, a412)?
                            }, {
                                let a414 = {
                                    let (_, h413) = v1;
                                    h413
                                };
                                let a415 = f81(f82(0u64), v4, 1u64);
                                append_bytes(a414, a415)
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f71(p419: Adt30, v1: Adt0, v2: Adt0) -> bool {
    let m418 = p419;
    {
        let mut m418 = m418;
        loop {
            let step = {
                let v0 = m418;
                let m416 = v0;
                match m416 {
                    Adt30::C0 => {
                        Err::<Adt30, bool>(false)
                    }
                    Adt30::C1(v3, v4, r417) => {
                        let v5 = (*r417).clone();
                        if f65(v3, v1.clone()) {
                            if f65(v4, v2.clone()) {
                                Err::<Adt30, bool>(true)
                            } else {
                                Ok::<Adt30, bool>(v5)
                            }
                        } else {
                            Ok::<Adt30, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(p424: Adt0, v1: Adt0) -> bool {
    let m423 = p424;
    {
        let mut m423 = m423;
        loop {
            let step = {
                let v0 = m423;
                let m420 = v0;
                match m420 {
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
                    Adt0::C15(r421, r422) => {
                        let v12 = (*r421).clone();
                        let v13 = (*r422).clone();
                        if f65(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: Adt30, v1: Adt0, v2: Adt0) -> Adt30 {
    if f71(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt30::C1(v1, v2, std::rc::Rc::new(v0))
    }
}

pub fn f74(v0: Adt0) -> u64 {
    let m425 = v0;
    match m425 {
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

pub fn f75(v0: Adt1) -> u64 {
    let m426 = v0;
    match m426 {
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

pub fn f76(v0: Adt0) -> Adt1 {
    let m427 = v0;
    match m427 {
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

pub fn f77(p431: Adt30, v1: Adt0, v2: Adt0, p432: Adt30) -> Adt30 {
    let m430 = (p431, p432);
    {
        let mut m430 = m430;
        loop {
            let step = {
                let (v0, v3) = m430;
                let m428 = v0;
                match m428 {
                    Adt30::C0 => {
                        Err::<(Adt30, Adt30), Adt30>(v3)
                    }
                    Adt30::C1(v4, v5, r429) => {
                        let v6 = (*r429).clone();
                        Ok::<(Adt30, Adt30), Adt30>((v6, if f65(v4, v1.clone()) {
                            f73(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m430 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Fn2, p437: u64, p438: (u64, u64)) -> R<((u64, u64), bool)> {
    let m436 = (p437, p438);
    {
        let mut m436 = m436;
        loop {
            let step = {
                let (v1, v2) = m436;
                let m433 = v1;
                if m433 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m433 - 1;
                    let m435 = {
                        let c434: Fn2 = v0.clone();
                        c434.apply(v2)?
                    };
                    match m435 {
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
                Ok(next) => { m436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f83(v0, v1)
}

pub fn f80(v0: u8, v1: u8) -> R<Option<u64>> {
    let m439 = f84(v0)?;
    match m439 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m440 = f84(v1)?;
            match m440 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    Ok(Some({
                        let a443 = {
                            let a441 = v2;
                            let a442 = 16u64;
                            nat_mul(a441, a442)?
                        };
                        let a444 = v3;
                        nat_add(a443, a444)?
                    }))
                }
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m448 = {
        let a445 = v0;
        let a446 = v1;
        let a447 = v2;
        slice_bytes(a445, a446, a447)
    };
    match m448 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f82(_: u64) -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f83(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m450 = f68(v0, {
        let (h449, _) = v1;
        h449
    });
    match m450 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let m451 = f85(v2)?;
            match m451 {
                None => {
                    Ok(None::<(u64, u64)>)
                }
                Some(v3) => {
                    Ok(Some(({
                        let a453 = {
                            let (h452, _) = v1;
                            h452
                        };
                        let a454 = 1u64;
                        nat_add(a453, a454)?
                    }, {
                        let a458 = {
                            let a456 = {
                                let (_, h455) = v1;
                                h455
                            };
                            let a457 = 10u64;
                            nat_mul(a456, a457)?
                        };
                        let a459 = v3;
                        nat_add(a458, a459)?
                    })))
                }
            }
        }
    }
}

pub fn f84(v0: u8) -> R<Option<u64>> {
    let m460 = f85(v0)?;
    match m460 {
        None => {
            let v3: u64 = {
                let v2: u8 = v0;
                let a505 = {
                    let a481 = {
                        let a469 = {
                            let a463 = {
                                let a461 = v2;
                                let a462 = 128u8;
                                fixed_u8::bit_and(a461, a462)
                            };
                            let a464 = 128u8;
                            if equal(a463, a464) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a470 = {
                            let a467 = {
                                let a465 = v2;
                                let a466 = 64u8;
                                fixed_u8::bit_and(a465, a466)
                            };
                            let a468 = 64u8;
                            if equal(a467, a468) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a469, a470)?
                    };
                    let a482 = {
                        let a479 = {
                            let a473 = {
                                let a471 = v2;
                                let a472 = 32u8;
                                fixed_u8::bit_and(a471, a472)
                            };
                            let a474 = 32u8;
                            if equal(a473, a474) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a480 = {
                            let a477 = {
                                let a475 = v2;
                                let a476 = 16u8;
                                fixed_u8::bit_and(a475, a476)
                            };
                            let a478 = 16u8;
                            if equal(a477, a478) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a479, a480)?
                    };
                    nat_add(a481, a482)?
                };
                let a506 = {
                    let a503 = {
                        let a491 = {
                            let a485 = {
                                let a483 = v2;
                                let a484 = 8u8;
                                fixed_u8::bit_and(a483, a484)
                            };
                            let a486 = 8u8;
                            if equal(a485, a486) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a492 = {
                            let a489 = {
                                let a487 = v2;
                                let a488 = 4u8;
                                fixed_u8::bit_and(a487, a488)
                            };
                            let a490 = 4u8;
                            if equal(a489, a490) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a491, a492)?
                    };
                    let a504 = {
                        let a501 = {
                            let a495 = {
                                let a493 = v2;
                                let a494 = 2u8;
                                fixed_u8::bit_and(a493, a494)
                            };
                            let a496 = 2u8;
                            if equal(a495, a496) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a502 = {
                            let a499 = {
                                let a497 = v2;
                                let a498 = 1u8;
                                fixed_u8::bit_and(a497, a498)
                            };
                            let a500 = 1u8;
                            if equal(a499, a500) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a501, a502)?
                    };
                    nat_add(a503, a504)?
                };
                nat_add(a505, a506)?
            };
            let a507 = v3;
            let a508 = 97u64;
            if nat_lt(a507, a508) {
                Ok(None::<u64>)
            } else {
                let a509 = v3;
                let a510 = 103u64;
                if nat_lt(a509, a510) {
                    Ok(Some({
                        let a511 = v3;
                        let a512 = 87u64;
                        nat_sub(a511, a512)
                    }))
                } else {
                    Ok(None::<u64>)
                }
            }
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f85(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a557 = {
            let a533 = {
                let a521 = {
                    let a515 = {
                        let a513 = v1;
                        let a514 = 128u8;
                        fixed_u8::bit_and(a513, a514)
                    };
                    let a516 = 128u8;
                    if equal(a515, a516) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a522 = {
                    let a519 = {
                        let a517 = v1;
                        let a518 = 64u8;
                        fixed_u8::bit_and(a517, a518)
                    };
                    let a520 = 64u8;
                    if equal(a519, a520) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a521, a522)?
            };
            let a534 = {
                let a531 = {
                    let a525 = {
                        let a523 = v1;
                        let a524 = 32u8;
                        fixed_u8::bit_and(a523, a524)
                    };
                    let a526 = 32u8;
                    if equal(a525, a526) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a532 = {
                    let a529 = {
                        let a527 = v1;
                        let a528 = 16u8;
                        fixed_u8::bit_and(a527, a528)
                    };
                    let a530 = 16u8;
                    if equal(a529, a530) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a531, a532)?
            };
            nat_add(a533, a534)?
        };
        let a558 = {
            let a555 = {
                let a543 = {
                    let a537 = {
                        let a535 = v1;
                        let a536 = 8u8;
                        fixed_u8::bit_and(a535, a536)
                    };
                    let a538 = 8u8;
                    if equal(a537, a538) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a544 = {
                    let a541 = {
                        let a539 = v1;
                        let a540 = 4u8;
                        fixed_u8::bit_and(a539, a540)
                    };
                    let a542 = 4u8;
                    if equal(a541, a542) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a543, a544)?
            };
            let a556 = {
                let a553 = {
                    let a547 = {
                        let a545 = v1;
                        let a546 = 2u8;
                        fixed_u8::bit_and(a545, a546)
                    };
                    let a548 = 2u8;
                    if equal(a547, a548) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a554 = {
                    let a551 = {
                        let a549 = v1;
                        let a550 = 1u8;
                        fixed_u8::bit_and(a549, a550)
                    };
                    let a552 = 1u8;
                    if equal(a551, a552) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a553, a554)?
            };
            nat_add(a555, a556)?
        };
        nat_add(a557, a558)?
    };
    let a559 = v2;
    let a560 = 48u64;
    if nat_lt(a559, a560) {
        Ok(None::<u64>)
    } else {
        let a561 = v2;
        let a562 = 58u64;
        if nat_lt(a561, a562) {
            Ok(Some({
                let a563 = v2;
                let a564 = 48u64;
                nat_sub(a563, a564)
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}
