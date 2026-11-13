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
    C2(List<(Option<Bytes>, Bytes)>),
    C3(List<(Option<Bytes>, Bytes)>),
    C4(std::rc::Rc<Adt0>),
    C5(Bytes, std::rc::Rc<Adt0>),
    C6(Adt1, List<(Option<Bytes>, Bytes)>),
    C7(Bytes, List<Bytes>, std::rc::Rc<Adt0>),
    C8(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C9,
    C10(Adt5),
    C11(Bytes, Adt4, Adt1, List<(Option<Bytes>, Bytes)>),
    C12(Option<Bytes>, Bytes, Adt4, Adt1),
    C13,
    C14(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C15(Bytes, List<Bytes>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C16(Adt4, Bytes, Adt1, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes),
    C1(Bytes),
    C2(Adt2, Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Adt3, std::rc::Rc<Adt1>, Adt4),
    C6(bool),
    C7,
    C8(Adt4),
    C9(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C10(Option<Bytes>, Bytes, Adt4, std::rc::Rc<Adt1>),
    C11(Option<Bytes>, Bytes, Adt4, std::rc::Rc<Adt1>),
    C12(Option<Bytes>, Bytes, Adt4, Option<Bytes>, Bytes),
    C13(std::rc::Rc<Adt1>, Bytes),
    C14(Option<Bytes>, Bytes, Adt4, std::rc::Rc<Adt1>),
    C15(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C16(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C17(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C18(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C19(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C20(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C21(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C22(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C23(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C24(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C25(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C26(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C27(std::rc::Rc<Adt1>),
    C28(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C29(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C30(Bytes, Adt4, std::rc::Rc<Adt1>),
    C31(Bytes, Adt4, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C32(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C33(std::rc::Rc<Adt1>),
    C34(std::rc::Rc<Adt1>),
    C35(List<(Bytes, Adt4)>, List<Bytes>, std::rc::Rc<Adt1>),
    C36(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C37(Option<Bytes>, Bytes, Adt4),
    C38(Adt4, Adt4, std::rc::Rc<Adt1>),
    C39(Adt4, std::rc::Rc<Adt1>),
    C40(Adt4, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C41(Option<Bytes>, Bytes, Adt4, std::rc::Rc<Adt1>, List<Bytes>),
    C42,
    C43(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C44(Bytes, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C45(Option<Bytes>, Bytes, List<Bytes>, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C46(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C47(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
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
    C0(Bytes, Bytes, Option<Bytes>, Bytes, Option<Bytes>, Bytes, Bytes),
    C1(Bytes, Option<Bytes>, Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt6>),
    C4(std::rc::Rc<Adt6>),
    C5,
    C6(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C7(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, bool, bool, Option<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F22,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt6>, p1: (Option<Bytes>, Bytes)) -> List<Adt6> {
        match self {
            Fn0::F22 => f22(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F29,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt6>, p1: Bytes) -> List<Adt6> {
        match self {
            Fn1::F29 => f29(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F32,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt6>, p1: (Bytes, Bytes)) -> List<Adt6> {
        match self {
            Fn2::F32 => f32(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F38(Bytes),
    F70(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F38(k0) => f38(k0.clone(), p0),
            Fn3::F70(k0, k1) => Ok(f70(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F41,
    F56,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn4::F41 => f41(p0, p1),
            Fn4::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F44,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn5::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F46,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt6, p1: Adt6) -> Adt6 {
        match self {
            Fn6::F46 => f46(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F49,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Option<Bytes>, Bytes)>, p1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
        match self {
            Fn7::F49 => f49(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F52,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt6>, p1: (Bytes, Adt4)) -> List<Adt6> {
        match self {
            Fn8::F52 => f52(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F55,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn9::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F61(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn10::F61(k0) => f61(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F66,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt6) -> List<Adt6> {
        match self {
            Fn11::F66 => f66(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F68,
}

impl Fn12 {
    pub fn apply(&self, p0: List<(Bytes, Adt4)>, p1: (Bytes, Adt4)) -> List<(Bytes, Adt4)> {
        match self {
            Fn12::F68 => f68(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F74,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn13::F74 => f74(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F76,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn14::F76 => f76(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F81,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F81 => f81(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F86,
}

impl Fn16 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn16::F86 => f86(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F90,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn17::F90 => f90(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0))
}

pub fn f1(v0: Adt6) -> R<Bytes> {
    f3(v0)
}

pub fn f2(v0: Adt0) -> Adt6 {
    let m1 = v0;
    match m1 {
        Adt0::C0 => {
            f4(Str::lit("reflexivity"), Adt6::C5)
        }
        Adt0::C1 => {
            f4(Str::lit("decide"), Adt6::C5)
        }
        Adt0::C2(v1) => {
            f4(Str::lit("linear_arithmetic"), {
                let m2 = v1.clone();
                match m2.uncons() {
                    None => {
                        Adt6::C5
                    }
                    Some((_, _)) => {
                        f5(Str::lit("definitions"), f6(v1), Adt6::C5)
                    }
                }
            })
        }
        Adt0::C3(v4) => {
            f4(Str::lit("simplify"), f5(Str::lit("definitions"), f6(v4), Adt6::C5))
        }
        Adt0::C4(r5) => {
            let v5 = (*r5).clone();
            f4(Str::lit("constructor"), f5(Str::lit("branches"), Adt6::C3(std::rc::Rc::new(f2(v5))), Adt6::C5))
        }
        Adt0::C5(v6, r6) => {
            let v7 = (*r6).clone();
            f4(Str::lit("cases"), f5(Str::lit("scrutinee"), Adt6::C2(v6), f5(Str::lit("branches"), Adt6::C3(std::rc::Rc::new(f2(v7))), Adt6::C5)))
        }
        Adt0::C6(v8, v9) => {
            f4(Str::lit("term_cases"), f5(Str::lit("scrutinee"), f7(v8), f5(Str::lit("definitions"), f6(v9), Adt6::C5)))
        }
        Adt0::C7(v10, v11, r7) => {
            let v12 = (*r7).clone();
            f4(Str::lit("induction"), f5(Str::lit("scrutinee"), Adt6::C2(v10), {
                let m3 = v11.clone();
                match m3.uncons() {
                    None => {
                        f5(Str::lit("branches"), Adt6::C3(std::rc::Rc::new(f2(v12))), Adt6::C5)
                    }
                    Some((_, _)) => {
                        f5(Str::lit("generalizing"), f8(v11), f5(Str::lit("branches"), Adt6::C3(std::rc::Rc::new(f2(v12))), Adt6::C5))
                    }
                }
            }))
        }
        Adt0::C8(v15, r8, r9) => {
            let v16 = (*r8).clone();
            let v17 = (*r9).clone();
            f4(Str::lit("nested_induction"), f5(Str::lit("scrutinee"), Adt6::C2(v15), f5(Str::lit("branches"), Adt6::C3(std::rc::Rc::new(f2(v16))), f5(Str::lit("members"), Adt6::C3(std::rc::Rc::new(f2(v17))), Adt6::C5))))
        }
        Adt0::C9 => {
            f4(Str::lit("congruence"), Adt6::C5)
        }
        Adt0::C10(v18) => {
            f4(Str::lit("boolean_reflection"), f5(Str::lit("reflection"), f9(v18), Adt6::C5))
        }
        Adt0::C11(v19, v20, v21, v22) => {
            f4(Str::lit("law"), f5(Str::lit("name"), Adt6::C2(v19), f10(Str::lit("type_arguments"), v20, f5(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f7(v21))), {
                let m4 = v22.clone();
                match m4.uncons() {
                    None => {
                        Adt6::C5
                    }
                    Some((_, _)) => {
                        f5(Str::lit("premises"), f6(v22), Adt6::C5)
                    }
                }
            }))))
        }
        Adt0::C12(v25, v26, v27, v28) => {
            f4(Str::lit("apply"), f5(Str::lit("theorem"), f11(v25, v26), f10(Str::lit("type_arguments"), v27, f5(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f7(v28))), Adt6::C5))))
        }
        Adt0::C13 => {
            Adt6::C5
        }
        Adt0::C14(r10, r11) => {
            let v29 = (*r10).clone();
            let v30 = (*r11).clone();
            Adt6::C6(std::rc::Rc::new(f2(v29)), std::rc::Rc::new(f2(v30)))
        }
        Adt0::C15(v31, v32, r12, r13) => {
            let v33 = (*r12).clone();
            let v34 = (*r13).clone();
            Adt6::C6(std::rc::Rc::new(Adt6::C4(std::rc::Rc::new(f5(Str::lit("constructor"), Adt6::C2(v31), f5(Str::lit("binders"), f8(v32), f5(Str::lit("proof"), f2(v33), Adt6::C5)))))), std::rc::Rc::new(f2(v34)))
        }
        Adt0::C16(v35, v36, v37, r14, r15) => {
            let v38 = (*r14).clone();
            let v39 = (*r15).clone();
            Adt6::C6(std::rc::Rc::new(Adt6::C4(std::rc::Rc::new(f5(Str::lit("type"), f12(v35), f5(Str::lit("binder"), Adt6::C2(v36), f5(Str::lit("statement"), f7(v37), f5(Str::lit("branches"), Adt6::C3(std::rc::Rc::new(f2(v38))), Adt6::C5))))))), std::rc::Rc::new(f2(v39)))
        }
    }
}

pub fn f3(v0: Adt6) -> R<Bytes> {
    let m16 = v0;
    match m16 {
        Adt6::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt6::C1(v2, v3) => {
            f13(v2, v3)
        }
        Adt6::C2(v4) => {
            f14(v4)
        }
        Adt6::C3(r17) => {
            let v5 = (*r17).clone();
            Ok(f15(f16(v5)?))
        }
        Adt6::C4(r18) => {
            let v6 = (*r18).clone();
            f17(f18(v6)?)
        }
        Adt6::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt6::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt6::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Str, v1: Adt6) -> Adt6 {
    Adt6::C4(std::rc::Rc::new(f5(Str::lit("kind"), Adt6::C2(f19(v0)), v1)))
}

pub fn f5(v0: Str, v1: Adt6, v2: Adt6) -> Adt6 {
    Adt6::C7(f19(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f6(v0: List<(Option<Bytes>, Bytes)>) -> Adt6 {
    Adt6::C3(std::rc::Rc::new(f20(f21(Fn0::F22, List::<Adt6>::nil(), f23(v0)))))
}

pub fn f7(v0: Adt1) -> Adt6 {
    let m19 = v0;
    match m19 {
        Adt1::C0(v1) => {
            f4(Str::lit("var"), f5(Str::lit("name"), Adt6::C2(v1), Adt6::C5))
        }
        Adt1::C1(v2) => {
            f4(Str::lit("nat"), f5(Str::lit("value"), Adt6::C2(v2), Adt6::C5))
        }
        Adt1::C2(v3, v4) => {
            f4(Str::lit("integer"), f5(Str::lit("representation"), Adt6::C2(f24(v3)), f5(Str::lit("value"), Adt6::C2(v4), Adt6::C5)))
        }
        Adt1::C3(v5) => {
            f4(Str::lit("string"), f5(Str::lit("value"), Adt6::C2(v5), Adt6::C5))
        }
        Adt1::C4(v6) => {
            f4(Str::lit("bytes"), f5(Str::lit("hex"), Adt6::C2(v6), Adt6::C5))
        }
        Adt1::C5(v7, r22, v9) => {
            let v8 = (*r22).clone();
            f4(Str::lit("primitive"), f5(Str::lit("operation"), Adt6::C2({
                let m21 = f25(v7);
                match m21 {
                    Adt7::C0(h20, _, _, _) => {
                        h20
                    }
                }
            }), f5(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f7(v8))), f5(Str::lit("result"), f12(v9), Adt6::C5))))
        }
        Adt1::C6(v10) => {
            f4(Str::lit("bool"), f5(Str::lit("value"), Adt6::C0(v10), Adt6::C5))
        }
        Adt1::C7 => {
            f4(Str::lit("unit"), Adt6::C5)
        }
        Adt1::C8(v11) => {
            f4(Str::lit("nil"), f5(Str::lit("element"), f12(v11), Adt6::C5))
        }
        Adt1::C9(r23, r24) => {
            let v12 = (*r23).clone();
            let v13 = (*r24).clone();
            f4(Str::lit("cons"), f5(Str::lit("head"), f7(v12), f5(Str::lit("tail"), f7(v13), Adt6::C5)))
        }
        Adt1::C10(v14, v15, v16, r25) => {
            let v17 = (*r25).clone();
            f4(Str::lit("record"), f5(Str::lit("type"), f11(v14, v15), f10(Str::lit("type_arguments"), v16, f5(Str::lit("fields"), Adt6::C3(std::rc::Rc::new(f7(v17))), Adt6::C5))))
        }
        Adt1::C11(v18, v19, v20, r26) => {
            let v21 = (*r26).clone();
            f4(Str::lit("constructor"), f5(Str::lit("constructor"), f11(v18, v19), f10(Str::lit("type_arguments"), v20, f5(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f7(v21))), Adt6::C5))))
        }
        Adt1::C12(v22, v23, v24, v25, v26) => {
            f4(Str::lit("instance_value"), f5(Str::lit("class"), f11(v22, v23), f5(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f12(v24))), f5(Str::lit("resolved"), f11(v25, v26), Adt6::C5))))
        }
        Adt1::C13(r27, v28) => {
            let v27 = (*r27).clone();
            f4(Str::lit("project"), f5(Str::lit("value"), f7(v27), f5(Str::lit("field"), Adt6::C2(v28), Adt6::C5)))
        }
        Adt1::C14(v29, v30, v31, r28) => {
            let v32 = (*r28).clone();
            f4(Str::lit("call"), f5(Str::lit("function"), f11(v29, v30), f10(Str::lit("type_arguments"), v31, f5(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f7(v32))), Adt6::C5))))
        }
        Adt1::C15(r29, r30, r31) => {
            let v33 = (*r29).clone();
            let v34 = (*r30).clone();
            let v35 = (*r31).clone();
            f4(Str::lit("if"), f5(Str::lit("condition"), f7(v33), f5(Str::lit("then_value"), f7(v34), f5(Str::lit("else_value"), f7(v35), Adt6::C5))))
        }
        Adt1::C16(r32, r33) => {
            let v36 = (*r32).clone();
            let v37 = (*r33).clone();
            f4(Str::lit("match"), f5(Str::lit("scrutinee"), f7(v36), f5(Str::lit("branches"), Adt6::C3(std::rc::Rc::new(f7(v37))), Adt6::C5)))
        }
        Adt1::C17(r34, r35) => {
            let v38 = (*r34).clone();
            let v39 = (*r35).clone();
            f4(Str::lit("eq"), f5(Str::lit("left"), f7(v38), f5(Str::lit("right"), f7(v39), Adt6::C5)))
        }
        Adt1::C18(r36, r37) => {
            let v40 = (*r36).clone();
            let v41 = (*r37).clone();
            f4(Str::lit("le"), f5(Str::lit("left"), f7(v40), f5(Str::lit("right"), f7(v41), Adt6::C5)))
        }
        Adt1::C19(r38, r39) => {
            let v42 = (*r38).clone();
            let v43 = (*r39).clone();
            f4(Str::lit("lt"), f5(Str::lit("left"), f7(v42), f5(Str::lit("right"), f7(v43), Adt6::C5)))
        }
        Adt1::C20(r40, r41) => {
            let v44 = (*r40).clone();
            let v45 = (*r41).clone();
            f4(Str::lit("add"), f5(Str::lit("left"), f7(v44), f5(Str::lit("right"), f7(v45), Adt6::C5)))
        }
        Adt1::C21(r42, r43) => {
            let v46 = (*r42).clone();
            let v47 = (*r43).clone();
            f4(Str::lit("beq"), f5(Str::lit("left"), f7(v46), f5(Str::lit("right"), f7(v47), Adt6::C5)))
        }
        Adt1::C22(r44, r45) => {
            let v48 = (*r44).clone();
            let v49 = (*r45).clone();
            f4(Str::lit("ble"), f5(Str::lit("left"), f7(v48), f5(Str::lit("right"), f7(v49), Adt6::C5)))
        }
        Adt1::C23(r46, r47) => {
            let v50 = (*r46).clone();
            let v51 = (*r47).clone();
            f4(Str::lit("blt"), f5(Str::lit("left"), f7(v50), f5(Str::lit("right"), f7(v51), Adt6::C5)))
        }
        Adt1::C24(r48, r49) => {
            let v52 = (*r48).clone();
            let v53 = (*r49).clone();
            f4(Str::lit("and"), f5(Str::lit("left"), f7(v52), f5(Str::lit("right"), f7(v53), Adt6::C5)))
        }
        Adt1::C25(r50, r51) => {
            let v54 = (*r50).clone();
            let v55 = (*r51).clone();
            f4(Str::lit("prop_and"), f5(Str::lit("left"), f7(v54), f5(Str::lit("right"), f7(v55), Adt6::C5)))
        }
        Adt1::C26(r52, r53) => {
            let v56 = (*r52).clone();
            let v57 = (*r53).clone();
            f4(Str::lit("or"), f5(Str::lit("left"), f7(v56), f5(Str::lit("right"), f7(v57), Adt6::C5)))
        }
        Adt1::C27(r54) => {
            let v58 = (*r54).clone();
            f4(Str::lit("not"), f5(Str::lit("value"), f7(v58), Adt6::C5))
        }
        Adt1::C28(r55, r56) => {
            let v59 = (*r55).clone();
            let v60 = (*r56).clone();
            f4(Str::lit("implies"), f5(Str::lit("premise"), f7(v59), f5(Str::lit("conclusion"), f7(v60), Adt6::C5)))
        }
        Adt1::C29(r57, r58) => {
            let v61 = (*r57).clone();
            let v62 = (*r58).clone();
            f4(Str::lit("iff"), f5(Str::lit("left"), f7(v61), f5(Str::lit("right"), f7(v62), Adt6::C5)))
        }
        Adt1::C30(v63, v64, r59) => {
            let v65 = (*r59).clone();
            f4(Str::lit("forall"), f5(Str::lit("binder"), f26(v63, v64), f5(Str::lit("body"), f7(v65), Adt6::C5)))
        }
        Adt1::C31(v66, v67, r60, r61) => {
            let v68 = (*r60).clone();
            let v69 = (*r61).clone();
            f4(Str::lit("let"), f5(Str::lit("binder"), f26(v66, v67), f5(Str::lit("value"), f7(v68), f5(Str::lit("body"), f7(v69), Adt6::C5))))
        }
        Adt1::C32(r62, r63) => {
            let v70 = (*r62).clone();
            let v71 = (*r63).clone();
            f4(Str::lit("pair"), f5(Str::lit("left"), f7(v70), f5(Str::lit("right"), f7(v71), Adt6::C5)))
        }
        Adt1::C33(r64) => {
            let v72 = (*r64).clone();
            f4(Str::lit("first"), f5(Str::lit("value"), f7(v72), Adt6::C5))
        }
        Adt1::C34(r65) => {
            let v73 = (*r65).clone();
            f4(Str::lit("second"), f5(Str::lit("value"), f7(v73), Adt6::C5))
        }
        Adt1::C35(v74, v75, r66) => {
            let v76 = (*r66).clone();
            f4(Str::lit("lambda"), f5(Str::lit("parameters"), f27(v74), f5(Str::lit("captures"), f8(v75), f5(Str::lit("body"), f7(v76), Adt6::C5))))
        }
        Adt1::C36(r67, r68) => {
            let v77 = (*r67).clone();
            let v78 = (*r68).clone();
            f4(Str::lit("apply"), f5(Str::lit("function"), f7(v77), f5(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f7(v78))), Adt6::C5)))
        }
        Adt1::C37(v79, v80, v81) => {
            f4(Str::lit("function_ref"), f5(Str::lit("function"), f11(v79, v80), f10(Str::lit("type_arguments"), v81, Adt6::C5)))
        }
        Adt1::C38(v82, v83, r69) => {
            let v84 = (*r69).clone();
            f4(Str::lit("map_literal"), f5(Str::lit("key"), f12(v82), f5(Str::lit("value"), f12(v83), f5(Str::lit("entries"), Adt6::C3(std::rc::Rc::new(f7(v84))), Adt6::C5))))
        }
        Adt1::C39(v85, r70) => {
            let v86 = (*r70).clone();
            f4(Str::lit("set_literal"), f5(Str::lit("element"), f12(v85), f5(Str::lit("elements"), Adt6::C3(std::rc::Rc::new(f7(v86))), Adt6::C5)))
        }
        Adt1::C40(v87, r71, r72) => {
            let v88 = (*r71).clone();
            let v89 = (*r72).clone();
            f4(Str::lit("graph_literal"), f5(Str::lit("node"), f12(v87), f5(Str::lit("nodes"), Adt6::C3(std::rc::Rc::new(f7(v88))), f5(Str::lit("edges"), Adt6::C3(std::rc::Rc::new(f7(v89))), Adt6::C5))))
        }
        Adt1::C41(v90, v91, v92, r73, v94) => {
            let v93 = (*r73).clone();
            f4(Str::lit("checked_apply"), f5(Str::lit("model"), f11(v90, v91), f10(Str::lit("type_arguments"), v92, f5(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f7(v93))), f5(Str::lit("checks"), f8(v94), Adt6::C5)))))
        }
        Adt1::C42 => {
            Adt6::C5
        }
        Adt1::C43(r74, r75) => {
            let v95 = (*r74).clone();
            let v96 = (*r75).clone();
            Adt6::C6(std::rc::Rc::new(f7(v95)), std::rc::Rc::new(f7(v96)))
        }
        Adt1::C44(v97, r76, r77) => {
            let v98 = (*r76).clone();
            let v99 = (*r77).clone();
            Adt6::C6(std::rc::Rc::new(Adt6::C4(std::rc::Rc::new(f5(Str::lit("field"), Adt6::C2(v97), f5(Str::lit("value"), f7(v98), Adt6::C5))))), std::rc::Rc::new(f7(v99)))
        }
        Adt1::C45(v100, v101, v102, r78, r79) => {
            let v103 = (*r78).clone();
            let v104 = (*r79).clone();
            Adt6::C6(std::rc::Rc::new(Adt6::C4(std::rc::Rc::new(f5(Str::lit("constructor"), f11(v100, v101), f5(Str::lit("binders"), f8(v102), f5(Str::lit("body"), f7(v103), Adt6::C5)))))), std::rc::Rc::new(f7(v104)))
        }
        Adt1::C46(r80, r81, r82) => {
            let v105 = (*r80).clone();
            let v106 = (*r81).clone();
            let v107 = (*r82).clone();
            Adt6::C6(std::rc::Rc::new(Adt6::C4(std::rc::Rc::new(f5(Str::lit("key"), f7(v105), f5(Str::lit("value"), f7(v106), Adt6::C5))))), std::rc::Rc::new(f7(v107)))
        }
        Adt1::C47(r83, r84, r85) => {
            let v108 = (*r83).clone();
            let v109 = (*r84).clone();
            let v110 = (*r85).clone();
            Adt6::C6(std::rc::Rc::new(Adt6::C4(std::rc::Rc::new(f5(Str::lit("source"), f7(v108), f5(Str::lit("target"), f7(v109), Adt6::C5))))), std::rc::Rc::new(f7(v110)))
        }
    }
}

pub fn f8(v0: List<Bytes>) -> Adt6 {
    Adt6::C3(std::rc::Rc::new(f20(f28(Fn1::F29, List::<Adt6>::nil(), f30(v0)))))
}

pub fn f9(v0: Adt5) -> Adt6 {
    let m86 = v0;
    match m86 {
        Adt5::C0(v1, v2, v3, v4, v5, v6, v7) => {
            f4(Str::lit("list"), f5(Str::lit("parameter"), Adt6::C2(v1), f5(Str::lit("values"), Adt6::C2(v2), f5(Str::lit("boolean_definition"), f11(v3, v4), f5(Str::lit("proposition_definition"), f11(v5, v6), f5(Str::lit("comparison"), Adt6::C2(v7), Adt6::C5))))))
        }
        Adt5::C1(v8, v9, v10, v11, v12, v13) => {
            f4(Str::lit("record"), f5(Str::lit("record"), Adt6::C2(v8), f5(Str::lit("boolean_definition"), f11(v9, v10), f5(Str::lit("proposition_definition"), f11(v11, v12), f5(Str::lit("fields"), Adt6::C3(std::rc::Rc::new(f20(f31(Fn2::F32, List::<Adt6>::nil(), f33(v13))))), Adt6::C5)))))
        }
    }
}

pub fn f10(v0: Str, v1: Adt4, v2: Adt6) -> Adt6 {
    let m87 = v1.clone();
    match m87 {
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
            f5(v0, Adt6::C3(std::rc::Rc::new(f12(v1))), v2)
        }
    }
}

pub fn f11(v0: Option<Bytes>, v1: Bytes) -> Adt6 {
    Adt6::C4(std::rc::Rc::new({
        let m88 = v0;
        match m88 {
            None => {
                f5(Str::lit("name"), Adt6::C2(v1), Adt6::C5)
            }
            Some(v2) => {
                f5(Str::lit("module"), Adt6::C2(v2), f5(Str::lit("name"), Adt6::C2(v1), Adt6::C5))
            }
        }
    }))
}

pub fn f12(v0: Adt4) -> Adt6 {
    let m89 = v0;
    match m89 {
        Adt4::C0 => {
            f34(Str::lit("type"), Adt6::C5)
        }
        Adt4::C1 => {
            f34(Str::lit("nat"), Adt6::C5)
        }
        Adt4::C2 => {
            f34(Str::lit("bool"), Adt6::C5)
        }
        Adt4::C3 => {
            f34(Str::lit("prop"), Adt6::C5)
        }
        Adt4::C4 => {
            f34(Str::lit("unit"), Adt6::C5)
        }
        Adt4::C5 => {
            f34(Str::lit("int"), Adt6::C5)
        }
        Adt4::C6 => {
            f34(Str::lit("int8"), Adt6::C5)
        }
        Adt4::C7 => {
            f34(Str::lit("int16"), Adt6::C5)
        }
        Adt4::C8 => {
            f34(Str::lit("int32"), Adt6::C5)
        }
        Adt4::C9 => {
            f34(Str::lit("int64"), Adt6::C5)
        }
        Adt4::C10 => {
            f34(Str::lit("uint8"), Adt6::C5)
        }
        Adt4::C11 => {
            f34(Str::lit("uint16"), Adt6::C5)
        }
        Adt4::C12 => {
            f34(Str::lit("uint32"), Adt6::C5)
        }
        Adt4::C13 => {
            f34(Str::lit("uint64"), Adt6::C5)
        }
        Adt4::C14 => {
            f34(Str::lit("string"), Adt6::C5)
        }
        Adt4::C15 => {
            f34(Str::lit("bytes"), Adt6::C5)
        }
        Adt4::C16 => {
            f34(Str::lit("ordering"), Adt6::C5)
        }
        Adt4::C17 => {
            f34(Str::lit("contract_violation"), Adt6::C5)
        }
        Adt4::C18 => {
            f34(Str::lit("reasoning_failure"), Adt6::C5)
        }
        Adt4::C19(v1) => {
            f34(Str::lit("parameter"), f35(Str::lit("name"), Adt6::C2(v1), Adt6::C5))
        }
        Adt4::C20(r91) => {
            let v2 = (*r91).clone();
            f34(Str::lit("option"), f35(Str::lit("value"), f12(v2), Adt6::C5))
        }
        Adt4::C21(r92, r93) => {
            let v3 = (*r92).clone();
            let v4 = (*r93).clone();
            f34(Str::lit("result"), f35(Str::lit("ok"), f12(v3), f35(Str::lit("error"), f12(v4), Adt6::C5)))
        }
        Adt4::C22(r94) => {
            let v5 = (*r94).clone();
            f34(Str::lit("list"), f35(Str::lit("element"), f12(v5), Adt6::C5))
        }
        Adt4::C23(v6, v7, r95) => {
            let v8 = (*r95).clone();
            f34(Str::lit("named"), f35(Str::lit("member"), Adt6::C4(std::rc::Rc::new({
                let m90 = v6;
                match m90 {
                    None => {
                        f35(Str::lit("name"), Adt6::C2(v7), Adt6::C5)
                    }
                    Some(v9) => {
                        f35(Str::lit("module"), Adt6::C2(v9), f35(Str::lit("name"), Adt6::C2(v7), Adt6::C5))
                    }
                }
            })), f35(Str::lit("arguments"), Adt6::C3(std::rc::Rc::new(f12(v8))), Adt6::C5)))
        }
        Adt4::C24(r96, r97) => {
            let v10 = (*r96).clone();
            let v11 = (*r97).clone();
            f34(Str::lit("product"), f35(Str::lit("left"), f12(v10), f35(Str::lit("right"), f12(v11), Adt6::C5)))
        }
        Adt4::C25(r98, r99) => {
            let v12 = (*r98).clone();
            let v13 = (*r99).clone();
            f34(Str::lit("function"), f35(Str::lit("parameters"), Adt6::C3(std::rc::Rc::new(f12(v12))), f35(Str::lit("result"), f12(v13), Adt6::C5)))
        }
        Adt4::C26(r100, r101) => {
            let v14 = (*r100).clone();
            let v15 = (*r101).clone();
            f34(Str::lit("map"), f35(Str::lit("key"), f12(v14), f35(Str::lit("value"), f12(v15), Adt6::C5)))
        }
        Adt4::C27(r102) => {
            let v16 = (*r102).clone();
            f34(Str::lit("set"), f35(Str::lit("element"), f12(v16), Adt6::C5))
        }
        Adt4::C28 => {
            Adt6::C5
        }
        Adt4::C29(r103, r104) => {
            let v17 = (*r103).clone();
            let v18 = (*r104).clone();
            Adt6::C6(std::rc::Rc::new(f12(v17)), std::rc::Rc::new(f12(v18)))
        }
    }
}

pub fn f13(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a105 = 0u64;
        let a106 = v1;
        nat_lt(a105, a106)
    } else {
        false
    } {
        let a107 = Bytes::lit(&[0x2d]);
        let a108 = f36(v1)?;
        Ok(append_bytes(a107, a108))
    } else {
        f36(v1)
    }
}

pub fn f14(v0: Bytes) -> R<Bytes> {
    let a114 = {
        let a112 = Bytes::lit(&[0x22]);
        let a113 = {
            let (_, h109) = {
                let (h110, _) = f37(Fn3::F38(v0.clone()), {
                    let a111 = v0;
                    length_bytes(a111)
                }, (0u64, Bytes::lit(&[])))?;
                h110
            };
            h109
        };
        append_bytes(a112, a113)
    };
    let a115 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a114, a115))
}

pub fn f15(v0: List<Bytes>) -> Bytes {
    let a118 = {
        let a116 = Bytes::lit(&[0x5b]);
        let a117 = f39(v0, Bytes::lit(&[0x2c]));
        append_bytes(a116, a117)
    };
    let a119 = Bytes::lit(&[0x5d]);
    append_bytes(a118, a119)
}

pub fn f16(v0: Adt6) -> R<List<Bytes>> {
    let m120 = v0;
    match m120 {
        Adt6::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C6(r121, r122) => {
            let v7 = (*r121).clone();
            let v8 = (*r122).clone();
            Ok(List::cons(f3(v7)?, f16(v8)?))
        }
        Adt6::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f17(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f40(Fn4::F41, List::<(Bytes, Bytes)>::nil(), f42(v0)?);
    let v2: List<Bytes> = f43(Fn5::F44, List::<Bytes>::nil(), v1)?;
    let a125 = {
        let a123 = Bytes::lit(&[0x7b]);
        let a124 = f39(v2, Bytes::lit(&[0x2c]));
        append_bytes(a123, a124)
    };
    let a126 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a125, a126))
}

pub fn f18(v0: Adt6) -> R<List<(Bytes, Bytes)>> {
    let m127 = v0;
    match m127 {
        Adt6::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C7(v9, r128, r129) => {
            let v10 = (*r128).clone();
            let v11 = (*r129).clone();
            Ok(List::cons((v9, f3(v10)?), f18(v11)?))
        }
    }
}

pub fn f19(v0: Str) -> Bytes {
    let a130 = v0;
    utf8_encode(a130)
}

pub fn f20(v0: List<Adt6>) -> Adt6 {
    f45(Fn6::F46, Adt6::C5, f47(v0))
}

pub fn f21(v0: Fn0, p134: List<Adt6>, p135: List<(Option<Bytes>, Bytes)>) -> List<Adt6> {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v1, v2) = m133;
                let m131 = v2;
                match m131.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<(Option<Bytes>, Bytes)>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<(Option<Bytes>, Bytes)>), List<Adt6>>(({
                            let c132: Fn0 = v0.clone();
                            c132.apply(v1, v3)
                        }, v4))
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

pub fn f22(v0: List<Adt6>, v1: (Option<Bytes>, Bytes)) -> List<Adt6> {
    List::cons(f11({
        let (h136, _) = v1;
        h136
    }, {
        let (_, h137) = v1;
        h137
    }), v0)
}

pub fn f23(v0: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    f48(Fn7::F49, List::<(Option<Bytes>, Bytes)>::nil(), v0)
}

pub fn f24(v0: Adt2) -> Bytes {
    let m138 = v0;
    match m138 {
        Adt2::C0 => {
            f50(Str::lit("int"))
        }
        Adt2::C1 => {
            f50(Str::lit("int8"))
        }
        Adt2::C2 => {
            f50(Str::lit("int16"))
        }
        Adt2::C3 => {
            f50(Str::lit("int32"))
        }
        Adt2::C4 => {
            f50(Str::lit("int64"))
        }
        Adt2::C5 => {
            f50(Str::lit("uint8"))
        }
        Adt2::C6 => {
            f50(Str::lit("uint16"))
        }
        Adt2::C7 => {
            f50(Str::lit("uint32"))
        }
        Adt2::C8 => {
            f50(Str::lit("uint64"))
        }
    }
}

pub fn f25(v0: Adt3) -> Adt7 {
    let m139 = v0;
    match m139 {
        Adt3::C0 => {
            Adt7::C0(f50(Str::lit("subtract")), false, false, None::<u64>)
        }
        Adt3::C1 => {
            Adt7::C0(f50(Str::lit("multiply")), false, false, None::<u64>)
        }
        Adt3::C2 => {
            Adt7::C0(f50(Str::lit("quotient")), false, false, None::<u64>)
        }
        Adt3::C3 => {
            Adt7::C0(f50(Str::lit("remainder")), false, false, None::<u64>)
        }
        Adt3::C4 => {
            Adt7::C0(f50(Str::lit("negate")), false, false, None::<u64>)
        }
        Adt3::C5 => {
            Adt7::C0(f50(Str::lit("checked_convert")), false, false, None::<u64>)
        }
        Adt3::C6 => {
            Adt7::C0(f50(Str::lit("checked_add")), false, false, None::<u64>)
        }
        Adt3::C7 => {
            Adt7::C0(f50(Str::lit("checked_subtract")), false, false, None::<u64>)
        }
        Adt3::C8 => {
            Adt7::C0(f50(Str::lit("checked_multiply")), false, false, None::<u64>)
        }
        Adt3::C9 => {
            Adt7::C0(f50(Str::lit("checked_negate")), false, false, None::<u64>)
        }
        Adt3::C10 => {
            Adt7::C0(f50(Str::lit("checked_quotient")), false, false, None::<u64>)
        }
        Adt3::C11 => {
            Adt7::C0(f50(Str::lit("bit_and")), false, false, None::<u64>)
        }
        Adt3::C12 => {
            Adt7::C0(f50(Str::lit("bit_or")), false, false, None::<u64>)
        }
        Adt3::C13 => {
            Adt7::C0(f50(Str::lit("bit_xor")), false, false, None::<u64>)
        }
        Adt3::C14 => {
            Adt7::C0(f50(Str::lit("bit_not")), false, false, None::<u64>)
        }
        Adt3::C15 => {
            Adt7::C0(f50(Str::lit("shift_left")), false, false, None::<u64>)
        }
        Adt3::C16 => {
            Adt7::C0(f50(Str::lit("shift_right")), false, false, None::<u64>)
        }
        Adt3::C17 => {
            Adt7::C0(f50(Str::lit("append")), false, false, None::<u64>)
        }
        Adt3::C18 => {
            Adt7::C0(f50(Str::lit("length")), false, false, None::<u64>)
        }
        Adt3::C19 => {
            Adt7::C0(f50(Str::lit("index")), false, false, None::<u64>)
        }
        Adt3::C20 => {
            Adt7::C0(f50(Str::lit("slice")), false, false, None::<u64>)
        }
        Adt3::C21 => {
            Adt7::C0(f50(Str::lit("utf8_encode")), false, false, None::<u64>)
        }
        Adt3::C22 => {
            Adt7::C0(f50(Str::lit("utf8_decode")), false, false, None::<u64>)
        }
        Adt3::C23 => {
            Adt7::C0(f50(Str::lit("compare_bytes")), false, false, None::<u64>)
        }
        Adt3::C24 => {
            Adt7::C0(f50(Str::lit("equal")), false, false, None::<u64>)
        }
        Adt3::C25 => {
            Adt7::C0(f50(Str::lit("split_exact")), false, false, None::<u64>)
        }
        Adt3::C26 => {
            Adt7::C0(f50(Str::lit("join")), false, false, None::<u64>)
        }
        Adt3::C27 => {
            Adt7::C0(f50(Str::lit("parse_decimal")), false, false, None::<u64>)
        }
        Adt3::C28 => {
            Adt7::C0(f50(Str::lit("format_decimal")), false, false, None::<u64>)
        }
        Adt3::C29 => {
            Adt7::C0(f50(Str::lit("map_insert")), true, false, None::<u64>)
        }
        Adt3::C30 => {
            Adt7::C0(f50(Str::lit("map_remove")), true, false, None::<u64>)
        }
        Adt3::C31 => {
            Adt7::C0(f50(Str::lit("map_lookup")), true, false, None::<u64>)
        }
        Adt3::C32 => {
            Adt7::C0(f50(Str::lit("map_contains")), true, false, None::<u64>)
        }
        Adt3::C33 => {
            Adt7::C0(f50(Str::lit("map_size")), true, false, None::<u64>)
        }
        Adt3::C34 => {
            Adt7::C0(f50(Str::lit("map_keys")), true, false, None::<u64>)
        }
        Adt3::C35 => {
            Adt7::C0(f50(Str::lit("map_values")), true, false, None::<u64>)
        }
        Adt3::C36 => {
            Adt7::C0(f50(Str::lit("map_entries")), true, false, None::<u64>)
        }
        Adt3::C37 => {
            Adt7::C0(f50(Str::lit("map_fold")), true, false, Some(0u64))
        }
        Adt3::C38 => {
            Adt7::C0(f50(Str::lit("set_insert")), true, false, None::<u64>)
        }
        Adt3::C39 => {
            Adt7::C0(f50(Str::lit("set_remove")), true, false, None::<u64>)
        }
        Adt3::C40 => {
            Adt7::C0(f50(Str::lit("set_contains")), true, false, None::<u64>)
        }
        Adt3::C41 => {
            Adt7::C0(f50(Str::lit("set_size")), true, false, None::<u64>)
        }
        Adt3::C42 => {
            Adt7::C0(f50(Str::lit("set_elements")), true, false, None::<u64>)
        }
        Adt3::C43 => {
            Adt7::C0(f50(Str::lit("set_union")), true, false, None::<u64>)
        }
        Adt3::C44 => {
            Adt7::C0(f50(Str::lit("set_intersection")), true, false, None::<u64>)
        }
        Adt3::C45 => {
            Adt7::C0(f50(Str::lit("set_difference")), true, false, None::<u64>)
        }
        Adt3::C46 => {
            Adt7::C0(f50(Str::lit("set_fold")), true, false, Some(0u64))
        }
        Adt3::C47 => {
            Adt7::C0(f50(Str::lit("list_fold")), true, false, Some(0u64))
        }
        Adt3::C48 => {
            Adt7::C0(f50(Str::lit("iterate")), true, false, Some(0u64))
        }
        Adt3::C49 => {
            Adt7::C0(f50(Str::lit("iterate_until")), true, false, Some(0u64))
        }
        Adt3::C50 => {
            Adt7::C0(f50(Str::lit("graph_successors")), true, false, None::<u64>)
        }
        Adt3::C51 => {
            Adt7::C0(f50(Str::lit("graph_reachable")), true, false, None::<u64>)
        }
        Adt3::C52 => {
            Adt7::C0(f50(Str::lit("graph_topological")), true, false, None::<u64>)
        }
        Adt3::C53 => {
            Adt7::C0(f50(Str::lit("less_than")), true, false, None::<u64>)
        }
        Adt3::C54 => {
            Adt7::C0(f50(Str::lit("to_int")), true, true, None::<u64>)
        }
        Adt3::C55 => {
            Adt7::C0(f50(Str::lit("to_nat")), true, true, None::<u64>)
        }
    }
}

pub fn f26(v0: Bytes, v1: Adt4) -> Adt6 {
    Adt6::C4(std::rc::Rc::new(f5(Str::lit("name"), Adt6::C2(v0), f5(Str::lit("type"), f12(v1), Adt6::C5))))
}

pub fn f27(v0: List<(Bytes, Adt4)>) -> Adt6 {
    Adt6::C3(std::rc::Rc::new(f20(f51(Fn8::F52, List::<Adt6>::nil(), f53(v0)))))
}

pub fn f28(v0: Fn1, p143: List<Adt6>, p144: List<Bytes>) -> List<Adt6> {
    let m142 = (p143, p144);
    {
        let mut m142 = m142;
        loop {
            let step = {
                let (v1, v2) = m142;
                let m140 = v2;
                match m140.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Bytes>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Bytes>), List<Adt6>>(({
                            let c141: Fn1 = v0.clone();
                            c141.apply(v1, v3)
                        }, v4))
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

pub fn f29(v0: List<Adt6>, v1: Bytes) -> List<Adt6> {
    List::cons(Adt6::C2(v1), v0)
}

pub fn f30(v0: List<Bytes>) -> List<Bytes> {
    f54(Fn9::F55, List::<Bytes>::nil(), v0)
}

pub fn f31(v0: Fn2, p148: List<Adt6>, p149: List<(Bytes, Bytes)>) -> List<Adt6> {
    let m147 = (p148, p149);
    {
        let mut m147 = m147;
        loop {
            let step = {
                let (v1, v2) = m147;
                let m145 = v2;
                match m145.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<(Bytes, Bytes)>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<(Bytes, Bytes)>), List<Adt6>>(({
                            let c146: Fn2 = v0.clone();
                            c146.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: List<Adt6>, v1: (Bytes, Bytes)) -> List<Adt6> {
    List::cons(Adt6::C4(std::rc::Rc::new(f5(Str::lit("field"), Adt6::C2({
        let (h150, _) = v1;
        h150
    }), f5(Str::lit("expected"), Adt6::C2({
        let (_, h151) = v1;
        h151
    }), Adt6::C5)))), v0)
}

pub fn f33(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f40(Fn4::F56, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f34(v0: Str, v1: Adt6) -> Adt6 {
    Adt6::C4(std::rc::Rc::new(f35(Str::lit("kind"), Adt6::C2(f50(v0)), v1)))
}

pub fn f35(v0: Str, v1: Adt6, v2: Adt6) -> Adt6 {
    Adt6::C7(f50(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f36(v0: u64) -> R<Bytes> {
    f57(f58(), 10u64, v0)
}

pub fn f37(v0: Fn3, p156: u64, p157: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m155 = (p156, p157);
    {
        let mut m155 = m155;
        loop {
            let step = {
                let (v1, v2) = m155;
                let m152 = v1;
                if m152 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m152 - 1;
                    let m154 = {
                        let c153: Fn3 = v0.clone();
                        c153.apply(v2.clone())?
                    };
                    match m154 {
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
                Ok(next) => { m155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f59(v0, v1)
}

pub fn f39(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m158 = f60(Fn10::F61(v1), None::<Bytes>, v0);
    match m158 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40(v0: Fn4, p162: List<(Bytes, Bytes)>, p163: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v1, v2) = m161;
                let m159 = v2;
                match m159.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c160: Fn4 = v0.clone();
                            c160.apply(v1, v3)
                        }, v4))
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

pub fn f41(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f62(v0, v1)
}

pub fn f42(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f63(v0)
}

pub fn f43(v0: Fn5, p167: List<Bytes>, p168: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m166 = (p167, p168);
    {
        let mut m166 = m166;
        loop {
            let step = {
                let (v1, v2) = m166;
                let m164 = v2;
                match m164.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c165: Fn5 = v0.clone();
                            c165.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f64(v1)?, v0))
}

pub fn f45(v0: Fn6, p172: Adt6, p173: List<Adt6>) -> Adt6 {
    let m171 = (p172, p173);
    {
        let mut m171 = m171;
        loop {
            let step = {
                let (v1, v2) = m171;
                let m169 = v2;
                match m169.uncons() {
                    None => {
                        Err::<(Adt6, List<Adt6>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<Adt6>), Adt6>(({
                            let c170: Fn6 = v0.clone();
                            c170.apply(v1, v3)
                        }, v4))
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

pub fn f46(v0: Adt6, v1: Adt6) -> Adt6 {
    Adt6::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f47(v0: List<Adt6>) -> List<Adt6> {
    f65(Fn11::F66, List::<Adt6>::nil(), v0)
}

pub fn f48(v0: Fn7, p177: List<(Option<Bytes>, Bytes)>, p178: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    let m176 = (p177, p178);
    {
        let mut m176 = m176;
        loop {
            let step = {
                let (v1, v2) = m176;
                let m174 = v2;
                match m174.uncons() {
                    None => {
                        Err::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(({
                            let c175: Fn7 = v0.clone();
                            c175.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<(Option<Bytes>, Bytes)>, v1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
    List::cons(v1, v0)
}

pub fn f50(v0: Str) -> Bytes {
    let a179 = v0;
    utf8_encode(a179)
}

pub fn f51(v0: Fn8, p183: List<Adt6>, p184: List<(Bytes, Adt4)>) -> List<Adt6> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v1, v2) = m182;
                let m180 = v2;
                match m180.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<(Bytes, Adt4)>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<(Bytes, Adt4)>), List<Adt6>>(({
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

pub fn f52(v0: List<Adt6>, v1: (Bytes, Adt4)) -> List<Adt6> {
    List::cons(f26({
        let (h185, _) = v1;
        h185
    }, {
        let (_, h186) = v1;
        h186
    }), v0)
}

pub fn f53(v0: List<(Bytes, Adt4)>) -> List<(Bytes, Adt4)> {
    f67(Fn12::F68, List::<(Bytes, Adt4)>::nil(), v0)
}

pub fn f54(v0: Fn9, p190: List<Bytes>, p191: List<Bytes>) -> List<Bytes> {
    let m189 = (p190, p191);
    {
        let mut m189 = m189;
        loop {
            let step = {
                let (v1, v2) = m189;
                let m187 = v2;
                match m187.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c188: Fn9 = v0.clone();
                            c188.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m189 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f56(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f57(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a192 = v2;
    let a193 = 0u64;
    if nat_eq(a192, a193) {
        Ok(f69(v0, 0u64))
    } else {
        let (_, h194) = {
            let (h195, _) = f37(Fn3::F70(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h195
        };
        Ok(h194)
    }
}

pub fn f58() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f59(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m199 = {
        let a197 = v0.clone();
        let a198 = {
            let (h196, _) = v1.clone();
            h196
        };
        index_bytes(a197, a198)
    };
    match m199 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a201 = {
                    let (h200, _) = v1.clone();
                    h200
                };
                let a202 = 1u64;
                nat_add(a201, a202)?
            }, {
                let a205 = {
                    let (_, h203) = v1;
                    h203
                };
                let a206 = f71(v0, {
                    let (h204, _) = v1;
                    h204
                }, v2)?;
                append_bytes(a205, a206)
            })))
        }
    }
}

pub fn f60(v0: Fn10, p210: Option<Bytes>, p211: List<Bytes>) -> Option<Bytes> {
    let m209 = (p210, p211);
    {
        let mut m209 = m209;
        loop {
            let step = {
                let (v1, v2) = m209;
                let m207 = v2;
                match m207.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c208: Fn10 = v0.clone();
                            c208.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m212 = v1;
    match m212 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a215 = {
                    let a213 = v3;
                    let a214 = v0;
                    append_bytes(a213, a214)
                };
                let a216 = v2;
                append_bytes(a215, a216)
            })
        }
    }
}

pub fn f62(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m217 = v0.clone();
    match m217.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a220 = {
                let (h218, _) = v2;
                h218
            };
            let a221 = {
                let (h219, _) = v1.clone();
                h219
            };
            if equal(a220, a221) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f63(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f72(f73(Fn13::F74, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m226 = {
        let (h222, _) = f75(Fn14::F76, {
            let a224 = {
                let a223 = v0;
                length_list(a223)
            };
            let a225 = 1u64;
            nat_add(a224, a225)?
        }, v1)?;
        h222
    };
    match m226.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f64(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a231 = {
        let a228 = f14({
            let (h227, _) = v0;
            h227
        })?;
        let a229 = Bytes::lit(&[0x3a]);
        append_bytes(a228, a229)
    };
    let a232 = {
        let (_, h230) = v0;
        h230
    };
    Ok(append_bytes(a231, a232))
}

pub fn f65(v0: Fn11, p236: List<Adt6>, p237: List<Adt6>) -> List<Adt6> {
    let m235 = (p236, p237);
    {
        let mut m235 = m235;
        loop {
            let step = {
                let (v1, v2) = m235;
                let m233 = v2;
                match m233.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>(({
                            let c234: Fn11 = v0.clone();
                            c234.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(v1, v0)
}

pub fn f67(v0: Fn12, p241: List<(Bytes, Adt4)>, p242: List<(Bytes, Adt4)>) -> List<(Bytes, Adt4)> {
    let m240 = (p241, p242);
    {
        let mut m240 = m240;
        loop {
            let step = {
                let (v1, v2) = m240;
                let m238 = v2;
                match m238.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt4)>, List<(Bytes, Adt4)>), List<(Bytes, Adt4)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt4)>, List<(Bytes, Adt4)>), List<(Bytes, Adt4)>>(({
                            let c239: Fn12 = v0.clone();
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

pub fn f68(v0: List<(Bytes, Adt4)>, v1: (Bytes, Adt4)) -> List<(Bytes, Adt4)> {
    List::cons(v1, v0)
}

pub fn f69(v0: Bytes, v1: u64) -> Bytes {
    let m246 = {
        let a243 = v0;
        let a244 = v1;
        let a245 = 1u64;
        slice_bytes(a243, a244, a245)
    };
    match m246 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f70(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f77(v1, v0, v2)
}

pub fn f71(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a291 = {
            let a267 = {
                let a255 = {
                    let a249 = {
                        let a247 = v3;
                        let a248 = 128u8;
                        fixed_u8::bit_and(a247, a248)
                    };
                    let a250 = 128u8;
                    if equal(a249, a250) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a256 = {
                    let a253 = {
                        let a251 = v3;
                        let a252 = 64u8;
                        fixed_u8::bit_and(a251, a252)
                    };
                    let a254 = 64u8;
                    if equal(a253, a254) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a255, a256)?
            };
            let a268 = {
                let a265 = {
                    let a259 = {
                        let a257 = v3;
                        let a258 = 32u8;
                        fixed_u8::bit_and(a257, a258)
                    };
                    let a260 = 32u8;
                    if equal(a259, a260) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a266 = {
                    let a263 = {
                        let a261 = v3;
                        let a262 = 16u8;
                        fixed_u8::bit_and(a261, a262)
                    };
                    let a264 = 16u8;
                    if equal(a263, a264) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a265, a266)?
            };
            nat_add(a267, a268)?
        };
        let a292 = {
            let a289 = {
                let a277 = {
                    let a271 = {
                        let a269 = v3;
                        let a270 = 8u8;
                        fixed_u8::bit_and(a269, a270)
                    };
                    let a272 = 8u8;
                    if equal(a271, a272) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a278 = {
                    let a275 = {
                        let a273 = v3;
                        let a274 = 4u8;
                        fixed_u8::bit_and(a273, a274)
                    };
                    let a276 = 4u8;
                    if equal(a275, a276) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a277, a278)?
            };
            let a290 = {
                let a287 = {
                    let a281 = {
                        let a279 = v3;
                        let a280 = 2u8;
                        fixed_u8::bit_and(a279, a280)
                    };
                    let a282 = 2u8;
                    if equal(a281, a282) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a288 = {
                    let a285 = {
                        let a283 = v3;
                        let a284 = 1u8;
                        fixed_u8::bit_and(a283, a284)
                    };
                    let a286 = 1u8;
                    if equal(a285, a286) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a287, a288)?
            };
            nat_add(a289, a290)?
        };
        nat_add(a291, a292)?
    };
    let a293 = v4;
    let a294 = 34u64;
    if nat_eq(a293, a294) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a295 = v4;
        let a296 = 92u64;
        if nat_eq(a295, a296) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a297 = v4;
            let a298 = 8u64;
            if nat_eq(a297, a298) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a299 = v4;
                let a300 = 12u64;
                if nat_eq(a299, a300) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a301 = v4;
                    let a302 = 10u64;
                    if nat_eq(a301, a302) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a303 = v4;
                        let a304 = 13u64;
                        if nat_eq(a303, a304) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a305 = v4;
                            let a306 = 9u64;
                            if nat_eq(a305, a306) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a307 = v4;
                                let a308 = 32u64;
                                if nat_lt(a307, a308) {
                                    let a309 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a310 = f78(v2)?;
                                    Ok(append_bytes(a309, a310))
                                } else {
                                    Ok(f79(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f72(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f80(Fn15::F81, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f73(v0: Fn13, p314: List<List<(Bytes, Bytes)>>, p315: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m313 = (p314, p315);
    {
        let mut m313 = m313;
        loop {
            let step = {
                let (v1, v2) = m313;
                let m311 = v2;
                match m311.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c312: Fn13 = v0.clone();
                            c312.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f75(v0: Fn14, p320: u64, p321: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m319 = (p320, p321);
    {
        let mut m319 = m319;
        loop {
            let step = {
                let (v1, v2) = m319;
                let m316 = v1;
                if m316 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m316 - 1;
                    let m318 = {
                        let c317: Fn14 = v0.clone();
                        c317.apply(v2.clone())?
                    };
                    match m318 {
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
                Ok(next) => { m319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f82(v0)
}

pub fn f77(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h322, _) = v2;
        h322
    };
    let a323 = v3;
    let a324 = 0u64;
    if nat_eq(a323, a324) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a325 = v3;
            let a326 = v1;
            let a327 = 0u64;
            nat_quot(a325, a326, a327)
        }, {
            let a332 = f69(v0, {
                let a328 = v3;
                let a329 = v1;
                let a330 = 0u64;
                nat_rem(a328, a329, a330)
            });
            let a333 = {
                let (_, h331) = v2;
                h331
            };
            append_bytes(a332, a333)
        }))
    }
}

pub fn f78(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a378 = {
            let a354 = {
                let a342 = {
                    let a336 = {
                        let a334 = v1;
                        let a335 = 128u8;
                        fixed_u8::bit_and(a334, a335)
                    };
                    let a337 = 128u8;
                    if equal(a336, a337) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a343 = {
                    let a340 = {
                        let a338 = v1;
                        let a339 = 64u8;
                        fixed_u8::bit_and(a338, a339)
                    };
                    let a341 = 64u8;
                    if equal(a340, a341) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a342, a343)?
            };
            let a355 = {
                let a352 = {
                    let a346 = {
                        let a344 = v1;
                        let a345 = 32u8;
                        fixed_u8::bit_and(a344, a345)
                    };
                    let a347 = 32u8;
                    if equal(a346, a347) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a353 = {
                    let a350 = {
                        let a348 = v1;
                        let a349 = 16u8;
                        fixed_u8::bit_and(a348, a349)
                    };
                    let a351 = 16u8;
                    if equal(a350, a351) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a352, a353)?
            };
            nat_add(a354, a355)?
        };
        let a379 = {
            let a376 = {
                let a364 = {
                    let a358 = {
                        let a356 = v1;
                        let a357 = 8u8;
                        fixed_u8::bit_and(a356, a357)
                    };
                    let a359 = 8u8;
                    if equal(a358, a359) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a365 = {
                    let a362 = {
                        let a360 = v1;
                        let a361 = 4u8;
                        fixed_u8::bit_and(a360, a361)
                    };
                    let a363 = 4u8;
                    if equal(a362, a363) {
                        4u64
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
                        let a367 = 2u8;
                        fixed_u8::bit_and(a366, a367)
                    };
                    let a369 = 2u8;
                    if equal(a368, a369) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a375 = {
                    let a372 = {
                        let a370 = v1;
                        let a371 = 1u8;
                        fixed_u8::bit_and(a370, a371)
                    };
                    let a373 = 1u8;
                    if equal(a372, a373) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a374, a375)?
            };
            nat_add(a376, a377)?
        };
        nat_add(a378, a379)?
    };
    let a386 = f69(f83(), {
        let a380 = v2;
        let a381 = 16u64;
        let a382 = 0u64;
        nat_quot(a380, a381, a382)
    });
    let a387 = f69(f83(), {
        let a383 = v2;
        let a384 = 16u64;
        let a385 = 0u64;
        nat_rem(a383, a384, a385)
    });
    Ok(append_bytes(a386, a387))
}

pub fn f79(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m391 = {
        let a388 = v0;
        let a389 = v1;
        let a390 = v2;
        slice_bytes(a388, a389, a390)
    };
    match m391 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f80(v0: Fn15, p395: List<List<(Bytes, Bytes)>>, p396: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v1, v2) = m394;
                let m392 = v2;
                match m392.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c393: Fn15 = v0.clone();
                            c393.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f82(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a398 = {
        let a397 = v0.clone();
        length_list(a397)
    };
    let a399 = 1u64;
    if nat_le(a398, a399) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f84(v0)?))
    }
}

pub fn f83() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f84(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f72({
        let (_, h400) = {
            let (h401, _) = f85(Fn16::F86, {
                let a403 = {
                    let a402 = v0.clone();
                    length_list(a402)
                };
                let a404 = 1u64;
                nat_add(a403, a404)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h401
        };
        h400
    }))
}

pub fn f85(v0: Fn16, p409: u64, p410: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m408 = (p409, p410);
    {
        let mut m408 = m408;
        loop {
            let step = {
                let (v1, v2) = m408;
                let m405 = v1;
                if m405 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m405 - 1;
                    let m407 = {
                        let c406: Fn16 = v0.clone();
                        c406.apply(v2.clone())?
                    };
                    match m407 {
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
                Ok(next) => { m408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f87(v0)
}

pub fn f87(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h411, _) = v0;
        h411
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h412) = v0;
        h412
    };
    let m413 = v1;
    match m413.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m414 = v4;
            match m414.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f88(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f88(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a419 = {
            let a417 = {
                let a415 = v0.clone();
                length_list(a415)
            };
            let a418 = {
                let a416 = v1.clone();
                length_list(a416)
            };
            nat_add(a417, a418)?
        };
        let a420 = 1u64;
        nat_add(a419, a420)?
    };
    Ok(f33({
        let (_, h421) = {
            let (_, h422) = {
                let (h423, _) = f89(Fn17::F90, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h423
            };
            h422
        };
        h421
    }))
}

pub fn f89(v0: Fn17, p428: u64, p429: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m427 = (p428, p429);
    {
        let mut m427 = m427;
        loop {
            let step = {
                let (v1, v2) = m427;
                let m424 = v1;
                if m424 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m424 - 1;
                    let m426 = {
                        let c425: Fn17 = v0.clone();
                        c425.apply(v2.clone())
                    };
                    match m426 {
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
                Ok(next) => { m427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f91(v0)
}

pub fn f91(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h430, _) = v0.clone();
        h430
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h431, _) = {
            let (_, h432) = v0.clone();
            h432
        };
        h431
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h433) = {
            let (_, h434) = v0;
            h434
        };
        h433
    };
    let m435 = v1.clone();
    match m435.uncons() {
        None => {
            let m436 = v2;
            match m436.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m437 = v2.clone();
            match m437.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f92({
                        let (h438, _) = v8.clone();
                        h438
                    }, {
                        let (h439, _) = v6.clone();
                        h439
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

pub fn f92(v0: Bytes, v1: Bytes) -> bool {
    let a444 = {
        let a440 = v0;
        let a441 = v1;
        compare_bytes(a440, a441)
    };
    let a445 = {
        let a442 = Bytes::lit(&[]);
        let a443 = Bytes::lit(&[0x01]);
        compare_bytes(a442, a443)
    };
    equal(a444, a445)
}
