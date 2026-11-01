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
    F13(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F13(k0) => f13(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F15(k0) => f15(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F22(Bytes, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn2::F22(k0, k1, k2) => f22(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F33,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn3::F33 => f33(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F49,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F49 => f49(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    if f1(v0.clone())? {
        Ok(Some(f2({
            let a1 = f3(Str::lit("sha256:"));
            let a2 = f4(f5(v0.clone()), f6(v0)?);
            append_bytes(a1, a2)
        })?))
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
        if f7(v0.clone(), f3(Str::lit("sha256:"))) {
            let a6 = f8(v0, 7u64)?;
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
    let a8 = f3(Str::lit("sha256:"));
    let a9 = f9(v0)?;
    Ok(append_bytes(a8, a9))
}

pub fn f3(v0: Str) -> Bytes {
    let a10 = v0;
    utf8_encode(a10)
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    if f10(v1.clone(), v0.clone()) {
        v1
    } else {
        v0
    }
}

pub fn f5(v0: Bytes) -> Bytes {
    f11(v0, 7u64, 64u64)
}

pub fn f6(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f5(v0);
    let (_, h11) = {
        let (h12, _) = f12(Fn0::F13(v1), 65u64, (0u64, Bytes::lit(&[])))?;
        h12
    };
    Ok(h11)
}

pub fn f7(v0: Bytes, v1: Bytes) -> bool {
    let m17 = {
        let a14 = v0;
        let a15 = 0u64;
        let a16 = {
            let a13 = v1.clone();
            length_bytes(a13)
        };
        slice_bytes(a14, a15, a16)
    };
    match m17 {
        None => {
            false
        }
        Some(v2) => {
            let a18 = v2;
            let a19 = v1;
            equal(a18, a19)
        }
    }
}

pub fn f8(v0: Bytes, v1: u64) -> R<u64> {
    let (h20, _) = f14(Fn1::F15(v0.clone()), {
        let a21 = v0;
        length_bytes(a21)
    }, v1)?;
    Ok(h20)
}

pub fn f9(v0: Bytes) -> R<Bytes> {
    Ok(f16(f17(v0)?))
}

pub fn f10(v0: Bytes, v1: Bytes) -> bool {
    let a26 = {
        let a22 = v0;
        let a23 = v1;
        compare_bytes(a22, a23)
    };
    let a27 = {
        let a24 = Bytes::lit(&[]);
        let a25 = Bytes::lit(&[0x01]);
        compare_bytes(a24, a25)
    };
    equal(a26, a27)
}

pub fn f11(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m31 = {
        let a28 = v0;
        let a29 = v1;
        let a30 = v2;
        slice_bytes(a28, a29, a30)
    };
    match m31 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f12(v0: Fn0, p36: u64, p37: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m35 = (p36, p37);
    {
        let mut m35 = m35;
        loop {
            let step = {
                let (v1, v2) = m35;
                let m32 = v1;
                if m32 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m32 - 1;
                    let m34 = {
                        let c33: Fn0 = v0.clone();
                        c33.apply(v2.clone())?
                    };
                    match m34 {
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
                Ok(next) => { m35 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f18(v0, v1)
}

pub fn f14(v0: Fn1, p42: u64, p43: u64) -> R<(u64, bool)> {
    let m41 = (p42, p43);
    {
        let mut m41 = m41;
        loop {
            let step = {
                let (v1, v2) = m41;
                let m38 = v1;
                if m38 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m38 - 1;
                    let m40 = {
                        let c39: Fn1 = v0.clone();
                        c39.apply(v2)?
                    };
                    match m40 {
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
                Ok(next) => { m41 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f19(v0, v1)? {
        Ok(Some({
            let a44 = v1;
            let a45 = 1u64;
            nat_add(a44, a45)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f16(v0: Adt0) -> Bytes {
    let a66 = f20(match v0.clone() {
        Adt0::C0(h46, _, _, _, _, _, _, _) => {
            h46
        }
    });
    let a67 = {
        let a64 = f20(match v0.clone() {
            Adt0::C0(_, h47, _, _, _, _, _, _) => {
                h47
            }
        });
        let a65 = {
            let a62 = f20(match v0.clone() {
                Adt0::C0(_, _, h48, _, _, _, _, _) => {
                    h48
                }
            });
            let a63 = {
                let a60 = f20(match v0.clone() {
                    Adt0::C0(_, _, _, h49, _, _, _, _) => {
                        h49
                    }
                });
                let a61 = {
                    let a58 = f20(match v0.clone() {
                        Adt0::C0(_, _, _, _, h50, _, _, _) => {
                            h50
                        }
                    });
                    let a59 = {
                        let a56 = f20(match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h51, _, _) => {
                                h51
                            }
                        });
                        let a57 = {
                            let a54 = f20(match v0.clone() {
                                Adt0::C0(_, _, _, _, _, _, h52, _) => {
                                    h52
                                }
                            });
                            let a55 = f20(match v0 {
                                Adt0::C0(_, _, _, _, _, _, _, h53) => {
                                    h53
                                }
                            });
                            append_bytes(a54, a55)
                        };
                        append_bytes(a56, a57)
                    };
                    append_bytes(a58, a59)
                };
                append_bytes(a60, a61)
            };
            append_bytes(a62, a63)
        };
        append_bytes(a64, a65)
    };
    append_bytes(a66, a67)
}

pub fn f17(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a68 = v0.clone();
        length_bytes(a68)
    };
    let v2: u64 = {
        let a74 = {
            let a71 = {
                let a69 = v1;
                let a70 = 72u64;
                nat_add(a69, a70)?
            };
            let a72 = 64u64;
            let a73 = 0u64;
            nat_quot(a71, a72, a73)
        };
        let a75 = 64u64;
        nat_mul(a74, a75)?
    };
    let (_, h76) = f21(Fn2::F22(v0, v1, v2), {
        let a77 = v2;
        let a78 = 64u64;
        let a79 = 0u64;
        nat_quot(a77, a78, a79)
    }, (0u64, f23()))?;
    Ok(h76)
}

pub fn f18(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let a81 = {
        let (h80, _) = v1.clone();
        h80
    };
    let a82 = 64u64;
    if nat_lt(a81, a82) {
        Ok(Some(({
            let a84 = {
                let (h83, _) = v1.clone();
                h83
            };
            let a85 = 1u64;
            nat_add(a84, a85)?
        }, {
            let a90 = {
                let (_, h86) = v1;
                h86
            };
            let a91 = f24({
                let a88 = 15u64;
                let a89 = f25(f26(v0, {
                    let (h87, _) = v1;
                    h87
                })?);
                nat_sub(a88, a89)
            })?;
            append_bytes(a90, a91)
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f19(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f26(v0, v1)?;
    let a92 = 47u64;
    let a93 = v2;
    if nat_lt(a92, a93) {
        let a94 = v2;
        let a95 = 58u64;
        if nat_lt(a94, a95) {
            Ok(true)
        } else {
            let a96 = 96u64;
            let a97 = v2;
            if nat_lt(a96, a97) {
                let a98 = v2;
                let a99 = 103u64;
                Ok(nat_lt(a98, a99))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f20(v0: u32) -> Bytes {
    let a128 = f27({
        let a100 = f28(v0, 28u32);
        let a101 = 15u32;
        fixed_u32::bit_and(a100, a101)
    });
    let a129 = {
        let a126 = f27({
            let a102 = f28(v0, 24u32);
            let a103 = 15u32;
            fixed_u32::bit_and(a102, a103)
        });
        let a127 = {
            let a124 = f27({
                let a104 = f28(v0, 20u32);
                let a105 = 15u32;
                fixed_u32::bit_and(a104, a105)
            });
            let a125 = {
                let a122 = f27({
                    let a106 = f28(v0, 16u32);
                    let a107 = 15u32;
                    fixed_u32::bit_and(a106, a107)
                });
                let a123 = {
                    let a120 = f27({
                        let a108 = f28(v0, 12u32);
                        let a109 = 15u32;
                        fixed_u32::bit_and(a108, a109)
                    });
                    let a121 = {
                        let a118 = f27({
                            let a110 = f28(v0, 8u32);
                            let a111 = 15u32;
                            fixed_u32::bit_and(a110, a111)
                        });
                        let a119 = {
                            let a116 = f27({
                                let a112 = f28(v0, 4u32);
                                let a113 = 15u32;
                                fixed_u32::bit_and(a112, a113)
                            });
                            let a117 = f27({
                                let a114 = v0;
                                let a115 = 15u32;
                                fixed_u32::bit_and(a114, a115)
                            });
                            append_bytes(a116, a117)
                        };
                        append_bytes(a118, a119)
                    };
                    append_bytes(a120, a121)
                };
                append_bytes(a122, a123)
            };
            append_bytes(a124, a125)
        };
        append_bytes(a126, a127)
    };
    append_bytes(a128, a129)
}

pub fn f21(v0: Fn2, p133: u64, p134: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m132 = (p133, p134);
    {
        let mut m132 = m132;
        loop {
            let step = {
                let (v1, v2) = m132;
                let m130 = v1;
                if m130 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m130 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c131: Fn2 = v0.clone();
                        c131.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a136 = {
            let (h135, _) = v3.clone();
            h135
        };
        let a137 = 64u64;
        nat_add(a136, a137)?
    }, f29({
        let (_, h138) = v3;
        h138
    }, f30(v0, v1, v2, {
        let (h139, _) = v3;
        h139
    })?)?))
}

pub fn f23() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f24(v0: u64) -> R<Bytes> {
    Ok(f31({
        let a140 = v0;
        let a141 = 10u64;
        if nat_lt(a140, a141) {
            let a142 = 48u64;
            let a143 = v0;
            nat_add(a142, a143)?
        } else {
            let a144 = 87u64;
            let a145 = v0;
            nat_add(a144, a145)?
        }
    }))
}

pub fn f25(v0: u64) -> u64 {
    let a146 = 47u64;
    let a147 = v0;
    if nat_lt(a146, a147) {
        let a148 = v0;
        let a149 = 58u64;
        if nat_lt(a148, a149) {
            let a150 = v0;
            let a151 = 48u64;
            nat_sub(a150, a151)
        } else {
            let a152 = 64u64;
            let a153 = v0;
            if nat_lt(a152, a153) {
                let a154 = v0;
                let a155 = 71u64;
                if nat_lt(a154, a155) {
                    let a156 = v0;
                    let a157 = 55u64;
                    nat_sub(a156, a157)
                } else {
                    let a158 = 96u64;
                    let a159 = v0;
                    if nat_lt(a158, a159) {
                        let a160 = v0;
                        let a161 = 103u64;
                        if nat_lt(a160, a161) {
                            let a162 = v0;
                            let a163 = 87u64;
                            nat_sub(a162, a163)
                        } else {
                            16u64
                        }
                    } else {
                        16u64
                    }
                }
            } else {
                16u64
            }
        }
    } else {
        16u64
    }
}

pub fn f26(v0: Bytes, v1: u64) -> R<u64> {
    let m166 = {
        let a164 = v0;
        let a165 = v1;
        index_bytes(a164, a165)
    };
    match m166 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a211 = {
                let a187 = {
                    let a175 = {
                        let a169 = {
                            let a167 = v3;
                            let a168 = 128u8;
                            fixed_u8::bit_and(a167, a168)
                        };
                        let a170 = 128u8;
                        if equal(a169, a170) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a176 = {
                        let a173 = {
                            let a171 = v3;
                            let a172 = 64u8;
                            fixed_u8::bit_and(a171, a172)
                        };
                        let a174 = 64u8;
                        if equal(a173, a174) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a175, a176)?
                };
                let a188 = {
                    let a185 = {
                        let a179 = {
                            let a177 = v3;
                            let a178 = 32u8;
                            fixed_u8::bit_and(a177, a178)
                        };
                        let a180 = 32u8;
                        if equal(a179, a180) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a186 = {
                        let a183 = {
                            let a181 = v3;
                            let a182 = 16u8;
                            fixed_u8::bit_and(a181, a182)
                        };
                        let a184 = 16u8;
                        if equal(a183, a184) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a185, a186)?
                };
                nat_add(a187, a188)?
            };
            let a212 = {
                let a209 = {
                    let a197 = {
                        let a191 = {
                            let a189 = v3;
                            let a190 = 8u8;
                            fixed_u8::bit_and(a189, a190)
                        };
                        let a192 = 8u8;
                        if equal(a191, a192) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a198 = {
                        let a195 = {
                            let a193 = v3;
                            let a194 = 4u8;
                            fixed_u8::bit_and(a193, a194)
                        };
                        let a196 = 4u8;
                        if equal(a195, a196) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a197, a198)?
                };
                let a210 = {
                    let a207 = {
                        let a201 = {
                            let a199 = v3;
                            let a200 = 2u8;
                            fixed_u8::bit_and(a199, a200)
                        };
                        let a202 = 2u8;
                        if equal(a201, a202) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a208 = {
                        let a205 = {
                            let a203 = v3;
                            let a204 = 1u8;
                            fixed_u8::bit_and(a203, a204)
                        };
                        let a206 = 1u8;
                        if equal(a205, a206) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a207, a208)?
                };
                nat_add(a209, a210)?
            };
            nat_add(a211, a212)
        }
    }
}

pub fn f27(v0: u32) -> Bytes {
    let a215 = {
        let a213 = v0;
        let a214 = 8u32;
        fixed_u32::bit_and(a213, a214)
    };
    let a216 = 0u32;
    if equal(a215, a216) {
        let a219 = {
            let a217 = v0;
            let a218 = 4u32;
            fixed_u32::bit_and(a217, a218)
        };
        let a220 = 0u32;
        if equal(a219, a220) {
            let a223 = {
                let a221 = v0;
                let a222 = 2u32;
                fixed_u32::bit_and(a221, a222)
            };
            let a224 = 0u32;
            if equal(a223, a224) {
                let a227 = {
                    let a225 = v0;
                    let a226 = 1u32;
                    fixed_u32::bit_and(a225, a226)
                };
                let a228 = 0u32;
                if equal(a227, a228) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a231 = {
                    let a229 = v0;
                    let a230 = 1u32;
                    fixed_u32::bit_and(a229, a230)
                };
                let a232 = 0u32;
                if equal(a231, a232) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a235 = {
                let a233 = v0;
                let a234 = 2u32;
                fixed_u32::bit_and(a233, a234)
            };
            let a236 = 0u32;
            if equal(a235, a236) {
                let a239 = {
                    let a237 = v0;
                    let a238 = 1u32;
                    fixed_u32::bit_and(a237, a238)
                };
                let a240 = 0u32;
                if equal(a239, a240) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a243 = {
                    let a241 = v0;
                    let a242 = 1u32;
                    fixed_u32::bit_and(a241, a242)
                };
                let a244 = 0u32;
                if equal(a243, a244) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a247 = {
            let a245 = v0;
            let a246 = 4u32;
            fixed_u32::bit_and(a245, a246)
        };
        let a248 = 0u32;
        if equal(a247, a248) {
            let a251 = {
                let a249 = v0;
                let a250 = 2u32;
                fixed_u32::bit_and(a249, a250)
            };
            let a252 = 0u32;
            if equal(a251, a252) {
                let a255 = {
                    let a253 = v0;
                    let a254 = 1u32;
                    fixed_u32::bit_and(a253, a254)
                };
                let a256 = 0u32;
                if equal(a255, a256) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a259 = {
                    let a257 = v0;
                    let a258 = 1u32;
                    fixed_u32::bit_and(a257, a258)
                };
                let a260 = 0u32;
                if equal(a259, a260) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a263 = {
                let a261 = v0;
                let a262 = 2u32;
                fixed_u32::bit_and(a261, a262)
            };
            let a264 = 0u32;
            if equal(a263, a264) {
                let a267 = {
                    let a265 = v0;
                    let a266 = 1u32;
                    fixed_u32::bit_and(a265, a266)
                };
                let a268 = 0u32;
                if equal(a267, a268) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a271 = {
                    let a269 = v0;
                    let a270 = 1u32;
                    fixed_u32::bit_and(a269, a270)
                };
                let a272 = 0u32;
                if equal(a271, a272) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f28(v0: u32, v1: u32) -> u32 {
    let m275 = {
        let a273 = v0;
        let a274 = v1;
        fixed_u32::shift_right(a273, a274)
    };
    match m275 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f29(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f32(Fn3::F33, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h276, _, _, _, _, _, _, _) => {
            h276
        }
    }, match v0.clone() {
        Adt0::C0(_, h277, _, _, _, _, _, _) => {
            h277
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h278, _, _, _, _, _) => {
            h278
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h279, _, _, _, _) => {
            h279
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h280, _, _, _) => {
            h280
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h281, _, _) => {
            h281
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h282, _) => {
            h282
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h283) => {
            h283
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f34(match v0.clone() {
        Adt0::C0(h284, _, _, _, _, _, _, _) => {
            h284
        }
    }, match v2.clone() {
        Adt2::C0(h285, _, _, _, _, _, _, _, _, _) => {
            h285
        }
    }), f34(match v0.clone() {
        Adt0::C0(_, h286, _, _, _, _, _, _) => {
            h286
        }
    }, match v2.clone() {
        Adt2::C0(_, h287, _, _, _, _, _, _, _, _) => {
            h287
        }
    }), f34(match v0.clone() {
        Adt0::C0(_, _, h288, _, _, _, _, _) => {
            h288
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h289, _, _, _, _, _, _, _) => {
            h289
        }
    }), f34(match v0.clone() {
        Adt0::C0(_, _, _, h290, _, _, _, _) => {
            h290
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h291, _, _, _, _, _, _) => {
            h291
        }
    }), f34(match v0.clone() {
        Adt0::C0(_, _, _, _, h292, _, _, _) => {
            h292
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h293, _, _, _, _, _) => {
            h293
        }
    }), f34(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h294, _, _) => {
            h294
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h295, _, _, _, _) => {
            h295
        }
    }), f34(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h296, _) => {
            h296
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h297, _, _, _) => {
            h297
        }
    }), f34(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h298) => {
            h298
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h299, _, _) => {
            h299
        }
    })))
}

pub fn f30(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f35(v0.clone(), v1, v2, v3)?, f35(v0.clone(), v1, v2, {
        let a300 = v3;
        let a301 = 4u64;
        nat_add(a300, a301)?
    })?, f35(v0.clone(), v1, v2, {
        let a302 = v3;
        let a303 = 8u64;
        nat_add(a302, a303)?
    })?, f35(v0.clone(), v1, v2, {
        let a304 = v3;
        let a305 = 12u64;
        nat_add(a304, a305)?
    })?, f35(v0.clone(), v1, v2, {
        let a306 = v3;
        let a307 = 16u64;
        nat_add(a306, a307)?
    })?, f35(v0.clone(), v1, v2, {
        let a308 = v3;
        let a309 = 20u64;
        nat_add(a308, a309)?
    })?, f35(v0.clone(), v1, v2, {
        let a310 = v3;
        let a311 = 24u64;
        nat_add(a310, a311)?
    })?, f35(v0.clone(), v1, v2, {
        let a312 = v3;
        let a313 = 28u64;
        nat_add(a312, a313)?
    })?, f35(v0.clone(), v1, v2, {
        let a314 = v3;
        let a315 = 32u64;
        nat_add(a314, a315)?
    })?, f35(v0.clone(), v1, v2, {
        let a316 = v3;
        let a317 = 36u64;
        nat_add(a316, a317)?
    })?, f35(v0.clone(), v1, v2, {
        let a318 = v3;
        let a319 = 40u64;
        nat_add(a318, a319)?
    })?, f35(v0.clone(), v1, v2, {
        let a320 = v3;
        let a321 = 44u64;
        nat_add(a320, a321)?
    })?, f35(v0.clone(), v1, v2, {
        let a322 = v3;
        let a323 = 48u64;
        nat_add(a322, a323)?
    })?, f35(v0.clone(), v1, v2, {
        let a324 = v3;
        let a325 = 52u64;
        nat_add(a324, a325)?
    })?, f35(v0.clone(), v1, v2, {
        let a326 = v3;
        let a327 = 56u64;
        nat_add(a326, a327)?
    })?, f35(v0, v1, v2, {
        let a328 = v3;
        let a329 = 60u64;
        nat_add(a328, a329)?
    })?))
}

pub fn f31(v0: u64) -> Bytes {
    f11(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f32(v0: Fn3, p333: u64, p334: Adt2) -> R<Adt2> {
    let m332 = (p333, p334);
    {
        let mut m332 = m332;
        loop {
            let step = {
                let (v1, v2) = m332;
                let m330 = v1;
                if m330 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m330 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c331: Fn3 = v0.clone();
                        c331.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Adt2) -> R<Adt2> {
    f36(v0)
}

pub fn f34(v0: u32, v1: u32) -> u32 {
    let m337 = {
        let a335 = f37(v0);
        let a336 = f37(v1);
        fixed_u64::checked_add(a335, a336)
    };
    match m337 {
        None => {
            0u32
        }
        Some(v2) => {
            f38(v2)
        }
    }
}

pub fn f35(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a340 = {
        let a338 = v3;
        let a339 = 8u64;
        nat_add(a338, a339)?
    };
    let a341 = v2;
    if nat_eq(a340, a341) {
        Ok(f39({
            let a344 = {
                let a342 = v1;
                let a343 = 8u64;
                nat_mul(a342, a343)?
            };
            let a345 = 4294967296u64;
            let a346 = 0u64;
            nat_quot(a344, a345, a346)
        }))
    } else {
        let a349 = {
            let a347 = v3;
            let a348 = 4u64;
            nat_add(a347, a348)?
        };
        let a350 = v2;
        if nat_eq(a349, a350) {
            Ok(f39({
                let a353 = {
                    let a351 = v1;
                    let a352 = 8u64;
                    nat_mul(a351, a352)?
                };
                let a354 = 4294967296u64;
                let a355 = 0u64;
                nat_rem(a353, a354, a355)
            }))
        } else {
            Ok(f40(f41(v0.clone(), v1, v3), f41(v0.clone(), v1, {
                let a356 = v3;
                let a357 = 1u64;
                nat_add(a356, a357)?
            }), f41(v0.clone(), v1, {
                let a358 = v3;
                let a359 = 2u64;
                nat_add(a358, a359)?
            }), f41(v0, v1, {
                let a360 = v3;
                let a361 = 3u64;
                nat_add(a360, a361)?
            })))
        }
    }
}

pub fn f36(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f42(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h362) => {
            h362
        }
    });
    let v2: u32 = f34(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h363, _, _) => {
            h363
        }
    }, f34(f43(match v0.clone() {
        Adt2::C0(_, _, _, _, h364, _, _, _, _, _) => {
            h364
        }
    }), f34(f44(match v0.clone() {
        Adt2::C0(_, _, _, _, h365, _, _, _, _, _) => {
            h365
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h366, _, _, _, _) => {
            h366
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h367, _, _, _) => {
            h367
        }
    }), f34(v1, {
        let m370 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h369, _) => {
                h369
            }
        };
        match m370 {
            Adt1::C0(h368, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h368
            }
        }
    }))));
    let v3: u32 = f34(f45(match v0.clone() {
        Adt2::C0(h371, _, _, _, _, _, _, _, _, _) => {
            h371
        }
    }), f46(match v0.clone() {
        Adt2::C0(h372, _, _, _, _, _, _, _, _, _) => {
            h372
        }
    }, match v0.clone() {
        Adt2::C0(_, h373, _, _, _, _, _, _, _, _) => {
            h373
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h374, _, _, _, _, _, _, _) => {
            h374
        }
    }));
    Ok(Adt2::C0(f34(v2, v3), match v0.clone() {
        Adt2::C0(h375, _, _, _, _, _, _, _, _, _) => {
            h375
        }
    }, match v0.clone() {
        Adt2::C0(_, h376, _, _, _, _, _, _, _, _) => {
            h376
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h377, _, _, _, _, _, _, _) => {
            h377
        }
    }, f34(match v0.clone() {
        Adt2::C0(_, _, _, h378, _, _, _, _, _, _) => {
            h378
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h379, _, _, _, _, _) => {
            h379
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h380, _, _, _, _) => {
            h380
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h381, _, _, _) => {
            h381
        }
    }, f47(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h382, _) => {
            h382
        }
    }), {
        let a384 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h383) => {
                h383
            }
        };
        let a385 = 1u64;
        nat_add(a384, a385)?
    }))
}

pub fn f37(v0: u32) -> u64 {
    let m387 = {
        let a386 = v0;
        fixed_u64::convert(i128::from(a386))
    };
    match m387 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f38(v0: u64) -> u32 {
    let m391 = {
        let a390 = {
            let a388 = v0;
            let a389 = 4294967295u64;
            fixed_u64::bit_and(a388, a389)
        };
        fixed_u32::convert(i128::from(a390))
    };
    match m391 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f39(v0: u64) -> u32 {
    let (_, h392) = {
        let (_, h393) = f48(Fn4::F49, 32u64, (v0, (1u32, 0u32)));
        h393
    };
    h392
}

pub fn f40(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a398 = f50(v0, 24u32);
    let a399 = {
        let a396 = f50(v1, 16u32);
        let a397 = {
            let a394 = f50(v2, 8u32);
            let a395 = v3;
            fixed_u32::bit_or(a394, a395)
        };
        fixed_u32::bit_or(a396, a397)
    };
    fixed_u32::bit_or(a398, a399)
}

pub fn f41(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a400 = v2;
    let a401 = v1;
    if nat_lt(a400, a401) {
        let m404 = {
            let a402 = v0;
            let a403 = v2;
            index_bytes(a402, a403)
        };
        match m404 {
            None => {
                0u32
            }
            Some(v3) => {
                let m406 = {
                    let a405 = v3;
                    fixed_u32::convert(i128::from(a405))
                };
                match m406 {
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
        let a407 = v2;
        let a408 = v1;
        if nat_eq(a407, a408) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f42(v0: u64) -> u32 {
    let a409 = v0;
    let a410 = 32u64;
    if nat_lt(a409, a410) {
        let a411 = v0;
        let a412 = 16u64;
        if nat_lt(a411, a412) {
            let a413 = v0;
            let a414 = 8u64;
            if nat_lt(a413, a414) {
                let a415 = v0;
                let a416 = 4u64;
                if nat_lt(a415, a416) {
                    let a417 = v0;
                    let a418 = 2u64;
                    if nat_lt(a417, a418) {
                        let a419 = v0;
                        let a420 = 1u64;
                        if nat_lt(a419, a420) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a421 = v0;
                        let a422 = 3u64;
                        if nat_lt(a421, a422) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a423 = v0;
                    let a424 = 6u64;
                    if nat_lt(a423, a424) {
                        let a425 = v0;
                        let a426 = 5u64;
                        if nat_lt(a425, a426) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a427 = v0;
                        let a428 = 7u64;
                        if nat_lt(a427, a428) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a429 = v0;
                let a430 = 12u64;
                if nat_lt(a429, a430) {
                    let a431 = v0;
                    let a432 = 10u64;
                    if nat_lt(a431, a432) {
                        let a433 = v0;
                        let a434 = 9u64;
                        if nat_lt(a433, a434) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a435 = v0;
                        let a436 = 11u64;
                        if nat_lt(a435, a436) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a437 = v0;
                    let a438 = 14u64;
                    if nat_lt(a437, a438) {
                        let a439 = v0;
                        let a440 = 13u64;
                        if nat_lt(a439, a440) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a441 = v0;
                        let a442 = 15u64;
                        if nat_lt(a441, a442) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a443 = v0;
            let a444 = 24u64;
            if nat_lt(a443, a444) {
                let a445 = v0;
                let a446 = 20u64;
                if nat_lt(a445, a446) {
                    let a447 = v0;
                    let a448 = 18u64;
                    if nat_lt(a447, a448) {
                        let a449 = v0;
                        let a450 = 17u64;
                        if nat_lt(a449, a450) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a451 = v0;
                        let a452 = 19u64;
                        if nat_lt(a451, a452) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a453 = v0;
                    let a454 = 22u64;
                    if nat_lt(a453, a454) {
                        let a455 = v0;
                        let a456 = 21u64;
                        if nat_lt(a455, a456) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a457 = v0;
                        let a458 = 23u64;
                        if nat_lt(a457, a458) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a459 = v0;
                let a460 = 28u64;
                if nat_lt(a459, a460) {
                    let a461 = v0;
                    let a462 = 26u64;
                    if nat_lt(a461, a462) {
                        let a463 = v0;
                        let a464 = 25u64;
                        if nat_lt(a463, a464) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a465 = v0;
                        let a466 = 27u64;
                        if nat_lt(a465, a466) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a467 = v0;
                    let a468 = 30u64;
                    if nat_lt(a467, a468) {
                        let a469 = v0;
                        let a470 = 29u64;
                        if nat_lt(a469, a470) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a471 = v0;
                        let a472 = 31u64;
                        if nat_lt(a471, a472) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a473 = v0;
        let a474 = 48u64;
        if nat_lt(a473, a474) {
            let a475 = v0;
            let a476 = 40u64;
            if nat_lt(a475, a476) {
                let a477 = v0;
                let a478 = 36u64;
                if nat_lt(a477, a478) {
                    let a479 = v0;
                    let a480 = 34u64;
                    if nat_lt(a479, a480) {
                        let a481 = v0;
                        let a482 = 33u64;
                        if nat_lt(a481, a482) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a483 = v0;
                        let a484 = 35u64;
                        if nat_lt(a483, a484) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a485 = v0;
                    let a486 = 38u64;
                    if nat_lt(a485, a486) {
                        let a487 = v0;
                        let a488 = 37u64;
                        if nat_lt(a487, a488) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a489 = v0;
                        let a490 = 39u64;
                        if nat_lt(a489, a490) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a491 = v0;
                let a492 = 44u64;
                if nat_lt(a491, a492) {
                    let a493 = v0;
                    let a494 = 42u64;
                    if nat_lt(a493, a494) {
                        let a495 = v0;
                        let a496 = 41u64;
                        if nat_lt(a495, a496) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a497 = v0;
                        let a498 = 43u64;
                        if nat_lt(a497, a498) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a499 = v0;
                    let a500 = 46u64;
                    if nat_lt(a499, a500) {
                        let a501 = v0;
                        let a502 = 45u64;
                        if nat_lt(a501, a502) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a503 = v0;
                        let a504 = 47u64;
                        if nat_lt(a503, a504) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a505 = v0;
            let a506 = 56u64;
            if nat_lt(a505, a506) {
                let a507 = v0;
                let a508 = 52u64;
                if nat_lt(a507, a508) {
                    let a509 = v0;
                    let a510 = 50u64;
                    if nat_lt(a509, a510) {
                        let a511 = v0;
                        let a512 = 49u64;
                        if nat_lt(a511, a512) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a513 = v0;
                        let a514 = 51u64;
                        if nat_lt(a513, a514) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a515 = v0;
                    let a516 = 54u64;
                    if nat_lt(a515, a516) {
                        let a517 = v0;
                        let a518 = 53u64;
                        if nat_lt(a517, a518) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a519 = v0;
                        let a520 = 55u64;
                        if nat_lt(a519, a520) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a521 = v0;
                let a522 = 60u64;
                if nat_lt(a521, a522) {
                    let a523 = v0;
                    let a524 = 58u64;
                    if nat_lt(a523, a524) {
                        let a525 = v0;
                        let a526 = 57u64;
                        if nat_lt(a525, a526) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a527 = v0;
                        let a528 = 59u64;
                        if nat_lt(a527, a528) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a529 = v0;
                    let a530 = 62u64;
                    if nat_lt(a529, a530) {
                        let a531 = v0;
                        let a532 = 61u64;
                        if nat_lt(a531, a532) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a533 = v0;
                        let a534 = 63u64;
                        if nat_lt(a533, a534) {
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

pub fn f43(v0: u32) -> u32 {
    let a537 = {
        let a535 = f51(v0, 6u32, 26u32);
        let a536 = f51(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a535, a536)
    };
    let a538 = f51(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a537, a538)
}

pub fn f44(v0: u32, v1: u32, v2: u32) -> u32 {
    let a544 = {
        let a539 = v0;
        let a540 = v1;
        fixed_u32::bit_and(a539, a540)
    };
    let a545 = {
        let a542 = {
            let a541 = v0;
            fixed_u32::bit_not(a541)
        };
        let a543 = v2;
        fixed_u32::bit_and(a542, a543)
    };
    fixed_u32::bit_xor(a544, a545)
}

pub fn f45(v0: u32) -> u32 {
    let a548 = {
        let a546 = f51(v0, 2u32, 30u32);
        let a547 = f51(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a546, a547)
    };
    let a549 = f51(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a548, a549)
}

pub fn f46(v0: u32, v1: u32, v2: u32) -> u32 {
    let a558 = {
        let a554 = {
            let a550 = v0;
            let a551 = v1;
            fixed_u32::bit_and(a550, a551)
        };
        let a555 = {
            let a552 = v0;
            let a553 = v2;
            fixed_u32::bit_and(a552, a553)
        };
        fixed_u32::bit_xor(a554, a555)
    };
    let a559 = {
        let a556 = v1;
        let a557 = v2;
        fixed_u32::bit_and(a556, a557)
    };
    fixed_u32::bit_xor(a558, a559)
}

pub fn f47(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h560, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h560
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h561, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h561
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h562, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h562
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h563, _, _, _, _, _, _, _, _, _, _, _) => {
            h563
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h564, _, _, _, _, _, _, _, _, _, _) => {
            h564
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h565, _, _, _, _, _, _, _, _, _) => {
            h565
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h566, _, _, _, _, _, _, _, _) => {
            h566
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h567, _, _, _, _, _, _, _) => {
            h567
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h568, _, _, _, _, _, _) => {
            h568
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h569, _, _, _, _, _) => {
            h569
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h570, _, _, _, _) => {
            h570
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h571, _, _, _) => {
            h571
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h572, _, _) => {
            h572
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h573, _) => {
            h573
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h574) => {
            h574
        }
    }, f34(f52(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h575, _) => {
            h575
        }
    }), f34(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h576, _, _, _, _, _, _) => {
            h576
        }
    }, f34(f53(match v0.clone() {
        Adt1::C0(_, h577, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h577
        }
    }), match v0 {
        Adt1::C0(h578, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h578
        }
    }))))
}

pub fn f48(v0: Fn4, p582: u64, p583: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m581 = (p582, p583);
    {
        let mut m581 = m581;
        loop {
            let step = {
                let (v1, v2) = m581;
                let m579 = v1;
                if m579 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m579 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c580: Fn4 = v0.clone();
                        c580.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m581 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h584, _) = v0;
        h584
    };
    let v2: u32 = {
        let (h585, _) = {
            let (_, h586) = v0;
            h586
        };
        h585
    };
    let v3: u32 = {
        let (_, h587) = {
            let (_, h588) = v0;
            h588
        };
        h587
    };
    ({
        let a589 = v1;
        let a590 = 2u64;
        let a591 = 0u64;
        nat_quot(a589, a590, a591)
    }, (f50(v2, 1u32), {
        let a595 = {
            let a592 = v1;
            let a593 = 2u64;
            let a594 = 0u64;
            nat_rem(a592, a593, a594)
        };
        let a596 = 1u64;
        if nat_eq(a595, a596) {
            let a597 = v3;
            let a598 = v2;
            fixed_u32::bit_or(a597, a598)
        } else {
            v3
        }
    }))
}

pub fn f50(v0: u32, v1: u32) -> u32 {
    let m601 = {
        let a599 = v0;
        let a600 = v1;
        fixed_u32::shift_left(a599, a600)
    };
    match m601 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f51(v0: u32, v1: u32, v2: u32) -> u32 {
    let a602 = f28(v0, v1);
    let a603 = f50(v0, v2);
    fixed_u32::bit_or(a602, a603)
}

pub fn f52(v0: u32) -> u32 {
    let a606 = {
        let a604 = f51(v0, 17u32, 15u32);
        let a605 = f51(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a604, a605)
    };
    let a607 = f28(v0, 10u32);
    fixed_u32::bit_xor(a606, a607)
}

pub fn f53(v0: u32) -> u32 {
    let a610 = {
        let a608 = f51(v0, 7u32, 25u32);
        let a609 = f51(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a608, a609)
    };
    let a611 = f28(v0, 3u32);
    fixed_u32::bit_xor(a610, a611)
}
