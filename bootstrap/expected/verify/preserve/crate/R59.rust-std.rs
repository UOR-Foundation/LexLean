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
    C0(Adt1, u64, bool, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Adt2>),
    C1,
    C2,
    C3(Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, bool, u64, List<Adt0>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, List<Adt2>, bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn0::F2(k0, k1) => f2(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F6,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn1::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F22(u64, Bytes),
    F49(Bytes),
    F60(Bytes),
    F66(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F22(k0, k1) => f22(*k0, k1.clone(), p0),
            Fn2::F49(k0) => f49(k0.clone(), p0),
            Fn2::F60(k0) => f60(k0.clone(), p0),
            Fn2::F66(k0) => f66(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F24(Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn3::F24(k0, k1) => f24(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F26(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn4::F26(k0, k1) => f26(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F42(u64, Bytes),
    F67(u64, u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn5::F42(k0, k1) => f42(*k0, k1.clone(), p0),
            Fn5::F67(k0, k1, k2) => f67(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F57(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
        match self {
            Fn6::F57(k0, k1) => f57(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F59(u64, u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
        match self {
            Fn7::F59(k0, k1, k2) => f59(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F64(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn8::F64(k0) => f64(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F75,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn9::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F83(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn10::F83(k0) => f83(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F85(u64, u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn11::F85(k0, k1, k2) => f85(*k0, *k1, k2.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Result<List<Adt0>, Bytes>> {
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let v2: Adt3 = {
        let (h2, _) = f1(Fn0::F2(v1, v0), {
            let a3 = v1;
            let a4 = 1u64;
            nat_add(a3, a4)?
        }, Adt3::C0(0u64, 0u64, true, 0u64, List::<Adt0>::nil(), None::<Bytes>))?;
        h2
    };
    let m6 = match v2.clone() {
        Adt3::C0(_, _, _, _, _, h5) => {
            h5
        }
    };
    match m6 {
        None => {
            let v4: List<Adt0> = f3(match v2 {
                Adt3::C0(_, _, _, _, h7, _) => {
                    h7
                }
            });
            Ok(Ok::<List<Adt0>, Bytes>(v4))
        }
        Some(v3) => {
            Ok(Err::<List<Adt0>, Bytes>(v3))
        }
    }
}

pub fn f1(v0: Fn0, p12: u64, p13: Adt3) -> R<(Adt3, bool)> {
    let m11 = (p12, p13);
    {
        let mut m11 = m11;
        loop {
            let step = {
                let (v1, v2) = m11;
                let m8 = v1;
                if m8 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m8 - 1;
                    let m10 = {
                        let c9: Fn0 = v0.clone();
                        c9.apply(v2.clone())?
                    };
                    match m10 {
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
                Ok(next) => { m11 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: u64, v1: Bytes, v2: Adt3) -> R<Option<Adt3>> {
    f4(v1, v0, v2)
}

pub fn f3(v0: List<Adt0>) -> List<Adt0> {
    f5(Fn1::F6, List::<Adt0>::nil(), v0)
}

pub fn f4(v0: Bytes, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let m15 = match v2.clone() {
        Adt3::C0(_, _, _, _, _, h14) => {
            h14
        }
    };
    match m15 {
        None => {
            let v4: Option<Adt3> = f7(v0, match v2.clone() {
                Adt3::C0(h16, _, _, _, _, _) => {
                    h16
                }
            }, match v2.clone() {
                Adt3::C0(_, h17, _, _, _, _) => {
                    h17
                }
            }, match v2.clone() {
                Adt3::C0(_, _, h18, _, _, _) => {
                    h18
                }
            }, match v2.clone() {
                Adt3::C0(_, _, _, h19, _, _) => {
                    h19
                }
            }, match v2.clone() {
                Adt3::C0(_, _, _, _, h20, _) => {
                    h20
                }
            })?;
            let a22 = match v2 {
                Adt3::C0(h21, _, _, _, _, _) => {
                    h21
                }
            };
            let a23 = v1;
            if nat_lt(a22, a23) {
                Ok(v4)
            } else {
                Ok(None::<Adt3>)
            }
        }
        Some(_) => {
            Ok(None::<Adt3>)
        }
    }
}

pub fn f5(v0: Fn1, p27: List<Adt0>, p28: List<Adt0>) -> List<Adt0> {
    let m26 = (p27, p28);
    {
        let mut m26 = m26;
        loop {
            let step = {
                let (v1, v2) = m26;
                let m24 = v2;
                match m24.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c25: Fn1 = v0.clone();
                            c25.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m26 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt0>) -> R<Option<Adt3>> {
    let v6: u8 = f8(v0.clone(), v1);
    let v7: bool = f9(v0.clone(), {
        let a29 = v1;
        let a30 = 1u64;
        nat_add(a29, a30)?
    }, 45u8);
    let a31 = v6;
    let a32 = 10u8;
    if equal(a31, a32) {
        Ok(Some(Adt3::C0({
            let a33 = v1;
            let a34 = 1u64;
            nat_add(a33, a34)?
        }, {
            let a35 = v2;
            let a36 = 1u64;
            nat_add(a35, a36)?
        }, true, {
            let a37 = v4;
            let a38 = 1u64;
            nat_add(a37, a38)?
        }, v5, None::<Bytes>)))
    } else {
        let a39 = v6;
        let a40 = 32u8;
        if if equal(a39, a40) {
            true
        } else {
            let a41 = v6;
            let a42 = 9u8;
            if equal(a41, a42) {
                true
            } else {
                let a43 = v6;
                let a44 = 13u8;
                equal(a43, a44)
            }
        } {
            Ok(Some(Adt3::C0({
                let a45 = v1;
                let a46 = 1u64;
                nat_add(a45, a46)?
            }, v2, v3, {
                let a47 = v4;
                let a48 = 1u64;
                nat_add(a47, a48)?
            }, v5, None::<Bytes>)))
        } else {
            let m49 = f10(v0.clone(), v1)?;
            match m49 {
                None => {
                    let a50 = v6;
                    let a51 = 45u8;
                    if if equal(a50, a51) {
                        v7
                    } else {
                        false
                    } {
                        let v10: u64 = f13(v0.clone(), v1)?;
                        Ok(Some(Adt3::C0(v10, v2, v3, {
                            let a52 = v4;
                            let a53 = f14(v0, v1, v10)?;
                            nat_add(a52, a53)?
                        }, v5, None::<Bytes>)))
                    } else {
                        let a54 = v6;
                        let a55 = 47u8;
                        if if equal(a54, a55) {
                            v7
                        } else {
                            false
                        } {
                            let v11: Adt4 = f15(v0.clone(), {
                                let a56 = v1;
                                let a57 = 2u64;
                                nat_add(a56, a57)?
                            }, v2)?;
                            if match v11.clone() {
                                Adt4::C0(_, _, _, h58) => {
                                    h58
                                }
                            } {
                                Ok(Some(f12(v1, v2, v3, v4, v5, {
                                    let a59 = Str::lit("a comment is not closed");
                                    utf8_encode(a59)
                                })))
                            } else {
                                Ok(Some(Adt3::C0(match v11.clone() {
                                    Adt4::C0(h60, _, _, _) => {
                                        h60
                                    }
                                }, match v11.clone() {
                                    Adt4::C0(_, h61, _, _) => {
                                        h61
                                    }
                                }, v3, {
                                    let a63 = v4;
                                    let a64 = f14(v0, v1, match v11 {
                                        Adt4::C0(h62, _, _, _) => {
                                            h62
                                        }
                                    })?;
                                    nat_add(a63, a64)?
                                }, v5, None::<Bytes>)))
                            }
                        } else {
                            Ok(Some(f16(v0, v6, v1, v2, v3, v4, v5)?))
                        }
                    }
                }
                Some(v8) => {
                    let v9: Bytes = f11(v8);
                    Ok(Some(f12(v1, v2, v3, v4, v5, v9)))
                }
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: u64) -> u8 {
    let m65 = f17(v0, v1);
    match m65 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f9(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m66 = f17(v0, v1);
    match m66 {
        None => {
            false
        }
        Some(v3) => {
            let a67 = v3;
            let a68 = v2;
            equal(a67, a68)
        }
    }
}

pub fn f10(v0: Bytes, v1: u64) -> R<Option<u8>> {
    let a70 = v1;
    let a71 = {
        let a69 = v0.clone();
        length_bytes(a69)
    };
    if nat_lt(a70, a71) {
        let v2: u8 = f8(v0.clone(), v1);
        if f18(v2, 0u8, 31u8) {
            Ok(Some(v2))
        } else {
            let a72 = v2;
            let a73 = 127u8;
            if equal(a72, a73) {
                Ok(Some(v2))
            } else {
                let a74 = v2;
                let a75 = 194u8;
                if equal(a74, a75) {
                    if f19(v0.clone(), {
                        let a76 = v1;
                        let a77 = 1u64;
                        nat_add(a76, a77)?
                    }, 128u8, 159u8) {
                        Ok(Some(f8(v0, {
                            let a78 = v1;
                            let a79 = 1u64;
                            nat_add(a78, a79)?
                        })))
                    } else {
                        Ok(None::<u8>)
                    }
                } else {
                    Ok(None::<u8>)
                }
            }
        }
    } else {
        Ok(None::<u8>)
    }
}

pub fn f11(v0: u8) -> Bytes {
    let v2: u8 = {
        let m82 = {
            let a80 = v0;
            let a81 = 4u32;
            fixed_u8::shift_right(a80, a81)
        };
        match m82 {
            None => {
                0u8
            }
            Some(v1) => {
                v1
            }
        }
    };
    let v3: u8 = {
        let a83 = v0;
        let a84 = 15u8;
        fixed_u8::bit_and(a83, a84)
    };
    let v4: Bytes = f20(v2);
    let v5: Bytes = f20(v3);
    let a91 = {
        let a88 = {
            let a85 = Str::lit("a control character U+00");
            utf8_encode(a85)
        };
        let a89 = {
            let a86 = v4;
            let a87 = v5;
            append_bytes(a86, a87)
        };
        append_bytes(a88, a89)
    };
    let a92 = {
        let a90 = Str::lit(" in the text");
        utf8_encode(a90)
    };
    append_bytes(a91, a92)
}

pub fn f12(v0: u64, v1: u64, v2: bool, v3: u64, v4: List<Adt0>, v5: Bytes) -> Adt3 {
    Adt3::C0(v0, v1, v2, v3, v4, Some(v5))
}

pub fn f13(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a93 = v0.clone();
        length_bytes(a93)
    };
    let (h94, _) = f21(Fn2::F22(v2, v0), {
        let a95 = v2;
        let a96 = v1;
        nat_sub(a95, a96)
    }, v1)?;
    Ok(h94)
}

pub fn f14(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h97) = {
        let (h98, _) = f23(Fn3::F24(v0, v2), {
            let a99 = v2;
            let a100 = v1;
            nat_sub(a99, a100)
        }, (v1, 0u64))?;
        h98
    };
    Ok(h97)
}

pub fn f15(v0: Bytes, v1: u64, v2: u64) -> R<Adt4> {
    let v3: u64 = {
        let a101 = v0.clone();
        length_bytes(a101)
    };
    let (h102, _) = f25(Fn4::F26(v3, v0), {
        let a105 = {
            let a103 = v3;
            let a104 = v1;
            nat_sub(a103, a104)
        };
        let a106 = 1u64;
        nat_add(a105, a106)?
    }, Adt4::C0(v1, v2, 1u64, false))?;
    Ok(h102)
}

pub fn f16(v0: Bytes, v1: u8, v2: u64, v3: u64, v4: bool, v5: u64, v6: List<Adt0>) -> R<Adt3> {
    let a107 = v1;
    let a108 = 34u8;
    if equal(a107, a108) {
        let v7: Adt5 = f27(v0.clone(), {
            let a109 = v2;
            let a110 = 1u64;
            nat_add(a109, a110)?
        }, v3)?;
        if match v7.clone() {
            Adt5::C0(_, _, _, h111) => {
                h111
            }
        } {
            Ok(f12(v2, v3, false, v5, v6, {
                let a112 = Str::lit("a string is not closed");
                utf8_encode(a112)
            }))
        } else {
            f28(match v7.clone() {
                Adt5::C0(h113, _, _, _) => {
                    h113
                }
            }, match v7.clone() {
                Adt5::C0(_, h114, _, _) => {
                    h114
                }
            }, v5, f14(v0, v2, match v7 {
                Adt5::C0(h115, _, _, _) => {
                    h115
                }
            })?, Adt1::C2, v3, v4, v6)
        }
    } else {
        let a116 = v1;
        let a117 = 39u8;
        if equal(a116, a117) {
            let m118 = f29(v0.clone(), v2)?;
            match m118 {
                None => {
                    Ok(f12(v2, v3, false, v5, v6, {
                        let a119 = Str::lit("a character literal is not closed");
                        utf8_encode(a119)
                    }))
                }
                Some(v8) => {
                    f28(v8, v3, v5, f14(v0, v2, v8)?, Adt1::C2, v3, v4, v6)
                }
            }
        } else {
            if f30(v1) {
                let m120 = f31(v0.clone(), v2)?;
                match m120 {
                    Ok(v10) => {
                        f28(v10, v3, v5, f14(v0, v2, v10)?, Adt1::C1, v3, v4, v6)
                    }
                    Err(v9) => {
                        Ok(f12(v2, v3, false, v5, v6, v9))
                    }
                }
            } else {
                let v11: u64 = f32(v0.clone(), {
                    let a121 = v2;
                    let a122 = 1u64;
                    nat_add(a121, a122)?
                })?;
                let a123 = v1;
                let a124 = 35u8;
                if if equal(a123, a124) {
                    let a125 = 0u64;
                    let a126 = v11;
                    nat_lt(a125, a126)
                } else {
                    false
                } {
                    let v12: u64 = f33(v0.clone(), {
                        let a129 = {
                            let a127 = v2;
                            let a128 = 1u64;
                            nat_add(a127, a128)?
                        };
                        let a130 = v11;
                        nat_add(a129, a130)?
                    })?;
                    f28(v12, v3, v5, f14(v0.clone(), v2, v12)?, Adt1::C3(f34(v0, v2, {
                        let a131 = v12;
                        let a132 = v2;
                        nat_sub(a131, a132)
                    })), v3, v4, v6)
                } else {
                    if f35(v0.clone(), v2)? {
                        f36(v0, v2, v3, v4, v5, v6)
                    } else {
                        let v13: u64 = f37(v0.clone(), v2);
                        f28({
                            let a133 = v2;
                            let a134 = v13;
                            nat_add(a133, a134)?
                        }, v3, v5, 1u64, Adt1::C4(f34(v0, v2, v13)), v3, v4, v6)
                    }
                }
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: u64) -> Option<u8> {
    let a135 = v0;
    let a136 = v1;
    index_bytes(a135, a136)
}

pub fn f18(v0: u8, v1: u8, v2: u8) -> bool {
    let m139 = {
        let a137 = v0;
        let a138 = v1;
        fixed_u8::checked_sub(a137, a138)
    };
    match m139 {
        None => {
            false
        }
        Some(_) => {
            let m142 = {
                let a140 = v2;
                let a141 = v0;
                fixed_u8::checked_sub(a140, a141)
            };
            match m142 {
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

pub fn f19(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m143 = f17(v0, v1);
    match m143 {
        None => {
            false
        }
        Some(v4) => {
            f18(v4, v2, v3)
        }
    }
}

pub fn f20(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a144 = Str::lit("0123456789ABCDEF");
        utf8_encode(a144)
    };
    let v2: u64 = {
        let a145 = v0;
        let a146 = 0u8;
        if equal(a145, a146) {
            0u64
        } else {
            let a147 = v0;
            let a148 = 1u8;
            if equal(a147, a148) {
                1u64
            } else {
                let a149 = v0;
                let a150 = 2u8;
                if equal(a149, a150) {
                    2u64
                } else {
                    let a151 = v0;
                    let a152 = 3u8;
                    if equal(a151, a152) {
                        3u64
                    } else {
                        let a153 = v0;
                        let a154 = 4u8;
                        if equal(a153, a154) {
                            4u64
                        } else {
                            let a155 = v0;
                            let a156 = 5u8;
                            if equal(a155, a156) {
                                5u64
                            } else {
                                let a157 = v0;
                                let a158 = 6u8;
                                if equal(a157, a158) {
                                    6u64
                                } else {
                                    let a159 = v0;
                                    let a160 = 7u8;
                                    if equal(a159, a160) {
                                        7u64
                                    } else {
                                        let a161 = v0;
                                        let a162 = 8u8;
                                        if equal(a161, a162) {
                                            8u64
                                        } else {
                                            let a163 = v0;
                                            let a164 = 9u8;
                                            if equal(a163, a164) {
                                                9u64
                                            } else {
                                                let a165 = v0;
                                                let a166 = 10u8;
                                                if equal(a165, a166) {
                                                    10u64
                                                } else {
                                                    let a167 = v0;
                                                    let a168 = 11u8;
                                                    if equal(a167, a168) {
                                                        11u64
                                                    } else {
                                                        let a169 = v0;
                                                        let a170 = 12u8;
                                                        if equal(a169, a170) {
                                                            12u64
                                                        } else {
                                                            let a171 = v0;
                                                            let a172 = 13u8;
                                                            if equal(a171, a172) {
                                                                13u64
                                                            } else {
                                                                let a173 = v0;
                                                                let a174 = 14u8;
                                                                if equal(a173, a174) {
                                                                    14u64
                                                                } else {
                                                                    15u64
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
    };
    f34(v1, v2, 1u64)
}

pub fn f21(v0: Fn2, p179: u64, p180: u64) -> R<(u64, bool)> {
    let m178 = (p179, p180);
    {
        let mut m178 = m178;
        loop {
            let step = {
                let (v1, v2) = m178;
                let m175 = v1;
                if m175 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m175 - 1;
                    let m177 = {
                        let c176: Fn2 = v0.clone();
                        c176.apply(v2)?
                    };
                    match m177 {
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
                Ok(next) => { m178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f38(v1, v0, v2)
}

pub fn f23(v0: Fn3, p185: u64, p186: (u64, u64)) -> R<((u64, u64), bool)> {
    let m184 = (p185, p186);
    {
        let mut m184 = m184;
        loop {
            let step = {
                let (v1, v2) = m184;
                let m181 = v1;
                if m181 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m181 - 1;
                    let m183 = {
                        let c182: Fn3 = v0.clone();
                        c182.apply(v2)?
                    };
                    match m183 {
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
                Ok(next) => { m184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f39(v0, v1, v2)
}

pub fn f25(v0: Fn4, p191: u64, p192: Adt4) -> R<(Adt4, bool)> {
    let m190 = (p191, p192);
    {
        let mut m190 = m190;
        loop {
            let step = {
                let (v1, v2) = m190;
                let m187 = v1;
                if m187 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m187 - 1;
                    let m189 = {
                        let c188: Fn4 = v0.clone();
                        c188.apply(v2.clone())?
                    };
                    match m189 {
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
                Ok(next) => { m190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f40(v1, v0, v2)
}

pub fn f27(v0: Bytes, v1: u64, v2: u64) -> R<Adt5> {
    let v3: u64 = {
        let a193 = v0.clone();
        length_bytes(a193)
    };
    let (h194, _) = f41(Fn5::F42(v3, v0), {
        let a197 = {
            let a195 = v3;
            let a196 = v1;
            nat_sub(a195, a196)
        };
        let a198 = 1u64;
        nat_add(a197, a198)?
    }, Adt5::C0(v1, v2, false, false))?;
    Ok(h194)
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt1, v5: u64, v6: bool, v7: List<Adt0>) -> R<Adt3> {
    Ok(Adt3::C0(v0, v1, false, {
        let a199 = v2;
        let a200 = v3;
        nat_add(a199, a200)?
    }, List::cons(Adt0::C0(v4, v5, v6, v2), v7), None::<Bytes>))
}

pub fn f29(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a201 = v0.clone();
        length_bytes(a201)
    };
    let v3: u64 = {
        let a202 = v1;
        let a203 = 1u64;
        nat_add(a202, a203)?
    };
    let v4: u8 = f8(v0.clone(), v3);
    let v5: u64 = {
        let a204 = v3;
        let a205 = 1u64;
        nat_add(a204, a205)?
    };
    let v6: u8 = f8(v0.clone(), v5);
    let v7: u64 = {
        let a206 = v5;
        let a207 = f37(v0.clone(), v5);
        nat_add(a206, a207)?
    };
    let v8: Option<u64> = f43(v0.clone(), v7, 2u64)?;
    let v9: Option<u64> = f43(v0.clone(), v7, 4u64)?;
    let v10: u64 = f37(v0.clone(), v3);
    let v11: Option<u64> = {
        let a208 = v3;
        let a209 = v2;
        if nat_lt(a208, a209) {
            let a210 = v4;
            let a211 = 92u8;
            if equal(a210, a211) {
                let a212 = v5;
                let a213 = v2;
                if nat_lt(a212, a213) {
                    let a214 = v6;
                    let a215 = 120u8;
                    if equal(a214, a215) {
                        v8
                    } else {
                        let a216 = v6;
                        let a217 = 117u8;
                        if equal(a216, a217) {
                            v9
                        } else {
                            Some(v7)
                        }
                    }
                } else {
                    None::<u64>
                }
            } else {
                let a218 = v4;
                let a219 = 39u8;
                if equal(a218, a219) {
                    None::<u64>
                } else {
                    let a220 = v4;
                    let a221 = 10u8;
                    if equal(a220, a221) {
                        None::<u64>
                    } else {
                        Some({
                            let a222 = v3;
                            let a223 = v10;
                            nat_add(a222, a223)?
                        })
                    }
                }
            }
        } else {
            None::<u64>
        }
    };
    let m224 = v11;
    match m224 {
        None => {
            Ok(None::<u64>)
        }
        Some(v12) => {
            let v13: bool = f9(v0, v12, 39u8);
            if v13 {
                Ok(Some({
                    let a225 = v12;
                    let a226 = 1u64;
                    nat_add(a225, a226)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f30(v0: u8) -> bool {
    f18(v0, 48u8, 57u8)
}

pub fn f31(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a227 = v1;
        let a228 = 1u64;
        nat_add(a227, a228)?
    });
    let v4: u64 = {
        let a229 = v2;
        let a230 = 48u8;
        if equal(a229, a230) {
            let a231 = v3;
            let a232 = 120u8;
            if equal(a231, a232) {
                16u64
            } else {
                let a233 = v3;
                let a234 = 88u8;
                if equal(a233, a234) {
                    16u64
                } else {
                    let a235 = v3;
                    let a236 = 98u8;
                    if equal(a235, a236) {
                        2u64
                    } else {
                        let a237 = v3;
                        let a238 = 66u8;
                        if equal(a237, a238) {
                            2u64
                        } else {
                            let a239 = v3;
                            let a240 = 111u8;
                            if equal(a239, a240) {
                                8u64
                            } else {
                                let a241 = v3;
                                let a242 = 79u8;
                                if equal(a241, a242) {
                                    8u64
                                } else {
                                    10u64
                                }
                            }
                        }
                    }
                }
            }
        } else {
            10u64
        }
    };
    let v5: Result<u64, Bytes> = f44(v0.clone(), {
        let a243 = v1;
        let a244 = 2u64;
        nat_add(a243, a244)?
    }, v4, true)?;
    let v6: u64 = f45(v0.clone(), v1)?;
    let v9: Result<u64, Bytes> = {
        let m247 = f44(v0.clone(), {
            let a245 = v1;
            let a246 = 1u64;
            nat_add(a245, a246)?
        }, 10u64, false)?;
        match m247 {
            Ok(v8) => {
                f46(v0.clone(), v8)?
            }
            Err(v7) => {
                Err::<u64, Bytes>(v7)
            }
        }
    };
    let a250 = {
        let a248 = v4;
        let a249 = 10u64;
        nat_eq(a248, a249)
    };
    if bool_not(a250) {
        Ok(v5)
    } else {
        let a253 = {
            let a251 = v2;
            let a252 = 48u8;
            equal(a251, a252)
        };
        if if bool_not(a253) {
            let a254 = 0u64;
            let a255 = v1;
            if nat_lt(a254, a255) {
                f9(v0.clone(), {
                    let a256 = v1;
                    let a257 = 1u64;
                    nat_sub(a256, a257)
                }, 46u8)
            } else {
                false
            }
        } else {
            false
        } {
            if if f9(v0.clone(), v6, 95u8) {
                true
            } else {
                if f9(v0.clone(), v6, 101u8) {
                    true
                } else {
                    f9(v0, v6, 69u8)
                }
            } {
                Ok(Err::<u64, Bytes>({
                    let a258 = Str::lit("a field index is followed by what could continue a number");
                    utf8_encode(a258)
                }))
            } else {
                Ok(Ok::<u64, Bytes>(v6))
            }
        } else {
            Ok(v9)
        }
    }
}

pub fn f32(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: bool = f47(v2);
    let v4: bool = {
        let a259 = v2;
        let a260 = 95u8;
        equal(a259, a260)
    };
    let v5: u64 = f48(v0.clone(), v1)?;
    let a262 = v1;
    let a263 = {
        let a261 = v0;
        length_bytes(a261)
    };
    if nat_lt(a262, a263) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                Ok(v5)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f33(v0: Bytes, v1: u64) -> R<u64> {
    let (h264, _) = f21(Fn2::F49(v0.clone()), {
        let a266 = {
            let a265 = v0;
            length_bytes(a265)
        };
        let a267 = v1;
        nat_sub(a266, a267)
    }, v1)?;
    Ok(h264)
}

pub fn f34(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m271 = {
        let a268 = v0;
        let a269 = v1;
        let a270 = v2;
        slice_bytes(a268, a269, a270)
    };
    match m271 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f35(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f32(v0.clone(), v1)?;
    let v3: bool = f50(v0, v1)?;
    let a272 = 0u64;
    let a273 = v2;
    if nat_lt(a272, a273) {
        Ok(true)
    } else {
        Ok(v3)
    }
}

pub fn f36(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt0>) -> R<Adt3> {
    let m274 = f51(v0.clone(), v1)?;
    match m274 {
        Ok(v7) => {
            let m276 = {
                let (h275, _) = v7.clone();
                h275
            };
            match m276.uncons() {
                None => {
                    Ok(Adt3::C0({
                        let (_, h277) = v7;
                        h277
                    }, v2, false, v4, v5, None::<Bytes>))
                }
                Some((v8, v9)) => {
                    let m278 = v9;
                    match m278.uncons() {
                        None => {
                            let v10: bool = {
                                let a280 = match v8.clone() {
                                    Adt2::C0(_, h279) => {
                                        h279
                                    }
                                };
                                if bool_not(a280) {
                                    let a283 = match v8.clone() {
                                        Adt2::C0(h281, _) => {
                                            h281
                                        }
                                    };
                                    let a284 = {
                                        let a282 = Str::lit("r");
                                        utf8_encode(a282)
                                    };
                                    equal(a283, a284)
                                } else {
                                    false
                                }
                            };
                            let v11: u64 = if v10 {
                                f52(v0.clone(), {
                                    let (_, h285) = v7.clone();
                                    h285
                                })?
                            } else {
                                0u64
                            };
                            if if v10 {
                                f9(v0.clone(), {
                                    let a287 = {
                                        let (_, h286) = v7.clone();
                                        h286
                                    };
                                    let a288 = v11;
                                    nat_add(a287, a288)?
                                }, 34u8)
                            } else {
                                false
                            } {
                                let v12: Adt5 = f53(v0.clone(), {
                                    let a292 = {
                                        let a290 = {
                                            let (_, h289) = v7;
                                            h289
                                        };
                                        let a291 = v11;
                                        nat_add(a290, a291)?
                                    };
                                    let a293 = 1u64;
                                    nat_add(a292, a293)?
                                }, v2, v11)?;
                                if match v12.clone() {
                                    Adt5::C0(_, _, _, h294) => {
                                        h294
                                    }
                                } {
                                    Ok(f12(v1, v2, false, v4, v5, {
                                        let a295 = Str::lit("a raw string is not closed");
                                        utf8_encode(a295)
                                    }))
                                } else {
                                    f28(match v12.clone() {
                                        Adt5::C0(h296, _, _, _) => {
                                            h296
                                        }
                                    }, match v12.clone() {
                                        Adt5::C0(_, h297, _, _) => {
                                            h297
                                        }
                                    }, v4, f14(v0, v1, match v12 {
                                        Adt5::C0(h298, _, _, _) => {
                                            h298
                                        }
                                    })?, Adt1::C2, v2, v3, v5)
                                }
                            } else {
                                let v13: bool = f54(match v8.clone() {
                                    Adt2::C0(h299, _) => {
                                        h299
                                    }
                                }, {
                                    let a300 = Str::lit("!");
                                    utf8_encode(a300)
                                });
                                let v14: bool = f9(v0.clone(), {
                                    let (_, h301) = v7.clone();
                                    h301
                                }, 34u8);
                                let a303 = match v8 {
                                    Adt2::C0(_, h302) => {
                                        h302
                                    }
                                };
                                if if bool_not(a303) {
                                    if v13 {
                                        v14
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                } {
                                    Ok(f12(v1, v2, false, v4, v5, {
                                        let a304 = Str::lit("an interpolated string holds code");
                                        utf8_encode(a304)
                                    }))
                                } else {
                                    f28({
                                        let (_, h305) = v7.clone();
                                        h305
                                    }, v2, v4, f14(v0, v1, {
                                        let (_, h306) = v7;
                                        h306
                                    })?, Adt1::C0({
                                        let (h307, _) = v7;
                                        h307
                                    }), v2, v3, v5)
                                }
                            }
                        }
                        Some((_, _)) => {
                            f28({
                                let (_, h308) = v7.clone();
                                h308
                            }, v2, v4, f14(v0, v1, {
                                let (_, h309) = v7;
                                h309
                            })?, Adt1::C0({
                                let (h310, _) = v7;
                                h310
                            }), v2, v3, v5)
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(f12(v1, v2, false, v4, v5, v6))
        }
    }
}

pub fn f37(v0: Bytes, v1: u64) -> u64 {
    let v2: u8 = f8(v0, v1);
    let v3: bool = f18(v2, 0u8, 127u8);
    if v3 {
        1u64
    } else {
        let v4: bool = f18(v2, 128u8, 223u8);
        if v4 {
            2u64
        } else {
            let v5: bool = f18(v2, 224u8, 239u8);
            if v5 {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a311 = v2;
    let a312 = v1;
    if nat_lt(a311, a312) {
        let v3: bool = f9(v0, v2, 10u8);
        if v3 {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a313 = v2;
                let a314 = 1u64;
                nat_add(a313, a314)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f39(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h315, _) = v2;
        h315
    };
    let v4: u64 = {
        let (_, h316) = v2;
        h316
    };
    let a317 = v3;
    let a318 = v1;
    if nat_lt(a317, a318) {
        let v5: u64 = f37(v0, v3);
        Ok(Some(({
            let a319 = v3;
            let a320 = v5;
            nat_add(a319, a320)?
        }, {
            let a321 = v4;
            let a322 = 1u64;
            nat_add(a321, a322)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h323, _, _, _) => {
            h323
        }
    };
    let v4: u64 = match v2.clone() {
        Adt4::C0(_, h324, _, _) => {
            h324
        }
    };
    let v5: u64 = match v2.clone() {
        Adt4::C0(_, _, h325, _) => {
            h325
        }
    };
    let a326 = v5;
    let a327 = 0u64;
    if nat_eq(a326, a327) {
        Ok(None::<Adt4>)
    } else {
        if match v2 {
            Adt4::C0(_, _, _, h328) => {
                h328
            }
        } {
            Ok(None::<Adt4>)
        } else {
            let a329 = v3;
            let a330 = v1;
            if nat_lt(a329, a330) {
                let v6: u8 = f8(v0.clone(), v3);
                let a331 = v6;
                let a332 = 47u8;
                if if equal(a331, a332) {
                    f9(v0.clone(), {
                        let a333 = v3;
                        let a334 = 1u64;
                        nat_add(a333, a334)?
                    }, 45u8)
                } else {
                    false
                } {
                    Ok(Some(Adt4::C0({
                        let a335 = v3;
                        let a336 = 2u64;
                        nat_add(a335, a336)?
                    }, v4, {
                        let a337 = v5;
                        let a338 = 1u64;
                        nat_add(a337, a338)?
                    }, false)))
                } else {
                    let a339 = v6;
                    let a340 = 45u8;
                    if if equal(a339, a340) {
                        f9(v0.clone(), {
                            let a341 = v3;
                            let a342 = 1u64;
                            nat_add(a341, a342)?
                        }, 47u8)
                    } else {
                        false
                    } {
                        Ok(Some(Adt4::C0({
                            let a343 = v3;
                            let a344 = 2u64;
                            nat_add(a343, a344)?
                        }, v4, {
                            let a345 = v5;
                            let a346 = 1u64;
                            nat_sub(a345, a346)
                        }, false)))
                    } else {
                        let a347 = v6;
                        let a348 = 10u8;
                        if equal(a347, a348) {
                            Ok(Some(Adt4::C0({
                                let a349 = v3;
                                let a350 = 1u64;
                                nat_add(a349, a350)?
                            }, {
                                let a351 = v4;
                                let a352 = 1u64;
                                nat_add(a351, a352)?
                            }, v5, false)))
                        } else {
                            Ok(Some(Adt4::C0({
                                let a353 = v3;
                                let a354 = f37(v0, v3);
                                nat_add(a353, a354)?
                            }, v4, v5, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt4::C0(v3, v4, v5, true)))
            }
        }
    }
}

pub fn f41(v0: Fn5, p359: u64, p360: Adt5) -> R<(Adt5, bool)> {
    let m358 = (p359, p360);
    {
        let mut m358 = m358;
        loop {
            let step = {
                let (v1, v2) = m358;
                let m355 = v1;
                if m355 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m355 - 1;
                    let m357 = {
                        let c356: Fn5 = v0.clone();
                        c356.apply(v2.clone())?
                    };
                    match m357 {
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
                Ok(next) => { m358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f55(v1, v0, v2)
}

pub fn f43(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: (u64, (u64, bool)) = {
        let (h361, _) = f56(Fn6::F57(v2, v0), v2, (v1, (0u64, true)))?;
        h361
    };
    let (_, h362) = {
        let (_, h363) = v3;
        h363
    };
    if h362 {
        Ok(Some({
            let (h364, _) = v3;
            h364
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f44(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<u64, Bytes>> {
    let v4: u64 = {
        let a365 = v0.clone();
        length_bytes(a365)
    };
    let v5: (u64, (bool, bool)) = {
        let (h366, _) = f58(Fn7::F59(v4, v2, v0), {
            let a369 = {
                let a367 = v4;
                let a368 = v1;
                nat_sub(a367, a368)
            };
            let a370 = 1u64;
            nat_add(a369, a370)?
        }, (v1, (v3, false)))?;
        h366
    };
    let (h371, _) = {
        let (_, h372) = v5;
        h372
    };
    if h371 {
        Ok(Err::<u64, Bytes>({
            let a373 = Str::lit("a number has a digit separator or a radix prefix with no digit after it");
            utf8_encode(a373)
        }))
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h374, _) = v5;
            h374
        }))
    }
}

pub fn f45(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a375 = v0.clone();
        length_bytes(a375)
    };
    let (h376, _) = f21(Fn2::F60(v0), {
        let a377 = v2;
        let a378 = v1;
        nat_sub(a377, a378)
    }, v1)?;
    Ok(h376)
}

pub fn f46(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: bool = f9(v0.clone(), v1, 46u8);
    let v3: bool = f9(v0.clone(), {
        let a379 = v1;
        let a380 = 1u64;
        nat_add(a379, a380)?
    }, 46u8);
    let v4: bool = f19(v0.clone(), {
        let a381 = v1;
        let a382 = 1u64;
        nat_add(a381, a382)?
    }, 48u8, 57u8);
    let v5: bool = f9(v0.clone(), v1, 101u8);
    let v6: bool = f9(v0.clone(), v1, 69u8);
    let v7: Result<u64, Bytes> = f61(v0.clone(), {
        let a383 = v1;
        let a384 = 1u64;
        nat_add(a383, a384)?
    }, true)?;
    let v8: Result<u64, Bytes> = f61(v0.clone(), v1, false)?;
    let v11: Result<u64, Bytes> = {
        let m387 = f44(v0.clone(), {
            let a385 = v1;
            let a386 = 1u64;
            nat_add(a385, a386)?
        }, 10u64, false)?;
        match m387 {
            Ok(v10) => {
                f61(v0, v10, false)?
            }
            Err(v9) => {
                Err::<u64, Bytes>(v9)
            }
        }
    };
    if if v2 {
        v3
    } else {
        false
    } {
        Ok(Ok::<u64, Bytes>(v1))
    } else {
        if v2 {
            if v4 {
                Ok(v11)
            } else {
                Ok(v7)
            }
        } else {
            if if v5 {
                true
            } else {
                v6
            } {
                Ok(v8)
            } else {
                Ok(Ok::<u64, Bytes>(v1))
            }
        }
    }
}

pub fn f47(v0: u8) -> bool {
    let v1: bool = f18(v0, 97u8, 122u8);
    let v2: bool = f18(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn f48(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a388 = v1;
        let a389 = 1u64;
        nat_add(a388, a389)?
    });
    let v4: u8 = f8(v0.clone(), {
        let a390 = v1;
        let a391 = 2u64;
        nat_add(a390, a391)?
    });
    let v5: u8 = f8(v0, {
        let a392 = v1;
        let a393 = 3u64;
        nat_add(a392, a393)?
    });
    let a394 = v2;
    let a395 = 206u8;
    if equal(a394, a395) {
        if f18(v3, 145u8, 169u8) {
            let a396 = v3;
            let a397 = 160u8;
            if equal(a396, a397) {
                Ok(0u64)
            } else {
                let a398 = v3;
                let a399 = 163u8;
                if equal(a398, a399) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            }
        } else {
            if f18(v3, 177u8, 191u8) {
                let a400 = v3;
                let a401 = 187u8;
                if equal(a400, a401) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            } else {
                Ok(0u64)
            }
        }
    } else {
        let a402 = v2;
        let a403 = 207u8;
        if equal(a402, a403) {
            if f18(v3, 128u8, 187u8) {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            let a404 = v2;
            let a405 = 225u8;
            if equal(a404, a405) {
                if f18(v3, 188u8, 191u8) {
                    if f18(v4, 128u8, 191u8) {
                        let a406 = v3;
                        let a407 = 191u8;
                        if equal(a406, a407) {
                            let a408 = v4;
                            let a409 = 191u8;
                            if equal(a408, a409) {
                                Ok(0u64)
                            } else {
                                Ok(3u64)
                            }
                        } else {
                            Ok(3u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            } else {
                let a410 = v2;
                let a411 = 226u8;
                if equal(a410, a411) {
                    let a412 = v3;
                    let a413 = 132u8;
                    if equal(a412, a413) {
                        if f18(v4, 128u8, 191u8) {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        let a414 = v3;
                        let a415 = 133u8;
                        if equal(a414, a415) {
                            if f18(v4, 128u8, 143u8) {
                                Ok(3u64)
                            } else {
                                Ok(0u64)
                            }
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    let a416 = v2;
                    let a417 = 240u8;
                    if equal(a416, a417) {
                        let a418 = v3;
                        let a419 = 157u8;
                        if equal(a418, a419) {
                            let a420 = v4;
                            let a421 = 146u8;
                            if equal(a420, a421) {
                                if f18(v5, 156u8, 191u8) {
                                    Ok(4u64)
                                } else {
                                    Ok(0u64)
                                }
                            } else {
                                if f18(v4, 147u8, 149u8) {
                                    if f18(v5, 128u8, 191u8) {
                                        Ok(4u64)
                                    } else {
                                        Ok(0u64)
                                    }
                                } else {
                                    let a422 = v4;
                                    let a423 = 150u8;
                                    if equal(a422, a423) {
                                        if f18(v5, 128u8, 159u8) {
                                            Ok(4u64)
                                        } else {
                                            Ok(0u64)
                                        }
                                    } else {
                                        Ok(0u64)
                                    }
                                }
                            }
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

pub fn f49(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f62(v0, v1)
}

pub fn f50(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f9(v0.clone(), v1, 194u8);
    let v3: bool = f9(v0, {
        let a424 = v1;
        let a425 = 1u64;
        nat_add(a424, a425)?
    }, 171u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}

pub fn f51(v0: Bytes, v1: u64) -> R<Result<(List<Adt2>, u64), Bytes>> {
    let v2: u64 = {
        let a426 = v0.clone();
        length_bytes(a426)
    };
    let v3: Adt6 = {
        let (h427, _) = f63(Fn8::F64(v0), {
            let a430 = {
                let a428 = v2;
                let a429 = v1;
                nat_sub(a428, a429)
            };
            let a431 = 1u64;
            nat_add(a430, a431)?
        }, Adt6::C0(v1, List::<Adt2>::nil(), false, None::<Bytes>))?;
        h427
    };
    let m433 = match v3.clone() {
        Adt6::C0(_, _, _, h432) => {
            h432
        }
    };
    match m433 {
        None => {
            let v5: List<Adt2> = f65(match v3.clone() {
                Adt6::C0(_, h434, _, _) => {
                    h434
                }
            });
            Ok(Ok::<(List<Adt2>, u64), Bytes>((v5, match v3 {
                Adt6::C0(h435, _, _, _) => {
                    h435
                }
            })))
        }
        Some(v4) => {
            Ok(Err::<(List<Adt2>, u64), Bytes>(v4))
        }
    }
}

pub fn f52(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a436 = v0.clone();
        length_bytes(a436)
    };
    let v3: u64 = {
        let (h437, _) = f21(Fn2::F66(v0), {
            let a438 = v2;
            let a439 = v1;
            nat_sub(a438, a439)
        }, v1)?;
        h437
    };
    let a440 = v3;
    let a441 = v1;
    Ok(nat_sub(a440, a441))
}

pub fn f53(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let v4: u64 = {
        let a442 = v0.clone();
        length_bytes(a442)
    };
    let (h443, _) = f41(Fn5::F67(v3, v4, v0), {
        let a446 = {
            let a444 = v4;
            let a445 = v1;
            nat_sub(a444, a445)
        };
        let a447 = 1u64;
        nat_add(a446, a447)?
    }, Adt5::C0(v1, v2, false, false))?;
    Ok(h443)
}

pub fn f54(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a448 = v0.clone();
        length_bytes(a448)
    };
    let v3: u64 = {
        let a449 = v1.clone();
        length_bytes(a449)
    };
    let a450 = v3;
    let a451 = v2;
    if nat_le(a450, a451) {
        let m457 = {
            let a454 = v0;
            let a455 = {
                let a452 = v2;
                let a453 = v3;
                nat_sub(a452, a453)
            };
            let a456 = v3;
            slice_bytes(a454, a455, a456)
        };
        match m457 {
            None => {
                false
            }
            Some(v4) => {
                let a458 = v4;
                let a459 = v1;
                equal(a458, a459)
            }
        }
    } else {
        false
    }
}

pub fn f55(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h460, _, _, _) => {
            h460
        }
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, h461, _, _) => {
            h461
        }
    };
    if match v2.clone() {
        Adt5::C0(_, _, h462, _) => {
            h462
        }
    } {
        Ok(None::<Adt5>)
    } else {
        if match v2 {
            Adt5::C0(_, _, _, h463) => {
                h463
            }
        } {
            Ok(None::<Adt5>)
        } else {
            let a464 = v3;
            let a465 = v1;
            if nat_lt(a464, a465) {
                let v5: u8 = f8(v0.clone(), v3);
                let a466 = v5;
                let a467 = 92u8;
                if equal(a466, a467) {
                    let a470 = {
                        let a468 = v3;
                        let a469 = 1u64;
                        nat_add(a468, a469)?
                    };
                    let a471 = v1;
                    if nat_lt(a470, a471) {
                        let v6: u64 = f37(v0, {
                            let a472 = v3;
                            let a473 = 1u64;
                            nat_add(a472, a473)?
                        });
                        Ok(Some(Adt5::C0({
                            let a476 = {
                                let a474 = v3;
                                let a475 = 1u64;
                                nat_add(a474, a475)?
                            };
                            let a477 = v6;
                            nat_add(a476, a477)?
                        }, v4, false, false)))
                    } else {
                        Ok(Some(Adt5::C0(v1, v4, false, true)))
                    }
                } else {
                    let a478 = v5;
                    let a479 = 34u8;
                    if equal(a478, a479) {
                        Ok(Some(Adt5::C0({
                            let a480 = v3;
                            let a481 = 1u64;
                            nat_add(a480, a481)?
                        }, v4, true, false)))
                    } else {
                        let a482 = v5;
                        let a483 = 10u8;
                        if equal(a482, a483) {
                            Ok(Some(Adt5::C0({
                                let a484 = v3;
                                let a485 = 1u64;
                                nat_add(a484, a485)?
                            }, {
                                let a486 = v4;
                                let a487 = 1u64;
                                nat_add(a486, a487)?
                            }, false, false)))
                        } else {
                            let v7: u64 = f37(v0, v3);
                            Ok(Some(Adt5::C0({
                                let a488 = v3;
                                let a489 = v7;
                                nat_add(a488, a489)?
                            }, v4, false, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt5::C0(v3, v4, false, true)))
            }
        }
    }
}

pub fn f56(v0: Fn6, p494: u64, p495: (u64, (u64, bool))) -> R<((u64, (u64, bool)), bool)> {
    let m493 = (p494, p495);
    {
        let mut m493 = m493;
        loop {
            let step = {
                let (v1, v2) = m493;
                let m490 = v1;
                if m490 == 0 {
                    Err::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m490 - 1;
                    let m492 = {
                        let c491: Fn6 = v0.clone();
                        c491.apply(v2)?
                    };
                    match m492 {
                        None => {
                            Err::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: u64, v1: Bytes, v2: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    f68(v1.clone(), {
        let a496 = v1;
        length_bytes(a496)
    }, v0, v2)
}

pub fn f58(v0: Fn7, p501: u64, p502: (u64, (bool, bool))) -> R<((u64, (bool, bool)), bool)> {
    let m500 = (p501, p502);
    {
        let mut m500 = m500;
        loop {
            let step = {
                let (v1, v2) = m500;
                let m497 = v1;
                if m497 == 0 {
                    Err::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m497 - 1;
                    let m499 = {
                        let c498: Fn7 = v0.clone();
                        c498.apply(v2)?
                    };
                    match m499 {
                        None => {
                            Err::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: u64, v1: u64, v2: Bytes, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    f69(v2, v0, v1, v3)
}

pub fn f60(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f70(v0, v1)
}

pub fn f61(v0: Bytes, v1: u64, v2: bool) -> R<Result<u64, Bytes>> {
    let v3: bool = f9(v0.clone(), v1, 101u8);
    let v4: bool = f9(v0.clone(), v1, 69u8);
    let v5: Result<u64, Bytes> = f71(v0.clone(), v1)?;
    let v6: bool = f35(v0, v1)?;
    if if v3 {
        true
    } else {
        v4
    } {
        Ok(v5)
    } else {
        if if v2 {
            v6
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>({
                let a503 = Str::lit("an identifier follows a decimal point");
                utf8_encode(a503)
            }))
        } else {
            Ok(Ok::<u64, Bytes>(v1))
        }
    }
}

pub fn f62(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f72(v0, v1)?;
    let a504 = 0u64;
    let a505 = v2;
    if nat_lt(a504, a505) {
        Ok(Some({
            let a506 = v1;
            let a507 = v2;
            nat_add(a506, a507)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f63(v0: Fn8, p512: u64, p513: Adt6) -> R<(Adt6, bool)> {
    let m511 = (p512, p513);
    {
        let mut m511 = m511;
        loop {
            let step = {
                let (v1, v2) = m511;
                let m508 = v1;
                if m508 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m508 - 1;
                    let m510 = {
                        let c509: Fn8 = v0.clone();
                        c509.apply(v2.clone())?
                    };
                    match m510 {
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
                Ok(next) => { m511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes, v1: Adt6) -> R<Option<Adt6>> {
    f73(v0, v1)
}

pub fn f65(v0: List<Adt2>) -> List<Adt2> {
    f74(Fn9::F75, List::<Adt2>::nil(), v0)
}

pub fn f66(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f76(v0, v1)
}

pub fn f67(v0: u64, v1: u64, v2: Bytes, v3: Adt5) -> R<Option<Adt5>> {
    f77(v2, v1, v0, v3)
}

pub fn f68(v0: Bytes, v1: u64, v2: u64, v3: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    let v4: u64 = {
        let (h514, _) = v3;
        h514
    };
    let v5: u64 = {
        let (h515, _) = {
            let (_, h516) = v3;
            h516
        };
        h515
    };
    let v6: bool = {
        let (_, h517) = {
            let (_, h518) = v3;
            h518
        };
        h517
    };
    if v6 {
        let a519 = v5;
        let a520 = v2;
        if nat_lt(a519, a520) {
            let a521 = v4;
            let a522 = v1;
            if nat_lt(a521, a522) {
                let v7: u64 = f37(v0, v4);
                Ok(Some(({
                    let a523 = v4;
                    let a524 = v7;
                    nat_add(a523, a524)?
                }, ({
                    let a525 = v5;
                    let a526 = 1u64;
                    nat_add(a525, a526)?
                }, true))))
            } else {
                Ok(Some((v4, ({
                    let a527 = v5;
                    let a528 = 1u64;
                    nat_add(a527, a528)?
                }, false))))
            }
        } else {
            Ok(None::<(u64, (u64, bool))>)
        }
    } else {
        Ok(None::<(u64, (u64, bool))>)
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: u64, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    let v4: u64 = {
        let (h529, _) = v3;
        h529
    };
    let v5: bool = {
        let (h530, _) = {
            let (_, h531) = v3;
            h531
        };
        h530
    };
    let v6: bool = {
        let (_, h532) = {
            let (_, h533) = v3;
            h533
        };
        h532
    };
    if v6 {
        Ok(None::<(u64, (bool, bool))>)
    } else {
        let a534 = v4;
        let a535 = v1;
        if nat_lt(a534, a535) {
            let v7: u8 = f8(v0, v4);
            let a536 = v7;
            let a537 = 95u8;
            if equal(a536, a537) {
                Ok(Some(({
                    let a538 = v4;
                    let a539 = 1u64;
                    nat_add(a538, a539)?
                }, (true, false))))
            } else {
                let v8: bool = f78(v7, v2);
                if v8 {
                    Ok(Some(({
                        let a540 = v4;
                        let a541 = 1u64;
                        nat_add(a540, a541)?
                    }, (false, false))))
                } else {
                    Ok(Some((v4, (v5, true))))
                }
            }
        } else {
            Ok(Some((v4, (v5, true))))
        }
    }
}

pub fn f70(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f19(v0, v1, 48u8, 57u8);
    if v2 {
        Ok(Some({
            let a542 = v1;
            let a543 = 1u64;
            nat_add(a542, a543)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f71(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = {
        let a544 = v1;
        let a545 = 1u64;
        nat_add(a544, a545)?
    };
    let v3: bool = f9(v0.clone(), v2, 43u8);
    let v4: bool = f9(v0.clone(), v2, 45u8);
    let v5: u64 = if v3 {
        let a546 = v2;
        let a547 = 1u64;
        nat_add(a546, a547)?
    } else {
        if v4 {
            let a548 = v2;
            let a549 = 1u64;
            nat_add(a548, a549)?
        } else {
            v2
        }
    };
    let v6: bool = f19(v0.clone(), v5, 48u8, 57u8);
    let v7: Result<u64, Bytes> = f44(v0, v5, 10u64, false)?;
    if v6 {
        Ok(v7)
    } else {
        Ok(Err::<u64, Bytes>({
            let a550 = Str::lit("a scientific literal has no exponent digits");
            utf8_encode(a550)
        }))
    }
}

pub fn f72(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: bool = f47(v2);
    let v4: bool = f30(v2);
    let v5: u64 = f48(v0.clone(), v1)?;
    let v6: u64 = f79(v0.clone(), v1)?;
    let a552 = v1;
    let a553 = {
        let a551 = v0;
        length_bytes(a551)
    };
    if nat_lt(a552, a553) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                let a554 = v2;
                let a555 = 95u8;
                if equal(a554, a555) {
                    Ok(1u64)
                } else {
                    let a556 = v2;
                    let a557 = 39u8;
                    if equal(a556, a557) {
                        Ok(1u64)
                    } else {
                        let a558 = v2;
                        let a559 = 33u8;
                        if equal(a558, a559) {
                            Ok(1u64)
                        } else {
                            let a560 = v2;
                            let a561 = 63u8;
                            if equal(a560, a561) {
                                Ok(1u64)
                            } else {
                                let a562 = 0u64;
                                let a563 = v5;
                                if nat_lt(a562, a563) {
                                    Ok(v5)
                                } else {
                                    Ok(v6)
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f73(v0: Bytes, v1: Adt6) -> R<Option<Adt6>> {
    if match v1.clone() {
        Adt6::C0(_, _, h564, _) => {
            h564
        }
    } {
        Ok(None::<Adt6>)
    } else {
        let m575 = if f50(v0.clone(), match v1.clone() {
            Adt6::C0(h565, _, _, _) => {
                h565
            }
        })? {
            f80(v0.clone(), match v1.clone() {
                Adt6::C0(h566, _, _, _) => {
                    h566
                }
            })?
        } else {
            let v2: u64 = f33(v0.clone(), {
                let a569 = match v1.clone() {
                    Adt6::C0(h567, _, _, _) => {
                        h567
                    }
                };
                let a570 = f32(v0.clone(), match v1.clone() {
                    Adt6::C0(h568, _, _, _) => {
                        h568
                    }
                })?;
                nat_add(a569, a570)?
            })?;
            Ok::<(Adt2, u64), Bytes>((Adt2::C0(f34(v0.clone(), match v1.clone() {
                Adt6::C0(h571, _, _, _) => {
                    h571
                }
            }, {
                let a573 = v2;
                let a574 = match v1.clone() {
                    Adt6::C0(h572, _, _, _) => {
                        h572
                    }
                };
                nat_sub(a573, a574)
            }), false), v2))
        };
        match m575 {
            Ok(v4) => {
                let v5: bool = f9(v0.clone(), {
                    let (_, h578) = v4.clone();
                    h578
                }, 46u8);
                let v6: bool = f35(v0, {
                    let a580 = {
                        let (_, h579) = v4.clone();
                        h579
                    };
                    let a581 = 1u64;
                    nat_add(a580, a581)?
                })?;
                if if v5 {
                    v6
                } else {
                    false
                } {
                    Ok(Some(Adt6::C0({
                        let a583 = {
                            let (_, h582) = v4;
                            h582
                        };
                        let a584 = 1u64;
                        nat_add(a583, a584)?
                    }, List::cons({
                        let (h585, _) = v4;
                        h585
                    }, match v1 {
                        Adt6::C0(_, h586, _, _) => {
                            h586
                        }
                    }), false, None::<Bytes>)))
                } else {
                    Ok(Some(Adt6::C0({
                        let (_, h587) = v4;
                        h587
                    }, List::cons({
                        let (h588, _) = v4;
                        h588
                    }, match v1 {
                        Adt6::C0(_, h589, _, _) => {
                            h589
                        }
                    }), true, None::<Bytes>)))
                }
            }
            Err(v3) => {
                Ok(Some(Adt6::C0(match v1.clone() {
                    Adt6::C0(h576, _, _, _) => {
                        h576
                    }
                }, match v1 {
                    Adt6::C0(_, h577, _, _) => {
                        h577
                    }
                }, true, Some(v3))))
            }
        }
    }
}

pub fn f74(v0: Fn9, p593: List<Adt2>, p594: List<Adt2>) -> List<Adt2> {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v1, v2) = m592;
                let m590 = v2;
                match m590.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c591: Fn9 = v0.clone();
                            c591.apply(v1, v3)
                        }, v4))
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

pub fn f75(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f9(v0, v1, 35u8);
    if v2 {
        Ok(Some({
            let a595 = v1;
            let a596 = 1u64;
            nat_add(a595, a596)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f77(v0: Bytes, v1: u64, v2: u64, v3: Adt5) -> R<Option<Adt5>> {
    let v4: u64 = match v3.clone() {
        Adt5::C0(h597, _, _, _) => {
            h597
        }
    };
    let v5: u64 = match v3.clone() {
        Adt5::C0(_, h598, _, _) => {
            h598
        }
    };
    if match v3.clone() {
        Adt5::C0(_, _, h599, _) => {
            h599
        }
    } {
        Ok(None::<Adt5>)
    } else {
        if match v3 {
            Adt5::C0(_, _, _, h600) => {
                h600
            }
        } {
            Ok(None::<Adt5>)
        } else {
            let a601 = v4;
            let a602 = v1;
            if nat_lt(a601, a602) {
                let v6: u8 = f8(v0.clone(), v4);
                let a603 = v6;
                let a604 = 34u8;
                if equal(a603, a604) {
                    let v7: bool = f81(v0, {
                        let a605 = v4;
                        let a606 = 1u64;
                        nat_add(a605, a606)?
                    }, v2)?;
                    if v7 {
                        Ok(Some(Adt5::C0({
                            let a609 = {
                                let a607 = v4;
                                let a608 = 1u64;
                                nat_add(a607, a608)?
                            };
                            let a610 = v2;
                            nat_add(a609, a610)?
                        }, v5, true, false)))
                    } else {
                        Ok(Some(Adt5::C0({
                            let a611 = v4;
                            let a612 = 1u64;
                            nat_add(a611, a612)?
                        }, v5, false, false)))
                    }
                } else {
                    let a613 = v6;
                    let a614 = 10u8;
                    if equal(a613, a614) {
                        Ok(Some(Adt5::C0({
                            let a615 = v4;
                            let a616 = 1u64;
                            nat_add(a615, a616)?
                        }, {
                            let a617 = v5;
                            let a618 = 1u64;
                            nat_add(a617, a618)?
                        }, false, false)))
                    } else {
                        let v8: u64 = f37(v0, v4);
                        Ok(Some(Adt5::C0({
                            let a619 = v4;
                            let a620 = v8;
                            nat_add(a619, a620)?
                        }, v5, false, false)))
                    }
                }
            } else {
                Ok(Some(Adt5::C0(v4, v5, false, true)))
            }
        }
    }
}

pub fn f78(v0: u8, v1: u64) -> bool {
    let v2: bool = f18(v0, 48u8, 49u8);
    let v3: bool = f18(v0, 48u8, 55u8);
    let v4: bool = f18(v0, 48u8, 57u8);
    let v5: bool = f18(v0, 97u8, 102u8);
    let v6: bool = f18(v0, 65u8, 70u8);
    let a621 = v1;
    let a622 = 2u64;
    if nat_eq(a621, a622) {
        v2
    } else {
        let a623 = v1;
        let a624 = 8u64;
        if nat_eq(a623, a624) {
            v3
        } else {
            let a625 = v1;
            let a626 = 16u64;
            if nat_eq(a625, a626) {
                if v4 {
                    true
                } else {
                    if v5 {
                        true
                    } else {
                        v6
                    }
                }
            } else {
                v4
            }
        }
    }
}

pub fn f79(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a627 = v1;
        let a628 = 1u64;
        nat_add(a627, a628)?
    });
    let v4: u8 = f8(v0, {
        let a629 = v1;
        let a630 = 2u64;
        nat_add(a629, a630)?
    });
    let a631 = v2;
    let a632 = 225u8;
    if equal(a631, a632) {
        let a633 = v3;
        let a634 = 181u8;
        if equal(a633, a634) {
            if f18(v4, 162u8, 170u8) {
                Ok(3u64)
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a635 = v2;
        let a636 = 226u8;
        if equal(a635, a636) {
            let a637 = v3;
            let a638 = 130u8;
            if equal(a637, a638) {
                if f18(v4, 128u8, 137u8) {
                    Ok(3u64)
                } else {
                    if f18(v4, 144u8, 156u8) {
                        Ok(3u64)
                    } else {
                        Ok(0u64)
                    }
                }
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    }
}

pub fn f80(v0: Bytes, v1: u64) -> R<Result<(Adt2, u64), Bytes>> {
    let v2: u64 = {
        let a639 = v1;
        let a640 = 2u64;
        nat_add(a639, a640)?
    };
    let v3: Adt7 = {
        let (h641, _) = f82(Fn10::F83(v0.clone()), {
            let a645 = {
                let a643 = {
                    let a642 = v0.clone();
                    length_bytes(a642)
                };
                let a644 = v2;
                nat_sub(a643, a644)
            };
            let a646 = 1u64;
            nat_add(a645, a646)?
        }, Adt7::C0(v2, false, false, true))?;
        h641
    };
    let v4: Bytes = f34(v0, v2, {
        let a648 = match v3.clone() {
            Adt7::C0(h647, _, _, _) => {
                h647
            }
        };
        let a649 = v2;
        nat_sub(a648, a649)
    });
    if match v3.clone() {
        Adt7::C0(_, h650, _, _) => {
            h650
        }
    } {
        let a652 = 0u64;
        let a653 = {
            let a651 = v4.clone();
            length_bytes(a651)
        };
        if if nat_lt(a652, a653) {
            match v3.clone() {
                Adt7::C0(_, _, _, h654) => {
                    h654
                }
            }
        } else {
            false
        } {
            Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C0(v4, true), {
                let a656 = match v3 {
                    Adt7::C0(h655, _, _, _) => {
                        h655
                    }
                };
                let a657 = 2u64;
                nat_add(a656, a657)?
            })))
        } else {
            Ok(Err::<(Adt2, u64), Bytes>({
                let a662 = {
                    let a659 = {
                        let a658 = Str::lit("the quoted name `");
                        utf8_encode(a658)
                    };
                    let a660 = v4;
                    append_bytes(a659, a660)
                };
                let a663 = {
                    let a661 = Str::lit("` is not a name");
                    utf8_encode(a661)
                };
                append_bytes(a662, a663)
            }))
        }
    } else {
        Ok(Err::<(Adt2, u64), Bytes>({
            let a664 = Str::lit("a quoted name is not closed");
            utf8_encode(a664)
        }))
    }
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let (_, h665) = {
        let (h666, _) = f84(Fn11::F85(v2, v1, v0), v2, (0u64, true))?;
        h666
    };
    Ok(h665)
}

pub fn f82(v0: Fn10, p671: u64, p672: Adt7) -> R<(Adt7, bool)> {
    let m670 = (p671, p672);
    {
        let mut m670 = m670;
        loop {
            let step = {
                let (v1, v2) = m670;
                let m667 = v1;
                if m667 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m667 - 1;
                    let m669 = {
                        let c668: Fn10 = v0.clone();
                        c668.apply(v2.clone())?
                    };
                    match m669 {
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
                Ok(next) => { m670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: Adt7) -> R<Option<Adt7>> {
    f86(v0.clone(), {
        let a673 = v0;
        length_bytes(a673)
    }, v1)
}

pub fn f84(v0: Fn11, p678: u64, p679: (u64, bool)) -> R<((u64, bool), bool)> {
    let m677 = (p678, p679);
    {
        let mut m677 = m677;
        loop {
            let step = {
                let (v1, v2) = m677;
                let m674 = v1;
                if m674 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m674 - 1;
                    let m676 = {
                        let c675: Fn11 = v0.clone();
                        c675.apply(v2)?
                    };
                    match m676 {
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
                Ok(next) => { m677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: u64, v1: u64, v2: Bytes, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    f87(v2, v1, v0, v3)
}

pub fn f86(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h680, _, _, _) => {
            h680
        }
    };
    let v4: bool = match v2.clone() {
        Adt7::C0(_, _, _, h681) => {
            h681
        }
    };
    if match v2.clone() {
        Adt7::C0(_, h682, _, _) => {
            h682
        }
    } {
        Ok(None::<Adt7>)
    } else {
        if match v2 {
            Adt7::C0(_, _, h683, _) => {
                h683
            }
        } {
            Ok(None::<Adt7>)
        } else {
            let a684 = v3;
            let a685 = v1;
            if nat_lt(a684, a685) {
                let v5: bool = f9(v0.clone(), v3, 10u8);
                if v5 {
                    Ok(Some(Adt7::C0(v3, false, true, v4)))
                } else {
                    let v6: bool = f88(v0.clone(), v3)?;
                    if v6 {
                        Ok(Some(Adt7::C0(v3, true, false, v4)))
                    } else {
                        let v7: u64 = f72(v0.clone(), v3)?;
                        let a686 = 0u64;
                        let a687 = v7;
                        if nat_lt(a686, a687) {
                            Ok(Some(Adt7::C0({
                                let a688 = v3;
                                let a689 = v7;
                                nat_add(a688, a689)?
                            }, false, false, v4)))
                        } else {
                            let v8: bool = f9(v0.clone(), v3, 46u8);
                            if v8 {
                                Ok(Some(Adt7::C0({
                                    let a690 = v3;
                                    let a691 = 1u64;
                                    nat_add(a690, a691)?
                                }, false, false, v4)))
                            } else {
                                let v9: u64 = f37(v0, v3);
                                Ok(Some(Adt7::C0({
                                    let a692 = v3;
                                    let a693 = v9;
                                    nat_add(a692, a693)?
                                }, false, false, false)))
                            }
                        }
                    }
                }
            } else {
                Ok(Some(Adt7::C0(v3, false, true, v4)))
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: u64, v2: u64, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v4: u64 = {
        let (h694, _) = v3;
        h694
    };
    let v5: bool = {
        let (_, h695) = v3;
        h695
    };
    if v5 {
        let a696 = v4;
        let a697 = v2;
        if nat_lt(a696, a697) {
            let v6: bool = f9(v0, {
                let a698 = v1;
                let a699 = v4;
                nat_add(a698, a699)?
            }, 35u8);
            Ok(Some(({
                let a700 = v4;
                let a701 = 1u64;
                nat_add(a700, a701)?
            }, v6)))
        } else {
            Ok(None::<(u64, bool)>)
        }
    } else {
        Ok(None::<(u64, bool)>)
    }
}

pub fn f88(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f9(v0.clone(), v1, 194u8);
    let v3: bool = f9(v0, {
        let a702 = v1;
        let a703 = 1u64;
        nat_add(a702, a703)?
    }, 187u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}
