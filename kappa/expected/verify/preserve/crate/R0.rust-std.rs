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
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt4, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, Adt6, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Adt1, u64, List<Adt1>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F19(Bytes, u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Adt2)) -> R<(u64, Adt2)> {
        match self {
            Fn0::F19(k0, k1, k2) => f19(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F24(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn1::F24(k0, k1, k2) => f24(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F30,
}

impl Fn2 {
    pub fn apply(&self, p0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
        match self {
            Fn2::F30 => f30(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F37(Bytes, u64, u64, u64),
    F95,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt3)) -> R<(u64, Adt3)> {
        match self {
            Fn3::F37(k0, k1, k2, k3) => f37(k0.clone(), *k1, *k2, *k3, p0),
            Fn3::F95 => f95(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F44(Bytes, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
        match self {
            Fn4::F44(k0, k1) => f44(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F53,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt5) -> R<Adt5> {
        match self {
            Fn5::F53 => f53(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F58,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt7) -> R<Adt7> {
        match self {
            Fn6::F58 => f58(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F63,
}

impl Fn7 {
    pub fn apply(&self, p0: (Adt9, Adt8)) -> (Adt9, Adt8) {
        match self {
            Fn7::F63 => f63(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F65(u64, Bytes, u64),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Adt1)) -> R<(u64, Adt1)> {
        match self {
            Fn8::F65(k0, k1, k2) => f65(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F69,
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn9::F69 => f69(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F84,
}

impl Fn10 {
    pub fn apply(&self, p0: Adt10) -> Adt10 {
        match self {
            Fn10::F84 => f84(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F105,
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
        match self {
            Fn11::F105 => f105(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m1 = f1(v0.clone(), v1)?;
    match m1 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a2 = f2(v0);
                let a3 = v2;
                append_bytes(a2, a3)
            }))
        }
    }
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f3(v0.clone(), Str::lit("sha256")) {
        Ok(Some(f4(v1)?))
    } else {
        if f3(v0.clone(), Str::lit("sha3-256")) {
            Ok(Some(f5(v1)?))
        } else {
            if f3(v0.clone(), Str::lit("keccak256")) {
                Ok(Some(f6(v1)?))
            } else {
                if f3(v0.clone(), Str::lit("sha512")) {
                    Ok(Some(f7(v1)?))
                } else {
                    if f3(v0, Str::lit("blake3")) {
                        Ok(Some(f8(v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f2(v0: Bytes) -> Bytes {
    let a4 = v0;
    let a5 = f9(Str::lit(":"));
    append_bytes(a4, a5)
}

pub fn f3(v0: Bytes, v1: Str) -> bool {
    let a6 = v0;
    let a7 = f9(v1);
    equal(a6, a7)
}

pub fn f4(v0: Bytes) -> R<Bytes> {
    Ok(f10(f11(v0)?))
}

pub fn f5(v0: Bytes) -> R<Bytes> {
    f12(v0, 6u64)
}

pub fn f6(v0: Bytes) -> R<Bytes> {
    f12(v0, 1u64)
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f13(v0)?;
    let a28 = f14(match v1.clone() {
        Adt0::C0(h8, _, _, _, _, _, _, _) => {
            h8
        }
    });
    let a29 = {
        let a26 = f14(match v1.clone() {
            Adt0::C0(_, h9, _, _, _, _, _, _) => {
                h9
            }
        });
        let a27 = {
            let a24 = f14(match v1.clone() {
                Adt0::C0(_, _, h10, _, _, _, _, _) => {
                    h10
                }
            });
            let a25 = {
                let a22 = f14(match v1.clone() {
                    Adt0::C0(_, _, _, h11, _, _, _, _) => {
                        h11
                    }
                });
                let a23 = {
                    let a20 = f14(match v1.clone() {
                        Adt0::C0(_, _, _, _, h12, _, _, _) => {
                            h12
                        }
                    });
                    let a21 = {
                        let a18 = f14(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h13, _, _) => {
                                h13
                            }
                        });
                        let a19 = {
                            let a16 = f14(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h14, _) => {
                                    h14
                                }
                            });
                            let a17 = f14(match v1 {
                                Adt0::C0(_, _, _, _, _, _, _, h15) => {
                                    h15
                                }
                            });
                            append_bytes(a16, a17)
                        };
                        append_bytes(a18, a19)
                    };
                    append_bytes(a20, a21)
                };
                append_bytes(a22, a23)
            };
            append_bytes(a24, a25)
        };
        append_bytes(a26, a27)
    };
    Ok(append_bytes(a28, a29))
}

pub fn f8(v0: Bytes) -> R<Bytes> {
    let v1: Adt1 = f15(v0)?;
    let a50 = f16(match v1.clone() {
        Adt1::C0(h30, _, _, _, _, _, _, _) => {
            h30
        }
    });
    let a51 = {
        let a48 = f16(match v1.clone() {
            Adt1::C0(_, h31, _, _, _, _, _, _) => {
                h31
            }
        });
        let a49 = {
            let a46 = f16(match v1.clone() {
                Adt1::C0(_, _, h32, _, _, _, _, _) => {
                    h32
                }
            });
            let a47 = {
                let a44 = f16(match v1.clone() {
                    Adt1::C0(_, _, _, h33, _, _, _, _) => {
                        h33
                    }
                });
                let a45 = {
                    let a42 = f16(match v1.clone() {
                        Adt1::C0(_, _, _, _, h34, _, _, _) => {
                            h34
                        }
                    });
                    let a43 = {
                        let a40 = f16(match v1.clone() {
                            Adt1::C0(_, _, _, _, _, h35, _, _) => {
                                h35
                            }
                        });
                        let a41 = {
                            let a38 = f16(match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, h36, _) => {
                                    h36
                                }
                            });
                            let a39 = f16(match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, h37) => {
                                    h37
                                }
                            });
                            append_bytes(a38, a39)
                        };
                        append_bytes(a40, a41)
                    };
                    append_bytes(a42, a43)
                };
                append_bytes(a44, a45)
            };
            append_bytes(a46, a47)
        };
        append_bytes(a48, a49)
    };
    Ok(append_bytes(a50, a51))
}

pub fn f9(v0: Str) -> Bytes {
    let a52 = v0;
    utf8_encode(a52)
}

pub fn f10(v0: Adt2) -> Bytes {
    let a73 = f17(match v0.clone() {
        Adt2::C0(h53, _, _, _, _, _, _, _) => {
            h53
        }
    });
    let a74 = {
        let a71 = f17(match v0.clone() {
            Adt2::C0(_, h54, _, _, _, _, _, _) => {
                h54
            }
        });
        let a72 = {
            let a69 = f17(match v0.clone() {
                Adt2::C0(_, _, h55, _, _, _, _, _) => {
                    h55
                }
            });
            let a70 = {
                let a67 = f17(match v0.clone() {
                    Adt2::C0(_, _, _, h56, _, _, _, _) => {
                        h56
                    }
                });
                let a68 = {
                    let a65 = f17(match v0.clone() {
                        Adt2::C0(_, _, _, _, h57, _, _, _) => {
                            h57
                        }
                    });
                    let a66 = {
                        let a63 = f17(match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h58, _, _) => {
                                h58
                            }
                        });
                        let a64 = {
                            let a61 = f17(match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h59, _) => {
                                    h59
                                }
                            });
                            let a62 = f17(match v0 {
                                Adt2::C0(_, _, _, _, _, _, _, h60) => {
                                    h60
                                }
                            });
                            append_bytes(a61, a62)
                        };
                        append_bytes(a63, a64)
                    };
                    append_bytes(a65, a66)
                };
                append_bytes(a67, a68)
            };
            append_bytes(a69, a70)
        };
        append_bytes(a71, a72)
    };
    append_bytes(a73, a74)
}

pub fn f11(v0: Bytes) -> R<Adt2> {
    let v1: u64 = {
        let a75 = v0.clone();
        length_bytes(a75)
    };
    let v2: u64 = {
        let a81 = {
            let a78 = {
                let a76 = v1;
                let a77 = 72u64;
                nat_add(a76, a77)?
            };
            let a79 = 64u64;
            let a80 = 0u64;
            nat_quot(a78, a79, a80)
        };
        let a82 = 64u64;
        nat_mul(a81, a82)?
    };
    let (_, h83) = f18(Fn0::F19(v0, v1, v2), {
        let a84 = v2;
        let a85 = 64u64;
        let a86 = 0u64;
        nat_quot(a84, a85, a86)
    }, (0u64, f20()))?;
    Ok(h83)
}

pub fn f12(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt3 = f21(v0, v1)?;
    let a95 = f22(match v2.clone() {
        Adt3::C0(h87, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h87
        }
    });
    let a96 = {
        let a93 = f22(match v2.clone() {
            Adt3::C0(_, h88, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h88
            }
        });
        let a94 = {
            let a91 = f22(match v2.clone() {
                Adt3::C0(_, _, h89, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h89
                }
            });
            let a92 = f22(match v2 {
                Adt3::C0(_, _, _, h90, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h90
                }
            });
            append_bytes(a91, a92)
        };
        append_bytes(a93, a94)
    };
    Ok(append_bytes(a95, a96))
}

pub fn f13(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a97 = v0.clone();
        length_bytes(a97)
    };
    let v2: u64 = {
        let a103 = {
            let a100 = {
                let a98 = v1;
                let a99 = 144u64;
                nat_add(a98, a99)?
            };
            let a101 = 128u64;
            let a102 = 0u64;
            nat_quot(a100, a101, a102)
        };
        let a104 = 128u64;
        nat_mul(a103, a104)?
    };
    let (_, h105) = f23(Fn1::F24(v0, v1, v2), {
        let a106 = v2;
        let a107 = 128u64;
        let a108 = 0u64;
        nat_quot(a106, a107, a108)
    }, (0u64, f25()))?;
    Ok(h105)
}

pub fn f14(v0: u64) -> Bytes {
    let a137 = f26(v0, 60u32);
    let a138 = {
        let a135 = f26(v0, 56u32);
        let a136 = {
            let a133 = f26(v0, 52u32);
            let a134 = {
                let a131 = f26(v0, 48u32);
                let a132 = {
                    let a129 = f26(v0, 44u32);
                    let a130 = {
                        let a127 = f26(v0, 40u32);
                        let a128 = {
                            let a125 = f26(v0, 36u32);
                            let a126 = {
                                let a123 = f26(v0, 32u32);
                                let a124 = {
                                    let a121 = f26(v0, 28u32);
                                    let a122 = {
                                        let a119 = f26(v0, 24u32);
                                        let a120 = {
                                            let a117 = f26(v0, 20u32);
                                            let a118 = {
                                                let a115 = f26(v0, 16u32);
                                                let a116 = {
                                                    let a113 = f26(v0, 12u32);
                                                    let a114 = {
                                                        let a111 = f26(v0, 8u32);
                                                        let a112 = {
                                                            let a109 = f26(v0, 4u32);
                                                            let a110 = f26(v0, 0u32);
                                                            append_bytes(a109, a110)
                                                        };
                                                        append_bytes(a111, a112)
                                                    };
                                                    append_bytes(a113, a114)
                                                };
                                                append_bytes(a115, a116)
                                            };
                                            append_bytes(a117, a118)
                                        };
                                        append_bytes(a119, a120)
                                    };
                                    append_bytes(a121, a122)
                                };
                                append_bytes(a123, a124)
                            };
                            append_bytes(a125, a126)
                        };
                        append_bytes(a127, a128)
                    };
                    append_bytes(a129, a130)
                };
                append_bytes(a131, a132)
            };
            append_bytes(a133, a134)
        };
        append_bytes(a135, a136)
    };
    append_bytes(a137, a138)
}

pub fn f15(v0: Bytes) -> R<Adt1> {
    let v1: u64 = {
        let a139 = v0.clone();
        length_bytes(a139)
    };
    let v2: u64 = {
        let a140 = v1;
        let a141 = 0u64;
        if nat_eq(a140, a141) {
            1u64
        } else {
            let a144 = {
                let a142 = v1;
                let a143 = 1023u64;
                nat_add(a142, a143)?
            };
            let a145 = 1024u64;
            let a146 = 0u64;
            nat_quot(a144, a145, a146)
        }
    };
    let v3: u64 = {
        let a147 = v2;
        let a148 = 1u64;
        nat_sub(a147, a148)
    };
    let m150 = {
        let (_, h149) = f27(v0.clone(), v1, v3)?;
        h149
    };
    match m150.uncons() {
        None => {
            f28(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h151) = f29(Fn2::F30, 64u64, (List::cons(v4, v5), f28(v0, v1, v3, false)?));
            Ok(h151)
        }
    }
}

pub fn f16(v0: u32) -> Bytes {
    let a164 = {
        let a152 = f31(v0, 4u32);
        let a153 = f31(v0, 0u32);
        append_bytes(a152, a153)
    };
    let a165 = {
        let a162 = {
            let a154 = f31(v0, 12u32);
            let a155 = f31(v0, 8u32);
            append_bytes(a154, a155)
        };
        let a163 = {
            let a160 = {
                let a156 = f31(v0, 20u32);
                let a157 = f31(v0, 16u32);
                append_bytes(a156, a157)
            };
            let a161 = {
                let a158 = f31(v0, 28u32);
                let a159 = f31(v0, 24u32);
                append_bytes(a158, a159)
            };
            append_bytes(a160, a161)
        };
        append_bytes(a162, a163)
    };
    append_bytes(a164, a165)
}

pub fn f17(v0: u32) -> Bytes {
    let a194 = f32({
        let a166 = f33(v0, 28u32);
        let a167 = 15u32;
        fixed_u32::bit_and(a166, a167)
    });
    let a195 = {
        let a192 = f32({
            let a168 = f33(v0, 24u32);
            let a169 = 15u32;
            fixed_u32::bit_and(a168, a169)
        });
        let a193 = {
            let a190 = f32({
                let a170 = f33(v0, 20u32);
                let a171 = 15u32;
                fixed_u32::bit_and(a170, a171)
            });
            let a191 = {
                let a188 = f32({
                    let a172 = f33(v0, 16u32);
                    let a173 = 15u32;
                    fixed_u32::bit_and(a172, a173)
                });
                let a189 = {
                    let a186 = f32({
                        let a174 = f33(v0, 12u32);
                        let a175 = 15u32;
                        fixed_u32::bit_and(a174, a175)
                    });
                    let a187 = {
                        let a184 = f32({
                            let a176 = f33(v0, 8u32);
                            let a177 = 15u32;
                            fixed_u32::bit_and(a176, a177)
                        });
                        let a185 = {
                            let a182 = f32({
                                let a178 = f33(v0, 4u32);
                                let a179 = 15u32;
                                fixed_u32::bit_and(a178, a179)
                            });
                            let a183 = f32({
                                let a180 = v0;
                                let a181 = 15u32;
                                fixed_u32::bit_and(a180, a181)
                            });
                            append_bytes(a182, a183)
                        };
                        append_bytes(a184, a185)
                    };
                    append_bytes(a186, a187)
                };
                append_bytes(a188, a189)
            };
            append_bytes(a190, a191)
        };
        append_bytes(a192, a193)
    };
    append_bytes(a194, a195)
}

pub fn f18(v0: Fn0, p199: u64, p200: (u64, Adt2)) -> R<(u64, Adt2)> {
    let m198 = (p199, p200);
    {
        let mut m198 = m198;
        loop {
            let step = {
                let (v1, v2) = m198;
                let m196 = v1;
                if m196 == 0 {
                    Err::<(u64, (u64, Adt2)), R<(u64, Adt2)>>(Ok(v2))
                } else {
                    let v3 = m196 - 1;
                    Ok::<(u64, (u64, Adt2)), R<(u64, Adt2)>>((v3, {
                        let c197: Fn0 = v0.clone();
                        c197.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt2)) -> R<(u64, Adt2)> {
    Ok(({
        let a202 = {
            let (h201, _) = v3.clone();
            h201
        };
        let a203 = 64u64;
        nat_add(a202, a203)?
    }, f34({
        let (_, h204) = v3;
        h204
    }, f35(v0, v1, v2, {
        let (h205, _) = v3;
        h205
    })?)?))
}

pub fn f20() -> Adt2 {
    Adt2::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f21(v0: Bytes, v1: u64) -> R<Adt3> {
    let v2: u64 = {
        let a206 = v0.clone();
        length_bytes(a206)
    };
    let v3: u64 = {
        let a212 = {
            let a210 = {
                let a207 = v2;
                let a208 = 136u64;
                let a209 = 0u64;
                nat_quot(a207, a208, a209)
            };
            let a211 = 1u64;
            nat_add(a210, a211)?
        };
        let a213 = 136u64;
        nat_mul(a212, a213)?
    };
    let (_, h214) = f36(Fn3::F37(v0, v2, v1, v3), {
        let a215 = v3;
        let a216 = 136u64;
        let a217 = 0u64;
        nat_quot(a215, a216, a217)
    }, (0u64, f38()))?;
    Ok(h214)
}

pub fn f22(v0: u64) -> Bytes {
    let a246 = {
        let a218 = f26(v0, 4u32);
        let a219 = f26(v0, 0u32);
        append_bytes(a218, a219)
    };
    let a247 = {
        let a244 = {
            let a220 = f26(v0, 12u32);
            let a221 = f26(v0, 8u32);
            append_bytes(a220, a221)
        };
        let a245 = {
            let a242 = {
                let a222 = f26(v0, 20u32);
                let a223 = f26(v0, 16u32);
                append_bytes(a222, a223)
            };
            let a243 = {
                let a240 = {
                    let a224 = f26(v0, 28u32);
                    let a225 = f26(v0, 24u32);
                    append_bytes(a224, a225)
                };
                let a241 = {
                    let a238 = {
                        let a226 = f26(v0, 36u32);
                        let a227 = f26(v0, 32u32);
                        append_bytes(a226, a227)
                    };
                    let a239 = {
                        let a236 = {
                            let a228 = f26(v0, 44u32);
                            let a229 = f26(v0, 40u32);
                            append_bytes(a228, a229)
                        };
                        let a237 = {
                            let a234 = {
                                let a230 = f26(v0, 52u32);
                                let a231 = f26(v0, 48u32);
                                append_bytes(a230, a231)
                            };
                            let a235 = {
                                let a232 = f26(v0, 60u32);
                                let a233 = f26(v0, 56u32);
                                append_bytes(a232, a233)
                            };
                            append_bytes(a234, a235)
                        };
                        append_bytes(a236, a237)
                    };
                    append_bytes(a238, a239)
                };
                append_bytes(a240, a241)
            };
            append_bytes(a242, a243)
        };
        append_bytes(a244, a245)
    };
    append_bytes(a246, a247)
}

pub fn f23(v0: Fn1, p251: u64, p252: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m250 = (p251, p252);
    {
        let mut m250 = m250;
        loop {
            let step = {
                let (v1, v2) = m250;
                let m248 = v1;
                if m248 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m248 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c249: Fn1 = v0.clone();
                        c249.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a254 = {
            let (h253, _) = v3.clone();
            h253
        };
        let a255 = 128u64;
        nat_add(a254, a255)?
    }, f39({
        let (_, h256) = v3;
        h256
    }, f40(v0, v1, v2, {
        let (h257, _) = v3;
        h257
    })?)?))
}

pub fn f25() -> Adt0 {
    Adt0::C0(7640891576956012808u64, 13503953896175478587u64, 4354685564936845355u64, 11912009170470909681u64, 5840696475078001361u64, 11170449401992604703u64, 2270897969802886507u64, 6620516959819538809u64)
}

pub fn f26(v0: u64, v1: u32) -> Bytes {
    f32(f41(f42(v0, v1)))
}

pub fn f27(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt1>)> {
    f43(Fn4::F44(v0, v1), v2, (0u64, List::<Adt1>::nil()))
}

pub fn f28(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt1> {
    let v4: u64 = {
        let a258 = 1024u64;
        let a259 = v2;
        nat_mul(a258, a259)?
    };
    let v5: u64 = f45(v1, v4);
    let v6: u64 = f46(v5)?;
    let v7: u64 = {
        let a260 = v6;
        let a261 = 1u64;
        nat_sub(a260, a261)
    };
    Ok(f47(f48(v0.clone(), v4, v2, v7)?, f49(v0, {
        let a264 = v4;
        let a265 = {
            let a262 = 64u64;
            let a263 = v7;
            nat_mul(a262, a263)?
        };
        nat_add(a264, a265)?
    })?, v2, f50({
        let a268 = v5;
        let a269 = {
            let a266 = 64u64;
            let a267 = v7;
            nat_mul(a266, a267)?
        };
        nat_sub(a268, a269)
    }), {
        let a272 = {
            let a270 = v7;
            let a271 = 0u64;
            if nat_eq(a270, a271) {
                3u32
            } else {
                2u32
            }
        };
        let a273 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a272, a273)
    }))
}

pub fn f29(v0: Fn2, p277: u64, p278: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m274 = v1;
                if m274 == 0 {
                    Err::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>(v2)
                } else {
                    let v3 = m274 - 1;
                    Ok::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>((v3, {
                        let c275: Fn2 = v0.clone();
                        c275.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    f51(v0)
}

pub fn f31(v0: u32, v1: u32) -> Bytes {
    f32(f33(v0, v1))
}

pub fn f32(v0: u32) -> Bytes {
    let a281 = {
        let a279 = v0;
        let a280 = 8u32;
        fixed_u32::bit_and(a279, a280)
    };
    let a282 = 0u32;
    if equal(a281, a282) {
        let a285 = {
            let a283 = v0;
            let a284 = 4u32;
            fixed_u32::bit_and(a283, a284)
        };
        let a286 = 0u32;
        if equal(a285, a286) {
            let a289 = {
                let a287 = v0;
                let a288 = 2u32;
                fixed_u32::bit_and(a287, a288)
            };
            let a290 = 0u32;
            if equal(a289, a290) {
                let a293 = {
                    let a291 = v0;
                    let a292 = 1u32;
                    fixed_u32::bit_and(a291, a292)
                };
                let a294 = 0u32;
                if equal(a293, a294) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a297 = {
                    let a295 = v0;
                    let a296 = 1u32;
                    fixed_u32::bit_and(a295, a296)
                };
                let a298 = 0u32;
                if equal(a297, a298) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a301 = {
                let a299 = v0;
                let a300 = 2u32;
                fixed_u32::bit_and(a299, a300)
            };
            let a302 = 0u32;
            if equal(a301, a302) {
                let a305 = {
                    let a303 = v0;
                    let a304 = 1u32;
                    fixed_u32::bit_and(a303, a304)
                };
                let a306 = 0u32;
                if equal(a305, a306) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a309 = {
                    let a307 = v0;
                    let a308 = 1u32;
                    fixed_u32::bit_and(a307, a308)
                };
                let a310 = 0u32;
                if equal(a309, a310) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a313 = {
            let a311 = v0;
            let a312 = 4u32;
            fixed_u32::bit_and(a311, a312)
        };
        let a314 = 0u32;
        if equal(a313, a314) {
            let a317 = {
                let a315 = v0;
                let a316 = 2u32;
                fixed_u32::bit_and(a315, a316)
            };
            let a318 = 0u32;
            if equal(a317, a318) {
                let a321 = {
                    let a319 = v0;
                    let a320 = 1u32;
                    fixed_u32::bit_and(a319, a320)
                };
                let a322 = 0u32;
                if equal(a321, a322) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a325 = {
                    let a323 = v0;
                    let a324 = 1u32;
                    fixed_u32::bit_and(a323, a324)
                };
                let a326 = 0u32;
                if equal(a325, a326) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a329 = {
                let a327 = v0;
                let a328 = 2u32;
                fixed_u32::bit_and(a327, a328)
            };
            let a330 = 0u32;
            if equal(a329, a330) {
                let a333 = {
                    let a331 = v0;
                    let a332 = 1u32;
                    fixed_u32::bit_and(a331, a332)
                };
                let a334 = 0u32;
                if equal(a333, a334) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a337 = {
                    let a335 = v0;
                    let a336 = 1u32;
                    fixed_u32::bit_and(a335, a336)
                };
                let a338 = 0u32;
                if equal(a337, a338) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f33(v0: u32, v1: u32) -> u32 {
    let m341 = {
        let a339 = v0;
        let a340 = v1;
        fixed_u32::shift_right(a339, a340)
    };
    match m341 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f34(v0: Adt2, v1: Adt4) -> R<Adt2> {
    let v2: Adt5 = f52(Fn5::F53, 64u64, Adt5::C0(match v0.clone() {
        Adt2::C0(h342, _, _, _, _, _, _, _) => {
            h342
        }
    }, match v0.clone() {
        Adt2::C0(_, h343, _, _, _, _, _, _) => {
            h343
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h344, _, _, _, _, _) => {
            h344
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h345, _, _, _, _) => {
            h345
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h346, _, _, _) => {
            h346
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h347, _, _) => {
            h347
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h348, _) => {
            h348
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h349) => {
            h349
        }
    }, v1, 0u64))?;
    Ok(Adt2::C0(f54(match v0.clone() {
        Adt2::C0(h350, _, _, _, _, _, _, _) => {
            h350
        }
    }, match v2.clone() {
        Adt5::C0(h351, _, _, _, _, _, _, _, _, _) => {
            h351
        }
    }), f54(match v0.clone() {
        Adt2::C0(_, h352, _, _, _, _, _, _) => {
            h352
        }
    }, match v2.clone() {
        Adt5::C0(_, h353, _, _, _, _, _, _, _, _) => {
            h353
        }
    }), f54(match v0.clone() {
        Adt2::C0(_, _, h354, _, _, _, _, _) => {
            h354
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h355, _, _, _, _, _, _, _) => {
            h355
        }
    }), f54(match v0.clone() {
        Adt2::C0(_, _, _, h356, _, _, _, _) => {
            h356
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, h357, _, _, _, _, _, _) => {
            h357
        }
    }), f54(match v0.clone() {
        Adt2::C0(_, _, _, _, h358, _, _, _) => {
            h358
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, h359, _, _, _, _, _) => {
            h359
        }
    }), f54(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h360, _, _) => {
            h360
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, h361, _, _, _, _) => {
            h361
        }
    }), f54(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h362, _) => {
            h362
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h363, _, _, _) => {
            h363
        }
    }), f54(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h364) => {
            h364
        }
    }, match v2 {
        Adt5::C0(_, _, _, _, _, _, _, h365, _, _) => {
            h365
        }
    })))
}

pub fn f35(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    Ok(Adt4::C0(f55(v0.clone(), v1, v2, v3)?, f55(v0.clone(), v1, v2, {
        let a366 = v3;
        let a367 = 4u64;
        nat_add(a366, a367)?
    })?, f55(v0.clone(), v1, v2, {
        let a368 = v3;
        let a369 = 8u64;
        nat_add(a368, a369)?
    })?, f55(v0.clone(), v1, v2, {
        let a370 = v3;
        let a371 = 12u64;
        nat_add(a370, a371)?
    })?, f55(v0.clone(), v1, v2, {
        let a372 = v3;
        let a373 = 16u64;
        nat_add(a372, a373)?
    })?, f55(v0.clone(), v1, v2, {
        let a374 = v3;
        let a375 = 20u64;
        nat_add(a374, a375)?
    })?, f55(v0.clone(), v1, v2, {
        let a376 = v3;
        let a377 = 24u64;
        nat_add(a376, a377)?
    })?, f55(v0.clone(), v1, v2, {
        let a378 = v3;
        let a379 = 28u64;
        nat_add(a378, a379)?
    })?, f55(v0.clone(), v1, v2, {
        let a380 = v3;
        let a381 = 32u64;
        nat_add(a380, a381)?
    })?, f55(v0.clone(), v1, v2, {
        let a382 = v3;
        let a383 = 36u64;
        nat_add(a382, a383)?
    })?, f55(v0.clone(), v1, v2, {
        let a384 = v3;
        let a385 = 40u64;
        nat_add(a384, a385)?
    })?, f55(v0.clone(), v1, v2, {
        let a386 = v3;
        let a387 = 44u64;
        nat_add(a386, a387)?
    })?, f55(v0.clone(), v1, v2, {
        let a388 = v3;
        let a389 = 48u64;
        nat_add(a388, a389)?
    })?, f55(v0.clone(), v1, v2, {
        let a390 = v3;
        let a391 = 52u64;
        nat_add(a390, a391)?
    })?, f55(v0.clone(), v1, v2, {
        let a392 = v3;
        let a393 = 56u64;
        nat_add(a392, a393)?
    })?, f55(v0, v1, v2, {
        let a394 = v3;
        let a395 = 60u64;
        nat_add(a394, a395)?
    })?))
}

pub fn f36(v0: Fn3, p399: u64, p400: (u64, Adt3)) -> R<(u64, Adt3)> {
    let m398 = (p399, p400);
    {
        let mut m398 = m398;
        loop {
            let step = {
                let (v1, v2) = m398;
                let m396 = v1;
                if m396 == 0 {
                    Err::<(u64, (u64, Adt3)), R<(u64, Adt3)>>(Ok(v2))
                } else {
                    let v3 = m396 - 1;
                    Ok::<(u64, (u64, Adt3)), R<(u64, Adt3)>>((v3, {
                        let c397: Fn3 = v0.clone();
                        c397.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a402 = {
            let (h401, _) = v4.clone();
            h401
        };
        let a403 = 136u64;
        nat_add(a402, a403)?
    }, f56(v0, v1, v3, v2, {
        let (_, h404) = v4;
        h404
    }, {
        let (h405, _) = v4;
        h405
    })?))
}

pub fn f38() -> Adt3 {
    Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f39(v0: Adt0, v1: Adt6) -> R<Adt0> {
    let v2: Adt7 = f57(Fn6::F58, 80u64, Adt7::C0(match v0.clone() {
        Adt0::C0(h406, _, _, _, _, _, _, _) => {
            h406
        }
    }, match v0.clone() {
        Adt0::C0(_, h407, _, _, _, _, _, _) => {
            h407
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h408, _, _, _, _, _) => {
            h408
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h409, _, _, _, _) => {
            h409
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h410, _, _, _) => {
            h410
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h411, _, _) => {
            h411
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h412, _) => {
            h412
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h413) => {
            h413
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f59(match v0.clone() {
        Adt0::C0(h414, _, _, _, _, _, _, _) => {
            h414
        }
    }, match v2.clone() {
        Adt7::C0(h415, _, _, _, _, _, _, _, _, _) => {
            h415
        }
    }), f59(match v0.clone() {
        Adt0::C0(_, h416, _, _, _, _, _, _) => {
            h416
        }
    }, match v2.clone() {
        Adt7::C0(_, h417, _, _, _, _, _, _, _, _) => {
            h417
        }
    }), f59(match v0.clone() {
        Adt0::C0(_, _, h418, _, _, _, _, _) => {
            h418
        }
    }, match v2.clone() {
        Adt7::C0(_, _, h419, _, _, _, _, _, _, _) => {
            h419
        }
    }), f59(match v0.clone() {
        Adt0::C0(_, _, _, h420, _, _, _, _) => {
            h420
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, h421, _, _, _, _, _, _) => {
            h421
        }
    }), f59(match v0.clone() {
        Adt0::C0(_, _, _, _, h422, _, _, _) => {
            h422
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, h423, _, _, _, _, _) => {
            h423
        }
    }), f59(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h424, _, _) => {
            h424
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, h425, _, _, _, _) => {
            h425
        }
    }), f59(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h426, _) => {
            h426
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, _, h427, _, _, _) => {
            h427
        }
    }), f59(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h428) => {
            h428
        }
    }, match v2 {
        Adt7::C0(_, _, _, _, _, _, _, h429, _, _) => {
            h429
        }
    })))
}

pub fn f40(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    Ok(Adt6::C0(f60(v0.clone(), v1, v2, {
        let a430 = v3;
        let a431 = 0u64;
        nat_add(a430, a431)?
    })?, f60(v0.clone(), v1, v2, {
        let a432 = v3;
        let a433 = 8u64;
        nat_add(a432, a433)?
    })?, f60(v0.clone(), v1, v2, {
        let a434 = v3;
        let a435 = 16u64;
        nat_add(a434, a435)?
    })?, f60(v0.clone(), v1, v2, {
        let a436 = v3;
        let a437 = 24u64;
        nat_add(a436, a437)?
    })?, f60(v0.clone(), v1, v2, {
        let a438 = v3;
        let a439 = 32u64;
        nat_add(a438, a439)?
    })?, f60(v0.clone(), v1, v2, {
        let a440 = v3;
        let a441 = 40u64;
        nat_add(a440, a441)?
    })?, f60(v0.clone(), v1, v2, {
        let a442 = v3;
        let a443 = 48u64;
        nat_add(a442, a443)?
    })?, f60(v0.clone(), v1, v2, {
        let a444 = v3;
        let a445 = 56u64;
        nat_add(a444, a445)?
    })?, f60(v0.clone(), v1, v2, {
        let a446 = v3;
        let a447 = 64u64;
        nat_add(a446, a447)?
    })?, f60(v0.clone(), v1, v2, {
        let a448 = v3;
        let a449 = 72u64;
        nat_add(a448, a449)?
    })?, f60(v0.clone(), v1, v2, {
        let a450 = v3;
        let a451 = 80u64;
        nat_add(a450, a451)?
    })?, f60(v0.clone(), v1, v2, {
        let a452 = v3;
        let a453 = 88u64;
        nat_add(a452, a453)?
    })?, f60(v0.clone(), v1, v2, {
        let a454 = v3;
        let a455 = 96u64;
        nat_add(a454, a455)?
    })?, f60(v0.clone(), v1, v2, {
        let a456 = v3;
        let a457 = 104u64;
        nat_add(a456, a457)?
    })?, f60(v0.clone(), v1, v2, {
        let a458 = v3;
        let a459 = 112u64;
        nat_add(a458, a459)?
    })?, f60(v0, v1, v2, {
        let a460 = v3;
        let a461 = 120u64;
        nat_add(a460, a461)?
    })?))
}

pub fn f41(v0: u64) -> u32 {
    let m465 = {
        let a464 = {
            let a462 = v0;
            let a463 = 4294967295u64;
            fixed_u64::bit_and(a462, a463)
        };
        fixed_u32::convert(i128::from(a464))
    };
    match m465 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f42(v0: u64, v1: u32) -> u64 {
    let m468 = {
        let a466 = v0;
        let a467 = v1;
        fixed_u64::shift_right(a466, a467)
    };
    match m468 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f43(v0: Fn4, p472: u64, p473: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    let m471 = (p472, p473);
    {
        let mut m471 = m471;
        loop {
            let step = {
                let (v1, v2) = m471;
                let m469 = v1;
                if m469 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>(Ok(v2))
                } else {
                    let v3 = m469 - 1;
                    Ok::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>((v3, {
                        let c470: Fn4 = v0.clone();
                        c470.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: u64, v2: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    f61(v2.clone(), f28(v0, v1, {
        let (h474, _) = v2;
        h474
    }, false)?)
}

pub fn f45(v0: u64, v1: u64) -> u64 {
    let a477 = 1024u64;
    let a478 = {
        let a475 = v0;
        let a476 = v1;
        nat_sub(a475, a476)
    };
    if nat_lt(a477, a478) {
        1024u64
    } else {
        let a479 = v0;
        let a480 = v1;
        nat_sub(a479, a480)
    }
}

pub fn f46(v0: u64) -> R<u64> {
    let a481 = v0;
    let a482 = 0u64;
    if nat_eq(a481, a482) {
        Ok(1u64)
    } else {
        let a485 = {
            let a483 = v0;
            let a484 = 63u64;
            nat_add(a483, a484)?
        };
        let a486 = 64u64;
        let a487 = 0u64;
        Ok(nat_quot(a485, a486, a487))
    }
}

pub fn f47(v0: Adt1, v1: Adt8, v2: u64, v3: u32, v4: u32) -> Adt1 {
    let v5: Adt9 = {
        let (h488, _) = f62(Fn7::F63, 7u64, (Adt9::C0(match v0.clone() {
            Adt1::C0(h489, _, _, _, _, _, _, _) => {
                h489
            }
        }, match v0.clone() {
            Adt1::C0(_, h490, _, _, _, _, _, _) => {
                h490
            }
        }, match v0.clone() {
            Adt1::C0(_, _, h491, _, _, _, _, _) => {
                h491
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h492, _, _, _, _) => {
                h492
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h493, _, _, _) => {
                h493
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h494, _, _) => {
                h494
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h495, _) => {
                h495
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, h496) => {
                h496
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f50({
            let a497 = v2;
            let a498 = 4294967296u64;
            let a499 = 0u64;
            nat_rem(a497, a498, a499)
        }), f50({
            let a500 = v2;
            let a501 = 4294967296u64;
            let a502 = 0u64;
            nat_quot(a500, a501, a502)
        }), v3, v4), v1));
        h488
    };
    Adt1::C0({
        let a505 = match v5.clone() {
            Adt9::C0(h503, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h503
            }
        };
        let a506 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, h504, _, _, _, _, _, _, _) => {
                h504
            }
        };
        fixed_u32::bit_xor(a505, a506)
    }, {
        let a509 = match v5.clone() {
            Adt9::C0(_, h507, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h507
            }
        };
        let a510 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, h508, _, _, _, _, _, _) => {
                h508
            }
        };
        fixed_u32::bit_xor(a509, a510)
    }, {
        let a513 = match v5.clone() {
            Adt9::C0(_, _, h511, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h511
            }
        };
        let a514 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, h512, _, _, _, _, _) => {
                h512
            }
        };
        fixed_u32::bit_xor(a513, a514)
    }, {
        let a517 = match v5.clone() {
            Adt9::C0(_, _, _, h515, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h515
            }
        };
        let a518 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h516, _, _, _, _) => {
                h516
            }
        };
        fixed_u32::bit_xor(a517, a518)
    }, {
        let a521 = match v5.clone() {
            Adt9::C0(_, _, _, _, h519, _, _, _, _, _, _, _, _, _, _, _) => {
                h519
            }
        };
        let a522 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h520, _, _, _) => {
                h520
            }
        };
        fixed_u32::bit_xor(a521, a522)
    }, {
        let a525 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, h523, _, _, _, _, _, _, _, _, _, _) => {
                h523
            }
        };
        let a526 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h524, _, _) => {
                h524
            }
        };
        fixed_u32::bit_xor(a525, a526)
    }, {
        let a529 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, h527, _, _, _, _, _, _, _, _, _) => {
                h527
            }
        };
        let a530 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h528, _) => {
                h528
            }
        };
        fixed_u32::bit_xor(a529, a530)
    }, {
        let a533 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, h531, _, _, _, _, _, _, _, _) => {
                h531
            }
        };
        let a534 = match v5 {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h532) => {
                h532
            }
        };
        fixed_u32::bit_xor(a533, a534)
    })
}

pub fn f48(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (_, h535) = f64(Fn8::F65(v2, v0, v1), v3, (0u64, f66()))?;
    Ok(h535)
}

pub fn f49(v0: Bytes, v1: u64) -> R<Adt8> {
    Ok(Adt8::C0(f67(v0.clone(), {
        let a536 = v1;
        let a537 = 0u64;
        nat_add(a536, a537)?
    })?, f67(v0.clone(), {
        let a538 = v1;
        let a539 = 4u64;
        nat_add(a538, a539)?
    })?, f67(v0.clone(), {
        let a540 = v1;
        let a541 = 8u64;
        nat_add(a540, a541)?
    })?, f67(v0.clone(), {
        let a542 = v1;
        let a543 = 12u64;
        nat_add(a542, a543)?
    })?, f67(v0.clone(), {
        let a544 = v1;
        let a545 = 16u64;
        nat_add(a544, a545)?
    })?, f67(v0.clone(), {
        let a546 = v1;
        let a547 = 20u64;
        nat_add(a546, a547)?
    })?, f67(v0.clone(), {
        let a548 = v1;
        let a549 = 24u64;
        nat_add(a548, a549)?
    })?, f67(v0.clone(), {
        let a550 = v1;
        let a551 = 28u64;
        nat_add(a550, a551)?
    })?, f67(v0.clone(), {
        let a552 = v1;
        let a553 = 32u64;
        nat_add(a552, a553)?
    })?, f67(v0.clone(), {
        let a554 = v1;
        let a555 = 36u64;
        nat_add(a554, a555)?
    })?, f67(v0.clone(), {
        let a556 = v1;
        let a557 = 40u64;
        nat_add(a556, a557)?
    })?, f67(v0.clone(), {
        let a558 = v1;
        let a559 = 44u64;
        nat_add(a558, a559)?
    })?, f67(v0.clone(), {
        let a560 = v1;
        let a561 = 48u64;
        nat_add(a560, a561)?
    })?, f67(v0.clone(), {
        let a562 = v1;
        let a563 = 52u64;
        nat_add(a562, a563)?
    })?, f67(v0.clone(), {
        let a564 = v1;
        let a565 = 56u64;
        nat_add(a564, a565)?
    })?, f67(v0, {
        let a566 = v1;
        let a567 = 60u64;
        nat_add(a566, a567)?
    })?))
}

pub fn f50(v0: u64) -> u32 {
    let (_, h568) = {
        let (_, h569) = f68(Fn9::F69, 32u64, (v0, (1u32, 0u32)));
        h569
    };
    h568
}

pub fn f51(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m571 = {
        let (h570, _) = v0.clone();
        h570
    };
    match m571.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f70(v1, {
                let (_, h572) = v0;
                h572
            }, {
                let m573 = v2;
                match m573.uncons() {
                    None => {
                        true
                    }
                    Some((_, _)) => {
                        false
                    }
                }
            }))
        }
    }
}

pub fn f52(v0: Fn5, p577: u64, p578: Adt5) -> R<Adt5> {
    let m576 = (p577, p578);
    {
        let mut m576 = m576;
        loop {
            let step = {
                let (v1, v2) = m576;
                let m574 = v1;
                if m574 == 0 {
                    Err::<(u64, Adt5), R<Adt5>>(Ok(v2))
                } else {
                    let v3 = m574 - 1;
                    Ok::<(u64, Adt5), R<Adt5>>((v3, {
                        let c575: Fn5 = v0.clone();
                        c575.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: Adt5) -> R<Adt5> {
    f71(v0)
}

pub fn f54(v0: u32, v1: u32) -> u32 {
    let m581 = {
        let a579 = f72(v0);
        let a580 = f72(v1);
        fixed_u64::checked_add(a579, a580)
    };
    match m581 {
        None => {
            0u32
        }
        Some(v2) => {
            f41(v2)
        }
    }
}

pub fn f55(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a584 = {
        let a582 = v3;
        let a583 = 8u64;
        nat_add(a582, a583)?
    };
    let a585 = v2;
    if nat_eq(a584, a585) {
        Ok(f50({
            let a588 = {
                let a586 = v1;
                let a587 = 8u64;
                nat_mul(a586, a587)?
            };
            let a589 = 4294967296u64;
            let a590 = 0u64;
            nat_quot(a588, a589, a590)
        }))
    } else {
        let a593 = {
            let a591 = v3;
            let a592 = 4u64;
            nat_add(a591, a592)?
        };
        let a594 = v2;
        if nat_eq(a593, a594) {
            Ok(f50({
                let a597 = {
                    let a595 = v1;
                    let a596 = 8u64;
                    nat_mul(a595, a596)?
                };
                let a598 = 4294967296u64;
                let a599 = 0u64;
                nat_rem(a597, a598, a599)
            }))
        } else {
            Ok(f73(f74(v0.clone(), v1, v3), f74(v0.clone(), v1, {
                let a600 = v3;
                let a601 = 1u64;
                nat_add(a600, a601)?
            }), f74(v0.clone(), v1, {
                let a602 = v3;
                let a603 = 2u64;
                nat_add(a602, a603)?
            }), f74(v0, v1, {
                let a604 = v3;
                let a605 = 3u64;
                nat_add(a604, a605)?
            })))
        }
    }
}

pub fn f56(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: u64) -> R<Adt3> {
    f75(Adt3::C0({
        let a609 = match v4.clone() {
            Adt3::C0(h606, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h606
            }
        };
        let a610 = f76(v0.clone(), v1, v2, v3, {
            let a607 = v5;
            let a608 = 0u64;
            nat_add(a607, a608)?
        })?;
        fixed_u64::bit_xor(a609, a610)
    }, {
        let a614 = match v4.clone() {
            Adt3::C0(_, h611, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h611
            }
        };
        let a615 = f76(v0.clone(), v1, v2, v3, {
            let a612 = v5;
            let a613 = 8u64;
            nat_add(a612, a613)?
        })?;
        fixed_u64::bit_xor(a614, a615)
    }, {
        let a619 = match v4.clone() {
            Adt3::C0(_, _, h616, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h616
            }
        };
        let a620 = f76(v0.clone(), v1, v2, v3, {
            let a617 = v5;
            let a618 = 16u64;
            nat_add(a617, a618)?
        })?;
        fixed_u64::bit_xor(a619, a620)
    }, {
        let a624 = match v4.clone() {
            Adt3::C0(_, _, _, h621, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h621
            }
        };
        let a625 = f76(v0.clone(), v1, v2, v3, {
            let a622 = v5;
            let a623 = 24u64;
            nat_add(a622, a623)?
        })?;
        fixed_u64::bit_xor(a624, a625)
    }, {
        let a629 = match v4.clone() {
            Adt3::C0(_, _, _, _, h626, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h626
            }
        };
        let a630 = f76(v0.clone(), v1, v2, v3, {
            let a627 = v5;
            let a628 = 32u64;
            nat_add(a627, a628)?
        })?;
        fixed_u64::bit_xor(a629, a630)
    }, {
        let a634 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, h631, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h631
            }
        };
        let a635 = f76(v0.clone(), v1, v2, v3, {
            let a632 = v5;
            let a633 = 40u64;
            nat_add(a632, a633)?
        })?;
        fixed_u64::bit_xor(a634, a635)
    }, {
        let a639 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, h636, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h636
            }
        };
        let a640 = f76(v0.clone(), v1, v2, v3, {
            let a637 = v5;
            let a638 = 48u64;
            nat_add(a637, a638)?
        })?;
        fixed_u64::bit_xor(a639, a640)
    }, {
        let a644 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h641, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h641
            }
        };
        let a645 = f76(v0.clone(), v1, v2, v3, {
            let a642 = v5;
            let a643 = 56u64;
            nat_add(a642, a643)?
        })?;
        fixed_u64::bit_xor(a644, a645)
    }, {
        let a649 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h646, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h646
            }
        };
        let a650 = f76(v0.clone(), v1, v2, v3, {
            let a647 = v5;
            let a648 = 64u64;
            nat_add(a647, a648)?
        })?;
        fixed_u64::bit_xor(a649, a650)
    }, {
        let a654 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h651, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h651
            }
        };
        let a655 = f76(v0.clone(), v1, v2, v3, {
            let a652 = v5;
            let a653 = 72u64;
            nat_add(a652, a653)?
        })?;
        fixed_u64::bit_xor(a654, a655)
    }, {
        let a659 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h656, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h656
            }
        };
        let a660 = f76(v0.clone(), v1, v2, v3, {
            let a657 = v5;
            let a658 = 80u64;
            nat_add(a657, a658)?
        })?;
        fixed_u64::bit_xor(a659, a660)
    }, {
        let a664 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h661, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h661
            }
        };
        let a665 = f76(v0.clone(), v1, v2, v3, {
            let a662 = v5;
            let a663 = 88u64;
            nat_add(a662, a663)?
        })?;
        fixed_u64::bit_xor(a664, a665)
    }, {
        let a669 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h666, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h666
            }
        };
        let a670 = f76(v0.clone(), v1, v2, v3, {
            let a667 = v5;
            let a668 = 96u64;
            nat_add(a667, a668)?
        })?;
        fixed_u64::bit_xor(a669, a670)
    }, {
        let a674 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h671, _, _, _, _, _, _, _, _, _, _, _) => {
                h671
            }
        };
        let a675 = f76(v0.clone(), v1, v2, v3, {
            let a672 = v5;
            let a673 = 104u64;
            nat_add(a672, a673)?
        })?;
        fixed_u64::bit_xor(a674, a675)
    }, {
        let a679 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h676, _, _, _, _, _, _, _, _, _, _) => {
                h676
            }
        };
        let a680 = f76(v0.clone(), v1, v2, v3, {
            let a677 = v5;
            let a678 = 112u64;
            nat_add(a677, a678)?
        })?;
        fixed_u64::bit_xor(a679, a680)
    }, {
        let a684 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h681, _, _, _, _, _, _, _, _, _) => {
                h681
            }
        };
        let a685 = f76(v0.clone(), v1, v2, v3, {
            let a682 = v5;
            let a683 = 120u64;
            nat_add(a682, a683)?
        })?;
        fixed_u64::bit_xor(a684, a685)
    }, {
        let a689 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h686, _, _, _, _, _, _, _, _) => {
                h686
            }
        };
        let a690 = f76(v0, v1, v2, v3, {
            let a687 = v5;
            let a688 = 128u64;
            nat_add(a687, a688)?
        })?;
        fixed_u64::bit_xor(a689, a690)
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h691, _, _, _, _, _, _, _) => {
            h691
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h692, _, _, _, _, _, _) => {
            h692
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h693, _, _, _, _, _) => {
            h693
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h694, _, _, _, _) => {
            h694
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h695, _, _, _) => {
            h695
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h696, _, _) => {
            h696
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h697, _) => {
            h697
        }
    }, match v4 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h698) => {
            h698
        }
    }))
}

pub fn f57(v0: Fn6, p702: u64, p703: Adt7) -> R<Adt7> {
    let m701 = (p702, p703);
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v1, v2) = m701;
                let m699 = v1;
                if m699 == 0 {
                    Err::<(u64, Adt7), R<Adt7>>(Ok(v2))
                } else {
                    let v3 = m699 - 1;
                    Ok::<(u64, Adt7), R<Adt7>>((v3, {
                        let c700: Fn6 = v0.clone();
                        c700.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Adt7) -> R<Adt7> {
    f77(v0)
}

pub fn f59(v0: u64, v1: u64) -> u64 {
    let v2: u64 = 4294967295u64;
    let v3: u64 = f78({
        let a704 = v0;
        let a705 = v2;
        fixed_u64::bit_and(a704, a705)
    }, {
        let a706 = v1;
        let a707 = v2;
        fixed_u64::bit_and(a706, a707)
    });
    let v4: u64 = f78(f78(f42(v0, 32u32), f42(v1, 32u32)), f42(v3, 32u32));
    let a712 = f79({
        let a708 = v4;
        let a709 = v2;
        fixed_u64::bit_and(a708, a709)
    }, 32u32);
    let a713 = {
        let a710 = v3;
        let a711 = v2;
        fixed_u64::bit_and(a710, a711)
    };
    fixed_u64::bit_or(a712, a713)
}

pub fn f60(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a716 = {
        let a714 = v3;
        let a715 = 16u64;
        nat_add(a714, a715)?
    };
    let a717 = v2;
    if nat_eq(a716, a717) {
        Ok(0u64)
    } else {
        let a720 = {
            let a718 = v3;
            let a719 = 8u64;
            nat_add(a718, a719)?
        };
        let a721 = v2;
        if nat_eq(a720, a721) {
            Ok(f80({
                let a722 = v1;
                let a723 = 8u64;
                nat_mul(a722, a723)?
            }))
        } else {
            Ok(f81(f82(v0.clone(), v1, {
                let a724 = v3;
                let a725 = 0u64;
                nat_add(a724, a725)?
            }), f82(v0.clone(), v1, {
                let a726 = v3;
                let a727 = 1u64;
                nat_add(a726, a727)?
            }), f82(v0.clone(), v1, {
                let a728 = v3;
                let a729 = 2u64;
                nat_add(a728, a729)?
            }), f82(v0.clone(), v1, {
                let a730 = v3;
                let a731 = 3u64;
                nat_add(a730, a731)?
            }), f82(v0.clone(), v1, {
                let a732 = v3;
                let a733 = 4u64;
                nat_add(a732, a733)?
            }), f82(v0.clone(), v1, {
                let a734 = v3;
                let a735 = 5u64;
                nat_add(a734, a735)?
            }), f82(v0.clone(), v1, {
                let a736 = v3;
                let a737 = 6u64;
                nat_add(a736, a737)?
            }), f82(v0, v1, {
                let a738 = v3;
                let a739 = 7u64;
                nat_add(a738, a739)?
            })))
        }
    }
}

pub fn f61(v0: (u64, List<Adt1>), v1: Adt1) -> R<(u64, List<Adt1>)> {
    let v2: Adt10 = f83(Fn10::F84, 40u64, Adt10::C0(v1, {
        let a741 = {
            let (h740, _) = v0.clone();
            h740
        };
        let a742 = 1u64;
        nat_add(a741, a742)?
    }, {
        let (_, h743) = v0;
        h743
    }));
    Ok(({
        let a745 = {
            let (h744, _) = v0;
            h744
        };
        let a746 = 1u64;
        nat_add(a745, a746)?
    }, List::cons(match v2.clone() {
        Adt10::C0(h747, _, _) => {
            h747
        }
    }, match v2 {
        Adt10::C0(_, _, h748) => {
            h748
        }
    })))
}

pub fn f62(v0: Fn7, p752: u64, p753: (Adt9, Adt8)) -> (Adt9, Adt8) {
    let m751 = (p752, p753);
    {
        let mut m751 = m751;
        loop {
            let step = {
                let (v1, v2) = m751;
                let m749 = v1;
                if m749 == 0 {
                    Err::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>(v2)
                } else {
                    let v3 = m749 - 1;
                    Ok::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>((v3, {
                        let c750: Fn7 = v0.clone();
                        c750.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m751 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    f85(v0)
}

pub fn f64(v0: Fn8, p757: u64, p758: (u64, Adt1)) -> R<(u64, Adt1)> {
    let m756 = (p757, p758);
    {
        let mut m756 = m756;
        loop {
            let step = {
                let (v1, v2) = m756;
                let m754 = v1;
                if m754 == 0 {
                    Err::<(u64, (u64, Adt1)), R<(u64, Adt1)>>(Ok(v2))
                } else {
                    let v3 = m754 - 1;
                    Ok::<(u64, (u64, Adt1)), R<(u64, Adt1)>>((v3, {
                        let c755: Fn8 = v0.clone();
                        c755.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt1)) -> R<(u64, Adt1)> {
    Ok(({
        let a760 = {
            let (h759, _) = v3.clone();
            h759
        };
        let a761 = 1u64;
        nat_add(a760, a761)?
    }, f47({
        let (_, h762) = v3.clone();
        h762
    }, f49(v1, {
        let a766 = v2;
        let a767 = {
            let a764 = 64u64;
            let a765 = {
                let (h763, _) = v3.clone();
                h763
            };
            nat_mul(a764, a765)?
        };
        nat_add(a766, a767)?
    })?, v0, 64u32, {
        let a769 = {
            let (h768, _) = v3;
            h768
        };
        let a770 = 0u64;
        if nat_eq(a769, a770) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f66() -> Adt1 {
    Adt1::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f67(v0: Bytes, v1: u64) -> R<u32> {
    let a781 = f86(v0.clone(), v1);
    let a782 = {
        let a779 = f87(f86(v0.clone(), {
            let a771 = v1;
            let a772 = 1u64;
            nat_add(a771, a772)?
        }), 8u32);
        let a780 = {
            let a777 = f87(f86(v0.clone(), {
                let a773 = v1;
                let a774 = 2u64;
                nat_add(a773, a774)?
            }), 16u32);
            let a778 = f87(f86(v0, {
                let a775 = v1;
                let a776 = 3u64;
                nat_add(a775, a776)?
            }), 24u32);
            fixed_u32::bit_or(a777, a778)
        };
        fixed_u32::bit_or(a779, a780)
    };
    Ok(fixed_u32::bit_or(a781, a782))
}

pub fn f68(v0: Fn9, p786: u64, p787: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m785 = (p786, p787);
    {
        let mut m785 = m785;
        loop {
            let step = {
                let (v1, v2) = m785;
                let m783 = v1;
                if m783 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m783 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c784: Fn9 = v0.clone();
                        c784.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h788, _) = v0;
        h788
    };
    let v2: u32 = {
        let (h789, _) = {
            let (_, h790) = v0;
            h790
        };
        h789
    };
    let v3: u32 = {
        let (_, h791) = {
            let (_, h792) = v0;
            h792
        };
        h791
    };
    ({
        let a793 = v1;
        let a794 = 2u64;
        let a795 = 0u64;
        nat_quot(a793, a794, a795)
    }, (f87(v2, 1u32), {
        let a799 = {
            let a796 = v1;
            let a797 = 2u64;
            let a798 = 0u64;
            nat_rem(a796, a797, a798)
        };
        let a800 = 1u64;
        if nat_eq(a799, a800) {
            let a801 = v3;
            let a802 = v2;
            fixed_u32::bit_or(a801, a802)
        } else {
            v3
        }
    }))
}

pub fn f70(v0: Adt1, v1: Adt1, v2: bool) -> Adt1 {
    f47(f66(), f88(v0, v1), 0u64, 64u32, {
        let a803 = 4u32;
        let a804 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a803, a804)
    })
}

pub fn f71(v0: Adt5) -> R<Adt5> {
    let v1: u32 = f89(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, _, h805) => {
            h805
        }
    });
    let v2: u32 = f54(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h806, _, _) => {
            h806
        }
    }, f54(f90(match v0.clone() {
        Adt5::C0(_, _, _, _, h807, _, _, _, _, _) => {
            h807
        }
    }), f54(f91(match v0.clone() {
        Adt5::C0(_, _, _, _, h808, _, _, _, _, _) => {
            h808
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h809, _, _, _, _) => {
            h809
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h810, _, _, _) => {
            h810
        }
    }), f54(v1, {
        let m813 = match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, _, _, h812, _) => {
                h812
            }
        };
        match m813 {
            Adt4::C0(h811, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h811
            }
        }
    }))));
    let v3: u32 = f54(f92(match v0.clone() {
        Adt5::C0(h814, _, _, _, _, _, _, _, _, _) => {
            h814
        }
    }), f93(match v0.clone() {
        Adt5::C0(h815, _, _, _, _, _, _, _, _, _) => {
            h815
        }
    }, match v0.clone() {
        Adt5::C0(_, h816, _, _, _, _, _, _, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h817, _, _, _, _, _, _, _) => {
            h817
        }
    }));
    Ok(Adt5::C0(f54(v2, v3), match v0.clone() {
        Adt5::C0(h818, _, _, _, _, _, _, _, _, _) => {
            h818
        }
    }, match v0.clone() {
        Adt5::C0(_, h819, _, _, _, _, _, _, _, _) => {
            h819
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h820, _, _, _, _, _, _, _) => {
            h820
        }
    }, f54(match v0.clone() {
        Adt5::C0(_, _, _, h821, _, _, _, _, _, _) => {
            h821
        }
    }, v2), match v0.clone() {
        Adt5::C0(_, _, _, _, h822, _, _, _, _, _) => {
            h822
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h823, _, _, _, _) => {
            h823
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h824, _, _, _) => {
            h824
        }
    }, f94(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, h825, _) => {
            h825
        }
    }), {
        let a827 = match v0 {
            Adt5::C0(_, _, _, _, _, _, _, _, _, h826) => {
                h826
            }
        };
        let a828 = 1u64;
        nat_add(a827, a828)?
    }))
}

pub fn f72(v0: u32) -> u64 {
    let m830 = {
        let a829 = v0;
        fixed_u64::convert(i128::from(a829))
    };
    match m830 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f73(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a835 = f87(v0, 24u32);
    let a836 = {
        let a833 = f87(v1, 16u32);
        let a834 = {
            let a831 = f87(v2, 8u32);
            let a832 = v3;
            fixed_u32::bit_or(a831, a832)
        };
        fixed_u32::bit_or(a833, a834)
    };
    fixed_u32::bit_or(a835, a836)
}

pub fn f74(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a837 = v2;
    let a838 = v1;
    if nat_lt(a837, a838) {
        let m841 = {
            let a839 = v0;
            let a840 = v2;
            index_bytes(a839, a840)
        };
        match m841 {
            None => {
                0u32
            }
            Some(v3) => {
                let m843 = {
                    let a842 = v3;
                    fixed_u32::convert(i128::from(a842))
                };
                match m843 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a844 = v2;
        let a845 = v1;
        if nat_eq(a844, a845) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f75(v0: Adt3) -> R<Adt3> {
    let (_, h846) = f36(Fn3::F95, 24u64, (0u64, v0))?;
    Ok(h846)
}

pub fn f76(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    Ok(f96(f97(v0.clone(), v1, v2, v3, {
        let a847 = v4;
        let a848 = 0u64;
        nat_add(a847, a848)?
    })?, f97(v0.clone(), v1, v2, v3, {
        let a849 = v4;
        let a850 = 1u64;
        nat_add(a849, a850)?
    })?, f97(v0.clone(), v1, v2, v3, {
        let a851 = v4;
        let a852 = 2u64;
        nat_add(a851, a852)?
    })?, f97(v0.clone(), v1, v2, v3, {
        let a853 = v4;
        let a854 = 3u64;
        nat_add(a853, a854)?
    })?, f97(v0.clone(), v1, v2, v3, {
        let a855 = v4;
        let a856 = 4u64;
        nat_add(a855, a856)?
    })?, f97(v0.clone(), v1, v2, v3, {
        let a857 = v4;
        let a858 = 5u64;
        nat_add(a857, a858)?
    })?, f97(v0.clone(), v1, v2, v3, {
        let a859 = v4;
        let a860 = 6u64;
        nat_add(a859, a860)?
    })?, f97(v0, v1, v2, v3, {
        let a861 = v4;
        let a862 = 7u64;
        nat_add(a861, a862)?
    })?))
}

pub fn f77(v0: Adt7) -> R<Adt7> {
    let v1: u64 = f98(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h863) => {
            h863
        }
    });
    let v2: u64 = f59(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h864, _, _) => {
            h864
        }
    }, f59(f99(match v0.clone() {
        Adt7::C0(_, _, _, _, h865, _, _, _, _, _) => {
            h865
        }
    }), f59(f100(match v0.clone() {
        Adt7::C0(_, _, _, _, h866, _, _, _, _, _) => {
            h866
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h867, _, _, _, _) => {
            h867
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h868, _, _, _) => {
            h868
        }
    }), f59(v1, {
        let m871 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h870, _) => {
                h870
            }
        };
        match m871 {
            Adt6::C0(h869, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h869
            }
        }
    }))));
    let v3: u64 = f59(f101(match v0.clone() {
        Adt7::C0(h872, _, _, _, _, _, _, _, _, _) => {
            h872
        }
    }), f102(match v0.clone() {
        Adt7::C0(h873, _, _, _, _, _, _, _, _, _) => {
            h873
        }
    }, match v0.clone() {
        Adt7::C0(_, h874, _, _, _, _, _, _, _, _) => {
            h874
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h875, _, _, _, _, _, _, _) => {
            h875
        }
    }));
    Ok(Adt7::C0(f59(v2, v3), match v0.clone() {
        Adt7::C0(h876, _, _, _, _, _, _, _, _, _) => {
            h876
        }
    }, match v0.clone() {
        Adt7::C0(_, h877, _, _, _, _, _, _, _, _) => {
            h877
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h878, _, _, _, _, _, _, _) => {
            h878
        }
    }, f59(match v0.clone() {
        Adt7::C0(_, _, _, h879, _, _, _, _, _, _) => {
            h879
        }
    }, v2), match v0.clone() {
        Adt7::C0(_, _, _, _, h880, _, _, _, _, _) => {
            h880
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h881, _, _, _, _) => {
            h881
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h882, _, _, _) => {
            h882
        }
    }, f103(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h883, _) => {
            h883
        }
    }), {
        let a885 = match v0 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h884) => {
                h884
            }
        };
        let a886 = 1u64;
        nat_add(a885, a886)?
    }))
}

pub fn f78(v0: u64, v1: u64) -> u64 {
    let m889 = {
        let a887 = v0;
        let a888 = v1;
        fixed_u64::checked_add(a887, a888)
    };
    match m889 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f79(v0: u64, v1: u32) -> u64 {
    let m892 = {
        let a890 = v0;
        let a891 = v1;
        fixed_u64::shift_left(a890, a891)
    };
    match m892 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f80(v0: u64) -> u64 {
    let (_, h893) = {
        let (_, h894) = f104(Fn11::F105, 64u64, (v0, (1u64, 0u64)));
        h894
    };
    h893
}

pub fn f81(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a907 = f79(v0, 56u32);
    let a908 = {
        let a905 = f79(v1, 48u32);
        let a906 = {
            let a903 = f79(v2, 40u32);
            let a904 = {
                let a901 = f79(v3, 32u32);
                let a902 = {
                    let a899 = f79(v4, 24u32);
                    let a900 = {
                        let a897 = f79(v5, 16u32);
                        let a898 = {
                            let a895 = f79(v6, 8u32);
                            let a896 = v7;
                            fixed_u64::bit_or(a895, a896)
                        };
                        fixed_u64::bit_or(a897, a898)
                    };
                    fixed_u64::bit_or(a899, a900)
                };
                fixed_u64::bit_or(a901, a902)
            };
            fixed_u64::bit_or(a903, a904)
        };
        fixed_u64::bit_or(a905, a906)
    };
    fixed_u64::bit_or(a907, a908)
}

pub fn f82(v0: Bytes, v1: u64, v2: u64) -> u64 {
    let a909 = v2;
    let a910 = v1;
    if nat_lt(a909, a910) {
        f106(v0, v2)
    } else {
        let a911 = v2;
        let a912 = v1;
        if nat_eq(a911, a912) {
            128u64
        } else {
            0u64
        }
    }
}

pub fn f83(v0: Fn10, p916: u64, p917: Adt10) -> Adt10 {
    let m915 = (p916, p917);
    {
        let mut m915 = m915;
        loop {
            let step = {
                let (v1, v2) = m915;
                let m913 = v1;
                if m913 == 0 {
                    Err::<(u64, Adt10), Adt10>(v2)
                } else {
                    let v3 = m913 - 1;
                    Ok::<(u64, Adt10), Adt10>((v3, {
                        let c914: Fn10 = v0.clone();
                        c914.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Adt10) -> Adt10 {
    f107(v0)
}

pub fn f85(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    (f108(f109({
        let (h918, _) = v0.clone();
        h918
    }, {
        let (_, h919) = v0.clone();
        h919
    }), {
        let (_, h920) = v0.clone();
        h920
    }), f110({
        let (_, h921) = v0;
        h921
    }))
}

pub fn f86(v0: Bytes, v1: u64) -> u32 {
    let m924 = {
        let a922 = v0;
        let a923 = v1;
        index_bytes(a922, a923)
    };
    match m924 {
        None => {
            0u32
        }
        Some(v2) => {
            let m926 = {
                let a925 = v2;
                fixed_u32::convert(i128::from(a925))
            };
            match m926 {
                None => {
                    0u32
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f87(v0: u32, v1: u32) -> u32 {
    let m929 = {
        let a927 = v0;
        let a928 = v1;
        fixed_u32::shift_left(a927, a928)
    };
    match m929 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f88(v0: Adt1, v1: Adt1) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt1::C0(h930, _, _, _, _, _, _, _) => {
            h930
        }
    }, match v0.clone() {
        Adt1::C0(_, h931, _, _, _, _, _, _) => {
            h931
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h932, _, _, _, _, _) => {
            h932
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h933, _, _, _, _) => {
            h933
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h934, _, _, _) => {
            h934
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h935, _, _) => {
            h935
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h936, _) => {
            h936
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h937) => {
            h937
        }
    }, match v1.clone() {
        Adt1::C0(h938, _, _, _, _, _, _, _) => {
            h938
        }
    }, match v1.clone() {
        Adt1::C0(_, h939, _, _, _, _, _, _) => {
            h939
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h940, _, _, _, _, _) => {
            h940
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h941, _, _, _, _) => {
            h941
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h942, _, _, _) => {
            h942
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h943, _, _) => {
            h943
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h944, _) => {
            h944
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h945) => {
            h945
        }
    })
}

pub fn f89(v0: u64) -> u32 {
    let a946 = v0;
    let a947 = 32u64;
    if nat_lt(a946, a947) {
        let a948 = v0;
        let a949 = 16u64;
        if nat_lt(a948, a949) {
            let a950 = v0;
            let a951 = 8u64;
            if nat_lt(a950, a951) {
                let a952 = v0;
                let a953 = 4u64;
                if nat_lt(a952, a953) {
                    let a954 = v0;
                    let a955 = 2u64;
                    if nat_lt(a954, a955) {
                        let a956 = v0;
                        let a957 = 1u64;
                        if nat_lt(a956, a957) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a958 = v0;
                        let a959 = 3u64;
                        if nat_lt(a958, a959) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a960 = v0;
                    let a961 = 6u64;
                    if nat_lt(a960, a961) {
                        let a962 = v0;
                        let a963 = 5u64;
                        if nat_lt(a962, a963) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a964 = v0;
                        let a965 = 7u64;
                        if nat_lt(a964, a965) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a966 = v0;
                let a967 = 12u64;
                if nat_lt(a966, a967) {
                    let a968 = v0;
                    let a969 = 10u64;
                    if nat_lt(a968, a969) {
                        let a970 = v0;
                        let a971 = 9u64;
                        if nat_lt(a970, a971) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a972 = v0;
                        let a973 = 11u64;
                        if nat_lt(a972, a973) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a974 = v0;
                    let a975 = 14u64;
                    if nat_lt(a974, a975) {
                        let a976 = v0;
                        let a977 = 13u64;
                        if nat_lt(a976, a977) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a978 = v0;
                        let a979 = 15u64;
                        if nat_lt(a978, a979) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a980 = v0;
            let a981 = 24u64;
            if nat_lt(a980, a981) {
                let a982 = v0;
                let a983 = 20u64;
                if nat_lt(a982, a983) {
                    let a984 = v0;
                    let a985 = 18u64;
                    if nat_lt(a984, a985) {
                        let a986 = v0;
                        let a987 = 17u64;
                        if nat_lt(a986, a987) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a988 = v0;
                        let a989 = 19u64;
                        if nat_lt(a988, a989) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a990 = v0;
                    let a991 = 22u64;
                    if nat_lt(a990, a991) {
                        let a992 = v0;
                        let a993 = 21u64;
                        if nat_lt(a992, a993) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a994 = v0;
                        let a995 = 23u64;
                        if nat_lt(a994, a995) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a996 = v0;
                let a997 = 28u64;
                if nat_lt(a996, a997) {
                    let a998 = v0;
                    let a999 = 26u64;
                    if nat_lt(a998, a999) {
                        let a1000 = v0;
                        let a1001 = 25u64;
                        if nat_lt(a1000, a1001) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1002 = v0;
                        let a1003 = 27u64;
                        if nat_lt(a1002, a1003) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1004 = v0;
                    let a1005 = 30u64;
                    if nat_lt(a1004, a1005) {
                        let a1006 = v0;
                        let a1007 = 29u64;
                        if nat_lt(a1006, a1007) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1008 = v0;
                        let a1009 = 31u64;
                        if nat_lt(a1008, a1009) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1010 = v0;
        let a1011 = 48u64;
        if nat_lt(a1010, a1011) {
            let a1012 = v0;
            let a1013 = 40u64;
            if nat_lt(a1012, a1013) {
                let a1014 = v0;
                let a1015 = 36u64;
                if nat_lt(a1014, a1015) {
                    let a1016 = v0;
                    let a1017 = 34u64;
                    if nat_lt(a1016, a1017) {
                        let a1018 = v0;
                        let a1019 = 33u64;
                        if nat_lt(a1018, a1019) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1020 = v0;
                        let a1021 = 35u64;
                        if nat_lt(a1020, a1021) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1022 = v0;
                    let a1023 = 38u64;
                    if nat_lt(a1022, a1023) {
                        let a1024 = v0;
                        let a1025 = 37u64;
                        if nat_lt(a1024, a1025) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1026 = v0;
                        let a1027 = 39u64;
                        if nat_lt(a1026, a1027) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1028 = v0;
                let a1029 = 44u64;
                if nat_lt(a1028, a1029) {
                    let a1030 = v0;
                    let a1031 = 42u64;
                    if nat_lt(a1030, a1031) {
                        let a1032 = v0;
                        let a1033 = 41u64;
                        if nat_lt(a1032, a1033) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1034 = v0;
                        let a1035 = 43u64;
                        if nat_lt(a1034, a1035) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1036 = v0;
                    let a1037 = 46u64;
                    if nat_lt(a1036, a1037) {
                        let a1038 = v0;
                        let a1039 = 45u64;
                        if nat_lt(a1038, a1039) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1040 = v0;
                        let a1041 = 47u64;
                        if nat_lt(a1040, a1041) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1042 = v0;
            let a1043 = 56u64;
            if nat_lt(a1042, a1043) {
                let a1044 = v0;
                let a1045 = 52u64;
                if nat_lt(a1044, a1045) {
                    let a1046 = v0;
                    let a1047 = 50u64;
                    if nat_lt(a1046, a1047) {
                        let a1048 = v0;
                        let a1049 = 49u64;
                        if nat_lt(a1048, a1049) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1050 = v0;
                        let a1051 = 51u64;
                        if nat_lt(a1050, a1051) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1052 = v0;
                    let a1053 = 54u64;
                    if nat_lt(a1052, a1053) {
                        let a1054 = v0;
                        let a1055 = 53u64;
                        if nat_lt(a1054, a1055) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1056 = v0;
                        let a1057 = 55u64;
                        if nat_lt(a1056, a1057) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1058 = v0;
                let a1059 = 60u64;
                if nat_lt(a1058, a1059) {
                    let a1060 = v0;
                    let a1061 = 58u64;
                    if nat_lt(a1060, a1061) {
                        let a1062 = v0;
                        let a1063 = 57u64;
                        if nat_lt(a1062, a1063) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1064 = v0;
                        let a1065 = 59u64;
                        if nat_lt(a1064, a1065) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1066 = v0;
                    let a1067 = 62u64;
                    if nat_lt(a1066, a1067) {
                        let a1068 = v0;
                        let a1069 = 61u64;
                        if nat_lt(a1068, a1069) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1070 = v0;
                        let a1071 = 63u64;
                        if nat_lt(a1070, a1071) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f90(v0: u32) -> u32 {
    let a1074 = {
        let a1072 = f111(v0, 6u32, 26u32);
        let a1073 = f111(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1072, a1073)
    };
    let a1075 = f111(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1074, a1075)
}

pub fn f91(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1081 = {
        let a1076 = v0;
        let a1077 = v1;
        fixed_u32::bit_and(a1076, a1077)
    };
    let a1082 = {
        let a1079 = {
            let a1078 = v0;
            fixed_u32::bit_not(a1078)
        };
        let a1080 = v2;
        fixed_u32::bit_and(a1079, a1080)
    };
    fixed_u32::bit_xor(a1081, a1082)
}

pub fn f92(v0: u32) -> u32 {
    let a1085 = {
        let a1083 = f111(v0, 2u32, 30u32);
        let a1084 = f111(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1083, a1084)
    };
    let a1086 = f111(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1085, a1086)
}

pub fn f93(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1095 = {
        let a1091 = {
            let a1087 = v0;
            let a1088 = v1;
            fixed_u32::bit_and(a1087, a1088)
        };
        let a1092 = {
            let a1089 = v0;
            let a1090 = v2;
            fixed_u32::bit_and(a1089, a1090)
        };
        fixed_u32::bit_xor(a1091, a1092)
    };
    let a1096 = {
        let a1093 = v1;
        let a1094 = v2;
        fixed_u32::bit_and(a1093, a1094)
    };
    fixed_u32::bit_xor(a1095, a1096)
}

pub fn f94(v0: Adt4) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(_, h1097, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1097
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h1098, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1098
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h1099, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1099
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h1100, _, _, _, _, _, _, _, _, _, _, _) => {
            h1100
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h1101, _, _, _, _, _, _, _, _, _, _) => {
            h1101
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h1102, _, _, _, _, _, _, _, _, _) => {
            h1102
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h1103, _, _, _, _, _, _, _, _) => {
            h1103
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h1104, _, _, _, _, _, _, _) => {
            h1104
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1105, _, _, _, _, _, _) => {
            h1105
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h1106, _, _, _, _, _) => {
            h1106
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, h1107, _, _, _, _) => {
            h1107
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1108, _, _, _) => {
            h1108
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1109, _, _) => {
            h1109
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1110, _) => {
            h1110
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1111) => {
            h1111
        }
    }, f54(f112(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1112, _) => {
            h1112
        }
    }), f54(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1113, _, _, _, _, _, _) => {
            h1113
        }
    }, f54(f113(match v0.clone() {
        Adt4::C0(_, h1114, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1114
        }
    }), match v0 {
        Adt4::C0(h1115, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1115
        }
    }))))
}

pub fn f95(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    f114(v0)
}

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1128 = f79(v7, 56u32);
    let a1129 = {
        let a1126 = f79(v6, 48u32);
        let a1127 = {
            let a1124 = f79(v5, 40u32);
            let a1125 = {
                let a1122 = f79(v4, 32u32);
                let a1123 = {
                    let a1120 = f79(v3, 24u32);
                    let a1121 = {
                        let a1118 = f79(v2, 16u32);
                        let a1119 = {
                            let a1116 = f79(v1, 8u32);
                            let a1117 = v0;
                            fixed_u64::bit_or(a1116, a1117)
                        };
                        fixed_u64::bit_or(a1118, a1119)
                    };
                    fixed_u64::bit_or(a1120, a1121)
                };
                fixed_u64::bit_or(a1122, a1123)
            };
            fixed_u64::bit_or(a1124, a1125)
        };
        fixed_u64::bit_or(a1126, a1127)
    };
    fixed_u64::bit_or(a1128, a1129)
}

pub fn f97(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let v5: u64 = {
        let a1130 = v4;
        let a1131 = v1;
        if nat_lt(a1130, a1131) {
            f106(v0, v4)
        } else {
            let a1132 = v4;
            let a1133 = v1;
            if nat_eq(a1132, a1133) {
                v3
            } else {
                0u64
            }
        }
    };
    let a1136 = {
        let a1134 = v4;
        let a1135 = 1u64;
        nat_add(a1134, a1135)?
    };
    let a1137 = v2;
    if nat_eq(a1136, a1137) {
        let a1138 = v5;
        let a1139 = 128u64;
        Ok(fixed_u64::bit_or(a1138, a1139))
    } else {
        Ok(v5)
    }
}

pub fn f98(v0: u64) -> u64 {
    let a1140 = v0;
    let a1141 = 40u64;
    if nat_lt(a1140, a1141) {
        let a1142 = v0;
        let a1143 = 20u64;
        if nat_lt(a1142, a1143) {
            let a1144 = v0;
            let a1145 = 10u64;
            if nat_lt(a1144, a1145) {
                let a1146 = v0;
                let a1147 = 5u64;
                if nat_lt(a1146, a1147) {
                    let a1148 = v0;
                    let a1149 = 2u64;
                    if nat_lt(a1148, a1149) {
                        let a1150 = v0;
                        let a1151 = 1u64;
                        if nat_lt(a1150, a1151) {
                            4794697086780616226u64
                        } else {
                            8158064640168781261u64
                        }
                    } else {
                        let a1152 = v0;
                        let a1153 = 3u64;
                        if nat_lt(a1152, a1153) {
                            13096744586834688815u64
                        } else {
                            let a1154 = v0;
                            let a1155 = 4u64;
                            if nat_lt(a1154, a1155) {
                                16840607885511220156u64
                            } else {
                                4131703408338449720u64
                            }
                        }
                    }
                } else {
                    let a1156 = v0;
                    let a1157 = 7u64;
                    if nat_lt(a1156, a1157) {
                        let a1158 = v0;
                        let a1159 = 6u64;
                        if nat_lt(a1158, a1159) {
                            6480981068601479193u64
                        } else {
                            10538285296894168987u64
                        }
                    } else {
                        let a1160 = v0;
                        let a1161 = 8u64;
                        if nat_lt(a1160, a1161) {
                            12329834152419229976u64
                        } else {
                            let a1162 = v0;
                            let a1163 = 9u64;
                            if nat_lt(a1162, a1163) {
                                15566598209576043074u64
                            } else {
                                1334009975649890238u64
                            }
                        }
                    }
                }
            } else {
                let a1164 = v0;
                let a1165 = 15u64;
                if nat_lt(a1164, a1165) {
                    let a1166 = v0;
                    let a1167 = 12u64;
                    if nat_lt(a1166, a1167) {
                        let a1168 = v0;
                        let a1169 = 11u64;
                        if nat_lt(a1168, a1169) {
                            2608012711638119052u64
                        } else {
                            6128411473006802146u64
                        }
                    } else {
                        let a1170 = v0;
                        let a1171 = 13u64;
                        if nat_lt(a1170, a1171) {
                            8268148722764581231u64
                        } else {
                            let a1172 = v0;
                            let a1173 = 14u64;
                            if nat_lt(a1172, a1173) {
                                9286055187155687089u64
                            } else {
                                11230858885718282805u64
                            }
                        }
                    }
                } else {
                    let a1174 = v0;
                    let a1175 = 17u64;
                    if nat_lt(a1174, a1175) {
                        let a1176 = v0;
                        let a1177 = 16u64;
                        if nat_lt(a1176, a1177) {
                            13951009754708518548u64
                        } else {
                            16472876342353939154u64
                        }
                    } else {
                        let a1178 = v0;
                        let a1179 = 18u64;
                        if nat_lt(a1178, a1179) {
                            17275323862435702243u64
                        } else {
                            let a1180 = v0;
                            let a1181 = 19u64;
                            if nat_lt(a1180, a1181) {
                                1135362057144423861u64
                            } else {
                                2597628984639134821u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1182 = v0;
            let a1183 = 30u64;
            if nat_lt(a1182, a1183) {
                let a1184 = v0;
                let a1185 = 25u64;
                if nat_lt(a1184, a1185) {
                    let a1186 = v0;
                    let a1187 = 22u64;
                    if nat_lt(a1186, a1187) {
                        let a1188 = v0;
                        let a1189 = 21u64;
                        if nat_lt(a1188, a1189) {
                            3308224258029322869u64
                        } else {
                            5365058923640841347u64
                        }
                    } else {
                        let a1190 = v0;
                        let a1191 = 23u64;
                        if nat_lt(a1190, a1191) {
                            6679025012923562964u64
                        } else {
                            let a1192 = v0;
                            let a1193 = 24u64;
                            if nat_lt(a1192, a1193) {
                                8573033837759648693u64
                            } else {
                                10970295158949994411u64
                            }
                        }
                    }
                } else {
                    let a1194 = v0;
                    let a1195 = 27u64;
                    if nat_lt(a1194, a1195) {
                        let a1196 = v0;
                        let a1197 = 26u64;
                        if nat_lt(a1196, a1197) {
                            12119686244451234320u64
                        } else {
                            12683024718118986047u64
                        }
                    } else {
                        let a1198 = v0;
                        let a1199 = 28u64;
                        if nat_lt(a1198, a1199) {
                            13788192230050041572u64
                        } else {
                            let a1200 = v0;
                            let a1201 = 29u64;
                            if nat_lt(a1200, a1201) {
                                14330467153632333762u64
                            } else {
                                15395433587784984357u64
                            }
                        }
                    }
                }
            } else {
                let a1202 = v0;
                let a1203 = 35u64;
                if nat_lt(a1202, a1203) {
                    let a1204 = v0;
                    let a1205 = 32u64;
                    if nat_lt(a1204, a1205) {
                        let a1206 = v0;
                        let a1207 = 31u64;
                        if nat_lt(a1206, a1207) {
                            489312712824947311u64
                        } else {
                            1452737877330783856u64
                        }
                    } else {
                        let a1208 = v0;
                        let a1209 = 33u64;
                        if nat_lt(a1208, a1209) {
                            2861767655752347644u64
                        } else {
                            let a1210 = v0;
                            let a1211 = 34u64;
                            if nat_lt(a1210, a1211) {
                                3322285676063803686u64
                            } else {
                                5560940570517711597u64
                            }
                        }
                    }
                } else {
                    let a1212 = v0;
                    let a1213 = 37u64;
                    if nat_lt(a1212, a1213) {
                        let a1214 = v0;
                        let a1215 = 36u64;
                        if nat_lt(a1214, a1215) {
                            5996557281743188959u64
                        } else {
                            7280758554555802590u64
                        }
                    } else {
                        let a1216 = v0;
                        let a1217 = 38u64;
                        if nat_lt(a1216, a1217) {
                            8532644243296465576u64
                        } else {
                            let a1218 = v0;
                            let a1219 = 39u64;
                            if nat_lt(a1218, a1219) {
                                9350256976987008742u64
                            } else {
                                10552545826968843579u64
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a1220 = v0;
        let a1221 = 60u64;
        if nat_lt(a1220, a1221) {
            let a1222 = v0;
            let a1223 = 50u64;
            if nat_lt(a1222, a1223) {
                let a1224 = v0;
                let a1225 = 45u64;
                if nat_lt(a1224, a1225) {
                    let a1226 = v0;
                    let a1227 = 42u64;
                    if nat_lt(a1226, a1227) {
                        let a1228 = v0;
                        let a1229 = 41u64;
                        if nat_lt(a1228, a1229) {
                            11727347734174303076u64
                        } else {
                            12113106623233404929u64
                        }
                    } else {
                        let a1230 = v0;
                        let a1231 = 43u64;
                        if nat_lt(a1230, a1231) {
                            14000437183269869457u64
                        } else {
                            let a1232 = v0;
                            let a1233 = 44u64;
                            if nat_lt(a1232, a1233) {
                                14369950271660146224u64
                            } else {
                                15101387698204529176u64
                            }
                        }
                    }
                } else {
                    let a1234 = v0;
                    let a1235 = 47u64;
                    if nat_lt(a1234, a1235) {
                        let a1236 = v0;
                        let a1237 = 46u64;
                        if nat_lt(a1236, a1237) {
                            15463397548674623760u64
                        } else {
                            17586052441742319658u64
                        }
                    } else {
                        let a1238 = v0;
                        let a1239 = 48u64;
                        if nat_lt(a1238, a1239) {
                            1182934255886127544u64
                        } else {
                            let a1240 = v0;
                            let a1241 = 49u64;
                            if nat_lt(a1240, a1241) {
                                1847814050463011016u64
                            } else {
                                2177327727835720531u64
                            }
                        }
                    }
                }
            } else {
                let a1242 = v0;
                let a1243 = 55u64;
                if nat_lt(a1242, a1243) {
                    let a1244 = v0;
                    let a1245 = 52u64;
                    if nat_lt(a1244, a1245) {
                        let a1246 = v0;
                        let a1247 = 51u64;
                        if nat_lt(a1246, a1247) {
                            2830643537854262169u64
                        } else {
                            3796741975233480872u64
                        }
                    } else {
                        let a1248 = v0;
                        let a1249 = 53u64;
                        if nat_lt(a1248, a1249) {
                            4115178125766777443u64
                        } else {
                            let a1250 = v0;
                            let a1251 = 54u64;
                            if nat_lt(a1250, a1251) {
                                5681478168544905931u64
                            } else {
                                6601373596472566643u64
                            }
                        }
                    }
                } else {
                    let a1252 = v0;
                    let a1253 = 57u64;
                    if nat_lt(a1252, a1253) {
                        let a1254 = v0;
                        let a1255 = 56u64;
                        if nat_lt(a1254, a1255) {
                            7507060721942968483u64
                        } else {
                            8399075790359081724u64
                        }
                    } else {
                        let a1256 = v0;
                        let a1257 = 58u64;
                        if nat_lt(a1256, a1257) {
                            8693463985226723168u64
                        } else {
                            let a1258 = v0;
                            let a1259 = 59u64;
                            if nat_lt(a1258, a1259) {
                                9568029438360202098u64
                            } else {
                                10144078919501101548u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1260 = v0;
            let a1261 = 70u64;
            if nat_lt(a1260, a1261) {
                let a1262 = v0;
                let a1263 = 65u64;
                if nat_lt(a1262, a1263) {
                    let a1264 = v0;
                    let a1265 = 62u64;
                    if nat_lt(a1264, a1265) {
                        let a1266 = v0;
                        let a1267 = 61u64;
                        if nat_lt(a1266, a1267) {
                            10430055236837252648u64
                        } else {
                            11840083180663258601u64
                        }
                    } else {
                        let a1268 = v0;
                        let a1269 = 63u64;
                        if nat_lt(a1268, a1269) {
                            13761210420658862357u64
                        } else {
                            let a1270 = v0;
                            let a1271 = 64u64;
                            if nat_lt(a1270, a1271) {
                                14299343276471374635u64
                            } else {
                                14566680578165727644u64
                            }
                        }
                    }
                } else {
                    let a1272 = v0;
                    let a1273 = 67u64;
                    if nat_lt(a1272, a1273) {
                        let a1274 = v0;
                        let a1275 = 66u64;
                        if nat_lt(a1274, a1275) {
                            15097957966210449927u64
                        } else {
                            16922976911328602910u64
                        }
                    } else {
                        let a1276 = v0;
                        let a1277 = 68u64;
                        if nat_lt(a1276, a1277) {
                            17689382322260857208u64
                        } else {
                            let a1278 = v0;
                            let a1279 = 69u64;
                            if nat_lt(a1278, a1279) {
                                500013540394364858u64
                            } else {
                                748580250866718886u64
                            }
                        }
                    }
                }
            } else {
                let a1280 = v0;
                let a1281 = 75u64;
                if nat_lt(a1280, a1281) {
                    let a1282 = v0;
                    let a1283 = 72u64;
                    if nat_lt(a1282, a1283) {
                        let a1284 = v0;
                        let a1285 = 71u64;
                        if nat_lt(a1284, a1285) {
                            1242879168328830382u64
                        } else {
                            1977374033974150939u64
                        }
                    } else {
                        let a1286 = v0;
                        let a1287 = 73u64;
                        if nat_lt(a1286, a1287) {
                            2944078676154940804u64
                        } else {
                            let a1288 = v0;
                            let a1289 = 74u64;
                            if nat_lt(a1288, a1289) {
                                3659926193048069267u64
                            } else {
                                4368137639120453308u64
                            }
                        }
                    }
                } else {
                    let a1290 = v0;
                    let a1291 = 77u64;
                    if nat_lt(a1290, a1291) {
                        let a1292 = v0;
                        let a1293 = 76u64;
                        if nat_lt(a1292, a1293) {
                            4836135668995329356u64
                        } else {
                            5532061633213252278u64
                        }
                    } else {
                        let a1294 = v0;
                        let a1295 = 78u64;
                        if nat_lt(a1294, a1295) {
                            6448918945643986474u64
                        } else {
                            let a1296 = v0;
                            let a1297 = 79u64;
                            if nat_lt(a1296, a1297) {
                                6902733635092675308u64
                            } else {
                                7801388544844847127u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f99(v0: u64) -> u64 {
    let a1300 = {
        let a1298 = f115(v0, 14u32, 50u32);
        let a1299 = f115(v0, 18u32, 46u32);
        fixed_u64::bit_xor(a1298, a1299)
    };
    let a1301 = f115(v0, 41u32, 23u32);
    fixed_u64::bit_xor(a1300, a1301)
}

pub fn f100(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1307 = {
        let a1302 = v0;
        let a1303 = v1;
        fixed_u64::bit_and(a1302, a1303)
    };
    let a1308 = {
        let a1305 = {
            let a1304 = v0;
            fixed_u64::bit_not(a1304)
        };
        let a1306 = v2;
        fixed_u64::bit_and(a1305, a1306)
    };
    fixed_u64::bit_xor(a1307, a1308)
}

pub fn f101(v0: u64) -> u64 {
    let a1311 = {
        let a1309 = f115(v0, 28u32, 36u32);
        let a1310 = f115(v0, 34u32, 30u32);
        fixed_u64::bit_xor(a1309, a1310)
    };
    let a1312 = f115(v0, 39u32, 25u32);
    fixed_u64::bit_xor(a1311, a1312)
}

pub fn f102(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1321 = {
        let a1317 = {
            let a1313 = v0;
            let a1314 = v1;
            fixed_u64::bit_and(a1313, a1314)
        };
        let a1318 = {
            let a1315 = v0;
            let a1316 = v2;
            fixed_u64::bit_and(a1315, a1316)
        };
        fixed_u64::bit_xor(a1317, a1318)
    };
    let a1322 = {
        let a1319 = v1;
        let a1320 = v2;
        fixed_u64::bit_and(a1319, a1320)
    };
    fixed_u64::bit_xor(a1321, a1322)
}

pub fn f103(v0: Adt6) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(_, h1323, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1323
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h1324, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1324
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h1325, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1325
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h1326, _, _, _, _, _, _, _, _, _, _, _) => {
            h1326
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h1327, _, _, _, _, _, _, _, _, _, _) => {
            h1327
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h1328, _, _, _, _, _, _, _, _, _) => {
            h1328
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h1329, _, _, _, _, _, _, _, _) => {
            h1329
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h1330, _, _, _, _, _, _, _) => {
            h1330
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1331, _, _, _, _, _, _) => {
            h1331
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1332, _, _, _, _, _) => {
            h1332
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h1333, _, _, _, _) => {
            h1333
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1334, _, _, _) => {
            h1334
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1335, _, _) => {
            h1335
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1336, _) => {
            h1336
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1337) => {
            h1337
        }
    }, f59(f116(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1338, _) => {
            h1338
        }
    }), f59(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1339, _, _, _, _, _, _) => {
            h1339
        }
    }, f59(f117(match v0.clone() {
        Adt6::C0(_, h1340, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1340
        }
    }), match v0 {
        Adt6::C0(h1341, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1341
        }
    }))))
}

pub fn f104(v0: Fn11, p1345: u64, p1346: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let m1344 = (p1345, p1346);
    {
        let mut m1344 = m1344;
        loop {
            let step = {
                let (v1, v2) = m1344;
                let m1342 = v1;
                if m1342 == 0 {
                    Err::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>(v2)
                } else {
                    let v3 = m1342 - 1;
                    Ok::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>((v3, {
                        let c1343: Fn11 = v0.clone();
                        c1343.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let v1: u64 = {
        let (h1347, _) = v0;
        h1347
    };
    let v2: u64 = {
        let (h1348, _) = {
            let (_, h1349) = v0;
            h1349
        };
        h1348
    };
    let v3: u64 = {
        let (_, h1350) = {
            let (_, h1351) = v0;
            h1351
        };
        h1350
    };
    ({
        let a1352 = v1;
        let a1353 = 2u64;
        let a1354 = 0u64;
        nat_quot(a1352, a1353, a1354)
    }, (f79(v2, 1u32), {
        let a1358 = {
            let a1355 = v1;
            let a1356 = 2u64;
            let a1357 = 0u64;
            nat_rem(a1355, a1356, a1357)
        };
        let a1359 = 1u64;
        if nat_eq(a1358, a1359) {
            let a1360 = v3;
            let a1361 = v2;
            fixed_u64::bit_or(a1360, a1361)
        } else {
            v3
        }
    }))
}

pub fn f106(v0: Bytes, v1: u64) -> u64 {
    let m1364 = {
        let a1362 = v0;
        let a1363 = v1;
        index_bytes(a1362, a1363)
    };
    match m1364 {
        None => {
            0u64
        }
        Some(v2) => {
            let m1366 = {
                let a1365 = v2;
                fixed_u64::convert(i128::from(a1365))
            };
            match m1366 {
                None => {
                    0u64
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f107(v0: Adt10) -> Adt10 {
    let a1371 = {
        let a1368 = match v0.clone() {
            Adt10::C0(_, h1367, _) => {
                h1367
            }
        };
        let a1369 = 2u64;
        let a1370 = 0u64;
        nat_rem(a1368, a1369, a1370)
    };
    let a1372 = 0u64;
    if nat_eq(a1371, a1372) {
        let m1374 = match v0.clone() {
            Adt10::C0(_, _, h1373) => {
                h1373
            }
        };
        match m1374.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt10::C0(f70(v1, match v0.clone() {
                    Adt10::C0(h1375, _, _) => {
                        h1375
                    }
                }, false), {
                    let a1377 = match v0 {
                        Adt10::C0(_, h1376, _) => {
                            h1376
                        }
                    };
                    let a1378 = 2u64;
                    let a1379 = 0u64;
                    nat_quot(a1377, a1378, a1379)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f108(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f118(match v0.clone() {
        Adt9::C0(h1380, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1380
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1381, _, _, _, _, _, _, _, _, _, _) => {
            h1381
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1382, _, _, _, _, _) => {
            h1382
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1383) => {
            h1383
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1384, _, _, _, _, _, _, _) => {
            h1384
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1385, _, _, _, _, _, _) => {
            h1385
        }
    });
    let v3: Adt11 = f118(match v0.clone() {
        Adt9::C0(_, h1386, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1386
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1387, _, _, _, _, _, _, _, _, _) => {
            h1387
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1388, _, _, _, _) => {
            h1388
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1389, _, _, _) => {
            h1389
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1390, _, _, _, _, _) => {
            h1390
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1391, _, _, _, _) => {
            h1391
        }
    });
    let v4: Adt11 = f118(match v0.clone() {
        Adt9::C0(_, _, h1392, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1392
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1393, _, _, _, _, _, _, _, _) => {
            h1393
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1394, _, _, _, _, _, _, _) => {
            h1394
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1395, _, _) => {
            h1395
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1396, _, _, _) => {
            h1396
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1397, _, _) => {
            h1397
        }
    });
    let v5: Adt11 = f118(match v0.clone() {
        Adt9::C0(_, _, _, h1398, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1398
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1399, _, _, _, _, _, _, _, _, _, _, _) => {
            h1399
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1400, _, _, _, _, _, _) => {
            h1400
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1401, _) => {
            h1401
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1402, _) => {
            h1402
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1403) => {
            h1403
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1404, _, _, _) => {
            h1404
        }
    }, match v3.clone() {
        Adt11::C0(h1405, _, _, _) => {
            h1405
        }
    }, match v4.clone() {
        Adt11::C0(h1406, _, _, _) => {
            h1406
        }
    }, match v5.clone() {
        Adt11::C0(h1407, _, _, _) => {
            h1407
        }
    }, match v5.clone() {
        Adt11::C0(_, h1408, _, _) => {
            h1408
        }
    }, match v2.clone() {
        Adt11::C0(_, h1409, _, _) => {
            h1409
        }
    }, match v3.clone() {
        Adt11::C0(_, h1410, _, _) => {
            h1410
        }
    }, match v4.clone() {
        Adt11::C0(_, h1411, _, _) => {
            h1411
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1412, _) => {
            h1412
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1413, _) => {
            h1413
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1414, _) => {
            h1414
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1415, _) => {
            h1415
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1416) => {
            h1416
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1417) => {
            h1417
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1418) => {
            h1418
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1419) => {
            h1419
        }
    })
}

pub fn f109(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f118(match v0.clone() {
        Adt9::C0(h1420, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1420
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1421, _, _, _, _, _, _, _, _, _, _, _) => {
            h1421
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1422, _, _, _, _, _, _, _) => {
            h1422
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1423, _, _, _) => {
            h1423
        }
    }, match v1.clone() {
        Adt8::C0(h1424, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1424
        }
    }, match v1.clone() {
        Adt8::C0(_, h1425, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1425
        }
    });
    let v3: Adt11 = f118(match v0.clone() {
        Adt9::C0(_, h1426, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1426
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1427, _, _, _, _, _, _, _, _, _, _) => {
            h1427
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1428, _, _, _, _, _, _) => {
            h1428
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1429, _, _) => {
            h1429
        }
    }, match v1.clone() {
        Adt8::C0(_, _, h1430, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1430
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, h1431, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1431
        }
    });
    let v4: Adt11 = f118(match v0.clone() {
        Adt9::C0(_, _, h1432, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1432
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1433, _, _, _, _, _, _, _, _, _) => {
            h1433
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1434, _, _, _, _, _) => {
            h1434
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1435, _) => {
            h1435
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, h1436, _, _, _, _, _, _, _, _, _, _, _) => {
            h1436
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, h1437, _, _, _, _, _, _, _, _, _, _) => {
            h1437
        }
    });
    let v5: Adt11 = f118(match v0.clone() {
        Adt9::C0(_, _, _, h1438, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1438
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1439, _, _, _, _, _, _, _, _) => {
            h1439
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1440, _, _, _, _) => {
            h1440
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1441) => {
            h1441
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, h1442, _, _, _, _, _, _, _, _, _) => {
            h1442
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, h1443, _, _, _, _, _, _, _, _) => {
            h1443
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1444, _, _, _) => {
            h1444
        }
    }, match v3.clone() {
        Adt11::C0(h1445, _, _, _) => {
            h1445
        }
    }, match v4.clone() {
        Adt11::C0(h1446, _, _, _) => {
            h1446
        }
    }, match v5.clone() {
        Adt11::C0(h1447, _, _, _) => {
            h1447
        }
    }, match v2.clone() {
        Adt11::C0(_, h1448, _, _) => {
            h1448
        }
    }, match v3.clone() {
        Adt11::C0(_, h1449, _, _) => {
            h1449
        }
    }, match v4.clone() {
        Adt11::C0(_, h1450, _, _) => {
            h1450
        }
    }, match v5.clone() {
        Adt11::C0(_, h1451, _, _) => {
            h1451
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1452, _) => {
            h1452
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1453, _) => {
            h1453
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1454, _) => {
            h1454
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1455, _) => {
            h1455
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1456) => {
            h1456
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1457) => {
            h1457
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1458) => {
            h1458
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1459) => {
            h1459
        }
    })
}

pub fn f110(v0: Adt8) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt8::C0(_, _, h1460, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1460
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h1461, _, _, _, _, _, _, _, _, _) => {
            h1461
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, h1462, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1462
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1463, _, _, _, _, _) => {
            h1463
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h1464, _, _, _, _, _, _, _, _) => {
            h1464
        }
    }, match v0.clone() {
        Adt8::C0(h1465, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1465
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h1466, _, _, _, _, _, _, _, _, _, _, _) => {
            h1466
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1467, _, _) => {
            h1467
        }
    }, match v0.clone() {
        Adt8::C0(_, h1468, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1468
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1469, _, _, _, _) => {
            h1469
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1470, _, _, _) => {
            h1470
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h1471, _, _, _, _, _, _, _, _, _, _) => {
            h1471
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1472, _, _, _, _, _, _) => {
            h1472
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1473, _) => {
            h1473
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1474) => {
            h1474
        }
    }, match v0 {
        Adt8::C0(_, _, _, _, _, _, _, _, h1475, _, _, _, _, _, _, _) => {
            h1475
        }
    })
}

pub fn f111(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1476 = f33(v0, v1);
    let a1477 = f87(v0, v2);
    fixed_u32::bit_or(a1476, a1477)
}

pub fn f112(v0: u32) -> u32 {
    let a1480 = {
        let a1478 = f111(v0, 17u32, 15u32);
        let a1479 = f111(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1478, a1479)
    };
    let a1481 = f33(v0, 10u32);
    fixed_u32::bit_xor(a1480, a1481)
}

pub fn f113(v0: u32) -> u32 {
    let a1484 = {
        let a1482 = f111(v0, 7u32, 25u32);
        let a1483 = f111(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1482, a1483)
    };
    let a1485 = f33(v0, 3u32);
    fixed_u32::bit_xor(a1484, a1485)
}

pub fn f114(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a1487 = {
            let (h1486, _) = v0.clone();
            h1486
        };
        let a1488 = 1u64;
        nat_add(a1487, a1488)?
    }, f119(f120(f121(f122({
        let (_, h1489) = v0;
        h1489
    }))), {
        let (h1490, _) = v0;
        h1490
    })))
}

pub fn f115(v0: u64, v1: u32, v2: u32) -> u64 {
    let a1491 = f42(v0, v1);
    let a1492 = f79(v0, v2);
    fixed_u64::bit_or(a1491, a1492)
}

pub fn f116(v0: u64) -> u64 {
    let a1495 = {
        let a1493 = f115(v0, 19u32, 45u32);
        let a1494 = f115(v0, 61u32, 3u32);
        fixed_u64::bit_xor(a1493, a1494)
    };
    let a1496 = f42(v0, 6u32);
    fixed_u64::bit_xor(a1495, a1496)
}

pub fn f117(v0: u64) -> u64 {
    let a1499 = {
        let a1497 = f115(v0, 1u32, 63u32);
        let a1498 = f115(v0, 8u32, 56u32);
        fixed_u64::bit_xor(a1497, a1498)
    };
    let a1500 = f42(v0, 7u32);
    fixed_u64::bit_xor(a1499, a1500)
}

pub fn f118(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt11 {
    let v6: u32 = f54(f54(v0, v1), v4);
    let v7: u32 = f111({
        let a1501 = v3;
        let a1502 = v6;
        fixed_u32::bit_xor(a1501, a1502)
    }, 16u32, 16u32);
    let v8: u32 = f54(v2, v7);
    let v9: u32 = f111({
        let a1503 = v1;
        let a1504 = v8;
        fixed_u32::bit_xor(a1503, a1504)
    }, 12u32, 20u32);
    let v10: u32 = f54(f54(v6, v9), v5);
    let v11: u32 = f111({
        let a1505 = v7;
        let a1506 = v10;
        fixed_u32::bit_xor(a1505, a1506)
    }, 8u32, 24u32);
    let v12: u32 = f54(v8, v11);
    let v13: u32 = f111({
        let a1507 = v9;
        let a1508 = v12;
        fixed_u32::bit_xor(a1507, a1508)
    }, 7u32, 25u32);
    Adt11::C0(v10, v13, v12, v11)
}

pub fn f119(v0: Adt3, v1: u64) -> Adt3 {
    let v2: u64 = f123(v1);
    Adt3::C0({
        let a1510 = match v0.clone() {
            Adt3::C0(h1509, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1509
            }
        };
        let a1511 = v2;
        fixed_u64::bit_xor(a1510, a1511)
    }, match v0.clone() {
        Adt3::C0(_, h1512, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1512
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1513, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1513
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1514, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1514
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h1515, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1515
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1516, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1516
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1517, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1517
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1518, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1518
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1519, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1519
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1520, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1520
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1521, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1521
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1522, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1522
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1523, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1523
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1524, _, _, _, _, _, _, _, _, _, _, _) => {
            h1524
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1525, _, _, _, _, _, _, _, _, _, _) => {
            h1525
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1526, _, _, _, _, _, _, _, _, _) => {
            h1526
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1527, _, _, _, _, _, _, _, _) => {
            h1527
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1528, _, _, _, _, _, _, _) => {
            h1528
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1529, _, _, _, _, _, _) => {
            h1529
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1530, _, _, _, _, _) => {
            h1530
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1531, _, _, _, _) => {
            h1531
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1532, _, _, _) => {
            h1532
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1533, _, _) => {
            h1533
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1534, _) => {
            h1534
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1535) => {
            h1535
        }
    })
}

pub fn f120(v0: Adt3) -> Adt3 {
    Adt3::C0({
        let a1542 = match v0.clone() {
            Adt3::C0(h1536, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1536
            }
        };
        let a1543 = {
            let a1540 = {
                let a1538 = match v0.clone() {
                    Adt3::C0(_, h1537, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1537
                    }
                };
                fixed_u64::bit_not(a1538)
            };
            let a1541 = match v0.clone() {
                Adt3::C0(_, _, h1539, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1539
                }
            };
            fixed_u64::bit_and(a1540, a1541)
        };
        fixed_u64::bit_xor(a1542, a1543)
    }, {
        let a1550 = match v0.clone() {
            Adt3::C0(_, h1544, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1544
            }
        };
        let a1551 = {
            let a1548 = {
                let a1546 = match v0.clone() {
                    Adt3::C0(_, _, h1545, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1545
                    }
                };
                fixed_u64::bit_not(a1546)
            };
            let a1549 = match v0.clone() {
                Adt3::C0(_, _, _, h1547, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1547
                }
            };
            fixed_u64::bit_and(a1548, a1549)
        };
        fixed_u64::bit_xor(a1550, a1551)
    }, {
        let a1558 = match v0.clone() {
            Adt3::C0(_, _, h1552, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1552
            }
        };
        let a1559 = {
            let a1556 = {
                let a1554 = match v0.clone() {
                    Adt3::C0(_, _, _, h1553, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1553
                    }
                };
                fixed_u64::bit_not(a1554)
            };
            let a1557 = match v0.clone() {
                Adt3::C0(_, _, _, _, h1555, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1555
                }
            };
            fixed_u64::bit_and(a1556, a1557)
        };
        fixed_u64::bit_xor(a1558, a1559)
    }, {
        let a1566 = match v0.clone() {
            Adt3::C0(_, _, _, h1560, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1560
            }
        };
        let a1567 = {
            let a1564 = {
                let a1562 = match v0.clone() {
                    Adt3::C0(_, _, _, _, h1561, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1561
                    }
                };
                fixed_u64::bit_not(a1562)
            };
            let a1565 = match v0.clone() {
                Adt3::C0(h1563, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1563
                }
            };
            fixed_u64::bit_and(a1564, a1565)
        };
        fixed_u64::bit_xor(a1566, a1567)
    }, {
        let a1574 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1568, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1568
            }
        };
        let a1575 = {
            let a1572 = {
                let a1570 = match v0.clone() {
                    Adt3::C0(h1569, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1569
                    }
                };
                fixed_u64::bit_not(a1570)
            };
            let a1573 = match v0.clone() {
                Adt3::C0(_, h1571, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1571
                }
            };
            fixed_u64::bit_and(a1572, a1573)
        };
        fixed_u64::bit_xor(a1574, a1575)
    }, {
        let a1582 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1576, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1576
            }
        };
        let a1583 = {
            let a1580 = {
                let a1578 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1577, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1577
                    }
                };
                fixed_u64::bit_not(a1578)
            };
            let a1581 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h1579, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1579
                }
            };
            fixed_u64::bit_and(a1580, a1581)
        };
        fixed_u64::bit_xor(a1582, a1583)
    }, {
        let a1590 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1584, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1584
            }
        };
        let a1591 = {
            let a1588 = {
                let a1586 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, h1585, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1585
                    }
                };
                fixed_u64::bit_not(a1586)
            };
            let a1589 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, h1587, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1587
                }
            };
            fixed_u64::bit_and(a1588, a1589)
        };
        fixed_u64::bit_xor(a1590, a1591)
    }, {
        let a1598 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1592, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1592
            }
        };
        let a1599 = {
            let a1596 = {
                let a1594 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, h1593, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1593
                    }
                };
                fixed_u64::bit_not(a1594)
            };
            let a1597 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, h1595, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1595
                }
            };
            fixed_u64::bit_and(a1596, a1597)
        };
        fixed_u64::bit_xor(a1598, a1599)
    }, {
        let a1606 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1600, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1600
            }
        };
        let a1607 = {
            let a1604 = {
                let a1602 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, h1601, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1601
                    }
                };
                fixed_u64::bit_not(a1602)
            };
            let a1605 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, h1603, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1603
                }
            };
            fixed_u64::bit_and(a1604, a1605)
        };
        fixed_u64::bit_xor(a1606, a1607)
    }, {
        let a1614 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1608, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1608
            }
        };
        let a1615 = {
            let a1612 = {
                let a1610 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, h1609, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1609
                    }
                };
                fixed_u64::bit_not(a1610)
            };
            let a1613 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, h1611, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1611
                }
            };
            fixed_u64::bit_and(a1612, a1613)
        };
        fixed_u64::bit_xor(a1614, a1615)
    }, {
        let a1622 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1616, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1616
            }
        };
        let a1623 = {
            let a1620 = {
                let a1618 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1617, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1617
                    }
                };
                fixed_u64::bit_not(a1618)
            };
            let a1621 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1619, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1619
                }
            };
            fixed_u64::bit_and(a1620, a1621)
        };
        fixed_u64::bit_xor(a1622, a1623)
    }, {
        let a1630 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1624, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1624
            }
        };
        let a1631 = {
            let a1628 = {
                let a1626 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1625, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1625
                    }
                };
                fixed_u64::bit_not(a1626)
            };
            let a1629 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1627, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1627
                }
            };
            fixed_u64::bit_and(a1628, a1629)
        };
        fixed_u64::bit_xor(a1630, a1631)
    }, {
        let a1638 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1632, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1632
            }
        };
        let a1639 = {
            let a1636 = {
                let a1634 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1633, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1633
                    }
                };
                fixed_u64::bit_not(a1634)
            };
            let a1637 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1635, _, _, _, _, _, _, _, _, _, _) => {
                    h1635
                }
            };
            fixed_u64::bit_and(a1636, a1637)
        };
        fixed_u64::bit_xor(a1638, a1639)
    }, {
        let a1646 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1640, _, _, _, _, _, _, _, _, _, _, _) => {
                h1640
            }
        };
        let a1647 = {
            let a1644 = {
                let a1642 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1641, _, _, _, _, _, _, _, _, _, _) => {
                        h1641
                    }
                };
                fixed_u64::bit_not(a1642)
            };
            let a1645 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1643, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1643
                }
            };
            fixed_u64::bit_and(a1644, a1645)
        };
        fixed_u64::bit_xor(a1646, a1647)
    }, {
        let a1654 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1648, _, _, _, _, _, _, _, _, _, _) => {
                h1648
            }
        };
        let a1655 = {
            let a1652 = {
                let a1650 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1649, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1649
                    }
                };
                fixed_u64::bit_not(a1650)
            };
            let a1653 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1651, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1651
                }
            };
            fixed_u64::bit_and(a1652, a1653)
        };
        fixed_u64::bit_xor(a1654, a1655)
    }, {
        let a1662 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1656, _, _, _, _, _, _, _, _, _) => {
                h1656
            }
        };
        let a1663 = {
            let a1660 = {
                let a1658 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1657, _, _, _, _, _, _, _, _) => {
                        h1657
                    }
                };
                fixed_u64::bit_not(a1658)
            };
            let a1661 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1659, _, _, _, _, _, _, _) => {
                    h1659
                }
            };
            fixed_u64::bit_and(a1660, a1661)
        };
        fixed_u64::bit_xor(a1662, a1663)
    }, {
        let a1670 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1664, _, _, _, _, _, _, _, _) => {
                h1664
            }
        };
        let a1671 = {
            let a1668 = {
                let a1666 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1665, _, _, _, _, _, _, _) => {
                        h1665
                    }
                };
                fixed_u64::bit_not(a1666)
            };
            let a1669 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1667, _, _, _, _, _, _) => {
                    h1667
                }
            };
            fixed_u64::bit_and(a1668, a1669)
        };
        fixed_u64::bit_xor(a1670, a1671)
    }, {
        let a1678 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1672, _, _, _, _, _, _, _) => {
                h1672
            }
        };
        let a1679 = {
            let a1676 = {
                let a1674 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1673, _, _, _, _, _, _) => {
                        h1673
                    }
                };
                fixed_u64::bit_not(a1674)
            };
            let a1677 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1675, _, _, _, _, _) => {
                    h1675
                }
            };
            fixed_u64::bit_and(a1676, a1677)
        };
        fixed_u64::bit_xor(a1678, a1679)
    }, {
        let a1686 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1680, _, _, _, _, _, _) => {
                h1680
            }
        };
        let a1687 = {
            let a1684 = {
                let a1682 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1681, _, _, _, _, _) => {
                        h1681
                    }
                };
                fixed_u64::bit_not(a1682)
            };
            let a1685 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1683, _, _, _, _, _, _, _, _, _) => {
                    h1683
                }
            };
            fixed_u64::bit_and(a1684, a1685)
        };
        fixed_u64::bit_xor(a1686, a1687)
    }, {
        let a1694 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1688, _, _, _, _, _) => {
                h1688
            }
        };
        let a1695 = {
            let a1692 = {
                let a1690 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1689, _, _, _, _, _, _, _, _, _) => {
                        h1689
                    }
                };
                fixed_u64::bit_not(a1690)
            };
            let a1693 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1691, _, _, _, _, _, _, _, _) => {
                    h1691
                }
            };
            fixed_u64::bit_and(a1692, a1693)
        };
        fixed_u64::bit_xor(a1694, a1695)
    }, {
        let a1702 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1696, _, _, _, _) => {
                h1696
            }
        };
        let a1703 = {
            let a1700 = {
                let a1698 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1697, _, _, _) => {
                        h1697
                    }
                };
                fixed_u64::bit_not(a1698)
            };
            let a1701 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1699, _, _) => {
                    h1699
                }
            };
            fixed_u64::bit_and(a1700, a1701)
        };
        fixed_u64::bit_xor(a1702, a1703)
    }, {
        let a1710 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1704, _, _, _) => {
                h1704
            }
        };
        let a1711 = {
            let a1708 = {
                let a1706 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1705, _, _) => {
                        h1705
                    }
                };
                fixed_u64::bit_not(a1706)
            };
            let a1709 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1707, _) => {
                    h1707
                }
            };
            fixed_u64::bit_and(a1708, a1709)
        };
        fixed_u64::bit_xor(a1710, a1711)
    }, {
        let a1718 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1712, _, _) => {
                h1712
            }
        };
        let a1719 = {
            let a1716 = {
                let a1714 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1713, _) => {
                        h1713
                    }
                };
                fixed_u64::bit_not(a1714)
            };
            let a1717 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1715) => {
                    h1715
                }
            };
            fixed_u64::bit_and(a1716, a1717)
        };
        fixed_u64::bit_xor(a1718, a1719)
    }, {
        let a1726 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1720, _) => {
                h1720
            }
        };
        let a1727 = {
            let a1724 = {
                let a1722 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1721) => {
                        h1721
                    }
                };
                fixed_u64::bit_not(a1722)
            };
            let a1725 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1723, _, _, _, _) => {
                    h1723
                }
            };
            fixed_u64::bit_and(a1724, a1725)
        };
        fixed_u64::bit_xor(a1726, a1727)
    }, {
        let a1734 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1728) => {
                h1728
            }
        };
        let a1735 = {
            let a1732 = {
                let a1730 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1729, _, _, _, _) => {
                        h1729
                    }
                };
                fixed_u64::bit_not(a1730)
            };
            let a1733 = match v0 {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1731, _, _, _) => {
                    h1731
                }
            };
            fixed_u64::bit_and(a1732, a1733)
        };
        fixed_u64::bit_xor(a1734, a1735)
    })
}

pub fn f121(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h1736, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1736
        }
    }, f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1737, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1737
        }
    }, 44u32, 20u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1738, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1738
        }
    }, 43u32, 21u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1739, _, _, _, _, _, _) => {
            h1739
        }
    }, 21u32, 43u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1740) => {
            h1740
        }
    }, 14u32, 50u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, h1741, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1741
        }
    }, 28u32, 36u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1742, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1742
        }
    }, 20u32, 44u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1743, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1743
        }
    }, 3u32, 61u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1744, _, _, _, _, _, _, _, _) => {
            h1744
        }
    }, 45u32, 19u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1745, _, _) => {
            h1745
        }
    }, 61u32, 3u32), f124(match v0.clone() {
        Adt3::C0(_, h1746, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1746
        }
    }, 1u32, 63u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1747, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1747
        }
    }, 6u32, 58u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1748, _, _, _, _, _, _, _, _, _, _, _) => {
            h1748
        }
    }, 25u32, 39u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1749, _, _, _, _, _) => {
            h1749
        }
    }, 8u32, 56u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1750, _, _, _, _) => {
            h1750
        }
    }, 18u32, 46u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, h1751, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1751
        }
    }, 27u32, 37u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1752, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1752
        }
    }, 36u32, 28u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1753, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1753
        }
    }, 10u32, 54u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1754, _, _, _, _, _, _, _) => {
            h1754
        }
    }, 15u32, 49u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1755, _) => {
            h1755
        }
    }, 56u32, 8u32), f124(match v0.clone() {
        Adt3::C0(_, _, h1756, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1756
        }
    }, 62u32, 2u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1757, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1757
        }
    }, 55u32, 9u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1758, _, _, _, _, _, _, _, _, _, _) => {
            h1758
        }
    }, 39u32, 25u32), f124(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1759, _, _, _, _, _, _, _, _, _) => {
            h1759
        }
    }, 41u32, 23u32), f124(match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1760, _, _, _) => {
            h1760
        }
    }, 2u32, 62u32))
}

pub fn f122(v0: Adt3) -> Adt3 {
    let v1: u64 = {
        let a1772 = {
            let a1769 = {
                let a1766 = {
                    let a1763 = match v0.clone() {
                        Adt3::C0(h1761, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1761
                        }
                    };
                    let a1764 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1762, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1762
                        }
                    };
                    fixed_u64::bit_xor(a1763, a1764)
                };
                let a1767 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1765, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1765
                    }
                };
                fixed_u64::bit_xor(a1766, a1767)
            };
            let a1770 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1768, _, _, _, _, _, _, _, _, _) => {
                    h1768
                }
            };
            fixed_u64::bit_xor(a1769, a1770)
        };
        let a1773 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1771, _, _, _, _) => {
                h1771
            }
        };
        fixed_u64::bit_xor(a1772, a1773)
    };
    let v2: u64 = {
        let a1785 = {
            let a1782 = {
                let a1779 = {
                    let a1776 = match v0.clone() {
                        Adt3::C0(_, h1774, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1774
                        }
                    };
                    let a1777 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, h1775, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1775
                        }
                    };
                    fixed_u64::bit_xor(a1776, a1777)
                };
                let a1780 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1778, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1778
                    }
                };
                fixed_u64::bit_xor(a1779, a1780)
            };
            let a1783 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1781, _, _, _, _, _, _, _, _) => {
                    h1781
                }
            };
            fixed_u64::bit_xor(a1782, a1783)
        };
        let a1786 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1784, _, _, _) => {
                h1784
            }
        };
        fixed_u64::bit_xor(a1785, a1786)
    };
    let v3: u64 = {
        let a1798 = {
            let a1795 = {
                let a1792 = {
                    let a1789 = match v0.clone() {
                        Adt3::C0(_, _, h1787, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1787
                        }
                    };
                    let a1790 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, h1788, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1788
                        }
                    };
                    fixed_u64::bit_xor(a1789, a1790)
                };
                let a1793 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1791, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1791
                    }
                };
                fixed_u64::bit_xor(a1792, a1793)
            };
            let a1796 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1794, _, _, _, _, _, _, _) => {
                    h1794
                }
            };
            fixed_u64::bit_xor(a1795, a1796)
        };
        let a1799 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1797, _, _) => {
                h1797
            }
        };
        fixed_u64::bit_xor(a1798, a1799)
    };
    let v4: u64 = {
        let a1811 = {
            let a1808 = {
                let a1805 = {
                    let a1802 = match v0.clone() {
                        Adt3::C0(_, _, _, h1800, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1800
                        }
                    };
                    let a1803 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, h1801, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1801
                        }
                    };
                    fixed_u64::bit_xor(a1802, a1803)
                };
                let a1806 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1804, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1804
                    }
                };
                fixed_u64::bit_xor(a1805, a1806)
            };
            let a1809 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1807, _, _, _, _, _, _) => {
                    h1807
                }
            };
            fixed_u64::bit_xor(a1808, a1809)
        };
        let a1812 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1810, _) => {
                h1810
            }
        };
        fixed_u64::bit_xor(a1811, a1812)
    };
    let v5: u64 = {
        let a1824 = {
            let a1821 = {
                let a1818 = {
                    let a1815 = match v0.clone() {
                        Adt3::C0(_, _, _, _, h1813, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1813
                        }
                    };
                    let a1816 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, _, h1814, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1814
                        }
                    };
                    fixed_u64::bit_xor(a1815, a1816)
                };
                let a1819 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1817, _, _, _, _, _, _, _, _, _, _) => {
                        h1817
                    }
                };
                fixed_u64::bit_xor(a1818, a1819)
            };
            let a1822 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1820, _, _, _, _, _) => {
                    h1820
                }
            };
            fixed_u64::bit_xor(a1821, a1822)
        };
        let a1825 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1823) => {
                h1823
            }
        };
        fixed_u64::bit_xor(a1824, a1825)
    };
    let v6: u64 = {
        let a1826 = v5;
        let a1827 = f124(v2, 1u32, 63u32);
        fixed_u64::bit_xor(a1826, a1827)
    };
    let v7: u64 = {
        let a1828 = v1;
        let a1829 = f124(v3, 1u32, 63u32);
        fixed_u64::bit_xor(a1828, a1829)
    };
    let v8: u64 = {
        let a1830 = v2;
        let a1831 = f124(v4, 1u32, 63u32);
        fixed_u64::bit_xor(a1830, a1831)
    };
    let v9: u64 = {
        let a1832 = v3;
        let a1833 = f124(v5, 1u32, 63u32);
        fixed_u64::bit_xor(a1832, a1833)
    };
    let v10: u64 = {
        let a1834 = v4;
        let a1835 = f124(v1, 1u32, 63u32);
        fixed_u64::bit_xor(a1834, a1835)
    };
    Adt3::C0({
        let a1837 = match v0.clone() {
            Adt3::C0(h1836, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1836
            }
        };
        let a1838 = v6;
        fixed_u64::bit_xor(a1837, a1838)
    }, {
        let a1840 = match v0.clone() {
            Adt3::C0(_, h1839, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1839
            }
        };
        let a1841 = v7;
        fixed_u64::bit_xor(a1840, a1841)
    }, {
        let a1843 = match v0.clone() {
            Adt3::C0(_, _, h1842, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1842
            }
        };
        let a1844 = v8;
        fixed_u64::bit_xor(a1843, a1844)
    }, {
        let a1846 = match v0.clone() {
            Adt3::C0(_, _, _, h1845, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1845
            }
        };
        let a1847 = v9;
        fixed_u64::bit_xor(a1846, a1847)
    }, {
        let a1849 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1848, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1848
            }
        };
        let a1850 = v10;
        fixed_u64::bit_xor(a1849, a1850)
    }, {
        let a1852 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1851, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1851
            }
        };
        let a1853 = v6;
        fixed_u64::bit_xor(a1852, a1853)
    }, {
        let a1855 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1854, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1854
            }
        };
        let a1856 = v7;
        fixed_u64::bit_xor(a1855, a1856)
    }, {
        let a1858 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1857, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1857
            }
        };
        let a1859 = v8;
        fixed_u64::bit_xor(a1858, a1859)
    }, {
        let a1861 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1860, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1860
            }
        };
        let a1862 = v9;
        fixed_u64::bit_xor(a1861, a1862)
    }, {
        let a1864 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1863, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1863
            }
        };
        let a1865 = v10;
        fixed_u64::bit_xor(a1864, a1865)
    }, {
        let a1867 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1866, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1866
            }
        };
        let a1868 = v6;
        fixed_u64::bit_xor(a1867, a1868)
    }, {
        let a1870 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1869, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1869
            }
        };
        let a1871 = v7;
        fixed_u64::bit_xor(a1870, a1871)
    }, {
        let a1873 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1872, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1872
            }
        };
        let a1874 = v8;
        fixed_u64::bit_xor(a1873, a1874)
    }, {
        let a1876 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1875, _, _, _, _, _, _, _, _, _, _, _) => {
                h1875
            }
        };
        let a1877 = v9;
        fixed_u64::bit_xor(a1876, a1877)
    }, {
        let a1879 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1878, _, _, _, _, _, _, _, _, _, _) => {
                h1878
            }
        };
        let a1880 = v10;
        fixed_u64::bit_xor(a1879, a1880)
    }, {
        let a1882 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1881, _, _, _, _, _, _, _, _, _) => {
                h1881
            }
        };
        let a1883 = v6;
        fixed_u64::bit_xor(a1882, a1883)
    }, {
        let a1885 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1884, _, _, _, _, _, _, _, _) => {
                h1884
            }
        };
        let a1886 = v7;
        fixed_u64::bit_xor(a1885, a1886)
    }, {
        let a1888 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1887, _, _, _, _, _, _, _) => {
                h1887
            }
        };
        let a1889 = v8;
        fixed_u64::bit_xor(a1888, a1889)
    }, {
        let a1891 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1890, _, _, _, _, _, _) => {
                h1890
            }
        };
        let a1892 = v9;
        fixed_u64::bit_xor(a1891, a1892)
    }, {
        let a1894 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1893, _, _, _, _, _) => {
                h1893
            }
        };
        let a1895 = v10;
        fixed_u64::bit_xor(a1894, a1895)
    }, {
        let a1897 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1896, _, _, _, _) => {
                h1896
            }
        };
        let a1898 = v6;
        fixed_u64::bit_xor(a1897, a1898)
    }, {
        let a1900 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1899, _, _, _) => {
                h1899
            }
        };
        let a1901 = v7;
        fixed_u64::bit_xor(a1900, a1901)
    }, {
        let a1903 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1902, _, _) => {
                h1902
            }
        };
        let a1904 = v8;
        fixed_u64::bit_xor(a1903, a1904)
    }, {
        let a1906 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1905, _) => {
                h1905
            }
        };
        let a1907 = v9;
        fixed_u64::bit_xor(a1906, a1907)
    }, {
        let a1909 = match v0 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1908) => {
                h1908
            }
        };
        let a1910 = v10;
        fixed_u64::bit_xor(a1909, a1910)
    })
}

pub fn f123(v0: u64) -> u64 {
    let a1911 = v0;
    let a1912 = 12u64;
    if nat_lt(a1911, a1912) {
        let a1913 = v0;
        let a1914 = 6u64;
        if nat_lt(a1913, a1914) {
            let a1915 = v0;
            let a1916 = 3u64;
            if nat_lt(a1915, a1916) {
                let a1917 = v0;
                let a1918 = 1u64;
                if nat_lt(a1917, a1918) {
                    1u64
                } else {
                    let a1919 = v0;
                    let a1920 = 2u64;
                    if nat_lt(a1919, a1920) {
                        32898u64
                    } else {
                        9223372036854808714u64
                    }
                }
            } else {
                let a1921 = v0;
                let a1922 = 4u64;
                if nat_lt(a1921, a1922) {
                    9223372039002292224u64
                } else {
                    let a1923 = v0;
                    let a1924 = 5u64;
                    if nat_lt(a1923, a1924) {
                        32907u64
                    } else {
                        2147483649u64
                    }
                }
            }
        } else {
            let a1925 = v0;
            let a1926 = 9u64;
            if nat_lt(a1925, a1926) {
                let a1927 = v0;
                let a1928 = 7u64;
                if nat_lt(a1927, a1928) {
                    9223372039002292353u64
                } else {
                    let a1929 = v0;
                    let a1930 = 8u64;
                    if nat_lt(a1929, a1930) {
                        9223372036854808585u64
                    } else {
                        138u64
                    }
                }
            } else {
                let a1931 = v0;
                let a1932 = 10u64;
                if nat_lt(a1931, a1932) {
                    136u64
                } else {
                    let a1933 = v0;
                    let a1934 = 11u64;
                    if nat_lt(a1933, a1934) {
                        2147516425u64
                    } else {
                        2147483658u64
                    }
                }
            }
        }
    } else {
        let a1935 = v0;
        let a1936 = 18u64;
        if nat_lt(a1935, a1936) {
            let a1937 = v0;
            let a1938 = 15u64;
            if nat_lt(a1937, a1938) {
                let a1939 = v0;
                let a1940 = 13u64;
                if nat_lt(a1939, a1940) {
                    2147516555u64
                } else {
                    let a1941 = v0;
                    let a1942 = 14u64;
                    if nat_lt(a1941, a1942) {
                        9223372036854775947u64
                    } else {
                        9223372036854808713u64
                    }
                }
            } else {
                let a1943 = v0;
                let a1944 = 16u64;
                if nat_lt(a1943, a1944) {
                    9223372036854808579u64
                } else {
                    let a1945 = v0;
                    let a1946 = 17u64;
                    if nat_lt(a1945, a1946) {
                        9223372036854808578u64
                    } else {
                        9223372036854775936u64
                    }
                }
            }
        } else {
            let a1947 = v0;
            let a1948 = 21u64;
            if nat_lt(a1947, a1948) {
                let a1949 = v0;
                let a1950 = 19u64;
                if nat_lt(a1949, a1950) {
                    32778u64
                } else {
                    let a1951 = v0;
                    let a1952 = 20u64;
                    if nat_lt(a1951, a1952) {
                        9223372039002259466u64
                    } else {
                        9223372039002292353u64
                    }
                }
            } else {
                let a1953 = v0;
                let a1954 = 22u64;
                if nat_lt(a1953, a1954) {
                    9223372036854808704u64
                } else {
                    let a1955 = v0;
                    let a1956 = 23u64;
                    if nat_lt(a1955, a1956) {
                        2147483649u64
                    } else {
                        9223372039002292232u64
                    }
                }
            }
        }
    }
}

pub fn f124(v0: u64, v1: u32, v2: u32) -> u64 {
    let a1957 = f79(v0, v1);
    let a1958 = f42(v0, v2);
    fixed_u64::bit_or(a1957, a1958)
}
