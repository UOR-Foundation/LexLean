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
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, List<(u64, u64)>, Option<Adt0>),
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
    C0(u64, List<Adt2>, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, List<Bytes>, bool, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64, List<Adt7>, Adt8, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(List<Adt8>),
    C1(Bytes, u64, List<(Bytes, Adt8)>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt8>),
    C4(std::rc::Rc<Adt8>),
    C5,
    C6(std::rc::Rc<Adt8>, std::rc::Rc<Adt8>),
    C7(Bytes, std::rc::Rc<Adt8>, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3(u64, List<Adt2>),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F3(k0, k1) => f3(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F8(u64, Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn1::F8(k0, k1) => f8(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F12,
}

impl Fn2 {
    pub fn apply(&self, p0: List<(u64, u64)>, p1: (u64, u64)) -> List<(u64, u64)> {
        match self {
            Fn2::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F15,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn3::F15 => f15(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F46(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn4::F46(k0, k1) => f46(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F50(u64, Bytes),
    F73(u64, Bytes),
    F131(Bytes),
    F132(Bytes),
    F136(Bytes),
    F151(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F50(k0, k1) => f50(*k0, k1.clone(), p0),
            Fn5::F73(k0, k1) => f73(*k0, k1.clone(), p0),
            Fn5::F131(k0) => f131(k0.clone(), p0),
            Fn5::F132(k0) => f132(k0.clone(), p0),
            Fn5::F136(k0) => f136(k0.clone(), p0),
            Fn5::F151(k0) => f151(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F53(u64, List<Adt2>),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn6::F53(k0, k1) => f53(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F56,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn7::F56 => f56(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F58,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn8::F58 => f58(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F75,
}

impl Fn9 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn9::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F110(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt8)) -> bool {
        match self {
            Fn10::F110(k0) => f110(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F122,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt8, p1: Adt8) -> Adt8 {
        match self {
            Fn11::F122 => f122(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F124,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt8, p1: (Bytes, Adt8)) -> Adt8 {
        match self {
            Fn12::F124 => f124(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F128,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>, p1: (Bytes, Adt8)) -> List<List<(Bytes, Adt8)>> {
        match self {
            Fn13::F128 => f128(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F130,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
        match self {
            Fn14::F130 => f130(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F135(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn15::F135(k0) => f135(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F138(u64, Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn16::F138(k0, k1) => f138(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F140,
}

impl Fn17 {
    pub fn apply(&self, p0: List<(Bytes, Adt8)>, p1: (Bytes, Adt8)) -> List<(Bytes, Adt8)> {
        match self {
            Fn17::F140 => f140(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F142,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>, p1: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
        match self {
            Fn18::F142 => f142(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F156,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
        match self {
            Fn19::F156 => f156(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F162,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
        match self {
            Fn20::F162 => f162(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Result<List<(u64, u64)>, Adt0>> {
    let m1 = f1(v0.clone())?;
    match m1 {
        Ok(v2) => {
            let v3: u64 = {
                let a2 = v0;
                length_bytes(a2)
            };
            let v4: Adt1 = {
                let (h3, _) = f2(Fn0::F3(v3, v2.clone()), {
                    let a7 = {
                        let a5 = {
                            let a4 = v2;
                            length_list(a4)
                        };
                        let a6 = 2u64;
                        nat_mul(a5, a6)?
                    };
                    let a8 = 4u64;
                    nat_add(a7, a8)?
                }, Adt1::C0(0u64, 0u64, List::<(u64, u64)>::nil(), None::<Adt0>))?;
                h3
            };
            let m10 = match v4.clone() {
                Adt1::C0(_, _, _, h9) => {
                    h9
                }
            };
            match m10 {
                None => {
                    let a12 = match v4.clone() {
                        Adt1::C0(_, h11, _, _) => {
                            h11
                        }
                    };
                    let a13 = 8u64;
                    if nat_eq(a12, a13) {
                        Ok(Ok::<List<(u64, u64)>, Adt0>(f4(match v4 {
                            Adt1::C0(_, _, h14, _) => {
                                h14
                            }
                        })))
                    } else {
                        Ok(Err::<List<(u64, u64)>, Adt0>(f5(v3, f6(Str::lit("the reader ran out of steps")))))
                    }
                }
                Some(v5) => {
                    Ok(Err::<List<(u64, u64)>, Adt0>(v5))
                }
            }
        }
        Err(v1) => {
            Ok(Err::<List<(u64, u64)>, Adt0>(v1))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt2>, Adt0>> {
    let v1: u64 = {
        let a15 = v0.clone();
        length_bytes(a15)
    };
    let v2: Adt4 = {
        let (h16, _) = f7(Fn1::F8(v1, v0), {
            let a17 = v1;
            let a18 = 1u64;
            nat_add(a17, a18)?
        }, Adt4::C0(0u64, List::<Adt2>::nil(), None::<Adt0>))?;
        h16
    };
    let m20 = match v2.clone() {
        Adt4::C0(_, _, h19) => {
            h19
        }
    };
    match m20 {
        None => {
            Ok(Ok::<List<Adt2>, Adt0>(f9(match v2 {
                Adt4::C0(_, h21, _) => {
                    h21
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt2>, Adt0>(v3))
        }
    }
}

pub fn f2(v0: Fn0, p26: u64, p27: Adt1) -> R<(Adt1, bool)> {
    let m25 = (p26, p27);
    {
        let mut m25 = m25;
        loop {
            let step = {
                let (v1, v2) = m25;
                let m22 = v1;
                if m22 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m22 - 1;
                    let m24 = {
                        let c23: Fn0 = v0.clone();
                        c23.apply(v2.clone())?
                    };
                    match m24 {
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
                Ok(next) => { m25 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: u64, v1: List<Adt2>, v2: Adt1) -> R<Option<Adt1>> {
    f10(v1, v0, v2)
}

pub fn f4(v0: List<(u64, u64)>) -> List<(u64, u64)> {
    f11(Fn2::F12, List::<(u64, u64)>::nil(), v0)
}

pub fn f5(v0: u64, v1: Bytes) -> Adt0 {
    Adt0::C0(v0, v1)
}

pub fn f6(v0: Str) -> Bytes {
    let a28 = v0;
    utf8_encode(a28)
}

pub fn f7(v0: Fn1, p33: u64, p34: Adt4) -> R<(Adt4, bool)> {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v1, v2) = m32;
                let m29 = v1;
                if m29 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m29 - 1;
                    let m31 = {
                        let c30: Fn1 = v0.clone();
                        c30.apply(v2.clone())?
                    };
                    match m31 {
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
                Ok(next) => { m32 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f13(v1, v0, v2)
}

pub fn f9(v0: List<Adt2>) -> List<Adt2> {
    f14(Fn3::F15, List::<Adt2>::nil(), v0)
}

pub fn f10(v0: List<Adt2>, v1: u64, v2: Adt1) -> R<Option<Adt1>> {
    let a36 = match v2.clone() {
        Adt1::C0(_, h35, _, _) => {
            h35
        }
    };
    let a37 = 8u64;
    if nat_lt(a36, a37) {
        Ok(Some(f16(v0, v1, v2)?))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f11(v0: Fn2, p41: List<(u64, u64)>, p42: List<(u64, u64)>) -> List<(u64, u64)> {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v1, v2) = m40;
                let m38 = v2;
                match m38.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>(({
                            let c39: Fn2 = v0.clone();
                            c39.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m40 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: List<(u64, u64)>, v1: (u64, u64)) -> List<(u64, u64)> {
    List::cons(v1, v0)
}

pub fn f13(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m44 = match v2.clone() {
        Adt4::C0(_, _, h43) => {
            h43
        }
    };
    match m44 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt4::C0(h45, _, _) => {
                    h45
                }
            };
            let a46 = v4;
            let a47 = v1;
            if nat_lt(a46, a47) {
                let v5: u64 = f17(v0.clone(), v4)?;
                let a48 = 0u64;
                let a49 = v5;
                if nat_lt(a48, a49) {
                    Ok(Some(Adt4::C0({
                        let a50 = v4;
                        let a51 = v5;
                        nat_add(a50, a51)?
                    }, match v2 {
                        Adt4::C0(_, h52, _) => {
                            h52
                        }
                    }, None::<Adt0>)))
                } else {
                    if f18(v0.clone(), v4, 123u8) {
                        Ok(f19(v2, Adt3::C0, {
                            let a53 = v4;
                            let a54 = 1u64;
                            nat_add(a53, a54)?
                        }))
                    } else {
                        if f18(v0.clone(), v4, 125u8) {
                            Ok(f19(v2, Adt3::C1, {
                                let a55 = v4;
                                let a56 = 1u64;
                                nat_add(a55, a56)?
                            }))
                        } else {
                            if f18(v0.clone(), v4, 91u8) {
                                Ok(f19(v2, Adt3::C2, {
                                    let a57 = v4;
                                    let a58 = 1u64;
                                    nat_add(a57, a58)?
                                }))
                            } else {
                                if f18(v0.clone(), v4, 93u8) {
                                    Ok(f19(v2, Adt3::C3, {
                                        let a59 = v4;
                                        let a60 = 1u64;
                                        nat_add(a59, a60)?
                                    }))
                                } else {
                                    if f18(v0.clone(), v4, 40u8) {
                                        Ok(f19(v2, Adt3::C4, {
                                            let a61 = v4;
                                            let a62 = 1u64;
                                            nat_add(a61, a62)?
                                        }))
                                    } else {
                                        if f18(v0.clone(), v4, 41u8) {
                                            Ok(f19(v2, Adt3::C5, {
                                                let a63 = v4;
                                                let a64 = 1u64;
                                                nat_add(a63, a64)?
                                            }))
                                        } else {
                                            if f18(v0.clone(), v4, 44u8) {
                                                Ok(f19(v2, Adt3::C6, {
                                                    let a65 = v4;
                                                    let a66 = 1u64;
                                                    nat_add(a65, a66)?
                                                }))
                                            } else {
                                                if f18(v0.clone(), v4, 59u8) {
                                                    Ok(f19(v2, Adt3::C10, {
                                                        let a67 = v4;
                                                        let a68 = 1u64;
                                                        nat_add(a67, a68)?
                                                    }))
                                                } else {
                                                    if f18(v0.clone(), v4, 58u8) {
                                                        f20(v0, v2)
                                                    } else {
                                                        if f18(v0.clone(), v4, 34u8) {
                                                            f21(v0, v1, v2)
                                                        } else {
                                                            f22(v0, v1, v2)
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

pub fn f14(v0: Fn3, p72: List<Adt2>, p73: List<Adt2>) -> List<Adt2> {
    let m71 = (p72, p73);
    {
        let mut m71 = m71;
        loop {
            let step = {
                let (v1, v2) = m71;
                let m69 = v2;
                match m69.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c70: Fn3 = v0.clone();
                            c70.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m71 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f16(v0: List<Adt2>, v1: u64, v2: Adt1) -> R<Adt1> {
    let v3: u64 = match v2.clone() {
        Adt1::C0(_, h74, _, _) => {
            h74
        }
    };
    let v4: u64 = match v2.clone() {
        Adt1::C0(h75, _, _, _) => {
            h75
        }
    };
    let a76 = v3;
    let a77 = 0u64;
    if nat_eq(a76, a77) {
        let a78 = f23(v0.clone(), v4);
        let a79 = 0u64;
        if nat_eq(a78, a79) {
            Ok(f24(v2, {
                let a80 = v4;
                let a81 = 1u64;
                nat_add(a80, a81)?
            }, 1u64))
        } else {
            Ok(f25(v2, f26(v0, v1, v4), f6(Str::lit("expected `{`"))))
        }
    } else {
        let a82 = v3;
        let a83 = 1u64;
        if nat_eq(a82, a83) {
            f27(v0, v1, v2)
        } else {
            let a84 = v3;
            let a85 = 2u64;
            if nat_eq(a84, a85) {
                let a86 = f23(v0.clone(), v4);
                let a87 = 3u64;
                if nat_eq(a86, a87) {
                    Ok(f24(v2, v4, 3u64))
                } else {
                    f28(v0, v1, v2, 2u64, true)
                }
            } else {
                let a88 = v3;
                let a89 = 3u64;
                if nat_eq(a88, a89) {
                    let a90 = f23(v0.clone(), v4);
                    let a91 = 3u64;
                    if nat_eq(a90, a91) {
                        Ok(f24(v2, {
                            let a92 = v4;
                            let a93 = 1u64;
                            nat_add(a92, a93)?
                        }, 4u64))
                    } else {
                        Ok(f25(v2, f26(v0, v1, v4), f6(Str::lit("expected `,` or `]`"))))
                    }
                } else {
                    let a94 = v3;
                    let a95 = 4u64;
                    if nat_eq(a94, a95) {
                        let a96 = f23(v0, v4);
                        let a97 = 6u64;
                        if nat_eq(a96, a97) {
                            Ok(f24(v2, {
                                let a98 = v4;
                                let a99 = 1u64;
                                nat_add(a98, a99)?
                            }, 1u64))
                        } else {
                            Ok(f24(v2, v4, 5u64))
                        }
                    } else {
                        f29(v0, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: u64) -> R<u64> {
    if if f30(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f31(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f31(v0.clone(), v1, 194u8) {
            if if f31(v0.clone(), {
                let a100 = v1;
                let a101 = 1u64;
                nat_add(a100, a101)?
            }, 133u8) {
                true
            } else {
                f31(v0, {
                    let a102 = v1;
                    let a103 = 1u64;
                    nat_add(a102, a103)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f31(v0.clone(), v1, 225u8) {
                if if f31(v0.clone(), {
                    let a104 = v1;
                    let a105 = 1u64;
                    nat_add(a104, a105)?
                }, 154u8) {
                    f31(v0, {
                        let a106 = v1;
                        let a107 = 2u64;
                        nat_add(a106, a107)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f31(v0.clone(), v1, 226u8) {
                    if f31(v0.clone(), {
                        let a108 = v1;
                        let a109 = 1u64;
                        nat_add(a108, a109)?
                    }, 128u8) {
                        if if f30(v0.clone(), {
                            let a110 = v1;
                            let a111 = 2u64;
                            nat_add(a110, a111)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f31(v0.clone(), {
                                let a112 = v1;
                                let a113 = 2u64;
                                nat_add(a112, a113)?
                            }, 168u8) {
                                true
                            } else {
                                if f31(v0.clone(), {
                                    let a114 = v1;
                                    let a115 = 2u64;
                                    nat_add(a114, a115)?
                                }, 169u8) {
                                    true
                                } else {
                                    f31(v0, {
                                        let a116 = v1;
                                        let a117 = 2u64;
                                        nat_add(a116, a117)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f31(v0.clone(), {
                            let a118 = v1;
                            let a119 = 1u64;
                            nat_add(a118, a119)?
                        }, 129u8) {
                            f31(v0, {
                                let a120 = v1;
                                let a121 = 2u64;
                                nat_add(a120, a121)?
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
                    if f31(v0.clone(), v1, 227u8) {
                        if if f31(v0.clone(), {
                            let a122 = v1;
                            let a123 = 1u64;
                            nat_add(a122, a123)?
                        }, 128u8) {
                            f31(v0, {
                                let a124 = v1;
                                let a125 = 2u64;
                                nat_add(a124, a125)?
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

pub fn f18(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m126 = f32(v0, v1);
    match m126 {
        None => {
            false
        }
        Some(v3) => {
            let a127 = v3;
            let a128 = v2;
            equal(a127, a128)
        }
    }
}

pub fn f19(v0: Adt4, v1: Adt3, v2: u64) -> Option<Adt4> {
    Some(Adt4::C0(v2, List::cons(Adt2::C0(v1, match v0.clone() {
        Adt4::C0(h129, _, _) => {
            h129
        }
    }, v2), match v0 {
        Adt4::C0(_, h130, _) => {
            h130
        }
    }), None::<Adt0>))
}

pub fn f20(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    let v2: u64 = match v1.clone() {
        Adt4::C0(h131, _, _) => {
            h131
        }
    };
    if f18(v0.clone(), {
        let a132 = v2;
        let a133 = 1u64;
        nat_add(a132, a133)?
    }, 61u8) {
        Ok(f19(v1, Adt3::C8, {
            let a134 = v2;
            let a135 = 2u64;
            nat_add(a134, a135)?
        }))
    } else {
        if f18(v0, {
            let a136 = v2;
            let a137 = 1u64;
            nat_add(a136, a137)?
        }, 58u8) {
            Ok(f19(v1, Adt3::C9, {
                let a138 = v2;
                let a139 = 2u64;
                nat_add(a138, a139)?
            }))
        } else {
            Ok(f19(v1, Adt3::C7, {
                let a140 = v2;
                let a141 = 1u64;
                nat_add(a140, a141)?
            }))
        }
    }
}

pub fn f21(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h142, _, _) => {
            h142
        }
    };
    let v4: Adt5 = f33(v0, v1, v3)?;
    let m144 = match v4.clone() {
        Adt5::C0(_, _, _, h143) => {
            h143
        }
    };
    match m144 {
        None => {
            if match v4.clone() {
                Adt5::C0(_, _, h146, _) => {
                    h146
                }
            } {
                Ok(f19(v2, Adt3::C11(f34(match v4.clone() {
                    Adt5::C0(_, h147, _, _) => {
                        h147
                    }
                })), match v4 {
                    Adt5::C0(h148, _, _, _) => {
                        h148
                    }
                }))
            } else {
                Ok(Some(Adt4::C0(v3, match v2 {
                    Adt4::C0(_, h149, _) => {
                        h149
                    }
                }, Some(f35(v3, f36(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt4::C0(v3, match v2 {
                Adt4::C0(_, h145, _) => {
                    h145
                }
            }, Some(v5))))
        }
    }
}

pub fn f22(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h150, _, _) => {
            h150
        }
    };
    let v4: u64 = f37(v0.clone(), v1, v3)?;
    Ok(f19(v2, Adt3::C12(f38(v0, v3, {
        let a151 = v4;
        let a152 = v3;
        nat_sub(a151, a152)
    })), v4))
}

pub fn f23(v0: List<Adt2>, v1: u64) -> u64 {
    let m153 = f39(v0, v1);
    match m153 {
        None => {
            99u64
        }
        Some(v2) => {
            f40(match v2 {
                Adt2::C0(h154, _, _) => {
                    h154
                }
            })
        }
    }
}

pub fn f24(v0: Adt1, v1: u64, v2: u64) -> Adt1 {
    Adt1::C0(v1, v2, match v0 {
        Adt1::C0(_, _, h155, _) => {
            h155
        }
    }, None::<Adt0>)
}

pub fn f25(v0: Adt1, v1: u64, v2: Bytes) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h156, _, _, _) => {
            h156
        }
    }, 9u64, match v0 {
        Adt1::C0(_, _, h157, _) => {
            h157
        }
    }, Some(f5(v1, v2)))
}

pub fn f26(v0: List<Adt2>, v1: u64, v2: u64) -> u64 {
    let m158 = f39(v0, v2);
    match m158 {
        None => {
            v1
        }
        Some(v3) => {
            match v3 {
                Adt2::C0(_, h159, _) => {
                    h159
                }
            }
        }
    }
}

pub fn f27(v0: List<Adt2>, v1: u64, v2: Adt1) -> R<Adt1> {
    let v3: u64 = match v2.clone() {
        Adt1::C0(h160, _, _, _) => {
            h160
        }
    };
    let v4: u64 = f23(v0.clone(), v3);
    let a161 = v4;
    let a162 = 1u64;
    if nat_eq(a161, a162) {
        Ok(f24(v2, v3, 5u64))
    } else {
        let a163 = v4;
        let a164 = 11u64;
        if if nat_eq(a163, a164) {
            true
        } else {
            let a165 = v4;
            let a166 = 12u64;
            nat_eq(a165, a166)
        } {
            let a169 = f23(v0.clone(), {
                let a167 = v3;
                let a168 = 1u64;
                nat_add(a167, a168)?
            });
            let a170 = 7u64;
            if nat_eq(a169, a170) {
                let a171 = f41(v0.clone(), v3);
                let a172 = f6(Str::lit("declarations"));
                if equal(a171, a172) {
                    let a175 = f23(v0.clone(), {
                        let a173 = v3;
                        let a174 = 2u64;
                        nat_add(a173, a174)?
                    });
                    let a176 = 2u64;
                    if nat_eq(a175, a176) {
                        Ok(f24(v2, {
                            let a177 = v3;
                            let a178 = 3u64;
                            nat_add(a177, a178)?
                        }, 2u64))
                    } else {
                        Ok(f25(v2, f26(v0, v1, {
                            let a179 = v3;
                            let a180 = 2u64;
                            nat_add(a179, a180)?
                        }), f6(Str::lit("expected `[` after `declarations`"))))
                    }
                } else {
                    f28(v0, v1, f24(v2, {
                        let a181 = v3;
                        let a182 = 2u64;
                        nat_add(a181, a182)?
                    }, 1u64), 1u64, false)
                }
            } else {
                Ok(f25(v2, f26(v0, v1, {
                    let a183 = v3;
                    let a184 = 1u64;
                    nat_add(a183, a184)?
                }), f6(Str::lit("expected `:` after a member name"))))
            }
        } else {
            Ok(f25(v2, f26(v0, v1, v3), f6(Str::lit("expected a member name"))))
        }
    }
}

pub fn f28(v0: List<Adt2>, v1: u64, v2: Adt1, v3: u64, v4: bool) -> R<Adt1> {
    let v5: u64 = match v2.clone() {
        Adt1::C0(h185, _, _, _) => {
            h185
        }
    };
    let v6: Adt6 = f42(v0.clone(), v1, v5, v3)?;
    let m187 = match v6.clone() {
        Adt6::C0(_, _, _, _, _, h186) => {
            h186
        }
    };
    match m187 {
        None => {
            let v8: u64 = match v6 {
                Adt6::C0(h190, _, _, _, _, _) => {
                    h190
                }
            };
            if v4 {
                let v9: List<(u64, u64)> = List::cons((f26(v0.clone(), v1, v5), f43(v0.clone(), v8)), match v2 {
                    Adt1::C0(_, _, h191, _) => {
                        h191
                    }
                });
                let a192 = f23(v0, v8);
                let a193 = 6u64;
                if nat_eq(a192, a193) {
                    Ok(Adt1::C0({
                        let a194 = v8;
                        let a195 = 1u64;
                        nat_add(a194, a195)?
                    }, 2u64, v9, None::<Adt0>))
                } else {
                    Ok(Adt1::C0(v8, 3u64, v9, None::<Adt0>))
                }
            } else {
                Ok(f24(v2, v8, 4u64))
            }
        }
        Some(v7) => {
            Ok(Adt1::C0(match v2.clone() {
                Adt1::C0(h188, _, _, _) => {
                    h188
                }
            }, 9u64, match v2 {
                Adt1::C0(_, _, h189, _) => {
                    h189
                }
            }, Some(v7)))
        }
    }
}

pub fn f29(v0: List<Adt2>, v1: u64, v2: Adt1) -> R<Adt1> {
    let v3: u64 = match v2.clone() {
        Adt1::C0(h196, _, _, _) => {
            h196
        }
    };
    let a197 = f23(v0.clone(), v3);
    let a198 = 1u64;
    if nat_eq(a197, a198) {
        let a202 = {
            let a199 = v3;
            let a200 = 1u64;
            nat_add(a199, a200)?
        };
        let a203 = {
            let a201 = v0.clone();
            length_list(a201)
        };
        if nat_lt(a202, a203) {
            Ok(f25(v2, f26(v0, v1, {
                let a204 = v3;
                let a205 = 1u64;
                nat_add(a204, a205)?
            }), f6(Str::lit("unexpected text after the module"))))
        } else {
            Ok(f24(v2, {
                let a206 = v3;
                let a207 = 1u64;
                nat_add(a206, a207)?
            }, 8u64))
        }
    } else {
        Ok(f25(v2, f26(v0, v1, v3), f6(Str::lit("expected `,` or `}`"))))
    }
}

pub fn f30(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m208 = f32(v0, v1);
    match m208 {
        None => {
            false
        }
        Some(v4) => {
            f44(v4, v2, v3)
        }
    }
}

pub fn f31(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m209 = f32(v0, v1);
    match m209 {
        None => {
            false
        }
        Some(v3) => {
            let a210 = v3;
            let a211 = v2;
            equal(a210, a211)
        }
    }
}

pub fn f32(v0: Bytes, v1: u64) -> Option<u8> {
    let a212 = v0;
    let a213 = v1;
    index_bytes(a212, a213)
}

pub fn f33(v0: Bytes, v1: u64, v2: u64) -> R<Adt5> {
    let (h214, _) = f45(Fn4::F46(v1, v0), {
        let a217 = {
            let a215 = v1;
            let a216 = v2;
            nat_sub(a215, a216)
        };
        let a218 = 1u64;
        nat_add(a217, a218)?
    }, Adt5::C0({
        let a219 = v2;
        let a220 = 1u64;
        nat_add(a219, a220)?
    }, List::<Bytes>::nil(), false, None::<Adt0>))?;
    Ok(h214)
}

pub fn f34(v0: List<Bytes>) -> Bytes {
    f47(f48(v0))
}

pub fn f35(v0: u64, v1: Bytes) -> Adt0 {
    Adt0::C0(v0, v1)
}

pub fn f36(v0: Str) -> Bytes {
    let a221 = v0;
    utf8_encode(a221)
}

pub fn f37(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h222, _) = f49(Fn5::F50(v1, v0), {
        let a223 = v1;
        let a224 = v2;
        nat_sub(a223, a224)
    }, v2)?;
    Ok(h222)
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m228 = {
        let a225 = v0;
        let a226 = v1;
        let a227 = v2;
        slice_bytes(a225, a226, a227)
    };
    match m228 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f39(v0: List<Adt2>, v1: u64) -> Option<Adt2> {
    let a229 = v0;
    let a230 = v1;
    index_list(a229, a230)
}

pub fn f40(v0: Adt3) -> u64 {
    let m231 = v0;
    match m231 {
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

pub fn f41(v0: List<Adt2>, v1: u64) -> Bytes {
    let m232 = f39(v0, v1);
    match m232 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f51(match v2 {
                Adt2::C0(h233, _, _) => {
                    h233
                }
            })
        }
    }
}

pub fn f42(v0: List<Adt2>, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    let (h234, _) = f52(Fn6::F53(v1, v0.clone()), {
        let a238 = {
            let a236 = {
                let a235 = v0;
                length_list(a235)
            };
            let a237 = 4u64;
            nat_mul(a236, a237)?
        };
        let a239 = 8u64;
        nat_add(a238, a239)?
    }, Adt6::C0(v2, v3, 0u64, List::<Adt7>::nil(), Adt8::C5, None::<Adt0>))?;
    Ok(h234)
}

pub fn f43(v0: List<Adt2>, v1: u64) -> u64 {
    let a240 = v1;
    let a241 = 0u64;
    if nat_eq(a240, a241) {
        0u64
    } else {
        let m246 = {
            let a244 = v0;
            let a245 = {
                let a242 = v1;
                let a243 = 1u64;
                nat_sub(a242, a243)
            };
            index_list(a244, a245)
        };
        match m246 {
            None => {
                0u64
            }
            Some(v2) => {
                match v2 {
                    Adt2::C0(_, _, h247) => {
                        h247
                    }
                }
            }
        }
    }
}

pub fn f44(v0: u8, v1: u8, v2: u8) -> bool {
    let m250 = {
        let a248 = v0;
        let a249 = v1;
        fixed_u8::checked_sub(a248, a249)
    };
    match m250 {
        None => {
            false
        }
        Some(_) => {
            let m253 = {
                let a251 = v2;
                let a252 = v0;
                fixed_u8::checked_sub(a251, a252)
            };
            match m253 {
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

pub fn f45(v0: Fn4, p258: u64, p259: Adt5) -> R<(Adt5, bool)> {
    let m257 = (p258, p259);
    {
        let mut m257 = m257;
        loop {
            let step = {
                let (v1, v2) = m257;
                let m254 = v1;
                if m254 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m254 - 1;
                    let m256 = {
                        let c255: Fn4 = v0.clone();
                        c255.apply(v2.clone())?
                    };
                    match m256 {
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
                Ok(next) => { m257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f54(v1, v0, v2)
}

pub fn f47(v0: List<Bytes>) -> Bytes {
    let m262 = {
        let (h260, _) = f55(Fn7::F56, {
            let a261 = v0.clone();
            length_list(a261)
        }, v0);
        h260
    };
    match m262.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f48(v0: List<Bytes>) -> List<Bytes> {
    f57(Fn8::F58, List::<Bytes>::nil(), v0)
}

pub fn f49(v0: Fn5, p267: u64, p268: u64) -> R<(u64, bool)> {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v1, v2) = m266;
                let m263 = v1;
                if m263 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m263 - 1;
                    let m265 = {
                        let c264: Fn5 = v0.clone();
                        c264.apply(v2)?
                    };
                    match m265 {
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
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f59(v1, v0, v2)
}

pub fn f51(v0: Adt3) -> Bytes {
    let m269 = v0;
    match m269 {
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

pub fn f52(v0: Fn6, p274: u64, p275: Adt6) -> R<(Adt6, bool)> {
    let m273 = (p274, p275);
    {
        let mut m273 = m273;
        loop {
            let step = {
                let (v1, v2) = m273;
                let m270 = v1;
                if m270 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m270 - 1;
                    let m272 = {
                        let c271: Fn6 = v0.clone();
                        c271.apply(v2.clone())?
                    };
                    match m272 {
                        None => {
                            Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), R<(Adt6, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: u64, v1: List<Adt2>, v2: Adt6) -> R<Option<Adt6>> {
    f60(v1, v0, v2)
}

pub fn f54(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let m277 = match v2.clone() {
        Adt5::C0(_, _, _, h276) => {
            h276
        }
    };
    match m277 {
        None => {
            if match v2.clone() {
                Adt5::C0(_, _, h278, _) => {
                    h278
                }
            } {
                Ok(None::<Adt5>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt5::C0(h279, _, _, _) => {
                        h279
                    }
                };
                let a280 = v4;
                let a281 = v1;
                if nat_lt(a280, a281) {
                    if f18(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt5::C0({
                            let a282 = v4;
                            let a283 = 1u64;
                            nat_add(a282, a283)?
                        }, match v2 {
                            Adt5::C0(_, h284, _, _) => {
                                h284
                            }
                        }, true, None::<Adt0>)))
                    } else {
                        if f18(v0.clone(), v4, 92u8) {
                            Ok(Some(f61(v0, v1, v2)?))
                        } else {
                            let a285 = f62(v0.clone(), v4)?;
                            let a286 = 32u64;
                            if nat_lt(a285, a286) {
                                Ok(Some(f63(v2, v4, f36(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f64(v0.clone(), v1, v4)?;
                                Ok(Some(f65(v2, v5, f38(v0, v4, {
                                    let a287 = v5;
                                    let a288 = v4;
                                    nat_sub(a287, a288)
                                }))))
                            }
                        }
                    }
                } else {
                    Ok(None::<Adt5>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f55(v0: Fn7, p293: u64, p294: List<Bytes>) -> (List<Bytes>, bool) {
    let m292 = (p293, p294);
    {
        let mut m292 = m292;
        loop {
            let step = {
                let (v1, v2) = m292;
                let m289 = v1;
                if m289 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m289 - 1;
                    let m291 = {
                        let c290: Fn7 = v0.clone();
                        c290.apply(v2.clone())
                    };
                    match m291 {
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
                Ok(next) => { m292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: List<Bytes>) -> Option<List<Bytes>> {
    f66(v0)
}

pub fn f57(v0: Fn8, p298: List<Bytes>, p299: List<Bytes>) -> List<Bytes> {
    let m297 = (p298, p299);
    {
        let mut m297 = m297;
        loop {
            let step = {
                let (v1, v2) = m297;
                let m295 = v2;
                match m295.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c296: Fn8 = v0.clone();
                            c296.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f59(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a300 = v2;
    let a301 = v1;
    if nat_lt(a300, a301) {
        if f67(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a302 = v2;
                let a303 = 1u64;
                nat_add(a302, a303)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f60(v0: List<Adt2>, v1: u64, v2: Adt6) -> R<Option<Adt6>> {
    let m305 = match v2.clone() {
        Adt6::C0(_, _, _, _, _, h304) => {
            h304
        }
    };
    match m305 {
        None => {
            let a307 = match v2.clone() {
                Adt6::C0(_, _, h306, _, _, _) => {
                    h306
                }
            };
            let a308 = 9u64;
            if nat_eq(a307, a308) {
                Ok(None::<Adt6>)
            } else {
                Ok(Some(f68(v0, v1, v2)?))
            }
        }
        Some(_) => {
            Ok(None::<Adt6>)
        }
    }
}

pub fn f61(v0: Bytes, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h309, _, _, _) => {
            h309
        }
    };
    let m312 = f32(v0.clone(), {
        let a310 = v3;
        let a311 = 1u64;
        nat_add(a310, a311)?
    });
    match m312 {
        None => {
            Ok(Adt5::C0(v1, match v2 {
                Adt5::C0(_, h313, _, _) => {
                    h313
                }
            }, false, None::<Adt0>))
        }
        Some(v4) => {
            let a314 = v4;
            let a315 = 34u8;
            if equal(a314, a315) {
                Ok(f65(v2, {
                    let a316 = v3;
                    let a317 = 2u64;
                    nat_add(a316, a317)?
                }, f36(Str::lit("\""))))
            } else {
                let a318 = v4;
                let a319 = 92u8;
                if equal(a318, a319) {
                    Ok(f65(v2, {
                        let a320 = v3;
                        let a321 = 2u64;
                        nat_add(a320, a321)?
                    }, f36(Str::lit("\\"))))
                } else {
                    let a322 = v4;
                    let a323 = 47u8;
                    if equal(a322, a323) {
                        Ok(f65(v2, {
                            let a324 = v3;
                            let a325 = 2u64;
                            nat_add(a324, a325)?
                        }, f36(Str::lit("/"))))
                    } else {
                        let a326 = v4;
                        let a327 = 98u8;
                        if equal(a326, a327) {
                            Ok(f65(v2, {
                                let a328 = v3;
                                let a329 = 2u64;
                                nat_add(a328, a329)?
                            }, f69(8u64)))
                        } else {
                            let a330 = v4;
                            let a331 = 102u8;
                            if equal(a330, a331) {
                                Ok(f65(v2, {
                                    let a332 = v3;
                                    let a333 = 2u64;
                                    nat_add(a332, a333)?
                                }, f69(12u64)))
                            } else {
                                let a334 = v4;
                                let a335 = 110u8;
                                if equal(a334, a335) {
                                    Ok(f65(v2, {
                                        let a336 = v3;
                                        let a337 = 2u64;
                                        nat_add(a336, a337)?
                                    }, f69(10u64)))
                                } else {
                                    let a338 = v4;
                                    let a339 = 114u8;
                                    if equal(a338, a339) {
                                        Ok(f65(v2, {
                                            let a340 = v3;
                                            let a341 = 2u64;
                                            nat_add(a340, a341)?
                                        }, f69(13u64)))
                                    } else {
                                        let a342 = v4;
                                        let a343 = 116u8;
                                        if equal(a342, a343) {
                                            Ok(f65(v2, {
                                                let a344 = v3;
                                                let a345 = 2u64;
                                                nat_add(a344, a345)?
                                            }, f69(9u64)))
                                        } else {
                                            let a346 = v4;
                                            let a347 = 117u8;
                                            if equal(a346, a347) {
                                                f70(v0, v2)
                                            } else {
                                                Ok(f63(v2, v3, f71(f71(f36(Str::lit("`\\")), f38(v0.clone(), {
                                                    let a348 = v3;
                                                    let a349 = 1u64;
                                                    nat_add(a348, a349)?
                                                }, f72(v0, {
                                                    let a350 = v3;
                                                    let a351 = 1u64;
                                                    nat_add(a350, a351)?
                                                })?)), f36(Str::lit("` is not an escape")))))
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

pub fn f62(v0: Bytes, v1: u64) -> R<u64> {
    let m352 = f32(v0, v1);
    match m352 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a397 = {
                let a373 = {
                    let a361 = {
                        let a355 = {
                            let a353 = v3;
                            let a354 = 128u8;
                            fixed_u8::bit_and(a353, a354)
                        };
                        let a356 = 128u8;
                        if equal(a355, a356) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a362 = {
                        let a359 = {
                            let a357 = v3;
                            let a358 = 64u8;
                            fixed_u8::bit_and(a357, a358)
                        };
                        let a360 = 64u8;
                        if equal(a359, a360) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a361, a362)?
                };
                let a374 = {
                    let a371 = {
                        let a365 = {
                            let a363 = v3;
                            let a364 = 32u8;
                            fixed_u8::bit_and(a363, a364)
                        };
                        let a366 = 32u8;
                        if equal(a365, a366) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a372 = {
                        let a369 = {
                            let a367 = v3;
                            let a368 = 16u8;
                            fixed_u8::bit_and(a367, a368)
                        };
                        let a370 = 16u8;
                        if equal(a369, a370) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a371, a372)?
                };
                nat_add(a373, a374)?
            };
            let a398 = {
                let a395 = {
                    let a383 = {
                        let a377 = {
                            let a375 = v3;
                            let a376 = 8u8;
                            fixed_u8::bit_and(a375, a376)
                        };
                        let a378 = 8u8;
                        if equal(a377, a378) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a384 = {
                        let a381 = {
                            let a379 = v3;
                            let a380 = 4u8;
                            fixed_u8::bit_and(a379, a380)
                        };
                        let a382 = 4u8;
                        if equal(a381, a382) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a383, a384)?
                };
                let a396 = {
                    let a393 = {
                        let a387 = {
                            let a385 = v3;
                            let a386 = 2u8;
                            fixed_u8::bit_and(a385, a386)
                        };
                        let a388 = 2u8;
                        if equal(a387, a388) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a394 = {
                        let a391 = {
                            let a389 = v3;
                            let a390 = 1u8;
                            fixed_u8::bit_and(a389, a390)
                        };
                        let a392 = 1u8;
                        if equal(a391, a392) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a393, a394)?
                };
                nat_add(a395, a396)?
            };
            nat_add(a397, a398)
        }
    }
}

pub fn f63(v0: Adt5, v1: u64, v2: Bytes) -> Adt5 {
    Adt5::C0(match v0.clone() {
        Adt5::C0(h399, _, _, _) => {
            h399
        }
    }, match v0 {
        Adt5::C0(_, h400, _, _) => {
            h400
        }
    }, false, Some(f35(v1, v2)))
}

pub fn f64(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h401, _) = f49(Fn5::F73(v1, v0), {
        let a402 = v1;
        let a403 = v2;
        nat_sub(a402, a403)
    }, v2)?;
    Ok(h401)
}

pub fn f65(v0: Adt5, v1: u64, v2: Bytes) -> Adt5 {
    Adt5::C0(v1, List::cons(v2, match v0 {
        Adt5::C0(_, h404, _, _) => {
            h404
        }
    }), false, None::<Adt0>)
}

pub fn f66(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m405 = v0.clone();
    match m405.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m406 = v2;
            match m406.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f74(Fn9::F75, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m408 = {
                        let (h407, _) = v5;
                        h407
                    };
                    match m408 {
                        None => {
                            Some(f48({
                                let (_, h410) = v5;
                                h410
                            }))
                        }
                        Some(v6) => {
                            Some(f48(List::cons(v6, {
                                let (_, h409) = v5;
                                h409
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f67(v0: Bytes, v1: u64) -> R<bool> {
    let a411 = 0u64;
    let a412 = f17(v0.clone(), v1)?;
    if nat_lt(a411, a412) {
        Ok(true)
    } else {
        Ok(f76(v0, v1))
    }
}

pub fn f68(v0: List<Adt2>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(_, _, h413, _, _, _) => {
            h413
        }
    };
    let a414 = v3;
    let a415 = 0u64;
    if nat_eq(a414, a415) {
        f77(v0, v1, v2)
    } else {
        let a416 = v3;
        let a417 = 1u64;
        if nat_eq(a416, a417) {
            f78(v0, v2)
        } else {
            let a418 = v3;
            let a419 = 2u64;
            if nat_eq(a418, a419) {
                f79(v0, v1, v2)
            } else {
                let a420 = v3;
                let a421 = 3u64;
                if nat_eq(a420, a421) {
                    f80(v0, v2)
                } else {
                    let a422 = v3;
                    let a423 = 4u64;
                    if nat_eq(a422, a423) {
                        f81(v0, v1, v2, 3u64, f82(Str::lit("expected `,` or `]`")))
                    } else {
                        f81(v0, v1, v2, 1u64, f82(Str::lit("expected `,` or `}`")))
                    }
                }
            }
        }
    }
}

pub fn f69(v0: u64) -> Bytes {
    f38(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f70(v0: Bytes, v1: Adt5) -> R<Adt5> {
    let v2: u64 = match v1.clone() {
        Adt5::C0(h424, _, _, _) => {
            h424
        }
    };
    let m427 = f83(v0, {
        let a425 = v2;
        let a426 = 2u64;
        nat_add(a425, a426)?
    })?;
    match m427 {
        None => {
            Ok(f63(v1, v2, f36(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a428 = 55296u64;
            let a429 = v3;
            if nat_le(a428, a429) {
                let a430 = v3;
                let a431 = 57343u64;
                if nat_le(a430, a431) {
                    Ok(f63(v1, v2, f71(f71(f36(Str::lit("`\\u")), f84(v3)), f36(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f65(v1, {
                        let a432 = v2;
                        let a433 = 6u64;
                        nat_add(a432, a433)?
                    }, f85(v3)?))
                }
            } else {
                Ok(f65(v1, {
                    let a434 = v2;
                    let a435 = 6u64;
                    nat_add(a434, a435)?
                }, f85(v3)?))
            }
        }
    }
}

pub fn f71(v0: Bytes, v1: Bytes) -> Bytes {
    let a436 = v0;
    let a437 = v1;
    append_bytes(a436, a437)
}

pub fn f72(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f62(v0, v1)?;
    let a438 = v2;
    let a439 = 128u64;
    if nat_lt(a438, a439) {
        Ok(1u64)
    } else {
        let a440 = v2;
        let a441 = 224u64;
        if nat_lt(a440, a441) {
            Ok(2u64)
        } else {
            let a442 = v2;
            let a443 = 240u64;
            if nat_lt(a442, a443) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f73(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f86(v1, v0, v2)
}

pub fn f74(v0: Fn9, p447: (Option<Bytes>, List<Bytes>), p448: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m446 = (p447, p448);
    {
        let mut m446 = m446;
        loop {
            let step = {
                let (v1, v2) = m446;
                let m444 = v2;
                match m444.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c445: Fn9 = v0.clone();
                            c445.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f87(v0, v1)
}

pub fn f76(v0: Bytes, v1: u64) -> bool {
    if f18(v0.clone(), v1, 123u8) {
        true
    } else {
        if f18(v0.clone(), v1, 125u8) {
            true
        } else {
            if f18(v0.clone(), v1, 91u8) {
                true
            } else {
                if f18(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f18(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f18(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f18(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f18(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f18(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f18(v0, v1, 34u8)
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

pub fn f77(v0: List<Adt2>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h449, _, _, _, _, _) => {
            h449
        }
    };
    let a451 = 256u64;
    let a452 = match v2.clone() {
        Adt6::C0(_, h450, _, _, _, _) => {
            h450
        }
    };
    if nat_lt(a451, a452) {
        Ok(f88(v2, f26(v0, v1, v3), f82(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v4: u64 = f23(v0.clone(), v3);
        let a453 = v4;
        let a454 = 99u64;
        if nat_eq(a453, a454) {
            Ok(f88(v2, v1, f82(Str::lit("expected a value, found the end of the text"))))
        } else {
            let a455 = v4;
            let a456 = 11u64;
            if nat_eq(a455, a456) {
                Ok(f89(v2, {
                    let a457 = v3;
                    let a458 = 1u64;
                    nat_add(a457, a458)?
                }, Adt8::C2(f41(v0, v3))))
            } else {
                let a459 = v4;
                let a460 = 2u64;
                if nat_eq(a459, a460) {
                    f90(v2, {
                        let a461 = v3;
                        let a462 = 1u64;
                        nat_add(a461, a462)?
                    }, Adt7::C0(List::<Adt8>::nil()), 1u64)
                } else {
                    let a463 = v4;
                    let a464 = 0u64;
                    if nat_eq(a463, a464) {
                        f90(v2, {
                            let a465 = v3;
                            let a466 = 1u64;
                            nat_add(a465, a466)?
                        }, Adt7::C1(Bytes::lit(&[]), 0u64, List::<(Bytes, Adt8)>::nil()), 2u64)
                    } else {
                        let a467 = v4;
                        let a468 = 12u64;
                        if nat_eq(a467, a468) {
                            f91(v0.clone(), v1, v2, f26(v0, v1, v3))
                        } else {
                            Ok(f88(v2, f26(v0.clone(), v1, v3), f92(f82(Str::lit("expected a value, found ")), f93(v0, v3)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f78(v0: List<Adt2>, v1: Adt6) -> R<Adt6> {
    let a470 = f23(v0, match v1.clone() {
        Adt6::C0(h469, _, _, _, _, _) => {
            h469
        }
    });
    let a471 = 3u64;
    if nat_eq(a470, a471) {
        f94(v1)
    } else {
        Ok(Adt6::C0(match v1.clone() {
            Adt6::C0(h472, _, _, _, _, _) => {
                h472
            }
        }, match v1.clone() {
            Adt6::C0(_, h473, _, _, _, _) => {
                h473
            }
        }, 0u64, match v1 {
            Adt6::C0(_, _, _, h474, _, _) => {
                h474
            }
        }, Adt8::C5, None::<Adt0>))
    }
}

pub fn f79(v0: List<Adt2>, v1: u64, v2: Adt6) -> R<Adt6> {
    let m476 = match v2.clone() {
        Adt6::C0(_, _, _, h475, _, _) => {
            h475
        }
    };
    match m476.uncons() {
        None => {
            Ok(f88(v2, 0u64, f82(Str::lit("no object is open"))))
        }
        Some((v3, v4)) => {
            let m477 = v3;
            match m477 {
                Adt7::C0(_) => {
                    Ok(f88(v2, 0u64, f82(Str::lit("no object is open"))))
                }
                Adt7::C1(_, _, v8) => {
                    let a479 = f23(v0.clone(), match v2.clone() {
                        Adt6::C0(h478, _, _, _, _, _) => {
                            h478
                        }
                    });
                    let a480 = 1u64;
                    if nat_eq(a479, a480) {
                        f94(v2)
                    } else {
                        f95(v0, v1, v2, v8, v4)
                    }
                }
            }
        }
    }
}

pub fn f80(v0: List<Adt2>, v1: Adt6) -> R<Adt6> {
    let m482 = match v1.clone() {
        Adt6::C0(_, _, _, h481, _, _) => {
            h481
        }
    };
    match m482.uncons() {
        None => {
            Ok(Adt6::C0(match v1.clone() {
                Adt6::C0(h483, _, _, _, _, _) => {
                    h483
                }
            }, match v1.clone() {
                Adt6::C0(_, h484, _, _, _, _) => {
                    h484
                }
            }, 9u64, List::<Adt7>::nil(), match v1 {
                Adt6::C0(_, _, _, _, h485, _) => {
                    h485
                }
            }, None::<Adt0>))
        }
        Some((v2, v3)) => {
            let m486 = v2;
            match m486 {
                Adt7::C0(v4) => {
                    f96(v0, v1.clone(), List::cons(Adt7::C0(List::cons(match v1 {
                        Adt6::C0(_, _, _, _, h487, _) => {
                            h487
                        }
                    }, v4)), v3), 1u64, 4u64)
                }
                Adt7::C1(v5, v6, v7) => {
                    if f97(v7.clone(), v5.clone()) {
                        Ok(f88(v1, v6, f92(f92(f82(Str::lit("the member `")), v5), f82(Str::lit("` is written twice")))))
                    } else {
                        f96(v0, v1.clone(), List::cons(Adt7::C1(Bytes::lit(&[]), 0u64, List::cons((v5, match v1 {
                            Adt6::C0(_, _, _, _, h488, _) => {
                                h488
                            }
                        }), v7)), v3), 2u64, 5u64)
                    }
                }
            }
        }
    }
}

pub fn f81(v0: List<Adt2>, v1: u64, v2: Adt6, v3: u64, v4: Bytes) -> R<Adt6> {
    let a490 = f23(v0.clone(), match v2.clone() {
        Adt6::C0(h489, _, _, _, _, _) => {
            h489
        }
    });
    let a491 = v3;
    if nat_eq(a490, a491) {
        f94(v2)
    } else {
        Ok(f88(v2.clone(), f26(v0, v1, match v2 {
            Adt6::C0(h492, _, _, _, _, _) => {
                h492
            }
        }), v4))
    }
}

pub fn f82(v0: Str) -> Bytes {
    let a493 = v0;
    utf8_encode(a493)
}

pub fn f83(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m494 = f98(v0.clone(), v1)?;
    match m494 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m497 = f98(v0.clone(), {
                let a495 = v1;
                let a496 = 1u64;
                nat_add(a495, a496)?
            })?;
            match m497 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m500 = f98(v0.clone(), {
                        let a498 = v1;
                        let a499 = 2u64;
                        nat_add(a498, a499)?
                    })?;
                    match m500 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m503 = f98(v0, {
                                let a501 = v1;
                                let a502 = 3u64;
                                nat_add(a501, a502)?
                            })?;
                            match m503 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a514 = {
                                            let a512 = {
                                                let a510 = {
                                                    let a508 = {
                                                        let a506 = {
                                                            let a504 = v2;
                                                            let a505 = 16u64;
                                                            nat_mul(a504, a505)?
                                                        };
                                                        let a507 = v3;
                                                        nat_add(a506, a507)?
                                                    };
                                                    let a509 = 16u64;
                                                    nat_mul(a508, a509)?
                                                };
                                                let a511 = v4;
                                                nat_add(a510, a511)?
                                            };
                                            let a513 = 16u64;
                                            nat_mul(a512, a513)?
                                        };
                                        let a515 = v5;
                                        nat_add(a514, a515)?
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

pub fn f84(v0: u64) -> Bytes {
    f71(f71(f99(v0, 4096u64), f99(v0, 256u64)), f71(f99(v0, 16u64), f100(v0)))
}

pub fn f85(v0: u64) -> R<Bytes> {
    let a516 = v0;
    let a517 = 128u64;
    if nat_lt(a516, a517) {
        Ok(f69(v0))
    } else {
        let a518 = v0;
        let a519 = 2048u64;
        if nat_lt(a518, a519) {
            Ok(f71(f69({
                let a523 = 192u64;
                let a524 = {
                    let a520 = v0;
                    let a521 = 64u64;
                    let a522 = 0u64;
                    nat_quot(a520, a521, a522)
                };
                nat_add(a523, a524)?
            }), f69({
                let a528 = 128u64;
                let a529 = {
                    let a525 = v0;
                    let a526 = 64u64;
                    let a527 = 0u64;
                    nat_rem(a525, a526, a527)
                };
                nat_add(a528, a529)?
            })))
        } else {
            Ok(f71(f69({
                let a533 = 224u64;
                let a534 = {
                    let a530 = v0;
                    let a531 = 4096u64;
                    let a532 = 0u64;
                    nat_quot(a530, a531, a532)
                };
                nat_add(a533, a534)?
            }), f71(f69({
                let a541 = 128u64;
                let a542 = {
                    let a538 = {
                        let a535 = v0;
                        let a536 = 64u64;
                        let a537 = 0u64;
                        nat_quot(a535, a536, a537)
                    };
                    let a539 = 64u64;
                    let a540 = 0u64;
                    nat_rem(a538, a539, a540)
                };
                nat_add(a541, a542)?
            }), f69({
                let a546 = 128u64;
                let a547 = {
                    let a543 = v0;
                    let a544 = 64u64;
                    let a545 = 0u64;
                    nat_rem(a543, a544, a545)
                };
                nat_add(a546, a547)?
            }))))
        }
    }
}

pub fn f86(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a548 = v2;
    let a549 = v1;
    if nat_lt(a548, a549) {
        let v3: u64 = f62(v0, v2)?;
        let a550 = v3;
        let a551 = 32u64;
        if nat_lt(a550, a551) {
            Ok(None::<u64>)
        } else {
            let a552 = v3;
            let a553 = 34u64;
            if nat_eq(a552, a553) {
                Ok(None::<u64>)
            } else {
                let a554 = v3;
                let a555 = 92u64;
                if nat_eq(a554, a555) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a556 = v2;
                        let a557 = 1u64;
                        nat_add(a556, a557)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f87(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m559 = {
        let (h558, _) = v0;
        h558
    };
    match m559 {
        None => {
            (Some(v1), {
                let (_, h560) = v0;
                h560
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a561 = v2;
                let a562 = v1;
                append_bytes(a561, a562)
            }, {
                let (_, h563) = v0;
                h563
            }))
        }
    }
}

pub fn f88(v0: Adt6, v1: u64, v2: Bytes) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h564, _, _, _, _, _) => {
            h564
        }
    }, match v0.clone() {
        Adt6::C0(_, h565, _, _, _, _) => {
            h565
        }
    }, 9u64, match v0.clone() {
        Adt6::C0(_, _, _, h566, _, _) => {
            h566
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h567, _) => {
            h567
        }
    }, Some(f101(v1, v2)))
}

pub fn f89(v0: Adt6, v1: u64, v2: Adt8) -> Adt6 {
    Adt6::C0(v1, match v0.clone() {
        Adt6::C0(_, h568, _, _, _, _) => {
            h568
        }
    }, 3u64, match v0 {
        Adt6::C0(_, _, _, h569, _, _) => {
            h569
        }
    }, v2, None::<Adt0>)
}

pub fn f90(v0: Adt6, v1: u64, v2: Adt7, v3: u64) -> R<Adt6> {
    Ok(Adt6::C0(v1, {
        let a571 = match v0.clone() {
            Adt6::C0(_, h570, _, _, _, _) => {
                h570
            }
        };
        let a572 = 1u64;
        nat_add(a571, a572)?
    }, v3, List::cons(v2, match v0 {
        Adt6::C0(_, _, _, h573, _, _) => {
            h573
        }
    }), Adt8::C5, None::<Adt0>))
}

pub fn f91(v0: List<Adt2>, _: u64, v2: Adt6, v3: u64) -> R<Adt6> {
    let v4: u64 = match v2.clone() {
        Adt6::C0(h574, _, _, _, _, _) => {
            h574
        }
    };
    let v5: Bytes = f41(v0.clone(), v4);
    let a575 = v5.clone();
    let a576 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a575, a576) {
        Ok(f89(v2, {
            let a577 = v4;
            let a578 = 1u64;
            nat_add(a577, a578)?
        }, Adt8::C0(true)))
    } else {
        let a579 = v5.clone();
        let a580 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        if equal(a579, a580) {
            Ok(f89(v2, {
                let a581 = v4;
                let a582 = 1u64;
                nat_add(a581, a582)?
            }, Adt8::C0(false)))
        } else {
            if f102(v5.clone())? {
                let m583 = f103(v5.clone())?;
                match m583 {
                    None => {
                        Ok(f88(v2, v3, f92(f92(f82(Str::lit("`")), v5), f82(Str::lit("` does not fit a 64-bit integer")))))
                    }
                    Some(v6) => {
                        Ok(f89(v2, {
                            let a584 = v4;
                            let a585 = 1u64;
                            nat_add(a584, a585)?
                        }, v6))
                    }
                }
            } else {
                if f104(v5.clone())? {
                    let a588 = f23(v0, {
                        let a586 = v4;
                        let a587 = 1u64;
                        nat_add(a586, a587)?
                    });
                    let a589 = 0u64;
                    if nat_eq(a588, a589) {
                        f90(v2, {
                            let a590 = v4;
                            let a591 = 2u64;
                            nat_add(a590, a591)?
                        }, Adt7::C1(Bytes::lit(&[]), 0u64, List::cons((f82(Str::lit("kind")), Adt8::C2(v5)), List::<(Bytes, Adt8)>::nil())), 2u64)
                    } else {
                        Ok(f89(v2, {
                            let a592 = v4;
                            let a593 = 1u64;
                            nat_add(a592, a593)?
                        }, Adt8::C2(v5)))
                    }
                } else {
                    Ok(f88(v2, v3, f92(f92(f82(Str::lit("`")), v5), f82(Str::lit("` is not a name; quote it to write a string")))))
                }
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: Bytes) -> Bytes {
    let a594 = v0;
    let a595 = v1;
    append_bytes(a594, a595)
}

pub fn f93(v0: List<Adt2>, v1: u64) -> R<Bytes> {
    let m596 = f39(v0, v1);
    match m596 {
        None => {
            Ok(f82(Str::lit("the end of the text")))
        }
        Some(v2) => {
            f105(match v2 {
                Adt2::C0(h597, _, _) => {
                    h597
                }
            })
        }
    }
}

pub fn f94(v0: Adt6) -> R<Adt6> {
    let m599 = match v0.clone() {
        Adt6::C0(_, _, _, h598, _, _) => {
            h598
        }
    };
    match m599.uncons() {
        None => {
            Ok(f88(v0, 0u64, f82(Str::lit("nothing to close"))))
        }
        Some((v1, v2)) => {
            let m600 = v1;
            match m600 {
                Adt7::C0(v3) => {
                    Ok(Adt6::C0({
                        let a602 = match v0.clone() {
                            Adt6::C0(h601, _, _, _, _, _) => {
                                h601
                            }
                        };
                        let a603 = 1u64;
                        nat_add(a602, a603)?
                    }, {
                        let a605 = match v0 {
                            Adt6::C0(_, h604, _, _, _, _) => {
                                h604
                            }
                        };
                        let a606 = 1u64;
                        nat_sub(a605, a606)
                    }, 3u64, v2, Adt8::C3(std::rc::Rc::new(f106(v3))), None::<Adt0>))
                }
                Adt7::C1(_, _, v6) => {
                    Ok(Adt6::C0({
                        let a608 = match v0.clone() {
                            Adt6::C0(h607, _, _, _, _, _) => {
                                h607
                            }
                        };
                        let a609 = 1u64;
                        nat_add(a608, a609)?
                    }, {
                        let a611 = match v0 {
                            Adt6::C0(_, h610, _, _, _, _) => {
                                h610
                            }
                        };
                        let a612 = 1u64;
                        nat_sub(a611, a612)
                    }, 3u64, v2, Adt8::C4(std::rc::Rc::new(f107(f108(v6)?))), None::<Adt0>))
                }
            }
        }
    }
}

pub fn f95(v0: List<Adt2>, v1: u64, v2: Adt6, v3: List<(Bytes, Adt8)>, v4: List<Adt7>) -> R<Adt6> {
    let v5: u64 = match v2.clone() {
        Adt6::C0(h613, _, _, _, _, _) => {
            h613
        }
    };
    let v6: u64 = f23(v0.clone(), v5);
    let v7: Bytes = f41(v0.clone(), v5);
    let a614 = v6;
    let a615 = 11u64;
    if if nat_eq(a614, a615) {
        true
    } else {
        let a616 = v6;
        let a617 = 12u64;
        if nat_eq(a616, a617) {
            f104(v7.clone())?
        } else {
            false
        }
    } {
        let a620 = f23(v0.clone(), {
            let a618 = v5;
            let a619 = 1u64;
            nat_add(a618, a619)?
        });
        let a621 = 7u64;
        if nat_eq(a620, a621) {
            Ok(Adt6::C0({
                let a622 = v5;
                let a623 = 2u64;
                nat_add(a622, a623)?
            }, match v2 {
                Adt6::C0(_, h624, _, _, _, _) => {
                    h624
                }
            }, 0u64, List::cons(Adt7::C1(v7, f26(v0, v1, v5), v3), v4), Adt8::C5, None::<Adt0>))
        } else {
            Ok(f88(v2, f26(v0, v1, {
                let a625 = v5;
                let a626 = 1u64;
                nat_add(a625, a626)?
            }), f82(Str::lit("expected `:` after a member name"))))
        }
    } else {
        Ok(f88(v2, f26(v0, v1, v5), f82(Str::lit("expected a member name"))))
    }
}

pub fn f96(v0: List<Adt2>, v1: Adt6, v2: List<Adt7>, v3: u64, v4: u64) -> R<Adt6> {
    let a628 = f23(v0, match v1.clone() {
        Adt6::C0(h627, _, _, _, _, _) => {
            h627
        }
    });
    let a629 = 6u64;
    if nat_eq(a628, a629) {
        Ok(Adt6::C0({
            let a631 = match v1.clone() {
                Adt6::C0(h630, _, _, _, _, _) => {
                    h630
                }
            };
            let a632 = 1u64;
            nat_add(a631, a632)?
        }, match v1 {
            Adt6::C0(_, h633, _, _, _, _) => {
                h633
            }
        }, v3, v2, Adt8::C5, None::<Adt0>))
    } else {
        Ok(Adt6::C0(match v1.clone() {
            Adt6::C0(h634, _, _, _, _, _) => {
                h634
            }
        }, match v1 {
            Adt6::C0(_, h635, _, _, _, _) => {
                h635
            }
        }, v4, v2, Adt8::C5, None::<Adt0>))
    }
}

pub fn f97(v0: List<(Bytes, Adt8)>, v1: Bytes) -> bool {
    f109(Fn10::F110(v1), false, v0)
}

pub fn f98(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m636 = f32(v0, v1);
    match m636 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f111(v2)
        }
    }
}

pub fn f99(v0: u64, v1: u64) -> Bytes {
    f100({
        let a637 = v0;
        let a638 = v1;
        let a639 = 0u64;
        nat_quot(a637, a638, a639)
    })
}

pub fn f100(v0: u64) -> Bytes {
    f38(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a640 = v0;
        let a641 = 16u64;
        let a642 = 0u64;
        nat_rem(a640, a641, a642)
    }, 1u64)
}

pub fn f101(v0: u64, v1: Bytes) -> Adt0 {
    Adt0::C0(v0, v1)
}

pub fn f102(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a643 = f112(v0.clone(), 0u64)?;
        let a644 = 45u64;
        if nat_eq(a643, a644) {
            1u64
        } else {
            0u64
        }
    };
    let a646 = v1;
    let a647 = {
        let a645 = v0.clone();
        length_bytes(a645)
    };
    if nat_lt(a646, a647) {
        let a649 = f113(v0.clone(), v1)?;
        let a650 = {
            let a648 = v0;
            length_bytes(a648)
        };
        Ok(nat_eq(a649, a650))
    } else {
        Ok(false)
    }
}

pub fn f103(v0: Bytes) -> R<Option<Adt8>> {
    let v1: bool = {
        let a651 = f112(v0.clone(), 0u64)?;
        let a652 = 45u64;
        nat_eq(a651, a652)
    };
    let v2: u64 = if v1 {
        1u64
    } else {
        0u64
    };
    let v3: u64 = f114(v0.clone(), v2)?;
    let v4: Bytes = f38(v0.clone(), v3, {
        let a654 = {
            let a653 = v0;
            length_bytes(a653)
        };
        let a655 = v3;
        nat_sub(a654, a655)
    });
    let a657 = {
        let a656 = v4.clone();
        length_bytes(a656)
    };
    let a658 = 0u64;
    if nat_eq(a657, a658) {
        Ok(Some(Adt8::C1(false, 0u64)))
    } else {
        let a660 = 19u64;
        let a661 = {
            let a659 = v4.clone();
            length_bytes(a659)
        };
        if nat_lt(a660, a661) {
            Ok(None::<Adt8>)
        } else {
            let m662 = f115(v4)?;
            match m662 {
                None => {
                    Ok(None::<Adt8>)
                }
                Some(v5) => {
                    if f116(v1, v5) {
                        Ok(Some(Adt8::C1(v1, v5)))
                    } else {
                        Ok(None::<Adt8>)
                    }
                }
            }
        }
    }
}

pub fn f104(v0: Bytes) -> R<bool> {
    let a664 = {
        let a663 = v0.clone();
        length_bytes(a663)
    };
    let a665 = 0u64;
    if nat_eq(a664, a665) {
        Ok(false)
    } else {
        if f117(f112(v0.clone(), 0u64)?) {
            let a667 = f118(v0.clone(), 1u64)?;
            let a668 = {
                let a666 = v0.clone();
                length_bytes(a666)
            };
            if nat_eq(a667, a668) {
                let a669 = f119(v0);
                Ok(bool_not(a669))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f105(v0: Adt3) -> R<Bytes> {
    let m670 = v0;
    match m670 {
        Adt3::C0 => {
            Ok(f82(Str::lit("`{`")))
        }
        Adt3::C1 => {
            Ok(f82(Str::lit("`}`")))
        }
        Adt3::C2 => {
            Ok(f82(Str::lit("`[`")))
        }
        Adt3::C3 => {
            Ok(f82(Str::lit("`]`")))
        }
        Adt3::C4 => {
            Ok(f82(Str::lit("`(`")))
        }
        Adt3::C5 => {
            Ok(f82(Str::lit("`)`")))
        }
        Adt3::C6 => {
            Ok(f82(Str::lit("`,`")))
        }
        Adt3::C7 => {
            Ok(f82(Str::lit("`:`")))
        }
        Adt3::C8 => {
            Ok(f82(Str::lit("`:=`")))
        }
        Adt3::C9 => {
            Ok(f82(Str::lit("`::`")))
        }
        Adt3::C10 => {
            Ok(f82(Str::lit("`;`")))
        }
        Adt3::C11(v1) => {
            Ok(f92(f82(Str::lit("the string ")), f120(v1)?))
        }
        Adt3::C12(v2) => {
            Ok(f92(f92(f82(Str::lit("`")), v2), f82(Str::lit("`"))))
        }
    }
}

pub fn f106(v0: List<Adt8>) -> Adt8 {
    f121(Fn11::F122, Adt8::C5, v0)
}

pub fn f107(v0: List<(Bytes, Adt8)>) -> Adt8 {
    f123(Fn12::F124, Adt8::C5, f125(v0))
}

pub fn f108(v0: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    let v1: List<List<(Bytes, Adt8)>> = f126(f127(Fn13::F128, List::<List<(Bytes, Adt8)>>::nil(), v0.clone()));
    let m675 = {
        let (h671, _) = f129(Fn14::F130, {
            let a673 = {
                let a672 = v0;
                length_list(a672)
            };
            let a674 = 1u64;
            nat_add(a673, a674)?
        }, v1)?;
        h671
    };
    match m675.uncons() {
        None => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f109(v0: Fn10, p679: bool, p680: List<(Bytes, Adt8)>) -> bool {
    let m678 = (p679, p680);
    {
        let mut m678 = m678;
        loop {
            let step = {
                let (v1, v2) = m678;
                let m676 = v2;
                match m676.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt8)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt8)>), bool>(({
                            let c677: Fn10 = v0.clone();
                            c677.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: Bytes, v1: bool, v2: (Bytes, Adt8)) -> bool {
    if v1 {
        true
    } else {
        let a682 = {
            let (h681, _) = v2;
            h681
        };
        let a683 = v0;
        equal(a682, a683)
    }
}

pub fn f111(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a728 = {
            let a704 = {
                let a692 = {
                    let a686 = {
                        let a684 = v1;
                        let a685 = 128u8;
                        fixed_u8::bit_and(a684, a685)
                    };
                    let a687 = 128u8;
                    if equal(a686, a687) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a693 = {
                    let a690 = {
                        let a688 = v1;
                        let a689 = 64u8;
                        fixed_u8::bit_and(a688, a689)
                    };
                    let a691 = 64u8;
                    if equal(a690, a691) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a692, a693)?
            };
            let a705 = {
                let a702 = {
                    let a696 = {
                        let a694 = v1;
                        let a695 = 32u8;
                        fixed_u8::bit_and(a694, a695)
                    };
                    let a697 = 32u8;
                    if equal(a696, a697) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a703 = {
                    let a700 = {
                        let a698 = v1;
                        let a699 = 16u8;
                        fixed_u8::bit_and(a698, a699)
                    };
                    let a701 = 16u8;
                    if equal(a700, a701) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a702, a703)?
            };
            nat_add(a704, a705)?
        };
        let a729 = {
            let a726 = {
                let a714 = {
                    let a708 = {
                        let a706 = v1;
                        let a707 = 8u8;
                        fixed_u8::bit_and(a706, a707)
                    };
                    let a709 = 8u8;
                    if equal(a708, a709) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a715 = {
                    let a712 = {
                        let a710 = v1;
                        let a711 = 4u8;
                        fixed_u8::bit_and(a710, a711)
                    };
                    let a713 = 4u8;
                    if equal(a712, a713) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a714, a715)?
            };
            let a727 = {
                let a724 = {
                    let a718 = {
                        let a716 = v1;
                        let a717 = 2u8;
                        fixed_u8::bit_and(a716, a717)
                    };
                    let a719 = 2u8;
                    if equal(a718, a719) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a725 = {
                    let a722 = {
                        let a720 = v1;
                        let a721 = 1u8;
                        fixed_u8::bit_and(a720, a721)
                    };
                    let a723 = 1u8;
                    if equal(a722, a723) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a724, a725)?
            };
            nat_add(a726, a727)?
        };
        nat_add(a728, a729)?
    };
    let a730 = v2;
    let a731 = 48u64;
    if nat_lt(a730, a731) {
        Ok(None::<u64>)
    } else {
        let a732 = v2;
        let a733 = 57u64;
        if nat_le(a732, a733) {
            Ok(Some({
                let a734 = v2;
                let a735 = 48u64;
                nat_sub(a734, a735)
            }))
        } else {
            let a736 = v2;
            let a737 = 65u64;
            if nat_lt(a736, a737) {
                Ok(None::<u64>)
            } else {
                let a738 = v2;
                let a739 = 70u64;
                if nat_le(a738, a739) {
                    Ok(Some({
                        let a740 = v2;
                        let a741 = 55u64;
                        nat_sub(a740, a741)
                    }))
                } else {
                    let a742 = v2;
                    let a743 = 97u64;
                    if nat_lt(a742, a743) {
                        Ok(None::<u64>)
                    } else {
                        let a744 = v2;
                        let a745 = 102u64;
                        if nat_le(a744, a745) {
                            Ok(Some({
                                let a746 = v2;
                                let a747 = 87u64;
                                nat_sub(a746, a747)
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

pub fn f112(v0: Bytes, v1: u64) -> R<u64> {
    let m748 = f32(v0, v1);
    match m748 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a793 = {
                let a769 = {
                    let a757 = {
                        let a751 = {
                            let a749 = v3;
                            let a750 = 128u8;
                            fixed_u8::bit_and(a749, a750)
                        };
                        let a752 = 128u8;
                        if equal(a751, a752) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a758 = {
                        let a755 = {
                            let a753 = v3;
                            let a754 = 64u8;
                            fixed_u8::bit_and(a753, a754)
                        };
                        let a756 = 64u8;
                        if equal(a755, a756) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a757, a758)?
                };
                let a770 = {
                    let a767 = {
                        let a761 = {
                            let a759 = v3;
                            let a760 = 32u8;
                            fixed_u8::bit_and(a759, a760)
                        };
                        let a762 = 32u8;
                        if equal(a761, a762) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a768 = {
                        let a765 = {
                            let a763 = v3;
                            let a764 = 16u8;
                            fixed_u8::bit_and(a763, a764)
                        };
                        let a766 = 16u8;
                        if equal(a765, a766) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a767, a768)?
                };
                nat_add(a769, a770)?
            };
            let a794 = {
                let a791 = {
                    let a779 = {
                        let a773 = {
                            let a771 = v3;
                            let a772 = 8u8;
                            fixed_u8::bit_and(a771, a772)
                        };
                        let a774 = 8u8;
                        if equal(a773, a774) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a780 = {
                        let a777 = {
                            let a775 = v3;
                            let a776 = 4u8;
                            fixed_u8::bit_and(a775, a776)
                        };
                        let a778 = 4u8;
                        if equal(a777, a778) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a779, a780)?
                };
                let a792 = {
                    let a789 = {
                        let a783 = {
                            let a781 = v3;
                            let a782 = 2u8;
                            fixed_u8::bit_and(a781, a782)
                        };
                        let a784 = 2u8;
                        if equal(a783, a784) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a790 = {
                        let a787 = {
                            let a785 = v3;
                            let a786 = 1u8;
                            fixed_u8::bit_and(a785, a786)
                        };
                        let a788 = 1u8;
                        if equal(a787, a788) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a789, a790)?
                };
                nat_add(a791, a792)?
            };
            nat_add(a793, a794)
        }
    }
}

pub fn f113(v0: Bytes, v1: u64) -> R<u64> {
    let (h795, _) = f49(Fn5::F131(v0.clone()), {
        let a797 = {
            let a796 = v0;
            length_bytes(a796)
        };
        let a798 = 1u64;
        nat_add(a797, a798)?
    }, v1)?;
    Ok(h795)
}

pub fn f114(v0: Bytes, v1: u64) -> R<u64> {
    let (h799, _) = f49(Fn5::F132(v0.clone()), {
        let a801 = {
            let a800 = v0;
            length_bytes(a800)
        };
        let a802 = 1u64;
        nat_add(a801, a802)?
    }, v1)?;
    Ok(h799)
}

pub fn f115(v0: Bytes) -> R<Option<u64>> {
    if f133(v0.clone())? {
        Ok(Some({
            let (_, h803) = {
                let (h804, _) = f134(Fn15::F135(v0.clone()), {
                    let a805 = v0;
                    length_bytes(a805)
                }, (0u64, 0u64))?;
                h804
            };
            h803
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f116(v0: bool, v1: u64) -> bool {
    if v0 {
        let a806 = 0u64;
        let a807 = v1;
        if nat_lt(a806, a807) {
            let a808 = v1;
            let a809 = 9223372036854775808u64;
            nat_le(a808, a809)
        } else {
            false
        }
    } else {
        let a810 = v1;
        let a811 = 9223372036854775807u64;
        nat_le(a810, a811)
    }
}

pub fn f117(v0: u64) -> bool {
    let a812 = 64u64;
    let a813 = v0;
    if if nat_lt(a812, a813) {
        let a814 = v0;
        let a815 = 91u64;
        nat_lt(a814, a815)
    } else {
        false
    } {
        true
    } else {
        let a816 = 96u64;
        let a817 = v0;
        if if nat_lt(a816, a817) {
            let a818 = v0;
            let a819 = 123u64;
            nat_lt(a818, a819)
        } else {
            false
        } {
            true
        } else {
            let a820 = v0;
            let a821 = 95u64;
            nat_eq(a820, a821)
        }
    }
}

pub fn f118(v0: Bytes, v1: u64) -> R<u64> {
    let (h822, _) = f49(Fn5::F136(v0.clone()), {
        let a824 = {
            let a823 = v0;
            length_bytes(a823)
        };
        let a825 = 1u64;
        nat_add(a824, a825)?
    }, v1)?;
    Ok(h822)
}

pub fn f119(v0: Bytes) -> bool {
    let a826 = v0.clone();
    let a827 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a826, a827) {
        true
    } else {
        let a828 = v0;
        let a829 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a828, a829)
    }
}

pub fn f120(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a830 = v0.clone();
        length_bytes(a830)
    };
    let v2: Adt9 = {
        let (h831, _) = f137(Fn16::F138(v1, v0), {
            let a832 = v1;
            let a833 = 1u64;
            nat_add(a832, a833)?
        }, Adt9::C0(0u64, List::<Bytes>::nil()))?;
        h831
    };
    Ok(f71(f71(f36(Str::lit("\"")), f34(match v2 {
        Adt9::C0(_, h834) => {
            h834
        }
    })), f36(Str::lit("\""))))
}

pub fn f121(v0: Fn11, p838: Adt8, p839: List<Adt8>) -> Adt8 {
    let m837 = (p838, p839);
    {
        let mut m837 = m837;
        loop {
            let step = {
                let (v1, v2) = m837;
                let m835 = v2;
                match m835.uncons() {
                    None => {
                        Err::<(Adt8, List<Adt8>), Adt8>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt8, List<Adt8>), Adt8>(({
                            let c836: Fn11 = v0.clone();
                            c836.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m837 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt8, v1: Adt8) -> Adt8 {
    Adt8::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f123(v0: Fn12, p843: Adt8, p844: List<(Bytes, Adt8)>) -> Adt8 {
    let m842 = (p843, p844);
    {
        let mut m842 = m842;
        loop {
            let step = {
                let (v1, v2) = m842;
                let m840 = v2;
                match m840.uncons() {
                    None => {
                        Err::<(Adt8, List<(Bytes, Adt8)>), Adt8>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt8, List<(Bytes, Adt8)>), Adt8>(({
                            let c841: Fn12 = v0.clone();
                            c841.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: Adt8, v1: (Bytes, Adt8)) -> Adt8 {
    Adt8::C7({
        let (h845, _) = v1;
        h845
    }, std::rc::Rc::new({
        let (_, h846) = v1;
        h846
    }), std::rc::Rc::new(v0))
}

pub fn f125(v0: List<(Bytes, Adt8)>) -> List<(Bytes, Adt8)> {
    f139(Fn17::F140, List::<(Bytes, Adt8)>::nil(), v0)
}

pub fn f126(v0: List<List<(Bytes, Adt8)>>) -> List<List<(Bytes, Adt8)>> {
    f141(Fn18::F142, List::<List<(Bytes, Adt8)>>::nil(), v0)
}

pub fn f127(v0: Fn13, p850: List<List<(Bytes, Adt8)>>, p851: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
    let m849 = (p850, p851);
    {
        let mut m849 = m849;
        loop {
            let step = {
                let (v1, v2) = m849;
                let m847 = v2;
                match m847.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt8)>>, List<(Bytes, Adt8)>), List<List<(Bytes, Adt8)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt8)>>, List<(Bytes, Adt8)>), List<List<(Bytes, Adt8)>>>(({
                            let c848: Fn13 = v0.clone();
                            c848.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: List<List<(Bytes, Adt8)>>, v1: (Bytes, Adt8)) -> List<List<(Bytes, Adt8)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt8)>::nil()), v0)
}

pub fn f129(v0: Fn14, p856: u64, p857: List<List<(Bytes, Adt8)>>) -> R<(List<List<(Bytes, Adt8)>>, bool)> {
    let m855 = (p856, p857);
    {
        let mut m855 = m855;
        loop {
            let step = {
                let (v1, v2) = m855;
                let m852 = v1;
                if m852 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m852 - 1;
                    let m854 = {
                        let c853: Fn14 = v0.clone();
                        c853.apply(v2.clone())?
                    };
                    match m854 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
    f143(v0)
}

pub fn f131(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f144(v0, v1)
}

pub fn f132(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f145(v0, v1)
}

pub fn f133(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a858 = v0.clone();
        length_bytes(a858)
    };
    let a859 = 0u64;
    let a860 = v1;
    if nat_lt(a859, a860) {
        let a861 = f146(v0.clone())?;
        let a862 = v1;
        if nat_eq(a861, a862) {
            let a863 = 1u64;
            let a864 = v1;
            if nat_lt(a863, a864) {
                let m867 = {
                    let a865 = v0;
                    let a866 = 0u64;
                    index_bytes(a865, a866)
                };
                match m867 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a868 = v2;
                        let a869 = 48u8;
                        Ok(!equal(a868, a869))
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

pub fn f134(v0: Fn15, p874: u64, p875: (u64, u64)) -> R<((u64, u64), bool)> {
    let m873 = (p874, p875);
    {
        let mut m873 = m873;
        loop {
            let step = {
                let (v1, v2) = m873;
                let m870 = v1;
                if m870 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m870 - 1;
                    let m872 = {
                        let c871: Fn15 = v0.clone();
                        c871.apply(v2)?
                    };
                    match m872 {
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
                Ok(next) => { m873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f147(v0, v1)
}

pub fn f136(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f148(v0, v1)
}

pub fn f137(v0: Fn16, p880: u64, p881: Adt9) -> R<(Adt9, bool)> {
    let m879 = (p880, p881);
    {
        let mut m879 = m879;
        loop {
            let step = {
                let (v1, v2) = m879;
                let m876 = v1;
                if m876 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m876 - 1;
                    let m878 = {
                        let c877: Fn16 = v0.clone();
                        c877.apply(v2.clone())?
                    };
                    match m878 {
                        None => {
                            Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt9), R<(Adt9, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: u64, v1: Bytes, v2: Adt9) -> R<Option<Adt9>> {
    f149(v1, v0, v2)
}

pub fn f139(v0: Fn17, p885: List<(Bytes, Adt8)>, p886: List<(Bytes, Adt8)>) -> List<(Bytes, Adt8)> {
    let m884 = (p885, p886);
    {
        let mut m884 = m884;
        loop {
            let step = {
                let (v1, v2) = m884;
                let m882 = v2;
                match m882.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt8)>, List<(Bytes, Adt8)>), List<(Bytes, Adt8)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt8)>, List<(Bytes, Adt8)>), List<(Bytes, Adt8)>>(({
                            let c883: Fn17 = v0.clone();
                            c883.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m884 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: List<(Bytes, Adt8)>, v1: (Bytes, Adt8)) -> List<(Bytes, Adt8)> {
    List::cons(v1, v0)
}

pub fn f141(v0: Fn18, p890: List<List<(Bytes, Adt8)>>, p891: List<List<(Bytes, Adt8)>>) -> List<List<(Bytes, Adt8)>> {
    let m889 = (p890, p891);
    {
        let mut m889 = m889;
        loop {
            let step = {
                let (v1, v2) = m889;
                let m887 = v2;
                match m887.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), List<List<(Bytes, Adt8)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), List<List<(Bytes, Adt8)>>>(({
                            let c888: Fn18 = v0.clone();
                            c888.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: List<List<(Bytes, Adt8)>>, v1: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
    List::cons(v1, v0)
}

pub fn f143(v0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
    let a893 = {
        let a892 = v0.clone();
        length_list(a892)
    };
    let a894 = 1u64;
    if nat_le(a893, a894) {
        Ok(None::<List<List<(Bytes, Adt8)>>>)
    } else {
        Ok(Some(f150(v0)?))
    }
}

pub fn f144(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f112(v0, v1)?;
    let a895 = 47u64;
    let a896 = v2;
    if nat_lt(a895, a896) {
        let a897 = v2;
        let a898 = 58u64;
        if nat_lt(a897, a898) {
            Ok(Some({
                let a899 = v1;
                let a900 = 1u64;
                nat_add(a899, a900)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f145(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a901 = f112(v0, v1)?;
    let a902 = 48u64;
    if nat_eq(a901, a902) {
        Ok(Some({
            let a903 = v1;
            let a904 = 1u64;
            nat_add(a903, a904)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f146(v0: Bytes) -> R<u64> {
    let (h905, _) = f49(Fn5::F151(v0.clone()), {
        let a906 = v0;
        length_bytes(a906)
    }, 0u64)?;
    Ok(h905)
}

pub fn f147(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m910 = {
        let a908 = v0;
        let a909 = {
            let (h907, _) = v1;
            h907
        };
        index_bytes(a908, a909)
    };
    match m910 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a912 = {
                    let (h911, _) = v1;
                    h911
                };
                let a913 = 1u64;
                nat_add(a912, a913)?
            }, {
                let a917 = {
                    let a915 = {
                        let (_, h914) = v1;
                        h914
                    };
                    let a916 = 10u64;
                    nat_mul(a915, a916)?
                };
                let a918 = f152(v2)?;
                nat_add(a917, a918)?
            })))
        }
    }
}

pub fn f148(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f153(f112(v0, v1)?) {
        Ok(Some({
            let a919 = v1;
            let a920 = 1u64;
            nat_add(a919, a920)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f149(v0: Bytes, v1: u64, v2: Adt9) -> R<Option<Adt9>> {
    let v3: u64 = match v2.clone() {
        Adt9::C0(h921, _) => {
            h921
        }
    };
    let a922 = v3;
    let a923 = v1;
    if nat_lt(a922, a923) {
        let v4: u64 = f62(v0.clone(), v3)?;
        let a924 = v4;
        let a925 = 32u64;
        if if nat_lt(a924, a925) {
            true
        } else {
            let a926 = v4;
            let a927 = 34u64;
            if nat_eq(a926, a927) {
                true
            } else {
                let a928 = v4;
                let a929 = 92u64;
                nat_eq(a928, a929)
            }
        } {
            Ok(Some(Adt9::C0({
                let a930 = v3;
                let a931 = 1u64;
                nat_add(a930, a931)?
            }, List::cons(f154(v4), match v2 {
                Adt9::C0(_, h932) => {
                    h932
                }
            }))))
        } else {
            let v5: u64 = f64(v0.clone(), v1, v3)?;
            Ok(Some(Adt9::C0(v5, List::cons(f38(v0, v3, {
                let a933 = v5;
                let a934 = v3;
                nat_sub(a933, a934)
            }), match v2 {
                Adt9::C0(_, h935) => {
                    h935
                }
            }))))
        }
    } else {
        Ok(None::<Adt9>)
    }
}

pub fn f150(v0: List<List<(Bytes, Adt8)>>) -> R<List<List<(Bytes, Adt8)>>> {
    Ok(f126({
        let (_, h936) = {
            let (h937, _) = f155(Fn19::F156, {
                let a939 = {
                    let a938 = v0.clone();
                    length_list(a938)
                };
                let a940 = 1u64;
                nat_add(a939, a940)?
            }, (v0, List::<List<(Bytes, Adt8)>>::nil()))?;
            h937
        };
        h936
    }))
}

pub fn f151(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m943 = {
        let a941 = v0;
        let a942 = v1;
        index_bytes(a941, a942)
    };
    match m943 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f157(v2)? {
                Ok(Some({
                    let a944 = v1;
                    let a945 = 1u64;
                    nat_add(a944, a945)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f152(v0: u8) -> R<u64> {
    let a992 = {
        let v1: u8 = v0;
        let a990 = {
            let a966 = {
                let a954 = {
                    let a948 = {
                        let a946 = v1;
                        let a947 = 128u8;
                        fixed_u8::bit_and(a946, a947)
                    };
                    let a949 = 128u8;
                    if equal(a948, a949) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a955 = {
                    let a952 = {
                        let a950 = v1;
                        let a951 = 64u8;
                        fixed_u8::bit_and(a950, a951)
                    };
                    let a953 = 64u8;
                    if equal(a952, a953) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a954, a955)?
            };
            let a967 = {
                let a964 = {
                    let a958 = {
                        let a956 = v1;
                        let a957 = 32u8;
                        fixed_u8::bit_and(a956, a957)
                    };
                    let a959 = 32u8;
                    if equal(a958, a959) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a965 = {
                    let a962 = {
                        let a960 = v1;
                        let a961 = 16u8;
                        fixed_u8::bit_and(a960, a961)
                    };
                    let a963 = 16u8;
                    if equal(a962, a963) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a964, a965)?
            };
            nat_add(a966, a967)?
        };
        let a991 = {
            let a988 = {
                let a976 = {
                    let a970 = {
                        let a968 = v1;
                        let a969 = 8u8;
                        fixed_u8::bit_and(a968, a969)
                    };
                    let a971 = 8u8;
                    if equal(a970, a971) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a977 = {
                    let a974 = {
                        let a972 = v1;
                        let a973 = 4u8;
                        fixed_u8::bit_and(a972, a973)
                    };
                    let a975 = 4u8;
                    if equal(a974, a975) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a976, a977)?
            };
            let a989 = {
                let a986 = {
                    let a980 = {
                        let a978 = v1;
                        let a979 = 2u8;
                        fixed_u8::bit_and(a978, a979)
                    };
                    let a981 = 2u8;
                    if equal(a980, a981) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a987 = {
                    let a984 = {
                        let a982 = v1;
                        let a983 = 1u8;
                        fixed_u8::bit_and(a982, a983)
                    };
                    let a985 = 1u8;
                    if equal(a984, a985) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a986, a987)?
            };
            nat_add(a988, a989)?
        };
        nat_add(a990, a991)?
    };
    let a993 = 48u64;
    Ok(nat_sub(a992, a993))
}

pub fn f153(v0: u64) -> bool {
    if f117(v0) {
        true
    } else {
        let a994 = 47u64;
        let a995 = v0;
        if if nat_lt(a994, a995) {
            let a996 = v0;
            let a997 = 58u64;
            nat_lt(a996, a997)
        } else {
            false
        } {
            true
        } else {
            let a998 = v0;
            let a999 = 46u64;
            if nat_eq(a998, a999) {
                true
            } else {
                let a1000 = v0;
                let a1001 = 39u64;
                if nat_eq(a1000, a1001) {
                    true
                } else {
                    let a1002 = v0;
                    let a1003 = 47u64;
                    if nat_eq(a1002, a1003) {
                        true
                    } else {
                        let a1004 = v0;
                        let a1005 = 45u64;
                        nat_eq(a1004, a1005)
                    }
                }
            }
        }
    }
}

pub fn f154(v0: u64) -> Bytes {
    let a1006 = v0;
    let a1007 = 34u64;
    if nat_eq(a1006, a1007) {
        f36(Str::lit("\\\""))
    } else {
        let a1008 = v0;
        let a1009 = 92u64;
        if nat_eq(a1008, a1009) {
            f36(Str::lit("\\\\"))
        } else {
            let a1010 = v0;
            let a1011 = 8u64;
            if nat_eq(a1010, a1011) {
                f36(Str::lit("\\b"))
            } else {
                let a1012 = v0;
                let a1013 = 12u64;
                if nat_eq(a1012, a1013) {
                    f36(Str::lit("\\f"))
                } else {
                    let a1014 = v0;
                    let a1015 = 10u64;
                    if nat_eq(a1014, a1015) {
                        f36(Str::lit("\\n"))
                    } else {
                        let a1016 = v0;
                        let a1017 = 13u64;
                        if nat_eq(a1016, a1017) {
                            f36(Str::lit("\\r"))
                        } else {
                            let a1018 = v0;
                            let a1019 = 9u64;
                            if nat_eq(a1018, a1019) {
                                f36(Str::lit("\\t"))
                            } else {
                                f71(f36(Str::lit("\\u00")), f158(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f155(v0: Fn19, p1024: u64, p1025: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)> {
    let m1023 = (p1024, p1025);
    {
        let mut m1023 = m1023;
        loop {
            let step = {
                let (v1, v2) = m1023;
                let m1020 = v1;
                if m1020 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1020 - 1;
                    let m1022 = {
                        let c1021: Fn19 = v0.clone();
                        c1021.apply(v2.clone())?
                    };
                    match m1022 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1023 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
    f159(v0)
}

pub fn f157(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1070 = {
            let a1046 = {
                let a1034 = {
                    let a1028 = {
                        let a1026 = v1;
                        let a1027 = 128u8;
                        fixed_u8::bit_and(a1026, a1027)
                    };
                    let a1029 = 128u8;
                    if equal(a1028, a1029) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1035 = {
                    let a1032 = {
                        let a1030 = v1;
                        let a1031 = 64u8;
                        fixed_u8::bit_and(a1030, a1031)
                    };
                    let a1033 = 64u8;
                    if equal(a1032, a1033) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1034, a1035)?
            };
            let a1047 = {
                let a1044 = {
                    let a1038 = {
                        let a1036 = v1;
                        let a1037 = 32u8;
                        fixed_u8::bit_and(a1036, a1037)
                    };
                    let a1039 = 32u8;
                    if equal(a1038, a1039) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1045 = {
                    let a1042 = {
                        let a1040 = v1;
                        let a1041 = 16u8;
                        fixed_u8::bit_and(a1040, a1041)
                    };
                    let a1043 = 16u8;
                    if equal(a1042, a1043) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1044, a1045)?
            };
            nat_add(a1046, a1047)?
        };
        let a1071 = {
            let a1068 = {
                let a1056 = {
                    let a1050 = {
                        let a1048 = v1;
                        let a1049 = 8u8;
                        fixed_u8::bit_and(a1048, a1049)
                    };
                    let a1051 = 8u8;
                    if equal(a1050, a1051) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1057 = {
                    let a1054 = {
                        let a1052 = v1;
                        let a1053 = 4u8;
                        fixed_u8::bit_and(a1052, a1053)
                    };
                    let a1055 = 4u8;
                    if equal(a1054, a1055) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1056, a1057)?
            };
            let a1069 = {
                let a1066 = {
                    let a1060 = {
                        let a1058 = v1;
                        let a1059 = 2u8;
                        fixed_u8::bit_and(a1058, a1059)
                    };
                    let a1061 = 2u8;
                    if equal(a1060, a1061) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1067 = {
                    let a1064 = {
                        let a1062 = v1;
                        let a1063 = 1u8;
                        fixed_u8::bit_and(a1062, a1063)
                    };
                    let a1065 = 1u8;
                    if equal(a1064, a1065) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1066, a1067)?
            };
            nat_add(a1068, a1069)?
        };
        nat_add(a1070, a1071)?
    };
    let a1072 = 48u64;
    let a1073 = v2;
    if nat_le(a1072, a1073) {
        let a1074 = v2;
        let a1075 = 57u64;
        Ok(nat_le(a1074, a1075))
    } else {
        Ok(false)
    }
}

pub fn f158(v0: u64) -> Bytes {
    f71(f99(v0, 16u64), f100(v0))
}

pub fn f159(v0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
    let v1: List<List<(Bytes, Adt8)>> = {
        let (h1076, _) = v0;
        h1076
    };
    let v2: List<List<(Bytes, Adt8)>> = {
        let (_, h1077) = v0;
        h1077
    };
    let m1078 = v1;
    match m1078.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>)
        }
        Some((v3, v4)) => {
            let m1079 = v4;
            match m1079.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt8)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f160(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f160(v0: List<(Bytes, Adt8)>, v1: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    let v2: u64 = {
        let a1084 = {
            let a1082 = {
                let a1080 = v0.clone();
                length_list(a1080)
            };
            let a1083 = {
                let a1081 = v1.clone();
                length_list(a1081)
            };
            nat_add(a1082, a1083)?
        };
        let a1085 = 1u64;
        nat_add(a1084, a1085)?
    };
    Ok(f125({
        let (_, h1086) = {
            let (_, h1087) = {
                let (h1088, _) = f161(Fn20::F162, v2, (v0, (v1, List::<(Bytes, Adt8)>::nil())));
                h1088
            };
            h1087
        };
        h1086
    }))
}

pub fn f161(v0: Fn20, p1093: u64, p1094: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool) {
    let m1092 = (p1093, p1094);
    {
        let mut m1092 = m1092;
        loop {
            let step = {
                let (v1, v2) = m1092;
                let m1089 = v1;
                if m1089 == 0 {
                    Err::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v2, false))
                } else {
                    let v3 = m1089 - 1;
                    let m1091 = {
                        let c1090: Fn20 = v0.clone();
                        c1090.apply(v2.clone())
                    };
                    match m1091 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1092 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
    f163(v0)
}

pub fn f163(v0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
    let v1: List<(Bytes, Adt8)> = {
        let (h1095, _) = v0.clone();
        h1095
    };
    let v2: List<(Bytes, Adt8)> = {
        let (h1096, _) = {
            let (_, h1097) = v0.clone();
            h1097
        };
        h1096
    };
    let v3: List<(Bytes, Adt8)> = {
        let (_, h1098) = {
            let (_, h1099) = v0;
            h1099
        };
        h1098
    };
    let m1100 = v1.clone();
    match m1100.uncons() {
        None => {
            let m1101 = v2;
            match m1101.uncons() {
                None => {
                    None::<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt8)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1102 = v2.clone();
            match m1102.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt8)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f164({
                        let (h1103, _) = v8.clone();
                        h1103
                    }, {
                        let (h1104, _) = v6.clone();
                        h1104
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

pub fn f164(v0: Bytes, v1: Bytes) -> bool {
    let a1109 = {
        let a1105 = v0;
        let a1106 = v1;
        compare_bytes(a1105, a1106)
    };
    let a1110 = {
        let a1107 = Bytes::lit(&[]);
        let a1108 = Bytes::lit(&[0x01]);
        compare_bytes(a1107, a1108)
    };
    equal(a1109, a1110)
}
