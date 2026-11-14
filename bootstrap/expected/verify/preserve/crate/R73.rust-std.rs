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
    C0(u64, List<Adt2>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, List<Adt6>, Adt0, u64, u64, Adt7, u64, Option<Adt1>, List<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64, u64, Bytes, Adt7, Option<Adt0>, List<Adt0>, List<(Bytes, Adt0)>, Adt0, Adt0, List<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt0),
    C5,
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
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn0::F8(k0, k1) => f8(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F23,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn1::F23 => f23(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F29(Adt0, u64, List<Adt2>),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn2::F29(k0, k1, k2) => f29(k0.clone(), *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F32,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt0, p1: (Bytes, Adt0)) -> Adt0 {
        match self {
            Fn3::F32 => f32(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F36,
}

impl Fn4 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn4::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F38,
}

impl Fn5 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn5::F38 => f38(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F47(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt8) -> Option<Adt8> {
        match self {
            Fn6::F47(k0) => f47(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F51,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn7::F51 => f51(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F54,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn8::F54 => f54(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F56,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn9::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F69(Adt0),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn10::F69(k0) => f69(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F78(u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn11::F78(k0, k1) => f78(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F82(u64, Bytes),
    F201(u64, Bytes),
    F207(Bytes),
    F208(Bytes),
    F221(Bytes),
    F277(Bytes),
    F284(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn12::F82(k0, k1) => f82(*k0, k1.clone(), p0),
            Fn12::F201(k0, k1) => f201(*k0, k1.clone(), p0),
            Fn12::F207(k0) => f207(k0.clone(), p0),
            Fn12::F208(k0) => f208(k0.clone(), p0),
            Fn12::F221(k0) => f221(k0.clone(), p0),
            Fn12::F277(k0) => f277(k0.clone(), p0),
            Fn12::F284(k0) => f284(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F87(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn13::F87(k0) => f87(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F101,
}

impl Fn14 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn14::F101 => f101(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F104,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn15::F104 => f104(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F106,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn16::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F176(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn17::F176(k0) => f176(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F188(u64, List<Adt2>),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt13) -> R<Option<Adt13>> {
        match self {
            Fn18::F188(k0, k1) => f188(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F190(Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt0)) -> bool {
        match self {
            Fn19::F190(k0) => f190(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F195(u64, List<Adt2>),
    F240(u64, List<Adt2>, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: Adt15) -> R<Option<Adt15>> {
        match self {
            Fn20::F195(k0, k1) => f195(*k0, k1.clone(), p0),
            Fn20::F240(k0, k1, k2) => f240(*k0, k1.clone(), k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F203,
}

impl Fn21 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn21::F203 => f203(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F211(Bytes, Adt0),
}

impl Fn22 {
    pub fn apply(&self, p0: Option<Adt0>, p1: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn22::F211(k0, k1) => f211(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F214(Adt0),
    F248(Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> R<bool> {
        match self {
            Fn23::F214(k0) => f214(k0.clone(), p0, p1),
            Fn23::F248(k0) => Ok(f248(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F225,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn24::F225 => f225(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F236,
}

impl Fn25 {
    pub fn apply(&self, p0: Adt16) -> Option<Adt16> {
        match self {
            Fn25::F236 => f236(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F239(u64, Bytes),
}

impl Fn26 {
    pub fn apply(&self, p0: Adt17) -> R<Option<Adt17>> {
        match self {
            Fn26::F239(k0, k1) => f239(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F250,
}

impl Fn27 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn27::F250 => f250(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F280(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn28::F280(k0) => f280(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: Bytes, v2: Bytes) -> R<Result<Adt0, Adt1>> {
    let m1 = f1(v1.clone())?;
    match m1 {
        Ok(v3) => {
            let m3 = f2(f3(v0, v3, {
                let a2 = v1;
                length_bytes(a2)
            })?);
            match m3 {
                Ok(v4) => {
                    Ok(Ok::<Adt0, Adt1>(f4(f5(Str::lit("spec"), f6(v2), Str::lit("declarations"), v4))?))
                }
                Err(v5) => {
                    Ok(Err::<Adt0, Adt1>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<Adt0, Adt1>(v6))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt2>, Adt1>> {
    let v1: u64 = {
        let a4 = v0.clone();
        length_bytes(a4)
    };
    let v2: Adt4 = {
        let (h5, _) = f7(Fn0::F8(v1, v0), {
            let a6 = v1;
            let a7 = 1u64;
            nat_add(a6, a7)?
        }, Adt4::C0(0u64, List::<Adt2>::nil(), None::<Adt1>))?;
        h5
    };
    let m9 = match v2.clone() {
        Adt4::C0(_, _, h8) => {
            h8
        }
    };
    match m9 {
        None => {
            Ok(Ok::<List<Adt2>, Adt1>(f9(match v2 {
                Adt4::C0(_, h10, _) => {
                    h10
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt2>, Adt1>(v3))
        }
    }
}

pub fn f2(v0: Adt5) -> Result<Adt0, Adt1> {
    let m12 = match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h11, _) => {
            h11
        }
    };
    match m12 {
        None => {
            let a14 = match v0.clone() {
                Adt5::C0(_, _, _, h13, _, _, _, _, _) => {
                    h13
                }
            };
            let a15 = 9u64;
            if nat_eq(a14, a15) {
                Ok::<Adt0, Adt1>(match v0 {
                    Adt5::C0(_, _, h16, _, _, _, _, _, _) => {
                        h16
                    }
                })
            } else {
                Err::<Adt0, Adt1>(Adt1::C0(0u64, f10(Str::lit("the reader ran out of steps"))))
            }
        }
        Some(v1) => {
            Err::<Adt0, Adt1>(v1)
        }
    }
}

pub fn f3(v0: Adt0, v1: List<Adt2>, v2: u64) -> R<Adt5> {
    let v3: Adt7 = f11(v0.clone(), 0u64)?;
    let a18 = {
        let a17 = v1.clone();
        length_list(a17)
    };
    let a19 = 0u64;
    if nat_eq(a18, a19) {
        Ok(Adt5::C0(0u64, List::<Adt6>::nil(), f12(List::<Adt0>::nil()), 9u64, 0u64, v3, 1u64, None::<Adt1>, List::<(u64, u64)>::nil()))
    } else {
        f13(v1.clone(), v2, v0, f14(f15(f16(f17(0u64, 1u64), v3.clone(), None::<Adt0>, List::<(Bytes, Adt0)>::nil()), 1u64), f10(Str::lit("")), f18(v1, v2, 0u64)), v3)
    }
}

pub fn f4(v0: List<(Bytes, Adt0)>) -> R<Adt0> {
    Ok(Adt0::C4(std::rc::Rc::new(f19(f20(v0)?))))
}

pub fn f5(v0: Str, v1: Adt0, v2: Str, v3: Adt0) -> List<(Bytes, Adt0)> {
    List::cons((f10(v0), v1), List::cons((f10(v2), v3), List::<(Bytes, Adt0)>::nil()))
}

pub fn f6(v0: Bytes) -> Adt0 {
    Adt0::C2(v0)
}

pub fn f7(v0: Fn0, p24: u64, p25: Adt4) -> R<(Adt4, bool)> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m20 = v1;
                if m20 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m20 - 1;
                    let m22 = {
                        let c21: Fn0 = v0.clone();
                        c21.apply(v2.clone())?
                    };
                    match m22 {
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
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f21(v1, v0, v2)
}

pub fn f9(v0: List<Adt2>) -> List<Adt2> {
    f22(Fn1::F23, List::<Adt2>::nil(), v0)
}

pub fn f10(v0: Str) -> Bytes {
    let a26 = v0;
    utf8_encode(a26)
}

pub fn f11(v0: Adt0, v1: u64) -> R<Adt7> {
    let a27 = v1;
    let a28 = 0u64;
    if nat_eq(a27, a28) {
        let m29 = f24(v0.clone(), f10(Str::lit("properties")));
        match m29 {
            None => {
                Ok(Adt7::C5)
            }
            Some(v2) => {
                let m30 = f24(v2, f10(Str::lit("declarations")));
                match m30 {
                    None => {
                        Ok(Adt7::C5)
                    }
                    Some(v3) => {
                        f25(v0, v3)
                    }
                }
            }
        }
    } else {
        let a31 = v1;
        let a32 = 1u64;
        if nat_eq(a31, a32) {
            Ok(Adt7::C0)
        } else {
            let a33 = v1;
            let a34 = 2u64;
            if nat_eq(a33, a34) {
                Ok(Adt7::C1)
            } else {
                let m35 = f26(v0, f10(Str::lit("proof")));
                match m35 {
                    None => {
                        Ok(Adt7::C5)
                    }
                    Some(v4) => {
                        Ok(Adt7::C4(v4))
                    }
                }
            }
        }
    }
}

pub fn f12(v0: List<Adt0>) -> Adt0 {
    Adt0::C3(std::rc::Rc::new(f27(v0)))
}

pub fn f13(v0: List<Adt2>, v1: u64, v2: Adt0, v3: Adt6, v4: Adt7) -> R<Adt5> {
    let v5: Adt5 = Adt5::C0(0u64, List::cons(v3, List::<Adt6>::nil()), Adt0::C5, 0u64, 0u64, v4, 1u64, None::<Adt1>, List::<(u64, u64)>::nil());
    let (h36, _) = f28(Fn2::F29(v2, v1, v0.clone()), {
        let a40 = {
            let a38 = {
                let a37 = v0;
                length_list(a37)
            };
            let a39 = 16u64;
            nat_mul(a38, a39)?
        };
        let a41 = 64u64;
        nat_add(a40, a41)?
    }, v5)?;
    Ok(h36)
}

pub fn f14(v0: Adt6, v1: Bytes, v2: u64) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h42, _, _, _, _, _, _, _, _, _, _, _) => {
            h42
        }
    }, match v0.clone() {
        Adt6::C0(_, h43, _, _, _, _, _, _, _, _, _, _) => {
            h43
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h44, _, _, _, _, _, _, _, _, _) => {
            h44
        }
    }, v2, v1, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h45, _, _, _, _, _, _) => {
            h45
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h46, _, _, _, _, _) => {
            h46
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h47, _, _, _, _) => {
            h47
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h48, _, _, _) => {
            h48
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h49, _, _) => {
            h49
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h50, _) => {
            h50
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h51) => {
            h51
        }
    })
}

pub fn f15(v0: Adt6, v1: u64) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h52, _, _, _, _, _, _, _, _, _, _, _) => {
            h52
        }
    }, v1, match v0.clone() {
        Adt6::C0(_, _, h53, _, _, _, _, _, _, _, _, _) => {
            h53
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h54, _, _, _, _, _, _, _, _) => {
            h54
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h55, _, _, _, _, _, _, _) => {
            h55
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h56, _, _, _, _, _, _) => {
            h56
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h57, _, _, _, _, _) => {
            h57
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h58, _, _, _, _) => {
            h58
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h59, _, _, _) => {
            h59
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h60, _, _) => {
            h60
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h61, _) => {
            h61
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h62) => {
            h62
        }
    })
}

pub fn f16(v0: Adt6, v1: Adt7, v2: Option<Adt0>, v3: List<(Bytes, Adt0)>) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h63, _, _, _, _, _, _, _, _, _, _, _) => {
            h63
        }
    }, match v0.clone() {
        Adt6::C0(_, h64, _, _, _, _, _, _, _, _, _, _) => {
            h64
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h65, _, _, _, _, _, _, _, _, _) => {
            h65
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h66, _, _, _, _, _, _, _, _) => {
            h66
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h67, _, _, _, _, _, _, _) => {
            h67
        }
    }, v1, v2, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h68, _, _, _, _) => {
            h68
        }
    }, v3, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h69, _, _) => {
            h69
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h70, _) => {
            h70
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h71) => {
            h71
        }
    })
}

pub fn f17(v0: u64, v1: u64) -> Adt6 {
    Adt6::C0(v0, 0u64, v1, 0u64, f10(Str::lit("")), Adt7::C5, None::<Adt0>, List::<Adt0>::nil(), List::<(Bytes, Adt0)>::nil(), Adt0::C5, Adt0::C5, List::<(u64, u64)>::nil())
}

pub fn f18(v0: List<Adt2>, v1: u64, v2: u64) -> u64 {
    let m72 = f30(v0, v2);
    match m72 {
        None => {
            v1
        }
        Some(v3) => {
            match v3 {
                Adt2::C0(_, h73, _) => {
                    h73
                }
            }
        }
    }
}

pub fn f19(v0: List<(Bytes, Adt0)>) -> Adt0 {
    f31(Fn3::F32, Adt0::C5, f33(v0))
}

pub fn f20(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f34(f35(Fn4::F36, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m78 = {
        let (h74, _) = f37(Fn5::F38, {
            let a76 = {
                let a75 = v0;
                length_list(a75)
            };
            let a77 = 1u64;
            nat_add(a76, a77)?
        }, v1)?;
        h74
    };
    match m78.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f21(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m80 = match v2.clone() {
        Adt4::C0(_, _, h79) => {
            h79
        }
    };
    match m80 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt4::C0(h81, _, _) => {
                    h81
                }
            };
            let a82 = v4;
            let a83 = v1;
            if nat_lt(a82, a83) {
                let v5: u64 = f39(v0.clone(), v4)?;
                let a84 = 0u64;
                let a85 = v5;
                if nat_lt(a84, a85) {
                    Ok(Some(Adt4::C0({
                        let a86 = v4;
                        let a87 = v5;
                        nat_add(a86, a87)?
                    }, match v2 {
                        Adt4::C0(_, h88, _) => {
                            h88
                        }
                    }, None::<Adt1>)))
                } else {
                    if f40(v0.clone(), v4, 123u8) {
                        Ok(f41(v2, Adt3::C0, {
                            let a89 = v4;
                            let a90 = 1u64;
                            nat_add(a89, a90)?
                        }))
                    } else {
                        if f40(v0.clone(), v4, 125u8) {
                            Ok(f41(v2, Adt3::C1, {
                                let a91 = v4;
                                let a92 = 1u64;
                                nat_add(a91, a92)?
                            }))
                        } else {
                            if f40(v0.clone(), v4, 91u8) {
                                Ok(f41(v2, Adt3::C2, {
                                    let a93 = v4;
                                    let a94 = 1u64;
                                    nat_add(a93, a94)?
                                }))
                            } else {
                                if f40(v0.clone(), v4, 93u8) {
                                    Ok(f41(v2, Adt3::C3, {
                                        let a95 = v4;
                                        let a96 = 1u64;
                                        nat_add(a95, a96)?
                                    }))
                                } else {
                                    if f40(v0.clone(), v4, 40u8) {
                                        Ok(f41(v2, Adt3::C4, {
                                            let a97 = v4;
                                            let a98 = 1u64;
                                            nat_add(a97, a98)?
                                        }))
                                    } else {
                                        if f40(v0.clone(), v4, 41u8) {
                                            Ok(f41(v2, Adt3::C5, {
                                                let a99 = v4;
                                                let a100 = 1u64;
                                                nat_add(a99, a100)?
                                            }))
                                        } else {
                                            if f40(v0.clone(), v4, 44u8) {
                                                Ok(f41(v2, Adt3::C6, {
                                                    let a101 = v4;
                                                    let a102 = 1u64;
                                                    nat_add(a101, a102)?
                                                }))
                                            } else {
                                                if f40(v0.clone(), v4, 59u8) {
                                                    Ok(f41(v2, Adt3::C10, {
                                                        let a103 = v4;
                                                        let a104 = 1u64;
                                                        nat_add(a103, a104)?
                                                    }))
                                                } else {
                                                    if f40(v0.clone(), v4, 58u8) {
                                                        f42(v0, v2)
                                                    } else {
                                                        if f40(v0.clone(), v4, 34u8) {
                                                            f43(v0, v1, v2)
                                                        } else {
                                                            f44(v0, v1, v2)
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

pub fn f22(v0: Fn1, p108: List<Adt2>, p109: List<Adt2>) -> List<Adt2> {
    let m107 = (p108, p109);
    {
        let mut m107 = m107;
        loop {
            let step = {
                let (v1, v2) = m107;
                let m105 = v2;
                match m105.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c106: Fn1 = v0.clone();
                            c106.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f24(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m110 = f45(v0);
    match m110 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            let m113 = {
                let (h112, _) = f46(Fn6::F47(v1), 1000000u64, Adt8::C0(v2, None::<Adt0>));
                h112
            };
            match m113 {
                Adt8::C0(_, h111) => {
                    h111
                }
            }
        }
    }
}

pub fn f25(v0: Adt0, v1: Adt0) -> R<Adt7> {
    let m114 = f24(f48(v0.clone(), v1)?, f10(Str::lit("items")));
    match m114 {
        None => {
            Ok(Adt7::C5)
        }
        Some(v2) => {
            f49(v0, v2)
        }
    }
}

pub fn f26(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m115 = f24(v0, f10(Str::lit("$defs")));
    match m115 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f24(v2, v1)
        }
    }
}

pub fn f27(v0: List<Adt0>) -> Adt0 {
    f50(Fn7::F51, Adt0::C5, v0)
}

pub fn f28(v0: Fn2, p120: u64, p121: Adt5) -> R<(Adt5, bool)> {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v1, v2) = m119;
                let m116 = v1;
                if m116 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m116 - 1;
                    let m118 = {
                        let c117: Fn2 = v0.clone();
                        c117.apply(v2.clone())?
                    };
                    match m118 {
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
                Ok(next) => { m119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt0, v1: u64, v2: List<Adt2>, v3: Adt5) -> R<Option<Adt5>> {
    f52(v2, v1, v0, v3)
}

pub fn f30(v0: List<Adt2>, v1: u64) -> Option<Adt2> {
    let a122 = v0;
    let a123 = v1;
    index_list(a122, a123)
}

pub fn f31(v0: Fn3, p127: Adt0, p128: List<(Bytes, Adt0)>) -> Adt0 {
    let m126 = (p127, p128);
    {
        let mut m126 = m126;
        loop {
            let step = {
                let (v1, v2) = m126;
                let m124 = v2;
                match m124.uncons() {
                    None => {
                        Err::<(Adt0, List<(Bytes, Adt0)>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<(Bytes, Adt0)>), Adt0>(({
                            let c125: Fn3 = v0.clone();
                            c125.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Adt0, v1: (Bytes, Adt0)) -> Adt0 {
    Adt0::C7({
        let (h129, _) = v1;
        h129
    }, std::rc::Rc::new({
        let (_, h130) = v1;
        h130
    }), std::rc::Rc::new(v0))
}

pub fn f33(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f53(Fn8::F54, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f34(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f55(Fn9::F56, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f35(v0: Fn4, p134: List<List<(Bytes, Adt0)>>, p135: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v1, v2) = m133;
                let m131 = v2;
                match m131.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c132: Fn4 = v0.clone();
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

pub fn f36(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f37(v0: Fn5, p140: u64, p141: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m136 = v1;
                if m136 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m136 - 1;
                    let m138 = {
                        let c137: Fn5 = v0.clone();
                        c137.apply(v2.clone())?
                    };
                    match m138 {
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
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f57(v0)
}

pub fn f39(v0: Bytes, v1: u64) -> R<u64> {
    if if f58(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f59(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f59(v0.clone(), v1, 194u8) {
            if if f59(v0.clone(), {
                let a142 = v1;
                let a143 = 1u64;
                nat_add(a142, a143)?
            }, 133u8) {
                true
            } else {
                f59(v0, {
                    let a144 = v1;
                    let a145 = 1u64;
                    nat_add(a144, a145)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f59(v0.clone(), v1, 225u8) {
                if if f59(v0.clone(), {
                    let a146 = v1;
                    let a147 = 1u64;
                    nat_add(a146, a147)?
                }, 154u8) {
                    f59(v0, {
                        let a148 = v1;
                        let a149 = 2u64;
                        nat_add(a148, a149)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f59(v0.clone(), v1, 226u8) {
                    if f59(v0.clone(), {
                        let a150 = v1;
                        let a151 = 1u64;
                        nat_add(a150, a151)?
                    }, 128u8) {
                        if if f58(v0.clone(), {
                            let a152 = v1;
                            let a153 = 2u64;
                            nat_add(a152, a153)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f59(v0.clone(), {
                                let a154 = v1;
                                let a155 = 2u64;
                                nat_add(a154, a155)?
                            }, 168u8) {
                                true
                            } else {
                                if f59(v0.clone(), {
                                    let a156 = v1;
                                    let a157 = 2u64;
                                    nat_add(a156, a157)?
                                }, 169u8) {
                                    true
                                } else {
                                    f59(v0, {
                                        let a158 = v1;
                                        let a159 = 2u64;
                                        nat_add(a158, a159)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f59(v0.clone(), {
                            let a160 = v1;
                            let a161 = 1u64;
                            nat_add(a160, a161)?
                        }, 129u8) {
                            f59(v0, {
                                let a162 = v1;
                                let a163 = 2u64;
                                nat_add(a162, a163)?
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
                    if f59(v0.clone(), v1, 227u8) {
                        if if f59(v0.clone(), {
                            let a164 = v1;
                            let a165 = 1u64;
                            nat_add(a164, a165)?
                        }, 128u8) {
                            f59(v0, {
                                let a166 = v1;
                                let a167 = 2u64;
                                nat_add(a166, a167)?
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

pub fn f40(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m168 = f60(v0, v1);
    match m168 {
        None => {
            false
        }
        Some(v3) => {
            let a169 = v3;
            let a170 = v2;
            equal(a169, a170)
        }
    }
}

pub fn f41(v0: Adt4, v1: Adt3, v2: u64) -> Option<Adt4> {
    Some(Adt4::C0(v2, List::cons(Adt2::C0(v1, match v0.clone() {
        Adt4::C0(h171, _, _) => {
            h171
        }
    }, v2), match v0 {
        Adt4::C0(_, h172, _) => {
            h172
        }
    }), None::<Adt1>))
}

pub fn f42(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    let v2: u64 = match v1.clone() {
        Adt4::C0(h173, _, _) => {
            h173
        }
    };
    if f40(v0.clone(), {
        let a174 = v2;
        let a175 = 1u64;
        nat_add(a174, a175)?
    }, 61u8) {
        Ok(f41(v1, Adt3::C8, {
            let a176 = v2;
            let a177 = 2u64;
            nat_add(a176, a177)?
        }))
    } else {
        if f40(v0, {
            let a178 = v2;
            let a179 = 1u64;
            nat_add(a178, a179)?
        }, 58u8) {
            Ok(f41(v1, Adt3::C9, {
                let a180 = v2;
                let a181 = 2u64;
                nat_add(a180, a181)?
            }))
        } else {
            Ok(f41(v1, Adt3::C7, {
                let a182 = v2;
                let a183 = 1u64;
                nat_add(a182, a183)?
            }))
        }
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h184, _, _) => {
            h184
        }
    };
    let v4: Adt9 = f61(v0, v1, v3)?;
    let m186 = match v4.clone() {
        Adt9::C0(_, _, _, h185) => {
            h185
        }
    };
    match m186 {
        None => {
            if match v4.clone() {
                Adt9::C0(_, _, h188, _) => {
                    h188
                }
            } {
                Ok(f41(v2, Adt3::C11(f62(match v4.clone() {
                    Adt9::C0(_, h189, _, _) => {
                        h189
                    }
                })), match v4 {
                    Adt9::C0(h190, _, _, _) => {
                        h190
                    }
                }))
            } else {
                Ok(Some(Adt4::C0(v3, match v2 {
                    Adt4::C0(_, h191, _) => {
                        h191
                    }
                }, Some(f63(v3, f64(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt4::C0(v3, match v2 {
                Adt4::C0(_, h187, _) => {
                    h187
                }
            }, Some(v5))))
        }
    }
}

pub fn f44(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h192, _, _) => {
            h192
        }
    };
    let v4: u64 = f65(v0.clone(), v1, v3)?;
    Ok(f41(v2, Adt3::C12(f66(v0, v3, {
        let a193 = v4;
        let a194 = v3;
        nat_sub(a193, a194)
    })), v4))
}

pub fn f45(v0: Adt0) -> Option<Adt0> {
    let m195 = v0;
    match m195 {
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
        Adt0::C4(r196) => {
            let v1 = (*r196).clone();
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

pub fn f46(v0: Fn6, p201: u64, p202: Adt8) -> (Adt8, bool) {
    let m200 = (p201, p202);
    {
        let mut m200 = m200;
        loop {
            let step = {
                let (v1, v2) = m200;
                let m197 = v1;
                if m197 == 0 {
                    Err::<(u64, Adt8), (Adt8, bool)>((v2, false))
                } else {
                    let v3 = m197 - 1;
                    let m199 = {
                        let c198: Fn6 = v0.clone();
                        c198.apply(v2.clone())
                    };
                    match m199 {
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
                Ok(next) => { m200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: Adt8) -> Option<Adt8> {
    f67(v0, v1)
}

pub fn f48(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let (h203, _) = f68(Fn10::F69(v0), 4096u64, v1)?;
    Ok(h203)
}

pub fn f49(v0: Adt0, v1: Adt0) -> R<Adt7> {
    let m204 = f70(f24(v1.clone(), f10(Str::lit("$ref"))));
    match m204 {
        None => {
            Ok(Adt7::C4(v1))
        }
        Some(v2) => {
            let v3: Bytes = f71(v2)?;
            if f72(v3.clone(), f10(Str::lit("term"))) {
                Ok(Adt7::C0)
            } else {
                if f72(v3.clone(), f10(Str::lit("type"))) {
                    Ok(Adt7::C1)
                } else {
                    if f72(v3.clone(), f10(Str::lit("member"))) {
                        Ok(Adt7::C2)
                    } else {
                        if f72(v3, f10(Str::lit("parameter"))) {
                            Ok(Adt7::C3)
                        } else {
                            Ok(Adt7::C4(f48(v0, v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f50(v0: Fn7, p208: Adt0, p209: List<Adt0>) -> Adt0 {
    let m207 = (p208, p209);
    {
        let mut m207 = m207;
        loop {
            let step = {
                let (v1, v2) = m207;
                let m205 = v2;
                match m205.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c206: Fn7 = v0.clone();
                            c206.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f52(v0: List<Adt2>, v1: u64, v2: Adt0, v3: Adt5) -> R<Option<Adt5>> {
    let m211 = match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h210, _) => {
            h210
        }
    };
    match m211 {
        None => {
            let v5: u64 = match v3.clone() {
                Adt5::C0(_, _, _, h212, _, _, _, _, _) => {
                    h212
                }
            };
            let a213 = v5;
            let a214 = 9u64;
            if nat_eq(a213, a214) {
                Ok(None::<Adt5>)
            } else {
                Ok(Some({
                    let a215 = v5;
                    let a216 = 0u64;
                    if nat_eq(a215, a216) {
                        f73(v0, v1, v2, v3)?
                    } else {
                        f74(v0, v1, v2, v3)?
                    }
                }))
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f53(v0: Fn8, p220: List<(Bytes, Adt0)>, p221: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m219 = (p220, p221);
    {
        let mut m219 = m219;
        loop {
            let step = {
                let (v1, v2) = m219;
                let m217 = v2;
                match m217.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c218: Fn8 = v0.clone();
                            c218.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f55(v0: Fn9, p225: List<List<(Bytes, Adt0)>>, p226: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m224 = (p225, p226);
    {
        let mut m224 = m224;
        loop {
            let step = {
                let (v1, v2) = m224;
                let m222 = v2;
                match m222.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c223: Fn9 = v0.clone();
                            c223.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f57(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a228 = {
        let a227 = v0.clone();
        length_list(a227)
    };
    let a229 = 1u64;
    if nat_le(a228, a229) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f75(v0)?))
    }
}

pub fn f58(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m230 = f60(v0, v1);
    match m230 {
        None => {
            false
        }
        Some(v4) => {
            f76(v4, v2, v3)
        }
    }
}

pub fn f59(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m231 = f60(v0, v1);
    match m231 {
        None => {
            false
        }
        Some(v3) => {
            let a232 = v3;
            let a233 = v2;
            equal(a232, a233)
        }
    }
}

pub fn f60(v0: Bytes, v1: u64) -> Option<u8> {
    let a234 = v0;
    let a235 = v1;
    index_bytes(a234, a235)
}

pub fn f61(v0: Bytes, v1: u64, v2: u64) -> R<Adt9> {
    let (h236, _) = f77(Fn11::F78(v1, v0), {
        let a239 = {
            let a237 = v1;
            let a238 = v2;
            nat_sub(a237, a238)
        };
        let a240 = 1u64;
        nat_add(a239, a240)?
    }, Adt9::C0({
        let a241 = v2;
        let a242 = 1u64;
        nat_add(a241, a242)?
    }, List::<Bytes>::nil(), false, None::<Adt1>))?;
    Ok(h236)
}

pub fn f62(v0: List<Bytes>) -> Bytes {
    f79(f80(v0))
}

pub fn f63(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f64(v0: Str) -> Bytes {
    let a243 = v0;
    utf8_encode(a243)
}

pub fn f65(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h244, _) = f81(Fn12::F82(v1, v0), {
        let a245 = v1;
        let a246 = v2;
        nat_sub(a245, a246)
    }, v2)?;
    Ok(h244)
}

pub fn f66(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m250 = {
        let a247 = v0;
        let a248 = v1;
        let a249 = v2;
        slice_bytes(a247, a248, a249)
    };
    match m250 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f67(v0: Bytes, v1: Adt8) -> Option<Adt8> {
    let m252 = match v1.clone() {
        Adt8::C0(_, h251) => {
            h251
        }
    };
    match m252 {
        None => {
            let m254 = f83(match v1 {
                Adt8::C0(h253, _) => {
                    h253
                }
            });
            match m254 {
                None => {
                    None::<Adt8>
                }
                Some(v3) => {
                    if f72(match v3.clone() {
                        Adt11::C0(h255, _, _) => {
                            h255
                        }
                    }, v0) {
                        Some(Adt8::C0(Adt0::C5, Some(match v3 {
                            Adt11::C0(_, h256, _) => {
                                h256
                            }
                        })))
                    } else {
                        Some(Adt8::C0(match v3 {
                            Adt11::C0(_, _, h257) => {
                                h257
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

pub fn f68(v0: Fn10, p262: u64, p263: Adt0) -> R<(Adt0, bool)> {
    let m261 = (p262, p263);
    {
        let mut m261 = m261;
        loop {
            let step = {
                let (v1, v2) = m261;
                let m258 = v1;
                if m258 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m258 - 1;
                    let m260 = {
                        let c259: Fn10 = v0.clone();
                        c259.apply(v2.clone())?
                    };
                    match m260 {
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
                Ok(next) => { m261 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    f84(v0, v1)
}

pub fn f70(v0: Option<Adt0>) -> Option<Bytes> {
    let m264 = v0;
    match m264 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f85(v1)
        }
    }
}

pub fn f71(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let m270 = {
            let (h266, _) = f86(Fn13::F87(v0.clone()), {
                let a268 = {
                    let a267 = v0.clone();
                    length_bytes(a267)
                };
                let a269 = 1u64;
                nat_add(a268, a269)?
            }, Adt10::C0(0u64, 0u64))?;
            h266
        };
        match m270 {
            Adt10::C0(_, h265) => {
                h265
            }
        }
    };
    Ok(f66(v0.clone(), v1, {
        let a272 = {
            let a271 = v0;
            length_bytes(a271)
        };
        let a273 = v1;
        nat_sub(a272, a273)
    }))
}

pub fn f72(v0: Bytes, v1: Bytes) -> bool {
    let a274 = v0;
    let a275 = v1;
    equal(a274, a275)
}

pub fn f73(v0: List<Adt2>, v1: u64, v2: Adt0, v3: Adt5) -> R<Adt5> {
    let v4: u64 = match v3.clone() {
        Adt5::C0(_, _, _, _, h276, _, _, _, _) => {
            h276
        }
    };
    let a277 = v4;
    let a278 = 0u64;
    if nat_eq(a277, a278) {
        f88(v0, v1, v2, v3)
    } else {
        let a279 = v4;
        let a280 = 1u64;
        if nat_eq(a279, a280) {
            f89(v0, v1, v2, v3)
        } else {
            let a281 = v4;
            let a282 = 2u64;
            if nat_eq(a281, a282) {
                f90(v0, v1, v3)
            } else {
                let a283 = v4;
                let a284 = 3u64;
                if nat_eq(a283, a284) {
                    f91(v0, v1, v3)
                } else {
                    let a285 = v4;
                    let a286 = 4u64;
                    if nat_eq(a285, a286) {
                        f92(v0, v1, v3)
                    } else {
                        let a287 = v4;
                        let a288 = 5u64;
                        if nat_eq(a287, a288) {
                            f93(v0, v1, v3.clone(), match v3 {
                                Adt5::C0(_, _, _, _, _, _, h289, _, _) => {
                                    h289
                                }
                            })
                        } else {
                            f94(v0, v1, v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: List<Adt2>, v1: u64, v2: Adt0, v3: Adt5) -> R<Adt5> {
    let m291 = match v3.clone() {
        Adt5::C0(_, h290, _, _, _, _, _, _, _) => {
            h290
        }
    };
    match m291.uncons() {
        None => {
            Ok(f95(v3.clone(), match v3 {
                Adt5::C0(_, _, h292, _, _, _, _, _, _) => {
                    h292
                }
            }, List::<(u64, u64)>::nil()))
        }
        Some((v4, v5)) => {
            let v6: Adt5 = Adt5::C0(match v3.clone() {
                Adt5::C0(h293, _, _, _, _, _, _, _, _) => {
                    h293
                }
            }, v5, match v3.clone() {
                Adt5::C0(_, _, h294, _, _, _, _, _, _) => {
                    h294
                }
            }, 1u64, match v3.clone() {
                Adt5::C0(_, _, _, _, h295, _, _, _, _) => {
                    h295
                }
            }, match v3.clone() {
                Adt5::C0(_, _, _, _, _, h296, _, _, _) => {
                    h296
                }
            }, match v3.clone() {
                Adt5::C0(_, _, _, _, _, _, h297, _, _) => {
                    h297
                }
            }, None::<Adt1>, match v3 {
                Adt5::C0(_, _, _, _, _, _, _, _, h298) => {
                    h298
                }
            });
            let v7: u64 = match v4.clone() {
                Adt6::C0(h299, _, _, _, _, _, _, _, _, _, _, _) => {
                    h299
                }
            };
            let a300 = v7;
            let a301 = 0u64;
            if nat_eq(a300, a301) {
                Ok(f96(v0, v1, v2, v6, v4))
            } else {
                let a302 = v7;
                let a303 = 10u64;
                if nat_lt(a302, a303) {
                    f97(v0, v1, v2, v6, v4)
                } else {
                    let a304 = v7;
                    let a305 = 30u64;
                    if nat_lt(a304, a305) {
                        f98(v0, v1, v6, v4)
                    } else {
                        f99(v0, v1, v6, v4)
                    }
                }
            }
        }
    }
}

pub fn f75(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f34({
        let (_, h306) = {
            let (h307, _) = f100(Fn14::F101, {
                let a309 = {
                    let a308 = v0.clone();
                    length_list(a308)
                };
                let a310 = 1u64;
                nat_add(a309, a310)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h307
        };
        h306
    }))
}

pub fn f76(v0: u8, v1: u8, v2: u8) -> bool {
    let m313 = {
        let a311 = v0;
        let a312 = v1;
        fixed_u8::checked_sub(a311, a312)
    };
    match m313 {
        None => {
            false
        }
        Some(_) => {
            let m316 = {
                let a314 = v2;
                let a315 = v0;
                fixed_u8::checked_sub(a314, a315)
            };
            match m316 {
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

pub fn f77(v0: Fn11, p321: u64, p322: Adt9) -> R<(Adt9, bool)> {
    let m320 = (p321, p322);
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v1, v2) = m320;
                let m317 = v1;
                if m317 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m317 - 1;
                    let m319 = {
                        let c318: Fn11 = v0.clone();
                        c318.apply(v2.clone())?
                    };
                    match m319 {
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
                Ok(next) => { m320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: u64, v1: Bytes, v2: Adt9) -> R<Option<Adt9>> {
    f102(v1, v0, v2)
}

pub fn f79(v0: List<Bytes>) -> Bytes {
    let m325 = {
        let (h323, _) = f103(Fn15::F104, {
            let a324 = v0.clone();
            length_list(a324)
        }, v0);
        h323
    };
    match m325.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f80(v0: List<Bytes>) -> List<Bytes> {
    f105(Fn16::F106, List::<Bytes>::nil(), v0)
}

pub fn f81(v0: Fn12, p330: u64, p331: u64) -> R<(u64, bool)> {
    let m329 = (p330, p331);
    {
        let mut m329 = m329;
        loop {
            let step = {
                let (v1, v2) = m329;
                let m326 = v1;
                if m326 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m326 - 1;
                    let m328 = {
                        let c327: Fn12 = v0.clone();
                        c327.apply(v2)?
                    };
                    match m328 {
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
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f107(v1, v0, v2)
}

pub fn f83(v0: Adt0) -> Option<Adt11> {
    let m332 = v0;
    match m332 {
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
        Adt0::C7(v1, r333, r334) => {
            let v2 = (*r333).clone();
            let v3 = (*r334).clone();
            Some(Adt11::C0(v1, v2, v3))
        }
    }
}

pub fn f84(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    let m335 = f70(f24(v1, f10(Str::lit("$ref"))));
    match m335 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v2) => {
            Ok(f26(v0, f71(v2)?))
        }
    }
}

pub fn f85(v0: Adt0) -> Option<Bytes> {
    let m336 = v0;
    match m336 {
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

pub fn f86(v0: Fn13, p341: u64, p342: Adt10) -> R<(Adt10, bool)> {
    let m340 = (p341, p342);
    {
        let mut m340 = m340;
        loop {
            let step = {
                let (v1, v2) = m340;
                let m337 = v1;
                if m337 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m337 - 1;
                    let m339 = {
                        let c338: Fn13 = v0.clone();
                        c338.apply(v2.clone())?
                    };
                    match m339 {
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
                Ok(next) => { m340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    f108(v0, v1)
}

pub fn f88(v0: List<Adt2>, v1: u64, _: Adt0, v3: Adt5) -> R<Adt5> {
    let v4: u64 = match v3.clone() {
        Adt5::C0(h343, _, _, _, _, _, _, _, _) => {
            h343
        }
    };
    let v5: u64 = match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, h344, _, _) => {
            h344
        }
    };
    let a345 = 256u64;
    let a346 = v5;
    if nat_lt(a345, a346) {
        Ok(f109(v0, v1, v3, v4, f10(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let m348 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h347, _, _, _) => {
                h347
            }
        };
        match m348 {
            Adt7::C0 => {
                Ok(f110(v3, v4, 3u64, Adt7::C0, v5))
            }
            Adt7::C1 => {
                Ok(f110(v3, v4, 2u64, Adt7::C1, v5))
            }
            Adt7::C2 => {
                let a349 = f111(v0.clone(), v4);
                let a350 = 0u64;
                if nat_eq(a349, a350) {
                    f93(v0, v1, v3, 0u64)
                } else {
                    let v6: Adt12 = f112(v0, v1, v4)?;
                    let m352 = match v6.clone() {
                        Adt12::C0(_, _, h351) => {
                            h351
                        }
                    };
                    match m352 {
                        None => {
                            Ok(f114(v3, match v6.clone() {
                                Adt12::C0(_, h354, _) => {
                                    h354
                                }
                            }, match v6 {
                                Adt12::C0(h355, _, _) => {
                                    h355
                                }
                            }))
                        }
                        Some(_) => {
                            Ok(f113(v3, match v6 {
                                Adt12::C0(_, _, h353) => {
                                    h353
                                }
                            }))
                        }
                    }
                }
            }
            Adt7::C3 => {
                Ok(f110(v3, v4, 4u64, Adt7::C3, v5))
            }
            Adt7::C4(_) => {
                Ok(f110(v3.clone(), v4, 1u64, match v3 {
                    Adt5::C0(_, _, _, _, _, h356, _, _, _) => {
                        h356
                    }
                }, v5))
            }
            Adt7::C5 => {
                Ok(f110(v3, v4, 5u64, Adt7::C5, v5))
            }
        }
    }
}

pub fn f89(v0: List<Adt2>, v1: u64, v2: Adt0, v3: Adt5) -> R<Adt5> {
    let v4: u64 = match v3.clone() {
        Adt5::C0(h357, _, _, _, _, _, _, _, _) => {
            h357
        }
    };
    let v5: u64 = match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, h358, _, _) => {
            h358
        }
    };
    let v6: Adt7 = match v3.clone() {
        Adt5::C0(_, _, _, _, _, h359, _, _, _) => {
            h359
        }
    };
    let v7: u64 = f111(v0.clone(), v4);
    let a360 = v7;
    let a361 = 2u64;
    if nat_eq(a360, a361) {
        let a364 = f111(v0, {
            let a362 = v4;
            let a363 = 1u64;
            nat_add(a362, a363)?
        });
        let a365 = 3u64;
        if nat_eq(a364, a365) {
            Ok(f114(v3, {
                let a366 = v4;
                let a367 = 2u64;
                nat_add(a366, a367)?
            }, f12(List::<Adt0>::nil())))
        } else {
            let v8: Adt7 = f115(v2, v6)?;
            Ok(f116(v3, f16(f17(1u64, v5), v8.clone(), None::<Adt0>, List::<(Bytes, Adt0)>::nil()), {
                let a368 = v4;
                let a369 = 1u64;
                nat_add(a368, a369)?
            }, 0u64, v8, {
                let a370 = v5;
                let a371 = 1u64;
                nat_add(a370, a371)?
            }))
        }
    } else {
        let a372 = v7;
        let a373 = 0u64;
        if nat_eq(a372, a373) {
            f117(v0, v1, v2.clone(), v3, f16(f17(2u64, v5), v6.clone(), f118(v2, v6), List::<(Bytes, Adt0)>::nil()), {
                let a374 = v4;
                let a375 = 1u64;
                nat_add(a374, a375)?
            })
        } else {
            let v9: Bytes = f119(v0.clone(), v4);
            if f120({
                let a376 = v7;
                let a377 = 12u64;
                nat_eq(a376, a377)
            }, f120({
                let a380 = f111(v0.clone(), {
                    let a378 = v4;
                    let a379 = 1u64;
                    nat_add(a378, a379)?
                });
                let a381 = 0u64;
                nat_eq(a380, a381)
            }, f120(f121(v9.clone())?, !f122(v9.clone())?))) {
                f117(v0, v1, v2.clone(), v3, f16(f17(2u64, v5), v6.clone(), f123(v2, v6, v9.clone())?, List::cons((f10(Str::lit("kind")), f6(v9)), List::<(Bytes, Adt0)>::nil())), {
                    let a382 = v4;
                    let a383 = 2u64;
                    nat_add(a382, a383)?
                })
            } else {
                Ok(f110(v3, v4, 5u64, v6, v5))
            }
        }
    }
}

pub fn f90(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h384, _, _, _, _, _, _, _, _) => {
            h384
        }
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h385, _, _) => {
            h385
        }
    };
    let a386 = 256u64;
    let a387 = v4;
    if nat_lt(a386, a387) {
        Ok(f109(v0, v1, v2, v3, f10(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v5: u64 = f111(v0.clone(), v3);
        let v6: Bytes = f119(v0.clone(), v3);
        let a388 = v5;
        let a389 = 12u64;
        if nat_eq(a388, a389) {
            if f120({
                let a392 = f111(v0.clone(), {
                    let a390 = v3;
                    let a391 = 1u64;
                    nat_add(a390, a391)?
                });
                let a393 = 0u64;
                nat_eq(a392, a393)
            }, f121(v6.clone())?) {
                Ok(f110(v2, v3, 1u64, Adt7::C1, v4))
            } else {
                let m394 = f124(v6.clone());
                match m394 {
                    None => {
                        if f120(f126(v6.clone(), f10(Str::lit("'"))), f121(f127(v6.clone(), 1u64))?) {
                            Ok(f114(v2, {
                                let a397 = v3;
                                let a398 = 1u64;
                                nat_add(a397, a398)?
                            }, f125(f10(Str::lit("parameter")), f128(Str::lit("name"), f6(f127(v6, 1u64))))?))
                        } else {
                            if f129(v6.clone(), f130()) {
                                Ok(f109(v0, v1, v2, v3, f131(f131(f131(f131(f10(Str::lit("`")), v6.clone()), f10(Str::lit("` takes arguments: write `("))), v6), f10(Str::lit(" \u{2026})`")))))
                            } else {
                                let v8: Adt12 = f112(v0, v1, v3)?;
                                let m400 = match v8.clone() {
                                    Adt12::C0(_, _, h399) => {
                                        h399
                                    }
                                };
                                match m400 {
                                    None => {
                                        Ok(f114(v2, match v8.clone() {
                                            Adt12::C0(_, h402, _) => {
                                                h402
                                            }
                                        }, f125(f10(Str::lit("named")), f5(Str::lit("member"), match v8 {
                                            Adt12::C0(h403, _, _) => {
                                                h403
                                            }
                                        }, Str::lit("arguments"), f12(List::<Adt0>::nil())))?))
                                    }
                                    Some(_) => {
                                        Ok(f113(v2, match v8 {
                                            Adt12::C0(_, _, h401) => {
                                                h401
                                            }
                                        }))
                                    }
                                }
                            }
                        }
                    }
                    Some(v7) => {
                        Ok(f114(v2, {
                            let a395 = v3;
                            let a396 = 1u64;
                            nat_add(a395, a396)?
                        }, f125(v7, List::<(Bytes, Adt0)>::nil())?))
                    }
                }
            }
        } else {
            let a404 = v5;
            let a405 = 4u64;
            if nat_eq(a404, a405) {
                f132(v0, v1, f116(v2, f17(11u64, v4), {
                    let a406 = v3;
                    let a407 = 1u64;
                    nat_add(a406, a407)?
                }, 2u64, Adt7::C1, v4), {
                    let a408 = v3;
                    let a409 = 1u64;
                    nat_add(a408, a409)?
                }, v4)
            } else {
                Ok(f109(v0.clone(), v1, v2, v3, f131(f10(Str::lit("expected a type, found ")), f133(v0, v3)?)))
            }
        }
    }
}

pub fn f91(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h410, _, _, _, _, _, _, _, _) => {
            h410
        }
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h411, _, _) => {
            h411
        }
    };
    let a412 = 256u64;
    let a413 = v4;
    if nat_lt(a412, a413) {
        Ok(f109(v0, v1, v2, v3, f10(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v5: u64 = f111(v0.clone(), v3);
        let v6: Bytes = f119(v0.clone(), v3);
        let a414 = v5;
        let a415 = 11u64;
        if nat_eq(a414, a415) {
            Ok(f114(v2, {
                let a416 = v3;
                let a417 = 1u64;
                nat_add(a416, a417)?
            }, f125(f10(Str::lit("string")), f128(Str::lit("value"), f6(v6)))?))
        } else {
            let a418 = v5;
            let a419 = 4u64;
            if nat_eq(a418, a419) {
                f134(v0, v1, f116(v2, f17(30u64, v4), {
                    let a420 = v3;
                    let a421 = 1u64;
                    nat_add(a420, a421)?
                }, 3u64, Adt7::C0, v4), {
                    let a422 = v3;
                    let a423 = 1u64;
                    nat_add(a422, a423)?
                }, v4)
            } else {
                let a424 = v5;
                let a425 = 12u64;
                if nat_eq(a424, a425) {
                    if f120({
                        let a428 = f111(v0.clone(), {
                            let a426 = v3;
                            let a427 = 1u64;
                            nat_add(a426, a427)?
                        });
                        let a429 = 0u64;
                        nat_eq(a428, a429)
                    }, f121(v6.clone())?) {
                        Ok(f110(v2, v3, 1u64, Adt7::C0, v4))
                    } else {
                        f135(v0, v1, v2, v3, v6)
                    }
                } else {
                    Ok(f109(v0.clone(), v1, v2, v3, f131(f10(Str::lit("expected a term, found ")), f133(v0, v3)?)))
                }
            }
        }
    }
}

pub fn f92(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h430, _, _, _, _, _, _, _, _) => {
            h430
        }
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h431, _, _) => {
            h431
        }
    };
    let a432 = f111(v0.clone(), v3);
    let a433 = 0u64;
    if nat_eq(a432, a433) {
        Ok(f110(v2, v3, 1u64, Adt7::C3, v4))
    } else {
        let v5: Adt12 = f136(v0.clone(), v1, v3, f10(Str::lit("a parameter name")))?;
        let m435 = match v5.clone() {
            Adt12::C0(_, _, h434) => {
                h434
            }
        };
        match m435 {
            None => {
                let a438 = f111(v0.clone(), match v5.clone() {
                    Adt12::C0(_, h437, _) => {
                        h437
                    }
                });
                let a439 = 7u64;
                if nat_eq(a438, a439) {
                    Ok(f116(v2, f137(f17(20u64, v4), match v5.clone() {
                        Adt12::C0(h440, _, _) => {
                            h440
                        }
                    }, Adt0::C5), {
                        let a442 = match v5 {
                            Adt12::C0(_, h441, _) => {
                                h441
                            }
                        };
                        let a443 = 1u64;
                        nat_add(a442, a443)?
                    }, 2u64, Adt7::C1, {
                        let a444 = v4;
                        let a445 = 1u64;
                        nat_add(a444, a445)?
                    }))
                } else {
                    Ok(f109(v0, v1, v2, match v5 {
                        Adt12::C0(_, h446, _) => {
                            h446
                        }
                    }, f10(Str::lit("expected `:` after a parameter name"))))
                }
            }
            Some(_) => {
                Ok(f113(v2, match v5 {
                    Adt12::C0(_, _, h436) => {
                        h436
                    }
                }))
            }
        }
    }
}

pub fn f93(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64) -> R<Adt5> {
    let v4: Adt13 = f138(v0, v1, match v2.clone() {
        Adt5::C0(h447, _, _, _, _, _, _, _, _) => {
            h447
        }
    }, v3)?;
    let m449 = match v4.clone() {
        Adt13::C0(_, _, _, _, _, h448) => {
            h448
        }
    };
    match m449 {
        None => {
            Ok(f114(v2, match v4.clone() {
                Adt13::C0(h451, _, _, _, _, _) => {
                    h451
                }
            }, match v4 {
                Adt13::C0(_, _, _, _, h452, _) => {
                    h452
                }
            }))
        }
        Some(_) => {
            Ok(f113(v2, match v4 {
                Adt13::C0(_, _, _, _, _, h450) => {
                    h450
                }
            }))
        }
    }
}

pub fn f94(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h453, _, _, _, _, _, _, _, _) => {
            h453
        }
    };
    let a454 = f111(v0.clone(), v3);
    let a455 = 2u64;
    if nat_eq(a454, a455) {
        f139(v0, v1, v2.clone(), f17(18u64, match v2 {
            Adt5::C0(_, _, _, _, _, _, h456, _, _) => {
                h456
            }
        }), {
            let a457 = v3;
            let a458 = 1u64;
            nat_add(a457, a458)?
        })
    } else {
        Ok(f109(v0, v1, v2, v3, f10(Str::lit("expected `[`"))))
    }
}

pub fn f95(v0: Adt5, v1: Adt0, v2: List<(u64, u64)>) -> Adt5 {
    Adt5::C0(match v0.clone() {
        Adt5::C0(h459, _, _, _, _, _, _, _, _) => {
            h459
        }
    }, match v0.clone() {
        Adt5::C0(_, h460, _, _, _, _, _, _, _) => {
            h460
        }
    }, v1, 9u64, match v0.clone() {
        Adt5::C0(_, _, _, _, h461, _, _, _, _) => {
            h461
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h462, _, _, _) => {
            h462
        }
    }, match v0 {
        Adt5::C0(_, _, _, _, _, _, h463, _, _) => {
            h463
        }
    }, None::<Adt1>, v2)
}

pub fn f96(v0: List<Adt2>, v1: u64, _: Adt0, v3: Adt5, v4: Adt6) -> Adt5 {
    let v5: u64 = match v3.clone() {
        Adt5::C0(h464, _, _, _, _, _, _, _, _) => {
            h464
        }
    };
    let v6: Adt0 = match v3.clone() {
        Adt5::C0(_, _, h465, _, _, _, _, _, _) => {
            h465
        }
    };
    let a467 = match v4.clone() {
        Adt6::C0(_, h466, _, _, _, _, _, _, _, _, _, _) => {
            h466
        }
    };
    let a468 = 0u64;
    if nat_eq(a467, a468) {
        let a469 = f111(v0.clone(), v5);
        let a470 = 99u64;
        if nat_eq(a469, a470) {
            f95(v3, v6, List::<(u64, u64)>::nil())
        } else {
            f109(v0, v1, v3, v5, f10(Str::lit("unexpected text after the value")))
        }
    } else {
        let v7: Adt6 = f140(f141(v4.clone(), v6), (match v4.clone() {
            Adt6::C0(_, _, _, h471, _, _, _, _, _, _, _, _) => {
                h471
            }
        }, f142(v0.clone(), v5)), 0u64);
        let a472 = f111(v0.clone(), v5);
        let a473 = 99u64;
        if nat_eq(a472, a473) {
            f95(v3, f12(match v7.clone() {
                Adt6::C0(_, _, _, _, _, _, _, h474, _, _, _, _) => {
                    h474
                }
            }), match v7 {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h475) => {
                    h475
                }
            })
        } else {
            f116(v3, f14(v7, f10(Str::lit("")), f18(v0, v1, v5)), v5, 0u64, match v4 {
                Adt6::C0(_, _, _, _, _, h476, _, _, _, _, _, _) => {
                    h476
                }
            }, 1u64)
        }
    }
}

pub fn f97(v0: List<Adt2>, v1: u64, v2: Adt0, v3: Adt5, v4: Adt6) -> R<Adt5> {
    let v5: u64 = match v3.clone() {
        Adt5::C0(h477, _, _, _, _, _, _, _, _) => {
            h477
        }
    };
    let v6: Adt0 = match v3.clone() {
        Adt5::C0(_, _, h478, _, _, _, _, _, _) => {
            h478
        }
    };
    let a480 = match v4.clone() {
        Adt6::C0(h479, _, _, _, _, _, _, _, _, _, _, _) => {
            h479
        }
    };
    let a481 = 1u64;
    if nat_eq(a480, a481) {
        let v7: Adt6 = f141(v4.clone(), v6);
        let a482 = f111(v0.clone(), v5);
        let a483 = 6u64;
        if nat_eq(a482, a483) {
            let a486 = f111(v0, {
                let a484 = v5;
                let a485 = 1u64;
                nat_add(a484, a485)?
            });
            let a487 = 3u64;
            if nat_eq(a486, a487) {
                Ok(f114(v3, {
                    let a488 = v5;
                    let a489 = 2u64;
                    nat_add(a488, a489)?
                }, f12(match v7 {
                    Adt6::C0(_, _, _, _, _, _, _, h490, _, _, _, _) => {
                        h490
                    }
                })))
            } else {
                Ok(f116(v3, v7, {
                    let a491 = v5;
                    let a492 = 1u64;
                    nat_add(a491, a492)?
                }, 0u64, match v4.clone() {
                    Adt6::C0(_, _, _, _, _, h493, _, _, _, _, _, _) => {
                        h493
                    }
                }, {
                    let a495 = match v4 {
                        Adt6::C0(_, _, h494, _, _, _, _, _, _, _, _, _) => {
                            h494
                        }
                    };
                    let a496 = 1u64;
                    nat_add(a495, a496)?
                }))
            }
        } else {
            let a497 = f111(v0.clone(), v5);
            let a498 = 3u64;
            if nat_eq(a497, a498) {
                Ok(f114(v3, {
                    let a499 = v5;
                    let a500 = 1u64;
                    nat_add(a499, a500)?
                }, f12(match v7 {
                    Adt6::C0(_, _, _, _, _, _, _, h501, _, _, _, _) => {
                        h501
                    }
                })))
            } else {
                Ok(f109(v0, v1, v3, v5, f10(Str::lit("expected `,` or `]`"))))
            }
        }
    } else {
        let v8: Bytes = match v4.clone() {
            Adt6::C0(_, _, _, _, h502, _, _, _, _, _, _, _) => {
                h502
            }
        };
        if f143(match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, h503, _, _, _) => {
                h503
            }
        }, v8.clone()) {
            Ok(f144(v3, match v4 {
                Adt6::C0(_, _, _, h504, _, _, _, _, _, _, _, _) => {
                    h504
                }
            }, f131(f131(f10(Str::lit("the member `")), v8), f10(Str::lit("` is written twice")))))
        } else {
            let v9: Adt6 = f16(v4.clone(), match v4.clone() {
                Adt6::C0(_, _, _, _, _, h505, _, _, _, _, _, _) => {
                    h505
                }
            }, match v4.clone() {
                Adt6::C0(_, _, _, _, _, _, h506, _, _, _, _, _) => {
                    h506
                }
            }, List::cons((v8, v6), match v4 {
                Adt6::C0(_, _, _, _, _, _, _, _, h507, _, _, _) => {
                    h507
                }
            }));
            let a508 = f111(v0.clone(), v5);
            let a509 = 6u64;
            if nat_eq(a508, a509) {
                f117(v0, v1, v2, v3, v9, {
                    let a510 = v5;
                    let a511 = 1u64;
                    nat_add(a510, a511)?
                })
            } else {
                let a512 = f111(v0.clone(), v5);
                let a513 = 1u64;
                if nat_eq(a512, a513) {
                    Ok(f114(v3, {
                        let a514 = v5;
                        let a515 = 1u64;
                        nat_add(a514, a515)?
                    }, f4(match v9 {
                        Adt6::C0(_, _, _, _, _, _, _, _, h516, _, _, _) => {
                            h516
                        }
                    })?))
                } else {
                    Ok(f109(v0, v1, v3, v5, f10(Str::lit("expected `,` or `}`"))))
                }
            }
        }
    }
}

pub fn f98(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6) -> R<Adt5> {
    let v4: u64 = match v2.clone() {
        Adt5::C0(h517, _, _, _, _, _, _, _, _) => {
            h517
        }
    };
    let v5: Adt0 = match v2.clone() {
        Adt5::C0(_, _, h518, _, _, _, _, _, _) => {
            h518
        }
    };
    let v6: u64 = match v3.clone() {
        Adt6::C0(h519, _, _, _, _, _, _, _, _, _, _, _) => {
            h519
        }
    };
    let v7: u64 = match v3.clone() {
        Adt6::C0(_, h520, _, _, _, _, _, _, _, _, _, _) => {
            h520
        }
    };
    let a521 = v6;
    let a522 = 11u64;
    if nat_eq(a521, a522) {
        let a523 = f111(v0.clone(), v4);
        let a524 = 5u64;
        if nat_eq(a523, a524) {
            Ok(f114(v2, {
                let a525 = v4;
                let a526 = 1u64;
                nat_add(a525, a526)?
            }, v5))
        } else {
            Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `)`"))))
        }
    } else {
        let a527 = v6;
        let a528 = 12u64;
        if nat_eq(a527, a528) {
            let v8: Bytes = match v3 {
                Adt6::C0(_, _, _, _, h529, _, _, _, _, _, _, _) => {
                    h529
                }
            };
            Ok(f114(v2, v4, if f72(v8.clone(), f10(Str::lit("List"))) {
                f125(f10(Str::lit("list")), f128(Str::lit("element"), v5))?
            } else {
                if f72(v8, f10(Str::lit("Set"))) {
                    f125(f10(Str::lit("set")), f128(Str::lit("element"), v5))?
                } else {
                    f125(f10(Str::lit("option")), f128(Str::lit("value"), v5))?
                }
            }))
        } else {
            let a530 = v6;
            let a531 = 13u64;
            if nat_eq(a530, a531) {
                let a532 = v7;
                let a533 = 0u64;
                if nat_eq(a532, a533) {
                    f145(v2, f15(f137(v3, v5, Adt0::C5), 1u64), v4)
                } else {
                    Ok(f114(v2, v4, if f72(match v3.clone() {
                        Adt6::C0(_, _, _, _, h534, _, _, _, _, _, _, _) => {
                            h534
                        }
                    }, f10(Str::lit("Result"))) {
                        f125(f10(Str::lit("result")), f5(Str::lit("ok"), match v3 {
                            Adt6::C0(_, _, _, _, _, _, _, _, _, h535, _, _) => {
                                h535
                            }
                        }, Str::lit("error"), v5))?
                    } else {
                        f125(f10(Str::lit("map")), f5(Str::lit("key"), match v3 {
                            Adt6::C0(_, _, _, _, _, _, _, _, _, h536, _, _) => {
                                h536
                            }
                        }, Str::lit("value"), v5))?
                    }))
                }
            } else {
                let a537 = v6;
                let a538 = 14u64;
                if nat_eq(a537, a538) {
                    let a539 = v7;
                    let a540 = 0u64;
                    if nat_eq(a539, a540) {
                        f146(v0, v1, v2, f141(v3, v5), v4)
                    } else {
                        Ok(f114(v2, v4, f125(f10(Str::lit("function")), f5(Str::lit("parameters"), f12(match v3 {
                            Adt6::C0(_, _, _, _, _, _, _, h541, _, _, _, _) => {
                                h541
                            }
                        }), Str::lit("result"), v5))?))
                    }
                } else {
                    let a542 = v6;
                    let a543 = 16u64;
                    if nat_eq(a542, a543) {
                        f147(v0, v1, v2, f141(v3, v5), v4)
                    } else {
                        let a544 = v6;
                        let a545 = 17u64;
                        if nat_eq(a544, a545) {
                            let a546 = v7;
                            let a547 = 0u64;
                            if nat_eq(a546, a547) {
                                if f148(v0.clone(), v4, f10(Str::lit("*"))) {
                                    f145(v2, f15(f137(v3, v5, Adt0::C5), 1u64), {
                                        let a548 = v4;
                                        let a549 = 1u64;
                                        nat_add(a548, a549)?
                                    })
                                } else {
                                    Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `*` between the two types of a product"))))
                                }
                            } else {
                                Ok(f114(v2, v4, f125(f10(Str::lit("product")), f5(Str::lit("left"), match v3 {
                                    Adt6::C0(_, _, _, _, _, _, _, _, _, h550, _, _) => {
                                        h550
                                    }
                                }, Str::lit("right"), v5))?))
                            }
                        } else {
                            let a551 = v6;
                            let a552 = 18u64;
                            if nat_eq(a551, a552) {
                                let v9: Adt6 = f141(v3, v5);
                                let a553 = f111(v0.clone(), v4);
                                let a554 = 6u64;
                                if nat_eq(a553, a554) {
                                    f139(v0, v1, v2, v9, {
                                        let a555 = v4;
                                        let a556 = 1u64;
                                        nat_add(a555, a556)?
                                    })
                                } else {
                                    let a557 = f111(v0.clone(), v4);
                                    let a558 = 3u64;
                                    if nat_eq(a557, a558) {
                                        Ok(f114(v2, {
                                            let a559 = v4;
                                            let a560 = 1u64;
                                            nat_add(a559, a560)?
                                        }, f12(match v9 {
                                            Adt6::C0(_, _, _, _, _, _, _, h561, _, _, _, _) => {
                                                h561
                                            }
                                        })))
                                    } else {
                                        Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `,` or `]`"))))
                                    }
                                }
                            } else {
                                Ok(f114(v2, v4, f4(f5(Str::lit("name"), match v3 {
                                    Adt6::C0(_, _, _, _, _, _, _, _, _, h562, _, _) => {
                                        h562
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

pub fn f99(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6) -> R<Adt5> {
    let v4: u64 = match v2.clone() {
        Adt5::C0(h563, _, _, _, _, _, _, _, _) => {
            h563
        }
    };
    let v5: Adt0 = match v2.clone() {
        Adt5::C0(_, _, h564, _, _, _, _, _, _) => {
            h564
        }
    };
    let v6: u64 = match v3.clone() {
        Adt6::C0(h565, _, _, _, _, _, _, _, _, _, _, _) => {
            h565
        }
    };
    let v7: u64 = match v3.clone() {
        Adt6::C0(_, h566, _, _, _, _, _, _, _, _, _, _) => {
            h566
        }
    };
    let a567 = v6;
    let a568 = 30u64;
    if nat_eq(a567, a568) {
        let a569 = f111(v0.clone(), v4);
        let a570 = 5u64;
        if nat_eq(a569, a570) {
            Ok(f114(v2, {
                let a571 = v4;
                let a572 = 1u64;
                nat_add(a571, a572)?
            }, v5))
        } else {
            Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `)`"))))
        }
    } else {
        let a573 = v6;
        let a574 = 31u64;
        if nat_eq(a573, a574) {
            Ok(f114(v2, v4, f125(match v3 {
                Adt6::C0(_, _, _, _, h575, _, _, _, _, _, _, _) => {
                    h575
                }
            }, f128(Str::lit("value"), v5))?))
        } else {
            let a576 = v6;
            let a577 = 32u64;
            if nat_eq(a576, a577) {
                let a578 = v7;
                let a579 = 0u64;
                if nat_eq(a578, a579) {
                    f149(v2, f15(f137(v3, v5, Adt0::C5), 1u64), v4)
                } else {
                    Ok(f114(v2, v4, f125(f10(Str::lit("cons")), f5(Str::lit("head"), match v3 {
                        Adt6::C0(_, _, _, _, _, _, _, _, _, h580, _, _) => {
                            h580
                        }
                    }, Str::lit("tail"), v5))?))
                }
            } else {
                let a581 = v6;
                let a582 = 33u64;
                if nat_eq(a581, a582) {
                    Ok(f114(v2, v4, f125(f10(Str::lit("nil")), f128(Str::lit("element"), v5))?))
                } else {
                    let a583 = v6;
                    let a584 = 34u64;
                    if nat_eq(a583, a584) {
                        let a585 = v7;
                        let a586 = 0u64;
                        if nat_eq(a585, a586) {
                            if f148(v0.clone(), v4, f10(Str::lit("then"))) {
                                f149(v2, f15(f137(v3, v5, Adt0::C5), 1u64), {
                                    let a587 = v4;
                                    let a588 = 1u64;
                                    nat_add(a587, a588)?
                                })
                            } else {
                                Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `then`"))))
                            }
                        } else {
                            let a589 = v7;
                            let a590 = 1u64;
                            if nat_eq(a589, a590) {
                                if f148(v0.clone(), v4, f10(Str::lit("else"))) {
                                    f149(v2, f15(f137(v3.clone(), match v3 {
                                        Adt6::C0(_, _, _, _, _, _, _, _, _, h591, _, _) => {
                                            h591
                                        }
                                    }, v5), 2u64), {
                                        let a592 = v4;
                                        let a593 = 1u64;
                                        nat_add(a592, a593)?
                                    })
                                } else {
                                    Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `else`"))))
                                }
                            } else {
                                Ok(f114(v2, v4, f125(f10(Str::lit("if")), List::cons((f10(Str::lit("else_value")), v5), f5(Str::lit("condition"), match v3.clone() {
                                    Adt6::C0(_, _, _, _, _, _, _, _, _, h594, _, _) => {
                                        h594
                                    }
                                }, Str::lit("then_value"), match v3 {
                                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, h595, _) => {
                                        h595
                                    }
                                })))?))
                            }
                        }
                    } else {
                        let a596 = v6;
                        let a597 = 35u64;
                        if nat_eq(a596, a597) {
                            let a598 = v7;
                            let a599 = 0u64;
                            if nat_eq(a598, a599) {
                                let a600 = f111(v0.clone(), v4);
                                let a601 = 8u64;
                                if nat_eq(a600, a601) {
                                    f149(v2, f15(f137(v3, v5, Adt0::C5), 1u64), {
                                        let a602 = v4;
                                        let a603 = 1u64;
                                        nat_add(a602, a603)?
                                    })
                                } else {
                                    Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `:=`"))))
                                }
                            } else {
                                let a604 = v7;
                                let a605 = 1u64;
                                if nat_eq(a604, a605) {
                                    let a606 = f111(v0.clone(), v4);
                                    let a607 = 10u64;
                                    if nat_eq(a606, a607) {
                                        f149(v2, f15(f137(v3.clone(), match v3 {
                                            Adt6::C0(_, _, _, _, _, _, _, _, _, h608, _, _) => {
                                                h608
                                            }
                                        }, v5), 2u64), {
                                            let a609 = v4;
                                            let a610 = 1u64;
                                            nat_add(a609, a610)?
                                        })
                                    } else {
                                        Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `;`"))))
                                    }
                                } else {
                                    Ok(f114(v2, v4, f125(f10(Str::lit("let")), List::cons((f10(Str::lit("body")), v5), f5(Str::lit("binder"), match v3.clone() {
                                        Adt6::C0(_, _, _, _, _, _, _, _, _, h611, _, _) => {
                                            h611
                                        }
                                    }, Str::lit("value"), match v3 {
                                        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h612, _) => {
                                            h612
                                        }
                                    })))?))
                                }
                            }
                        } else {
                            let a613 = v6;
                            let a614 = 36u64;
                            if nat_eq(a613, a614) {
                                let a615 = v7;
                                let a616 = 0u64;
                                if nat_eq(a615, a616) {
                                    let a617 = f111(v0.clone(), v4);
                                    let a618 = 5u64;
                                    if nat_eq(a617, a618) {
                                        let a621 = f111(v0.clone(), {
                                            let a619 = v4;
                                            let a620 = 1u64;
                                            nat_add(a619, a620)?
                                        });
                                        let a622 = 6u64;
                                        if nat_eq(a621, a622) {
                                            f149(v2, f15(f137(v3, v5, Adt0::C5), 1u64), {
                                                let a623 = v4;
                                                let a624 = 2u64;
                                                nat_add(a623, a624)?
                                            })
                                        } else {
                                            Ok(f109(v0, v1, v2, {
                                                let a625 = v4;
                                                let a626 = 1u64;
                                                nat_add(a625, a626)?
                                            }, f10(Str::lit("expected `,` before the body"))))
                                        }
                                    } else {
                                        Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `)`"))))
                                    }
                                } else {
                                    Ok(f114(v2, v4, f125(f10(Str::lit("forall")), f5(Str::lit("binder"), match v3 {
                                        Adt6::C0(_, _, _, _, _, _, _, _, _, h627, _, _) => {
                                            h627
                                        }
                                    }, Str::lit("body"), v5))?))
                                }
                            } else {
                                let a628 = v6;
                                let a629 = 37u64;
                                if nat_eq(a628, a629) {
                                    let a630 = v7;
                                    let a631 = 0u64;
                                    if nat_eq(a630, a631) {
                                        let a632 = f111(v0.clone(), v4);
                                        let a633 = 5u64;
                                        if nat_eq(a632, a633) {
                                            f150(v0, v1, v2, f141(v3, v5), {
                                                let a634 = v4;
                                                let a635 = 1u64;
                                                nat_add(a634, a635)?
                                            })
                                        } else {
                                            Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `)`"))))
                                        }
                                    } else {
                                        Ok(f114(v2, v4, f125(f10(Str::lit("lambda")), List::cons((f10(Str::lit("body")), v5), f5(Str::lit("parameters"), f12(match v3.clone() {
                                            Adt6::C0(_, _, _, _, _, _, _, h636, _, _, _, _) => {
                                                h636
                                            }
                                        }), Str::lit("captures"), match v3 {
                                            Adt6::C0(_, _, _, _, _, _, _, _, _, h637, _, _) => {
                                                h637
                                            }
                                        })))?))
                                    }
                                } else {
                                    let a638 = v6;
                                    let a639 = 38u64;
                                    if nat_eq(a638, a639) {
                                        let a640 = v7;
                                        let a641 = 0u64;
                                        if nat_eq(a640, a641) {
                                            f151(v0, v1, v2, f15(f137(v3, v5, Adt0::C5), 1u64), v4)
                                        } else {
                                            f151(v0, v1, v2, f141(v3, v5), v4)
                                        }
                                    } else {
                                        let a642 = v6;
                                        let a643 = 39u64;
                                        if nat_eq(a642, a643) {
                                            let v8: Adt12 = f136(v0, v1, v4, f10(Str::lit("a field name")))?;
                                            let m645 = match v8.clone() {
                                                Adt12::C0(_, _, h644) => {
                                                    h644
                                                }
                                            };
                                            match m645 {
                                                None => {
                                                    Ok(f114(v2, match v8.clone() {
                                                        Adt12::C0(_, h647, _) => {
                                                            h647
                                                        }
                                                    }, f125(f10(Str::lit("project")), f5(Str::lit("value"), v5, Str::lit("field"), match v8 {
                                                        Adt12::C0(h648, _, _) => {
                                                            h648
                                                        }
                                                    }))?))
                                                }
                                                Some(_) => {
                                                    Ok(f113(v2, match v8 {
                                                        Adt12::C0(_, _, h646) => {
                                                            h646
                                                        }
                                                    }))
                                                }
                                            }
                                        } else {
                                            let a649 = v6;
                                            let a650 = 40u64;
                                            if if nat_eq(a649, a650) {
                                                true
                                            } else {
                                                let a651 = v6;
                                                let a652 = 44u64;
                                                if nat_eq(a651, a652) {
                                                    true
                                                } else {
                                                    let a653 = v6;
                                                    let a654 = 46u64;
                                                    nat_eq(a653, a654)
                                                }
                                            } {
                                                let a655 = v7;
                                                let a656 = 0u64;
                                                if nat_eq(a655, a656) {
                                                    f152(v0, v1, v2, v3)
                                                } else {
                                                    f151(v0, v1, v2, f141(v3, v5), v4)
                                                }
                                            } else {
                                                let a657 = v6;
                                                let a658 = 41u64;
                                                if nat_eq(a657, a658) {
                                                    f152(v0, v1, v2, v3)
                                                } else {
                                                    let a659 = v6;
                                                    let a660 = 42u64;
                                                    if nat_eq(a659, a660) {
                                                        let a661 = v7;
                                                        let a662 = 0u64;
                                                        if nat_eq(a661, a662) {
                                                            f152(v0, v1, v2, v3)
                                                        } else {
                                                            let v10: Adt6 = f141(v3.clone(), f4(f5(Str::lit("field"), f6(match v3 {
                                                                Adt6::C0(_, _, _, _, h663, _, _, _, _, _, _, _) => {
                                                                    h663
                                                                }
                                                            }), Str::lit("value"), v5))?);
                                                            let a664 = f111(v0.clone(), v4);
                                                            let a665 = 6u64;
                                                            if nat_eq(a664, a665) {
                                                                f153(v0, v1, v2, v10, {
                                                                    let a666 = v4;
                                                                    let a667 = 1u64;
                                                                    nat_add(a666, a667)?
                                                                })
                                                            } else {
                                                                let a668 = f111(v0.clone(), v4);
                                                                let a669 = 1u64;
                                                                if nat_eq(a668, a669) {
                                                                    f153(v0, v1, v2, v10, v4)
                                                                } else {
                                                                    Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `,` or `}`"))))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        let a670 = v6;
                                                        let a671 = 43u64;
                                                        if nat_eq(a670, a671) {
                                                            let a672 = v7;
                                                            let a673 = 0u64;
                                                            if nat_eq(a672, a673) {
                                                                f154(v0, v1, v2, f141(v3, v5), v4)
                                                            } else {
                                                                Ok(f114(v2, v4, f125(f10(Str::lit("primitive")), List::cons((f10(Str::lit("result")), v5), f5(Str::lit("operation"), match v3.clone() {
                                                                    Adt6::C0(_, _, _, _, _, _, _, _, _, h674, _, _) => {
                                                                        h674
                                                                    }
                                                                }, Str::lit("arguments"), f12(match v3 {
                                                                    Adt6::C0(_, _, _, _, _, _, _, h675, _, _, _, _) => {
                                                                        h675
                                                                    }
                                                                }))))?))
                                                            }
                                                        } else {
                                                            let a676 = v6;
                                                            let a677 = 45u64;
                                                            if nat_eq(a676, a677) {
                                                                let a678 = v7;
                                                                let a679 = 0u64;
                                                                if nat_eq(a678, a679) {
                                                                    f155(v0, v1, v2, f15(f137(v3, v5, Adt0::C5), 2u64), v4)
                                                                } else {
                                                                    let a680 = v7;
                                                                    let a681 = 1u64;
                                                                    if nat_eq(a680, a681) {
                                                                        f155(v0, v1, v2, f15(f141(v3.clone(), f4(List::cons((f10(Str::lit("body")), v5), match v3 {
                                                                            Adt6::C0(_, _, _, _, _, _, _, _, h682, _, _, _) => {
                                                                                h682
                                                                            }
                                                                        }))?), 2u64), v4)
                                                                    } else {
                                                                        f155(v0, v1, v2, v3, v4)
                                                                    }
                                                                }
                                                            } else {
                                                                let a683 = v7;
                                                                let a684 = 0u64;
                                                                if nat_eq(a683, a684) {
                                                                    let a685 = f111(v0.clone(), v4);
                                                                    let a686 = 6u64;
                                                                    if nat_eq(a685, a686) {
                                                                        f149(v2, f15(f137(v3, v5, Adt0::C5), 1u64), {
                                                                            let a687 = v4;
                                                                            let a688 = 1u64;
                                                                            nat_add(a687, a688)?
                                                                        })
                                                                    } else {
                                                                        let m689 = f156(f119(v0.clone(), v4));
                                                                        match m689 {
                                                                            None => {
                                                                                Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected an operator or `,` after the first operand"))))
                                                                            }
                                                                            Some(v11) => {
                                                                                let a690 = f111(v0.clone(), v4);
                                                                                let a691 = 12u64;
                                                                                if nat_eq(a690, a691) {
                                                                                    f149(v2, f14(f15(f137(v3, v5, Adt0::C5), 2u64), v11, 0u64), {
                                                                                        let a692 = v4;
                                                                                        let a693 = 1u64;
                                                                                        nat_add(a692, a693)?
                                                                                    })
                                                                                } else {
                                                                                    Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected an operator or `,` after the first operand"))))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    let a694 = v7;
                                                                    let a695 = 1u64;
                                                                    if nat_eq(a694, a695) {
                                                                        Ok(f114(v2, v4, f125(f10(Str::lit("pair")), f5(Str::lit("left"), match v3 {
                                                                            Adt6::C0(_, _, _, _, _, _, _, _, _, h696, _, _) => {
                                                                                h696
                                                                            }
                                                                        }, Str::lit("right"), v5))?))
                                                                    } else {
                                                                        let v12: Bytes = match v3.clone() {
                                                                            Adt6::C0(_, _, _, _, h697, _, _, _, _, _, _, _) => {
                                                                                h697
                                                                            }
                                                                        };
                                                                        let v13: bool = f72(v12.clone(), f10(Str::lit("implies")));
                                                                        Ok(f114(v2, v4, f125(v12, List::cons((if v13 {
                                                                            f10(Str::lit("premise"))
                                                                        } else {
                                                                            f10(Str::lit("left"))
                                                                        }, match v3 {
                                                                            Adt6::C0(_, _, _, _, _, _, _, _, _, h698, _, _) => {
                                                                                h698
                                                                            }
                                                                        }), List::cons((if v13 {
                                                                            f10(Str::lit("conclusion"))
                                                                        } else {
                                                                            f10(Str::lit("right"))
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

pub fn f100(v0: Fn14, p703: u64, p704: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m702 = (p703, p704);
    {
        let mut m702 = m702;
        loop {
            let step = {
                let (v1, v2) = m702;
                let m699 = v1;
                if m699 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m699 - 1;
                    let m701 = {
                        let c700: Fn14 = v0.clone();
                        c700.apply(v2.clone())?
                    };
                    match m701 {
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
                Ok(next) => { m702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f157(v0)
}

pub fn f102(v0: Bytes, v1: u64, v2: Adt9) -> R<Option<Adt9>> {
    let m706 = match v2.clone() {
        Adt9::C0(_, _, _, h705) => {
            h705
        }
    };
    match m706 {
        None => {
            if match v2.clone() {
                Adt9::C0(_, _, h707, _) => {
                    h707
                }
            } {
                Ok(None::<Adt9>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt9::C0(h708, _, _, _) => {
                        h708
                    }
                };
                let a709 = v4;
                let a710 = v1;
                if nat_lt(a709, a710) {
                    if f40(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt9::C0({
                            let a711 = v4;
                            let a712 = 1u64;
                            nat_add(a711, a712)?
                        }, match v2 {
                            Adt9::C0(_, h713, _, _) => {
                                h713
                            }
                        }, true, None::<Adt1>)))
                    } else {
                        if f40(v0.clone(), v4, 92u8) {
                            Ok(Some(f158(v0, v1, v2)?))
                        } else {
                            let a714 = f159(v0.clone(), v4)?;
                            let a715 = 32u64;
                            if nat_lt(a714, a715) {
                                Ok(Some(f160(v2, v4, f64(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f161(v0.clone(), v1, v4)?;
                                Ok(Some(f162(v2, v5, f66(v0, v4, {
                                    let a716 = v5;
                                    let a717 = v4;
                                    nat_sub(a716, a717)
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

pub fn f103(v0: Fn15, p722: u64, p723: List<Bytes>) -> (List<Bytes>, bool) {
    let m721 = (p722, p723);
    {
        let mut m721 = m721;
        loop {
            let step = {
                let (v1, v2) = m721;
                let m718 = v1;
                if m718 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m718 - 1;
                    let m720 = {
                        let c719: Fn15 = v0.clone();
                        c719.apply(v2.clone())
                    };
                    match m720 {
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
                Ok(next) => { m721 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: List<Bytes>) -> Option<List<Bytes>> {
    f163(v0)
}

pub fn f105(v0: Fn16, p727: List<Bytes>, p728: List<Bytes>) -> List<Bytes> {
    let m726 = (p727, p728);
    {
        let mut m726 = m726;
        loop {
            let step = {
                let (v1, v2) = m726;
                let m724 = v2;
                match m724.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c725: Fn16 = v0.clone();
                            c725.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f107(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a729 = v2;
    let a730 = v1;
    if nat_lt(a729, a730) {
        if f164(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a731 = v2;
                let a732 = 1u64;
                nat_add(a731, a732)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f108(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    let v2: u64 = match v1.clone() {
        Adt10::C0(h733, _) => {
            h733
        }
    };
    let m734 = f60(v0, v2);
    match m734 {
        None => {
            Ok(None::<Adt10>)
        }
        Some(v3) => {
            Ok(Some(Adt10::C0({
                let a735 = v2;
                let a736 = 1u64;
                nat_add(a735, a736)?
            }, {
                let a783 = {
                    let v4: u8 = v3;
                    let a781 = {
                        let a757 = {
                            let a745 = {
                                let a739 = {
                                    let a737 = v4;
                                    let a738 = 128u8;
                                    fixed_u8::bit_and(a737, a738)
                                };
                                let a740 = 128u8;
                                if equal(a739, a740) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a746 = {
                                let a743 = {
                                    let a741 = v4;
                                    let a742 = 64u8;
                                    fixed_u8::bit_and(a741, a742)
                                };
                                let a744 = 64u8;
                                if equal(a743, a744) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a745, a746)?
                        };
                        let a758 = {
                            let a755 = {
                                let a749 = {
                                    let a747 = v4;
                                    let a748 = 32u8;
                                    fixed_u8::bit_and(a747, a748)
                                };
                                let a750 = 32u8;
                                if equal(a749, a750) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a756 = {
                                let a753 = {
                                    let a751 = v4;
                                    let a752 = 16u8;
                                    fixed_u8::bit_and(a751, a752)
                                };
                                let a754 = 16u8;
                                if equal(a753, a754) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a755, a756)?
                        };
                        nat_add(a757, a758)?
                    };
                    let a782 = {
                        let a779 = {
                            let a767 = {
                                let a761 = {
                                    let a759 = v4;
                                    let a760 = 8u8;
                                    fixed_u8::bit_and(a759, a760)
                                };
                                let a762 = 8u8;
                                if equal(a761, a762) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a768 = {
                                let a765 = {
                                    let a763 = v4;
                                    let a764 = 4u8;
                                    fixed_u8::bit_and(a763, a764)
                                };
                                let a766 = 4u8;
                                if equal(a765, a766) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a767, a768)?
                        };
                        let a780 = {
                            let a777 = {
                                let a771 = {
                                    let a769 = v4;
                                    let a770 = 2u8;
                                    fixed_u8::bit_and(a769, a770)
                                };
                                let a772 = 2u8;
                                if equal(a771, a772) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a778 = {
                                let a775 = {
                                    let a773 = v4;
                                    let a774 = 1u8;
                                    fixed_u8::bit_and(a773, a774)
                                };
                                let a776 = 1u8;
                                if equal(a775, a776) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a777, a778)?
                        };
                        nat_add(a779, a780)?
                    };
                    nat_add(a781, a782)?
                };
                let a784 = 47u64;
                if nat_eq(a783, a784) {
                    let a785 = v2;
                    let a786 = 1u64;
                    nat_add(a785, a786)?
                } else {
                    match v1 {
                        Adt10::C0(_, h787) => {
                            h787
                        }
                    }
                }
            })))
        }
    }
}

pub fn f109(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: Bytes) -> Adt5 {
    f144(v2, f18(v0, v1, v3), v4)
}

pub fn f110(v0: Adt5, v1: u64, v2: u64, v3: Adt7, v4: u64) -> Adt5 {
    Adt5::C0(v1, match v0.clone() {
        Adt5::C0(_, h788, _, _, _, _, _, _, _) => {
            h788
        }
    }, Adt0::C5, 0u64, v2, v3, v4, None::<Adt1>, match v0 {
        Adt5::C0(_, _, _, _, _, _, _, _, h789) => {
            h789
        }
    })
}

pub fn f111(v0: List<Adt2>, v1: u64) -> u64 {
    f165(v0, v1)
}

pub fn f112(v0: List<Adt2>, v1: u64, v2: u64) -> R<Adt12> {
    let v3: Adt12 = f136(v0.clone(), v1, v2, f10(Str::lit("a name")))?;
    let m791 = match v3.clone() {
        Adt12::C0(_, _, h790) => {
            h790
        }
    };
    match m791 {
        None => {
            let v5: u64 = match v3.clone() {
                Adt12::C0(_, h792, _) => {
                    h792
                }
            };
            let a793 = f111(v0.clone(), v5);
            let a794 = 9u64;
            if nat_eq(a793, a794) {
                let v6: Adt12 = f136(v0, v1, {
                    let a795 = v5;
                    let a796 = 1u64;
                    nat_add(a795, a796)?
                }, f10(Str::lit("a name after `::`")))?;
                let m798 = match v6.clone() {
                    Adt12::C0(_, _, h797) => {
                        h797
                    }
                };
                match m798 {
                    None => {
                        Ok(Adt12::C0(f4(f5(Str::lit("module"), match v3 {
                            Adt12::C0(h799, _, _) => {
                                h799
                            }
                        }, Str::lit("name"), match v6.clone() {
                            Adt12::C0(h800, _, _) => {
                                h800
                            }
                        }))?, match v6 {
                            Adt12::C0(_, h801, _) => {
                                h801
                            }
                        }, None::<Adt1>))
                    }
                    Some(_) => {
                        Ok(v6)
                    }
                }
            } else {
                Ok(Adt12::C0(f4(f128(Str::lit("name"), match v3 {
                    Adt12::C0(h802, _, _) => {
                        h802
                    }
                }))?, v5, None::<Adt1>))
            }
        }
        Some(_) => {
            Ok(v3)
        }
    }
}

pub fn f113(v0: Adt5, v1: Option<Adt1>) -> Adt5 {
    Adt5::C0(match v0.clone() {
        Adt5::C0(h803, _, _, _, _, _, _, _, _) => {
            h803
        }
    }, match v0.clone() {
        Adt5::C0(_, h804, _, _, _, _, _, _, _) => {
            h804
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h805, _, _, _, _, _, _) => {
            h805
        }
    }, 9u64, match v0.clone() {
        Adt5::C0(_, _, _, _, h806, _, _, _, _) => {
            h806
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h807, _, _, _) => {
            h807
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h808, _, _) => {
            h808
        }
    }, v1, match v0 {
        Adt5::C0(_, _, _, _, _, _, _, _, h809) => {
            h809
        }
    })
}

pub fn f114(v0: Adt5, v1: u64, v2: Adt0) -> Adt5 {
    Adt5::C0(v1, match v0.clone() {
        Adt5::C0(_, h810, _, _, _, _, _, _, _) => {
            h810
        }
    }, v2, 1u64, match v0.clone() {
        Adt5::C0(_, _, _, _, h811, _, _, _, _) => {
            h811
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h812, _, _, _) => {
            h812
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h813, _, _) => {
            h813
        }
    }, None::<Adt1>, match v0 {
        Adt5::C0(_, _, _, _, _, _, _, _, h814) => {
            h814
        }
    })
}

pub fn f115(v0: Adt0, v1: Adt7) -> R<Adt7> {
    let m815 = v1;
    match m815 {
        Adt7::C0 => {
            Ok(Adt7::C5)
        }
        Adt7::C1 => {
            Ok(Adt7::C5)
        }
        Adt7::C2 => {
            Ok(Adt7::C5)
        }
        Adt7::C3 => {
            Ok(Adt7::C5)
        }
        Adt7::C4(v2) => {
            f25(v0, v2)
        }
        Adt7::C5 => {
            Ok(Adt7::C5)
        }
    }
}

pub fn f116(v0: Adt5, v1: Adt6, v2: u64, v3: u64, v4: Adt7, v5: u64) -> Adt5 {
    Adt5::C0(v2, List::cons(v1, match v0.clone() {
        Adt5::C0(_, h816, _, _, _, _, _, _, _) => {
            h816
        }
    }), Adt0::C5, 0u64, v3, v4, v5, None::<Adt1>, match v0 {
        Adt5::C0(_, _, _, _, _, _, _, _, h817) => {
            h817
        }
    })
}

pub fn f117(v0: List<Adt2>, v1: u64, v2: Adt0, v3: Adt5, v4: Adt6, v5: u64) -> R<Adt5> {
    let v6: u64 = f111(v0.clone(), v5);
    let a818 = v6;
    let a819 = 1u64;
    if nat_eq(a818, a819) {
        Ok(f114(v3, {
            let a820 = v5;
            let a821 = 1u64;
            nat_add(a820, a821)?
        }, f4(match v4 {
            Adt6::C0(_, _, _, _, _, _, _, _, h822, _, _, _) => {
                h822
            }
        })?))
    } else {
        let v7: u64 = f18(v0.clone(), v1, v5);
        let v8: Bytes = f119(v0.clone(), v5);
        let a823 = v6;
        let a824 = 11u64;
        if if nat_eq(a823, a824) {
            true
        } else {
            f120({
                let a825 = v6;
                let a826 = 12u64;
                nat_eq(a825, a826)
            }, f121(v8.clone())?)
        } {
            let a829 = f111(v0.clone(), {
                let a827 = v5;
                let a828 = 1u64;
                nat_add(a827, a828)?
            });
            let a830 = 7u64;
            if nat_eq(a829, a830) {
                Ok(f116(v3, f14(v4.clone(), v8.clone(), v7), {
                    let a831 = v5;
                    let a832 = 2u64;
                    nat_add(a831, a832)?
                }, 0u64, f166(v2, match v4.clone() {
                    Adt6::C0(_, _, _, _, _, _, h833, _, _, _, _, _) => {
                        h833
                    }
                }, v8)?, {
                    let a835 = match v4 {
                        Adt6::C0(_, _, h834, _, _, _, _, _, _, _, _, _) => {
                            h834
                        }
                    };
                    let a836 = 1u64;
                    nat_add(a835, a836)?
                }))
            } else {
                Ok(f109(v0, v1, v3, {
                    let a837 = v5;
                    let a838 = 1u64;
                    nat_add(a837, a838)?
                }, f10(Str::lit("expected `:` after a member name"))))
            }
        } else {
            Ok(f144(v3, v7, f10(Str::lit("expected a member name"))))
        }
    }
}

pub fn f118(v0: Adt0, v1: Adt7) -> Option<Adt0> {
    let m839 = v1;
    match m839 {
        Adt7::C0 => {
            None::<Adt0>
        }
        Adt7::C1 => {
            None::<Adt0>
        }
        Adt7::C2 => {
            None::<Adt0>
        }
        Adt7::C3 => {
            f26(v0, f10(Str::lit("parameter")))
        }
        Adt7::C4(v2) => {
            Some(v2)
        }
        Adt7::C5 => {
            None::<Adt0>
        }
    }
}

pub fn f119(v0: List<Adt2>, v1: u64) -> Bytes {
    f167(v0, v1)
}

pub fn f120(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f121(v0: Bytes) -> R<bool> {
    let a841 = {
        let a840 = v0.clone();
        length_bytes(a840)
    };
    let a842 = 0u64;
    if nat_eq(a841, a842) {
        Ok(false)
    } else {
        if f168(f169(v0.clone(), 0u64)?) {
            let a844 = f170(v0.clone(), 1u64)?;
            let a845 = {
                let a843 = v0.clone();
                length_bytes(a843)
            };
            if nat_eq(a844, a845) {
                let a846 = f171(v0);
                Ok(bool_not(a846))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f122(v0: Bytes) -> R<bool> {
    Ok(f120({
        let a848 = 0u64;
        let a849 = {
            let a847 = v0.clone();
            length_bytes(a847)
        };
        nat_lt(a848, a849)
    }, {
        let a851 = f172(v0.clone(), 0u64)?;
        let a852 = {
            let a850 = v0;
            length_bytes(a850)
        };
        nat_eq(a851, a852)
    }))
}

pub fn f123(v0: Adt0, v1: Adt7, v2: Bytes) -> R<Option<Adt0>> {
    let m853 = v1;
    match m853 {
        Adt7::C0 => {
            let m854 = f26(v0.clone(), f10(Str::lit("term")));
            match m854 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v3) => {
                    f173(v0, v3, v2)
                }
            }
        }
        Adt7::C1 => {
            let m855 = f26(v0.clone(), f10(Str::lit("type")));
            match m855 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    f173(v0, v4, v2)
                }
            }
        }
        Adt7::C2 => {
            Ok(None::<Adt0>)
        }
        Adt7::C3 => {
            Ok(None::<Adt0>)
        }
        Adt7::C4(v5) => {
            if f174(v0.clone(), v5.clone())? {
                f173(v0, v5, v2)
            } else {
                Ok(Some(v5))
            }
        }
        Adt7::C5 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f124(v0: Bytes) -> Option<Bytes> {
    if f72(v0.clone(), f10(Str::lit("Bool"))) {
        Some(f10(Str::lit("bool")))
    } else {
        if f72(v0.clone(), f10(Str::lit("Bytes"))) {
            Some(f10(Str::lit("bytes")))
        } else {
            if f72(v0.clone(), f10(Str::lit("Int"))) {
                Some(f10(Str::lit("int")))
            } else {
                if f72(v0.clone(), f10(Str::lit("Int8"))) {
                    Some(f10(Str::lit("int8")))
                } else {
                    if f72(v0.clone(), f10(Str::lit("Int16"))) {
                        Some(f10(Str::lit("int16")))
                    } else {
                        if f72(v0.clone(), f10(Str::lit("Int32"))) {
                            Some(f10(Str::lit("int32")))
                        } else {
                            if f72(v0.clone(), f10(Str::lit("Int64"))) {
                                Some(f10(Str::lit("int64")))
                            } else {
                                if f72(v0.clone(), f10(Str::lit("Nat"))) {
                                    Some(f10(Str::lit("nat")))
                                } else {
                                    if f72(v0.clone(), f10(Str::lit("Ordering"))) {
                                        Some(f10(Str::lit("ordering")))
                                    } else {
                                        if f72(v0.clone(), f10(Str::lit("Prop"))) {
                                            Some(f10(Str::lit("prop")))
                                        } else {
                                            if f72(v0.clone(), f10(Str::lit("String"))) {
                                                Some(f10(Str::lit("string")))
                                            } else {
                                                if f72(v0.clone(), f10(Str::lit("Type"))) {
                                                    Some(f10(Str::lit("type")))
                                                } else {
                                                    if f72(v0.clone(), f10(Str::lit("UInt8"))) {
                                                        Some(f10(Str::lit("uint8")))
                                                    } else {
                                                        if f72(v0.clone(), f10(Str::lit("UInt16"))) {
                                                            Some(f10(Str::lit("uint16")))
                                                        } else {
                                                            if f72(v0.clone(), f10(Str::lit("UInt32"))) {
                                                                Some(f10(Str::lit("uint32")))
                                                            } else {
                                                                if f72(v0.clone(), f10(Str::lit("UInt64"))) {
                                                                    Some(f10(Str::lit("uint64")))
                                                                } else {
                                                                    if f72(v0.clone(), f10(Str::lit("Unit"))) {
                                                                        Some(f10(Str::lit("unit")))
                                                                    } else {
                                                                        if f72(v0.clone(), f10(Str::lit("ContractViolation"))) {
                                                                            Some(f10(Str::lit("contract_violation")))
                                                                        } else {
                                                                            if f72(v0, f10(Str::lit("ReasoningFailure"))) {
                                                                                Some(f10(Str::lit("reasoning_failure")))
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

pub fn f125(v0: Bytes, v1: List<(Bytes, Adt0)>) -> R<Adt0> {
    f4(List::cons((f10(Str::lit("kind")), Adt0::C2(v0)), v1))
}

pub fn f126(v0: Bytes, v1: Bytes) -> bool {
    let m860 = {
        let a857 = v0;
        let a858 = 0u64;
        let a859 = {
            let a856 = v1.clone();
            length_bytes(a856)
        };
        slice_bytes(a857, a858, a859)
    };
    match m860 {
        None => {
            false
        }
        Some(v2) => {
            let a861 = v2;
            let a862 = v1;
            equal(a861, a862)
        }
    }
}

pub fn f127(v0: Bytes, v1: u64) -> Bytes {
    f66(v0.clone(), v1, {
        let a864 = {
            let a863 = v0;
            length_bytes(a863)
        };
        let a865 = v1;
        nat_sub(a864, a865)
    })
}

pub fn f128(v0: Str, v1: Adt0) -> List<(Bytes, Adt0)> {
    List::cons((f10(v0), v1), List::<(Bytes, Adt0)>::nil())
}

pub fn f129(v0: Bytes, v1: List<Bytes>) -> bool {
    f175(Fn17::F176(v0), false, v1)
}

pub fn f130() -> List<Bytes> {
    List::cons(f10(Str::lit("List")), List::cons(f10(Str::lit("Option")), List::cons(f10(Str::lit("Result")), List::cons(f10(Str::lit("Set")), List::cons(f10(Str::lit("Map")), List::cons(f10(Str::lit("Fn")), List::<Bytes>::nil()))))))
}

pub fn f131(v0: Bytes, v1: Bytes) -> Bytes {
    let a866 = v0;
    let a867 = v1;
    append_bytes(a866, a867)
}

pub fn f132(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: u64) -> R<Adt5> {
    let v5: Bytes = f119(v0.clone(), v3);
    let v6: bool = {
        let a868 = f111(v0.clone(), v3);
        let a869 = 12u64;
        nat_eq(a868, a869)
    };
    if f120(v6, if f72(v5.clone(), f10(Str::lit("List"))) {
        true
    } else {
        if f72(v5.clone(), f10(Str::lit("Set"))) {
            true
        } else {
            f72(v5.clone(), f10(Str::lit("Option")))
        }
    }) {
        Ok(f116(v2, f14(f17(12u64, v4), v5, 0u64), {
            let a870 = v3;
            let a871 = 1u64;
            nat_add(a870, a871)?
        }, 2u64, Adt7::C1, {
            let a872 = v4;
            let a873 = 1u64;
            nat_add(a872, a873)?
        }))
    } else {
        if f120(v6, if f72(v5.clone(), f10(Str::lit("Result"))) {
            true
        } else {
            f72(v5.clone(), f10(Str::lit("Map")))
        }) {
            Ok(f116(v2, f14(f17(13u64, v4), v5, 0u64), {
                let a874 = v3;
                let a875 = 1u64;
                nat_add(a874, a875)?
            }, 2u64, Adt7::C1, {
                let a876 = v4;
                let a877 = 1u64;
                nat_add(a876, a877)?
            }))
        } else {
            if f120(v6, f72(v5.clone(), f10(Str::lit("Fn")))) {
                let a880 = f111(v0.clone(), {
                    let a878 = v3;
                    let a879 = 1u64;
                    nat_add(a878, a879)?
                });
                let a881 = 4u64;
                if nat_eq(a880, a881) {
                    f146(v0, v1, v2, f17(14u64, v4), {
                        let a882 = v3;
                        let a883 = 2u64;
                        nat_add(a882, a883)?
                    })
                } else {
                    Ok(f109(v0, v1, v2, {
                        let a884 = v3;
                        let a885 = 1u64;
                        nat_add(a884, a885)?
                    }, f10(Str::lit("expected `(` before the parameter types"))))
                }
            } else {
                let v7: u64 = {
                    let a888 = f111(v0.clone(), {
                        let a886 = v3;
                        let a887 = 1u64;
                        nat_add(a886, a887)?
                    });
                    let a889 = 9u64;
                    if nat_eq(a888, a889) {
                        3u64
                    } else {
                        1u64
                    }
                };
                let v8: bool = f148(v0.clone(), {
                    let a890 = v3;
                    let a891 = v7;
                    nat_add(a890, a891)?
                }, f10(Str::lit("*")));
                if f120(v6, f120(!v8, f120(!f126(v5.clone(), f10(Str::lit("'"))), !f177(v5)))) {
                    let v9: Adt12 = f112(v0.clone(), v1, v3)?;
                    let m893 = match v9.clone() {
                        Adt12::C0(_, _, h892) => {
                            h892
                        }
                    };
                    match m893 {
                        None => {
                            f147(v0, v1, v2, f137(f17(16u64, v4), match v9.clone() {
                                Adt12::C0(h895, _, _) => {
                                    h895
                                }
                            }, Adt0::C5), match v9 {
                                Adt12::C0(_, h896, _) => {
                                    h896
                                }
                            })
                        }
                        Some(_) => {
                            Ok(f113(v2, match v9 {
                                Adt12::C0(_, _, h894) => {
                                    h894
                                }
                            }))
                        }
                    }
                } else {
                    Ok(f116(v2, f17(17u64, v4), v3, 2u64, Adt7::C1, {
                        let a897 = v4;
                        let a898 = 1u64;
                        nat_add(a897, a898)?
                    }))
                }
            }
        }
    }
}

pub fn f133(v0: List<Adt2>, v1: u64) -> R<Bytes> {
    let m899 = f30(v0, v1);
    match m899 {
        None => {
            Ok(f179(Str::lit("the end of the text")))
        }
        Some(v2) => {
            f178(match v2 {
                Adt2::C0(h900, _, _) => {
                    h900
                }
            })
        }
    }
}

pub fn f134(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = f111(v0.clone(), v3);
    let v6: Bytes = f119(v0.clone(), v3);
    let a901 = v5;
    let a902 = 5u64;
    if nat_eq(a901, a902) {
        Ok(f114(v2, v3, f125(f10(Str::lit("unit")), List::<(Bytes, Adt0)>::nil())?))
    } else {
        let a903 = v5;
        let a904 = 12u64;
        if nat_eq(a903, a904) {
            let v7: Bytes = if f126(v6.clone(), f10(Str::lit("-"))) {
                f127(v6.clone(), 1u64)
            } else {
                v6.clone()
            };
            if f120(f122(v7)?, {
                let a907 = f111(v0.clone(), {
                    let a905 = v3;
                    let a906 = 1u64;
                    nat_add(a905, a906)?
                });
                let a908 = 7u64;
                nat_eq(a907, a908)
            }) {
                f180(v0, v1, v2, v3, v6)
            } else {
                let v8: bool = f120(f72(v6.clone(), f10(Str::lit("let"))), f148(v0.clone(), {
                    let a909 = v3;
                    let a910 = 2u64;
                    nat_add(a909, a910)?
                }, f10(Str::lit("name"))));
                let v9: Bytes = if f120({
                    let a913 = f111(v0.clone(), {
                        let a911 = v3;
                        let a912 = 1u64;
                        nat_add(a911, a912)?
                    });
                    let a914 = 0u64;
                    nat_eq(a913, a914)
                }, !v8) {
                    f10(Str::lit(""))
                } else {
                    v6
                };
                f181(v0, v1, v2, v3, v4, v9)
            }
        } else {
            f149(v2, f17(47u64, v4), v3)
        }
    }
}

pub fn f135(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: Bytes) -> R<Adt5> {
    let v5: u64 = f18(v0, v1, v3);
    if f72(v4.clone(), f10(Str::lit("true"))) {
        Ok(f114(v2, {
            let a915 = v3;
            let a916 = 1u64;
            nat_add(a915, a916)?
        }, f125(f10(Str::lit("bool")), f128(Str::lit("value"), Adt0::C0(true)))?))
    } else {
        if f72(v4.clone(), f10(Str::lit("false"))) {
            Ok(f114(v2, {
                let a917 = v3;
                let a918 = 1u64;
                nat_add(a917, a918)?
            }, f125(f10(Str::lit("bool")), f128(Str::lit("value"), Adt0::C0(false)))?))
        } else {
            if f122(v4.clone())? {
                if f120({
                    let a920 = 1u64;
                    let a921 = {
                        let a919 = v4.clone();
                        length_bytes(a919)
                    };
                    nat_lt(a920, a921)
                }, f126(v4.clone(), f10(Str::lit("0")))) {
                    Ok(f144(v2, v5, f131(f182(v4), f10(Str::lit(" has a leading zero")))))
                } else {
                    Ok(f114(v2, {
                        let a922 = v3;
                        let a923 = 1u64;
                        nat_add(a922, a923)?
                    }, f125(f10(Str::lit("nat")), f128(Str::lit("value"), f6(v4)))?))
                }
            } else {
                if f126(v4.clone(), f10(Str::lit("0x"))) {
                    if f183(f127(v4.clone(), 2u64))? {
                        Ok(f114(v2, {
                            let a924 = v3;
                            let a925 = 1u64;
                            nat_add(a924, a925)?
                        }, f125(f10(Str::lit("bytes")), f128(Str::lit("hex"), f6(f127(v4, 2u64))))?))
                    } else {
                        Ok(f144(v2, v5, f10(Str::lit("a byte string is `0x` and an even number of lower-case hexadecimal digits"))))
                    }
                } else {
                    if f184(v4.clone())? {
                        Ok(f114(v2, {
                            let a926 = v3;
                            let a927 = 1u64;
                            nat_add(a926, a927)?
                        }, f125(f10(Str::lit("var")), f128(Str::lit("name"), f6(v4)))?))
                    } else {
                        Ok(f144(v2, v5, if f129(v4.clone(), f185()) {
                            f131(f131(f131(f182(v4.clone()), f10(Str::lit(" is a keyword; write `var{name: "))), v4), f10(Str::lit("}` for a variable of that name")))
                        } else {
                            f131(f182(v4), f10(Str::lit(" is not a term")))
                        }))
                    }
                }
            }
        }
    }
}

pub fn f136(v0: List<Adt2>, v1: u64, v2: u64, v3: Bytes) -> R<Adt12> {
    let v4: Bytes = f119(v0.clone(), v2);
    if f120({
        let a928 = f111(v0.clone(), v2);
        let a929 = 12u64;
        nat_eq(a928, a929)
    }, f121(v4.clone())?) {
        Ok(Adt12::C0(f6(v4), {
            let a930 = v2;
            let a931 = 1u64;
            nat_add(a930, a931)?
        }, None::<Adt1>))
    } else {
        Ok(Adt12::C0(Adt0::C5, v2, f186(f18(v0.clone(), v1, v2), f131(f131(f131(f10(Str::lit("expected ")), v3), f10(Str::lit(", found "))), f133(v0, v2)?))))
    }
}

pub fn f137(v0: Adt6, v1: Adt0, v2: Adt0) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h932, _, _, _, _, _, _, _, _, _, _, _) => {
            h932
        }
    }, match v0.clone() {
        Adt6::C0(_, h933, _, _, _, _, _, _, _, _, _, _) => {
            h933
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h934, _, _, _, _, _, _, _, _, _) => {
            h934
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h935, _, _, _, _, _, _, _, _) => {
            h935
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h936, _, _, _, _, _, _, _) => {
            h936
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h937, _, _, _, _, _, _) => {
            h937
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h938, _, _, _, _, _) => {
            h938
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h939, _, _, _, _) => {
            h939
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h940, _, _, _) => {
            h940
        }
    }, v1, v2, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h941) => {
            h941
        }
    })
}

pub fn f138(v0: List<Adt2>, v1: u64, v2: u64, v3: u64) -> R<Adt13> {
    let (h942, _) = f187(Fn18::F188(v1, v0.clone()), {
        let a946 = {
            let a944 = {
                let a943 = v0;
                length_list(a943)
            };
            let a945 = 4u64;
            nat_mul(a944, a945)?
        };
        let a947 = 8u64;
        nat_add(a946, a947)?
    }, Adt13::C0(v2, v3, 0u64, List::<Adt14>::nil(), Adt0::C5, None::<Adt1>))?;
    Ok(h942)
}

pub fn f139(v0: List<Adt2>, _: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let a948 = f111(v0, v4);
    let a949 = 3u64;
    if nat_eq(a948, a949) {
        Ok(f114(v2, {
            let a950 = v4;
            let a951 = 1u64;
            nat_add(a950, a951)?
        }, f12(match v3 {
            Adt6::C0(_, _, _, _, _, _, _, h952, _, _, _, _) => {
                h952
            }
        })))
    } else {
        f145(v2, v3, v4)
    }
}

pub fn f140(v0: Adt6, v1: (u64, u64), v2: u64) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h953, _, _, _, _, _, _, _, _, _, _, _) => {
            h953
        }
    }, match v0.clone() {
        Adt6::C0(_, h954, _, _, _, _, _, _, _, _, _, _) => {
            h954
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h955, _, _, _, _, _, _, _, _, _) => {
            h955
        }
    }, v2, match v0.clone() {
        Adt6::C0(_, _, _, _, h956, _, _, _, _, _, _, _) => {
            h956
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h957, _, _, _, _, _, _) => {
            h957
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h958, _, _, _, _, _) => {
            h958
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h959, _, _, _, _) => {
            h959
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h960, _, _, _) => {
            h960
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h961, _, _) => {
            h961
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h962, _) => {
            h962
        }
    }, List::cons(v1, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h963) => {
            h963
        }
    }))
}

pub fn f141(v0: Adt6, v1: Adt0) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h964, _, _, _, _, _, _, _, _, _, _, _) => {
            h964
        }
    }, match v0.clone() {
        Adt6::C0(_, h965, _, _, _, _, _, _, _, _, _, _) => {
            h965
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h966, _, _, _, _, _, _, _, _, _) => {
            h966
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h967, _, _, _, _, _, _, _, _) => {
            h967
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h968, _, _, _, _, _, _, _) => {
            h968
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h969, _, _, _, _, _, _) => {
            h969
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h970, _, _, _, _, _) => {
            h970
        }
    }, List::cons(v1, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h971, _, _, _, _) => {
            h971
        }
    }), match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h972, _, _, _) => {
            h972
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h973, _, _) => {
            h973
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h974, _) => {
            h974
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h975) => {
            h975
        }
    })
}

pub fn f142(v0: List<Adt2>, v1: u64) -> u64 {
    let a976 = v1;
    let a977 = 0u64;
    if nat_eq(a976, a977) {
        0u64
    } else {
        let m980 = f30(v0, {
            let a978 = v1;
            let a979 = 1u64;
            nat_sub(a978, a979)
        });
        match m980 {
            None => {
                0u64
            }
            Some(v2) => {
                match v2 {
                    Adt2::C0(_, _, h981) => {
                        h981
                    }
                }
            }
        }
    }
}

pub fn f143(v0: List<(Bytes, Adt0)>, v1: Bytes) -> bool {
    f189(Fn19::F190(v1), false, v0)
}

pub fn f144(v0: Adt5, v1: u64, v2: Bytes) -> Adt5 {
    Adt5::C0(match v0.clone() {
        Adt5::C0(h982, _, _, _, _, _, _, _, _) => {
            h982
        }
    }, match v0.clone() {
        Adt5::C0(_, h983, _, _, _, _, _, _, _) => {
            h983
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h984, _, _, _, _, _, _) => {
            h984
        }
    }, 9u64, match v0.clone() {
        Adt5::C0(_, _, _, _, h985, _, _, _, _) => {
            h985
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h986, _, _, _) => {
            h986
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h987, _, _) => {
            h987
        }
    }, f186(v1, v2), match v0 {
        Adt5::C0(_, _, _, _, _, _, _, _, h988) => {
            h988
        }
    })
}

pub fn f145(v0: Adt5, v1: Adt6, v2: u64) -> R<Adt5> {
    Ok(f116(v0, v1.clone(), v2, 2u64, Adt7::C1, {
        let a990 = match v1 {
            Adt6::C0(_, _, h989, _, _, _, _, _, _, _, _, _) => {
                h989
            }
        };
        let a991 = 1u64;
        nat_add(a990, a991)?
    }))
}

pub fn f146(v0: List<Adt2>, _: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let a992 = f111(v0, v4);
    let a993 = 5u64;
    if nat_eq(a992, a993) {
        f145(v2, f15(v3, 1u64), {
            let a994 = v4;
            let a995 = 1u64;
            nat_add(a994, a995)?
        })
    } else {
        f145(v2, v3, v4)
    }
}

pub fn f147(v0: List<Adt2>, _: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let a996 = f111(v0, v4);
    let a997 = 5u64;
    if nat_eq(a996, a997) {
        Ok(f114(v2, v4, f125(f10(Str::lit("named")), f5(Str::lit("member"), match v3.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, h998, _, _) => {
                h998
            }
        }, Str::lit("arguments"), f12(match v3 {
            Adt6::C0(_, _, _, _, _, _, _, h999, _, _, _, _) => {
                h999
            }
        })))?))
    } else {
        f145(v2, v3, v4)
    }
}

pub fn f148(v0: List<Adt2>, v1: u64, v2: Bytes) -> bool {
    f120({
        let a1000 = f111(v0.clone(), v1);
        let a1001 = 12u64;
        nat_eq(a1000, a1001)
    }, f72(f119(v0, v1), v2))
}

pub fn f149(v0: Adt5, v1: Adt6, v2: u64) -> R<Adt5> {
    Ok(f116(v0, v1.clone(), v2, 3u64, Adt7::C0, {
        let a1003 = match v1 {
            Adt6::C0(_, _, h1002, _, _, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1004 = 1u64;
        nat_add(a1003, a1004)?
    }))
}

pub fn f150(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let a1005 = f111(v0.clone(), v4);
    let a1006 = 4u64;
    if nat_eq(a1005, a1006) {
        f191(v2, v3, {
            let a1007 = v4;
            let a1008 = 1u64;
            nat_add(a1007, a1008)?
        })
    } else {
        let m1010 = match v3.clone() {
            Adt6::C0(_, _, _, _, _, _, _, h1009, _, _, _, _) => {
                h1009
            }
        };
        match m1010.uncons() {
            None => {
                Ok(f109(v0, v1, v2, v4, f10(Str::lit("a function takes at least one parameter"))))
            }
            Some((_, _)) => {
                if f148(v0.clone(), v4, f10(Str::lit("=>"))) {
                    f149(v2, f15(v3, 1u64), {
                        let a1011 = v4;
                        let a1012 = 1u64;
                        nat_add(a1011, a1012)?
                    })
                } else {
                    Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `=>`"))))
                }
            }
        }
    }
}

pub fn f151(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let v5: Bytes = match v3.clone() {
        Adt6::C0(_, _, _, _, h1013, _, _, _, _, _, _, _) => {
            h1013
        }
    };
    let a1015 = {
        let a1014 = v5.clone();
        length_bytes(a1014)
    };
    let a1016 = 0u64;
    if if nat_eq(a1015, a1016) {
        let a1017 = f111(v0.clone(), v4);
        let a1018 = 5u64;
        nat_eq(a1017, a1018)
    } else {
        f148(v0.clone(), v4, v5)
    } {
        f192(v0, v1, v2, v3, v4)
    } else {
        let a1019 = f111(v0, v4);
        let a1020 = 99u64;
        if nat_eq(a1019, a1020) {
            Ok(f144(v2, v1, f10(Str::lit("expected `)`"))))
        } else {
            f149(v2, v3, v4)
        }
    }
}

pub fn f152(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6) -> R<Adt5> {
    let v4: u64 = match v2.clone() {
        Adt5::C0(h1021, _, _, _, _, _, _, _, _) => {
            h1021
        }
    };
    let v5: u64 = match v3.clone() {
        Adt6::C0(h1022, _, _, _, _, _, _, _, _, _, _, _) => {
            h1022
        }
    };
    let v6: Adt6 = f137(v3.clone(), match v3.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1023, _, _) => {
            h1023
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h1024, _, _, _, _, _, _) => {
            h1024
        }
    });
    let a1025 = v5;
    let a1026 = 41u64;
    if nat_eq(a1025, a1026) {
        Ok(f114(v2.clone(), v4, f125(f10(Str::lit("function_ref")), f193(f128(Str::lit("function"), match v3 {
            Adt6::C0(_, _, _, _, _, _, _, _, _, h1027, _, _) => {
                h1027
            }
        }), match v2 {
            Adt5::C0(_, _, h1028, _, _, _, _, _, _) => {
                h1028
            }
        }))?))
    } else {
        let a1029 = v5;
        let a1030 = 42u64;
        if nat_eq(a1029, a1030) {
            let a1031 = f111(v0.clone(), v4);
            let a1032 = 0u64;
            if nat_eq(a1031, a1032) {
                f153(v0, v1, v2, f15(v6, 1u64), {
                    let a1033 = v4;
                    let a1034 = 1u64;
                    nat_add(a1033, a1034)?
                })
            } else {
                Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `{` before the fields"))))
            }
        } else {
            f151(v0, v1, v2, f15(v6, 1u64), v4)
        }
    }
}

pub fn f153(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let a1035 = f111(v0.clone(), v4);
    let a1036 = 1u64;
    if nat_eq(a1035, a1036) {
        Ok(f114(v2, {
            let a1037 = v4;
            let a1038 = 1u64;
            nat_add(a1037, a1038)?
        }, f125(f10(Str::lit("record")), f193(f5(Str::lit("type"), match v3.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, h1039, _, _) => {
                h1039
            }
        }, Str::lit("fields"), f12(match v3.clone() {
            Adt6::C0(_, _, _, _, _, _, _, h1040, _, _, _, _) => {
                h1040
            }
        })), match v3 {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1041, _) => {
                h1041
            }
        }))?))
    } else {
        let v5: Adt12 = f136(v0.clone(), v1, v4, f10(Str::lit("a field name")))?;
        let m1043 = match v5.clone() {
            Adt12::C0(_, _, h1042) => {
                h1042
            }
        };
        match m1043 {
            None => {
                let a1046 = f111(v0.clone(), match v5.clone() {
                    Adt12::C0(_, h1045, _) => {
                        h1045
                    }
                });
                let a1047 = 8u64;
                if nat_eq(a1046, a1047) {
                    f149(v2, f14(v3, {
                        let m1049 = f85(match v5.clone() {
                            Adt12::C0(h1048, _, _) => {
                                h1048
                            }
                        });
                        match m1049 {
                            None => {
                                f10(Str::lit(""))
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, 0u64), {
                        let a1051 = match v5 {
                            Adt12::C0(_, h1050, _) => {
                                h1050
                            }
                        };
                        let a1052 = 1u64;
                        nat_add(a1051, a1052)?
                    })
                } else {
                    Ok(f109(v0, v1, v2, match v5 {
                        Adt12::C0(_, h1053, _) => {
                            h1053
                        }
                    }, f10(Str::lit("expected `:=` after a field name"))))
                }
            }
            Some(_) => {
                Ok(f113(v2, match v5 {
                    Adt12::C0(_, _, h1044) => {
                        h1044
                    }
                }))
            }
        }
    }
}

pub fn f154(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let v5: u64 = f111(v0.clone(), v4);
    let a1054 = v5;
    let a1055 = 7u64;
    if nat_eq(a1054, a1055) {
        f145(v2, f15(v3, 1u64), {
            let a1056 = v4;
            let a1057 = 1u64;
            nat_add(a1056, a1057)?
        })
    } else {
        let a1058 = v5;
        let a1059 = 99u64;
        if if nat_eq(a1058, a1059) {
            true
        } else {
            let a1060 = v5;
            let a1061 = 5u64;
            nat_eq(a1060, a1061)
        } {
            Ok(f109(v0, v1, v2, v4, f10(Str::lit("expected `:` and the result type"))))
        } else {
            f149(v2, v3, v4)
        }
    }
}

pub fn f155(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    if f148(v0.clone(), v4, f10(Str::lit("|"))) {
        let v5: Adt12 = f112(v0.clone(), v1, {
            let a1062 = v4;
            let a1063 = 1u64;
            nat_add(a1062, a1063)?
        })?;
        let m1065 = match v5.clone() {
            Adt12::C0(_, _, h1064) => {
                h1064
            }
        };
        match m1065 {
            None => {
                let v7: Adt15 = {
                    let (h1067, _) = f194(Fn20::F195(v1, v0.clone()), {
                        let a1069 = {
                            let a1068 = v0;
                            length_list(a1068)
                        };
                        let a1070 = 2u64;
                        nat_add(a1069, a1070)?
                    }, Adt15::C0(List::<Adt0>::nil(), match v5.clone() {
                        Adt12::C0(_, h1071, _) => {
                            h1071
                        }
                    }, true, None::<Adt1>))?;
                    h1067
                };
                let m1073 = match v7.clone() {
                    Adt15::C0(_, _, _, h1072) => {
                        h1072
                    }
                };
                match m1073 {
                    None => {
                        f149(v2, f16(f15(v3.clone(), 1u64), match v3.clone() {
                            Adt6::C0(_, _, _, _, _, h1075, _, _, _, _, _, _) => {
                                h1075
                            }
                        }, match v3 {
                            Adt6::C0(_, _, _, _, _, _, h1076, _, _, _, _, _) => {
                                h1076
                            }
                        }, f5(Str::lit("constructor"), match v5 {
                            Adt12::C0(h1077, _, _) => {
                                h1077
                            }
                        }, Str::lit("binders"), f12(match v7.clone() {
                            Adt15::C0(h1078, _, _, _) => {
                                h1078
                            }
                        }))), {
                            let a1080 = match v7 {
                                Adt15::C0(_, h1079, _, _) => {
                                    h1079
                                }
                            };
                            let a1081 = 1u64;
                            nat_add(a1080, a1081)?
                        })
                    }
                    Some(_) => {
                        Ok(f113(v2, match v7 {
                            Adt15::C0(_, _, _, h1074) => {
                                h1074
                            }
                        }))
                    }
                }
            }
            Some(_) => {
                Ok(f113(v2, match v5 {
                    Adt12::C0(_, _, h1066) => {
                        h1066
                    }
                }))
            }
        }
    } else {
        let m1083 = match v3.clone() {
            Adt6::C0(_, _, _, _, _, _, _, h1082, _, _, _, _) => {
                h1082
            }
        };
        match m1083.uncons() {
            None => {
                Ok(f109(v0, v1, v2, v4, f10(Str::lit("a match has at least one `| \u{2026}` branch"))))
            }
            Some((_, _)) => {
                Ok(f114(v2, v4, f125(f10(Str::lit("match")), f5(Str::lit("scrutinee"), match v3.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, h1084, _, _) => {
                        h1084
                    }
                }, Str::lit("branches"), f12(match v3 {
                    Adt6::C0(_, _, _, _, _, _, _, h1085, _, _, _, _) => {
                        h1085
                    }
                })))?))
            }
        }
    }
}

pub fn f156(v0: Bytes) -> Option<Bytes> {
    if f72(v0.clone(), f10(Str::lit("+"))) {
        Some(f10(Str::lit("add")))
    } else {
        if f72(v0.clone(), f10(Str::lit("="))) {
            Some(f10(Str::lit("eq")))
        } else {
            if f72(v0.clone(), f10(Str::lit("<="))) {
                Some(f10(Str::lit("le")))
            } else {
                if f72(v0.clone(), f10(Str::lit("<"))) {
                    Some(f10(Str::lit("lt")))
                } else {
                    if f72(v0.clone(), f10(Str::lit("=?"))) {
                        Some(f10(Str::lit("beq")))
                    } else {
                        if f72(v0.clone(), f10(Str::lit("<=?"))) {
                            Some(f10(Str::lit("ble")))
                        } else {
                            if f72(v0.clone(), f10(Str::lit("<?"))) {
                                Some(f10(Str::lit("blt")))
                            } else {
                                if f72(v0.clone(), f10(Str::lit("&&"))) {
                                    Some(f10(Str::lit("and")))
                                } else {
                                    if f72(v0.clone(), f10(Str::lit("||"))) {
                                        Some(f10(Str::lit("or")))
                                    } else {
                                        if f72(v0.clone(), f10(Str::lit("/\\"))) {
                                            Some(f10(Str::lit("prop_and")))
                                        } else {
                                            if f72(v0.clone(), f10(Str::lit("<->"))) {
                                                Some(f10(Str::lit("iff")))
                                            } else {
                                                if f72(v0, f10(Str::lit("->"))) {
                                                    Some(f10(Str::lit("implies")))
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

pub fn f157(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h1086, _) = v0;
        h1086
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h1087) = v0;
        h1087
    };
    let m1088 = v1;
    match m1088.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m1089 = v4;
            match m1089.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f196(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f158(v0: Bytes, v1: u64, v2: Adt9) -> R<Adt9> {
    let v3: u64 = match v2.clone() {
        Adt9::C0(h1090, _, _, _) => {
            h1090
        }
    };
    let m1093 = f60(v0.clone(), {
        let a1091 = v3;
        let a1092 = 1u64;
        nat_add(a1091, a1092)?
    });
    match m1093 {
        None => {
            Ok(Adt9::C0(v1, match v2 {
                Adt9::C0(_, h1094, _, _) => {
                    h1094
                }
            }, false, None::<Adt1>))
        }
        Some(v4) => {
            let a1095 = v4;
            let a1096 = 34u8;
            if equal(a1095, a1096) {
                Ok(f162(v2, {
                    let a1097 = v3;
                    let a1098 = 2u64;
                    nat_add(a1097, a1098)?
                }, f64(Str::lit("\""))))
            } else {
                let a1099 = v4;
                let a1100 = 92u8;
                if equal(a1099, a1100) {
                    Ok(f162(v2, {
                        let a1101 = v3;
                        let a1102 = 2u64;
                        nat_add(a1101, a1102)?
                    }, f64(Str::lit("\\"))))
                } else {
                    let a1103 = v4;
                    let a1104 = 47u8;
                    if equal(a1103, a1104) {
                        Ok(f162(v2, {
                            let a1105 = v3;
                            let a1106 = 2u64;
                            nat_add(a1105, a1106)?
                        }, f64(Str::lit("/"))))
                    } else {
                        let a1107 = v4;
                        let a1108 = 98u8;
                        if equal(a1107, a1108) {
                            Ok(f162(v2, {
                                let a1109 = v3;
                                let a1110 = 2u64;
                                nat_add(a1109, a1110)?
                            }, f197(8u64)))
                        } else {
                            let a1111 = v4;
                            let a1112 = 102u8;
                            if equal(a1111, a1112) {
                                Ok(f162(v2, {
                                    let a1113 = v3;
                                    let a1114 = 2u64;
                                    nat_add(a1113, a1114)?
                                }, f197(12u64)))
                            } else {
                                let a1115 = v4;
                                let a1116 = 110u8;
                                if equal(a1115, a1116) {
                                    Ok(f162(v2, {
                                        let a1117 = v3;
                                        let a1118 = 2u64;
                                        nat_add(a1117, a1118)?
                                    }, f197(10u64)))
                                } else {
                                    let a1119 = v4;
                                    let a1120 = 114u8;
                                    if equal(a1119, a1120) {
                                        Ok(f162(v2, {
                                            let a1121 = v3;
                                            let a1122 = 2u64;
                                            nat_add(a1121, a1122)?
                                        }, f197(13u64)))
                                    } else {
                                        let a1123 = v4;
                                        let a1124 = 116u8;
                                        if equal(a1123, a1124) {
                                            Ok(f162(v2, {
                                                let a1125 = v3;
                                                let a1126 = 2u64;
                                                nat_add(a1125, a1126)?
                                            }, f197(9u64)))
                                        } else {
                                            let a1127 = v4;
                                            let a1128 = 117u8;
                                            if equal(a1127, a1128) {
                                                f198(v0, v2)
                                            } else {
                                                Ok(f160(v2, v3, f199(f199(f64(Str::lit("`\\")), f66(v0.clone(), {
                                                    let a1129 = v3;
                                                    let a1130 = 1u64;
                                                    nat_add(a1129, a1130)?
                                                }, f200(v0, {
                                                    let a1131 = v3;
                                                    let a1132 = 1u64;
                                                    nat_add(a1131, a1132)?
                                                })?)), f64(Str::lit("` is not an escape")))))
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

pub fn f159(v0: Bytes, v1: u64) -> R<u64> {
    let m1133 = f60(v0, v1);
    match m1133 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1178 = {
                let a1154 = {
                    let a1142 = {
                        let a1136 = {
                            let a1134 = v3;
                            let a1135 = 128u8;
                            fixed_u8::bit_and(a1134, a1135)
                        };
                        let a1137 = 128u8;
                        if equal(a1136, a1137) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1143 = {
                        let a1140 = {
                            let a1138 = v3;
                            let a1139 = 64u8;
                            fixed_u8::bit_and(a1138, a1139)
                        };
                        let a1141 = 64u8;
                        if equal(a1140, a1141) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1142, a1143)?
                };
                let a1155 = {
                    let a1152 = {
                        let a1146 = {
                            let a1144 = v3;
                            let a1145 = 32u8;
                            fixed_u8::bit_and(a1144, a1145)
                        };
                        let a1147 = 32u8;
                        if equal(a1146, a1147) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1153 = {
                        let a1150 = {
                            let a1148 = v3;
                            let a1149 = 16u8;
                            fixed_u8::bit_and(a1148, a1149)
                        };
                        let a1151 = 16u8;
                        if equal(a1150, a1151) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1152, a1153)?
                };
                nat_add(a1154, a1155)?
            };
            let a1179 = {
                let a1176 = {
                    let a1164 = {
                        let a1158 = {
                            let a1156 = v3;
                            let a1157 = 8u8;
                            fixed_u8::bit_and(a1156, a1157)
                        };
                        let a1159 = 8u8;
                        if equal(a1158, a1159) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1165 = {
                        let a1162 = {
                            let a1160 = v3;
                            let a1161 = 4u8;
                            fixed_u8::bit_and(a1160, a1161)
                        };
                        let a1163 = 4u8;
                        if equal(a1162, a1163) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1164, a1165)?
                };
                let a1177 = {
                    let a1174 = {
                        let a1168 = {
                            let a1166 = v3;
                            let a1167 = 2u8;
                            fixed_u8::bit_and(a1166, a1167)
                        };
                        let a1169 = 2u8;
                        if equal(a1168, a1169) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1175 = {
                        let a1172 = {
                            let a1170 = v3;
                            let a1171 = 1u8;
                            fixed_u8::bit_and(a1170, a1171)
                        };
                        let a1173 = 1u8;
                        if equal(a1172, a1173) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1174, a1175)?
                };
                nat_add(a1176, a1177)?
            };
            nat_add(a1178, a1179)
        }
    }
}

pub fn f160(v0: Adt9, v1: u64, v2: Bytes) -> Adt9 {
    Adt9::C0(match v0.clone() {
        Adt9::C0(h1180, _, _, _) => {
            h1180
        }
    }, match v0 {
        Adt9::C0(_, h1181, _, _) => {
            h1181
        }
    }, false, Some(f63(v1, v2)))
}

pub fn f161(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1182, _) = f81(Fn12::F201(v1, v0), {
        let a1183 = v1;
        let a1184 = v2;
        nat_sub(a1183, a1184)
    }, v2)?;
    Ok(h1182)
}

pub fn f162(v0: Adt9, v1: u64, v2: Bytes) -> Adt9 {
    Adt9::C0(v1, List::cons(v2, match v0 {
        Adt9::C0(_, h1185, _, _) => {
            h1185
        }
    }), false, None::<Adt1>)
}

pub fn f163(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m1186 = v0.clone();
    match m1186.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m1187 = v2;
            match m1187.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f202(Fn21::F203, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m1189 = {
                        let (h1188, _) = v5;
                        h1188
                    };
                    match m1189 {
                        None => {
                            Some(f80({
                                let (_, h1191) = v5;
                                h1191
                            }))
                        }
                        Some(v6) => {
                            Some(f80(List::cons(v6, {
                                let (_, h1190) = v5;
                                h1190
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f164(v0: Bytes, v1: u64) -> R<bool> {
    let a1192 = 0u64;
    let a1193 = f39(v0.clone(), v1)?;
    if nat_lt(a1192, a1193) {
        Ok(true)
    } else {
        Ok(f204(v0, v1))
    }
}

pub fn f165(v0: List<Adt2>, v1: u64) -> u64 {
    let m1194 = f30(v0, v1);
    match m1194 {
        None => {
            99u64
        }
        Some(v2) => {
            f205(match v2 {
                Adt2::C0(h1195, _, _) => {
                    h1195
                }
            })
        }
    }
}

pub fn f166(v0: Adt0, v1: Option<Adt0>, v2: Bytes) -> R<Adt7> {
    let m1196 = v1;
    match m1196 {
        None => {
            Ok(Adt7::C5)
        }
        Some(v3) => {
            let m1197 = f24(f48(v0.clone(), v3)?, f10(Str::lit("properties")));
            match m1197 {
                None => {
                    Ok(Adt7::C5)
                }
                Some(v4) => {
                    let m1198 = f24(v4, v2);
                    match m1198 {
                        None => {
                            Ok(Adt7::C5)
                        }
                        Some(v5) => {
                            f49(v0, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f167(v0: List<Adt2>, v1: u64) -> Bytes {
    let m1199 = f30(v0, v1);
    match m1199 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f206(match v2 {
                Adt2::C0(h1200, _, _) => {
                    h1200
                }
            })
        }
    }
}

pub fn f168(v0: u64) -> bool {
    let a1201 = 64u64;
    let a1202 = v0;
    if if nat_lt(a1201, a1202) {
        let a1203 = v0;
        let a1204 = 91u64;
        nat_lt(a1203, a1204)
    } else {
        false
    } {
        true
    } else {
        let a1205 = 96u64;
        let a1206 = v0;
        if if nat_lt(a1205, a1206) {
            let a1207 = v0;
            let a1208 = 123u64;
            nat_lt(a1207, a1208)
        } else {
            false
        } {
            true
        } else {
            let a1209 = v0;
            let a1210 = 95u64;
            nat_eq(a1209, a1210)
        }
    }
}

pub fn f169(v0: Bytes, v1: u64) -> R<u64> {
    let m1211 = f60(v0, v1);
    match m1211 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1256 = {
                let a1232 = {
                    let a1220 = {
                        let a1214 = {
                            let a1212 = v3;
                            let a1213 = 128u8;
                            fixed_u8::bit_and(a1212, a1213)
                        };
                        let a1215 = 128u8;
                        if equal(a1214, a1215) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1221 = {
                        let a1218 = {
                            let a1216 = v3;
                            let a1217 = 64u8;
                            fixed_u8::bit_and(a1216, a1217)
                        };
                        let a1219 = 64u8;
                        if equal(a1218, a1219) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1220, a1221)?
                };
                let a1233 = {
                    let a1230 = {
                        let a1224 = {
                            let a1222 = v3;
                            let a1223 = 32u8;
                            fixed_u8::bit_and(a1222, a1223)
                        };
                        let a1225 = 32u8;
                        if equal(a1224, a1225) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1231 = {
                        let a1228 = {
                            let a1226 = v3;
                            let a1227 = 16u8;
                            fixed_u8::bit_and(a1226, a1227)
                        };
                        let a1229 = 16u8;
                        if equal(a1228, a1229) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1230, a1231)?
                };
                nat_add(a1232, a1233)?
            };
            let a1257 = {
                let a1254 = {
                    let a1242 = {
                        let a1236 = {
                            let a1234 = v3;
                            let a1235 = 8u8;
                            fixed_u8::bit_and(a1234, a1235)
                        };
                        let a1237 = 8u8;
                        if equal(a1236, a1237) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1243 = {
                        let a1240 = {
                            let a1238 = v3;
                            let a1239 = 4u8;
                            fixed_u8::bit_and(a1238, a1239)
                        };
                        let a1241 = 4u8;
                        if equal(a1240, a1241) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1242, a1243)?
                };
                let a1255 = {
                    let a1252 = {
                        let a1246 = {
                            let a1244 = v3;
                            let a1245 = 2u8;
                            fixed_u8::bit_and(a1244, a1245)
                        };
                        let a1247 = 2u8;
                        if equal(a1246, a1247) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1253 = {
                        let a1250 = {
                            let a1248 = v3;
                            let a1249 = 1u8;
                            fixed_u8::bit_and(a1248, a1249)
                        };
                        let a1251 = 1u8;
                        if equal(a1250, a1251) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1252, a1253)?
                };
                nat_add(a1254, a1255)?
            };
            nat_add(a1256, a1257)
        }
    }
}

pub fn f170(v0: Bytes, v1: u64) -> R<u64> {
    let (h1258, _) = f81(Fn12::F207(v0.clone()), {
        let a1260 = {
            let a1259 = v0;
            length_bytes(a1259)
        };
        let a1261 = 1u64;
        nat_add(a1260, a1261)?
    }, v1)?;
    Ok(h1258)
}

pub fn f171(v0: Bytes) -> bool {
    let a1262 = v0.clone();
    let a1263 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a1262, a1263) {
        true
    } else {
        let a1264 = v0;
        let a1265 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a1264, a1265)
    }
}

pub fn f172(v0: Bytes, v1: u64) -> R<u64> {
    let (h1266, _) = f81(Fn12::F208(v0.clone()), {
        let a1268 = {
            let a1267 = v0;
            length_bytes(a1267)
        };
        let a1269 = 1u64;
        nat_add(a1268, a1269)?
    }, v1)?;
    Ok(h1266)
}

pub fn f173(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Option<Adt0>> {
    let m1270 = f209(f24(f48(v0.clone(), v1)?, f10(Str::lit("oneOf"))));
    match m1270 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v3) => {
            f210(Fn22::F211(v2, v0), None::<Adt0>, f212(v3))
        }
    }
}

pub fn f174(v0: Adt0, v1: Adt0) -> R<bool> {
    let m1271 = f209(f24(f48(v0.clone(), v1)?, f10(Str::lit("oneOf"))));
    match m1271 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            f213(Fn23::F214(v0), true, f212(v2))
        }
    }
}

pub fn f175(v0: Fn17, p1275: bool, p1276: List<Bytes>) -> bool {
    let m1274 = (p1275, p1276);
    {
        let mut m1274 = m1274;
        loop {
            let step = {
                let (v1, v2) = m1274;
                let m1272 = v2;
                match m1272.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1273: Fn17 = v0.clone();
                            c1273.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1277 = v2;
        let a1278 = v0;
        equal(a1277, a1278)
    }
}

pub fn f177(v0: Bytes) -> bool {
    let m1279 = f124(v0.clone());
    match m1279 {
        None => {
            f129(v0, f130())
        }
        Some(_) => {
            true
        }
    }
}

pub fn f178(v0: Adt3) -> R<Bytes> {
    let m1280 = v0;
    match m1280 {
        Adt3::C0 => {
            Ok(f179(Str::lit("`{`")))
        }
        Adt3::C1 => {
            Ok(f179(Str::lit("`}`")))
        }
        Adt3::C2 => {
            Ok(f179(Str::lit("`[`")))
        }
        Adt3::C3 => {
            Ok(f179(Str::lit("`]`")))
        }
        Adt3::C4 => {
            Ok(f179(Str::lit("`(`")))
        }
        Adt3::C5 => {
            Ok(f179(Str::lit("`)`")))
        }
        Adt3::C6 => {
            Ok(f179(Str::lit("`,`")))
        }
        Adt3::C7 => {
            Ok(f179(Str::lit("`:`")))
        }
        Adt3::C8 => {
            Ok(f179(Str::lit("`:=`")))
        }
        Adt3::C9 => {
            Ok(f179(Str::lit("`::`")))
        }
        Adt3::C10 => {
            Ok(f179(Str::lit("`;`")))
        }
        Adt3::C11(v1) => {
            Ok(f215(f179(Str::lit("the string ")), f216(v1)?))
        }
        Adt3::C12(v2) => {
            Ok(f215(f215(f179(Str::lit("`")), v2), f179(Str::lit("`"))))
        }
    }
}

pub fn f179(v0: Str) -> Bytes {
    let a1281 = v0;
    utf8_encode(a1281)
}

pub fn f180(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: Bytes) -> R<Adt5> {
    let v5: u64 = f18(v0.clone(), v1, v3);
    let v6: Bytes = if f126(v4.clone(), f10(Str::lit("-"))) {
        f127(v4.clone(), 1u64)
    } else {
        v4.clone()
    };
    let v7: Adt12 = f136(v0, v1, {
        let a1282 = v3;
        let a1283 = 2u64;
        nat_add(a1282, a1283)?
    }, f10(Str::lit("an integer type")))?;
    let m1285 = match v7.clone() {
        Adt12::C0(_, _, h1284) => {
            h1284
        }
    };
    match m1285 {
        None => {
            let v10: Bytes = {
                let m1288 = f85(match v7.clone() {
                    Adt12::C0(h1287, _, _) => {
                        h1287
                    }
                });
                match m1288 {
                    None => {
                        f10(Str::lit(""))
                    }
                    Some(v9) => {
                        v9
                    }
                }
            };
            let m1289 = f217(v10.clone());
            match m1289 {
                None => {
                    Ok(f144(v2, v5, f131(f182(v10), f10(Str::lit(" is not an integer type")))))
                }
                Some(v11) => {
                    if if f72(v4.clone(), f10(Str::lit("-0"))) {
                        true
                    } else {
                        f120({
                            let a1291 = 1u64;
                            let a1292 = {
                                let a1290 = v6.clone();
                                length_bytes(a1290)
                            };
                            nat_lt(a1291, a1292)
                        }, f126(v6, f10(Str::lit("0"))))
                    } {
                        Ok(f144(v2, v5, f131(f182(v4), f10(Str::lit(" is not a canonical integer")))))
                    } else {
                        Ok(f114(v2, match v7 {
                            Adt12::C0(_, h1293, _) => {
                                h1293
                            }
                        }, f125(f10(Str::lit("integer")), f5(Str::lit("representation"), f6(v11), Str::lit("value"), f6(v4)))?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(f113(v2, match v7 {
                Adt12::C0(_, _, h1286) => {
                    h1286
                }
            }))
        }
    }
}

pub fn f181(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: u64, v5: Bytes) -> R<Adt5> {
    let v6: u64 = {
        let a1294 = v3;
        let a1295 = 1u64;
        nat_add(a1294, a1295)?
    };
    if if f72(v5.clone(), f10(Str::lit("not"))) {
        true
    } else {
        if f72(v5.clone(), f10(Str::lit("first"))) {
            true
        } else {
            f72(v5.clone(), f10(Str::lit("second")))
        }
    } {
        f149(v2, f14(f17(31u64, v4), v5, 0u64), v6)
    } else {
        if f72(v5.clone(), f10(Str::lit("cons"))) {
            f149(v2, f17(32u64, v4), v6)
        } else {
            if f72(v5.clone(), f10(Str::lit("nil"))) {
                f145(v2, f17(33u64, v4), v6)
            } else {
                if f72(v5.clone(), f10(Str::lit("if"))) {
                    f149(v2, f17(34u64, v4), v6)
                } else {
                    if f72(v5.clone(), f10(Str::lit("let"))) {
                        f191(v2, f17(35u64, v4), v6)
                    } else {
                        if f72(v5.clone(), f10(Str::lit("forall"))) {
                            let a1296 = f111(v0.clone(), v6);
                            let a1297 = 4u64;
                            if nat_eq(a1296, a1297) {
                                f191(v2, f17(36u64, v4), {
                                    let a1298 = v6;
                                    let a1299 = 1u64;
                                    nat_add(a1298, a1299)?
                                })
                            } else {
                                Ok(f109(v0, v1, v2, v6, f10(Str::lit("expected `(` before the bound variable"))))
                            }
                        } else {
                            if f72(v5.clone(), f10(Str::lit("fun"))) {
                                let a1300 = f111(v0.clone(), v6);
                                let a1301 = 2u64;
                                if nat_eq(a1300, a1301) {
                                    let v7: Adt12 = f218(v0.clone(), v1, v6, f10(Str::lit("a name")))?;
                                    let m1303 = match v7.clone() {
                                        Adt12::C0(_, _, h1302) => {
                                            h1302
                                        }
                                    };
                                    match m1303 {
                                        None => {
                                            f150(v0, v1, v2, f137(f17(37u64, v4), match v7.clone() {
                                                Adt12::C0(h1305, _, _) => {
                                                    h1305
                                                }
                                            }, Adt0::C5), match v7 {
                                                Adt12::C0(_, h1306, _) => {
                                                    h1306
                                                }
                                            })
                                        }
                                        Some(_) => {
                                            Ok(f113(v2, match v7 {
                                                Adt12::C0(_, _, h1304) => {
                                                    h1304
                                                }
                                            }))
                                        }
                                    }
                                } else {
                                    f150(v0, v1, v2, f137(f17(37u64, v4), f12(List::<Adt0>::nil()), Adt0::C5), v6)
                                }
                            } else {
                                if f72(v5.clone(), f10(Str::lit("apply"))) {
                                    f149(v2, f17(38u64, v4), v6)
                                } else {
                                    if f72(v5.clone(), f10(Str::lit("project"))) {
                                        f149(v2, f17(39u64, v4), v6)
                                    } else {
                                        if if f72(v5.clone(), f10(Str::lit("ctor"))) {
                                            true
                                        } else {
                                            if f72(v5.clone(), f10(Str::lit("ref"))) {
                                                true
                                            } else {
                                                if f72(v5.clone(), f10(Str::lit("record"))) {
                                                    true
                                                } else {
                                                    f72(v5.clone(), f10(Str::lit("checked")))
                                                }
                                            }
                                        } {
                                            let v9: Adt12 = f112(v0.clone(), v1, v6)?;
                                            let m1308 = match v9.clone() {
                                                Adt12::C0(_, _, h1307) => {
                                                    h1307
                                                }
                                            };
                                            match m1308 {
                                                None => {
                                                    f219(v0, v1, v2, f14(f137(f17(if f72(v5.clone(), f10(Str::lit("ctor"))) {
                                                        40u64
                                                    } else {
                                                        if f72(v5.clone(), f10(Str::lit("ref"))) {
                                                            41u64
                                                        } else {
                                                            if f72(v5.clone(), f10(Str::lit("record"))) {
                                                                42u64
                                                            } else {
                                                                44u64
                                                            }
                                                        }
                                                    }, v4), match v9.clone() {
                                                        Adt12::C0(h1310, _, _) => {
                                                            h1310
                                                        }
                                                    }, Adt0::C5), if f72(v5, f10(Str::lit("checked"))) {
                                                        f10(Str::lit("with"))
                                                    } else {
                                                        f10(Str::lit(""))
                                                    }, 0u64), match v9 {
                                                        Adt12::C0(_, h1311, _) => {
                                                            h1311
                                                        }
                                                    })
                                                }
                                                Some(_) => {
                                                    Ok(f113(v2, match v9 {
                                                        Adt12::C0(_, _, h1309) => {
                                                            h1309
                                                        }
                                                    }))
                                                }
                                            }
                                        } else {
                                            if f72(v5.clone(), f10(Str::lit("prim"))) {
                                                let v11: Adt12 = f136(v0.clone(), v1, v6, f10(Str::lit("a primitive operation")))?;
                                                let m1313 = match v11.clone() {
                                                    Adt12::C0(_, _, h1312) => {
                                                        h1312
                                                    }
                                                };
                                                match m1313 {
                                                    None => {
                                                        f154(v0, v1, v2, f137(f17(43u64, v4), match v11.clone() {
                                                            Adt12::C0(h1315, _, _) => {
                                                                h1315
                                                            }
                                                        }, Adt0::C5), match v11 {
                                                            Adt12::C0(_, h1316, _) => {
                                                                h1316
                                                            }
                                                        })
                                                    }
                                                    Some(_) => {
                                                        Ok(f113(v2, match v11 {
                                                            Adt12::C0(_, _, h1314) => {
                                                                h1314
                                                            }
                                                        }))
                                                    }
                                                }
                                            } else {
                                                if f72(v5, f10(Str::lit("match"))) {
                                                    f149(v2, f17(45u64, v4), v6)
                                                } else {
                                                    f220(v0, v1, v2, v3, v4)
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

pub fn f182(v0: Bytes) -> Bytes {
    f131(f131(f10(Str::lit("`")), v0), f10(Str::lit("`")))
}

pub fn f183(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let (h1317, _) = f81(Fn12::F221(v0.clone()), {
            let a1319 = {
                let a1318 = v0.clone();
                length_bytes(a1318)
            };
            let a1320 = 1u64;
            nat_add(a1319, a1320)?
        }, 0u64)?;
        h1317
    };
    Ok(f120({
        let a1322 = v1;
        let a1323 = {
            let a1321 = v0.clone();
            length_bytes(a1321)
        };
        nat_eq(a1322, a1323)
    }, {
        let a1328 = {
            let a1325 = {
                let a1324 = v0;
                length_bytes(a1324)
            };
            let a1326 = 2u64;
            let a1327 = 0u64;
            nat_rem(a1325, a1326, a1327)
        };
        let a1329 = 0u64;
        nat_eq(a1328, a1329)
    }))
}

pub fn f184(v0: Bytes) -> R<bool> {
    Ok(f120(f121(v0.clone())?, f120(!f129(v0.clone(), f185()), !f72(v0, f10(Str::lit("false"))))))
}

pub fn f185() -> List<Bytes> {
    List::cons(f10(Str::lit("let")), List::cons(f10(Str::lit("if")), List::cons(f10(Str::lit("match")), List::cons(f10(Str::lit("fun")), List::cons(f10(Str::lit("forall")), List::cons(f10(Str::lit("nil")), List::cons(f10(Str::lit("cons")), List::cons(f10(Str::lit("first")), List::cons(f10(Str::lit("second")), List::cons(f10(Str::lit("not")), List::cons(f10(Str::lit("apply")), List::cons(f10(Str::lit("ctor")), List::cons(f10(Str::lit("record")), List::cons(f10(Str::lit("project")), List::cons(f10(Str::lit("prim")), List::cons(f10(Str::lit("ref")), List::cons(f10(Str::lit("checked")), List::cons(f10(Str::lit("then")), List::cons(f10(Str::lit("else")), List::cons(f10(Str::lit("with")), List::cons(f10(Str::lit("true")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f186(v0: u64, v1: Bytes) -> Option<Adt1> {
    Some(Adt1::C0(v0, v1))
}

pub fn f187(v0: Fn18, p1334: u64, p1335: Adt13) -> R<(Adt13, bool)> {
    let m1333 = (p1334, p1335);
    {
        let mut m1333 = m1333;
        loop {
            let step = {
                let (v1, v2) = m1333;
                let m1330 = v1;
                if m1330 == 0 {
                    Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1330 - 1;
                    let m1332 = {
                        let c1331: Fn18 = v0.clone();
                        c1331.apply(v2.clone())?
                    };
                    match m1332 {
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
                Ok(next) => { m1333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: u64, v1: List<Adt2>, v2: Adt13) -> R<Option<Adt13>> {
    f222(v1, v0, v2)
}

pub fn f189(v0: Fn19, p1339: bool, p1340: List<(Bytes, Adt0)>) -> bool {
    let m1338 = (p1339, p1340);
    {
        let mut m1338 = m1338;
        loop {
            let step = {
                let (v1, v2) = m1338;
                let m1336 = v2;
                match m1336.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt0)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt0)>), bool>(({
                            let c1337: Fn19 = v0.clone();
                            c1337.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: bool, v2: (Bytes, Adt0)) -> bool {
    if v1 {
        true
    } else {
        let a1342 = {
            let (h1341, _) = v2;
            h1341
        };
        let a1343 = v0;
        equal(a1342, a1343)
    }
}

pub fn f191(v0: Adt5, v1: Adt6, v2: u64) -> R<Adt5> {
    Ok(f116(v0, v1.clone(), v2, 4u64, Adt7::C3, {
        let a1345 = match v1 {
            Adt6::C0(_, _, h1344, _, _, _, _, _, _, _, _, _) => {
                h1344
            }
        };
        let a1346 = 1u64;
        nat_add(a1345, a1346)?
    }))
}

pub fn f192(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let v5: u64 = match v3.clone() {
        Adt6::C0(h1347, _, _, _, _, _, _, _, _, _, _, _) => {
            h1347
        }
    };
    let v6: Adt0 = f12(match v3.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h1348, _, _, _, _) => {
            h1348
        }
    });
    let a1349 = v5;
    let a1350 = 38u64;
    if nat_eq(a1349, a1350) {
        Ok(f114(v2, v4, f125(f10(Str::lit("apply")), f5(Str::lit("function"), match v3 {
            Adt6::C0(_, _, _, _, _, _, _, _, _, h1351, _, _) => {
                h1351
            }
        }, Str::lit("arguments"), v6))?))
    } else {
        let a1352 = v5;
        let a1353 = 40u64;
        if nat_eq(a1352, a1353) {
            Ok(f114(v2, v4, f125(f10(Str::lit("constructor")), f193(f5(Str::lit("constructor"), match v3.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, h1354, _, _) => {
                    h1354
                }
            }, Str::lit("arguments"), v6), match v3 {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1355, _) => {
                    h1355
                }
            }))?))
        } else {
            let a1356 = v5;
            let a1357 = 46u64;
            if nat_eq(a1356, a1357) {
                Ok(f114(v2, v4, f125(f10(Str::lit("call")), f193(f5(Str::lit("function"), match v3.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, h1358, _, _) => {
                        h1358
                    }
                }, Str::lit("arguments"), v6), match v3 {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1359, _) => {
                        h1359
                    }
                }))?))
            } else {
                let v7: Adt12 = f218(v0, v1, {
                    let a1360 = v4;
                    let a1361 = 1u64;
                    nat_add(a1360, a1361)?
                }, f10(Str::lit("a check")))?;
                let m1363 = match v7.clone() {
                    Adt12::C0(_, _, h1362) => {
                        h1362
                    }
                };
                match m1363 {
                    None => {
                        Ok(f114(v2, match v7.clone() {
                            Adt12::C0(_, h1365, _) => {
                                h1365
                            }
                        }, f125(f10(Str::lit("checked_apply")), f193(List::cons((f10(Str::lit("checks")), match v7 {
                            Adt12::C0(h1366, _, _) => {
                                h1366
                            }
                        }), f5(Str::lit("model"), match v3.clone() {
                            Adt6::C0(_, _, _, _, _, _, _, _, _, h1367, _, _) => {
                                h1367
                            }
                        }, Str::lit("arguments"), v6)), match v3 {
                            Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1368, _) => {
                                h1368
                            }
                        }))?))
                    }
                    Some(_) => {
                        Ok(f113(v2, match v7 {
                            Adt12::C0(_, _, h1364) => {
                                h1364
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f193(v0: List<(Bytes, Adt0)>, v1: Adt0) -> List<(Bytes, Adt0)> {
    let m1369 = f223(v1.clone());
    match m1369 {
        None => {
            v0
        }
        Some(_) => {
            List::cons((f10(Str::lit("type_arguments")), v1), v0)
        }
    }
}

pub fn f194(v0: Fn20, p1374: u64, p1375: Adt15) -> R<(Adt15, bool)> {
    let m1373 = (p1374, p1375);
    {
        let mut m1373 = m1373;
        loop {
            let step = {
                let (v1, v2) = m1373;
                let m1370 = v1;
                if m1370 == 0 {
                    Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1370 - 1;
                    let m1372 = {
                        let c1371: Fn20 = v0.clone();
                        c1371.apply(v2.clone())?
                    };
                    match m1372 {
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
                Ok(next) => { m1373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: u64, v1: List<Adt2>, v2: Adt15) -> R<Option<Adt15>> {
    let m1377 = match v2.clone() {
        Adt15::C0(_, _, _, h1376) => {
            h1376
        }
    };
    match m1377 {
        None => {
            if f148(v1.clone(), match v2.clone() {
                Adt15::C0(_, h1378, _, _) => {
                    h1378
                }
            }, f10(Str::lit("=>"))) {
                Ok(None::<Adt15>)
            } else {
                let v4: Adt12 = f136(v1, v0, match v2.clone() {
                    Adt15::C0(_, h1379, _, _) => {
                        h1379
                    }
                }, f10(Str::lit("a binder name")))?;
                let m1381 = match v4.clone() {
                    Adt12::C0(_, _, h1380) => {
                        h1380
                    }
                };
                match m1381 {
                    None => {
                        Ok(Some(Adt15::C0(List::cons(match v4.clone() {
                            Adt12::C0(h1385, _, _) => {
                                h1385
                            }
                        }, match v2 {
                            Adt15::C0(h1386, _, _, _) => {
                                h1386
                            }
                        }), match v4 {
                            Adt12::C0(_, h1387, _) => {
                                h1387
                            }
                        }, true, None::<Adt1>)))
                    }
                    Some(_) => {
                        Ok(Some(Adt15::C0(match v2.clone() {
                            Adt15::C0(h1382, _, _, _) => {
                                h1382
                            }
                        }, match v2 {
                            Adt15::C0(_, h1383, _, _) => {
                                h1383
                            }
                        }, false, match v4 {
                            Adt12::C0(_, _, h1384) => {
                                h1384
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

pub fn f196(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a1392 = {
            let a1390 = {
                let a1388 = v0.clone();
                length_list(a1388)
            };
            let a1391 = {
                let a1389 = v1.clone();
                length_list(a1389)
            };
            nat_add(a1390, a1391)?
        };
        let a1393 = 1u64;
        nat_add(a1392, a1393)?
    };
    Ok(f33({
        let (_, h1394) = {
            let (_, h1395) = {
                let (h1396, _) = f224(Fn24::F225, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h1396
            };
            h1395
        };
        h1394
    }))
}

pub fn f197(v0: u64) -> Bytes {
    f66(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f198(v0: Bytes, v1: Adt9) -> R<Adt9> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h1397, _, _, _) => {
            h1397
        }
    };
    let m1400 = f226(v0, {
        let a1398 = v2;
        let a1399 = 2u64;
        nat_add(a1398, a1399)?
    })?;
    match m1400 {
        None => {
            Ok(f160(v1, v2, f64(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a1401 = 55296u64;
            let a1402 = v3;
            if nat_le(a1401, a1402) {
                let a1403 = v3;
                let a1404 = 57343u64;
                if nat_le(a1403, a1404) {
                    Ok(f160(v1, v2, f199(f199(f64(Str::lit("`\\u")), f227(v3)), f64(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f162(v1, {
                        let a1405 = v2;
                        let a1406 = 6u64;
                        nat_add(a1405, a1406)?
                    }, f228(v3)?))
                }
            } else {
                Ok(f162(v1, {
                    let a1407 = v2;
                    let a1408 = 6u64;
                    nat_add(a1407, a1408)?
                }, f228(v3)?))
            }
        }
    }
}

pub fn f199(v0: Bytes, v1: Bytes) -> Bytes {
    let a1409 = v0;
    let a1410 = v1;
    append_bytes(a1409, a1410)
}

pub fn f200(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f159(v0, v1)?;
    let a1411 = v2;
    let a1412 = 128u64;
    if nat_lt(a1411, a1412) {
        Ok(1u64)
    } else {
        let a1413 = v2;
        let a1414 = 224u64;
        if nat_lt(a1413, a1414) {
            Ok(2u64)
        } else {
            let a1415 = v2;
            let a1416 = 240u64;
            if nat_lt(a1415, a1416) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f201(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f229(v1, v0, v2)
}

pub fn f202(v0: Fn21, p1420: (Option<Bytes>, List<Bytes>), p1421: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m1419 = (p1420, p1421);
    {
        let mut m1419 = m1419;
        loop {
            let step = {
                let (v1, v2) = m1419;
                let m1417 = v2;
                match m1417.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c1418: Fn21 = v0.clone();
                            c1418.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f230(v0, v1)
}

pub fn f204(v0: Bytes, v1: u64) -> bool {
    if f40(v0.clone(), v1, 123u8) {
        true
    } else {
        if f40(v0.clone(), v1, 125u8) {
            true
        } else {
            if f40(v0.clone(), v1, 91u8) {
                true
            } else {
                if f40(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f40(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f40(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f40(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f40(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f40(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f40(v0, v1, 34u8)
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

pub fn f205(v0: Adt3) -> u64 {
    let m1422 = v0;
    match m1422 {
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

pub fn f206(v0: Adt3) -> Bytes {
    let m1423 = v0;
    match m1423 {
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

pub fn f207(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f231(v0, v1)
}

pub fn f208(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f232(v0, v1)
}

pub fn f209(v0: Option<Adt0>) -> Option<Adt0> {
    let m1424 = v0;
    match m1424 {
        None => {
            None::<Adt0>
        }
        Some(v1) => {
            f223(v1)
        }
    }
}

pub fn f210(v0: Fn22, p1428: Option<Adt0>, p1429: List<Adt0>) -> R<Option<Adt0>> {
    let m1427 = (p1428, p1429);
    {
        let mut m1427 = m1427;
        loop {
            let step = {
                let (v1, v2) = m1427;
                let m1425 = v2;
                match m1425.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(({
                            let c1426: Fn22 = v0.clone();
                            c1426.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt0) -> R<Option<Adt0>> {
    let m1430 = v2.clone();
    match m1430 {
        None => {
            let v5: Adt0 = f48(v1, v3)?;
            if f233(v5.clone(), v0)? {
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

pub fn f212(v0: Adt0) -> List<Adt0> {
    f234({
        let m1433 = {
            let (h1432, _) = f235(Fn25::F236, 1000000u64, Adt16::C0(v0, List::<Adt0>::nil()));
            h1432
        };
        match m1433 {
            Adt16::C0(_, h1431) => {
                h1431
            }
        }
    })
}

pub fn f213(v0: Fn23, p1437: bool, p1438: List<Adt0>) -> R<bool> {
    let m1436 = (p1437, p1438);
    {
        let mut m1436 = m1436;
        loop {
            let step = {
                let (v1, v2) = m1436;
                let m1434 = v2;
                match m1434.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), R<bool>>(({
                            let c1435: Fn23 = v0.clone();
                            c1435.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: Adt0, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let m1439 = f24(f48(v0, v2)?, f10(Str::lit("properties")));
        match m1439 {
            None => {
                Ok(false)
            }
            Some(v3) => {
                Ok(f237(f24(v3, f10(Str::lit("kind")))))
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f215(v0: Bytes, v1: Bytes) -> Bytes {
    let a1440 = v0;
    let a1441 = v1;
    append_bytes(a1440, a1441)
}

pub fn f216(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a1442 = v0.clone();
        length_bytes(a1442)
    };
    let v2: Adt17 = {
        let (h1443, _) = f238(Fn26::F239(v1, v0), {
            let a1444 = v1;
            let a1445 = 1u64;
            nat_add(a1444, a1445)?
        }, Adt17::C0(0u64, List::<Bytes>::nil()))?;
        h1443
    };
    Ok(f199(f199(f64(Str::lit("\"")), f62(match v2 {
        Adt17::C0(_, h1446) => {
            h1446
        }
    })), f64(Str::lit("\""))))
}

pub fn f217(v0: Bytes) -> Option<Bytes> {
    if f72(v0.clone(), f10(Str::lit("Int"))) {
        Some(f10(Str::lit("int")))
    } else {
        if f72(v0.clone(), f10(Str::lit("Int8"))) {
            Some(f10(Str::lit("int8")))
        } else {
            if f72(v0.clone(), f10(Str::lit("Int16"))) {
                Some(f10(Str::lit("int16")))
            } else {
                if f72(v0.clone(), f10(Str::lit("Int32"))) {
                    Some(f10(Str::lit("int32")))
                } else {
                    if f72(v0.clone(), f10(Str::lit("Int64"))) {
                        Some(f10(Str::lit("int64")))
                    } else {
                        if f72(v0.clone(), f10(Str::lit("UInt8"))) {
                            Some(f10(Str::lit("uint8")))
                        } else {
                            if f72(v0.clone(), f10(Str::lit("UInt16"))) {
                                Some(f10(Str::lit("uint16")))
                            } else {
                                if f72(v0.clone(), f10(Str::lit("UInt32"))) {
                                    Some(f10(Str::lit("uint32")))
                                } else {
                                    if f72(v0, f10(Str::lit("UInt64"))) {
                                        Some(f10(Str::lit("uint64")))
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

pub fn f218(v0: List<Adt2>, v1: u64, v2: u64, v3: Bytes) -> R<Adt12> {
    let a1447 = f111(v0.clone(), v2);
    let a1448 = 2u64;
    if nat_eq(a1447, a1448) {
        let v4: Adt15 = {
            let (h1449, _) = f194(Fn20::F240(v1, v0.clone(), v3), {
                let a1451 = {
                    let a1450 = v0.clone();
                    length_list(a1450)
                };
                let a1452 = 2u64;
                nat_add(a1451, a1452)?
            }, Adt15::C0(List::<Adt0>::nil(), {
                let a1453 = v2;
                let a1454 = 1u64;
                nat_add(a1453, a1454)?
            }, true, None::<Adt1>))?;
            h1449
        };
        let m1456 = match v4.clone() {
            Adt15::C0(_, _, _, h1455) => {
                h1455
            }
        };
        match m1456 {
            None => {
                let a1460 = f111(v0.clone(), match v4.clone() {
                    Adt15::C0(_, h1459, _, _) => {
                        h1459
                    }
                });
                let a1461 = 3u64;
                if nat_eq(a1460, a1461) {
                    Ok(Adt12::C0(f12(match v4.clone() {
                        Adt15::C0(h1462, _, _, _) => {
                            h1462
                        }
                    }), {
                        let a1464 = match v4 {
                            Adt15::C0(_, h1463, _, _) => {
                                h1463
                            }
                        };
                        let a1465 = 1u64;
                        nat_add(a1464, a1465)?
                    }, None::<Adt1>))
                } else {
                    Ok(Adt12::C0(Adt0::C5, match v4.clone() {
                        Adt15::C0(_, h1466, _, _) => {
                            h1466
                        }
                    }, f186(f18(v0, v1, match v4 {
                        Adt15::C0(_, h1467, _, _) => {
                            h1467
                        }
                    }), f10(Str::lit("expected `,` or `]`")))))
                }
            }
            Some(_) => {
                Ok(Adt12::C0(Adt0::C5, match v4.clone() {
                    Adt15::C0(_, h1457, _, _) => {
                        h1457
                    }
                }, match v4 {
                    Adt15::C0(_, _, _, h1458) => {
                        h1458
                    }
                }))
            }
        }
    } else {
        Ok(Adt12::C0(Adt0::C5, v2, f186(f18(v0, v1, v2), f10(Str::lit("expected `[`")))))
    }
}

pub fn f219(v0: List<Adt2>, v1: u64, v2: Adt5, v3: Adt6, v4: u64) -> R<Adt5> {
    let a1468 = f111(v0.clone(), v4);
    let a1469 = 2u64;
    if nat_eq(a1468, a1469) {
        Ok(f116(v2, v3.clone(), v4, 6u64, Adt7::C5, match v3 {
            Adt6::C0(_, _, h1470, _, _, _, _, _, _, _, _, _) => {
                h1470
            }
        }))
    } else {
        f152(v0, v1, Adt5::C0(v4, match v2.clone() {
            Adt5::C0(_, h1471, _, _, _, _, _, _, _) => {
                h1471
            }
        }, Adt0::C5, 1u64, match v2.clone() {
            Adt5::C0(_, _, _, _, h1472, _, _, _, _) => {
                h1472
            }
        }, match v2.clone() {
            Adt5::C0(_, _, _, _, _, h1473, _, _, _) => {
                h1473
            }
        }, match v2.clone() {
            Adt5::C0(_, _, _, _, _, _, h1474, _, _) => {
                h1474
            }
        }, None::<Adt1>, match v2 {
            Adt5::C0(_, _, _, _, _, _, _, _, h1475) => {
                h1475
            }
        }), v3)
    }
}

pub fn f220(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: u64) -> R<Adt5> {
    let v5: Bytes = f119(v0.clone(), v3);
    let v6: u64 = {
        let a1478 = f111(v0.clone(), {
            let a1476 = v3;
            let a1477 = 1u64;
            nat_add(a1476, a1477)?
        });
        let a1479 = 9u64;
        if nat_eq(a1478, a1479) {
            3u64
        } else {
            1u64
        }
    };
    let v7: bool = {
        let a1482 = f111(v0.clone(), {
            let a1480 = v3;
            let a1481 = v6;
            nat_add(a1480, a1481)?
        });
        let a1483 = 6u64;
        if nat_eq(a1482, a1483) {
            true
        } else {
            f120({
                let a1486 = f111(v0.clone(), {
                    let a1484 = v3;
                    let a1485 = v6;
                    nat_add(a1484, a1485)?
                });
                let a1487 = 12u64;
                nat_eq(a1486, a1487)
            }, f241(f119(v0.clone(), {
                let a1488 = v3;
                let a1489 = v6;
                nat_add(a1488, a1489)?
            })))
        }
    };
    let v8: bool = {
        let a1492 = f111(v0.clone(), {
            let a1490 = v3;
            let a1491 = 1u64;
            nat_add(a1490, a1491)?
        });
        let a1493 = 0u64;
        nat_eq(a1492, a1493)
    };
    if f120({
        let a1494 = f111(v0.clone(), v3);
        let a1495 = 12u64;
        nat_eq(a1494, a1495)
    }, f120(f184(v5.clone())?, f120(!v7, f120(!v8, !f122(v5)?)))) {
        let v9: Adt12 = f112(v0.clone(), v1, v3)?;
        let m1497 = match v9.clone() {
            Adt12::C0(_, _, h1496) => {
                h1496
            }
        };
        match m1497 {
            None => {
                f219(v0, v1, v2, f137(f17(46u64, v4), match v9.clone() {
                    Adt12::C0(h1499, _, _) => {
                        h1499
                    }
                }, Adt0::C5), match v9 {
                    Adt12::C0(_, h1500, _) => {
                        h1500
                    }
                })
            }
            Some(_) => {
                Ok(f113(v2, match v9 {
                    Adt12::C0(_, _, h1498) => {
                        h1498
                    }
                }))
            }
        }
    } else {
        f149(v2, f17(47u64, v4), v3)
    }
}

pub fn f221(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1501 = f60(v0, v1);
    match m1501 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1546 = {
                    let a1522 = {
                        let a1510 = {
                            let a1504 = {
                                let a1502 = v3;
                                let a1503 = 128u8;
                                fixed_u8::bit_and(a1502, a1503)
                            };
                            let a1505 = 128u8;
                            if equal(a1504, a1505) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1511 = {
                            let a1508 = {
                                let a1506 = v3;
                                let a1507 = 64u8;
                                fixed_u8::bit_and(a1506, a1507)
                            };
                            let a1509 = 64u8;
                            if equal(a1508, a1509) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1510, a1511)?
                    };
                    let a1523 = {
                        let a1520 = {
                            let a1514 = {
                                let a1512 = v3;
                                let a1513 = 32u8;
                                fixed_u8::bit_and(a1512, a1513)
                            };
                            let a1515 = 32u8;
                            if equal(a1514, a1515) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1521 = {
                            let a1518 = {
                                let a1516 = v3;
                                let a1517 = 16u8;
                                fixed_u8::bit_and(a1516, a1517)
                            };
                            let a1519 = 16u8;
                            if equal(a1518, a1519) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1520, a1521)?
                    };
                    nat_add(a1522, a1523)?
                };
                let a1547 = {
                    let a1544 = {
                        let a1532 = {
                            let a1526 = {
                                let a1524 = v3;
                                let a1525 = 8u8;
                                fixed_u8::bit_and(a1524, a1525)
                            };
                            let a1527 = 8u8;
                            if equal(a1526, a1527) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1533 = {
                            let a1530 = {
                                let a1528 = v3;
                                let a1529 = 4u8;
                                fixed_u8::bit_and(a1528, a1529)
                            };
                            let a1531 = 4u8;
                            if equal(a1530, a1531) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1532, a1533)?
                    };
                    let a1545 = {
                        let a1542 = {
                            let a1536 = {
                                let a1534 = v3;
                                let a1535 = 2u8;
                                fixed_u8::bit_and(a1534, a1535)
                            };
                            let a1537 = 2u8;
                            if equal(a1536, a1537) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1543 = {
                            let a1540 = {
                                let a1538 = v3;
                                let a1539 = 1u8;
                                fixed_u8::bit_and(a1538, a1539)
                            };
                            let a1541 = 1u8;
                            if equal(a1540, a1541) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1542, a1543)?
                    };
                    nat_add(a1544, a1545)?
                };
                nat_add(a1546, a1547)?
            };
            if if f120({
                let a1548 = 48u64;
                let a1549 = v4;
                nat_le(a1548, a1549)
            }, {
                let a1550 = v4;
                let a1551 = 57u64;
                nat_le(a1550, a1551)
            }) {
                true
            } else {
                f120({
                    let a1552 = 97u64;
                    let a1553 = v4;
                    nat_le(a1552, a1553)
                }, {
                    let a1554 = v4;
                    let a1555 = 102u64;
                    nat_le(a1554, a1555)
                })
            } {
                Ok(Some({
                    let a1556 = v1;
                    let a1557 = 1u64;
                    nat_add(a1556, a1557)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f222(v0: List<Adt2>, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    let m1559 = match v2.clone() {
        Adt13::C0(_, _, _, _, _, h1558) => {
            h1558
        }
    };
    match m1559 {
        None => {
            let a1561 = match v2.clone() {
                Adt13::C0(_, _, h1560, _, _, _) => {
                    h1560
                }
            };
            let a1562 = 9u64;
            if nat_eq(a1561, a1562) {
                Ok(None::<Adt13>)
            } else {
                Ok(Some(f242(v0, v1, v2)?))
            }
        }
        Some(_) => {
            Ok(None::<Adt13>)
        }
    }
}

pub fn f223(v0: Adt0) -> Option<Adt0> {
    let m1563 = v0;
    match m1563 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(r1564) => {
            let v1 = (*r1564).clone();
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

pub fn f224(v0: Fn24, p1569: u64, p1570: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m1568 = (p1569, p1570);
    {
        let mut m1568 = m1568;
        loop {
            let step = {
                let (v1, v2) = m1568;
                let m1565 = v1;
                if m1565 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m1565 - 1;
                    let m1567 = {
                        let c1566: Fn24 = v0.clone();
                        c1566.apply(v2.clone())
                    };
                    match m1567 {
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
                Ok(next) => { m1568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f243(v0)
}

pub fn f226(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1571 = f244(v0.clone(), v1)?;
    match m1571 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1574 = f244(v0.clone(), {
                let a1572 = v1;
                let a1573 = 1u64;
                nat_add(a1572, a1573)?
            })?;
            match m1574 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m1577 = f244(v0.clone(), {
                        let a1575 = v1;
                        let a1576 = 2u64;
                        nat_add(a1575, a1576)?
                    })?;
                    match m1577 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m1580 = f244(v0, {
                                let a1578 = v1;
                                let a1579 = 3u64;
                                nat_add(a1578, a1579)?
                            })?;
                            match m1580 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a1591 = {
                                            let a1589 = {
                                                let a1587 = {
                                                    let a1585 = {
                                                        let a1583 = {
                                                            let a1581 = v2;
                                                            let a1582 = 16u64;
                                                            nat_mul(a1581, a1582)?
                                                        };
                                                        let a1584 = v3;
                                                        nat_add(a1583, a1584)?
                                                    };
                                                    let a1586 = 16u64;
                                                    nat_mul(a1585, a1586)?
                                                };
                                                let a1588 = v4;
                                                nat_add(a1587, a1588)?
                                            };
                                            let a1590 = 16u64;
                                            nat_mul(a1589, a1590)?
                                        };
                                        let a1592 = v5;
                                        nat_add(a1591, a1592)?
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

pub fn f227(v0: u64) -> Bytes {
    f199(f199(f245(v0, 4096u64), f245(v0, 256u64)), f199(f245(v0, 16u64), f246(v0)))
}

pub fn f228(v0: u64) -> R<Bytes> {
    let a1593 = v0;
    let a1594 = 128u64;
    if nat_lt(a1593, a1594) {
        Ok(f197(v0))
    } else {
        let a1595 = v0;
        let a1596 = 2048u64;
        if nat_lt(a1595, a1596) {
            Ok(f199(f197({
                let a1600 = 192u64;
                let a1601 = {
                    let a1597 = v0;
                    let a1598 = 64u64;
                    let a1599 = 0u64;
                    nat_quot(a1597, a1598, a1599)
                };
                nat_add(a1600, a1601)?
            }), f197({
                let a1605 = 128u64;
                let a1606 = {
                    let a1602 = v0;
                    let a1603 = 64u64;
                    let a1604 = 0u64;
                    nat_rem(a1602, a1603, a1604)
                };
                nat_add(a1605, a1606)?
            })))
        } else {
            Ok(f199(f197({
                let a1610 = 224u64;
                let a1611 = {
                    let a1607 = v0;
                    let a1608 = 4096u64;
                    let a1609 = 0u64;
                    nat_quot(a1607, a1608, a1609)
                };
                nat_add(a1610, a1611)?
            }), f199(f197({
                let a1618 = 128u64;
                let a1619 = {
                    let a1615 = {
                        let a1612 = v0;
                        let a1613 = 64u64;
                        let a1614 = 0u64;
                        nat_quot(a1612, a1613, a1614)
                    };
                    let a1616 = 64u64;
                    let a1617 = 0u64;
                    nat_rem(a1615, a1616, a1617)
                };
                nat_add(a1618, a1619)?
            }), f197({
                let a1623 = 128u64;
                let a1624 = {
                    let a1620 = v0;
                    let a1621 = 64u64;
                    let a1622 = 0u64;
                    nat_rem(a1620, a1621, a1622)
                };
                nat_add(a1623, a1624)?
            }))))
        }
    }
}

pub fn f229(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1625 = v2;
    let a1626 = v1;
    if nat_lt(a1625, a1626) {
        let v3: u64 = f159(v0, v2)?;
        let a1627 = v3;
        let a1628 = 32u64;
        if nat_lt(a1627, a1628) {
            Ok(None::<u64>)
        } else {
            let a1629 = v3;
            let a1630 = 34u64;
            if nat_eq(a1629, a1630) {
                Ok(None::<u64>)
            } else {
                let a1631 = v3;
                let a1632 = 92u64;
                if nat_eq(a1631, a1632) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1633 = v2;
                        let a1634 = 1u64;
                        nat_add(a1633, a1634)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f230(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m1636 = {
        let (h1635, _) = v0;
        h1635
    };
    match m1636 {
        None => {
            (Some(v1), {
                let (_, h1637) = v0;
                h1637
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a1638 = v2;
                let a1639 = v1;
                append_bytes(a1638, a1639)
            }, {
                let (_, h1640) = v0;
                h1640
            }))
        }
    }
}

pub fn f231(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f247(f169(v0, v1)?) {
        Ok(Some({
            let a1641 = v1;
            let a1642 = 1u64;
            nat_add(a1641, a1642)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f232(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f169(v0, v1)?;
    let a1643 = 47u64;
    let a1644 = v2;
    if nat_lt(a1643, a1644) {
        let a1645 = v2;
        let a1646 = 58u64;
        if nat_lt(a1645, a1646) {
            Ok(Some({
                let a1647 = v1;
                let a1648 = 1u64;
                nat_add(a1647, a1648)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f233(v0: Adt0, v1: Bytes) -> R<bool> {
    let m1649 = f24(v0, f10(Str::lit("properties")));
    match m1649 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m1650 = f24(v2, f10(Str::lit("kind")));
            match m1650 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m1651 = f70(f24(v3.clone(), f10(Str::lit("const"))));
                    if match m1651 {
                        None => {
                            false
                        }
                        Some(v4) => {
                            f72(v4, v1.clone())
                        }
                    } {
                        Ok(true)
                    } else {
                        let m1652 = f209(f24(v3, f10(Str::lit("enum"))));
                        match m1652 {
                            None => {
                                Ok(false)
                            }
                            Some(v5) => {
                                f213(Fn23::F248(v1), false, f212(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f234(v0: List<Adt0>) -> List<Adt0> {
    f249(Fn27::F250, List::<Adt0>::nil(), v0)
}

pub fn f235(v0: Fn25, p1657: u64, p1658: Adt16) -> (Adt16, bool) {
    let m1656 = (p1657, p1658);
    {
        let mut m1656 = m1656;
        loop {
            let step = {
                let (v1, v2) = m1656;
                let m1653 = v1;
                if m1653 == 0 {
                    Err::<(u64, Adt16), (Adt16, bool)>((v2, false))
                } else {
                    let v3 = m1653 - 1;
                    let m1655 = {
                        let c1654: Fn25 = v0.clone();
                        c1654.apply(v2.clone())
                    };
                    match m1655 {
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
                Ok(next) => { m1656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Adt16) -> Option<Adt16> {
    f251(v0)
}

pub fn f237(v0: Option<Adt0>) -> bool {
    let m1659 = v0;
    match m1659 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f238(v0: Fn26, p1664: u64, p1665: Adt17) -> R<(Adt17, bool)> {
    let m1663 = (p1664, p1665);
    {
        let mut m1663 = m1663;
        loop {
            let step = {
                let (v1, v2) = m1663;
                let m1660 = v1;
                if m1660 == 0 {
                    Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1660 - 1;
                    let m1662 = {
                        let c1661: Fn26 = v0.clone();
                        c1661.apply(v2.clone())?
                    };
                    match m1662 {
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
                Ok(next) => { m1663 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: u64, v1: Bytes, v2: Adt17) -> R<Option<Adt17>> {
    f252(v1, v0, v2)
}

pub fn f240(v0: u64, v1: List<Adt2>, v2: Bytes, v3: Adt15) -> R<Option<Adt15>> {
    f253(v1, v0, v2, v3)
}

pub fn f241(v0: Bytes) -> bool {
    let m1666 = f156(v0);
    match m1666 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f242(v0: List<Adt2>, v1: u64, v2: Adt13) -> R<Adt13> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(_, _, h1667, _, _, _) => {
            h1667
        }
    };
    let a1668 = v3;
    let a1669 = 0u64;
    if nat_eq(a1668, a1669) {
        f254(v0, v1, v2)
    } else {
        let a1670 = v3;
        let a1671 = 1u64;
        if nat_eq(a1670, a1671) {
            f255(v0, v2)
        } else {
            let a1672 = v3;
            let a1673 = 2u64;
            if nat_eq(a1672, a1673) {
                f256(v0, v1, v2)
            } else {
                let a1674 = v3;
                let a1675 = 3u64;
                if nat_eq(a1674, a1675) {
                    f257(v0, v2)
                } else {
                    let a1676 = v3;
                    let a1677 = 4u64;
                    if nat_eq(a1676, a1677) {
                        f258(v0, v1, v2, 3u64, f179(Str::lit("expected `,` or `]`")))
                    } else {
                        f258(v0, v1, v2, 1u64, f179(Str::lit("expected `,` or `}`")))
                    }
                }
            }
        }
    }
}

pub fn f243(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h1678, _) = v0.clone();
        h1678
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h1679, _) = {
            let (_, h1680) = v0.clone();
            h1680
        };
        h1679
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h1681) = {
            let (_, h1682) = v0;
            h1682
        };
        h1681
    };
    let m1683 = v1.clone();
    match m1683.uncons() {
        None => {
            let m1684 = v2;
            match m1684.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1685 = v2.clone();
            match m1685.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f259({
                        let (h1686, _) = v8.clone();
                        h1686
                    }, {
                        let (h1687, _) = v6.clone();
                        h1687
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

pub fn f244(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1688 = f60(v0, v1);
    match m1688 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f260(v2)
        }
    }
}

pub fn f245(v0: u64, v1: u64) -> Bytes {
    f246({
        let a1689 = v0;
        let a1690 = v1;
        let a1691 = 0u64;
        nat_quot(a1689, a1690, a1691)
    })
}

pub fn f246(v0: u64) -> Bytes {
    f66(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1692 = v0;
        let a1693 = 16u64;
        let a1694 = 0u64;
        nat_rem(a1692, a1693, a1694)
    }, 1u64)
}

pub fn f247(v0: u64) -> bool {
    if f168(v0) {
        true
    } else {
        let a1695 = 47u64;
        let a1696 = v0;
        if if nat_lt(a1695, a1696) {
            let a1697 = v0;
            let a1698 = 58u64;
            nat_lt(a1697, a1698)
        } else {
            false
        } {
            true
        } else {
            let a1699 = v0;
            let a1700 = 46u64;
            if nat_eq(a1699, a1700) {
                true
            } else {
                let a1701 = v0;
                let a1702 = 39u64;
                if nat_eq(a1701, a1702) {
                    true
                } else {
                    let a1703 = v0;
                    let a1704 = 47u64;
                    if nat_eq(a1703, a1704) {
                        true
                    } else {
                        let a1705 = v0;
                        let a1706 = 45u64;
                        nat_eq(a1705, a1706)
                    }
                }
            }
        }
    }
}

pub fn f248(v0: Bytes, v1: bool, v2: Adt0) -> bool {
    if v1 {
        true
    } else {
        let m1707 = f85(v2);
        match m1707 {
            None => {
                false
            }
            Some(v3) => {
                f72(v3, v0)
            }
        }
    }
}

pub fn f249(v0: Fn27, p1711: List<Adt0>, p1712: List<Adt0>) -> List<Adt0> {
    let m1710 = (p1711, p1712);
    {
        let mut m1710 = m1710;
        loop {
            let step = {
                let (v1, v2) = m1710;
                let m1708 = v2;
                match m1708.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c1709: Fn27 = v0.clone();
                            c1709.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f251(v0: Adt16) -> Option<Adt16> {
    let m1714 = f261(match v0.clone() {
        Adt16::C0(h1713, _) => {
            h1713
        }
    });
    match m1714 {
        None => {
            None::<Adt16>
        }
        Some(v1) => {
            Some(Adt16::C0(match v1.clone() {
                Adt18::C0(_, h1715) => {
                    h1715
                }
            }, List::cons(match v1 {
                Adt18::C0(h1716, _) => {
                    h1716
                }
            }, match v0 {
                Adt16::C0(_, h1717) => {
                    h1717
                }
            })))
        }
    }
}

pub fn f252(v0: Bytes, v1: u64, v2: Adt17) -> R<Option<Adt17>> {
    let v3: u64 = match v2.clone() {
        Adt17::C0(h1718, _) => {
            h1718
        }
    };
    let a1719 = v3;
    let a1720 = v1;
    if nat_lt(a1719, a1720) {
        let v4: u64 = f159(v0.clone(), v3)?;
        let a1721 = v4;
        let a1722 = 32u64;
        if if nat_lt(a1721, a1722) {
            true
        } else {
            let a1723 = v4;
            let a1724 = 34u64;
            if nat_eq(a1723, a1724) {
                true
            } else {
                let a1725 = v4;
                let a1726 = 92u64;
                nat_eq(a1725, a1726)
            }
        } {
            Ok(Some(Adt17::C0({
                let a1727 = v3;
                let a1728 = 1u64;
                nat_add(a1727, a1728)?
            }, List::cons(f262(v4), match v2 {
                Adt17::C0(_, h1729) => {
                    h1729
                }
            }))))
        } else {
            let v5: u64 = f161(v0.clone(), v1, v3)?;
            Ok(Some(Adt17::C0(v5, List::cons(f66(v0, v3, {
                let a1730 = v5;
                let a1731 = v3;
                nat_sub(a1730, a1731)
            }), match v2 {
                Adt17::C0(_, h1732) => {
                    h1732
                }
            }))))
        }
    } else {
        Ok(None::<Adt17>)
    }
}

pub fn f253(v0: List<Adt2>, v1: u64, v2: Bytes, v3: Adt15) -> R<Option<Adt15>> {
    let m1734 = match v3.clone() {
        Adt15::C0(_, _, _, h1733) => {
            h1733
        }
    };
    match m1734 {
        None => {
            if match v3.clone() {
                Adt15::C0(_, _, h1735, _) => {
                    h1735
                }
            } {
                let v5: u64 = match v3.clone() {
                    Adt15::C0(_, h1736, _, _) => {
                        h1736
                    }
                };
                let a1737 = f111(v0.clone(), v5);
                let a1738 = 3u64;
                if nat_eq(a1737, a1738) {
                    Ok(None::<Adt15>)
                } else {
                    let v6: Adt12 = f136(v0.clone(), v1, v5, v2)?;
                    let m1740 = match v6.clone() {
                        Adt12::C0(_, _, h1739) => {
                            h1739
                        }
                    };
                    match m1740 {
                        None => {
                            let a1744 = f111(v0, match v6.clone() {
                                Adt12::C0(_, h1743, _) => {
                                    h1743
                                }
                            });
                            let a1745 = 6u64;
                            if nat_eq(a1744, a1745) {
                                Ok(Some(Adt15::C0(List::cons(match v6.clone() {
                                    Adt12::C0(h1746, _, _) => {
                                        h1746
                                    }
                                }, match v3 {
                                    Adt15::C0(h1747, _, _, _) => {
                                        h1747
                                    }
                                }), {
                                    let a1749 = match v6 {
                                        Adt12::C0(_, h1748, _) => {
                                            h1748
                                        }
                                    };
                                    let a1750 = 1u64;
                                    nat_add(a1749, a1750)?
                                }, true, None::<Adt1>)))
                            } else {
                                Ok(Some(Adt15::C0(List::cons(match v6.clone() {
                                    Adt12::C0(h1751, _, _) => {
                                        h1751
                                    }
                                }, match v3 {
                                    Adt15::C0(h1752, _, _, _) => {
                                        h1752
                                    }
                                }), match v6 {
                                    Adt12::C0(_, h1753, _) => {
                                        h1753
                                    }
                                }, false, None::<Adt1>)))
                            }
                        }
                        Some(_) => {
                            Ok(Some(Adt15::C0(match v3 {
                                Adt15::C0(h1741, _, _, _) => {
                                    h1741
                                }
                            }, v5, false, match v6 {
                                Adt12::C0(_, _, h1742) => {
                                    h1742
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

pub fn f254(v0: List<Adt2>, v1: u64, v2: Adt13) -> R<Adt13> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(h1754, _, _, _, _, _) => {
            h1754
        }
    };
    let a1756 = 256u64;
    let a1757 = match v2.clone() {
        Adt13::C0(_, h1755, _, _, _, _) => {
            h1755
        }
    };
    if nat_lt(a1756, a1757) {
        Ok(f263(v2, f18(v0, v1, v3), f179(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v4: u64 = f165(v0.clone(), v3);
        let a1758 = v4;
        let a1759 = 99u64;
        if nat_eq(a1758, a1759) {
            Ok(f263(v2, v1, f179(Str::lit("expected a value, found the end of the text"))))
        } else {
            let a1760 = v4;
            let a1761 = 11u64;
            if nat_eq(a1760, a1761) {
                Ok(f264(v2, {
                    let a1762 = v3;
                    let a1763 = 1u64;
                    nat_add(a1762, a1763)?
                }, Adt0::C2(f167(v0, v3))))
            } else {
                let a1764 = v4;
                let a1765 = 2u64;
                if nat_eq(a1764, a1765) {
                    f265(v2, {
                        let a1766 = v3;
                        let a1767 = 1u64;
                        nat_add(a1766, a1767)?
                    }, Adt14::C0(List::<Adt0>::nil()), 1u64)
                } else {
                    let a1768 = v4;
                    let a1769 = 0u64;
                    if nat_eq(a1768, a1769) {
                        f265(v2, {
                            let a1770 = v3;
                            let a1771 = 1u64;
                            nat_add(a1770, a1771)?
                        }, Adt14::C1(Bytes::lit(&[]), 0u64, List::<(Bytes, Adt0)>::nil()), 2u64)
                    } else {
                        let a1772 = v4;
                        let a1773 = 12u64;
                        if nat_eq(a1772, a1773) {
                            f266(v0.clone(), v1, v2, f18(v0, v1, v3))
                        } else {
                            Ok(f263(v2, f18(v0.clone(), v1, v3), f215(f179(Str::lit("expected a value, found ")), f133(v0, v3)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f255(v0: List<Adt2>, v1: Adt13) -> R<Adt13> {
    let a1775 = f165(v0, match v1.clone() {
        Adt13::C0(h1774, _, _, _, _, _) => {
            h1774
        }
    });
    let a1776 = 3u64;
    if nat_eq(a1775, a1776) {
        f267(v1)
    } else {
        Ok(Adt13::C0(match v1.clone() {
            Adt13::C0(h1777, _, _, _, _, _) => {
                h1777
            }
        }, match v1.clone() {
            Adt13::C0(_, h1778, _, _, _, _) => {
                h1778
            }
        }, 0u64, match v1 {
            Adt13::C0(_, _, _, h1779, _, _) => {
                h1779
            }
        }, Adt0::C5, None::<Adt1>))
    }
}

pub fn f256(v0: List<Adt2>, v1: u64, v2: Adt13) -> R<Adt13> {
    let m1781 = match v2.clone() {
        Adt13::C0(_, _, _, h1780, _, _) => {
            h1780
        }
    };
    match m1781.uncons() {
        None => {
            Ok(f263(v2, 0u64, f179(Str::lit("no object is open"))))
        }
        Some((v3, v4)) => {
            let m1782 = v3;
            match m1782 {
                Adt14::C0(_) => {
                    Ok(f263(v2, 0u64, f179(Str::lit("no object is open"))))
                }
                Adt14::C1(_, _, v8) => {
                    let a1784 = f165(v0.clone(), match v2.clone() {
                        Adt13::C0(h1783, _, _, _, _, _) => {
                            h1783
                        }
                    });
                    let a1785 = 1u64;
                    if nat_eq(a1784, a1785) {
                        f267(v2)
                    } else {
                        f268(v0, v1, v2, v8, v4)
                    }
                }
            }
        }
    }
}

pub fn f257(v0: List<Adt2>, v1: Adt13) -> R<Adt13> {
    let m1787 = match v1.clone() {
        Adt13::C0(_, _, _, h1786, _, _) => {
            h1786
        }
    };
    match m1787.uncons() {
        None => {
            Ok(Adt13::C0(match v1.clone() {
                Adt13::C0(h1788, _, _, _, _, _) => {
                    h1788
                }
            }, match v1.clone() {
                Adt13::C0(_, h1789, _, _, _, _) => {
                    h1789
                }
            }, 9u64, List::<Adt14>::nil(), match v1 {
                Adt13::C0(_, _, _, _, h1790, _) => {
                    h1790
                }
            }, None::<Adt1>))
        }
        Some((v2, v3)) => {
            let m1791 = v2;
            match m1791 {
                Adt14::C0(v4) => {
                    f269(v0, v1.clone(), List::cons(Adt14::C0(List::cons(match v1 {
                        Adt13::C0(_, _, _, _, h1792, _) => {
                            h1792
                        }
                    }, v4)), v3), 1u64, 4u64)
                }
                Adt14::C1(v5, v6, v7) => {
                    if f143(v7.clone(), v5.clone()) {
                        Ok(f263(v1, v6, f215(f215(f179(Str::lit("the member `")), v5), f179(Str::lit("` is written twice")))))
                    } else {
                        f269(v0, v1.clone(), List::cons(Adt14::C1(Bytes::lit(&[]), 0u64, List::cons((v5, match v1 {
                            Adt13::C0(_, _, _, _, h1793, _) => {
                                h1793
                            }
                        }), v7)), v3), 2u64, 5u64)
                    }
                }
            }
        }
    }
}

pub fn f258(v0: List<Adt2>, v1: u64, v2: Adt13, v3: u64, v4: Bytes) -> R<Adt13> {
    let a1795 = f165(v0.clone(), match v2.clone() {
        Adt13::C0(h1794, _, _, _, _, _) => {
            h1794
        }
    });
    let a1796 = v3;
    if nat_eq(a1795, a1796) {
        f267(v2)
    } else {
        Ok(f263(v2.clone(), f18(v0, v1, match v2 {
            Adt13::C0(h1797, _, _, _, _, _) => {
                h1797
            }
        }), v4))
    }
}

pub fn f259(v0: Bytes, v1: Bytes) -> bool {
    let a1802 = {
        let a1798 = v0;
        let a1799 = v1;
        compare_bytes(a1798, a1799)
    };
    let a1803 = {
        let a1800 = Bytes::lit(&[]);
        let a1801 = Bytes::lit(&[0x01]);
        compare_bytes(a1800, a1801)
    };
    equal(a1802, a1803)
}

pub fn f260(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1848 = {
            let a1824 = {
                let a1812 = {
                    let a1806 = {
                        let a1804 = v1;
                        let a1805 = 128u8;
                        fixed_u8::bit_and(a1804, a1805)
                    };
                    let a1807 = 128u8;
                    if equal(a1806, a1807) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1813 = {
                    let a1810 = {
                        let a1808 = v1;
                        let a1809 = 64u8;
                        fixed_u8::bit_and(a1808, a1809)
                    };
                    let a1811 = 64u8;
                    if equal(a1810, a1811) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1812, a1813)?
            };
            let a1825 = {
                let a1822 = {
                    let a1816 = {
                        let a1814 = v1;
                        let a1815 = 32u8;
                        fixed_u8::bit_and(a1814, a1815)
                    };
                    let a1817 = 32u8;
                    if equal(a1816, a1817) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1823 = {
                    let a1820 = {
                        let a1818 = v1;
                        let a1819 = 16u8;
                        fixed_u8::bit_and(a1818, a1819)
                    };
                    let a1821 = 16u8;
                    if equal(a1820, a1821) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1822, a1823)?
            };
            nat_add(a1824, a1825)?
        };
        let a1849 = {
            let a1846 = {
                let a1834 = {
                    let a1828 = {
                        let a1826 = v1;
                        let a1827 = 8u8;
                        fixed_u8::bit_and(a1826, a1827)
                    };
                    let a1829 = 8u8;
                    if equal(a1828, a1829) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1835 = {
                    let a1832 = {
                        let a1830 = v1;
                        let a1831 = 4u8;
                        fixed_u8::bit_and(a1830, a1831)
                    };
                    let a1833 = 4u8;
                    if equal(a1832, a1833) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1834, a1835)?
            };
            let a1847 = {
                let a1844 = {
                    let a1838 = {
                        let a1836 = v1;
                        let a1837 = 2u8;
                        fixed_u8::bit_and(a1836, a1837)
                    };
                    let a1839 = 2u8;
                    if equal(a1838, a1839) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1845 = {
                    let a1842 = {
                        let a1840 = v1;
                        let a1841 = 1u8;
                        fixed_u8::bit_and(a1840, a1841)
                    };
                    let a1843 = 1u8;
                    if equal(a1842, a1843) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1844, a1845)?
            };
            nat_add(a1846, a1847)?
        };
        nat_add(a1848, a1849)?
    };
    let a1850 = v2;
    let a1851 = 48u64;
    if nat_lt(a1850, a1851) {
        Ok(None::<u64>)
    } else {
        let a1852 = v2;
        let a1853 = 57u64;
        if nat_le(a1852, a1853) {
            Ok(Some({
                let a1854 = v2;
                let a1855 = 48u64;
                nat_sub(a1854, a1855)
            }))
        } else {
            let a1856 = v2;
            let a1857 = 65u64;
            if nat_lt(a1856, a1857) {
                Ok(None::<u64>)
            } else {
                let a1858 = v2;
                let a1859 = 70u64;
                if nat_le(a1858, a1859) {
                    Ok(Some({
                        let a1860 = v2;
                        let a1861 = 55u64;
                        nat_sub(a1860, a1861)
                    }))
                } else {
                    let a1862 = v2;
                    let a1863 = 97u64;
                    if nat_lt(a1862, a1863) {
                        Ok(None::<u64>)
                    } else {
                        let a1864 = v2;
                        let a1865 = 102u64;
                        if nat_le(a1864, a1865) {
                            Ok(Some({
                                let a1866 = v2;
                                let a1867 = 87u64;
                                nat_sub(a1866, a1867)
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

pub fn f261(v0: Adt0) -> Option<Adt18> {
    let m1868 = v0;
    match m1868 {
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
        Adt0::C6(r1869, r1870) => {
            let v1 = (*r1869).clone();
            let v2 = (*r1870).clone();
            Some(Adt18::C0(v1, v2))
        }
        Adt0::C7(_, _, _) => {
            None::<Adt18>
        }
    }
}

pub fn f262(v0: u64) -> Bytes {
    let a1871 = v0;
    let a1872 = 34u64;
    if nat_eq(a1871, a1872) {
        f64(Str::lit("\\\""))
    } else {
        let a1873 = v0;
        let a1874 = 92u64;
        if nat_eq(a1873, a1874) {
            f64(Str::lit("\\\\"))
        } else {
            let a1875 = v0;
            let a1876 = 8u64;
            if nat_eq(a1875, a1876) {
                f64(Str::lit("\\b"))
            } else {
                let a1877 = v0;
                let a1878 = 12u64;
                if nat_eq(a1877, a1878) {
                    f64(Str::lit("\\f"))
                } else {
                    let a1879 = v0;
                    let a1880 = 10u64;
                    if nat_eq(a1879, a1880) {
                        f64(Str::lit("\\n"))
                    } else {
                        let a1881 = v0;
                        let a1882 = 13u64;
                        if nat_eq(a1881, a1882) {
                            f64(Str::lit("\\r"))
                        } else {
                            let a1883 = v0;
                            let a1884 = 9u64;
                            if nat_eq(a1883, a1884) {
                                f64(Str::lit("\\t"))
                            } else {
                                f199(f64(Str::lit("\\u00")), f270(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f263(v0: Adt13, v1: u64, v2: Bytes) -> Adt13 {
    Adt13::C0(match v0.clone() {
        Adt13::C0(h1885, _, _, _, _, _) => {
            h1885
        }
    }, match v0.clone() {
        Adt13::C0(_, h1886, _, _, _, _) => {
            h1886
        }
    }, 9u64, match v0.clone() {
        Adt13::C0(_, _, _, h1887, _, _) => {
            h1887
        }
    }, match v0 {
        Adt13::C0(_, _, _, _, h1888, _) => {
            h1888
        }
    }, Some(f271(v1, v2)))
}

pub fn f264(v0: Adt13, v1: u64, v2: Adt0) -> Adt13 {
    Adt13::C0(v1, match v0.clone() {
        Adt13::C0(_, h1889, _, _, _, _) => {
            h1889
        }
    }, 3u64, match v0 {
        Adt13::C0(_, _, _, h1890, _, _) => {
            h1890
        }
    }, v2, None::<Adt1>)
}

pub fn f265(v0: Adt13, v1: u64, v2: Adt14, v3: u64) -> R<Adt13> {
    Ok(Adt13::C0(v1, {
        let a1892 = match v0.clone() {
            Adt13::C0(_, h1891, _, _, _, _) => {
                h1891
            }
        };
        let a1893 = 1u64;
        nat_add(a1892, a1893)?
    }, v3, List::cons(v2, match v0 {
        Adt13::C0(_, _, _, h1894, _, _) => {
            h1894
        }
    }), Adt0::C5, None::<Adt1>))
}

pub fn f266(v0: List<Adt2>, _: u64, v2: Adt13, v3: u64) -> R<Adt13> {
    let v4: u64 = match v2.clone() {
        Adt13::C0(h1895, _, _, _, _, _) => {
            h1895
        }
    };
    let v5: Bytes = f167(v0.clone(), v4);
    let a1896 = v5.clone();
    let a1897 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a1896, a1897) {
        Ok(f264(v2, {
            let a1898 = v4;
            let a1899 = 1u64;
            nat_add(a1898, a1899)?
        }, Adt0::C0(true)))
    } else {
        let a1900 = v5.clone();
        let a1901 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        if equal(a1900, a1901) {
            Ok(f264(v2, {
                let a1902 = v4;
                let a1903 = 1u64;
                nat_add(a1902, a1903)?
            }, Adt0::C0(false)))
        } else {
            if f272(v5.clone())? {
                let m1904 = f273(v5.clone())?;
                match m1904 {
                    None => {
                        Ok(f263(v2, v3, f215(f215(f179(Str::lit("`")), v5), f179(Str::lit("` does not fit a 64-bit integer")))))
                    }
                    Some(v6) => {
                        Ok(f264(v2, {
                            let a1905 = v4;
                            let a1906 = 1u64;
                            nat_add(a1905, a1906)?
                        }, v6))
                    }
                }
            } else {
                if f121(v5.clone())? {
                    let a1909 = f165(v0, {
                        let a1907 = v4;
                        let a1908 = 1u64;
                        nat_add(a1907, a1908)?
                    });
                    let a1910 = 0u64;
                    if nat_eq(a1909, a1910) {
                        f265(v2, {
                            let a1911 = v4;
                            let a1912 = 2u64;
                            nat_add(a1911, a1912)?
                        }, Adt14::C1(Bytes::lit(&[]), 0u64, List::cons((f179(Str::lit("kind")), Adt0::C2(v5)), List::<(Bytes, Adt0)>::nil())), 2u64)
                    } else {
                        Ok(f264(v2, {
                            let a1913 = v4;
                            let a1914 = 1u64;
                            nat_add(a1913, a1914)?
                        }, Adt0::C2(v5)))
                    }
                } else {
                    Ok(f263(v2, v3, f215(f215(f179(Str::lit("`")), v5), f179(Str::lit("` is not a name; quote it to write a string")))))
                }
            }
        }
    }
}

pub fn f267(v0: Adt13) -> R<Adt13> {
    let m1916 = match v0.clone() {
        Adt13::C0(_, _, _, h1915, _, _) => {
            h1915
        }
    };
    match m1916.uncons() {
        None => {
            Ok(f263(v0, 0u64, f179(Str::lit("nothing to close"))))
        }
        Some((v1, v2)) => {
            let m1917 = v1;
            match m1917 {
                Adt14::C0(v3) => {
                    Ok(Adt13::C0({
                        let a1919 = match v0.clone() {
                            Adt13::C0(h1918, _, _, _, _, _) => {
                                h1918
                            }
                        };
                        let a1920 = 1u64;
                        nat_add(a1919, a1920)?
                    }, {
                        let a1922 = match v0 {
                            Adt13::C0(_, h1921, _, _, _, _) => {
                                h1921
                            }
                        };
                        let a1923 = 1u64;
                        nat_sub(a1922, a1923)
                    }, 3u64, v2, Adt0::C3(std::rc::Rc::new(f27(v3))), None::<Adt1>))
                }
                Adt14::C1(_, _, v6) => {
                    Ok(Adt13::C0({
                        let a1925 = match v0.clone() {
                            Adt13::C0(h1924, _, _, _, _, _) => {
                                h1924
                            }
                        };
                        let a1926 = 1u64;
                        nat_add(a1925, a1926)?
                    }, {
                        let a1928 = match v0 {
                            Adt13::C0(_, h1927, _, _, _, _) => {
                                h1927
                            }
                        };
                        let a1929 = 1u64;
                        nat_sub(a1928, a1929)
                    }, 3u64, v2, Adt0::C4(std::rc::Rc::new(f19(f20(v6)?))), None::<Adt1>))
                }
            }
        }
    }
}

pub fn f268(v0: List<Adt2>, v1: u64, v2: Adt13, v3: List<(Bytes, Adt0)>, v4: List<Adt14>) -> R<Adt13> {
    let v5: u64 = match v2.clone() {
        Adt13::C0(h1930, _, _, _, _, _) => {
            h1930
        }
    };
    let v6: u64 = f165(v0.clone(), v5);
    let v7: Bytes = f167(v0.clone(), v5);
    let a1931 = v6;
    let a1932 = 11u64;
    if if nat_eq(a1931, a1932) {
        true
    } else {
        let a1933 = v6;
        let a1934 = 12u64;
        if nat_eq(a1933, a1934) {
            f121(v7.clone())?
        } else {
            false
        }
    } {
        let a1937 = f165(v0.clone(), {
            let a1935 = v5;
            let a1936 = 1u64;
            nat_add(a1935, a1936)?
        });
        let a1938 = 7u64;
        if nat_eq(a1937, a1938) {
            Ok(Adt13::C0({
                let a1939 = v5;
                let a1940 = 2u64;
                nat_add(a1939, a1940)?
            }, match v2 {
                Adt13::C0(_, h1941, _, _, _, _) => {
                    h1941
                }
            }, 0u64, List::cons(Adt14::C1(v7, f18(v0, v1, v5), v3), v4), Adt0::C5, None::<Adt1>))
        } else {
            Ok(f263(v2, f18(v0, v1, {
                let a1942 = v5;
                let a1943 = 1u64;
                nat_add(a1942, a1943)?
            }), f179(Str::lit("expected `:` after a member name"))))
        }
    } else {
        Ok(f263(v2, f18(v0, v1, v5), f179(Str::lit("expected a member name"))))
    }
}

pub fn f269(v0: List<Adt2>, v1: Adt13, v2: List<Adt14>, v3: u64, v4: u64) -> R<Adt13> {
    let a1945 = f165(v0, match v1.clone() {
        Adt13::C0(h1944, _, _, _, _, _) => {
            h1944
        }
    });
    let a1946 = 6u64;
    if nat_eq(a1945, a1946) {
        Ok(Adt13::C0({
            let a1948 = match v1.clone() {
                Adt13::C0(h1947, _, _, _, _, _) => {
                    h1947
                }
            };
            let a1949 = 1u64;
            nat_add(a1948, a1949)?
        }, match v1 {
            Adt13::C0(_, h1950, _, _, _, _) => {
                h1950
            }
        }, v3, v2, Adt0::C5, None::<Adt1>))
    } else {
        Ok(Adt13::C0(match v1.clone() {
            Adt13::C0(h1951, _, _, _, _, _) => {
                h1951
            }
        }, match v1 {
            Adt13::C0(_, h1952, _, _, _, _) => {
                h1952
            }
        }, v4, v2, Adt0::C5, None::<Adt1>))
    }
}

pub fn f270(v0: u64) -> Bytes {
    f199(f245(v0, 16u64), f246(v0))
}

pub fn f271(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f272(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1953 = f169(v0.clone(), 0u64)?;
        let a1954 = 45u64;
        if nat_eq(a1953, a1954) {
            1u64
        } else {
            0u64
        }
    };
    let a1956 = v1;
    let a1957 = {
        let a1955 = v0.clone();
        length_bytes(a1955)
    };
    if nat_lt(a1956, a1957) {
        let a1959 = f172(v0.clone(), v1)?;
        let a1960 = {
            let a1958 = v0;
            length_bytes(a1958)
        };
        Ok(nat_eq(a1959, a1960))
    } else {
        Ok(false)
    }
}

pub fn f273(v0: Bytes) -> R<Option<Adt0>> {
    let v1: bool = {
        let a1961 = f169(v0.clone(), 0u64)?;
        let a1962 = 45u64;
        nat_eq(a1961, a1962)
    };
    let v2: u64 = if v1 {
        1u64
    } else {
        0u64
    };
    let v3: u64 = f274(v0.clone(), v2)?;
    let v4: Bytes = f66(v0.clone(), v3, {
        let a1964 = {
            let a1963 = v0;
            length_bytes(a1963)
        };
        let a1965 = v3;
        nat_sub(a1964, a1965)
    });
    let a1967 = {
        let a1966 = v4.clone();
        length_bytes(a1966)
    };
    let a1968 = 0u64;
    if nat_eq(a1967, a1968) {
        Ok(Some(Adt0::C1(false, 0u64)))
    } else {
        let a1970 = 19u64;
        let a1971 = {
            let a1969 = v4.clone();
            length_bytes(a1969)
        };
        if nat_lt(a1970, a1971) {
            Ok(None::<Adt0>)
        } else {
            let m1972 = f275(v4)?;
            match m1972 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v5) => {
                    if f276(v1, v5) {
                        Ok(Some(Adt0::C1(v1, v5)))
                    } else {
                        Ok(None::<Adt0>)
                    }
                }
            }
        }
    }
}

pub fn f274(v0: Bytes, v1: u64) -> R<u64> {
    let (h1973, _) = f81(Fn12::F277(v0.clone()), {
        let a1975 = {
            let a1974 = v0;
            length_bytes(a1974)
        };
        let a1976 = 1u64;
        nat_add(a1975, a1976)?
    }, v1)?;
    Ok(h1973)
}

pub fn f275(v0: Bytes) -> R<Option<u64>> {
    if f278(v0.clone())? {
        Ok(Some({
            let (_, h1977) = {
                let (h1978, _) = f279(Fn28::F280(v0.clone()), {
                    let a1979 = v0;
                    length_bytes(a1979)
                }, (0u64, 0u64))?;
                h1978
            };
            h1977
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f276(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1980 = 0u64;
        let a1981 = v1;
        if nat_lt(a1980, a1981) {
            let a1982 = v1;
            let a1983 = 9223372036854775808u64;
            nat_le(a1982, a1983)
        } else {
            false
        }
    } else {
        let a1984 = v1;
        let a1985 = 9223372036854775807u64;
        nat_le(a1984, a1985)
    }
}

pub fn f277(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f281(v0, v1)
}

pub fn f278(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1986 = v0.clone();
        length_bytes(a1986)
    };
    let a1987 = 0u64;
    let a1988 = v1;
    if nat_lt(a1987, a1988) {
        let a1989 = f282(v0.clone())?;
        let a1990 = v1;
        if nat_eq(a1989, a1990) {
            let a1991 = 1u64;
            let a1992 = v1;
            if nat_lt(a1991, a1992) {
                let m1995 = {
                    let a1993 = v0;
                    let a1994 = 0u64;
                    index_bytes(a1993, a1994)
                };
                match m1995 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1996 = v2;
                        let a1997 = 48u8;
                        Ok(!equal(a1996, a1997))
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

pub fn f279(v0: Fn28, p2002: u64, p2003: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2001 = (p2002, p2003);
    {
        let mut m2001 = m2001;
        loop {
            let step = {
                let (v1, v2) = m2001;
                let m1998 = v1;
                if m1998 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1998 - 1;
                    let m2000 = {
                        let c1999: Fn28 = v0.clone();
                        c1999.apply(v2)?
                    };
                    match m2000 {
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
                Ok(next) => { m2001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f283(v0, v1)
}

pub fn f281(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2004 = f169(v0, v1)?;
    let a2005 = 48u64;
    if nat_eq(a2004, a2005) {
        Ok(Some({
            let a2006 = v1;
            let a2007 = 1u64;
            nat_add(a2006, a2007)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f282(v0: Bytes) -> R<u64> {
    let (h2008, _) = f81(Fn12::F284(v0.clone()), {
        let a2009 = v0;
        length_bytes(a2009)
    }, 0u64)?;
    Ok(h2008)
}

pub fn f283(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2013 = {
        let a2011 = v0;
        let a2012 = {
            let (h2010, _) = v1;
            h2010
        };
        index_bytes(a2011, a2012)
    };
    match m2013 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2015 = {
                    let (h2014, _) = v1;
                    h2014
                };
                let a2016 = 1u64;
                nat_add(a2015, a2016)?
            }, {
                let a2020 = {
                    let a2018 = {
                        let (_, h2017) = v1;
                        h2017
                    };
                    let a2019 = 10u64;
                    nat_mul(a2018, a2019)?
                };
                let a2021 = f285(v2)?;
                nat_add(a2020, a2021)?
            })))
        }
    }
}

pub fn f284(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2024 = {
        let a2022 = v0;
        let a2023 = v1;
        index_bytes(a2022, a2023)
    };
    match m2024 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f286(v2)? {
                Ok(Some({
                    let a2025 = v1;
                    let a2026 = 1u64;
                    nat_add(a2025, a2026)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f285(v0: u8) -> R<u64> {
    let a2073 = {
        let v1: u8 = v0;
        let a2071 = {
            let a2047 = {
                let a2035 = {
                    let a2029 = {
                        let a2027 = v1;
                        let a2028 = 128u8;
                        fixed_u8::bit_and(a2027, a2028)
                    };
                    let a2030 = 128u8;
                    if equal(a2029, a2030) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2036 = {
                    let a2033 = {
                        let a2031 = v1;
                        let a2032 = 64u8;
                        fixed_u8::bit_and(a2031, a2032)
                    };
                    let a2034 = 64u8;
                    if equal(a2033, a2034) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2035, a2036)?
            };
            let a2048 = {
                let a2045 = {
                    let a2039 = {
                        let a2037 = v1;
                        let a2038 = 32u8;
                        fixed_u8::bit_and(a2037, a2038)
                    };
                    let a2040 = 32u8;
                    if equal(a2039, a2040) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2046 = {
                    let a2043 = {
                        let a2041 = v1;
                        let a2042 = 16u8;
                        fixed_u8::bit_and(a2041, a2042)
                    };
                    let a2044 = 16u8;
                    if equal(a2043, a2044) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2045, a2046)?
            };
            nat_add(a2047, a2048)?
        };
        let a2072 = {
            let a2069 = {
                let a2057 = {
                    let a2051 = {
                        let a2049 = v1;
                        let a2050 = 8u8;
                        fixed_u8::bit_and(a2049, a2050)
                    };
                    let a2052 = 8u8;
                    if equal(a2051, a2052) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2058 = {
                    let a2055 = {
                        let a2053 = v1;
                        let a2054 = 4u8;
                        fixed_u8::bit_and(a2053, a2054)
                    };
                    let a2056 = 4u8;
                    if equal(a2055, a2056) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2057, a2058)?
            };
            let a2070 = {
                let a2067 = {
                    let a2061 = {
                        let a2059 = v1;
                        let a2060 = 2u8;
                        fixed_u8::bit_and(a2059, a2060)
                    };
                    let a2062 = 2u8;
                    if equal(a2061, a2062) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2068 = {
                    let a2065 = {
                        let a2063 = v1;
                        let a2064 = 1u8;
                        fixed_u8::bit_and(a2063, a2064)
                    };
                    let a2066 = 1u8;
                    if equal(a2065, a2066) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2067, a2068)?
            };
            nat_add(a2069, a2070)?
        };
        nat_add(a2071, a2072)?
    };
    let a2074 = 48u64;
    Ok(nat_sub(a2073, a2074))
}

pub fn f286(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2119 = {
            let a2095 = {
                let a2083 = {
                    let a2077 = {
                        let a2075 = v1;
                        let a2076 = 128u8;
                        fixed_u8::bit_and(a2075, a2076)
                    };
                    let a2078 = 128u8;
                    if equal(a2077, a2078) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2084 = {
                    let a2081 = {
                        let a2079 = v1;
                        let a2080 = 64u8;
                        fixed_u8::bit_and(a2079, a2080)
                    };
                    let a2082 = 64u8;
                    if equal(a2081, a2082) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2083, a2084)?
            };
            let a2096 = {
                let a2093 = {
                    let a2087 = {
                        let a2085 = v1;
                        let a2086 = 32u8;
                        fixed_u8::bit_and(a2085, a2086)
                    };
                    let a2088 = 32u8;
                    if equal(a2087, a2088) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2094 = {
                    let a2091 = {
                        let a2089 = v1;
                        let a2090 = 16u8;
                        fixed_u8::bit_and(a2089, a2090)
                    };
                    let a2092 = 16u8;
                    if equal(a2091, a2092) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2093, a2094)?
            };
            nat_add(a2095, a2096)?
        };
        let a2120 = {
            let a2117 = {
                let a2105 = {
                    let a2099 = {
                        let a2097 = v1;
                        let a2098 = 8u8;
                        fixed_u8::bit_and(a2097, a2098)
                    };
                    let a2100 = 8u8;
                    if equal(a2099, a2100) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2106 = {
                    let a2103 = {
                        let a2101 = v1;
                        let a2102 = 4u8;
                        fixed_u8::bit_and(a2101, a2102)
                    };
                    let a2104 = 4u8;
                    if equal(a2103, a2104) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2105, a2106)?
            };
            let a2118 = {
                let a2115 = {
                    let a2109 = {
                        let a2107 = v1;
                        let a2108 = 2u8;
                        fixed_u8::bit_and(a2107, a2108)
                    };
                    let a2110 = 2u8;
                    if equal(a2109, a2110) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2116 = {
                    let a2113 = {
                        let a2111 = v1;
                        let a2112 = 1u8;
                        fixed_u8::bit_and(a2111, a2112)
                    };
                    let a2114 = 1u8;
                    if equal(a2113, a2114) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2115, a2116)?
            };
            nat_add(a2117, a2118)?
        };
        nat_add(a2119, a2120)?
    };
    let a2121 = 48u64;
    let a2122 = v2;
    if nat_le(a2121, a2122) {
        let a2123 = v2;
        let a2124 = 57u64;
        Ok(nat_le(a2123, a2124))
    } else {
        Ok(false)
    }
}
