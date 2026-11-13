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
    C0(Bytes, Adt1, Bytes, u64, Adt2, Adt4, List<Bytes>),
    C1(Bytes, List<Bytes>, Adt5, Adt5, Option<Adt6>, Option<(Option<Bytes>, Bytes)>, Option<(Option<Bytes>, Bytes)>, Option<(Option<Bytes>, Bytes)>, List<Adt7>, List<Bytes>),
    C2(Bytes, List<Bytes>, Adt5, Adt4, Option<Adt9>, Adt13, bool, List<Bytes>),
    C3(Bytes, List<Bytes>, Adt20, Adt20, List<Adt22>, List<Bytes>),
    C4(Bytes, List<Bytes>, Adt20, Adt20, List<Adt20>, List<(Option<Bytes>, Bytes)>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(Adt3, List<u64>),
    C2,
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
    C19(Bytes),
    C20(std::rc::Rc<Adt4>),
    C21(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
    C22(std::rc::Rc<Adt4>),
    C23(Option<Bytes>, Bytes, std::rc::Rc<Adt4>),
    C24(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
    C25(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
    C26(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
    C27(std::rc::Rc<Adt4>),
    C28,
    C29(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, Adt4),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, Bytes, Adt4),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt8, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), Option<(Option<Bytes>, Bytes)>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Adt4, Adt10),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Bytes),
    C1(Bytes),
    C2(Adt11, Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Adt12, std::rc::Rc<Adt10>, Adt4),
    C6(bool),
    C7,
    C8(Adt4),
    C9(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C10(Option<Bytes>, Bytes, Adt4, std::rc::Rc<Adt10>),
    C11(Option<Bytes>, Bytes, Adt4, std::rc::Rc<Adt10>),
    C12(Option<Bytes>, Bytes, Adt4, Option<Bytes>, Bytes),
    C13(std::rc::Rc<Adt10>, Bytes),
    C14(Option<Bytes>, Bytes, Adt4, std::rc::Rc<Adt10>),
    C15(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C16(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C17(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C18(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C19(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C20(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C21(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C22(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C23(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C24(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C25(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C26(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C27(std::rc::Rc<Adt10>),
    C28(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C29(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C30(Bytes, Adt4, std::rc::Rc<Adt10>),
    C31(Bytes, Adt4, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C32(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C33(std::rc::Rc<Adt10>),
    C34(std::rc::Rc<Adt10>),
    C35(List<(Bytes, Adt4)>, List<Bytes>, std::rc::Rc<Adt10>),
    C36(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C37(Option<Bytes>, Bytes, Adt4),
    C38(Adt4, Adt4, std::rc::Rc<Adt10>),
    C39(Adt4, std::rc::Rc<Adt10>),
    C40(Adt4, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C41(Option<Bytes>, Bytes, Adt4, std::rc::Rc<Adt10>, List<Bytes>),
    C42,
    C43(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C44(Bytes, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C45(Option<Bytes>, Bytes, List<Bytes>, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C46(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C47(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
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
pub enum Adt12 {
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
pub enum Adt13 {
    C0(Adt10),
    C1(List<Adt14>, Adt10),
    C2(Adt15, Adt10, u64, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), List<Adt10>),
    C3(Adt16, Adt10, u64, (Option<Bytes>, Bytes), List<Adt17>, Adt18),
    C4(Adt19),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Bytes, Adt10, Adt10),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64, u64, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes)),
    C1,
    C2(u64, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Adt10>),
    C1(Bytes, Adt10),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(List<Adt20>, List<Adt21>),
    C1(Adt20, Adt20),
    C2(Adt20, Adt20),
    C3(Adt10, Adt20, Adt20),
    C4(Adt20, Adt21),
}

#[derive(Clone)]
pub enum Adt20 {
    C0((Option<Bytes>, Bytes), List<Adt4>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1((Option<Bytes>, Bytes)),
    C2,
}

#[derive(Clone)]
pub enum Adt22 {
    C0((Option<Bytes>, Bytes), (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), u64, u64, (Option<Bytes>, Bytes)),
    C1(Adt20, (Option<Bytes>, Bytes)),
    C2((Option<Bytes>, Bytes)),
    C3((Option<Bytes>, Bytes)),
    C4((Option<Bytes>, Bytes)),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt23>),
    C4(std::rc::Rc<Adt23>),
    C5,
    C6(std::rc::Rc<Adt23>, std::rc::Rc<Adt23>),
    C7(Bytes, std::rc::Rc<Adt23>, std::rc::Rc<Adt23>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, bool, bool, Option<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F10,
    F13,
    F20,
    F21,
    F24,
    F25,
    F30,
    F31,
    F38,
    F136,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt23>, p1: Bytes) -> List<Adt23> {
        match self {
            Fn0::F10 => f10(p0, p1),
            Fn0::F13 => f13(p0, p1),
            Fn0::F20 => f20(p0, p1),
            Fn0::F21 => f21(p0, p1),
            Fn0::F24 => f24(p0, p1),
            Fn0::F25 => f25(p0, p1),
            Fn0::F30 => f30(p0, p1),
            Fn0::F31 => f31(p0, p1),
            Fn0::F38 => f38(p0, p1),
            Fn0::F136 => f136(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt7) -> List<Adt23> {
        match self {
            Fn1::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F28,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt22) -> List<Adt23> {
        match self {
            Fn2::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F33,
    F118,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt20) -> List<Adt23> {
        match self {
            Fn3::F33 => f33(p0, p1),
            Fn3::F118 => f118(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F36,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt23>, p1: (Option<Bytes>, Bytes)) -> List<Adt23> {
        match self {
            Fn4::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F47,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt23>, p1: u64) -> List<Adt23> {
        match self {
            Fn5::F47 => f47(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F52,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt23, p1: Adt23) -> Adt23 {
        match self {
            Fn6::F52 => f52(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F55,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn7::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F57,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt23, p1: (Bytes, Adt23)) -> Adt23 {
        match self {
            Fn8::F57 => f57(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F61,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt7) -> List<Adt7> {
        match self {
            Fn9::F61 => f61(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F64,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt14) -> List<Adt23> {
        match self {
            Fn10::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F68,
    F117,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt10) -> List<Adt23> {
        match self {
            Fn11::F68 => f68(p0, p1),
            Fn11::F117 => f117(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F72,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt17) -> List<Adt23> {
        match self {
            Fn12::F72 => f72(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F77,
}

impl Fn13 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt4) -> List<Adt23> {
        match self {
            Fn13::F77 => f77(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F81,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt22>, p1: Adt22) -> List<Adt22> {
        match self {
            Fn14::F81 => f81(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F83,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Adt20>, p1: Adt20) -> List<Adt20> {
        match self {
            Fn15::F83 => f83(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F85,
}

impl Fn16 {
    pub fn apply(&self, p0: List<(Option<Bytes>, Bytes)>, p1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
        match self {
            Fn16::F85 => f85(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F88(Bytes),
    F140(u64, Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn17::F88(k0) => f88(k0.clone(), p0),
            Fn17::F140(k0, k1) => Ok(f140(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F91,
    F164,
}

impl Fn18 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn18::F91 => f91(p0, p1),
            Fn18::F164 => f164(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F94,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn19::F94 => f94(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F96,
}

impl Fn20 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn20::F96 => f96(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F99,
}

impl Fn21 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt23) -> List<Adt23> {
        match self {
            Fn21::F99 => f99(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F111,
}

impl Fn22 {
    pub fn apply(&self, p0: List<Adt14>, p1: Adt14) -> List<Adt14> {
        match self {
            Fn22::F111 => f111(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F113,
}

impl Fn23 {
    pub fn apply(&self, p0: List<Adt10>, p1: Adt10) -> List<Adt10> {
        match self {
            Fn23::F113 => f113(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F116,
}

impl Fn24 {
    pub fn apply(&self, p0: List<Adt17>, p1: Adt17) -> List<Adt17> {
        match self {
            Fn24::F116 => f116(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F120,
}

impl Fn25 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt21) -> List<Adt23> {
        match self {
            Fn25::F120 => f120(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F124,
}

impl Fn26 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn26::F124 => f124(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F129(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn27::F129(k0) => f129(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F134,
}

impl Fn28 {
    pub fn apply(&self, p0: List<Adt23>, p1: (Bytes, Adt4)) -> List<Adt23> {
        match self {
            Fn28::F134 => f134(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F138,
}

impl Fn29 {
    pub fn apply(&self, p0: List<Adt21>, p1: Adt21) -> List<Adt21> {
        match self {
            Fn29::F138 => f138(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F144,
}

impl Fn30 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn30::F144 => f144(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F146,
}

impl Fn31 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn31::F146 => f146(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F148,
}

impl Fn32 {
    pub fn apply(&self, p0: List<(Bytes, Adt4)>, p1: (Bytes, Adt4)) -> List<(Bytes, Adt4)> {
        match self {
            Fn32::F148 => f148(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F153,
}

impl Fn33 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn33::F153 => f153(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F158,
}

impl Fn34 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn34::F158 => f158(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F163,
}

impl Fn35 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn35::F163 => f163(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0))
}

pub fn f1(v0: Adt23) -> R<Bytes> {
    f3(v0)
}

pub fn f2(v0: Adt0) -> Adt23 {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7) => {
            let v8: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("artifact")))), List::<(Bytes, Adt23)>::nil());
            let v9: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v1)), v8);
            let v10: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("role")), f5(v2)), v9);
            let v11: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("sha256")), Adt23::C2(v3)), v10);
            let v12: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("length")), Adt23::C1(false, v4)), v11);
            let v13: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("schema")), f6(v5)), v12);
            let v14: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("type")), f7(v6)), v13);
            let v17: List<(Bytes, Adt23)> = {
                let m2 = v7.clone();
                match m2.uncons() {
                    None => {
                        v14
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F10, List::<Adt23>::nil(), f11(v7)))))), v14)
                    }
                }
            };
            f12(v17)
        }
        Adt0::C1(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
            let v28: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("contract")))), List::<(Bytes, Adt23)>::nil());
            let v29: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v18)), v28);
            let v32: List<(Bytes, Adt23)> = {
                let m3 = v19.clone();
                match m3.uncons() {
                    None => {
                        v29
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F13, List::<Adt23>::nil(), f11(v19)))))), v29)
                    }
                }
            };
            let v33: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("input")), f14(v20)), v32);
            let v34: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("output")), f14(v21)), v33);
            let v36: List<(Bytes, Adt23)> = {
                let m4 = v22;
                match m4 {
                    None => {
                        v34
                    }
                    Some(v35) => {
                        List::cons((f4(Str::lit("state")), f15(v35)), v34)
                    }
                }
            };
            let v38: List<(Bytes, Adt23)> = {
                let m5 = v23;
                match m5 {
                    None => {
                        v36
                    }
                    Some(v37) => {
                        List::cons((f4(Str::lit("precondition")), f16({
                            let (h6, _) = v37;
                            h6
                        }, {
                            let (_, h7) = v37;
                            h7
                        })), v36)
                    }
                }
            };
            let v40: List<(Bytes, Adt23)> = {
                let m8 = v24;
                match m8 {
                    None => {
                        v38
                    }
                    Some(v39) => {
                        List::cons((f4(Str::lit("postcondition")), f16({
                            let (h9, _) = v39;
                            h9
                        }, {
                            let (_, h10) = v39;
                            h10
                        })), v38)
                    }
                }
            };
            let v42: List<(Bytes, Adt23)> = {
                let m11 = v25;
                match m11 {
                    None => {
                        v40
                    }
                    Some(v41) => {
                        List::cons((f4(Str::lit("invariant")), f16({
                            let (h12, _) = v41;
                            h12
                        }, {
                            let (_, h13) = v41;
                            h13
                        })), v40)
                    }
                }
            };
            let v45: List<(Bytes, Adt23)> = {
                let m14 = v26.clone();
                match m14.uncons() {
                    None => {
                        v42
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("validators")), Adt23::C3(std::rc::Rc::new(f8(f17(Fn1::F18, List::<Adt23>::nil(), f19(v26)))))), v42)
                    }
                }
            };
            let v48: List<(Bytes, Adt23)> = {
                let m15 = v27.clone();
                match m15.uncons() {
                    None => {
                        v45
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F20, List::<Adt23>::nil(), f11(v27)))))), v45)
                    }
                }
            };
            f12(v48)
        }
        Adt0::C2(v49, v50, v51, v52, v53, v54, v55, v56) => {
            let v57: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("realization")))), List::<(Bytes, Adt23)>::nil());
            let v58: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v49)), v57);
            let v61: List<(Bytes, Adt23)> = {
                let m16 = v50.clone();
                match m16.uncons() {
                    None => {
                        v58
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F21, List::<Adt23>::nil(), f11(v50)))))), v58)
                    }
                }
            };
            let v62: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("input")), f14(v51)), v61);
            let v63: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("output")), f7(v52)), v62);
            let v65: List<(Bytes, Adt23)> = {
                let m17 = v53;
                match m17 {
                    None => {
                        v63
                    }
                    Some(v64) => {
                        List::cons((f4(Str::lit("state")), f22(v64)), v63)
                    }
                }
            };
            let v66: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("descriptor")), f23(v54)), v65);
            let v67: List<(Bytes, Adt23)> = if v55 {
                List::cons((f4(Str::lit("executable")), Adt23::C0(true)), v66)
            } else {
                v66
            };
            let v70: List<(Bytes, Adt23)> = {
                let m18 = v56.clone();
                match m18.uncons() {
                    None => {
                        v67
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F24, List::<Adt23>::nil(), f11(v56)))))), v67)
                    }
                }
            };
            f12(v70)
        }
        Adt0::C3(v71, v72, v73, v74, v75, v76) => {
            let v77: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("evidence")))), List::<(Bytes, Adt23)>::nil());
            let v78: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v71)), v77);
            let v81: List<(Bytes, Adt23)> = {
                let m19 = v72.clone();
                match m19.uncons() {
                    None => {
                        v78
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F25, List::<Adt23>::nil(), f11(v72)))))), v78)
                    }
                }
            };
            let v82: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("contract")), f26(v73)), v81);
            let v83: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("realization")), f26(v74)), v82);
            let v84: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("claims")), Adt23::C3(std::rc::Rc::new(f8(f27(Fn2::F28, List::<Adt23>::nil(), f29(v75)))))), v83);
            let v87: List<(Bytes, Adt23)> = {
                let m20 = v76.clone();
                match m20.uncons() {
                    None => {
                        v84
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F30, List::<Adt23>::nil(), f11(v76)))))), v84)
                    }
                }
            };
            f12(v87)
        }
        Adt0::C4(v88, v89, v90, v91, v92, v93, v94) => {
            let v95: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("model")))), List::<(Bytes, Adt23)>::nil());
            let v96: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v88)), v95);
            let v99: List<(Bytes, Adt23)> = {
                let m21 = v89.clone();
                match m21.uncons() {
                    None => {
                        v96
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F31, List::<Adt23>::nil(), f11(v89)))))), v96)
                    }
                }
            };
            let v100: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("contract")), f26(v90)), v99);
            let v101: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("realization")), f26(v91)), v100);
            let v104: List<(Bytes, Adt23)> = {
                let m22 = v92.clone();
                match m22.uncons() {
                    None => {
                        v101
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("evidence")), Adt23::C3(std::rc::Rc::new(f8(f32(Fn3::F33, List::<Adt23>::nil(), f34(v92)))))), v101)
                    }
                }
            };
            let v107: List<(Bytes, Adt23)> = {
                let m23 = v93.clone();
                match m23.uncons() {
                    None => {
                        v104
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("entry")), Adt23::C3(std::rc::Rc::new(f8(f35(Fn4::F36, List::<Adt23>::nil(), f37(v93)))))), v104)
                    }
                }
            };
            let v110: List<(Bytes, Adt23)> = {
                let m24 = v94.clone();
                match m24.uncons() {
                    None => {
                        v107
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F38, List::<Adt23>::nil(), f11(v94)))))), v107)
                    }
                }
            };
            f12(v110)
        }
    }
}

pub fn f3(v0: Adt23) -> R<Bytes> {
    let m25 = v0;
    match m25 {
        Adt23::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt23::C1(v2, v3) => {
            f39(v2, v3)
        }
        Adt23::C2(v4) => {
            f40(v4)
        }
        Adt23::C3(r26) => {
            let v5 = (*r26).clone();
            Ok(f41(f42(v5)?))
        }
        Adt23::C4(r27) => {
            let v6 = (*r27).clone();
            f43(f44(v6)?)
        }
        Adt23::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt23::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt23::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Str) -> Bytes {
    let a28 = v0;
    utf8_encode(a28)
}

pub fn f5(v0: Adt1) -> Adt23 {
    let m29 = v0;
    match m29 {
        Adt1::C0 => {
            Adt23::C2(f4(Str::lit("parameters")))
        }
        Adt1::C1 => {
            Adt23::C2(f4(Str::lit("vocabulary")))
        }
        Adt1::C2 => {
            Adt23::C2(f4(Str::lit("dataset")))
        }
        Adt1::C3 => {
            Adt23::C2(f4(Str::lit("table")))
        }
        Adt1::C4 => {
            Adt23::C2(f4(Str::lit("binary")))
        }
    }
}

pub fn f6(v0: Adt2) -> Adt23 {
    let m30 = v0;
    match m30 {
        Adt2::C0 => {
            let v1: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("bytes")))), List::<(Bytes, Adt23)>::nil());
            f12(v1)
        }
        Adt2::C1(v2, v3) => {
            let v4: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("int_tensor")))), List::<(Bytes, Adt23)>::nil());
            let v5: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("element")), f45(v2)), v4);
            let v6: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("shape")), Adt23::C3(std::rc::Rc::new(f8(f46(Fn5::F47, List::<Adt23>::nil(), f48(v3)))))), v5);
            f12(v6)
        }
        Adt2::C2 => {
            let v7: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("utf8_lines")))), List::<(Bytes, Adt23)>::nil());
            f12(v7)
        }
    }
}

pub fn f7(v0: Adt4) -> Adt23 {
    let m31 = v0;
    match m31 {
        Adt4::C0 => {
            f49(Str::lit("type"), Adt23::C5)
        }
        Adt4::C1 => {
            f49(Str::lit("nat"), Adt23::C5)
        }
        Adt4::C2 => {
            f49(Str::lit("bool"), Adt23::C5)
        }
        Adt4::C3 => {
            f49(Str::lit("prop"), Adt23::C5)
        }
        Adt4::C4 => {
            f49(Str::lit("unit"), Adt23::C5)
        }
        Adt4::C5 => {
            f49(Str::lit("int"), Adt23::C5)
        }
        Adt4::C6 => {
            f49(Str::lit("int8"), Adt23::C5)
        }
        Adt4::C7 => {
            f49(Str::lit("int16"), Adt23::C5)
        }
        Adt4::C8 => {
            f49(Str::lit("int32"), Adt23::C5)
        }
        Adt4::C9 => {
            f49(Str::lit("int64"), Adt23::C5)
        }
        Adt4::C10 => {
            f49(Str::lit("uint8"), Adt23::C5)
        }
        Adt4::C11 => {
            f49(Str::lit("uint16"), Adt23::C5)
        }
        Adt4::C12 => {
            f49(Str::lit("uint32"), Adt23::C5)
        }
        Adt4::C13 => {
            f49(Str::lit("uint64"), Adt23::C5)
        }
        Adt4::C14 => {
            f49(Str::lit("string"), Adt23::C5)
        }
        Adt4::C15 => {
            f49(Str::lit("bytes"), Adt23::C5)
        }
        Adt4::C16 => {
            f49(Str::lit("ordering"), Adt23::C5)
        }
        Adt4::C17 => {
            f49(Str::lit("contract_violation"), Adt23::C5)
        }
        Adt4::C18 => {
            f49(Str::lit("reasoning_failure"), Adt23::C5)
        }
        Adt4::C19(v1) => {
            f49(Str::lit("parameter"), f50(Str::lit("name"), Adt23::C2(v1), Adt23::C5))
        }
        Adt4::C20(r33) => {
            let v2 = (*r33).clone();
            f49(Str::lit("option"), f50(Str::lit("value"), f7(v2), Adt23::C5))
        }
        Adt4::C21(r34, r35) => {
            let v3 = (*r34).clone();
            let v4 = (*r35).clone();
            f49(Str::lit("result"), f50(Str::lit("ok"), f7(v3), f50(Str::lit("error"), f7(v4), Adt23::C5)))
        }
        Adt4::C22(r36) => {
            let v5 = (*r36).clone();
            f49(Str::lit("list"), f50(Str::lit("element"), f7(v5), Adt23::C5))
        }
        Adt4::C23(v6, v7, r37) => {
            let v8 = (*r37).clone();
            f49(Str::lit("named"), f50(Str::lit("member"), Adt23::C4(std::rc::Rc::new({
                let m32 = v6;
                match m32 {
                    None => {
                        f50(Str::lit("name"), Adt23::C2(v7), Adt23::C5)
                    }
                    Some(v9) => {
                        f50(Str::lit("module"), Adt23::C2(v9), f50(Str::lit("name"), Adt23::C2(v7), Adt23::C5))
                    }
                }
            })), f50(Str::lit("arguments"), Adt23::C3(std::rc::Rc::new(f7(v8))), Adt23::C5)))
        }
        Adt4::C24(r38, r39) => {
            let v10 = (*r38).clone();
            let v11 = (*r39).clone();
            f49(Str::lit("product"), f50(Str::lit("left"), f7(v10), f50(Str::lit("right"), f7(v11), Adt23::C5)))
        }
        Adt4::C25(r40, r41) => {
            let v12 = (*r40).clone();
            let v13 = (*r41).clone();
            f49(Str::lit("function"), f50(Str::lit("parameters"), Adt23::C3(std::rc::Rc::new(f7(v12))), f50(Str::lit("result"), f7(v13), Adt23::C5)))
        }
        Adt4::C26(r42, r43) => {
            let v14 = (*r42).clone();
            let v15 = (*r43).clone();
            f49(Str::lit("map"), f50(Str::lit("key"), f7(v14), f50(Str::lit("value"), f7(v15), Adt23::C5)))
        }
        Adt4::C27(r44) => {
            let v16 = (*r44).clone();
            f49(Str::lit("set"), f50(Str::lit("element"), f7(v16), Adt23::C5))
        }
        Adt4::C28 => {
            Adt23::C5
        }
        Adt4::C29(r45, r46) => {
            let v17 = (*r45).clone();
            let v18 = (*r46).clone();
            Adt23::C6(std::rc::Rc::new(f7(v17)), std::rc::Rc::new(f7(v18)))
        }
    }
}

pub fn f8(v0: List<Adt23>) -> Adt23 {
    f51(Fn6::F52, Adt23::C5, f53(v0))
}

pub fn f9(v0: Fn0, p50: List<Adt23>, p51: List<Bytes>) -> List<Adt23> {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v1, v2) = m49;
                let m47 = v2;
                match m47.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Bytes>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Bytes>), List<Adt23>>(({
                            let c48: Fn0 = v0.clone();
                            c48.apply(v1, v3)
                        }, v4))
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

pub fn f10(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f11(v0: List<Bytes>) -> List<Bytes> {
    f54(Fn7::F55, List::<Bytes>::nil(), v0)
}

pub fn f12(v0: List<(Bytes, Adt23)>) -> Adt23 {
    Adt23::C4(std::rc::Rc::new(f56(Fn8::F57, Adt23::C5, v0)))
}

pub fn f13(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f14(v0: Adt5) -> Adt23 {
    let m52 = v0;
    match m52 {
        Adt5::C0(v1, v2) => {
            let v3: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v1)), List::<(Bytes, Adt23)>::nil());
            let v4: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("type")), f7(v2)), v3);
            f12(v4)
        }
    }
}

pub fn f15(v0: Adt6) -> Adt23 {
    let m53 = v0;
    match m53 {
        Adt6::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v1)), List::<(Bytes, Adt23)>::nil());
            let v5: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("next")), Adt23::C2(v2)), v4);
            let v6: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("type")), f7(v3)), v5);
            f12(v6)
        }
    }
}

pub fn f16(v0: Option<Bytes>, v1: Bytes) -> Adt23 {
    Adt23::C4(std::rc::Rc::new({
        let m54 = v0;
        match m54 {
            None => {
                f58(Str::lit("name"), Adt23::C2(v1), Adt23::C5)
            }
            Some(v2) => {
                f58(Str::lit("module"), Adt23::C2(v2), f58(Str::lit("name"), Adt23::C2(v1), Adt23::C5))
            }
        }
    }))
}

pub fn f17(v0: Fn1, p58: List<Adt23>, p59: List<Adt7>) -> List<Adt23> {
    let m57 = (p58, p59);
    {
        let mut m57 = m57;
        loop {
            let step = {
                let (v1, v2) = m57;
                let m55 = v2;
                match m55.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt7>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt7>), List<Adt23>>(({
                            let c56: Fn1 = v0.clone();
                            c56.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m57 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<Adt23>, v1: Adt7) -> List<Adt23> {
    List::cons(f59(v1), v0)
}

pub fn f19(v0: List<Adt7>) -> List<Adt7> {
    f60(Fn9::F61, List::<Adt7>::nil(), v0)
}

pub fn f20(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f21(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f22(v0: Adt9) -> Adt23 {
    let m60 = v0;
    match m60 {
        Adt9::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v1)), List::<(Bytes, Adt23)>::nil());
            let v5: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("type")), f7(v2)), v4);
            let v6: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("initial")), f62(v3)), v5);
            f12(v6)
        }
    }
}

pub fn f23(v0: Adt13) -> Adt23 {
    let m61 = v0;
    match m61 {
        Adt13::C0(v1) => {
            let v2: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("deterministic")))), List::<(Bytes, Adt23)>::nil());
            let v3: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("body")), f62(v1)), v2);
            f12(v3)
        }
        Adt13::C1(v4, v5) => {
            let v6: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("rule")))), List::<(Bytes, Adt23)>::nil());
            let v7: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("rules")), Adt23::C3(std::rc::Rc::new(f8(f63(Fn10::F64, List::<Adt23>::nil(), f65(v4)))))), v6);
            let v8: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("default")), f62(v5)), v7);
            f12(v8)
        }
        Adt13::C2(v9, v10, v11, v12, v13, v14, v15) => {
            let v16: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("statistical")))), List::<(Bytes, Adt23)>::nil());
            let v17: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("scheme")), f66(v9)), v16);
            let v18: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("features")), f62(v10)), v17);
            let v19: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("width")), Adt23::C1(false, v11)), v18);
            let v20: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("width_evidence")), f16({
                let (h62, _) = v12;
                h62
            }, {
                let (_, h63) = v12;
                h63
            })), v19);
            let v21: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("weights")), f16({
                let (h64, _) = v13;
                h64
            }, {
                let (_, h65) = v13;
                h65
            })), v20);
            let v22: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("bias")), f16({
                let (h66, _) = v14;
                h66
            }, {
                let (_, h67) = v14;
                h67
            })), v21);
            let v23: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("labels")), Adt23::C3(std::rc::Rc::new(f8(f67(Fn11::F68, List::<Adt23>::nil(), f69(v15)))))), v22);
            f12(v23)
        }
        Adt13::C3(v24, v25, v26, v27, v28, v29) => {
            let v30: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("neural")))), List::<(Bytes, Adt23)>::nil());
            let v31: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("architecture")), f70(v24)), v30);
            let v32: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("encoder")), f62(v25)), v31);
            let v33: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("width")), Adt23::C1(false, v26)), v32);
            let v34: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("width_evidence")), f16({
                let (h68, _) = v27;
                h68
            }, {
                let (_, h69) = v27;
                h69
            })), v33);
            let v35: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("layers")), Adt23::C3(std::rc::Rc::new(f8(f71(Fn12::F72, List::<Adt23>::nil(), f73(v28)))))), v34);
            let v36: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("decoder")), f74(v29)), v35);
            f12(v36)
        }
        Adt13::C4(v37) => {
            let v38: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("composite")))), List::<(Bytes, Adt23)>::nil());
            let v39: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("form")), f75(v37)), v38);
            f12(v39)
        }
    }
}

pub fn f24(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f25(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f26(v0: Adt20) -> Adt23 {
    let m70 = v0;
    match m70 {
        Adt20::C0(v1, v2) => {
            let v3: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("member")), f16({
                let (h71, _) = v1;
                h71
            }, {
                let (_, h72) = v1;
                h72
            })), List::<(Bytes, Adt23)>::nil());
            let v6: List<(Bytes, Adt23)> = {
                let m73 = v2.clone();
                match m73.uncons() {
                    None => {
                        v3
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_arguments")), Adt23::C3(std::rc::Rc::new(f8(f76(Fn13::F77, List::<Adt23>::nil(), f78(v2)))))), v3)
                    }
                }
            };
            f12(v6)
        }
    }
}

pub fn f27(v0: Fn2, p77: List<Adt23>, p78: List<Adt22>) -> List<Adt23> {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, v2) = m76;
                let m74 = v2;
                match m74.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt22>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt22>), List<Adt23>>(({
                            let c75: Fn2 = v0.clone();
                            c75.apply(v1, v3)
                        }, v4))
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

pub fn f28(v0: List<Adt23>, v1: Adt22) -> List<Adt23> {
    List::cons(f79(v1), v0)
}

pub fn f29(v0: List<Adt22>) -> List<Adt22> {
    f80(Fn14::F81, List::<Adt22>::nil(), v0)
}

pub fn f30(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f31(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f32(v0: Fn3, p82: List<Adt23>, p83: List<Adt20>) -> List<Adt23> {
    let m81 = (p82, p83);
    {
        let mut m81 = m81;
        loop {
            let step = {
                let (v1, v2) = m81;
                let m79 = v2;
                match m79.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt20>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt20>), List<Adt23>>(({
                            let c80: Fn3 = v0.clone();
                            c80.apply(v1, v3)
                        }, v4))
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

pub fn f33(v0: List<Adt23>, v1: Adt20) -> List<Adt23> {
    List::cons(f26(v1), v0)
}

pub fn f34(v0: List<Adt20>) -> List<Adt20> {
    f82(Fn15::F83, List::<Adt20>::nil(), v0)
}

pub fn f35(v0: Fn4, p87: List<Adt23>, p88: List<(Option<Bytes>, Bytes)>) -> List<Adt23> {
    let m86 = (p87, p88);
    {
        let mut m86 = m86;
        loop {
            let step = {
                let (v1, v2) = m86;
                let m84 = v2;
                match m84.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<(Option<Bytes>, Bytes)>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<(Option<Bytes>, Bytes)>), List<Adt23>>(({
                            let c85: Fn4 = v0.clone();
                            c85.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m86 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: List<Adt23>, v1: (Option<Bytes>, Bytes)) -> List<Adt23> {
    List::cons(f16({
        let (h89, _) = v1;
        h89
    }, {
        let (_, h90) = v1;
        h90
    }), v0)
}

pub fn f37(v0: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    f84(Fn16::F85, List::<(Option<Bytes>, Bytes)>::nil(), v0)
}

pub fn f38(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f39(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a91 = 0u64;
        let a92 = v1;
        nat_lt(a91, a92)
    } else {
        false
    } {
        let a93 = Bytes::lit(&[0x2d]);
        let a94 = f86(v1)?;
        Ok(append_bytes(a93, a94))
    } else {
        f86(v1)
    }
}

pub fn f40(v0: Bytes) -> R<Bytes> {
    let a100 = {
        let a98 = Bytes::lit(&[0x22]);
        let a99 = {
            let (_, h95) = {
                let (h96, _) = f87(Fn17::F88(v0.clone()), {
                    let a97 = v0;
                    length_bytes(a97)
                }, (0u64, Bytes::lit(&[])))?;
                h96
            };
            h95
        };
        append_bytes(a98, a99)
    };
    let a101 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a100, a101))
}

pub fn f41(v0: List<Bytes>) -> Bytes {
    let a104 = {
        let a102 = Bytes::lit(&[0x5b]);
        let a103 = f89(v0, Bytes::lit(&[0x2c]));
        append_bytes(a102, a103)
    };
    let a105 = Bytes::lit(&[0x5d]);
    append_bytes(a104, a105)
}

pub fn f42(v0: Adt23) -> R<List<Bytes>> {
    let m106 = v0;
    match m106 {
        Adt23::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt23::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt23::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt23::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt23::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt23::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt23::C6(r107, r108) => {
            let v7 = (*r107).clone();
            let v8 = (*r108).clone();
            Ok(List::cons(f3(v7)?, f42(v8)?))
        }
        Adt23::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f43(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f90(Fn18::F91, List::<(Bytes, Bytes)>::nil(), f92(v0)?);
    let v2: List<Bytes> = f93(Fn19::F94, List::<Bytes>::nil(), v1)?;
    let a111 = {
        let a109 = Bytes::lit(&[0x7b]);
        let a110 = f89(v2, Bytes::lit(&[0x2c]));
        append_bytes(a109, a110)
    };
    let a112 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a111, a112))
}

pub fn f44(v0: Adt23) -> R<List<(Bytes, Bytes)>> {
    let m113 = v0;
    match m113 {
        Adt23::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt23::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt23::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt23::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt23::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt23::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt23::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt23::C7(v9, r114, r115) => {
            let v10 = (*r114).clone();
            let v11 = (*r115).clone();
            Ok(List::cons((v9, f3(v10)?), f44(v11)?))
        }
    }
}

pub fn f45(v0: Adt3) -> Adt23 {
    let m116 = v0;
    match m116 {
        Adt3::C0 => {
            Adt23::C2(f4(Str::lit("int8")))
        }
        Adt3::C1 => {
            Adt23::C2(f4(Str::lit("int16")))
        }
        Adt3::C2 => {
            Adt23::C2(f4(Str::lit("int32")))
        }
        Adt3::C3 => {
            Adt23::C2(f4(Str::lit("int64")))
        }
        Adt3::C4 => {
            Adt23::C2(f4(Str::lit("uint8")))
        }
        Adt3::C5 => {
            Adt23::C2(f4(Str::lit("uint16")))
        }
        Adt3::C6 => {
            Adt23::C2(f4(Str::lit("uint32")))
        }
        Adt3::C7 => {
            Adt23::C2(f4(Str::lit("uint64")))
        }
    }
}

pub fn f46(v0: Fn5, p120: List<Adt23>, p121: List<u64>) -> List<Adt23> {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v1, v2) = m119;
                let m117 = v2;
                match m117.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<u64>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<u64>), List<Adt23>>(({
                            let c118: Fn5 = v0.clone();
                            c118.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: List<Adt23>, v1: u64) -> List<Adt23> {
    List::cons(Adt23::C1(false, v1), v0)
}

pub fn f48(v0: List<u64>) -> List<u64> {
    f95(Fn20::F96, List::<u64>::nil(), v0)
}

pub fn f49(v0: Str, v1: Adt23) -> Adt23 {
    Adt23::C4(std::rc::Rc::new(f50(Str::lit("kind"), Adt23::C2(f97(v0)), v1)))
}

pub fn f50(v0: Str, v1: Adt23, v2: Adt23) -> Adt23 {
    Adt23::C7(f97(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f51(v0: Fn6, p125: Adt23, p126: List<Adt23>) -> Adt23 {
    let m124 = (p125, p126);
    {
        let mut m124 = m124;
        loop {
            let step = {
                let (v1, v2) = m124;
                let m122 = v2;
                match m122.uncons() {
                    None => {
                        Err::<(Adt23, List<Adt23>), Adt23>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt23, List<Adt23>), Adt23>(({
                            let c123: Fn6 = v0.clone();
                            c123.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m124 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Adt23, v1: Adt23) -> Adt23 {
    Adt23::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f53(v0: List<Adt23>) -> List<Adt23> {
    f98(Fn21::F99, List::<Adt23>::nil(), v0)
}

pub fn f54(v0: Fn7, p130: List<Bytes>, p131: List<Bytes>) -> List<Bytes> {
    let m129 = (p130, p131);
    {
        let mut m129 = m129;
        loop {
            let step = {
                let (v1, v2) = m129;
                let m127 = v2;
                match m127.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c128: Fn7 = v0.clone();
                            c128.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f56(v0: Fn8, p135: Adt23, p136: List<(Bytes, Adt23)>) -> Adt23 {
    let m134 = (p135, p136);
    {
        let mut m134 = m134;
        loop {
            let step = {
                let (v1, v2) = m134;
                let m132 = v2;
                match m132.uncons() {
                    None => {
                        Err::<(Adt23, List<(Bytes, Adt23)>), Adt23>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt23, List<(Bytes, Adt23)>), Adt23>(({
                            let c133: Fn8 = v0.clone();
                            c133.apply(v1, v3)
                        }, v4))
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

pub fn f57(v0: Adt23, v1: (Bytes, Adt23)) -> Adt23 {
    Adt23::C7({
        let (h137, _) = v1;
        h137
    }, std::rc::Rc::new({
        let (_, h138) = v1;
        h138
    }), std::rc::Rc::new(v0))
}

pub fn f58(v0: Str, v1: Adt23, v2: Adt23) -> Adt23 {
    Adt23::C7(f100(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f59(v0: Adt7) -> Adt23 {
    let m139 = v0;
    match m139 {
        Adt7::C0(v1, v2, v3, v4) => {
            let v5: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("predicate")), f101(v1)), List::<(Bytes, Adt23)>::nil());
            let v6: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("validator")), f16({
                let (h140, _) = v2;
                h140
            }, {
                let (_, h141) = v2;
                h141
            })), v5);
            let v7: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("sound")), f16({
                let (h142, _) = v3;
                h142
            }, {
                let (_, h143) = v3;
                h143
            })), v6);
            let v9: List<(Bytes, Adt23)> = {
                let m144 = v4;
                match m144 {
                    None => {
                        v7
                    }
                    Some(v8) => {
                        List::cons((f4(Str::lit("complete")), f16({
                            let (h145, _) = v8;
                            h145
                        }, {
                            let (_, h146) = v8;
                            h146
                        })), v7)
                    }
                }
            };
            f12(v9)
        }
    }
}

pub fn f60(v0: Fn9, p150: List<Adt7>, p151: List<Adt7>) -> List<Adt7> {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v1, v2) = m149;
                let m147 = v2;
                match m147.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt7>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt7>), List<Adt7>>(({
                            let c148: Fn9 = v0.clone();
                            c148.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f62(v0: Adt10) -> Adt23 {
    let m152 = v0;
    match m152 {
        Adt10::C0(v1) => {
            f102(Str::lit("var"), f58(Str::lit("name"), Adt23::C2(v1), Adt23::C5))
        }
        Adt10::C1(v2) => {
            f102(Str::lit("nat"), f58(Str::lit("value"), Adt23::C2(v2), Adt23::C5))
        }
        Adt10::C2(v3, v4) => {
            f102(Str::lit("integer"), f58(Str::lit("representation"), Adt23::C2(f103(v3)), f58(Str::lit("value"), Adt23::C2(v4), Adt23::C5)))
        }
        Adt10::C3(v5) => {
            f102(Str::lit("string"), f58(Str::lit("value"), Adt23::C2(v5), Adt23::C5))
        }
        Adt10::C4(v6) => {
            f102(Str::lit("bytes"), f58(Str::lit("hex"), Adt23::C2(v6), Adt23::C5))
        }
        Adt10::C5(v7, r155, v9) => {
            let v8 = (*r155).clone();
            f102(Str::lit("primitive"), f58(Str::lit("operation"), Adt23::C2({
                let m154 = f104(v7);
                match m154 {
                    Adt24::C0(h153, _, _, _) => {
                        h153
                    }
                }
            }), f58(Str::lit("arguments"), Adt23::C3(std::rc::Rc::new(f62(v8))), f58(Str::lit("result"), f7(v9), Adt23::C5))))
        }
        Adt10::C6(v10) => {
            f102(Str::lit("bool"), f58(Str::lit("value"), Adt23::C0(v10), Adt23::C5))
        }
        Adt10::C7 => {
            f102(Str::lit("unit"), Adt23::C5)
        }
        Adt10::C8(v11) => {
            f102(Str::lit("nil"), f58(Str::lit("element"), f7(v11), Adt23::C5))
        }
        Adt10::C9(r156, r157) => {
            let v12 = (*r156).clone();
            let v13 = (*r157).clone();
            f102(Str::lit("cons"), f58(Str::lit("head"), f62(v12), f58(Str::lit("tail"), f62(v13), Adt23::C5)))
        }
        Adt10::C10(v14, v15, v16, r158) => {
            let v17 = (*r158).clone();
            f102(Str::lit("record"), f58(Str::lit("type"), f16(v14, v15), f105(Str::lit("type_arguments"), v16, f58(Str::lit("fields"), Adt23::C3(std::rc::Rc::new(f62(v17))), Adt23::C5))))
        }
        Adt10::C11(v18, v19, v20, r159) => {
            let v21 = (*r159).clone();
            f102(Str::lit("constructor"), f58(Str::lit("constructor"), f16(v18, v19), f105(Str::lit("type_arguments"), v20, f58(Str::lit("arguments"), Adt23::C3(std::rc::Rc::new(f62(v21))), Adt23::C5))))
        }
        Adt10::C12(v22, v23, v24, v25, v26) => {
            f102(Str::lit("instance_value"), f58(Str::lit("class"), f16(v22, v23), f58(Str::lit("arguments"), Adt23::C3(std::rc::Rc::new(f7(v24))), f58(Str::lit("resolved"), f16(v25, v26), Adt23::C5))))
        }
        Adt10::C13(r160, v28) => {
            let v27 = (*r160).clone();
            f102(Str::lit("project"), f58(Str::lit("value"), f62(v27), f58(Str::lit("field"), Adt23::C2(v28), Adt23::C5)))
        }
        Adt10::C14(v29, v30, v31, r161) => {
            let v32 = (*r161).clone();
            f102(Str::lit("call"), f58(Str::lit("function"), f16(v29, v30), f105(Str::lit("type_arguments"), v31, f58(Str::lit("arguments"), Adt23::C3(std::rc::Rc::new(f62(v32))), Adt23::C5))))
        }
        Adt10::C15(r162, r163, r164) => {
            let v33 = (*r162).clone();
            let v34 = (*r163).clone();
            let v35 = (*r164).clone();
            f102(Str::lit("if"), f58(Str::lit("condition"), f62(v33), f58(Str::lit("then_value"), f62(v34), f58(Str::lit("else_value"), f62(v35), Adt23::C5))))
        }
        Adt10::C16(r165, r166) => {
            let v36 = (*r165).clone();
            let v37 = (*r166).clone();
            f102(Str::lit("match"), f58(Str::lit("scrutinee"), f62(v36), f58(Str::lit("branches"), Adt23::C3(std::rc::Rc::new(f62(v37))), Adt23::C5)))
        }
        Adt10::C17(r167, r168) => {
            let v38 = (*r167).clone();
            let v39 = (*r168).clone();
            f102(Str::lit("eq"), f58(Str::lit("left"), f62(v38), f58(Str::lit("right"), f62(v39), Adt23::C5)))
        }
        Adt10::C18(r169, r170) => {
            let v40 = (*r169).clone();
            let v41 = (*r170).clone();
            f102(Str::lit("le"), f58(Str::lit("left"), f62(v40), f58(Str::lit("right"), f62(v41), Adt23::C5)))
        }
        Adt10::C19(r171, r172) => {
            let v42 = (*r171).clone();
            let v43 = (*r172).clone();
            f102(Str::lit("lt"), f58(Str::lit("left"), f62(v42), f58(Str::lit("right"), f62(v43), Adt23::C5)))
        }
        Adt10::C20(r173, r174) => {
            let v44 = (*r173).clone();
            let v45 = (*r174).clone();
            f102(Str::lit("add"), f58(Str::lit("left"), f62(v44), f58(Str::lit("right"), f62(v45), Adt23::C5)))
        }
        Adt10::C21(r175, r176) => {
            let v46 = (*r175).clone();
            let v47 = (*r176).clone();
            f102(Str::lit("beq"), f58(Str::lit("left"), f62(v46), f58(Str::lit("right"), f62(v47), Adt23::C5)))
        }
        Adt10::C22(r177, r178) => {
            let v48 = (*r177).clone();
            let v49 = (*r178).clone();
            f102(Str::lit("ble"), f58(Str::lit("left"), f62(v48), f58(Str::lit("right"), f62(v49), Adt23::C5)))
        }
        Adt10::C23(r179, r180) => {
            let v50 = (*r179).clone();
            let v51 = (*r180).clone();
            f102(Str::lit("blt"), f58(Str::lit("left"), f62(v50), f58(Str::lit("right"), f62(v51), Adt23::C5)))
        }
        Adt10::C24(r181, r182) => {
            let v52 = (*r181).clone();
            let v53 = (*r182).clone();
            f102(Str::lit("and"), f58(Str::lit("left"), f62(v52), f58(Str::lit("right"), f62(v53), Adt23::C5)))
        }
        Adt10::C25(r183, r184) => {
            let v54 = (*r183).clone();
            let v55 = (*r184).clone();
            f102(Str::lit("prop_and"), f58(Str::lit("left"), f62(v54), f58(Str::lit("right"), f62(v55), Adt23::C5)))
        }
        Adt10::C26(r185, r186) => {
            let v56 = (*r185).clone();
            let v57 = (*r186).clone();
            f102(Str::lit("or"), f58(Str::lit("left"), f62(v56), f58(Str::lit("right"), f62(v57), Adt23::C5)))
        }
        Adt10::C27(r187) => {
            let v58 = (*r187).clone();
            f102(Str::lit("not"), f58(Str::lit("value"), f62(v58), Adt23::C5))
        }
        Adt10::C28(r188, r189) => {
            let v59 = (*r188).clone();
            let v60 = (*r189).clone();
            f102(Str::lit("implies"), f58(Str::lit("premise"), f62(v59), f58(Str::lit("conclusion"), f62(v60), Adt23::C5)))
        }
        Adt10::C29(r190, r191) => {
            let v61 = (*r190).clone();
            let v62 = (*r191).clone();
            f102(Str::lit("iff"), f58(Str::lit("left"), f62(v61), f58(Str::lit("right"), f62(v62), Adt23::C5)))
        }
        Adt10::C30(v63, v64, r192) => {
            let v65 = (*r192).clone();
            f102(Str::lit("forall"), f58(Str::lit("binder"), f106(v63, v64), f58(Str::lit("body"), f62(v65), Adt23::C5)))
        }
        Adt10::C31(v66, v67, r193, r194) => {
            let v68 = (*r193).clone();
            let v69 = (*r194).clone();
            f102(Str::lit("let"), f58(Str::lit("binder"), f106(v66, v67), f58(Str::lit("value"), f62(v68), f58(Str::lit("body"), f62(v69), Adt23::C5))))
        }
        Adt10::C32(r195, r196) => {
            let v70 = (*r195).clone();
            let v71 = (*r196).clone();
            f102(Str::lit("pair"), f58(Str::lit("left"), f62(v70), f58(Str::lit("right"), f62(v71), Adt23::C5)))
        }
        Adt10::C33(r197) => {
            let v72 = (*r197).clone();
            f102(Str::lit("first"), f58(Str::lit("value"), f62(v72), Adt23::C5))
        }
        Adt10::C34(r198) => {
            let v73 = (*r198).clone();
            f102(Str::lit("second"), f58(Str::lit("value"), f62(v73), Adt23::C5))
        }
        Adt10::C35(v74, v75, r199) => {
            let v76 = (*r199).clone();
            f102(Str::lit("lambda"), f58(Str::lit("parameters"), f107(v74), f58(Str::lit("captures"), f108(v75), f58(Str::lit("body"), f62(v76), Adt23::C5))))
        }
        Adt10::C36(r200, r201) => {
            let v77 = (*r200).clone();
            let v78 = (*r201).clone();
            f102(Str::lit("apply"), f58(Str::lit("function"), f62(v77), f58(Str::lit("arguments"), Adt23::C3(std::rc::Rc::new(f62(v78))), Adt23::C5)))
        }
        Adt10::C37(v79, v80, v81) => {
            f102(Str::lit("function_ref"), f58(Str::lit("function"), f16(v79, v80), f105(Str::lit("type_arguments"), v81, Adt23::C5)))
        }
        Adt10::C38(v82, v83, r202) => {
            let v84 = (*r202).clone();
            f102(Str::lit("map_literal"), f58(Str::lit("key"), f7(v82), f58(Str::lit("value"), f7(v83), f58(Str::lit("entries"), Adt23::C3(std::rc::Rc::new(f62(v84))), Adt23::C5))))
        }
        Adt10::C39(v85, r203) => {
            let v86 = (*r203).clone();
            f102(Str::lit("set_literal"), f58(Str::lit("element"), f7(v85), f58(Str::lit("elements"), Adt23::C3(std::rc::Rc::new(f62(v86))), Adt23::C5)))
        }
        Adt10::C40(v87, r204, r205) => {
            let v88 = (*r204).clone();
            let v89 = (*r205).clone();
            f102(Str::lit("graph_literal"), f58(Str::lit("node"), f7(v87), f58(Str::lit("nodes"), Adt23::C3(std::rc::Rc::new(f62(v88))), f58(Str::lit("edges"), Adt23::C3(std::rc::Rc::new(f62(v89))), Adt23::C5))))
        }
        Adt10::C41(v90, v91, v92, r206, v94) => {
            let v93 = (*r206).clone();
            f102(Str::lit("checked_apply"), f58(Str::lit("model"), f16(v90, v91), f105(Str::lit("type_arguments"), v92, f58(Str::lit("arguments"), Adt23::C3(std::rc::Rc::new(f62(v93))), f58(Str::lit("checks"), f108(v94), Adt23::C5)))))
        }
        Adt10::C42 => {
            Adt23::C5
        }
        Adt10::C43(r207, r208) => {
            let v95 = (*r207).clone();
            let v96 = (*r208).clone();
            Adt23::C6(std::rc::Rc::new(f62(v95)), std::rc::Rc::new(f62(v96)))
        }
        Adt10::C44(v97, r209, r210) => {
            let v98 = (*r209).clone();
            let v99 = (*r210).clone();
            Adt23::C6(std::rc::Rc::new(Adt23::C4(std::rc::Rc::new(f58(Str::lit("field"), Adt23::C2(v97), f58(Str::lit("value"), f62(v98), Adt23::C5))))), std::rc::Rc::new(f62(v99)))
        }
        Adt10::C45(v100, v101, v102, r211, r212) => {
            let v103 = (*r211).clone();
            let v104 = (*r212).clone();
            Adt23::C6(std::rc::Rc::new(Adt23::C4(std::rc::Rc::new(f58(Str::lit("constructor"), f16(v100, v101), f58(Str::lit("binders"), f108(v102), f58(Str::lit("body"), f62(v103), Adt23::C5)))))), std::rc::Rc::new(f62(v104)))
        }
        Adt10::C46(r213, r214, r215) => {
            let v105 = (*r213).clone();
            let v106 = (*r214).clone();
            let v107 = (*r215).clone();
            Adt23::C6(std::rc::Rc::new(Adt23::C4(std::rc::Rc::new(f58(Str::lit("key"), f62(v105), f58(Str::lit("value"), f62(v106), Adt23::C5))))), std::rc::Rc::new(f62(v107)))
        }
        Adt10::C47(r216, r217, r218) => {
            let v108 = (*r216).clone();
            let v109 = (*r217).clone();
            let v110 = (*r218).clone();
            Adt23::C6(std::rc::Rc::new(Adt23::C4(std::rc::Rc::new(f58(Str::lit("source"), f62(v108), f58(Str::lit("target"), f62(v109), Adt23::C5))))), std::rc::Rc::new(f62(v110)))
        }
    }
}

pub fn f63(v0: Fn10, p222: List<Adt23>, p223: List<Adt14>) -> List<Adt23> {
    let m221 = (p222, p223);
    {
        let mut m221 = m221;
        loop {
            let step = {
                let (v1, v2) = m221;
                let m219 = v2;
                match m219.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt14>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt14>), List<Adt23>>(({
                            let c220: Fn10 = v0.clone();
                            c220.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<Adt23>, v1: Adt14) -> List<Adt23> {
    List::cons(f109(v1), v0)
}

pub fn f65(v0: List<Adt14>) -> List<Adt14> {
    f110(Fn22::F111, List::<Adt14>::nil(), v0)
}

pub fn f66(v0: Adt15) -> Adt23 {
    let m224 = v0;
    match m224 {
        Adt15::C0 => {
            Adt23::C2(f4(Str::lit("linear_scoring")))
        }
    }
}

pub fn f67(v0: Fn11, p228: List<Adt23>, p229: List<Adt10>) -> List<Adt23> {
    let m227 = (p228, p229);
    {
        let mut m227 = m227;
        loop {
            let step = {
                let (v1, v2) = m227;
                let m225 = v2;
                match m225.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt10>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt10>), List<Adt23>>(({
                            let c226: Fn11 = v0.clone();
                            c226.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt23>, v1: Adt10) -> List<Adt23> {
    List::cons(f62(v1), v0)
}

pub fn f69(v0: List<Adt10>) -> List<Adt10> {
    f112(Fn23::F113, List::<Adt10>::nil(), v0)
}

pub fn f70(v0: Adt16) -> Adt23 {
    let m230 = v0;
    match m230 {
        Adt16::C0 => {
            Adt23::C2(f4(Str::lit("integer_feedforward")))
        }
    }
}

pub fn f71(v0: Fn12, p234: List<Adt23>, p235: List<Adt17>) -> List<Adt23> {
    let m233 = (p234, p235);
    {
        let mut m233 = m233;
        loop {
            let step = {
                let (v1, v2) = m233;
                let m231 = v2;
                match m231.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt17>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt17>), List<Adt23>>(({
                            let c232: Fn12 = v0.clone();
                            c232.apply(v1, v3)
                        }, v4))
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

pub fn f72(v0: List<Adt23>, v1: Adt17) -> List<Adt23> {
    List::cons(f114(v1), v0)
}

pub fn f73(v0: List<Adt17>) -> List<Adt17> {
    f115(Fn24::F116, List::<Adt17>::nil(), v0)
}

pub fn f74(v0: Adt18) -> Adt23 {
    let m236 = v0;
    match m236 {
        Adt18::C0(v1) => {
            let v2: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("argmax")))), List::<(Bytes, Adt23)>::nil());
            let v3: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("labels")), Adt23::C3(std::rc::Rc::new(f8(f67(Fn11::F117, List::<Adt23>::nil(), f69(v1)))))), v2);
            f12(v3)
        }
        Adt18::C1(v4, v5) => {
            let v6: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("function")))), List::<(Bytes, Adt23)>::nil());
            let v7: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("binder")), Adt23::C2(v4)), v6);
            let v8: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("body")), f62(v5)), v7);
            f12(v8)
        }
    }
}

pub fn f75(v0: Adt19) -> Adt23 {
    let m237 = v0;
    match m237 {
        Adt19::C0(v1, v2) => {
            let v3: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("sequence")))), List::<(Bytes, Adt23)>::nil());
            let v4: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("stages")), Adt23::C3(std::rc::Rc::new(f8(f32(Fn3::F118, List::<Adt23>::nil(), f34(v1)))))), v3);
            let v5: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("junctions")), Adt23::C3(std::rc::Rc::new(f8(f119(Fn25::F120, List::<Adt23>::nil(), f121(v2)))))), v4);
            f12(v5)
        }
        Adt19::C1(v6, v7) => {
            let v8: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("fanout")))), List::<(Bytes, Adt23)>::nil());
            let v9: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("left")), f26(v6)), v8);
            let v10: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("right")), f26(v7)), v9);
            f12(v10)
        }
        Adt19::C2(v11, v12) => {
            let v13: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("product")))), List::<(Bytes, Adt23)>::nil());
            let v14: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("left")), f26(v11)), v13);
            let v15: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("right")), f26(v12)), v14);
            f12(v15)
        }
        Adt19::C3(v16, v17, v18) => {
            let v19: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("branch")))), List::<(Bytes, Adt23)>::nil());
            let v20: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("guard")), f62(v16)), v19);
            let v21: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("then")), f26(v17)), v20);
            let v22: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("else")), f26(v18)), v21);
            f12(v22)
        }
        Adt19::C4(v23, v24) => {
            let v25: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("scan")))), List::<(Bytes, Adt23)>::nil());
            let v26: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("stage")), f26(v23)), v25);
            let v27: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("junction")), f122(v24)), v26);
            f12(v27)
        }
    }
}

pub fn f76(v0: Fn13, p241: List<Adt23>, p242: List<Adt4>) -> List<Adt23> {
    let m240 = (p241, p242);
    {
        let mut m240 = m240;
        loop {
            let step = {
                let (v1, v2) = m240;
                let m238 = v2;
                match m238.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt4>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt4>), List<Adt23>>(({
                            let c239: Fn13 = v0.clone();
                            c239.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: List<Adt23>, v1: Adt4) -> List<Adt23> {
    List::cons(f7(v1), v0)
}

pub fn f78(v0: List<Adt4>) -> List<Adt4> {
    f123(Fn26::F124, List::<Adt4>::nil(), v0)
}

pub fn f79(v0: Adt22) -> Adt23 {
    let m243 = v0;
    match m243 {
        Adt22::C0(v1, v2, v3, v4, v5, v6) => {
            let v7: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("dataset_agreement")))), List::<(Bytes, Adt23)>::nil());
            let v8: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("dataset")), f16({
                let (h244, _) = v1;
                h244
            }, {
                let (_, h245) = v1;
                h245
            })), v7);
            let v9: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("comparison")), f16({
                let (h246, _) = v2;
                h246
            }, {
                let (_, h247) = v2;
                h247
            })), v8);
            let v10: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("comparison_sound")), f16({
                let (h248, _) = v3;
                h248
            }, {
                let (_, h249) = v3;
                h249
            })), v9);
            let v11: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("examples")), Adt23::C1(false, v4)), v10);
            let v12: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("agreements")), Adt23::C1(false, v5)), v11);
            let v13: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("theorem")), f16({
                let (h250, _) = v6;
                h250
            }, {
                let (_, h251) = v6;
                h251
            })), v12);
            f12(v13)
        }
        Adt22::C1(v14, v15) => {
            let v16: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("equivalent_to")))), List::<(Bytes, Adt23)>::nil());
            let v17: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("reference")), f26(v14)), v16);
            let v18: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("theorem")), f16({
                let (h252, _) = v15;
                h252
            }, {
                let (_, h253) = v15;
                h253
            })), v17);
            f12(v18)
        }
        Adt22::C2(v19) => {
            let v20: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("initial_invariant")))), List::<(Bytes, Adt23)>::nil());
            let v21: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("theorem")), f16({
                let (h254, _) = v19;
                h254
            }, {
                let (_, h255) = v19;
                h255
            })), v20);
            f12(v21)
        }
        Adt22::C3(v22) => {
            let v23: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("preserves_invariant")))), List::<(Bytes, Adt23)>::nil());
            let v24: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("theorem")), f16({
                let (h256, _) = v22;
                h256
            }, {
                let (_, h257) = v22;
                h257
            })), v23);
            f12(v24)
        }
        Adt22::C4(v25) => {
            let v26: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("satisfies_contract")))), List::<(Bytes, Adt23)>::nil());
            let v27: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("theorem")), f16({
                let (h258, _) = v25;
                h258
            }, {
                let (_, h259) = v25;
                h259
            })), v26);
            f12(v27)
        }
    }
}

pub fn f80(v0: Fn14, p263: List<Adt22>, p264: List<Adt22>) -> List<Adt22> {
    let m262 = (p263, p264);
    {
        let mut m262 = m262;
        loop {
            let step = {
                let (v1, v2) = m262;
                let m260 = v2;
                match m260.uncons() {
                    None => {
                        Err::<(List<Adt22>, List<Adt22>), List<Adt22>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt22>, List<Adt22>), List<Adt22>>(({
                            let c261: Fn14 = v0.clone();
                            c261.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<Adt22>, v1: Adt22) -> List<Adt22> {
    List::cons(v1, v0)
}

pub fn f82(v0: Fn15, p268: List<Adt20>, p269: List<Adt20>) -> List<Adt20> {
    let m267 = (p268, p269);
    {
        let mut m267 = m267;
        loop {
            let step = {
                let (v1, v2) = m267;
                let m265 = v2;
                match m265.uncons() {
                    None => {
                        Err::<(List<Adt20>, List<Adt20>), List<Adt20>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt20>, List<Adt20>), List<Adt20>>(({
                            let c266: Fn15 = v0.clone();
                            c266.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m267 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: List<Adt20>, v1: Adt20) -> List<Adt20> {
    List::cons(v1, v0)
}

pub fn f84(v0: Fn16, p273: List<(Option<Bytes>, Bytes)>, p274: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    let m272 = (p273, p274);
    {
        let mut m272 = m272;
        loop {
            let step = {
                let (v1, v2) = m272;
                let m270 = v2;
                match m270.uncons() {
                    None => {
                        Err::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(({
                            let c271: Fn16 = v0.clone();
                            c271.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<(Option<Bytes>, Bytes)>, v1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
    List::cons(v1, v0)
}

pub fn f86(v0: u64) -> R<Bytes> {
    f125(f126(), 10u64, v0)
}

pub fn f87(v0: Fn17, p279: u64, p280: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m278 = (p279, p280);
    {
        let mut m278 = m278;
        loop {
            let step = {
                let (v1, v2) = m278;
                let m275 = v1;
                if m275 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m275 - 1;
                    let m277 = {
                        let c276: Fn17 = v0.clone();
                        c276.apply(v2.clone())?
                    };
                    match m277 {
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
                Ok(next) => { m278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f127(v0, v1)
}

pub fn f89(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m281 = f128(Fn27::F129(v1), None::<Bytes>, v0);
    match m281 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f90(v0: Fn18, p285: List<(Bytes, Bytes)>, p286: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v1, v2) = m284;
                let m282 = v2;
                match m282.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c283: Fn18 = v0.clone();
                            c283.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f130(v0, v1)
}

pub fn f92(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f131(v0)
}

pub fn f93(v0: Fn19, p290: List<Bytes>, p291: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m289 = (p290, p291);
    {
        let mut m289 = m289;
        loop {
            let step = {
                let (v1, v2) = m289;
                let m287 = v2;
                match m287.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c288: Fn19 = v0.clone();
                            c288.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f132(v1)?, v0))
}

pub fn f95(v0: Fn20, p295: List<u64>, p296: List<u64>) -> List<u64> {
    let m294 = (p295, p296);
    {
        let mut m294 = m294;
        loop {
            let step = {
                let (v1, v2) = m294;
                let m292 = v2;
                match m292.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c293: Fn20 = v0.clone();
                            c293.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f97(v0: Str) -> Bytes {
    let a297 = v0;
    utf8_encode(a297)
}

pub fn f98(v0: Fn21, p301: List<Adt23>, p302: List<Adt23>) -> List<Adt23> {
    let m300 = (p301, p302);
    {
        let mut m300 = m300;
        loop {
            let step = {
                let (v1, v2) = m300;
                let m298 = v2;
                match m298.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt23>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt23>), List<Adt23>>(({
                            let c299: Fn21 = v0.clone();
                            c299.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: List<Adt23>, v1: Adt23) -> List<Adt23> {
    List::cons(v1, v0)
}

pub fn f100(v0: Str) -> Bytes {
    let a303 = v0;
    utf8_encode(a303)
}

pub fn f101(v0: Adt8) -> Adt23 {
    let m304 = v0;
    match m304 {
        Adt8::C0 => {
            Adt23::C2(f4(Str::lit("invariant")))
        }
        Adt8::C1 => {
            Adt23::C2(f4(Str::lit("postcondition")))
        }
        Adt8::C2 => {
            Adt23::C2(f4(Str::lit("precondition")))
        }
    }
}

pub fn f102(v0: Str, v1: Adt23) -> Adt23 {
    Adt23::C4(std::rc::Rc::new(f58(Str::lit("kind"), Adt23::C2(f100(v0)), v1)))
}

pub fn f103(v0: Adt11) -> Bytes {
    let m305 = v0;
    match m305 {
        Adt11::C0 => {
            f97(Str::lit("int"))
        }
        Adt11::C1 => {
            f97(Str::lit("int8"))
        }
        Adt11::C2 => {
            f97(Str::lit("int16"))
        }
        Adt11::C3 => {
            f97(Str::lit("int32"))
        }
        Adt11::C4 => {
            f97(Str::lit("int64"))
        }
        Adt11::C5 => {
            f97(Str::lit("uint8"))
        }
        Adt11::C6 => {
            f97(Str::lit("uint16"))
        }
        Adt11::C7 => {
            f97(Str::lit("uint32"))
        }
        Adt11::C8 => {
            f97(Str::lit("uint64"))
        }
    }
}

pub fn f104(v0: Adt12) -> Adt24 {
    let m306 = v0;
    match m306 {
        Adt12::C0 => {
            Adt24::C0(f97(Str::lit("subtract")), false, false, None::<u64>)
        }
        Adt12::C1 => {
            Adt24::C0(f97(Str::lit("multiply")), false, false, None::<u64>)
        }
        Adt12::C2 => {
            Adt24::C0(f97(Str::lit("quotient")), false, false, None::<u64>)
        }
        Adt12::C3 => {
            Adt24::C0(f97(Str::lit("remainder")), false, false, None::<u64>)
        }
        Adt12::C4 => {
            Adt24::C0(f97(Str::lit("negate")), false, false, None::<u64>)
        }
        Adt12::C5 => {
            Adt24::C0(f97(Str::lit("checked_convert")), false, false, None::<u64>)
        }
        Adt12::C6 => {
            Adt24::C0(f97(Str::lit("checked_add")), false, false, None::<u64>)
        }
        Adt12::C7 => {
            Adt24::C0(f97(Str::lit("checked_subtract")), false, false, None::<u64>)
        }
        Adt12::C8 => {
            Adt24::C0(f97(Str::lit("checked_multiply")), false, false, None::<u64>)
        }
        Adt12::C9 => {
            Adt24::C0(f97(Str::lit("checked_negate")), false, false, None::<u64>)
        }
        Adt12::C10 => {
            Adt24::C0(f97(Str::lit("checked_quotient")), false, false, None::<u64>)
        }
        Adt12::C11 => {
            Adt24::C0(f97(Str::lit("bit_and")), false, false, None::<u64>)
        }
        Adt12::C12 => {
            Adt24::C0(f97(Str::lit("bit_or")), false, false, None::<u64>)
        }
        Adt12::C13 => {
            Adt24::C0(f97(Str::lit("bit_xor")), false, false, None::<u64>)
        }
        Adt12::C14 => {
            Adt24::C0(f97(Str::lit("bit_not")), false, false, None::<u64>)
        }
        Adt12::C15 => {
            Adt24::C0(f97(Str::lit("shift_left")), false, false, None::<u64>)
        }
        Adt12::C16 => {
            Adt24::C0(f97(Str::lit("shift_right")), false, false, None::<u64>)
        }
        Adt12::C17 => {
            Adt24::C0(f97(Str::lit("append")), false, false, None::<u64>)
        }
        Adt12::C18 => {
            Adt24::C0(f97(Str::lit("length")), false, false, None::<u64>)
        }
        Adt12::C19 => {
            Adt24::C0(f97(Str::lit("index")), false, false, None::<u64>)
        }
        Adt12::C20 => {
            Adt24::C0(f97(Str::lit("slice")), false, false, None::<u64>)
        }
        Adt12::C21 => {
            Adt24::C0(f97(Str::lit("utf8_encode")), false, false, None::<u64>)
        }
        Adt12::C22 => {
            Adt24::C0(f97(Str::lit("utf8_decode")), false, false, None::<u64>)
        }
        Adt12::C23 => {
            Adt24::C0(f97(Str::lit("compare_bytes")), false, false, None::<u64>)
        }
        Adt12::C24 => {
            Adt24::C0(f97(Str::lit("equal")), false, false, None::<u64>)
        }
        Adt12::C25 => {
            Adt24::C0(f97(Str::lit("split_exact")), false, false, None::<u64>)
        }
        Adt12::C26 => {
            Adt24::C0(f97(Str::lit("join")), false, false, None::<u64>)
        }
        Adt12::C27 => {
            Adt24::C0(f97(Str::lit("parse_decimal")), false, false, None::<u64>)
        }
        Adt12::C28 => {
            Adt24::C0(f97(Str::lit("format_decimal")), false, false, None::<u64>)
        }
        Adt12::C29 => {
            Adt24::C0(f97(Str::lit("map_insert")), true, false, None::<u64>)
        }
        Adt12::C30 => {
            Adt24::C0(f97(Str::lit("map_remove")), true, false, None::<u64>)
        }
        Adt12::C31 => {
            Adt24::C0(f97(Str::lit("map_lookup")), true, false, None::<u64>)
        }
        Adt12::C32 => {
            Adt24::C0(f97(Str::lit("map_contains")), true, false, None::<u64>)
        }
        Adt12::C33 => {
            Adt24::C0(f97(Str::lit("map_size")), true, false, None::<u64>)
        }
        Adt12::C34 => {
            Adt24::C0(f97(Str::lit("map_keys")), true, false, None::<u64>)
        }
        Adt12::C35 => {
            Adt24::C0(f97(Str::lit("map_values")), true, false, None::<u64>)
        }
        Adt12::C36 => {
            Adt24::C0(f97(Str::lit("map_entries")), true, false, None::<u64>)
        }
        Adt12::C37 => {
            Adt24::C0(f97(Str::lit("map_fold")), true, false, Some(0u64))
        }
        Adt12::C38 => {
            Adt24::C0(f97(Str::lit("set_insert")), true, false, None::<u64>)
        }
        Adt12::C39 => {
            Adt24::C0(f97(Str::lit("set_remove")), true, false, None::<u64>)
        }
        Adt12::C40 => {
            Adt24::C0(f97(Str::lit("set_contains")), true, false, None::<u64>)
        }
        Adt12::C41 => {
            Adt24::C0(f97(Str::lit("set_size")), true, false, None::<u64>)
        }
        Adt12::C42 => {
            Adt24::C0(f97(Str::lit("set_elements")), true, false, None::<u64>)
        }
        Adt12::C43 => {
            Adt24::C0(f97(Str::lit("set_union")), true, false, None::<u64>)
        }
        Adt12::C44 => {
            Adt24::C0(f97(Str::lit("set_intersection")), true, false, None::<u64>)
        }
        Adt12::C45 => {
            Adt24::C0(f97(Str::lit("set_difference")), true, false, None::<u64>)
        }
        Adt12::C46 => {
            Adt24::C0(f97(Str::lit("set_fold")), true, false, Some(0u64))
        }
        Adt12::C47 => {
            Adt24::C0(f97(Str::lit("list_fold")), true, false, Some(0u64))
        }
        Adt12::C48 => {
            Adt24::C0(f97(Str::lit("iterate")), true, false, Some(0u64))
        }
        Adt12::C49 => {
            Adt24::C0(f97(Str::lit("iterate_until")), true, false, Some(0u64))
        }
        Adt12::C50 => {
            Adt24::C0(f97(Str::lit("graph_successors")), true, false, None::<u64>)
        }
        Adt12::C51 => {
            Adt24::C0(f97(Str::lit("graph_reachable")), true, false, None::<u64>)
        }
        Adt12::C52 => {
            Adt24::C0(f97(Str::lit("graph_topological")), true, false, None::<u64>)
        }
        Adt12::C53 => {
            Adt24::C0(f97(Str::lit("less_than")), true, false, None::<u64>)
        }
        Adt12::C54 => {
            Adt24::C0(f97(Str::lit("to_int")), true, true, None::<u64>)
        }
        Adt12::C55 => {
            Adt24::C0(f97(Str::lit("to_nat")), true, true, None::<u64>)
        }
    }
}

pub fn f105(v0: Str, v1: Adt4, v2: Adt23) -> Adt23 {
    let m307 = v1.clone();
    match m307 {
        Adt4::C0 => {
            v2
        }
        Adt4::C1 => {
            v2
        }
        Adt4::C2 => {
            v2
        }
        Adt4::C3 => {
            v2
        }
        Adt4::C4 => {
            v2
        }
        Adt4::C5 => {
            v2
        }
        Adt4::C6 => {
            v2
        }
        Adt4::C7 => {
            v2
        }
        Adt4::C8 => {
            v2
        }
        Adt4::C9 => {
            v2
        }
        Adt4::C10 => {
            v2
        }
        Adt4::C11 => {
            v2
        }
        Adt4::C12 => {
            v2
        }
        Adt4::C13 => {
            v2
        }
        Adt4::C14 => {
            v2
        }
        Adt4::C15 => {
            v2
        }
        Adt4::C16 => {
            v2
        }
        Adt4::C17 => {
            v2
        }
        Adt4::C18 => {
            v2
        }
        Adt4::C19(_) => {
            v2
        }
        Adt4::C20(_) => {
            v2
        }
        Adt4::C21(_, _) => {
            v2
        }
        Adt4::C22(_) => {
            v2
        }
        Adt4::C23(_, _, _) => {
            v2
        }
        Adt4::C24(_, _) => {
            v2
        }
        Adt4::C25(_, _) => {
            v2
        }
        Adt4::C26(_, _) => {
            v2
        }
        Adt4::C27(_) => {
            v2
        }
        Adt4::C28 => {
            v2
        }
        Adt4::C29(_, _) => {
            f58(v0, Adt23::C3(std::rc::Rc::new(f7(v1))), v2)
        }
    }
}

pub fn f106(v0: Bytes, v1: Adt4) -> Adt23 {
    Adt23::C4(std::rc::Rc::new(f58(Str::lit("name"), Adt23::C2(v0), f58(Str::lit("type"), f7(v1), Adt23::C5))))
}

pub fn f107(v0: List<(Bytes, Adt4)>) -> Adt23 {
    Adt23::C3(std::rc::Rc::new(f8(f133(Fn28::F134, List::<Adt23>::nil(), f135(v0)))))
}

pub fn f108(v0: List<Bytes>) -> Adt23 {
    Adt23::C3(std::rc::Rc::new(f8(f9(Fn0::F136, List::<Adt23>::nil(), f11(v0)))))
}

pub fn f109(v0: Adt14) -> Adt23 {
    let m308 = v0;
    match m308 {
        Adt14::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("name")), Adt23::C2(v1)), List::<(Bytes, Adt23)>::nil());
            let v5: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("guard")), f62(v2)), v4);
            let v6: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("action")), f62(v3)), v5);
            f12(v6)
        }
    }
}

pub fn f110(v0: Fn22, p312: List<Adt14>, p313: List<Adt14>) -> List<Adt14> {
    let m311 = (p312, p313);
    {
        let mut m311 = m311;
        loop {
            let step = {
                let (v1, v2) = m311;
                let m309 = v2;
                match m309.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>(({
                            let c310: Fn22 = v0.clone();
                            c310.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    List::cons(v1, v0)
}

pub fn f112(v0: Fn23, p317: List<Adt10>, p318: List<Adt10>) -> List<Adt10> {
    let m316 = (p317, p318);
    {
        let mut m316 = m316;
        loop {
            let step = {
                let (v1, v2) = m316;
                let m314 = v2;
                match m314.uncons() {
                    None => {
                        Err::<(List<Adt10>, List<Adt10>), List<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt10>, List<Adt10>), List<Adt10>>(({
                            let c315: Fn23 = v0.clone();
                            c315.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: List<Adt10>, v1: Adt10) -> List<Adt10> {
    List::cons(v1, v0)
}

pub fn f114(v0: Adt17) -> Adt23 {
    let m319 = v0;
    match m319 {
        Adt17::C0(v1, v2, v3, v4) => {
            let v5: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("dense")))), List::<(Bytes, Adt23)>::nil());
            let v6: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("inputs")), Adt23::C1(false, v1)), v5);
            let v7: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("outputs")), Adt23::C1(false, v2)), v6);
            let v8: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("weights")), f16({
                let (h320, _) = v3;
                h320
            }, {
                let (_, h321) = v3;
                h321
            })), v7);
            let v9: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("bias")), f16({
                let (h322, _) = v4;
                h322
            }, {
                let (_, h323) = v4;
                h323
            })), v8);
            f12(v9)
        }
        Adt17::C1 => {
            let v10: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("relu")))), List::<(Bytes, Adt23)>::nil());
            f12(v10)
        }
        Adt17::C2(v11, v12, v13) => {
            let v14: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("requantize")))), List::<(Bytes, Adt23)>::nil());
            let v15: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("shift")), Adt23::C1(false, v11)), v14);
            let v16: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("minimum")), Adt23::C2(v12)), v15);
            let v17: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("maximum")), Adt23::C2(v13)), v16);
            f12(v17)
        }
    }
}

pub fn f115(v0: Fn24, p327: List<Adt17>, p328: List<Adt17>) -> List<Adt17> {
    let m326 = (p327, p328);
    {
        let mut m326 = m326;
        loop {
            let step = {
                let (v1, v2) = m326;
                let m324 = v2;
                match m324.uncons() {
                    None => {
                        Err::<(List<Adt17>, List<Adt17>), List<Adt17>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt17>, List<Adt17>), List<Adt17>>(({
                            let c325: Fn24 = v0.clone();
                            c325.apply(v1, v3)
                        }, v4))
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

pub fn f116(v0: List<Adt17>, v1: Adt17) -> List<Adt17> {
    List::cons(v1, v0)
}

pub fn f117(v0: List<Adt23>, v1: Adt10) -> List<Adt23> {
    List::cons(f62(v1), v0)
}

pub fn f118(v0: List<Adt23>, v1: Adt20) -> List<Adt23> {
    List::cons(f26(v1), v0)
}

pub fn f119(v0: Fn25, p332: List<Adt23>, p333: List<Adt21>) -> List<Adt23> {
    let m331 = (p332, p333);
    {
        let mut m331 = m331;
        loop {
            let step = {
                let (v1, v2) = m331;
                let m329 = v2;
                match m329.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt21>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt21>), List<Adt23>>(({
                            let c330: Fn25 = v0.clone();
                            c330.apply(v1, v3)
                        }, v4))
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

pub fn f120(v0: List<Adt23>, v1: Adt21) -> List<Adt23> {
    List::cons(f122(v1), v0)
}

pub fn f121(v0: List<Adt21>) -> List<Adt21> {
    f137(Fn29::F138, List::<Adt21>::nil(), v0)
}

pub fn f122(v0: Adt21) -> Adt23 {
    let m334 = v0;
    match m334 {
        Adt21::C0 => {
            let v1: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("unconditional")))), List::<(Bytes, Adt23)>::nil());
            f12(v1)
        }
        Adt21::C1(v2) => {
            let v3: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("proved")))), List::<(Bytes, Adt23)>::nil());
            let v4: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("evidence")), f16({
                let (h335, _) = v2;
                h335
            }, {
                let (_, h336) = v2;
                h336
            })), v3);
            f12(v4)
        }
        Adt21::C2 => {
            let v5: List<(Bytes, Adt23)> = List::cons((f4(Str::lit("kind")), Adt23::C2(f4(Str::lit("checked")))), List::<(Bytes, Adt23)>::nil());
            f12(v5)
        }
    }
}

pub fn f123(v0: Fn26, p340: List<Adt4>, p341: List<Adt4>) -> List<Adt4> {
    let m339 = (p340, p341);
    {
        let mut m339 = m339;
        loop {
            let step = {
                let (v1, v2) = m339;
                let m337 = v2;
                match m337.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c338: Fn26 = v0.clone();
                            c338.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f125(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a342 = v2;
    let a343 = 0u64;
    if nat_eq(a342, a343) {
        Ok(f139(v0, 0u64))
    } else {
        let (_, h344) = {
            let (h345, _) = f87(Fn17::F140(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h345
        };
        Ok(h344)
    }
}

pub fn f126() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f127(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m349 = {
        let a347 = v0.clone();
        let a348 = {
            let (h346, _) = v1.clone();
            h346
        };
        index_bytes(a347, a348)
    };
    match m349 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a351 = {
                    let (h350, _) = v1.clone();
                    h350
                };
                let a352 = 1u64;
                nat_add(a351, a352)?
            }, {
                let a355 = {
                    let (_, h353) = v1;
                    h353
                };
                let a356 = f141(v0, {
                    let (h354, _) = v1;
                    h354
                }, v2)?;
                append_bytes(a355, a356)
            })))
        }
    }
}

pub fn f128(v0: Fn27, p360: Option<Bytes>, p361: List<Bytes>) -> Option<Bytes> {
    let m359 = (p360, p361);
    {
        let mut m359 = m359;
        loop {
            let step = {
                let (v1, v2) = m359;
                let m357 = v2;
                match m357.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c358: Fn27 = v0.clone();
                            c358.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m362 = v1;
    match m362 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a365 = {
                    let a363 = v3;
                    let a364 = v0;
                    append_bytes(a363, a364)
                };
                let a366 = v2;
                append_bytes(a365, a366)
            })
        }
    }
}

pub fn f130(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m367 = v0.clone();
    match m367.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a370 = {
                let (h368, _) = v2;
                h368
            };
            let a371 = {
                let (h369, _) = v1.clone();
                h369
            };
            if equal(a370, a371) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f131(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f142(f143(Fn30::F144, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m376 = {
        let (h372, _) = f145(Fn31::F146, {
            let a374 = {
                let a373 = v0;
                length_list(a373)
            };
            let a375 = 1u64;
            nat_add(a374, a375)?
        }, v1)?;
        h372
    };
    match m376.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f132(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a381 = {
        let a378 = f40({
            let (h377, _) = v0;
            h377
        })?;
        let a379 = Bytes::lit(&[0x3a]);
        append_bytes(a378, a379)
    };
    let a382 = {
        let (_, h380) = v0;
        h380
    };
    Ok(append_bytes(a381, a382))
}

pub fn f133(v0: Fn28, p386: List<Adt23>, p387: List<(Bytes, Adt4)>) -> List<Adt23> {
    let m385 = (p386, p387);
    {
        let mut m385 = m385;
        loop {
            let step = {
                let (v1, v2) = m385;
                let m383 = v2;
                match m383.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<(Bytes, Adt4)>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<(Bytes, Adt4)>), List<Adt23>>(({
                            let c384: Fn28 = v0.clone();
                            c384.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: List<Adt23>, v1: (Bytes, Adt4)) -> List<Adt23> {
    List::cons(f106({
        let (h388, _) = v1;
        h388
    }, {
        let (_, h389) = v1;
        h389
    }), v0)
}

pub fn f135(v0: List<(Bytes, Adt4)>) -> List<(Bytes, Adt4)> {
    f147(Fn32::F148, List::<(Bytes, Adt4)>::nil(), v0)
}

pub fn f136(v0: List<Adt23>, v1: Bytes) -> List<Adt23> {
    List::cons(Adt23::C2(v1), v0)
}

pub fn f137(v0: Fn29, p393: List<Adt21>, p394: List<Adt21>) -> List<Adt21> {
    let m392 = (p393, p394);
    {
        let mut m392 = m392;
        loop {
            let step = {
                let (v1, v2) = m392;
                let m390 = v2;
                match m390.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>(({
                            let c391: Fn29 = v0.clone();
                            c391.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: List<Adt21>, v1: Adt21) -> List<Adt21> {
    List::cons(v1, v0)
}

pub fn f139(v0: Bytes, v1: u64) -> Bytes {
    let m398 = {
        let a395 = v0;
        let a396 = v1;
        let a397 = 1u64;
        slice_bytes(a395, a396, a397)
    };
    match m398 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f140(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f149(v1, v0, v2)
}

pub fn f141(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a443 = {
            let a419 = {
                let a407 = {
                    let a401 = {
                        let a399 = v3;
                        let a400 = 128u8;
                        fixed_u8::bit_and(a399, a400)
                    };
                    let a402 = 128u8;
                    if equal(a401, a402) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a408 = {
                    let a405 = {
                        let a403 = v3;
                        let a404 = 64u8;
                        fixed_u8::bit_and(a403, a404)
                    };
                    let a406 = 64u8;
                    if equal(a405, a406) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a407, a408)?
            };
            let a420 = {
                let a417 = {
                    let a411 = {
                        let a409 = v3;
                        let a410 = 32u8;
                        fixed_u8::bit_and(a409, a410)
                    };
                    let a412 = 32u8;
                    if equal(a411, a412) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a418 = {
                    let a415 = {
                        let a413 = v3;
                        let a414 = 16u8;
                        fixed_u8::bit_and(a413, a414)
                    };
                    let a416 = 16u8;
                    if equal(a415, a416) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a417, a418)?
            };
            nat_add(a419, a420)?
        };
        let a444 = {
            let a441 = {
                let a429 = {
                    let a423 = {
                        let a421 = v3;
                        let a422 = 8u8;
                        fixed_u8::bit_and(a421, a422)
                    };
                    let a424 = 8u8;
                    if equal(a423, a424) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a430 = {
                    let a427 = {
                        let a425 = v3;
                        let a426 = 4u8;
                        fixed_u8::bit_and(a425, a426)
                    };
                    let a428 = 4u8;
                    if equal(a427, a428) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a429, a430)?
            };
            let a442 = {
                let a439 = {
                    let a433 = {
                        let a431 = v3;
                        let a432 = 2u8;
                        fixed_u8::bit_and(a431, a432)
                    };
                    let a434 = 2u8;
                    if equal(a433, a434) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a440 = {
                    let a437 = {
                        let a435 = v3;
                        let a436 = 1u8;
                        fixed_u8::bit_and(a435, a436)
                    };
                    let a438 = 1u8;
                    if equal(a437, a438) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a439, a440)?
            };
            nat_add(a441, a442)?
        };
        nat_add(a443, a444)?
    };
    let a445 = v4;
    let a446 = 34u64;
    if nat_eq(a445, a446) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a447 = v4;
        let a448 = 92u64;
        if nat_eq(a447, a448) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a449 = v4;
            let a450 = 8u64;
            if nat_eq(a449, a450) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a451 = v4;
                let a452 = 12u64;
                if nat_eq(a451, a452) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a453 = v4;
                    let a454 = 10u64;
                    if nat_eq(a453, a454) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a455 = v4;
                        let a456 = 13u64;
                        if nat_eq(a455, a456) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a457 = v4;
                            let a458 = 9u64;
                            if nat_eq(a457, a458) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a459 = v4;
                                let a460 = 32u64;
                                if nat_lt(a459, a460) {
                                    let a461 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a462 = f150(v2)?;
                                    Ok(append_bytes(a461, a462))
                                } else {
                                    Ok(f151(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f142(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f152(Fn33::F153, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f143(v0: Fn30, p466: List<List<(Bytes, Bytes)>>, p467: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m465 = (p466, p467);
    {
        let mut m465 = m465;
        loop {
            let step = {
                let (v1, v2) = m465;
                let m463 = v2;
                match m463.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c464: Fn30 = v0.clone();
                            c464.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f145(v0: Fn31, p472: u64, p473: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m471 = (p472, p473);
    {
        let mut m471 = m471;
        loop {
            let step = {
                let (v1, v2) = m471;
                let m468 = v1;
                if m468 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m468 - 1;
                    let m470 = {
                        let c469: Fn31 = v0.clone();
                        c469.apply(v2.clone())?
                    };
                    match m470 {
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
                Ok(next) => { m471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f154(v0)
}

pub fn f147(v0: Fn32, p477: List<(Bytes, Adt4)>, p478: List<(Bytes, Adt4)>) -> List<(Bytes, Adt4)> {
    let m476 = (p477, p478);
    {
        let mut m476 = m476;
        loop {
            let step = {
                let (v1, v2) = m476;
                let m474 = v2;
                match m474.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt4)>, List<(Bytes, Adt4)>), List<(Bytes, Adt4)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt4)>, List<(Bytes, Adt4)>), List<(Bytes, Adt4)>>(({
                            let c475: Fn32 = v0.clone();
                            c475.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m476 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: List<(Bytes, Adt4)>, v1: (Bytes, Adt4)) -> List<(Bytes, Adt4)> {
    List::cons(v1, v0)
}

pub fn f149(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h479, _) = v2;
        h479
    };
    let a480 = v3;
    let a481 = 0u64;
    if nat_eq(a480, a481) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a482 = v3;
            let a483 = v1;
            let a484 = 0u64;
            nat_quot(a482, a483, a484)
        }, {
            let a489 = f139(v0, {
                let a485 = v3;
                let a486 = v1;
                let a487 = 0u64;
                nat_rem(a485, a486, a487)
            });
            let a490 = {
                let (_, h488) = v2;
                h488
            };
            append_bytes(a489, a490)
        }))
    }
}

pub fn f150(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a535 = {
            let a511 = {
                let a499 = {
                    let a493 = {
                        let a491 = v1;
                        let a492 = 128u8;
                        fixed_u8::bit_and(a491, a492)
                    };
                    let a494 = 128u8;
                    if equal(a493, a494) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a500 = {
                    let a497 = {
                        let a495 = v1;
                        let a496 = 64u8;
                        fixed_u8::bit_and(a495, a496)
                    };
                    let a498 = 64u8;
                    if equal(a497, a498) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a499, a500)?
            };
            let a512 = {
                let a509 = {
                    let a503 = {
                        let a501 = v1;
                        let a502 = 32u8;
                        fixed_u8::bit_and(a501, a502)
                    };
                    let a504 = 32u8;
                    if equal(a503, a504) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a510 = {
                    let a507 = {
                        let a505 = v1;
                        let a506 = 16u8;
                        fixed_u8::bit_and(a505, a506)
                    };
                    let a508 = 16u8;
                    if equal(a507, a508) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a509, a510)?
            };
            nat_add(a511, a512)?
        };
        let a536 = {
            let a533 = {
                let a521 = {
                    let a515 = {
                        let a513 = v1;
                        let a514 = 8u8;
                        fixed_u8::bit_and(a513, a514)
                    };
                    let a516 = 8u8;
                    if equal(a515, a516) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a522 = {
                    let a519 = {
                        let a517 = v1;
                        let a518 = 4u8;
                        fixed_u8::bit_and(a517, a518)
                    };
                    let a520 = 4u8;
                    if equal(a519, a520) {
                        4u64
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
                        let a524 = 2u8;
                        fixed_u8::bit_and(a523, a524)
                    };
                    let a526 = 2u8;
                    if equal(a525, a526) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a532 = {
                    let a529 = {
                        let a527 = v1;
                        let a528 = 1u8;
                        fixed_u8::bit_and(a527, a528)
                    };
                    let a530 = 1u8;
                    if equal(a529, a530) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a531, a532)?
            };
            nat_add(a533, a534)?
        };
        nat_add(a535, a536)?
    };
    let a543 = f139(f155(), {
        let a537 = v2;
        let a538 = 16u64;
        let a539 = 0u64;
        nat_quot(a537, a538, a539)
    });
    let a544 = f139(f155(), {
        let a540 = v2;
        let a541 = 16u64;
        let a542 = 0u64;
        nat_rem(a540, a541, a542)
    });
    Ok(append_bytes(a543, a544))
}

pub fn f151(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m548 = {
        let a545 = v0;
        let a546 = v1;
        let a547 = v2;
        slice_bytes(a545, a546, a547)
    };
    match m548 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f152(v0: Fn33, p552: List<List<(Bytes, Bytes)>>, p553: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m551 = (p552, p553);
    {
        let mut m551 = m551;
        loop {
            let step = {
                let (v1, v2) = m551;
                let m549 = v2;
                match m549.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c550: Fn33 = v0.clone();
                            c550.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f154(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a555 = {
        let a554 = v0.clone();
        length_list(a554)
    };
    let a556 = 1u64;
    if nat_le(a555, a556) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f156(v0)?))
    }
}

pub fn f155() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f156(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f142({
        let (_, h557) = {
            let (h558, _) = f157(Fn34::F158, {
                let a560 = {
                    let a559 = v0.clone();
                    length_list(a559)
                };
                let a561 = 1u64;
                nat_add(a560, a561)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h558
        };
        h557
    }))
}

pub fn f157(v0: Fn34, p566: u64, p567: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m565 = (p566, p567);
    {
        let mut m565 = m565;
        loop {
            let step = {
                let (v1, v2) = m565;
                let m562 = v1;
                if m562 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m562 - 1;
                    let m564 = {
                        let c563: Fn34 = v0.clone();
                        c563.apply(v2.clone())?
                    };
                    match m564 {
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
                Ok(next) => { m565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f159(v0)
}

pub fn f159(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h568, _) = v0;
        h568
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h569) = v0;
        h569
    };
    let m570 = v1;
    match m570.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m571 = v4;
            match m571.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f160(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f160(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a576 = {
            let a574 = {
                let a572 = v0.clone();
                length_list(a572)
            };
            let a575 = {
                let a573 = v1.clone();
                length_list(a573)
            };
            nat_add(a574, a575)?
        };
        let a577 = 1u64;
        nat_add(a576, a577)?
    };
    Ok(f161({
        let (_, h578) = {
            let (_, h579) = {
                let (h580, _) = f162(Fn35::F163, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h580
            };
            h579
        };
        h578
    }))
}

pub fn f161(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f90(Fn18::F164, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f162(v0: Fn35, p585: u64, p586: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m584 = (p585, p586);
    {
        let mut m584 = m584;
        loop {
            let step = {
                let (v1, v2) = m584;
                let m581 = v1;
                if m581 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m581 - 1;
                    let m583 = {
                        let c582: Fn35 = v0.clone();
                        c582.apply(v2.clone())
                    };
                    match m583 {
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
                Ok(next) => { m584 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f165(v0)
}

pub fn f164(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f165(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h587, _) = v0.clone();
        h587
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h588, _) = {
            let (_, h589) = v0.clone();
            h589
        };
        h588
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h590) = {
            let (_, h591) = v0;
            h591
        };
        h590
    };
    let m592 = v1.clone();
    match m592.uncons() {
        None => {
            let m593 = v2;
            match m593.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m594 = v2.clone();
            match m594.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f166({
                        let (h595, _) = v8.clone();
                        h595
                    }, {
                        let (h596, _) = v6.clone();
                        h596
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

pub fn f166(v0: Bytes, v1: Bytes) -> bool {
    let a601 = {
        let a597 = v0;
        let a598 = v1;
        compare_bytes(a597, a598)
    };
    let a602 = {
        let a599 = Bytes::lit(&[]);
        let a600 = Bytes::lit(&[0x01]);
        compare_bytes(a599, a600)
    };
    equal(a601, a602)
}
