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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt1, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F11(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F11(k0) => f11(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F17(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn1::F17(k0, k1, k2) => f17(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F25,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn2::F25 => f25(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F41,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn3::F41 => f41(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f1(v0.clone())? {
        if f1(v1.clone())? {
            Ok(Some(f2({
                let a1 = f3(v0.clone(), v1.clone());
                let a2 = f4(v0, v1);
                append_bytes(a1, a2)
            })?))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1(v0: Bytes) -> R<bool> {
    let a4 = {
        let a3 = v0.clone();
        length_bytes(a3)
    };
    let a5 = 71u64;
    if nat_eq(a4, a5) {
        if f5(v0.clone(), f6(Str::lit("sha256:"))) {
            let a6 = f7(v0, 7u64)?;
            let a7 = 71u64;
            Ok(nat_eq(a6, a7))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f2(v0: Bytes) -> R<Bytes> {
    let a8 = f6(Str::lit("sha256:"));
    let a9 = f8(v0)?;
    Ok(append_bytes(a8, a9))
}

pub fn f3(v0: Bytes, v1: Bytes) -> Bytes {
    if f9(v1.clone(), v0.clone()) {
        v1
    } else {
        v0
    }
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    if f9(v1.clone(), v0.clone()) {
        v0
    } else {
        v1
    }
}

pub fn f5(v0: Bytes, v1: Bytes) -> bool {
    let m14 = {
        let a11 = v0;
        let a12 = 0u64;
        let a13 = {
            let a10 = v1.clone();
            length_bytes(a10)
        };
        slice_bytes(a11, a12, a13)
    };
    match m14 {
        None => {
            false
        }
        Some(v2) => {
            let a15 = v2;
            let a16 = v1;
            equal(a15, a16)
        }
    }
}

pub fn f6(v0: Str) -> Bytes {
    let a17 = v0;
    utf8_encode(a17)
}

pub fn f7(v0: Bytes, v1: u64) -> R<u64> {
    let (h18, _) = f10(Fn0::F11(v0.clone()), {
        let a19 = v0;
        length_bytes(a19)
    }, v1)?;
    Ok(h18)
}

pub fn f8(v0: Bytes) -> R<Bytes> {
    Ok(f12(f13(v0)?))
}

pub fn f9(v0: Bytes, v1: Bytes) -> bool {
    let a24 = {
        let a20 = v0;
        let a21 = v1;
        compare_bytes(a20, a21)
    };
    let a25 = {
        let a22 = Bytes::lit(&[]);
        let a23 = Bytes::lit(&[0x01]);
        compare_bytes(a22, a23)
    };
    equal(a24, a25)
}

pub fn f10(v0: Fn0, p30: u64, p31: u64) -> R<(u64, bool)> {
    let m29 = (p30, p31);
    {
        let mut m29 = m29;
        loop {
            let step = {
                let (v1, v2) = m29;
                let m26 = v1;
                if m26 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m26 - 1;
                    let m28 = {
                        let c27: Fn0 = v0.clone();
                        c27.apply(v2)?
                    };
                    match m28 {
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
                Ok(next) => { m29 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f14(v0, v1)? {
        Ok(Some({
            let a32 = v1;
            let a33 = 1u64;
            nat_add(a32, a33)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f12(v0: Adt0) -> Bytes {
    let a54 = f15(match v0.clone() {
        Adt0::C0(h34, _, _, _, _, _, _, _) => {
            h34
        }
    });
    let a55 = {
        let a52 = f15(match v0.clone() {
            Adt0::C0(_, h35, _, _, _, _, _, _) => {
                h35
            }
        });
        let a53 = {
            let a50 = f15(match v0.clone() {
                Adt0::C0(_, _, h36, _, _, _, _, _) => {
                    h36
                }
            });
            let a51 = {
                let a48 = f15(match v0.clone() {
                    Adt0::C0(_, _, _, h37, _, _, _, _) => {
                        h37
                    }
                });
                let a49 = {
                    let a46 = f15(match v0.clone() {
                        Adt0::C0(_, _, _, _, h38, _, _, _) => {
                            h38
                        }
                    });
                    let a47 = {
                        let a44 = f15(match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h39, _, _) => {
                                h39
                            }
                        });
                        let a45 = {
                            let a42 = f15(match v0.clone() {
                                Adt0::C0(_, _, _, _, _, _, h40, _) => {
                                    h40
                                }
                            });
                            let a43 = f15(match v0 {
                                Adt0::C0(_, _, _, _, _, _, _, h41) => {
                                    h41
                                }
                            });
                            append_bytes(a42, a43)
                        };
                        append_bytes(a44, a45)
                    };
                    append_bytes(a46, a47)
                };
                append_bytes(a48, a49)
            };
            append_bytes(a50, a51)
        };
        append_bytes(a52, a53)
    };
    append_bytes(a54, a55)
}

pub fn f13(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a56 = v0.clone();
        length_bytes(a56)
    };
    let v2: u64 = {
        let a62 = {
            let a59 = {
                let a57 = v1;
                let a58 = 72u64;
                nat_add(a57, a58)?
            };
            let a60 = 64u64;
            let a61 = 0u64;
            nat_quot(a59, a60, a61)
        };
        let a63 = 64u64;
        nat_mul(a62, a63)?
    };
    let (_, h64) = f16(Fn1::F17(v0, v1, v2), {
        let a65 = v2;
        let a66 = 64u64;
        let a67 = 0u64;
        nat_quot(a65, a66, a67)
    }, (0u64, f18()))?;
    Ok(h64)
}

pub fn f14(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f19(v0, v1)?;
    let a68 = 47u64;
    let a69 = v2;
    if nat_lt(a68, a69) {
        let a70 = v2;
        let a71 = 58u64;
        if nat_lt(a70, a71) {
            Ok(true)
        } else {
            let a72 = 96u64;
            let a73 = v2;
            if nat_lt(a72, a73) {
                let a74 = v2;
                let a75 = 103u64;
                Ok(nat_lt(a74, a75))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f15(v0: u32) -> Bytes {
    let a104 = f20({
        let a76 = f21(v0, 28u32);
        let a77 = 15u32;
        fixed_u32::bit_and(a76, a77)
    });
    let a105 = {
        let a102 = f20({
            let a78 = f21(v0, 24u32);
            let a79 = 15u32;
            fixed_u32::bit_and(a78, a79)
        });
        let a103 = {
            let a100 = f20({
                let a80 = f21(v0, 20u32);
                let a81 = 15u32;
                fixed_u32::bit_and(a80, a81)
            });
            let a101 = {
                let a98 = f20({
                    let a82 = f21(v0, 16u32);
                    let a83 = 15u32;
                    fixed_u32::bit_and(a82, a83)
                });
                let a99 = {
                    let a96 = f20({
                        let a84 = f21(v0, 12u32);
                        let a85 = 15u32;
                        fixed_u32::bit_and(a84, a85)
                    });
                    let a97 = {
                        let a94 = f20({
                            let a86 = f21(v0, 8u32);
                            let a87 = 15u32;
                            fixed_u32::bit_and(a86, a87)
                        });
                        let a95 = {
                            let a92 = f20({
                                let a88 = f21(v0, 4u32);
                                let a89 = 15u32;
                                fixed_u32::bit_and(a88, a89)
                            });
                            let a93 = f20({
                                let a90 = v0;
                                let a91 = 15u32;
                                fixed_u32::bit_and(a90, a91)
                            });
                            append_bytes(a92, a93)
                        };
                        append_bytes(a94, a95)
                    };
                    append_bytes(a96, a97)
                };
                append_bytes(a98, a99)
            };
            append_bytes(a100, a101)
        };
        append_bytes(a102, a103)
    };
    append_bytes(a104, a105)
}

pub fn f16(v0: Fn1, p109: u64, p110: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m108 = (p109, p110);
    {
        let mut m108 = m108;
        loop {
            let step = {
                let (v1, v2) = m108;
                let m106 = v1;
                if m106 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m106 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c107: Fn1 = v0.clone();
                        c107.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a112 = {
            let (h111, _) = v3.clone();
            h111
        };
        let a113 = 64u64;
        nat_add(a112, a113)?
    }, f22({
        let (_, h114) = v3;
        h114
    }, f23(v0, v1, v2, {
        let (h115, _) = v3;
        h115
    })?)?))
}

pub fn f18() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f19(v0: Bytes, v1: u64) -> R<u64> {
    let m118 = {
        let a116 = v0;
        let a117 = v1;
        index_bytes(a116, a117)
    };
    match m118 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a163 = {
                let a139 = {
                    let a127 = {
                        let a121 = {
                            let a119 = v3;
                            let a120 = 128u8;
                            fixed_u8::bit_and(a119, a120)
                        };
                        let a122 = 128u8;
                        if equal(a121, a122) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a128 = {
                        let a125 = {
                            let a123 = v3;
                            let a124 = 64u8;
                            fixed_u8::bit_and(a123, a124)
                        };
                        let a126 = 64u8;
                        if equal(a125, a126) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a127, a128)?
                };
                let a140 = {
                    let a137 = {
                        let a131 = {
                            let a129 = v3;
                            let a130 = 32u8;
                            fixed_u8::bit_and(a129, a130)
                        };
                        let a132 = 32u8;
                        if equal(a131, a132) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a138 = {
                        let a135 = {
                            let a133 = v3;
                            let a134 = 16u8;
                            fixed_u8::bit_and(a133, a134)
                        };
                        let a136 = 16u8;
                        if equal(a135, a136) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a137, a138)?
                };
                nat_add(a139, a140)?
            };
            let a164 = {
                let a161 = {
                    let a149 = {
                        let a143 = {
                            let a141 = v3;
                            let a142 = 8u8;
                            fixed_u8::bit_and(a141, a142)
                        };
                        let a144 = 8u8;
                        if equal(a143, a144) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a150 = {
                        let a147 = {
                            let a145 = v3;
                            let a146 = 4u8;
                            fixed_u8::bit_and(a145, a146)
                        };
                        let a148 = 4u8;
                        if equal(a147, a148) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a149, a150)?
                };
                let a162 = {
                    let a159 = {
                        let a153 = {
                            let a151 = v3;
                            let a152 = 2u8;
                            fixed_u8::bit_and(a151, a152)
                        };
                        let a154 = 2u8;
                        if equal(a153, a154) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a160 = {
                        let a157 = {
                            let a155 = v3;
                            let a156 = 1u8;
                            fixed_u8::bit_and(a155, a156)
                        };
                        let a158 = 1u8;
                        if equal(a157, a158) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a159, a160)?
                };
                nat_add(a161, a162)?
            };
            nat_add(a163, a164)
        }
    }
}

pub fn f20(v0: u32) -> Bytes {
    let a167 = {
        let a165 = v0;
        let a166 = 8u32;
        fixed_u32::bit_and(a165, a166)
    };
    let a168 = 0u32;
    if equal(a167, a168) {
        let a171 = {
            let a169 = v0;
            let a170 = 4u32;
            fixed_u32::bit_and(a169, a170)
        };
        let a172 = 0u32;
        if equal(a171, a172) {
            let a175 = {
                let a173 = v0;
                let a174 = 2u32;
                fixed_u32::bit_and(a173, a174)
            };
            let a176 = 0u32;
            if equal(a175, a176) {
                let a179 = {
                    let a177 = v0;
                    let a178 = 1u32;
                    fixed_u32::bit_and(a177, a178)
                };
                let a180 = 0u32;
                if equal(a179, a180) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a183 = {
                    let a181 = v0;
                    let a182 = 1u32;
                    fixed_u32::bit_and(a181, a182)
                };
                let a184 = 0u32;
                if equal(a183, a184) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a187 = {
                let a185 = v0;
                let a186 = 2u32;
                fixed_u32::bit_and(a185, a186)
            };
            let a188 = 0u32;
            if equal(a187, a188) {
                let a191 = {
                    let a189 = v0;
                    let a190 = 1u32;
                    fixed_u32::bit_and(a189, a190)
                };
                let a192 = 0u32;
                if equal(a191, a192) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a195 = {
                    let a193 = v0;
                    let a194 = 1u32;
                    fixed_u32::bit_and(a193, a194)
                };
                let a196 = 0u32;
                if equal(a195, a196) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a199 = {
            let a197 = v0;
            let a198 = 4u32;
            fixed_u32::bit_and(a197, a198)
        };
        let a200 = 0u32;
        if equal(a199, a200) {
            let a203 = {
                let a201 = v0;
                let a202 = 2u32;
                fixed_u32::bit_and(a201, a202)
            };
            let a204 = 0u32;
            if equal(a203, a204) {
                let a207 = {
                    let a205 = v0;
                    let a206 = 1u32;
                    fixed_u32::bit_and(a205, a206)
                };
                let a208 = 0u32;
                if equal(a207, a208) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a211 = {
                    let a209 = v0;
                    let a210 = 1u32;
                    fixed_u32::bit_and(a209, a210)
                };
                let a212 = 0u32;
                if equal(a211, a212) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a215 = {
                let a213 = v0;
                let a214 = 2u32;
                fixed_u32::bit_and(a213, a214)
            };
            let a216 = 0u32;
            if equal(a215, a216) {
                let a219 = {
                    let a217 = v0;
                    let a218 = 1u32;
                    fixed_u32::bit_and(a217, a218)
                };
                let a220 = 0u32;
                if equal(a219, a220) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a223 = {
                    let a221 = v0;
                    let a222 = 1u32;
                    fixed_u32::bit_and(a221, a222)
                };
                let a224 = 0u32;
                if equal(a223, a224) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f21(v0: u32, v1: u32) -> u32 {
    let m227 = {
        let a225 = v0;
        let a226 = v1;
        fixed_u32::shift_right(a225, a226)
    };
    match m227 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f22(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f24(Fn2::F25, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h228, _, _, _, _, _, _, _) => {
            h228
        }
    }, match v0.clone() {
        Adt0::C0(_, h229, _, _, _, _, _, _) => {
            h229
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h230, _, _, _, _, _) => {
            h230
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h231, _, _, _, _) => {
            h231
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h232, _, _, _) => {
            h232
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h233, _, _) => {
            h233
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h234, _) => {
            h234
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h235) => {
            h235
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f26(match v0.clone() {
        Adt0::C0(h236, _, _, _, _, _, _, _) => {
            h236
        }
    }, match v2.clone() {
        Adt2::C0(h237, _, _, _, _, _, _, _, _, _) => {
            h237
        }
    }), f26(match v0.clone() {
        Adt0::C0(_, h238, _, _, _, _, _, _) => {
            h238
        }
    }, match v2.clone() {
        Adt2::C0(_, h239, _, _, _, _, _, _, _, _) => {
            h239
        }
    }), f26(match v0.clone() {
        Adt0::C0(_, _, h240, _, _, _, _, _) => {
            h240
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h241, _, _, _, _, _, _, _) => {
            h241
        }
    }), f26(match v0.clone() {
        Adt0::C0(_, _, _, h242, _, _, _, _) => {
            h242
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h243, _, _, _, _, _, _) => {
            h243
        }
    }), f26(match v0.clone() {
        Adt0::C0(_, _, _, _, h244, _, _, _) => {
            h244
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h245, _, _, _, _, _) => {
            h245
        }
    }), f26(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h246, _, _) => {
            h246
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h247, _, _, _, _) => {
            h247
        }
    }), f26(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h248, _) => {
            h248
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h249, _, _, _) => {
            h249
        }
    }), f26(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h250) => {
            h250
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h251, _, _) => {
            h251
        }
    })))
}

pub fn f23(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f27(v0.clone(), v1, v2, v3)?, f27(v0.clone(), v1, v2, {
        let a252 = v3;
        let a253 = 4u64;
        nat_add(a252, a253)?
    })?, f27(v0.clone(), v1, v2, {
        let a254 = v3;
        let a255 = 8u64;
        nat_add(a254, a255)?
    })?, f27(v0.clone(), v1, v2, {
        let a256 = v3;
        let a257 = 12u64;
        nat_add(a256, a257)?
    })?, f27(v0.clone(), v1, v2, {
        let a258 = v3;
        let a259 = 16u64;
        nat_add(a258, a259)?
    })?, f27(v0.clone(), v1, v2, {
        let a260 = v3;
        let a261 = 20u64;
        nat_add(a260, a261)?
    })?, f27(v0.clone(), v1, v2, {
        let a262 = v3;
        let a263 = 24u64;
        nat_add(a262, a263)?
    })?, f27(v0.clone(), v1, v2, {
        let a264 = v3;
        let a265 = 28u64;
        nat_add(a264, a265)?
    })?, f27(v0.clone(), v1, v2, {
        let a266 = v3;
        let a267 = 32u64;
        nat_add(a266, a267)?
    })?, f27(v0.clone(), v1, v2, {
        let a268 = v3;
        let a269 = 36u64;
        nat_add(a268, a269)?
    })?, f27(v0.clone(), v1, v2, {
        let a270 = v3;
        let a271 = 40u64;
        nat_add(a270, a271)?
    })?, f27(v0.clone(), v1, v2, {
        let a272 = v3;
        let a273 = 44u64;
        nat_add(a272, a273)?
    })?, f27(v0.clone(), v1, v2, {
        let a274 = v3;
        let a275 = 48u64;
        nat_add(a274, a275)?
    })?, f27(v0.clone(), v1, v2, {
        let a276 = v3;
        let a277 = 52u64;
        nat_add(a276, a277)?
    })?, f27(v0.clone(), v1, v2, {
        let a278 = v3;
        let a279 = 56u64;
        nat_add(a278, a279)?
    })?, f27(v0, v1, v2, {
        let a280 = v3;
        let a281 = 60u64;
        nat_add(a280, a281)?
    })?))
}

pub fn f24(v0: Fn2, p285: u64, p286: Adt2) -> R<Adt2> {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v1, v2) = m284;
                let m282 = v1;
                if m282 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m282 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c283: Fn2 = v0.clone();
                        c283.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt2) -> R<Adt2> {
    f28(v0)
}

pub fn f26(v0: u32, v1: u32) -> u32 {
    let m289 = {
        let a287 = f29(v0);
        let a288 = f29(v1);
        fixed_u64::checked_add(a287, a288)
    };
    match m289 {
        None => {
            0u32
        }
        Some(v2) => {
            f30(v2)
        }
    }
}

pub fn f27(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a292 = {
        let a290 = v3;
        let a291 = 8u64;
        nat_add(a290, a291)?
    };
    let a293 = v2;
    if nat_eq(a292, a293) {
        Ok(f31({
            let a296 = {
                let a294 = v1;
                let a295 = 8u64;
                nat_mul(a294, a295)?
            };
            let a297 = 4294967296u64;
            let a298 = 0u64;
            nat_quot(a296, a297, a298)
        }))
    } else {
        let a301 = {
            let a299 = v3;
            let a300 = 4u64;
            nat_add(a299, a300)?
        };
        let a302 = v2;
        if nat_eq(a301, a302) {
            Ok(f31({
                let a305 = {
                    let a303 = v1;
                    let a304 = 8u64;
                    nat_mul(a303, a304)?
                };
                let a306 = 4294967296u64;
                let a307 = 0u64;
                nat_rem(a305, a306, a307)
            }))
        } else {
            Ok(f32(f33(v0.clone(), v1, v3), f33(v0.clone(), v1, {
                let a308 = v3;
                let a309 = 1u64;
                nat_add(a308, a309)?
            }), f33(v0.clone(), v1, {
                let a310 = v3;
                let a311 = 2u64;
                nat_add(a310, a311)?
            }), f33(v0, v1, {
                let a312 = v3;
                let a313 = 3u64;
                nat_add(a312, a313)?
            })))
        }
    }
}

pub fn f28(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f34(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h314) => {
            h314
        }
    });
    let v2: u32 = f26(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h315, _, _) => {
            h315
        }
    }, f26(f35(match v0.clone() {
        Adt2::C0(_, _, _, _, h316, _, _, _, _, _) => {
            h316
        }
    }), f26(f36(match v0.clone() {
        Adt2::C0(_, _, _, _, h317, _, _, _, _, _) => {
            h317
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h318, _, _, _, _) => {
            h318
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h319, _, _, _) => {
            h319
        }
    }), f26(v1, {
        let m322 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h321, _) => {
                h321
            }
        };
        match m322 {
            Adt1::C0(h320, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h320
            }
        }
    }))));
    let v3: u32 = f26(f37(match v0.clone() {
        Adt2::C0(h323, _, _, _, _, _, _, _, _, _) => {
            h323
        }
    }), f38(match v0.clone() {
        Adt2::C0(h324, _, _, _, _, _, _, _, _, _) => {
            h324
        }
    }, match v0.clone() {
        Adt2::C0(_, h325, _, _, _, _, _, _, _, _) => {
            h325
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h326, _, _, _, _, _, _, _) => {
            h326
        }
    }));
    Ok(Adt2::C0(f26(v2, v3), match v0.clone() {
        Adt2::C0(h327, _, _, _, _, _, _, _, _, _) => {
            h327
        }
    }, match v0.clone() {
        Adt2::C0(_, h328, _, _, _, _, _, _, _, _) => {
            h328
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h329, _, _, _, _, _, _, _) => {
            h329
        }
    }, f26(match v0.clone() {
        Adt2::C0(_, _, _, h330, _, _, _, _, _, _) => {
            h330
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h331, _, _, _, _, _) => {
            h331
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h332, _, _, _, _) => {
            h332
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h333, _, _, _) => {
            h333
        }
    }, f39(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h334, _) => {
            h334
        }
    }), {
        let a336 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h335) => {
                h335
            }
        };
        let a337 = 1u64;
        nat_add(a336, a337)?
    }))
}

pub fn f29(v0: u32) -> u64 {
    let m339 = {
        let a338 = v0;
        fixed_u64::convert(i128::from(a338))
    };
    match m339 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f30(v0: u64) -> u32 {
    let m343 = {
        let a342 = {
            let a340 = v0;
            let a341 = 4294967295u64;
            fixed_u64::bit_and(a340, a341)
        };
        fixed_u32::convert(i128::from(a342))
    };
    match m343 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f31(v0: u64) -> u32 {
    let (_, h344) = {
        let (_, h345) = f40(Fn3::F41, 32u64, (v0, (1u32, 0u32)));
        h345
    };
    h344
}

pub fn f32(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a350 = f42(v0, 24u32);
    let a351 = {
        let a348 = f42(v1, 16u32);
        let a349 = {
            let a346 = f42(v2, 8u32);
            let a347 = v3;
            fixed_u32::bit_or(a346, a347)
        };
        fixed_u32::bit_or(a348, a349)
    };
    fixed_u32::bit_or(a350, a351)
}

pub fn f33(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a352 = v2;
    let a353 = v1;
    if nat_lt(a352, a353) {
        let m356 = {
            let a354 = v0;
            let a355 = v2;
            index_bytes(a354, a355)
        };
        match m356 {
            None => {
                0u32
            }
            Some(v3) => {
                let m358 = {
                    let a357 = v3;
                    fixed_u32::convert(i128::from(a357))
                };
                match m358 {
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
        let a359 = v2;
        let a360 = v1;
        if nat_eq(a359, a360) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f34(v0: u64) -> u32 {
    let a361 = v0;
    let a362 = 32u64;
    if nat_lt(a361, a362) {
        let a363 = v0;
        let a364 = 16u64;
        if nat_lt(a363, a364) {
            let a365 = v0;
            let a366 = 8u64;
            if nat_lt(a365, a366) {
                let a367 = v0;
                let a368 = 4u64;
                if nat_lt(a367, a368) {
                    let a369 = v0;
                    let a370 = 2u64;
                    if nat_lt(a369, a370) {
                        let a371 = v0;
                        let a372 = 1u64;
                        if nat_lt(a371, a372) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a373 = v0;
                        let a374 = 3u64;
                        if nat_lt(a373, a374) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a375 = v0;
                    let a376 = 6u64;
                    if nat_lt(a375, a376) {
                        let a377 = v0;
                        let a378 = 5u64;
                        if nat_lt(a377, a378) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a379 = v0;
                        let a380 = 7u64;
                        if nat_lt(a379, a380) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a381 = v0;
                let a382 = 12u64;
                if nat_lt(a381, a382) {
                    let a383 = v0;
                    let a384 = 10u64;
                    if nat_lt(a383, a384) {
                        let a385 = v0;
                        let a386 = 9u64;
                        if nat_lt(a385, a386) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a387 = v0;
                        let a388 = 11u64;
                        if nat_lt(a387, a388) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a389 = v0;
                    let a390 = 14u64;
                    if nat_lt(a389, a390) {
                        let a391 = v0;
                        let a392 = 13u64;
                        if nat_lt(a391, a392) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a393 = v0;
                        let a394 = 15u64;
                        if nat_lt(a393, a394) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a395 = v0;
            let a396 = 24u64;
            if nat_lt(a395, a396) {
                let a397 = v0;
                let a398 = 20u64;
                if nat_lt(a397, a398) {
                    let a399 = v0;
                    let a400 = 18u64;
                    if nat_lt(a399, a400) {
                        let a401 = v0;
                        let a402 = 17u64;
                        if nat_lt(a401, a402) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a403 = v0;
                        let a404 = 19u64;
                        if nat_lt(a403, a404) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a405 = v0;
                    let a406 = 22u64;
                    if nat_lt(a405, a406) {
                        let a407 = v0;
                        let a408 = 21u64;
                        if nat_lt(a407, a408) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a409 = v0;
                        let a410 = 23u64;
                        if nat_lt(a409, a410) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a411 = v0;
                let a412 = 28u64;
                if nat_lt(a411, a412) {
                    let a413 = v0;
                    let a414 = 26u64;
                    if nat_lt(a413, a414) {
                        let a415 = v0;
                        let a416 = 25u64;
                        if nat_lt(a415, a416) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a417 = v0;
                        let a418 = 27u64;
                        if nat_lt(a417, a418) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a419 = v0;
                    let a420 = 30u64;
                    if nat_lt(a419, a420) {
                        let a421 = v0;
                        let a422 = 29u64;
                        if nat_lt(a421, a422) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a423 = v0;
                        let a424 = 31u64;
                        if nat_lt(a423, a424) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a425 = v0;
        let a426 = 48u64;
        if nat_lt(a425, a426) {
            let a427 = v0;
            let a428 = 40u64;
            if nat_lt(a427, a428) {
                let a429 = v0;
                let a430 = 36u64;
                if nat_lt(a429, a430) {
                    let a431 = v0;
                    let a432 = 34u64;
                    if nat_lt(a431, a432) {
                        let a433 = v0;
                        let a434 = 33u64;
                        if nat_lt(a433, a434) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a435 = v0;
                        let a436 = 35u64;
                        if nat_lt(a435, a436) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a437 = v0;
                    let a438 = 38u64;
                    if nat_lt(a437, a438) {
                        let a439 = v0;
                        let a440 = 37u64;
                        if nat_lt(a439, a440) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a441 = v0;
                        let a442 = 39u64;
                        if nat_lt(a441, a442) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a443 = v0;
                let a444 = 44u64;
                if nat_lt(a443, a444) {
                    let a445 = v0;
                    let a446 = 42u64;
                    if nat_lt(a445, a446) {
                        let a447 = v0;
                        let a448 = 41u64;
                        if nat_lt(a447, a448) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a449 = v0;
                        let a450 = 43u64;
                        if nat_lt(a449, a450) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a451 = v0;
                    let a452 = 46u64;
                    if nat_lt(a451, a452) {
                        let a453 = v0;
                        let a454 = 45u64;
                        if nat_lt(a453, a454) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a455 = v0;
                        let a456 = 47u64;
                        if nat_lt(a455, a456) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a457 = v0;
            let a458 = 56u64;
            if nat_lt(a457, a458) {
                let a459 = v0;
                let a460 = 52u64;
                if nat_lt(a459, a460) {
                    let a461 = v0;
                    let a462 = 50u64;
                    if nat_lt(a461, a462) {
                        let a463 = v0;
                        let a464 = 49u64;
                        if nat_lt(a463, a464) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a465 = v0;
                        let a466 = 51u64;
                        if nat_lt(a465, a466) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a467 = v0;
                    let a468 = 54u64;
                    if nat_lt(a467, a468) {
                        let a469 = v0;
                        let a470 = 53u64;
                        if nat_lt(a469, a470) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a471 = v0;
                        let a472 = 55u64;
                        if nat_lt(a471, a472) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a473 = v0;
                let a474 = 60u64;
                if nat_lt(a473, a474) {
                    let a475 = v0;
                    let a476 = 58u64;
                    if nat_lt(a475, a476) {
                        let a477 = v0;
                        let a478 = 57u64;
                        if nat_lt(a477, a478) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a479 = v0;
                        let a480 = 59u64;
                        if nat_lt(a479, a480) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a481 = v0;
                    let a482 = 62u64;
                    if nat_lt(a481, a482) {
                        let a483 = v0;
                        let a484 = 61u64;
                        if nat_lt(a483, a484) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a485 = v0;
                        let a486 = 63u64;
                        if nat_lt(a485, a486) {
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

pub fn f35(v0: u32) -> u32 {
    let a489 = {
        let a487 = f43(v0, 6u32, 26u32);
        let a488 = f43(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a487, a488)
    };
    let a490 = f43(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a489, a490)
}

pub fn f36(v0: u32, v1: u32, v2: u32) -> u32 {
    let a496 = {
        let a491 = v0;
        let a492 = v1;
        fixed_u32::bit_and(a491, a492)
    };
    let a497 = {
        let a494 = {
            let a493 = v0;
            fixed_u32::bit_not(a493)
        };
        let a495 = v2;
        fixed_u32::bit_and(a494, a495)
    };
    fixed_u32::bit_xor(a496, a497)
}

pub fn f37(v0: u32) -> u32 {
    let a500 = {
        let a498 = f43(v0, 2u32, 30u32);
        let a499 = f43(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a498, a499)
    };
    let a501 = f43(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a500, a501)
}

pub fn f38(v0: u32, v1: u32, v2: u32) -> u32 {
    let a510 = {
        let a506 = {
            let a502 = v0;
            let a503 = v1;
            fixed_u32::bit_and(a502, a503)
        };
        let a507 = {
            let a504 = v0;
            let a505 = v2;
            fixed_u32::bit_and(a504, a505)
        };
        fixed_u32::bit_xor(a506, a507)
    };
    let a511 = {
        let a508 = v1;
        let a509 = v2;
        fixed_u32::bit_and(a508, a509)
    };
    fixed_u32::bit_xor(a510, a511)
}

pub fn f39(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h512, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h512
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h513, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h513
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h514, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h514
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h515, _, _, _, _, _, _, _, _, _, _, _) => {
            h515
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h516, _, _, _, _, _, _, _, _, _, _) => {
            h516
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h517, _, _, _, _, _, _, _, _, _) => {
            h517
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h518, _, _, _, _, _, _, _, _) => {
            h518
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h519, _, _, _, _, _, _, _) => {
            h519
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h520, _, _, _, _, _, _) => {
            h520
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h521, _, _, _, _, _) => {
            h521
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h522, _, _, _, _) => {
            h522
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h523, _, _, _) => {
            h523
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h524, _, _) => {
            h524
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h525, _) => {
            h525
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h526) => {
            h526
        }
    }, f26(f44(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h527, _) => {
            h527
        }
    }), f26(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h528, _, _, _, _, _, _) => {
            h528
        }
    }, f26(f45(match v0.clone() {
        Adt1::C0(_, h529, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h529
        }
    }), match v0 {
        Adt1::C0(h530, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h530
        }
    }))))
}

pub fn f40(v0: Fn3, p534: u64, p535: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m533 = (p534, p535);
    {
        let mut m533 = m533;
        loop {
            let step = {
                let (v1, v2) = m533;
                let m531 = v1;
                if m531 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m531 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c532: Fn3 = v0.clone();
                        c532.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h536, _) = v0;
        h536
    };
    let v2: u32 = {
        let (h537, _) = {
            let (_, h538) = v0;
            h538
        };
        h537
    };
    let v3: u32 = {
        let (_, h539) = {
            let (_, h540) = v0;
            h540
        };
        h539
    };
    ({
        let a541 = v1;
        let a542 = 2u64;
        let a543 = 0u64;
        nat_quot(a541, a542, a543)
    }, (f42(v2, 1u32), {
        let a547 = {
            let a544 = v1;
            let a545 = 2u64;
            let a546 = 0u64;
            nat_rem(a544, a545, a546)
        };
        let a548 = 1u64;
        if nat_eq(a547, a548) {
            let a549 = v3;
            let a550 = v2;
            fixed_u32::bit_or(a549, a550)
        } else {
            v3
        }
    }))
}

pub fn f42(v0: u32, v1: u32) -> u32 {
    let m553 = {
        let a551 = v0;
        let a552 = v1;
        fixed_u32::shift_left(a551, a552)
    };
    match m553 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f43(v0: u32, v1: u32, v2: u32) -> u32 {
    let a554 = f21(v0, v1);
    let a555 = f42(v0, v2);
    fixed_u32::bit_or(a554, a555)
}

pub fn f44(v0: u32) -> u32 {
    let a558 = {
        let a556 = f43(v0, 17u32, 15u32);
        let a557 = f43(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a556, a557)
    };
    let a559 = f21(v0, 10u32);
    fixed_u32::bit_xor(a558, a559)
}

pub fn f45(v0: u32) -> u32 {
    let a562 = {
        let a560 = f43(v0, 7u32, 25u32);
        let a561 = f43(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a560, a561)
    };
    let a563 = f21(v0, 3u32);
    fixed_u32::bit_xor(a562, a563)
}
