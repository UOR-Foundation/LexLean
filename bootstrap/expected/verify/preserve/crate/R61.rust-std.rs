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
    C0(Bytes, List<Bytes>, List<(Bytes, Adt1)>, List<(Bytes, Adt1)>),
    C1(Bytes, List<Bytes>, List<(Bytes, Adt1)>, List<(Bytes, Adt1)>),
    C2(Bytes, Option<Bytes>, Bytes, Adt1, u64, Adt2),
    C3(Bytes, List<Bytes>, List<(Bytes, Adt1)>, List<(Bytes, Adt1)>, Option<Bytes>),
    C4(Bytes, List<Bytes>, List<(Bytes, Adt1)>, Adt1, Option<Bytes>, Adt2, List<Bytes>, bool, Option<Bytes>, Option<(Adt2, List<(Option<Bytes>, Bytes)>)>, Option<(List<Bytes>, List<Bytes>)>),
    C5(Bytes, List<Bytes>, List<(Bytes, Adt1)>, Adt2, Adt5, List<Bytes>),
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
    C19(Bytes),
    C20(std::rc::Rc<Adt1>),
    C21(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C22(std::rc::Rc<Adt1>),
    C23(Option<Bytes>, Bytes, std::rc::Rc<Adt1>),
    C24(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C25(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C26(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C27(std::rc::Rc<Adt1>),
    C28,
    C29(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes),
    C1(Bytes),
    C2(Adt3, Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Adt4, std::rc::Rc<Adt2>, Adt1),
    C6(bool),
    C7,
    C8(Adt1),
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(Option<Bytes>, Bytes, Adt1, std::rc::Rc<Adt2>),
    C11(Option<Bytes>, Bytes, Adt1, std::rc::Rc<Adt2>),
    C12(Option<Bytes>, Bytes, Adt1, Option<Bytes>, Bytes),
    C13(std::rc::Rc<Adt2>, Bytes),
    C14(Option<Bytes>, Bytes, Adt1, std::rc::Rc<Adt2>),
    C15(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C16(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C17(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C18(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C19(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C20(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C21(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C22(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C23(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C24(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C25(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C26(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C27(std::rc::Rc<Adt2>),
    C28(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C29(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C30(Bytes, Adt1, std::rc::Rc<Adt2>),
    C31(Bytes, Adt1, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C32(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C33(std::rc::Rc<Adt2>),
    C34(std::rc::Rc<Adt2>),
    C35(List<(Bytes, Adt1)>, List<Bytes>, std::rc::Rc<Adt2>),
    C36(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C37(Option<Bytes>, Bytes, Adt1),
    C38(Adt1, Adt1, std::rc::Rc<Adt2>),
    C39(Adt1, std::rc::Rc<Adt2>),
    C40(Adt1, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C41(Option<Bytes>, Bytes, Adt1, std::rc::Rc<Adt2>, List<Bytes>),
    C42,
    C43(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C44(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C45(Option<Bytes>, Bytes, List<Bytes>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C46(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C47(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
    C40,
    C41,
    C42,
    C43,
    C44,
    C45,
    C46,
    C47,
    C48,
    C49,
    C50,
    C51,
    C52,
    C53,
    C54,
    C55,
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2(List<(Option<Bytes>, Bytes)>),
    C3(List<(Option<Bytes>, Bytes)>),
    C4(std::rc::Rc<Adt5>),
    C5(Bytes, std::rc::Rc<Adt5>),
    C6(Adt2, List<(Option<Bytes>, Bytes)>),
    C7(Bytes, List<Bytes>, std::rc::Rc<Adt5>),
    C8(Bytes, std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
    C9,
    C10(Adt6),
    C11(Bytes, Adt1, Adt2, List<(Option<Bytes>, Bytes)>),
    C12(Option<Bytes>, Bytes, Adt1, Adt2),
    C13,
    C14(std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
    C15(Bytes, List<Bytes>, std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
    C16(Adt1, Bytes, Adt2, std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, Bytes, Option<Bytes>, Bytes, Option<Bytes>, Bytes, Bytes),
    C1(Bytes, Option<Bytes>, Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt7>),
    C4(std::rc::Rc<Adt7>),
    C5,
    C6(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C7(Bytes, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, bool, bool, Option<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt0) -> List<Adt7> {
        match self {
            Fn0::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt7, p1: Adt7) -> Adt7 {
        match self {
            Fn1::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn2::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F24,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt7) -> List<Adt7> {
        match self {
            Fn3::F24 => f24(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F40(Bytes),
    F85(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F40(k0) => f40(k0.clone(), p0),
            Fn4::F85(k0, k1) => Ok(f85(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F43,
    F94,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F43 => f43(p0, p1),
            Fn5::F94 => f94(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F46,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F46 => f46(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F48,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt7>, p1: Bytes) -> List<Adt7> {
        match self {
            Fn7::F48 => f48(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F51,
    F61,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt7>, p1: (Bytes, Adt1)) -> List<Adt7> {
        match self {
            Fn8::F51 => f51(p0, p1),
            Fn8::F61 => f61(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F68(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn9::F68(k0) => f68(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F73,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn10::F73 => f73(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F75,
}

impl Fn11 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
        match self {
            Fn11::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F79,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Adt7>, p1: (Option<Bytes>, Bytes)) -> List<Adt7> {
        match self {
            Fn12::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F82,
}

impl Fn13 {
    pub fn apply(&self, p0: List<Adt7>, p1: (Bytes, Bytes)) -> List<Adt7> {
        match self {
            Fn13::F82 => f82(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F89,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn14::F89 => f89(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F91,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn15::F91 => f91(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F93,
}

impl Fn16 {
    pub fn apply(&self, p0: List<(Option<Bytes>, Bytes)>, p1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
        match self {
            Fn16::F93 => f93(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F99,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F99 => f99(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F104,
}

impl Fn18 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn18::F104 => f104(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F108,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn19::F108 => f108(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>) -> R<Bytes> {
    f1(Adt7::C4(std::rc::Rc::new(f2(Str::lit("spec"), f3(v0), f2(Str::lit("declarations"), Adt7::C3(std::rc::Rc::new(f4(f5(Fn0::F6, List::<Adt7>::nil(), f7(v1))))), f8())))))
}

pub fn f1(v0: Adt7) -> R<Bytes> {
    f9(v0)
}

pub fn f2(v0: Str, v1: Adt7, v2: Adt7) -> Adt7 {
    Adt7::C7(f10(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f3(v0: Bytes) -> Adt7 {
    Adt7::C2(v0)
}

pub fn f4(v0: List<Adt7>) -> Adt7 {
    f11(Fn1::F12, Adt7::C5, f13(v0))
}

pub fn f5(v0: Fn0, p4: List<Adt7>, p5: List<Adt0>) -> List<Adt7> {
    let m3 = (p4, p5);
    {
        let mut m3 = m3;
        loop {
            let step = {
                let (v1, v2) = m3;
                let m1 = v2;
                match m1.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt0>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt0>), List<Adt7>>(({
                            let c2: Fn0 = v0.clone();
                            c2.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: List<Adt7>, v1: Adt0) -> List<Adt7> {
    List::cons(f14(v1), v0)
}

pub fn f7(v0: List<Adt0>) -> List<Adt0> {
    f15(Fn2::F16, List::<Adt0>::nil(), v0)
}

pub fn f8() -> Adt7 {
    Adt7::C5
}

pub fn f9(v0: Adt7) -> R<Bytes> {
    let m6 = v0;
    match m6 {
        Adt7::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt7::C1(v2, v3) => {
            f17(v2, v3)
        }
        Adt7::C2(v4) => {
            f18(v4)
        }
        Adt7::C3(r7) => {
            let v5 = (*r7).clone();
            Ok(f19(f20(v5)?))
        }
        Adt7::C4(r8) => {
            let v6 = (*r8).clone();
            f21(f22(v6)?)
        }
        Adt7::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt7::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt7::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f10(v0: Str) -> Bytes {
    let a9 = v0;
    utf8_encode(a9)
}

pub fn f11(v0: Fn1, p13: Adt7, p14: List<Adt7>) -> Adt7 {
    let m12 = (p13, p14);
    {
        let mut m12 = m12;
        loop {
            let step = {
                let (v1, v2) = m12;
                let m10 = v2;
                match m10.uncons() {
                    None => {
                        Err::<(Adt7, List<Adt7>), Adt7>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt7, List<Adt7>), Adt7>(({
                            let c11: Fn1 = v0.clone();
                            c11.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m12 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt7, v1: Adt7) -> Adt7 {
    Adt7::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f13(v0: List<Adt7>) -> List<Adt7> {
    f23(Fn3::F24, List::<Adt7>::nil(), v0)
}

pub fn f14(v0: Adt0) -> Adt7 {
    let m15 = v0;
    match m15 {
        Adt0::C0(v1, v2, v3, v4) => {
            f25(Str::lit("structure"), f2(Str::lit("name"), f3(v1), f2(Str::lit("type_parameters"), f26(v2), f2(Str::lit("parameters"), f27(v3), f2(Str::lit("fields"), f28(v4), f8())))))
        }
        Adt0::C1(v5, v6, v7, v8) => {
            f25(Str::lit("class"), f2(Str::lit("name"), f3(v5), f2(Str::lit("type_parameters"), f26(v6), f2(Str::lit("parameters"), f27(v7), f2(Str::lit("fields"), f28(v8), f8())))))
        }
        Adt0::C2(v9, v10, v11, v12, v13, v14) => {
            f25(Str::lit("instance"), f2(Str::lit("name"), f3(v9), f2(Str::lit("class"), f29(v10, v11), f2(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f30(v12))), f2(Str::lit("priority"), Adt7::C1(false, v13), f2(Str::lit("fields"), Adt7::C3(std::rc::Rc::new(f31(v14))), f8()))))))
        }
        Adt0::C3(v15, v16, v17, v18, v19) => {
            f25(Str::lit("inductive"), f2(Str::lit("name"), f3(v15), f2(Str::lit("type_parameters"), f26(v16), f2(Str::lit("parameters"), f27(v17), f2(Str::lit("constructors"), f32(v18), f33(Str::lit("mutual"), v19, f8()))))))
        }
        Adt0::C4(v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) => {
            f25(Str::lit("definition"), f2(Str::lit("name"), f3(v20), f34(Str::lit("type_parameters"), v21, f2(Str::lit("parameters"), f27(v22), f2(Str::lit("result"), f30(v23), f33(Str::lit("recursive_argument"), v24, f2(Str::lit("body"), f31(v25), f34(Str::lit("axioms"), v26, if v27 {
                f2(Str::lit("executable"), Adt7::C0(true), f33(Str::lit("mutual"), v28, f35(v29, f36(v30, f8()))))
            } else {
                f33(Str::lit("mutual"), v28, f35(v29, f36(v30, f8())))
            }))))))))
        }
        Adt0::C5(v31, v32, v33, v34, v35, v36) => {
            f25(Str::lit("theorem"), f2(Str::lit("name"), f3(v31), f34(Str::lit("type_parameters"), v32, f2(Str::lit("parameters"), f27(v33), f2(Str::lit("statement"), f31(v34), f2(Str::lit("proof"), f37(v35), f34(Str::lit("axioms"), v36, f8())))))))
        }
    }
}

pub fn f15(v0: Fn2, p19: List<Adt0>, p20: List<Adt0>) -> List<Adt0> {
    let m18 = (p19, p20);
    {
        let mut m18 = m18;
        loop {
            let step = {
                let (v1, v2) = m18;
                let m16 = v2;
                match m16.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c17: Fn2 = v0.clone();
                            c17.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m18 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f17(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a21 = 0u64;
        let a22 = v1;
        nat_lt(a21, a22)
    } else {
        false
    } {
        let a23 = Bytes::lit(&[0x2d]);
        let a24 = f38(v1)?;
        Ok(append_bytes(a23, a24))
    } else {
        f38(v1)
    }
}

pub fn f18(v0: Bytes) -> R<Bytes> {
    let a30 = {
        let a28 = Bytes::lit(&[0x22]);
        let a29 = {
            let (_, h25) = {
                let (h26, _) = f39(Fn4::F40(v0.clone()), {
                    let a27 = v0;
                    length_bytes(a27)
                }, (0u64, Bytes::lit(&[])))?;
                h26
            };
            h25
        };
        append_bytes(a28, a29)
    };
    let a31 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a30, a31))
}

pub fn f19(v0: List<Bytes>) -> Bytes {
    let a34 = {
        let a32 = Bytes::lit(&[0x5b]);
        let a33 = f41(v0, Bytes::lit(&[0x2c]));
        append_bytes(a32, a33)
    };
    let a35 = Bytes::lit(&[0x5d]);
    append_bytes(a34, a35)
}

pub fn f20(v0: Adt7) -> R<List<Bytes>> {
    let m36 = v0;
    match m36 {
        Adt7::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C6(r37, r38) => {
            let v7 = (*r37).clone();
            let v8 = (*r38).clone();
            Ok(List::cons(f9(v7)?, f20(v8)?))
        }
        Adt7::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f21(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f42(Fn5::F43, List::<(Bytes, Bytes)>::nil(), f44(v0)?);
    let v2: List<Bytes> = f45(Fn6::F46, List::<Bytes>::nil(), v1)?;
    let a41 = {
        let a39 = Bytes::lit(&[0x7b]);
        let a40 = f41(v2, Bytes::lit(&[0x2c]));
        append_bytes(a39, a40)
    };
    let a42 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a41, a42))
}

pub fn f22(v0: Adt7) -> R<List<(Bytes, Bytes)>> {
    let m43 = v0;
    match m43 {
        Adt7::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C7(v9, r44, r45) => {
            let v10 = (*r44).clone();
            let v11 = (*r45).clone();
            Ok(List::cons((v9, f9(v10)?), f22(v11)?))
        }
    }
}

pub fn f23(v0: Fn3, p49: List<Adt7>, p50: List<Adt7>) -> List<Adt7> {
    let m48 = (p49, p50);
    {
        let mut m48 = m48;
        loop {
            let step = {
                let (v1, v2) = m48;
                let m46 = v2;
                match m46.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt7>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt7>), List<Adt7>>(({
                            let c47: Fn3 = v0.clone();
                            c47.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m48 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f25(v0: Str, v1: Adt7) -> Adt7 {
    Adt7::C4(std::rc::Rc::new(f2(Str::lit("kind"), f3(f10(v0)), v1)))
}

pub fn f26(v0: List<Bytes>) -> Adt7 {
    Adt7::C3(std::rc::Rc::new(f4(f47(Fn7::F48, List::<Adt7>::nil(), f49(v0)))))
}

pub fn f27(v0: List<(Bytes, Adt1)>) -> Adt7 {
    Adt7::C3(std::rc::Rc::new(f4(f50(Fn8::F51, List::<Adt7>::nil(), f52(v0)))))
}

pub fn f28(v0: List<(Bytes, Adt1)>) -> Adt7 {
    f27(v0)
}

pub fn f29(v0: Option<Bytes>, v1: Bytes) -> Adt7 {
    Adt7::C4(std::rc::Rc::new({
        let m51 = v0;
        match m51 {
            None => {
                f53(Str::lit("name"), Adt7::C2(v1), Adt7::C5)
            }
            Some(v2) => {
                f53(Str::lit("module"), Adt7::C2(v2), f53(Str::lit("name"), Adt7::C2(v1), Adt7::C5))
            }
        }
    }))
}

pub fn f30(v0: Adt1) -> Adt7 {
    let m52 = v0;
    match m52 {
        Adt1::C0 => {
            f54(Str::lit("type"), Adt7::C5)
        }
        Adt1::C1 => {
            f54(Str::lit("nat"), Adt7::C5)
        }
        Adt1::C2 => {
            f54(Str::lit("bool"), Adt7::C5)
        }
        Adt1::C3 => {
            f54(Str::lit("prop"), Adt7::C5)
        }
        Adt1::C4 => {
            f54(Str::lit("unit"), Adt7::C5)
        }
        Adt1::C5 => {
            f54(Str::lit("int"), Adt7::C5)
        }
        Adt1::C6 => {
            f54(Str::lit("int8"), Adt7::C5)
        }
        Adt1::C7 => {
            f54(Str::lit("int16"), Adt7::C5)
        }
        Adt1::C8 => {
            f54(Str::lit("int32"), Adt7::C5)
        }
        Adt1::C9 => {
            f54(Str::lit("int64"), Adt7::C5)
        }
        Adt1::C10 => {
            f54(Str::lit("uint8"), Adt7::C5)
        }
        Adt1::C11 => {
            f54(Str::lit("uint16"), Adt7::C5)
        }
        Adt1::C12 => {
            f54(Str::lit("uint32"), Adt7::C5)
        }
        Adt1::C13 => {
            f54(Str::lit("uint64"), Adt7::C5)
        }
        Adt1::C14 => {
            f54(Str::lit("string"), Adt7::C5)
        }
        Adt1::C15 => {
            f54(Str::lit("bytes"), Adt7::C5)
        }
        Adt1::C16 => {
            f54(Str::lit("ordering"), Adt7::C5)
        }
        Adt1::C17 => {
            f54(Str::lit("contract_violation"), Adt7::C5)
        }
        Adt1::C18 => {
            f54(Str::lit("reasoning_failure"), Adt7::C5)
        }
        Adt1::C19(v1) => {
            f54(Str::lit("parameter"), f55(Str::lit("name"), Adt7::C2(v1), Adt7::C5))
        }
        Adt1::C20(r54) => {
            let v2 = (*r54).clone();
            f54(Str::lit("option"), f55(Str::lit("value"), f30(v2), Adt7::C5))
        }
        Adt1::C21(r55, r56) => {
            let v3 = (*r55).clone();
            let v4 = (*r56).clone();
            f54(Str::lit("result"), f55(Str::lit("ok"), f30(v3), f55(Str::lit("error"), f30(v4), Adt7::C5)))
        }
        Adt1::C22(r57) => {
            let v5 = (*r57).clone();
            f54(Str::lit("list"), f55(Str::lit("element"), f30(v5), Adt7::C5))
        }
        Adt1::C23(v6, v7, r58) => {
            let v8 = (*r58).clone();
            f54(Str::lit("named"), f55(Str::lit("member"), Adt7::C4(std::rc::Rc::new({
                let m53 = v6;
                match m53 {
                    None => {
                        f55(Str::lit("name"), Adt7::C2(v7), Adt7::C5)
                    }
                    Some(v9) => {
                        f55(Str::lit("module"), Adt7::C2(v9), f55(Str::lit("name"), Adt7::C2(v7), Adt7::C5))
                    }
                }
            })), f55(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f30(v8))), Adt7::C5)))
        }
        Adt1::C24(r59, r60) => {
            let v10 = (*r59).clone();
            let v11 = (*r60).clone();
            f54(Str::lit("product"), f55(Str::lit("left"), f30(v10), f55(Str::lit("right"), f30(v11), Adt7::C5)))
        }
        Adt1::C25(r61, r62) => {
            let v12 = (*r61).clone();
            let v13 = (*r62).clone();
            f54(Str::lit("function"), f55(Str::lit("parameters"), Adt7::C3(std::rc::Rc::new(f30(v12))), f55(Str::lit("result"), f30(v13), Adt7::C5)))
        }
        Adt1::C26(r63, r64) => {
            let v14 = (*r63).clone();
            let v15 = (*r64).clone();
            f54(Str::lit("map"), f55(Str::lit("key"), f30(v14), f55(Str::lit("value"), f30(v15), Adt7::C5)))
        }
        Adt1::C27(r65) => {
            let v16 = (*r65).clone();
            f54(Str::lit("set"), f55(Str::lit("element"), f30(v16), Adt7::C5))
        }
        Adt1::C28 => {
            Adt7::C5
        }
        Adt1::C29(r66, r67) => {
            let v17 = (*r66).clone();
            let v18 = (*r67).clone();
            Adt7::C6(std::rc::Rc::new(f30(v17)), std::rc::Rc::new(f30(v18)))
        }
    }
}

pub fn f31(v0: Adt2) -> Adt7 {
    let m68 = v0;
    match m68 {
        Adt2::C0(v1) => {
            f56(Str::lit("var"), f53(Str::lit("name"), Adt7::C2(v1), Adt7::C5))
        }
        Adt2::C1(v2) => {
            f56(Str::lit("nat"), f53(Str::lit("value"), Adt7::C2(v2), Adt7::C5))
        }
        Adt2::C2(v3, v4) => {
            f56(Str::lit("integer"), f53(Str::lit("representation"), Adt7::C2(f57(v3)), f53(Str::lit("value"), Adt7::C2(v4), Adt7::C5)))
        }
        Adt2::C3(v5) => {
            f56(Str::lit("string"), f53(Str::lit("value"), Adt7::C2(v5), Adt7::C5))
        }
        Adt2::C4(v6) => {
            f56(Str::lit("bytes"), f53(Str::lit("hex"), Adt7::C2(v6), Adt7::C5))
        }
        Adt2::C5(v7, r71, v9) => {
            let v8 = (*r71).clone();
            f56(Str::lit("primitive"), f53(Str::lit("operation"), Adt7::C2({
                let m70 = f58(v7);
                match m70 {
                    Adt8::C0(h69, _, _, _) => {
                        h69
                    }
                }
            }), f53(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f31(v8))), f53(Str::lit("result"), f30(v9), Adt7::C5))))
        }
        Adt2::C6(v10) => {
            f56(Str::lit("bool"), f53(Str::lit("value"), Adt7::C0(v10), Adt7::C5))
        }
        Adt2::C7 => {
            f56(Str::lit("unit"), Adt7::C5)
        }
        Adt2::C8(v11) => {
            f56(Str::lit("nil"), f53(Str::lit("element"), f30(v11), Adt7::C5))
        }
        Adt2::C9(r72, r73) => {
            let v12 = (*r72).clone();
            let v13 = (*r73).clone();
            f56(Str::lit("cons"), f53(Str::lit("head"), f31(v12), f53(Str::lit("tail"), f31(v13), Adt7::C5)))
        }
        Adt2::C10(v14, v15, v16, r74) => {
            let v17 = (*r74).clone();
            f56(Str::lit("record"), f53(Str::lit("type"), f29(v14, v15), f59(Str::lit("type_arguments"), v16, f53(Str::lit("fields"), Adt7::C3(std::rc::Rc::new(f31(v17))), Adt7::C5))))
        }
        Adt2::C11(v18, v19, v20, r75) => {
            let v21 = (*r75).clone();
            f56(Str::lit("constructor"), f53(Str::lit("constructor"), f29(v18, v19), f59(Str::lit("type_arguments"), v20, f53(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f31(v21))), Adt7::C5))))
        }
        Adt2::C12(v22, v23, v24, v25, v26) => {
            f56(Str::lit("instance_value"), f53(Str::lit("class"), f29(v22, v23), f53(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f30(v24))), f53(Str::lit("resolved"), f29(v25, v26), Adt7::C5))))
        }
        Adt2::C13(r76, v28) => {
            let v27 = (*r76).clone();
            f56(Str::lit("project"), f53(Str::lit("value"), f31(v27), f53(Str::lit("field"), Adt7::C2(v28), Adt7::C5)))
        }
        Adt2::C14(v29, v30, v31, r77) => {
            let v32 = (*r77).clone();
            f56(Str::lit("call"), f53(Str::lit("function"), f29(v29, v30), f59(Str::lit("type_arguments"), v31, f53(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f31(v32))), Adt7::C5))))
        }
        Adt2::C15(r78, r79, r80) => {
            let v33 = (*r78).clone();
            let v34 = (*r79).clone();
            let v35 = (*r80).clone();
            f56(Str::lit("if"), f53(Str::lit("condition"), f31(v33), f53(Str::lit("then_value"), f31(v34), f53(Str::lit("else_value"), f31(v35), Adt7::C5))))
        }
        Adt2::C16(r81, r82) => {
            let v36 = (*r81).clone();
            let v37 = (*r82).clone();
            f56(Str::lit("match"), f53(Str::lit("scrutinee"), f31(v36), f53(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f31(v37))), Adt7::C5)))
        }
        Adt2::C17(r83, r84) => {
            let v38 = (*r83).clone();
            let v39 = (*r84).clone();
            f56(Str::lit("eq"), f53(Str::lit("left"), f31(v38), f53(Str::lit("right"), f31(v39), Adt7::C5)))
        }
        Adt2::C18(r85, r86) => {
            let v40 = (*r85).clone();
            let v41 = (*r86).clone();
            f56(Str::lit("le"), f53(Str::lit("left"), f31(v40), f53(Str::lit("right"), f31(v41), Adt7::C5)))
        }
        Adt2::C19(r87, r88) => {
            let v42 = (*r87).clone();
            let v43 = (*r88).clone();
            f56(Str::lit("lt"), f53(Str::lit("left"), f31(v42), f53(Str::lit("right"), f31(v43), Adt7::C5)))
        }
        Adt2::C20(r89, r90) => {
            let v44 = (*r89).clone();
            let v45 = (*r90).clone();
            f56(Str::lit("add"), f53(Str::lit("left"), f31(v44), f53(Str::lit("right"), f31(v45), Adt7::C5)))
        }
        Adt2::C21(r91, r92) => {
            let v46 = (*r91).clone();
            let v47 = (*r92).clone();
            f56(Str::lit("beq"), f53(Str::lit("left"), f31(v46), f53(Str::lit("right"), f31(v47), Adt7::C5)))
        }
        Adt2::C22(r93, r94) => {
            let v48 = (*r93).clone();
            let v49 = (*r94).clone();
            f56(Str::lit("ble"), f53(Str::lit("left"), f31(v48), f53(Str::lit("right"), f31(v49), Adt7::C5)))
        }
        Adt2::C23(r95, r96) => {
            let v50 = (*r95).clone();
            let v51 = (*r96).clone();
            f56(Str::lit("blt"), f53(Str::lit("left"), f31(v50), f53(Str::lit("right"), f31(v51), Adt7::C5)))
        }
        Adt2::C24(r97, r98) => {
            let v52 = (*r97).clone();
            let v53 = (*r98).clone();
            f56(Str::lit("and"), f53(Str::lit("left"), f31(v52), f53(Str::lit("right"), f31(v53), Adt7::C5)))
        }
        Adt2::C25(r99, r100) => {
            let v54 = (*r99).clone();
            let v55 = (*r100).clone();
            f56(Str::lit("prop_and"), f53(Str::lit("left"), f31(v54), f53(Str::lit("right"), f31(v55), Adt7::C5)))
        }
        Adt2::C26(r101, r102) => {
            let v56 = (*r101).clone();
            let v57 = (*r102).clone();
            f56(Str::lit("or"), f53(Str::lit("left"), f31(v56), f53(Str::lit("right"), f31(v57), Adt7::C5)))
        }
        Adt2::C27(r103) => {
            let v58 = (*r103).clone();
            f56(Str::lit("not"), f53(Str::lit("value"), f31(v58), Adt7::C5))
        }
        Adt2::C28(r104, r105) => {
            let v59 = (*r104).clone();
            let v60 = (*r105).clone();
            f56(Str::lit("implies"), f53(Str::lit("premise"), f31(v59), f53(Str::lit("conclusion"), f31(v60), Adt7::C5)))
        }
        Adt2::C29(r106, r107) => {
            let v61 = (*r106).clone();
            let v62 = (*r107).clone();
            f56(Str::lit("iff"), f53(Str::lit("left"), f31(v61), f53(Str::lit("right"), f31(v62), Adt7::C5)))
        }
        Adt2::C30(v63, v64, r108) => {
            let v65 = (*r108).clone();
            f56(Str::lit("forall"), f53(Str::lit("binder"), f60(v63, v64), f53(Str::lit("body"), f31(v65), Adt7::C5)))
        }
        Adt2::C31(v66, v67, r109, r110) => {
            let v68 = (*r109).clone();
            let v69 = (*r110).clone();
            f56(Str::lit("let"), f53(Str::lit("binder"), f60(v66, v67), f53(Str::lit("value"), f31(v68), f53(Str::lit("body"), f31(v69), Adt7::C5))))
        }
        Adt2::C32(r111, r112) => {
            let v70 = (*r111).clone();
            let v71 = (*r112).clone();
            f56(Str::lit("pair"), f53(Str::lit("left"), f31(v70), f53(Str::lit("right"), f31(v71), Adt7::C5)))
        }
        Adt2::C33(r113) => {
            let v72 = (*r113).clone();
            f56(Str::lit("first"), f53(Str::lit("value"), f31(v72), Adt7::C5))
        }
        Adt2::C34(r114) => {
            let v73 = (*r114).clone();
            f56(Str::lit("second"), f53(Str::lit("value"), f31(v73), Adt7::C5))
        }
        Adt2::C35(v74, v75, r115) => {
            let v76 = (*r115).clone();
            f56(Str::lit("lambda"), f53(Str::lit("parameters"), f27(v74), f53(Str::lit("captures"), f26(v75), f53(Str::lit("body"), f31(v76), Adt7::C5))))
        }
        Adt2::C36(r116, r117) => {
            let v77 = (*r116).clone();
            let v78 = (*r117).clone();
            f56(Str::lit("apply"), f53(Str::lit("function"), f31(v77), f53(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f31(v78))), Adt7::C5)))
        }
        Adt2::C37(v79, v80, v81) => {
            f56(Str::lit("function_ref"), f53(Str::lit("function"), f29(v79, v80), f59(Str::lit("type_arguments"), v81, Adt7::C5)))
        }
        Adt2::C38(v82, v83, r118) => {
            let v84 = (*r118).clone();
            f56(Str::lit("map_literal"), f53(Str::lit("key"), f30(v82), f53(Str::lit("value"), f30(v83), f53(Str::lit("entries"), Adt7::C3(std::rc::Rc::new(f31(v84))), Adt7::C5))))
        }
        Adt2::C39(v85, r119) => {
            let v86 = (*r119).clone();
            f56(Str::lit("set_literal"), f53(Str::lit("element"), f30(v85), f53(Str::lit("elements"), Adt7::C3(std::rc::Rc::new(f31(v86))), Adt7::C5)))
        }
        Adt2::C40(v87, r120, r121) => {
            let v88 = (*r120).clone();
            let v89 = (*r121).clone();
            f56(Str::lit("graph_literal"), f53(Str::lit("node"), f30(v87), f53(Str::lit("nodes"), Adt7::C3(std::rc::Rc::new(f31(v88))), f53(Str::lit("edges"), Adt7::C3(std::rc::Rc::new(f31(v89))), Adt7::C5))))
        }
        Adt2::C41(v90, v91, v92, r122, v94) => {
            let v93 = (*r122).clone();
            f56(Str::lit("checked_apply"), f53(Str::lit("model"), f29(v90, v91), f59(Str::lit("type_arguments"), v92, f53(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f31(v93))), f53(Str::lit("checks"), f26(v94), Adt7::C5)))))
        }
        Adt2::C42 => {
            Adt7::C5
        }
        Adt2::C43(r123, r124) => {
            let v95 = (*r123).clone();
            let v96 = (*r124).clone();
            Adt7::C6(std::rc::Rc::new(f31(v95)), std::rc::Rc::new(f31(v96)))
        }
        Adt2::C44(v97, r125, r126) => {
            let v98 = (*r125).clone();
            let v99 = (*r126).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f53(Str::lit("field"), Adt7::C2(v97), f53(Str::lit("value"), f31(v98), Adt7::C5))))), std::rc::Rc::new(f31(v99)))
        }
        Adt2::C45(v100, v101, v102, r127, r128) => {
            let v103 = (*r127).clone();
            let v104 = (*r128).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f53(Str::lit("constructor"), f29(v100, v101), f53(Str::lit("binders"), f26(v102), f53(Str::lit("body"), f31(v103), Adt7::C5)))))), std::rc::Rc::new(f31(v104)))
        }
        Adt2::C46(r129, r130, r131) => {
            let v105 = (*r129).clone();
            let v106 = (*r130).clone();
            let v107 = (*r131).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f53(Str::lit("key"), f31(v105), f53(Str::lit("value"), f31(v106), Adt7::C5))))), std::rc::Rc::new(f31(v107)))
        }
        Adt2::C47(r132, r133, r134) => {
            let v108 = (*r132).clone();
            let v109 = (*r133).clone();
            let v110 = (*r134).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f53(Str::lit("source"), f31(v108), f53(Str::lit("target"), f31(v109), Adt7::C5))))), std::rc::Rc::new(f31(v110)))
        }
    }
}

pub fn f32(v0: List<(Bytes, Adt1)>) -> Adt7 {
    Adt7::C3(std::rc::Rc::new(f4(f50(Fn8::F61, List::<Adt7>::nil(), f52(v0)))))
}

pub fn f33(v0: Str, v1: Option<Bytes>, v2: Adt7) -> Adt7 {
    let m135 = v1;
    match m135 {
        None => {
            v2
        }
        Some(v3) => {
            f2(v0, f3(v3), v2)
        }
    }
}

pub fn f34(v0: Str, v1: List<Bytes>, v2: Adt7) -> Adt7 {
    let m136 = v1.clone();
    match m136.uncons() {
        None => {
            v2
        }
        Some((_, _)) => {
            f2(v0, f26(v1), v2)
        }
    }
}

pub fn f35(v0: Option<(Adt2, List<(Option<Bytes>, Bytes)>)>, v1: Adt7) -> Adt7 {
    let m137 = v0;
    match m137 {
        None => {
            v1
        }
        Some(v2) => {
            f2(Str::lit("termination"), Adt7::C4(std::rc::Rc::new(f2(Str::lit("measure"), f31({
                let (h138, _) = v2;
                h138
            }), f2(Str::lit("evidence"), f62({
                let (_, h139) = v2;
                h139
            }), f8())))), v1)
        }
    }
}

pub fn f36(v0: Option<(List<Bytes>, List<Bytes>)>, v1: Adt7) -> Adt7 {
    let m140 = v0;
    match m140 {
        None => {
            v1
        }
        Some(v2) => {
            f2(Str::lit("production"), Adt7::C4(std::rc::Rc::new(f2(Str::lit("targets"), f26({
                let (h141, _) = v2;
                h141
            }), f2(Str::lit("effects"), f26({
                let (_, h142) = v2;
                h142
            }), f8())))), v1)
        }
    }
}

pub fn f37(v0: Adt5) -> Adt7 {
    let m143 = v0;
    match m143 {
        Adt5::C0 => {
            f56(Str::lit("reflexivity"), Adt7::C5)
        }
        Adt5::C1 => {
            f56(Str::lit("decide"), Adt7::C5)
        }
        Adt5::C2(v1) => {
            f56(Str::lit("linear_arithmetic"), {
                let m144 = v1.clone();
                match m144.uncons() {
                    None => {
                        Adt7::C5
                    }
                    Some((_, _)) => {
                        f53(Str::lit("definitions"), f62(v1), Adt7::C5)
                    }
                }
            })
        }
        Adt5::C3(v4) => {
            f56(Str::lit("simplify"), f53(Str::lit("definitions"), f62(v4), Adt7::C5))
        }
        Adt5::C4(r147) => {
            let v5 = (*r147).clone();
            f56(Str::lit("constructor"), f53(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f37(v5))), Adt7::C5))
        }
        Adt5::C5(v6, r148) => {
            let v7 = (*r148).clone();
            f56(Str::lit("cases"), f53(Str::lit("scrutinee"), Adt7::C2(v6), f53(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f37(v7))), Adt7::C5)))
        }
        Adt5::C6(v8, v9) => {
            f56(Str::lit("term_cases"), f53(Str::lit("scrutinee"), f31(v8), f53(Str::lit("definitions"), f62(v9), Adt7::C5)))
        }
        Adt5::C7(v10, v11, r149) => {
            let v12 = (*r149).clone();
            f56(Str::lit("induction"), f53(Str::lit("scrutinee"), Adt7::C2(v10), {
                let m145 = v11.clone();
                match m145.uncons() {
                    None => {
                        f53(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f37(v12))), Adt7::C5)
                    }
                    Some((_, _)) => {
                        f53(Str::lit("generalizing"), f26(v11), f53(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f37(v12))), Adt7::C5))
                    }
                }
            }))
        }
        Adt5::C8(v15, r150, r151) => {
            let v16 = (*r150).clone();
            let v17 = (*r151).clone();
            f56(Str::lit("nested_induction"), f53(Str::lit("scrutinee"), Adt7::C2(v15), f53(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f37(v16))), f53(Str::lit("members"), Adt7::C3(std::rc::Rc::new(f37(v17))), Adt7::C5))))
        }
        Adt5::C9 => {
            f56(Str::lit("congruence"), Adt7::C5)
        }
        Adt5::C10(v18) => {
            f56(Str::lit("boolean_reflection"), f53(Str::lit("reflection"), f63(v18), Adt7::C5))
        }
        Adt5::C11(v19, v20, v21, v22) => {
            f56(Str::lit("law"), f53(Str::lit("name"), Adt7::C2(v19), f59(Str::lit("type_arguments"), v20, f53(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f31(v21))), {
                let m146 = v22.clone();
                match m146.uncons() {
                    None => {
                        Adt7::C5
                    }
                    Some((_, _)) => {
                        f53(Str::lit("premises"), f62(v22), Adt7::C5)
                    }
                }
            }))))
        }
        Adt5::C12(v25, v26, v27, v28) => {
            f56(Str::lit("apply"), f53(Str::lit("theorem"), f29(v25, v26), f59(Str::lit("type_arguments"), v27, f53(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f31(v28))), Adt7::C5))))
        }
        Adt5::C13 => {
            Adt7::C5
        }
        Adt5::C14(r152, r153) => {
            let v29 = (*r152).clone();
            let v30 = (*r153).clone();
            Adt7::C6(std::rc::Rc::new(f37(v29)), std::rc::Rc::new(f37(v30)))
        }
        Adt5::C15(v31, v32, r154, r155) => {
            let v33 = (*r154).clone();
            let v34 = (*r155).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f53(Str::lit("constructor"), Adt7::C2(v31), f53(Str::lit("binders"), f26(v32), f53(Str::lit("proof"), f37(v33), Adt7::C5)))))), std::rc::Rc::new(f37(v34)))
        }
        Adt5::C16(v35, v36, v37, r156, r157) => {
            let v38 = (*r156).clone();
            let v39 = (*r157).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f53(Str::lit("type"), f30(v35), f53(Str::lit("binder"), Adt7::C2(v36), f53(Str::lit("statement"), f31(v37), f53(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f37(v38))), Adt7::C5))))))), std::rc::Rc::new(f37(v39)))
        }
    }
}

pub fn f38(v0: u64) -> R<Bytes> {
    f64(f65(), 10u64, v0)
}

pub fn f39(v0: Fn4, p162: u64, p163: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v1, v2) = m161;
                let m158 = v1;
                if m158 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m158 - 1;
                    let m160 = {
                        let c159: Fn4 = v0.clone();
                        c159.apply(v2.clone())?
                    };
                    match m160 {
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
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f66(v0, v1)
}

pub fn f41(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m164 = f67(Fn9::F68(v1), None::<Bytes>, v0);
    match m164 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f42(v0: Fn5, p168: List<(Bytes, Bytes)>, p169: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m167 = (p168, p169);
    {
        let mut m167 = m167;
        loop {
            let step = {
                let (v1, v2) = m167;
                let m165 = v2;
                match m165.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c166: Fn5 = v0.clone();
                            c166.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f69(v0, v1)
}

pub fn f44(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f70(v0)
}

pub fn f45(v0: Fn6, p173: List<Bytes>, p174: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m172 = (p173, p174);
    {
        let mut m172 = m172;
        loop {
            let step = {
                let (v1, v2) = m172;
                let m170 = v2;
                match m170.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c171: Fn6 = v0.clone();
                            c171.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f71(v1)?, v0))
}

pub fn f47(v0: Fn7, p178: List<Adt7>, p179: List<Bytes>) -> List<Adt7> {
    let m177 = (p178, p179);
    {
        let mut m177 = m177;
        loop {
            let step = {
                let (v1, v2) = m177;
                let m175 = v2;
                match m175.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Bytes>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Bytes>), List<Adt7>>(({
                            let c176: Fn7 = v0.clone();
                            c176.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: List<Adt7>, v1: Bytes) -> List<Adt7> {
    List::cons(Adt7::C2(v1), v0)
}

pub fn f49(v0: List<Bytes>) -> List<Bytes> {
    f72(Fn10::F73, List::<Bytes>::nil(), v0)
}

pub fn f50(v0: Fn8, p183: List<Adt7>, p184: List<(Bytes, Adt1)>) -> List<Adt7> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v1, v2) = m182;
                let m180 = v2;
                match m180.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<(Bytes, Adt1)>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<(Bytes, Adt1)>), List<Adt7>>(({
                            let c181: Fn8 = v0.clone();
                            c181.apply(v1, v3)
                        }, v4))
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

pub fn f51(v0: List<Adt7>, v1: (Bytes, Adt1)) -> List<Adt7> {
    List::cons(f60({
        let (h185, _) = v1;
        h185
    }, {
        let (_, h186) = v1;
        h186
    }), v0)
}

pub fn f52(v0: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    f74(Fn11::F75, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f53(v0: Str, v1: Adt7, v2: Adt7) -> Adt7 {
    Adt7::C7(f76(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f54(v0: Str, v1: Adt7) -> Adt7 {
    Adt7::C4(std::rc::Rc::new(f55(Str::lit("kind"), Adt7::C2(f77(v0)), v1)))
}

pub fn f55(v0: Str, v1: Adt7, v2: Adt7) -> Adt7 {
    Adt7::C7(f77(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f56(v0: Str, v1: Adt7) -> Adt7 {
    Adt7::C4(std::rc::Rc::new(f53(Str::lit("kind"), Adt7::C2(f76(v0)), v1)))
}

pub fn f57(v0: Adt3) -> Bytes {
    let m187 = v0;
    match m187 {
        Adt3::C0 => {
            f77(Str::lit("int"))
        }
        Adt3::C1 => {
            f77(Str::lit("int8"))
        }
        Adt3::C2 => {
            f77(Str::lit("int16"))
        }
        Adt3::C3 => {
            f77(Str::lit("int32"))
        }
        Adt3::C4 => {
            f77(Str::lit("int64"))
        }
        Adt3::C5 => {
            f77(Str::lit("uint8"))
        }
        Adt3::C6 => {
            f77(Str::lit("uint16"))
        }
        Adt3::C7 => {
            f77(Str::lit("uint32"))
        }
        Adt3::C8 => {
            f77(Str::lit("uint64"))
        }
    }
}

pub fn f58(v0: Adt4) -> Adt8 {
    let m188 = v0;
    match m188 {
        Adt4::C0 => {
            Adt8::C0(f77(Str::lit("subtract")), false, false, None::<u64>)
        }
        Adt4::C1 => {
            Adt8::C0(f77(Str::lit("multiply")), false, false, None::<u64>)
        }
        Adt4::C2 => {
            Adt8::C0(f77(Str::lit("quotient")), false, false, None::<u64>)
        }
        Adt4::C3 => {
            Adt8::C0(f77(Str::lit("remainder")), false, false, None::<u64>)
        }
        Adt4::C4 => {
            Adt8::C0(f77(Str::lit("negate")), false, false, None::<u64>)
        }
        Adt4::C5 => {
            Adt8::C0(f77(Str::lit("checked_convert")), false, false, None::<u64>)
        }
        Adt4::C6 => {
            Adt8::C0(f77(Str::lit("checked_add")), false, false, None::<u64>)
        }
        Adt4::C7 => {
            Adt8::C0(f77(Str::lit("checked_subtract")), false, false, None::<u64>)
        }
        Adt4::C8 => {
            Adt8::C0(f77(Str::lit("checked_multiply")), false, false, None::<u64>)
        }
        Adt4::C9 => {
            Adt8::C0(f77(Str::lit("checked_negate")), false, false, None::<u64>)
        }
        Adt4::C10 => {
            Adt8::C0(f77(Str::lit("checked_quotient")), false, false, None::<u64>)
        }
        Adt4::C11 => {
            Adt8::C0(f77(Str::lit("bit_and")), false, false, None::<u64>)
        }
        Adt4::C12 => {
            Adt8::C0(f77(Str::lit("bit_or")), false, false, None::<u64>)
        }
        Adt4::C13 => {
            Adt8::C0(f77(Str::lit("bit_xor")), false, false, None::<u64>)
        }
        Adt4::C14 => {
            Adt8::C0(f77(Str::lit("bit_not")), false, false, None::<u64>)
        }
        Adt4::C15 => {
            Adt8::C0(f77(Str::lit("shift_left")), false, false, None::<u64>)
        }
        Adt4::C16 => {
            Adt8::C0(f77(Str::lit("shift_right")), false, false, None::<u64>)
        }
        Adt4::C17 => {
            Adt8::C0(f77(Str::lit("append")), false, false, None::<u64>)
        }
        Adt4::C18 => {
            Adt8::C0(f77(Str::lit("length")), false, false, None::<u64>)
        }
        Adt4::C19 => {
            Adt8::C0(f77(Str::lit("index")), false, false, None::<u64>)
        }
        Adt4::C20 => {
            Adt8::C0(f77(Str::lit("slice")), false, false, None::<u64>)
        }
        Adt4::C21 => {
            Adt8::C0(f77(Str::lit("utf8_encode")), false, false, None::<u64>)
        }
        Adt4::C22 => {
            Adt8::C0(f77(Str::lit("utf8_decode")), false, false, None::<u64>)
        }
        Adt4::C23 => {
            Adt8::C0(f77(Str::lit("compare_bytes")), false, false, None::<u64>)
        }
        Adt4::C24 => {
            Adt8::C0(f77(Str::lit("equal")), false, false, None::<u64>)
        }
        Adt4::C25 => {
            Adt8::C0(f77(Str::lit("split_exact")), false, false, None::<u64>)
        }
        Adt4::C26 => {
            Adt8::C0(f77(Str::lit("join")), false, false, None::<u64>)
        }
        Adt4::C27 => {
            Adt8::C0(f77(Str::lit("parse_decimal")), false, false, None::<u64>)
        }
        Adt4::C28 => {
            Adt8::C0(f77(Str::lit("format_decimal")), false, false, None::<u64>)
        }
        Adt4::C29 => {
            Adt8::C0(f77(Str::lit("map_insert")), true, false, None::<u64>)
        }
        Adt4::C30 => {
            Adt8::C0(f77(Str::lit("map_remove")), true, false, None::<u64>)
        }
        Adt4::C31 => {
            Adt8::C0(f77(Str::lit("map_lookup")), true, false, None::<u64>)
        }
        Adt4::C32 => {
            Adt8::C0(f77(Str::lit("map_contains")), true, false, None::<u64>)
        }
        Adt4::C33 => {
            Adt8::C0(f77(Str::lit("map_size")), true, false, None::<u64>)
        }
        Adt4::C34 => {
            Adt8::C0(f77(Str::lit("map_keys")), true, false, None::<u64>)
        }
        Adt4::C35 => {
            Adt8::C0(f77(Str::lit("map_values")), true, false, None::<u64>)
        }
        Adt4::C36 => {
            Adt8::C0(f77(Str::lit("map_entries")), true, false, None::<u64>)
        }
        Adt4::C37 => {
            Adt8::C0(f77(Str::lit("map_fold")), true, false, Some(0u64))
        }
        Adt4::C38 => {
            Adt8::C0(f77(Str::lit("set_insert")), true, false, None::<u64>)
        }
        Adt4::C39 => {
            Adt8::C0(f77(Str::lit("set_remove")), true, false, None::<u64>)
        }
        Adt4::C40 => {
            Adt8::C0(f77(Str::lit("set_contains")), true, false, None::<u64>)
        }
        Adt4::C41 => {
            Adt8::C0(f77(Str::lit("set_size")), true, false, None::<u64>)
        }
        Adt4::C42 => {
            Adt8::C0(f77(Str::lit("set_elements")), true, false, None::<u64>)
        }
        Adt4::C43 => {
            Adt8::C0(f77(Str::lit("set_union")), true, false, None::<u64>)
        }
        Adt4::C44 => {
            Adt8::C0(f77(Str::lit("set_intersection")), true, false, None::<u64>)
        }
        Adt4::C45 => {
            Adt8::C0(f77(Str::lit("set_difference")), true, false, None::<u64>)
        }
        Adt4::C46 => {
            Adt8::C0(f77(Str::lit("set_fold")), true, false, Some(0u64))
        }
        Adt4::C47 => {
            Adt8::C0(f77(Str::lit("list_fold")), true, false, Some(0u64))
        }
        Adt4::C48 => {
            Adt8::C0(f77(Str::lit("iterate")), true, false, Some(0u64))
        }
        Adt4::C49 => {
            Adt8::C0(f77(Str::lit("iterate_until")), true, false, Some(0u64))
        }
        Adt4::C50 => {
            Adt8::C0(f77(Str::lit("graph_successors")), true, false, None::<u64>)
        }
        Adt4::C51 => {
            Adt8::C0(f77(Str::lit("graph_reachable")), true, false, None::<u64>)
        }
        Adt4::C52 => {
            Adt8::C0(f77(Str::lit("graph_topological")), true, false, None::<u64>)
        }
        Adt4::C53 => {
            Adt8::C0(f77(Str::lit("less_than")), true, false, None::<u64>)
        }
        Adt4::C54 => {
            Adt8::C0(f77(Str::lit("to_int")), true, true, None::<u64>)
        }
        Adt4::C55 => {
            Adt8::C0(f77(Str::lit("to_nat")), true, true, None::<u64>)
        }
    }
}

pub fn f59(v0: Str, v1: Adt1, v2: Adt7) -> Adt7 {
    let m189 = v1.clone();
    match m189 {
        Adt1::C0 => {
            v2
        }
        Adt1::C1 => {
            v2
        }
        Adt1::C2 => {
            v2
        }
        Adt1::C3 => {
            v2
        }
        Adt1::C4 => {
            v2
        }
        Adt1::C5 => {
            v2
        }
        Adt1::C6 => {
            v2
        }
        Adt1::C7 => {
            v2
        }
        Adt1::C8 => {
            v2
        }
        Adt1::C9 => {
            v2
        }
        Adt1::C10 => {
            v2
        }
        Adt1::C11 => {
            v2
        }
        Adt1::C12 => {
            v2
        }
        Adt1::C13 => {
            v2
        }
        Adt1::C14 => {
            v2
        }
        Adt1::C15 => {
            v2
        }
        Adt1::C16 => {
            v2
        }
        Adt1::C17 => {
            v2
        }
        Adt1::C18 => {
            v2
        }
        Adt1::C19(_) => {
            v2
        }
        Adt1::C20(_) => {
            v2
        }
        Adt1::C21(_, _) => {
            v2
        }
        Adt1::C22(_) => {
            v2
        }
        Adt1::C23(_, _, _) => {
            v2
        }
        Adt1::C24(_, _) => {
            v2
        }
        Adt1::C25(_, _) => {
            v2
        }
        Adt1::C26(_, _) => {
            v2
        }
        Adt1::C27(_) => {
            v2
        }
        Adt1::C28 => {
            v2
        }
        Adt1::C29(_, _) => {
            f53(v0, Adt7::C3(std::rc::Rc::new(f30(v1))), v2)
        }
    }
}

pub fn f60(v0: Bytes, v1: Adt1) -> Adt7 {
    Adt7::C4(std::rc::Rc::new(f53(Str::lit("name"), Adt7::C2(v0), f53(Str::lit("type"), f30(v1), Adt7::C5))))
}

pub fn f61(v0: List<Adt7>, v1: (Bytes, Adt1)) -> List<Adt7> {
    List::cons(Adt7::C4(std::rc::Rc::new(f2(Str::lit("name"), f3({
        let (h190, _) = v1;
        h190
    }), f2(Str::lit("fields"), Adt7::C3(std::rc::Rc::new(f30({
        let (_, h191) = v1;
        h191
    }))), f8())))), v0)
}

pub fn f62(v0: List<(Option<Bytes>, Bytes)>) -> Adt7 {
    Adt7::C3(std::rc::Rc::new(f4(f78(Fn12::F79, List::<Adt7>::nil(), f80(v0)))))
}

pub fn f63(v0: Adt6) -> Adt7 {
    let m192 = v0;
    match m192 {
        Adt6::C0(v1, v2, v3, v4, v5, v6, v7) => {
            f56(Str::lit("list"), f53(Str::lit("parameter"), Adt7::C2(v1), f53(Str::lit("values"), Adt7::C2(v2), f53(Str::lit("boolean_definition"), f29(v3, v4), f53(Str::lit("proposition_definition"), f29(v5, v6), f53(Str::lit("comparison"), Adt7::C2(v7), Adt7::C5))))))
        }
        Adt6::C1(v8, v9, v10, v11, v12, v13) => {
            f56(Str::lit("record"), f53(Str::lit("record"), Adt7::C2(v8), f53(Str::lit("boolean_definition"), f29(v9, v10), f53(Str::lit("proposition_definition"), f29(v11, v12), f53(Str::lit("fields"), Adt7::C3(std::rc::Rc::new(f4(f81(Fn13::F82, List::<Adt7>::nil(), f83(v13))))), Adt7::C5)))))
        }
    }
}

pub fn f64(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a193 = v2;
    let a194 = 0u64;
    if nat_eq(a193, a194) {
        Ok(f84(v0, 0u64))
    } else {
        let (_, h195) = {
            let (h196, _) = f39(Fn4::F85(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h196
        };
        Ok(h195)
    }
}

pub fn f65() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f66(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m200 = {
        let a198 = v0.clone();
        let a199 = {
            let (h197, _) = v1.clone();
            h197
        };
        index_bytes(a198, a199)
    };
    match m200 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a202 = {
                    let (h201, _) = v1.clone();
                    h201
                };
                let a203 = 1u64;
                nat_add(a202, a203)?
            }, {
                let a206 = {
                    let (_, h204) = v1;
                    h204
                };
                let a207 = f86(v0, {
                    let (h205, _) = v1;
                    h205
                }, v2)?;
                append_bytes(a206, a207)
            })))
        }
    }
}

pub fn f67(v0: Fn9, p211: Option<Bytes>, p212: List<Bytes>) -> Option<Bytes> {
    let m210 = (p211, p212);
    {
        let mut m210 = m210;
        loop {
            let step = {
                let (v1, v2) = m210;
                let m208 = v2;
                match m208.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c209: Fn9 = v0.clone();
                            c209.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m213 = v1;
    match m213 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a216 = {
                    let a214 = v3;
                    let a215 = v0;
                    append_bytes(a214, a215)
                };
                let a217 = v2;
                append_bytes(a216, a217)
            })
        }
    }
}

pub fn f69(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m218 = v0.clone();
    match m218.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a221 = {
                let (h219, _) = v2;
                h219
            };
            let a222 = {
                let (h220, _) = v1.clone();
                h220
            };
            if equal(a221, a222) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f70(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f87(f88(Fn14::F89, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m227 = {
        let (h223, _) = f90(Fn15::F91, {
            let a225 = {
                let a224 = v0;
                length_list(a224)
            };
            let a226 = 1u64;
            nat_add(a225, a226)?
        }, v1)?;
        h223
    };
    match m227.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f71(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a232 = {
        let a229 = f18({
            let (h228, _) = v0;
            h228
        })?;
        let a230 = Bytes::lit(&[0x3a]);
        append_bytes(a229, a230)
    };
    let a233 = {
        let (_, h231) = v0;
        h231
    };
    Ok(append_bytes(a232, a233))
}

pub fn f72(v0: Fn10, p237: List<Bytes>, p238: List<Bytes>) -> List<Bytes> {
    let m236 = (p237, p238);
    {
        let mut m236 = m236;
        loop {
            let step = {
                let (v1, v2) = m236;
                let m234 = v2;
                match m234.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c235: Fn10 = v0.clone();
                            c235.apply(v1, v3)
                        }, v4))
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

pub fn f73(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f74(v0: Fn11, p242: List<(Bytes, Adt1)>, p243: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    let m241 = (p242, p243);
    {
        let mut m241 = m241;
        loop {
            let step = {
                let (v1, v2) = m241;
                let m239 = v2;
                match m239.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(({
                            let c240: Fn11 = v0.clone();
                            c240.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f76(v0: Str) -> Bytes {
    let a244 = v0;
    utf8_encode(a244)
}

pub fn f77(v0: Str) -> Bytes {
    let a245 = v0;
    utf8_encode(a245)
}

pub fn f78(v0: Fn12, p249: List<Adt7>, p250: List<(Option<Bytes>, Bytes)>) -> List<Adt7> {
    let m248 = (p249, p250);
    {
        let mut m248 = m248;
        loop {
            let step = {
                let (v1, v2) = m248;
                let m246 = v2;
                match m246.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<(Option<Bytes>, Bytes)>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<(Option<Bytes>, Bytes)>), List<Adt7>>(({
                            let c247: Fn12 = v0.clone();
                            c247.apply(v1, v3)
                        }, v4))
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

pub fn f79(v0: List<Adt7>, v1: (Option<Bytes>, Bytes)) -> List<Adt7> {
    List::cons(f29({
        let (h251, _) = v1;
        h251
    }, {
        let (_, h252) = v1;
        h252
    }), v0)
}

pub fn f80(v0: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    f92(Fn16::F93, List::<(Option<Bytes>, Bytes)>::nil(), v0)
}

pub fn f81(v0: Fn13, p256: List<Adt7>, p257: List<(Bytes, Bytes)>) -> List<Adt7> {
    let m255 = (p256, p257);
    {
        let mut m255 = m255;
        loop {
            let step = {
                let (v1, v2) = m255;
                let m253 = v2;
                match m253.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<(Bytes, Bytes)>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<(Bytes, Bytes)>), List<Adt7>>(({
                            let c254: Fn13 = v0.clone();
                            c254.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: List<Adt7>, v1: (Bytes, Bytes)) -> List<Adt7> {
    List::cons(Adt7::C4(std::rc::Rc::new(f53(Str::lit("field"), Adt7::C2({
        let (h258, _) = v1;
        h258
    }), f53(Str::lit("expected"), Adt7::C2({
        let (_, h259) = v1;
        h259
    }), Adt7::C5)))), v0)
}

pub fn f83(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f42(Fn5::F94, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f84(v0: Bytes, v1: u64) -> Bytes {
    let m263 = {
        let a260 = v0;
        let a261 = v1;
        let a262 = 1u64;
        slice_bytes(a260, a261, a262)
    };
    match m263 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f85(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f95(v1, v0, v2)
}

pub fn f86(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a308 = {
            let a284 = {
                let a272 = {
                    let a266 = {
                        let a264 = v3;
                        let a265 = 128u8;
                        fixed_u8::bit_and(a264, a265)
                    };
                    let a267 = 128u8;
                    if equal(a266, a267) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a273 = {
                    let a270 = {
                        let a268 = v3;
                        let a269 = 64u8;
                        fixed_u8::bit_and(a268, a269)
                    };
                    let a271 = 64u8;
                    if equal(a270, a271) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a272, a273)?
            };
            let a285 = {
                let a282 = {
                    let a276 = {
                        let a274 = v3;
                        let a275 = 32u8;
                        fixed_u8::bit_and(a274, a275)
                    };
                    let a277 = 32u8;
                    if equal(a276, a277) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a283 = {
                    let a280 = {
                        let a278 = v3;
                        let a279 = 16u8;
                        fixed_u8::bit_and(a278, a279)
                    };
                    let a281 = 16u8;
                    if equal(a280, a281) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a282, a283)?
            };
            nat_add(a284, a285)?
        };
        let a309 = {
            let a306 = {
                let a294 = {
                    let a288 = {
                        let a286 = v3;
                        let a287 = 8u8;
                        fixed_u8::bit_and(a286, a287)
                    };
                    let a289 = 8u8;
                    if equal(a288, a289) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a295 = {
                    let a292 = {
                        let a290 = v3;
                        let a291 = 4u8;
                        fixed_u8::bit_and(a290, a291)
                    };
                    let a293 = 4u8;
                    if equal(a292, a293) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a294, a295)?
            };
            let a307 = {
                let a304 = {
                    let a298 = {
                        let a296 = v3;
                        let a297 = 2u8;
                        fixed_u8::bit_and(a296, a297)
                    };
                    let a299 = 2u8;
                    if equal(a298, a299) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a305 = {
                    let a302 = {
                        let a300 = v3;
                        let a301 = 1u8;
                        fixed_u8::bit_and(a300, a301)
                    };
                    let a303 = 1u8;
                    if equal(a302, a303) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a304, a305)?
            };
            nat_add(a306, a307)?
        };
        nat_add(a308, a309)?
    };
    let a310 = v4;
    let a311 = 34u64;
    if nat_eq(a310, a311) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a312 = v4;
        let a313 = 92u64;
        if nat_eq(a312, a313) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a314 = v4;
            let a315 = 8u64;
            if nat_eq(a314, a315) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a316 = v4;
                let a317 = 12u64;
                if nat_eq(a316, a317) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a318 = v4;
                    let a319 = 10u64;
                    if nat_eq(a318, a319) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a320 = v4;
                        let a321 = 13u64;
                        if nat_eq(a320, a321) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a322 = v4;
                            let a323 = 9u64;
                            if nat_eq(a322, a323) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a324 = v4;
                                let a325 = 32u64;
                                if nat_lt(a324, a325) {
                                    let a326 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a327 = f96(v2)?;
                                    Ok(append_bytes(a326, a327))
                                } else {
                                    Ok(f97(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f87(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f98(Fn17::F99, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f88(v0: Fn14, p331: List<List<(Bytes, Bytes)>>, p332: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m330 = (p331, p332);
    {
        let mut m330 = m330;
        loop {
            let step = {
                let (v1, v2) = m330;
                let m328 = v2;
                match m328.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c329: Fn14 = v0.clone();
                            c329.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f90(v0: Fn15, p337: u64, p338: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m336 = (p337, p338);
    {
        let mut m336 = m336;
        loop {
            let step = {
                let (v1, v2) = m336;
                let m333 = v1;
                if m333 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m333 - 1;
                    let m335 = {
                        let c334: Fn15 = v0.clone();
                        c334.apply(v2.clone())?
                    };
                    match m335 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m336 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f100(v0)
}

pub fn f92(v0: Fn16, p342: List<(Option<Bytes>, Bytes)>, p343: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    let m341 = (p342, p343);
    {
        let mut m341 = m341;
        loop {
            let step = {
                let (v1, v2) = m341;
                let m339 = v2;
                match m339.uncons() {
                    None => {
                        Err::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(({
                            let c340: Fn16 = v0.clone();
                            c340.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m341 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: List<(Option<Bytes>, Bytes)>, v1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
    List::cons(v1, v0)
}

pub fn f94(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f95(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h344, _) = v2;
        h344
    };
    let a345 = v3;
    let a346 = 0u64;
    if nat_eq(a345, a346) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a347 = v3;
            let a348 = v1;
            let a349 = 0u64;
            nat_quot(a347, a348, a349)
        }, {
            let a354 = f84(v0, {
                let a350 = v3;
                let a351 = v1;
                let a352 = 0u64;
                nat_rem(a350, a351, a352)
            });
            let a355 = {
                let (_, h353) = v2;
                h353
            };
            append_bytes(a354, a355)
        }))
    }
}

pub fn f96(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a400 = {
            let a376 = {
                let a364 = {
                    let a358 = {
                        let a356 = v1;
                        let a357 = 128u8;
                        fixed_u8::bit_and(a356, a357)
                    };
                    let a359 = 128u8;
                    if equal(a358, a359) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a365 = {
                    let a362 = {
                        let a360 = v1;
                        let a361 = 64u8;
                        fixed_u8::bit_and(a360, a361)
                    };
                    let a363 = 64u8;
                    if equal(a362, a363) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a364, a365)?
            };
            let a377 = {
                let a374 = {
                    let a368 = {
                        let a366 = v1;
                        let a367 = 32u8;
                        fixed_u8::bit_and(a366, a367)
                    };
                    let a369 = 32u8;
                    if equal(a368, a369) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a375 = {
                    let a372 = {
                        let a370 = v1;
                        let a371 = 16u8;
                        fixed_u8::bit_and(a370, a371)
                    };
                    let a373 = 16u8;
                    if equal(a372, a373) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a374, a375)?
            };
            nat_add(a376, a377)?
        };
        let a401 = {
            let a398 = {
                let a386 = {
                    let a380 = {
                        let a378 = v1;
                        let a379 = 8u8;
                        fixed_u8::bit_and(a378, a379)
                    };
                    let a381 = 8u8;
                    if equal(a380, a381) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a387 = {
                    let a384 = {
                        let a382 = v1;
                        let a383 = 4u8;
                        fixed_u8::bit_and(a382, a383)
                    };
                    let a385 = 4u8;
                    if equal(a384, a385) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a386, a387)?
            };
            let a399 = {
                let a396 = {
                    let a390 = {
                        let a388 = v1;
                        let a389 = 2u8;
                        fixed_u8::bit_and(a388, a389)
                    };
                    let a391 = 2u8;
                    if equal(a390, a391) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a397 = {
                    let a394 = {
                        let a392 = v1;
                        let a393 = 1u8;
                        fixed_u8::bit_and(a392, a393)
                    };
                    let a395 = 1u8;
                    if equal(a394, a395) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a396, a397)?
            };
            nat_add(a398, a399)?
        };
        nat_add(a400, a401)?
    };
    let a408 = f84(f101(), {
        let a402 = v2;
        let a403 = 16u64;
        let a404 = 0u64;
        nat_quot(a402, a403, a404)
    });
    let a409 = f84(f101(), {
        let a405 = v2;
        let a406 = 16u64;
        let a407 = 0u64;
        nat_rem(a405, a406, a407)
    });
    Ok(append_bytes(a408, a409))
}

pub fn f97(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m413 = {
        let a410 = v0;
        let a411 = v1;
        let a412 = v2;
        slice_bytes(a410, a411, a412)
    };
    match m413 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f98(v0: Fn17, p417: List<List<(Bytes, Bytes)>>, p418: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m416 = (p417, p418);
    {
        let mut m416 = m416;
        loop {
            let step = {
                let (v1, v2) = m416;
                let m414 = v2;
                match m414.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c415: Fn17 = v0.clone();
                            c415.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m416 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f100(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a420 = {
        let a419 = v0.clone();
        length_list(a419)
    };
    let a421 = 1u64;
    if nat_le(a420, a421) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f102(v0)?))
    }
}

pub fn f101() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f102(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f87({
        let (_, h422) = {
            let (h423, _) = f103(Fn18::F104, {
                let a425 = {
                    let a424 = v0.clone();
                    length_list(a424)
                };
                let a426 = 1u64;
                nat_add(a425, a426)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h423
        };
        h422
    }))
}

pub fn f103(v0: Fn18, p431: u64, p432: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m430 = (p431, p432);
    {
        let mut m430 = m430;
        loop {
            let step = {
                let (v1, v2) = m430;
                let m427 = v1;
                if m427 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m427 - 1;
                    let m429 = {
                        let c428: Fn18 = v0.clone();
                        c428.apply(v2.clone())?
                    };
                    match m429 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>((v3, v4))
                        }
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

pub fn f104(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f105(v0)
}

pub fn f105(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h433, _) = v0;
        h433
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h434) = v0;
        h434
    };
    let m435 = v1;
    match m435.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m436 = v4;
            match m436.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f106(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f106(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a441 = {
            let a439 = {
                let a437 = v0.clone();
                length_list(a437)
            };
            let a440 = {
                let a438 = v1.clone();
                length_list(a438)
            };
            nat_add(a439, a440)?
        };
        let a442 = 1u64;
        nat_add(a441, a442)?
    };
    Ok(f83({
        let (_, h443) = {
            let (_, h444) = {
                let (h445, _) = f107(Fn19::F108, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h445
            };
            h444
        };
        h443
    }))
}

pub fn f107(v0: Fn19, p450: u64, p451: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m449 = (p450, p451);
    {
        let mut m449 = m449;
        loop {
            let step = {
                let (v1, v2) = m449;
                let m446 = v1;
                if m446 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m446 - 1;
                    let m448 = {
                        let c447: Fn19 = v0.clone();
                        c447.apply(v2.clone())
                    };
                    match m448 {
                        None => {
                            Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f109(v0)
}

pub fn f109(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h452, _) = v0.clone();
        h452
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h453, _) = {
            let (_, h454) = v0.clone();
            h454
        };
        h453
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h455) = {
            let (_, h456) = v0;
            h456
        };
        h455
    };
    let m457 = v1.clone();
    match m457.uncons() {
        None => {
            let m458 = v2;
            match m458.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m459 = v2.clone();
            match m459.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f110({
                        let (h460, _) = v8.clone();
                        h460
                    }, {
                        let (h461, _) = v6.clone();
                        h461
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

pub fn f110(v0: Bytes, v1: Bytes) -> bool {
    let a466 = {
        let a462 = v0;
        let a463 = v1;
        compare_bytes(a462, a463)
    };
    let a467 = {
        let a464 = Bytes::lit(&[]);
        let a465 = Bytes::lit(&[0x01]);
        compare_bytes(a464, a465)
    };
    equal(a466, a467)
}
