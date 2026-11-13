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
    C0(Bytes),
    C1(Bytes),
    C2(Adt1, Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Adt2, std::rc::Rc<Adt0>, Adt3),
    C6(bool),
    C7,
    C8(Adt3),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(Option<Bytes>, Bytes, Adt3, std::rc::Rc<Adt0>),
    C11(Option<Bytes>, Bytes, Adt3, std::rc::Rc<Adt0>),
    C12(Option<Bytes>, Bytes, Adt3, Option<Bytes>, Bytes),
    C13(std::rc::Rc<Adt0>, Bytes),
    C14(Option<Bytes>, Bytes, Adt3, std::rc::Rc<Adt0>),
    C15(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C16(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C17(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C18(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C19(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C20(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C21(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C22(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C23(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C24(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C25(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C26(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C27(std::rc::Rc<Adt0>),
    C28(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C29(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C30(Bytes, Adt3, std::rc::Rc<Adt0>),
    C31(Bytes, Adt3, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C32(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C33(std::rc::Rc<Adt0>),
    C34(std::rc::Rc<Adt0>),
    C35(List<(Bytes, Adt3)>, List<Bytes>, std::rc::Rc<Adt0>),
    C36(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C37(Option<Bytes>, Bytes, Adt3),
    C38(Adt3, Adt3, std::rc::Rc<Adt0>),
    C39(Adt3, std::rc::Rc<Adt0>),
    C40(Adt3, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C41(Option<Bytes>, Bytes, Adt3, std::rc::Rc<Adt0>, List<Bytes>),
    C42,
    C43(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C44(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C45(Option<Bytes>, Bytes, List<Bytes>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C46(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C47(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
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
}

#[derive(Clone)]
pub enum Adt2 {
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
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19(Bytes),
    C20(std::rc::Rc<Adt3>),
    C21(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C22(std::rc::Rc<Adt3>),
    C23(Option<Bytes>, Bytes, std::rc::Rc<Adt3>),
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C25(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C26(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C27(std::rc::Rc<Adt3>),
    C28,
    C29(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt4>),
    C4(std::rc::Rc<Adt4>),
    C5,
    C6(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
    C7(Bytes, std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, bool, bool, Option<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F26,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt4>, p1: (Bytes, Adt3)) -> List<Adt4> {
        match self {
            Fn0::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F29,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt4>, p1: Bytes) -> List<Adt4> {
        match self {
            Fn1::F29 => f29(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F33(Bytes),
    F58(u64, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn2::F33(k0) => f33(k0.clone(), p0),
            Fn2::F58(k0, k1) => Ok(f58(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F36,
    F80,
}

impl Fn3 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn3::F36 => f36(p0, p1),
            Fn3::F80 => f80(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F39,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn4::F39 => f39(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F41,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt4, p1: Adt4) -> Adt4 {
        match self {
            Fn5::F41 => f41(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F44,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Adt3)>, p1: (Bytes, Adt3)) -> List<(Bytes, Adt3)> {
        match self {
            Fn6::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F46,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn7::F46 => f46(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F51(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn8::F51(k0) => f51(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F56,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn9::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F62,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn10::F62 => f62(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F64,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn11::F64 => f64(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F69,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn12::F69 => f69(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F74,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn13::F74 => f74(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F79,
}

impl Fn14 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn14::F79 => f79(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0))
}

pub fn f1(v0: Adt4) -> R<Bytes> {
    f3(v0)
}

pub fn f2(v0: Adt0) -> Adt4 {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1) => {
            f4(Str::lit("var"), f5(Str::lit("name"), Adt4::C2(v1), Adt4::C5))
        }
        Adt0::C1(v2) => {
            f4(Str::lit("nat"), f5(Str::lit("value"), Adt4::C2(v2), Adt4::C5))
        }
        Adt0::C2(v3, v4) => {
            f4(Str::lit("integer"), f5(Str::lit("representation"), Adt4::C2(f6(v3)), f5(Str::lit("value"), Adt4::C2(v4), Adt4::C5)))
        }
        Adt0::C3(v5) => {
            f4(Str::lit("string"), f5(Str::lit("value"), Adt4::C2(v5), Adt4::C5))
        }
        Adt0::C4(v6) => {
            f4(Str::lit("bytes"), f5(Str::lit("hex"), Adt4::C2(v6), Adt4::C5))
        }
        Adt0::C5(v7, r4, v9) => {
            let v8 = (*r4).clone();
            f4(Str::lit("primitive"), f5(Str::lit("operation"), Adt4::C2({
                let m3 = f7(v7);
                match m3 {
                    Adt5::C0(h2, _, _, _) => {
                        h2
                    }
                }
            }), f5(Str::lit("arguments"), Adt4::C3(std::rc::Rc::new(f2(v8))), f5(Str::lit("result"), f8(v9), Adt4::C5))))
        }
        Adt0::C6(v10) => {
            f4(Str::lit("bool"), f5(Str::lit("value"), Adt4::C0(v10), Adt4::C5))
        }
        Adt0::C7 => {
            f4(Str::lit("unit"), Adt4::C5)
        }
        Adt0::C8(v11) => {
            f4(Str::lit("nil"), f5(Str::lit("element"), f8(v11), Adt4::C5))
        }
        Adt0::C9(r5, r6) => {
            let v12 = (*r5).clone();
            let v13 = (*r6).clone();
            f4(Str::lit("cons"), f5(Str::lit("head"), f2(v12), f5(Str::lit("tail"), f2(v13), Adt4::C5)))
        }
        Adt0::C10(v14, v15, v16, r7) => {
            let v17 = (*r7).clone();
            f4(Str::lit("record"), f5(Str::lit("type"), f9(v14, v15), f10(Str::lit("type_arguments"), v16, f5(Str::lit("fields"), Adt4::C3(std::rc::Rc::new(f2(v17))), Adt4::C5))))
        }
        Adt0::C11(v18, v19, v20, r8) => {
            let v21 = (*r8).clone();
            f4(Str::lit("constructor"), f5(Str::lit("constructor"), f9(v18, v19), f10(Str::lit("type_arguments"), v20, f5(Str::lit("arguments"), Adt4::C3(std::rc::Rc::new(f2(v21))), Adt4::C5))))
        }
        Adt0::C12(v22, v23, v24, v25, v26) => {
            f4(Str::lit("instance_value"), f5(Str::lit("class"), f9(v22, v23), f5(Str::lit("arguments"), Adt4::C3(std::rc::Rc::new(f8(v24))), f5(Str::lit("resolved"), f9(v25, v26), Adt4::C5))))
        }
        Adt0::C13(r9, v28) => {
            let v27 = (*r9).clone();
            f4(Str::lit("project"), f5(Str::lit("value"), f2(v27), f5(Str::lit("field"), Adt4::C2(v28), Adt4::C5)))
        }
        Adt0::C14(v29, v30, v31, r10) => {
            let v32 = (*r10).clone();
            f4(Str::lit("call"), f5(Str::lit("function"), f9(v29, v30), f10(Str::lit("type_arguments"), v31, f5(Str::lit("arguments"), Adt4::C3(std::rc::Rc::new(f2(v32))), Adt4::C5))))
        }
        Adt0::C15(r11, r12, r13) => {
            let v33 = (*r11).clone();
            let v34 = (*r12).clone();
            let v35 = (*r13).clone();
            f4(Str::lit("if"), f5(Str::lit("condition"), f2(v33), f5(Str::lit("then_value"), f2(v34), f5(Str::lit("else_value"), f2(v35), Adt4::C5))))
        }
        Adt0::C16(r14, r15) => {
            let v36 = (*r14).clone();
            let v37 = (*r15).clone();
            f4(Str::lit("match"), f5(Str::lit("scrutinee"), f2(v36), f5(Str::lit("branches"), Adt4::C3(std::rc::Rc::new(f2(v37))), Adt4::C5)))
        }
        Adt0::C17(r16, r17) => {
            let v38 = (*r16).clone();
            let v39 = (*r17).clone();
            f4(Str::lit("eq"), f5(Str::lit("left"), f2(v38), f5(Str::lit("right"), f2(v39), Adt4::C5)))
        }
        Adt0::C18(r18, r19) => {
            let v40 = (*r18).clone();
            let v41 = (*r19).clone();
            f4(Str::lit("le"), f5(Str::lit("left"), f2(v40), f5(Str::lit("right"), f2(v41), Adt4::C5)))
        }
        Adt0::C19(r20, r21) => {
            let v42 = (*r20).clone();
            let v43 = (*r21).clone();
            f4(Str::lit("lt"), f5(Str::lit("left"), f2(v42), f5(Str::lit("right"), f2(v43), Adt4::C5)))
        }
        Adt0::C20(r22, r23) => {
            let v44 = (*r22).clone();
            let v45 = (*r23).clone();
            f4(Str::lit("add"), f5(Str::lit("left"), f2(v44), f5(Str::lit("right"), f2(v45), Adt4::C5)))
        }
        Adt0::C21(r24, r25) => {
            let v46 = (*r24).clone();
            let v47 = (*r25).clone();
            f4(Str::lit("beq"), f5(Str::lit("left"), f2(v46), f5(Str::lit("right"), f2(v47), Adt4::C5)))
        }
        Adt0::C22(r26, r27) => {
            let v48 = (*r26).clone();
            let v49 = (*r27).clone();
            f4(Str::lit("ble"), f5(Str::lit("left"), f2(v48), f5(Str::lit("right"), f2(v49), Adt4::C5)))
        }
        Adt0::C23(r28, r29) => {
            let v50 = (*r28).clone();
            let v51 = (*r29).clone();
            f4(Str::lit("blt"), f5(Str::lit("left"), f2(v50), f5(Str::lit("right"), f2(v51), Adt4::C5)))
        }
        Adt0::C24(r30, r31) => {
            let v52 = (*r30).clone();
            let v53 = (*r31).clone();
            f4(Str::lit("and"), f5(Str::lit("left"), f2(v52), f5(Str::lit("right"), f2(v53), Adt4::C5)))
        }
        Adt0::C25(r32, r33) => {
            let v54 = (*r32).clone();
            let v55 = (*r33).clone();
            f4(Str::lit("prop_and"), f5(Str::lit("left"), f2(v54), f5(Str::lit("right"), f2(v55), Adt4::C5)))
        }
        Adt0::C26(r34, r35) => {
            let v56 = (*r34).clone();
            let v57 = (*r35).clone();
            f4(Str::lit("or"), f5(Str::lit("left"), f2(v56), f5(Str::lit("right"), f2(v57), Adt4::C5)))
        }
        Adt0::C27(r36) => {
            let v58 = (*r36).clone();
            f4(Str::lit("not"), f5(Str::lit("value"), f2(v58), Adt4::C5))
        }
        Adt0::C28(r37, r38) => {
            let v59 = (*r37).clone();
            let v60 = (*r38).clone();
            f4(Str::lit("implies"), f5(Str::lit("premise"), f2(v59), f5(Str::lit("conclusion"), f2(v60), Adt4::C5)))
        }
        Adt0::C29(r39, r40) => {
            let v61 = (*r39).clone();
            let v62 = (*r40).clone();
            f4(Str::lit("iff"), f5(Str::lit("left"), f2(v61), f5(Str::lit("right"), f2(v62), Adt4::C5)))
        }
        Adt0::C30(v63, v64, r41) => {
            let v65 = (*r41).clone();
            f4(Str::lit("forall"), f5(Str::lit("binder"), f11(v63, v64), f5(Str::lit("body"), f2(v65), Adt4::C5)))
        }
        Adt0::C31(v66, v67, r42, r43) => {
            let v68 = (*r42).clone();
            let v69 = (*r43).clone();
            f4(Str::lit("let"), f5(Str::lit("binder"), f11(v66, v67), f5(Str::lit("value"), f2(v68), f5(Str::lit("body"), f2(v69), Adt4::C5))))
        }
        Adt0::C32(r44, r45) => {
            let v70 = (*r44).clone();
            let v71 = (*r45).clone();
            f4(Str::lit("pair"), f5(Str::lit("left"), f2(v70), f5(Str::lit("right"), f2(v71), Adt4::C5)))
        }
        Adt0::C33(r46) => {
            let v72 = (*r46).clone();
            f4(Str::lit("first"), f5(Str::lit("value"), f2(v72), Adt4::C5))
        }
        Adt0::C34(r47) => {
            let v73 = (*r47).clone();
            f4(Str::lit("second"), f5(Str::lit("value"), f2(v73), Adt4::C5))
        }
        Adt0::C35(v74, v75, r48) => {
            let v76 = (*r48).clone();
            f4(Str::lit("lambda"), f5(Str::lit("parameters"), f12(v74), f5(Str::lit("captures"), f13(v75), f5(Str::lit("body"), f2(v76), Adt4::C5))))
        }
        Adt0::C36(r49, r50) => {
            let v77 = (*r49).clone();
            let v78 = (*r50).clone();
            f4(Str::lit("apply"), f5(Str::lit("function"), f2(v77), f5(Str::lit("arguments"), Adt4::C3(std::rc::Rc::new(f2(v78))), Adt4::C5)))
        }
        Adt0::C37(v79, v80, v81) => {
            f4(Str::lit("function_ref"), f5(Str::lit("function"), f9(v79, v80), f10(Str::lit("type_arguments"), v81, Adt4::C5)))
        }
        Adt0::C38(v82, v83, r51) => {
            let v84 = (*r51).clone();
            f4(Str::lit("map_literal"), f5(Str::lit("key"), f8(v82), f5(Str::lit("value"), f8(v83), f5(Str::lit("entries"), Adt4::C3(std::rc::Rc::new(f2(v84))), Adt4::C5))))
        }
        Adt0::C39(v85, r52) => {
            let v86 = (*r52).clone();
            f4(Str::lit("set_literal"), f5(Str::lit("element"), f8(v85), f5(Str::lit("elements"), Adt4::C3(std::rc::Rc::new(f2(v86))), Adt4::C5)))
        }
        Adt0::C40(v87, r53, r54) => {
            let v88 = (*r53).clone();
            let v89 = (*r54).clone();
            f4(Str::lit("graph_literal"), f5(Str::lit("node"), f8(v87), f5(Str::lit("nodes"), Adt4::C3(std::rc::Rc::new(f2(v88))), f5(Str::lit("edges"), Adt4::C3(std::rc::Rc::new(f2(v89))), Adt4::C5))))
        }
        Adt0::C41(v90, v91, v92, r55, v94) => {
            let v93 = (*r55).clone();
            f4(Str::lit("checked_apply"), f5(Str::lit("model"), f9(v90, v91), f10(Str::lit("type_arguments"), v92, f5(Str::lit("arguments"), Adt4::C3(std::rc::Rc::new(f2(v93))), f5(Str::lit("checks"), f13(v94), Adt4::C5)))))
        }
        Adt0::C42 => {
            Adt4::C5
        }
        Adt0::C43(r56, r57) => {
            let v95 = (*r56).clone();
            let v96 = (*r57).clone();
            Adt4::C6(std::rc::Rc::new(f2(v95)), std::rc::Rc::new(f2(v96)))
        }
        Adt0::C44(v97, r58, r59) => {
            let v98 = (*r58).clone();
            let v99 = (*r59).clone();
            Adt4::C6(std::rc::Rc::new(Adt4::C4(std::rc::Rc::new(f5(Str::lit("field"), Adt4::C2(v97), f5(Str::lit("value"), f2(v98), Adt4::C5))))), std::rc::Rc::new(f2(v99)))
        }
        Adt0::C45(v100, v101, v102, r60, r61) => {
            let v103 = (*r60).clone();
            let v104 = (*r61).clone();
            Adt4::C6(std::rc::Rc::new(Adt4::C4(std::rc::Rc::new(f5(Str::lit("constructor"), f9(v100, v101), f5(Str::lit("binders"), f13(v102), f5(Str::lit("body"), f2(v103), Adt4::C5)))))), std::rc::Rc::new(f2(v104)))
        }
        Adt0::C46(r62, r63, r64) => {
            let v105 = (*r62).clone();
            let v106 = (*r63).clone();
            let v107 = (*r64).clone();
            Adt4::C6(std::rc::Rc::new(Adt4::C4(std::rc::Rc::new(f5(Str::lit("key"), f2(v105), f5(Str::lit("value"), f2(v106), Adt4::C5))))), std::rc::Rc::new(f2(v107)))
        }
        Adt0::C47(r65, r66, r67) => {
            let v108 = (*r65).clone();
            let v109 = (*r66).clone();
            let v110 = (*r67).clone();
            Adt4::C6(std::rc::Rc::new(Adt4::C4(std::rc::Rc::new(f5(Str::lit("source"), f2(v108), f5(Str::lit("target"), f2(v109), Adt4::C5))))), std::rc::Rc::new(f2(v110)))
        }
    }
}

pub fn f3(v0: Adt4) -> R<Bytes> {
    let m68 = v0;
    match m68 {
        Adt4::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt4::C1(v2, v3) => {
            f14(v2, v3)
        }
        Adt4::C2(v4) => {
            f15(v4)
        }
        Adt4::C3(r69) => {
            let v5 = (*r69).clone();
            Ok(f16(f17(v5)?))
        }
        Adt4::C4(r70) => {
            let v6 = (*r70).clone();
            f18(f19(v6)?)
        }
        Adt4::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt4::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt4::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Str, v1: Adt4) -> Adt4 {
    Adt4::C4(std::rc::Rc::new(f5(Str::lit("kind"), Adt4::C2(f20(v0)), v1)))
}

pub fn f5(v0: Str, v1: Adt4, v2: Adt4) -> Adt4 {
    Adt4::C7(f20(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f6(v0: Adt1) -> Bytes {
    let m71 = v0;
    match m71 {
        Adt1::C0 => {
            f21(Str::lit("int"))
        }
        Adt1::C1 => {
            f21(Str::lit("int8"))
        }
        Adt1::C2 => {
            f21(Str::lit("int16"))
        }
        Adt1::C3 => {
            f21(Str::lit("int32"))
        }
        Adt1::C4 => {
            f21(Str::lit("int64"))
        }
        Adt1::C5 => {
            f21(Str::lit("uint8"))
        }
        Adt1::C6 => {
            f21(Str::lit("uint16"))
        }
        Adt1::C7 => {
            f21(Str::lit("uint32"))
        }
        Adt1::C8 => {
            f21(Str::lit("uint64"))
        }
    }
}

pub fn f7(v0: Adt2) -> Adt5 {
    let m72 = v0;
    match m72 {
        Adt2::C0 => {
            Adt5::C0(f21(Str::lit("subtract")), false, false, None::<u64>)
        }
        Adt2::C1 => {
            Adt5::C0(f21(Str::lit("multiply")), false, false, None::<u64>)
        }
        Adt2::C2 => {
            Adt5::C0(f21(Str::lit("quotient")), false, false, None::<u64>)
        }
        Adt2::C3 => {
            Adt5::C0(f21(Str::lit("remainder")), false, false, None::<u64>)
        }
        Adt2::C4 => {
            Adt5::C0(f21(Str::lit("negate")), false, false, None::<u64>)
        }
        Adt2::C5 => {
            Adt5::C0(f21(Str::lit("checked_convert")), false, false, None::<u64>)
        }
        Adt2::C6 => {
            Adt5::C0(f21(Str::lit("checked_add")), false, false, None::<u64>)
        }
        Adt2::C7 => {
            Adt5::C0(f21(Str::lit("checked_subtract")), false, false, None::<u64>)
        }
        Adt2::C8 => {
            Adt5::C0(f21(Str::lit("checked_multiply")), false, false, None::<u64>)
        }
        Adt2::C9 => {
            Adt5::C0(f21(Str::lit("checked_negate")), false, false, None::<u64>)
        }
        Adt2::C10 => {
            Adt5::C0(f21(Str::lit("checked_quotient")), false, false, None::<u64>)
        }
        Adt2::C11 => {
            Adt5::C0(f21(Str::lit("bit_and")), false, false, None::<u64>)
        }
        Adt2::C12 => {
            Adt5::C0(f21(Str::lit("bit_or")), false, false, None::<u64>)
        }
        Adt2::C13 => {
            Adt5::C0(f21(Str::lit("bit_xor")), false, false, None::<u64>)
        }
        Adt2::C14 => {
            Adt5::C0(f21(Str::lit("bit_not")), false, false, None::<u64>)
        }
        Adt2::C15 => {
            Adt5::C0(f21(Str::lit("shift_left")), false, false, None::<u64>)
        }
        Adt2::C16 => {
            Adt5::C0(f21(Str::lit("shift_right")), false, false, None::<u64>)
        }
        Adt2::C17 => {
            Adt5::C0(f21(Str::lit("append")), false, false, None::<u64>)
        }
        Adt2::C18 => {
            Adt5::C0(f21(Str::lit("length")), false, false, None::<u64>)
        }
        Adt2::C19 => {
            Adt5::C0(f21(Str::lit("index")), false, false, None::<u64>)
        }
        Adt2::C20 => {
            Adt5::C0(f21(Str::lit("slice")), false, false, None::<u64>)
        }
        Adt2::C21 => {
            Adt5::C0(f21(Str::lit("utf8_encode")), false, false, None::<u64>)
        }
        Adt2::C22 => {
            Adt5::C0(f21(Str::lit("utf8_decode")), false, false, None::<u64>)
        }
        Adt2::C23 => {
            Adt5::C0(f21(Str::lit("compare_bytes")), false, false, None::<u64>)
        }
        Adt2::C24 => {
            Adt5::C0(f21(Str::lit("equal")), false, false, None::<u64>)
        }
        Adt2::C25 => {
            Adt5::C0(f21(Str::lit("split_exact")), false, false, None::<u64>)
        }
        Adt2::C26 => {
            Adt5::C0(f21(Str::lit("join")), false, false, None::<u64>)
        }
        Adt2::C27 => {
            Adt5::C0(f21(Str::lit("parse_decimal")), false, false, None::<u64>)
        }
        Adt2::C28 => {
            Adt5::C0(f21(Str::lit("format_decimal")), false, false, None::<u64>)
        }
        Adt2::C29 => {
            Adt5::C0(f21(Str::lit("map_insert")), true, false, None::<u64>)
        }
        Adt2::C30 => {
            Adt5::C0(f21(Str::lit("map_remove")), true, false, None::<u64>)
        }
        Adt2::C31 => {
            Adt5::C0(f21(Str::lit("map_lookup")), true, false, None::<u64>)
        }
        Adt2::C32 => {
            Adt5::C0(f21(Str::lit("map_contains")), true, false, None::<u64>)
        }
        Adt2::C33 => {
            Adt5::C0(f21(Str::lit("map_size")), true, false, None::<u64>)
        }
        Adt2::C34 => {
            Adt5::C0(f21(Str::lit("map_keys")), true, false, None::<u64>)
        }
        Adt2::C35 => {
            Adt5::C0(f21(Str::lit("map_values")), true, false, None::<u64>)
        }
        Adt2::C36 => {
            Adt5::C0(f21(Str::lit("map_entries")), true, false, None::<u64>)
        }
        Adt2::C37 => {
            Adt5::C0(f21(Str::lit("map_fold")), true, false, Some(0u64))
        }
        Adt2::C38 => {
            Adt5::C0(f21(Str::lit("set_insert")), true, false, None::<u64>)
        }
        Adt2::C39 => {
            Adt5::C0(f21(Str::lit("set_remove")), true, false, None::<u64>)
        }
        Adt2::C40 => {
            Adt5::C0(f21(Str::lit("set_contains")), true, false, None::<u64>)
        }
        Adt2::C41 => {
            Adt5::C0(f21(Str::lit("set_size")), true, false, None::<u64>)
        }
        Adt2::C42 => {
            Adt5::C0(f21(Str::lit("set_elements")), true, false, None::<u64>)
        }
        Adt2::C43 => {
            Adt5::C0(f21(Str::lit("set_union")), true, false, None::<u64>)
        }
        Adt2::C44 => {
            Adt5::C0(f21(Str::lit("set_intersection")), true, false, None::<u64>)
        }
        Adt2::C45 => {
            Adt5::C0(f21(Str::lit("set_difference")), true, false, None::<u64>)
        }
        Adt2::C46 => {
            Adt5::C0(f21(Str::lit("set_fold")), true, false, Some(0u64))
        }
        Adt2::C47 => {
            Adt5::C0(f21(Str::lit("list_fold")), true, false, Some(0u64))
        }
        Adt2::C48 => {
            Adt5::C0(f21(Str::lit("iterate")), true, false, Some(0u64))
        }
        Adt2::C49 => {
            Adt5::C0(f21(Str::lit("iterate_until")), true, false, Some(0u64))
        }
        Adt2::C50 => {
            Adt5::C0(f21(Str::lit("graph_successors")), true, false, None::<u64>)
        }
        Adt2::C51 => {
            Adt5::C0(f21(Str::lit("graph_reachable")), true, false, None::<u64>)
        }
        Adt2::C52 => {
            Adt5::C0(f21(Str::lit("graph_topological")), true, false, None::<u64>)
        }
        Adt2::C53 => {
            Adt5::C0(f21(Str::lit("less_than")), true, false, None::<u64>)
        }
        Adt2::C54 => {
            Adt5::C0(f21(Str::lit("to_int")), true, true, None::<u64>)
        }
        Adt2::C55 => {
            Adt5::C0(f21(Str::lit("to_nat")), true, true, None::<u64>)
        }
    }
}

pub fn f8(v0: Adt3) -> Adt4 {
    let m73 = v0;
    match m73 {
        Adt3::C0 => {
            f22(Str::lit("type"), Adt4::C5)
        }
        Adt3::C1 => {
            f22(Str::lit("nat"), Adt4::C5)
        }
        Adt3::C2 => {
            f22(Str::lit("bool"), Adt4::C5)
        }
        Adt3::C3 => {
            f22(Str::lit("prop"), Adt4::C5)
        }
        Adt3::C4 => {
            f22(Str::lit("unit"), Adt4::C5)
        }
        Adt3::C5 => {
            f22(Str::lit("int"), Adt4::C5)
        }
        Adt3::C6 => {
            f22(Str::lit("int8"), Adt4::C5)
        }
        Adt3::C7 => {
            f22(Str::lit("int16"), Adt4::C5)
        }
        Adt3::C8 => {
            f22(Str::lit("int32"), Adt4::C5)
        }
        Adt3::C9 => {
            f22(Str::lit("int64"), Adt4::C5)
        }
        Adt3::C10 => {
            f22(Str::lit("uint8"), Adt4::C5)
        }
        Adt3::C11 => {
            f22(Str::lit("uint16"), Adt4::C5)
        }
        Adt3::C12 => {
            f22(Str::lit("uint32"), Adt4::C5)
        }
        Adt3::C13 => {
            f22(Str::lit("uint64"), Adt4::C5)
        }
        Adt3::C14 => {
            f22(Str::lit("string"), Adt4::C5)
        }
        Adt3::C15 => {
            f22(Str::lit("bytes"), Adt4::C5)
        }
        Adt3::C16 => {
            f22(Str::lit("ordering"), Adt4::C5)
        }
        Adt3::C17 => {
            f22(Str::lit("contract_violation"), Adt4::C5)
        }
        Adt3::C18 => {
            f22(Str::lit("reasoning_failure"), Adt4::C5)
        }
        Adt3::C19(v1) => {
            f22(Str::lit("parameter"), f23(Str::lit("name"), Adt4::C2(v1), Adt4::C5))
        }
        Adt3::C20(r75) => {
            let v2 = (*r75).clone();
            f22(Str::lit("option"), f23(Str::lit("value"), f8(v2), Adt4::C5))
        }
        Adt3::C21(r76, r77) => {
            let v3 = (*r76).clone();
            let v4 = (*r77).clone();
            f22(Str::lit("result"), f23(Str::lit("ok"), f8(v3), f23(Str::lit("error"), f8(v4), Adt4::C5)))
        }
        Adt3::C22(r78) => {
            let v5 = (*r78).clone();
            f22(Str::lit("list"), f23(Str::lit("element"), f8(v5), Adt4::C5))
        }
        Adt3::C23(v6, v7, r79) => {
            let v8 = (*r79).clone();
            f22(Str::lit("named"), f23(Str::lit("member"), Adt4::C4(std::rc::Rc::new({
                let m74 = v6;
                match m74 {
                    None => {
                        f23(Str::lit("name"), Adt4::C2(v7), Adt4::C5)
                    }
                    Some(v9) => {
                        f23(Str::lit("module"), Adt4::C2(v9), f23(Str::lit("name"), Adt4::C2(v7), Adt4::C5))
                    }
                }
            })), f23(Str::lit("arguments"), Adt4::C3(std::rc::Rc::new(f8(v8))), Adt4::C5)))
        }
        Adt3::C24(r80, r81) => {
            let v10 = (*r80).clone();
            let v11 = (*r81).clone();
            f22(Str::lit("product"), f23(Str::lit("left"), f8(v10), f23(Str::lit("right"), f8(v11), Adt4::C5)))
        }
        Adt3::C25(r82, r83) => {
            let v12 = (*r82).clone();
            let v13 = (*r83).clone();
            f22(Str::lit("function"), f23(Str::lit("parameters"), Adt4::C3(std::rc::Rc::new(f8(v12))), f23(Str::lit("result"), f8(v13), Adt4::C5)))
        }
        Adt3::C26(r84, r85) => {
            let v14 = (*r84).clone();
            let v15 = (*r85).clone();
            f22(Str::lit("map"), f23(Str::lit("key"), f8(v14), f23(Str::lit("value"), f8(v15), Adt4::C5)))
        }
        Adt3::C27(r86) => {
            let v16 = (*r86).clone();
            f22(Str::lit("set"), f23(Str::lit("element"), f8(v16), Adt4::C5))
        }
        Adt3::C28 => {
            Adt4::C5
        }
        Adt3::C29(r87, r88) => {
            let v17 = (*r87).clone();
            let v18 = (*r88).clone();
            Adt4::C6(std::rc::Rc::new(f8(v17)), std::rc::Rc::new(f8(v18)))
        }
    }
}

pub fn f9(v0: Option<Bytes>, v1: Bytes) -> Adt4 {
    Adt4::C4(std::rc::Rc::new({
        let m89 = v0;
        match m89 {
            None => {
                f5(Str::lit("name"), Adt4::C2(v1), Adt4::C5)
            }
            Some(v2) => {
                f5(Str::lit("module"), Adt4::C2(v2), f5(Str::lit("name"), Adt4::C2(v1), Adt4::C5))
            }
        }
    }))
}

pub fn f10(v0: Str, v1: Adt3, v2: Adt4) -> Adt4 {
    let m90 = v1.clone();
    match m90 {
        Adt3::C0 => {
            v2
        }
        Adt3::C1 => {
            v2
        }
        Adt3::C2 => {
            v2
        }
        Adt3::C3 => {
            v2
        }
        Adt3::C4 => {
            v2
        }
        Adt3::C5 => {
            v2
        }
        Adt3::C6 => {
            v2
        }
        Adt3::C7 => {
            v2
        }
        Adt3::C8 => {
            v2
        }
        Adt3::C9 => {
            v2
        }
        Adt3::C10 => {
            v2
        }
        Adt3::C11 => {
            v2
        }
        Adt3::C12 => {
            v2
        }
        Adt3::C13 => {
            v2
        }
        Adt3::C14 => {
            v2
        }
        Adt3::C15 => {
            v2
        }
        Adt3::C16 => {
            v2
        }
        Adt3::C17 => {
            v2
        }
        Adt3::C18 => {
            v2
        }
        Adt3::C19(_) => {
            v2
        }
        Adt3::C20(_) => {
            v2
        }
        Adt3::C21(_, _) => {
            v2
        }
        Adt3::C22(_) => {
            v2
        }
        Adt3::C23(_, _, _) => {
            v2
        }
        Adt3::C24(_, _) => {
            v2
        }
        Adt3::C25(_, _) => {
            v2
        }
        Adt3::C26(_, _) => {
            v2
        }
        Adt3::C27(_) => {
            v2
        }
        Adt3::C28 => {
            v2
        }
        Adt3::C29(_, _) => {
            f5(v0, Adt4::C3(std::rc::Rc::new(f8(v1))), v2)
        }
    }
}

pub fn f11(v0: Bytes, v1: Adt3) -> Adt4 {
    Adt4::C4(std::rc::Rc::new(f5(Str::lit("name"), Adt4::C2(v0), f5(Str::lit("type"), f8(v1), Adt4::C5))))
}

pub fn f12(v0: List<(Bytes, Adt3)>) -> Adt4 {
    Adt4::C3(std::rc::Rc::new(f24(f25(Fn0::F26, List::<Adt4>::nil(), f27(v0)))))
}

pub fn f13(v0: List<Bytes>) -> Adt4 {
    Adt4::C3(std::rc::Rc::new(f24(f28(Fn1::F29, List::<Adt4>::nil(), f30(v0)))))
}

pub fn f14(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a91 = 0u64;
        let a92 = v1;
        nat_lt(a91, a92)
    } else {
        false
    } {
        let a93 = Bytes::lit(&[0x2d]);
        let a94 = f31(v1)?;
        Ok(append_bytes(a93, a94))
    } else {
        f31(v1)
    }
}

pub fn f15(v0: Bytes) -> R<Bytes> {
    let a100 = {
        let a98 = Bytes::lit(&[0x22]);
        let a99 = {
            let (_, h95) = {
                let (h96, _) = f32(Fn2::F33(v0.clone()), {
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

pub fn f16(v0: List<Bytes>) -> Bytes {
    let a104 = {
        let a102 = Bytes::lit(&[0x5b]);
        let a103 = f34(v0, Bytes::lit(&[0x2c]));
        append_bytes(a102, a103)
    };
    let a105 = Bytes::lit(&[0x5d]);
    append_bytes(a104, a105)
}

pub fn f17(v0: Adt4) -> R<List<Bytes>> {
    let m106 = v0;
    match m106 {
        Adt4::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt4::C6(r107, r108) => {
            let v7 = (*r107).clone();
            let v8 = (*r108).clone();
            Ok(List::cons(f3(v7)?, f17(v8)?))
        }
        Adt4::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f18(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f35(Fn3::F36, List::<(Bytes, Bytes)>::nil(), f37(v0)?);
    let v2: List<Bytes> = f38(Fn4::F39, List::<Bytes>::nil(), v1)?;
    let a111 = {
        let a109 = Bytes::lit(&[0x7b]);
        let a110 = f34(v2, Bytes::lit(&[0x2c]));
        append_bytes(a109, a110)
    };
    let a112 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a111, a112))
}

pub fn f19(v0: Adt4) -> R<List<(Bytes, Bytes)>> {
    let m113 = v0;
    match m113 {
        Adt4::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt4::C7(v9, r114, r115) => {
            let v10 = (*r114).clone();
            let v11 = (*r115).clone();
            Ok(List::cons((v9, f3(v10)?), f19(v11)?))
        }
    }
}

pub fn f20(v0: Str) -> Bytes {
    let a116 = v0;
    utf8_encode(a116)
}

pub fn f21(v0: Str) -> Bytes {
    let a117 = v0;
    utf8_encode(a117)
}

pub fn f22(v0: Str, v1: Adt4) -> Adt4 {
    Adt4::C4(std::rc::Rc::new(f23(Str::lit("kind"), Adt4::C2(f21(v0)), v1)))
}

pub fn f23(v0: Str, v1: Adt4, v2: Adt4) -> Adt4 {
    Adt4::C7(f21(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f24(v0: List<Adt4>) -> Adt4 {
    f40(Fn5::F41, Adt4::C5, f42(v0))
}

pub fn f25(v0: Fn0, p121: List<Adt4>, p122: List<(Bytes, Adt3)>) -> List<Adt4> {
    let m120 = (p121, p122);
    {
        let mut m120 = m120;
        loop {
            let step = {
                let (v1, v2) = m120;
                let m118 = v2;
                match m118.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<(Bytes, Adt3)>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<(Bytes, Adt3)>), List<Adt4>>(({
                            let c119: Fn0 = v0.clone();
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

pub fn f26(v0: List<Adt4>, v1: (Bytes, Adt3)) -> List<Adt4> {
    List::cons(f11({
        let (h123, _) = v1;
        h123
    }, {
        let (_, h124) = v1;
        h124
    }), v0)
}

pub fn f27(v0: List<(Bytes, Adt3)>) -> List<(Bytes, Adt3)> {
    f43(Fn6::F44, List::<(Bytes, Adt3)>::nil(), v0)
}

pub fn f28(v0: Fn1, p128: List<Adt4>, p129: List<Bytes>) -> List<Adt4> {
    let m127 = (p128, p129);
    {
        let mut m127 = m127;
        loop {
            let step = {
                let (v1, v2) = m127;
                let m125 = v2;
                match m125.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Bytes>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Bytes>), List<Adt4>>(({
                            let c126: Fn1 = v0.clone();
                            c126.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: List<Adt4>, v1: Bytes) -> List<Adt4> {
    List::cons(Adt4::C2(v1), v0)
}

pub fn f30(v0: List<Bytes>) -> List<Bytes> {
    f45(Fn7::F46, List::<Bytes>::nil(), v0)
}

pub fn f31(v0: u64) -> R<Bytes> {
    f47(f48(), 10u64, v0)
}

pub fn f32(v0: Fn2, p134: u64, p135: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v1, v2) = m133;
                let m130 = v1;
                if m130 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m130 - 1;
                    let m132 = {
                        let c131: Fn2 = v0.clone();
                        c131.apply(v2.clone())?
                    };
                    match m132 {
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
                Ok(next) => { m133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f49(v0, v1)
}

pub fn f34(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m136 = f50(Fn8::F51(v1), None::<Bytes>, v0);
    match m136 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f35(v0: Fn3, p140: List<(Bytes, Bytes)>, p141: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m137 = v2;
                match m137.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c138: Fn3 = v0.clone();
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

pub fn f36(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f52(v0, v1)
}

pub fn f37(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f53(v0)
}

pub fn f38(v0: Fn4, p145: List<Bytes>, p146: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m144 = (p145, p146);
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v1, v2) = m144;
                let m142 = v2;
                match m142.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c143: Fn4 = v0.clone();
                            c143.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f54(v1)?, v0))
}

pub fn f40(v0: Fn5, p150: Adt4, p151: List<Adt4>) -> Adt4 {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v1, v2) = m149;
                let m147 = v2;
                match m147.uncons() {
                    None => {
                        Err::<(Adt4, List<Adt4>), Adt4>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<Adt4>), Adt4>(({
                            let c148: Fn5 = v0.clone();
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

pub fn f41(v0: Adt4, v1: Adt4) -> Adt4 {
    Adt4::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f42(v0: List<Adt4>) -> List<Adt4> {
    f55(Fn9::F56, List::<Adt4>::nil(), v0)
}

pub fn f43(v0: Fn6, p155: List<(Bytes, Adt3)>, p156: List<(Bytes, Adt3)>) -> List<(Bytes, Adt3)> {
    let m154 = (p155, p156);
    {
        let mut m154 = m154;
        loop {
            let step = {
                let (v1, v2) = m154;
                let m152 = v2;
                match m152.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt3)>, List<(Bytes, Adt3)>), List<(Bytes, Adt3)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt3)>, List<(Bytes, Adt3)>), List<(Bytes, Adt3)>>(({
                            let c153: Fn6 = v0.clone();
                            c153.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: List<(Bytes, Adt3)>, v1: (Bytes, Adt3)) -> List<(Bytes, Adt3)> {
    List::cons(v1, v0)
}

pub fn f45(v0: Fn7, p160: List<Bytes>, p161: List<Bytes>) -> List<Bytes> {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v1, v2) = m159;
                let m157 = v2;
                match m157.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c158: Fn7 = v0.clone();
                            c158.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f47(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a162 = v2;
    let a163 = 0u64;
    if nat_eq(a162, a163) {
        Ok(f57(v0, 0u64))
    } else {
        let (_, h164) = {
            let (h165, _) = f32(Fn2::F58(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h165
        };
        Ok(h164)
    }
}

pub fn f48() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f49(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m169 = {
        let a167 = v0.clone();
        let a168 = {
            let (h166, _) = v1.clone();
            h166
        };
        index_bytes(a167, a168)
    };
    match m169 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a171 = {
                    let (h170, _) = v1.clone();
                    h170
                };
                let a172 = 1u64;
                nat_add(a171, a172)?
            }, {
                let a175 = {
                    let (_, h173) = v1;
                    h173
                };
                let a176 = f59(v0, {
                    let (h174, _) = v1;
                    h174
                }, v2)?;
                append_bytes(a175, a176)
            })))
        }
    }
}

pub fn f50(v0: Fn8, p180: Option<Bytes>, p181: List<Bytes>) -> Option<Bytes> {
    let m179 = (p180, p181);
    {
        let mut m179 = m179;
        loop {
            let step = {
                let (v1, v2) = m179;
                let m177 = v2;
                match m177.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c178: Fn8 = v0.clone();
                            c178.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m179 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m182 = v1;
    match m182 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a185 = {
                    let a183 = v3;
                    let a184 = v0;
                    append_bytes(a183, a184)
                };
                let a186 = v2;
                append_bytes(a185, a186)
            })
        }
    }
}

pub fn f52(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m187 = v0.clone();
    match m187.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a190 = {
                let (h188, _) = v2;
                h188
            };
            let a191 = {
                let (h189, _) = v1.clone();
                h189
            };
            if equal(a190, a191) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f53(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f60(f61(Fn10::F62, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m196 = {
        let (h192, _) = f63(Fn11::F64, {
            let a194 = {
                let a193 = v0;
                length_list(a193)
            };
            let a195 = 1u64;
            nat_add(a194, a195)?
        }, v1)?;
        h192
    };
    match m196.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f54(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a201 = {
        let a198 = f15({
            let (h197, _) = v0;
            h197
        })?;
        let a199 = Bytes::lit(&[0x3a]);
        append_bytes(a198, a199)
    };
    let a202 = {
        let (_, h200) = v0;
        h200
    };
    Ok(append_bytes(a201, a202))
}

pub fn f55(v0: Fn9, p206: List<Adt4>, p207: List<Adt4>) -> List<Adt4> {
    let m205 = (p206, p207);
    {
        let mut m205 = m205;
        loop {
            let step = {
                let (v1, v2) = m205;
                let m203 = v2;
                match m203.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c204: Fn9 = v0.clone();
                            c204.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f57(v0: Bytes, v1: u64) -> Bytes {
    let m211 = {
        let a208 = v0;
        let a209 = v1;
        let a210 = 1u64;
        slice_bytes(a208, a209, a210)
    };
    match m211 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f58(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f65(v1, v0, v2)
}

pub fn f59(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a256 = {
            let a232 = {
                let a220 = {
                    let a214 = {
                        let a212 = v3;
                        let a213 = 128u8;
                        fixed_u8::bit_and(a212, a213)
                    };
                    let a215 = 128u8;
                    if equal(a214, a215) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a221 = {
                    let a218 = {
                        let a216 = v3;
                        let a217 = 64u8;
                        fixed_u8::bit_and(a216, a217)
                    };
                    let a219 = 64u8;
                    if equal(a218, a219) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a220, a221)?
            };
            let a233 = {
                let a230 = {
                    let a224 = {
                        let a222 = v3;
                        let a223 = 32u8;
                        fixed_u8::bit_and(a222, a223)
                    };
                    let a225 = 32u8;
                    if equal(a224, a225) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a231 = {
                    let a228 = {
                        let a226 = v3;
                        let a227 = 16u8;
                        fixed_u8::bit_and(a226, a227)
                    };
                    let a229 = 16u8;
                    if equal(a228, a229) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a230, a231)?
            };
            nat_add(a232, a233)?
        };
        let a257 = {
            let a254 = {
                let a242 = {
                    let a236 = {
                        let a234 = v3;
                        let a235 = 8u8;
                        fixed_u8::bit_and(a234, a235)
                    };
                    let a237 = 8u8;
                    if equal(a236, a237) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a243 = {
                    let a240 = {
                        let a238 = v3;
                        let a239 = 4u8;
                        fixed_u8::bit_and(a238, a239)
                    };
                    let a241 = 4u8;
                    if equal(a240, a241) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a242, a243)?
            };
            let a255 = {
                let a252 = {
                    let a246 = {
                        let a244 = v3;
                        let a245 = 2u8;
                        fixed_u8::bit_and(a244, a245)
                    };
                    let a247 = 2u8;
                    if equal(a246, a247) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a253 = {
                    let a250 = {
                        let a248 = v3;
                        let a249 = 1u8;
                        fixed_u8::bit_and(a248, a249)
                    };
                    let a251 = 1u8;
                    if equal(a250, a251) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a252, a253)?
            };
            nat_add(a254, a255)?
        };
        nat_add(a256, a257)?
    };
    let a258 = v4;
    let a259 = 34u64;
    if nat_eq(a258, a259) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a260 = v4;
        let a261 = 92u64;
        if nat_eq(a260, a261) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a262 = v4;
            let a263 = 8u64;
            if nat_eq(a262, a263) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a264 = v4;
                let a265 = 12u64;
                if nat_eq(a264, a265) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a266 = v4;
                    let a267 = 10u64;
                    if nat_eq(a266, a267) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a268 = v4;
                        let a269 = 13u64;
                        if nat_eq(a268, a269) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a270 = v4;
                            let a271 = 9u64;
                            if nat_eq(a270, a271) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a272 = v4;
                                let a273 = 32u64;
                                if nat_lt(a272, a273) {
                                    let a274 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a275 = f66(v2)?;
                                    Ok(append_bytes(a274, a275))
                                } else {
                                    Ok(f67(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f60(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f68(Fn12::F69, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f61(v0: Fn10, p279: List<List<(Bytes, Bytes)>>, p280: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m278 = (p279, p280);
    {
        let mut m278 = m278;
        loop {
            let step = {
                let (v1, v2) = m278;
                let m276 = v2;
                match m276.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c277: Fn10 = v0.clone();
                            c277.apply(v1, v3)
                        }, v4))
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

pub fn f62(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f63(v0: Fn11, p285: u64, p286: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v1, v2) = m284;
                let m281 = v1;
                if m281 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m281 - 1;
                    let m283 = {
                        let c282: Fn11 = v0.clone();
                        c282.apply(v2.clone())?
                    };
                    match m283 {
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
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f70(v0)
}

pub fn f65(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h287, _) = v2;
        h287
    };
    let a288 = v3;
    let a289 = 0u64;
    if nat_eq(a288, a289) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a290 = v3;
            let a291 = v1;
            let a292 = 0u64;
            nat_quot(a290, a291, a292)
        }, {
            let a297 = f57(v0, {
                let a293 = v3;
                let a294 = v1;
                let a295 = 0u64;
                nat_rem(a293, a294, a295)
            });
            let a298 = {
                let (_, h296) = v2;
                h296
            };
            append_bytes(a297, a298)
        }))
    }
}

pub fn f66(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a343 = {
            let a319 = {
                let a307 = {
                    let a301 = {
                        let a299 = v1;
                        let a300 = 128u8;
                        fixed_u8::bit_and(a299, a300)
                    };
                    let a302 = 128u8;
                    if equal(a301, a302) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a308 = {
                    let a305 = {
                        let a303 = v1;
                        let a304 = 64u8;
                        fixed_u8::bit_and(a303, a304)
                    };
                    let a306 = 64u8;
                    if equal(a305, a306) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a307, a308)?
            };
            let a320 = {
                let a317 = {
                    let a311 = {
                        let a309 = v1;
                        let a310 = 32u8;
                        fixed_u8::bit_and(a309, a310)
                    };
                    let a312 = 32u8;
                    if equal(a311, a312) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a318 = {
                    let a315 = {
                        let a313 = v1;
                        let a314 = 16u8;
                        fixed_u8::bit_and(a313, a314)
                    };
                    let a316 = 16u8;
                    if equal(a315, a316) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a317, a318)?
            };
            nat_add(a319, a320)?
        };
        let a344 = {
            let a341 = {
                let a329 = {
                    let a323 = {
                        let a321 = v1;
                        let a322 = 8u8;
                        fixed_u8::bit_and(a321, a322)
                    };
                    let a324 = 8u8;
                    if equal(a323, a324) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a330 = {
                    let a327 = {
                        let a325 = v1;
                        let a326 = 4u8;
                        fixed_u8::bit_and(a325, a326)
                    };
                    let a328 = 4u8;
                    if equal(a327, a328) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a329, a330)?
            };
            let a342 = {
                let a339 = {
                    let a333 = {
                        let a331 = v1;
                        let a332 = 2u8;
                        fixed_u8::bit_and(a331, a332)
                    };
                    let a334 = 2u8;
                    if equal(a333, a334) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a340 = {
                    let a337 = {
                        let a335 = v1;
                        let a336 = 1u8;
                        fixed_u8::bit_and(a335, a336)
                    };
                    let a338 = 1u8;
                    if equal(a337, a338) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a339, a340)?
            };
            nat_add(a341, a342)?
        };
        nat_add(a343, a344)?
    };
    let a351 = f57(f71(), {
        let a345 = v2;
        let a346 = 16u64;
        let a347 = 0u64;
        nat_quot(a345, a346, a347)
    });
    let a352 = f57(f71(), {
        let a348 = v2;
        let a349 = 16u64;
        let a350 = 0u64;
        nat_rem(a348, a349, a350)
    });
    Ok(append_bytes(a351, a352))
}

pub fn f67(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m356 = {
        let a353 = v0;
        let a354 = v1;
        let a355 = v2;
        slice_bytes(a353, a354, a355)
    };
    match m356 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f68(v0: Fn12, p360: List<List<(Bytes, Bytes)>>, p361: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m359 = (p360, p361);
    {
        let mut m359 = m359;
        loop {
            let step = {
                let (v1, v2) = m359;
                let m357 = v2;
                match m357.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c358: Fn12 = v0.clone();
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

pub fn f69(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f70(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a363 = {
        let a362 = v0.clone();
        length_list(a362)
    };
    let a364 = 1u64;
    if nat_le(a363, a364) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f72(v0)?))
    }
}

pub fn f71() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f72(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f60({
        let (_, h365) = {
            let (h366, _) = f73(Fn13::F74, {
                let a368 = {
                    let a367 = v0.clone();
                    length_list(a367)
                };
                let a369 = 1u64;
                nat_add(a368, a369)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h366
        };
        h365
    }))
}

pub fn f73(v0: Fn13, p374: u64, p375: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m373 = (p374, p375);
    {
        let mut m373 = m373;
        loop {
            let step = {
                let (v1, v2) = m373;
                let m370 = v1;
                if m370 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m370 - 1;
                    let m372 = {
                        let c371: Fn13 = v0.clone();
                        c371.apply(v2.clone())?
                    };
                    match m372 {
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
                Ok(next) => { m373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f75(v0)
}

pub fn f75(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h376, _) = v0;
        h376
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h377) = v0;
        h377
    };
    let m378 = v1;
    match m378.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m379 = v4;
            match m379.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f76(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f76(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a384 = {
            let a382 = {
                let a380 = v0.clone();
                length_list(a380)
            };
            let a383 = {
                let a381 = v1.clone();
                length_list(a381)
            };
            nat_add(a382, a383)?
        };
        let a385 = 1u64;
        nat_add(a384, a385)?
    };
    Ok(f77({
        let (_, h386) = {
            let (_, h387) = {
                let (h388, _) = f78(Fn14::F79, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h388
            };
            h387
        };
        h386
    }))
}

pub fn f77(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f35(Fn3::F80, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f78(v0: Fn14, p393: u64, p394: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m392 = (p393, p394);
    {
        let mut m392 = m392;
        loop {
            let step = {
                let (v1, v2) = m392;
                let m389 = v1;
                if m389 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m389 - 1;
                    let m391 = {
                        let c390: Fn14 = v0.clone();
                        c390.apply(v2.clone())
                    };
                    match m391 {
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
                Ok(next) => { m392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f81(v0)
}

pub fn f80(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f81(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h395, _) = v0.clone();
        h395
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h396, _) = {
            let (_, h397) = v0.clone();
            h397
        };
        h396
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h398) = {
            let (_, h399) = v0;
            h399
        };
        h398
    };
    let m400 = v1.clone();
    match m400.uncons() {
        None => {
            let m401 = v2;
            match m401.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m402 = v2.clone();
            match m402.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f82({
                        let (h403, _) = v8.clone();
                        h403
                    }, {
                        let (h404, _) = v6.clone();
                        h404
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

pub fn f82(v0: Bytes, v1: Bytes) -> bool {
    let a409 = {
        let a405 = v0;
        let a406 = v1;
        compare_bytes(a405, a406)
    };
    let a410 = {
        let a407 = Bytes::lit(&[]);
        let a408 = Bytes::lit(&[0x01]);
        compare_bytes(a407, a408)
    };
    equal(a409, a410)
}
