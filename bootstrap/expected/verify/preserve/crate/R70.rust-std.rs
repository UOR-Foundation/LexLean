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
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C3(u64, std::rc::Rc<Adt1>),
    C4,
    C5,
    C6(std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt0),
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
}

#[derive(Clone)]
pub enum Adt3 {
    C0(List<Bytes>, u64, List<Adt4>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, bool, Adt1),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Option<Adt0>),
    C1(Bytes),
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Adt0, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt0, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Adt0, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64, bool, List<Adt4>, List<Adt4>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes, Adt0, Adt0),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Adt0, Adt0),
}

#[derive(Clone)]
pub enum Fn0 {
    F18(u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn0::F18(k0, k1) => f18(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F40,
    F88(Bytes, Bytes),
    F94,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn1::F40 => f40(p0, p1),
            Fn1::F88(k0, k1) => f88(k0.clone(), k1.clone(), p0, p1),
            Fn1::F94 => f94(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F79,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt1, p1: Adt1) -> Adt1 {
        match self {
            Fn2::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F81(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt1, p1: (Bytes, Adt1)) -> Adt1 {
        match self {
            Fn3::F81(k0) => f81(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F90(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (bool, List<Adt1>), p1: Adt1) -> (bool, List<Adt1>) {
        match self {
            Fn4::F90(k0) => f90(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F93,
}

impl Fn5 {
    pub fn apply(&self, p0: (bool, Adt1), p1: Adt1) -> (bool, Adt1) {
        match self {
            Fn5::F93 => f93(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F97(Option<Bytes>),
    F241,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> R<List<(Bytes, Adt1)>> {
        match self {
            Fn6::F97(k0) => f97(k0.clone(), p0, p1),
            Fn6::F241 => Ok(f241(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F99,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt1>, p1: (Bytes, Adt1)) -> List<Adt1> {
        match self {
            Fn7::F99 => f99(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F102(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt6) -> Option<Adt6> {
        match self {
            Fn8::F102(k0) => f102(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F104(Adt0),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn9::F104(k0) => f104(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F108,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn10::F108 => f108(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F110,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn11::F110 => f110(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F121,
    F122,
    F127,
    F128,
    F129,
    F208,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt0) -> R<bool> {
        match self {
            Fn12::F121 => f121(p0),
            Fn12::F122 => f122(p0),
            Fn12::F127 => f127(p0),
            Fn12::F128 => f128(p0),
            Fn12::F129 => f129(p0),
            Fn12::F208 => f208(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F131(u64, Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn13::F131(k0, k1) => f131(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F136,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt0) -> List<Adt1> {
        match self {
            Fn14::F136 => f136(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F141,
}

impl Fn15 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn15::F141 => f141(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F147(Bytes, Adt0),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<Adt0>, p1: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn16::F147(k0, k1) => f147(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F149(Adt0),
    F174(Fn12),
    F217(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> R<bool> {
        match self {
            Fn17::F149(k0) => f149(k0.clone(), p0, p1),
            Fn17::F174(k0) => f174(k0.clone(), p0, p1),
            Fn17::F217(k0) => Ok(f217(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F157,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn18::F157 => f157(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F159,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
        match self {
            Fn19::F159 => f159(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F163(Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn20::F163(k0) => f163(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F181,
}

impl Fn21 {
    pub fn apply(&self, p0: Adt10) -> Option<Adt10> {
        match self {
            Fn21::F181 => f181(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F185(Bytes),
    F207(bool, Bytes),
    F226(u64, Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn22::F185(k0) => f185(k0.clone(), p0),
            Fn22::F207(k0, k1) => f207(*k0, k1.clone(), p0),
            Fn22::F226(k0, k1) => f226(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F189(u64, Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn23::F189(k0, k1) => f189(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F193,
}

impl Fn24 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn24::F193 => f193(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F198(Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn25::F198(k0) => f198(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F201,
}

impl Fn26 {
    pub fn apply(&self, p0: Adt11) -> R<Option<Adt11>> {
        match self {
            Fn26::F201 => f201(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F203,
}

impl Fn27 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn27::F203 => f203(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F205(Adt0),
    F206(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn28::F205(k0) => f205(k0.clone(), p0, p1),
            Fn28::F206(k0) => f206(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F213,
}

impl Fn29 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn29::F213 => f213(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F222(u64),
}

impl Fn30 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn30::F222(k0) => f222(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F229,
}

impl Fn31 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
        match self {
            Fn31::F229 => f229(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F240,
}

impl Fn32 {
    pub fn apply(&self, p0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
        match self {
            Fn32::F240 => f240(p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: Adt0, v2: u64) -> R<Bytes> {
    f1(f2(v0.clone(), v1, f3(v0, v2)?)?)
}

pub fn f1(v0: Adt1) -> R<Bytes> {
    f4(v0.clone(), 100u64, f5(v0)?)
}

pub fn f2(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Adt1> {
    let m1 = v1;
    match m1 {
        Adt0::C0(v5) => {
            f12(Adt0::C0(v5))
        }
        Adt0::C1(v6, v7) => {
            f12(Adt0::C1(v6, v7))
        }
        Adt0::C2(v8) => {
            f12(Adt0::C2(v8))
        }
        Adt0::C3(r2) => {
            let v4 = (*r2).clone();
            Ok(f9(v2.clone(), f10(v0.clone(), v4, f11(v0, v2)?)?))
        }
        Adt0::C4(r3) => {
            let v3 = (*r3).clone();
            f6(v3.clone(), v2.clone(), f7(v0.clone(), v3.clone(), f8(v0, v3, v2)?)?)
        }
        Adt0::C5 => {
            Ok(Adt1::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt1::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt1::C0)
        }
    }
}

pub fn f3(v0: Adt0, v1: u64) -> R<Adt2> {
    let a4 = v1;
    let a5 = 0u64;
    if nat_eq(a4, a5) {
        let m6 = f13(v0.clone(), f14(Str::lit("properties")));
        match m6 {
            None => {
                Ok(Adt2::C5)
            }
            Some(v2) => {
                let m7 = f13(v2, f14(Str::lit("declarations")));
                match m7 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v3) => {
                        f15(v0, v3)
                    }
                }
            }
        }
    } else {
        let a8 = v1;
        let a9 = 1u64;
        if nat_eq(a8, a9) {
            Ok(Adt2::C0)
        } else {
            let a10 = v1;
            let a11 = 2u64;
            if nat_eq(a10, a11) {
                Ok(Adt2::C1)
            } else {
                let m12 = f16(v0, f14(Str::lit("proof")));
                match m12 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v4) => {
                        Ok(Adt2::C4(v4))
                    }
                }
            }
        }
    }
}

pub fn f4(v0: Adt1, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt3 = {
        let (h13, _) = f17(Fn0::F18(v2, v1), {
            let a14 = v2;
            let a15 = 1u64;
            nat_add(a14, a15)?
        }, Adt3::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt4::C0(0u64, false, v0), List::<Adt4>::nil())))?;
        h13
    };
    Ok(f19(match v3 {
        Adt3::C0(h16, _, _) => {
            h16
        }
    }))
}

pub fn f5(v0: Adt1) -> R<u64> {
    let m17 = v0;
    match m17 {
        Adt1::C0 => {
            Ok(1u64)
        }
        Adt1::C1(_) => {
            Ok(1u64)
        }
        Adt1::C2(r26, r27) => {
            let v2 = (*r26).clone();
            let v3 = (*r27).clone();
            let v4: u64 = f5(v2)?;
            let v5: u64 = f5(v3)?;
            let a20 = {
                let a18 = 1u64;
                let a19 = v4;
                nat_add(a18, a19)?
            };
            let a21 = v5;
            nat_add(a20, a21)
        }
        Adt1::C3(_, r28) => {
            let v7 = (*r28).clone();
            let v8: u64 = f5(v7)?;
            let a22 = 1u64;
            let a23 = v8;
            nat_add(a22, a23)
        }
        Adt1::C4 => {
            Ok(1u64)
        }
        Adt1::C5 => {
            Ok(1u64)
        }
        Adt1::C6(r29) => {
            let v9 = (*r29).clone();
            let v10: u64 = f5(v9)?;
            let a24 = 1u64;
            let a25 = v10;
            nat_add(a24, a25)
        }
    }
}

pub fn f6(v0: Adt0, v1: Adt2, v2: List<(Bytes, Adt1)>) -> R<Adt1> {
    let m30 = v1;
    match m30 {
        Adt2::C0 => {
            if f20(v0.clone())? {
                f21(v0.clone(), {
                    let m31 = f22(v0);
                    match m31 {
                        None => {
                            f14(Str::lit(""))
                        }
                        Some(v3) => {
                            v3
                        }
                    }
                }, v2)
            } else {
                f23(v0, v2)
            }
        }
        Adt2::C1 => {
            if f24(v0.clone())? {
                f25(v0.clone(), {
                    let m32 = f22(v0);
                    match m32 {
                        None => {
                            f14(Str::lit(""))
                        }
                        Some(v4) => {
                            v4
                        }
                    }
                }, v2)
            } else {
                f23(v0, v2)
            }
        }
        Adt2::C2 => {
            let m33 = f26(Adt0::C4(std::rc::Rc::new(v0.clone())))?;
            match m33 {
                None => {
                    f12(Adt0::C4(std::rc::Rc::new(v0)))
                }
                Some(v5) => {
                    Ok(f27(v5))
                }
            }
        }
        Adt2::C3 => {
            if f28(Adt0::C4(std::rc::Rc::new(v0.clone())))? {
                Ok(f29(List::cons(f27(f30(f31(v0, f14(Str::lit("name"))), f14(Str::lit(" : ")))), List::cons(f32(v2, f14(Str::lit("type"))), List::<Adt1>::nil()))))
            } else {
                f23(v0, v2)
            }
        }
        Adt2::C4(_) => {
            f23(v0, v2)
        }
        Adt2::C5 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C6 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C7 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C8 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C9 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C10 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C11 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C12 => {
            Ok(f29(List::cons(f27(f30(f31(v0, f14(Str::lit("field"))), f14(Str::lit(" := ")))), List::cons(f32(v2, f14(Str::lit("value"))), List::<Adt1>::nil()))))
        }
        Adt2::C13 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C14 => {
            f33(v0, v2)
        }
    }
}

pub fn f7(v0: Adt0, v1: Adt0, v2: Adt5) -> R<List<(Bytes, Adt1)>> {
    let m34 = v1;
    match m34 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C7(v3, r35, r36) => {
            let v4 = (*r35).clone();
            let v5 = (*r36).clone();
            if f34(v2.clone()) {
                Ok(List::<(Bytes, Adt1)>::nil())
            } else {
                Ok(List::cons((v3.clone(), f2(v0.clone(), v4, f35(v0.clone(), v2.clone(), v3)?)?), f7(v0, v5, v2)?))
            }
        }
    }
}

pub fn f8(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Adt5> {
    let m37 = v2.clone();
    match m37 {
        Adt2::C0 => {
            if f20(v1.clone())? {
                Ok(Adt5::C1({
                    let m38 = f22(v1);
                    match m38 {
                        None => {
                            f14(Str::lit(""))
                        }
                        Some(v3) => {
                            v3
                        }
                    }
                }))
            } else {
                Ok(Adt5::C0(f36(v0, v1, v2)?))
            }
        }
        Adt2::C1 => {
            if f24(v1.clone())? {
                Ok(Adt5::C2)
            } else {
                Ok(Adt5::C0(f36(v0, v1, v2)?))
            }
        }
        Adt2::C2 => {
            Ok(Adt5::C6)
        }
        Adt2::C3 => {
            if f28(Adt0::C4(std::rc::Rc::new(v1.clone())))? {
                Ok(Adt5::C3)
            } else {
                Ok(Adt5::C0(f36(v0, v1, v2)?))
            }
        }
        Adt2::C4(_) => {
            Ok(Adt5::C0(f36(v0, v1, v2)?))
        }
        Adt2::C5 => {
            Ok(Adt5::C6)
        }
        Adt2::C6 => {
            Ok(Adt5::C6)
        }
        Adt2::C7 => {
            Ok(Adt5::C6)
        }
        Adt2::C8 => {
            Ok(Adt5::C6)
        }
        Adt2::C9 => {
            Ok(Adt5::C6)
        }
        Adt2::C10 => {
            Ok(Adt5::C6)
        }
        Adt2::C11 => {
            Ok(Adt5::C6)
        }
        Adt2::C12 => {
            Ok(Adt5::C4)
        }
        Adt2::C13 => {
            Ok(Adt5::C6)
        }
        Adt2::C14 => {
            Ok(Adt5::C5)
        }
    }
}

pub fn f9(v0: Adt2, v1: List<Adt1>) -> Adt1 {
    let m39 = v0;
    match m39 {
        Adt2::C0 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C1 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C2 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C3 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C4(_) => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C5 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C6 => {
            f29(f37(v1, f14(Str::lit(" ")), f14(Str::lit(""))))
        }
        Adt2::C7 => {
            f29(List::cons(f27(f14(Str::lit("["))), {
                let a40 = f38(v1, f14(Str::lit(", ")));
                let a41 = List::cons(f27(f14(Str::lit("]"))), List::<Adt1>::nil());
                append_list(a40, a41)
            }))
        }
        Adt2::C8 => {
            f29(f37(v1, f14(Str::lit("(")), f14(Str::lit(") "))))
        }
        Adt2::C9 => {
            f29(List::cons(f27(f14(Str::lit("("))), {
                let a42 = f38(v1, f14(Str::lit(" ")));
                let a43 = List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil());
                append_list(a42, a43)
            }))
        }
        Adt2::C10 => {
            f29(f37(v1, f14(Str::lit(" ")), f14(Str::lit(""))))
        }
        Adt2::C11 => {
            f29(List::cons(f27(f14(Str::lit("{"))), {
                let a44 = f38(v1, f14(Str::lit(", ")));
                let a45 = List::cons(f27(f14(Str::lit("}"))), List::<Adt1>::nil());
                append_list(a44, a45)
            }))
        }
        Adt2::C12 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C13 => {
            f29(f39(Fn1::F40, List::<Adt1>::nil(), v1))
        }
        Adt2::C14 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
    }
}

pub fn f10(v0: Adt0, v1: Adt0, v2: Adt2) -> R<List<Adt1>> {
    let m46 = v1;
    match m46 {
        Adt0::C0(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C6(r47, r48) => {
            let v3 = (*r47).clone();
            let v4 = (*r48).clone();
            Ok(List::cons(f2(v0.clone(), v3, v2.clone())?, f10(v0, v4, v2)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt1>::nil())
        }
    }
}

pub fn f11(v0: Adt0, v1: Adt2) -> R<Adt2> {
    let m49 = v1;
    match m49 {
        Adt2::C0 => {
            Ok(Adt2::C5)
        }
        Adt2::C1 => {
            Ok(Adt2::C5)
        }
        Adt2::C2 => {
            Ok(Adt2::C5)
        }
        Adt2::C3 => {
            Ok(Adt2::C5)
        }
        Adt2::C4(v2) => {
            f15(v0, v2)
        }
        Adt2::C5 => {
            Ok(Adt2::C5)
        }
        Adt2::C6 => {
            Ok(Adt2::C0)
        }
        Adt2::C7 => {
            Ok(Adt2::C1)
        }
        Adt2::C8 => {
            Ok(Adt2::C3)
        }
        Adt2::C9 => {
            Ok(Adt2::C1)
        }
        Adt2::C10 => {
            Ok(Adt2::C1)
        }
        Adt2::C11 => {
            Ok(Adt2::C12)
        }
        Adt2::C12 => {
            Ok(Adt2::C5)
        }
        Adt2::C13 => {
            Ok(Adt2::C14)
        }
        Adt2::C14 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f12(v0: Adt0) -> R<Adt1> {
    let m50 = v0;
    match m50 {
        Adt0::C0(v1) => {
            Ok(Adt1::C1(if v1 {
                Bytes::lit(&[0x74, 0x72, 0x75, 0x65])
            } else {
                Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65])
            }))
        }
        Adt0::C1(v2, v3) => {
            Ok(Adt1::C1(f43(v2, v3)))
        }
        Adt0::C2(v4) => {
            Ok(Adt1::C1(f44(v4)?))
        }
        Adt0::C3(r51) => {
            let v5 = (*r51).clone();
            Ok(f41(f45(Str::lit("[")), f45(Str::lit("]")), f42(f46(v5)?)))
        }
        Adt0::C4(r52) => {
            let v6 = (*r52).clone();
            f47(f48(v6.clone())?, f49(v6)?)
        }
        Adt0::C5 => {
            Ok(Adt1::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt1::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt1::C0)
        }
    }
}

pub fn f13(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m53 = f50(v0);
    match m53 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f51(v2, v1)
        }
    }
}

pub fn f14(v0: Str) -> Bytes {
    let a54 = v0;
    utf8_encode(a54)
}

pub fn f15(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m55 = f13(f52(v0.clone(), v1)?, f14(Str::lit("items")));
    match m55 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v2) => {
            f53(v0, v2)
        }
    }
}

pub fn f16(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m56 = f13(v0, f14(Str::lit("$defs")));
    match m56 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f13(v2, v1)
        }
    }
}

pub fn f17(v0: Fn0, p61: u64, p62: Adt3) -> R<(Adt3, bool)> {
    let m60 = (p61, p62);
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v1, v2) = m60;
                let m57 = v1;
                if m57 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m57 - 1;
                    let m59 = {
                        let c58: Fn0 = v0.clone();
                        c58.apply(v2.clone())?
                    };
                    match m59 {
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
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    f54(v1, v0, v2)
}

pub fn f19(v0: List<Bytes>) -> Bytes {
    f55(f56(v0))
}

pub fn f20(v0: Adt0) -> R<bool> {
    let m63 = f22(v0.clone());
    match m63 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f57(v0, v1)
        }
    }
}

pub fn f21(v0: Adt0, v1: Bytes, v2: List<(Bytes, Adt1)>) -> R<Adt1> {
    let m64 = f58(v1.clone());
    match m64 {
        None => {
            if f61(v1.clone(), f14(Str::lit("var"))) {
                Ok(f27(f31(v0, f14(Str::lit("name")))))
            } else {
                if f61(v1.clone(), f14(Str::lit("nat"))) {
                    Ok(f27(f31(v0, f14(Str::lit("value")))))
                } else {
                    if f61(v1.clone(), f14(Str::lit("bool"))) {
                        Ok(f27({
                            let m65 = f51(v0, f14(Str::lit("value")));
                            match m65 {
                                None => {
                                    f14(Str::lit(""))
                                }
                                Some(v4) => {
                                    let m66 = f62(v4);
                                    match m66 {
                                        None => {
                                            f14(Str::lit(""))
                                        }
                                        Some(v5) => {
                                            if v5 {
                                                f14(Str::lit("true"))
                                            } else {
                                                f14(Str::lit("false"))
                                            }
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if f61(v1.clone(), f14(Str::lit("string"))) {
                            Ok(f27(f63(f31(v0, f14(Str::lit("value"))))?))
                        } else {
                            if f61(v1.clone(), f14(Str::lit("bytes"))) {
                                Ok(f27(f30(f14(Str::lit("0x")), f31(v0, f14(Str::lit("hex"))))))
                            } else {
                                if f61(v1.clone(), f14(Str::lit("unit"))) {
                                    Ok(f27(f14(Str::lit("()"))))
                                } else {
                                    if f61(v1.clone(), f14(Str::lit("integer"))) {
                                        Ok(f27(f30(f30(f30(f30(f14(Str::lit("(")), f31(v0.clone(), f14(Str::lit("value")))), f14(Str::lit(" : "))), {
                                            let m67 = f64(f31(v0, f14(Str::lit("representation"))));
                                            match m67 {
                                                None => {
                                                    f14(Str::lit(""))
                                                }
                                                Some(v6) => {
                                                    v6
                                                }
                                            }
                                        }), f14(Str::lit(")")))))
                                    } else {
                                        if f61(v1.clone(), f14(Str::lit("pair"))) {
                                            Ok(f29(List::cons(f27(f14(Str::lit("("))), List::cons(f32(v2.clone(), f14(Str::lit("left"))), List::cons(f27(f14(Str::lit(", "))), List::cons(f32(v2, f14(Str::lit("right"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
                                        } else {
                                            if if f61(v1.clone(), f14(Str::lit("not"))) {
                                                true
                                            } else {
                                                if f61(v1.clone(), f14(Str::lit("first"))) {
                                                    true
                                                } else {
                                                    f61(v1.clone(), f14(Str::lit("second")))
                                                }
                                            } {
                                                Ok(f65(v1, List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("value"))), List::<Adt1>::nil()))))
                                            } else {
                                                if f61(v1.clone(), f14(Str::lit("cons"))) {
                                                    Ok(f65(v1, List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("head"))), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("tail"))), List::<Adt1>::nil()))))))
                                                } else {
                                                    if f61(v1.clone(), f14(Str::lit("nil"))) {
                                                        Ok(f65(v1, List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("element"))), List::<Adt1>::nil()))))
                                                    } else {
                                                        if f61(v1.clone(), f14(Str::lit("if"))) {
                                                            Ok(f29(List::cons(f27(f14(Str::lit("(if "))), List::cons(f32(v2.clone(), f14(Str::lit("condition"))), List::cons(f27(f14(Str::lit(" then "))), List::cons(f32(v2.clone(), f14(Str::lit("then_value"))), List::cons(f27(f14(Str::lit(" else "))), List::cons(f32(v2, f14(Str::lit("else_value"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))))
                                                        } else {
                                                            if f61(v1.clone(), f14(Str::lit("let"))) {
                                                                Ok(f29(List::cons(f27(f14(Str::lit("(let "))), List::cons(f32(v2.clone(), f14(Str::lit("binder"))), List::cons(f27(f14(Str::lit(" := "))), List::cons(f32(v2.clone(), f14(Str::lit("value"))), List::cons(f27(f14(Str::lit("; "))), List::cons(f32(v2, f14(Str::lit("body"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))))
                                                            } else {
                                                                if f61(v1.clone(), f14(Str::lit("forall"))) {
                                                                    Ok(f29(List::cons(f27(f14(Str::lit("(forall ("))), List::cons(f32(v2.clone(), f14(Str::lit("binder"))), List::cons(f27(f14(Str::lit("), "))), List::cons(f32(v2, f14(Str::lit("body"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
                                                                } else {
                                                                    if f61(v1.clone(), f14(Str::lit("lambda"))) {
                                                                        let v7: List<Adt1> = f66(v0, f14(Str::lit("captures")));
                                                                        Ok(f29(List::cons(f27(f14(Str::lit("(fun "))), {
                                                                            let a73 = {
                                                                                let a69 = 0u64;
                                                                                let a70 = {
                                                                                    let a68 = v7.clone();
                                                                                    length_list(a68)
                                                                                };
                                                                                if nat_lt(a69, a70) {
                                                                                    List::cons(f27(f14(Str::lit("["))), {
                                                                                        let a71 = f38(v7, f14(Str::lit(", ")));
                                                                                        let a72 = List::cons(f27(f14(Str::lit("]"))), List::cons(f27(f14(Str::lit(" "))), List::<Adt1>::nil()));
                                                                                        append_list(a71, a72)
                                                                                    })
                                                                                } else {
                                                                                    List::<Adt1>::nil()
                                                                                }
                                                                            };
                                                                            let a74 = List::cons(f32(v2.clone(), f14(Str::lit("parameters"))), List::cons(f27(f14(Str::lit("=> "))), List::cons(f32(v2, f14(Str::lit("body"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil()))));
                                                                            append_list(a73, a74)
                                                                        })))
                                                                    } else {
                                                                        if f61(v1.clone(), f14(Str::lit("apply"))) {
                                                                            Ok(f65(f14(Str::lit("apply")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("function"))), List::cons(f32(v2, f14(Str::lit("arguments"))), List::<Adt1>::nil())))))
                                                                        } else {
                                                                            if f61(v1.clone(), f14(Str::lit("project"))) {
                                                                                Ok(f65(f14(Str::lit("project")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("value"))), List::cons(f27(f30(f14(Str::lit(" ")), f31(v0, f14(Str::lit("field"))))), List::<Adt1>::nil())))))
                                                                            } else {
                                                                                if f61(v1.clone(), f14(Str::lit("call"))) {
                                                                                    Ok(f29(List::cons(f27(f30(f14(Str::lit("(")), f67(v0.clone(), f14(Str::lit("function")))?)), {
                                                                                        let a75 = f68(v0, v2.clone());
                                                                                        let a76 = List::cons(f32(v2, f14(Str::lit("arguments"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil()));
                                                                                        append_list(a75, a76)
                                                                                    })))
                                                                                } else {
                                                                                    if f61(v1.clone(), f14(Str::lit("constructor"))) {
                                                                                        Ok(f65(f14(Str::lit("ctor")), List::cons(f27(f30(f14(Str::lit(" ")), f67(v0.clone(), f14(Str::lit("constructor")))?)), {
                                                                                            let a77 = f68(v0, v2.clone());
                                                                                            let a78 = List::cons(f32(v2, f14(Str::lit("arguments"))), List::<Adt1>::nil());
                                                                                            append_list(a77, a78)
                                                                                        })))
                                                                                    } else {
                                                                                        if f61(v1.clone(), f14(Str::lit("function_ref"))) {
                                                                                            Ok(f65(f14(Str::lit("ref")), List::cons(f27(f30(f14(Str::lit(" ")), f67(v0.clone(), f14(Str::lit("function")))?)), f68(v0, v2))))
                                                                                        } else {
                                                                                            if f61(v1.clone(), f14(Str::lit("record"))) {
                                                                                                Ok(f65(f14(Str::lit("record")), List::cons(f27(f30(f14(Str::lit(" ")), f67(v0.clone(), f14(Str::lit("type")))?)), {
                                                                                                    let a79 = f68(v0, v2.clone());
                                                                                                    let a80 = List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("fields"))), List::<Adt1>::nil()));
                                                                                                    append_list(a79, a80)
                                                                                                })))
                                                                                            } else {
                                                                                                if f61(v1.clone(), f14(Str::lit("primitive"))) {
                                                                                                    Ok(f29(List::cons(f27(f30(f14(Str::lit("(prim ")), f31(v0, f14(Str::lit("operation"))))), List::cons(f32(v2.clone(), f14(Str::lit("arguments"))), List::cons(f27(f14(Str::lit(" : "))), List::cons(f32(v2, f14(Str::lit("result"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
                                                                                                } else {
                                                                                                    if f61(v1, f14(Str::lit("checked_apply"))) {
                                                                                                        Ok(f65(f14(Str::lit("checked")), List::cons(f27(f30(f14(Str::lit(" ")), f67(v0.clone(), f14(Str::lit("model")))?)), {
                                                                                                            let a83 = f68(v0.clone(), v2.clone());
                                                                                                            let a84 = List::cons(f32(v2, f14(Str::lit("arguments"))), List::cons(f27(f14(Str::lit(" with "))), List::cons(f27(f14(Str::lit("["))), {
                                                                                                                let a81 = f38(f66(v0, f14(Str::lit("checks"))), f14(Str::lit(", ")));
                                                                                                                let a82 = List::cons(f27(f14(Str::lit("]"))), List::<Adt1>::nil());
                                                                                                                append_list(a81, a82)
                                                                                                            })));
                                                                                                            append_list(a83, a84)
                                                                                                        })))
                                                                                                    } else {
                                                                                                        Ok(Adt1::C6(std::rc::Rc::new(Adt1::C3(2u64, std::rc::Rc::new(f29(List::cons(f27(f14(Str::lit("(match "))), List::cons(f32(v2.clone(), f14(Str::lit("scrutinee"))), List::cons(f32(v2, f14(Str::lit("branches"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil()))))))))))
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(f29(List::cons(f27(f14(Str::lit("("))), List::cons(f32(v2.clone(), f59(v1.clone())), List::cons(f27(f30(f30(f14(Str::lit(" ")), v3), f14(Str::lit(" ")))), List::cons(f32(v2, f60(v1)), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
        }
    }
}

pub fn f22(v0: Adt0) -> Option<Bytes> {
    f69(f51(v0, f14(Str::lit("kind"))))
}

pub fn f23(v0: Adt0, v1: List<(Bytes, Adt1)>) -> R<Adt1> {
    f47(f48(v0)?, v1)
}

pub fn f24(v0: Adt0) -> R<bool> {
    let m85 = f22(v0.clone());
    match m85 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f70(v0, v1)
        }
    }
}

pub fn f25(v0: Adt0, v1: Bytes, v2: List<(Bytes, Adt1)>) -> R<Adt1> {
    let m86 = f71(v1.clone());
    match m86 {
        None => {
            if f61(v1.clone(), f14(Str::lit("parameter"))) {
                Ok(f27(f30(f14(Str::lit("'")), f31(v0, f14(Str::lit("name"))))))
            } else {
                if f61(v1.clone(), f14(Str::lit("list"))) {
                    Ok(f65(f14(Str::lit("List")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("element"))), List::<Adt1>::nil()))))
                } else {
                    if f61(v1.clone(), f14(Str::lit("set"))) {
                        Ok(f65(f14(Str::lit("Set")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("element"))), List::<Adt1>::nil()))))
                    } else {
                        if f61(v1.clone(), f14(Str::lit("option"))) {
                            Ok(f65(f14(Str::lit("Option")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("value"))), List::<Adt1>::nil()))))
                        } else {
                            if f61(v1.clone(), f14(Str::lit("result"))) {
                                Ok(f65(f14(Str::lit("Result")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("ok"))), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("error"))), List::<Adt1>::nil()))))))
                            } else {
                                if f61(v1.clone(), f14(Str::lit("map"))) {
                                    Ok(f65(f14(Str::lit("Map")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("key"))), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("value"))), List::<Adt1>::nil()))))))
                                } else {
                                    if f61(v1.clone(), f14(Str::lit("product"))) {
                                        Ok(f29(List::cons(f27(f14(Str::lit("("))), List::cons(f32(v2.clone(), f14(Str::lit("left"))), List::cons(f27(f14(Str::lit(" * "))), List::cons(f32(v2, f14(Str::lit("right"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
                                    } else {
                                        if f61(v1, f14(Str::lit("function"))) {
                                            Ok(f65(f14(Str::lit("Fn")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("parameters"))), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("result"))), List::<Adt1>::nil()))))))
                                        } else {
                                            let v4: Bytes = f67(v0.clone(), f14(Str::lit("member")))?;
                                            let m87 = f72(f51(v0, f14(Str::lit("arguments"))));
                                            match m87 {
                                                None => {
                                                    Ok(f27(v4))
                                                }
                                                Some(v5) => {
                                                    let a88 = f73(v5)?;
                                                    let a89 = 0u64;
                                                    if nat_eq(a88, a89) {
                                                        Ok(f27(v4))
                                                    } else {
                                                        Ok(f29(List::cons(f27(f30(f14(Str::lit("(")), v4)), List::cons(f32(v2, f14(Str::lit("arguments"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))
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
        Some(v3) => {
            Ok(f27(v3))
        }
    }
}

pub fn f26(v0: Adt0) -> R<Option<Bytes>> {
    let m90 = f50(v0);
    match m90 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let m91 = f69(f51(v1.clone(), f14(Str::lit("name"))));
            match m91 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    let v3: Option<Adt0> = f51(v1.clone(), f14(Str::lit("module")));
                    if f74({
                        let a92 = f73(v1)?;
                        let a93 = if f75(v3.clone()) {
                            2u64
                        } else {
                            1u64
                        };
                        nat_eq(a92, a93)
                    }, f76(v2.clone())?) {
                        let m94 = v3;
                        match m94 {
                            None => {
                                Ok(Some(v2))
                            }
                            Some(v4) => {
                                let m95 = f77(v4);
                                match m95 {
                                    None => {
                                        Ok(None::<Bytes>)
                                    }
                                    Some(v5) => {
                                        if f76(v5.clone())? {
                                            Ok(Some(f30(f30(v5, f14(Str::lit("::"))), v2)))
                                        } else {
                                            Ok(None::<Bytes>)
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f27(v0: Bytes) -> Adt1 {
    Adt1::C1(v0)
}

pub fn f28(v0: Adt0) -> R<bool> {
    let m96 = f50(v0);
    match m96 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            let m97 = f69(f51(v1.clone(), f14(Str::lit("name"))));
            match m97 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    Ok(f74(f75(f51(v1.clone(), f14(Str::lit("type")))), f74({
                        let a98 = f73(v1)?;
                        let a99 = 2u64;
                        nat_eq(a98, a99)
                    }, f76(v2)?)))
                }
            }
        }
    }
}

pub fn f29(v0: List<Adt1>) -> Adt1 {
    f78(Fn2::F79, Adt1::C0, f42(v0))
}

pub fn f30(v0: Bytes, v1: Bytes) -> Bytes {
    let a100 = v0;
    let a101 = v1;
    append_bytes(a100, a101)
}

pub fn f31(v0: Adt0, v1: Bytes) -> Bytes {
    let m102 = f69(f51(v0, v1));
    match m102 {
        None => {
            f14(Str::lit(""))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f32(v0: List<(Bytes, Adt1)>, v1: Bytes) -> Adt1 {
    f80(Fn3::F81(v1), Adt1::C0, v0)
}

pub fn f33(v0: Adt0, v1: List<(Bytes, Adt1)>) -> R<Adt1> {
    Ok(f29(List::cons(f27(f30(f14(Str::lit("| ")), f67(v0.clone(), f14(Str::lit("constructor")))?)), {
        let a103 = f37(f66(v0, f14(Str::lit("binders"))), f14(Str::lit(" ")), f14(Str::lit("")));
        let a104 = List::cons(f27(f14(Str::lit(" => "))), List::cons(f32(v1, f14(Str::lit("body"))), List::<Adt1>::nil()));
        append_list(a103, a104)
    })))
}

pub fn f34(v0: Adt5) -> bool {
    let m105 = v0;
    match m105 {
        Adt5::C0(_) => {
            false
        }
        Adt5::C1(_) => {
            false
        }
        Adt5::C2 => {
            false
        }
        Adt5::C3 => {
            false
        }
        Adt5::C4 => {
            false
        }
        Adt5::C5 => {
            false
        }
        Adt5::C6 => {
            true
        }
    }
}

pub fn f35(v0: Adt0, v1: Adt5, v2: Bytes) -> R<Adt2> {
    let m106 = v1;
    match m106 {
        Adt5::C0(v3) => {
            f82(v0, v3, v2)
        }
        Adt5::C1(v4) => {
            Ok(f83(v4, v2))
        }
        Adt5::C2 => {
            Ok(f84(v2))
        }
        Adt5::C3 => {
            if f61(v2, f14(Str::lit("type"))) {
                Ok(Adt2::C1)
            } else {
                Ok(Adt2::C5)
            }
        }
        Adt5::C4 => {
            if f61(v2, f14(Str::lit("value"))) {
                Ok(Adt2::C0)
            } else {
                Ok(Adt2::C5)
            }
        }
        Adt5::C5 => {
            if f61(v2.clone(), f14(Str::lit("body"))) {
                Ok(Adt2::C0)
            } else {
                if f61(v2, f14(Str::lit("constructor"))) {
                    Ok(Adt2::C2)
                } else {
                    Ok(Adt2::C5)
                }
            }
        }
        Adt5::C6 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Option<Adt0>> {
    let v3: Option<Bytes> = f48(v1)?;
    let m107 = v2.clone();
    match m107 {
        Adt2::C0 => {
            let m108 = v3;
            match m108 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    let m109 = f85(v0.clone(), v2)?;
                    match m109 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v5) => {
                            f86(v0, v5, v4)
                        }
                    }
                }
            }
        }
        Adt2::C1 => {
            let m110 = v3;
            match m110 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v6) => {
                    let m111 = f85(v0.clone(), v2)?;
                    match m111 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v7) => {
                            f86(v0, v7, v6)
                        }
                    }
                }
            }
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(f16(v0, f14(Str::lit("parameter"))))
        }
        Adt2::C4(v8) => {
            let m112 = v3;
            match m112 {
                None => {
                    Ok(Some(v8))
                }
                Some(v9) => {
                    if f87(v0.clone(), v8.clone())? {
                        f86(v0, v8, v9)
                    } else {
                        Ok(Some(v8))
                    }
                }
            }
        }
        Adt2::C5 => {
            Ok(None::<Adt0>)
        }
        Adt2::C6 => {
            Ok(None::<Adt0>)
        }
        Adt2::C7 => {
            Ok(None::<Adt0>)
        }
        Adt2::C8 => {
            Ok(None::<Adt0>)
        }
        Adt2::C9 => {
            Ok(None::<Adt0>)
        }
        Adt2::C10 => {
            Ok(None::<Adt0>)
        }
        Adt2::C11 => {
            Ok(None::<Adt0>)
        }
        Adt2::C12 => {
            Ok(None::<Adt0>)
        }
        Adt2::C13 => {
            Ok(None::<Adt0>)
        }
        Adt2::C14 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f37(v0: List<Adt1>, v1: Bytes, v2: Bytes) -> List<Adt1> {
    f42(f39(Fn1::F88(v2, v1), List::<Adt1>::nil(), v0))
}

pub fn f38(v0: List<Adt1>, v1: Bytes) -> List<Adt1> {
    f42({
        let (_, h113) = f89(Fn4::F90(v1), (true, List::<Adt1>::nil()), v0);
        h113
    })
}

pub fn f39(v0: Fn1, p117: List<Adt1>, p118: List<Adt1>) -> List<Adt1> {
    let m116 = (p117, p118);
    {
        let mut m116 = m116;
        loop {
            let step = {
                let (v1, v2) = m116;
                let m114 = v2;
                match m114.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c115: Fn1 = v0.clone();
                            c115.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    let a119 = v0;
    let a120 = List::cons(Adt1::C4, List::cons(v1, List::<Adt1>::nil()));
    append_list(a119, a120)
}

pub fn f41(v0: Bytes, v1: Bytes, v2: List<Adt1>) -> Adt1 {
    let m121 = v2.clone();
    match m121.uncons() {
        None => {
            Adt1::C1(f91(v0, v1))
        }
        Some((_, _)) => {
            let v5: (bool, Adt1) = f92(Fn5::F93, (true, Adt1::C0), v2);
            Adt1::C6(std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(v0)), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C3(2u64, std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C5), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new({
                let (_, h122) = v5;
                h122
            }), std::rc::Rc::new(Adt1::C0))))))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C5), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(v1)), std::rc::Rc::new(Adt1::C0))))))))))
        }
    }
}

pub fn f42(v0: List<Adt1>) -> List<Adt1> {
    f39(Fn1::F94, List::<Adt1>::nil(), v0)
}

pub fn f43(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a123 = 0u64;
        let a124 = v1;
        nat_lt(a123, a124)
    } else {
        false
    } {
        let a125 = Bytes::lit(&[0x2d]);
        let a126 = f95(v1);
        append_bytes(a125, a126)
    } else {
        f95(v1)
    }
}

pub fn f44(v0: Bytes) -> R<Bytes> {
    if f76(v0.clone())? {
        Ok(v0)
    } else {
        f63(v0)
    }
}

pub fn f45(v0: Str) -> Bytes {
    let a127 = v0;
    utf8_encode(a127)
}

pub fn f46(v0: Adt0) -> R<List<Adt1>> {
    let m128 = v0;
    match m128 {
        Adt0::C0(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C6(r129, r130) => {
            let v7 = (*r129).clone();
            let v8 = (*r130).clone();
            Ok(List::cons(f12(v7)?, f46(v8)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt1>::nil())
        }
    }
}

pub fn f47(v0: Option<Bytes>, v1: List<(Bytes, Adt1)>) -> R<Adt1> {
    let v2: List<(Bytes, Adt1)> = f96(Fn6::F97(v0.clone()), List::<(Bytes, Adt1)>::nil(), v1)?;
    Ok(f41({
        let m131 = v0;
        match m131 {
            None => {
                f45(Str::lit("{"))
            }
            Some(v3) => {
                f91(v3, f45(Str::lit("{")))
            }
        }
    }, f45(Str::lit("}")), f98(Fn7::F99, List::<Adt1>::nil(), f100(v2)?)))
}

pub fn f48(p139: Adt0) -> R<Option<Bytes>> {
    let m138 = p139;
    {
        let mut m138 = m138;
        loop {
            let step = {
                let v0 = m138;
                let m132 = v0;
                match m132 {
                    Adt0::C0(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C1(_, _) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C2(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C3(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C5 => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C6(_, _) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C7(v9, r136, r137) => {
                        let v10 = (*r136).clone();
                        let v11 = (*r137).clone();
                        let a133 = v9;
                        let a134 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
                        if equal(a133, a134) {
                            let m135 = v10;
                            match m135 {
                                Adt0::C0(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C1(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C2(v12) => {
                                    if f76(v12.clone())? {
                                        Err::<Adt0, R<Option<Bytes>>>(Ok(Some(v12)))
                                    } else {
                                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                    }
                                }
                                Adt0::C3(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C4(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C5 => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C6(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C7(_, _, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                            }
                        } else {
                            Ok::<Adt0, R<Option<Bytes>>>(v11)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Adt0) -> R<List<(Bytes, Adt1)>> {
    let m140 = v0;
    match m140 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C7(v9, r141, r142) => {
            let v10 = (*r141).clone();
            let v11 = (*r142).clone();
            Ok(List::cons((v9, f12(v10)?), f49(v11)?))
        }
    }
}

pub fn f50(v0: Adt0) -> Option<Adt0> {
    let m143 = v0;
    match m143 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(_) => {
            None::<Adt0>
        }
        Adt0::C4(r144) => {
            let v1 = (*r144).clone();
            Some(v1)
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6(_, _) => {
            None::<Adt0>
        }
        Adt0::C7(_, _, _) => {
            None::<Adt0>
        }
    }
}

pub fn f51(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m147 = {
        let (h146, _) = f101(Fn8::F102(v1), 1000000u64, Adt6::C0(v0, None::<Adt0>));
        h146
    };
    match m147 {
        Adt6::C0(_, h145) => {
            h145
        }
    }
}

pub fn f52(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let (h148, _) = f103(Fn9::F104(v0), 4096u64, v1)?;
    Ok(h148)
}

pub fn f53(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m149 = f69(f13(v1.clone(), f14(Str::lit("$ref"))));
    match m149 {
        None => {
            Ok(Adt2::C4(v1))
        }
        Some(v2) => {
            let v3: Bytes = f105(v2)?;
            if f61(v3.clone(), f14(Str::lit("term"))) {
                Ok(Adt2::C0)
            } else {
                if f61(v3.clone(), f14(Str::lit("type"))) {
                    Ok(Adt2::C1)
                } else {
                    if f61(v3.clone(), f14(Str::lit("member"))) {
                        Ok(Adt2::C2)
                    } else {
                        if f61(v3, f14(Str::lit("parameter"))) {
                            Ok(Adt2::C3)
                        } else {
                            Ok(Adt2::C4(f52(v0, v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f54(v0: u64, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let m151 = match v2.clone() {
        Adt3::C0(_, _, h150) => {
            h150
        }
    };
    match m151.uncons() {
        None => {
            Ok(None::<Adt3>)
        }
        Some((v3, v4)) => {
            Ok(Some(f106(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f55(v0: List<Bytes>) -> Bytes {
    let m154 = {
        let (h152, _) = f107(Fn10::F108, {
            let a153 = v0.clone();
            length_list(a153)
        }, v0);
        h152
    };
    match m154.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f56(v0: List<Bytes>) -> List<Bytes> {
    f109(Fn11::F110, List::<Bytes>::nil(), v0)
}

pub fn f57(v0: Adt0, v1: Bytes) -> R<bool> {
    let m155 = f58(v1.clone());
    match m155 {
        None => {
            if f61(v1.clone(), f14(Str::lit("var"))) {
                Ok(f74(f111(v0.clone(), f112(Str::lit("name")))?, {
                    let m156 = f69(f51(v0, f14(Str::lit("name"))));
                    match m156 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            f113(v3)?
                        }
                    }
                }))
            } else {
                if f61(v1.clone(), f14(Str::lit("nat"))) {
                    Ok(f74(f111(v0.clone(), f112(Str::lit("value")))?, {
                        let m157 = f69(f51(v0, f14(Str::lit("value"))));
                        match m157 {
                            None => {
                                false
                            }
                            Some(v4) => {
                                f114(v4)?
                            }
                        }
                    }))
                } else {
                    if f61(v1.clone(), f14(Str::lit("bool"))) {
                        Ok(f74(f111(v0.clone(), f112(Str::lit("value")))?, {
                            let m158 = f51(v0, f14(Str::lit("value")));
                            match m158 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    let m159 = f62(v5);
                                    match m159 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if f61(v1.clone(), f14(Str::lit("string"))) {
                            Ok(f74(f111(v0.clone(), f112(Str::lit("value")))?, {
                                let m160 = f69(f51(v0, f14(Str::lit("value"))));
                                match m160 {
                                    None => {
                                        false
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }))
                        } else {
                            if f61(v1.clone(), f14(Str::lit("bytes"))) {
                                Ok(f74(f111(v0.clone(), f112(Str::lit("hex")))?, {
                                    let m161 = f69(f51(v0, f14(Str::lit("hex"))));
                                    match m161 {
                                        None => {
                                            false
                                        }
                                        Some(v8) => {
                                            f115(v8)?
                                        }
                                    }
                                }))
                            } else {
                                if f61(v1.clone(), f14(Str::lit("unit"))) {
                                    f111(v0, List::cons(f14(Str::lit("kind")), List::<Bytes>::nil()))
                                } else {
                                    if f61(v1.clone(), f14(Str::lit("integer"))) {
                                        Ok(f74(f111(v0.clone(), f116(Str::lit("representation"), Str::lit("value")))?, {
                                            let m162 = f69(f51(v0.clone(), f14(Str::lit("value"))));
                                            match m162 {
                                                None => {
                                                    false
                                                }
                                                Some(v9) => {
                                                    let m163 = f69(f51(v0, f14(Str::lit("representation"))));
                                                    match m163 {
                                                        None => {
                                                            false
                                                        }
                                                        Some(v10) => {
                                                            f74(f75({
                                                                let m164 = f64(v10);
                                                                match m164 {
                                                                    None => {
                                                                        None::<Adt0>
                                                                    }
                                                                    Some(_) => {
                                                                        Some(Adt0::C5)
                                                                    }
                                                                }
                                                            }), f117(v9)?)
                                                        }
                                                    }
                                                }
                                            }
                                        }))
                                    } else {
                                        if f61(v1.clone(), f14(Str::lit("pair"))) {
                                            f111(v0, f116(Str::lit("left"), Str::lit("right")))
                                        } else {
                                            if if f61(v1.clone(), f14(Str::lit("not"))) {
                                                true
                                            } else {
                                                if f61(v1.clone(), f14(Str::lit("first"))) {
                                                    true
                                                } else {
                                                    f61(v1.clone(), f14(Str::lit("second")))
                                                }
                                            } {
                                                f111(v0, f112(Str::lit("value")))
                                            } else {
                                                if f61(v1.clone(), f14(Str::lit("cons"))) {
                                                    f111(v0, f116(Str::lit("head"), Str::lit("tail")))
                                                } else {
                                                    if f61(v1.clone(), f14(Str::lit("nil"))) {
                                                        f111(v0, f112(Str::lit("element")))
                                                    } else {
                                                        if f61(v1.clone(), f14(Str::lit("if"))) {
                                                            f111(v0, f118(Str::lit("condition"), Str::lit("then_value"), Str::lit("else_value")))
                                                        } else {
                                                            if f61(v1.clone(), f14(Str::lit("let"))) {
                                                                Ok(f74(f111(v0.clone(), f118(Str::lit("binder"), Str::lit("value"), Str::lit("body")))?, f119(f51(v0, f14(Str::lit("binder"))))?))
                                                            } else {
                                                                if f61(v1.clone(), f14(Str::lit("forall"))) {
                                                                    Ok(f74(f111(v0.clone(), f116(Str::lit("binder"), Str::lit("body")))?, f119(f51(v0, f14(Str::lit("binder"))))?))
                                                                } else {
                                                                    if f61(v1.clone(), f14(Str::lit("lambda"))) {
                                                                        Ok(f74(f111(v0.clone(), f118(Str::lit("parameters"), Str::lit("captures"), Str::lit("body")))?, f74(f120(v0.clone(), f14(Str::lit("parameters")), Fn12::F121)?, f120(v0, f14(Str::lit("captures")), Fn12::F122)?)))
                                                                    } else {
                                                                        if f61(v1.clone(), f14(Str::lit("apply"))) {
                                                                            Ok(f74(f111(v0.clone(), f116(Str::lit("function"), Str::lit("arguments")))?, f123(v0, f14(Str::lit("arguments")))))
                                                                        } else {
                                                                            if f61(v1.clone(), f14(Str::lit("project"))) {
                                                                                Ok(f74(f111(v0.clone(), f116(Str::lit("value"), Str::lit("field")))?, {
                                                                                    let m165 = f69(f51(v0, f14(Str::lit("field"))));
                                                                                    match m165 {
                                                                                        None => {
                                                                                            false
                                                                                        }
                                                                                        Some(v12) => {
                                                                                            f76(v12)?
                                                                                        }
                                                                                    }
                                                                                }))
                                                                            } else {
                                                                                if f61(v1.clone(), f14(Str::lit("call"))) {
                                                                                    Ok(f74(f124(v0.clone(), f116(Str::lit("function"), Str::lit("arguments")))?, f74({
                                                                                        let m166 = f51(v0.clone(), f14(Str::lit("function")));
                                                                                        match m166 {
                                                                                            None => {
                                                                                                false
                                                                                            }
                                                                                            Some(v13) => {
                                                                                                let m167 = f26(v13.clone())?;
                                                                                                match m167 {
                                                                                                    None => {
                                                                                                        false
                                                                                                    }
                                                                                                    Some(_) => {
                                                                                                        let m168 = f69(f13(v13, f14(Str::lit("name"))));
                                                                                                        match m168 {
                                                                                                            None => {
                                                                                                                false
                                                                                                            }
                                                                                                            Some(v15) => {
                                                                                                                f113(v15)?
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }, f74(f125(v0.clone())?, f123(v0, f14(Str::lit("arguments")))))))
                                                                                } else {
                                                                                    if f61(v1.clone(), f14(Str::lit("constructor"))) {
                                                                                        Ok(f74(f124(v0.clone(), f116(Str::lit("constructor"), Str::lit("arguments")))?, f74(f126(f51(v0.clone(), f14(Str::lit("constructor"))))?, f74(f125(v0.clone())?, f123(v0, f14(Str::lit("arguments")))))))
                                                                                    } else {
                                                                                        if f61(v1.clone(), f14(Str::lit("function_ref"))) {
                                                                                            Ok(f74(f124(v0.clone(), f112(Str::lit("function")))?, f74(f126(f51(v0.clone(), f14(Str::lit("function"))))?, f125(v0)?)))
                                                                                        } else {
                                                                                            if f61(v1.clone(), f14(Str::lit("record"))) {
                                                                                                Ok(f74(f124(v0.clone(), f116(Str::lit("type"), Str::lit("fields")))?, f74(f126(f51(v0.clone(), f14(Str::lit("type"))))?, f74(f125(v0.clone())?, f120(v0, f14(Str::lit("fields")), Fn12::F127)?))))
                                                                                            } else {
                                                                                                if f61(v1.clone(), f14(Str::lit("primitive"))) {
                                                                                                    Ok(f74(f111(v0.clone(), f118(Str::lit("operation"), Str::lit("arguments"), Str::lit("result")))?, f74({
                                                                                                        let m169 = f69(f51(v0.clone(), f14(Str::lit("operation"))));
                                                                                                        match m169 {
                                                                                                            None => {
                                                                                                                false
                                                                                                            }
                                                                                                            Some(v16) => {
                                                                                                                f76(v16)?
                                                                                                            }
                                                                                                        }
                                                                                                    }, f123(v0, f14(Str::lit("arguments"))))))
                                                                                                } else {
                                                                                                    if f61(v1.clone(), f14(Str::lit("checked_apply"))) {
                                                                                                        Ok(f74(f124(v0.clone(), f118(Str::lit("model"), Str::lit("arguments"), Str::lit("checks")))?, f74(f126(f51(v0.clone(), f14(Str::lit("model"))))?, f74(f125(v0.clone())?, f74(f123(v0.clone(), f14(Str::lit("arguments"))), f120(v0, f14(Str::lit("checks")), Fn12::F128)?)))))
                                                                                                    } else {
                                                                                                        if f61(v1, f14(Str::lit("match"))) {
                                                                                                            Ok(f74(f111(v0.clone(), f116(Str::lit("scrutinee"), Str::lit("branches")))?, f120(v0, f14(Str::lit("branches")), Fn12::F129)?))
                                                                                                        } else {
                                                                                                            Ok(false)
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
        Some(_) => {
            f111(v0, List::cons(f14(Str::lit("kind")), List::cons(f59(v1.clone()), List::cons(f60(v1), List::<Bytes>::nil()))))
        }
    }
}

pub fn f58(v0: Bytes) -> Option<Bytes> {
    if f61(v0.clone(), f14(Str::lit("add"))) {
        Some(f14(Str::lit("+")))
    } else {
        if f61(v0.clone(), f14(Str::lit("eq"))) {
            Some(f14(Str::lit("=")))
        } else {
            if f61(v0.clone(), f14(Str::lit("le"))) {
                Some(f14(Str::lit("<=")))
            } else {
                if f61(v0.clone(), f14(Str::lit("lt"))) {
                    Some(f14(Str::lit("<")))
                } else {
                    if f61(v0.clone(), f14(Str::lit("beq"))) {
                        Some(f14(Str::lit("=?")))
                    } else {
                        if f61(v0.clone(), f14(Str::lit("ble"))) {
                            Some(f14(Str::lit("<=?")))
                        } else {
                            if f61(v0.clone(), f14(Str::lit("blt"))) {
                                Some(f14(Str::lit("<?")))
                            } else {
                                if f61(v0.clone(), f14(Str::lit("and"))) {
                                    Some(f14(Str::lit("&&")))
                                } else {
                                    if f61(v0.clone(), f14(Str::lit("or"))) {
                                        Some(f14(Str::lit("||")))
                                    } else {
                                        if f61(v0.clone(), f14(Str::lit("prop_and"))) {
                                            Some(f14(Str::lit("/\\")))
                                        } else {
                                            if f61(v0.clone(), f14(Str::lit("iff"))) {
                                                Some(f14(Str::lit("<->")))
                                            } else {
                                                if f61(v0, f14(Str::lit("implies"))) {
                                                    Some(f14(Str::lit("->")))
                                                } else {
                                                    None::<Bytes>
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

pub fn f59(v0: Bytes) -> Bytes {
    if f61(v0, f14(Str::lit("implies"))) {
        f14(Str::lit("premise"))
    } else {
        f14(Str::lit("left"))
    }
}

pub fn f60(v0: Bytes) -> Bytes {
    if f61(v0, f14(Str::lit("implies"))) {
        f14(Str::lit("conclusion"))
    } else {
        f14(Str::lit("right"))
    }
}

pub fn f61(v0: Bytes, v1: Bytes) -> bool {
    let a170 = v0;
    let a171 = v1;
    equal(a170, a171)
}

pub fn f62(v0: Adt0) -> Option<bool> {
    let m172 = v0;
    match m172 {
        Adt0::C0(v1) => {
            Some(v1)
        }
        Adt0::C1(_, _) => {
            None::<bool>
        }
        Adt0::C2(_) => {
            None::<bool>
        }
        Adt0::C3(_) => {
            None::<bool>
        }
        Adt0::C4(_) => {
            None::<bool>
        }
        Adt0::C5 => {
            None::<bool>
        }
        Adt0::C6(_, _) => {
            None::<bool>
        }
        Adt0::C7(_, _, _) => {
            None::<bool>
        }
    }
}

pub fn f63(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a173 = v0.clone();
        length_bytes(a173)
    };
    let v2: Adt7 = {
        let (h174, _) = f130(Fn13::F131(v1, v0), {
            let a175 = v1;
            let a176 = 1u64;
            nat_add(a175, a176)?
        }, Adt7::C0(0u64, List::<Bytes>::nil()))?;
        h174
    };
    Ok(f132(f132(f133(Str::lit("\"")), f134(match v2 {
        Adt7::C0(_, h177) => {
            h177
        }
    })), f133(Str::lit("\""))))
}

pub fn f64(v0: Bytes) -> Option<Bytes> {
    if f61(v0.clone(), f14(Str::lit("int"))) {
        Some(f14(Str::lit("Int")))
    } else {
        if f61(v0.clone(), f14(Str::lit("int8"))) {
            Some(f14(Str::lit("Int8")))
        } else {
            if f61(v0.clone(), f14(Str::lit("int16"))) {
                Some(f14(Str::lit("Int16")))
            } else {
                if f61(v0.clone(), f14(Str::lit("int32"))) {
                    Some(f14(Str::lit("Int32")))
                } else {
                    if f61(v0.clone(), f14(Str::lit("int64"))) {
                        Some(f14(Str::lit("Int64")))
                    } else {
                        if f61(v0.clone(), f14(Str::lit("uint8"))) {
                            Some(f14(Str::lit("UInt8")))
                        } else {
                            if f61(v0.clone(), f14(Str::lit("uint16"))) {
                                Some(f14(Str::lit("UInt16")))
                            } else {
                                if f61(v0.clone(), f14(Str::lit("uint32"))) {
                                    Some(f14(Str::lit("UInt32")))
                                } else {
                                    if f61(v0, f14(Str::lit("uint64"))) {
                                        Some(f14(Str::lit("UInt64")))
                                    } else {
                                        None::<Bytes>
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

pub fn f65(v0: Bytes, v1: List<Adt1>) -> Adt1 {
    f29(List::cons(f27(f30(f14(Str::lit("(")), v0)), {
        let a178 = v1;
        let a179 = List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil());
        append_list(a178, a179)
    }))
}

pub fn f66(v0: Adt0, v1: Bytes) -> List<Adt1> {
    let m180 = f72(f51(v0, v1));
    match m180 {
        None => {
            List::<Adt1>::nil()
        }
        Some(v2) => {
            f135(Fn14::F136, List::<Adt1>::nil(), f137(v2))
        }
    }
}

pub fn f67(v0: Adt0, v1: Bytes) -> R<Bytes> {
    let m181 = f51(v0, v1);
    match m181 {
        None => {
            Ok(f14(Str::lit("")))
        }
        Some(v2) => {
            let m182 = f26(v2)?;
            match m182 {
                None => {
                    Ok(f14(Str::lit("")))
                }
                Some(v3) => {
                    Ok(v3)
                }
            }
        }
    }
}

pub fn f68(v0: Adt0, v1: List<(Bytes, Adt1)>) -> List<Adt1> {
    if f75(f51(v0, f14(Str::lit("type_arguments")))) {
        List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v1, f14(Str::lit("type_arguments"))), List::<Adt1>::nil()))
    } else {
        List::<Adt1>::nil()
    }
}

pub fn f69(v0: Option<Adt0>) -> Option<Bytes> {
    let m183 = v0;
    match m183 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f77(v1)
        }
    }
}

pub fn f70(v0: Adt0, v1: Bytes) -> R<bool> {
    let m184 = f71(v1.clone());
    match m184 {
        None => {
            if f61(v1.clone(), f14(Str::lit("parameter"))) {
                Ok(f74(f111(v0.clone(), f112(Str::lit("name")))?, {
                    let m185 = f69(f51(v0, f14(Str::lit("name"))));
                    match m185 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            f76(v3)?
                        }
                    }
                }))
            } else {
                if if f61(v1.clone(), f14(Str::lit("list"))) {
                    true
                } else {
                    f61(v1.clone(), f14(Str::lit("set")))
                } {
                    f111(v0, f112(Str::lit("element")))
                } else {
                    if f61(v1.clone(), f14(Str::lit("option"))) {
                        f111(v0, f112(Str::lit("value")))
                    } else {
                        if f61(v1.clone(), f14(Str::lit("result"))) {
                            f111(v0, f116(Str::lit("ok"), Str::lit("error")))
                        } else {
                            if f61(v1.clone(), f14(Str::lit("map"))) {
                                f111(v0, f116(Str::lit("key"), Str::lit("value")))
                            } else {
                                if f61(v1.clone(), f14(Str::lit("product"))) {
                                    f111(v0, f116(Str::lit("left"), Str::lit("right")))
                                } else {
                                    if f61(v1.clone(), f14(Str::lit("function"))) {
                                        Ok(f74(f111(v0.clone(), f116(Str::lit("parameters"), Str::lit("result")))?, f123(v0, f14(Str::lit("parameters")))))
                                    } else {
                                        if f61(v1, f14(Str::lit("named"))) {
                                            Ok(f74(f111(v0.clone(), f116(Str::lit("member"), Str::lit("arguments")))?, {
                                                let m186 = f51(v0.clone(), f14(Str::lit("member")));
                                                match m186 {
                                                    None => {
                                                        false
                                                    }
                                                    Some(v4) => {
                                                        let m187 = f50(v4.clone());
                                                        match m187 {
                                                            None => {
                                                                false
                                                            }
                                                            Some(v5) => {
                                                                let m188 = f69(f51(v5.clone(), f14(Str::lit("name"))));
                                                                match m188 {
                                                                    None => {
                                                                        false
                                                                    }
                                                                    Some(v6) => {
                                                                        f74(!f74(!f75(f51(v5, f14(Str::lit("module")))), f138(v6)), f74(f123(v0, f14(Str::lit("arguments"))), f75({
                                                                            let m189 = f26(v4)?;
                                                                            match m189 {
                                                                                None => {
                                                                                    None::<Adt0>
                                                                                }
                                                                                Some(_) => {
                                                                                    Some(Adt0::C5)
                                                                                }
                                                                            }
                                                                        })))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }))
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            f111(v0, List::cons(f14(Str::lit("kind")), List::<Bytes>::nil()))
        }
    }
}

pub fn f71(v0: Bytes) -> Option<Bytes> {
    if f61(v0.clone(), f14(Str::lit("bool"))) {
        Some(f14(Str::lit("Bool")))
    } else {
        if f61(v0.clone(), f14(Str::lit("bytes"))) {
            Some(f14(Str::lit("Bytes")))
        } else {
            if f61(v0.clone(), f14(Str::lit("int"))) {
                Some(f14(Str::lit("Int")))
            } else {
                if f61(v0.clone(), f14(Str::lit("int8"))) {
                    Some(f14(Str::lit("Int8")))
                } else {
                    if f61(v0.clone(), f14(Str::lit("int16"))) {
                        Some(f14(Str::lit("Int16")))
                    } else {
                        if f61(v0.clone(), f14(Str::lit("int32"))) {
                            Some(f14(Str::lit("Int32")))
                        } else {
                            if f61(v0.clone(), f14(Str::lit("int64"))) {
                                Some(f14(Str::lit("Int64")))
                            } else {
                                if f61(v0.clone(), f14(Str::lit("nat"))) {
                                    Some(f14(Str::lit("Nat")))
                                } else {
                                    if f61(v0.clone(), f14(Str::lit("ordering"))) {
                                        Some(f14(Str::lit("Ordering")))
                                    } else {
                                        if f61(v0.clone(), f14(Str::lit("prop"))) {
                                            Some(f14(Str::lit("Prop")))
                                        } else {
                                            if f61(v0.clone(), f14(Str::lit("string"))) {
                                                Some(f14(Str::lit("String")))
                                            } else {
                                                if f61(v0.clone(), f14(Str::lit("type"))) {
                                                    Some(f14(Str::lit("Type")))
                                                } else {
                                                    if f61(v0.clone(), f14(Str::lit("uint8"))) {
                                                        Some(f14(Str::lit("UInt8")))
                                                    } else {
                                                        if f61(v0.clone(), f14(Str::lit("uint16"))) {
                                                            Some(f14(Str::lit("UInt16")))
                                                        } else {
                                                            if f61(v0.clone(), f14(Str::lit("uint32"))) {
                                                                Some(f14(Str::lit("UInt32")))
                                                            } else {
                                                                if f61(v0.clone(), f14(Str::lit("uint64"))) {
                                                                    Some(f14(Str::lit("UInt64")))
                                                                } else {
                                                                    if f61(v0.clone(), f14(Str::lit("unit"))) {
                                                                        Some(f14(Str::lit("Unit")))
                                                                    } else {
                                                                        if f61(v0.clone(), f14(Str::lit("contract_violation"))) {
                                                                            Some(f14(Str::lit("ContractViolation")))
                                                                        } else {
                                                                            if f61(v0, f14(Str::lit("reasoning_failure"))) {
                                                                                Some(f14(Str::lit("ReasoningFailure")))
                                                                            } else {
                                                                                None::<Bytes>
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
                }
            }
        }
    }
}

pub fn f72(v0: Option<Adt0>) -> Option<Adt0> {
    let m190 = v0;
    match m190 {
        None => {
            None::<Adt0>
        }
        Some(v1) => {
            f139(v1)
        }
    }
}

pub fn f73(v0: Adt0) -> R<u64> {
    let m193 = {
        let (h192, _) = f140(Fn15::F141, 1000000u64, Adt8::C0(v0, 0u64))?;
        h192
    };
    match m193 {
        Adt8::C0(_, h191) => {
            Ok(h191)
        }
    }
}

pub fn f74(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f75(v0: Option<Adt0>) -> bool {
    let m194 = v0;
    match m194 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f76(v0: Bytes) -> R<bool> {
    let a196 = {
        let a195 = v0.clone();
        length_bytes(a195)
    };
    let a197 = 0u64;
    if nat_eq(a196, a197) {
        Ok(false)
    } else {
        if f142(f143(v0.clone(), 0u64)?) {
            let a199 = f144(v0.clone(), 1u64)?;
            let a200 = {
                let a198 = v0.clone();
                length_bytes(a198)
            };
            if nat_eq(a199, a200) {
                let a201 = f145(v0);
                Ok(bool_not(a201))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f77(v0: Adt0) -> Option<Bytes> {
    let m202 = v0;
    match m202 {
        Adt0::C0(_) => {
            None::<Bytes>
        }
        Adt0::C1(_, _) => {
            None::<Bytes>
        }
        Adt0::C2(v1) => {
            Some(v1)
        }
        Adt0::C3(_) => {
            None::<Bytes>
        }
        Adt0::C4(_) => {
            None::<Bytes>
        }
        Adt0::C5 => {
            None::<Bytes>
        }
        Adt0::C6(_, _) => {
            None::<Bytes>
        }
        Adt0::C7(_, _, _) => {
            None::<Bytes>
        }
    }
}

pub fn f78(v0: Fn2, p206: Adt1, p207: List<Adt1>) -> Adt1 {
    let m205 = (p206, p207);
    {
        let mut m205 = m205;
        loop {
            let step = {
                let (v1, v2) = m205;
                let m203 = v2;
                match m203.uncons() {
                    None => {
                        Err::<(Adt1, List<Adt1>), Adt1>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<Adt1>), Adt1>(({
                            let c204: Fn2 = v0.clone();
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

pub fn f79(v0: Adt1, v1: Adt1) -> Adt1 {
    Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f80(v0: Fn3, p211: Adt1, p212: List<(Bytes, Adt1)>) -> Adt1 {
    let m210 = (p211, p212);
    {
        let mut m210 = m210;
        loop {
            let step = {
                let (v1, v2) = m210;
                let m208 = v2;
                match m208.uncons() {
                    None => {
                        Err::<(Adt1, List<(Bytes, Adt1)>), Adt1>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<(Bytes, Adt1)>), Adt1>(({
                            let c209: Fn3 = v0.clone();
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

pub fn f81(v0: Bytes, v1: Adt1, v2: (Bytes, Adt1)) -> Adt1 {
    if f61({
        let (h213, _) = v2;
        h213
    }, v0) {
        let (_, h214) = v2;
        h214
    } else {
        v1
    }
}

pub fn f82(v0: Adt0, v1: Option<Adt0>, v2: Bytes) -> R<Adt2> {
    let m215 = v1;
    match m215 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v3) => {
            let m216 = f13(f52(v0.clone(), v3)?, f14(Str::lit("properties")));
            match m216 {
                None => {
                    Ok(Adt2::C5)
                }
                Some(v4) => {
                    let m217 = f13(v4, v2);
                    match m217 {
                        None => {
                            Ok(Adt2::C5)
                        }
                        Some(v5) => {
                            f53(v0, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: Bytes) -> Adt2 {
    if f61(v1.clone(), f14(Str::lit("element"))) {
        Adt2::C1
    } else {
        if f61(v1.clone(), f14(Str::lit("binder"))) {
            Adt2::C3
        } else {
            if f61(v1.clone(), f14(Str::lit("parameters"))) {
                Adt2::C8
            } else {
                if f61(v1.clone(), f14(Str::lit("arguments"))) {
                    Adt2::C6
                } else {
                    if f61(v1.clone(), f14(Str::lit("type_arguments"))) {
                        Adt2::C7
                    } else {
                        if f61(v1.clone(), f14(Str::lit("fields"))) {
                            Adt2::C11
                        } else {
                            if f61(v1.clone(), f14(Str::lit("branches"))) {
                                Adt2::C13
                            } else {
                                if f61(v1.clone(), f14(Str::lit("result"))) {
                                    Adt2::C1
                                } else {
                                    if if f61(v1.clone(), f14(Str::lit("type"))) {
                                        true
                                    } else {
                                        if f61(v1.clone(), f14(Str::lit("constructor"))) {
                                            true
                                        } else {
                                            f61(v1.clone(), f14(Str::lit("model")))
                                        }
                                    } {
                                        Adt2::C2
                                    } else {
                                        if f61(v1, f14(Str::lit("function"))) {
                                            if f61(v0, f14(Str::lit("apply"))) {
                                                Adt2::C0
                                            } else {
                                                Adt2::C2
                                            }
                                        } else {
                                            Adt2::C0
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

pub fn f84(v0: Bytes) -> Adt2 {
    if f61(v0.clone(), f14(Str::lit("parameters"))) {
        Adt2::C9
    } else {
        if f61(v0.clone(), f14(Str::lit("arguments"))) {
            Adt2::C10
        } else {
            if f61(v0.clone(), f14(Str::lit("member"))) {
                Adt2::C2
            } else {
                if f61(v0, f14(Str::lit("name"))) {
                    Adt2::C5
                } else {
                    Adt2::C1
                }
            }
        }
    }
}

pub fn f85(v0: Adt0, v1: Adt2) -> R<Option<Adt0>> {
    let m218 = v1;
    match m218 {
        Adt2::C0 => {
            Ok(f16(v0, f14(Str::lit("term"))))
        }
        Adt2::C1 => {
            Ok(f16(v0, f14(Str::lit("type"))))
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(None::<Adt0>)
        }
        Adt2::C4(v2) => {
            if f87(v0, v2.clone())? {
                Ok(Some(v2))
            } else {
                Ok(None::<Adt0>)
            }
        }
        Adt2::C5 => {
            Ok(None::<Adt0>)
        }
        Adt2::C6 => {
            Ok(None::<Adt0>)
        }
        Adt2::C7 => {
            Ok(None::<Adt0>)
        }
        Adt2::C8 => {
            Ok(None::<Adt0>)
        }
        Adt2::C9 => {
            Ok(None::<Adt0>)
        }
        Adt2::C10 => {
            Ok(None::<Adt0>)
        }
        Adt2::C11 => {
            Ok(None::<Adt0>)
        }
        Adt2::C12 => {
            Ok(None::<Adt0>)
        }
        Adt2::C13 => {
            Ok(None::<Adt0>)
        }
        Adt2::C14 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f86(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Option<Adt0>> {
    let m219 = f72(f13(f52(v0.clone(), v1)?, f14(Str::lit("oneOf"))));
    match m219 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v3) => {
            f146(Fn16::F147(v2, v0), None::<Adt0>, f137(v3))
        }
    }
}

pub fn f87(v0: Adt0, v1: Adt0) -> R<bool> {
    let m220 = f72(f13(f52(v0.clone(), v1)?, f14(Str::lit("oneOf"))));
    match m220 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            f148(Fn17::F149(v0), true, f137(v2))
        }
    }
}

pub fn f88(v0: Bytes, v1: Bytes, v2: List<Adt1>, v3: Adt1) -> List<Adt1> {
    List::cons(f27(v0), List::cons(v3, List::cons(f27(v1), v2)))
}

pub fn f89(v0: Fn4, p224: (bool, List<Adt1>), p225: List<Adt1>) -> (bool, List<Adt1>) {
    let m223 = (p224, p225);
    {
        let mut m223 = m223;
        loop {
            let step = {
                let (v1, v2) = m223;
                let m221 = v2;
                match m221.uncons() {
                    None => {
                        Err::<((bool, List<Adt1>), List<Adt1>), (bool, List<Adt1>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt1>), List<Adt1>), (bool, List<Adt1>)>(({
                            let c222: Fn4 = v0.clone();
                            c222.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: (bool, List<Adt1>), v2: Adt1) -> (bool, List<Adt1>) {
    let (h226, _) = v1;
    if h226 {
        (false, List::cons(v2, {
            let (_, h227) = v1;
            h227
        }))
    } else {
        (false, List::cons(v2, List::cons(f27(v0), {
            let (_, h228) = v1;
            h228
        })))
    }
}

pub fn f91(v0: Bytes, v1: Bytes) -> Bytes {
    let a229 = v0;
    let a230 = v1;
    append_bytes(a229, a230)
}

pub fn f92(v0: Fn5, p234: (bool, Adt1), p235: List<Adt1>) -> (bool, Adt1) {
    let m233 = (p234, p235);
    {
        let mut m233 = m233;
        loop {
            let step = {
                let (v1, v2) = m233;
                let m231 = v2;
                match m231.uncons() {
                    None => {
                        Err::<((bool, Adt1), List<Adt1>), (bool, Adt1)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt1), List<Adt1>), (bool, Adt1)>(({
                            let c232: Fn5 = v0.clone();
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

pub fn f93(v0: (bool, Adt1), v1: Adt1) -> (bool, Adt1) {
    f150(v0, v1)
}

pub fn f94(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f95(v0: u64) -> Bytes {
    f151(f152(), 10u64, v0)
}

pub fn f96(v0: Fn6, p239: List<(Bytes, Adt1)>, p240: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let m238 = (p239, p240);
    {
        let mut m238 = m238;
        loop {
            let step = {
                let (v1, v2) = m238;
                let m236 = v2;
                match m236.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), R<List<(Bytes, Adt1)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), R<List<(Bytes, Adt1)>>>(({
                            let c237: Fn6 = v0.clone();
                            c237.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Option<Bytes>, v1: List<(Bytes, Adt1)>, v2: (Bytes, Adt1)) -> R<List<(Bytes, Adt1)>> {
    let m241 = v0;
    match m241 {
        None => {
            Ok(List::cons((f153({
                let (h248, _) = v2.clone();
                h248
            }), f154({
                let (h249, _) = v2;
                h249
            }, {
                let (_, h250) = v2;
                h250
            })?), v1))
        }
        Some(_) => {
            let a243 = {
                let (h242, _) = v2.clone();
                h242
            };
            let a244 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
            if equal(a243, a244) {
                Ok(v1)
            } else {
                Ok(List::cons((f153({
                    let (h245, _) = v2.clone();
                    h245
                }), f154({
                    let (h246, _) = v2;
                    h246
                }, {
                    let (_, h247) = v2;
                    h247
                })?), v1))
            }
        }
    }
}

pub fn f98(v0: Fn7, p254: List<Adt1>, p255: List<(Bytes, Adt1)>) -> List<Adt1> {
    let m253 = (p254, p255);
    {
        let mut m253 = m253;
        loop {
            let step = {
                let (v1, v2) = m253;
                let m251 = v2;
                match m251.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(({
                            let c252: Fn7 = v0.clone();
                            c252.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: List<Adt1>, v1: (Bytes, Adt1)) -> List<Adt1> {
    List::cons({
        let (_, h256) = v1;
        h256
    }, v0)
}

pub fn f100(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v1: List<List<(Bytes, Adt1)>> = f155(f156(Fn18::F157, List::<List<(Bytes, Adt1)>>::nil(), v0.clone()));
    let m261 = {
        let (h257, _) = f158(Fn19::F159, {
            let a259 = {
                let a258 = v0;
                length_list(a258)
            };
            let a260 = 1u64;
            nat_add(a259, a260)?
        }, v1)?;
        h257
    };
    match m261.uncons() {
        None => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f101(v0: Fn8, p266: u64, p267: Adt6) -> (Adt6, bool) {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v1, v2) = m265;
                let m262 = v1;
                if m262 == 0 {
                    Err::<(u64, Adt6), (Adt6, bool)>((v2, false))
                } else {
                    let v3 = m262 - 1;
                    let m264 = {
                        let c263: Fn8 = v0.clone();
                        c263.apply(v2.clone())
                    };
                    match m264 {
                        None => {
                            Err::<(u64, Adt6), (Adt6, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), (Adt6, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Bytes, v1: Adt6) -> Option<Adt6> {
    f160(v0, v1)
}

pub fn f103(v0: Fn9, p272: u64, p273: Adt0) -> R<(Adt0, bool)> {
    let m271 = (p272, p273);
    {
        let mut m271 = m271;
        loop {
            let step = {
                let (v1, v2) = m271;
                let m268 = v1;
                if m268 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m268 - 1;
                    let m270 = {
                        let c269: Fn9 = v0.clone();
                        c269.apply(v2.clone())?
                    };
                    match m270 {
                        None => {
                            Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt0), R<(Adt0, bool)>>((v3, v4))
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

pub fn f104(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    f161(v0, v1)
}

pub fn f105(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let m279 = {
            let (h275, _) = f162(Fn20::F163(v0.clone()), {
                let a277 = {
                    let a276 = v0.clone();
                    length_bytes(a276)
                };
                let a278 = 1u64;
                nat_add(a277, a278)?
            }, Adt9::C0(0u64, 0u64))?;
            h275
        };
        match m279 {
            Adt9::C0(_, h274) => {
                h274
            }
        }
    };
    Ok(f164(v0.clone(), v1, {
        let a281 = {
            let a280 = v0;
            length_bytes(a280)
        };
        let a282 = v1;
        nat_sub(a281, a282)
    }))
}

pub fn f106(v0: u64, v1: u64, v2: Adt3, v3: Adt4, v4: List<Adt4>) -> R<Adt3> {
    let v5: u64 = match v3.clone() {
        Adt4::C0(h283, _, _) => {
            h283
        }
    };
    let v6: bool = match v3.clone() {
        Adt4::C0(_, h284, _) => {
            h284
        }
    };
    let m286 = match v3 {
        Adt4::C0(_, _, h285) => {
            h285
        }
    };
    match m286 {
        Adt1::C0 => {
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h287, _, _) => {
                    h287
                }
            }, match v2 {
                Adt3::C0(_, h288, _) => {
                    h288
                }
            }, v4))
        }
        Adt1::C1(v7) => {
            Ok(Adt3::C0(List::cons(v7.clone(), match v2.clone() {
                Adt3::C0(h289, _, _) => {
                    h289
                }
            }), {
                let a291 = match v2 {
                    Adt3::C0(_, h290, _) => {
                        h290
                    }
                };
                let a292 = f165(v7)?;
                nat_add(a291, a292)?
            }, v4))
        }
        Adt1::C2(r309, r310) => {
            let v8 = (*r309).clone();
            let v9 = (*r310).clone();
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h293, _, _) => {
                    h293
                }
            }, match v2 {
                Adt3::C0(_, h294, _) => {
                    h294
                }
            }, List::cons(Adt4::C0(v5, v6, v8), List::cons(Adt4::C0(v5, v6, v9), v4))))
        }
        Adt1::C3(v10, r311) => {
            let v11 = (*r311).clone();
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h295, _, _) => {
                    h295
                }
            }, match v2 {
                Adt3::C0(_, h296, _) => {
                    h296
                }
            }, List::cons(Adt4::C0({
                let a297 = v5;
                let a298 = v10;
                nat_add(a297, a298)?
            }, v6, v11), v4)))
        }
        Adt1::C4 => {
            if v6 {
                Ok(Adt3::C0(List::cons({
                    let a299 = Str::lit(" ");
                    utf8_encode(a299)
                }, match v2.clone() {
                    Adt3::C0(h300, _, _) => {
                        h300
                    }
                }), {
                    let a302 = match v2 {
                        Adt3::C0(_, h301, _) => {
                            h301
                        }
                    };
                    let a303 = 1u64;
                    nat_add(a302, a303)?
                }, v4))
            } else {
                f166(v2, v5, v4)
            }
        }
        Adt1::C5 => {
            if v6 {
                Ok(Adt3::C0(match v2.clone() {
                    Adt3::C0(h304, _, _) => {
                        h304
                    }
                }, match v2 {
                    Adt3::C0(_, h305, _) => {
                        h305
                    }
                }, v4))
            } else {
                f166(v2, v5, v4)
            }
        }
        Adt1::C6(r312) => {
            let v12 = (*r312).clone();
            let v13: bool = f167(v1, v0, match v2.clone() {
                Adt3::C0(_, h306, _) => {
                    h306
                }
            }, v4.clone(), Adt4::C0(v5, true, v12.clone()))?;
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h307, _, _) => {
                    h307
                }
            }, match v2 {
                Adt3::C0(_, h308, _) => {
                    h308
                }
            }, List::cons(Adt4::C0(v5, v13, v12), v4)))
        }
    }
}

pub fn f107(v0: Fn10, p317: u64, p318: List<Bytes>) -> (List<Bytes>, bool) {
    let m316 = (p317, p318);
    {
        let mut m316 = m316;
        loop {
            let step = {
                let (v1, v2) = m316;
                let m313 = v1;
                if m313 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m313 - 1;
                    let m315 = {
                        let c314: Fn10 = v0.clone();
                        c314.apply(v2.clone())
                    };
                    match m315 {
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
                Ok(next) => { m316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: List<Bytes>) -> Option<List<Bytes>> {
    f168(v0)
}

pub fn f109(v0: Fn11, p322: List<Bytes>, p323: List<Bytes>) -> List<Bytes> {
    let m321 = (p322, p323);
    {
        let mut m321 = m321;
        loop {
            let step = {
                let (v1, v2) = m321;
                let m319 = v2;
                match m319.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c320: Fn11 = v0.clone();
                            c320.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f111(v0: Adt0, v1: List<Bytes>) -> R<bool> {
    Ok(f74({
        let a325 = f73(v0.clone())?;
        let a326 = {
            let a324 = v1.clone();
            length_list(a324)
        };
        nat_eq(a325, a326)
    }, f169(v0, v1)))
}

pub fn f112(v0: Str) -> List<Bytes> {
    List::cons(f14(Str::lit("kind")), List::cons(f14(v0), List::<Bytes>::nil()))
}

pub fn f113(v0: Bytes) -> R<bool> {
    Ok(f74(f76(v0.clone())?, f74(!f170(v0.clone(), f171()), !f61(v0, f14(Str::lit("false"))))))
}

pub fn f114(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a327 = v0.clone();
        length_bytes(a327)
    };
    Ok(f74({
        let a328 = 0u64;
        let a329 = v1;
        nat_lt(a328, a329)
    }, f74(f172(v0.clone(), 0u64, false)?, {
        let a330 = v1;
        let a331 = 1u64;
        if nat_eq(a330, a331) {
            true
        } else {
            !f173(v0, f14(Str::lit("0")))
        }
    })))
}

pub fn f115(v0: Bytes) -> R<bool> {
    Ok(f74({
        let a336 = {
            let a333 = {
                let a332 = v0.clone();
                length_bytes(a332)
            };
            let a334 = 2u64;
            let a335 = 0u64;
            nat_rem(a333, a334, a335)
        };
        let a337 = 0u64;
        nat_eq(a336, a337)
    }, f172(v0, 0u64, true)?))
}

pub fn f116(v0: Str, v1: Str) -> List<Bytes> {
    List::cons(f14(Str::lit("kind")), List::cons(f14(v0), List::cons(f14(v1), List::<Bytes>::nil())))
}

pub fn f117(v0: Bytes) -> R<bool> {
    if f173(v0.clone(), f14(Str::lit("-"))) {
        let v1: Bytes = f164(v0.clone(), 1u64, {
            let a339 = {
                let a338 = v0;
                length_bytes(a338)
            };
            let a340 = 1u64;
            nat_sub(a339, a340)
        });
        Ok(f74(f114(v1.clone())?, !f61(v1, f14(Str::lit("0")))))
    } else {
        f114(v0)
    }
}

pub fn f118(v0: Str, v1: Str, v2: Str) -> List<Bytes> {
    List::cons(f14(Str::lit("kind")), List::cons(f14(v0), List::cons(f14(v1), List::cons(f14(v2), List::<Bytes>::nil()))))
}

pub fn f119(v0: Option<Adt0>) -> R<bool> {
    let m341 = v0;
    match m341 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f28(v1)
        }
    }
}

pub fn f120(v0: Adt0, v1: Bytes, v2: Fn12) -> R<bool> {
    let m342 = f72(f51(v0, v1));
    match m342 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            f148(Fn17::F174(v2), true, f137(v3))
        }
    }
}

pub fn f121(v0: Adt0) -> R<bool> {
    f28(v0)
}

pub fn f122(v0: Adt0) -> R<bool> {
    f175(v0)
}

pub fn f123(v0: Adt0, v1: Bytes) -> bool {
    f75(f72(f51(v0, v1)))
}

pub fn f124(v0: Adt0, v1: List<Bytes>) -> R<bool> {
    Ok(f74({
        let a346 = f73(v0.clone())?;
        let a347 = {
            let a344 = {
                let a343 = v1.clone();
                length_list(a343)
            };
            let a345 = if f75(f51(v0.clone(), f14(Str::lit("type_arguments")))) {
                1u64
            } else {
                0u64
            };
            nat_add(a344, a345)?
        };
        nat_eq(a346, a347)
    }, f169(v0, v1)))
}

pub fn f125(v0: Adt0) -> R<bool> {
    let m348 = f51(v0, f14(Str::lit("type_arguments")));
    match m348 {
        None => {
            Ok(true)
        }
        Some(v1) => {
            let m349 = f139(v1);
            match m349 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    let a350 = 0u64;
                    let a351 = f73(v2)?;
                    Ok(nat_lt(a350, a351))
                }
            }
        }
    }
}

pub fn f126(v0: Option<Adt0>) -> R<bool> {
    let m352 = v0;
    match m352 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            let m353 = f26(v1)?;
            match m353 {
                None => {
                    Ok(false)
                }
                Some(_) => {
                    Ok(true)
                }
            }
        }
    }
}

pub fn f127(v0: Adt0) -> R<bool> {
    f176(v0)
}

pub fn f128(v0: Adt0) -> R<bool> {
    f175(v0)
}

pub fn f129(v0: Adt0) -> R<bool> {
    f177(v0)
}

pub fn f130(v0: Fn13, p358: u64, p359: Adt7) -> R<(Adt7, bool)> {
    let m357 = (p358, p359);
    {
        let mut m357 = m357;
        loop {
            let step = {
                let (v1, v2) = m357;
                let m354 = v1;
                if m354 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m354 - 1;
                    let m356 = {
                        let c355: Fn13 = v0.clone();
                        c355.apply(v2.clone())?
                    };
                    match m356 {
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
                Ok(next) => { m357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: u64, v1: Bytes, v2: Adt7) -> R<Option<Adt7>> {
    f178(v1, v0, v2)
}

pub fn f132(v0: Bytes, v1: Bytes) -> Bytes {
    let a360 = v0;
    let a361 = v1;
    append_bytes(a360, a361)
}

pub fn f133(v0: Str) -> Bytes {
    let a362 = v0;
    utf8_encode(a362)
}

pub fn f134(v0: List<Bytes>) -> Bytes {
    f55(f56(v0))
}

pub fn f135(v0: Fn14, p366: List<Adt1>, p367: List<Adt0>) -> List<Adt1> {
    let m365 = (p366, p367);
    {
        let mut m365 = m365;
        loop {
            let step = {
                let (v1, v2) = m365;
                let m363 = v2;
                match m363.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt0>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt0>), List<Adt1>>(({
                            let c364: Fn14 = v0.clone();
                            c364.apply(v1, v3)
                        }, v4))
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

pub fn f136(v0: List<Adt1>, v1: Adt0) -> List<Adt1> {
    let a369 = v0;
    let a370 = List::cons(f27({
        let m368 = f77(v1);
        match m368 {
            None => {
                f14(Str::lit(""))
            }
            Some(v2) => {
                v2
            }
        }
    }), List::<Adt1>::nil());
    append_list(a369, a370)
}

pub fn f137(v0: Adt0) -> List<Adt0> {
    f179({
        let m373 = {
            let (h372, _) = f180(Fn21::F181, 1000000u64, Adt10::C0(v0, List::<Adt0>::nil()));
            h372
        };
        match m373 {
            Adt10::C0(_, h371) => {
                h371
            }
        }
    })
}

pub fn f138(v0: Bytes) -> bool {
    f170(v0, List::cons(f14(Str::lit("Bool")), List::cons(f14(Str::lit("Bytes")), List::cons(f14(Str::lit("Int")), List::cons(f14(Str::lit("Int8")), List::cons(f14(Str::lit("Int16")), List::cons(f14(Str::lit("Int32")), List::cons(f14(Str::lit("Int64")), List::cons(f14(Str::lit("Nat")), List::cons(f14(Str::lit("Ordering")), List::cons(f14(Str::lit("Prop")), List::cons(f14(Str::lit("String")), List::cons(f14(Str::lit("Type")), List::cons(f14(Str::lit("UInt8")), List::cons(f14(Str::lit("UInt16")), List::cons(f14(Str::lit("UInt32")), List::cons(f14(Str::lit("UInt64")), List::cons(f14(Str::lit("Unit")), List::cons(f14(Str::lit("ContractViolation")), List::cons(f14(Str::lit("ReasoningFailure")), List::cons(f14(Str::lit("List")), List::cons(f14(Str::lit("Option")), List::cons(f14(Str::lit("Result")), List::cons(f14(Str::lit("Set")), List::cons(f14(Str::lit("Map")), List::cons(f14(Str::lit("Fn")), List::<Bytes>::nil()))))))))))))))))))))))))))
}

pub fn f139(v0: Adt0) -> Option<Adt0> {
    let m374 = v0;
    match m374 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(r375) => {
            let v1 = (*r375).clone();
            Some(v1)
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6(_, _) => {
            None::<Adt0>
        }
        Adt0::C7(_, _, _) => {
            None::<Adt0>
        }
    }
}

pub fn f140(v0: Fn15, p380: u64, p381: Adt8) -> R<(Adt8, bool)> {
    let m379 = (p380, p381);
    {
        let mut m379 = m379;
        loop {
            let step = {
                let (v1, v2) = m379;
                let m376 = v1;
                if m376 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m376 - 1;
                    let m378 = {
                        let c377: Fn15 = v0.clone();
                        c377.apply(v2.clone())?
                    };
                    match m378 {
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
                Ok(next) => { m379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt8) -> R<Option<Adt8>> {
    f182(v0)
}

pub fn f142(v0: u64) -> bool {
    let a382 = 64u64;
    let a383 = v0;
    if if nat_lt(a382, a383) {
        let a384 = v0;
        let a385 = 91u64;
        nat_lt(a384, a385)
    } else {
        false
    } {
        true
    } else {
        let a386 = 96u64;
        let a387 = v0;
        if if nat_lt(a386, a387) {
            let a388 = v0;
            let a389 = 123u64;
            nat_lt(a388, a389)
        } else {
            false
        } {
            true
        } else {
            let a390 = v0;
            let a391 = 95u64;
            nat_eq(a390, a391)
        }
    }
}

pub fn f143(v0: Bytes, v1: u64) -> R<u64> {
    let m392 = f183(v0, v1);
    match m392 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a437 = {
                let a413 = {
                    let a401 = {
                        let a395 = {
                            let a393 = v3;
                            let a394 = 128u8;
                            fixed_u8::bit_and(a393, a394)
                        };
                        let a396 = 128u8;
                        if equal(a395, a396) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a402 = {
                        let a399 = {
                            let a397 = v3;
                            let a398 = 64u8;
                            fixed_u8::bit_and(a397, a398)
                        };
                        let a400 = 64u8;
                        if equal(a399, a400) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a401, a402)?
                };
                let a414 = {
                    let a411 = {
                        let a405 = {
                            let a403 = v3;
                            let a404 = 32u8;
                            fixed_u8::bit_and(a403, a404)
                        };
                        let a406 = 32u8;
                        if equal(a405, a406) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a412 = {
                        let a409 = {
                            let a407 = v3;
                            let a408 = 16u8;
                            fixed_u8::bit_and(a407, a408)
                        };
                        let a410 = 16u8;
                        if equal(a409, a410) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a411, a412)?
                };
                nat_add(a413, a414)?
            };
            let a438 = {
                let a435 = {
                    let a423 = {
                        let a417 = {
                            let a415 = v3;
                            let a416 = 8u8;
                            fixed_u8::bit_and(a415, a416)
                        };
                        let a418 = 8u8;
                        if equal(a417, a418) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a424 = {
                        let a421 = {
                            let a419 = v3;
                            let a420 = 4u8;
                            fixed_u8::bit_and(a419, a420)
                        };
                        let a422 = 4u8;
                        if equal(a421, a422) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a423, a424)?
                };
                let a436 = {
                    let a433 = {
                        let a427 = {
                            let a425 = v3;
                            let a426 = 2u8;
                            fixed_u8::bit_and(a425, a426)
                        };
                        let a428 = 2u8;
                        if equal(a427, a428) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a434 = {
                        let a431 = {
                            let a429 = v3;
                            let a430 = 1u8;
                            fixed_u8::bit_and(a429, a430)
                        };
                        let a432 = 1u8;
                        if equal(a431, a432) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a433, a434)?
                };
                nat_add(a435, a436)?
            };
            nat_add(a437, a438)
        }
    }
}

pub fn f144(v0: Bytes, v1: u64) -> R<u64> {
    let (h439, _) = f184(Fn22::F185(v0.clone()), {
        let a441 = {
            let a440 = v0;
            length_bytes(a440)
        };
        let a442 = 1u64;
        nat_add(a441, a442)?
    }, v1)?;
    Ok(h439)
}

pub fn f145(v0: Bytes) -> bool {
    let a443 = v0.clone();
    let a444 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a443, a444) {
        true
    } else {
        let a445 = v0;
        let a446 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a445, a446)
    }
}

pub fn f146(v0: Fn16, p450: Option<Adt0>, p451: List<Adt0>) -> R<Option<Adt0>> {
    let m449 = (p450, p451);
    {
        let mut m449 = m449;
        loop {
            let step = {
                let (v1, v2) = m449;
                let m447 = v2;
                match m447.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(({
                            let c448: Fn16 = v0.clone();
                            c448.apply(v1, v3)?
                        }, v4))
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

pub fn f147(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt0) -> R<Option<Adt0>> {
    let m452 = v2.clone();
    match m452 {
        None => {
            let v5: Adt0 = f52(v1, v3)?;
            if f186(v5.clone(), v0)? {
                Ok(Some(v5))
            } else {
                Ok(None::<Adt0>)
            }
        }
        Some(_) => {
            Ok(v2)
        }
    }
}

pub fn f148(v0: Fn17, p456: bool, p457: List<Adt0>) -> R<bool> {
    let m455 = (p456, p457);
    {
        let mut m455 = m455;
        loop {
            let step = {
                let (v1, v2) = m455;
                let m453 = v2;
                match m453.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), R<bool>>(({
                            let c454: Fn17 = v0.clone();
                            c454.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Adt0, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let m458 = f13(f52(v0, v2)?, f14(Str::lit("properties")));
        match m458 {
            None => {
                Ok(false)
            }
            Some(v3) => {
                Ok(f75(f13(v3, f14(Str::lit("kind")))))
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f150(v0: (bool, Adt1), v1: Adt1) -> (bool, Adt1) {
    let (h459, _) = v0;
    if h459 {
        (false, Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C0)))
    } else {
        (false, Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(f45(Str::lit(",")))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C4), std::rc::Rc::new({
            let (_, h460) = v0;
            h460
        })))))))
    }
}

pub fn f151(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a461 = v2;
    let a462 = 0u64;
    if nat_eq(a461, a462) {
        f187(v0, 0u64)
    } else {
        let (_, h463) = {
            let (h464, _) = f188(Fn23::F189(v1, v0), v2, (v2, Bytes::lit(&[])));
            h464
        };
        h463
    }
}

pub fn f152() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f153(v0: Bytes) -> Bytes {
    f91(f190(f191(v0.clone())), v0)
}

pub fn f154(v0: Bytes, v1: Adt1) -> R<Adt1> {
    Ok(Adt1::C2(std::rc::Rc::new(Adt1::C1(f91(f44(v0)?, f45(Str::lit(": "))))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C0)))))
}

pub fn f155(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f192(Fn24::F193, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f156(v0: Fn18, p468: List<List<(Bytes, Adt1)>>, p469: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    let m467 = (p468, p469);
    {
        let mut m467 = m467;
        loop {
            let step = {
                let (v1, v2) = m467;
                let m465 = v2;
                match m465.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(({
                            let c466: Fn18 = v0.clone();
                            c466.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<List<(Bytes, Adt1)>>, v1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt1)>::nil()), v0)
}

pub fn f158(v0: Fn19, p474: u64, p475: List<List<(Bytes, Adt1)>>) -> R<(List<List<(Bytes, Adt1)>>, bool)> {
    let m473 = (p474, p475);
    {
        let mut m473 = m473;
        loop {
            let step = {
                let (v1, v2) = m473;
                let m470 = v1;
                if m470 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m470 - 1;
                    let m472 = {
                        let c471: Fn19 = v0.clone();
                        c471.apply(v2.clone())?
                    };
                    match m472 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    f194(v0)
}

pub fn f160(v0: Bytes, v1: Adt6) -> Option<Adt6> {
    let m477 = match v1.clone() {
        Adt6::C0(_, h476) => {
            h476
        }
    };
    match m477 {
        None => {
            let m479 = f195(match v1 {
                Adt6::C0(h478, _) => {
                    h478
                }
            });
            match m479 {
                None => {
                    None::<Adt6>
                }
                Some(v3) => {
                    if f61(match v3.clone() {
                        Adt12::C0(h480, _, _) => {
                            h480
                        }
                    }, v0) {
                        Some(Adt6::C0(Adt0::C5, Some(match v3 {
                            Adt12::C0(_, h481, _) => {
                                h481
                            }
                        })))
                    } else {
                        Some(Adt6::C0(match v3 {
                            Adt12::C0(_, _, h482) => {
                                h482
                            }
                        }, None::<Adt0>))
                    }
                }
            }
        }
        Some(_) => {
            None::<Adt6>
        }
    }
}

pub fn f161(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    let m483 = f69(f13(v1, f14(Str::lit("$ref"))));
    match m483 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v2) => {
            Ok(f16(v0, f105(v2)?))
        }
    }
}

pub fn f162(v0: Fn20, p488: u64, p489: Adt9) -> R<(Adt9, bool)> {
    let m487 = (p488, p489);
    {
        let mut m487 = m487;
        loop {
            let step = {
                let (v1, v2) = m487;
                let m484 = v1;
                if m484 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m484 - 1;
                    let m486 = {
                        let c485: Fn20 = v0.clone();
                        c485.apply(v2.clone())?
                    };
                    match m486 {
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
                Ok(next) => { m487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: Adt9) -> R<Option<Adt9>> {
    f196(v0, v1)
}

pub fn f164(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m493 = {
        let a490 = v0;
        let a491 = v1;
        let a492 = v2;
        slice_bytes(a490, a491, a492)
    };
    match m493 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f165(v0: Bytes) -> R<u64> {
    let (_, h494) = {
        let (h495, _) = f197(Fn25::F198(v0.clone()), {
            let a496 = v0;
            length_bytes(a496)
        }, (0u64, 0u64))?;
        h495
    };
    Ok(h494)
}

pub fn f166(v0: Adt3, v1: u64, v2: List<Adt4>) -> R<Adt3> {
    Ok(Adt3::C0(f199(match v0 {
        Adt3::C0(h497, _, _) => {
            h497
        }
    }, v1)?, v1, v2))
}

pub fn f167(v0: u64, v1: u64, v2: u64, v3: List<Adt4>, v4: Adt4) -> R<bool> {
    let v5: Adt11 = {
        let (h498, _) = f200(Fn26::F201, {
            let a499 = v0;
            let a500 = 1u64;
            nat_add(a499, a500)?
        }, Adt11::C0(0u64, {
            let a501 = v1;
            let a502 = v2;
            nat_sub(a501, a502)
        }, {
            let a503 = v1;
            let a504 = v2;
            nat_lt(a503, a504)
        }, List::cons(v4, List::<Adt4>::nil()), v3, None::<bool>))?;
        h498
    };
    let m506 = match v5 {
        Adt11::C0(_, _, _, _, _, h505) => {
            h505
        }
    };
    match m506 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f168(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m507 = v0.clone();
    match m507.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m508 = v2;
            match m508.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f202(Fn27::F203, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m510 = {
                        let (h509, _) = v5;
                        h509
                    };
                    match m510 {
                        None => {
                            Some(f56({
                                let (_, h512) = v5;
                                h512
                            }))
                        }
                        Some(v6) => {
                            Some(f56(List::cons(v6, {
                                let (_, h511) = v5;
                                h511
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f169(v0: Adt0, v1: List<Bytes>) -> bool {
    f204(Fn28::F205(v0), true, v1)
}

pub fn f170(v0: Bytes, v1: List<Bytes>) -> bool {
    f204(Fn28::F206(v0), false, v1)
}

pub fn f171() -> List<Bytes> {
    List::cons(f14(Str::lit("let")), List::cons(f14(Str::lit("if")), List::cons(f14(Str::lit("match")), List::cons(f14(Str::lit("fun")), List::cons(f14(Str::lit("forall")), List::cons(f14(Str::lit("nil")), List::cons(f14(Str::lit("cons")), List::cons(f14(Str::lit("first")), List::cons(f14(Str::lit("second")), List::cons(f14(Str::lit("not")), List::cons(f14(Str::lit("apply")), List::cons(f14(Str::lit("ctor")), List::cons(f14(Str::lit("record")), List::cons(f14(Str::lit("project")), List::cons(f14(Str::lit("prim")), List::cons(f14(Str::lit("ref")), List::cons(f14(Str::lit("checked")), List::cons(f14(Str::lit("then")), List::cons(f14(Str::lit("else")), List::cons(f14(Str::lit("with")), List::cons(f14(Str::lit("true")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f172(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = {
        let (h513, _) = f184(Fn22::F207(v2, v0.clone()), {
            let a515 = {
                let a514 = v0.clone();
                length_bytes(a514)
            };
            let a516 = 1u64;
            nat_add(a515, a516)?
        }, v1)?;
        h513
    };
    let a518 = v3;
    let a519 = {
        let a517 = v0;
        length_bytes(a517)
    };
    Ok(nat_eq(a518, a519))
}

pub fn f173(v0: Bytes, v1: Bytes) -> bool {
    let m524 = {
        let a521 = v0;
        let a522 = 0u64;
        let a523 = {
            let a520 = v1.clone();
            length_bytes(a520)
        };
        slice_bytes(a521, a522, a523)
    };
    match m524 {
        None => {
            false
        }
        Some(v2) => {
            let a525 = v2;
            let a526 = v1;
            equal(a525, a526)
        }
    }
}

pub fn f174(v0: Fn12, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let c527: Fn12 = v0;
        c527.apply(v2)
    } else {
        Ok(false)
    }
}

pub fn f175(v0: Adt0) -> R<bool> {
    let m528 = f77(v0);
    match m528 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f76(v1)
        }
    }
}

pub fn f176(v0: Adt0) -> R<bool> {
    let m529 = f50(v0);
    match m529 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f74(f111(v1.clone(), List::cons(f14(Str::lit("field")), List::cons(f14(Str::lit("value")), List::<Bytes>::nil())))?, {
                let m530 = f69(f51(v1, f14(Str::lit("field"))));
                match m530 {
                    None => {
                        false
                    }
                    Some(v2) => {
                        f76(v2)?
                    }
                }
            }))
        }
    }
}

pub fn f177(v0: Adt0) -> R<bool> {
    let m531 = f50(v0);
    match m531 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f74(f111(v1.clone(), List::cons(f14(Str::lit("constructor")), List::cons(f14(Str::lit("binders")), List::cons(f14(Str::lit("body")), List::<Bytes>::nil()))))?, f74(f126(f51(v1.clone(), f14(Str::lit("constructor"))))?, f120(v1, f14(Str::lit("binders")), Fn12::F208)?)))
        }
    }
}

pub fn f178(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h532, _) => {
            h532
        }
    };
    let a533 = v3;
    let a534 = v1;
    if nat_lt(a533, a534) {
        let v4: u64 = f209(v0.clone(), v3)?;
        let a535 = v4;
        let a536 = 32u64;
        if if nat_lt(a535, a536) {
            true
        } else {
            let a537 = v4;
            let a538 = 34u64;
            if nat_eq(a537, a538) {
                true
            } else {
                let a539 = v4;
                let a540 = 92u64;
                nat_eq(a539, a540)
            }
        } {
            Ok(Some(Adt7::C0({
                let a541 = v3;
                let a542 = 1u64;
                nat_add(a541, a542)?
            }, List::cons(f210(v4), match v2 {
                Adt7::C0(_, h543) => {
                    h543
                }
            }))))
        } else {
            let v5: u64 = f211(v0.clone(), v1, v3)?;
            Ok(Some(Adt7::C0(v5, List::cons(f164(v0, v3, {
                let a544 = v5;
                let a545 = v3;
                nat_sub(a544, a545)
            }), match v2 {
                Adt7::C0(_, h546) => {
                    h546
                }
            }))))
        }
    } else {
        Ok(None::<Adt7>)
    }
}

pub fn f179(v0: List<Adt0>) -> List<Adt0> {
    f212(Fn29::F213, List::<Adt0>::nil(), v0)
}

pub fn f180(v0: Fn21, p551: u64, p552: Adt10) -> (Adt10, bool) {
    let m550 = (p551, p552);
    {
        let mut m550 = m550;
        loop {
            let step = {
                let (v1, v2) = m550;
                let m547 = v1;
                if m547 == 0 {
                    Err::<(u64, Adt10), (Adt10, bool)>((v2, false))
                } else {
                    let v3 = m547 - 1;
                    let m549 = {
                        let c548: Fn21 = v0.clone();
                        c548.apply(v2.clone())
                    };
                    match m549 {
                        None => {
                            Err::<(u64, Adt10), (Adt10, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt10), (Adt10, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: Adt10) -> Option<Adt10> {
    f214(v0)
}

pub fn f182(v0: Adt8) -> R<Option<Adt8>> {
    let m554 = f195(match v0.clone() {
        Adt8::C0(h553, _) => {
            h553
        }
    });
    match m554 {
        None => {
            let m560 = f215(match v0.clone() {
                Adt8::C0(h559, _) => {
                    h559
                }
            });
            match m560 {
                None => {
                    Ok(None::<Adt8>)
                }
                Some(v2) => {
                    Ok(Some(Adt8::C0(match v2 {
                        Adt13::C0(_, h561) => {
                            h561
                        }
                    }, {
                        let a563 = match v0 {
                            Adt8::C0(_, h562) => {
                                h562
                            }
                        };
                        let a564 = 1u64;
                        nat_add(a563, a564)?
                    })))
                }
            }
        }
        Some(v1) => {
            Ok(Some(Adt8::C0(match v1 {
                Adt12::C0(_, _, h555) => {
                    h555
                }
            }, {
                let a557 = match v0 {
                    Adt8::C0(_, h556) => {
                        h556
                    }
                };
                let a558 = 1u64;
                nat_add(a557, a558)?
            })))
        }
    }
}

pub fn f183(v0: Bytes, v1: u64) -> Option<u8> {
    let a565 = v0;
    let a566 = v1;
    index_bytes(a565, a566)
}

pub fn f184(v0: Fn22, p571: u64, p572: u64) -> R<(u64, bool)> {
    let m570 = (p571, p572);
    {
        let mut m570 = m570;
        loop {
            let step = {
                let (v1, v2) = m570;
                let m567 = v1;
                if m567 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m567 - 1;
                    let m569 = {
                        let c568: Fn22 = v0.clone();
                        c568.apply(v2)?
                    };
                    match m569 {
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
                Ok(next) => { m570 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f216(v0, v1)
}

pub fn f186(v0: Adt0, v1: Bytes) -> R<bool> {
    let m573 = f13(v0, f14(Str::lit("properties")));
    match m573 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m574 = f13(v2, f14(Str::lit("kind")));
            match m574 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m575 = f69(f13(v3.clone(), f14(Str::lit("const"))));
                    if match m575 {
                        None => {
                            false
                        }
                        Some(v4) => {
                            f61(v4, v1.clone())
                        }
                    } {
                        Ok(true)
                    } else {
                        let m576 = f72(f13(v3, f14(Str::lit("enum"))));
                        match m576 {
                            None => {
                                Ok(false)
                            }
                            Some(v5) => {
                                f148(Fn17::F217(v1), false, f137(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: u64) -> Bytes {
    let m580 = {
        let a577 = v0;
        let a578 = v1;
        let a579 = 1u64;
        slice_bytes(a577, a578, a579)
    };
    match m580 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f188(v0: Fn23, p585: u64, p586: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m584 = (p585, p586);
    {
        let mut m584 = m584;
        loop {
            let step = {
                let (v1, v2) = m584;
                let m581 = v1;
                if m581 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m581 - 1;
                    let m583 = {
                        let c582: Fn23 = v0.clone();
                        c582.apply(v2.clone())
                    };
                    match m583 {
                        None => {
                            Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v3, v4))
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

pub fn f189(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f218(v1, v0, v2)
}

pub fn f190(v0: u64) -> Bytes {
    f164(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f191(v0: Bytes) -> u64 {
    let a587 = v0.clone();
    let a588 = Bytes::lit(&[0x6e, 0x61, 0x6d, 0x65]);
    if equal(a587, a588) {
        0u64
    } else {
        let a589 = v0.clone();
        let a590 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
        if equal(a589, a590) {
            1u64
        } else {
            let a591 = v0.clone();
            let a592 = Bytes::lit(&[0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
            if equal(a591, a592) {
                2u64
            } else {
                let a593 = v0.clone();
                let a594 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65]);
                if equal(a593, a594) {
                    3u64
                } else {
                    let a595 = v0.clone();
                    let a596 = Bytes::lit(&[0x72, 0x65, 0x73, 0x75, 0x6c, 0x74]);
                    if equal(a595, a596) {
                        4u64
                    } else {
                        let a597 = v0.clone();
                        let a598 = Bytes::lit(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74]);
                        if equal(a597, a598) {
                            5u64
                        } else {
                            let a599 = v0.clone();
                            let a600 = Bytes::lit(&[0x66, 0x69, 0x65, 0x6c, 0x64, 0x73]);
                            if equal(a599, a600) {
                                6u64
                            } else {
                                let a601 = v0.clone();
                                let a602 = Bytes::lit(&[0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x73]);
                                if equal(a601, a602) {
                                    7u64
                                } else {
                                    let a603 = v0.clone();
                                    let a604 = Bytes::lit(&[0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73]);
                                    if equal(a603, a604) {
                                        8u64
                                    } else {
                                        let a605 = v0.clone();
                                        let a606 = Bytes::lit(&[0x62, 0x6f, 0x64, 0x79]);
                                        if equal(a605, a606) {
                                            9u64
                                        } else {
                                            let a607 = v0.clone();
                                            let a608 = Bytes::lit(&[0x70, 0x72, 0x6f, 0x6f, 0x66]);
                                            if equal(a607, a608) {
                                                10u64
                                            } else {
                                                let a609 = v0;
                                                let a610 = Bytes::lit(&[0x61, 0x78, 0x69, 0x6f, 0x6d, 0x73]);
                                                if equal(a609, a610) {
                                                    11u64
                                                } else {
                                                    12u64
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

pub fn f192(v0: Fn24, p614: List<List<(Bytes, Adt1)>>, p615: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m613 = (p614, p615);
    {
        let mut m613 = m613;
        loop {
            let step = {
                let (v1, v2) = m613;
                let m611 = v2;
                match m611.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c612: Fn24 = v0.clone();
                            c612.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f194(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    let a617 = {
        let a616 = v0.clone();
        length_list(a616)
    };
    let a618 = 1u64;
    if nat_le(a617, a618) {
        Ok(None::<List<List<(Bytes, Adt1)>>>)
    } else {
        Ok(Some(f219(v0)?))
    }
}

pub fn f195(v0: Adt0) -> Option<Adt12> {
    let m619 = v0;
    match m619 {
        Adt0::C0(_) => {
            None::<Adt12>
        }
        Adt0::C1(_, _) => {
            None::<Adt12>
        }
        Adt0::C2(_) => {
            None::<Adt12>
        }
        Adt0::C3(_) => {
            None::<Adt12>
        }
        Adt0::C4(_) => {
            None::<Adt12>
        }
        Adt0::C5 => {
            None::<Adt12>
        }
        Adt0::C6(_, _) => {
            None::<Adt12>
        }
        Adt0::C7(v1, r620, r621) => {
            let v2 = (*r620).clone();
            let v3 = (*r621).clone();
            Some(Adt12::C0(v1, v2, v3))
        }
    }
}

pub fn f196(v0: Bytes, v1: Adt9) -> R<Option<Adt9>> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h622, _) => {
            h622
        }
    };
    let m623 = f183(v0, v2);
    match m623 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v3) => {
            Ok(Some(Adt9::C0({
                let a624 = v2;
                let a625 = 1u64;
                nat_add(a624, a625)?
            }, {
                let a672 = {
                    let v4: u8 = v3;
                    let a670 = {
                        let a646 = {
                            let a634 = {
                                let a628 = {
                                    let a626 = v4;
                                    let a627 = 128u8;
                                    fixed_u8::bit_and(a626, a627)
                                };
                                let a629 = 128u8;
                                if equal(a628, a629) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a635 = {
                                let a632 = {
                                    let a630 = v4;
                                    let a631 = 64u8;
                                    fixed_u8::bit_and(a630, a631)
                                };
                                let a633 = 64u8;
                                if equal(a632, a633) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a634, a635)?
                        };
                        let a647 = {
                            let a644 = {
                                let a638 = {
                                    let a636 = v4;
                                    let a637 = 32u8;
                                    fixed_u8::bit_and(a636, a637)
                                };
                                let a639 = 32u8;
                                if equal(a638, a639) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a645 = {
                                let a642 = {
                                    let a640 = v4;
                                    let a641 = 16u8;
                                    fixed_u8::bit_and(a640, a641)
                                };
                                let a643 = 16u8;
                                if equal(a642, a643) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a644, a645)?
                        };
                        nat_add(a646, a647)?
                    };
                    let a671 = {
                        let a668 = {
                            let a656 = {
                                let a650 = {
                                    let a648 = v4;
                                    let a649 = 8u8;
                                    fixed_u8::bit_and(a648, a649)
                                };
                                let a651 = 8u8;
                                if equal(a650, a651) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a657 = {
                                let a654 = {
                                    let a652 = v4;
                                    let a653 = 4u8;
                                    fixed_u8::bit_and(a652, a653)
                                };
                                let a655 = 4u8;
                                if equal(a654, a655) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a656, a657)?
                        };
                        let a669 = {
                            let a666 = {
                                let a660 = {
                                    let a658 = v4;
                                    let a659 = 2u8;
                                    fixed_u8::bit_and(a658, a659)
                                };
                                let a661 = 2u8;
                                if equal(a660, a661) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a667 = {
                                let a664 = {
                                    let a662 = v4;
                                    let a663 = 1u8;
                                    fixed_u8::bit_and(a662, a663)
                                };
                                let a665 = 1u8;
                                if equal(a664, a665) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a666, a667)?
                        };
                        nat_add(a668, a669)?
                    };
                    nat_add(a670, a671)?
                };
                let a673 = 47u64;
                if nat_eq(a672, a673) {
                    let a674 = v2;
                    let a675 = 1u64;
                    nat_add(a674, a675)?
                } else {
                    match v1 {
                        Adt9::C0(_, h676) => {
                            h676
                        }
                    }
                }
            })))
        }
    }
}

pub fn f197(v0: Fn25, p681: u64, p682: (u64, u64)) -> R<((u64, u64), bool)> {
    let m680 = (p681, p682);
    {
        let mut m680 = m680;
        loop {
            let step = {
                let (v1, v2) = m680;
                let m677 = v1;
                if m677 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m677 - 1;
                    let m679 = {
                        let c678: Fn25 = v0.clone();
                        c678.apply(v2)?
                    };
                    match m679 {
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
                Ok(next) => { m680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f220(v0, v1)
}

pub fn f199(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a683 = v1;
        let a684 = 256u64;
        let a685 = 0u64;
        nat_quot(a683, a684, a685)
    };
    let v3: List<Bytes> = {
        let (_, h686) = {
            let (h687, _) = f221(Fn30::F222(v2), {
                let a688 = v2;
                let a689 = 1u64;
                nat_add(a688, a689)?
            }, (0u64, List::cons({
                let a690 = Str::lit("\u{a}");
                utf8_encode(a690)
            }, v0)))?;
            h687
        };
        h686
    };
    Ok(List::cons(f164(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a691 = v1;
        let a692 = 256u64;
        let a693 = 0u64;
        nat_rem(a691, a692, a693)
    }), v3))
}

pub fn f200(v0: Fn26, p698: u64, p699: Adt11) -> R<(Adt11, bool)> {
    let m697 = (p698, p699);
    {
        let mut m697 = m697;
        loop {
            let step = {
                let (v1, v2) = m697;
                let m694 = v1;
                if m694 == 0 {
                    Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m694 - 1;
                    let m696 = {
                        let c695: Fn26 = v0.clone();
                        c695.apply(v2.clone())?
                    };
                    match m696 {
                        None => {
                            Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt11), R<(Adt11, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m697 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Adt11) -> R<Option<Adt11>> {
    f223(v0)
}

pub fn f202(v0: Fn27, p703: (Option<Bytes>, List<Bytes>), p704: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m702 = (p703, p704);
    {
        let mut m702 = m702;
        loop {
            let step = {
                let (v1, v2) = m702;
                let m700 = v2;
                match m700.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c701: Fn27 = v0.clone();
                            c701.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f224(v0, v1)
}

pub fn f204(v0: Fn28, p708: bool, p709: List<Bytes>) -> bool {
    let m707 = (p708, p709);
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v1, v2) = m707;
                let m705 = v2;
                match m705.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c706: Fn28 = v0.clone();
                            c706.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Adt0, v1: bool, v2: Bytes) -> bool {
    if v1 {
        f75(f51(v0, v2))
    } else {
        false
    }
}

pub fn f206(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a710 = v2;
        let a711 = v0;
        equal(a710, a711)
    }
}

pub fn f207(v0: bool, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let m712 = f183(v1, v2);
    match m712 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let v5: u64 = {
                let v4: u8 = v3;
                let a757 = {
                    let a733 = {
                        let a721 = {
                            let a715 = {
                                let a713 = v4;
                                let a714 = 128u8;
                                fixed_u8::bit_and(a713, a714)
                            };
                            let a716 = 128u8;
                            if equal(a715, a716) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a722 = {
                            let a719 = {
                                let a717 = v4;
                                let a718 = 64u8;
                                fixed_u8::bit_and(a717, a718)
                            };
                            let a720 = 64u8;
                            if equal(a719, a720) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a721, a722)?
                    };
                    let a734 = {
                        let a731 = {
                            let a725 = {
                                let a723 = v4;
                                let a724 = 32u8;
                                fixed_u8::bit_and(a723, a724)
                            };
                            let a726 = 32u8;
                            if equal(a725, a726) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a732 = {
                            let a729 = {
                                let a727 = v4;
                                let a728 = 16u8;
                                fixed_u8::bit_and(a727, a728)
                            };
                            let a730 = 16u8;
                            if equal(a729, a730) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a731, a732)?
                    };
                    nat_add(a733, a734)?
                };
                let a758 = {
                    let a755 = {
                        let a743 = {
                            let a737 = {
                                let a735 = v4;
                                let a736 = 8u8;
                                fixed_u8::bit_and(a735, a736)
                            };
                            let a738 = 8u8;
                            if equal(a737, a738) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a744 = {
                            let a741 = {
                                let a739 = v4;
                                let a740 = 4u8;
                                fixed_u8::bit_and(a739, a740)
                            };
                            let a742 = 4u8;
                            if equal(a741, a742) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a743, a744)?
                    };
                    let a756 = {
                        let a753 = {
                            let a747 = {
                                let a745 = v4;
                                let a746 = 2u8;
                                fixed_u8::bit_and(a745, a746)
                            };
                            let a748 = 2u8;
                            if equal(a747, a748) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a754 = {
                            let a751 = {
                                let a749 = v4;
                                let a750 = 1u8;
                                fixed_u8::bit_and(a749, a750)
                            };
                            let a752 = 1u8;
                            if equal(a751, a752) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a753, a754)?
                    };
                    nat_add(a755, a756)?
                };
                nat_add(a757, a758)?
            };
            if if f74({
                let a759 = 48u64;
                let a760 = v5;
                nat_le(a759, a760)
            }, {
                let a761 = v5;
                let a762 = 57u64;
                nat_le(a761, a762)
            }) {
                true
            } else {
                f74(v0, f74({
                    let a763 = 97u64;
                    let a764 = v5;
                    nat_le(a763, a764)
                }, {
                    let a765 = v5;
                    let a766 = 102u64;
                    nat_le(a765, a766)
                }))
            } {
                Ok(Some({
                    let a767 = v2;
                    let a768 = 1u64;
                    nat_add(a767, a768)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f208(v0: Adt0) -> R<bool> {
    let m769 = f77(v0);
    match m769 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f113(v1)
        }
    }
}

pub fn f209(v0: Bytes, v1: u64) -> R<u64> {
    let m770 = f183(v0, v1);
    match m770 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a815 = {
                let a791 = {
                    let a779 = {
                        let a773 = {
                            let a771 = v3;
                            let a772 = 128u8;
                            fixed_u8::bit_and(a771, a772)
                        };
                        let a774 = 128u8;
                        if equal(a773, a774) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a780 = {
                        let a777 = {
                            let a775 = v3;
                            let a776 = 64u8;
                            fixed_u8::bit_and(a775, a776)
                        };
                        let a778 = 64u8;
                        if equal(a777, a778) {
                            64u64
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
                            let a782 = 32u8;
                            fixed_u8::bit_and(a781, a782)
                        };
                        let a784 = 32u8;
                        if equal(a783, a784) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a790 = {
                        let a787 = {
                            let a785 = v3;
                            let a786 = 16u8;
                            fixed_u8::bit_and(a785, a786)
                        };
                        let a788 = 16u8;
                        if equal(a787, a788) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a789, a790)?
                };
                nat_add(a791, a792)?
            };
            let a816 = {
                let a813 = {
                    let a801 = {
                        let a795 = {
                            let a793 = v3;
                            let a794 = 8u8;
                            fixed_u8::bit_and(a793, a794)
                        };
                        let a796 = 8u8;
                        if equal(a795, a796) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a802 = {
                        let a799 = {
                            let a797 = v3;
                            let a798 = 4u8;
                            fixed_u8::bit_and(a797, a798)
                        };
                        let a800 = 4u8;
                        if equal(a799, a800) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a801, a802)?
                };
                let a814 = {
                    let a811 = {
                        let a805 = {
                            let a803 = v3;
                            let a804 = 2u8;
                            fixed_u8::bit_and(a803, a804)
                        };
                        let a806 = 2u8;
                        if equal(a805, a806) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a812 = {
                        let a809 = {
                            let a807 = v3;
                            let a808 = 1u8;
                            fixed_u8::bit_and(a807, a808)
                        };
                        let a810 = 1u8;
                        if equal(a809, a810) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a811, a812)?
                };
                nat_add(a813, a814)?
            };
            nat_add(a815, a816)
        }
    }
}

pub fn f210(v0: u64) -> Bytes {
    let a817 = v0;
    let a818 = 34u64;
    if nat_eq(a817, a818) {
        f133(Str::lit("\\\""))
    } else {
        let a819 = v0;
        let a820 = 92u64;
        if nat_eq(a819, a820) {
            f133(Str::lit("\\\\"))
        } else {
            let a821 = v0;
            let a822 = 8u64;
            if nat_eq(a821, a822) {
                f133(Str::lit("\\b"))
            } else {
                let a823 = v0;
                let a824 = 12u64;
                if nat_eq(a823, a824) {
                    f133(Str::lit("\\f"))
                } else {
                    let a825 = v0;
                    let a826 = 10u64;
                    if nat_eq(a825, a826) {
                        f133(Str::lit("\\n"))
                    } else {
                        let a827 = v0;
                        let a828 = 13u64;
                        if nat_eq(a827, a828) {
                            f133(Str::lit("\\r"))
                        } else {
                            let a829 = v0;
                            let a830 = 9u64;
                            if nat_eq(a829, a830) {
                                f133(Str::lit("\\t"))
                            } else {
                                f132(f133(Str::lit("\\u00")), f225(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f211(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h831, _) = f184(Fn22::F226(v1, v0), {
        let a832 = v1;
        let a833 = v2;
        nat_sub(a832, a833)
    }, v2)?;
    Ok(h831)
}

pub fn f212(v0: Fn29, p837: List<Adt0>, p838: List<Adt0>) -> List<Adt0> {
    let m836 = (p837, p838);
    {
        let mut m836 = m836;
        loop {
            let step = {
                let (v1, v2) = m836;
                let m834 = v2;
                match m834.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c835: Fn29 = v0.clone();
                            c835.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m836 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f214(v0: Adt10) -> Option<Adt10> {
    let m840 = f215(match v0.clone() {
        Adt10::C0(h839, _) => {
            h839
        }
    });
    match m840 {
        None => {
            None::<Adt10>
        }
        Some(v1) => {
            Some(Adt10::C0(match v1.clone() {
                Adt13::C0(_, h841) => {
                    h841
                }
            }, List::cons(match v1 {
                Adt13::C0(h842, _) => {
                    h842
                }
            }, match v0 {
                Adt10::C0(_, h843) => {
                    h843
                }
            })))
        }
    }
}

pub fn f215(v0: Adt0) -> Option<Adt13> {
    let m844 = v0;
    match m844 {
        Adt0::C0(_) => {
            None::<Adt13>
        }
        Adt0::C1(_, _) => {
            None::<Adt13>
        }
        Adt0::C2(_) => {
            None::<Adt13>
        }
        Adt0::C3(_) => {
            None::<Adt13>
        }
        Adt0::C4(_) => {
            None::<Adt13>
        }
        Adt0::C5 => {
            None::<Adt13>
        }
        Adt0::C6(r845, r846) => {
            let v1 = (*r845).clone();
            let v2 = (*r846).clone();
            Some(Adt13::C0(v1, v2))
        }
        Adt0::C7(_, _, _) => {
            None::<Adt13>
        }
    }
}

pub fn f216(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f227(f143(v0, v1)?) {
        Ok(Some({
            let a847 = v1;
            let a848 = 1u64;
            nat_add(a847, a848)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f217(v0: Bytes, v1: bool, v2: Adt0) -> bool {
    if v1 {
        true
    } else {
        let m849 = f77(v2);
        match m849 {
            None => {
                false
            }
            Some(v3) => {
                f61(v3, v0)
            }
        }
    }
}

pub fn f218(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h850, _) = v2;
        h850
    };
    let a851 = v3;
    let a852 = 0u64;
    if nat_eq(a851, a852) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a853 = v3;
            let a854 = v1;
            let a855 = 0u64;
            nat_quot(a853, a854, a855)
        }, {
            let a860 = f187(v0, {
                let a856 = v3;
                let a857 = v1;
                let a858 = 0u64;
                nat_rem(a856, a857, a858)
            });
            let a861 = {
                let (_, h859) = v2;
                h859
            };
            append_bytes(a860, a861)
        }))
    }
}

pub fn f219(v0: List<List<(Bytes, Adt1)>>) -> R<List<List<(Bytes, Adt1)>>> {
    Ok(f155({
        let (_, h862) = {
            let (h863, _) = f228(Fn31::F229, {
                let a865 = {
                    let a864 = v0.clone();
                    length_list(a864)
                };
                let a866 = 1u64;
                nat_add(a865, a866)?
            }, (v0, List::<List<(Bytes, Adt1)>>::nil()))?;
            h863
        };
        h862
    }))
}

pub fn f220(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m868 = f183(v0, {
        let (h867, _) = v1;
        h867
    });
    match m868 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a913 = {
                    let a889 = {
                        let a877 = {
                            let a871 = {
                                let a869 = v3;
                                let a870 = 128u8;
                                fixed_u8::bit_and(a869, a870)
                            };
                            let a872 = 128u8;
                            if equal(a871, a872) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a878 = {
                            let a875 = {
                                let a873 = v3;
                                let a874 = 64u8;
                                fixed_u8::bit_and(a873, a874)
                            };
                            let a876 = 64u8;
                            if equal(a875, a876) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a877, a878)?
                    };
                    let a890 = {
                        let a887 = {
                            let a881 = {
                                let a879 = v3;
                                let a880 = 32u8;
                                fixed_u8::bit_and(a879, a880)
                            };
                            let a882 = 32u8;
                            if equal(a881, a882) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a888 = {
                            let a885 = {
                                let a883 = v3;
                                let a884 = 16u8;
                                fixed_u8::bit_and(a883, a884)
                            };
                            let a886 = 16u8;
                            if equal(a885, a886) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a887, a888)?
                    };
                    nat_add(a889, a890)?
                };
                let a914 = {
                    let a911 = {
                        let a899 = {
                            let a893 = {
                                let a891 = v3;
                                let a892 = 8u8;
                                fixed_u8::bit_and(a891, a892)
                            };
                            let a894 = 8u8;
                            if equal(a893, a894) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a900 = {
                            let a897 = {
                                let a895 = v3;
                                let a896 = 4u8;
                                fixed_u8::bit_and(a895, a896)
                            };
                            let a898 = 4u8;
                            if equal(a897, a898) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a899, a900)?
                    };
                    let a912 = {
                        let a909 = {
                            let a903 = {
                                let a901 = v3;
                                let a902 = 2u8;
                                fixed_u8::bit_and(a901, a902)
                            };
                            let a904 = 2u8;
                            if equal(a903, a904) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a910 = {
                            let a907 = {
                                let a905 = v3;
                                let a906 = 1u8;
                                fixed_u8::bit_and(a905, a906)
                            };
                            let a908 = 1u8;
                            if equal(a907, a908) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a909, a910)?
                    };
                    nat_add(a911, a912)?
                };
                nat_add(a913, a914)?
            };
            let a915 = v4;
            let a916 = 128u64;
            if nat_lt(a915, a916) {
                Ok(Some(({
                    let a918 = {
                        let (h917, _) = v1;
                        h917
                    };
                    let a919 = 1u64;
                    nat_add(a918, a919)?
                }, {
                    let a921 = {
                        let (_, h920) = v1;
                        h920
                    };
                    let a922 = 1u64;
                    nat_add(a921, a922)?
                })))
            } else {
                let a923 = v4;
                let a924 = 192u64;
                if nat_lt(a923, a924) {
                    Ok(Some(({
                        let a926 = {
                            let (h925, _) = v1;
                            h925
                        };
                        let a927 = 1u64;
                        nat_add(a926, a927)?
                    }, {
                        let (_, h928) = v1;
                        h928
                    })))
                } else {
                    Ok(Some(({
                        let a930 = {
                            let (h929, _) = v1;
                            h929
                        };
                        let a931 = 1u64;
                        nat_add(a930, a931)?
                    }, {
                        let a933 = {
                            let (_, h932) = v1;
                            h932
                        };
                        let a934 = 1u64;
                        nat_add(a933, a934)?
                    })))
                }
            }
        }
    }
}

pub fn f221(v0: Fn30, p939: u64, p940: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m938 = (p939, p940);
    {
        let mut m938 = m938;
        loop {
            let step = {
                let (v1, v2) = m938;
                let m935 = v1;
                if m935 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m935 - 1;
                    let m937 = {
                        let c936: Fn30 = v0.clone();
                        c936.apply(v2.clone())?
                    };
                    match m937 {
                        None => {
                            Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m938 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f230(v0, v1)
}

pub fn f223(v0: Adt11) -> R<Option<Adt11>> {
    let m942 = match v0.clone() {
        Adt11::C0(_, _, _, _, _, h941) => {
            h941
        }
    };
    match m942 {
        None => {
            if if match v0.clone() {
                Adt11::C0(_, _, h943, _, _, _) => {
                    h943
                }
            } {
                true
            } else {
                let a946 = match v0.clone() {
                    Adt11::C0(_, h944, _, _, _, _) => {
                        h944
                    }
                };
                let a947 = match v0.clone() {
                    Adt11::C0(h945, _, _, _, _, _) => {
                        h945
                    }
                };
                nat_lt(a946, a947)
            } {
                Ok(Some(f231(v0.clone(), match v0.clone() {
                    Adt11::C0(h948, _, _, _, _, _) => {
                        h948
                    }
                }, match v0.clone() {
                    Adt11::C0(_, _, _, h949, _, _) => {
                        h949
                    }
                }, match v0 {
                    Adt11::C0(_, _, _, _, h950, _) => {
                        h950
                    }
                }, Some(false))))
            } else {
                let m952 = match v0.clone() {
                    Adt11::C0(_, _, _, h951, _, _) => {
                        h951
                    }
                };
                match m952.uncons() {
                    None => {
                        let m955 = match v0.clone() {
                            Adt11::C0(_, _, _, _, h954, _) => {
                                h954
                            }
                        };
                        match m955.uncons() {
                            None => {
                                Ok(Some(f231(v0.clone(), match v0 {
                                    Adt11::C0(h956, _, _, _, _, _) => {
                                        h956
                                    }
                                }, List::<Adt4>::nil(), List::<Adt4>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f232(v0, v4, List::<Adt4>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f232(v0.clone(), v2, v3, match v0 {
                            Adt11::C0(_, _, _, _, h953, _) => {
                                h953
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt11>)
        }
    }
}

pub fn f224(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m958 = {
        let (h957, _) = v0;
        h957
    };
    match m958 {
        None => {
            (Some(v1), {
                let (_, h959) = v0;
                h959
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a960 = v2;
                let a961 = v1;
                append_bytes(a960, a961)
            }, {
                let (_, h962) = v0;
                h962
            }))
        }
    }
}

pub fn f225(v0: u64) -> Bytes {
    f132(f233(v0, 16u64), f234(v0))
}

pub fn f226(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f235(v1, v0, v2)
}

pub fn f227(v0: u64) -> bool {
    if f142(v0) {
        true
    } else {
        let a963 = 47u64;
        let a964 = v0;
        if if nat_lt(a963, a964) {
            let a965 = v0;
            let a966 = 58u64;
            nat_lt(a965, a966)
        } else {
            false
        } {
            true
        } else {
            let a967 = v0;
            let a968 = 46u64;
            if nat_eq(a967, a968) {
                true
            } else {
                let a969 = v0;
                let a970 = 39u64;
                if nat_eq(a969, a970) {
                    true
                } else {
                    let a971 = v0;
                    let a972 = 47u64;
                    if nat_eq(a971, a972) {
                        true
                    } else {
                        let a973 = v0;
                        let a974 = 45u64;
                        nat_eq(a973, a974)
                    }
                }
            }
        }
    }
}

pub fn f228(v0: Fn31, p979: u64, p980: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)> {
    let m978 = (p979, p980);
    {
        let mut m978 = m978;
        loop {
            let step = {
                let (v1, v2) = m978;
                let m975 = v1;
                if m975 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m975 - 1;
                    let m977 = {
                        let c976: Fn31 = v0.clone();
                        c976.apply(v2.clone())?
                    };
                    match m977 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    f236(v0)
}

pub fn f230(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a982 = {
        let (h981, _) = v1.clone();
        h981
    };
    let a983 = v0;
    if nat_lt(a982, a983) {
        Ok(Some(({
            let a985 = {
                let (h984, _) = v1;
                h984
            };
            let a986 = 1u64;
            nat_add(a985, a986)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h987) = v1;
            h987
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f231(v0: Adt11, v1: u64, v2: List<Adt4>, v3: List<Adt4>, v4: Option<bool>) -> Adt11 {
    Adt11::C0(v1, match v0.clone() {
        Adt11::C0(_, h988, _, _, _, _) => {
            h988
        }
    }, match v0 {
        Adt11::C0(_, _, h989, _, _, _) => {
            h989
        }
    }, v2, v3, v4)
}

pub fn f232(v0: Adt11, v1: Adt4, v2: List<Adt4>, v3: List<Adt4>) -> R<Adt11> {
    let v4: u64 = match v1.clone() {
        Adt4::C0(h990, _, _) => {
            h990
        }
    };
    let v5: bool = match v1.clone() {
        Adt4::C0(_, h991, _) => {
            h991
        }
    };
    let v6: u64 = match v0.clone() {
        Adt11::C0(h992, _, _, _, _, _) => {
            h992
        }
    };
    let m994 = match v1 {
        Adt4::C0(_, _, h993) => {
            h993
        }
    };
    match m994 {
        Adt1::C0 => {
            Ok(f231(v0, v6, v2, v3, None::<bool>))
        }
        Adt1::C1(v7) => {
            Ok(f231(v0, {
                let a995 = v6;
                let a996 = f165(v7)?;
                nat_add(a995, a996)?
            }, v2, v3, None::<bool>))
        }
        Adt1::C2(r1001, r1002) => {
            let v8 = (*r1001).clone();
            let v9 = (*r1002).clone();
            Ok(f231(v0, v6, List::cons(Adt4::C0(v4, v5, v8), List::cons(Adt4::C0(v4, v5, v9), v2)), v3, None::<bool>))
        }
        Adt1::C3(v10, r1003) => {
            let v11 = (*r1003).clone();
            Ok(f231(v0, v6, List::cons(Adt4::C0({
                let a997 = v4;
                let a998 = v10;
                nat_add(a997, a998)?
            }, v5, v11), v2), v3, None::<bool>))
        }
        Adt1::C4 => {
            if v5 {
                Ok(f231(v0, {
                    let a999 = v6;
                    let a1000 = 1u64;
                    nat_add(a999, a1000)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f231(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt1::C5 => {
            if v5 {
                Ok(f231(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f231(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt1::C6(r1004) => {
            let v12 = (*r1004).clone();
            Ok(f231(v0, v6, List::cons(Adt4::C0(v4, true, v12), v2), v3, None::<bool>))
        }
    }
}

pub fn f233(v0: u64, v1: u64) -> Bytes {
    f234({
        let a1005 = v0;
        let a1006 = v1;
        let a1007 = 0u64;
        nat_quot(a1005, a1006, a1007)
    })
}

pub fn f234(v0: u64) -> Bytes {
    f164(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1008 = v0;
        let a1009 = 16u64;
        let a1010 = 0u64;
        nat_rem(a1008, a1009, a1010)
    }, 1u64)
}

pub fn f235(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1011 = v2;
    let a1012 = v1;
    if nat_lt(a1011, a1012) {
        let v3: u64 = f209(v0, v2)?;
        let a1013 = v3;
        let a1014 = 32u64;
        if nat_lt(a1013, a1014) {
            Ok(None::<u64>)
        } else {
            let a1015 = v3;
            let a1016 = 34u64;
            if nat_eq(a1015, a1016) {
                Ok(None::<u64>)
            } else {
                let a1017 = v3;
                let a1018 = 92u64;
                if nat_eq(a1017, a1018) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1019 = v2;
                        let a1020 = 1u64;
                        nat_add(a1019, a1020)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f236(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    let v1: List<List<(Bytes, Adt1)>> = {
        let (h1021, _) = v0;
        h1021
    };
    let v2: List<List<(Bytes, Adt1)>> = {
        let (_, h1022) = v0;
        h1022
    };
    let m1023 = v1;
    match m1023.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>)
        }
        Some((v3, v4)) => {
            let m1024 = v4;
            match m1024.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt1)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f237(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f237(v0: List<(Bytes, Adt1)>, v1: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v2: u64 = {
        let a1029 = {
            let a1027 = {
                let a1025 = v0.clone();
                length_list(a1025)
            };
            let a1028 = {
                let a1026 = v1.clone();
                length_list(a1026)
            };
            nat_add(a1027, a1028)?
        };
        let a1030 = 1u64;
        nat_add(a1029, a1030)?
    };
    f238({
        let (_, h1031) = {
            let (_, h1032) = {
                let (h1033, _) = f239(Fn32::F240, v2, (v0, (v1, List::<(Bytes, Adt1)>::nil())));
                h1033
            };
            h1032
        };
        h1031
    })
}

pub fn f238(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    f96(Fn6::F241, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f239(v0: Fn32, p1038: u64, p1039: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool) {
    let m1037 = (p1038, p1039);
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let (v1, v2) = m1037;
                let m1034 = v1;
                if m1034 == 0 {
                    Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, false))
                } else {
                    let v3 = m1034 - 1;
                    let m1036 = {
                        let c1035: Fn32 = v0.clone();
                        c1035.apply(v2.clone())
                    };
                    match m1036 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    f242(v0)
}

pub fn f241(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f242(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    let v1: List<(Bytes, Adt1)> = {
        let (h1040, _) = v0.clone();
        h1040
    };
    let v2: List<(Bytes, Adt1)> = {
        let (h1041, _) = {
            let (_, h1042) = v0.clone();
            h1042
        };
        h1041
    };
    let v3: List<(Bytes, Adt1)> = {
        let (_, h1043) = {
            let (_, h1044) = v0;
            h1044
        };
        h1043
    };
    let m1045 = v1.clone();
    match m1045.uncons() {
        None => {
            let m1046 = v2;
            match m1046.uncons() {
                None => {
                    None::<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt1)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1047 = v2.clone();
            match m1047.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt1)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f243({
                        let (h1048, _) = v8.clone();
                        h1048
                    }, {
                        let (h1049, _) = v6.clone();
                        h1049
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

pub fn f243(v0: Bytes, v1: Bytes) -> bool {
    let a1054 = {
        let a1050 = v0;
        let a1051 = v1;
        compare_bytes(a1050, a1051)
    };
    let a1055 = {
        let a1052 = Bytes::lit(&[]);
        let a1053 = Bytes::lit(&[0x01]);
        compare_bytes(a1052, a1053)
    };
    equal(a1054, a1055)
}
