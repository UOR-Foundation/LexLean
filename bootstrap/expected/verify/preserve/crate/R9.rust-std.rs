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
    C4(Adt1),
    C5,
    C6,
    C7,
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(u64),
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14,
    C15(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
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
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt0, Adt3),
    C1(u64),
    C2(u64, Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(Adt5, Adt0, std::rc::Rc<Adt2>),
    C6(u64, std::rc::Rc<Adt2>),
    C7(u64, std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C9(Adt6, std::rc::Rc<Adt2>),
    C10(std::rc::Rc<Adt2>),
    C11(std::rc::Rc<Adt2>),
    C12(std::rc::Rc<Adt2>, u64),
    C13,
    C14(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C15(Adt5, List<u64>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Bytes),
    C6(Bytes),
    C7(Bytes),
    C8(Bytes),
    C9(Bytes),
    C10(Bytes),
    C11(Bytes),
    C12(Bytes),
    C13(Bytes),
    C14(Adt4),
    C15,
    C16(std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>),
    C22(u64, std::rc::Rc<Adt3>),
    C23,
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt5 {
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
    C15(u64),
}

#[derive(Clone)]
pub enum Adt6 {
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
    C40(Adt1),
    C41(Adt0),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt0, Adt8, Adt0, Adt9),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(Adt0, Adt0, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(Adt0, u64, Adt0, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Adt11, Adt17, Adt18),
    C1(u64, Adt11, Adt11, bool, Adt19, Adt18),
    C2(Adt21, Adt22, Adt11, Adt27, Adt18),
    C3,
    C4(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt12),
    C1(Adt13),
    C2(Adt14, std::rc::Rc<Adt11>),
    C3(Adt15, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C4(Adt16, u64),
    C5,
    C6(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
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
}

#[derive(Clone)]
pub enum Adt13 {
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
pub enum Adt14 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0,
    C1(u64, Adt11, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1(u64, List<Adt20>, bool, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4(u64),
    C5(u64),
    C6(u64),
    C7(u64),
    C8(u64),
    C9(Bytes),
}

#[derive(Clone)]
pub enum Adt22 {
    C0,
    C1(Adt23, Adt11, std::rc::Rc<Adt22>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt24),
    C1(Adt21),
    C2(std::rc::Rc<Adt23>),
    C3(Adt25, std::rc::Rc<Adt23>),
    C4(u64, u64, std::rc::Rc<Adt23>),
    C5,
    C6(std::rc::Rc<Adt23>, std::rc::Rc<Adt23>),
    C7(Adt26),
}

#[derive(Clone)]
pub enum Adt24 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt26 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Adt28, Adt18),
    C1(Adt29, Adt21, Adt18),
    C2(Adt30, std::rc::Rc<Adt27>, Adt18),
    C3(Adt31, std::rc::Rc<Adt27>, bool, Adt18),
    C4(Adt21, std::rc::Rc<Adt27>, bool, Adt18),
    C5(Adt33, std::rc::Rc<Adt27>, Adt18),
    C6(std::rc::Rc<Adt27>, std::rc::Rc<Adt27>, Adt18),
    C7(std::rc::Rc<Adt27>, std::rc::Rc<Adt27>, std::rc::Rc<Adt27>, Adt18),
    C8(std::rc::Rc<Adt27>, std::rc::Rc<Adt27>, Adt18),
    C9(std::rc::Rc<Adt27>),
    C10(Adt34, Adt21, Adt18),
    C11(std::rc::Rc<Adt27>, std::rc::Rc<Adt27>),
    C12,
    C13(std::rc::Rc<Adt27>, std::rc::Rc<Adt27>),
    C14(Adt23, bool, Adt11, std::rc::Rc<Adt27>, Adt18, std::rc::Rc<Adt27>),
    C15(Adt23, std::rc::Rc<Adt27>, std::rc::Rc<Adt27>),
    C16(Adt21, std::rc::Rc<Adt27>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt13, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt26),
    C8,
}

#[derive(Clone)]
pub enum Adt29 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt30 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt31 {
    C0(u64),
    C1(Adt32),
    C2(u64, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(u64, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt11),
    C1,
    C2(Adt11, Adt11),
    C3(Adt11, Adt11),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt11),
}

#[derive(Clone)]
pub enum Adt34 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Fn0 {
    F8(Adt9, Adt8, Adt0, Adt0, Adt35),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn0::F8(k0, k1, k2, k3, k4) => f8(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: List<bool>, v7: Adt7) -> R<Adt10> {
    let v8: Adt35 = Adt35::C0(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone());
    let v9: Adt0 = match v7.clone() {
        Adt7::C0(h1, _, _, _) => {
            h1
        }
    };
    let v10: Adt0 = f1(f2(f3(v1)?), v9.clone());
    let v11: Adt0 = f4(v8, v9.clone(), f5(match v7.clone() {
        Adt7::C0(_, _, _, h2) => {
            h2
        }
    }, Adt9::C0), match v7 {
        Adt7::C0(_, h3, _, _) => {
            h3
        }
    }, v10)?;
    f6(v5, v2, v3, v4, v6, v9, v11, 0u64)
}

pub fn f1(v0: Adt0, v1: Adt0) -> Adt0 {
    let m4 = v0;
    match m4 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(_) => {
            v1
        }
        Adt0::C9(_, _) => {
            v1
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(_, _) => {
            v1
        }
        Adt0::C12(_) => {
            v1
        }
        Adt0::C13(_, _) => {
            v1
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(r5, r6) => {
            let v12 = (*r5).clone();
            let v13 = (*r6).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f1(v13, v1)))
        }
    }
}

pub fn f2(v0: u64) -> Adt0 {
    let m7 = v0;
    if m7 == 0 {
        Adt0::C14
    } else {
        let v1 = m7 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f2(v1)))
    }
}

pub fn f3(v0: Adt0) -> R<u64> {
    let m8 = v0;
    match m8 {
        Adt0::C0 => {
            Ok(0u64)
        }
        Adt0::C1 => {
            Ok(0u64)
        }
        Adt0::C2 => {
            Ok(0u64)
        }
        Adt0::C3 => {
            Ok(0u64)
        }
        Adt0::C4(_) => {
            Ok(0u64)
        }
        Adt0::C5 => {
            Ok(0u64)
        }
        Adt0::C6 => {
            Ok(0u64)
        }
        Adt0::C7 => {
            Ok(0u64)
        }
        Adt0::C8(_) => {
            Ok(0u64)
        }
        Adt0::C9(_, _) => {
            Ok(0u64)
        }
        Adt0::C10(_) => {
            Ok(0u64)
        }
        Adt0::C11(_, _) => {
            Ok(0u64)
        }
        Adt0::C12(_) => {
            Ok(0u64)
        }
        Adt0::C13(_, _) => {
            Ok(0u64)
        }
        Adt0::C14 => {
            Ok(0u64)
        }
        Adt0::C15(_, r11) => {
            let v12 = (*r11).clone();
            let v13: u64 = f3(v12)?;
            let a9 = 1u64;
            let a10 = v13;
            nat_add(a9, a10)
        }
    }
}

pub fn f4(v0: Adt35, v1: Adt0, v2: Adt9, v3: Adt8, v4: Adt0) -> R<Adt0> {
    let (h12, _) = f7(Fn0::F8(v2, v3, v1, v4.clone(), v0), {
        let a13 = 1u64;
        let a14 = f3(v4)?;
        nat_add(a13, a14)?
    }, Adt0::C14)?;
    Ok(h12)
}

pub fn f5(p18: Adt9, p19: Adt9) -> Adt9 {
    let m17 = (p18, p19);
    {
        let mut m17 = m17;
        loop {
            let step = {
                let (v0, v1) = m17;
                let m15 = v0;
                match m15 {
                    Adt9::C0 => {
                        Err::<(Adt9, Adt9), Adt9>(v1)
                    }
                    Adt9::C1(v2, v3, v4, r16) => {
                        let v5 = (*r16).clone();
                        Ok::<(Adt9, Adt9), Adt9>((v5, Adt9::C1(v2, v3, v4, std::rc::Rc::new(v1))))
                    }
                }
            };
            match step {
                Ok(next) => { m17 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Adt2, v1: List<List<u64>>, v2: Adt0, v3: Adt0, v4: List<bool>, v5: Adt0, v6: Adt0, v7: u64) -> R<Adt10> {
    let m20 = v0;
    match m20 {
        Adt2::C0(_, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C1(_) => {
            Ok(Adt10::C3)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C3(_, _, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C4(_, _, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C5(_, _, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C6(_, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C7(_, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C8(_, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C9(_, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C10(_) => {
            Ok(Adt10::C3)
        }
        Adt2::C11(_) => {
            Ok(Adt10::C3)
        }
        Adt2::C12(_, _) => {
            Ok(Adt10::C3)
        }
        Adt2::C13 => {
            Ok(Adt10::C3)
        }
        Adt2::C14(r30, r31) => {
            let v36 = (*r30).clone();
            let v37 = (*r31).clone();
            let m21 = v1;
            match m21.uncons() {
                None => {
                    Ok(Adt10::C3)
                }
                Some((v38, v39)) => {
                    let m22 = v2;
                    match m22 {
                        Adt0::C0 => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C1 => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C2 => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C3 => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C4(_) => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C5 => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C6 => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C7 => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C8(_) => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C9(_, _) => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C10(_) => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C11(_, _) => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C12(_) => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C13(_, _) => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C14 => {
                            Ok(Adt10::C3)
                        }
                        Adt0::C15(r28, r29) => {
                            let v40 = (*r28).clone();
                            let v41 = (*r29).clone();
                            let m23 = v3;
                            match m23 {
                                Adt0::C0 => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C1 => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C2 => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C3 => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C4(_) => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C5 => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C6 => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C7 => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C8(_) => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C9(_, _) => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C10(_) => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C11(_, _) => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C12(_) => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C13(_, _) => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C14 => {
                                    Ok(Adt10::C3)
                                }
                                Adt0::C15(r26, r27) => {
                                    let v42 = (*r26).clone();
                                    let v43 = (*r27).clone();
                                    Ok(Adt10::C4(std::rc::Rc::new(f9(v38, v40, v42, f10(v4.clone(), v7), v36, v5.clone(), v6.clone(), v7)?), std::rc::Rc::new(f6(v37, v39, v41, v43, v4, v5, v6, {
                                        let a24 = v7;
                                        let a25 = 1u64;
                                        nat_add(a24, a25)?
                                    })?)))
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt2::C15(_, _, _) => {
            Ok(Adt10::C3)
        }
    }
}

pub fn f7(v0: Fn0, p36: u64, p37: Adt0) -> R<(Adt0, bool)> {
    let m35 = (p36, p37);
    {
        let mut m35 = m35;
        loop {
            let step = {
                let (v1, v2) = m35;
                let m32 = v1;
                if m32 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m32 - 1;
                    let m34 = {
                        let c33: Fn0 = v0.clone();
                        c33.apply(v2.clone())?
                    };
                    match m34 {
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
                Ok(next) => { m35 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Adt9, v1: Adt8, v2: Adt0, v3: Adt0, v4: Adt35, v5: Adt0) -> R<Option<Adt0>> {
    f11(v4, v2, v0, v1, v3, v5)
}

pub fn f9(v0: List<u64>, v1: Adt0, v2: Adt0, v3: bool, v4: Adt2, v5: Adt0, v6: Adt0, v7: u64) -> R<Adt10> {
    let v8: Adt11 = if v3 {
        Adt11::C2(Adt14::C3, std::rc::Rc::new(f12(v5.clone(), v2)?))
    } else {
        f12(v5.clone(), v2)?
    };
    let m38 = f13(v1.clone(), v6, 0u64)?;
    match m38 {
        None => {
            Ok(Adt10::C2(Adt21::C8(v7), f14(v1, v0, v4, v5, 0u64, 0u64)?, v8, Adt27::C11(std::rc::Rc::new(Adt27::C12), std::rc::Rc::new(Adt27::C12)), f18(0u64)))
        }
        Some(v9) => {
            Ok(Adt10::C2(Adt21::C8(v7), f14(v1.clone(), v0.clone(), v4, v5, 0u64, {
                let a39 = v9;
                let a40 = 1u64;
                nat_add(a39, a40)?
            })?, v8, f15(f16(v0, v9), f17(v1, v9)), f18(0u64)))
        }
    }
}

pub fn f10(v0: List<bool>, v1: u64) -> bool {
    let m41 = f19(v0, v1);
    match m41 {
        None => {
            false
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f11(v0: Adt35, v1: Adt0, v2: Adt9, v3: Adt8, v4: Adt0, v5: Adt0) -> R<Option<Adt0>> {
    let v6: Adt0 = f20(v0, v1, v2, v3, v4, v5.clone());
    let a42 = f3(v6.clone())?;
    let a43 = f3(v5)?;
    if nat_eq(a42, a43) {
        Ok(None::<Adt0>)
    } else {
        Ok(Some(v6))
    }
}

pub fn f12(v0: Adt0, v1: Adt0) -> R<Adt11> {
    let m44 = v1;
    match m44 {
        Adt0::C0 => {
            Ok(Adt11::C0(Adt12::C0))
        }
        Adt0::C1 => {
            Ok(Adt11::C0(Adt12::C1))
        }
        Adt0::C2 => {
            Ok(Adt11::C0(Adt12::C2))
        }
        Adt0::C3 => {
            Ok(Adt11::C0(Adt12::C3))
        }
        Adt0::C4(v2) => {
            Ok(Adt11::C1(f21(v2)))
        }
        Adt0::C5 => {
            Ok(Adt11::C0(Adt12::C5))
        }
        Adt0::C6 => {
            Ok(Adt11::C0(Adt12::C6))
        }
        Adt0::C7 => {
            Ok(Adt11::C0(Adt12::C4))
        }
        Adt0::C8(r45) => {
            let v3 = (*r45).clone();
            Ok(Adt11::C2(Adt14::C0, std::rc::Rc::new(f12(v0, v3)?)))
        }
        Adt0::C9(r46, r47) => {
            let v4 = (*r46).clone();
            let v5 = (*r47).clone();
            Ok(Adt11::C3(Adt15::C0, std::rc::Rc::new(f12(v0.clone(), v4)?), std::rc::Rc::new(f12(v0, v5)?)))
        }
        Adt0::C10(r48) => {
            let v6 = (*r48).clone();
            Ok(Adt11::C2(Adt14::C1, std::rc::Rc::new(f12(v0, v6)?)))
        }
        Adt0::C11(r49, r50) => {
            let v7 = (*r49).clone();
            let v8 = (*r50).clone();
            Ok(Adt11::C3(Adt15::C1, std::rc::Rc::new(f12(v0.clone(), v7)?), std::rc::Rc::new(f12(v0, v8)?)))
        }
        Adt0::C12(v9) => {
            Ok(Adt11::C4(Adt16::C0, v9))
        }
        Adt0::C13(r51, r52) => {
            let v10 = (*r51).clone();
            let v11 = (*r52).clone();
            Ok(Adt11::C4(Adt16::C1, f22(v0, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)), 0u64)?))
        }
        Adt0::C14 => {
            Ok(Adt11::C5)
        }
        Adt0::C15(r53, r54) => {
            let v12 = (*r53).clone();
            let v13 = (*r54).clone();
            Ok(Adt11::C6(std::rc::Rc::new(f12(v0.clone(), v12)?), std::rc::Rc::new(f12(v0, v13)?)))
        }
    }
}

pub fn f13(p61: Adt0, v1: Adt0, p62: u64) -> R<Option<u64>> {
    let m60 = (p61, p62);
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v0, v2) = m60;
                let m55 = v0;
                match m55 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Adt0::C15(r58, r59) => {
                        let v13 = (*r58).clone();
                        let v14 = (*r59).clone();
                        if f23(v13, v1.clone()) {
                            Ok::<(Adt0, u64), R<Option<u64>>>((v14, {
                                let a56 = v2;
                                let a57 = 1u64;
                                nat_add(a56, a57)?
                            }))
                        } else {
                            Err::<(Adt0, u64), R<Option<u64>>>(Ok(Some(v2)))
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

pub fn f14(v0: Adt0, v1: List<u64>, v2: Adt2, v3: Adt0, v4: u64, v5: u64) -> R<Adt22> {
    let m63 = v0;
    match m63 {
        Adt0::C0 => {
            Ok(Adt22::C0)
        }
        Adt0::C1 => {
            Ok(Adt22::C0)
        }
        Adt0::C2 => {
            Ok(Adt22::C0)
        }
        Adt0::C3 => {
            Ok(Adt22::C0)
        }
        Adt0::C4(_) => {
            Ok(Adt22::C0)
        }
        Adt0::C5 => {
            Ok(Adt22::C0)
        }
        Adt0::C6 => {
            Ok(Adt22::C0)
        }
        Adt0::C7 => {
            Ok(Adt22::C0)
        }
        Adt0::C8(_) => {
            Ok(Adt22::C0)
        }
        Adt0::C9(_, _) => {
            Ok(Adt22::C0)
        }
        Adt0::C10(_) => {
            Ok(Adt22::C0)
        }
        Adt0::C11(_, _) => {
            Ok(Adt22::C0)
        }
        Adt0::C12(_) => {
            Ok(Adt22::C0)
        }
        Adt0::C13(_, _) => {
            Ok(Adt22::C0)
        }
        Adt0::C14 => {
            Ok(Adt22::C0)
        }
        Adt0::C15(r73, r74) => {
            let v16 = (*r73).clone();
            let v17 = (*r74).clone();
            let m64 = v1;
            match m64.uncons() {
                None => {
                    Ok(Adt22::C0)
                }
                Some((v18, v19)) => {
                    Ok(Adt22::C1({
                        let a65 = v5;
                        let a66 = 0u64;
                        if nat_eq(a65, a66) {
                            f24(v16.clone(), v2.clone(), v18)
                        } else {
                            let a69 = {
                                let a67 = v4;
                                let a68 = 1u64;
                                nat_add(a67, a68)?
                            };
                            let a70 = v5;
                            if nat_eq(a69, a70) {
                                Adt23::C1(Adt21::C0(v18))
                            } else {
                                f25(0u64)
                            }
                        }
                    }, f12(v3.clone(), v16)?, std::rc::Rc::new(f14(v17, v19, v2, v3, {
                        let a71 = v4;
                        let a72 = 1u64;
                        nat_add(a71, a72)?
                    }, v5)?)))
                }
            }
        }
    }
}

pub fn f15(v0: u64, v1: Adt0) -> Adt27 {
    Adt27::C11(std::rc::Rc::new(Adt27::C12), std::rc::Rc::new(Adt27::C8(std::rc::Rc::new(Adt27::C1(Adt29::C0, Adt21::C0(v0), f26(v1.clone()))), std::rc::Rc::new(Adt27::C12), f26(v1))))
}

pub fn f16(v0: List<u64>, v1: u64) -> u64 {
    let m75 = f27(v0, v1);
    match m75 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f17(v0: Adt0, v1: u64) -> Adt0 {
    let m76 = f28(v0, v1);
    match m76 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f18(_: u64) -> Adt18 {
    Adt18::C0({
        let a77 = Str::lit("function");
        utf8_encode(a77)
    }, None::<Adt13>)
}

pub fn f19(p81: List<bool>, p82: u64) -> Option<bool> {
    let m80 = (p81, p82);
    {
        let mut m80 = m80;
        loop {
            let step = {
                let (v0, v1) = m80;
                let m78 = v0;
                match m78.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m79 = v1;
                        if m79 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m79 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
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

pub fn f20(v0: Adt35, v1: Adt0, v2: Adt9, v3: Adt8, p87: Adt0, p88: Adt0) -> Adt0 {
    let m86 = (p87, p88);
    {
        let mut m86 = m86;
        loop {
            let step = {
                let (v4, v5) = m86;
                let m83 = v4;
                match m83 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C15(r84, r85) => {
                        let v16 = (*r84).clone();
                        let v17 = (*r85).clone();
                        Ok::<(Adt0, Adt0), Adt0>((v17, if f29(v5.clone(), v16.clone()) {
                            v5
                        } else {
                            if f30(v1.clone(), v3.clone(), v5.clone(), v16.clone(), f31(v0.clone(), v2.clone(), v16.clone())) {
                                f32(v5, v16)
                            } else {
                                v5
                            }
                        }))
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

pub fn f21(v0: Adt1) -> Adt13 {
    let m89 = v0;
    match m89 {
        Adt1::C0 => {
            Adt13::C0
        }
        Adt1::C1 => {
            Adt13::C1
        }
        Adt1::C2 => {
            Adt13::C2
        }
        Adt1::C3 => {
            Adt13::C3
        }
        Adt1::C4 => {
            Adt13::C4
        }
        Adt1::C5 => {
            Adt13::C5
        }
        Adt1::C6 => {
            Adt13::C6
        }
        Adt1::C7 => {
            Adt13::C7
        }
    }
}

pub fn f22(p96: Adt0, v1: Adt0, p97: u64) -> R<u64> {
    let m95 = (p96, p97);
    {
        let mut m95 = m95;
        loop {
            let step = {
                let (v0, v2) = m95;
                let m90 = v0;
                match m90 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C15(r93, r94) => {
                        let v13 = (*r93).clone();
                        let v14 = (*r94).clone();
                        if f33(v13, v1.clone()) {
                            Err::<(Adt0, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(Adt0, u64), R<u64>>((v14, {
                                let a91 = v2;
                                let a92 = 1u64;
                                nat_add(a91, a92)?
                            }))
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

pub fn f23(v0: Adt0, v1: Adt0) -> bool {
    let m98 = v0;
    match m98 {
        Adt0::C0 => {
            true
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6 => {
            true
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(_) => {
            true
        }
        Adt0::C9(r99, r100) => {
            let v4 = (*r99).clone();
            let v5 = (*r100).clone();
            if f23(v4, v1.clone()) {
                true
            } else {
                f23(v5, v1)
            }
        }
        Adt0::C10(_) => {
            true
        }
        Adt0::C11(r101, r102) => {
            let v7 = (*r101).clone();
            let v8 = (*r102).clone();
            if f23(v7, v1.clone()) {
                f23(v8, v1)
            } else {
                false
            }
        }
        Adt0::C12(v9) => {
            f29(v1, Adt0::C12(v9))
        }
        Adt0::C13(r103, r104) => {
            let v10 = (*r103).clone();
            let v11 = (*r104).clone();
            f29(v1, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)))
        }
        Adt0::C14 => {
            true
        }
        Adt0::C15(_, _) => {
            true
        }
    }
}

pub fn f24(v0: Adt0, v1: Adt2, v2: u64) -> Adt23 {
    let a105 = f33(v0, Adt0::C0);
    if if bool_not(a105) {
        f34(v1, v2)
    } else {
        false
    } {
        Adt23::C1(Adt21::C0(v2))
    } else {
        f25(0u64)
    }
}

pub fn f25(_: u64) -> Adt23 {
    Adt23::C0(Adt24::C0)
}

pub fn f26(v0: Adt0) -> Adt18 {
    Adt18::C0(f35(v0), None::<Adt13>)
}

pub fn f27(p109: List<u64>, p110: u64) -> Option<u64> {
    let m108 = (p109, p110);
    {
        let mut m108 = m108;
        loop {
            let step = {
                let (v0, v1) = m108;
                let m106 = v0;
                match m106.uncons() {
                    None => {
                        Err::<(List<u64>, u64), Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        let m107 = v1;
                        if m107 == 0 {
                            Err::<(List<u64>, u64), Option<u64>>(Some(v2))
                        } else {
                            let v4 = m107 - 1;
                            Ok::<(List<u64>, u64), Option<u64>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(p116: Adt0, p117: u64) -> Option<Adt0> {
    let m115 = (p116, p117);
    {
        let mut m115 = m115;
        loop {
            let step = {
                let (v0, v1) = m115;
                let m111 = v0;
                match m111 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C15(r113, r114) => {
                        let v12 = (*r113).clone();
                        let v13 = (*r114).clone();
                        let m112 = v1;
                        if m112 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m112 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(p122: Adt0, v1: Adt0) -> bool {
    let m121 = p122;
    {
        let mut m121 = m121;
        loop {
            let step = {
                let v0 = m121;
                let m118 = v0;
                match m118 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r119, r120) => {
                        let v12 = (*r119).clone();
                        let v13 = (*r120).clone();
                        if f33(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0, p127: Adt0) -> bool {
    let m126 = p127;
    {
        let mut m126 = m126;
        loop {
            let step = {
                let v4 = m126;
                let m123 = v4;
                match m123 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r124, r125) => {
                        let v15 = (*r124).clone();
                        let v16 = (*r125).clone();
                        if f36(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v15) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v16)
                        }
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

pub fn f31(v0: Adt35, v1: Adt9, v2: Adt0) -> Adt0 {
    let m128 = v2;
    match m128 {
        Adt0::C0 => {
            Adt0::C14
        }
        Adt0::C1 => {
            Adt0::C14
        }
        Adt0::C2 => {
            Adt0::C14
        }
        Adt0::C3 => {
            Adt0::C14
        }
        Adt0::C4(_) => {
            Adt0::C14
        }
        Adt0::C5 => {
            Adt0::C14
        }
        Adt0::C6 => {
            Adt0::C14
        }
        Adt0::C7 => {
            Adt0::C14
        }
        Adt0::C8(_) => {
            Adt0::C14
        }
        Adt0::C9(_, _) => {
            Adt0::C14
        }
        Adt0::C10(_) => {
            Adt0::C14
        }
        Adt0::C11(_, _) => {
            Adt0::C14
        }
        Adt0::C12(v10) => {
            let m129 = f37(v0, v10);
            match m129 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r130, r131) => {
            let v12 = (*r130).clone();
            let v13 = (*r131).clone();
            f38(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f32(v0: Adt0, v1: Adt0) -> Adt0 {
    if f29(v0.clone(), v1.clone()) {
        v0
    } else {
        f1(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f33(v0: Adt0, v1: Adt0) -> bool {
    let m132 = v0;
    match m132 {
        Adt0::C0 => {
            let a133 = f39(v1);
            let a134 = 0u64;
            nat_eq(a133, a134)
        }
        Adt0::C1 => {
            let a135 = f39(v1);
            let a136 = 1u64;
            nat_eq(a135, a136)
        }
        Adt0::C2 => {
            let a137 = f39(v1);
            let a138 = 2u64;
            nat_eq(a137, a138)
        }
        Adt0::C3 => {
            let a139 = f39(v1);
            let a140 = 3u64;
            nat_eq(a139, a140)
        }
        Adt0::C4(v2) => {
            let a149 = f39(v1.clone());
            let a150 = 4u64;
            if nat_eq(a149, a150) {
                let a151 = f40(v2);
                let a152 = f40(f41(v1));
                nat_eq(a151, a152)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a141 = f39(v1);
            let a142 = 5u64;
            nat_eq(a141, a142)
        }
        Adt0::C6 => {
            let a143 = f39(v1);
            let a144 = 6u64;
            nat_eq(a143, a144)
        }
        Adt0::C7 => {
            let a145 = f39(v1);
            let a146 = 7u64;
            nat_eq(a145, a146)
        }
        Adt0::C8(r169) => {
            let v3 = (*r169).clone();
            let v4: bool = f33(v3, f42(v1.clone()));
            let a153 = f39(v1);
            let a154 = 8u64;
            if nat_eq(a153, a154) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r170, r171) => {
            let v5 = (*r170).clone();
            let v6 = (*r171).clone();
            let v7: bool = f33(v5, f42(v1.clone()));
            let v8: bool = f33(v6, f43(v1.clone()));
            let a155 = f39(v1);
            let a156 = 9u64;
            if nat_eq(a155, a156) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r172) => {
            let v9 = (*r172).clone();
            let v10: bool = f33(v9, f42(v1.clone()));
            let a157 = f39(v1);
            let a158 = 10u64;
            if nat_eq(a157, a158) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r173, r174) => {
            let v11 = (*r173).clone();
            let v12 = (*r174).clone();
            let v13: bool = f33(v11, f42(v1.clone()));
            let v14: bool = f33(v12, f43(v1.clone()));
            let a159 = f39(v1);
            let a160 = 11u64;
            if nat_eq(a159, a160) {
                if v13 {
                    v14
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C12(v15) => {
            let a161 = f39(v1.clone());
            let a162 = 12u64;
            if nat_eq(a161, a162) {
                let a163 = v15;
                let a164 = f44(v1);
                nat_eq(a163, a164)
            } else {
                false
            }
        }
        Adt0::C13(r175, r176) => {
            let v16 = (*r175).clone();
            let v17 = (*r176).clone();
            let v18: bool = f33(v16, f42(v1.clone()));
            let v19: bool = f33(v17, f43(v1.clone()));
            let a165 = f39(v1);
            let a166 = 13u64;
            if nat_eq(a165, a166) {
                if v18 {
                    v19
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C14 => {
            let a147 = f39(v1);
            let a148 = 14u64;
            nat_eq(a147, a148)
        }
        Adt0::C15(r177, r178) => {
            let v20 = (*r177).clone();
            let v21 = (*r178).clone();
            let v22: bool = f33(v20, f42(v1.clone()));
            let v23: bool = f33(v21, f43(v1.clone()));
            let a167 = f39(v1);
            let a168 = 15u64;
            if nat_eq(a167, a168) {
                if v22 {
                    v23
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f34(v0: Adt2, v1: u64) -> bool {
    let m179 = v0;
    match m179 {
        Adt2::C0(_, _) => {
            false
        }
        Adt2::C1(v4) => {
            let a180 = v4;
            let a181 = v1;
            nat_eq(a180, a181)
        }
        Adt2::C2(v5, _, r184, r185) => {
            let v7 = (*r184).clone();
            let v8 = (*r185).clone();
            if f34(v7, v1) {
                true
            } else {
                let a182 = v5;
                let a183 = v1;
                if nat_eq(a182, a183) {
                    false
                } else {
                    f34(v8, v1)
                }
            }
        }
        Adt2::C3(r186, r187, r188) => {
            let v9 = (*r186).clone();
            let v10 = (*r187).clone();
            let v11 = (*r188).clone();
            if f34(v9, v1) {
                true
            } else {
                if f34(v10, v1) {
                    true
                } else {
                    f34(v11, v1)
                }
            }
        }
        Adt2::C4(_, r189, r190) => {
            let v13 = (*r189).clone();
            let v14 = (*r190).clone();
            if f34(v13, v1) {
                true
            } else {
                f34(v14, v1)
            }
        }
        Adt2::C5(_, _, r191) => {
            let v17 = (*r191).clone();
            f34(v17, v1)
        }
        Adt2::C6(_, r192) => {
            let v19 = (*r192).clone();
            f34(v19, v1)
        }
        Adt2::C7(_, r193) => {
            let v21 = (*r193).clone();
            f34(v21, v1)
        }
        Adt2::C8(r194, r195) => {
            let v22 = (*r194).clone();
            let v23 = (*r195).clone();
            if f34(v22, v1) {
                true
            } else {
                f34(v23, v1)
            }
        }
        Adt2::C9(_, r196) => {
            let v25 = (*r196).clone();
            f34(v25, v1)
        }
        Adt2::C10(r197) => {
            let v26 = (*r197).clone();
            f34(v26, v1)
        }
        Adt2::C11(r198) => {
            let v27 = (*r198).clone();
            f34(v27, v1)
        }
        Adt2::C12(r199, _) => {
            let v28 = (*r199).clone();
            f34(v28, v1)
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14(r200, r201) => {
            let v30 = (*r200).clone();
            let v31 = (*r201).clone();
            if f34(v30, v1) {
                true
            } else {
                f34(v31, v1)
            }
        }
        Adt2::C15(_, v33, r202) => {
            let v34 = (*r202).clone();
            if f45(v33, v1) {
                false
            } else {
                f34(v34, v1)
            }
        }
    }
}

pub fn f35(v0: Adt0) -> Bytes {
    let m203 = v0;
    match m203 {
        Adt0::C0 => {
            let a204 = Str::lit("type:unit");
            utf8_encode(a204)
        }
        Adt0::C1 => {
            let a205 = Str::lit("type:bool");
            utf8_encode(a205)
        }
        Adt0::C2 => {
            let a206 = Str::lit("type:nat");
            utf8_encode(a206)
        }
        Adt0::C3 => {
            let a207 = Str::lit("type:int");
            utf8_encode(a207)
        }
        Adt0::C4(_) => {
            let a208 = Str::lit("type:fixed");
            utf8_encode(a208)
        }
        Adt0::C5 => {
            let a209 = Str::lit("type:string");
            utf8_encode(a209)
        }
        Adt0::C6 => {
            let a210 = Str::lit("type:bytes");
            utf8_encode(a210)
        }
        Adt0::C7 => {
            let a211 = Str::lit("type:ordering");
            utf8_encode(a211)
        }
        Adt0::C8(_) => {
            let a212 = Str::lit("type:option");
            utf8_encode(a212)
        }
        Adt0::C9(_, _) => {
            let a213 = Str::lit("type:result");
            utf8_encode(a213)
        }
        Adt0::C10(_) => {
            let a214 = Str::lit("type:list");
            utf8_encode(a214)
        }
        Adt0::C11(_, _) => {
            let a215 = Str::lit("type:pair");
            utf8_encode(a215)
        }
        Adt0::C12(_) => {
            let a216 = Str::lit("type:adt");
            utf8_encode(a216)
        }
        Adt0::C13(_, _) => {
            let a217 = Str::lit("type:fn");
            utf8_encode(a217)
        }
        Adt0::C14 => {
            let a218 = Str::lit("type:nil");
            utf8_encode(a218)
        }
        Adt0::C15(_, _) => {
            let a219 = Str::lit("type:nil");
            utf8_encode(a219)
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0, p224: Adt0) -> bool {
    let m223 = p224;
    {
        let mut m223 = m223;
        loop {
            let step = {
                let v4 = m223;
                let m220 = v4;
                match m220 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C15(r221, r222) => {
                        let v15 = (*r221).clone();
                        let v16 = (*r222).clone();
                        if if f46(v0.clone(), v1.clone(), v3.clone(), v15.clone()) {
                            true
                        } else {
                            f23(v15, v2.clone())
                        } {
                            Ok::<Adt0, bool>(v16)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
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

pub fn f37(v0: Adt35, v1: u64) -> Option<Adt0> {
    f47(match v0 {
        Adt35::C0(_, h225, _, _, _, _) => {
            h225
        }
    }, v1)
}

pub fn f38(v0: Adt9, v1: Adt0) -> Adt0 {
    let m226 = v0;
    match m226 {
        Adt9::C0 => {
            Adt0::C14
        }
        Adt9::C1(v2, _, v4, r227) => {
            let v5 = (*r227).clone();
            if f33(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f38(v5, v1)))
            } else {
                f38(v5, v1)
            }
        }
    }
}

pub fn f39(v0: Adt0) -> u64 {
    let m228 = v0;
    match m228 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            1u64
        }
        Adt0::C2 => {
            2u64
        }
        Adt0::C3 => {
            3u64
        }
        Adt0::C4(_) => {
            4u64
        }
        Adt0::C5 => {
            5u64
        }
        Adt0::C6 => {
            6u64
        }
        Adt0::C7 => {
            7u64
        }
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9(_, _) => {
            9u64
        }
        Adt0::C10(_) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12(_) => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14 => {
            14u64
        }
        Adt0::C15(_, _) => {
            15u64
        }
    }
}

pub fn f40(v0: Adt1) -> u64 {
    let m229 = v0;
    match m229 {
        Adt1::C0 => {
            0u64
        }
        Adt1::C1 => {
            1u64
        }
        Adt1::C2 => {
            2u64
        }
        Adt1::C3 => {
            3u64
        }
        Adt1::C4 => {
            4u64
        }
        Adt1::C5 => {
            5u64
        }
        Adt1::C6 => {
            6u64
        }
        Adt1::C7 => {
            7u64
        }
    }
}

pub fn f41(v0: Adt0) -> Adt1 {
    let m230 = v0;
    match m230 {
        Adt0::C0 => {
            Adt1::C0
        }
        Adt0::C1 => {
            Adt1::C0
        }
        Adt0::C2 => {
            Adt1::C0
        }
        Adt0::C3 => {
            Adt1::C0
        }
        Adt0::C4(v1) => {
            v1
        }
        Adt0::C5 => {
            Adt1::C0
        }
        Adt0::C6 => {
            Adt1::C0
        }
        Adt0::C7 => {
            Adt1::C0
        }
        Adt0::C8(_) => {
            Adt1::C0
        }
        Adt0::C9(_, _) => {
            Adt1::C0
        }
        Adt0::C10(_) => {
            Adt1::C0
        }
        Adt0::C11(_, _) => {
            Adt1::C0
        }
        Adt0::C12(_) => {
            Adt1::C0
        }
        Adt0::C13(_, _) => {
            Adt1::C0
        }
        Adt0::C14 => {
            Adt1::C0
        }
        Adt0::C15(_, _) => {
            Adt1::C0
        }
    }
}

pub fn f42(v0: Adt0) -> Adt0 {
    let m231 = f48(v0);
    match m231 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f43(v0: Adt0) -> Adt0 {
    let m232 = f49(v0);
    match m232 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f44(v0: Adt0) -> u64 {
    let m233 = v0;
    match m233 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            0u64
        }
        Adt0::C2 => {
            0u64
        }
        Adt0::C3 => {
            0u64
        }
        Adt0::C4(_) => {
            0u64
        }
        Adt0::C5 => {
            0u64
        }
        Adt0::C6 => {
            0u64
        }
        Adt0::C7 => {
            0u64
        }
        Adt0::C8(_) => {
            0u64
        }
        Adt0::C9(_, _) => {
            0u64
        }
        Adt0::C10(_) => {
            0u64
        }
        Adt0::C11(_, _) => {
            0u64
        }
        Adt0::C12(v8) => {
            v8
        }
        Adt0::C13(_, _) => {
            0u64
        }
        Adt0::C14 => {
            0u64
        }
        Adt0::C15(_, _) => {
            0u64
        }
    }
}

pub fn f45(p238: List<u64>, v1: u64) -> bool {
    let m237 = p238;
    {
        let mut m237 = m237;
        loop {
            let step = {
                let v0 = m237;
                let m234 = v0;
                match m234.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a235 = v2;
                        let a236 = v1;
                        if nat_eq(a235, a236) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0) -> bool {
    f50(v2, v1, f51(v0, Adt0::C14, v3))
}

pub fn f47(p244: Adt0, p245: u64) -> Option<Adt0> {
    let m243 = (p244, p245);
    {
        let mut m243 = m243;
        loop {
            let step = {
                let (v0, v1) = m243;
                let m239 = v0;
                match m239 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C15(r241, r242) => {
                        let v12 = (*r241).clone();
                        let v13 = (*r242).clone();
                        let m240 = v1;
                        if m240 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m240 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Adt0) -> Option<Adt0> {
    let m246 = v0;
    match m246 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(r247) => {
            let v2 = (*r247).clone();
            Some(v2)
        }
        Adt0::C9(r248, _) => {
            let v3 = (*r248).clone();
            Some(v3)
        }
        Adt0::C10(r249) => {
            let v5 = (*r249).clone();
            Some(v5)
        }
        Adt0::C11(r250, _) => {
            let v6 = (*r250).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r251, _) => {
            let v9 = (*r251).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r252, _) => {
            let v11 = (*r252).clone();
            Some(v11)
        }
    }
}

pub fn f49(v0: Adt0) -> Option<Adt0> {
    let m253 = v0;
    match m253 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(_) => {
            None::<Adt0>
        }
        Adt0::C9(_, r254) => {
            let v4 = (*r254).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r255) => {
            let v7 = (*r255).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r256) => {
            let v10 = (*r256).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r257) => {
            let v12 = (*r257).clone();
            Some(v12)
        }
    }
}

pub fn f50(v0: Adt0, v1: Adt8, p262: Adt0) -> bool {
    let m261 = p262;
    {
        let mut m261 = m261;
        loop {
            let step = {
                let v2 = m261;
                let m258 = v2;
                match m258 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r259, r260) => {
                        let v13 = (*r259).clone();
                        let v14 = (*r260).clone();
                        if f33(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f52(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
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

pub fn f51(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m263 = v2;
    match m263 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(r264) => {
            let v4 = (*r264).clone();
            f51(v0, v1, v4)
        }
        Adt0::C9(r265, r266) => {
            let v5 = (*r265).clone();
            let v6 = (*r266).clone();
            f51(v0.clone(), f51(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r267, r268) => {
            let v8 = (*r267).clone();
            let v9 = (*r268).clone();
            f51(v0.clone(), f51(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f32(v1, Adt0::C12(v10))
        }
        Adt0::C13(r269, r270) => {
            let v11 = (*r269).clone();
            let v12 = (*r270).clone();
            if f29(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f32(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
            } else {
                v1
            }
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(_, _) => {
            v1
        }
    }
}

pub fn f52(p274: Adt8, v1: Adt0, v2: Adt0) -> bool {
    let m273 = p274;
    {
        let mut m273 = m273;
        loop {
            let step = {
                let v0 = m273;
                let m271 = v0;
                match m271 {
                    Adt8::C0 => {
                        Err::<Adt8, bool>(false)
                    }
                    Adt8::C1(v3, v4, r272) => {
                        let v5 = (*r272).clone();
                        if f33(v3, v1.clone()) {
                            if f33(v4, v2.clone()) {
                                Err::<Adt8, bool>(true)
                            } else {
                                Ok::<Adt8, bool>(v5)
                            }
                        } else {
                            Ok::<Adt8, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m273 = next; }
                Err(done) => break done,
            }
        }
    }
}
