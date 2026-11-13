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
    C20(std::rc::Rc<Adt0>),
    C21(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C22(std::rc::Rc<Adt0>),
    C23(Option<Bytes>, Bytes, std::rc::Rc<Adt0>),
    C24(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C25(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C26(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C27(std::rc::Rc<Adt0>),
    C28,
    C29(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt1>),
    C4(std::rc::Rc<Adt1>),
    C5,
    C6(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C7(Bytes, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Fn0 {
    F15(Bytes),
    F31(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F15(k0) => f15(k0.clone(), p0),
            Fn0::F31(k0, k1) => Ok(f31(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18,
    F53,
}

impl Fn1 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn1::F18 => f18(p0, p1),
            Fn1::F53 => f53(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn2::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F26(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn3::F26(k0) => f26(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F35,
}

impl Fn4 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn4::F35 => f35(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F37,
}

impl Fn5 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn5::F37 => f37(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F42,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn6::F42 => f42(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F47,
}

impl Fn7 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn7::F47 => f47(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F52,
}

impl Fn8 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn8::F52 => f52(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0))
}

pub fn f1(v0: Adt1) -> R<Bytes> {
    f3(v0)
}

pub fn f2(v0: Adt0) -> Adt1 {
    let m1 = v0;
    match m1 {
        Adt0::C0 => {
            f4(Str::lit("type"), Adt1::C5)
        }
        Adt0::C1 => {
            f4(Str::lit("nat"), Adt1::C5)
        }
        Adt0::C2 => {
            f4(Str::lit("bool"), Adt1::C5)
        }
        Adt0::C3 => {
            f4(Str::lit("prop"), Adt1::C5)
        }
        Adt0::C4 => {
            f4(Str::lit("unit"), Adt1::C5)
        }
        Adt0::C5 => {
            f4(Str::lit("int"), Adt1::C5)
        }
        Adt0::C6 => {
            f4(Str::lit("int8"), Adt1::C5)
        }
        Adt0::C7 => {
            f4(Str::lit("int16"), Adt1::C5)
        }
        Adt0::C8 => {
            f4(Str::lit("int32"), Adt1::C5)
        }
        Adt0::C9 => {
            f4(Str::lit("int64"), Adt1::C5)
        }
        Adt0::C10 => {
            f4(Str::lit("uint8"), Adt1::C5)
        }
        Adt0::C11 => {
            f4(Str::lit("uint16"), Adt1::C5)
        }
        Adt0::C12 => {
            f4(Str::lit("uint32"), Adt1::C5)
        }
        Adt0::C13 => {
            f4(Str::lit("uint64"), Adt1::C5)
        }
        Adt0::C14 => {
            f4(Str::lit("string"), Adt1::C5)
        }
        Adt0::C15 => {
            f4(Str::lit("bytes"), Adt1::C5)
        }
        Adt0::C16 => {
            f4(Str::lit("ordering"), Adt1::C5)
        }
        Adt0::C17 => {
            f4(Str::lit("contract_violation"), Adt1::C5)
        }
        Adt0::C18 => {
            f4(Str::lit("reasoning_failure"), Adt1::C5)
        }
        Adt0::C19(v1) => {
            f4(Str::lit("parameter"), f5(Str::lit("name"), Adt1::C2(v1), Adt1::C5))
        }
        Adt0::C20(r3) => {
            let v2 = (*r3).clone();
            f4(Str::lit("option"), f5(Str::lit("value"), f2(v2), Adt1::C5))
        }
        Adt0::C21(r4, r5) => {
            let v3 = (*r4).clone();
            let v4 = (*r5).clone();
            f4(Str::lit("result"), f5(Str::lit("ok"), f2(v3), f5(Str::lit("error"), f2(v4), Adt1::C5)))
        }
        Adt0::C22(r6) => {
            let v5 = (*r6).clone();
            f4(Str::lit("list"), f5(Str::lit("element"), f2(v5), Adt1::C5))
        }
        Adt0::C23(v6, v7, r7) => {
            let v8 = (*r7).clone();
            f4(Str::lit("named"), f5(Str::lit("member"), Adt1::C4(std::rc::Rc::new({
                let m2 = v6;
                match m2 {
                    None => {
                        f5(Str::lit("name"), Adt1::C2(v7), Adt1::C5)
                    }
                    Some(v9) => {
                        f5(Str::lit("module"), Adt1::C2(v9), f5(Str::lit("name"), Adt1::C2(v7), Adt1::C5))
                    }
                }
            })), f5(Str::lit("arguments"), Adt1::C3(std::rc::Rc::new(f2(v8))), Adt1::C5)))
        }
        Adt0::C24(r8, r9) => {
            let v10 = (*r8).clone();
            let v11 = (*r9).clone();
            f4(Str::lit("product"), f5(Str::lit("left"), f2(v10), f5(Str::lit("right"), f2(v11), Adt1::C5)))
        }
        Adt0::C25(r10, r11) => {
            let v12 = (*r10).clone();
            let v13 = (*r11).clone();
            f4(Str::lit("function"), f5(Str::lit("parameters"), Adt1::C3(std::rc::Rc::new(f2(v12))), f5(Str::lit("result"), f2(v13), Adt1::C5)))
        }
        Adt0::C26(r12, r13) => {
            let v14 = (*r12).clone();
            let v15 = (*r13).clone();
            f4(Str::lit("map"), f5(Str::lit("key"), f2(v14), f5(Str::lit("value"), f2(v15), Adt1::C5)))
        }
        Adt0::C27(r14) => {
            let v16 = (*r14).clone();
            f4(Str::lit("set"), f5(Str::lit("element"), f2(v16), Adt1::C5))
        }
        Adt0::C28 => {
            Adt1::C5
        }
        Adt0::C29(r15, r16) => {
            let v17 = (*r15).clone();
            let v18 = (*r16).clone();
            Adt1::C6(std::rc::Rc::new(f2(v17)), std::rc::Rc::new(f2(v18)))
        }
    }
}

pub fn f3(v0: Adt1) -> R<Bytes> {
    let m17 = v0;
    match m17 {
        Adt1::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt1::C1(v2, v3) => {
            f6(v2, v3)
        }
        Adt1::C2(v4) => {
            f7(v4)
        }
        Adt1::C3(r18) => {
            let v5 = (*r18).clone();
            Ok(f8(f9(v5)?))
        }
        Adt1::C4(r19) => {
            let v6 = (*r19).clone();
            f10(f11(v6)?)
        }
        Adt1::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt1::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt1::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Str, v1: Adt1) -> Adt1 {
    Adt1::C4(std::rc::Rc::new(f5(Str::lit("kind"), Adt1::C2(f12(v0)), v1)))
}

pub fn f5(v0: Str, v1: Adt1, v2: Adt1) -> Adt1 {
    Adt1::C7(f12(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f6(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a20 = 0u64;
        let a21 = v1;
        nat_lt(a20, a21)
    } else {
        false
    } {
        let a22 = Bytes::lit(&[0x2d]);
        let a23 = f13(v1)?;
        Ok(append_bytes(a22, a23))
    } else {
        f13(v1)
    }
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    let a29 = {
        let a27 = Bytes::lit(&[0x22]);
        let a28 = {
            let (_, h24) = {
                let (h25, _) = f14(Fn0::F15(v0.clone()), {
                    let a26 = v0;
                    length_bytes(a26)
                }, (0u64, Bytes::lit(&[])))?;
                h25
            };
            h24
        };
        append_bytes(a27, a28)
    };
    let a30 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a29, a30))
}

pub fn f8(v0: List<Bytes>) -> Bytes {
    let a33 = {
        let a31 = Bytes::lit(&[0x5b]);
        let a32 = f16(v0, Bytes::lit(&[0x2c]));
        append_bytes(a31, a32)
    };
    let a34 = Bytes::lit(&[0x5d]);
    append_bytes(a33, a34)
}

pub fn f9(v0: Adt1) -> R<List<Bytes>> {
    let m35 = v0;
    match m35 {
        Adt1::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt1::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt1::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt1::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt1::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt1::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt1::C6(r36, r37) => {
            let v7 = (*r36).clone();
            let v8 = (*r37).clone();
            Ok(List::cons(f3(v7)?, f9(v8)?))
        }
        Adt1::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f10(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f17(Fn1::F18, List::<(Bytes, Bytes)>::nil(), f19(v0)?);
    let v2: List<Bytes> = f20(Fn2::F21, List::<Bytes>::nil(), v1)?;
    let a40 = {
        let a38 = Bytes::lit(&[0x7b]);
        let a39 = f16(v2, Bytes::lit(&[0x2c]));
        append_bytes(a38, a39)
    };
    let a41 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a40, a41))
}

pub fn f11(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m42 = v0;
    match m42 {
        Adt1::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt1::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt1::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt1::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt1::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt1::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt1::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt1::C7(v9, r43, r44) => {
            let v10 = (*r43).clone();
            let v11 = (*r44).clone();
            Ok(List::cons((v9, f3(v10)?), f11(v11)?))
        }
    }
}

pub fn f12(v0: Str) -> Bytes {
    let a45 = v0;
    utf8_encode(a45)
}

pub fn f13(v0: u64) -> R<Bytes> {
    f22(f23(), 10u64, v0)
}

pub fn f14(v0: Fn0, p50: u64, p51: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v1, v2) = m49;
                let m46 = v1;
                if m46 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m46 - 1;
                    let m48 = {
                        let c47: Fn0 = v0.clone();
                        c47.apply(v2.clone())?
                    };
                    match m48 {
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
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f24(v0, v1)
}

pub fn f16(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m52 = f25(Fn3::F26(v1), None::<Bytes>, v0);
    match m52 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f17(v0: Fn1, p56: List<(Bytes, Bytes)>, p57: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m55 = (p56, p57);
    {
        let mut m55 = m55;
        loop {
            let step = {
                let (v1, v2) = m55;
                let m53 = v2;
                match m53.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c54: Fn1 = v0.clone();
                            c54.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m55 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f27(v0, v1)
}

pub fn f19(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f28(v0)
}

pub fn f20(v0: Fn2, p61: List<Bytes>, p62: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m60 = (p61, p62);
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v1, v2) = m60;
                let m58 = v2;
                match m58.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c59: Fn2 = v0.clone();
                            c59.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f29(v1)?, v0))
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a63 = v2;
    let a64 = 0u64;
    if nat_eq(a63, a64) {
        Ok(f30(v0, 0u64))
    } else {
        let (_, h65) = {
            let (h66, _) = f14(Fn0::F31(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h66
        };
        Ok(h65)
    }
}

pub fn f23() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f24(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m70 = {
        let a68 = v0.clone();
        let a69 = {
            let (h67, _) = v1.clone();
            h67
        };
        index_bytes(a68, a69)
    };
    match m70 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a72 = {
                    let (h71, _) = v1.clone();
                    h71
                };
                let a73 = 1u64;
                nat_add(a72, a73)?
            }, {
                let a76 = {
                    let (_, h74) = v1;
                    h74
                };
                let a77 = f32(v0, {
                    let (h75, _) = v1;
                    h75
                }, v2)?;
                append_bytes(a76, a77)
            })))
        }
    }
}

pub fn f25(v0: Fn3, p81: Option<Bytes>, p82: List<Bytes>) -> Option<Bytes> {
    let m80 = (p81, p82);
    {
        let mut m80 = m80;
        loop {
            let step = {
                let (v1, v2) = m80;
                let m78 = v2;
                match m78.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c79: Fn3 = v0.clone();
                            c79.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m80 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m83 = v1;
    match m83 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a86 = {
                    let a84 = v3;
                    let a85 = v0;
                    append_bytes(a84, a85)
                };
                let a87 = v2;
                append_bytes(a86, a87)
            })
        }
    }
}

pub fn f27(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m88 = v0.clone();
    match m88.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a91 = {
                let (h89, _) = v2;
                h89
            };
            let a92 = {
                let (h90, _) = v1.clone();
                h90
            };
            if equal(a91, a92) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f28(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f33(f34(Fn4::F35, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m97 = {
        let (h93, _) = f36(Fn5::F37, {
            let a95 = {
                let a94 = v0;
                length_list(a94)
            };
            let a96 = 1u64;
            nat_add(a95, a96)?
        }, v1)?;
        h93
    };
    match m97.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f29(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a102 = {
        let a99 = f7({
            let (h98, _) = v0;
            h98
        })?;
        let a100 = Bytes::lit(&[0x3a]);
        append_bytes(a99, a100)
    };
    let a103 = {
        let (_, h101) = v0;
        h101
    };
    Ok(append_bytes(a102, a103))
}

pub fn f30(v0: Bytes, v1: u64) -> Bytes {
    let m107 = {
        let a104 = v0;
        let a105 = v1;
        let a106 = 1u64;
        slice_bytes(a104, a105, a106)
    };
    match m107 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f31(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f38(v1, v0, v2)
}

pub fn f32(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a152 = {
            let a128 = {
                let a116 = {
                    let a110 = {
                        let a108 = v3;
                        let a109 = 128u8;
                        fixed_u8::bit_and(a108, a109)
                    };
                    let a111 = 128u8;
                    if equal(a110, a111) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a117 = {
                    let a114 = {
                        let a112 = v3;
                        let a113 = 64u8;
                        fixed_u8::bit_and(a112, a113)
                    };
                    let a115 = 64u8;
                    if equal(a114, a115) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a116, a117)?
            };
            let a129 = {
                let a126 = {
                    let a120 = {
                        let a118 = v3;
                        let a119 = 32u8;
                        fixed_u8::bit_and(a118, a119)
                    };
                    let a121 = 32u8;
                    if equal(a120, a121) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a127 = {
                    let a124 = {
                        let a122 = v3;
                        let a123 = 16u8;
                        fixed_u8::bit_and(a122, a123)
                    };
                    let a125 = 16u8;
                    if equal(a124, a125) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a126, a127)?
            };
            nat_add(a128, a129)?
        };
        let a153 = {
            let a150 = {
                let a138 = {
                    let a132 = {
                        let a130 = v3;
                        let a131 = 8u8;
                        fixed_u8::bit_and(a130, a131)
                    };
                    let a133 = 8u8;
                    if equal(a132, a133) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a139 = {
                    let a136 = {
                        let a134 = v3;
                        let a135 = 4u8;
                        fixed_u8::bit_and(a134, a135)
                    };
                    let a137 = 4u8;
                    if equal(a136, a137) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a138, a139)?
            };
            let a151 = {
                let a148 = {
                    let a142 = {
                        let a140 = v3;
                        let a141 = 2u8;
                        fixed_u8::bit_and(a140, a141)
                    };
                    let a143 = 2u8;
                    if equal(a142, a143) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a149 = {
                    let a146 = {
                        let a144 = v3;
                        let a145 = 1u8;
                        fixed_u8::bit_and(a144, a145)
                    };
                    let a147 = 1u8;
                    if equal(a146, a147) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a148, a149)?
            };
            nat_add(a150, a151)?
        };
        nat_add(a152, a153)?
    };
    let a154 = v4;
    let a155 = 34u64;
    if nat_eq(a154, a155) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a156 = v4;
        let a157 = 92u64;
        if nat_eq(a156, a157) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a158 = v4;
            let a159 = 8u64;
            if nat_eq(a158, a159) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a160 = v4;
                let a161 = 12u64;
                if nat_eq(a160, a161) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a162 = v4;
                    let a163 = 10u64;
                    if nat_eq(a162, a163) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a164 = v4;
                        let a165 = 13u64;
                        if nat_eq(a164, a165) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a166 = v4;
                            let a167 = 9u64;
                            if nat_eq(a166, a167) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a168 = v4;
                                let a169 = 32u64;
                                if nat_lt(a168, a169) {
                                    let a170 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a171 = f39(v2)?;
                                    Ok(append_bytes(a170, a171))
                                } else {
                                    Ok(f40(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f33(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f41(Fn6::F42, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f34(v0: Fn4, p175: List<List<(Bytes, Bytes)>>, p176: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m174 = (p175, p176);
    {
        let mut m174 = m174;
        loop {
            let step = {
                let (v1, v2) = m174;
                let m172 = v2;
                match m172.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c173: Fn4 = v0.clone();
                            c173.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f36(v0: Fn5, p181: u64, p182: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m180 = (p181, p182);
    {
        let mut m180 = m180;
        loop {
            let step = {
                let (v1, v2) = m180;
                let m177 = v1;
                if m177 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m177 - 1;
                    let m179 = {
                        let c178: Fn5 = v0.clone();
                        c178.apply(v2.clone())?
                    };
                    match m179 {
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
                Ok(next) => { m180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f43(v0)
}

pub fn f38(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h183, _) = v2;
        h183
    };
    let a184 = v3;
    let a185 = 0u64;
    if nat_eq(a184, a185) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a186 = v3;
            let a187 = v1;
            let a188 = 0u64;
            nat_quot(a186, a187, a188)
        }, {
            let a193 = f30(v0, {
                let a189 = v3;
                let a190 = v1;
                let a191 = 0u64;
                nat_rem(a189, a190, a191)
            });
            let a194 = {
                let (_, h192) = v2;
                h192
            };
            append_bytes(a193, a194)
        }))
    }
}

pub fn f39(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a239 = {
            let a215 = {
                let a203 = {
                    let a197 = {
                        let a195 = v1;
                        let a196 = 128u8;
                        fixed_u8::bit_and(a195, a196)
                    };
                    let a198 = 128u8;
                    if equal(a197, a198) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a204 = {
                    let a201 = {
                        let a199 = v1;
                        let a200 = 64u8;
                        fixed_u8::bit_and(a199, a200)
                    };
                    let a202 = 64u8;
                    if equal(a201, a202) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a203, a204)?
            };
            let a216 = {
                let a213 = {
                    let a207 = {
                        let a205 = v1;
                        let a206 = 32u8;
                        fixed_u8::bit_and(a205, a206)
                    };
                    let a208 = 32u8;
                    if equal(a207, a208) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a214 = {
                    let a211 = {
                        let a209 = v1;
                        let a210 = 16u8;
                        fixed_u8::bit_and(a209, a210)
                    };
                    let a212 = 16u8;
                    if equal(a211, a212) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a213, a214)?
            };
            nat_add(a215, a216)?
        };
        let a240 = {
            let a237 = {
                let a225 = {
                    let a219 = {
                        let a217 = v1;
                        let a218 = 8u8;
                        fixed_u8::bit_and(a217, a218)
                    };
                    let a220 = 8u8;
                    if equal(a219, a220) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a226 = {
                    let a223 = {
                        let a221 = v1;
                        let a222 = 4u8;
                        fixed_u8::bit_and(a221, a222)
                    };
                    let a224 = 4u8;
                    if equal(a223, a224) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a225, a226)?
            };
            let a238 = {
                let a235 = {
                    let a229 = {
                        let a227 = v1;
                        let a228 = 2u8;
                        fixed_u8::bit_and(a227, a228)
                    };
                    let a230 = 2u8;
                    if equal(a229, a230) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a236 = {
                    let a233 = {
                        let a231 = v1;
                        let a232 = 1u8;
                        fixed_u8::bit_and(a231, a232)
                    };
                    let a234 = 1u8;
                    if equal(a233, a234) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a235, a236)?
            };
            nat_add(a237, a238)?
        };
        nat_add(a239, a240)?
    };
    let a247 = f30(f44(), {
        let a241 = v2;
        let a242 = 16u64;
        let a243 = 0u64;
        nat_quot(a241, a242, a243)
    });
    let a248 = f30(f44(), {
        let a244 = v2;
        let a245 = 16u64;
        let a246 = 0u64;
        nat_rem(a244, a245, a246)
    });
    Ok(append_bytes(a247, a248))
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m252 = {
        let a249 = v0;
        let a250 = v1;
        let a251 = v2;
        slice_bytes(a249, a250, a251)
    };
    match m252 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f41(v0: Fn6, p256: List<List<(Bytes, Bytes)>>, p257: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m255 = (p256, p257);
    {
        let mut m255 = m255;
        loop {
            let step = {
                let (v1, v2) = m255;
                let m253 = v2;
                match m253.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c254: Fn6 = v0.clone();
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

pub fn f42(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f43(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a259 = {
        let a258 = v0.clone();
        length_list(a258)
    };
    let a260 = 1u64;
    if nat_le(a259, a260) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f45(v0)?))
    }
}

pub fn f44() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f45(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f33({
        let (_, h261) = {
            let (h262, _) = f46(Fn7::F47, {
                let a264 = {
                    let a263 = v0.clone();
                    length_list(a263)
                };
                let a265 = 1u64;
                nat_add(a264, a265)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h262
        };
        h261
    }))
}

pub fn f46(v0: Fn7, p270: u64, p271: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m269 = (p270, p271);
    {
        let mut m269 = m269;
        loop {
            let step = {
                let (v1, v2) = m269;
                let m266 = v1;
                if m266 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m266 - 1;
                    let m268 = {
                        let c267: Fn7 = v0.clone();
                        c267.apply(v2.clone())?
                    };
                    match m268 {
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
                Ok(next) => { m269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f48(v0)
}

pub fn f48(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h272, _) = v0;
        h272
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h273) = v0;
        h273
    };
    let m274 = v1;
    match m274.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m275 = v4;
            match m275.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f49(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f49(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a280 = {
            let a278 = {
                let a276 = v0.clone();
                length_list(a276)
            };
            let a279 = {
                let a277 = v1.clone();
                length_list(a277)
            };
            nat_add(a278, a279)?
        };
        let a281 = 1u64;
        nat_add(a280, a281)?
    };
    Ok(f50({
        let (_, h282) = {
            let (_, h283) = {
                let (h284, _) = f51(Fn8::F52, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h284
            };
            h283
        };
        h282
    }))
}

pub fn f50(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f17(Fn1::F53, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f51(v0: Fn8, p289: u64, p290: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m288 = (p289, p290);
    {
        let mut m288 = m288;
        loop {
            let step = {
                let (v1, v2) = m288;
                let m285 = v1;
                if m285 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m285 - 1;
                    let m287 = {
                        let c286: Fn8 = v0.clone();
                        c286.apply(v2.clone())
                    };
                    match m287 {
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
                Ok(next) => { m288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f54(v0)
}

pub fn f53(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f54(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h291, _) = v0.clone();
        h291
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h292, _) = {
            let (_, h293) = v0.clone();
            h293
        };
        h292
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h294) = {
            let (_, h295) = v0;
            h295
        };
        h294
    };
    let m296 = v1.clone();
    match m296.uncons() {
        None => {
            let m297 = v2;
            match m297.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m298 = v2.clone();
            match m298.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f55({
                        let (h299, _) = v8.clone();
                        h299
                    }, {
                        let (h300, _) = v6.clone();
                        h300
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

pub fn f55(v0: Bytes, v1: Bytes) -> bool {
    let a305 = {
        let a301 = v0;
        let a302 = v1;
        compare_bytes(a301, a302)
    };
    let a306 = {
        let a303 = Bytes::lit(&[]);
        let a304 = Bytes::lit(&[0x01]);
        compare_bytes(a303, a304)
    };
    equal(a305, a306)
}
