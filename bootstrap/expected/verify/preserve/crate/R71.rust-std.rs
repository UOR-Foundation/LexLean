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
    C0(u64, Bytes),
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
    C0(Adt0, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt0, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt0, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C3(u64, std::rc::Rc<Adt6>),
    C4,
    C5,
    C6(std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(List<Bytes>, u64, List<Adt8>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, bool, Adt6),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Option<Adt0>),
    C1(Bytes),
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Bytes, Adt0, Adt0),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt0, Adt0),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, u64, bool, List<Adt8>, List<Adt8>, Option<bool>),
}

#[derive(Clone)]
pub enum Fn0 {
    F10(Adt2, Adt0),
}

impl Fn0 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Adt0) -> R<(bool, Bytes)> {
        match self {
            Fn0::F10(k0, k1) => f10(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt3) -> Option<Adt3> {
        match self {
            Fn1::F15(k0) => f15(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn2::F17 => f17(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F27,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt5) -> Option<Adt5> {
        match self {
            Fn3::F27 => f27(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F42,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn4::F42 => f42(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F48(Adt0),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn5::F48(k0) => f48(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F51(u64, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn6::F51(k0, k1) => f51(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F72,
    F114(Bytes, Bytes),
    F120,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt6) -> List<Adt6> {
        match self {
            Fn7::F72 => f72(p0, p1),
            Fn7::F114(k0, k1) => f114(k0.clone(), k1.clone(), p0, p1),
            Fn7::F120 => f120(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F84(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt12) -> R<Option<Adt12>> {
        match self {
            Fn8::F84(k0) => f84(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F105,
}

impl Fn9 {
    pub fn apply(&self, p0: Adt6, p1: Adt6) -> Adt6 {
        match self {
            Fn9::F105 => f105(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F107(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt6, p1: (Bytes, Adt6)) -> Adt6 {
        match self {
            Fn10::F107(k0) => f107(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F116(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (bool, List<Adt6>), p1: Adt6) -> (bool, List<Adt6>) {
        match self {
            Fn11::F116(k0) => f116(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F119,
}

impl Fn12 {
    pub fn apply(&self, p0: (bool, Adt6), p1: Adt6) -> (bool, Adt6) {
        match self {
            Fn12::F119 => f119(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F123(Option<Bytes>),
    F244,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Adt6)>, p1: (Bytes, Adt6)) -> R<List<(Bytes, Adt6)>> {
        match self {
            Fn13::F123(k0) => f123(k0.clone(), p0, p1),
            Fn13::F244 => Ok(f244(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F125,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt6>, p1: (Bytes, Adt6)) -> List<Adt6> {
        match self {
            Fn14::F125 => f125(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F130,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn15::F130 => f130(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F132,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn16::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F143,
    F144,
    F149,
    F150,
    F151,
    F215,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt0) -> R<bool> {
        match self {
            Fn17::F143 => f143(p0),
            Fn17::F144 => f144(p0),
            Fn17::F149 => f149(p0),
            Fn17::F150 => f150(p0),
            Fn17::F151 => f151(p0),
            Fn17::F215 => f215(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F153(u64, Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt13) -> R<Option<Adt13>> {
        match self {
            Fn18::F153(k0, k1) => f153(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F158,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt0) -> List<Adt6> {
        match self {
            Fn19::F158 => f158(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F165(Bytes, Adt0),
}

impl Fn20 {
    pub fn apply(&self, p0: Option<Adt0>, p1: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn20::F165(k0, k1) => f165(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F167(Adt0),
    F188(Fn17),
    F220(Bytes),
}

impl Fn21 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> R<bool> {
        match self {
            Fn21::F167(k0) => f167(k0.clone(), p0, p1),
            Fn21::F188(k0) => f188(k0.clone(), p0, p1),
            Fn21::F220(k0) => Ok(f220(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F175,
}

impl Fn22 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>, p1: (Bytes, Adt6)) -> List<List<(Bytes, Adt6)>> {
        match self {
            Fn22::F175 => f175(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F177,
}

impl Fn23 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
        match self {
            Fn23::F177 => f177(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F194(Bytes),
    F214(bool, Bytes),
    F229(u64, Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn24::F194(k0) => f194(k0.clone(), p0),
            Fn24::F214(k0, k1) => f214(*k0, k1.clone(), p0),
            Fn24::F229(k0, k1) => f229(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F198(u64, Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn25::F198(k0, k1) => f198(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F202,
}

impl Fn26 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>, p1: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
        match self {
            Fn26::F202 => f202(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F205(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn27::F205(k0) => f205(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F208,
}

impl Fn28 {
    pub fn apply(&self, p0: Adt14) -> R<Option<Adt14>> {
        match self {
            Fn28::F208 => f208(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F210,
}

impl Fn29 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn29::F210 => f210(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F212(Adt0),
    F213(Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn30::F212(k0) => f212(k0.clone(), p0, p1),
            Fn30::F213(k0) => f213(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F225(u64),
}

impl Fn31 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn31::F225(k0) => f225(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F232,
}

impl Fn32 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
        match self {
            Fn32::F232 => f232(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F243,
}

impl Fn33 {
    pub fn apply(&self, p0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
        match self {
            Fn33::F243 => f243(p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: Adt0) -> R<Result<Bytes, Adt1>> {
    let m1 = f1(v1);
    match m1 {
        None => {
            Ok(f12(f4(Str::lit("a module is an object"))))
        }
        Some(v2) => {
            let m2 = f2(f3(v2.clone(), f4(Str::lit("declarations"))));
            match m2 {
                None => {
                    Ok(f12(f4(Str::lit("a module has a `declarations` array"))))
                }
                Some(v3) => {
                    if f5({
                        let a3 = f6(v2.clone())?;
                        let a4 = 2u64;
                        nat_eq(a3, a4)
                    }, {
                        let m5 = f7(f3(v2, f4(Str::lit("spec"))));
                        match m5 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        }
                    }) {
                        let v5: Adt2 = f8(v0.clone(), 0u64)?;
                        Ok(Ok::<Bytes, Adt1>({
                            let (_, h6) = f9(Fn0::F10(v5, v0), (true, f4(Str::lit(""))), f11(v3))?;
                            h6
                        }))
                    } else {
                        Ok(f12(f4(Str::lit("a module has exactly a `spec` and its `declarations`"))))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: Adt0) -> Option<Adt0> {
    let m7 = v0;
    match m7 {
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
        Adt0::C4(r8) => {
            let v1 = (*r8).clone();
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

pub fn f2(v0: Option<Adt0>) -> Option<Adt0> {
    let m9 = v0;
    match m9 {
        None => {
            None::<Adt0>
        }
        Some(v1) => {
            f13(v1)
        }
    }
}

pub fn f3(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m12 = {
        let (h11, _) = f14(Fn1::F15(v1), 1000000u64, Adt3::C0(v0, None::<Adt0>));
        h11
    };
    match m12 {
        Adt3::C0(_, h10) => {
            h10
        }
    }
}

pub fn f4(v0: Str) -> Bytes {
    let a13 = v0;
    utf8_encode(a13)
}

pub fn f5(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f6(v0: Adt0) -> R<u64> {
    let m16 = {
        let (h15, _) = f16(Fn2::F17, 1000000u64, Adt4::C0(v0, 0u64))?;
        h15
    };
    match m16 {
        Adt4::C0(_, h14) => {
            Ok(h14)
        }
    }
}

pub fn f7(v0: Option<Adt0>) -> Option<Bytes> {
    let m17 = v0;
    match m17 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f18(v1)
        }
    }
}

pub fn f8(v0: Adt0, v1: u64) -> R<Adt2> {
    let a18 = v1;
    let a19 = 0u64;
    if nat_eq(a18, a19) {
        let m20 = f19(v0.clone(), f4(Str::lit("properties")));
        match m20 {
            None => {
                Ok(Adt2::C5)
            }
            Some(v2) => {
                let m21 = f19(v2, f4(Str::lit("declarations")));
                match m21 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v3) => {
                        f20(v0, v3)
                    }
                }
            }
        }
    } else {
        let a22 = v1;
        let a23 = 1u64;
        if nat_eq(a22, a23) {
            Ok(Adt2::C0)
        } else {
            let a24 = v1;
            let a25 = 2u64;
            if nat_eq(a24, a25) {
                Ok(Adt2::C1)
            } else {
                let m26 = f21(v0, f4(Str::lit("proof")));
                match m26 {
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

pub fn f9(v0: Fn0, p30: (bool, Bytes), p31: List<Adt0>) -> R<(bool, Bytes)> {
    let m29 = (p30, p31);
    {
        let mut m29 = m29;
        loop {
            let step = {
                let (v1, v2) = m29;
                let m27 = v2;
                match m27.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Adt0>), R<(bool, Bytes)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Adt0>), R<(bool, Bytes)>>(({
                            let c28: Fn0 = v0.clone();
                            c28.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m29 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Adt2, v1: Adt0, v2: (bool, Bytes), v3: Adt0) -> R<(bool, Bytes)> {
    Ok((false, f22({
        let (h32, _) = v2;
        if h32 {
            let (_, h33) = v2;
            h33
        } else {
            f22({
                let (_, h34) = v2;
                h34
            }, f4(Str::lit("\u{a}")))
        }
    }, f22(f23(f24(v1, v3, v0)?)?, f4(Str::lit("\u{a}"))))))
}

pub fn f11(v0: Adt0) -> List<Adt0> {
    f25({
        let m37 = {
            let (h36, _) = f26(Fn3::F27, 1000000u64, Adt5::C0(v0, List::<Adt0>::nil()));
            h36
        };
        match m37 {
            Adt5::C0(_, h35) => {
                h35
            }
        }
    })
}

pub fn f12(v0: Bytes) -> Result<Bytes, Adt1> {
    Err::<Bytes, Adt1>(Adt1::C0(0u64, v0))
}

pub fn f13(v0: Adt0) -> Option<Adt0> {
    let m38 = v0;
    match m38 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(r39) => {
            let v1 = (*r39).clone();
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

pub fn f14(v0: Fn1, p44: u64, p45: Adt3) -> (Adt3, bool) {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m40 = v1;
                if m40 == 0 {
                    Err::<(u64, Adt3), (Adt3, bool)>((v2, false))
                } else {
                    let v3 = m40 - 1;
                    let m42 = {
                        let c41: Fn1 = v0.clone();
                        c41.apply(v2.clone())
                    };
                    match m42 {
                        None => {
                            Err::<(u64, Adt3), (Adt3, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), (Adt3, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m43 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: Adt3) -> Option<Adt3> {
    f28(v0, v1)
}

pub fn f16(v0: Fn2, p50: u64, p51: Adt4) -> R<(Adt4, bool)> {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v1, v2) = m49;
                let m46 = v1;
                if m46 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m46 - 1;
                    let m48 = {
                        let c47: Fn2 = v0.clone();
                        c47.apply(v2.clone())?
                    };
                    match m48 {
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
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Adt4) -> R<Option<Adt4>> {
    f29(v0)
}

pub fn f18(v0: Adt0) -> Option<Bytes> {
    let m52 = v0;
    match m52 {
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

pub fn f19(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m53 = f1(v0);
    match m53 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f3(v2, v1)
        }
    }
}

pub fn f20(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m54 = f19(f30(v0.clone(), v1)?, f4(Str::lit("items")));
    match m54 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v2) => {
            f31(v0, v2)
        }
    }
}

pub fn f21(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m55 = f19(v0, f4(Str::lit("$defs")));
    match m55 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f19(v2, v1)
        }
    }
}

pub fn f22(v0: Bytes, v1: Bytes) -> Bytes {
    let a56 = v0;
    let a57 = v1;
    append_bytes(a56, a57)
}

pub fn f23(v0: Adt6) -> R<Bytes> {
    f32(v0.clone(), 100u64, f33(v0)?)
}

pub fn f24(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Adt6> {
    let m58 = v1;
    match m58 {
        Adt0::C0(v5) => {
            f40(Adt0::C0(v5))
        }
        Adt0::C1(v6, v7) => {
            f40(Adt0::C1(v6, v7))
        }
        Adt0::C2(v8) => {
            f40(Adt0::C2(v8))
        }
        Adt0::C3(r59) => {
            let v4 = (*r59).clone();
            Ok(f37(v2.clone(), f38(v0.clone(), v4, f39(v0, v2)?)?))
        }
        Adt0::C4(r60) => {
            let v3 = (*r60).clone();
            f34(v3.clone(), v2.clone(), f35(v0.clone(), v3.clone(), f36(v0, v3, v2)?)?)
        }
        Adt0::C5 => {
            Ok(Adt6::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt6::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt6::C0)
        }
    }
}

pub fn f25(v0: List<Adt0>) -> List<Adt0> {
    f41(Fn4::F42, List::<Adt0>::nil(), v0)
}

pub fn f26(v0: Fn3, p65: u64, p66: Adt5) -> (Adt5, bool) {
    let m64 = (p65, p66);
    {
        let mut m64 = m64;
        loop {
            let step = {
                let (v1, v2) = m64;
                let m61 = v1;
                if m61 == 0 {
                    Err::<(u64, Adt5), (Adt5, bool)>((v2, false))
                } else {
                    let v3 = m61 - 1;
                    let m63 = {
                        let c62: Fn3 = v0.clone();
                        c62.apply(v2.clone())
                    };
                    match m63 {
                        None => {
                            Err::<(u64, Adt5), (Adt5, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), (Adt5, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m64 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt5) -> Option<Adt5> {
    f43(v0)
}

pub fn f28(v0: Bytes, v1: Adt3) -> Option<Adt3> {
    let m68 = match v1.clone() {
        Adt3::C0(_, h67) => {
            h67
        }
    };
    match m68 {
        None => {
            let m70 = f44(match v1 {
                Adt3::C0(h69, _) => {
                    h69
                }
            });
            match m70 {
                None => {
                    None::<Adt3>
                }
                Some(v3) => {
                    if f45(match v3.clone() {
                        Adt10::C0(h71, _, _) => {
                            h71
                        }
                    }, v0) {
                        Some(Adt3::C0(Adt0::C5, Some(match v3 {
                            Adt10::C0(_, h72, _) => {
                                h72
                            }
                        })))
                    } else {
                        Some(Adt3::C0(match v3 {
                            Adt10::C0(_, _, h73) => {
                                h73
                            }
                        }, None::<Adt0>))
                    }
                }
            }
        }
        Some(_) => {
            None::<Adt3>
        }
    }
}

pub fn f29(v0: Adt4) -> R<Option<Adt4>> {
    let m75 = f44(match v0.clone() {
        Adt4::C0(h74, _) => {
            h74
        }
    });
    match m75 {
        None => {
            let m81 = f46(match v0.clone() {
                Adt4::C0(h80, _) => {
                    h80
                }
            });
            match m81 {
                None => {
                    Ok(None::<Adt4>)
                }
                Some(v2) => {
                    Ok(Some(Adt4::C0(match v2 {
                        Adt11::C0(_, h82) => {
                            h82
                        }
                    }, {
                        let a84 = match v0 {
                            Adt4::C0(_, h83) => {
                                h83
                            }
                        };
                        let a85 = 1u64;
                        nat_add(a84, a85)?
                    })))
                }
            }
        }
        Some(v1) => {
            Ok(Some(Adt4::C0(match v1 {
                Adt10::C0(_, _, h76) => {
                    h76
                }
            }, {
                let a78 = match v0 {
                    Adt4::C0(_, h77) => {
                        h77
                    }
                };
                let a79 = 1u64;
                nat_add(a78, a79)?
            })))
        }
    }
}

pub fn f30(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let (h86, _) = f47(Fn5::F48(v0), 4096u64, v1)?;
    Ok(h86)
}

pub fn f31(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m87 = f7(f19(v1.clone(), f4(Str::lit("$ref"))));
    match m87 {
        None => {
            Ok(Adt2::C4(v1))
        }
        Some(v2) => {
            let v3: Bytes = f49(v2)?;
            if f45(v3.clone(), f4(Str::lit("term"))) {
                Ok(Adt2::C0)
            } else {
                if f45(v3.clone(), f4(Str::lit("type"))) {
                    Ok(Adt2::C1)
                } else {
                    if f45(v3.clone(), f4(Str::lit("member"))) {
                        Ok(Adt2::C2)
                    } else {
                        if f45(v3, f4(Str::lit("parameter"))) {
                            Ok(Adt2::C3)
                        } else {
                            Ok(Adt2::C4(f30(v0, v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f32(v0: Adt6, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt7 = {
        let (h88, _) = f50(Fn6::F51(v2, v1), {
            let a89 = v2;
            let a90 = 1u64;
            nat_add(a89, a90)?
        }, Adt7::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt8::C0(0u64, false, v0), List::<Adt8>::nil())))?;
        h88
    };
    Ok(f52(match v3 {
        Adt7::C0(h91, _, _) => {
            h91
        }
    }))
}

pub fn f33(v0: Adt6) -> R<u64> {
    let m92 = v0;
    match m92 {
        Adt6::C0 => {
            Ok(1u64)
        }
        Adt6::C1(_) => {
            Ok(1u64)
        }
        Adt6::C2(r101, r102) => {
            let v2 = (*r101).clone();
            let v3 = (*r102).clone();
            let v4: u64 = f33(v2)?;
            let v5: u64 = f33(v3)?;
            let a95 = {
                let a93 = 1u64;
                let a94 = v4;
                nat_add(a93, a94)?
            };
            let a96 = v5;
            nat_add(a95, a96)
        }
        Adt6::C3(_, r103) => {
            let v7 = (*r103).clone();
            let v8: u64 = f33(v7)?;
            let a97 = 1u64;
            let a98 = v8;
            nat_add(a97, a98)
        }
        Adt6::C4 => {
            Ok(1u64)
        }
        Adt6::C5 => {
            Ok(1u64)
        }
        Adt6::C6(r104) => {
            let v9 = (*r104).clone();
            let v10: u64 = f33(v9)?;
            let a99 = 1u64;
            let a100 = v10;
            nat_add(a99, a100)
        }
    }
}

pub fn f34(v0: Adt0, v1: Adt2, v2: List<(Bytes, Adt6)>) -> R<Adt6> {
    let m105 = v1;
    match m105 {
        Adt2::C0 => {
            if f53(v0.clone())? {
                f54(v0.clone(), {
                    let m106 = f55(v0);
                    match m106 {
                        None => {
                            f4(Str::lit(""))
                        }
                        Some(v3) => {
                            v3
                        }
                    }
                }, v2)
            } else {
                f56(v0, v2)
            }
        }
        Adt2::C1 => {
            if f57(v0.clone())? {
                f58(v0.clone(), {
                    let m107 = f55(v0);
                    match m107 {
                        None => {
                            f4(Str::lit(""))
                        }
                        Some(v4) => {
                            v4
                        }
                    }
                }, v2)
            } else {
                f56(v0, v2)
            }
        }
        Adt2::C2 => {
            let m108 = f59(Adt0::C4(std::rc::Rc::new(v0.clone())))?;
            match m108 {
                None => {
                    f40(Adt0::C4(std::rc::Rc::new(v0)))
                }
                Some(v5) => {
                    Ok(f60(v5))
                }
            }
        }
        Adt2::C3 => {
            if f61(Adt0::C4(std::rc::Rc::new(v0.clone())))? {
                Ok(f62(List::cons(f60(f22(f63(v0, f4(Str::lit("name"))), f4(Str::lit(" : ")))), List::cons(f64(v2, f4(Str::lit("type"))), List::<Adt6>::nil()))))
            } else {
                f56(v0, v2)
            }
        }
        Adt2::C4(_) => {
            f56(v0, v2)
        }
        Adt2::C5 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C6 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C7 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C8 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C9 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C10 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C11 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C12 => {
            Ok(f62(List::cons(f60(f22(f63(v0, f4(Str::lit("field"))), f4(Str::lit(" := ")))), List::cons(f64(v2, f4(Str::lit("value"))), List::<Adt6>::nil()))))
        }
        Adt2::C13 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C14 => {
            f65(v0, v2)
        }
    }
}

pub fn f35(v0: Adt0, v1: Adt0, v2: Adt9) -> R<List<(Bytes, Adt6)>> {
    let m109 = v1;
    match m109 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C7(v3, r110, r111) => {
            let v4 = (*r110).clone();
            let v5 = (*r111).clone();
            if f66(v2.clone()) {
                Ok(List::<(Bytes, Adt6)>::nil())
            } else {
                Ok(List::cons((v3.clone(), f24(v0.clone(), v4, f67(v0.clone(), v2.clone(), v3)?)?), f35(v0, v5, v2)?))
            }
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Adt9> {
    let m112 = v2.clone();
    match m112 {
        Adt2::C0 => {
            if f53(v1.clone())? {
                Ok(Adt9::C1({
                    let m113 = f55(v1);
                    match m113 {
                        None => {
                            f4(Str::lit(""))
                        }
                        Some(v3) => {
                            v3
                        }
                    }
                }))
            } else {
                Ok(Adt9::C0(f68(v0, v1, v2)?))
            }
        }
        Adt2::C1 => {
            if f57(v1.clone())? {
                Ok(Adt9::C2)
            } else {
                Ok(Adt9::C0(f68(v0, v1, v2)?))
            }
        }
        Adt2::C2 => {
            Ok(Adt9::C6)
        }
        Adt2::C3 => {
            if f61(Adt0::C4(std::rc::Rc::new(v1.clone())))? {
                Ok(Adt9::C3)
            } else {
                Ok(Adt9::C0(f68(v0, v1, v2)?))
            }
        }
        Adt2::C4(_) => {
            Ok(Adt9::C0(f68(v0, v1, v2)?))
        }
        Adt2::C5 => {
            Ok(Adt9::C6)
        }
        Adt2::C6 => {
            Ok(Adt9::C6)
        }
        Adt2::C7 => {
            Ok(Adt9::C6)
        }
        Adt2::C8 => {
            Ok(Adt9::C6)
        }
        Adt2::C9 => {
            Ok(Adt9::C6)
        }
        Adt2::C10 => {
            Ok(Adt9::C6)
        }
        Adt2::C11 => {
            Ok(Adt9::C6)
        }
        Adt2::C12 => {
            Ok(Adt9::C4)
        }
        Adt2::C13 => {
            Ok(Adt9::C6)
        }
        Adt2::C14 => {
            Ok(Adt9::C5)
        }
    }
}

pub fn f37(v0: Adt2, v1: List<Adt6>) -> Adt6 {
    let m114 = v0;
    match m114 {
        Adt2::C0 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C1 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C2 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C3 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C4(_) => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C5 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C6 => {
            f62(f69(v1, f4(Str::lit(" ")), f4(Str::lit(""))))
        }
        Adt2::C7 => {
            f62(List::cons(f60(f4(Str::lit("["))), {
                let a115 = f70(v1, f4(Str::lit(", ")));
                let a116 = List::cons(f60(f4(Str::lit("]"))), List::<Adt6>::nil());
                append_list(a115, a116)
            }))
        }
        Adt2::C8 => {
            f62(f69(v1, f4(Str::lit("(")), f4(Str::lit(") "))))
        }
        Adt2::C9 => {
            f62(List::cons(f60(f4(Str::lit("("))), {
                let a117 = f70(v1, f4(Str::lit(" ")));
                let a118 = List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil());
                append_list(a117, a118)
            }))
        }
        Adt2::C10 => {
            f62(f69(v1, f4(Str::lit(" ")), f4(Str::lit(""))))
        }
        Adt2::C11 => {
            f62(List::cons(f60(f4(Str::lit("{"))), {
                let a119 = f70(v1, f4(Str::lit(", ")));
                let a120 = List::cons(f60(f4(Str::lit("}"))), List::<Adt6>::nil());
                append_list(a119, a120)
            }))
        }
        Adt2::C12 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C13 => {
            f62(f71(Fn7::F72, List::<Adt6>::nil(), v1))
        }
        Adt2::C14 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
    }
}

pub fn f38(v0: Adt0, v1: Adt0, v2: Adt2) -> R<List<Adt6>> {
    let m121 = v1;
    match m121 {
        Adt0::C0(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C6(r122, r123) => {
            let v3 = (*r122).clone();
            let v4 = (*r123).clone();
            Ok(List::cons(f24(v0.clone(), v3, v2.clone())?, f38(v0, v4, v2)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt6>::nil())
        }
    }
}

pub fn f39(v0: Adt0, v1: Adt2) -> R<Adt2> {
    let m124 = v1;
    match m124 {
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
            f20(v0, v2)
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

pub fn f40(v0: Adt0) -> R<Adt6> {
    let m125 = v0;
    match m125 {
        Adt0::C0(v1) => {
            Ok(Adt6::C1(if v1 {
                Bytes::lit(&[0x74, 0x72, 0x75, 0x65])
            } else {
                Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65])
            }))
        }
        Adt0::C1(v2, v3) => {
            Ok(Adt6::C1(f75(v2, v3)))
        }
        Adt0::C2(v4) => {
            Ok(Adt6::C1(f76(v4)?))
        }
        Adt0::C3(r126) => {
            let v5 = (*r126).clone();
            Ok(f73(f77(Str::lit("[")), f77(Str::lit("]")), f74(f78(v5)?)))
        }
        Adt0::C4(r127) => {
            let v6 = (*r127).clone();
            f79(f80(v6.clone())?, f81(v6)?)
        }
        Adt0::C5 => {
            Ok(Adt6::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt6::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt6::C0)
        }
    }
}

pub fn f41(v0: Fn4, p131: List<Adt0>, p132: List<Adt0>) -> List<Adt0> {
    let m130 = (p131, p132);
    {
        let mut m130 = m130;
        loop {
            let step = {
                let (v1, v2) = m130;
                let m128 = v2;
                match m128.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c129: Fn4 = v0.clone();
                            c129.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f43(v0: Adt5) -> Option<Adt5> {
    let m134 = f46(match v0.clone() {
        Adt5::C0(h133, _) => {
            h133
        }
    });
    match m134 {
        None => {
            None::<Adt5>
        }
        Some(v1) => {
            Some(Adt5::C0(match v1.clone() {
                Adt11::C0(_, h135) => {
                    h135
                }
            }, List::cons(match v1 {
                Adt11::C0(h136, _) => {
                    h136
                }
            }, match v0 {
                Adt5::C0(_, h137) => {
                    h137
                }
            })))
        }
    }
}

pub fn f44(v0: Adt0) -> Option<Adt10> {
    let m138 = v0;
    match m138 {
        Adt0::C0(_) => {
            None::<Adt10>
        }
        Adt0::C1(_, _) => {
            None::<Adt10>
        }
        Adt0::C2(_) => {
            None::<Adt10>
        }
        Adt0::C3(_) => {
            None::<Adt10>
        }
        Adt0::C4(_) => {
            None::<Adt10>
        }
        Adt0::C5 => {
            None::<Adt10>
        }
        Adt0::C6(_, _) => {
            None::<Adt10>
        }
        Adt0::C7(v1, r139, r140) => {
            let v2 = (*r139).clone();
            let v3 = (*r140).clone();
            Some(Adt10::C0(v1, v2, v3))
        }
    }
}

pub fn f45(v0: Bytes, v1: Bytes) -> bool {
    let a141 = v0;
    let a142 = v1;
    equal(a141, a142)
}

pub fn f46(v0: Adt0) -> Option<Adt11> {
    let m143 = v0;
    match m143 {
        Adt0::C0(_) => {
            None::<Adt11>
        }
        Adt0::C1(_, _) => {
            None::<Adt11>
        }
        Adt0::C2(_) => {
            None::<Adt11>
        }
        Adt0::C3(_) => {
            None::<Adt11>
        }
        Adt0::C4(_) => {
            None::<Adt11>
        }
        Adt0::C5 => {
            None::<Adt11>
        }
        Adt0::C6(r144, r145) => {
            let v1 = (*r144).clone();
            let v2 = (*r145).clone();
            Some(Adt11::C0(v1, v2))
        }
        Adt0::C7(_, _, _) => {
            None::<Adt11>
        }
    }
}

pub fn f47(v0: Fn5, p150: u64, p151: Adt0) -> R<(Adt0, bool)> {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v1, v2) = m149;
                let m146 = v1;
                if m146 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m146 - 1;
                    let m148 = {
                        let c147: Fn5 = v0.clone();
                        c147.apply(v2.clone())?
                    };
                    match m148 {
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
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    f82(v0, v1)
}

pub fn f49(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let m157 = {
            let (h153, _) = f83(Fn8::F84(v0.clone()), {
                let a155 = {
                    let a154 = v0.clone();
                    length_bytes(a154)
                };
                let a156 = 1u64;
                nat_add(a155, a156)?
            }, Adt12::C0(0u64, 0u64))?;
            h153
        };
        match m157 {
            Adt12::C0(_, h152) => {
                h152
            }
        }
    };
    Ok(f85(v0.clone(), v1, {
        let a159 = {
            let a158 = v0;
            length_bytes(a158)
        };
        let a160 = v1;
        nat_sub(a159, a160)
    }))
}

pub fn f50(v0: Fn6, p165: u64, p166: Adt7) -> R<(Adt7, bool)> {
    let m164 = (p165, p166);
    {
        let mut m164 = m164;
        loop {
            let step = {
                let (v1, v2) = m164;
                let m161 = v1;
                if m161 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m161 - 1;
                    let m163 = {
                        let c162: Fn6 = v0.clone();
                        c162.apply(v2.clone())?
                    };
                    match m163 {
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
                Ok(next) => { m164 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: u64, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    f86(v1, v0, v2)
}

pub fn f52(v0: List<Bytes>) -> Bytes {
    f87(f88(v0))
}

pub fn f53(v0: Adt0) -> R<bool> {
    let m167 = f55(v0.clone());
    match m167 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f89(v0, v1)
        }
    }
}

pub fn f54(v0: Adt0, v1: Bytes, v2: List<(Bytes, Adt6)>) -> R<Adt6> {
    let m168 = f90(v1.clone());
    match m168 {
        None => {
            if f45(v1.clone(), f4(Str::lit("var"))) {
                Ok(f60(f63(v0, f4(Str::lit("name")))))
            } else {
                if f45(v1.clone(), f4(Str::lit("nat"))) {
                    Ok(f60(f63(v0, f4(Str::lit("value")))))
                } else {
                    if f45(v1.clone(), f4(Str::lit("bool"))) {
                        Ok(f60({
                            let m169 = f3(v0, f4(Str::lit("value")));
                            match m169 {
                                None => {
                                    f4(Str::lit(""))
                                }
                                Some(v4) => {
                                    let m170 = f93(v4);
                                    match m170 {
                                        None => {
                                            f4(Str::lit(""))
                                        }
                                        Some(v5) => {
                                            if v5 {
                                                f4(Str::lit("true"))
                                            } else {
                                                f4(Str::lit("false"))
                                            }
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if f45(v1.clone(), f4(Str::lit("string"))) {
                            Ok(f60(f94(f63(v0, f4(Str::lit("value"))))?))
                        } else {
                            if f45(v1.clone(), f4(Str::lit("bytes"))) {
                                Ok(f60(f22(f4(Str::lit("0x")), f63(v0, f4(Str::lit("hex"))))))
                            } else {
                                if f45(v1.clone(), f4(Str::lit("unit"))) {
                                    Ok(f60(f4(Str::lit("()"))))
                                } else {
                                    if f45(v1.clone(), f4(Str::lit("integer"))) {
                                        Ok(f60(f22(f22(f22(f22(f4(Str::lit("(")), f63(v0.clone(), f4(Str::lit("value")))), f4(Str::lit(" : "))), {
                                            let m171 = f95(f63(v0, f4(Str::lit("representation"))));
                                            match m171 {
                                                None => {
                                                    f4(Str::lit(""))
                                                }
                                                Some(v6) => {
                                                    v6
                                                }
                                            }
                                        }), f4(Str::lit(")")))))
                                    } else {
                                        if f45(v1.clone(), f4(Str::lit("pair"))) {
                                            Ok(f62(List::cons(f60(f4(Str::lit("("))), List::cons(f64(v2.clone(), f4(Str::lit("left"))), List::cons(f60(f4(Str::lit(", "))), List::cons(f64(v2, f4(Str::lit("right"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
                                        } else {
                                            if if f45(v1.clone(), f4(Str::lit("not"))) {
                                                true
                                            } else {
                                                if f45(v1.clone(), f4(Str::lit("first"))) {
                                                    true
                                                } else {
                                                    f45(v1.clone(), f4(Str::lit("second")))
                                                }
                                            } {
                                                Ok(f96(v1, List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("value"))), List::<Adt6>::nil()))))
                                            } else {
                                                if f45(v1.clone(), f4(Str::lit("cons"))) {
                                                    Ok(f96(v1, List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("head"))), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("tail"))), List::<Adt6>::nil()))))))
                                                } else {
                                                    if f45(v1.clone(), f4(Str::lit("nil"))) {
                                                        Ok(f96(v1, List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("element"))), List::<Adt6>::nil()))))
                                                    } else {
                                                        if f45(v1.clone(), f4(Str::lit("if"))) {
                                                            Ok(f62(List::cons(f60(f4(Str::lit("(if "))), List::cons(f64(v2.clone(), f4(Str::lit("condition"))), List::cons(f60(f4(Str::lit(" then "))), List::cons(f64(v2.clone(), f4(Str::lit("then_value"))), List::cons(f60(f4(Str::lit(" else "))), List::cons(f64(v2, f4(Str::lit("else_value"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))))
                                                        } else {
                                                            if f45(v1.clone(), f4(Str::lit("let"))) {
                                                                Ok(f62(List::cons(f60(f4(Str::lit("(let "))), List::cons(f64(v2.clone(), f4(Str::lit("binder"))), List::cons(f60(f4(Str::lit(" := "))), List::cons(f64(v2.clone(), f4(Str::lit("value"))), List::cons(f60(f4(Str::lit("; "))), List::cons(f64(v2, f4(Str::lit("body"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))))
                                                            } else {
                                                                if f45(v1.clone(), f4(Str::lit("forall"))) {
                                                                    Ok(f62(List::cons(f60(f4(Str::lit("(forall ("))), List::cons(f64(v2.clone(), f4(Str::lit("binder"))), List::cons(f60(f4(Str::lit("), "))), List::cons(f64(v2, f4(Str::lit("body"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
                                                                } else {
                                                                    if f45(v1.clone(), f4(Str::lit("lambda"))) {
                                                                        let v7: List<Adt6> = f97(v0, f4(Str::lit("captures")));
                                                                        Ok(f62(List::cons(f60(f4(Str::lit("(fun "))), {
                                                                            let a177 = {
                                                                                let a173 = 0u64;
                                                                                let a174 = {
                                                                                    let a172 = v7.clone();
                                                                                    length_list(a172)
                                                                                };
                                                                                if nat_lt(a173, a174) {
                                                                                    List::cons(f60(f4(Str::lit("["))), {
                                                                                        let a175 = f70(v7, f4(Str::lit(", ")));
                                                                                        let a176 = List::cons(f60(f4(Str::lit("]"))), List::cons(f60(f4(Str::lit(" "))), List::<Adt6>::nil()));
                                                                                        append_list(a175, a176)
                                                                                    })
                                                                                } else {
                                                                                    List::<Adt6>::nil()
                                                                                }
                                                                            };
                                                                            let a178 = List::cons(f64(v2.clone(), f4(Str::lit("parameters"))), List::cons(f60(f4(Str::lit("=> "))), List::cons(f64(v2, f4(Str::lit("body"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil()))));
                                                                            append_list(a177, a178)
                                                                        })))
                                                                    } else {
                                                                        if f45(v1.clone(), f4(Str::lit("apply"))) {
                                                                            Ok(f96(f4(Str::lit("apply")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("function"))), List::cons(f64(v2, f4(Str::lit("arguments"))), List::<Adt6>::nil())))))
                                                                        } else {
                                                                            if f45(v1.clone(), f4(Str::lit("project"))) {
                                                                                Ok(f96(f4(Str::lit("project")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("value"))), List::cons(f60(f22(f4(Str::lit(" ")), f63(v0, f4(Str::lit("field"))))), List::<Adt6>::nil())))))
                                                                            } else {
                                                                                if f45(v1.clone(), f4(Str::lit("call"))) {
                                                                                    Ok(f62(List::cons(f60(f22(f4(Str::lit("(")), f98(v0.clone(), f4(Str::lit("function")))?)), {
                                                                                        let a179 = f99(v0, v2.clone());
                                                                                        let a180 = List::cons(f64(v2, f4(Str::lit("arguments"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil()));
                                                                                        append_list(a179, a180)
                                                                                    })))
                                                                                } else {
                                                                                    if f45(v1.clone(), f4(Str::lit("constructor"))) {
                                                                                        Ok(f96(f4(Str::lit("ctor")), List::cons(f60(f22(f4(Str::lit(" ")), f98(v0.clone(), f4(Str::lit("constructor")))?)), {
                                                                                            let a181 = f99(v0, v2.clone());
                                                                                            let a182 = List::cons(f64(v2, f4(Str::lit("arguments"))), List::<Adt6>::nil());
                                                                                            append_list(a181, a182)
                                                                                        })))
                                                                                    } else {
                                                                                        if f45(v1.clone(), f4(Str::lit("function_ref"))) {
                                                                                            Ok(f96(f4(Str::lit("ref")), List::cons(f60(f22(f4(Str::lit(" ")), f98(v0.clone(), f4(Str::lit("function")))?)), f99(v0, v2))))
                                                                                        } else {
                                                                                            if f45(v1.clone(), f4(Str::lit("record"))) {
                                                                                                Ok(f96(f4(Str::lit("record")), List::cons(f60(f22(f4(Str::lit(" ")), f98(v0.clone(), f4(Str::lit("type")))?)), {
                                                                                                    let a183 = f99(v0, v2.clone());
                                                                                                    let a184 = List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("fields"))), List::<Adt6>::nil()));
                                                                                                    append_list(a183, a184)
                                                                                                })))
                                                                                            } else {
                                                                                                if f45(v1.clone(), f4(Str::lit("primitive"))) {
                                                                                                    Ok(f62(List::cons(f60(f22(f4(Str::lit("(prim ")), f63(v0, f4(Str::lit("operation"))))), List::cons(f64(v2.clone(), f4(Str::lit("arguments"))), List::cons(f60(f4(Str::lit(" : "))), List::cons(f64(v2, f4(Str::lit("result"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
                                                                                                } else {
                                                                                                    if f45(v1, f4(Str::lit("checked_apply"))) {
                                                                                                        Ok(f96(f4(Str::lit("checked")), List::cons(f60(f22(f4(Str::lit(" ")), f98(v0.clone(), f4(Str::lit("model")))?)), {
                                                                                                            let a187 = f99(v0.clone(), v2.clone());
                                                                                                            let a188 = List::cons(f64(v2, f4(Str::lit("arguments"))), List::cons(f60(f4(Str::lit(" with "))), List::cons(f60(f4(Str::lit("["))), {
                                                                                                                let a185 = f70(f97(v0, f4(Str::lit("checks"))), f4(Str::lit(", ")));
                                                                                                                let a186 = List::cons(f60(f4(Str::lit("]"))), List::<Adt6>::nil());
                                                                                                                append_list(a185, a186)
                                                                                                            })));
                                                                                                            append_list(a187, a188)
                                                                                                        })))
                                                                                                    } else {
                                                                                                        Ok(Adt6::C6(std::rc::Rc::new(Adt6::C3(2u64, std::rc::Rc::new(f62(List::cons(f60(f4(Str::lit("(match "))), List::cons(f64(v2.clone(), f4(Str::lit("scrutinee"))), List::cons(f64(v2, f4(Str::lit("branches"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil()))))))))))
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
            Ok(f62(List::cons(f60(f4(Str::lit("("))), List::cons(f64(v2.clone(), f91(v1.clone())), List::cons(f60(f22(f22(f4(Str::lit(" ")), v3), f4(Str::lit(" ")))), List::cons(f64(v2, f92(v1)), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
        }
    }
}

pub fn f55(v0: Adt0) -> Option<Bytes> {
    f7(f3(v0, f4(Str::lit("kind"))))
}

pub fn f56(v0: Adt0, v1: List<(Bytes, Adt6)>) -> R<Adt6> {
    f79(f80(v0)?, v1)
}

pub fn f57(v0: Adt0) -> R<bool> {
    let m189 = f55(v0.clone());
    match m189 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f100(v0, v1)
        }
    }
}

pub fn f58(v0: Adt0, v1: Bytes, v2: List<(Bytes, Adt6)>) -> R<Adt6> {
    let m190 = f101(v1.clone());
    match m190 {
        None => {
            if f45(v1.clone(), f4(Str::lit("parameter"))) {
                Ok(f60(f22(f4(Str::lit("'")), f63(v0, f4(Str::lit("name"))))))
            } else {
                if f45(v1.clone(), f4(Str::lit("list"))) {
                    Ok(f96(f4(Str::lit("List")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("element"))), List::<Adt6>::nil()))))
                } else {
                    if f45(v1.clone(), f4(Str::lit("set"))) {
                        Ok(f96(f4(Str::lit("Set")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("element"))), List::<Adt6>::nil()))))
                    } else {
                        if f45(v1.clone(), f4(Str::lit("option"))) {
                            Ok(f96(f4(Str::lit("Option")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("value"))), List::<Adt6>::nil()))))
                        } else {
                            if f45(v1.clone(), f4(Str::lit("result"))) {
                                Ok(f96(f4(Str::lit("Result")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("ok"))), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("error"))), List::<Adt6>::nil()))))))
                            } else {
                                if f45(v1.clone(), f4(Str::lit("map"))) {
                                    Ok(f96(f4(Str::lit("Map")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("key"))), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("value"))), List::<Adt6>::nil()))))))
                                } else {
                                    if f45(v1.clone(), f4(Str::lit("product"))) {
                                        Ok(f62(List::cons(f60(f4(Str::lit("("))), List::cons(f64(v2.clone(), f4(Str::lit("left"))), List::cons(f60(f4(Str::lit(" * "))), List::cons(f64(v2, f4(Str::lit("right"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
                                    } else {
                                        if f45(v1, f4(Str::lit("function"))) {
                                            Ok(f96(f4(Str::lit("Fn")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("parameters"))), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("result"))), List::<Adt6>::nil()))))))
                                        } else {
                                            let v4: Bytes = f98(v0.clone(), f4(Str::lit("member")))?;
                                            let m191 = f2(f3(v0, f4(Str::lit("arguments"))));
                                            match m191 {
                                                None => {
                                                    Ok(f60(v4))
                                                }
                                                Some(v5) => {
                                                    let a192 = f6(v5)?;
                                                    let a193 = 0u64;
                                                    if nat_eq(a192, a193) {
                                                        Ok(f60(v4))
                                                    } else {
                                                        Ok(f62(List::cons(f60(f22(f4(Str::lit("(")), v4)), List::cons(f64(v2, f4(Str::lit("arguments"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))
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
            Ok(f60(v3))
        }
    }
}

pub fn f59(v0: Adt0) -> R<Option<Bytes>> {
    let m194 = f1(v0);
    match m194 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let m195 = f7(f3(v1.clone(), f4(Str::lit("name"))));
            match m195 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    let v3: Option<Adt0> = f3(v1.clone(), f4(Str::lit("module")));
                    if f5({
                        let a196 = f6(v1)?;
                        let a197 = if f102(v3.clone()) {
                            2u64
                        } else {
                            1u64
                        };
                        nat_eq(a196, a197)
                    }, f103(v2.clone())?) {
                        let m198 = v3;
                        match m198 {
                            None => {
                                Ok(Some(v2))
                            }
                            Some(v4) => {
                                let m199 = f18(v4);
                                match m199 {
                                    None => {
                                        Ok(None::<Bytes>)
                                    }
                                    Some(v5) => {
                                        if f103(v5.clone())? {
                                            Ok(Some(f22(f22(v5, f4(Str::lit("::"))), v2)))
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

pub fn f60(v0: Bytes) -> Adt6 {
    Adt6::C1(v0)
}

pub fn f61(v0: Adt0) -> R<bool> {
    let m200 = f1(v0);
    match m200 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            let m201 = f7(f3(v1.clone(), f4(Str::lit("name"))));
            match m201 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    Ok(f5(f102(f3(v1.clone(), f4(Str::lit("type")))), f5({
                        let a202 = f6(v1)?;
                        let a203 = 2u64;
                        nat_eq(a202, a203)
                    }, f103(v2)?)))
                }
            }
        }
    }
}

pub fn f62(v0: List<Adt6>) -> Adt6 {
    f104(Fn9::F105, Adt6::C0, f74(v0))
}

pub fn f63(v0: Adt0, v1: Bytes) -> Bytes {
    let m204 = f7(f3(v0, v1));
    match m204 {
        None => {
            f4(Str::lit(""))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f64(v0: List<(Bytes, Adt6)>, v1: Bytes) -> Adt6 {
    f106(Fn10::F107(v1), Adt6::C0, v0)
}

pub fn f65(v0: Adt0, v1: List<(Bytes, Adt6)>) -> R<Adt6> {
    Ok(f62(List::cons(f60(f22(f4(Str::lit("| ")), f98(v0.clone(), f4(Str::lit("constructor")))?)), {
        let a205 = f69(f97(v0, f4(Str::lit("binders"))), f4(Str::lit(" ")), f4(Str::lit("")));
        let a206 = List::cons(f60(f4(Str::lit(" => "))), List::cons(f64(v1, f4(Str::lit("body"))), List::<Adt6>::nil()));
        append_list(a205, a206)
    })))
}

pub fn f66(v0: Adt9) -> bool {
    let m207 = v0;
    match m207 {
        Adt9::C0(_) => {
            false
        }
        Adt9::C1(_) => {
            false
        }
        Adt9::C2 => {
            false
        }
        Adt9::C3 => {
            false
        }
        Adt9::C4 => {
            false
        }
        Adt9::C5 => {
            false
        }
        Adt9::C6 => {
            true
        }
    }
}

pub fn f67(v0: Adt0, v1: Adt9, v2: Bytes) -> R<Adt2> {
    let m208 = v1;
    match m208 {
        Adt9::C0(v3) => {
            f108(v0, v3, v2)
        }
        Adt9::C1(v4) => {
            Ok(f109(v4, v2))
        }
        Adt9::C2 => {
            Ok(f110(v2))
        }
        Adt9::C3 => {
            if f45(v2, f4(Str::lit("type"))) {
                Ok(Adt2::C1)
            } else {
                Ok(Adt2::C5)
            }
        }
        Adt9::C4 => {
            if f45(v2, f4(Str::lit("value"))) {
                Ok(Adt2::C0)
            } else {
                Ok(Adt2::C5)
            }
        }
        Adt9::C5 => {
            if f45(v2.clone(), f4(Str::lit("body"))) {
                Ok(Adt2::C0)
            } else {
                if f45(v2, f4(Str::lit("constructor"))) {
                    Ok(Adt2::C2)
                } else {
                    Ok(Adt2::C5)
                }
            }
        }
        Adt9::C6 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f68(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Option<Adt0>> {
    let v3: Option<Bytes> = f80(v1)?;
    let m209 = v2.clone();
    match m209 {
        Adt2::C0 => {
            let m210 = v3;
            match m210 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    let m211 = f111(v0.clone(), v2)?;
                    match m211 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v5) => {
                            f112(v0, v5, v4)
                        }
                    }
                }
            }
        }
        Adt2::C1 => {
            let m212 = v3;
            match m212 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v6) => {
                    let m213 = f111(v0.clone(), v2)?;
                    match m213 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v7) => {
                            f112(v0, v7, v6)
                        }
                    }
                }
            }
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(f21(v0, f4(Str::lit("parameter"))))
        }
        Adt2::C4(v8) => {
            let m214 = v3;
            match m214 {
                None => {
                    Ok(Some(v8))
                }
                Some(v9) => {
                    if f113(v0.clone(), v8.clone())? {
                        f112(v0, v8, v9)
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

pub fn f69(v0: List<Adt6>, v1: Bytes, v2: Bytes) -> List<Adt6> {
    f74(f71(Fn7::F114(v2, v1), List::<Adt6>::nil(), v0))
}

pub fn f70(v0: List<Adt6>, v1: Bytes) -> List<Adt6> {
    f74({
        let (_, h215) = f115(Fn11::F116(v1), (true, List::<Adt6>::nil()), v0);
        h215
    })
}

pub fn f71(v0: Fn7, p219: List<Adt6>, p220: List<Adt6>) -> List<Adt6> {
    let m218 = (p219, p220);
    {
        let mut m218 = m218;
        loop {
            let step = {
                let (v1, v2) = m218;
                let m216 = v2;
                match m216.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>(({
                            let c217: Fn7 = v0.clone();
                            c217.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    let a221 = v0;
    let a222 = List::cons(Adt6::C4, List::cons(v1, List::<Adt6>::nil()));
    append_list(a221, a222)
}

pub fn f73(v0: Bytes, v1: Bytes, v2: List<Adt6>) -> Adt6 {
    let m223 = v2.clone();
    match m223.uncons() {
        None => {
            Adt6::C1(f117(v0, v1))
        }
        Some((_, _)) => {
            let v5: (bool, Adt6) = f118(Fn12::F119, (true, Adt6::C0), v2);
            Adt6::C6(std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C1(v0)), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C3(2u64, std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C5), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new({
                let (_, h224) = v5;
                h224
            }), std::rc::Rc::new(Adt6::C0))))))), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C5), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C1(v1)), std::rc::Rc::new(Adt6::C0))))))))))
        }
    }
}

pub fn f74(v0: List<Adt6>) -> List<Adt6> {
    f71(Fn7::F120, List::<Adt6>::nil(), v0)
}

pub fn f75(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a225 = 0u64;
        let a226 = v1;
        nat_lt(a225, a226)
    } else {
        false
    } {
        let a227 = Bytes::lit(&[0x2d]);
        let a228 = f121(v1);
        append_bytes(a227, a228)
    } else {
        f121(v1)
    }
}

pub fn f76(v0: Bytes) -> R<Bytes> {
    if f103(v0.clone())? {
        Ok(v0)
    } else {
        f94(v0)
    }
}

pub fn f77(v0: Str) -> Bytes {
    let a229 = v0;
    utf8_encode(a229)
}

pub fn f78(v0: Adt0) -> R<List<Adt6>> {
    let m230 = v0;
    match m230 {
        Adt0::C0(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C6(r231, r232) => {
            let v7 = (*r231).clone();
            let v8 = (*r232).clone();
            Ok(List::cons(f40(v7)?, f78(v8)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt6>::nil())
        }
    }
}

pub fn f79(v0: Option<Bytes>, v1: List<(Bytes, Adt6)>) -> R<Adt6> {
    let v2: List<(Bytes, Adt6)> = f122(Fn13::F123(v0.clone()), List::<(Bytes, Adt6)>::nil(), v1)?;
    Ok(f73({
        let m233 = v0;
        match m233 {
            None => {
                f77(Str::lit("{"))
            }
            Some(v3) => {
                f117(v3, f77(Str::lit("{")))
            }
        }
    }, f77(Str::lit("}")), f124(Fn14::F125, List::<Adt6>::nil(), f126(v2)?)))
}

pub fn f80(p241: Adt0) -> R<Option<Bytes>> {
    let m240 = p241;
    {
        let mut m240 = m240;
        loop {
            let step = {
                let v0 = m240;
                let m234 = v0;
                match m234 {
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
                    Adt0::C7(v9, r238, r239) => {
                        let v10 = (*r238).clone();
                        let v11 = (*r239).clone();
                        let a235 = v9;
                        let a236 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
                        if equal(a235, a236) {
                            let m237 = v10;
                            match m237 {
                                Adt0::C0(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C1(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C2(v12) => {
                                    if f103(v12.clone())? {
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
                Ok(next) => { m240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Adt0) -> R<List<(Bytes, Adt6)>> {
    let m242 = v0;
    match m242 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C7(v9, r243, r244) => {
            let v10 = (*r243).clone();
            let v11 = (*r244).clone();
            Ok(List::cons((v9, f40(v10)?), f81(v11)?))
        }
    }
}

pub fn f82(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    let m245 = f7(f19(v1, f4(Str::lit("$ref"))));
    match m245 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v2) => {
            Ok(f21(v0, f49(v2)?))
        }
    }
}

pub fn f83(v0: Fn8, p250: u64, p251: Adt12) -> R<(Adt12, bool)> {
    let m249 = (p250, p251);
    {
        let mut m249 = m249;
        loop {
            let step = {
                let (v1, v2) = m249;
                let m246 = v1;
                if m246 == 0 {
                    Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m246 - 1;
                    let m248 = {
                        let c247: Fn8 = v0.clone();
                        c247.apply(v2.clone())?
                    };
                    match m248 {
                        None => {
                            Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt12), R<(Adt12, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: Adt12) -> R<Option<Adt12>> {
    f127(v0, v1)
}

pub fn f85(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m255 = {
        let a252 = v0;
        let a253 = v1;
        let a254 = v2;
        slice_bytes(a252, a253, a254)
    };
    match m255 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f86(v0: u64, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let m257 = match v2.clone() {
        Adt7::C0(_, _, h256) => {
            h256
        }
    };
    match m257.uncons() {
        None => {
            Ok(None::<Adt7>)
        }
        Some((v3, v4)) => {
            Ok(Some(f128(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f87(v0: List<Bytes>) -> Bytes {
    let m260 = {
        let (h258, _) = f129(Fn15::F130, {
            let a259 = v0.clone();
            length_list(a259)
        }, v0);
        h258
    };
    match m260.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f88(v0: List<Bytes>) -> List<Bytes> {
    f131(Fn16::F132, List::<Bytes>::nil(), v0)
}

pub fn f89(v0: Adt0, v1: Bytes) -> R<bool> {
    let m261 = f90(v1.clone());
    match m261 {
        None => {
            if f45(v1.clone(), f4(Str::lit("var"))) {
                Ok(f5(f133(v0.clone(), f134(Str::lit("name")))?, {
                    let m262 = f7(f3(v0, f4(Str::lit("name"))));
                    match m262 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            f135(v3)?
                        }
                    }
                }))
            } else {
                if f45(v1.clone(), f4(Str::lit("nat"))) {
                    Ok(f5(f133(v0.clone(), f134(Str::lit("value")))?, {
                        let m263 = f7(f3(v0, f4(Str::lit("value"))));
                        match m263 {
                            None => {
                                false
                            }
                            Some(v4) => {
                                f136(v4)?
                            }
                        }
                    }))
                } else {
                    if f45(v1.clone(), f4(Str::lit("bool"))) {
                        Ok(f5(f133(v0.clone(), f134(Str::lit("value")))?, {
                            let m264 = f3(v0, f4(Str::lit("value")));
                            match m264 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    let m265 = f93(v5);
                                    match m265 {
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
                        if f45(v1.clone(), f4(Str::lit("string"))) {
                            Ok(f5(f133(v0.clone(), f134(Str::lit("value")))?, {
                                let m266 = f7(f3(v0, f4(Str::lit("value"))));
                                match m266 {
                                    None => {
                                        false
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }))
                        } else {
                            if f45(v1.clone(), f4(Str::lit("bytes"))) {
                                Ok(f5(f133(v0.clone(), f134(Str::lit("hex")))?, {
                                    let m267 = f7(f3(v0, f4(Str::lit("hex"))));
                                    match m267 {
                                        None => {
                                            false
                                        }
                                        Some(v8) => {
                                            f137(v8)?
                                        }
                                    }
                                }))
                            } else {
                                if f45(v1.clone(), f4(Str::lit("unit"))) {
                                    f133(v0, List::cons(f4(Str::lit("kind")), List::<Bytes>::nil()))
                                } else {
                                    if f45(v1.clone(), f4(Str::lit("integer"))) {
                                        Ok(f5(f133(v0.clone(), f138(Str::lit("representation"), Str::lit("value")))?, {
                                            let m268 = f7(f3(v0.clone(), f4(Str::lit("value"))));
                                            match m268 {
                                                None => {
                                                    false
                                                }
                                                Some(v9) => {
                                                    let m269 = f7(f3(v0, f4(Str::lit("representation"))));
                                                    match m269 {
                                                        None => {
                                                            false
                                                        }
                                                        Some(v10) => {
                                                            f5(f102({
                                                                let m270 = f95(v10);
                                                                match m270 {
                                                                    None => {
                                                                        None::<Adt0>
                                                                    }
                                                                    Some(_) => {
                                                                        Some(Adt0::C5)
                                                                    }
                                                                }
                                                            }), f139(v9)?)
                                                        }
                                                    }
                                                }
                                            }
                                        }))
                                    } else {
                                        if f45(v1.clone(), f4(Str::lit("pair"))) {
                                            f133(v0, f138(Str::lit("left"), Str::lit("right")))
                                        } else {
                                            if if f45(v1.clone(), f4(Str::lit("not"))) {
                                                true
                                            } else {
                                                if f45(v1.clone(), f4(Str::lit("first"))) {
                                                    true
                                                } else {
                                                    f45(v1.clone(), f4(Str::lit("second")))
                                                }
                                            } {
                                                f133(v0, f134(Str::lit("value")))
                                            } else {
                                                if f45(v1.clone(), f4(Str::lit("cons"))) {
                                                    f133(v0, f138(Str::lit("head"), Str::lit("tail")))
                                                } else {
                                                    if f45(v1.clone(), f4(Str::lit("nil"))) {
                                                        f133(v0, f134(Str::lit("element")))
                                                    } else {
                                                        if f45(v1.clone(), f4(Str::lit("if"))) {
                                                            f133(v0, f140(Str::lit("condition"), Str::lit("then_value"), Str::lit("else_value")))
                                                        } else {
                                                            if f45(v1.clone(), f4(Str::lit("let"))) {
                                                                Ok(f5(f133(v0.clone(), f140(Str::lit("binder"), Str::lit("value"), Str::lit("body")))?, f141(f3(v0, f4(Str::lit("binder"))))?))
                                                            } else {
                                                                if f45(v1.clone(), f4(Str::lit("forall"))) {
                                                                    Ok(f5(f133(v0.clone(), f138(Str::lit("binder"), Str::lit("body")))?, f141(f3(v0, f4(Str::lit("binder"))))?))
                                                                } else {
                                                                    if f45(v1.clone(), f4(Str::lit("lambda"))) {
                                                                        Ok(f5(f133(v0.clone(), f140(Str::lit("parameters"), Str::lit("captures"), Str::lit("body")))?, f5(f142(v0.clone(), f4(Str::lit("parameters")), Fn17::F143)?, f142(v0, f4(Str::lit("captures")), Fn17::F144)?)))
                                                                    } else {
                                                                        if f45(v1.clone(), f4(Str::lit("apply"))) {
                                                                            Ok(f5(f133(v0.clone(), f138(Str::lit("function"), Str::lit("arguments")))?, f145(v0, f4(Str::lit("arguments")))))
                                                                        } else {
                                                                            if f45(v1.clone(), f4(Str::lit("project"))) {
                                                                                Ok(f5(f133(v0.clone(), f138(Str::lit("value"), Str::lit("field")))?, {
                                                                                    let m271 = f7(f3(v0, f4(Str::lit("field"))));
                                                                                    match m271 {
                                                                                        None => {
                                                                                            false
                                                                                        }
                                                                                        Some(v12) => {
                                                                                            f103(v12)?
                                                                                        }
                                                                                    }
                                                                                }))
                                                                            } else {
                                                                                if f45(v1.clone(), f4(Str::lit("call"))) {
                                                                                    Ok(f5(f146(v0.clone(), f138(Str::lit("function"), Str::lit("arguments")))?, f5({
                                                                                        let m272 = f3(v0.clone(), f4(Str::lit("function")));
                                                                                        match m272 {
                                                                                            None => {
                                                                                                false
                                                                                            }
                                                                                            Some(v13) => {
                                                                                                let m273 = f59(v13.clone())?;
                                                                                                match m273 {
                                                                                                    None => {
                                                                                                        false
                                                                                                    }
                                                                                                    Some(_) => {
                                                                                                        let m274 = f7(f19(v13, f4(Str::lit("name"))));
                                                                                                        match m274 {
                                                                                                            None => {
                                                                                                                false
                                                                                                            }
                                                                                                            Some(v15) => {
                                                                                                                f135(v15)?
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }, f5(f147(v0.clone())?, f145(v0, f4(Str::lit("arguments")))))))
                                                                                } else {
                                                                                    if f45(v1.clone(), f4(Str::lit("constructor"))) {
                                                                                        Ok(f5(f146(v0.clone(), f138(Str::lit("constructor"), Str::lit("arguments")))?, f5(f148(f3(v0.clone(), f4(Str::lit("constructor"))))?, f5(f147(v0.clone())?, f145(v0, f4(Str::lit("arguments")))))))
                                                                                    } else {
                                                                                        if f45(v1.clone(), f4(Str::lit("function_ref"))) {
                                                                                            Ok(f5(f146(v0.clone(), f134(Str::lit("function")))?, f5(f148(f3(v0.clone(), f4(Str::lit("function"))))?, f147(v0)?)))
                                                                                        } else {
                                                                                            if f45(v1.clone(), f4(Str::lit("record"))) {
                                                                                                Ok(f5(f146(v0.clone(), f138(Str::lit("type"), Str::lit("fields")))?, f5(f148(f3(v0.clone(), f4(Str::lit("type"))))?, f5(f147(v0.clone())?, f142(v0, f4(Str::lit("fields")), Fn17::F149)?))))
                                                                                            } else {
                                                                                                if f45(v1.clone(), f4(Str::lit("primitive"))) {
                                                                                                    Ok(f5(f133(v0.clone(), f140(Str::lit("operation"), Str::lit("arguments"), Str::lit("result")))?, f5({
                                                                                                        let m275 = f7(f3(v0.clone(), f4(Str::lit("operation"))));
                                                                                                        match m275 {
                                                                                                            None => {
                                                                                                                false
                                                                                                            }
                                                                                                            Some(v16) => {
                                                                                                                f103(v16)?
                                                                                                            }
                                                                                                        }
                                                                                                    }, f145(v0, f4(Str::lit("arguments"))))))
                                                                                                } else {
                                                                                                    if f45(v1.clone(), f4(Str::lit("checked_apply"))) {
                                                                                                        Ok(f5(f146(v0.clone(), f140(Str::lit("model"), Str::lit("arguments"), Str::lit("checks")))?, f5(f148(f3(v0.clone(), f4(Str::lit("model"))))?, f5(f147(v0.clone())?, f5(f145(v0.clone(), f4(Str::lit("arguments"))), f142(v0, f4(Str::lit("checks")), Fn17::F150)?)))))
                                                                                                    } else {
                                                                                                        if f45(v1, f4(Str::lit("match"))) {
                                                                                                            Ok(f5(f133(v0.clone(), f138(Str::lit("scrutinee"), Str::lit("branches")))?, f142(v0, f4(Str::lit("branches")), Fn17::F151)?))
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
            f133(v0, List::cons(f4(Str::lit("kind")), List::cons(f91(v1.clone()), List::cons(f92(v1), List::<Bytes>::nil()))))
        }
    }
}

pub fn f90(v0: Bytes) -> Option<Bytes> {
    if f45(v0.clone(), f4(Str::lit("add"))) {
        Some(f4(Str::lit("+")))
    } else {
        if f45(v0.clone(), f4(Str::lit("eq"))) {
            Some(f4(Str::lit("=")))
        } else {
            if f45(v0.clone(), f4(Str::lit("le"))) {
                Some(f4(Str::lit("<=")))
            } else {
                if f45(v0.clone(), f4(Str::lit("lt"))) {
                    Some(f4(Str::lit("<")))
                } else {
                    if f45(v0.clone(), f4(Str::lit("beq"))) {
                        Some(f4(Str::lit("=?")))
                    } else {
                        if f45(v0.clone(), f4(Str::lit("ble"))) {
                            Some(f4(Str::lit("<=?")))
                        } else {
                            if f45(v0.clone(), f4(Str::lit("blt"))) {
                                Some(f4(Str::lit("<?")))
                            } else {
                                if f45(v0.clone(), f4(Str::lit("and"))) {
                                    Some(f4(Str::lit("&&")))
                                } else {
                                    if f45(v0.clone(), f4(Str::lit("or"))) {
                                        Some(f4(Str::lit("||")))
                                    } else {
                                        if f45(v0.clone(), f4(Str::lit("prop_and"))) {
                                            Some(f4(Str::lit("/\\")))
                                        } else {
                                            if f45(v0.clone(), f4(Str::lit("iff"))) {
                                                Some(f4(Str::lit("<->")))
                                            } else {
                                                if f45(v0, f4(Str::lit("implies"))) {
                                                    Some(f4(Str::lit("->")))
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

pub fn f91(v0: Bytes) -> Bytes {
    if f45(v0, f4(Str::lit("implies"))) {
        f4(Str::lit("premise"))
    } else {
        f4(Str::lit("left"))
    }
}

pub fn f92(v0: Bytes) -> Bytes {
    if f45(v0, f4(Str::lit("implies"))) {
        f4(Str::lit("conclusion"))
    } else {
        f4(Str::lit("right"))
    }
}

pub fn f93(v0: Adt0) -> Option<bool> {
    let m276 = v0;
    match m276 {
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

pub fn f94(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a277 = v0.clone();
        length_bytes(a277)
    };
    let v2: Adt13 = {
        let (h278, _) = f152(Fn18::F153(v1, v0), {
            let a279 = v1;
            let a280 = 1u64;
            nat_add(a279, a280)?
        }, Adt13::C0(0u64, List::<Bytes>::nil()))?;
        h278
    };
    Ok(f154(f154(f155(Str::lit("\"")), f156(match v2 {
        Adt13::C0(_, h281) => {
            h281
        }
    })), f155(Str::lit("\""))))
}

pub fn f95(v0: Bytes) -> Option<Bytes> {
    if f45(v0.clone(), f4(Str::lit("int"))) {
        Some(f4(Str::lit("Int")))
    } else {
        if f45(v0.clone(), f4(Str::lit("int8"))) {
            Some(f4(Str::lit("Int8")))
        } else {
            if f45(v0.clone(), f4(Str::lit("int16"))) {
                Some(f4(Str::lit("Int16")))
            } else {
                if f45(v0.clone(), f4(Str::lit("int32"))) {
                    Some(f4(Str::lit("Int32")))
                } else {
                    if f45(v0.clone(), f4(Str::lit("int64"))) {
                        Some(f4(Str::lit("Int64")))
                    } else {
                        if f45(v0.clone(), f4(Str::lit("uint8"))) {
                            Some(f4(Str::lit("UInt8")))
                        } else {
                            if f45(v0.clone(), f4(Str::lit("uint16"))) {
                                Some(f4(Str::lit("UInt16")))
                            } else {
                                if f45(v0.clone(), f4(Str::lit("uint32"))) {
                                    Some(f4(Str::lit("UInt32")))
                                } else {
                                    if f45(v0, f4(Str::lit("uint64"))) {
                                        Some(f4(Str::lit("UInt64")))
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

pub fn f96(v0: Bytes, v1: List<Adt6>) -> Adt6 {
    f62(List::cons(f60(f22(f4(Str::lit("(")), v0)), {
        let a282 = v1;
        let a283 = List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil());
        append_list(a282, a283)
    }))
}

pub fn f97(v0: Adt0, v1: Bytes) -> List<Adt6> {
    let m284 = f2(f3(v0, v1));
    match m284 {
        None => {
            List::<Adt6>::nil()
        }
        Some(v2) => {
            f157(Fn19::F158, List::<Adt6>::nil(), f11(v2))
        }
    }
}

pub fn f98(v0: Adt0, v1: Bytes) -> R<Bytes> {
    let m285 = f3(v0, v1);
    match m285 {
        None => {
            Ok(f4(Str::lit("")))
        }
        Some(v2) => {
            let m286 = f59(v2)?;
            match m286 {
                None => {
                    Ok(f4(Str::lit("")))
                }
                Some(v3) => {
                    Ok(v3)
                }
            }
        }
    }
}

pub fn f99(v0: Adt0, v1: List<(Bytes, Adt6)>) -> List<Adt6> {
    if f102(f3(v0, f4(Str::lit("type_arguments")))) {
        List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v1, f4(Str::lit("type_arguments"))), List::<Adt6>::nil()))
    } else {
        List::<Adt6>::nil()
    }
}

pub fn f100(v0: Adt0, v1: Bytes) -> R<bool> {
    let m287 = f101(v1.clone());
    match m287 {
        None => {
            if f45(v1.clone(), f4(Str::lit("parameter"))) {
                Ok(f5(f133(v0.clone(), f134(Str::lit("name")))?, {
                    let m288 = f7(f3(v0, f4(Str::lit("name"))));
                    match m288 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            f103(v3)?
                        }
                    }
                }))
            } else {
                if if f45(v1.clone(), f4(Str::lit("list"))) {
                    true
                } else {
                    f45(v1.clone(), f4(Str::lit("set")))
                } {
                    f133(v0, f134(Str::lit("element")))
                } else {
                    if f45(v1.clone(), f4(Str::lit("option"))) {
                        f133(v0, f134(Str::lit("value")))
                    } else {
                        if f45(v1.clone(), f4(Str::lit("result"))) {
                            f133(v0, f138(Str::lit("ok"), Str::lit("error")))
                        } else {
                            if f45(v1.clone(), f4(Str::lit("map"))) {
                                f133(v0, f138(Str::lit("key"), Str::lit("value")))
                            } else {
                                if f45(v1.clone(), f4(Str::lit("product"))) {
                                    f133(v0, f138(Str::lit("left"), Str::lit("right")))
                                } else {
                                    if f45(v1.clone(), f4(Str::lit("function"))) {
                                        Ok(f5(f133(v0.clone(), f138(Str::lit("parameters"), Str::lit("result")))?, f145(v0, f4(Str::lit("parameters")))))
                                    } else {
                                        if f45(v1, f4(Str::lit("named"))) {
                                            Ok(f5(f133(v0.clone(), f138(Str::lit("member"), Str::lit("arguments")))?, {
                                                let m289 = f3(v0.clone(), f4(Str::lit("member")));
                                                match m289 {
                                                    None => {
                                                        false
                                                    }
                                                    Some(v4) => {
                                                        let m290 = f1(v4.clone());
                                                        match m290 {
                                                            None => {
                                                                false
                                                            }
                                                            Some(v5) => {
                                                                let m291 = f7(f3(v5.clone(), f4(Str::lit("name"))));
                                                                match m291 {
                                                                    None => {
                                                                        false
                                                                    }
                                                                    Some(v6) => {
                                                                        f5(!f5(!f102(f3(v5, f4(Str::lit("module")))), f159(v6)), f5(f145(v0, f4(Str::lit("arguments"))), f102({
                                                                            let m292 = f59(v4)?;
                                                                            match m292 {
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
            f133(v0, List::cons(f4(Str::lit("kind")), List::<Bytes>::nil()))
        }
    }
}

pub fn f101(v0: Bytes) -> Option<Bytes> {
    if f45(v0.clone(), f4(Str::lit("bool"))) {
        Some(f4(Str::lit("Bool")))
    } else {
        if f45(v0.clone(), f4(Str::lit("bytes"))) {
            Some(f4(Str::lit("Bytes")))
        } else {
            if f45(v0.clone(), f4(Str::lit("int"))) {
                Some(f4(Str::lit("Int")))
            } else {
                if f45(v0.clone(), f4(Str::lit("int8"))) {
                    Some(f4(Str::lit("Int8")))
                } else {
                    if f45(v0.clone(), f4(Str::lit("int16"))) {
                        Some(f4(Str::lit("Int16")))
                    } else {
                        if f45(v0.clone(), f4(Str::lit("int32"))) {
                            Some(f4(Str::lit("Int32")))
                        } else {
                            if f45(v0.clone(), f4(Str::lit("int64"))) {
                                Some(f4(Str::lit("Int64")))
                            } else {
                                if f45(v0.clone(), f4(Str::lit("nat"))) {
                                    Some(f4(Str::lit("Nat")))
                                } else {
                                    if f45(v0.clone(), f4(Str::lit("ordering"))) {
                                        Some(f4(Str::lit("Ordering")))
                                    } else {
                                        if f45(v0.clone(), f4(Str::lit("prop"))) {
                                            Some(f4(Str::lit("Prop")))
                                        } else {
                                            if f45(v0.clone(), f4(Str::lit("string"))) {
                                                Some(f4(Str::lit("String")))
                                            } else {
                                                if f45(v0.clone(), f4(Str::lit("type"))) {
                                                    Some(f4(Str::lit("Type")))
                                                } else {
                                                    if f45(v0.clone(), f4(Str::lit("uint8"))) {
                                                        Some(f4(Str::lit("UInt8")))
                                                    } else {
                                                        if f45(v0.clone(), f4(Str::lit("uint16"))) {
                                                            Some(f4(Str::lit("UInt16")))
                                                        } else {
                                                            if f45(v0.clone(), f4(Str::lit("uint32"))) {
                                                                Some(f4(Str::lit("UInt32")))
                                                            } else {
                                                                if f45(v0.clone(), f4(Str::lit("uint64"))) {
                                                                    Some(f4(Str::lit("UInt64")))
                                                                } else {
                                                                    if f45(v0.clone(), f4(Str::lit("unit"))) {
                                                                        Some(f4(Str::lit("Unit")))
                                                                    } else {
                                                                        if f45(v0.clone(), f4(Str::lit("contract_violation"))) {
                                                                            Some(f4(Str::lit("ContractViolation")))
                                                                        } else {
                                                                            if f45(v0, f4(Str::lit("reasoning_failure"))) {
                                                                                Some(f4(Str::lit("ReasoningFailure")))
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

pub fn f102(v0: Option<Adt0>) -> bool {
    let m293 = v0;
    match m293 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f103(v0: Bytes) -> R<bool> {
    let a295 = {
        let a294 = v0.clone();
        length_bytes(a294)
    };
    let a296 = 0u64;
    if nat_eq(a295, a296) {
        Ok(false)
    } else {
        if f160(f161(v0.clone(), 0u64)?) {
            let a298 = f162(v0.clone(), 1u64)?;
            let a299 = {
                let a297 = v0.clone();
                length_bytes(a297)
            };
            if nat_eq(a298, a299) {
                let a300 = f163(v0);
                Ok(bool_not(a300))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f104(v0: Fn9, p304: Adt6, p305: List<Adt6>) -> Adt6 {
    let m303 = (p304, p305);
    {
        let mut m303 = m303;
        loop {
            let step = {
                let (v1, v2) = m303;
                let m301 = v2;
                match m301.uncons() {
                    None => {
                        Err::<(Adt6, List<Adt6>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<Adt6>), Adt6>(({
                            let c302: Fn9 = v0.clone();
                            c302.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Adt6, v1: Adt6) -> Adt6 {
    Adt6::C2(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f106(v0: Fn10, p309: Adt6, p310: List<(Bytes, Adt6)>) -> Adt6 {
    let m308 = (p309, p310);
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v1, v2) = m308;
                let m306 = v2;
                match m306.uncons() {
                    None => {
                        Err::<(Adt6, List<(Bytes, Adt6)>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<(Bytes, Adt6)>), Adt6>(({
                            let c307: Fn10 = v0.clone();
                            c307.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: Bytes, v1: Adt6, v2: (Bytes, Adt6)) -> Adt6 {
    if f45({
        let (h311, _) = v2;
        h311
    }, v0) {
        let (_, h312) = v2;
        h312
    } else {
        v1
    }
}

pub fn f108(v0: Adt0, v1: Option<Adt0>, v2: Bytes) -> R<Adt2> {
    let m313 = v1;
    match m313 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v3) => {
            let m314 = f19(f30(v0.clone(), v3)?, f4(Str::lit("properties")));
            match m314 {
                None => {
                    Ok(Adt2::C5)
                }
                Some(v4) => {
                    let m315 = f19(v4, v2);
                    match m315 {
                        None => {
                            Ok(Adt2::C5)
                        }
                        Some(v5) => {
                            f31(v0, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f109(v0: Bytes, v1: Bytes) -> Adt2 {
    if f45(v1.clone(), f4(Str::lit("element"))) {
        Adt2::C1
    } else {
        if f45(v1.clone(), f4(Str::lit("binder"))) {
            Adt2::C3
        } else {
            if f45(v1.clone(), f4(Str::lit("parameters"))) {
                Adt2::C8
            } else {
                if f45(v1.clone(), f4(Str::lit("arguments"))) {
                    Adt2::C6
                } else {
                    if f45(v1.clone(), f4(Str::lit("type_arguments"))) {
                        Adt2::C7
                    } else {
                        if f45(v1.clone(), f4(Str::lit("fields"))) {
                            Adt2::C11
                        } else {
                            if f45(v1.clone(), f4(Str::lit("branches"))) {
                                Adt2::C13
                            } else {
                                if f45(v1.clone(), f4(Str::lit("result"))) {
                                    Adt2::C1
                                } else {
                                    if if f45(v1.clone(), f4(Str::lit("type"))) {
                                        true
                                    } else {
                                        if f45(v1.clone(), f4(Str::lit("constructor"))) {
                                            true
                                        } else {
                                            f45(v1.clone(), f4(Str::lit("model")))
                                        }
                                    } {
                                        Adt2::C2
                                    } else {
                                        if f45(v1, f4(Str::lit("function"))) {
                                            if f45(v0, f4(Str::lit("apply"))) {
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

pub fn f110(v0: Bytes) -> Adt2 {
    if f45(v0.clone(), f4(Str::lit("parameters"))) {
        Adt2::C9
    } else {
        if f45(v0.clone(), f4(Str::lit("arguments"))) {
            Adt2::C10
        } else {
            if f45(v0.clone(), f4(Str::lit("member"))) {
                Adt2::C2
            } else {
                if f45(v0, f4(Str::lit("name"))) {
                    Adt2::C5
                } else {
                    Adt2::C1
                }
            }
        }
    }
}

pub fn f111(v0: Adt0, v1: Adt2) -> R<Option<Adt0>> {
    let m316 = v1;
    match m316 {
        Adt2::C0 => {
            Ok(f21(v0, f4(Str::lit("term"))))
        }
        Adt2::C1 => {
            Ok(f21(v0, f4(Str::lit("type"))))
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(None::<Adt0>)
        }
        Adt2::C4(v2) => {
            if f113(v0, v2.clone())? {
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

pub fn f112(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Option<Adt0>> {
    let m317 = f2(f19(f30(v0.clone(), v1)?, f4(Str::lit("oneOf"))));
    match m317 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v3) => {
            f164(Fn20::F165(v2, v0), None::<Adt0>, f11(v3))
        }
    }
}

pub fn f113(v0: Adt0, v1: Adt0) -> R<bool> {
    let m318 = f2(f19(f30(v0.clone(), v1)?, f4(Str::lit("oneOf"))));
    match m318 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            f166(Fn21::F167(v0), true, f11(v2))
        }
    }
}

pub fn f114(v0: Bytes, v1: Bytes, v2: List<Adt6>, v3: Adt6) -> List<Adt6> {
    List::cons(f60(v0), List::cons(v3, List::cons(f60(v1), v2)))
}

pub fn f115(v0: Fn11, p322: (bool, List<Adt6>), p323: List<Adt6>) -> (bool, List<Adt6>) {
    let m321 = (p322, p323);
    {
        let mut m321 = m321;
        loop {
            let step = {
                let (v1, v2) = m321;
                let m319 = v2;
                match m319.uncons() {
                    None => {
                        Err::<((bool, List<Adt6>), List<Adt6>), (bool, List<Adt6>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt6>), List<Adt6>), (bool, List<Adt6>)>(({
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

pub fn f116(v0: Bytes, v1: (bool, List<Adt6>), v2: Adt6) -> (bool, List<Adt6>) {
    let (h324, _) = v1;
    if h324 {
        (false, List::cons(v2, {
            let (_, h325) = v1;
            h325
        }))
    } else {
        (false, List::cons(v2, List::cons(f60(v0), {
            let (_, h326) = v1;
            h326
        })))
    }
}

pub fn f117(v0: Bytes, v1: Bytes) -> Bytes {
    let a327 = v0;
    let a328 = v1;
    append_bytes(a327, a328)
}

pub fn f118(v0: Fn12, p332: (bool, Adt6), p333: List<Adt6>) -> (bool, Adt6) {
    let m331 = (p332, p333);
    {
        let mut m331 = m331;
        loop {
            let step = {
                let (v1, v2) = m331;
                let m329 = v2;
                match m329.uncons() {
                    None => {
                        Err::<((bool, Adt6), List<Adt6>), (bool, Adt6)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt6), List<Adt6>), (bool, Adt6)>(({
                            let c330: Fn12 = v0.clone();
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

pub fn f119(v0: (bool, Adt6), v1: Adt6) -> (bool, Adt6) {
    f168(v0, v1)
}

pub fn f120(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(v1, v0)
}

pub fn f121(v0: u64) -> Bytes {
    f169(f170(), 10u64, v0)
}

pub fn f122(v0: Fn13, p337: List<(Bytes, Adt6)>, p338: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    let m336 = (p337, p338);
    {
        let mut m336 = m336;
        loop {
            let step = {
                let (v1, v2) = m336;
                let m334 = v2;
                match m334.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt6)>, List<(Bytes, Adt6)>), R<List<(Bytes, Adt6)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt6)>, List<(Bytes, Adt6)>), R<List<(Bytes, Adt6)>>>(({
                            let c335: Fn13 = v0.clone();
                            c335.apply(v1, v3)?
                        }, v4))
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

pub fn f123(v0: Option<Bytes>, v1: List<(Bytes, Adt6)>, v2: (Bytes, Adt6)) -> R<List<(Bytes, Adt6)>> {
    let m339 = v0;
    match m339 {
        None => {
            Ok(List::cons((f171({
                let (h346, _) = v2.clone();
                h346
            }), f172({
                let (h347, _) = v2;
                h347
            }, {
                let (_, h348) = v2;
                h348
            })?), v1))
        }
        Some(_) => {
            let a341 = {
                let (h340, _) = v2.clone();
                h340
            };
            let a342 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
            if equal(a341, a342) {
                Ok(v1)
            } else {
                Ok(List::cons((f171({
                    let (h343, _) = v2.clone();
                    h343
                }), f172({
                    let (h344, _) = v2;
                    h344
                }, {
                    let (_, h345) = v2;
                    h345
                })?), v1))
            }
        }
    }
}

pub fn f124(v0: Fn14, p352: List<Adt6>, p353: List<(Bytes, Adt6)>) -> List<Adt6> {
    let m351 = (p352, p353);
    {
        let mut m351 = m351;
        loop {
            let step = {
                let (v1, v2) = m351;
                let m349 = v2;
                match m349.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<(Bytes, Adt6)>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<(Bytes, Adt6)>), List<Adt6>>(({
                            let c350: Fn14 = v0.clone();
                            c350.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: List<Adt6>, v1: (Bytes, Adt6)) -> List<Adt6> {
    List::cons({
        let (_, h354) = v1;
        h354
    }, v0)
}

pub fn f126(v0: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    let v1: List<List<(Bytes, Adt6)>> = f173(f174(Fn22::F175, List::<List<(Bytes, Adt6)>>::nil(), v0.clone()));
    let m359 = {
        let (h355, _) = f176(Fn23::F177, {
            let a357 = {
                let a356 = v0;
                length_list(a356)
            };
            let a358 = 1u64;
            nat_add(a357, a358)?
        }, v1)?;
        h355
    };
    match m359.uncons() {
        None => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f127(v0: Bytes, v1: Adt12) -> R<Option<Adt12>> {
    let v2: u64 = match v1.clone() {
        Adt12::C0(h360, _) => {
            h360
        }
    };
    let m361 = f178(v0, v2);
    match m361 {
        None => {
            Ok(None::<Adt12>)
        }
        Some(v3) => {
            Ok(Some(Adt12::C0({
                let a362 = v2;
                let a363 = 1u64;
                nat_add(a362, a363)?
            }, {
                let a410 = {
                    let v4: u8 = v3;
                    let a408 = {
                        let a384 = {
                            let a372 = {
                                let a366 = {
                                    let a364 = v4;
                                    let a365 = 128u8;
                                    fixed_u8::bit_and(a364, a365)
                                };
                                let a367 = 128u8;
                                if equal(a366, a367) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a373 = {
                                let a370 = {
                                    let a368 = v4;
                                    let a369 = 64u8;
                                    fixed_u8::bit_and(a368, a369)
                                };
                                let a371 = 64u8;
                                if equal(a370, a371) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a372, a373)?
                        };
                        let a385 = {
                            let a382 = {
                                let a376 = {
                                    let a374 = v4;
                                    let a375 = 32u8;
                                    fixed_u8::bit_and(a374, a375)
                                };
                                let a377 = 32u8;
                                if equal(a376, a377) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a383 = {
                                let a380 = {
                                    let a378 = v4;
                                    let a379 = 16u8;
                                    fixed_u8::bit_and(a378, a379)
                                };
                                let a381 = 16u8;
                                if equal(a380, a381) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a382, a383)?
                        };
                        nat_add(a384, a385)?
                    };
                    let a409 = {
                        let a406 = {
                            let a394 = {
                                let a388 = {
                                    let a386 = v4;
                                    let a387 = 8u8;
                                    fixed_u8::bit_and(a386, a387)
                                };
                                let a389 = 8u8;
                                if equal(a388, a389) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a395 = {
                                let a392 = {
                                    let a390 = v4;
                                    let a391 = 4u8;
                                    fixed_u8::bit_and(a390, a391)
                                };
                                let a393 = 4u8;
                                if equal(a392, a393) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a394, a395)?
                        };
                        let a407 = {
                            let a404 = {
                                let a398 = {
                                    let a396 = v4;
                                    let a397 = 2u8;
                                    fixed_u8::bit_and(a396, a397)
                                };
                                let a399 = 2u8;
                                if equal(a398, a399) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a405 = {
                                let a402 = {
                                    let a400 = v4;
                                    let a401 = 1u8;
                                    fixed_u8::bit_and(a400, a401)
                                };
                                let a403 = 1u8;
                                if equal(a402, a403) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a404, a405)?
                        };
                        nat_add(a406, a407)?
                    };
                    nat_add(a408, a409)?
                };
                let a411 = 47u64;
                if nat_eq(a410, a411) {
                    let a412 = v2;
                    let a413 = 1u64;
                    nat_add(a412, a413)?
                } else {
                    match v1 {
                        Adt12::C0(_, h414) => {
                            h414
                        }
                    }
                }
            })))
        }
    }
}

pub fn f128(v0: u64, v1: u64, v2: Adt7, v3: Adt8, v4: List<Adt8>) -> R<Adt7> {
    let v5: u64 = match v3.clone() {
        Adt8::C0(h415, _, _) => {
            h415
        }
    };
    let v6: bool = match v3.clone() {
        Adt8::C0(_, h416, _) => {
            h416
        }
    };
    let m418 = match v3 {
        Adt8::C0(_, _, h417) => {
            h417
        }
    };
    match m418 {
        Adt6::C0 => {
            Ok(Adt7::C0(match v2.clone() {
                Adt7::C0(h419, _, _) => {
                    h419
                }
            }, match v2 {
                Adt7::C0(_, h420, _) => {
                    h420
                }
            }, v4))
        }
        Adt6::C1(v7) => {
            Ok(Adt7::C0(List::cons(v7.clone(), match v2.clone() {
                Adt7::C0(h421, _, _) => {
                    h421
                }
            }), {
                let a423 = match v2 {
                    Adt7::C0(_, h422, _) => {
                        h422
                    }
                };
                let a424 = f179(v7)?;
                nat_add(a423, a424)?
            }, v4))
        }
        Adt6::C2(r441, r442) => {
            let v8 = (*r441).clone();
            let v9 = (*r442).clone();
            Ok(Adt7::C0(match v2.clone() {
                Adt7::C0(h425, _, _) => {
                    h425
                }
            }, match v2 {
                Adt7::C0(_, h426, _) => {
                    h426
                }
            }, List::cons(Adt8::C0(v5, v6, v8), List::cons(Adt8::C0(v5, v6, v9), v4))))
        }
        Adt6::C3(v10, r443) => {
            let v11 = (*r443).clone();
            Ok(Adt7::C0(match v2.clone() {
                Adt7::C0(h427, _, _) => {
                    h427
                }
            }, match v2 {
                Adt7::C0(_, h428, _) => {
                    h428
                }
            }, List::cons(Adt8::C0({
                let a429 = v5;
                let a430 = v10;
                nat_add(a429, a430)?
            }, v6, v11), v4)))
        }
        Adt6::C4 => {
            if v6 {
                Ok(Adt7::C0(List::cons({
                    let a431 = Str::lit(" ");
                    utf8_encode(a431)
                }, match v2.clone() {
                    Adt7::C0(h432, _, _) => {
                        h432
                    }
                }), {
                    let a434 = match v2 {
                        Adt7::C0(_, h433, _) => {
                            h433
                        }
                    };
                    let a435 = 1u64;
                    nat_add(a434, a435)?
                }, v4))
            } else {
                f180(v2, v5, v4)
            }
        }
        Adt6::C5 => {
            if v6 {
                Ok(Adt7::C0(match v2.clone() {
                    Adt7::C0(h436, _, _) => {
                        h436
                    }
                }, match v2 {
                    Adt7::C0(_, h437, _) => {
                        h437
                    }
                }, v4))
            } else {
                f180(v2, v5, v4)
            }
        }
        Adt6::C6(r444) => {
            let v12 = (*r444).clone();
            let v13: bool = f181(v1, v0, match v2.clone() {
                Adt7::C0(_, h438, _) => {
                    h438
                }
            }, v4.clone(), Adt8::C0(v5, true, v12.clone()))?;
            Ok(Adt7::C0(match v2.clone() {
                Adt7::C0(h439, _, _) => {
                    h439
                }
            }, match v2 {
                Adt7::C0(_, h440, _) => {
                    h440
                }
            }, List::cons(Adt8::C0(v5, v13, v12), v4)))
        }
    }
}

pub fn f129(v0: Fn15, p449: u64, p450: List<Bytes>) -> (List<Bytes>, bool) {
    let m448 = (p449, p450);
    {
        let mut m448 = m448;
        loop {
            let step = {
                let (v1, v2) = m448;
                let m445 = v1;
                if m445 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m445 - 1;
                    let m447 = {
                        let c446: Fn15 = v0.clone();
                        c446.apply(v2.clone())
                    };
                    match m447 {
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
                Ok(next) => { m448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Bytes>) -> Option<List<Bytes>> {
    f182(v0)
}

pub fn f131(v0: Fn16, p454: List<Bytes>, p455: List<Bytes>) -> List<Bytes> {
    let m453 = (p454, p455);
    {
        let mut m453 = m453;
        loop {
            let step = {
                let (v1, v2) = m453;
                let m451 = v2;
                match m451.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c452: Fn16 = v0.clone();
                            c452.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f133(v0: Adt0, v1: List<Bytes>) -> R<bool> {
    Ok(f5({
        let a457 = f6(v0.clone())?;
        let a458 = {
            let a456 = v1.clone();
            length_list(a456)
        };
        nat_eq(a457, a458)
    }, f183(v0, v1)))
}

pub fn f134(v0: Str) -> List<Bytes> {
    List::cons(f4(Str::lit("kind")), List::cons(f4(v0), List::<Bytes>::nil()))
}

pub fn f135(v0: Bytes) -> R<bool> {
    Ok(f5(f103(v0.clone())?, f5(!f184(v0.clone(), f185()), !f45(v0, f4(Str::lit("false"))))))
}

pub fn f136(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a459 = v0.clone();
        length_bytes(a459)
    };
    Ok(f5({
        let a460 = 0u64;
        let a461 = v1;
        nat_lt(a460, a461)
    }, f5(f186(v0.clone(), 0u64, false)?, {
        let a462 = v1;
        let a463 = 1u64;
        if nat_eq(a462, a463) {
            true
        } else {
            !f187(v0, f4(Str::lit("0")))
        }
    })))
}

pub fn f137(v0: Bytes) -> R<bool> {
    Ok(f5({
        let a468 = {
            let a465 = {
                let a464 = v0.clone();
                length_bytes(a464)
            };
            let a466 = 2u64;
            let a467 = 0u64;
            nat_rem(a465, a466, a467)
        };
        let a469 = 0u64;
        nat_eq(a468, a469)
    }, f186(v0, 0u64, true)?))
}

pub fn f138(v0: Str, v1: Str) -> List<Bytes> {
    List::cons(f4(Str::lit("kind")), List::cons(f4(v0), List::cons(f4(v1), List::<Bytes>::nil())))
}

pub fn f139(v0: Bytes) -> R<bool> {
    if f187(v0.clone(), f4(Str::lit("-"))) {
        let v1: Bytes = f85(v0.clone(), 1u64, {
            let a471 = {
                let a470 = v0;
                length_bytes(a470)
            };
            let a472 = 1u64;
            nat_sub(a471, a472)
        });
        Ok(f5(f136(v1.clone())?, !f45(v1, f4(Str::lit("0")))))
    } else {
        f136(v0)
    }
}

pub fn f140(v0: Str, v1: Str, v2: Str) -> List<Bytes> {
    List::cons(f4(Str::lit("kind")), List::cons(f4(v0), List::cons(f4(v1), List::cons(f4(v2), List::<Bytes>::nil()))))
}

pub fn f141(v0: Option<Adt0>) -> R<bool> {
    let m473 = v0;
    match m473 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f61(v1)
        }
    }
}

pub fn f142(v0: Adt0, v1: Bytes, v2: Fn17) -> R<bool> {
    let m474 = f2(f3(v0, v1));
    match m474 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            f166(Fn21::F188(v2), true, f11(v3))
        }
    }
}

pub fn f143(v0: Adt0) -> R<bool> {
    f61(v0)
}

pub fn f144(v0: Adt0) -> R<bool> {
    f189(v0)
}

pub fn f145(v0: Adt0, v1: Bytes) -> bool {
    f102(f2(f3(v0, v1)))
}

pub fn f146(v0: Adt0, v1: List<Bytes>) -> R<bool> {
    Ok(f5({
        let a478 = f6(v0.clone())?;
        let a479 = {
            let a476 = {
                let a475 = v1.clone();
                length_list(a475)
            };
            let a477 = if f102(f3(v0.clone(), f4(Str::lit("type_arguments")))) {
                1u64
            } else {
                0u64
            };
            nat_add(a476, a477)?
        };
        nat_eq(a478, a479)
    }, f183(v0, v1)))
}

pub fn f147(v0: Adt0) -> R<bool> {
    let m480 = f3(v0, f4(Str::lit("type_arguments")));
    match m480 {
        None => {
            Ok(true)
        }
        Some(v1) => {
            let m481 = f13(v1);
            match m481 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    let a482 = 0u64;
                    let a483 = f6(v2)?;
                    Ok(nat_lt(a482, a483))
                }
            }
        }
    }
}

pub fn f148(v0: Option<Adt0>) -> R<bool> {
    let m484 = v0;
    match m484 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            let m485 = f59(v1)?;
            match m485 {
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

pub fn f149(v0: Adt0) -> R<bool> {
    f190(v0)
}

pub fn f150(v0: Adt0) -> R<bool> {
    f189(v0)
}

pub fn f151(v0: Adt0) -> R<bool> {
    f191(v0)
}

pub fn f152(v0: Fn18, p490: u64, p491: Adt13) -> R<(Adt13, bool)> {
    let m489 = (p490, p491);
    {
        let mut m489 = m489;
        loop {
            let step = {
                let (v1, v2) = m489;
                let m486 = v1;
                if m486 == 0 {
                    Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m486 - 1;
                    let m488 = {
                        let c487: Fn18 = v0.clone();
                        c487.apply(v2.clone())?
                    };
                    match m488 {
                        None => {
                            Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt13), R<(Adt13, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m489 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: u64, v1: Bytes, v2: Adt13) -> R<Option<Adt13>> {
    f192(v1, v0, v2)
}

pub fn f154(v0: Bytes, v1: Bytes) -> Bytes {
    let a492 = v0;
    let a493 = v1;
    append_bytes(a492, a493)
}

pub fn f155(v0: Str) -> Bytes {
    let a494 = v0;
    utf8_encode(a494)
}

pub fn f156(v0: List<Bytes>) -> Bytes {
    f87(f88(v0))
}

pub fn f157(v0: Fn19, p498: List<Adt6>, p499: List<Adt0>) -> List<Adt6> {
    let m497 = (p498, p499);
    {
        let mut m497 = m497;
        loop {
            let step = {
                let (v1, v2) = m497;
                let m495 = v2;
                match m495.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt0>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt0>), List<Adt6>>(({
                            let c496: Fn19 = v0.clone();
                            c496.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: List<Adt6>, v1: Adt0) -> List<Adt6> {
    let a501 = v0;
    let a502 = List::cons(f60({
        let m500 = f18(v1);
        match m500 {
            None => {
                f4(Str::lit(""))
            }
            Some(v2) => {
                v2
            }
        }
    }), List::<Adt6>::nil());
    append_list(a501, a502)
}

pub fn f159(v0: Bytes) -> bool {
    f184(v0, List::cons(f4(Str::lit("Bool")), List::cons(f4(Str::lit("Bytes")), List::cons(f4(Str::lit("Int")), List::cons(f4(Str::lit("Int8")), List::cons(f4(Str::lit("Int16")), List::cons(f4(Str::lit("Int32")), List::cons(f4(Str::lit("Int64")), List::cons(f4(Str::lit("Nat")), List::cons(f4(Str::lit("Ordering")), List::cons(f4(Str::lit("Prop")), List::cons(f4(Str::lit("String")), List::cons(f4(Str::lit("Type")), List::cons(f4(Str::lit("UInt8")), List::cons(f4(Str::lit("UInt16")), List::cons(f4(Str::lit("UInt32")), List::cons(f4(Str::lit("UInt64")), List::cons(f4(Str::lit("Unit")), List::cons(f4(Str::lit("ContractViolation")), List::cons(f4(Str::lit("ReasoningFailure")), List::cons(f4(Str::lit("List")), List::cons(f4(Str::lit("Option")), List::cons(f4(Str::lit("Result")), List::cons(f4(Str::lit("Set")), List::cons(f4(Str::lit("Map")), List::cons(f4(Str::lit("Fn")), List::<Bytes>::nil()))))))))))))))))))))))))))
}

pub fn f160(v0: u64) -> bool {
    let a503 = 64u64;
    let a504 = v0;
    if if nat_lt(a503, a504) {
        let a505 = v0;
        let a506 = 91u64;
        nat_lt(a505, a506)
    } else {
        false
    } {
        true
    } else {
        let a507 = 96u64;
        let a508 = v0;
        if if nat_lt(a507, a508) {
            let a509 = v0;
            let a510 = 123u64;
            nat_lt(a509, a510)
        } else {
            false
        } {
            true
        } else {
            let a511 = v0;
            let a512 = 95u64;
            nat_eq(a511, a512)
        }
    }
}

pub fn f161(v0: Bytes, v1: u64) -> R<u64> {
    let m513 = f178(v0, v1);
    match m513 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a558 = {
                let a534 = {
                    let a522 = {
                        let a516 = {
                            let a514 = v3;
                            let a515 = 128u8;
                            fixed_u8::bit_and(a514, a515)
                        };
                        let a517 = 128u8;
                        if equal(a516, a517) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a523 = {
                        let a520 = {
                            let a518 = v3;
                            let a519 = 64u8;
                            fixed_u8::bit_and(a518, a519)
                        };
                        let a521 = 64u8;
                        if equal(a520, a521) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a522, a523)?
                };
                let a535 = {
                    let a532 = {
                        let a526 = {
                            let a524 = v3;
                            let a525 = 32u8;
                            fixed_u8::bit_and(a524, a525)
                        };
                        let a527 = 32u8;
                        if equal(a526, a527) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a533 = {
                        let a530 = {
                            let a528 = v3;
                            let a529 = 16u8;
                            fixed_u8::bit_and(a528, a529)
                        };
                        let a531 = 16u8;
                        if equal(a530, a531) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a532, a533)?
                };
                nat_add(a534, a535)?
            };
            let a559 = {
                let a556 = {
                    let a544 = {
                        let a538 = {
                            let a536 = v3;
                            let a537 = 8u8;
                            fixed_u8::bit_and(a536, a537)
                        };
                        let a539 = 8u8;
                        if equal(a538, a539) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a545 = {
                        let a542 = {
                            let a540 = v3;
                            let a541 = 4u8;
                            fixed_u8::bit_and(a540, a541)
                        };
                        let a543 = 4u8;
                        if equal(a542, a543) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a544, a545)?
                };
                let a557 = {
                    let a554 = {
                        let a548 = {
                            let a546 = v3;
                            let a547 = 2u8;
                            fixed_u8::bit_and(a546, a547)
                        };
                        let a549 = 2u8;
                        if equal(a548, a549) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a555 = {
                        let a552 = {
                            let a550 = v3;
                            let a551 = 1u8;
                            fixed_u8::bit_and(a550, a551)
                        };
                        let a553 = 1u8;
                        if equal(a552, a553) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a554, a555)?
                };
                nat_add(a556, a557)?
            };
            nat_add(a558, a559)
        }
    }
}

pub fn f162(v0: Bytes, v1: u64) -> R<u64> {
    let (h560, _) = f193(Fn24::F194(v0.clone()), {
        let a562 = {
            let a561 = v0;
            length_bytes(a561)
        };
        let a563 = 1u64;
        nat_add(a562, a563)?
    }, v1)?;
    Ok(h560)
}

pub fn f163(v0: Bytes) -> bool {
    let a564 = v0.clone();
    let a565 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a564, a565) {
        true
    } else {
        let a566 = v0;
        let a567 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a566, a567)
    }
}

pub fn f164(v0: Fn20, p571: Option<Adt0>, p572: List<Adt0>) -> R<Option<Adt0>> {
    let m570 = (p571, p572);
    {
        let mut m570 = m570;
        loop {
            let step = {
                let (v1, v2) = m570;
                let m568 = v2;
                match m568.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(({
                            let c569: Fn20 = v0.clone();
                            c569.apply(v1, v3)?
                        }, v4))
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

pub fn f165(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt0) -> R<Option<Adt0>> {
    let m573 = v2.clone();
    match m573 {
        None => {
            let v5: Adt0 = f30(v1, v3)?;
            if f195(v5.clone(), v0)? {
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

pub fn f166(v0: Fn21, p577: bool, p578: List<Adt0>) -> R<bool> {
    let m576 = (p577, p578);
    {
        let mut m576 = m576;
        loop {
            let step = {
                let (v1, v2) = m576;
                let m574 = v2;
                match m574.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), R<bool>>(({
                            let c575: Fn21 = v0.clone();
                            c575.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Adt0, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let m579 = f19(f30(v0, v2)?, f4(Str::lit("properties")));
        match m579 {
            None => {
                Ok(false)
            }
            Some(v3) => {
                Ok(f102(f19(v3, f4(Str::lit("kind")))))
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f168(v0: (bool, Adt6), v1: Adt6) -> (bool, Adt6) {
    let (h580, _) = v0;
    if h580 {
        (false, Adt6::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt6::C0)))
    } else {
        (false, Adt6::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C1(f77(Str::lit(",")))), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C4), std::rc::Rc::new({
            let (_, h581) = v0;
            h581
        })))))))
    }
}

pub fn f169(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a582 = v2;
    let a583 = 0u64;
    if nat_eq(a582, a583) {
        f196(v0, 0u64)
    } else {
        let (_, h584) = {
            let (h585, _) = f197(Fn25::F198(v1, v0), v2, (v2, Bytes::lit(&[])));
            h585
        };
        h584
    }
}

pub fn f170() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f171(v0: Bytes) -> Bytes {
    f117(f199(f200(v0.clone())), v0)
}

pub fn f172(v0: Bytes, v1: Adt6) -> R<Adt6> {
    Ok(Adt6::C2(std::rc::Rc::new(Adt6::C1(f117(f76(v0)?, f77(Str::lit(": "))))), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt6::C0)))))
}

pub fn f173(v0: List<List<(Bytes, Adt6)>>) -> List<List<(Bytes, Adt6)>> {
    f201(Fn26::F202, List::<List<(Bytes, Adt6)>>::nil(), v0)
}

pub fn f174(v0: Fn22, p589: List<List<(Bytes, Adt6)>>, p590: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
    let m588 = (p589, p590);
    {
        let mut m588 = m588;
        loop {
            let step = {
                let (v1, v2) = m588;
                let m586 = v2;
                match m586.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt6)>>, List<(Bytes, Adt6)>), List<List<(Bytes, Adt6)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt6)>>, List<(Bytes, Adt6)>), List<List<(Bytes, Adt6)>>>(({
                            let c587: Fn22 = v0.clone();
                            c587.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m588 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: List<List<(Bytes, Adt6)>>, v1: (Bytes, Adt6)) -> List<List<(Bytes, Adt6)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt6)>::nil()), v0)
}

pub fn f176(v0: Fn23, p595: u64, p596: List<List<(Bytes, Adt6)>>) -> R<(List<List<(Bytes, Adt6)>>, bool)> {
    let m594 = (p595, p596);
    {
        let mut m594 = m594;
        loop {
            let step = {
                let (v1, v2) = m594;
                let m591 = v1;
                if m591 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m591 - 1;
                    let m593 = {
                        let c592: Fn23 = v0.clone();
                        c592.apply(v2.clone())?
                    };
                    match m593 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
    f203(v0)
}

pub fn f178(v0: Bytes, v1: u64) -> Option<u8> {
    let a597 = v0;
    let a598 = v1;
    index_bytes(a597, a598)
}

pub fn f179(v0: Bytes) -> R<u64> {
    let (_, h599) = {
        let (h600, _) = f204(Fn27::F205(v0.clone()), {
            let a601 = v0;
            length_bytes(a601)
        }, (0u64, 0u64))?;
        h600
    };
    Ok(h599)
}

pub fn f180(v0: Adt7, v1: u64, v2: List<Adt8>) -> R<Adt7> {
    Ok(Adt7::C0(f206(match v0 {
        Adt7::C0(h602, _, _) => {
            h602
        }
    }, v1)?, v1, v2))
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: List<Adt8>, v4: Adt8) -> R<bool> {
    let v5: Adt14 = {
        let (h603, _) = f207(Fn28::F208, {
            let a604 = v0;
            let a605 = 1u64;
            nat_add(a604, a605)?
        }, Adt14::C0(0u64, {
            let a606 = v1;
            let a607 = v2;
            nat_sub(a606, a607)
        }, {
            let a608 = v1;
            let a609 = v2;
            nat_lt(a608, a609)
        }, List::cons(v4, List::<Adt8>::nil()), v3, None::<bool>))?;
        h603
    };
    let m611 = match v5 {
        Adt14::C0(_, _, _, _, _, h610) => {
            h610
        }
    };
    match m611 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f182(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m612 = v0.clone();
    match m612.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m613 = v2;
            match m613.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f209(Fn29::F210, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m615 = {
                        let (h614, _) = v5;
                        h614
                    };
                    match m615 {
                        None => {
                            Some(f88({
                                let (_, h617) = v5;
                                h617
                            }))
                        }
                        Some(v6) => {
                            Some(f88(List::cons(v6, {
                                let (_, h616) = v5;
                                h616
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f183(v0: Adt0, v1: List<Bytes>) -> bool {
    f211(Fn30::F212(v0), true, v1)
}

pub fn f184(v0: Bytes, v1: List<Bytes>) -> bool {
    f211(Fn30::F213(v0), false, v1)
}

pub fn f185() -> List<Bytes> {
    List::cons(f4(Str::lit("let")), List::cons(f4(Str::lit("if")), List::cons(f4(Str::lit("match")), List::cons(f4(Str::lit("fun")), List::cons(f4(Str::lit("forall")), List::cons(f4(Str::lit("nil")), List::cons(f4(Str::lit("cons")), List::cons(f4(Str::lit("first")), List::cons(f4(Str::lit("second")), List::cons(f4(Str::lit("not")), List::cons(f4(Str::lit("apply")), List::cons(f4(Str::lit("ctor")), List::cons(f4(Str::lit("record")), List::cons(f4(Str::lit("project")), List::cons(f4(Str::lit("prim")), List::cons(f4(Str::lit("ref")), List::cons(f4(Str::lit("checked")), List::cons(f4(Str::lit("then")), List::cons(f4(Str::lit("else")), List::cons(f4(Str::lit("with")), List::cons(f4(Str::lit("true")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f186(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = {
        let (h618, _) = f193(Fn24::F214(v2, v0.clone()), {
            let a620 = {
                let a619 = v0.clone();
                length_bytes(a619)
            };
            let a621 = 1u64;
            nat_add(a620, a621)?
        }, v1)?;
        h618
    };
    let a623 = v3;
    let a624 = {
        let a622 = v0;
        length_bytes(a622)
    };
    Ok(nat_eq(a623, a624))
}

pub fn f187(v0: Bytes, v1: Bytes) -> bool {
    let m629 = {
        let a626 = v0;
        let a627 = 0u64;
        let a628 = {
            let a625 = v1.clone();
            length_bytes(a625)
        };
        slice_bytes(a626, a627, a628)
    };
    match m629 {
        None => {
            false
        }
        Some(v2) => {
            let a630 = v2;
            let a631 = v1;
            equal(a630, a631)
        }
    }
}

pub fn f188(v0: Fn17, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let c632: Fn17 = v0;
        c632.apply(v2)
    } else {
        Ok(false)
    }
}

pub fn f189(v0: Adt0) -> R<bool> {
    let m633 = f18(v0);
    match m633 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f103(v1)
        }
    }
}

pub fn f190(v0: Adt0) -> R<bool> {
    let m634 = f1(v0);
    match m634 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f5(f133(v1.clone(), List::cons(f4(Str::lit("field")), List::cons(f4(Str::lit("value")), List::<Bytes>::nil())))?, {
                let m635 = f7(f3(v1, f4(Str::lit("field"))));
                match m635 {
                    None => {
                        false
                    }
                    Some(v2) => {
                        f103(v2)?
                    }
                }
            }))
        }
    }
}

pub fn f191(v0: Adt0) -> R<bool> {
    let m636 = f1(v0);
    match m636 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f5(f133(v1.clone(), List::cons(f4(Str::lit("constructor")), List::cons(f4(Str::lit("binders")), List::cons(f4(Str::lit("body")), List::<Bytes>::nil()))))?, f5(f148(f3(v1.clone(), f4(Str::lit("constructor"))))?, f142(v1, f4(Str::lit("binders")), Fn17::F215)?)))
        }
    }
}

pub fn f192(v0: Bytes, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(h637, _) => {
            h637
        }
    };
    let a638 = v3;
    let a639 = v1;
    if nat_lt(a638, a639) {
        let v4: u64 = f216(v0.clone(), v3)?;
        let a640 = v4;
        let a641 = 32u64;
        if if nat_lt(a640, a641) {
            true
        } else {
            let a642 = v4;
            let a643 = 34u64;
            if nat_eq(a642, a643) {
                true
            } else {
                let a644 = v4;
                let a645 = 92u64;
                nat_eq(a644, a645)
            }
        } {
            Ok(Some(Adt13::C0({
                let a646 = v3;
                let a647 = 1u64;
                nat_add(a646, a647)?
            }, List::cons(f217(v4), match v2 {
                Adt13::C0(_, h648) => {
                    h648
                }
            }))))
        } else {
            let v5: u64 = f218(v0.clone(), v1, v3)?;
            Ok(Some(Adt13::C0(v5, List::cons(f85(v0, v3, {
                let a649 = v5;
                let a650 = v3;
                nat_sub(a649, a650)
            }), match v2 {
                Adt13::C0(_, h651) => {
                    h651
                }
            }))))
        }
    } else {
        Ok(None::<Adt13>)
    }
}

pub fn f193(v0: Fn24, p656: u64, p657: u64) -> R<(u64, bool)> {
    let m655 = (p656, p657);
    {
        let mut m655 = m655;
        loop {
            let step = {
                let (v1, v2) = m655;
                let m652 = v1;
                if m652 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m652 - 1;
                    let m654 = {
                        let c653: Fn24 = v0.clone();
                        c653.apply(v2)?
                    };
                    match m654 {
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
                Ok(next) => { m655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f219(v0, v1)
}

pub fn f195(v0: Adt0, v1: Bytes) -> R<bool> {
    let m658 = f19(v0, f4(Str::lit("properties")));
    match m658 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m659 = f19(v2, f4(Str::lit("kind")));
            match m659 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m660 = f7(f19(v3.clone(), f4(Str::lit("const"))));
                    if match m660 {
                        None => {
                            false
                        }
                        Some(v4) => {
                            f45(v4, v1.clone())
                        }
                    } {
                        Ok(true)
                    } else {
                        let m661 = f2(f19(v3, f4(Str::lit("enum"))));
                        match m661 {
                            None => {
                                Ok(false)
                            }
                            Some(v5) => {
                                f166(Fn21::F220(v1), false, f11(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f196(v0: Bytes, v1: u64) -> Bytes {
    let m665 = {
        let a662 = v0;
        let a663 = v1;
        let a664 = 1u64;
        slice_bytes(a662, a663, a664)
    };
    match m665 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f197(v0: Fn25, p670: u64, p671: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m669 = (p670, p671);
    {
        let mut m669 = m669;
        loop {
            let step = {
                let (v1, v2) = m669;
                let m666 = v1;
                if m666 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m666 - 1;
                    let m668 = {
                        let c667: Fn25 = v0.clone();
                        c667.apply(v2.clone())
                    };
                    match m668 {
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
                Ok(next) => { m669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f221(v1, v0, v2)
}

pub fn f199(v0: u64) -> Bytes {
    f85(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f200(v0: Bytes) -> u64 {
    let a672 = v0.clone();
    let a673 = Bytes::lit(&[0x6e, 0x61, 0x6d, 0x65]);
    if equal(a672, a673) {
        0u64
    } else {
        let a674 = v0.clone();
        let a675 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
        if equal(a674, a675) {
            1u64
        } else {
            let a676 = v0.clone();
            let a677 = Bytes::lit(&[0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
            if equal(a676, a677) {
                2u64
            } else {
                let a678 = v0.clone();
                let a679 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65]);
                if equal(a678, a679) {
                    3u64
                } else {
                    let a680 = v0.clone();
                    let a681 = Bytes::lit(&[0x72, 0x65, 0x73, 0x75, 0x6c, 0x74]);
                    if equal(a680, a681) {
                        4u64
                    } else {
                        let a682 = v0.clone();
                        let a683 = Bytes::lit(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74]);
                        if equal(a682, a683) {
                            5u64
                        } else {
                            let a684 = v0.clone();
                            let a685 = Bytes::lit(&[0x66, 0x69, 0x65, 0x6c, 0x64, 0x73]);
                            if equal(a684, a685) {
                                6u64
                            } else {
                                let a686 = v0.clone();
                                let a687 = Bytes::lit(&[0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x73]);
                                if equal(a686, a687) {
                                    7u64
                                } else {
                                    let a688 = v0.clone();
                                    let a689 = Bytes::lit(&[0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73]);
                                    if equal(a688, a689) {
                                        8u64
                                    } else {
                                        let a690 = v0.clone();
                                        let a691 = Bytes::lit(&[0x62, 0x6f, 0x64, 0x79]);
                                        if equal(a690, a691) {
                                            9u64
                                        } else {
                                            let a692 = v0.clone();
                                            let a693 = Bytes::lit(&[0x70, 0x72, 0x6f, 0x6f, 0x66]);
                                            if equal(a692, a693) {
                                                10u64
                                            } else {
                                                let a694 = v0;
                                                let a695 = Bytes::lit(&[0x61, 0x78, 0x69, 0x6f, 0x6d, 0x73]);
                                                if equal(a694, a695) {
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

pub fn f201(v0: Fn26, p699: List<List<(Bytes, Adt6)>>, p700: List<List<(Bytes, Adt6)>>) -> List<List<(Bytes, Adt6)>> {
    let m698 = (p699, p700);
    {
        let mut m698 = m698;
        loop {
            let step = {
                let (v1, v2) = m698;
                let m696 = v2;
                match m696.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), List<List<(Bytes, Adt6)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), List<List<(Bytes, Adt6)>>>(({
                            let c697: Fn26 = v0.clone();
                            c697.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: List<List<(Bytes, Adt6)>>, v1: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
    List::cons(v1, v0)
}

pub fn f203(v0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
    let a702 = {
        let a701 = v0.clone();
        length_list(a701)
    };
    let a703 = 1u64;
    if nat_le(a702, a703) {
        Ok(None::<List<List<(Bytes, Adt6)>>>)
    } else {
        Ok(Some(f222(v0)?))
    }
}

pub fn f204(v0: Fn27, p708: u64, p709: (u64, u64)) -> R<((u64, u64), bool)> {
    let m707 = (p708, p709);
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v1, v2) = m707;
                let m704 = v1;
                if m704 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m704 - 1;
                    let m706 = {
                        let c705: Fn27 = v0.clone();
                        c705.apply(v2)?
                    };
                    match m706 {
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
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f223(v0, v1)
}

pub fn f206(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a710 = v1;
        let a711 = 256u64;
        let a712 = 0u64;
        nat_quot(a710, a711, a712)
    };
    let v3: List<Bytes> = {
        let (_, h713) = {
            let (h714, _) = f224(Fn31::F225(v2), {
                let a715 = v2;
                let a716 = 1u64;
                nat_add(a715, a716)?
            }, (0u64, List::cons({
                let a717 = Str::lit("\u{a}");
                utf8_encode(a717)
            }, v0)))?;
            h714
        };
        h713
    };
    Ok(List::cons(f85(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a718 = v1;
        let a719 = 256u64;
        let a720 = 0u64;
        nat_rem(a718, a719, a720)
    }), v3))
}

pub fn f207(v0: Fn28, p725: u64, p726: Adt14) -> R<(Adt14, bool)> {
    let m724 = (p725, p726);
    {
        let mut m724 = m724;
        loop {
            let step = {
                let (v1, v2) = m724;
                let m721 = v1;
                if m721 == 0 {
                    Err::<(u64, Adt14), R<(Adt14, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m721 - 1;
                    let m723 = {
                        let c722: Fn28 = v0.clone();
                        c722.apply(v2.clone())?
                    };
                    match m723 {
                        None => {
                            Err::<(u64, Adt14), R<(Adt14, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt14), R<(Adt14, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Adt14) -> R<Option<Adt14>> {
    f226(v0)
}

pub fn f209(v0: Fn29, p730: (Option<Bytes>, List<Bytes>), p731: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m729 = (p730, p731);
    {
        let mut m729 = m729;
        loop {
            let step = {
                let (v1, v2) = m729;
                let m727 = v2;
                match m727.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c728: Fn29 = v0.clone();
                            c728.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f227(v0, v1)
}

pub fn f211(v0: Fn30, p735: bool, p736: List<Bytes>) -> bool {
    let m734 = (p735, p736);
    {
        let mut m734 = m734;
        loop {
            let step = {
                let (v1, v2) = m734;
                let m732 = v2;
                match m732.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c733: Fn30 = v0.clone();
                            c733.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Adt0, v1: bool, v2: Bytes) -> bool {
    if v1 {
        f102(f3(v0, v2))
    } else {
        false
    }
}

pub fn f213(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a737 = v2;
        let a738 = v0;
        equal(a737, a738)
    }
}

pub fn f214(v0: bool, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let m739 = f178(v1, v2);
    match m739 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let v5: u64 = {
                let v4: u8 = v3;
                let a784 = {
                    let a760 = {
                        let a748 = {
                            let a742 = {
                                let a740 = v4;
                                let a741 = 128u8;
                                fixed_u8::bit_and(a740, a741)
                            };
                            let a743 = 128u8;
                            if equal(a742, a743) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a749 = {
                            let a746 = {
                                let a744 = v4;
                                let a745 = 64u8;
                                fixed_u8::bit_and(a744, a745)
                            };
                            let a747 = 64u8;
                            if equal(a746, a747) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a748, a749)?
                    };
                    let a761 = {
                        let a758 = {
                            let a752 = {
                                let a750 = v4;
                                let a751 = 32u8;
                                fixed_u8::bit_and(a750, a751)
                            };
                            let a753 = 32u8;
                            if equal(a752, a753) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a759 = {
                            let a756 = {
                                let a754 = v4;
                                let a755 = 16u8;
                                fixed_u8::bit_and(a754, a755)
                            };
                            let a757 = 16u8;
                            if equal(a756, a757) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a758, a759)?
                    };
                    nat_add(a760, a761)?
                };
                let a785 = {
                    let a782 = {
                        let a770 = {
                            let a764 = {
                                let a762 = v4;
                                let a763 = 8u8;
                                fixed_u8::bit_and(a762, a763)
                            };
                            let a765 = 8u8;
                            if equal(a764, a765) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a771 = {
                            let a768 = {
                                let a766 = v4;
                                let a767 = 4u8;
                                fixed_u8::bit_and(a766, a767)
                            };
                            let a769 = 4u8;
                            if equal(a768, a769) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a770, a771)?
                    };
                    let a783 = {
                        let a780 = {
                            let a774 = {
                                let a772 = v4;
                                let a773 = 2u8;
                                fixed_u8::bit_and(a772, a773)
                            };
                            let a775 = 2u8;
                            if equal(a774, a775) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a781 = {
                            let a778 = {
                                let a776 = v4;
                                let a777 = 1u8;
                                fixed_u8::bit_and(a776, a777)
                            };
                            let a779 = 1u8;
                            if equal(a778, a779) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a780, a781)?
                    };
                    nat_add(a782, a783)?
                };
                nat_add(a784, a785)?
            };
            if if f5({
                let a786 = 48u64;
                let a787 = v5;
                nat_le(a786, a787)
            }, {
                let a788 = v5;
                let a789 = 57u64;
                nat_le(a788, a789)
            }) {
                true
            } else {
                f5(v0, f5({
                    let a790 = 97u64;
                    let a791 = v5;
                    nat_le(a790, a791)
                }, {
                    let a792 = v5;
                    let a793 = 102u64;
                    nat_le(a792, a793)
                }))
            } {
                Ok(Some({
                    let a794 = v2;
                    let a795 = 1u64;
                    nat_add(a794, a795)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f215(v0: Adt0) -> R<bool> {
    let m796 = f18(v0);
    match m796 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f135(v1)
        }
    }
}

pub fn f216(v0: Bytes, v1: u64) -> R<u64> {
    let m797 = f178(v0, v1);
    match m797 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a842 = {
                let a818 = {
                    let a806 = {
                        let a800 = {
                            let a798 = v3;
                            let a799 = 128u8;
                            fixed_u8::bit_and(a798, a799)
                        };
                        let a801 = 128u8;
                        if equal(a800, a801) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a807 = {
                        let a804 = {
                            let a802 = v3;
                            let a803 = 64u8;
                            fixed_u8::bit_and(a802, a803)
                        };
                        let a805 = 64u8;
                        if equal(a804, a805) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a806, a807)?
                };
                let a819 = {
                    let a816 = {
                        let a810 = {
                            let a808 = v3;
                            let a809 = 32u8;
                            fixed_u8::bit_and(a808, a809)
                        };
                        let a811 = 32u8;
                        if equal(a810, a811) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a817 = {
                        let a814 = {
                            let a812 = v3;
                            let a813 = 16u8;
                            fixed_u8::bit_and(a812, a813)
                        };
                        let a815 = 16u8;
                        if equal(a814, a815) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a816, a817)?
                };
                nat_add(a818, a819)?
            };
            let a843 = {
                let a840 = {
                    let a828 = {
                        let a822 = {
                            let a820 = v3;
                            let a821 = 8u8;
                            fixed_u8::bit_and(a820, a821)
                        };
                        let a823 = 8u8;
                        if equal(a822, a823) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a829 = {
                        let a826 = {
                            let a824 = v3;
                            let a825 = 4u8;
                            fixed_u8::bit_and(a824, a825)
                        };
                        let a827 = 4u8;
                        if equal(a826, a827) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a828, a829)?
                };
                let a841 = {
                    let a838 = {
                        let a832 = {
                            let a830 = v3;
                            let a831 = 2u8;
                            fixed_u8::bit_and(a830, a831)
                        };
                        let a833 = 2u8;
                        if equal(a832, a833) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a839 = {
                        let a836 = {
                            let a834 = v3;
                            let a835 = 1u8;
                            fixed_u8::bit_and(a834, a835)
                        };
                        let a837 = 1u8;
                        if equal(a836, a837) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a838, a839)?
                };
                nat_add(a840, a841)?
            };
            nat_add(a842, a843)
        }
    }
}

pub fn f217(v0: u64) -> Bytes {
    let a844 = v0;
    let a845 = 34u64;
    if nat_eq(a844, a845) {
        f155(Str::lit("\\\""))
    } else {
        let a846 = v0;
        let a847 = 92u64;
        if nat_eq(a846, a847) {
            f155(Str::lit("\\\\"))
        } else {
            let a848 = v0;
            let a849 = 8u64;
            if nat_eq(a848, a849) {
                f155(Str::lit("\\b"))
            } else {
                let a850 = v0;
                let a851 = 12u64;
                if nat_eq(a850, a851) {
                    f155(Str::lit("\\f"))
                } else {
                    let a852 = v0;
                    let a853 = 10u64;
                    if nat_eq(a852, a853) {
                        f155(Str::lit("\\n"))
                    } else {
                        let a854 = v0;
                        let a855 = 13u64;
                        if nat_eq(a854, a855) {
                            f155(Str::lit("\\r"))
                        } else {
                            let a856 = v0;
                            let a857 = 9u64;
                            if nat_eq(a856, a857) {
                                f155(Str::lit("\\t"))
                            } else {
                                f154(f155(Str::lit("\\u00")), f228(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f218(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h858, _) = f193(Fn24::F229(v1, v0), {
        let a859 = v1;
        let a860 = v2;
        nat_sub(a859, a860)
    }, v2)?;
    Ok(h858)
}

pub fn f219(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f230(f161(v0, v1)?) {
        Ok(Some({
            let a861 = v1;
            let a862 = 1u64;
            nat_add(a861, a862)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f220(v0: Bytes, v1: bool, v2: Adt0) -> bool {
    if v1 {
        true
    } else {
        let m863 = f18(v2);
        match m863 {
            None => {
                false
            }
            Some(v3) => {
                f45(v3, v0)
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h864, _) = v2;
        h864
    };
    let a865 = v3;
    let a866 = 0u64;
    if nat_eq(a865, a866) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a867 = v3;
            let a868 = v1;
            let a869 = 0u64;
            nat_quot(a867, a868, a869)
        }, {
            let a874 = f196(v0, {
                let a870 = v3;
                let a871 = v1;
                let a872 = 0u64;
                nat_rem(a870, a871, a872)
            });
            let a875 = {
                let (_, h873) = v2;
                h873
            };
            append_bytes(a874, a875)
        }))
    }
}

pub fn f222(v0: List<List<(Bytes, Adt6)>>) -> R<List<List<(Bytes, Adt6)>>> {
    Ok(f173({
        let (_, h876) = {
            let (h877, _) = f231(Fn32::F232, {
                let a879 = {
                    let a878 = v0.clone();
                    length_list(a878)
                };
                let a880 = 1u64;
                nat_add(a879, a880)?
            }, (v0, List::<List<(Bytes, Adt6)>>::nil()))?;
            h877
        };
        h876
    }))
}

pub fn f223(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m882 = f178(v0, {
        let (h881, _) = v1;
        h881
    });
    match m882 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a927 = {
                    let a903 = {
                        let a891 = {
                            let a885 = {
                                let a883 = v3;
                                let a884 = 128u8;
                                fixed_u8::bit_and(a883, a884)
                            };
                            let a886 = 128u8;
                            if equal(a885, a886) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a892 = {
                            let a889 = {
                                let a887 = v3;
                                let a888 = 64u8;
                                fixed_u8::bit_and(a887, a888)
                            };
                            let a890 = 64u8;
                            if equal(a889, a890) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a891, a892)?
                    };
                    let a904 = {
                        let a901 = {
                            let a895 = {
                                let a893 = v3;
                                let a894 = 32u8;
                                fixed_u8::bit_and(a893, a894)
                            };
                            let a896 = 32u8;
                            if equal(a895, a896) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a902 = {
                            let a899 = {
                                let a897 = v3;
                                let a898 = 16u8;
                                fixed_u8::bit_and(a897, a898)
                            };
                            let a900 = 16u8;
                            if equal(a899, a900) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a901, a902)?
                    };
                    nat_add(a903, a904)?
                };
                let a928 = {
                    let a925 = {
                        let a913 = {
                            let a907 = {
                                let a905 = v3;
                                let a906 = 8u8;
                                fixed_u8::bit_and(a905, a906)
                            };
                            let a908 = 8u8;
                            if equal(a907, a908) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a914 = {
                            let a911 = {
                                let a909 = v3;
                                let a910 = 4u8;
                                fixed_u8::bit_and(a909, a910)
                            };
                            let a912 = 4u8;
                            if equal(a911, a912) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a913, a914)?
                    };
                    let a926 = {
                        let a923 = {
                            let a917 = {
                                let a915 = v3;
                                let a916 = 2u8;
                                fixed_u8::bit_and(a915, a916)
                            };
                            let a918 = 2u8;
                            if equal(a917, a918) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a924 = {
                            let a921 = {
                                let a919 = v3;
                                let a920 = 1u8;
                                fixed_u8::bit_and(a919, a920)
                            };
                            let a922 = 1u8;
                            if equal(a921, a922) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a923, a924)?
                    };
                    nat_add(a925, a926)?
                };
                nat_add(a927, a928)?
            };
            let a929 = v4;
            let a930 = 128u64;
            if nat_lt(a929, a930) {
                Ok(Some(({
                    let a932 = {
                        let (h931, _) = v1;
                        h931
                    };
                    let a933 = 1u64;
                    nat_add(a932, a933)?
                }, {
                    let a935 = {
                        let (_, h934) = v1;
                        h934
                    };
                    let a936 = 1u64;
                    nat_add(a935, a936)?
                })))
            } else {
                let a937 = v4;
                let a938 = 192u64;
                if nat_lt(a937, a938) {
                    Ok(Some(({
                        let a940 = {
                            let (h939, _) = v1;
                            h939
                        };
                        let a941 = 1u64;
                        nat_add(a940, a941)?
                    }, {
                        let (_, h942) = v1;
                        h942
                    })))
                } else {
                    Ok(Some(({
                        let a944 = {
                            let (h943, _) = v1;
                            h943
                        };
                        let a945 = 1u64;
                        nat_add(a944, a945)?
                    }, {
                        let a947 = {
                            let (_, h946) = v1;
                            h946
                        };
                        let a948 = 1u64;
                        nat_add(a947, a948)?
                    })))
                }
            }
        }
    }
}

pub fn f224(v0: Fn31, p953: u64, p954: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m952 = (p953, p954);
    {
        let mut m952 = m952;
        loop {
            let step = {
                let (v1, v2) = m952;
                let m949 = v1;
                if m949 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m949 - 1;
                    let m951 = {
                        let c950: Fn31 = v0.clone();
                        c950.apply(v2.clone())?
                    };
                    match m951 {
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
                Ok(next) => { m952 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f233(v0, v1)
}

pub fn f226(v0: Adt14) -> R<Option<Adt14>> {
    let m956 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, h955) => {
            h955
        }
    };
    match m956 {
        None => {
            if if match v0.clone() {
                Adt14::C0(_, _, h957, _, _, _) => {
                    h957
                }
            } {
                true
            } else {
                let a960 = match v0.clone() {
                    Adt14::C0(_, h958, _, _, _, _) => {
                        h958
                    }
                };
                let a961 = match v0.clone() {
                    Adt14::C0(h959, _, _, _, _, _) => {
                        h959
                    }
                };
                nat_lt(a960, a961)
            } {
                Ok(Some(f234(v0.clone(), match v0.clone() {
                    Adt14::C0(h962, _, _, _, _, _) => {
                        h962
                    }
                }, match v0.clone() {
                    Adt14::C0(_, _, _, h963, _, _) => {
                        h963
                    }
                }, match v0 {
                    Adt14::C0(_, _, _, _, h964, _) => {
                        h964
                    }
                }, Some(false))))
            } else {
                let m966 = match v0.clone() {
                    Adt14::C0(_, _, _, h965, _, _) => {
                        h965
                    }
                };
                match m966.uncons() {
                    None => {
                        let m969 = match v0.clone() {
                            Adt14::C0(_, _, _, _, h968, _) => {
                                h968
                            }
                        };
                        match m969.uncons() {
                            None => {
                                Ok(Some(f234(v0.clone(), match v0 {
                                    Adt14::C0(h970, _, _, _, _, _) => {
                                        h970
                                    }
                                }, List::<Adt8>::nil(), List::<Adt8>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f235(v0, v4, List::<Adt8>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f235(v0.clone(), v2, v3, match v0 {
                            Adt14::C0(_, _, _, _, h967, _) => {
                                h967
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt14>)
        }
    }
}

pub fn f227(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m972 = {
        let (h971, _) = v0;
        h971
    };
    match m972 {
        None => {
            (Some(v1), {
                let (_, h973) = v0;
                h973
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a974 = v2;
                let a975 = v1;
                append_bytes(a974, a975)
            }, {
                let (_, h976) = v0;
                h976
            }))
        }
    }
}

pub fn f228(v0: u64) -> Bytes {
    f154(f236(v0, 16u64), f237(v0))
}

pub fn f229(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f238(v1, v0, v2)
}

pub fn f230(v0: u64) -> bool {
    if f160(v0) {
        true
    } else {
        let a977 = 47u64;
        let a978 = v0;
        if if nat_lt(a977, a978) {
            let a979 = v0;
            let a980 = 58u64;
            nat_lt(a979, a980)
        } else {
            false
        } {
            true
        } else {
            let a981 = v0;
            let a982 = 46u64;
            if nat_eq(a981, a982) {
                true
            } else {
                let a983 = v0;
                let a984 = 39u64;
                if nat_eq(a983, a984) {
                    true
                } else {
                    let a985 = v0;
                    let a986 = 47u64;
                    if nat_eq(a985, a986) {
                        true
                    } else {
                        let a987 = v0;
                        let a988 = 45u64;
                        nat_eq(a987, a988)
                    }
                }
            }
        }
    }
}

pub fn f231(v0: Fn32, p993: u64, p994: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)> {
    let m992 = (p993, p994);
    {
        let mut m992 = m992;
        loop {
            let step = {
                let (v1, v2) = m992;
                let m989 = v1;
                if m989 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m989 - 1;
                    let m991 = {
                        let c990: Fn32 = v0.clone();
                        c990.apply(v2.clone())?
                    };
                    match m991 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
    f239(v0)
}

pub fn f233(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a996 = {
        let (h995, _) = v1.clone();
        h995
    };
    let a997 = v0;
    if nat_lt(a996, a997) {
        Ok(Some(({
            let a999 = {
                let (h998, _) = v1;
                h998
            };
            let a1000 = 1u64;
            nat_add(a999, a1000)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h1001) = v1;
            h1001
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f234(v0: Adt14, v1: u64, v2: List<Adt8>, v3: List<Adt8>, v4: Option<bool>) -> Adt14 {
    Adt14::C0(v1, match v0.clone() {
        Adt14::C0(_, h1002, _, _, _, _) => {
            h1002
        }
    }, match v0 {
        Adt14::C0(_, _, h1003, _, _, _) => {
            h1003
        }
    }, v2, v3, v4)
}

pub fn f235(v0: Adt14, v1: Adt8, v2: List<Adt8>, v3: List<Adt8>) -> R<Adt14> {
    let v4: u64 = match v1.clone() {
        Adt8::C0(h1004, _, _) => {
            h1004
        }
    };
    let v5: bool = match v1.clone() {
        Adt8::C0(_, h1005, _) => {
            h1005
        }
    };
    let v6: u64 = match v0.clone() {
        Adt14::C0(h1006, _, _, _, _, _) => {
            h1006
        }
    };
    let m1008 = match v1 {
        Adt8::C0(_, _, h1007) => {
            h1007
        }
    };
    match m1008 {
        Adt6::C0 => {
            Ok(f234(v0, v6, v2, v3, None::<bool>))
        }
        Adt6::C1(v7) => {
            Ok(f234(v0, {
                let a1009 = v6;
                let a1010 = f179(v7)?;
                nat_add(a1009, a1010)?
            }, v2, v3, None::<bool>))
        }
        Adt6::C2(r1015, r1016) => {
            let v8 = (*r1015).clone();
            let v9 = (*r1016).clone();
            Ok(f234(v0, v6, List::cons(Adt8::C0(v4, v5, v8), List::cons(Adt8::C0(v4, v5, v9), v2)), v3, None::<bool>))
        }
        Adt6::C3(v10, r1017) => {
            let v11 = (*r1017).clone();
            Ok(f234(v0, v6, List::cons(Adt8::C0({
                let a1011 = v4;
                let a1012 = v10;
                nat_add(a1011, a1012)?
            }, v5, v11), v2), v3, None::<bool>))
        }
        Adt6::C4 => {
            if v5 {
                Ok(f234(v0, {
                    let a1013 = v6;
                    let a1014 = 1u64;
                    nat_add(a1013, a1014)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f234(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt6::C5 => {
            if v5 {
                Ok(f234(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f234(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt6::C6(r1018) => {
            let v12 = (*r1018).clone();
            Ok(f234(v0, v6, List::cons(Adt8::C0(v4, true, v12), v2), v3, None::<bool>))
        }
    }
}

pub fn f236(v0: u64, v1: u64) -> Bytes {
    f237({
        let a1019 = v0;
        let a1020 = v1;
        let a1021 = 0u64;
        nat_quot(a1019, a1020, a1021)
    })
}

pub fn f237(v0: u64) -> Bytes {
    f85(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1022 = v0;
        let a1023 = 16u64;
        let a1024 = 0u64;
        nat_rem(a1022, a1023, a1024)
    }, 1u64)
}

pub fn f238(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1025 = v2;
    let a1026 = v1;
    if nat_lt(a1025, a1026) {
        let v3: u64 = f216(v0, v2)?;
        let a1027 = v3;
        let a1028 = 32u64;
        if nat_lt(a1027, a1028) {
            Ok(None::<u64>)
        } else {
            let a1029 = v3;
            let a1030 = 34u64;
            if nat_eq(a1029, a1030) {
                Ok(None::<u64>)
            } else {
                let a1031 = v3;
                let a1032 = 92u64;
                if nat_eq(a1031, a1032) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1033 = v2;
                        let a1034 = 1u64;
                        nat_add(a1033, a1034)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f239(v0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
    let v1: List<List<(Bytes, Adt6)>> = {
        let (h1035, _) = v0;
        h1035
    };
    let v2: List<List<(Bytes, Adt6)>> = {
        let (_, h1036) = v0;
        h1036
    };
    let m1037 = v1;
    match m1037.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>)
        }
        Some((v3, v4)) => {
            let m1038 = v4;
            match m1038.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt6)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f240(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f240(v0: List<(Bytes, Adt6)>, v1: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    let v2: u64 = {
        let a1043 = {
            let a1041 = {
                let a1039 = v0.clone();
                length_list(a1039)
            };
            let a1042 = {
                let a1040 = v1.clone();
                length_list(a1040)
            };
            nat_add(a1041, a1042)?
        };
        let a1044 = 1u64;
        nat_add(a1043, a1044)?
    };
    f241({
        let (_, h1045) = {
            let (_, h1046) = {
                let (h1047, _) = f242(Fn33::F243, v2, (v0, (v1, List::<(Bytes, Adt6)>::nil())));
                h1047
            };
            h1046
        };
        h1045
    })
}

pub fn f241(v0: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    f122(Fn13::F244, List::<(Bytes, Adt6)>::nil(), v0)
}

pub fn f242(v0: Fn33, p1052: u64, p1053: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool) {
    let m1051 = (p1052, p1053);
    {
        let mut m1051 = m1051;
        loop {
            let step = {
                let (v1, v2) = m1051;
                let m1048 = v1;
                if m1048 == 0 {
                    Err::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v2, false))
                } else {
                    let v3 = m1048 - 1;
                    let m1050 = {
                        let c1049: Fn33 = v0.clone();
                        c1049.apply(v2.clone())
                    };
                    match m1050 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1051 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
    f245(v0)
}

pub fn f244(v0: List<(Bytes, Adt6)>, v1: (Bytes, Adt6)) -> List<(Bytes, Adt6)> {
    List::cons(v1, v0)
}

pub fn f245(v0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
    let v1: List<(Bytes, Adt6)> = {
        let (h1054, _) = v0.clone();
        h1054
    };
    let v2: List<(Bytes, Adt6)> = {
        let (h1055, _) = {
            let (_, h1056) = v0.clone();
            h1056
        };
        h1055
    };
    let v3: List<(Bytes, Adt6)> = {
        let (_, h1057) = {
            let (_, h1058) = v0;
            h1058
        };
        h1057
    };
    let m1059 = v1.clone();
    match m1059.uncons() {
        None => {
            let m1060 = v2;
            match m1060.uncons() {
                None => {
                    None::<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt6)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1061 = v2.clone();
            match m1061.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt6)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f246({
                        let (h1062, _) = v8.clone();
                        h1062
                    }, {
                        let (h1063, _) = v6.clone();
                        h1063
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

pub fn f246(v0: Bytes, v1: Bytes) -> bool {
    let a1068 = {
        let a1064 = v0;
        let a1065 = v1;
        compare_bytes(a1064, a1065)
    };
    let a1069 = {
        let a1066 = Bytes::lit(&[]);
        let a1067 = Bytes::lit(&[0x01]);
        compare_bytes(a1066, a1067)
    };
    equal(a1068, a1069)
}
