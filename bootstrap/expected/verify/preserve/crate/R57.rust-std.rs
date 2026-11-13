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
    C0(u64, u64, u64, Option<(u64, u64)>),
}

#[derive(Clone)]
pub enum Fn0 {
    F11(Bytes, u64, Bytes, u64),
    F13(Bytes, Bytes),
    F21(Bytes),
    F22(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F11(k0, k1, k2, k3) => f11(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn0::F13(k0, k1) => Ok(f13(k0.clone(), k1.clone(), p0)),
            Fn0::F21(k0) => f21(k0.clone(), p0),
            Fn0::F22(k0, k1) => f22(*k0, k1.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Adt0>> {
    let v1: (u64, Option<Adt0>) = f1(v0.clone(), 0u64)?;
    let a2 = {
        let (h1, _) = v1;
        h1
    };
    let a3 = 0u64;
    if nat_eq(a2, a3) {
        let v2: (u64, Option<Adt0>) = f1(v0.clone(), 1u64)?;
        let a5 = {
            let (h4, _) = v2;
            h4
        };
        let a6 = 0u64;
        if nat_eq(a5, a6) {
            f2(v0, 2u64)
        } else {
            let (_, h7) = v2;
            Ok(h7)
        }
    } else {
        let (_, h8) = v1;
        Ok(h8)
    }
}

pub fn f1(v0: Bytes, v1: u64) -> R<(u64, Option<Adt0>)> {
    let v2: Bytes = f3(v1);
    let m9 = f4(v0.clone(), v2.clone())?;
    match m9 {
        None => {
            Ok((0u64, None::<Adt0>))
        }
        Some(v3) => {
            let v4: u64 = {
                let a11 = v3;
                let a12 = {
                    let a10 = v2;
                    length_bytes(a10)
                };
                nat_add(a11, a12)?
            };
            let m13 = f5(v0.clone(), f6(v1))?;
            match m13 {
                None => {
                    Ok((1u64, None::<Adt0>))
                }
                Some(v5) => {
                    let a14 = v5;
                    let a15 = v4;
                    if nat_lt(a14, a15) {
                        Ok((1u64, None::<Adt0>))
                    } else {
                        Ok((2u64, Some(Adt0::C0(v1, v4, v5, f7(v0, v1, v4, v5)?))))
                    }
                }
            }
        }
    }
}

pub fn f2(v0: Bytes, v1: u64) -> R<Option<Adt0>> {
    let v2: (u64, Option<Adt0>) = f1(v0, v1)?;
    let a17 = {
        let (h16, _) = v2;
        h16
    };
    let a18 = 0u64;
    if nat_eq(a17, a18) {
        Ok(None::<Adt0>)
    } else {
        let (_, h19) = v2;
        Ok(h19)
    }
}

pub fn f3(v0: u64) -> Bytes {
    let a22 = {
        let a20 = f8(Str::lit("\\begin{"));
        let a21 = f9(v0);
        append_bytes(a20, a21)
    };
    let a23 = f8(Str::lit("}"));
    append_bytes(a22, a23)
}

pub fn f4(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a24 = v0.clone();
        length_bytes(a24)
    };
    let v3: u64 = {
        let a25 = v1.clone();
        length_bytes(a25)
    };
    let v4: u64 = {
        let (h26, _) = f10(Fn0::F11(v1.clone(), v3, v0.clone(), v2), {
            let a27 = v2;
            let a28 = 1u64;
            nat_add(a27, a28)?
        }, 0u64)?;
        h26
    };
    if f12(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f5(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a29 = v0.clone();
        length_bytes(a29)
    };
    let v3: u64 = {
        let a30 = v1.clone();
        length_bytes(a30)
    };
    let a31 = v2;
    let a32 = v3;
    if nat_lt(a31, a32) {
        Ok(None::<u64>)
    } else {
        let v4: u64 = {
            let (h33, _) = f10(Fn0::F13(v1.clone(), v0.clone()), {
                let a34 = v2;
                let a35 = 1u64;
                nat_add(a34, a35)?
            }, {
                let a36 = v2;
                let a37 = v3;
                nat_sub(a36, a37)
            })?;
            h33
        };
        if f12(v0, v1, v4) {
            Ok(Some(v4))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f6(v0: u64) -> Bytes {
    let a40 = {
        let a38 = f8(Str::lit("\\end{"));
        let a39 = f9(v0);
        append_bytes(a38, a39)
    };
    let a41 = f8(Str::lit("}"));
    append_bytes(a40, a41)
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(u64, u64)>> {
    let v4: Bytes = f14(v0, v2, {
        let a42 = v3;
        let a43 = v2;
        nat_sub(a42, a43)
    });
    let v5: u64 = f15(v4.clone())?;
    let v6: Bytes = f16(v1);
    if f17(f18(v4.clone(), v5), v6.clone()) {
        let v7: u64 = {
            let a45 = v5;
            let a46 = {
                let a44 = v6;
                length_bytes(a44)
            };
            nat_add(a45, a46)?
        };
        let v8: u64 = {
            let a47 = v7;
            let a48 = f15(f18(v4.clone(), v7))?;
            nat_add(a47, a48)?
        };
        if f17(f18(v4.clone(), v8), f8(Str::lit("{"))) {
            let v9: Bytes = f18(v4, {
                let a49 = v8;
                let a50 = 1u64;
                nat_add(a49, a50)?
            });
            let v10: u64 = f19(v9.clone(), 0u64)?;
            let a51 = 0u64;
            let a52 = v10;
            if if nat_lt(a51, a52) {
                f12(v9, f8(Str::lit("}")), {
                    let a53 = v10;
                    let a54 = 1u64;
                    nat_sub(a53, a54)
                })
            } else {
                false
            } {
                Ok(Some(({
                    let a57 = {
                        let a55 = v2;
                        let a56 = v8;
                        nat_add(a55, a56)?
                    };
                    let a58 = 1u64;
                    nat_add(a57, a58)?
                }, {
                    let a65 = {
                        let a61 = {
                            let a59 = v2;
                            let a60 = v8;
                            nat_add(a59, a60)?
                        };
                        let a62 = 1u64;
                        nat_add(a61, a62)?
                    };
                    let a66 = {
                        let a63 = v10;
                        let a64 = 1u64;
                        nat_sub(a63, a64)
                    };
                    nat_add(a65, a66)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        } else {
            Ok(None::<(u64, u64)>)
        }
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f8(v0: Str) -> Bytes {
    let a67 = v0;
    utf8_encode(a67)
}

pub fn f9(v0: u64) -> Bytes {
    let a68 = v0;
    let a69 = 0u64;
    if nat_eq(a68, a69) {
        f8(Str::lit("semanticmodule"))
    } else {
        let a70 = v0;
        let a71 = 1u64;
        if nat_eq(a70, a71) {
            f8(Str::lit("toolingmodule"))
        } else {
            f8(Str::lit("coremodule"))
        }
    }
}

pub fn f10(v0: Fn0, p76: u64, p77: u64) -> R<(u64, bool)> {
    let m75 = (p76, p77);
    {
        let mut m75 = m75;
        loop {
            let step = {
                let (v1, v2) = m75;
                let m72 = v1;
                if m72 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m72 - 1;
                    let m74 = {
                        let c73: Fn0 = v0.clone();
                        c73.apply(v2)?
                    };
                    match m74 {
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
                Ok(next) => { m75 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f12(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a80 = {
            let a78 = v4;
            let a79 = v1;
            nat_add(a78, a79)?
        };
        let a81 = v3;
        if nat_lt(a80, a81) {
            Ok(Some({
                let a82 = v4;
                let a83 = 1u64;
                nat_add(a82, a83)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f12(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m88 = {
        let a85 = v0;
        let a86 = v2;
        let a87 = {
            let a84 = v1.clone();
            length_bytes(a84)
        };
        slice_bytes(a85, a86, a87)
    };
    match m88 {
        None => {
            false
        }
        Some(v3) => {
            let a89 = v3;
            let a90 = v1;
            equal(a89, a90)
        }
    }
}

pub fn f13(v0: Bytes, v1: Bytes, v2: u64) -> Option<u64> {
    f20(v1, v0, v2)
}

pub fn f14(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m94 = {
        let a91 = v0;
        let a92 = v1;
        let a93 = v2;
        slice_bytes(a91, a92, a93)
    };
    match m94 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f15(v0: Bytes) -> R<u64> {
    let (h95, _) = f10(Fn0::F21(v0.clone()), {
        let a96 = v0;
        length_bytes(a96)
    }, 0u64)?;
    Ok(h95)
}

pub fn f16(v0: u64) -> Bytes {
    let a97 = v0;
    let a98 = 0u64;
    if nat_eq(a97, a98) {
        f8(Str::lit("\\semanticdata"))
    } else {
        let a99 = v0;
        let a100 = 1u64;
        if nat_eq(a99, a100) {
            f8(Str::lit("\\toolingdata"))
        } else {
            f8(Str::lit("\\coredata"))
        }
    }
}

pub fn f17(v0: Bytes, v1: Bytes) -> bool {
    let m105 = {
        let a102 = v0;
        let a103 = 0u64;
        let a104 = {
            let a101 = v1.clone();
            length_bytes(a101)
        };
        slice_bytes(a102, a103, a104)
    };
    match m105 {
        None => {
            false
        }
        Some(v2) => {
            let a106 = v2;
            let a107 = v1;
            equal(a106, a107)
        }
    }
}

pub fn f18(v0: Bytes, v1: u64) -> Bytes {
    f14(v0.clone(), v1, {
        let a109 = {
            let a108 = v0;
            length_bytes(a108)
        };
        let a110 = v1;
        nat_sub(a109, a110)
    })
}

pub fn f19(v0: Bytes, v1: u64) -> R<u64> {
    let (h111, _) = f10(Fn0::F22(v1, v0.clone()), {
        let a112 = v0.clone();
        length_bytes(a112)
    }, {
        let a113 = v0;
        length_bytes(a113)
    })?;
    Ok(h111)
}

pub fn f20(v0: Bytes, v1: Bytes, v2: u64) -> Option<u64> {
    if f12(v0, v1, v2) {
        None::<u64>
    } else {
        let a114 = 0u64;
        let a115 = v2;
        if nat_lt(a114, a115) {
            Some({
                let a116 = v2;
                let a117 = 1u64;
                nat_sub(a116, a117)
            })
        } else {
            None::<u64>
        }
    }
}

pub fn f21(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a118 = 0u64;
    let a119 = f23(v0.clone(), v1)?;
    if nat_lt(a118, a119) {
        Ok(Some({
            let a120 = v1;
            let a121 = f23(v0, v1)?;
            nat_add(a120, a121)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f22(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a122 = 0u64;
    let a123 = f24(v1.clone(), v2)?;
    if if nat_lt(a122, a123) {
        let a126 = {
            let a124 = v0;
            let a125 = f24(v1.clone(), v2)?;
            nat_add(a124, a125)?
        };
        let a127 = v2;
        nat_le(a126, a127)
    } else {
        false
    } {
        Ok(Some({
            let a128 = v2;
            let a129 = f24(v1, v2)?;
            nat_sub(a128, a129)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f23(v0: Bytes, v1: u64) -> R<u64> {
    if if f25(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f26(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f26(v0.clone(), v1, 194u8) {
            if if f26(v0.clone(), {
                let a130 = v1;
                let a131 = 1u64;
                nat_add(a130, a131)?
            }, 133u8) {
                true
            } else {
                f26(v0, {
                    let a132 = v1;
                    let a133 = 1u64;
                    nat_add(a132, a133)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f26(v0.clone(), v1, 225u8) {
                if if f26(v0.clone(), {
                    let a134 = v1;
                    let a135 = 1u64;
                    nat_add(a134, a135)?
                }, 154u8) {
                    f26(v0, {
                        let a136 = v1;
                        let a137 = 2u64;
                        nat_add(a136, a137)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f26(v0.clone(), v1, 226u8) {
                    if f26(v0.clone(), {
                        let a138 = v1;
                        let a139 = 1u64;
                        nat_add(a138, a139)?
                    }, 128u8) {
                        if if f25(v0.clone(), {
                            let a140 = v1;
                            let a141 = 2u64;
                            nat_add(a140, a141)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f26(v0.clone(), {
                                let a142 = v1;
                                let a143 = 2u64;
                                nat_add(a142, a143)?
                            }, 168u8) {
                                true
                            } else {
                                if f26(v0.clone(), {
                                    let a144 = v1;
                                    let a145 = 2u64;
                                    nat_add(a144, a145)?
                                }, 169u8) {
                                    true
                                } else {
                                    f26(v0, {
                                        let a146 = v1;
                                        let a147 = 2u64;
                                        nat_add(a146, a147)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f26(v0.clone(), {
                            let a148 = v1;
                            let a149 = 1u64;
                            nat_add(a148, a149)?
                        }, 129u8) {
                            f26(v0, {
                                let a150 = v1;
                                let a151 = 2u64;
                                nat_add(a150, a151)?
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
                    if f26(v0.clone(), v1, 227u8) {
                        if if f26(v0.clone(), {
                            let a152 = v1;
                            let a153 = 1u64;
                            nat_add(a152, a153)?
                        }, 128u8) {
                            f26(v0, {
                                let a154 = v1;
                                let a155 = 2u64;
                                nat_add(a154, a155)?
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

pub fn f24(v0: Bytes, v1: u64) -> R<u64> {
    let a156 = v1;
    let a157 = 0u64;
    if nat_eq(a156, a157) {
        Ok(0u64)
    } else {
        let a160 = f23(v0.clone(), {
            let a158 = v1;
            let a159 = 1u64;
            nat_sub(a158, a159)
        })?;
        let a161 = 1u64;
        if nat_eq(a160, a161) {
            Ok(1u64)
        } else {
            let a162 = 2u64;
            let a163 = v1;
            if if nat_le(a162, a163) {
                let a166 = f23(v0.clone(), {
                    let a164 = v1;
                    let a165 = 2u64;
                    nat_sub(a164, a165)
                })?;
                let a167 = 2u64;
                nat_eq(a166, a167)
            } else {
                false
            } {
                Ok(2u64)
            } else {
                let a168 = 3u64;
                let a169 = v1;
                if if nat_le(a168, a169) {
                    let a172 = f23(v0, {
                        let a170 = v1;
                        let a171 = 3u64;
                        nat_sub(a170, a171)
                    })?;
                    let a173 = 3u64;
                    nat_eq(a172, a173)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            }
        }
    }
}

pub fn f25(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m174 = f27(v0, v1);
    match m174 {
        None => {
            false
        }
        Some(v4) => {
            f28(v4, v2, v3)
        }
    }
}

pub fn f26(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m175 = f27(v0, v1);
    match m175 {
        None => {
            false
        }
        Some(v3) => {
            let a176 = v3;
            let a177 = v2;
            equal(a176, a177)
        }
    }
}

pub fn f27(v0: Bytes, v1: u64) -> Option<u8> {
    let a178 = v0;
    let a179 = v1;
    index_bytes(a178, a179)
}

pub fn f28(v0: u8, v1: u8, v2: u8) -> bool {
    let m182 = {
        let a180 = v0;
        let a181 = v1;
        fixed_u8::checked_sub(a180, a181)
    };
    match m182 {
        None => {
            false
        }
        Some(_) => {
            let m185 = {
                let a183 = v2;
                let a184 = v0;
                fixed_u8::checked_sub(a183, a184)
            };
            match m185 {
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
