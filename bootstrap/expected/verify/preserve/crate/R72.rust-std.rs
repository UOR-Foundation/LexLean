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
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt4, u64, u64),
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
    C11(Bytes),
    C12(Bytes),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, List<Adt3>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, List<Adt7>, Adt0, u64, u64, Adt2, u64, Option<Adt1>, List<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, u64, u64, u64, Bytes, Adt2, Option<Adt0>, List<Adt0>, List<(Bytes, Adt0)>, Adt0, Adt0, List<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt0, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, List<Bytes>, bool, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, Adt0, Adt0),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Adt0, u64, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, u64, u64, List<Adt14>, Adt0, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Adt0>),
    C1(Bytes, u64, List<(Bytes, Adt0)>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<Adt0>, u64, bool, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Adt0, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt0, Adt0),
}

#[derive(Clone)]
pub enum Fn0 {
    F8(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn0::F8(k0, k1) => f8(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15(Adt0, u64, List<Adt3>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn1::F15(k0, k1, k2) => f15(k0.clone(), *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F18,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn2::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F21(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt8) -> Option<Adt8> {
        match self {
            Fn3::F21(k0) => f21(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F33(Adt0),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn4::F33(k0) => f33(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F52(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn5::F52(k0) => f52(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F67(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn6::F67(k0, k1) => f67(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F71(u64, Bytes),
    F180(Bytes),
    F181(Bytes),
    F194(Bytes),
    F209(u64, Bytes),
    F272(Bytes),
    F282(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F71(k0, k1) => f71(*k0, k1.clone(), p0),
            Fn7::F180(k0) => f180(k0.clone(), p0),
            Fn7::F181(k0) => f181(k0.clone(), p0),
            Fn7::F194(k0) => f194(k0.clone(), p0),
            Fn7::F209(k0, k1) => f209(*k0, k1.clone(), p0),
            Fn7::F272(k0) => f272(k0.clone(), p0),
            Fn7::F282(k0) => f282(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F130,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn8::F130 => f130(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F132,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn9::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F146(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn10::F146(k0) => f146(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F159(u64, List<Adt3>),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt13) -> R<Option<Adt13>> {
        match self {
            Fn11::F159(k0, k1) => f159(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F161(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt0)) -> bool {
        match self {
            Fn12::F161(k0) => f161(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F168(u64, List<Adt3>),
    F222(u64, List<Adt3>, Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt15) -> R<Option<Adt15>> {
        match self {
            Fn13::F168(k0, k1) => f168(*k0, k1.clone(), p0),
            Fn13::F222(k0, k1, k2) => f222(*k0, k1.clone(), k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F178,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn14::F178 => f178(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F184(Bytes, Adt0),
}

impl Fn15 {
    pub fn apply(&self, p0: Option<Adt0>, p1: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn15::F184(k0, k1) => f184(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F187(Adt0),
    F236(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> R<bool> {
        match self {
            Fn16::F187(k0) => f187(k0.clone(), p0, p1),
            Fn16::F236(k0) => Ok(f236(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F197,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt0, p1: (Bytes, Adt0)) -> Adt0 {
        match self {
            Fn17::F197 => f197(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F201,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn18::F201 => f201(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F203,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn19::F203 => f203(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F211,
}

impl Fn20 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn20::F211 => f211(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F218,
}

impl Fn21 {
    pub fn apply(&self, p0: Adt16) -> Option<Adt16> {
        match self {
            Fn21::F218 => f218(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F221(u64, Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: Adt17) -> R<Option<Adt17>> {
        match self {
            Fn22::F221(k0, k1) => f221(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F226,
}

impl Fn23 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn23::F226 => f226(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F228,
}

impl Fn24 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn24::F228 => f228(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F238,
}

impl Fn25 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn25::F238 => f238(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F261,
}

impl Fn26 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn26::F261 => f261(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F275(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn27::F275(k0) => f275(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F277,
}

impl Fn28 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn28::F277 => f277(p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: Bytes, v2: u64) -> R<Result<Adt0, Adt1>> {
    let m1 = f1(v1.clone())?;
    match m1 {
        Ok(v3) => {
            let v4: Adt2 = f2(v0.clone(), v2)?;
            Ok(f3(f4(v3, {
                let a2 = v1;
                length_bytes(a2)
            }, v0, f5(f6(0u64, 1u64), v4.clone(), None::<Adt0>, List::<(Bytes, Adt0)>::nil()), v4)?))
        }
        Err(v5) => {
            Ok(Err::<Adt0, Adt1>(v5))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt3>, Adt1>> {
    let v1: u64 = {
        let a3 = v0.clone();
        length_bytes(a3)
    };
    let v2: Adt5 = {
        let (h4, _) = f7(Fn0::F8(v1, v0), {
            let a5 = v1;
            let a6 = 1u64;
            nat_add(a5, a6)?
        }, Adt5::C0(0u64, List::<Adt3>::nil(), None::<Adt1>))?;
        h4
    };
    let m8 = match v2.clone() {
        Adt5::C0(_, _, h7) => {
            h7
        }
    };
    match m8 {
        None => {
            Ok(Ok::<List<Adt3>, Adt1>(f9(match v2 {
                Adt5::C0(_, h9, _) => {
                    h9
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt3>, Adt1>(v3))
        }
    }
}

pub fn f2(v0: Adt0, v1: u64) -> R<Adt2> {
    let a10 = v1;
    let a11 = 0u64;
    if nat_eq(a10, a11) {
        let m12 = f10(v0.clone(), f11(Str::lit("properties")));
        match m12 {
            None => {
                Ok(Adt2::C5)
            }
            Some(v2) => {
                let m13 = f10(v2, f11(Str::lit("declarations")));
                match m13 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v3) => {
                        f12(v0, v3)
                    }
                }
            }
        }
    } else {
        let a14 = v1;
        let a15 = 1u64;
        if nat_eq(a14, a15) {
            Ok(Adt2::C0)
        } else {
            let a16 = v1;
            let a17 = 2u64;
            if nat_eq(a16, a17) {
                Ok(Adt2::C1)
            } else {
                let m18 = f13(v0, f11(Str::lit("proof")));
                match m18 {
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

pub fn f3(v0: Adt6) -> Result<Adt0, Adt1> {
    let m20 = match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h19, _) => {
            h19
        }
    };
    match m20 {
        None => {
            let a22 = match v0.clone() {
                Adt6::C0(_, _, _, h21, _, _, _, _, _) => {
                    h21
                }
            };
            let a23 = 9u64;
            if nat_eq(a22, a23) {
                Ok::<Adt0, Adt1>(match v0 {
                    Adt6::C0(_, _, h24, _, _, _, _, _, _) => {
                        h24
                    }
                })
            } else {
                Err::<Adt0, Adt1>(Adt1::C0(0u64, f11(Str::lit("the reader ran out of steps"))))
            }
        }
        Some(v1) => {
            Err::<Adt0, Adt1>(v1)
        }
    }
}

pub fn f4(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt7, v4: Adt2) -> R<Adt6> {
    let v5: Adt6 = Adt6::C0(0u64, List::cons(v3, List::<Adt7>::nil()), Adt0::C5, 0u64, 0u64, v4, 1u64, None::<Adt1>, List::<(u64, u64)>::nil());
    let (h25, _) = f14(Fn1::F15(v2, v1, v0.clone()), {
        let a29 = {
            let a27 = {
                let a26 = v0;
                length_list(a26)
            };
            let a28 = 16u64;
            nat_mul(a27, a28)?
        };
        let a30 = 64u64;
        nat_add(a29, a30)?
    }, v5)?;
    Ok(h25)
}

pub fn f5(v0: Adt7, v1: Adt2, v2: Option<Adt0>, v3: List<(Bytes, Adt0)>) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h31, _, _, _, _, _, _, _, _, _, _, _) => {
            h31
        }
    }, match v0.clone() {
        Adt7::C0(_, h32, _, _, _, _, _, _, _, _, _, _) => {
            h32
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h33, _, _, _, _, _, _, _, _, _) => {
            h33
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h34, _, _, _, _, _, _, _, _) => {
            h34
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h35, _, _, _, _, _, _, _) => {
            h35
        }
    }, v1, v2, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h36, _, _, _, _) => {
            h36
        }
    }, v3, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h37, _, _) => {
            h37
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h38, _) => {
            h38
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h39) => {
            h39
        }
    })
}

pub fn f6(v0: u64, v1: u64) -> Adt7 {
    Adt7::C0(v0, 0u64, v1, 0u64, f11(Str::lit("")), Adt2::C5, None::<Adt0>, List::<Adt0>::nil(), List::<(Bytes, Adt0)>::nil(), Adt0::C5, Adt0::C5, List::<(u64, u64)>::nil())
}

pub fn f7(v0: Fn0, p44: u64, p45: Adt5) -> R<(Adt5, bool)> {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m40 = v1;
                if m40 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m40 - 1;
                    let m42 = {
                        let c41: Fn0 = v0.clone();
                        c41.apply(v2.clone())?
                    };
                    match m42 {
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
                Ok(next) => { m43 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f16(v1, v0, v2)
}

pub fn f9(v0: List<Adt3>) -> List<Adt3> {
    f17(Fn2::F18, List::<Adt3>::nil(), v0)
}

pub fn f10(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m46 = f19(v0);
    match m46 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            let m49 = {
                let (h48, _) = f20(Fn3::F21(v1), 1000000u64, Adt8::C0(v2, None::<Adt0>));
                h48
            };
            match m49 {
                Adt8::C0(_, h47) => {
                    h47
                }
            }
        }
    }
}

pub fn f11(v0: Str) -> Bytes {
    let a50 = v0;
    utf8_encode(a50)
}

pub fn f12(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m51 = f10(f22(v0.clone(), v1)?, f11(Str::lit("items")));
    match m51 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v2) => {
            f23(v0, v2)
        }
    }
}

pub fn f13(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m52 = f10(v0, f11(Str::lit("$defs")));
    match m52 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f10(v2, v1)
        }
    }
}

pub fn f14(v0: Fn1, p57: u64, p58: Adt6) -> R<(Adt6, bool)> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v1, v2) = m56;
                let m53 = v1;
                if m53 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m53 - 1;
                    let m55 = {
                        let c54: Fn1 = v0.clone();
                        c54.apply(v2.clone())?
                    };
                    match m55 {
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
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Adt0, v1: u64, v2: List<Adt3>, v3: Adt6) -> R<Option<Adt6>> {
    f24(v2, v1, v0, v3)
}

pub fn f16(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let m60 = match v2.clone() {
        Adt5::C0(_, _, h59) => {
            h59
        }
    };
    match m60 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt5::C0(h61, _, _) => {
                    h61
                }
            };
            let a62 = v4;
            let a63 = v1;
            if nat_lt(a62, a63) {
                let v5: u64 = f25(v0.clone(), v4)?;
                let a64 = 0u64;
                let a65 = v5;
                if nat_lt(a64, a65) {
                    Ok(Some(Adt5::C0({
                        let a66 = v4;
                        let a67 = v5;
                        nat_add(a66, a67)?
                    }, match v2 {
                        Adt5::C0(_, h68, _) => {
                            h68
                        }
                    }, None::<Adt1>)))
                } else {
                    if f26(v0.clone(), v4, 123u8) {
                        Ok(f27(v2, Adt4::C0, {
                            let a69 = v4;
                            let a70 = 1u64;
                            nat_add(a69, a70)?
                        }))
                    } else {
                        if f26(v0.clone(), v4, 125u8) {
                            Ok(f27(v2, Adt4::C1, {
                                let a71 = v4;
                                let a72 = 1u64;
                                nat_add(a71, a72)?
                            }))
                        } else {
                            if f26(v0.clone(), v4, 91u8) {
                                Ok(f27(v2, Adt4::C2, {
                                    let a73 = v4;
                                    let a74 = 1u64;
                                    nat_add(a73, a74)?
                                }))
                            } else {
                                if f26(v0.clone(), v4, 93u8) {
                                    Ok(f27(v2, Adt4::C3, {
                                        let a75 = v4;
                                        let a76 = 1u64;
                                        nat_add(a75, a76)?
                                    }))
                                } else {
                                    if f26(v0.clone(), v4, 40u8) {
                                        Ok(f27(v2, Adt4::C4, {
                                            let a77 = v4;
                                            let a78 = 1u64;
                                            nat_add(a77, a78)?
                                        }))
                                    } else {
                                        if f26(v0.clone(), v4, 41u8) {
                                            Ok(f27(v2, Adt4::C5, {
                                                let a79 = v4;
                                                let a80 = 1u64;
                                                nat_add(a79, a80)?
                                            }))
                                        } else {
                                            if f26(v0.clone(), v4, 44u8) {
                                                Ok(f27(v2, Adt4::C6, {
                                                    let a81 = v4;
                                                    let a82 = 1u64;
                                                    nat_add(a81, a82)?
                                                }))
                                            } else {
                                                if f26(v0.clone(), v4, 59u8) {
                                                    Ok(f27(v2, Adt4::C10, {
                                                        let a83 = v4;
                                                        let a84 = 1u64;
                                                        nat_add(a83, a84)?
                                                    }))
                                                } else {
                                                    if f26(v0.clone(), v4, 58u8) {
                                                        f28(v0, v2)
                                                    } else {
                                                        if f26(v0.clone(), v4, 34u8) {
                                                            f29(v0, v1, v2)
                                                        } else {
                                                            f30(v0, v1, v2)
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
                Ok(None::<Adt5>)
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f17(v0: Fn2, p88: List<Adt3>, p89: List<Adt3>) -> List<Adt3> {
    let m87 = (p88, p89);
    {
        let mut m87 = m87;
        loop {
            let step = {
                let (v1, v2) = m87;
                let m85 = v2;
                match m85.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c86: Fn2 = v0.clone();
                            c86.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m87 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f19(v0: Adt0) -> Option<Adt0> {
    let m90 = v0;
    match m90 {
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
        Adt0::C4(r91) => {
            let v1 = (*r91).clone();
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

pub fn f20(v0: Fn3, p96: u64, p97: Adt8) -> (Adt8, bool) {
    let m95 = (p96, p97);
    {
        let mut m95 = m95;
        loop {
            let step = {
                let (v1, v2) = m95;
                let m92 = v1;
                if m92 == 0 {
                    Err::<(u64, Adt8), (Adt8, bool)>((v2, false))
                } else {
                    let v3 = m92 - 1;
                    let m94 = {
                        let c93: Fn3 = v0.clone();
                        c93.apply(v2.clone())
                    };
                    match m94 {
                        None => {
                            Err::<(u64, Adt8), (Adt8, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt8), (Adt8, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m95 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Bytes, v1: Adt8) -> Option<Adt8> {
    f31(v0, v1)
}

pub fn f22(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let (h98, _) = f32(Fn4::F33(v0), 4096u64, v1)?;
    Ok(h98)
}

pub fn f23(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m99 = f34(f10(v1.clone(), f11(Str::lit("$ref"))));
    match m99 {
        None => {
            Ok(Adt2::C4(v1))
        }
        Some(v2) => {
            let v3: Bytes = f35(v2)?;
            if f36(v3.clone(), f11(Str::lit("term"))) {
                Ok(Adt2::C0)
            } else {
                if f36(v3.clone(), f11(Str::lit("type"))) {
                    Ok(Adt2::C1)
                } else {
                    if f36(v3.clone(), f11(Str::lit("member"))) {
                        Ok(Adt2::C2)
                    } else {
                        if f36(v3, f11(Str::lit("parameter"))) {
                            Ok(Adt2::C3)
                        } else {
                            Ok(Adt2::C4(f22(v0, v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f24(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6) -> R<Option<Adt6>> {
    let m101 = match v3.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h100, _) => {
            h100
        }
    };
    match m101 {
        None => {
            let v5: u64 = match v3.clone() {
                Adt6::C0(_, _, _, h102, _, _, _, _, _) => {
                    h102
                }
            };
            let a103 = v5;
            let a104 = 9u64;
            if nat_eq(a103, a104) {
                Ok(None::<Adt6>)
            } else {
                Ok(Some({
                    let a105 = v5;
                    let a106 = 0u64;
                    if nat_eq(a105, a106) {
                        f37(v0, v1, v2, v3)?
                    } else {
                        f38(v0, v1, v2, v3)?
                    }
                }))
            }
        }
        Some(_) => {
            Ok(None::<Adt6>)
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<u64> {
    if if f39(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f40(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f40(v0.clone(), v1, 194u8) {
            if if f40(v0.clone(), {
                let a107 = v1;
                let a108 = 1u64;
                nat_add(a107, a108)?
            }, 133u8) {
                true
            } else {
                f40(v0, {
                    let a109 = v1;
                    let a110 = 1u64;
                    nat_add(a109, a110)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f40(v0.clone(), v1, 225u8) {
                if if f40(v0.clone(), {
                    let a111 = v1;
                    let a112 = 1u64;
                    nat_add(a111, a112)?
                }, 154u8) {
                    f40(v0, {
                        let a113 = v1;
                        let a114 = 2u64;
                        nat_add(a113, a114)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f40(v0.clone(), v1, 226u8) {
                    if f40(v0.clone(), {
                        let a115 = v1;
                        let a116 = 1u64;
                        nat_add(a115, a116)?
                    }, 128u8) {
                        if if f39(v0.clone(), {
                            let a117 = v1;
                            let a118 = 2u64;
                            nat_add(a117, a118)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f40(v0.clone(), {
                                let a119 = v1;
                                let a120 = 2u64;
                                nat_add(a119, a120)?
                            }, 168u8) {
                                true
                            } else {
                                if f40(v0.clone(), {
                                    let a121 = v1;
                                    let a122 = 2u64;
                                    nat_add(a121, a122)?
                                }, 169u8) {
                                    true
                                } else {
                                    f40(v0, {
                                        let a123 = v1;
                                        let a124 = 2u64;
                                        nat_add(a123, a124)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f40(v0.clone(), {
                            let a125 = v1;
                            let a126 = 1u64;
                            nat_add(a125, a126)?
                        }, 129u8) {
                            f40(v0, {
                                let a127 = v1;
                                let a128 = 2u64;
                                nat_add(a127, a128)?
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
                    if f40(v0.clone(), v1, 227u8) {
                        if if f40(v0.clone(), {
                            let a129 = v1;
                            let a130 = 1u64;
                            nat_add(a129, a130)?
                        }, 128u8) {
                            f40(v0, {
                                let a131 = v1;
                                let a132 = 2u64;
                                nat_add(a131, a132)?
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

pub fn f26(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m133 = f41(v0, v1);
    match m133 {
        None => {
            false
        }
        Some(v3) => {
            let a134 = v3;
            let a135 = v2;
            equal(a134, a135)
        }
    }
}

pub fn f27(v0: Adt5, v1: Adt4, v2: u64) -> Option<Adt5> {
    Some(Adt5::C0(v2, List::cons(Adt3::C0(v1, match v0.clone() {
        Adt5::C0(h136, _, _) => {
            h136
        }
    }, v2), match v0 {
        Adt5::C0(_, h137, _) => {
            h137
        }
    }), None::<Adt1>))
}

pub fn f28(v0: Bytes, v1: Adt5) -> R<Option<Adt5>> {
    let v2: u64 = match v1.clone() {
        Adt5::C0(h138, _, _) => {
            h138
        }
    };
    if f26(v0.clone(), {
        let a139 = v2;
        let a140 = 1u64;
        nat_add(a139, a140)?
    }, 61u8) {
        Ok(f27(v1, Adt4::C8, {
            let a141 = v2;
            let a142 = 2u64;
            nat_add(a141, a142)?
        }))
    } else {
        if f26(v0, {
            let a143 = v2;
            let a144 = 1u64;
            nat_add(a143, a144)?
        }, 58u8) {
            Ok(f27(v1, Adt4::C9, {
                let a145 = v2;
                let a146 = 2u64;
                nat_add(a145, a146)?
            }))
        } else {
            Ok(f27(v1, Adt4::C7, {
                let a147 = v2;
                let a148 = 1u64;
                nat_add(a147, a148)?
            }))
        }
    }
}

pub fn f29(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h149, _, _) => {
            h149
        }
    };
    let v4: Adt9 = f42(v0, v1, v3)?;
    let m151 = match v4.clone() {
        Adt9::C0(_, _, _, h150) => {
            h150
        }
    };
    match m151 {
        None => {
            if match v4.clone() {
                Adt9::C0(_, _, h153, _) => {
                    h153
                }
            } {
                Ok(f27(v2, Adt4::C11(f43(match v4.clone() {
                    Adt9::C0(_, h154, _, _) => {
                        h154
                    }
                })), match v4 {
                    Adt9::C0(h155, _, _, _) => {
                        h155
                    }
                }))
            } else {
                Ok(Some(Adt5::C0(v3, match v2 {
                    Adt5::C0(_, h156, _) => {
                        h156
                    }
                }, Some(f44(v3, f45(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt5::C0(v3, match v2 {
                Adt5::C0(_, h152, _) => {
                    h152
                }
            }, Some(v5))))
        }
    }
}

pub fn f30(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h157, _, _) => {
            h157
        }
    };
    let v4: u64 = f46(v0.clone(), v1, v3)?;
    Ok(f27(v2, Adt4::C12(f47(v0, v3, {
        let a158 = v4;
        let a159 = v3;
        nat_sub(a158, a159)
    })), v4))
}

pub fn f31(v0: Bytes, v1: Adt8) -> Option<Adt8> {
    let m161 = match v1.clone() {
        Adt8::C0(_, h160) => {
            h160
        }
    };
    match m161 {
        None => {
            let m163 = f48(match v1 {
                Adt8::C0(h162, _) => {
                    h162
                }
            });
            match m163 {
                None => {
                    None::<Adt8>
                }
                Some(v3) => {
                    if f36(match v3.clone() {
                        Adt11::C0(h164, _, _) => {
                            h164
                        }
                    }, v0) {
                        Some(Adt8::C0(Adt0::C5, Some(match v3 {
                            Adt11::C0(_, h165, _) => {
                                h165
                            }
                        })))
                    } else {
                        Some(Adt8::C0(match v3 {
                            Adt11::C0(_, _, h166) => {
                                h166
                            }
                        }, None::<Adt0>))
                    }
                }
            }
        }
        Some(_) => {
            None::<Adt8>
        }
    }
}

pub fn f32(v0: Fn4, p171: u64, p172: Adt0) -> R<(Adt0, bool)> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v1, v2) = m170;
                let m167 = v1;
                if m167 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m167 - 1;
                    let m169 = {
                        let c168: Fn4 = v0.clone();
                        c168.apply(v2.clone())?
                    };
                    match m169 {
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
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    f49(v0, v1)
}

pub fn f34(v0: Option<Adt0>) -> Option<Bytes> {
    let m173 = v0;
    match m173 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f50(v1)
        }
    }
}

pub fn f35(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let m179 = {
            let (h175, _) = f51(Fn5::F52(v0.clone()), {
                let a177 = {
                    let a176 = v0.clone();
                    length_bytes(a176)
                };
                let a178 = 1u64;
                nat_add(a177, a178)?
            }, Adt10::C0(0u64, 0u64))?;
            h175
        };
        match m179 {
            Adt10::C0(_, h174) => {
                h174
            }
        }
    };
    Ok(f47(v0.clone(), v1, {
        let a181 = {
            let a180 = v0;
            length_bytes(a180)
        };
        let a182 = v1;
        nat_sub(a181, a182)
    }))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a183 = v0;
    let a184 = v1;
    equal(a183, a184)
}

pub fn f37(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6) -> R<Adt6> {
    let v4: u64 = match v3.clone() {
        Adt6::C0(_, _, _, _, h185, _, _, _, _) => {
            h185
        }
    };
    let a186 = v4;
    let a187 = 0u64;
    if nat_eq(a186, a187) {
        f53(v0, v1, v2, v3)
    } else {
        let a188 = v4;
        let a189 = 1u64;
        if nat_eq(a188, a189) {
            f54(v0, v1, v2, v3)
        } else {
            let a190 = v4;
            let a191 = 2u64;
            if nat_eq(a190, a191) {
                f55(v0, v1, v3)
            } else {
                let a192 = v4;
                let a193 = 3u64;
                if nat_eq(a192, a193) {
                    f56(v0, v1, v3)
                } else {
                    let a194 = v4;
                    let a195 = 4u64;
                    if nat_eq(a194, a195) {
                        f57(v0, v1, v3)
                    } else {
                        let a196 = v4;
                        let a197 = 5u64;
                        if nat_eq(a196, a197) {
                            f58(v0, v1, v3.clone(), match v3 {
                                Adt6::C0(_, _, _, _, _, _, h198, _, _) => {
                                    h198
                                }
                            })
                        } else {
                            f59(v0, v1, v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f38(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6) -> R<Adt6> {
    let m200 = match v3.clone() {
        Adt6::C0(_, h199, _, _, _, _, _, _, _) => {
            h199
        }
    };
    match m200.uncons() {
        None => {
            Ok(f60(v3.clone(), match v3 {
                Adt6::C0(_, _, h201, _, _, _, _, _, _) => {
                    h201
                }
            }, List::<(u64, u64)>::nil()))
        }
        Some((v4, v5)) => {
            let v6: Adt6 = Adt6::C0(match v3.clone() {
                Adt6::C0(h202, _, _, _, _, _, _, _, _) => {
                    h202
                }
            }, v5, match v3.clone() {
                Adt6::C0(_, _, h203, _, _, _, _, _, _) => {
                    h203
                }
            }, 1u64, match v3.clone() {
                Adt6::C0(_, _, _, _, h204, _, _, _, _) => {
                    h204
                }
            }, match v3.clone() {
                Adt6::C0(_, _, _, _, _, h205, _, _, _) => {
                    h205
                }
            }, match v3.clone() {
                Adt6::C0(_, _, _, _, _, _, h206, _, _) => {
                    h206
                }
            }, None::<Adt1>, match v3 {
                Adt6::C0(_, _, _, _, _, _, _, _, h207) => {
                    h207
                }
            });
            let v7: u64 = match v4.clone() {
                Adt7::C0(h208, _, _, _, _, _, _, _, _, _, _, _) => {
                    h208
                }
            };
            let a209 = v7;
            let a210 = 0u64;
            if nat_eq(a209, a210) {
                Ok(f61(v0, v1, v2, v6, v4))
            } else {
                let a211 = v7;
                let a212 = 10u64;
                if nat_lt(a211, a212) {
                    f62(v0, v1, v2, v6, v4)
                } else {
                    let a213 = v7;
                    let a214 = 30u64;
                    if nat_lt(a213, a214) {
                        f63(v0, v1, v6, v4)
                    } else {
                        f64(v0, v1, v6, v4)
                    }
                }
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m215 = f41(v0, v1);
    match m215 {
        None => {
            false
        }
        Some(v4) => {
            f65(v4, v2, v3)
        }
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m216 = f41(v0, v1);
    match m216 {
        None => {
            false
        }
        Some(v3) => {
            let a217 = v3;
            let a218 = v2;
            equal(a217, a218)
        }
    }
}

pub fn f41(v0: Bytes, v1: u64) -> Option<u8> {
    let a219 = v0;
    let a220 = v1;
    index_bytes(a219, a220)
}

pub fn f42(v0: Bytes, v1: u64, v2: u64) -> R<Adt9> {
    let (h221, _) = f66(Fn6::F67(v1, v0), {
        let a224 = {
            let a222 = v1;
            let a223 = v2;
            nat_sub(a222, a223)
        };
        let a225 = 1u64;
        nat_add(a224, a225)?
    }, Adt9::C0({
        let a226 = v2;
        let a227 = 1u64;
        nat_add(a226, a227)?
    }, List::<Bytes>::nil(), false, None::<Adt1>))?;
    Ok(h221)
}

pub fn f43(v0: List<Bytes>) -> Bytes {
    f68(f69(v0))
}

pub fn f44(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f45(v0: Str) -> Bytes {
    let a228 = v0;
    utf8_encode(a228)
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h229, _) = f70(Fn7::F71(v1, v0), {
        let a230 = v1;
        let a231 = v2;
        nat_sub(a230, a231)
    }, v2)?;
    Ok(h229)
}

pub fn f47(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m235 = {
        let a232 = v0;
        let a233 = v1;
        let a234 = v2;
        slice_bytes(a232, a233, a234)
    };
    match m235 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f48(v0: Adt0) -> Option<Adt11> {
    let m236 = v0;
    match m236 {
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
        Adt0::C6(_, _) => {
            None::<Adt11>
        }
        Adt0::C7(v1, r237, r238) => {
            let v2 = (*r237).clone();
            let v3 = (*r238).clone();
            Some(Adt11::C0(v1, v2, v3))
        }
    }
}

pub fn f49(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    let m239 = f34(f10(v1, f11(Str::lit("$ref"))));
    match m239 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v2) => {
            Ok(f13(v0, f35(v2)?))
        }
    }
}

pub fn f50(v0: Adt0) -> Option<Bytes> {
    let m240 = v0;
    match m240 {
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

pub fn f51(v0: Fn5, p245: u64, p246: Adt10) -> R<(Adt10, bool)> {
    let m244 = (p245, p246);
    {
        let mut m244 = m244;
        loop {
            let step = {
                let (v1, v2) = m244;
                let m241 = v1;
                if m241 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m241 - 1;
                    let m243 = {
                        let c242: Fn5 = v0.clone();
                        c242.apply(v2.clone())?
                    };
                    match m243 {
                        None => {
                            Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt10), R<(Adt10, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    f72(v0, v1)
}

pub fn f53(v0: List<Adt3>, v1: u64, _: Adt0, v3: Adt6) -> R<Adt6> {
    let v4: u64 = match v3.clone() {
        Adt6::C0(h247, _, _, _, _, _, _, _, _) => {
            h247
        }
    };
    let v5: u64 = match v3.clone() {
        Adt6::C0(_, _, _, _, _, _, h248, _, _) => {
            h248
        }
    };
    let a249 = 256u64;
    let a250 = v5;
    if nat_lt(a249, a250) {
        Ok(f73(v0, v1, v3, v4, f11(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let m252 = match v3.clone() {
            Adt6::C0(_, _, _, _, _, h251, _, _, _) => {
                h251
            }
        };
        match m252 {
            Adt2::C0 => {
                Ok(f74(v3, v4, 3u64, Adt2::C0, v5))
            }
            Adt2::C1 => {
                Ok(f74(v3, v4, 2u64, Adt2::C1, v5))
            }
            Adt2::C2 => {
                let a253 = f75(v0.clone(), v4);
                let a254 = 0u64;
                if nat_eq(a253, a254) {
                    f58(v0, v1, v3, 0u64)
                } else {
                    let v6: Adt12 = f76(v0, v1, v4)?;
                    let m256 = match v6.clone() {
                        Adt12::C0(_, _, h255) => {
                            h255
                        }
                    };
                    match m256 {
                        None => {
                            Ok(f78(v3, match v6.clone() {
                                Adt12::C0(_, h258, _) => {
                                    h258
                                }
                            }, match v6 {
                                Adt12::C0(h259, _, _) => {
                                    h259
                                }
                            }))
                        }
                        Some(_) => {
                            Ok(f77(v3, match v6 {
                                Adt12::C0(_, _, h257) => {
                                    h257
                                }
                            }))
                        }
                    }
                }
            }
            Adt2::C3 => {
                Ok(f74(v3, v4, 4u64, Adt2::C3, v5))
            }
            Adt2::C4(_) => {
                Ok(f74(v3.clone(), v4, 1u64, match v3 {
                    Adt6::C0(_, _, _, _, _, h260, _, _, _) => {
                        h260
                    }
                }, v5))
            }
            Adt2::C5 => {
                Ok(f74(v3, v4, 5u64, Adt2::C5, v5))
            }
        }
    }
}

pub fn f54(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6) -> R<Adt6> {
    let v4: u64 = match v3.clone() {
        Adt6::C0(h261, _, _, _, _, _, _, _, _) => {
            h261
        }
    };
    let v5: u64 = match v3.clone() {
        Adt6::C0(_, _, _, _, _, _, h262, _, _) => {
            h262
        }
    };
    let v6: Adt2 = match v3.clone() {
        Adt6::C0(_, _, _, _, _, h263, _, _, _) => {
            h263
        }
    };
    let v7: u64 = f75(v0.clone(), v4);
    let a264 = v7;
    let a265 = 2u64;
    if nat_eq(a264, a265) {
        let a268 = f75(v0, {
            let a266 = v4;
            let a267 = 1u64;
            nat_add(a266, a267)?
        });
        let a269 = 3u64;
        if nat_eq(a268, a269) {
            Ok(f78(v3, {
                let a270 = v4;
                let a271 = 2u64;
                nat_add(a270, a271)?
            }, f79(List::<Adt0>::nil())))
        } else {
            let v8: Adt2 = f80(v2, v6)?;
            Ok(f81(v3, f5(f6(1u64, v5), v8.clone(), None::<Adt0>, List::<(Bytes, Adt0)>::nil()), {
                let a272 = v4;
                let a273 = 1u64;
                nat_add(a272, a273)?
            }, 0u64, v8, {
                let a274 = v5;
                let a275 = 1u64;
                nat_add(a274, a275)?
            }))
        }
    } else {
        let a276 = v7;
        let a277 = 0u64;
        if nat_eq(a276, a277) {
            f82(v0, v1, v2.clone(), v3, f5(f6(2u64, v5), v6.clone(), f83(v2, v6), List::<(Bytes, Adt0)>::nil()), {
                let a278 = v4;
                let a279 = 1u64;
                nat_add(a278, a279)?
            })
        } else {
            let v9: Bytes = f84(v0.clone(), v4);
            if f85({
                let a280 = v7;
                let a281 = 12u64;
                nat_eq(a280, a281)
            }, f85({
                let a284 = f75(v0.clone(), {
                    let a282 = v4;
                    let a283 = 1u64;
                    nat_add(a282, a283)?
                });
                let a285 = 0u64;
                nat_eq(a284, a285)
            }, f85(f86(v9.clone())?, !f87(v9.clone())?))) {
                f82(v0, v1, v2.clone(), v3, f5(f6(2u64, v5), v6.clone(), f88(v2, v6, v9.clone())?, List::cons((f11(Str::lit("kind")), f89(v9)), List::<(Bytes, Adt0)>::nil())), {
                    let a286 = v4;
                    let a287 = 2u64;
                    nat_add(a286, a287)?
                })
            } else {
                Ok(f74(v3, v4, 5u64, v6, v5))
            }
        }
    }
}

pub fn f55(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h288, _, _, _, _, _, _, _, _) => {
            h288
        }
    };
    let v4: u64 = match v2.clone() {
        Adt6::C0(_, _, _, _, _, _, h289, _, _) => {
            h289
        }
    };
    let a290 = 256u64;
    let a291 = v4;
    if nat_lt(a290, a291) {
        Ok(f73(v0, v1, v2, v3, f11(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v5: u64 = f75(v0.clone(), v3);
        let v6: Bytes = f84(v0.clone(), v3);
        let a292 = v5;
        let a293 = 12u64;
        if nat_eq(a292, a293) {
            if f85({
                let a296 = f75(v0.clone(), {
                    let a294 = v3;
                    let a295 = 1u64;
                    nat_add(a294, a295)?
                });
                let a297 = 0u64;
                nat_eq(a296, a297)
            }, f86(v6.clone())?) {
                Ok(f74(v2, v3, 1u64, Adt2::C1, v4))
            } else {
                let m298 = f90(v6.clone());
                match m298 {
                    None => {
                        if f85(f92(v6.clone(), f11(Str::lit("'"))), f86(f93(v6.clone(), 1u64))?) {
                            Ok(f78(v2, {
                                let a301 = v3;
                                let a302 = 1u64;
                                nat_add(a301, a302)?
                            }, f91(f11(Str::lit("parameter")), f94(Str::lit("name"), f89(f93(v6, 1u64))))?))
                        } else {
                            if f95(v6.clone(), f96()) {
                                Ok(f73(v0, v1, v2, v3, f97(f97(f97(f97(f11(Str::lit("`")), v6.clone()), f11(Str::lit("` takes arguments: write `("))), v6), f11(Str::lit(" \u{2026})`")))))
                            } else {
                                let v8: Adt12 = f76(v0, v1, v3)?;
                                let m304 = match v8.clone() {
                                    Adt12::C0(_, _, h303) => {
                                        h303
                                    }
                                };
                                match m304 {
                                    None => {
                                        Ok(f78(v2, match v8.clone() {
                                            Adt12::C0(_, h306, _) => {
                                                h306
                                            }
                                        }, f91(f11(Str::lit("named")), f98(Str::lit("member"), match v8 {
                                            Adt12::C0(h307, _, _) => {
                                                h307
                                            }
                                        }, Str::lit("arguments"), f79(List::<Adt0>::nil())))?))
                                    }
                                    Some(_) => {
                                        Ok(f77(v2, match v8 {
                                            Adt12::C0(_, _, h305) => {
                                                h305
                                            }
                                        }))
                                    }
                                }
                            }
                        }
                    }
                    Some(v7) => {
                        Ok(f78(v2, {
                            let a299 = v3;
                            let a300 = 1u64;
                            nat_add(a299, a300)?
                        }, f91(v7, List::<(Bytes, Adt0)>::nil())?))
                    }
                }
            }
        } else {
            let a308 = v5;
            let a309 = 4u64;
            if nat_eq(a308, a309) {
                f99(v0, v1, f81(v2, f6(11u64, v4), {
                    let a310 = v3;
                    let a311 = 1u64;
                    nat_add(a310, a311)?
                }, 2u64, Adt2::C1, v4), {
                    let a312 = v3;
                    let a313 = 1u64;
                    nat_add(a312, a313)?
                }, v4)
            } else {
                Ok(f73(v0.clone(), v1, v2, v3, f97(f11(Str::lit("expected a type, found ")), f100(v0, v3)?)))
            }
        }
    }
}

pub fn f56(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h314, _, _, _, _, _, _, _, _) => {
            h314
        }
    };
    let v4: u64 = match v2.clone() {
        Adt6::C0(_, _, _, _, _, _, h315, _, _) => {
            h315
        }
    };
    let a316 = 256u64;
    let a317 = v4;
    if nat_lt(a316, a317) {
        Ok(f73(v0, v1, v2, v3, f11(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v5: u64 = f75(v0.clone(), v3);
        let v6: Bytes = f84(v0.clone(), v3);
        let a318 = v5;
        let a319 = 11u64;
        if nat_eq(a318, a319) {
            Ok(f78(v2, {
                let a320 = v3;
                let a321 = 1u64;
                nat_add(a320, a321)?
            }, f91(f11(Str::lit("string")), f94(Str::lit("value"), f89(v6)))?))
        } else {
            let a322 = v5;
            let a323 = 4u64;
            if nat_eq(a322, a323) {
                f101(v0, v1, f81(v2, f6(30u64, v4), {
                    let a324 = v3;
                    let a325 = 1u64;
                    nat_add(a324, a325)?
                }, 3u64, Adt2::C0, v4), {
                    let a326 = v3;
                    let a327 = 1u64;
                    nat_add(a326, a327)?
                }, v4)
            } else {
                let a328 = v5;
                let a329 = 12u64;
                if nat_eq(a328, a329) {
                    if f85({
                        let a332 = f75(v0.clone(), {
                            let a330 = v3;
                            let a331 = 1u64;
                            nat_add(a330, a331)?
                        });
                        let a333 = 0u64;
                        nat_eq(a332, a333)
                    }, f86(v6.clone())?) {
                        Ok(f74(v2, v3, 1u64, Adt2::C0, v4))
                    } else {
                        f102(v0, v1, v2, v3, v6)
                    }
                } else {
                    Ok(f73(v0.clone(), v1, v2, v3, f97(f11(Str::lit("expected a term, found ")), f100(v0, v3)?)))
                }
            }
        }
    }
}

pub fn f57(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h334, _, _, _, _, _, _, _, _) => {
            h334
        }
    };
    let v4: u64 = match v2.clone() {
        Adt6::C0(_, _, _, _, _, _, h335, _, _) => {
            h335
        }
    };
    let a336 = f75(v0.clone(), v3);
    let a337 = 0u64;
    if nat_eq(a336, a337) {
        Ok(f74(v2, v3, 1u64, Adt2::C3, v4))
    } else {
        let v5: Adt12 = f103(v0.clone(), v1, v3, f11(Str::lit("a parameter name")))?;
        let m339 = match v5.clone() {
            Adt12::C0(_, _, h338) => {
                h338
            }
        };
        match m339 {
            None => {
                let a342 = f75(v0.clone(), match v5.clone() {
                    Adt12::C0(_, h341, _) => {
                        h341
                    }
                });
                let a343 = 7u64;
                if nat_eq(a342, a343) {
                    Ok(f81(v2, f104(f6(20u64, v4), match v5.clone() {
                        Adt12::C0(h344, _, _) => {
                            h344
                        }
                    }, Adt0::C5), {
                        let a346 = match v5 {
                            Adt12::C0(_, h345, _) => {
                                h345
                            }
                        };
                        let a347 = 1u64;
                        nat_add(a346, a347)?
                    }, 2u64, Adt2::C1, {
                        let a348 = v4;
                        let a349 = 1u64;
                        nat_add(a348, a349)?
                    }))
                } else {
                    Ok(f73(v0, v1, v2, match v5 {
                        Adt12::C0(_, h350, _) => {
                            h350
                        }
                    }, f11(Str::lit("expected `:` after a parameter name"))))
                }
            }
            Some(_) => {
                Ok(f77(v2, match v5 {
                    Adt12::C0(_, _, h340) => {
                        h340
                    }
                }))
            }
        }
    }
}

pub fn f58(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64) -> R<Adt6> {
    let v4: Adt13 = f105(v0, v1, match v2.clone() {
        Adt6::C0(h351, _, _, _, _, _, _, _, _) => {
            h351
        }
    }, v3)?;
    let m353 = match v4.clone() {
        Adt13::C0(_, _, _, _, _, h352) => {
            h352
        }
    };
    match m353 {
        None => {
            Ok(f78(v2, match v4.clone() {
                Adt13::C0(h355, _, _, _, _, _) => {
                    h355
                }
            }, match v4 {
                Adt13::C0(_, _, _, _, h356, _) => {
                    h356
                }
            }))
        }
        Some(_) => {
            Ok(f77(v2, match v4 {
                Adt13::C0(_, _, _, _, _, h354) => {
                    h354
                }
            }))
        }
    }
}

pub fn f59(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h357, _, _, _, _, _, _, _, _) => {
            h357
        }
    };
    let a358 = f75(v0.clone(), v3);
    let a359 = 2u64;
    if nat_eq(a358, a359) {
        f106(v0, v1, v2.clone(), f6(18u64, match v2 {
            Adt6::C0(_, _, _, _, _, _, h360, _, _) => {
                h360
            }
        }), {
            let a361 = v3;
            let a362 = 1u64;
            nat_add(a361, a362)?
        })
    } else {
        Ok(f73(v0, v1, v2, v3, f11(Str::lit("expected `[`"))))
    }
}

pub fn f60(v0: Adt6, v1: Adt0, v2: List<(u64, u64)>) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h363, _, _, _, _, _, _, _, _) => {
            h363
        }
    }, match v0.clone() {
        Adt6::C0(_, h364, _, _, _, _, _, _, _) => {
            h364
        }
    }, v1, 9u64, match v0.clone() {
        Adt6::C0(_, _, _, _, h365, _, _, _, _) => {
            h365
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h366, _, _, _) => {
            h366
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, h367, _, _) => {
            h367
        }
    }, None::<Adt1>, v2)
}

pub fn f61(v0: List<Adt3>, v1: u64, _: Adt0, v3: Adt6, v4: Adt7) -> Adt6 {
    let v5: u64 = match v3.clone() {
        Adt6::C0(h368, _, _, _, _, _, _, _, _) => {
            h368
        }
    };
    let v6: Adt0 = match v3.clone() {
        Adt6::C0(_, _, h369, _, _, _, _, _, _) => {
            h369
        }
    };
    let a371 = match v4.clone() {
        Adt7::C0(_, h370, _, _, _, _, _, _, _, _, _, _) => {
            h370
        }
    };
    let a372 = 0u64;
    if nat_eq(a371, a372) {
        let a373 = f75(v0.clone(), v5);
        let a374 = 99u64;
        if nat_eq(a373, a374) {
            f60(v3, v6, List::<(u64, u64)>::nil())
        } else {
            f73(v0, v1, v3, v5, f11(Str::lit("unexpected text after the value")))
        }
    } else {
        let v7: Adt7 = f107(f108(v4.clone(), v6), (match v4.clone() {
            Adt7::C0(_, _, _, h375, _, _, _, _, _, _, _, _) => {
                h375
            }
        }, f109(v0.clone(), v5)), 0u64);
        let a376 = f75(v0.clone(), v5);
        let a377 = 99u64;
        if nat_eq(a376, a377) {
            f60(v3, f79(match v7.clone() {
                Adt7::C0(_, _, _, _, _, _, _, h378, _, _, _, _) => {
                    h378
                }
            }), match v7 {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h379) => {
                    h379
                }
            })
        } else {
            f81(v3, f110(v7, f11(Str::lit("")), f111(v0, v1, v5)), v5, 0u64, match v4 {
                Adt7::C0(_, _, _, _, _, h380, _, _, _, _, _, _) => {
                    h380
                }
            }, 1u64)
        }
    }
}

pub fn f62(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6, v4: Adt7) -> R<Adt6> {
    let v5: u64 = match v3.clone() {
        Adt6::C0(h381, _, _, _, _, _, _, _, _) => {
            h381
        }
    };
    let v6: Adt0 = match v3.clone() {
        Adt6::C0(_, _, h382, _, _, _, _, _, _) => {
            h382
        }
    };
    let a384 = match v4.clone() {
        Adt7::C0(h383, _, _, _, _, _, _, _, _, _, _, _) => {
            h383
        }
    };
    let a385 = 1u64;
    if nat_eq(a384, a385) {
        let v7: Adt7 = f108(v4.clone(), v6);
        let a386 = f75(v0.clone(), v5);
        let a387 = 6u64;
        if nat_eq(a386, a387) {
            let a390 = f75(v0, {
                let a388 = v5;
                let a389 = 1u64;
                nat_add(a388, a389)?
            });
            let a391 = 3u64;
            if nat_eq(a390, a391) {
                Ok(f78(v3, {
                    let a392 = v5;
                    let a393 = 2u64;
                    nat_add(a392, a393)?
                }, f79(match v7 {
                    Adt7::C0(_, _, _, _, _, _, _, h394, _, _, _, _) => {
                        h394
                    }
                })))
            } else {
                Ok(f81(v3, v7, {
                    let a395 = v5;
                    let a396 = 1u64;
                    nat_add(a395, a396)?
                }, 0u64, match v4.clone() {
                    Adt7::C0(_, _, _, _, _, h397, _, _, _, _, _, _) => {
                        h397
                    }
                }, {
                    let a399 = match v4 {
                        Adt7::C0(_, _, h398, _, _, _, _, _, _, _, _, _) => {
                            h398
                        }
                    };
                    let a400 = 1u64;
                    nat_add(a399, a400)?
                }))
            }
        } else {
            let a401 = f75(v0.clone(), v5);
            let a402 = 3u64;
            if nat_eq(a401, a402) {
                Ok(f78(v3, {
                    let a403 = v5;
                    let a404 = 1u64;
                    nat_add(a403, a404)?
                }, f79(match v7 {
                    Adt7::C0(_, _, _, _, _, _, _, h405, _, _, _, _) => {
                        h405
                    }
                })))
            } else {
                Ok(f73(v0, v1, v3, v5, f11(Str::lit("expected `,` or `]`"))))
            }
        }
    } else {
        let v8: Bytes = match v4.clone() {
            Adt7::C0(_, _, _, _, h406, _, _, _, _, _, _, _) => {
                h406
            }
        };
        if f112(match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h407, _, _, _) => {
                h407
            }
        }, v8.clone()) {
            Ok(f113(v3, match v4 {
                Adt7::C0(_, _, _, h408, _, _, _, _, _, _, _, _) => {
                    h408
                }
            }, f97(f97(f11(Str::lit("the member `")), v8), f11(Str::lit("` is written twice")))))
        } else {
            let v9: Adt7 = f5(v4.clone(), match v4.clone() {
                Adt7::C0(_, _, _, _, _, h409, _, _, _, _, _, _) => {
                    h409
                }
            }, match v4.clone() {
                Adt7::C0(_, _, _, _, _, _, h410, _, _, _, _, _) => {
                    h410
                }
            }, List::cons((v8, v6), match v4 {
                Adt7::C0(_, _, _, _, _, _, _, _, h411, _, _, _) => {
                    h411
                }
            }));
            let a412 = f75(v0.clone(), v5);
            let a413 = 6u64;
            if nat_eq(a412, a413) {
                f82(v0, v1, v2, v3, v9, {
                    let a414 = v5;
                    let a415 = 1u64;
                    nat_add(a414, a415)?
                })
            } else {
                let a416 = f75(v0.clone(), v5);
                let a417 = 1u64;
                if nat_eq(a416, a417) {
                    Ok(f78(v3, {
                        let a418 = v5;
                        let a419 = 1u64;
                        nat_add(a418, a419)?
                    }, f114(match v9 {
                        Adt7::C0(_, _, _, _, _, _, _, _, h420, _, _, _) => {
                            h420
                        }
                    })?))
                } else {
                    Ok(f73(v0, v1, v3, v5, f11(Str::lit("expected `,` or `}`"))))
                }
            }
        }
    }
}

pub fn f63(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7) -> R<Adt6> {
    let v4: u64 = match v2.clone() {
        Adt6::C0(h421, _, _, _, _, _, _, _, _) => {
            h421
        }
    };
    let v5: Adt0 = match v2.clone() {
        Adt6::C0(_, _, h422, _, _, _, _, _, _) => {
            h422
        }
    };
    let v6: u64 = match v3.clone() {
        Adt7::C0(h423, _, _, _, _, _, _, _, _, _, _, _) => {
            h423
        }
    };
    let v7: u64 = match v3.clone() {
        Adt7::C0(_, h424, _, _, _, _, _, _, _, _, _, _) => {
            h424
        }
    };
    let a425 = v6;
    let a426 = 11u64;
    if nat_eq(a425, a426) {
        let a427 = f75(v0.clone(), v4);
        let a428 = 5u64;
        if nat_eq(a427, a428) {
            Ok(f78(v2, {
                let a429 = v4;
                let a430 = 1u64;
                nat_add(a429, a430)?
            }, v5))
        } else {
            Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `)`"))))
        }
    } else {
        let a431 = v6;
        let a432 = 12u64;
        if nat_eq(a431, a432) {
            let v8: Bytes = match v3 {
                Adt7::C0(_, _, _, _, h433, _, _, _, _, _, _, _) => {
                    h433
                }
            };
            Ok(f78(v2, v4, if f36(v8.clone(), f11(Str::lit("List"))) {
                f91(f11(Str::lit("list")), f94(Str::lit("element"), v5))?
            } else {
                if f36(v8, f11(Str::lit("Set"))) {
                    f91(f11(Str::lit("set")), f94(Str::lit("element"), v5))?
                } else {
                    f91(f11(Str::lit("option")), f94(Str::lit("value"), v5))?
                }
            }))
        } else {
            let a434 = v6;
            let a435 = 13u64;
            if nat_eq(a434, a435) {
                let a436 = v7;
                let a437 = 0u64;
                if nat_eq(a436, a437) {
                    f115(v2, f116(f104(v3, v5, Adt0::C5), 1u64), v4)
                } else {
                    Ok(f78(v2, v4, if f36(match v3.clone() {
                        Adt7::C0(_, _, _, _, h438, _, _, _, _, _, _, _) => {
                            h438
                        }
                    }, f11(Str::lit("Result"))) {
                        f91(f11(Str::lit("result")), f98(Str::lit("ok"), match v3 {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, h439, _, _) => {
                                h439
                            }
                        }, Str::lit("error"), v5))?
                    } else {
                        f91(f11(Str::lit("map")), f98(Str::lit("key"), match v3 {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, h440, _, _) => {
                                h440
                            }
                        }, Str::lit("value"), v5))?
                    }))
                }
            } else {
                let a441 = v6;
                let a442 = 14u64;
                if nat_eq(a441, a442) {
                    let a443 = v7;
                    let a444 = 0u64;
                    if nat_eq(a443, a444) {
                        f117(v0, v1, v2, f108(v3, v5), v4)
                    } else {
                        Ok(f78(v2, v4, f91(f11(Str::lit("function")), f98(Str::lit("parameters"), f79(match v3 {
                            Adt7::C0(_, _, _, _, _, _, _, h445, _, _, _, _) => {
                                h445
                            }
                        }), Str::lit("result"), v5))?))
                    }
                } else {
                    let a446 = v6;
                    let a447 = 16u64;
                    if nat_eq(a446, a447) {
                        f118(v0, v1, v2, f108(v3, v5), v4)
                    } else {
                        let a448 = v6;
                        let a449 = 17u64;
                        if nat_eq(a448, a449) {
                            let a450 = v7;
                            let a451 = 0u64;
                            if nat_eq(a450, a451) {
                                if f119(v0.clone(), v4, f11(Str::lit("*"))) {
                                    f115(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                        let a452 = v4;
                                        let a453 = 1u64;
                                        nat_add(a452, a453)?
                                    })
                                } else {
                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `*` between the two types of a product"))))
                                }
                            } else {
                                Ok(f78(v2, v4, f91(f11(Str::lit("product")), f98(Str::lit("left"), match v3 {
                                    Adt7::C0(_, _, _, _, _, _, _, _, _, h454, _, _) => {
                                        h454
                                    }
                                }, Str::lit("right"), v5))?))
                            }
                        } else {
                            let a455 = v6;
                            let a456 = 18u64;
                            if nat_eq(a455, a456) {
                                let v9: Adt7 = f108(v3, v5);
                                let a457 = f75(v0.clone(), v4);
                                let a458 = 6u64;
                                if nat_eq(a457, a458) {
                                    f106(v0, v1, v2, v9, {
                                        let a459 = v4;
                                        let a460 = 1u64;
                                        nat_add(a459, a460)?
                                    })
                                } else {
                                    let a461 = f75(v0.clone(), v4);
                                    let a462 = 3u64;
                                    if nat_eq(a461, a462) {
                                        Ok(f78(v2, {
                                            let a463 = v4;
                                            let a464 = 1u64;
                                            nat_add(a463, a464)?
                                        }, f79(match v9 {
                                            Adt7::C0(_, _, _, _, _, _, _, h465, _, _, _, _) => {
                                                h465
                                            }
                                        })))
                                    } else {
                                        Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `,` or `]`"))))
                                    }
                                }
                            } else {
                                Ok(f78(v2, v4, f114(f98(Str::lit("name"), match v3 {
                                    Adt7::C0(_, _, _, _, _, _, _, _, _, h466, _, _) => {
                                        h466
                                    }
                                }, Str::lit("type"), v5))?))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f64(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7) -> R<Adt6> {
    let v4: u64 = match v2.clone() {
        Adt6::C0(h467, _, _, _, _, _, _, _, _) => {
            h467
        }
    };
    let v5: Adt0 = match v2.clone() {
        Adt6::C0(_, _, h468, _, _, _, _, _, _) => {
            h468
        }
    };
    let v6: u64 = match v3.clone() {
        Adt7::C0(h469, _, _, _, _, _, _, _, _, _, _, _) => {
            h469
        }
    };
    let v7: u64 = match v3.clone() {
        Adt7::C0(_, h470, _, _, _, _, _, _, _, _, _, _) => {
            h470
        }
    };
    let a471 = v6;
    let a472 = 30u64;
    if nat_eq(a471, a472) {
        let a473 = f75(v0.clone(), v4);
        let a474 = 5u64;
        if nat_eq(a473, a474) {
            Ok(f78(v2, {
                let a475 = v4;
                let a476 = 1u64;
                nat_add(a475, a476)?
            }, v5))
        } else {
            Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `)`"))))
        }
    } else {
        let a477 = v6;
        let a478 = 31u64;
        if nat_eq(a477, a478) {
            Ok(f78(v2, v4, f91(match v3 {
                Adt7::C0(_, _, _, _, h479, _, _, _, _, _, _, _) => {
                    h479
                }
            }, f94(Str::lit("value"), v5))?))
        } else {
            let a480 = v6;
            let a481 = 32u64;
            if nat_eq(a480, a481) {
                let a482 = v7;
                let a483 = 0u64;
                if nat_eq(a482, a483) {
                    f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), v4)
                } else {
                    Ok(f78(v2, v4, f91(f11(Str::lit("cons")), f98(Str::lit("head"), match v3 {
                        Adt7::C0(_, _, _, _, _, _, _, _, _, h484, _, _) => {
                            h484
                        }
                    }, Str::lit("tail"), v5))?))
                }
            } else {
                let a485 = v6;
                let a486 = 33u64;
                if nat_eq(a485, a486) {
                    Ok(f78(v2, v4, f91(f11(Str::lit("nil")), f94(Str::lit("element"), v5))?))
                } else {
                    let a487 = v6;
                    let a488 = 34u64;
                    if nat_eq(a487, a488) {
                        let a489 = v7;
                        let a490 = 0u64;
                        if nat_eq(a489, a490) {
                            if f119(v0.clone(), v4, f11(Str::lit("then"))) {
                                f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                    let a491 = v4;
                                    let a492 = 1u64;
                                    nat_add(a491, a492)?
                                })
                            } else {
                                Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `then`"))))
                            }
                        } else {
                            let a493 = v7;
                            let a494 = 1u64;
                            if nat_eq(a493, a494) {
                                if f119(v0.clone(), v4, f11(Str::lit("else"))) {
                                    f120(v2, f116(f104(v3.clone(), match v3 {
                                        Adt7::C0(_, _, _, _, _, _, _, _, _, h495, _, _) => {
                                            h495
                                        }
                                    }, v5), 2u64), {
                                        let a496 = v4;
                                        let a497 = 1u64;
                                        nat_add(a496, a497)?
                                    })
                                } else {
                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `else`"))))
                                }
                            } else {
                                Ok(f78(v2, v4, f91(f11(Str::lit("if")), List::cons((f11(Str::lit("else_value")), v5), f98(Str::lit("condition"), match v3.clone() {
                                    Adt7::C0(_, _, _, _, _, _, _, _, _, h498, _, _) => {
                                        h498
                                    }
                                }, Str::lit("then_value"), match v3 {
                                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, h499, _) => {
                                        h499
                                    }
                                })))?))
                            }
                        }
                    } else {
                        let a500 = v6;
                        let a501 = 35u64;
                        if nat_eq(a500, a501) {
                            let a502 = v7;
                            let a503 = 0u64;
                            if nat_eq(a502, a503) {
                                let a504 = f75(v0.clone(), v4);
                                let a505 = 8u64;
                                if nat_eq(a504, a505) {
                                    f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                        let a506 = v4;
                                        let a507 = 1u64;
                                        nat_add(a506, a507)?
                                    })
                                } else {
                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `:=`"))))
                                }
                            } else {
                                let a508 = v7;
                                let a509 = 1u64;
                                if nat_eq(a508, a509) {
                                    let a510 = f75(v0.clone(), v4);
                                    let a511 = 10u64;
                                    if nat_eq(a510, a511) {
                                        f120(v2, f116(f104(v3.clone(), match v3 {
                                            Adt7::C0(_, _, _, _, _, _, _, _, _, h512, _, _) => {
                                                h512
                                            }
                                        }, v5), 2u64), {
                                            let a513 = v4;
                                            let a514 = 1u64;
                                            nat_add(a513, a514)?
                                        })
                                    } else {
                                        Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `;`"))))
                                    }
                                } else {
                                    Ok(f78(v2, v4, f91(f11(Str::lit("let")), List::cons((f11(Str::lit("body")), v5), f98(Str::lit("binder"), match v3.clone() {
                                        Adt7::C0(_, _, _, _, _, _, _, _, _, h515, _, _) => {
                                            h515
                                        }
                                    }, Str::lit("value"), match v3 {
                                        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h516, _) => {
                                            h516
                                        }
                                    })))?))
                                }
                            }
                        } else {
                            let a517 = v6;
                            let a518 = 36u64;
                            if nat_eq(a517, a518) {
                                let a519 = v7;
                                let a520 = 0u64;
                                if nat_eq(a519, a520) {
                                    let a521 = f75(v0.clone(), v4);
                                    let a522 = 5u64;
                                    if nat_eq(a521, a522) {
                                        let a525 = f75(v0.clone(), {
                                            let a523 = v4;
                                            let a524 = 1u64;
                                            nat_add(a523, a524)?
                                        });
                                        let a526 = 6u64;
                                        if nat_eq(a525, a526) {
                                            f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                                let a527 = v4;
                                                let a528 = 2u64;
                                                nat_add(a527, a528)?
                                            })
                                        } else {
                                            Ok(f73(v0, v1, v2, {
                                                let a529 = v4;
                                                let a530 = 1u64;
                                                nat_add(a529, a530)?
                                            }, f11(Str::lit("expected `,` before the body"))))
                                        }
                                    } else {
                                        Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `)`"))))
                                    }
                                } else {
                                    Ok(f78(v2, v4, f91(f11(Str::lit("forall")), f98(Str::lit("binder"), match v3 {
                                        Adt7::C0(_, _, _, _, _, _, _, _, _, h531, _, _) => {
                                            h531
                                        }
                                    }, Str::lit("body"), v5))?))
                                }
                            } else {
                                let a532 = v6;
                                let a533 = 37u64;
                                if nat_eq(a532, a533) {
                                    let a534 = v7;
                                    let a535 = 0u64;
                                    if nat_eq(a534, a535) {
                                        let a536 = f75(v0.clone(), v4);
                                        let a537 = 5u64;
                                        if nat_eq(a536, a537) {
                                            f121(v0, v1, v2, f108(v3, v5), {
                                                let a538 = v4;
                                                let a539 = 1u64;
                                                nat_add(a538, a539)?
                                            })
                                        } else {
                                            Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `)`"))))
                                        }
                                    } else {
                                        Ok(f78(v2, v4, f91(f11(Str::lit("lambda")), List::cons((f11(Str::lit("body")), v5), f98(Str::lit("parameters"), f79(match v3.clone() {
                                            Adt7::C0(_, _, _, _, _, _, _, h540, _, _, _, _) => {
                                                h540
                                            }
                                        }), Str::lit("captures"), match v3 {
                                            Adt7::C0(_, _, _, _, _, _, _, _, _, h541, _, _) => {
                                                h541
                                            }
                                        })))?))
                                    }
                                } else {
                                    let a542 = v6;
                                    let a543 = 38u64;
                                    if nat_eq(a542, a543) {
                                        let a544 = v7;
                                        let a545 = 0u64;
                                        if nat_eq(a544, a545) {
                                            f122(v0, v1, v2, f116(f104(v3, v5, Adt0::C5), 1u64), v4)
                                        } else {
                                            f122(v0, v1, v2, f108(v3, v5), v4)
                                        }
                                    } else {
                                        let a546 = v6;
                                        let a547 = 39u64;
                                        if nat_eq(a546, a547) {
                                            let v8: Adt12 = f103(v0, v1, v4, f11(Str::lit("a field name")))?;
                                            let m549 = match v8.clone() {
                                                Adt12::C0(_, _, h548) => {
                                                    h548
                                                }
                                            };
                                            match m549 {
                                                None => {
                                                    Ok(f78(v2, match v8.clone() {
                                                        Adt12::C0(_, h551, _) => {
                                                            h551
                                                        }
                                                    }, f91(f11(Str::lit("project")), f98(Str::lit("value"), v5, Str::lit("field"), match v8 {
                                                        Adt12::C0(h552, _, _) => {
                                                            h552
                                                        }
                                                    }))?))
                                                }
                                                Some(_) => {
                                                    Ok(f77(v2, match v8 {
                                                        Adt12::C0(_, _, h550) => {
                                                            h550
                                                        }
                                                    }))
                                                }
                                            }
                                        } else {
                                            let a553 = v6;
                                            let a554 = 40u64;
                                            if if nat_eq(a553, a554) {
                                                true
                                            } else {
                                                let a555 = v6;
                                                let a556 = 44u64;
                                                if nat_eq(a555, a556) {
                                                    true
                                                } else {
                                                    let a557 = v6;
                                                    let a558 = 46u64;
                                                    nat_eq(a557, a558)
                                                }
                                            } {
                                                let a559 = v7;
                                                let a560 = 0u64;
                                                if nat_eq(a559, a560) {
                                                    f123(v0, v1, v2, v3)
                                                } else {
                                                    f122(v0, v1, v2, f108(v3, v5), v4)
                                                }
                                            } else {
                                                let a561 = v6;
                                                let a562 = 41u64;
                                                if nat_eq(a561, a562) {
                                                    f123(v0, v1, v2, v3)
                                                } else {
                                                    let a563 = v6;
                                                    let a564 = 42u64;
                                                    if nat_eq(a563, a564) {
                                                        let a565 = v7;
                                                        let a566 = 0u64;
                                                        if nat_eq(a565, a566) {
                                                            f123(v0, v1, v2, v3)
                                                        } else {
                                                            let v10: Adt7 = f108(v3.clone(), f114(f98(Str::lit("field"), f89(match v3 {
                                                                Adt7::C0(_, _, _, _, h567, _, _, _, _, _, _, _) => {
                                                                    h567
                                                                }
                                                            }), Str::lit("value"), v5))?);
                                                            let a568 = f75(v0.clone(), v4);
                                                            let a569 = 6u64;
                                                            if nat_eq(a568, a569) {
                                                                f124(v0, v1, v2, v10, {
                                                                    let a570 = v4;
                                                                    let a571 = 1u64;
                                                                    nat_add(a570, a571)?
                                                                })
                                                            } else {
                                                                let a572 = f75(v0.clone(), v4);
                                                                let a573 = 1u64;
                                                                if nat_eq(a572, a573) {
                                                                    f124(v0, v1, v2, v10, v4)
                                                                } else {
                                                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `,` or `}`"))))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        let a574 = v6;
                                                        let a575 = 43u64;
                                                        if nat_eq(a574, a575) {
                                                            let a576 = v7;
                                                            let a577 = 0u64;
                                                            if nat_eq(a576, a577) {
                                                                f125(v0, v1, v2, f108(v3, v5), v4)
                                                            } else {
                                                                Ok(f78(v2, v4, f91(f11(Str::lit("primitive")), List::cons((f11(Str::lit("result")), v5), f98(Str::lit("operation"), match v3.clone() {
                                                                    Adt7::C0(_, _, _, _, _, _, _, _, _, h578, _, _) => {
                                                                        h578
                                                                    }
                                                                }, Str::lit("arguments"), f79(match v3 {
                                                                    Adt7::C0(_, _, _, _, _, _, _, h579, _, _, _, _) => {
                                                                        h579
                                                                    }
                                                                }))))?))
                                                            }
                                                        } else {
                                                            let a580 = v6;
                                                            let a581 = 45u64;
                                                            if nat_eq(a580, a581) {
                                                                let a582 = v7;
                                                                let a583 = 0u64;
                                                                if nat_eq(a582, a583) {
                                                                    f126(v0, v1, v2, f116(f104(v3, v5, Adt0::C5), 2u64), v4)
                                                                } else {
                                                                    let a584 = v7;
                                                                    let a585 = 1u64;
                                                                    if nat_eq(a584, a585) {
                                                                        f126(v0, v1, v2, f116(f108(v3.clone(), f114(List::cons((f11(Str::lit("body")), v5), match v3 {
                                                                            Adt7::C0(_, _, _, _, _, _, _, _, h586, _, _, _) => {
                                                                                h586
                                                                            }
                                                                        }))?), 2u64), v4)
                                                                    } else {
                                                                        f126(v0, v1, v2, v3, v4)
                                                                    }
                                                                }
                                                            } else {
                                                                let a587 = v7;
                                                                let a588 = 0u64;
                                                                if nat_eq(a587, a588) {
                                                                    let a589 = f75(v0.clone(), v4);
                                                                    let a590 = 6u64;
                                                                    if nat_eq(a589, a590) {
                                                                        f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                                                            let a591 = v4;
                                                                            let a592 = 1u64;
                                                                            nat_add(a591, a592)?
                                                                        })
                                                                    } else {
                                                                        let m593 = f127(f84(v0.clone(), v4));
                                                                        match m593 {
                                                                            None => {
                                                                                Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected an operator or `,` after the first operand"))))
                                                                            }
                                                                            Some(v11) => {
                                                                                let a594 = f75(v0.clone(), v4);
                                                                                let a595 = 12u64;
                                                                                if nat_eq(a594, a595) {
                                                                                    f120(v2, f110(f116(f104(v3, v5, Adt0::C5), 2u64), v11, 0u64), {
                                                                                        let a596 = v4;
                                                                                        let a597 = 1u64;
                                                                                        nat_add(a596, a597)?
                                                                                    })
                                                                                } else {
                                                                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected an operator or `,` after the first operand"))))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    let a598 = v7;
                                                                    let a599 = 1u64;
                                                                    if nat_eq(a598, a599) {
                                                                        Ok(f78(v2, v4, f91(f11(Str::lit("pair")), f98(Str::lit("left"), match v3 {
                                                                            Adt7::C0(_, _, _, _, _, _, _, _, _, h600, _, _) => {
                                                                                h600
                                                                            }
                                                                        }, Str::lit("right"), v5))?))
                                                                    } else {
                                                                        let v12: Bytes = match v3.clone() {
                                                                            Adt7::C0(_, _, _, _, h601, _, _, _, _, _, _, _) => {
                                                                                h601
                                                                            }
                                                                        };
                                                                        let v13: bool = f36(v12.clone(), f11(Str::lit("implies")));
                                                                        Ok(f78(v2, v4, f91(v12, List::cons((if v13 {
                                                                            f11(Str::lit("premise"))
                                                                        } else {
                                                                            f11(Str::lit("left"))
                                                                        }, match v3 {
                                                                            Adt7::C0(_, _, _, _, _, _, _, _, _, h602, _, _) => {
                                                                                h602
                                                                            }
                                                                        }), List::cons((if v13 {
                                                                            f11(Str::lit("conclusion"))
                                                                        } else {
                                                                            f11(Str::lit("right"))
                                                                        }, v5), List::<(Bytes, Adt0)>::nil())))?))
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

pub fn f65(v0: u8, v1: u8, v2: u8) -> bool {
    let m605 = {
        let a603 = v0;
        let a604 = v1;
        fixed_u8::checked_sub(a603, a604)
    };
    match m605 {
        None => {
            false
        }
        Some(_) => {
            let m608 = {
                let a606 = v2;
                let a607 = v0;
                fixed_u8::checked_sub(a606, a607)
            };
            match m608 {
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

pub fn f66(v0: Fn6, p613: u64, p614: Adt9) -> R<(Adt9, bool)> {
    let m612 = (p613, p614);
    {
        let mut m612 = m612;
        loop {
            let step = {
                let (v1, v2) = m612;
                let m609 = v1;
                if m609 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m609 - 1;
                    let m611 = {
                        let c610: Fn6 = v0.clone();
                        c610.apply(v2.clone())?
                    };
                    match m611 {
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
                Ok(next) => { m612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: u64, v1: Bytes, v2: Adt9) -> R<Option<Adt9>> {
    f128(v1, v0, v2)
}

pub fn f68(v0: List<Bytes>) -> Bytes {
    let m617 = {
        let (h615, _) = f129(Fn8::F130, {
            let a616 = v0.clone();
            length_list(a616)
        }, v0);
        h615
    };
    match m617.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f69(v0: List<Bytes>) -> List<Bytes> {
    f131(Fn9::F132, List::<Bytes>::nil(), v0)
}

pub fn f70(v0: Fn7, p622: u64, p623: u64) -> R<(u64, bool)> {
    let m621 = (p622, p623);
    {
        let mut m621 = m621;
        loop {
            let step = {
                let (v1, v2) = m621;
                let m618 = v1;
                if m618 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m618 - 1;
                    let m620 = {
                        let c619: Fn7 = v0.clone();
                        c619.apply(v2)?
                    };
                    match m620 {
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
                Ok(next) => { m621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f133(v1, v0, v2)
}

pub fn f72(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    let v2: u64 = match v1.clone() {
        Adt10::C0(h624, _) => {
            h624
        }
    };
    let m625 = f41(v0, v2);
    match m625 {
        None => {
            Ok(None::<Adt10>)
        }
        Some(v3) => {
            Ok(Some(Adt10::C0({
                let a626 = v2;
                let a627 = 1u64;
                nat_add(a626, a627)?
            }, {
                let a674 = {
                    let v4: u8 = v3;
                    let a672 = {
                        let a648 = {
                            let a636 = {
                                let a630 = {
                                    let a628 = v4;
                                    let a629 = 128u8;
                                    fixed_u8::bit_and(a628, a629)
                                };
                                let a631 = 128u8;
                                if equal(a630, a631) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a637 = {
                                let a634 = {
                                    let a632 = v4;
                                    let a633 = 64u8;
                                    fixed_u8::bit_and(a632, a633)
                                };
                                let a635 = 64u8;
                                if equal(a634, a635) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a636, a637)?
                        };
                        let a649 = {
                            let a646 = {
                                let a640 = {
                                    let a638 = v4;
                                    let a639 = 32u8;
                                    fixed_u8::bit_and(a638, a639)
                                };
                                let a641 = 32u8;
                                if equal(a640, a641) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a647 = {
                                let a644 = {
                                    let a642 = v4;
                                    let a643 = 16u8;
                                    fixed_u8::bit_and(a642, a643)
                                };
                                let a645 = 16u8;
                                if equal(a644, a645) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a646, a647)?
                        };
                        nat_add(a648, a649)?
                    };
                    let a673 = {
                        let a670 = {
                            let a658 = {
                                let a652 = {
                                    let a650 = v4;
                                    let a651 = 8u8;
                                    fixed_u8::bit_and(a650, a651)
                                };
                                let a653 = 8u8;
                                if equal(a652, a653) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a659 = {
                                let a656 = {
                                    let a654 = v4;
                                    let a655 = 4u8;
                                    fixed_u8::bit_and(a654, a655)
                                };
                                let a657 = 4u8;
                                if equal(a656, a657) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a658, a659)?
                        };
                        let a671 = {
                            let a668 = {
                                let a662 = {
                                    let a660 = v4;
                                    let a661 = 2u8;
                                    fixed_u8::bit_and(a660, a661)
                                };
                                let a663 = 2u8;
                                if equal(a662, a663) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a669 = {
                                let a666 = {
                                    let a664 = v4;
                                    let a665 = 1u8;
                                    fixed_u8::bit_and(a664, a665)
                                };
                                let a667 = 1u8;
                                if equal(a666, a667) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a668, a669)?
                        };
                        nat_add(a670, a671)?
                    };
                    nat_add(a672, a673)?
                };
                let a675 = 47u64;
                if nat_eq(a674, a675) {
                    let a676 = v2;
                    let a677 = 1u64;
                    nat_add(a676, a677)?
                } else {
                    match v1 {
                        Adt10::C0(_, h678) => {
                            h678
                        }
                    }
                }
            })))
        }
    }
}

pub fn f73(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: Bytes) -> Adt6 {
    f113(v2, f111(v0, v1, v3), v4)
}

pub fn f74(v0: Adt6, v1: u64, v2: u64, v3: Adt2, v4: u64) -> Adt6 {
    Adt6::C0(v1, match v0.clone() {
        Adt6::C0(_, h679, _, _, _, _, _, _, _) => {
            h679
        }
    }, Adt0::C5, 0u64, v2, v3, v4, None::<Adt1>, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h680) => {
            h680
        }
    })
}

pub fn f75(v0: List<Adt3>, v1: u64) -> u64 {
    f134(v0, v1)
}

pub fn f76(v0: List<Adt3>, v1: u64, v2: u64) -> R<Adt12> {
    let v3: Adt12 = f103(v0.clone(), v1, v2, f11(Str::lit("a name")))?;
    let m682 = match v3.clone() {
        Adt12::C0(_, _, h681) => {
            h681
        }
    };
    match m682 {
        None => {
            let v5: u64 = match v3.clone() {
                Adt12::C0(_, h683, _) => {
                    h683
                }
            };
            let a684 = f75(v0.clone(), v5);
            let a685 = 9u64;
            if nat_eq(a684, a685) {
                let v6: Adt12 = f103(v0, v1, {
                    let a686 = v5;
                    let a687 = 1u64;
                    nat_add(a686, a687)?
                }, f11(Str::lit("a name after `::`")))?;
                let m689 = match v6.clone() {
                    Adt12::C0(_, _, h688) => {
                        h688
                    }
                };
                match m689 {
                    None => {
                        Ok(Adt12::C0(f114(f98(Str::lit("module"), match v3 {
                            Adt12::C0(h690, _, _) => {
                                h690
                            }
                        }, Str::lit("name"), match v6.clone() {
                            Adt12::C0(h691, _, _) => {
                                h691
                            }
                        }))?, match v6 {
                            Adt12::C0(_, h692, _) => {
                                h692
                            }
                        }, None::<Adt1>))
                    }
                    Some(_) => {
                        Ok(v6)
                    }
                }
            } else {
                Ok(Adt12::C0(f114(f94(Str::lit("name"), match v3 {
                    Adt12::C0(h693, _, _) => {
                        h693
                    }
                }))?, v5, None::<Adt1>))
            }
        }
        Some(_) => {
            Ok(v3)
        }
    }
}

pub fn f77(v0: Adt6, v1: Option<Adt1>) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h694, _, _, _, _, _, _, _, _) => {
            h694
        }
    }, match v0.clone() {
        Adt6::C0(_, h695, _, _, _, _, _, _, _) => {
            h695
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h696, _, _, _, _, _, _) => {
            h696
        }
    }, 9u64, match v0.clone() {
        Adt6::C0(_, _, _, _, h697, _, _, _, _) => {
            h697
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h698, _, _, _) => {
            h698
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h699, _, _) => {
            h699
        }
    }, v1, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h700) => {
            h700
        }
    })
}

pub fn f78(v0: Adt6, v1: u64, v2: Adt0) -> Adt6 {
    Adt6::C0(v1, match v0.clone() {
        Adt6::C0(_, h701, _, _, _, _, _, _, _) => {
            h701
        }
    }, v2, 1u64, match v0.clone() {
        Adt6::C0(_, _, _, _, h702, _, _, _, _) => {
            h702
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h703, _, _, _) => {
            h703
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h704, _, _) => {
            h704
        }
    }, None::<Adt1>, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h705) => {
            h705
        }
    })
}

pub fn f79(v0: List<Adt0>) -> Adt0 {
    Adt0::C3(std::rc::Rc::new(f135(v0)))
}

pub fn f80(v0: Adt0, v1: Adt2) -> R<Adt2> {
    let m706 = v1;
    match m706 {
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
            f12(v0, v2)
        }
        Adt2::C5 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f81(v0: Adt6, v1: Adt7, v2: u64, v3: u64, v4: Adt2, v5: u64) -> Adt6 {
    Adt6::C0(v2, List::cons(v1, match v0.clone() {
        Adt6::C0(_, h707, _, _, _, _, _, _, _) => {
            h707
        }
    }), Adt0::C5, 0u64, v3, v4, v5, None::<Adt1>, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h708) => {
            h708
        }
    })
}

pub fn f82(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6, v4: Adt7, v5: u64) -> R<Adt6> {
    let v6: u64 = f75(v0.clone(), v5);
    let a709 = v6;
    let a710 = 1u64;
    if nat_eq(a709, a710) {
        Ok(f78(v3, {
            let a711 = v5;
            let a712 = 1u64;
            nat_add(a711, a712)?
        }, f114(match v4 {
            Adt7::C0(_, _, _, _, _, _, _, _, h713, _, _, _) => {
                h713
            }
        })?))
    } else {
        let v7: u64 = f111(v0.clone(), v1, v5);
        let v8: Bytes = f84(v0.clone(), v5);
        let a714 = v6;
        let a715 = 11u64;
        if if nat_eq(a714, a715) {
            true
        } else {
            f85({
                let a716 = v6;
                let a717 = 12u64;
                nat_eq(a716, a717)
            }, f86(v8.clone())?)
        } {
            let a720 = f75(v0.clone(), {
                let a718 = v5;
                let a719 = 1u64;
                nat_add(a718, a719)?
            });
            let a721 = 7u64;
            if nat_eq(a720, a721) {
                Ok(f81(v3, f110(v4.clone(), v8.clone(), v7), {
                    let a722 = v5;
                    let a723 = 2u64;
                    nat_add(a722, a723)?
                }, 0u64, f136(v2, match v4.clone() {
                    Adt7::C0(_, _, _, _, _, _, h724, _, _, _, _, _) => {
                        h724
                    }
                }, v8)?, {
                    let a726 = match v4 {
                        Adt7::C0(_, _, h725, _, _, _, _, _, _, _, _, _) => {
                            h725
                        }
                    };
                    let a727 = 1u64;
                    nat_add(a726, a727)?
                }))
            } else {
                Ok(f73(v0, v1, v3, {
                    let a728 = v5;
                    let a729 = 1u64;
                    nat_add(a728, a729)?
                }, f11(Str::lit("expected `:` after a member name"))))
            }
        } else {
            Ok(f113(v3, v7, f11(Str::lit("expected a member name"))))
        }
    }
}

pub fn f83(v0: Adt0, v1: Adt2) -> Option<Adt0> {
    let m730 = v1;
    match m730 {
        Adt2::C0 => {
            None::<Adt0>
        }
        Adt2::C1 => {
            None::<Adt0>
        }
        Adt2::C2 => {
            None::<Adt0>
        }
        Adt2::C3 => {
            f13(v0, f11(Str::lit("parameter")))
        }
        Adt2::C4(v2) => {
            Some(v2)
        }
        Adt2::C5 => {
            None::<Adt0>
        }
    }
}

pub fn f84(v0: List<Adt3>, v1: u64) -> Bytes {
    f137(v0, v1)
}

pub fn f85(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f86(v0: Bytes) -> R<bool> {
    let a732 = {
        let a731 = v0.clone();
        length_bytes(a731)
    };
    let a733 = 0u64;
    if nat_eq(a732, a733) {
        Ok(false)
    } else {
        if f138(f139(v0.clone(), 0u64)?) {
            let a735 = f140(v0.clone(), 1u64)?;
            let a736 = {
                let a734 = v0.clone();
                length_bytes(a734)
            };
            if nat_eq(a735, a736) {
                let a737 = f141(v0);
                Ok(bool_not(a737))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f87(v0: Bytes) -> R<bool> {
    Ok(f85({
        let a739 = 0u64;
        let a740 = {
            let a738 = v0.clone();
            length_bytes(a738)
        };
        nat_lt(a739, a740)
    }, {
        let a742 = f142(v0.clone(), 0u64)?;
        let a743 = {
            let a741 = v0;
            length_bytes(a741)
        };
        nat_eq(a742, a743)
    }))
}

pub fn f88(v0: Adt0, v1: Adt2, v2: Bytes) -> R<Option<Adt0>> {
    let m744 = v1;
    match m744 {
        Adt2::C0 => {
            let m745 = f13(v0.clone(), f11(Str::lit("term")));
            match m745 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v3) => {
                    f143(v0, v3, v2)
                }
            }
        }
        Adt2::C1 => {
            let m746 = f13(v0.clone(), f11(Str::lit("type")));
            match m746 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    f143(v0, v4, v2)
                }
            }
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(None::<Adt0>)
        }
        Adt2::C4(v5) => {
            if f144(v0.clone(), v5.clone())? {
                f143(v0, v5, v2)
            } else {
                Ok(Some(v5))
            }
        }
        Adt2::C5 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f89(v0: Bytes) -> Adt0 {
    Adt0::C2(v0)
}

pub fn f90(v0: Bytes) -> Option<Bytes> {
    if f36(v0.clone(), f11(Str::lit("Bool"))) {
        Some(f11(Str::lit("bool")))
    } else {
        if f36(v0.clone(), f11(Str::lit("Bytes"))) {
            Some(f11(Str::lit("bytes")))
        } else {
            if f36(v0.clone(), f11(Str::lit("Int"))) {
                Some(f11(Str::lit("int")))
            } else {
                if f36(v0.clone(), f11(Str::lit("Int8"))) {
                    Some(f11(Str::lit("int8")))
                } else {
                    if f36(v0.clone(), f11(Str::lit("Int16"))) {
                        Some(f11(Str::lit("int16")))
                    } else {
                        if f36(v0.clone(), f11(Str::lit("Int32"))) {
                            Some(f11(Str::lit("int32")))
                        } else {
                            if f36(v0.clone(), f11(Str::lit("Int64"))) {
                                Some(f11(Str::lit("int64")))
                            } else {
                                if f36(v0.clone(), f11(Str::lit("Nat"))) {
                                    Some(f11(Str::lit("nat")))
                                } else {
                                    if f36(v0.clone(), f11(Str::lit("Ordering"))) {
                                        Some(f11(Str::lit("ordering")))
                                    } else {
                                        if f36(v0.clone(), f11(Str::lit("Prop"))) {
                                            Some(f11(Str::lit("prop")))
                                        } else {
                                            if f36(v0.clone(), f11(Str::lit("String"))) {
                                                Some(f11(Str::lit("string")))
                                            } else {
                                                if f36(v0.clone(), f11(Str::lit("Type"))) {
                                                    Some(f11(Str::lit("type")))
                                                } else {
                                                    if f36(v0.clone(), f11(Str::lit("UInt8"))) {
                                                        Some(f11(Str::lit("uint8")))
                                                    } else {
                                                        if f36(v0.clone(), f11(Str::lit("UInt16"))) {
                                                            Some(f11(Str::lit("uint16")))
                                                        } else {
                                                            if f36(v0.clone(), f11(Str::lit("UInt32"))) {
                                                                Some(f11(Str::lit("uint32")))
                                                            } else {
                                                                if f36(v0.clone(), f11(Str::lit("UInt64"))) {
                                                                    Some(f11(Str::lit("uint64")))
                                                                } else {
                                                                    if f36(v0.clone(), f11(Str::lit("Unit"))) {
                                                                        Some(f11(Str::lit("unit")))
                                                                    } else {
                                                                        if f36(v0.clone(), f11(Str::lit("ContractViolation"))) {
                                                                            Some(f11(Str::lit("contract_violation")))
                                                                        } else {
                                                                            if f36(v0, f11(Str::lit("ReasoningFailure"))) {
                                                                                Some(f11(Str::lit("reasoning_failure")))
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

pub fn f91(v0: Bytes, v1: List<(Bytes, Adt0)>) -> R<Adt0> {
    f114(List::cons((f11(Str::lit("kind")), Adt0::C2(v0)), v1))
}

pub fn f92(v0: Bytes, v1: Bytes) -> bool {
    let m751 = {
        let a748 = v0;
        let a749 = 0u64;
        let a750 = {
            let a747 = v1.clone();
            length_bytes(a747)
        };
        slice_bytes(a748, a749, a750)
    };
    match m751 {
        None => {
            false
        }
        Some(v2) => {
            let a752 = v2;
            let a753 = v1;
            equal(a752, a753)
        }
    }
}

pub fn f93(v0: Bytes, v1: u64) -> Bytes {
    f47(v0.clone(), v1, {
        let a755 = {
            let a754 = v0;
            length_bytes(a754)
        };
        let a756 = v1;
        nat_sub(a755, a756)
    })
}

pub fn f94(v0: Str, v1: Adt0) -> List<(Bytes, Adt0)> {
    List::cons((f11(v0), v1), List::<(Bytes, Adt0)>::nil())
}

pub fn f95(v0: Bytes, v1: List<Bytes>) -> bool {
    f145(Fn10::F146(v0), false, v1)
}

pub fn f96() -> List<Bytes> {
    List::cons(f11(Str::lit("List")), List::cons(f11(Str::lit("Option")), List::cons(f11(Str::lit("Result")), List::cons(f11(Str::lit("Set")), List::cons(f11(Str::lit("Map")), List::cons(f11(Str::lit("Fn")), List::<Bytes>::nil()))))))
}

pub fn f97(v0: Bytes, v1: Bytes) -> Bytes {
    let a757 = v0;
    let a758 = v1;
    append_bytes(a757, a758)
}

pub fn f98(v0: Str, v1: Adt0, v2: Str, v3: Adt0) -> List<(Bytes, Adt0)> {
    List::cons((f11(v0), v1), List::cons((f11(v2), v3), List::<(Bytes, Adt0)>::nil()))
}

pub fn f99(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: u64) -> R<Adt6> {
    let v5: Bytes = f84(v0.clone(), v3);
    let v6: bool = {
        let a759 = f75(v0.clone(), v3);
        let a760 = 12u64;
        nat_eq(a759, a760)
    };
    if f85(v6, if f36(v5.clone(), f11(Str::lit("List"))) {
        true
    } else {
        if f36(v5.clone(), f11(Str::lit("Set"))) {
            true
        } else {
            f36(v5.clone(), f11(Str::lit("Option")))
        }
    }) {
        Ok(f81(v2, f110(f6(12u64, v4), v5, 0u64), {
            let a761 = v3;
            let a762 = 1u64;
            nat_add(a761, a762)?
        }, 2u64, Adt2::C1, {
            let a763 = v4;
            let a764 = 1u64;
            nat_add(a763, a764)?
        }))
    } else {
        if f85(v6, if f36(v5.clone(), f11(Str::lit("Result"))) {
            true
        } else {
            f36(v5.clone(), f11(Str::lit("Map")))
        }) {
            Ok(f81(v2, f110(f6(13u64, v4), v5, 0u64), {
                let a765 = v3;
                let a766 = 1u64;
                nat_add(a765, a766)?
            }, 2u64, Adt2::C1, {
                let a767 = v4;
                let a768 = 1u64;
                nat_add(a767, a768)?
            }))
        } else {
            if f85(v6, f36(v5.clone(), f11(Str::lit("Fn")))) {
                let a771 = f75(v0.clone(), {
                    let a769 = v3;
                    let a770 = 1u64;
                    nat_add(a769, a770)?
                });
                let a772 = 4u64;
                if nat_eq(a771, a772) {
                    f117(v0, v1, v2, f6(14u64, v4), {
                        let a773 = v3;
                        let a774 = 2u64;
                        nat_add(a773, a774)?
                    })
                } else {
                    Ok(f73(v0, v1, v2, {
                        let a775 = v3;
                        let a776 = 1u64;
                        nat_add(a775, a776)?
                    }, f11(Str::lit("expected `(` before the parameter types"))))
                }
            } else {
                let v7: u64 = {
                    let a779 = f75(v0.clone(), {
                        let a777 = v3;
                        let a778 = 1u64;
                        nat_add(a777, a778)?
                    });
                    let a780 = 9u64;
                    if nat_eq(a779, a780) {
                        3u64
                    } else {
                        1u64
                    }
                };
                let v8: bool = f119(v0.clone(), {
                    let a781 = v3;
                    let a782 = v7;
                    nat_add(a781, a782)?
                }, f11(Str::lit("*")));
                if f85(v6, f85(!v8, f85(!f92(v5.clone(), f11(Str::lit("'"))), !f147(v5)))) {
                    let v9: Adt12 = f76(v0.clone(), v1, v3)?;
                    let m784 = match v9.clone() {
                        Adt12::C0(_, _, h783) => {
                            h783
                        }
                    };
                    match m784 {
                        None => {
                            f118(v0, v1, v2, f104(f6(16u64, v4), match v9.clone() {
                                Adt12::C0(h786, _, _) => {
                                    h786
                                }
                            }, Adt0::C5), match v9 {
                                Adt12::C0(_, h787, _) => {
                                    h787
                                }
                            })
                        }
                        Some(_) => {
                            Ok(f77(v2, match v9 {
                                Adt12::C0(_, _, h785) => {
                                    h785
                                }
                            }))
                        }
                    }
                } else {
                    Ok(f81(v2, f6(17u64, v4), v3, 2u64, Adt2::C1, {
                        let a788 = v4;
                        let a789 = 1u64;
                        nat_add(a788, a789)?
                    }))
                }
            }
        }
    }
}

pub fn f100(v0: List<Adt3>, v1: u64) -> R<Bytes> {
    let m790 = f148(v0, v1);
    match m790 {
        None => {
            Ok(f150(Str::lit("the end of the text")))
        }
        Some(v2) => {
            f149(match v2 {
                Adt3::C0(h791, _, _) => {
                    h791
                }
            })
        }
    }
}

pub fn f101(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: u64) -> R<Adt6> {
    let v5: u64 = f75(v0.clone(), v3);
    let v6: Bytes = f84(v0.clone(), v3);
    let a792 = v5;
    let a793 = 5u64;
    if nat_eq(a792, a793) {
        Ok(f78(v2, v3, f91(f11(Str::lit("unit")), List::<(Bytes, Adt0)>::nil())?))
    } else {
        let a794 = v5;
        let a795 = 12u64;
        if nat_eq(a794, a795) {
            let v7: Bytes = if f92(v6.clone(), f11(Str::lit("-"))) {
                f93(v6.clone(), 1u64)
            } else {
                v6.clone()
            };
            if f85(f87(v7)?, {
                let a798 = f75(v0.clone(), {
                    let a796 = v3;
                    let a797 = 1u64;
                    nat_add(a796, a797)?
                });
                let a799 = 7u64;
                nat_eq(a798, a799)
            }) {
                f151(v0, v1, v2, v3, v6)
            } else {
                let v8: bool = f85(f36(v6.clone(), f11(Str::lit("let"))), f119(v0.clone(), {
                    let a800 = v3;
                    let a801 = 2u64;
                    nat_add(a800, a801)?
                }, f11(Str::lit("name"))));
                let v9: Bytes = if f85({
                    let a804 = f75(v0.clone(), {
                        let a802 = v3;
                        let a803 = 1u64;
                        nat_add(a802, a803)?
                    });
                    let a805 = 0u64;
                    nat_eq(a804, a805)
                }, !v8) {
                    f11(Str::lit(""))
                } else {
                    v6
                };
                f152(v0, v1, v2, v3, v4, v9)
            }
        } else {
            f120(v2, f6(47u64, v4), v3)
        }
    }
}

pub fn f102(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: Bytes) -> R<Adt6> {
    let v5: u64 = f111(v0, v1, v3);
    if f36(v4.clone(), f11(Str::lit("true"))) {
        Ok(f78(v2, {
            let a806 = v3;
            let a807 = 1u64;
            nat_add(a806, a807)?
        }, f91(f11(Str::lit("bool")), f94(Str::lit("value"), Adt0::C0(true)))?))
    } else {
        if f36(v4.clone(), f11(Str::lit("false"))) {
            Ok(f78(v2, {
                let a808 = v3;
                let a809 = 1u64;
                nat_add(a808, a809)?
            }, f91(f11(Str::lit("bool")), f94(Str::lit("value"), Adt0::C0(false)))?))
        } else {
            if f87(v4.clone())? {
                if f85({
                    let a811 = 1u64;
                    let a812 = {
                        let a810 = v4.clone();
                        length_bytes(a810)
                    };
                    nat_lt(a811, a812)
                }, f92(v4.clone(), f11(Str::lit("0")))) {
                    Ok(f113(v2, v5, f97(f153(v4), f11(Str::lit(" has a leading zero")))))
                } else {
                    Ok(f78(v2, {
                        let a813 = v3;
                        let a814 = 1u64;
                        nat_add(a813, a814)?
                    }, f91(f11(Str::lit("nat")), f94(Str::lit("value"), f89(v4)))?))
                }
            } else {
                if f92(v4.clone(), f11(Str::lit("0x"))) {
                    if f154(f93(v4.clone(), 2u64))? {
                        Ok(f78(v2, {
                            let a815 = v3;
                            let a816 = 1u64;
                            nat_add(a815, a816)?
                        }, f91(f11(Str::lit("bytes")), f94(Str::lit("hex"), f89(f93(v4, 2u64))))?))
                    } else {
                        Ok(f113(v2, v5, f11(Str::lit("a byte string is `0x` and an even number of lower-case hexadecimal digits"))))
                    }
                } else {
                    if f155(v4.clone())? {
                        Ok(f78(v2, {
                            let a817 = v3;
                            let a818 = 1u64;
                            nat_add(a817, a818)?
                        }, f91(f11(Str::lit("var")), f94(Str::lit("name"), f89(v4)))?))
                    } else {
                        Ok(f113(v2, v5, if f95(v4.clone(), f156()) {
                            f97(f97(f97(f153(v4.clone()), f11(Str::lit(" is a keyword; write `var{name: "))), v4), f11(Str::lit("}` for a variable of that name")))
                        } else {
                            f97(f153(v4), f11(Str::lit(" is not a term")))
                        }))
                    }
                }
            }
        }
    }
}

pub fn f103(v0: List<Adt3>, v1: u64, v2: u64, v3: Bytes) -> R<Adt12> {
    let v4: Bytes = f84(v0.clone(), v2);
    if f85({
        let a819 = f75(v0.clone(), v2);
        let a820 = 12u64;
        nat_eq(a819, a820)
    }, f86(v4.clone())?) {
        Ok(Adt12::C0(f89(v4), {
            let a821 = v2;
            let a822 = 1u64;
            nat_add(a821, a822)?
        }, None::<Adt1>))
    } else {
        Ok(Adt12::C0(Adt0::C5, v2, f157(f111(v0.clone(), v1, v2), f97(f97(f97(f11(Str::lit("expected ")), v3), f11(Str::lit(", found "))), f100(v0, v2)?))))
    }
}

pub fn f104(v0: Adt7, v1: Adt0, v2: Adt0) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h823, _, _, _, _, _, _, _, _, _, _, _) => {
            h823
        }
    }, match v0.clone() {
        Adt7::C0(_, h824, _, _, _, _, _, _, _, _, _, _) => {
            h824
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h825, _, _, _, _, _, _, _, _, _) => {
            h825
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h826, _, _, _, _, _, _, _, _) => {
            h826
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h827, _, _, _, _, _, _, _) => {
            h827
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h828, _, _, _, _, _, _) => {
            h828
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h829, _, _, _, _, _) => {
            h829
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h830, _, _, _, _) => {
            h830
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h831, _, _, _) => {
            h831
        }
    }, v1, v2, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h832) => {
            h832
        }
    })
}

pub fn f105(v0: List<Adt3>, v1: u64, v2: u64, v3: u64) -> R<Adt13> {
    let (h833, _) = f158(Fn11::F159(v1, v0.clone()), {
        let a837 = {
            let a835 = {
                let a834 = v0;
                length_list(a834)
            };
            let a836 = 4u64;
            nat_mul(a835, a836)?
        };
        let a838 = 8u64;
        nat_add(a837, a838)?
    }, Adt13::C0(v2, v3, 0u64, List::<Adt14>::nil(), Adt0::C5, None::<Adt1>))?;
    Ok(h833)
}

pub fn f106(v0: List<Adt3>, _: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a839 = f75(v0, v4);
    let a840 = 3u64;
    if nat_eq(a839, a840) {
        Ok(f78(v2, {
            let a841 = v4;
            let a842 = 1u64;
            nat_add(a841, a842)?
        }, f79(match v3 {
            Adt7::C0(_, _, _, _, _, _, _, h843, _, _, _, _) => {
                h843
            }
        })))
    } else {
        f115(v2, v3, v4)
    }
}

pub fn f107(v0: Adt7, v1: (u64, u64), v2: u64) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h844, _, _, _, _, _, _, _, _, _, _, _) => {
            h844
        }
    }, match v0.clone() {
        Adt7::C0(_, h845, _, _, _, _, _, _, _, _, _, _) => {
            h845
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h846, _, _, _, _, _, _, _, _, _) => {
            h846
        }
    }, v2, match v0.clone() {
        Adt7::C0(_, _, _, _, h847, _, _, _, _, _, _, _) => {
            h847
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h848, _, _, _, _, _, _) => {
            h848
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h849, _, _, _, _, _) => {
            h849
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h850, _, _, _, _) => {
            h850
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h851, _, _, _) => {
            h851
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h852, _, _) => {
            h852
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h853, _) => {
            h853
        }
    }, List::cons(v1, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h854) => {
            h854
        }
    }))
}

pub fn f108(v0: Adt7, v1: Adt0) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h855, _, _, _, _, _, _, _, _, _, _, _) => {
            h855
        }
    }, match v0.clone() {
        Adt7::C0(_, h856, _, _, _, _, _, _, _, _, _, _) => {
            h856
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h857, _, _, _, _, _, _, _, _, _) => {
            h857
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h858, _, _, _, _, _, _, _, _) => {
            h858
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h859, _, _, _, _, _, _, _) => {
            h859
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h860, _, _, _, _, _, _) => {
            h860
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h861, _, _, _, _, _) => {
            h861
        }
    }, List::cons(v1, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h862, _, _, _, _) => {
            h862
        }
    }), match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h863, _, _, _) => {
            h863
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h864, _, _) => {
            h864
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h865, _) => {
            h865
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h866) => {
            h866
        }
    })
}

pub fn f109(v0: List<Adt3>, v1: u64) -> u64 {
    let a867 = v1;
    let a868 = 0u64;
    if nat_eq(a867, a868) {
        0u64
    } else {
        let m871 = f148(v0, {
            let a869 = v1;
            let a870 = 1u64;
            nat_sub(a869, a870)
        });
        match m871 {
            None => {
                0u64
            }
            Some(v2) => {
                match v2 {
                    Adt3::C0(_, _, h872) => {
                        h872
                    }
                }
            }
        }
    }
}

pub fn f110(v0: Adt7, v1: Bytes, v2: u64) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h873, _, _, _, _, _, _, _, _, _, _, _) => {
            h873
        }
    }, match v0.clone() {
        Adt7::C0(_, h874, _, _, _, _, _, _, _, _, _, _) => {
            h874
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h875, _, _, _, _, _, _, _, _, _) => {
            h875
        }
    }, v2, v1, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h876, _, _, _, _, _, _) => {
            h876
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h877, _, _, _, _, _) => {
            h877
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h878, _, _, _, _) => {
            h878
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h879, _, _, _) => {
            h879
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h880, _, _) => {
            h880
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h881, _) => {
            h881
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h882) => {
            h882
        }
    })
}

pub fn f111(v0: List<Adt3>, v1: u64, v2: u64) -> u64 {
    let m883 = f148(v0, v2);
    match m883 {
        None => {
            v1
        }
        Some(v3) => {
            match v3 {
                Adt3::C0(_, h884, _) => {
                    h884
                }
            }
        }
    }
}

pub fn f112(v0: List<(Bytes, Adt0)>, v1: Bytes) -> bool {
    f160(Fn12::F161(v1), false, v0)
}

pub fn f113(v0: Adt6, v1: u64, v2: Bytes) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h885, _, _, _, _, _, _, _, _) => {
            h885
        }
    }, match v0.clone() {
        Adt6::C0(_, h886, _, _, _, _, _, _, _) => {
            h886
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h887, _, _, _, _, _, _) => {
            h887
        }
    }, 9u64, match v0.clone() {
        Adt6::C0(_, _, _, _, h888, _, _, _, _) => {
            h888
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h889, _, _, _) => {
            h889
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h890, _, _) => {
            h890
        }
    }, f157(v1, v2), match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h891) => {
            h891
        }
    })
}

pub fn f114(v0: List<(Bytes, Adt0)>) -> R<Adt0> {
    Ok(Adt0::C4(std::rc::Rc::new(f162(f163(v0)?))))
}

pub fn f115(v0: Adt6, v1: Adt7, v2: u64) -> R<Adt6> {
    Ok(f81(v0, v1.clone(), v2, 2u64, Adt2::C1, {
        let a893 = match v1 {
            Adt7::C0(_, _, h892, _, _, _, _, _, _, _, _, _) => {
                h892
            }
        };
        let a894 = 1u64;
        nat_add(a893, a894)?
    }))
}

pub fn f116(v0: Adt7, v1: u64) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h895, _, _, _, _, _, _, _, _, _, _, _) => {
            h895
        }
    }, v1, match v0.clone() {
        Adt7::C0(_, _, h896, _, _, _, _, _, _, _, _, _) => {
            h896
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h897, _, _, _, _, _, _, _, _) => {
            h897
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h898, _, _, _, _, _, _, _) => {
            h898
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h899, _, _, _, _, _, _) => {
            h899
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h900, _, _, _, _, _) => {
            h900
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h901, _, _, _, _) => {
            h901
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h902, _, _, _) => {
            h902
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h903, _, _) => {
            h903
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h904, _) => {
            h904
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h905) => {
            h905
        }
    })
}

pub fn f117(v0: List<Adt3>, _: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a906 = f75(v0, v4);
    let a907 = 5u64;
    if nat_eq(a906, a907) {
        f115(v2, f116(v3, 1u64), {
            let a908 = v4;
            let a909 = 1u64;
            nat_add(a908, a909)?
        })
    } else {
        f115(v2, v3, v4)
    }
}

pub fn f118(v0: List<Adt3>, _: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a910 = f75(v0, v4);
    let a911 = 5u64;
    if nat_eq(a910, a911) {
        Ok(f78(v2, v4, f91(f11(Str::lit("named")), f98(Str::lit("member"), match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h912, _, _) => {
                h912
            }
        }, Str::lit("arguments"), f79(match v3 {
            Adt7::C0(_, _, _, _, _, _, _, h913, _, _, _, _) => {
                h913
            }
        })))?))
    } else {
        f115(v2, v3, v4)
    }
}

pub fn f119(v0: List<Adt3>, v1: u64, v2: Bytes) -> bool {
    f85({
        let a914 = f75(v0.clone(), v1);
        let a915 = 12u64;
        nat_eq(a914, a915)
    }, f36(f84(v0, v1), v2))
}

pub fn f120(v0: Adt6, v1: Adt7, v2: u64) -> R<Adt6> {
    Ok(f81(v0, v1.clone(), v2, 3u64, Adt2::C0, {
        let a917 = match v1 {
            Adt7::C0(_, _, h916, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a918 = 1u64;
        nat_add(a917, a918)?
    }))
}

pub fn f121(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a919 = f75(v0.clone(), v4);
    let a920 = 4u64;
    if nat_eq(a919, a920) {
        f164(v2, v3, {
            let a921 = v4;
            let a922 = 1u64;
            nat_add(a921, a922)?
        })
    } else {
        let m924 = match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h923, _, _, _, _) => {
                h923
            }
        };
        match m924.uncons() {
            None => {
                Ok(f73(v0, v1, v2, v4, f11(Str::lit("a function takes at least one parameter"))))
            }
            Some((_, _)) => {
                if f119(v0.clone(), v4, f11(Str::lit("=>"))) {
                    f120(v2, f116(v3, 1u64), {
                        let a925 = v4;
                        let a926 = 1u64;
                        nat_add(a925, a926)?
                    })
                } else {
                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `=>`"))))
                }
            }
        }
    }
}

pub fn f122(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let v5: Bytes = match v3.clone() {
        Adt7::C0(_, _, _, _, h927, _, _, _, _, _, _, _) => {
            h927
        }
    };
    let a929 = {
        let a928 = v5.clone();
        length_bytes(a928)
    };
    let a930 = 0u64;
    if if nat_eq(a929, a930) {
        let a931 = f75(v0.clone(), v4);
        let a932 = 5u64;
        nat_eq(a931, a932)
    } else {
        f119(v0.clone(), v4, v5)
    } {
        f165(v0, v1, v2, v3, v4)
    } else {
        let a933 = f75(v0, v4);
        let a934 = 99u64;
        if nat_eq(a933, a934) {
            Ok(f113(v2, v1, f11(Str::lit("expected `)`"))))
        } else {
            f120(v2, v3, v4)
        }
    }
}

pub fn f123(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7) -> R<Adt6> {
    let v4: u64 = match v2.clone() {
        Adt6::C0(h935, _, _, _, _, _, _, _, _) => {
            h935
        }
    };
    let v5: u64 = match v3.clone() {
        Adt7::C0(h936, _, _, _, _, _, _, _, _, _, _, _) => {
            h936
        }
    };
    let v6: Adt7 = f104(v3.clone(), match v3.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h937, _, _) => {
            h937
        }
    }, match v2.clone() {
        Adt6::C0(_, _, h938, _, _, _, _, _, _) => {
            h938
        }
    });
    let a939 = v5;
    let a940 = 41u64;
    if nat_eq(a939, a940) {
        Ok(f78(v2.clone(), v4, f91(f11(Str::lit("function_ref")), f166(f94(Str::lit("function"), match v3 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h941, _, _) => {
                h941
            }
        }), match v2 {
            Adt6::C0(_, _, h942, _, _, _, _, _, _) => {
                h942
            }
        }))?))
    } else {
        let a943 = v5;
        let a944 = 42u64;
        if nat_eq(a943, a944) {
            let a945 = f75(v0.clone(), v4);
            let a946 = 0u64;
            if nat_eq(a945, a946) {
                f124(v0, v1, v2, f116(v6, 1u64), {
                    let a947 = v4;
                    let a948 = 1u64;
                    nat_add(a947, a948)?
                })
            } else {
                Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `{` before the fields"))))
            }
        } else {
            f122(v0, v1, v2, f116(v6, 1u64), v4)
        }
    }
}

pub fn f124(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a949 = f75(v0.clone(), v4);
    let a950 = 1u64;
    if nat_eq(a949, a950) {
        Ok(f78(v2, {
            let a951 = v4;
            let a952 = 1u64;
            nat_add(a951, a952)?
        }, f91(f11(Str::lit("record")), f166(f98(Str::lit("type"), match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h953, _, _) => {
                h953
            }
        }, Str::lit("fields"), f79(match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h954, _, _, _, _) => {
                h954
            }
        })), match v3 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h955, _) => {
                h955
            }
        }))?))
    } else {
        let v5: Adt12 = f103(v0.clone(), v1, v4, f11(Str::lit("a field name")))?;
        let m957 = match v5.clone() {
            Adt12::C0(_, _, h956) => {
                h956
            }
        };
        match m957 {
            None => {
                let a960 = f75(v0.clone(), match v5.clone() {
                    Adt12::C0(_, h959, _) => {
                        h959
                    }
                });
                let a961 = 8u64;
                if nat_eq(a960, a961) {
                    f120(v2, f110(v3, {
                        let m963 = f50(match v5.clone() {
                            Adt12::C0(h962, _, _) => {
                                h962
                            }
                        });
                        match m963 {
                            None => {
                                f11(Str::lit(""))
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, 0u64), {
                        let a965 = match v5 {
                            Adt12::C0(_, h964, _) => {
                                h964
                            }
                        };
                        let a966 = 1u64;
                        nat_add(a965, a966)?
                    })
                } else {
                    Ok(f73(v0, v1, v2, match v5 {
                        Adt12::C0(_, h967, _) => {
                            h967
                        }
                    }, f11(Str::lit("expected `:=` after a field name"))))
                }
            }
            Some(_) => {
                Ok(f77(v2, match v5 {
                    Adt12::C0(_, _, h958) => {
                        h958
                    }
                }))
            }
        }
    }
}

pub fn f125(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let v5: u64 = f75(v0.clone(), v4);
    let a968 = v5;
    let a969 = 7u64;
    if nat_eq(a968, a969) {
        f115(v2, f116(v3, 1u64), {
            let a970 = v4;
            let a971 = 1u64;
            nat_add(a970, a971)?
        })
    } else {
        let a972 = v5;
        let a973 = 99u64;
        if if nat_eq(a972, a973) {
            true
        } else {
            let a974 = v5;
            let a975 = 5u64;
            nat_eq(a974, a975)
        } {
            Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `:` and the result type"))))
        } else {
            f120(v2, v3, v4)
        }
    }
}

pub fn f126(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    if f119(v0.clone(), v4, f11(Str::lit("|"))) {
        let v5: Adt12 = f76(v0.clone(), v1, {
            let a976 = v4;
            let a977 = 1u64;
            nat_add(a976, a977)?
        })?;
        let m979 = match v5.clone() {
            Adt12::C0(_, _, h978) => {
                h978
            }
        };
        match m979 {
            None => {
                let v7: Adt15 = {
                    let (h981, _) = f167(Fn13::F168(v1, v0.clone()), {
                        let a983 = {
                            let a982 = v0;
                            length_list(a982)
                        };
                        let a984 = 2u64;
                        nat_add(a983, a984)?
                    }, Adt15::C0(List::<Adt0>::nil(), match v5.clone() {
                        Adt12::C0(_, h985, _) => {
                            h985
                        }
                    }, true, None::<Adt1>))?;
                    h981
                };
                let m987 = match v7.clone() {
                    Adt15::C0(_, _, _, h986) => {
                        h986
                    }
                };
                match m987 {
                    None => {
                        f120(v2, f5(f116(v3.clone(), 1u64), match v3.clone() {
                            Adt7::C0(_, _, _, _, _, h989, _, _, _, _, _, _) => {
                                h989
                            }
                        }, match v3 {
                            Adt7::C0(_, _, _, _, _, _, h990, _, _, _, _, _) => {
                                h990
                            }
                        }, f98(Str::lit("constructor"), match v5 {
                            Adt12::C0(h991, _, _) => {
                                h991
                            }
                        }, Str::lit("binders"), f79(match v7.clone() {
                            Adt15::C0(h992, _, _, _) => {
                                h992
                            }
                        }))), {
                            let a994 = match v7 {
                                Adt15::C0(_, h993, _, _) => {
                                    h993
                                }
                            };
                            let a995 = 1u64;
                            nat_add(a994, a995)?
                        })
                    }
                    Some(_) => {
                        Ok(f77(v2, match v7 {
                            Adt15::C0(_, _, _, h988) => {
                                h988
                            }
                        }))
                    }
                }
            }
            Some(_) => {
                Ok(f77(v2, match v5 {
                    Adt12::C0(_, _, h980) => {
                        h980
                    }
                }))
            }
        }
    } else {
        let m997 = match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h996, _, _, _, _) => {
                h996
            }
        };
        match m997.uncons() {
            None => {
                Ok(f73(v0, v1, v2, v4, f11(Str::lit("a match has at least one `| \u{2026}` branch"))))
            }
            Some((_, _)) => {
                Ok(f78(v2, v4, f91(f11(Str::lit("match")), f98(Str::lit("scrutinee"), match v3.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, h998, _, _) => {
                        h998
                    }
                }, Str::lit("branches"), f79(match v3 {
                    Adt7::C0(_, _, _, _, _, _, _, h999, _, _, _, _) => {
                        h999
                    }
                })))?))
            }
        }
    }
}

pub fn f127(v0: Bytes) -> Option<Bytes> {
    if f36(v0.clone(), f11(Str::lit("+"))) {
        Some(f11(Str::lit("add")))
    } else {
        if f36(v0.clone(), f11(Str::lit("="))) {
            Some(f11(Str::lit("eq")))
        } else {
            if f36(v0.clone(), f11(Str::lit("<="))) {
                Some(f11(Str::lit("le")))
            } else {
                if f36(v0.clone(), f11(Str::lit("<"))) {
                    Some(f11(Str::lit("lt")))
                } else {
                    if f36(v0.clone(), f11(Str::lit("=?"))) {
                        Some(f11(Str::lit("beq")))
                    } else {
                        if f36(v0.clone(), f11(Str::lit("<=?"))) {
                            Some(f11(Str::lit("ble")))
                        } else {
                            if f36(v0.clone(), f11(Str::lit("<?"))) {
                                Some(f11(Str::lit("blt")))
                            } else {
                                if f36(v0.clone(), f11(Str::lit("&&"))) {
                                    Some(f11(Str::lit("and")))
                                } else {
                                    if f36(v0.clone(), f11(Str::lit("||"))) {
                                        Some(f11(Str::lit("or")))
                                    } else {
                                        if f36(v0.clone(), f11(Str::lit("/\\"))) {
                                            Some(f11(Str::lit("prop_and")))
                                        } else {
                                            if f36(v0.clone(), f11(Str::lit("<->"))) {
                                                Some(f11(Str::lit("iff")))
                                            } else {
                                                if f36(v0, f11(Str::lit("->"))) {
                                                    Some(f11(Str::lit("implies")))
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

pub fn f128(v0: Bytes, v1: u64, v2: Adt9) -> R<Option<Adt9>> {
    let m1001 = match v2.clone() {
        Adt9::C0(_, _, _, h1000) => {
            h1000
        }
    };
    match m1001 {
        None => {
            if match v2.clone() {
                Adt9::C0(_, _, h1002, _) => {
                    h1002
                }
            } {
                Ok(None::<Adt9>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt9::C0(h1003, _, _, _) => {
                        h1003
                    }
                };
                let a1004 = v4;
                let a1005 = v1;
                if nat_lt(a1004, a1005) {
                    if f26(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt9::C0({
                            let a1006 = v4;
                            let a1007 = 1u64;
                            nat_add(a1006, a1007)?
                        }, match v2 {
                            Adt9::C0(_, h1008, _, _) => {
                                h1008
                            }
                        }, true, None::<Adt1>)))
                    } else {
                        if f26(v0.clone(), v4, 92u8) {
                            Ok(Some(f169(v0, v1, v2)?))
                        } else {
                            let a1009 = f170(v0.clone(), v4)?;
                            let a1010 = 32u64;
                            if nat_lt(a1009, a1010) {
                                Ok(Some(f171(v2, v4, f45(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f172(v0.clone(), v1, v4)?;
                                Ok(Some(f173(v2, v5, f47(v0, v4, {
                                    let a1011 = v5;
                                    let a1012 = v4;
                                    nat_sub(a1011, a1012)
                                }))))
                            }
                        }
                    }
                } else {
                    Ok(None::<Adt9>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt9>)
        }
    }
}

pub fn f129(v0: Fn8, p1017: u64, p1018: List<Bytes>) -> (List<Bytes>, bool) {
    let m1016 = (p1017, p1018);
    {
        let mut m1016 = m1016;
        loop {
            let step = {
                let (v1, v2) = m1016;
                let m1013 = v1;
                if m1013 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m1013 - 1;
                    let m1015 = {
                        let c1014: Fn8 = v0.clone();
                        c1014.apply(v2.clone())
                    };
                    match m1015 {
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
                Ok(next) => { m1016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Bytes>) -> Option<List<Bytes>> {
    f174(v0)
}

pub fn f131(v0: Fn9, p1022: List<Bytes>, p1023: List<Bytes>) -> List<Bytes> {
    let m1021 = (p1022, p1023);
    {
        let mut m1021 = m1021;
        loop {
            let step = {
                let (v1, v2) = m1021;
                let m1019 = v2;
                match m1019.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1020: Fn9 = v0.clone();
                            c1020.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1021 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f133(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1024 = v2;
    let a1025 = v1;
    if nat_lt(a1024, a1025) {
        if f175(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a1026 = v2;
                let a1027 = 1u64;
                nat_add(a1026, a1027)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f134(v0: List<Adt3>, v1: u64) -> u64 {
    let m1028 = f148(v0, v1);
    match m1028 {
        None => {
            99u64
        }
        Some(v2) => {
            f176(match v2 {
                Adt3::C0(h1029, _, _) => {
                    h1029
                }
            })
        }
    }
}

pub fn f135(v0: List<Adt0>) -> Adt0 {
    f177(Fn14::F178, Adt0::C5, v0)
}

pub fn f136(v0: Adt0, v1: Option<Adt0>, v2: Bytes) -> R<Adt2> {
    let m1030 = v1;
    match m1030 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v3) => {
            let m1031 = f10(f22(v0.clone(), v3)?, f11(Str::lit("properties")));
            match m1031 {
                None => {
                    Ok(Adt2::C5)
                }
                Some(v4) => {
                    let m1032 = f10(v4, v2);
                    match m1032 {
                        None => {
                            Ok(Adt2::C5)
                        }
                        Some(v5) => {
                            f23(v0, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f137(v0: List<Adt3>, v1: u64) -> Bytes {
    let m1033 = f148(v0, v1);
    match m1033 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f179(match v2 {
                Adt3::C0(h1034, _, _) => {
                    h1034
                }
            })
        }
    }
}

pub fn f138(v0: u64) -> bool {
    let a1035 = 64u64;
    let a1036 = v0;
    if if nat_lt(a1035, a1036) {
        let a1037 = v0;
        let a1038 = 91u64;
        nat_lt(a1037, a1038)
    } else {
        false
    } {
        true
    } else {
        let a1039 = 96u64;
        let a1040 = v0;
        if if nat_lt(a1039, a1040) {
            let a1041 = v0;
            let a1042 = 123u64;
            nat_lt(a1041, a1042)
        } else {
            false
        } {
            true
        } else {
            let a1043 = v0;
            let a1044 = 95u64;
            nat_eq(a1043, a1044)
        }
    }
}

pub fn f139(v0: Bytes, v1: u64) -> R<u64> {
    let m1045 = f41(v0, v1);
    match m1045 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1090 = {
                let a1066 = {
                    let a1054 = {
                        let a1048 = {
                            let a1046 = v3;
                            let a1047 = 128u8;
                            fixed_u8::bit_and(a1046, a1047)
                        };
                        let a1049 = 128u8;
                        if equal(a1048, a1049) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1055 = {
                        let a1052 = {
                            let a1050 = v3;
                            let a1051 = 64u8;
                            fixed_u8::bit_and(a1050, a1051)
                        };
                        let a1053 = 64u8;
                        if equal(a1052, a1053) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1054, a1055)?
                };
                let a1067 = {
                    let a1064 = {
                        let a1058 = {
                            let a1056 = v3;
                            let a1057 = 32u8;
                            fixed_u8::bit_and(a1056, a1057)
                        };
                        let a1059 = 32u8;
                        if equal(a1058, a1059) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1065 = {
                        let a1062 = {
                            let a1060 = v3;
                            let a1061 = 16u8;
                            fixed_u8::bit_and(a1060, a1061)
                        };
                        let a1063 = 16u8;
                        if equal(a1062, a1063) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1064, a1065)?
                };
                nat_add(a1066, a1067)?
            };
            let a1091 = {
                let a1088 = {
                    let a1076 = {
                        let a1070 = {
                            let a1068 = v3;
                            let a1069 = 8u8;
                            fixed_u8::bit_and(a1068, a1069)
                        };
                        let a1071 = 8u8;
                        if equal(a1070, a1071) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1077 = {
                        let a1074 = {
                            let a1072 = v3;
                            let a1073 = 4u8;
                            fixed_u8::bit_and(a1072, a1073)
                        };
                        let a1075 = 4u8;
                        if equal(a1074, a1075) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1076, a1077)?
                };
                let a1089 = {
                    let a1086 = {
                        let a1080 = {
                            let a1078 = v3;
                            let a1079 = 2u8;
                            fixed_u8::bit_and(a1078, a1079)
                        };
                        let a1081 = 2u8;
                        if equal(a1080, a1081) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1087 = {
                        let a1084 = {
                            let a1082 = v3;
                            let a1083 = 1u8;
                            fixed_u8::bit_and(a1082, a1083)
                        };
                        let a1085 = 1u8;
                        if equal(a1084, a1085) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1086, a1087)?
                };
                nat_add(a1088, a1089)?
            };
            nat_add(a1090, a1091)
        }
    }
}

pub fn f140(v0: Bytes, v1: u64) -> R<u64> {
    let (h1092, _) = f70(Fn7::F180(v0.clone()), {
        let a1094 = {
            let a1093 = v0;
            length_bytes(a1093)
        };
        let a1095 = 1u64;
        nat_add(a1094, a1095)?
    }, v1)?;
    Ok(h1092)
}

pub fn f141(v0: Bytes) -> bool {
    let a1096 = v0.clone();
    let a1097 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a1096, a1097) {
        true
    } else {
        let a1098 = v0;
        let a1099 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a1098, a1099)
    }
}

pub fn f142(v0: Bytes, v1: u64) -> R<u64> {
    let (h1100, _) = f70(Fn7::F181(v0.clone()), {
        let a1102 = {
            let a1101 = v0;
            length_bytes(a1101)
        };
        let a1103 = 1u64;
        nat_add(a1102, a1103)?
    }, v1)?;
    Ok(h1100)
}

pub fn f143(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Option<Adt0>> {
    let m1104 = f182(f10(f22(v0.clone(), v1)?, f11(Str::lit("oneOf"))));
    match m1104 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v3) => {
            f183(Fn15::F184(v2, v0), None::<Adt0>, f185(v3))
        }
    }
}

pub fn f144(v0: Adt0, v1: Adt0) -> R<bool> {
    let m1105 = f182(f10(f22(v0.clone(), v1)?, f11(Str::lit("oneOf"))));
    match m1105 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            f186(Fn16::F187(v0), true, f185(v2))
        }
    }
}

pub fn f145(v0: Fn10, p1109: bool, p1110: List<Bytes>) -> bool {
    let m1108 = (p1109, p1110);
    {
        let mut m1108 = m1108;
        loop {
            let step = {
                let (v1, v2) = m1108;
                let m1106 = v2;
                match m1106.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1107: Fn10 = v0.clone();
                            c1107.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1111 = v2;
        let a1112 = v0;
        equal(a1111, a1112)
    }
}

pub fn f147(v0: Bytes) -> bool {
    let m1113 = f90(v0.clone());
    match m1113 {
        None => {
            f95(v0, f96())
        }
        Some(_) => {
            true
        }
    }
}

pub fn f148(v0: List<Adt3>, v1: u64) -> Option<Adt3> {
    let a1114 = v0;
    let a1115 = v1;
    index_list(a1114, a1115)
}

pub fn f149(v0: Adt4) -> R<Bytes> {
    let m1116 = v0;
    match m1116 {
        Adt4::C0 => {
            Ok(f150(Str::lit("`{`")))
        }
        Adt4::C1 => {
            Ok(f150(Str::lit("`}`")))
        }
        Adt4::C2 => {
            Ok(f150(Str::lit("`[`")))
        }
        Adt4::C3 => {
            Ok(f150(Str::lit("`]`")))
        }
        Adt4::C4 => {
            Ok(f150(Str::lit("`(`")))
        }
        Adt4::C5 => {
            Ok(f150(Str::lit("`)`")))
        }
        Adt4::C6 => {
            Ok(f150(Str::lit("`,`")))
        }
        Adt4::C7 => {
            Ok(f150(Str::lit("`:`")))
        }
        Adt4::C8 => {
            Ok(f150(Str::lit("`:=`")))
        }
        Adt4::C9 => {
            Ok(f150(Str::lit("`::`")))
        }
        Adt4::C10 => {
            Ok(f150(Str::lit("`;`")))
        }
        Adt4::C11(v1) => {
            Ok(f188(f150(Str::lit("the string ")), f189(v1)?))
        }
        Adt4::C12(v2) => {
            Ok(f188(f188(f150(Str::lit("`")), v2), f150(Str::lit("`"))))
        }
    }
}

pub fn f150(v0: Str) -> Bytes {
    let a1117 = v0;
    utf8_encode(a1117)
}

pub fn f151(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: Bytes) -> R<Adt6> {
    let v5: u64 = f111(v0.clone(), v1, v3);
    let v6: Bytes = if f92(v4.clone(), f11(Str::lit("-"))) {
        f93(v4.clone(), 1u64)
    } else {
        v4.clone()
    };
    let v7: Adt12 = f103(v0, v1, {
        let a1118 = v3;
        let a1119 = 2u64;
        nat_add(a1118, a1119)?
    }, f11(Str::lit("an integer type")))?;
    let m1121 = match v7.clone() {
        Adt12::C0(_, _, h1120) => {
            h1120
        }
    };
    match m1121 {
        None => {
            let v10: Bytes = {
                let m1124 = f50(match v7.clone() {
                    Adt12::C0(h1123, _, _) => {
                        h1123
                    }
                });
                match m1124 {
                    None => {
                        f11(Str::lit(""))
                    }
                    Some(v9) => {
                        v9
                    }
                }
            };
            let m1125 = f190(v10.clone());
            match m1125 {
                None => {
                    Ok(f113(v2, v5, f97(f153(v10), f11(Str::lit(" is not an integer type")))))
                }
                Some(v11) => {
                    if if f36(v4.clone(), f11(Str::lit("-0"))) {
                        true
                    } else {
                        f85({
                            let a1127 = 1u64;
                            let a1128 = {
                                let a1126 = v6.clone();
                                length_bytes(a1126)
                            };
                            nat_lt(a1127, a1128)
                        }, f92(v6, f11(Str::lit("0"))))
                    } {
                        Ok(f113(v2, v5, f97(f153(v4), f11(Str::lit(" is not a canonical integer")))))
                    } else {
                        Ok(f78(v2, match v7 {
                            Adt12::C0(_, h1129, _) => {
                                h1129
                            }
                        }, f91(f11(Str::lit("integer")), f98(Str::lit("representation"), f89(v11), Str::lit("value"), f89(v4)))?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(f77(v2, match v7 {
                Adt12::C0(_, _, h1122) => {
                    h1122
                }
            }))
        }
    }
}

pub fn f152(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: u64, v5: Bytes) -> R<Adt6> {
    let v6: u64 = {
        let a1130 = v3;
        let a1131 = 1u64;
        nat_add(a1130, a1131)?
    };
    if if f36(v5.clone(), f11(Str::lit("not"))) {
        true
    } else {
        if f36(v5.clone(), f11(Str::lit("first"))) {
            true
        } else {
            f36(v5.clone(), f11(Str::lit("second")))
        }
    } {
        f120(v2, f110(f6(31u64, v4), v5, 0u64), v6)
    } else {
        if f36(v5.clone(), f11(Str::lit("cons"))) {
            f120(v2, f6(32u64, v4), v6)
        } else {
            if f36(v5.clone(), f11(Str::lit("nil"))) {
                f115(v2, f6(33u64, v4), v6)
            } else {
                if f36(v5.clone(), f11(Str::lit("if"))) {
                    f120(v2, f6(34u64, v4), v6)
                } else {
                    if f36(v5.clone(), f11(Str::lit("let"))) {
                        f164(v2, f6(35u64, v4), v6)
                    } else {
                        if f36(v5.clone(), f11(Str::lit("forall"))) {
                            let a1132 = f75(v0.clone(), v6);
                            let a1133 = 4u64;
                            if nat_eq(a1132, a1133) {
                                f164(v2, f6(36u64, v4), {
                                    let a1134 = v6;
                                    let a1135 = 1u64;
                                    nat_add(a1134, a1135)?
                                })
                            } else {
                                Ok(f73(v0, v1, v2, v6, f11(Str::lit("expected `(` before the bound variable"))))
                            }
                        } else {
                            if f36(v5.clone(), f11(Str::lit("fun"))) {
                                let a1136 = f75(v0.clone(), v6);
                                let a1137 = 2u64;
                                if nat_eq(a1136, a1137) {
                                    let v7: Adt12 = f191(v0.clone(), v1, v6, f11(Str::lit("a name")))?;
                                    let m1139 = match v7.clone() {
                                        Adt12::C0(_, _, h1138) => {
                                            h1138
                                        }
                                    };
                                    match m1139 {
                                        None => {
                                            f121(v0, v1, v2, f104(f6(37u64, v4), match v7.clone() {
                                                Adt12::C0(h1141, _, _) => {
                                                    h1141
                                                }
                                            }, Adt0::C5), match v7 {
                                                Adt12::C0(_, h1142, _) => {
                                                    h1142
                                                }
                                            })
                                        }
                                        Some(_) => {
                                            Ok(f77(v2, match v7 {
                                                Adt12::C0(_, _, h1140) => {
                                                    h1140
                                                }
                                            }))
                                        }
                                    }
                                } else {
                                    f121(v0, v1, v2, f104(f6(37u64, v4), f79(List::<Adt0>::nil()), Adt0::C5), v6)
                                }
                            } else {
                                if f36(v5.clone(), f11(Str::lit("apply"))) {
                                    f120(v2, f6(38u64, v4), v6)
                                } else {
                                    if f36(v5.clone(), f11(Str::lit("project"))) {
                                        f120(v2, f6(39u64, v4), v6)
                                    } else {
                                        if if f36(v5.clone(), f11(Str::lit("ctor"))) {
                                            true
                                        } else {
                                            if f36(v5.clone(), f11(Str::lit("ref"))) {
                                                true
                                            } else {
                                                if f36(v5.clone(), f11(Str::lit("record"))) {
                                                    true
                                                } else {
                                                    f36(v5.clone(), f11(Str::lit("checked")))
                                                }
                                            }
                                        } {
                                            let v9: Adt12 = f76(v0.clone(), v1, v6)?;
                                            let m1144 = match v9.clone() {
                                                Adt12::C0(_, _, h1143) => {
                                                    h1143
                                                }
                                            };
                                            match m1144 {
                                                None => {
                                                    f192(v0, v1, v2, f110(f104(f6(if f36(v5.clone(), f11(Str::lit("ctor"))) {
                                                        40u64
                                                    } else {
                                                        if f36(v5.clone(), f11(Str::lit("ref"))) {
                                                            41u64
                                                        } else {
                                                            if f36(v5.clone(), f11(Str::lit("record"))) {
                                                                42u64
                                                            } else {
                                                                44u64
                                                            }
                                                        }
                                                    }, v4), match v9.clone() {
                                                        Adt12::C0(h1146, _, _) => {
                                                            h1146
                                                        }
                                                    }, Adt0::C5), if f36(v5, f11(Str::lit("checked"))) {
                                                        f11(Str::lit("with"))
                                                    } else {
                                                        f11(Str::lit(""))
                                                    }, 0u64), match v9 {
                                                        Adt12::C0(_, h1147, _) => {
                                                            h1147
                                                        }
                                                    })
                                                }
                                                Some(_) => {
                                                    Ok(f77(v2, match v9 {
                                                        Adt12::C0(_, _, h1145) => {
                                                            h1145
                                                        }
                                                    }))
                                                }
                                            }
                                        } else {
                                            if f36(v5.clone(), f11(Str::lit("prim"))) {
                                                let v11: Adt12 = f103(v0.clone(), v1, v6, f11(Str::lit("a primitive operation")))?;
                                                let m1149 = match v11.clone() {
                                                    Adt12::C0(_, _, h1148) => {
                                                        h1148
                                                    }
                                                };
                                                match m1149 {
                                                    None => {
                                                        f125(v0, v1, v2, f104(f6(43u64, v4), match v11.clone() {
                                                            Adt12::C0(h1151, _, _) => {
                                                                h1151
                                                            }
                                                        }, Adt0::C5), match v11 {
                                                            Adt12::C0(_, h1152, _) => {
                                                                h1152
                                                            }
                                                        })
                                                    }
                                                    Some(_) => {
                                                        Ok(f77(v2, match v11 {
                                                            Adt12::C0(_, _, h1150) => {
                                                                h1150
                                                            }
                                                        }))
                                                    }
                                                }
                                            } else {
                                                if f36(v5, f11(Str::lit("match"))) {
                                                    f120(v2, f6(45u64, v4), v6)
                                                } else {
                                                    f193(v0, v1, v2, v3, v4)
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

pub fn f153(v0: Bytes) -> Bytes {
    f97(f97(f11(Str::lit("`")), v0), f11(Str::lit("`")))
}

pub fn f154(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let (h1153, _) = f70(Fn7::F194(v0.clone()), {
            let a1155 = {
                let a1154 = v0.clone();
                length_bytes(a1154)
            };
            let a1156 = 1u64;
            nat_add(a1155, a1156)?
        }, 0u64)?;
        h1153
    };
    Ok(f85({
        let a1158 = v1;
        let a1159 = {
            let a1157 = v0.clone();
            length_bytes(a1157)
        };
        nat_eq(a1158, a1159)
    }, {
        let a1164 = {
            let a1161 = {
                let a1160 = v0;
                length_bytes(a1160)
            };
            let a1162 = 2u64;
            let a1163 = 0u64;
            nat_rem(a1161, a1162, a1163)
        };
        let a1165 = 0u64;
        nat_eq(a1164, a1165)
    }))
}

pub fn f155(v0: Bytes) -> R<bool> {
    Ok(f85(f86(v0.clone())?, f85(!f95(v0.clone(), f156()), !f36(v0, f11(Str::lit("false"))))))
}

pub fn f156() -> List<Bytes> {
    List::cons(f11(Str::lit("let")), List::cons(f11(Str::lit("if")), List::cons(f11(Str::lit("match")), List::cons(f11(Str::lit("fun")), List::cons(f11(Str::lit("forall")), List::cons(f11(Str::lit("nil")), List::cons(f11(Str::lit("cons")), List::cons(f11(Str::lit("first")), List::cons(f11(Str::lit("second")), List::cons(f11(Str::lit("not")), List::cons(f11(Str::lit("apply")), List::cons(f11(Str::lit("ctor")), List::cons(f11(Str::lit("record")), List::cons(f11(Str::lit("project")), List::cons(f11(Str::lit("prim")), List::cons(f11(Str::lit("ref")), List::cons(f11(Str::lit("checked")), List::cons(f11(Str::lit("then")), List::cons(f11(Str::lit("else")), List::cons(f11(Str::lit("with")), List::cons(f11(Str::lit("true")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f157(v0: u64, v1: Bytes) -> Option<Adt1> {
    Some(Adt1::C0(v0, v1))
}

pub fn f158(v0: Fn11, p1170: u64, p1171: Adt13) -> R<(Adt13, bool)> {
    let m1169 = (p1170, p1171);
    {
        let mut m1169 = m1169;
        loop {
            let step = {
                let (v1, v2) = m1169;
                let m1166 = v1;
                if m1166 == 0 {
                    Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1166 - 1;
                    let m1168 = {
                        let c1167: Fn11 = v0.clone();
                        c1167.apply(v2.clone())?
                    };
                    match m1168 {
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
                Ok(next) => { m1169 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: u64, v1: List<Adt3>, v2: Adt13) -> R<Option<Adt13>> {
    f195(v1, v0, v2)
}

pub fn f160(v0: Fn12, p1175: bool, p1176: List<(Bytes, Adt0)>) -> bool {
    let m1174 = (p1175, p1176);
    {
        let mut m1174 = m1174;
        loop {
            let step = {
                let (v1, v2) = m1174;
                let m1172 = v2;
                match m1172.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt0)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt0)>), bool>(({
                            let c1173: Fn12 = v0.clone();
                            c1173.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Bytes, v1: bool, v2: (Bytes, Adt0)) -> bool {
    if v1 {
        true
    } else {
        let a1178 = {
            let (h1177, _) = v2;
            h1177
        };
        let a1179 = v0;
        equal(a1178, a1179)
    }
}

pub fn f162(v0: List<(Bytes, Adt0)>) -> Adt0 {
    f196(Fn17::F197, Adt0::C5, f198(v0))
}

pub fn f163(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f199(f200(Fn18::F201, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m1184 = {
        let (h1180, _) = f202(Fn19::F203, {
            let a1182 = {
                let a1181 = v0;
                length_list(a1181)
            };
            let a1183 = 1u64;
            nat_add(a1182, a1183)?
        }, v1)?;
        h1180
    };
    match m1184.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f164(v0: Adt6, v1: Adt7, v2: u64) -> R<Adt6> {
    Ok(f81(v0, v1.clone(), v2, 4u64, Adt2::C3, {
        let a1186 = match v1 {
            Adt7::C0(_, _, h1185, _, _, _, _, _, _, _, _, _) => {
                h1185
            }
        };
        let a1187 = 1u64;
        nat_add(a1186, a1187)?
    }))
}

pub fn f165(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let v5: u64 = match v3.clone() {
        Adt7::C0(h1188, _, _, _, _, _, _, _, _, _, _, _) => {
            h1188
        }
    };
    let v6: Adt0 = f79(match v3.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h1189, _, _, _, _) => {
            h1189
        }
    });
    let a1190 = v5;
    let a1191 = 38u64;
    if nat_eq(a1190, a1191) {
        Ok(f78(v2, v4, f91(f11(Str::lit("apply")), f98(Str::lit("function"), match v3 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h1192, _, _) => {
                h1192
            }
        }, Str::lit("arguments"), v6))?))
    } else {
        let a1193 = v5;
        let a1194 = 40u64;
        if nat_eq(a1193, a1194) {
            Ok(f78(v2, v4, f91(f11(Str::lit("constructor")), f166(f98(Str::lit("constructor"), match v3.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, h1195, _, _) => {
                    h1195
                }
            }, Str::lit("arguments"), v6), match v3 {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, h1196, _) => {
                    h1196
                }
            }))?))
        } else {
            let a1197 = v5;
            let a1198 = 46u64;
            if nat_eq(a1197, a1198) {
                Ok(f78(v2, v4, f91(f11(Str::lit("call")), f166(f98(Str::lit("function"), match v3.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, h1199, _, _) => {
                        h1199
                    }
                }, Str::lit("arguments"), v6), match v3 {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, h1200, _) => {
                        h1200
                    }
                }))?))
            } else {
                let v7: Adt12 = f191(v0, v1, {
                    let a1201 = v4;
                    let a1202 = 1u64;
                    nat_add(a1201, a1202)?
                }, f11(Str::lit("a check")))?;
                let m1204 = match v7.clone() {
                    Adt12::C0(_, _, h1203) => {
                        h1203
                    }
                };
                match m1204 {
                    None => {
                        Ok(f78(v2, match v7.clone() {
                            Adt12::C0(_, h1206, _) => {
                                h1206
                            }
                        }, f91(f11(Str::lit("checked_apply")), f166(List::cons((f11(Str::lit("checks")), match v7 {
                            Adt12::C0(h1207, _, _) => {
                                h1207
                            }
                        }), f98(Str::lit("model"), match v3.clone() {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, h1208, _, _) => {
                                h1208
                            }
                        }, Str::lit("arguments"), v6)), match v3 {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h1209, _) => {
                                h1209
                            }
                        }))?))
                    }
                    Some(_) => {
                        Ok(f77(v2, match v7 {
                            Adt12::C0(_, _, h1205) => {
                                h1205
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f166(v0: List<(Bytes, Adt0)>, v1: Adt0) -> List<(Bytes, Adt0)> {
    let m1210 = f204(v1.clone());
    match m1210 {
        None => {
            v0
        }
        Some(_) => {
            List::cons((f11(Str::lit("type_arguments")), v1), v0)
        }
    }
}

pub fn f167(v0: Fn13, p1215: u64, p1216: Adt15) -> R<(Adt15, bool)> {
    let m1214 = (p1215, p1216);
    {
        let mut m1214 = m1214;
        loop {
            let step = {
                let (v1, v2) = m1214;
                let m1211 = v1;
                if m1211 == 0 {
                    Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1211 - 1;
                    let m1213 = {
                        let c1212: Fn13 = v0.clone();
                        c1212.apply(v2.clone())?
                    };
                    match m1213 {
                        None => {
                            Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt15), R<(Adt15, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: u64, v1: List<Adt3>, v2: Adt15) -> R<Option<Adt15>> {
    let m1218 = match v2.clone() {
        Adt15::C0(_, _, _, h1217) => {
            h1217
        }
    };
    match m1218 {
        None => {
            if f119(v1.clone(), match v2.clone() {
                Adt15::C0(_, h1219, _, _) => {
                    h1219
                }
            }, f11(Str::lit("=>"))) {
                Ok(None::<Adt15>)
            } else {
                let v4: Adt12 = f103(v1, v0, match v2.clone() {
                    Adt15::C0(_, h1220, _, _) => {
                        h1220
                    }
                }, f11(Str::lit("a binder name")))?;
                let m1222 = match v4.clone() {
                    Adt12::C0(_, _, h1221) => {
                        h1221
                    }
                };
                match m1222 {
                    None => {
                        Ok(Some(Adt15::C0(List::cons(match v4.clone() {
                            Adt12::C0(h1226, _, _) => {
                                h1226
                            }
                        }, match v2 {
                            Adt15::C0(h1227, _, _, _) => {
                                h1227
                            }
                        }), match v4 {
                            Adt12::C0(_, h1228, _) => {
                                h1228
                            }
                        }, true, None::<Adt1>)))
                    }
                    Some(_) => {
                        Ok(Some(Adt15::C0(match v2.clone() {
                            Adt15::C0(h1223, _, _, _) => {
                                h1223
                            }
                        }, match v2 {
                            Adt15::C0(_, h1224, _, _) => {
                                h1224
                            }
                        }, false, match v4 {
                            Adt12::C0(_, _, h1225) => {
                                h1225
                            }
                        })))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt15>)
        }
    }
}

pub fn f169(v0: Bytes, v1: u64, v2: Adt9) -> R<Adt9> {
    let v3: u64 = match v2.clone() {
        Adt9::C0(h1229, _, _, _) => {
            h1229
        }
    };
    let m1232 = f41(v0.clone(), {
        let a1230 = v3;
        let a1231 = 1u64;
        nat_add(a1230, a1231)?
    });
    match m1232 {
        None => {
            Ok(Adt9::C0(v1, match v2 {
                Adt9::C0(_, h1233, _, _) => {
                    h1233
                }
            }, false, None::<Adt1>))
        }
        Some(v4) => {
            let a1234 = v4;
            let a1235 = 34u8;
            if equal(a1234, a1235) {
                Ok(f173(v2, {
                    let a1236 = v3;
                    let a1237 = 2u64;
                    nat_add(a1236, a1237)?
                }, f45(Str::lit("\""))))
            } else {
                let a1238 = v4;
                let a1239 = 92u8;
                if equal(a1238, a1239) {
                    Ok(f173(v2, {
                        let a1240 = v3;
                        let a1241 = 2u64;
                        nat_add(a1240, a1241)?
                    }, f45(Str::lit("\\"))))
                } else {
                    let a1242 = v4;
                    let a1243 = 47u8;
                    if equal(a1242, a1243) {
                        Ok(f173(v2, {
                            let a1244 = v3;
                            let a1245 = 2u64;
                            nat_add(a1244, a1245)?
                        }, f45(Str::lit("/"))))
                    } else {
                        let a1246 = v4;
                        let a1247 = 98u8;
                        if equal(a1246, a1247) {
                            Ok(f173(v2, {
                                let a1248 = v3;
                                let a1249 = 2u64;
                                nat_add(a1248, a1249)?
                            }, f205(8u64)))
                        } else {
                            let a1250 = v4;
                            let a1251 = 102u8;
                            if equal(a1250, a1251) {
                                Ok(f173(v2, {
                                    let a1252 = v3;
                                    let a1253 = 2u64;
                                    nat_add(a1252, a1253)?
                                }, f205(12u64)))
                            } else {
                                let a1254 = v4;
                                let a1255 = 110u8;
                                if equal(a1254, a1255) {
                                    Ok(f173(v2, {
                                        let a1256 = v3;
                                        let a1257 = 2u64;
                                        nat_add(a1256, a1257)?
                                    }, f205(10u64)))
                                } else {
                                    let a1258 = v4;
                                    let a1259 = 114u8;
                                    if equal(a1258, a1259) {
                                        Ok(f173(v2, {
                                            let a1260 = v3;
                                            let a1261 = 2u64;
                                            nat_add(a1260, a1261)?
                                        }, f205(13u64)))
                                    } else {
                                        let a1262 = v4;
                                        let a1263 = 116u8;
                                        if equal(a1262, a1263) {
                                            Ok(f173(v2, {
                                                let a1264 = v3;
                                                let a1265 = 2u64;
                                                nat_add(a1264, a1265)?
                                            }, f205(9u64)))
                                        } else {
                                            let a1266 = v4;
                                            let a1267 = 117u8;
                                            if equal(a1266, a1267) {
                                                f206(v0, v2)
                                            } else {
                                                Ok(f171(v2, v3, f207(f207(f45(Str::lit("`\\")), f47(v0.clone(), {
                                                    let a1268 = v3;
                                                    let a1269 = 1u64;
                                                    nat_add(a1268, a1269)?
                                                }, f208(v0, {
                                                    let a1270 = v3;
                                                    let a1271 = 1u64;
                                                    nat_add(a1270, a1271)?
                                                })?)), f45(Str::lit("` is not an escape")))))
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

pub fn f170(v0: Bytes, v1: u64) -> R<u64> {
    let m1272 = f41(v0, v1);
    match m1272 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1317 = {
                let a1293 = {
                    let a1281 = {
                        let a1275 = {
                            let a1273 = v3;
                            let a1274 = 128u8;
                            fixed_u8::bit_and(a1273, a1274)
                        };
                        let a1276 = 128u8;
                        if equal(a1275, a1276) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1282 = {
                        let a1279 = {
                            let a1277 = v3;
                            let a1278 = 64u8;
                            fixed_u8::bit_and(a1277, a1278)
                        };
                        let a1280 = 64u8;
                        if equal(a1279, a1280) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1281, a1282)?
                };
                let a1294 = {
                    let a1291 = {
                        let a1285 = {
                            let a1283 = v3;
                            let a1284 = 32u8;
                            fixed_u8::bit_and(a1283, a1284)
                        };
                        let a1286 = 32u8;
                        if equal(a1285, a1286) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1292 = {
                        let a1289 = {
                            let a1287 = v3;
                            let a1288 = 16u8;
                            fixed_u8::bit_and(a1287, a1288)
                        };
                        let a1290 = 16u8;
                        if equal(a1289, a1290) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1291, a1292)?
                };
                nat_add(a1293, a1294)?
            };
            let a1318 = {
                let a1315 = {
                    let a1303 = {
                        let a1297 = {
                            let a1295 = v3;
                            let a1296 = 8u8;
                            fixed_u8::bit_and(a1295, a1296)
                        };
                        let a1298 = 8u8;
                        if equal(a1297, a1298) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1304 = {
                        let a1301 = {
                            let a1299 = v3;
                            let a1300 = 4u8;
                            fixed_u8::bit_and(a1299, a1300)
                        };
                        let a1302 = 4u8;
                        if equal(a1301, a1302) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1303, a1304)?
                };
                let a1316 = {
                    let a1313 = {
                        let a1307 = {
                            let a1305 = v3;
                            let a1306 = 2u8;
                            fixed_u8::bit_and(a1305, a1306)
                        };
                        let a1308 = 2u8;
                        if equal(a1307, a1308) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1314 = {
                        let a1311 = {
                            let a1309 = v3;
                            let a1310 = 1u8;
                            fixed_u8::bit_and(a1309, a1310)
                        };
                        let a1312 = 1u8;
                        if equal(a1311, a1312) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1313, a1314)?
                };
                nat_add(a1315, a1316)?
            };
            nat_add(a1317, a1318)
        }
    }
}

pub fn f171(v0: Adt9, v1: u64, v2: Bytes) -> Adt9 {
    Adt9::C0(match v0.clone() {
        Adt9::C0(h1319, _, _, _) => {
            h1319
        }
    }, match v0 {
        Adt9::C0(_, h1320, _, _) => {
            h1320
        }
    }, false, Some(f44(v1, v2)))
}

pub fn f172(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1321, _) = f70(Fn7::F209(v1, v0), {
        let a1322 = v1;
        let a1323 = v2;
        nat_sub(a1322, a1323)
    }, v2)?;
    Ok(h1321)
}

pub fn f173(v0: Adt9, v1: u64, v2: Bytes) -> Adt9 {
    Adt9::C0(v1, List::cons(v2, match v0 {
        Adt9::C0(_, h1324, _, _) => {
            h1324
        }
    }), false, None::<Adt1>)
}

pub fn f174(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m1325 = v0.clone();
    match m1325.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m1326 = v2;
            match m1326.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f210(Fn20::F211, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m1328 = {
                        let (h1327, _) = v5;
                        h1327
                    };
                    match m1328 {
                        None => {
                            Some(f69({
                                let (_, h1330) = v5;
                                h1330
                            }))
                        }
                        Some(v6) => {
                            Some(f69(List::cons(v6, {
                                let (_, h1329) = v5;
                                h1329
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: u64) -> R<bool> {
    let a1331 = 0u64;
    let a1332 = f25(v0.clone(), v1)?;
    if nat_lt(a1331, a1332) {
        Ok(true)
    } else {
        Ok(f212(v0, v1))
    }
}

pub fn f176(v0: Adt4) -> u64 {
    let m1333 = v0;
    match m1333 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1 => {
            1u64
        }
        Adt4::C2 => {
            2u64
        }
        Adt4::C3 => {
            3u64
        }
        Adt4::C4 => {
            4u64
        }
        Adt4::C5 => {
            5u64
        }
        Adt4::C6 => {
            6u64
        }
        Adt4::C7 => {
            7u64
        }
        Adt4::C8 => {
            8u64
        }
        Adt4::C9 => {
            9u64
        }
        Adt4::C10 => {
            10u64
        }
        Adt4::C11(_) => {
            11u64
        }
        Adt4::C12(_) => {
            12u64
        }
    }
}

pub fn f177(v0: Fn14, p1337: Adt0, p1338: List<Adt0>) -> Adt0 {
    let m1336 = (p1337, p1338);
    {
        let mut m1336 = m1336;
        loop {
            let step = {
                let (v1, v2) = m1336;
                let m1334 = v2;
                match m1334.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c1335: Fn14 = v0.clone();
                            c1335.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1336 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f179(v0: Adt4) -> Bytes {
    let m1339 = v0;
    match m1339 {
        Adt4::C0 => {
            Bytes::lit(&[])
        }
        Adt4::C1 => {
            Bytes::lit(&[])
        }
        Adt4::C2 => {
            Bytes::lit(&[])
        }
        Adt4::C3 => {
            Bytes::lit(&[])
        }
        Adt4::C4 => {
            Bytes::lit(&[])
        }
        Adt4::C5 => {
            Bytes::lit(&[])
        }
        Adt4::C6 => {
            Bytes::lit(&[])
        }
        Adt4::C7 => {
            Bytes::lit(&[])
        }
        Adt4::C8 => {
            Bytes::lit(&[])
        }
        Adt4::C9 => {
            Bytes::lit(&[])
        }
        Adt4::C10 => {
            Bytes::lit(&[])
        }
        Adt4::C11(v1) => {
            v1
        }
        Adt4::C12(v2) => {
            v2
        }
    }
}

pub fn f180(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f213(v0, v1)
}

pub fn f181(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f214(v0, v1)
}

pub fn f182(v0: Option<Adt0>) -> Option<Adt0> {
    let m1340 = v0;
    match m1340 {
        None => {
            None::<Adt0>
        }
        Some(v1) => {
            f204(v1)
        }
    }
}

pub fn f183(v0: Fn15, p1344: Option<Adt0>, p1345: List<Adt0>) -> R<Option<Adt0>> {
    let m1343 = (p1344, p1345);
    {
        let mut m1343 = m1343;
        loop {
            let step = {
                let (v1, v2) = m1343;
                let m1341 = v2;
                match m1341.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(({
                            let c1342: Fn15 = v0.clone();
                            c1342.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt0) -> R<Option<Adt0>> {
    let m1346 = v2.clone();
    match m1346 {
        None => {
            let v5: Adt0 = f22(v1, v3)?;
            if f215(v5.clone(), v0)? {
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

pub fn f185(v0: Adt0) -> List<Adt0> {
    f216({
        let m1349 = {
            let (h1348, _) = f217(Fn21::F218, 1000000u64, Adt16::C0(v0, List::<Adt0>::nil()));
            h1348
        };
        match m1349 {
            Adt16::C0(_, h1347) => {
                h1347
            }
        }
    })
}

pub fn f186(v0: Fn16, p1353: bool, p1354: List<Adt0>) -> R<bool> {
    let m1352 = (p1353, p1354);
    {
        let mut m1352 = m1352;
        loop {
            let step = {
                let (v1, v2) = m1352;
                let m1350 = v2;
                match m1350.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), R<bool>>(({
                            let c1351: Fn16 = v0.clone();
                            c1351.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1352 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Adt0, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let m1355 = f10(f22(v0, v2)?, f11(Str::lit("properties")));
        match m1355 {
            None => {
                Ok(false)
            }
            Some(v3) => {
                Ok(f219(f10(v3, f11(Str::lit("kind")))))
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f188(v0: Bytes, v1: Bytes) -> Bytes {
    let a1356 = v0;
    let a1357 = v1;
    append_bytes(a1356, a1357)
}

pub fn f189(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a1358 = v0.clone();
        length_bytes(a1358)
    };
    let v2: Adt17 = {
        let (h1359, _) = f220(Fn22::F221(v1, v0), {
            let a1360 = v1;
            let a1361 = 1u64;
            nat_add(a1360, a1361)?
        }, Adt17::C0(0u64, List::<Bytes>::nil()))?;
        h1359
    };
    Ok(f207(f207(f45(Str::lit("\"")), f43(match v2 {
        Adt17::C0(_, h1362) => {
            h1362
        }
    })), f45(Str::lit("\""))))
}

pub fn f190(v0: Bytes) -> Option<Bytes> {
    if f36(v0.clone(), f11(Str::lit("Int"))) {
        Some(f11(Str::lit("int")))
    } else {
        if f36(v0.clone(), f11(Str::lit("Int8"))) {
            Some(f11(Str::lit("int8")))
        } else {
            if f36(v0.clone(), f11(Str::lit("Int16"))) {
                Some(f11(Str::lit("int16")))
            } else {
                if f36(v0.clone(), f11(Str::lit("Int32"))) {
                    Some(f11(Str::lit("int32")))
                } else {
                    if f36(v0.clone(), f11(Str::lit("Int64"))) {
                        Some(f11(Str::lit("int64")))
                    } else {
                        if f36(v0.clone(), f11(Str::lit("UInt8"))) {
                            Some(f11(Str::lit("uint8")))
                        } else {
                            if f36(v0.clone(), f11(Str::lit("UInt16"))) {
                                Some(f11(Str::lit("uint16")))
                            } else {
                                if f36(v0.clone(), f11(Str::lit("UInt32"))) {
                                    Some(f11(Str::lit("uint32")))
                                } else {
                                    if f36(v0, f11(Str::lit("UInt64"))) {
                                        Some(f11(Str::lit("uint64")))
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

pub fn f191(v0: List<Adt3>, v1: u64, v2: u64, v3: Bytes) -> R<Adt12> {
    let a1363 = f75(v0.clone(), v2);
    let a1364 = 2u64;
    if nat_eq(a1363, a1364) {
        let v4: Adt15 = {
            let (h1365, _) = f167(Fn13::F222(v1, v0.clone(), v3), {
                let a1367 = {
                    let a1366 = v0.clone();
                    length_list(a1366)
                };
                let a1368 = 2u64;
                nat_add(a1367, a1368)?
            }, Adt15::C0(List::<Adt0>::nil(), {
                let a1369 = v2;
                let a1370 = 1u64;
                nat_add(a1369, a1370)?
            }, true, None::<Adt1>))?;
            h1365
        };
        let m1372 = match v4.clone() {
            Adt15::C0(_, _, _, h1371) => {
                h1371
            }
        };
        match m1372 {
            None => {
                let a1376 = f75(v0.clone(), match v4.clone() {
                    Adt15::C0(_, h1375, _, _) => {
                        h1375
                    }
                });
                let a1377 = 3u64;
                if nat_eq(a1376, a1377) {
                    Ok(Adt12::C0(f79(match v4.clone() {
                        Adt15::C0(h1378, _, _, _) => {
                            h1378
                        }
                    }), {
                        let a1380 = match v4 {
                            Adt15::C0(_, h1379, _, _) => {
                                h1379
                            }
                        };
                        let a1381 = 1u64;
                        nat_add(a1380, a1381)?
                    }, None::<Adt1>))
                } else {
                    Ok(Adt12::C0(Adt0::C5, match v4.clone() {
                        Adt15::C0(_, h1382, _, _) => {
                            h1382
                        }
                    }, f157(f111(v0, v1, match v4 {
                        Adt15::C0(_, h1383, _, _) => {
                            h1383
                        }
                    }), f11(Str::lit("expected `,` or `]`")))))
                }
            }
            Some(_) => {
                Ok(Adt12::C0(Adt0::C5, match v4.clone() {
                    Adt15::C0(_, h1373, _, _) => {
                        h1373
                    }
                }, match v4 {
                    Adt15::C0(_, _, _, h1374) => {
                        h1374
                    }
                }))
            }
        }
    } else {
        Ok(Adt12::C0(Adt0::C5, v2, f157(f111(v0, v1, v2), f11(Str::lit("expected `[`")))))
    }
}

pub fn f192(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a1384 = f75(v0.clone(), v4);
    let a1385 = 2u64;
    if nat_eq(a1384, a1385) {
        Ok(f81(v2, v3.clone(), v4, 6u64, Adt2::C5, match v3 {
            Adt7::C0(_, _, h1386, _, _, _, _, _, _, _, _, _) => {
                h1386
            }
        }))
    } else {
        f123(v0, v1, Adt6::C0(v4, match v2.clone() {
            Adt6::C0(_, h1387, _, _, _, _, _, _, _) => {
                h1387
            }
        }, Adt0::C5, 1u64, match v2.clone() {
            Adt6::C0(_, _, _, _, h1388, _, _, _, _) => {
                h1388
            }
        }, match v2.clone() {
            Adt6::C0(_, _, _, _, _, h1389, _, _, _) => {
                h1389
            }
        }, match v2.clone() {
            Adt6::C0(_, _, _, _, _, _, h1390, _, _) => {
                h1390
            }
        }, None::<Adt1>, match v2 {
            Adt6::C0(_, _, _, _, _, _, _, _, h1391) => {
                h1391
            }
        }), v3)
    }
}

pub fn f193(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: u64) -> R<Adt6> {
    let v5: Bytes = f84(v0.clone(), v3);
    let v6: u64 = {
        let a1394 = f75(v0.clone(), {
            let a1392 = v3;
            let a1393 = 1u64;
            nat_add(a1392, a1393)?
        });
        let a1395 = 9u64;
        if nat_eq(a1394, a1395) {
            3u64
        } else {
            1u64
        }
    };
    let v7: bool = {
        let a1398 = f75(v0.clone(), {
            let a1396 = v3;
            let a1397 = v6;
            nat_add(a1396, a1397)?
        });
        let a1399 = 6u64;
        if nat_eq(a1398, a1399) {
            true
        } else {
            f85({
                let a1402 = f75(v0.clone(), {
                    let a1400 = v3;
                    let a1401 = v6;
                    nat_add(a1400, a1401)?
                });
                let a1403 = 12u64;
                nat_eq(a1402, a1403)
            }, f223(f84(v0.clone(), {
                let a1404 = v3;
                let a1405 = v6;
                nat_add(a1404, a1405)?
            })))
        }
    };
    let v8: bool = {
        let a1408 = f75(v0.clone(), {
            let a1406 = v3;
            let a1407 = 1u64;
            nat_add(a1406, a1407)?
        });
        let a1409 = 0u64;
        nat_eq(a1408, a1409)
    };
    if f85({
        let a1410 = f75(v0.clone(), v3);
        let a1411 = 12u64;
        nat_eq(a1410, a1411)
    }, f85(f155(v5.clone())?, f85(!v7, f85(!v8, !f87(v5)?)))) {
        let v9: Adt12 = f76(v0.clone(), v1, v3)?;
        let m1413 = match v9.clone() {
            Adt12::C0(_, _, h1412) => {
                h1412
            }
        };
        match m1413 {
            None => {
                f192(v0, v1, v2, f104(f6(46u64, v4), match v9.clone() {
                    Adt12::C0(h1415, _, _) => {
                        h1415
                    }
                }, Adt0::C5), match v9 {
                    Adt12::C0(_, h1416, _) => {
                        h1416
                    }
                })
            }
            Some(_) => {
                Ok(f77(v2, match v9 {
                    Adt12::C0(_, _, h1414) => {
                        h1414
                    }
                }))
            }
        }
    } else {
        f120(v2, f6(47u64, v4), v3)
    }
}

pub fn f194(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1417 = f41(v0, v1);
    match m1417 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1462 = {
                    let a1438 = {
                        let a1426 = {
                            let a1420 = {
                                let a1418 = v3;
                                let a1419 = 128u8;
                                fixed_u8::bit_and(a1418, a1419)
                            };
                            let a1421 = 128u8;
                            if equal(a1420, a1421) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1427 = {
                            let a1424 = {
                                let a1422 = v3;
                                let a1423 = 64u8;
                                fixed_u8::bit_and(a1422, a1423)
                            };
                            let a1425 = 64u8;
                            if equal(a1424, a1425) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1426, a1427)?
                    };
                    let a1439 = {
                        let a1436 = {
                            let a1430 = {
                                let a1428 = v3;
                                let a1429 = 32u8;
                                fixed_u8::bit_and(a1428, a1429)
                            };
                            let a1431 = 32u8;
                            if equal(a1430, a1431) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1437 = {
                            let a1434 = {
                                let a1432 = v3;
                                let a1433 = 16u8;
                                fixed_u8::bit_and(a1432, a1433)
                            };
                            let a1435 = 16u8;
                            if equal(a1434, a1435) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1436, a1437)?
                    };
                    nat_add(a1438, a1439)?
                };
                let a1463 = {
                    let a1460 = {
                        let a1448 = {
                            let a1442 = {
                                let a1440 = v3;
                                let a1441 = 8u8;
                                fixed_u8::bit_and(a1440, a1441)
                            };
                            let a1443 = 8u8;
                            if equal(a1442, a1443) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1449 = {
                            let a1446 = {
                                let a1444 = v3;
                                let a1445 = 4u8;
                                fixed_u8::bit_and(a1444, a1445)
                            };
                            let a1447 = 4u8;
                            if equal(a1446, a1447) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1448, a1449)?
                    };
                    let a1461 = {
                        let a1458 = {
                            let a1452 = {
                                let a1450 = v3;
                                let a1451 = 2u8;
                                fixed_u8::bit_and(a1450, a1451)
                            };
                            let a1453 = 2u8;
                            if equal(a1452, a1453) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1459 = {
                            let a1456 = {
                                let a1454 = v3;
                                let a1455 = 1u8;
                                fixed_u8::bit_and(a1454, a1455)
                            };
                            let a1457 = 1u8;
                            if equal(a1456, a1457) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1458, a1459)?
                    };
                    nat_add(a1460, a1461)?
                };
                nat_add(a1462, a1463)?
            };
            if if f85({
                let a1464 = 48u64;
                let a1465 = v4;
                nat_le(a1464, a1465)
            }, {
                let a1466 = v4;
                let a1467 = 57u64;
                nat_le(a1466, a1467)
            }) {
                true
            } else {
                f85({
                    let a1468 = 97u64;
                    let a1469 = v4;
                    nat_le(a1468, a1469)
                }, {
                    let a1470 = v4;
                    let a1471 = 102u64;
                    nat_le(a1470, a1471)
                })
            } {
                Ok(Some({
                    let a1472 = v1;
                    let a1473 = 1u64;
                    nat_add(a1472, a1473)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f195(v0: List<Adt3>, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    let m1475 = match v2.clone() {
        Adt13::C0(_, _, _, _, _, h1474) => {
            h1474
        }
    };
    match m1475 {
        None => {
            let a1477 = match v2.clone() {
                Adt13::C0(_, _, h1476, _, _, _) => {
                    h1476
                }
            };
            let a1478 = 9u64;
            if nat_eq(a1477, a1478) {
                Ok(None::<Adt13>)
            } else {
                Ok(Some(f224(v0, v1, v2)?))
            }
        }
        Some(_) => {
            Ok(None::<Adt13>)
        }
    }
}

pub fn f196(v0: Fn17, p1482: Adt0, p1483: List<(Bytes, Adt0)>) -> Adt0 {
    let m1481 = (p1482, p1483);
    {
        let mut m1481 = m1481;
        loop {
            let step = {
                let (v1, v2) = m1481;
                let m1479 = v2;
                match m1479.uncons() {
                    None => {
                        Err::<(Adt0, List<(Bytes, Adt0)>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<(Bytes, Adt0)>), Adt0>(({
                            let c1480: Fn17 = v0.clone();
                            c1480.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1481 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt0, v1: (Bytes, Adt0)) -> Adt0 {
    Adt0::C7({
        let (h1484, _) = v1;
        h1484
    }, std::rc::Rc::new({
        let (_, h1485) = v1;
        h1485
    }), std::rc::Rc::new(v0))
}

pub fn f198(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f225(Fn23::F226, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f199(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f227(Fn24::F228, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f200(v0: Fn18, p1489: List<List<(Bytes, Adt0)>>, p1490: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m1488 = (p1489, p1490);
    {
        let mut m1488 = m1488;
        loop {
            let step = {
                let (v1, v2) = m1488;
                let m1486 = v2;
                match m1486.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c1487: Fn18 = v0.clone();
                            c1487.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f202(v0: Fn19, p1495: u64, p1496: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m1494 = (p1495, p1496);
    {
        let mut m1494 = m1494;
        loop {
            let step = {
                let (v1, v2) = m1494;
                let m1491 = v1;
                if m1491 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1491 - 1;
                    let m1493 = {
                        let c1492: Fn19 = v0.clone();
                        c1492.apply(v2.clone())?
                    };
                    match m1493 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f229(v0)
}

pub fn f204(v0: Adt0) -> Option<Adt0> {
    let m1497 = v0;
    match m1497 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(r1498) => {
            let v1 = (*r1498).clone();
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

pub fn f205(v0: u64) -> Bytes {
    f47(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f206(v0: Bytes, v1: Adt9) -> R<Adt9> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h1499, _, _, _) => {
            h1499
        }
    };
    let m1502 = f230(v0, {
        let a1500 = v2;
        let a1501 = 2u64;
        nat_add(a1500, a1501)?
    })?;
    match m1502 {
        None => {
            Ok(f171(v1, v2, f45(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a1503 = 55296u64;
            let a1504 = v3;
            if nat_le(a1503, a1504) {
                let a1505 = v3;
                let a1506 = 57343u64;
                if nat_le(a1505, a1506) {
                    Ok(f171(v1, v2, f207(f207(f45(Str::lit("`\\u")), f231(v3)), f45(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f173(v1, {
                        let a1507 = v2;
                        let a1508 = 6u64;
                        nat_add(a1507, a1508)?
                    }, f232(v3)?))
                }
            } else {
                Ok(f173(v1, {
                    let a1509 = v2;
                    let a1510 = 6u64;
                    nat_add(a1509, a1510)?
                }, f232(v3)?))
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: Bytes) -> Bytes {
    let a1511 = v0;
    let a1512 = v1;
    append_bytes(a1511, a1512)
}

pub fn f208(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f170(v0, v1)?;
    let a1513 = v2;
    let a1514 = 128u64;
    if nat_lt(a1513, a1514) {
        Ok(1u64)
    } else {
        let a1515 = v2;
        let a1516 = 224u64;
        if nat_lt(a1515, a1516) {
            Ok(2u64)
        } else {
            let a1517 = v2;
            let a1518 = 240u64;
            if nat_lt(a1517, a1518) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f209(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f233(v1, v0, v2)
}

pub fn f210(v0: Fn20, p1522: (Option<Bytes>, List<Bytes>), p1523: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m1521 = (p1522, p1523);
    {
        let mut m1521 = m1521;
        loop {
            let step = {
                let (v1, v2) = m1521;
                let m1519 = v2;
                match m1519.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c1520: Fn20 = v0.clone();
                            c1520.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f234(v0, v1)
}

pub fn f212(v0: Bytes, v1: u64) -> bool {
    if f26(v0.clone(), v1, 123u8) {
        true
    } else {
        if f26(v0.clone(), v1, 125u8) {
            true
        } else {
            if f26(v0.clone(), v1, 91u8) {
                true
            } else {
                if f26(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f26(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f26(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f26(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f26(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f26(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f26(v0, v1, 34u8)
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

pub fn f213(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f235(f139(v0, v1)?) {
        Ok(Some({
            let a1524 = v1;
            let a1525 = 1u64;
            nat_add(a1524, a1525)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f214(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f139(v0, v1)?;
    let a1526 = 47u64;
    let a1527 = v2;
    if nat_lt(a1526, a1527) {
        let a1528 = v2;
        let a1529 = 58u64;
        if nat_lt(a1528, a1529) {
            Ok(Some({
                let a1530 = v1;
                let a1531 = 1u64;
                nat_add(a1530, a1531)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f215(v0: Adt0, v1: Bytes) -> R<bool> {
    let m1532 = f10(v0, f11(Str::lit("properties")));
    match m1532 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m1533 = f10(v2, f11(Str::lit("kind")));
            match m1533 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m1534 = f34(f10(v3.clone(), f11(Str::lit("const"))));
                    if match m1534 {
                        None => {
                            false
                        }
                        Some(v4) => {
                            f36(v4, v1.clone())
                        }
                    } {
                        Ok(true)
                    } else {
                        let m1535 = f182(f10(v3, f11(Str::lit("enum"))));
                        match m1535 {
                            None => {
                                Ok(false)
                            }
                            Some(v5) => {
                                f186(Fn16::F236(v1), false, f185(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f216(v0: List<Adt0>) -> List<Adt0> {
    f237(Fn25::F238, List::<Adt0>::nil(), v0)
}

pub fn f217(v0: Fn21, p1540: u64, p1541: Adt16) -> (Adt16, bool) {
    let m1539 = (p1540, p1541);
    {
        let mut m1539 = m1539;
        loop {
            let step = {
                let (v1, v2) = m1539;
                let m1536 = v1;
                if m1536 == 0 {
                    Err::<(u64, Adt16), (Adt16, bool)>((v2, false))
                } else {
                    let v3 = m1536 - 1;
                    let m1538 = {
                        let c1537: Fn21 = v0.clone();
                        c1537.apply(v2.clone())
                    };
                    match m1538 {
                        None => {
                            Err::<(u64, Adt16), (Adt16, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt16), (Adt16, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: Adt16) -> Option<Adt16> {
    f239(v0)
}

pub fn f219(v0: Option<Adt0>) -> bool {
    let m1542 = v0;
    match m1542 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f220(v0: Fn22, p1547: u64, p1548: Adt17) -> R<(Adt17, bool)> {
    let m1546 = (p1547, p1548);
    {
        let mut m1546 = m1546;
        loop {
            let step = {
                let (v1, v2) = m1546;
                let m1543 = v1;
                if m1543 == 0 {
                    Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1543 - 1;
                    let m1545 = {
                        let c1544: Fn22 = v0.clone();
                        c1544.apply(v2.clone())?
                    };
                    match m1545 {
                        None => {
                            Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt17), R<(Adt17, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: u64, v1: Bytes, v2: Adt17) -> R<Option<Adt17>> {
    f240(v1, v0, v2)
}

pub fn f222(v0: u64, v1: List<Adt3>, v2: Bytes, v3: Adt15) -> R<Option<Adt15>> {
    f241(v1, v0, v2, v3)
}

pub fn f223(v0: Bytes) -> bool {
    let m1549 = f127(v0);
    match m1549 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f224(v0: List<Adt3>, v1: u64, v2: Adt13) -> R<Adt13> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(_, _, h1550, _, _, _) => {
            h1550
        }
    };
    let a1551 = v3;
    let a1552 = 0u64;
    if nat_eq(a1551, a1552) {
        f242(v0, v1, v2)
    } else {
        let a1553 = v3;
        let a1554 = 1u64;
        if nat_eq(a1553, a1554) {
            f243(v0, v2)
        } else {
            let a1555 = v3;
            let a1556 = 2u64;
            if nat_eq(a1555, a1556) {
                f244(v0, v1, v2)
            } else {
                let a1557 = v3;
                let a1558 = 3u64;
                if nat_eq(a1557, a1558) {
                    f245(v0, v2)
                } else {
                    let a1559 = v3;
                    let a1560 = 4u64;
                    if nat_eq(a1559, a1560) {
                        f246(v0, v1, v2, 3u64, f150(Str::lit("expected `,` or `]`")))
                    } else {
                        f246(v0, v1, v2, 1u64, f150(Str::lit("expected `,` or `}`")))
                    }
                }
            }
        }
    }
}

pub fn f225(v0: Fn23, p1564: List<(Bytes, Adt0)>, p1565: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m1563 = (p1564, p1565);
    {
        let mut m1563 = m1563;
        loop {
            let step = {
                let (v1, v2) = m1563;
                let m1561 = v2;
                match m1561.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c1562: Fn23 = v0.clone();
                            c1562.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f227(v0: Fn24, p1569: List<List<(Bytes, Adt0)>>, p1570: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m1568 = (p1569, p1570);
    {
        let mut m1568 = m1568;
        loop {
            let step = {
                let (v1, v2) = m1568;
                let m1566 = v2;
                match m1566.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c1567: Fn24 = v0.clone();
                            c1567.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f229(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a1572 = {
        let a1571 = v0.clone();
        length_list(a1571)
    };
    let a1573 = 1u64;
    if nat_le(a1572, a1573) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f247(v0)?))
    }
}

pub fn f230(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1574 = f248(v0.clone(), v1)?;
    match m1574 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1577 = f248(v0.clone(), {
                let a1575 = v1;
                let a1576 = 1u64;
                nat_add(a1575, a1576)?
            })?;
            match m1577 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m1580 = f248(v0.clone(), {
                        let a1578 = v1;
                        let a1579 = 2u64;
                        nat_add(a1578, a1579)?
                    })?;
                    match m1580 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m1583 = f248(v0, {
                                let a1581 = v1;
                                let a1582 = 3u64;
                                nat_add(a1581, a1582)?
                            })?;
                            match m1583 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a1594 = {
                                            let a1592 = {
                                                let a1590 = {
                                                    let a1588 = {
                                                        let a1586 = {
                                                            let a1584 = v2;
                                                            let a1585 = 16u64;
                                                            nat_mul(a1584, a1585)?
                                                        };
                                                        let a1587 = v3;
                                                        nat_add(a1586, a1587)?
                                                    };
                                                    let a1589 = 16u64;
                                                    nat_mul(a1588, a1589)?
                                                };
                                                let a1591 = v4;
                                                nat_add(a1590, a1591)?
                                            };
                                            let a1593 = 16u64;
                                            nat_mul(a1592, a1593)?
                                        };
                                        let a1595 = v5;
                                        nat_add(a1594, a1595)?
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

pub fn f231(v0: u64) -> Bytes {
    f207(f207(f249(v0, 4096u64), f249(v0, 256u64)), f207(f249(v0, 16u64), f250(v0)))
}

pub fn f232(v0: u64) -> R<Bytes> {
    let a1596 = v0;
    let a1597 = 128u64;
    if nat_lt(a1596, a1597) {
        Ok(f205(v0))
    } else {
        let a1598 = v0;
        let a1599 = 2048u64;
        if nat_lt(a1598, a1599) {
            Ok(f207(f205({
                let a1603 = 192u64;
                let a1604 = {
                    let a1600 = v0;
                    let a1601 = 64u64;
                    let a1602 = 0u64;
                    nat_quot(a1600, a1601, a1602)
                };
                nat_add(a1603, a1604)?
            }), f205({
                let a1608 = 128u64;
                let a1609 = {
                    let a1605 = v0;
                    let a1606 = 64u64;
                    let a1607 = 0u64;
                    nat_rem(a1605, a1606, a1607)
                };
                nat_add(a1608, a1609)?
            })))
        } else {
            Ok(f207(f205({
                let a1613 = 224u64;
                let a1614 = {
                    let a1610 = v0;
                    let a1611 = 4096u64;
                    let a1612 = 0u64;
                    nat_quot(a1610, a1611, a1612)
                };
                nat_add(a1613, a1614)?
            }), f207(f205({
                let a1621 = 128u64;
                let a1622 = {
                    let a1618 = {
                        let a1615 = v0;
                        let a1616 = 64u64;
                        let a1617 = 0u64;
                        nat_quot(a1615, a1616, a1617)
                    };
                    let a1619 = 64u64;
                    let a1620 = 0u64;
                    nat_rem(a1618, a1619, a1620)
                };
                nat_add(a1621, a1622)?
            }), f205({
                let a1626 = 128u64;
                let a1627 = {
                    let a1623 = v0;
                    let a1624 = 64u64;
                    let a1625 = 0u64;
                    nat_rem(a1623, a1624, a1625)
                };
                nat_add(a1626, a1627)?
            }))))
        }
    }
}

pub fn f233(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1628 = v2;
    let a1629 = v1;
    if nat_lt(a1628, a1629) {
        let v3: u64 = f170(v0, v2)?;
        let a1630 = v3;
        let a1631 = 32u64;
        if nat_lt(a1630, a1631) {
            Ok(None::<u64>)
        } else {
            let a1632 = v3;
            let a1633 = 34u64;
            if nat_eq(a1632, a1633) {
                Ok(None::<u64>)
            } else {
                let a1634 = v3;
                let a1635 = 92u64;
                if nat_eq(a1634, a1635) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1636 = v2;
                        let a1637 = 1u64;
                        nat_add(a1636, a1637)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f234(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m1639 = {
        let (h1638, _) = v0;
        h1638
    };
    match m1639 {
        None => {
            (Some(v1), {
                let (_, h1640) = v0;
                h1640
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a1641 = v2;
                let a1642 = v1;
                append_bytes(a1641, a1642)
            }, {
                let (_, h1643) = v0;
                h1643
            }))
        }
    }
}

pub fn f235(v0: u64) -> bool {
    if f138(v0) {
        true
    } else {
        let a1644 = 47u64;
        let a1645 = v0;
        if if nat_lt(a1644, a1645) {
            let a1646 = v0;
            let a1647 = 58u64;
            nat_lt(a1646, a1647)
        } else {
            false
        } {
            true
        } else {
            let a1648 = v0;
            let a1649 = 46u64;
            if nat_eq(a1648, a1649) {
                true
            } else {
                let a1650 = v0;
                let a1651 = 39u64;
                if nat_eq(a1650, a1651) {
                    true
                } else {
                    let a1652 = v0;
                    let a1653 = 47u64;
                    if nat_eq(a1652, a1653) {
                        true
                    } else {
                        let a1654 = v0;
                        let a1655 = 45u64;
                        nat_eq(a1654, a1655)
                    }
                }
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: bool, v2: Adt0) -> bool {
    if v1 {
        true
    } else {
        let m1656 = f50(v2);
        match m1656 {
            None => {
                false
            }
            Some(v3) => {
                f36(v3, v0)
            }
        }
    }
}

pub fn f237(v0: Fn25, p1660: List<Adt0>, p1661: List<Adt0>) -> List<Adt0> {
    let m1659 = (p1660, p1661);
    {
        let mut m1659 = m1659;
        loop {
            let step = {
                let (v1, v2) = m1659;
                let m1657 = v2;
                match m1657.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c1658: Fn25 = v0.clone();
                            c1658.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1659 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f239(v0: Adt16) -> Option<Adt16> {
    let m1663 = f251(match v0.clone() {
        Adt16::C0(h1662, _) => {
            h1662
        }
    });
    match m1663 {
        None => {
            None::<Adt16>
        }
        Some(v1) => {
            Some(Adt16::C0(match v1.clone() {
                Adt18::C0(_, h1664) => {
                    h1664
                }
            }, List::cons(match v1 {
                Adt18::C0(h1665, _) => {
                    h1665
                }
            }, match v0 {
                Adt16::C0(_, h1666) => {
                    h1666
                }
            })))
        }
    }
}

pub fn f240(v0: Bytes, v1: u64, v2: Adt17) -> R<Option<Adt17>> {
    let v3: u64 = match v2.clone() {
        Adt17::C0(h1667, _) => {
            h1667
        }
    };
    let a1668 = v3;
    let a1669 = v1;
    if nat_lt(a1668, a1669) {
        let v4: u64 = f170(v0.clone(), v3)?;
        let a1670 = v4;
        let a1671 = 32u64;
        if if nat_lt(a1670, a1671) {
            true
        } else {
            let a1672 = v4;
            let a1673 = 34u64;
            if nat_eq(a1672, a1673) {
                true
            } else {
                let a1674 = v4;
                let a1675 = 92u64;
                nat_eq(a1674, a1675)
            }
        } {
            Ok(Some(Adt17::C0({
                let a1676 = v3;
                let a1677 = 1u64;
                nat_add(a1676, a1677)?
            }, List::cons(f252(v4), match v2 {
                Adt17::C0(_, h1678) => {
                    h1678
                }
            }))))
        } else {
            let v5: u64 = f172(v0.clone(), v1, v3)?;
            Ok(Some(Adt17::C0(v5, List::cons(f47(v0, v3, {
                let a1679 = v5;
                let a1680 = v3;
                nat_sub(a1679, a1680)
            }), match v2 {
                Adt17::C0(_, h1681) => {
                    h1681
                }
            }))))
        }
    } else {
        Ok(None::<Adt17>)
    }
}

pub fn f241(v0: List<Adt3>, v1: u64, v2: Bytes, v3: Adt15) -> R<Option<Adt15>> {
    let m1683 = match v3.clone() {
        Adt15::C0(_, _, _, h1682) => {
            h1682
        }
    };
    match m1683 {
        None => {
            if match v3.clone() {
                Adt15::C0(_, _, h1684, _) => {
                    h1684
                }
            } {
                let v5: u64 = match v3.clone() {
                    Adt15::C0(_, h1685, _, _) => {
                        h1685
                    }
                };
                let a1686 = f75(v0.clone(), v5);
                let a1687 = 3u64;
                if nat_eq(a1686, a1687) {
                    Ok(None::<Adt15>)
                } else {
                    let v6: Adt12 = f103(v0.clone(), v1, v5, v2)?;
                    let m1689 = match v6.clone() {
                        Adt12::C0(_, _, h1688) => {
                            h1688
                        }
                    };
                    match m1689 {
                        None => {
                            let a1693 = f75(v0, match v6.clone() {
                                Adt12::C0(_, h1692, _) => {
                                    h1692
                                }
                            });
                            let a1694 = 6u64;
                            if nat_eq(a1693, a1694) {
                                Ok(Some(Adt15::C0(List::cons(match v6.clone() {
                                    Adt12::C0(h1695, _, _) => {
                                        h1695
                                    }
                                }, match v3 {
                                    Adt15::C0(h1696, _, _, _) => {
                                        h1696
                                    }
                                }), {
                                    let a1698 = match v6 {
                                        Adt12::C0(_, h1697, _) => {
                                            h1697
                                        }
                                    };
                                    let a1699 = 1u64;
                                    nat_add(a1698, a1699)?
                                }, true, None::<Adt1>)))
                            } else {
                                Ok(Some(Adt15::C0(List::cons(match v6.clone() {
                                    Adt12::C0(h1700, _, _) => {
                                        h1700
                                    }
                                }, match v3 {
                                    Adt15::C0(h1701, _, _, _) => {
                                        h1701
                                    }
                                }), match v6 {
                                    Adt12::C0(_, h1702, _) => {
                                        h1702
                                    }
                                }, false, None::<Adt1>)))
                            }
                        }
                        Some(_) => {
                            Ok(Some(Adt15::C0(match v3 {
                                Adt15::C0(h1690, _, _, _) => {
                                    h1690
                                }
                            }, v5, false, match v6 {
                                Adt12::C0(_, _, h1691) => {
                                    h1691
                                }
                            })))
                        }
                    }
                }
            } else {
                Ok(None::<Adt15>)
            }
        }
        Some(_) => {
            Ok(None::<Adt15>)
        }
    }
}

pub fn f242(v0: List<Adt3>, v1: u64, v2: Adt13) -> R<Adt13> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(h1703, _, _, _, _, _) => {
            h1703
        }
    };
    let a1705 = 256u64;
    let a1706 = match v2.clone() {
        Adt13::C0(_, h1704, _, _, _, _) => {
            h1704
        }
    };
    if nat_lt(a1705, a1706) {
        Ok(f253(v2, f111(v0, v1, v3), f150(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v4: u64 = f134(v0.clone(), v3);
        let a1707 = v4;
        let a1708 = 99u64;
        if nat_eq(a1707, a1708) {
            Ok(f253(v2, v1, f150(Str::lit("expected a value, found the end of the text"))))
        } else {
            let a1709 = v4;
            let a1710 = 11u64;
            if nat_eq(a1709, a1710) {
                Ok(f254(v2, {
                    let a1711 = v3;
                    let a1712 = 1u64;
                    nat_add(a1711, a1712)?
                }, Adt0::C2(f137(v0, v3))))
            } else {
                let a1713 = v4;
                let a1714 = 2u64;
                if nat_eq(a1713, a1714) {
                    f255(v2, {
                        let a1715 = v3;
                        let a1716 = 1u64;
                        nat_add(a1715, a1716)?
                    }, Adt14::C0(List::<Adt0>::nil()), 1u64)
                } else {
                    let a1717 = v4;
                    let a1718 = 0u64;
                    if nat_eq(a1717, a1718) {
                        f255(v2, {
                            let a1719 = v3;
                            let a1720 = 1u64;
                            nat_add(a1719, a1720)?
                        }, Adt14::C1(Bytes::lit(&[]), 0u64, List::<(Bytes, Adt0)>::nil()), 2u64)
                    } else {
                        let a1721 = v4;
                        let a1722 = 12u64;
                        if nat_eq(a1721, a1722) {
                            f256(v0.clone(), v1, v2, f111(v0, v1, v3))
                        } else {
                            Ok(f253(v2, f111(v0.clone(), v1, v3), f188(f150(Str::lit("expected a value, found ")), f100(v0, v3)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f243(v0: List<Adt3>, v1: Adt13) -> R<Adt13> {
    let a1724 = f134(v0, match v1.clone() {
        Adt13::C0(h1723, _, _, _, _, _) => {
            h1723
        }
    });
    let a1725 = 3u64;
    if nat_eq(a1724, a1725) {
        f257(v1)
    } else {
        Ok(Adt13::C0(match v1.clone() {
            Adt13::C0(h1726, _, _, _, _, _) => {
                h1726
            }
        }, match v1.clone() {
            Adt13::C0(_, h1727, _, _, _, _) => {
                h1727
            }
        }, 0u64, match v1 {
            Adt13::C0(_, _, _, h1728, _, _) => {
                h1728
            }
        }, Adt0::C5, None::<Adt1>))
    }
}

pub fn f244(v0: List<Adt3>, v1: u64, v2: Adt13) -> R<Adt13> {
    let m1730 = match v2.clone() {
        Adt13::C0(_, _, _, h1729, _, _) => {
            h1729
        }
    };
    match m1730.uncons() {
        None => {
            Ok(f253(v2, 0u64, f150(Str::lit("no object is open"))))
        }
        Some((v3, v4)) => {
            let m1731 = v3;
            match m1731 {
                Adt14::C0(_) => {
                    Ok(f253(v2, 0u64, f150(Str::lit("no object is open"))))
                }
                Adt14::C1(_, _, v8) => {
                    let a1733 = f134(v0.clone(), match v2.clone() {
                        Adt13::C0(h1732, _, _, _, _, _) => {
                            h1732
                        }
                    });
                    let a1734 = 1u64;
                    if nat_eq(a1733, a1734) {
                        f257(v2)
                    } else {
                        f258(v0, v1, v2, v8, v4)
                    }
                }
            }
        }
    }
}

pub fn f245(v0: List<Adt3>, v1: Adt13) -> R<Adt13> {
    let m1736 = match v1.clone() {
        Adt13::C0(_, _, _, h1735, _, _) => {
            h1735
        }
    };
    match m1736.uncons() {
        None => {
            Ok(Adt13::C0(match v1.clone() {
                Adt13::C0(h1737, _, _, _, _, _) => {
                    h1737
                }
            }, match v1.clone() {
                Adt13::C0(_, h1738, _, _, _, _) => {
                    h1738
                }
            }, 9u64, List::<Adt14>::nil(), match v1 {
                Adt13::C0(_, _, _, _, h1739, _) => {
                    h1739
                }
            }, None::<Adt1>))
        }
        Some((v2, v3)) => {
            let m1740 = v2;
            match m1740 {
                Adt14::C0(v4) => {
                    f259(v0, v1.clone(), List::cons(Adt14::C0(List::cons(match v1 {
                        Adt13::C0(_, _, _, _, h1741, _) => {
                            h1741
                        }
                    }, v4)), v3), 1u64, 4u64)
                }
                Adt14::C1(v5, v6, v7) => {
                    if f112(v7.clone(), v5.clone()) {
                        Ok(f253(v1, v6, f188(f188(f150(Str::lit("the member `")), v5), f150(Str::lit("` is written twice")))))
                    } else {
                        f259(v0, v1.clone(), List::cons(Adt14::C1(Bytes::lit(&[]), 0u64, List::cons((v5, match v1 {
                            Adt13::C0(_, _, _, _, h1742, _) => {
                                h1742
                            }
                        }), v7)), v3), 2u64, 5u64)
                    }
                }
            }
        }
    }
}

pub fn f246(v0: List<Adt3>, v1: u64, v2: Adt13, v3: u64, v4: Bytes) -> R<Adt13> {
    let a1744 = f134(v0.clone(), match v2.clone() {
        Adt13::C0(h1743, _, _, _, _, _) => {
            h1743
        }
    });
    let a1745 = v3;
    if nat_eq(a1744, a1745) {
        f257(v2)
    } else {
        Ok(f253(v2.clone(), f111(v0, v1, match v2 {
            Adt13::C0(h1746, _, _, _, _, _) => {
                h1746
            }
        }), v4))
    }
}

pub fn f247(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f199({
        let (_, h1747) = {
            let (h1748, _) = f260(Fn26::F261, {
                let a1750 = {
                    let a1749 = v0.clone();
                    length_list(a1749)
                };
                let a1751 = 1u64;
                nat_add(a1750, a1751)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h1748
        };
        h1747
    }))
}

pub fn f248(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1752 = f41(v0, v1);
    match m1752 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f262(v2)
        }
    }
}

pub fn f249(v0: u64, v1: u64) -> Bytes {
    f250({
        let a1753 = v0;
        let a1754 = v1;
        let a1755 = 0u64;
        nat_quot(a1753, a1754, a1755)
    })
}

pub fn f250(v0: u64) -> Bytes {
    f47(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1756 = v0;
        let a1757 = 16u64;
        let a1758 = 0u64;
        nat_rem(a1756, a1757, a1758)
    }, 1u64)
}

pub fn f251(v0: Adt0) -> Option<Adt18> {
    let m1759 = v0;
    match m1759 {
        Adt0::C0(_) => {
            None::<Adt18>
        }
        Adt0::C1(_, _) => {
            None::<Adt18>
        }
        Adt0::C2(_) => {
            None::<Adt18>
        }
        Adt0::C3(_) => {
            None::<Adt18>
        }
        Adt0::C4(_) => {
            None::<Adt18>
        }
        Adt0::C5 => {
            None::<Adt18>
        }
        Adt0::C6(r1760, r1761) => {
            let v1 = (*r1760).clone();
            let v2 = (*r1761).clone();
            Some(Adt18::C0(v1, v2))
        }
        Adt0::C7(_, _, _) => {
            None::<Adt18>
        }
    }
}

pub fn f252(v0: u64) -> Bytes {
    let a1762 = v0;
    let a1763 = 34u64;
    if nat_eq(a1762, a1763) {
        f45(Str::lit("\\\""))
    } else {
        let a1764 = v0;
        let a1765 = 92u64;
        if nat_eq(a1764, a1765) {
            f45(Str::lit("\\\\"))
        } else {
            let a1766 = v0;
            let a1767 = 8u64;
            if nat_eq(a1766, a1767) {
                f45(Str::lit("\\b"))
            } else {
                let a1768 = v0;
                let a1769 = 12u64;
                if nat_eq(a1768, a1769) {
                    f45(Str::lit("\\f"))
                } else {
                    let a1770 = v0;
                    let a1771 = 10u64;
                    if nat_eq(a1770, a1771) {
                        f45(Str::lit("\\n"))
                    } else {
                        let a1772 = v0;
                        let a1773 = 13u64;
                        if nat_eq(a1772, a1773) {
                            f45(Str::lit("\\r"))
                        } else {
                            let a1774 = v0;
                            let a1775 = 9u64;
                            if nat_eq(a1774, a1775) {
                                f45(Str::lit("\\t"))
                            } else {
                                f207(f45(Str::lit("\\u00")), f263(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f253(v0: Adt13, v1: u64, v2: Bytes) -> Adt13 {
    Adt13::C0(match v0.clone() {
        Adt13::C0(h1776, _, _, _, _, _) => {
            h1776
        }
    }, match v0.clone() {
        Adt13::C0(_, h1777, _, _, _, _) => {
            h1777
        }
    }, 9u64, match v0.clone() {
        Adt13::C0(_, _, _, h1778, _, _) => {
            h1778
        }
    }, match v0 {
        Adt13::C0(_, _, _, _, h1779, _) => {
            h1779
        }
    }, Some(f264(v1, v2)))
}

pub fn f254(v0: Adt13, v1: u64, v2: Adt0) -> Adt13 {
    Adt13::C0(v1, match v0.clone() {
        Adt13::C0(_, h1780, _, _, _, _) => {
            h1780
        }
    }, 3u64, match v0 {
        Adt13::C0(_, _, _, h1781, _, _) => {
            h1781
        }
    }, v2, None::<Adt1>)
}

pub fn f255(v0: Adt13, v1: u64, v2: Adt14, v3: u64) -> R<Adt13> {
    Ok(Adt13::C0(v1, {
        let a1783 = match v0.clone() {
            Adt13::C0(_, h1782, _, _, _, _) => {
                h1782
            }
        };
        let a1784 = 1u64;
        nat_add(a1783, a1784)?
    }, v3, List::cons(v2, match v0 {
        Adt13::C0(_, _, _, h1785, _, _) => {
            h1785
        }
    }), Adt0::C5, None::<Adt1>))
}

pub fn f256(v0: List<Adt3>, _: u64, v2: Adt13, v3: u64) -> R<Adt13> {
    let v4: u64 = match v2.clone() {
        Adt13::C0(h1786, _, _, _, _, _) => {
            h1786
        }
    };
    let v5: Bytes = f137(v0.clone(), v4);
    let a1787 = v5.clone();
    let a1788 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a1787, a1788) {
        Ok(f254(v2, {
            let a1789 = v4;
            let a1790 = 1u64;
            nat_add(a1789, a1790)?
        }, Adt0::C0(true)))
    } else {
        let a1791 = v5.clone();
        let a1792 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        if equal(a1791, a1792) {
            Ok(f254(v2, {
                let a1793 = v4;
                let a1794 = 1u64;
                nat_add(a1793, a1794)?
            }, Adt0::C0(false)))
        } else {
            if f265(v5.clone())? {
                let m1795 = f266(v5.clone())?;
                match m1795 {
                    None => {
                        Ok(f253(v2, v3, f188(f188(f150(Str::lit("`")), v5), f150(Str::lit("` does not fit a 64-bit integer")))))
                    }
                    Some(v6) => {
                        Ok(f254(v2, {
                            let a1796 = v4;
                            let a1797 = 1u64;
                            nat_add(a1796, a1797)?
                        }, v6))
                    }
                }
            } else {
                if f86(v5.clone())? {
                    let a1800 = f134(v0, {
                        let a1798 = v4;
                        let a1799 = 1u64;
                        nat_add(a1798, a1799)?
                    });
                    let a1801 = 0u64;
                    if nat_eq(a1800, a1801) {
                        f255(v2, {
                            let a1802 = v4;
                            let a1803 = 2u64;
                            nat_add(a1802, a1803)?
                        }, Adt14::C1(Bytes::lit(&[]), 0u64, List::cons((f150(Str::lit("kind")), Adt0::C2(v5)), List::<(Bytes, Adt0)>::nil())), 2u64)
                    } else {
                        Ok(f254(v2, {
                            let a1804 = v4;
                            let a1805 = 1u64;
                            nat_add(a1804, a1805)?
                        }, Adt0::C2(v5)))
                    }
                } else {
                    Ok(f253(v2, v3, f188(f188(f150(Str::lit("`")), v5), f150(Str::lit("` is not a name; quote it to write a string")))))
                }
            }
        }
    }
}

pub fn f257(v0: Adt13) -> R<Adt13> {
    let m1807 = match v0.clone() {
        Adt13::C0(_, _, _, h1806, _, _) => {
            h1806
        }
    };
    match m1807.uncons() {
        None => {
            Ok(f253(v0, 0u64, f150(Str::lit("nothing to close"))))
        }
        Some((v1, v2)) => {
            let m1808 = v1;
            match m1808 {
                Adt14::C0(v3) => {
                    Ok(Adt13::C0({
                        let a1810 = match v0.clone() {
                            Adt13::C0(h1809, _, _, _, _, _) => {
                                h1809
                            }
                        };
                        let a1811 = 1u64;
                        nat_add(a1810, a1811)?
                    }, {
                        let a1813 = match v0 {
                            Adt13::C0(_, h1812, _, _, _, _) => {
                                h1812
                            }
                        };
                        let a1814 = 1u64;
                        nat_sub(a1813, a1814)
                    }, 3u64, v2, Adt0::C3(std::rc::Rc::new(f135(v3))), None::<Adt1>))
                }
                Adt14::C1(_, _, v6) => {
                    Ok(Adt13::C0({
                        let a1816 = match v0.clone() {
                            Adt13::C0(h1815, _, _, _, _, _) => {
                                h1815
                            }
                        };
                        let a1817 = 1u64;
                        nat_add(a1816, a1817)?
                    }, {
                        let a1819 = match v0 {
                            Adt13::C0(_, h1818, _, _, _, _) => {
                                h1818
                            }
                        };
                        let a1820 = 1u64;
                        nat_sub(a1819, a1820)
                    }, 3u64, v2, Adt0::C4(std::rc::Rc::new(f162(f163(v6)?))), None::<Adt1>))
                }
            }
        }
    }
}

pub fn f258(v0: List<Adt3>, v1: u64, v2: Adt13, v3: List<(Bytes, Adt0)>, v4: List<Adt14>) -> R<Adt13> {
    let v5: u64 = match v2.clone() {
        Adt13::C0(h1821, _, _, _, _, _) => {
            h1821
        }
    };
    let v6: u64 = f134(v0.clone(), v5);
    let v7: Bytes = f137(v0.clone(), v5);
    let a1822 = v6;
    let a1823 = 11u64;
    if if nat_eq(a1822, a1823) {
        true
    } else {
        let a1824 = v6;
        let a1825 = 12u64;
        if nat_eq(a1824, a1825) {
            f86(v7.clone())?
        } else {
            false
        }
    } {
        let a1828 = f134(v0.clone(), {
            let a1826 = v5;
            let a1827 = 1u64;
            nat_add(a1826, a1827)?
        });
        let a1829 = 7u64;
        if nat_eq(a1828, a1829) {
            Ok(Adt13::C0({
                let a1830 = v5;
                let a1831 = 2u64;
                nat_add(a1830, a1831)?
            }, match v2 {
                Adt13::C0(_, h1832, _, _, _, _) => {
                    h1832
                }
            }, 0u64, List::cons(Adt14::C1(v7, f111(v0, v1, v5), v3), v4), Adt0::C5, None::<Adt1>))
        } else {
            Ok(f253(v2, f111(v0, v1, {
                let a1833 = v5;
                let a1834 = 1u64;
                nat_add(a1833, a1834)?
            }), f150(Str::lit("expected `:` after a member name"))))
        }
    } else {
        Ok(f253(v2, f111(v0, v1, v5), f150(Str::lit("expected a member name"))))
    }
}

pub fn f259(v0: List<Adt3>, v1: Adt13, v2: List<Adt14>, v3: u64, v4: u64) -> R<Adt13> {
    let a1836 = f134(v0, match v1.clone() {
        Adt13::C0(h1835, _, _, _, _, _) => {
            h1835
        }
    });
    let a1837 = 6u64;
    if nat_eq(a1836, a1837) {
        Ok(Adt13::C0({
            let a1839 = match v1.clone() {
                Adt13::C0(h1838, _, _, _, _, _) => {
                    h1838
                }
            };
            let a1840 = 1u64;
            nat_add(a1839, a1840)?
        }, match v1 {
            Adt13::C0(_, h1841, _, _, _, _) => {
                h1841
            }
        }, v3, v2, Adt0::C5, None::<Adt1>))
    } else {
        Ok(Adt13::C0(match v1.clone() {
            Adt13::C0(h1842, _, _, _, _, _) => {
                h1842
            }
        }, match v1 {
            Adt13::C0(_, h1843, _, _, _, _) => {
                h1843
            }
        }, v4, v2, Adt0::C5, None::<Adt1>))
    }
}

pub fn f260(v0: Fn26, p1848: u64, p1849: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m1847 = (p1848, p1849);
    {
        let mut m1847 = m1847;
        loop {
            let step = {
                let (v1, v2) = m1847;
                let m1844 = v1;
                if m1844 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1844 - 1;
                    let m1846 = {
                        let c1845: Fn26 = v0.clone();
                        c1845.apply(v2.clone())?
                    };
                    match m1846 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f267(v0)
}

pub fn f262(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1894 = {
            let a1870 = {
                let a1858 = {
                    let a1852 = {
                        let a1850 = v1;
                        let a1851 = 128u8;
                        fixed_u8::bit_and(a1850, a1851)
                    };
                    let a1853 = 128u8;
                    if equal(a1852, a1853) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1859 = {
                    let a1856 = {
                        let a1854 = v1;
                        let a1855 = 64u8;
                        fixed_u8::bit_and(a1854, a1855)
                    };
                    let a1857 = 64u8;
                    if equal(a1856, a1857) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1858, a1859)?
            };
            let a1871 = {
                let a1868 = {
                    let a1862 = {
                        let a1860 = v1;
                        let a1861 = 32u8;
                        fixed_u8::bit_and(a1860, a1861)
                    };
                    let a1863 = 32u8;
                    if equal(a1862, a1863) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1869 = {
                    let a1866 = {
                        let a1864 = v1;
                        let a1865 = 16u8;
                        fixed_u8::bit_and(a1864, a1865)
                    };
                    let a1867 = 16u8;
                    if equal(a1866, a1867) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1868, a1869)?
            };
            nat_add(a1870, a1871)?
        };
        let a1895 = {
            let a1892 = {
                let a1880 = {
                    let a1874 = {
                        let a1872 = v1;
                        let a1873 = 8u8;
                        fixed_u8::bit_and(a1872, a1873)
                    };
                    let a1875 = 8u8;
                    if equal(a1874, a1875) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1881 = {
                    let a1878 = {
                        let a1876 = v1;
                        let a1877 = 4u8;
                        fixed_u8::bit_and(a1876, a1877)
                    };
                    let a1879 = 4u8;
                    if equal(a1878, a1879) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1880, a1881)?
            };
            let a1893 = {
                let a1890 = {
                    let a1884 = {
                        let a1882 = v1;
                        let a1883 = 2u8;
                        fixed_u8::bit_and(a1882, a1883)
                    };
                    let a1885 = 2u8;
                    if equal(a1884, a1885) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1891 = {
                    let a1888 = {
                        let a1886 = v1;
                        let a1887 = 1u8;
                        fixed_u8::bit_and(a1886, a1887)
                    };
                    let a1889 = 1u8;
                    if equal(a1888, a1889) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1890, a1891)?
            };
            nat_add(a1892, a1893)?
        };
        nat_add(a1894, a1895)?
    };
    let a1896 = v2;
    let a1897 = 48u64;
    if nat_lt(a1896, a1897) {
        Ok(None::<u64>)
    } else {
        let a1898 = v2;
        let a1899 = 57u64;
        if nat_le(a1898, a1899) {
            Ok(Some({
                let a1900 = v2;
                let a1901 = 48u64;
                nat_sub(a1900, a1901)
            }))
        } else {
            let a1902 = v2;
            let a1903 = 65u64;
            if nat_lt(a1902, a1903) {
                Ok(None::<u64>)
            } else {
                let a1904 = v2;
                let a1905 = 70u64;
                if nat_le(a1904, a1905) {
                    Ok(Some({
                        let a1906 = v2;
                        let a1907 = 55u64;
                        nat_sub(a1906, a1907)
                    }))
                } else {
                    let a1908 = v2;
                    let a1909 = 97u64;
                    if nat_lt(a1908, a1909) {
                        Ok(None::<u64>)
                    } else {
                        let a1910 = v2;
                        let a1911 = 102u64;
                        if nat_le(a1910, a1911) {
                            Ok(Some({
                                let a1912 = v2;
                                let a1913 = 87u64;
                                nat_sub(a1912, a1913)
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

pub fn f263(v0: u64) -> Bytes {
    f207(f249(v0, 16u64), f250(v0))
}

pub fn f264(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f265(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1914 = f139(v0.clone(), 0u64)?;
        let a1915 = 45u64;
        if nat_eq(a1914, a1915) {
            1u64
        } else {
            0u64
        }
    };
    let a1917 = v1;
    let a1918 = {
        let a1916 = v0.clone();
        length_bytes(a1916)
    };
    if nat_lt(a1917, a1918) {
        let a1920 = f142(v0.clone(), v1)?;
        let a1921 = {
            let a1919 = v0;
            length_bytes(a1919)
        };
        Ok(nat_eq(a1920, a1921))
    } else {
        Ok(false)
    }
}

pub fn f266(v0: Bytes) -> R<Option<Adt0>> {
    let v1: bool = {
        let a1922 = f139(v0.clone(), 0u64)?;
        let a1923 = 45u64;
        nat_eq(a1922, a1923)
    };
    let v2: u64 = if v1 {
        1u64
    } else {
        0u64
    };
    let v3: u64 = f268(v0.clone(), v2)?;
    let v4: Bytes = f47(v0.clone(), v3, {
        let a1925 = {
            let a1924 = v0;
            length_bytes(a1924)
        };
        let a1926 = v3;
        nat_sub(a1925, a1926)
    });
    let a1928 = {
        let a1927 = v4.clone();
        length_bytes(a1927)
    };
    let a1929 = 0u64;
    if nat_eq(a1928, a1929) {
        Ok(Some(Adt0::C1(false, 0u64)))
    } else {
        let a1931 = 19u64;
        let a1932 = {
            let a1930 = v4.clone();
            length_bytes(a1930)
        };
        if nat_lt(a1931, a1932) {
            Ok(None::<Adt0>)
        } else {
            let m1933 = f269(v4)?;
            match m1933 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v5) => {
                    if f270(v1, v5) {
                        Ok(Some(Adt0::C1(v1, v5)))
                    } else {
                        Ok(None::<Adt0>)
                    }
                }
            }
        }
    }
}

pub fn f267(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h1934, _) = v0;
        h1934
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h1935) = v0;
        h1935
    };
    let m1936 = v1;
    match m1936.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m1937 = v4;
            match m1937.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f271(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f268(v0: Bytes, v1: u64) -> R<u64> {
    let (h1938, _) = f70(Fn7::F272(v0.clone()), {
        let a1940 = {
            let a1939 = v0;
            length_bytes(a1939)
        };
        let a1941 = 1u64;
        nat_add(a1940, a1941)?
    }, v1)?;
    Ok(h1938)
}

pub fn f269(v0: Bytes) -> R<Option<u64>> {
    if f273(v0.clone())? {
        Ok(Some({
            let (_, h1942) = {
                let (h1943, _) = f274(Fn27::F275(v0.clone()), {
                    let a1944 = v0;
                    length_bytes(a1944)
                }, (0u64, 0u64))?;
                h1943
            };
            h1942
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f270(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1945 = 0u64;
        let a1946 = v1;
        if nat_lt(a1945, a1946) {
            let a1947 = v1;
            let a1948 = 9223372036854775808u64;
            nat_le(a1947, a1948)
        } else {
            false
        }
    } else {
        let a1949 = v1;
        let a1950 = 9223372036854775807u64;
        nat_le(a1949, a1950)
    }
}

pub fn f271(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a1955 = {
            let a1953 = {
                let a1951 = v0.clone();
                length_list(a1951)
            };
            let a1954 = {
                let a1952 = v1.clone();
                length_list(a1952)
            };
            nat_add(a1953, a1954)?
        };
        let a1956 = 1u64;
        nat_add(a1955, a1956)?
    };
    Ok(f198({
        let (_, h1957) = {
            let (_, h1958) = {
                let (h1959, _) = f276(Fn28::F277, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h1959
            };
            h1958
        };
        h1957
    }))
}

pub fn f272(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f278(v0, v1)
}

pub fn f273(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1960 = v0.clone();
        length_bytes(a1960)
    };
    let a1961 = 0u64;
    let a1962 = v1;
    if nat_lt(a1961, a1962) {
        let a1963 = f279(v0.clone())?;
        let a1964 = v1;
        if nat_eq(a1963, a1964) {
            let a1965 = 1u64;
            let a1966 = v1;
            if nat_lt(a1965, a1966) {
                let m1969 = {
                    let a1967 = v0;
                    let a1968 = 0u64;
                    index_bytes(a1967, a1968)
                };
                match m1969 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1970 = v2;
                        let a1971 = 48u8;
                        Ok(!equal(a1970, a1971))
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

pub fn f274(v0: Fn27, p1976: u64, p1977: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1975 = (p1976, p1977);
    {
        let mut m1975 = m1975;
        loop {
            let step = {
                let (v1, v2) = m1975;
                let m1972 = v1;
                if m1972 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1972 - 1;
                    let m1974 = {
                        let c1973: Fn27 = v0.clone();
                        c1973.apply(v2)?
                    };
                    match m1974 {
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
                Ok(next) => { m1975 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f280(v0, v1)
}

pub fn f276(v0: Fn28, p1982: u64, p1983: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m1981 = (p1982, p1983);
    {
        let mut m1981 = m1981;
        loop {
            let step = {
                let (v1, v2) = m1981;
                let m1978 = v1;
                if m1978 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m1978 - 1;
                    let m1980 = {
                        let c1979: Fn28 = v0.clone();
                        c1979.apply(v2.clone())
                    };
                    match m1980 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f281(v0)
}

pub fn f278(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1984 = f139(v0, v1)?;
    let a1985 = 48u64;
    if nat_eq(a1984, a1985) {
        Ok(Some({
            let a1986 = v1;
            let a1987 = 1u64;
            nat_add(a1986, a1987)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f279(v0: Bytes) -> R<u64> {
    let (h1988, _) = f70(Fn7::F282(v0.clone()), {
        let a1989 = v0;
        length_bytes(a1989)
    }, 0u64)?;
    Ok(h1988)
}

pub fn f280(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1993 = {
        let a1991 = v0;
        let a1992 = {
            let (h1990, _) = v1;
            h1990
        };
        index_bytes(a1991, a1992)
    };
    match m1993 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1995 = {
                    let (h1994, _) = v1;
                    h1994
                };
                let a1996 = 1u64;
                nat_add(a1995, a1996)?
            }, {
                let a2000 = {
                    let a1998 = {
                        let (_, h1997) = v1;
                        h1997
                    };
                    let a1999 = 10u64;
                    nat_mul(a1998, a1999)?
                };
                let a2001 = f283(v2)?;
                nat_add(a2000, a2001)?
            })))
        }
    }
}

pub fn f281(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h2002, _) = v0.clone();
        h2002
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h2003, _) = {
            let (_, h2004) = v0.clone();
            h2004
        };
        h2003
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h2005) = {
            let (_, h2006) = v0;
            h2006
        };
        h2005
    };
    let m2007 = v1.clone();
    match m2007.uncons() {
        None => {
            let m2008 = v2;
            match m2008.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2009 = v2.clone();
            match m2009.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f284({
                        let (h2010, _) = v8.clone();
                        h2010
                    }, {
                        let (h2011, _) = v6.clone();
                        h2011
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

pub fn f282(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2014 = {
        let a2012 = v0;
        let a2013 = v1;
        index_bytes(a2012, a2013)
    };
    match m2014 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f285(v2)? {
                Ok(Some({
                    let a2015 = v1;
                    let a2016 = 1u64;
                    nat_add(a2015, a2016)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f283(v0: u8) -> R<u64> {
    let a2063 = {
        let v1: u8 = v0;
        let a2061 = {
            let a2037 = {
                let a2025 = {
                    let a2019 = {
                        let a2017 = v1;
                        let a2018 = 128u8;
                        fixed_u8::bit_and(a2017, a2018)
                    };
                    let a2020 = 128u8;
                    if equal(a2019, a2020) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2026 = {
                    let a2023 = {
                        let a2021 = v1;
                        let a2022 = 64u8;
                        fixed_u8::bit_and(a2021, a2022)
                    };
                    let a2024 = 64u8;
                    if equal(a2023, a2024) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2025, a2026)?
            };
            let a2038 = {
                let a2035 = {
                    let a2029 = {
                        let a2027 = v1;
                        let a2028 = 32u8;
                        fixed_u8::bit_and(a2027, a2028)
                    };
                    let a2030 = 32u8;
                    if equal(a2029, a2030) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2036 = {
                    let a2033 = {
                        let a2031 = v1;
                        let a2032 = 16u8;
                        fixed_u8::bit_and(a2031, a2032)
                    };
                    let a2034 = 16u8;
                    if equal(a2033, a2034) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2035, a2036)?
            };
            nat_add(a2037, a2038)?
        };
        let a2062 = {
            let a2059 = {
                let a2047 = {
                    let a2041 = {
                        let a2039 = v1;
                        let a2040 = 8u8;
                        fixed_u8::bit_and(a2039, a2040)
                    };
                    let a2042 = 8u8;
                    if equal(a2041, a2042) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2048 = {
                    let a2045 = {
                        let a2043 = v1;
                        let a2044 = 4u8;
                        fixed_u8::bit_and(a2043, a2044)
                    };
                    let a2046 = 4u8;
                    if equal(a2045, a2046) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2047, a2048)?
            };
            let a2060 = {
                let a2057 = {
                    let a2051 = {
                        let a2049 = v1;
                        let a2050 = 2u8;
                        fixed_u8::bit_and(a2049, a2050)
                    };
                    let a2052 = 2u8;
                    if equal(a2051, a2052) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2058 = {
                    let a2055 = {
                        let a2053 = v1;
                        let a2054 = 1u8;
                        fixed_u8::bit_and(a2053, a2054)
                    };
                    let a2056 = 1u8;
                    if equal(a2055, a2056) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2057, a2058)?
            };
            nat_add(a2059, a2060)?
        };
        nat_add(a2061, a2062)?
    };
    let a2064 = 48u64;
    Ok(nat_sub(a2063, a2064))
}

pub fn f284(v0: Bytes, v1: Bytes) -> bool {
    let a2069 = {
        let a2065 = v0;
        let a2066 = v1;
        compare_bytes(a2065, a2066)
    };
    let a2070 = {
        let a2067 = Bytes::lit(&[]);
        let a2068 = Bytes::lit(&[0x01]);
        compare_bytes(a2067, a2068)
    };
    equal(a2069, a2070)
}

pub fn f285(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2115 = {
            let a2091 = {
                let a2079 = {
                    let a2073 = {
                        let a2071 = v1;
                        let a2072 = 128u8;
                        fixed_u8::bit_and(a2071, a2072)
                    };
                    let a2074 = 128u8;
                    if equal(a2073, a2074) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2080 = {
                    let a2077 = {
                        let a2075 = v1;
                        let a2076 = 64u8;
                        fixed_u8::bit_and(a2075, a2076)
                    };
                    let a2078 = 64u8;
                    if equal(a2077, a2078) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2079, a2080)?
            };
            let a2092 = {
                let a2089 = {
                    let a2083 = {
                        let a2081 = v1;
                        let a2082 = 32u8;
                        fixed_u8::bit_and(a2081, a2082)
                    };
                    let a2084 = 32u8;
                    if equal(a2083, a2084) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2090 = {
                    let a2087 = {
                        let a2085 = v1;
                        let a2086 = 16u8;
                        fixed_u8::bit_and(a2085, a2086)
                    };
                    let a2088 = 16u8;
                    if equal(a2087, a2088) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2089, a2090)?
            };
            nat_add(a2091, a2092)?
        };
        let a2116 = {
            let a2113 = {
                let a2101 = {
                    let a2095 = {
                        let a2093 = v1;
                        let a2094 = 8u8;
                        fixed_u8::bit_and(a2093, a2094)
                    };
                    let a2096 = 8u8;
                    if equal(a2095, a2096) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2102 = {
                    let a2099 = {
                        let a2097 = v1;
                        let a2098 = 4u8;
                        fixed_u8::bit_and(a2097, a2098)
                    };
                    let a2100 = 4u8;
                    if equal(a2099, a2100) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2101, a2102)?
            };
            let a2114 = {
                let a2111 = {
                    let a2105 = {
                        let a2103 = v1;
                        let a2104 = 2u8;
                        fixed_u8::bit_and(a2103, a2104)
                    };
                    let a2106 = 2u8;
                    if equal(a2105, a2106) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2112 = {
                    let a2109 = {
                        let a2107 = v1;
                        let a2108 = 1u8;
                        fixed_u8::bit_and(a2107, a2108)
                    };
                    let a2110 = 1u8;
                    if equal(a2109, a2110) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2111, a2112)?
            };
            nat_add(a2113, a2114)?
        };
        nat_add(a2115, a2116)?
    };
    let a2117 = 48u64;
    let a2118 = v2;
    if nat_le(a2117, a2118) {
        let a2119 = v2;
        let a2120 = 57u64;
        Ok(nat_le(a2119, a2120))
    } else {
        Ok(false)
    }
}
