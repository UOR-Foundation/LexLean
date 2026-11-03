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
    F15(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F15(k0) => f15(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F31(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt2)) -> R<(u64, Adt2)> {
        match self {
            Fn1::F31(k0, k1, k2) => f31(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F36(Bytes, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn2::F36(k0, k1, k2) => f36(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F42,
}

impl Fn3 {
    pub fn apply(&self, p0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
        match self {
            Fn3::F42 => f42(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F49(Bytes, u64, u64, u64),
    F107,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt3)) -> R<(u64, Adt3)> {
        match self {
            Fn4::F49(k0, k1, k2, k3) => f49(k0.clone(), *k1, *k2, *k3, p0),
            Fn4::F107 => f107(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F56(Bytes, u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
        match self {
            Fn5::F56(k0, k1) => f56(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F65,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt5) -> R<Adt5> {
        match self {
            Fn6::F65 => f65(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F70,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt7) -> R<Adt7> {
        match self {
            Fn7::F70 => f70(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F75,
}

impl Fn8 {
    pub fn apply(&self, p0: (Adt9, Adt8)) -> (Adt9, Adt8) {
        match self {
            Fn8::F75 => f75(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F77(u64, Bytes, u64),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Adt1)) -> R<(u64, Adt1)> {
        match self {
            Fn9::F77(k0, k1, k2) => f77(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F81,
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn10::F81 => f81(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F96,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt10) -> Adt10 {
        match self {
            Fn11::F96 => f96(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F117,
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
        match self {
            Fn12::F117 => f117(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    f1(f2(Str::lit("sha256")), v0)
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f3(v0.clone(), v1.clone())? {
        f4(v0.clone(), {
            let a2 = f5(v1.clone(), {
                let a1 = f6(v0);
                length_bytes(a1)
            })?;
            let a3 = v1;
            append_bytes(a2, a3)
        })
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f2(v0: Str) -> Bytes {
    let a4 = v0;
    utf8_encode(a4)
}

pub fn f3(v0: Bytes, v1: Bytes) -> R<bool> {
    let a5 = 0u64;
    let a6 = f7(v0.clone());
    if nat_lt(a5, a6) {
        let a11 = {
            let a7 = v1.clone();
            length_bytes(a7)
        };
        let a12 = {
            let a9 = {
                let a8 = f6(v0.clone());
                length_bytes(a8)
            };
            let a10 = f7(v0.clone());
            nat_add(a9, a10)?
        };
        if nat_eq(a11, a12) {
            if f8(v1.clone(), f6(v0.clone())) {
                let a15 = f9(v1.clone(), {
                    let a13 = f6(v0);
                    length_bytes(a13)
                })?;
                let a16 = {
                    let a14 = v1;
                    length_bytes(a14)
                };
                Ok(nat_eq(a15, a16))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f4(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m17 = f10(v0.clone(), v1)?;
    match m17 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a18 = f6(v0);
                let a19 = v2;
                append_bytes(a18, a19)
            }))
        }
    }
}

pub fn f5(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f11(f12(v0.clone(), v1)?);
    let v3: u64 = f11(f12(v0, {
        let a20 = v1;
        let a21 = 1u64;
        nat_add(a20, a21)?
    })?);
    let a29 = {
        let a26 = {
            let a24 = {
                let a22 = v2;
                let a23 = 16u64;
                nat_mul(a22, a23)?
            };
            let a25 = v3;
            nat_add(a24, a25)?
        };
        let a27 = 9u64;
        let a28 = 0u64;
        nat_rem(a26, a27, a28)
    };
    let a30 = 8u64;
    if nat_eq(a29, a30) {
        Ok(Bytes::lit(&[0x06]))
    } else {
        Ok(Bytes::lit(&[0x05]))
    }
}

pub fn f6(v0: Bytes) -> Bytes {
    let a31 = v0;
    let a32 = f2(Str::lit(":"));
    append_bytes(a31, a32)
}

pub fn f7(v0: Bytes) -> u64 {
    if f13(v0.clone(), Str::lit("sha256")) {
        64u64
    } else {
        if f13(v0.clone(), Str::lit("sha3-256")) {
            64u64
        } else {
            if f13(v0.clone(), Str::lit("keccak256")) {
                64u64
            } else {
                if f13(v0.clone(), Str::lit("sha512")) {
                    128u64
                } else {
                    if f13(v0, Str::lit("blake3")) {
                        64u64
                    } else {
                        0u64
                    }
                }
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: Bytes) -> bool {
    let m37 = {
        let a34 = v0;
        let a35 = 0u64;
        let a36 = {
            let a33 = v1.clone();
            length_bytes(a33)
        };
        slice_bytes(a34, a35, a36)
    };
    match m37 {
        None => {
            false
        }
        Some(v2) => {
            let a38 = v2;
            let a39 = v1;
            equal(a38, a39)
        }
    }
}

pub fn f9(v0: Bytes, v1: u64) -> R<u64> {
    let (h40, _) = f14(Fn0::F15(v0.clone()), {
        let a41 = v0;
        length_bytes(a41)
    }, v1)?;
    Ok(h40)
}

pub fn f10(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f13(v0.clone(), Str::lit("sha256")) {
        Ok(Some(f16(v1)?))
    } else {
        if f13(v0.clone(), Str::lit("sha3-256")) {
            Ok(Some(f17(v1)?))
        } else {
            if f13(v0.clone(), Str::lit("keccak256")) {
                Ok(Some(f18(v1)?))
            } else {
                if f13(v0.clone(), Str::lit("sha512")) {
                    Ok(Some(f19(v1)?))
                } else {
                    if f13(v0, Str::lit("blake3")) {
                        Ok(Some(f20(v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> u64 {
    let a42 = 47u64;
    let a43 = v0;
    if nat_lt(a42, a43) {
        let a44 = v0;
        let a45 = 58u64;
        if nat_lt(a44, a45) {
            let a46 = v0;
            let a47 = 48u64;
            nat_sub(a46, a47)
        } else {
            let a48 = 64u64;
            let a49 = v0;
            if nat_lt(a48, a49) {
                let a50 = v0;
                let a51 = 71u64;
                if nat_lt(a50, a51) {
                    let a52 = v0;
                    let a53 = 55u64;
                    nat_sub(a52, a53)
                } else {
                    let a54 = 96u64;
                    let a55 = v0;
                    if nat_lt(a54, a55) {
                        let a56 = v0;
                        let a57 = 103u64;
                        if nat_lt(a56, a57) {
                            let a58 = v0;
                            let a59 = 87u64;
                            nat_sub(a58, a59)
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

pub fn f12(v0: Bytes, v1: u64) -> R<u64> {
    let m62 = {
        let a60 = v0;
        let a61 = v1;
        index_bytes(a60, a61)
    };
    match m62 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a107 = {
                let a83 = {
                    let a71 = {
                        let a65 = {
                            let a63 = v3;
                            let a64 = 128u8;
                            fixed_u8::bit_and(a63, a64)
                        };
                        let a66 = 128u8;
                        if equal(a65, a66) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a72 = {
                        let a69 = {
                            let a67 = v3;
                            let a68 = 64u8;
                            fixed_u8::bit_and(a67, a68)
                        };
                        let a70 = 64u8;
                        if equal(a69, a70) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a71, a72)?
                };
                let a84 = {
                    let a81 = {
                        let a75 = {
                            let a73 = v3;
                            let a74 = 32u8;
                            fixed_u8::bit_and(a73, a74)
                        };
                        let a76 = 32u8;
                        if equal(a75, a76) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a82 = {
                        let a79 = {
                            let a77 = v3;
                            let a78 = 16u8;
                            fixed_u8::bit_and(a77, a78)
                        };
                        let a80 = 16u8;
                        if equal(a79, a80) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a81, a82)?
                };
                nat_add(a83, a84)?
            };
            let a108 = {
                let a105 = {
                    let a93 = {
                        let a87 = {
                            let a85 = v3;
                            let a86 = 8u8;
                            fixed_u8::bit_and(a85, a86)
                        };
                        let a88 = 8u8;
                        if equal(a87, a88) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a94 = {
                        let a91 = {
                            let a89 = v3;
                            let a90 = 4u8;
                            fixed_u8::bit_and(a89, a90)
                        };
                        let a92 = 4u8;
                        if equal(a91, a92) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a93, a94)?
                };
                let a106 = {
                    let a103 = {
                        let a97 = {
                            let a95 = v3;
                            let a96 = 2u8;
                            fixed_u8::bit_and(a95, a96)
                        };
                        let a98 = 2u8;
                        if equal(a97, a98) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a104 = {
                        let a101 = {
                            let a99 = v3;
                            let a100 = 1u8;
                            fixed_u8::bit_and(a99, a100)
                        };
                        let a102 = 1u8;
                        if equal(a101, a102) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a103, a104)?
                };
                nat_add(a105, a106)?
            };
            nat_add(a107, a108)
        }
    }
}

pub fn f13(v0: Bytes, v1: Str) -> bool {
    let a109 = v0;
    let a110 = f2(v1);
    equal(a109, a110)
}

pub fn f14(v0: Fn0, p115: u64, p116: u64) -> R<(u64, bool)> {
    let m114 = (p115, p116);
    {
        let mut m114 = m114;
        loop {
            let step = {
                let (v1, v2) = m114;
                let m111 = v1;
                if m111 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m111 - 1;
                    let m113 = {
                        let c112: Fn0 = v0.clone();
                        c112.apply(v2)?
                    };
                    match m113 {
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
                Ok(next) => { m114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f21(v0, v1)? {
        Ok(Some({
            let a117 = v1;
            let a118 = 1u64;
            nat_add(a117, a118)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f16(v0: Bytes) -> R<Bytes> {
    Ok(f22(f23(v0)?))
}

pub fn f17(v0: Bytes) -> R<Bytes> {
    f24(v0, 6u64)
}

pub fn f18(v0: Bytes) -> R<Bytes> {
    f24(v0, 1u64)
}

pub fn f19(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f25(v0)?;
    let a139 = f26(match v1.clone() {
        Adt0::C0(h119, _, _, _, _, _, _, _) => {
            h119
        }
    });
    let a140 = {
        let a137 = f26(match v1.clone() {
            Adt0::C0(_, h120, _, _, _, _, _, _) => {
                h120
            }
        });
        let a138 = {
            let a135 = f26(match v1.clone() {
                Adt0::C0(_, _, h121, _, _, _, _, _) => {
                    h121
                }
            });
            let a136 = {
                let a133 = f26(match v1.clone() {
                    Adt0::C0(_, _, _, h122, _, _, _, _) => {
                        h122
                    }
                });
                let a134 = {
                    let a131 = f26(match v1.clone() {
                        Adt0::C0(_, _, _, _, h123, _, _, _) => {
                            h123
                        }
                    });
                    let a132 = {
                        let a129 = f26(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h124, _, _) => {
                                h124
                            }
                        });
                        let a130 = {
                            let a127 = f26(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h125, _) => {
                                    h125
                                }
                            });
                            let a128 = f26(match v1 {
                                Adt0::C0(_, _, _, _, _, _, _, h126) => {
                                    h126
                                }
                            });
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
    };
    Ok(append_bytes(a139, a140))
}

pub fn f20(v0: Bytes) -> R<Bytes> {
    let v1: Adt1 = f27(v0)?;
    let a161 = f28(match v1.clone() {
        Adt1::C0(h141, _, _, _, _, _, _, _) => {
            h141
        }
    });
    let a162 = {
        let a159 = f28(match v1.clone() {
            Adt1::C0(_, h142, _, _, _, _, _, _) => {
                h142
            }
        });
        let a160 = {
            let a157 = f28(match v1.clone() {
                Adt1::C0(_, _, h143, _, _, _, _, _) => {
                    h143
                }
            });
            let a158 = {
                let a155 = f28(match v1.clone() {
                    Adt1::C0(_, _, _, h144, _, _, _, _) => {
                        h144
                    }
                });
                let a156 = {
                    let a153 = f28(match v1.clone() {
                        Adt1::C0(_, _, _, _, h145, _, _, _) => {
                            h145
                        }
                    });
                    let a154 = {
                        let a151 = f28(match v1.clone() {
                            Adt1::C0(_, _, _, _, _, h146, _, _) => {
                                h146
                            }
                        });
                        let a152 = {
                            let a149 = f28(match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, h147, _) => {
                                    h147
                                }
                            });
                            let a150 = f28(match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, h148) => {
                                    h148
                                }
                            });
                            append_bytes(a149, a150)
                        };
                        append_bytes(a151, a152)
                    };
                    append_bytes(a153, a154)
                };
                append_bytes(a155, a156)
            };
            append_bytes(a157, a158)
        };
        append_bytes(a159, a160)
    };
    Ok(append_bytes(a161, a162))
}

pub fn f21(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f12(v0, v1)?;
    let a163 = 47u64;
    let a164 = v2;
    if nat_lt(a163, a164) {
        let a165 = v2;
        let a166 = 58u64;
        if nat_lt(a165, a166) {
            Ok(true)
        } else {
            let a167 = 96u64;
            let a168 = v2;
            if nat_lt(a167, a168) {
                let a169 = v2;
                let a170 = 103u64;
                Ok(nat_lt(a169, a170))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt2) -> Bytes {
    let a191 = f29(match v0.clone() {
        Adt2::C0(h171, _, _, _, _, _, _, _) => {
            h171
        }
    });
    let a192 = {
        let a189 = f29(match v0.clone() {
            Adt2::C0(_, h172, _, _, _, _, _, _) => {
                h172
            }
        });
        let a190 = {
            let a187 = f29(match v0.clone() {
                Adt2::C0(_, _, h173, _, _, _, _, _) => {
                    h173
                }
            });
            let a188 = {
                let a185 = f29(match v0.clone() {
                    Adt2::C0(_, _, _, h174, _, _, _, _) => {
                        h174
                    }
                });
                let a186 = {
                    let a183 = f29(match v0.clone() {
                        Adt2::C0(_, _, _, _, h175, _, _, _) => {
                            h175
                        }
                    });
                    let a184 = {
                        let a181 = f29(match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h176, _, _) => {
                                h176
                            }
                        });
                        let a182 = {
                            let a179 = f29(match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h177, _) => {
                                    h177
                                }
                            });
                            let a180 = f29(match v0 {
                                Adt2::C0(_, _, _, _, _, _, _, h178) => {
                                    h178
                                }
                            });
                            append_bytes(a179, a180)
                        };
                        append_bytes(a181, a182)
                    };
                    append_bytes(a183, a184)
                };
                append_bytes(a185, a186)
            };
            append_bytes(a187, a188)
        };
        append_bytes(a189, a190)
    };
    append_bytes(a191, a192)
}

pub fn f23(v0: Bytes) -> R<Adt2> {
    let v1: u64 = {
        let a193 = v0.clone();
        length_bytes(a193)
    };
    let v2: u64 = {
        let a199 = {
            let a196 = {
                let a194 = v1;
                let a195 = 72u64;
                nat_add(a194, a195)?
            };
            let a197 = 64u64;
            let a198 = 0u64;
            nat_quot(a196, a197, a198)
        };
        let a200 = 64u64;
        nat_mul(a199, a200)?
    };
    let (_, h201) = f30(Fn1::F31(v0, v1, v2), {
        let a202 = v2;
        let a203 = 64u64;
        let a204 = 0u64;
        nat_quot(a202, a203, a204)
    }, (0u64, f32()))?;
    Ok(h201)
}

pub fn f24(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt3 = f33(v0, v1)?;
    let a213 = f34(match v2.clone() {
        Adt3::C0(h205, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h205
        }
    });
    let a214 = {
        let a211 = f34(match v2.clone() {
            Adt3::C0(_, h206, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h206
            }
        });
        let a212 = {
            let a209 = f34(match v2.clone() {
                Adt3::C0(_, _, h207, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h207
                }
            });
            let a210 = f34(match v2 {
                Adt3::C0(_, _, _, h208, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h208
                }
            });
            append_bytes(a209, a210)
        };
        append_bytes(a211, a212)
    };
    Ok(append_bytes(a213, a214))
}

pub fn f25(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a215 = v0.clone();
        length_bytes(a215)
    };
    let v2: u64 = {
        let a221 = {
            let a218 = {
                let a216 = v1;
                let a217 = 144u64;
                nat_add(a216, a217)?
            };
            let a219 = 128u64;
            let a220 = 0u64;
            nat_quot(a218, a219, a220)
        };
        let a222 = 128u64;
        nat_mul(a221, a222)?
    };
    let (_, h223) = f35(Fn2::F36(v0, v1, v2), {
        let a224 = v2;
        let a225 = 128u64;
        let a226 = 0u64;
        nat_quot(a224, a225, a226)
    }, (0u64, f37()))?;
    Ok(h223)
}

pub fn f26(v0: u64) -> Bytes {
    let a255 = f38(v0, 60u32);
    let a256 = {
        let a253 = f38(v0, 56u32);
        let a254 = {
            let a251 = f38(v0, 52u32);
            let a252 = {
                let a249 = f38(v0, 48u32);
                let a250 = {
                    let a247 = f38(v0, 44u32);
                    let a248 = {
                        let a245 = f38(v0, 40u32);
                        let a246 = {
                            let a243 = f38(v0, 36u32);
                            let a244 = {
                                let a241 = f38(v0, 32u32);
                                let a242 = {
                                    let a239 = f38(v0, 28u32);
                                    let a240 = {
                                        let a237 = f38(v0, 24u32);
                                        let a238 = {
                                            let a235 = f38(v0, 20u32);
                                            let a236 = {
                                                let a233 = f38(v0, 16u32);
                                                let a234 = {
                                                    let a231 = f38(v0, 12u32);
                                                    let a232 = {
                                                        let a229 = f38(v0, 8u32);
                                                        let a230 = {
                                                            let a227 = f38(v0, 4u32);
                                                            let a228 = f38(v0, 0u32);
                                                            append_bytes(a227, a228)
                                                        };
                                                        append_bytes(a229, a230)
                                                    };
                                                    append_bytes(a231, a232)
                                                };
                                                append_bytes(a233, a234)
                                            };
                                            append_bytes(a235, a236)
                                        };
                                        append_bytes(a237, a238)
                                    };
                                    append_bytes(a239, a240)
                                };
                                append_bytes(a241, a242)
                            };
                            append_bytes(a243, a244)
                        };
                        append_bytes(a245, a246)
                    };
                    append_bytes(a247, a248)
                };
                append_bytes(a249, a250)
            };
            append_bytes(a251, a252)
        };
        append_bytes(a253, a254)
    };
    append_bytes(a255, a256)
}

pub fn f27(v0: Bytes) -> R<Adt1> {
    let v1: u64 = {
        let a257 = v0.clone();
        length_bytes(a257)
    };
    let v2: u64 = {
        let a258 = v1;
        let a259 = 0u64;
        if nat_eq(a258, a259) {
            1u64
        } else {
            let a262 = {
                let a260 = v1;
                let a261 = 1023u64;
                nat_add(a260, a261)?
            };
            let a263 = 1024u64;
            let a264 = 0u64;
            nat_quot(a262, a263, a264)
        }
    };
    let v3: u64 = {
        let a265 = v2;
        let a266 = 1u64;
        nat_sub(a265, a266)
    };
    let m268 = {
        let (_, h267) = f39(v0.clone(), v1, v3)?;
        h267
    };
    match m268.uncons() {
        None => {
            f40(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h269) = f41(Fn3::F42, 64u64, (List::cons(v4, v5), f40(v0, v1, v3, false)?));
            Ok(h269)
        }
    }
}

pub fn f28(v0: u32) -> Bytes {
    let a282 = {
        let a270 = f43(v0, 4u32);
        let a271 = f43(v0, 0u32);
        append_bytes(a270, a271)
    };
    let a283 = {
        let a280 = {
            let a272 = f43(v0, 12u32);
            let a273 = f43(v0, 8u32);
            append_bytes(a272, a273)
        };
        let a281 = {
            let a278 = {
                let a274 = f43(v0, 20u32);
                let a275 = f43(v0, 16u32);
                append_bytes(a274, a275)
            };
            let a279 = {
                let a276 = f43(v0, 28u32);
                let a277 = f43(v0, 24u32);
                append_bytes(a276, a277)
            };
            append_bytes(a278, a279)
        };
        append_bytes(a280, a281)
    };
    append_bytes(a282, a283)
}

pub fn f29(v0: u32) -> Bytes {
    let a312 = f44({
        let a284 = f45(v0, 28u32);
        let a285 = 15u32;
        fixed_u32::bit_and(a284, a285)
    });
    let a313 = {
        let a310 = f44({
            let a286 = f45(v0, 24u32);
            let a287 = 15u32;
            fixed_u32::bit_and(a286, a287)
        });
        let a311 = {
            let a308 = f44({
                let a288 = f45(v0, 20u32);
                let a289 = 15u32;
                fixed_u32::bit_and(a288, a289)
            });
            let a309 = {
                let a306 = f44({
                    let a290 = f45(v0, 16u32);
                    let a291 = 15u32;
                    fixed_u32::bit_and(a290, a291)
                });
                let a307 = {
                    let a304 = f44({
                        let a292 = f45(v0, 12u32);
                        let a293 = 15u32;
                        fixed_u32::bit_and(a292, a293)
                    });
                    let a305 = {
                        let a302 = f44({
                            let a294 = f45(v0, 8u32);
                            let a295 = 15u32;
                            fixed_u32::bit_and(a294, a295)
                        });
                        let a303 = {
                            let a300 = f44({
                                let a296 = f45(v0, 4u32);
                                let a297 = 15u32;
                                fixed_u32::bit_and(a296, a297)
                            });
                            let a301 = f44({
                                let a298 = v0;
                                let a299 = 15u32;
                                fixed_u32::bit_and(a298, a299)
                            });
                            append_bytes(a300, a301)
                        };
                        append_bytes(a302, a303)
                    };
                    append_bytes(a304, a305)
                };
                append_bytes(a306, a307)
            };
            append_bytes(a308, a309)
        };
        append_bytes(a310, a311)
    };
    append_bytes(a312, a313)
}

pub fn f30(v0: Fn1, p317: u64, p318: (u64, Adt2)) -> R<(u64, Adt2)> {
    let m316 = (p317, p318);
    {
        let mut m316 = m316;
        loop {
            let step = {
                let (v1, v2) = m316;
                let m314 = v1;
                if m314 == 0 {
                    Err::<(u64, (u64, Adt2)), R<(u64, Adt2)>>(Ok(v2))
                } else {
                    let v3 = m314 - 1;
                    Ok::<(u64, (u64, Adt2)), R<(u64, Adt2)>>((v3, {
                        let c315: Fn1 = v0.clone();
                        c315.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt2)) -> R<(u64, Adt2)> {
    Ok(({
        let a320 = {
            let (h319, _) = v3.clone();
            h319
        };
        let a321 = 64u64;
        nat_add(a320, a321)?
    }, f46({
        let (_, h322) = v3;
        h322
    }, f47(v0, v1, v2, {
        let (h323, _) = v3;
        h323
    })?)?))
}

pub fn f32() -> Adt2 {
    Adt2::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f33(v0: Bytes, v1: u64) -> R<Adt3> {
    let v2: u64 = {
        let a324 = v0.clone();
        length_bytes(a324)
    };
    let v3: u64 = {
        let a330 = {
            let a328 = {
                let a325 = v2;
                let a326 = 136u64;
                let a327 = 0u64;
                nat_quot(a325, a326, a327)
            };
            let a329 = 1u64;
            nat_add(a328, a329)?
        };
        let a331 = 136u64;
        nat_mul(a330, a331)?
    };
    let (_, h332) = f48(Fn4::F49(v0, v2, v1, v3), {
        let a333 = v3;
        let a334 = 136u64;
        let a335 = 0u64;
        nat_quot(a333, a334, a335)
    }, (0u64, f50()))?;
    Ok(h332)
}

pub fn f34(v0: u64) -> Bytes {
    let a364 = {
        let a336 = f38(v0, 4u32);
        let a337 = f38(v0, 0u32);
        append_bytes(a336, a337)
    };
    let a365 = {
        let a362 = {
            let a338 = f38(v0, 12u32);
            let a339 = f38(v0, 8u32);
            append_bytes(a338, a339)
        };
        let a363 = {
            let a360 = {
                let a340 = f38(v0, 20u32);
                let a341 = f38(v0, 16u32);
                append_bytes(a340, a341)
            };
            let a361 = {
                let a358 = {
                    let a342 = f38(v0, 28u32);
                    let a343 = f38(v0, 24u32);
                    append_bytes(a342, a343)
                };
                let a359 = {
                    let a356 = {
                        let a344 = f38(v0, 36u32);
                        let a345 = f38(v0, 32u32);
                        append_bytes(a344, a345)
                    };
                    let a357 = {
                        let a354 = {
                            let a346 = f38(v0, 44u32);
                            let a347 = f38(v0, 40u32);
                            append_bytes(a346, a347)
                        };
                        let a355 = {
                            let a352 = {
                                let a348 = f38(v0, 52u32);
                                let a349 = f38(v0, 48u32);
                                append_bytes(a348, a349)
                            };
                            let a353 = {
                                let a350 = f38(v0, 60u32);
                                let a351 = f38(v0, 56u32);
                                append_bytes(a350, a351)
                            };
                            append_bytes(a352, a353)
                        };
                        append_bytes(a354, a355)
                    };
                    append_bytes(a356, a357)
                };
                append_bytes(a358, a359)
            };
            append_bytes(a360, a361)
        };
        append_bytes(a362, a363)
    };
    append_bytes(a364, a365)
}

pub fn f35(v0: Fn2, p369: u64, p370: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m368 = (p369, p370);
    {
        let mut m368 = m368;
        loop {
            let step = {
                let (v1, v2) = m368;
                let m366 = v1;
                if m366 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m366 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c367: Fn2 = v0.clone();
                        c367.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a372 = {
            let (h371, _) = v3.clone();
            h371
        };
        let a373 = 128u64;
        nat_add(a372, a373)?
    }, f51({
        let (_, h374) = v3;
        h374
    }, f52(v0, v1, v2, {
        let (h375, _) = v3;
        h375
    })?)?))
}

pub fn f37() -> Adt0 {
    Adt0::C0(7640891576956012808u64, 13503953896175478587u64, 4354685564936845355u64, 11912009170470909681u64, 5840696475078001361u64, 11170449401992604703u64, 2270897969802886507u64, 6620516959819538809u64)
}

pub fn f38(v0: u64, v1: u32) -> Bytes {
    f44(f53(f54(v0, v1)))
}

pub fn f39(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt1>)> {
    f55(Fn5::F56(v0, v1), v2, (0u64, List::<Adt1>::nil()))
}

pub fn f40(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt1> {
    let v4: u64 = {
        let a376 = 1024u64;
        let a377 = v2;
        nat_mul(a376, a377)?
    };
    let v5: u64 = f57(v1, v4);
    let v6: u64 = f58(v5)?;
    let v7: u64 = {
        let a378 = v6;
        let a379 = 1u64;
        nat_sub(a378, a379)
    };
    Ok(f59(f60(v0.clone(), v4, v2, v7)?, f61(v0, {
        let a382 = v4;
        let a383 = {
            let a380 = 64u64;
            let a381 = v7;
            nat_mul(a380, a381)?
        };
        nat_add(a382, a383)?
    })?, v2, f62({
        let a386 = v5;
        let a387 = {
            let a384 = 64u64;
            let a385 = v7;
            nat_mul(a384, a385)?
        };
        nat_sub(a386, a387)
    }), {
        let a390 = {
            let a388 = v7;
            let a389 = 0u64;
            if nat_eq(a388, a389) {
                3u32
            } else {
                2u32
            }
        };
        let a391 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a390, a391)
    }))
}

pub fn f41(v0: Fn3, p395: u64, p396: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v1, v2) = m394;
                let m392 = v1;
                if m392 == 0 {
                    Err::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>(v2)
                } else {
                    let v3 = m392 - 1;
                    Ok::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>((v3, {
                        let c393: Fn3 = v0.clone();
                        c393.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    f63(v0)
}

pub fn f43(v0: u32, v1: u32) -> Bytes {
    f44(f45(v0, v1))
}

pub fn f44(v0: u32) -> Bytes {
    let a399 = {
        let a397 = v0;
        let a398 = 8u32;
        fixed_u32::bit_and(a397, a398)
    };
    let a400 = 0u32;
    if equal(a399, a400) {
        let a403 = {
            let a401 = v0;
            let a402 = 4u32;
            fixed_u32::bit_and(a401, a402)
        };
        let a404 = 0u32;
        if equal(a403, a404) {
            let a407 = {
                let a405 = v0;
                let a406 = 2u32;
                fixed_u32::bit_and(a405, a406)
            };
            let a408 = 0u32;
            if equal(a407, a408) {
                let a411 = {
                    let a409 = v0;
                    let a410 = 1u32;
                    fixed_u32::bit_and(a409, a410)
                };
                let a412 = 0u32;
                if equal(a411, a412) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a415 = {
                    let a413 = v0;
                    let a414 = 1u32;
                    fixed_u32::bit_and(a413, a414)
                };
                let a416 = 0u32;
                if equal(a415, a416) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a419 = {
                let a417 = v0;
                let a418 = 2u32;
                fixed_u32::bit_and(a417, a418)
            };
            let a420 = 0u32;
            if equal(a419, a420) {
                let a423 = {
                    let a421 = v0;
                    let a422 = 1u32;
                    fixed_u32::bit_and(a421, a422)
                };
                let a424 = 0u32;
                if equal(a423, a424) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a427 = {
                    let a425 = v0;
                    let a426 = 1u32;
                    fixed_u32::bit_and(a425, a426)
                };
                let a428 = 0u32;
                if equal(a427, a428) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a431 = {
            let a429 = v0;
            let a430 = 4u32;
            fixed_u32::bit_and(a429, a430)
        };
        let a432 = 0u32;
        if equal(a431, a432) {
            let a435 = {
                let a433 = v0;
                let a434 = 2u32;
                fixed_u32::bit_and(a433, a434)
            };
            let a436 = 0u32;
            if equal(a435, a436) {
                let a439 = {
                    let a437 = v0;
                    let a438 = 1u32;
                    fixed_u32::bit_and(a437, a438)
                };
                let a440 = 0u32;
                if equal(a439, a440) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a443 = {
                    let a441 = v0;
                    let a442 = 1u32;
                    fixed_u32::bit_and(a441, a442)
                };
                let a444 = 0u32;
                if equal(a443, a444) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a447 = {
                let a445 = v0;
                let a446 = 2u32;
                fixed_u32::bit_and(a445, a446)
            };
            let a448 = 0u32;
            if equal(a447, a448) {
                let a451 = {
                    let a449 = v0;
                    let a450 = 1u32;
                    fixed_u32::bit_and(a449, a450)
                };
                let a452 = 0u32;
                if equal(a451, a452) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a455 = {
                    let a453 = v0;
                    let a454 = 1u32;
                    fixed_u32::bit_and(a453, a454)
                };
                let a456 = 0u32;
                if equal(a455, a456) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f45(v0: u32, v1: u32) -> u32 {
    let m459 = {
        let a457 = v0;
        let a458 = v1;
        fixed_u32::shift_right(a457, a458)
    };
    match m459 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f46(v0: Adt2, v1: Adt4) -> R<Adt2> {
    let v2: Adt5 = f64(Fn6::F65, 64u64, Adt5::C0(match v0.clone() {
        Adt2::C0(h460, _, _, _, _, _, _, _) => {
            h460
        }
    }, match v0.clone() {
        Adt2::C0(_, h461, _, _, _, _, _, _) => {
            h461
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h462, _, _, _, _, _) => {
            h462
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h463, _, _, _, _) => {
            h463
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h464, _, _, _) => {
            h464
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h465, _, _) => {
            h465
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h466, _) => {
            h466
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h467) => {
            h467
        }
    }, v1, 0u64))?;
    Ok(Adt2::C0(f66(match v0.clone() {
        Adt2::C0(h468, _, _, _, _, _, _, _) => {
            h468
        }
    }, match v2.clone() {
        Adt5::C0(h469, _, _, _, _, _, _, _, _, _) => {
            h469
        }
    }), f66(match v0.clone() {
        Adt2::C0(_, h470, _, _, _, _, _, _) => {
            h470
        }
    }, match v2.clone() {
        Adt5::C0(_, h471, _, _, _, _, _, _, _, _) => {
            h471
        }
    }), f66(match v0.clone() {
        Adt2::C0(_, _, h472, _, _, _, _, _) => {
            h472
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h473, _, _, _, _, _, _, _) => {
            h473
        }
    }), f66(match v0.clone() {
        Adt2::C0(_, _, _, h474, _, _, _, _) => {
            h474
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, h475, _, _, _, _, _, _) => {
            h475
        }
    }), f66(match v0.clone() {
        Adt2::C0(_, _, _, _, h476, _, _, _) => {
            h476
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, h477, _, _, _, _, _) => {
            h477
        }
    }), f66(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h478, _, _) => {
            h478
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, h479, _, _, _, _) => {
            h479
        }
    }), f66(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h480, _) => {
            h480
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h481, _, _, _) => {
            h481
        }
    }), f66(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h482) => {
            h482
        }
    }, match v2 {
        Adt5::C0(_, _, _, _, _, _, _, h483, _, _) => {
            h483
        }
    })))
}

pub fn f47(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    Ok(Adt4::C0(f67(v0.clone(), v1, v2, v3)?, f67(v0.clone(), v1, v2, {
        let a484 = v3;
        let a485 = 4u64;
        nat_add(a484, a485)?
    })?, f67(v0.clone(), v1, v2, {
        let a486 = v3;
        let a487 = 8u64;
        nat_add(a486, a487)?
    })?, f67(v0.clone(), v1, v2, {
        let a488 = v3;
        let a489 = 12u64;
        nat_add(a488, a489)?
    })?, f67(v0.clone(), v1, v2, {
        let a490 = v3;
        let a491 = 16u64;
        nat_add(a490, a491)?
    })?, f67(v0.clone(), v1, v2, {
        let a492 = v3;
        let a493 = 20u64;
        nat_add(a492, a493)?
    })?, f67(v0.clone(), v1, v2, {
        let a494 = v3;
        let a495 = 24u64;
        nat_add(a494, a495)?
    })?, f67(v0.clone(), v1, v2, {
        let a496 = v3;
        let a497 = 28u64;
        nat_add(a496, a497)?
    })?, f67(v0.clone(), v1, v2, {
        let a498 = v3;
        let a499 = 32u64;
        nat_add(a498, a499)?
    })?, f67(v0.clone(), v1, v2, {
        let a500 = v3;
        let a501 = 36u64;
        nat_add(a500, a501)?
    })?, f67(v0.clone(), v1, v2, {
        let a502 = v3;
        let a503 = 40u64;
        nat_add(a502, a503)?
    })?, f67(v0.clone(), v1, v2, {
        let a504 = v3;
        let a505 = 44u64;
        nat_add(a504, a505)?
    })?, f67(v0.clone(), v1, v2, {
        let a506 = v3;
        let a507 = 48u64;
        nat_add(a506, a507)?
    })?, f67(v0.clone(), v1, v2, {
        let a508 = v3;
        let a509 = 52u64;
        nat_add(a508, a509)?
    })?, f67(v0.clone(), v1, v2, {
        let a510 = v3;
        let a511 = 56u64;
        nat_add(a510, a511)?
    })?, f67(v0, v1, v2, {
        let a512 = v3;
        let a513 = 60u64;
        nat_add(a512, a513)?
    })?))
}

pub fn f48(v0: Fn4, p517: u64, p518: (u64, Adt3)) -> R<(u64, Adt3)> {
    let m516 = (p517, p518);
    {
        let mut m516 = m516;
        loop {
            let step = {
                let (v1, v2) = m516;
                let m514 = v1;
                if m514 == 0 {
                    Err::<(u64, (u64, Adt3)), R<(u64, Adt3)>>(Ok(v2))
                } else {
                    let v3 = m514 - 1;
                    Ok::<(u64, (u64, Adt3)), R<(u64, Adt3)>>((v3, {
                        let c515: Fn4 = v0.clone();
                        c515.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m516 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a520 = {
            let (h519, _) = v4.clone();
            h519
        };
        let a521 = 136u64;
        nat_add(a520, a521)?
    }, f68(v0, v1, v3, v2, {
        let (_, h522) = v4;
        h522
    }, {
        let (h523, _) = v4;
        h523
    })?))
}

pub fn f50() -> Adt3 {
    Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f51(v0: Adt0, v1: Adt6) -> R<Adt0> {
    let v2: Adt7 = f69(Fn7::F70, 80u64, Adt7::C0(match v0.clone() {
        Adt0::C0(h524, _, _, _, _, _, _, _) => {
            h524
        }
    }, match v0.clone() {
        Adt0::C0(_, h525, _, _, _, _, _, _) => {
            h525
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h526, _, _, _, _, _) => {
            h526
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h527, _, _, _, _) => {
            h527
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h528, _, _, _) => {
            h528
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h529, _, _) => {
            h529
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h530, _) => {
            h530
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h531) => {
            h531
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f71(match v0.clone() {
        Adt0::C0(h532, _, _, _, _, _, _, _) => {
            h532
        }
    }, match v2.clone() {
        Adt7::C0(h533, _, _, _, _, _, _, _, _, _) => {
            h533
        }
    }), f71(match v0.clone() {
        Adt0::C0(_, h534, _, _, _, _, _, _) => {
            h534
        }
    }, match v2.clone() {
        Adt7::C0(_, h535, _, _, _, _, _, _, _, _) => {
            h535
        }
    }), f71(match v0.clone() {
        Adt0::C0(_, _, h536, _, _, _, _, _) => {
            h536
        }
    }, match v2.clone() {
        Adt7::C0(_, _, h537, _, _, _, _, _, _, _) => {
            h537
        }
    }), f71(match v0.clone() {
        Adt0::C0(_, _, _, h538, _, _, _, _) => {
            h538
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, h539, _, _, _, _, _, _) => {
            h539
        }
    }), f71(match v0.clone() {
        Adt0::C0(_, _, _, _, h540, _, _, _) => {
            h540
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, h541, _, _, _, _, _) => {
            h541
        }
    }), f71(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h542, _, _) => {
            h542
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, h543, _, _, _, _) => {
            h543
        }
    }), f71(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h544, _) => {
            h544
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, _, h545, _, _, _) => {
            h545
        }
    }), f71(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h546) => {
            h546
        }
    }, match v2 {
        Adt7::C0(_, _, _, _, _, _, _, h547, _, _) => {
            h547
        }
    })))
}

pub fn f52(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    Ok(Adt6::C0(f72(v0.clone(), v1, v2, {
        let a548 = v3;
        let a549 = 0u64;
        nat_add(a548, a549)?
    })?, f72(v0.clone(), v1, v2, {
        let a550 = v3;
        let a551 = 8u64;
        nat_add(a550, a551)?
    })?, f72(v0.clone(), v1, v2, {
        let a552 = v3;
        let a553 = 16u64;
        nat_add(a552, a553)?
    })?, f72(v0.clone(), v1, v2, {
        let a554 = v3;
        let a555 = 24u64;
        nat_add(a554, a555)?
    })?, f72(v0.clone(), v1, v2, {
        let a556 = v3;
        let a557 = 32u64;
        nat_add(a556, a557)?
    })?, f72(v0.clone(), v1, v2, {
        let a558 = v3;
        let a559 = 40u64;
        nat_add(a558, a559)?
    })?, f72(v0.clone(), v1, v2, {
        let a560 = v3;
        let a561 = 48u64;
        nat_add(a560, a561)?
    })?, f72(v0.clone(), v1, v2, {
        let a562 = v3;
        let a563 = 56u64;
        nat_add(a562, a563)?
    })?, f72(v0.clone(), v1, v2, {
        let a564 = v3;
        let a565 = 64u64;
        nat_add(a564, a565)?
    })?, f72(v0.clone(), v1, v2, {
        let a566 = v3;
        let a567 = 72u64;
        nat_add(a566, a567)?
    })?, f72(v0.clone(), v1, v2, {
        let a568 = v3;
        let a569 = 80u64;
        nat_add(a568, a569)?
    })?, f72(v0.clone(), v1, v2, {
        let a570 = v3;
        let a571 = 88u64;
        nat_add(a570, a571)?
    })?, f72(v0.clone(), v1, v2, {
        let a572 = v3;
        let a573 = 96u64;
        nat_add(a572, a573)?
    })?, f72(v0.clone(), v1, v2, {
        let a574 = v3;
        let a575 = 104u64;
        nat_add(a574, a575)?
    })?, f72(v0.clone(), v1, v2, {
        let a576 = v3;
        let a577 = 112u64;
        nat_add(a576, a577)?
    })?, f72(v0, v1, v2, {
        let a578 = v3;
        let a579 = 120u64;
        nat_add(a578, a579)?
    })?))
}

pub fn f53(v0: u64) -> u32 {
    let m583 = {
        let a582 = {
            let a580 = v0;
            let a581 = 4294967295u64;
            fixed_u64::bit_and(a580, a581)
        };
        fixed_u32::convert(i128::from(a582))
    };
    match m583 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f54(v0: u64, v1: u32) -> u64 {
    let m586 = {
        let a584 = v0;
        let a585 = v1;
        fixed_u64::shift_right(a584, a585)
    };
    match m586 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f55(v0: Fn5, p590: u64, p591: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    let m589 = (p590, p591);
    {
        let mut m589 = m589;
        loop {
            let step = {
                let (v1, v2) = m589;
                let m587 = v1;
                if m587 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>(Ok(v2))
                } else {
                    let v3 = m587 - 1;
                    Ok::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>((v3, {
                        let c588: Fn5 = v0.clone();
                        c588.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m589 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: u64, v2: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    f73(v2.clone(), f40(v0, v1, {
        let (h592, _) = v2;
        h592
    }, false)?)
}

pub fn f57(v0: u64, v1: u64) -> u64 {
    let a595 = 1024u64;
    let a596 = {
        let a593 = v0;
        let a594 = v1;
        nat_sub(a593, a594)
    };
    if nat_lt(a595, a596) {
        1024u64
    } else {
        let a597 = v0;
        let a598 = v1;
        nat_sub(a597, a598)
    }
}

pub fn f58(v0: u64) -> R<u64> {
    let a599 = v0;
    let a600 = 0u64;
    if nat_eq(a599, a600) {
        Ok(1u64)
    } else {
        let a603 = {
            let a601 = v0;
            let a602 = 63u64;
            nat_add(a601, a602)?
        };
        let a604 = 64u64;
        let a605 = 0u64;
        Ok(nat_quot(a603, a604, a605))
    }
}

pub fn f59(v0: Adt1, v1: Adt8, v2: u64, v3: u32, v4: u32) -> Adt1 {
    let v5: Adt9 = {
        let (h606, _) = f74(Fn8::F75, 7u64, (Adt9::C0(match v0.clone() {
            Adt1::C0(h607, _, _, _, _, _, _, _) => {
                h607
            }
        }, match v0.clone() {
            Adt1::C0(_, h608, _, _, _, _, _, _) => {
                h608
            }
        }, match v0.clone() {
            Adt1::C0(_, _, h609, _, _, _, _, _) => {
                h609
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h610, _, _, _, _) => {
                h610
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h611, _, _, _) => {
                h611
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h612, _, _) => {
                h612
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h613, _) => {
                h613
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, h614) => {
                h614
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f62({
            let a615 = v2;
            let a616 = 4294967296u64;
            let a617 = 0u64;
            nat_rem(a615, a616, a617)
        }), f62({
            let a618 = v2;
            let a619 = 4294967296u64;
            let a620 = 0u64;
            nat_quot(a618, a619, a620)
        }), v3, v4), v1));
        h606
    };
    Adt1::C0({
        let a623 = match v5.clone() {
            Adt9::C0(h621, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h621
            }
        };
        let a624 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, h622, _, _, _, _, _, _, _) => {
                h622
            }
        };
        fixed_u32::bit_xor(a623, a624)
    }, {
        let a627 = match v5.clone() {
            Adt9::C0(_, h625, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h625
            }
        };
        let a628 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, h626, _, _, _, _, _, _) => {
                h626
            }
        };
        fixed_u32::bit_xor(a627, a628)
    }, {
        let a631 = match v5.clone() {
            Adt9::C0(_, _, h629, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h629
            }
        };
        let a632 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, h630, _, _, _, _, _) => {
                h630
            }
        };
        fixed_u32::bit_xor(a631, a632)
    }, {
        let a635 = match v5.clone() {
            Adt9::C0(_, _, _, h633, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h633
            }
        };
        let a636 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h634, _, _, _, _) => {
                h634
            }
        };
        fixed_u32::bit_xor(a635, a636)
    }, {
        let a639 = match v5.clone() {
            Adt9::C0(_, _, _, _, h637, _, _, _, _, _, _, _, _, _, _, _) => {
                h637
            }
        };
        let a640 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h638, _, _, _) => {
                h638
            }
        };
        fixed_u32::bit_xor(a639, a640)
    }, {
        let a643 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, h641, _, _, _, _, _, _, _, _, _, _) => {
                h641
            }
        };
        let a644 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h642, _, _) => {
                h642
            }
        };
        fixed_u32::bit_xor(a643, a644)
    }, {
        let a647 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, h645, _, _, _, _, _, _, _, _, _) => {
                h645
            }
        };
        let a648 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h646, _) => {
                h646
            }
        };
        fixed_u32::bit_xor(a647, a648)
    }, {
        let a651 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, h649, _, _, _, _, _, _, _, _) => {
                h649
            }
        };
        let a652 = match v5 {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h650) => {
                h650
            }
        };
        fixed_u32::bit_xor(a651, a652)
    })
}

pub fn f60(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (_, h653) = f76(Fn9::F77(v2, v0, v1), v3, (0u64, f78()))?;
    Ok(h653)
}

pub fn f61(v0: Bytes, v1: u64) -> R<Adt8> {
    Ok(Adt8::C0(f79(v0.clone(), {
        let a654 = v1;
        let a655 = 0u64;
        nat_add(a654, a655)?
    })?, f79(v0.clone(), {
        let a656 = v1;
        let a657 = 4u64;
        nat_add(a656, a657)?
    })?, f79(v0.clone(), {
        let a658 = v1;
        let a659 = 8u64;
        nat_add(a658, a659)?
    })?, f79(v0.clone(), {
        let a660 = v1;
        let a661 = 12u64;
        nat_add(a660, a661)?
    })?, f79(v0.clone(), {
        let a662 = v1;
        let a663 = 16u64;
        nat_add(a662, a663)?
    })?, f79(v0.clone(), {
        let a664 = v1;
        let a665 = 20u64;
        nat_add(a664, a665)?
    })?, f79(v0.clone(), {
        let a666 = v1;
        let a667 = 24u64;
        nat_add(a666, a667)?
    })?, f79(v0.clone(), {
        let a668 = v1;
        let a669 = 28u64;
        nat_add(a668, a669)?
    })?, f79(v0.clone(), {
        let a670 = v1;
        let a671 = 32u64;
        nat_add(a670, a671)?
    })?, f79(v0.clone(), {
        let a672 = v1;
        let a673 = 36u64;
        nat_add(a672, a673)?
    })?, f79(v0.clone(), {
        let a674 = v1;
        let a675 = 40u64;
        nat_add(a674, a675)?
    })?, f79(v0.clone(), {
        let a676 = v1;
        let a677 = 44u64;
        nat_add(a676, a677)?
    })?, f79(v0.clone(), {
        let a678 = v1;
        let a679 = 48u64;
        nat_add(a678, a679)?
    })?, f79(v0.clone(), {
        let a680 = v1;
        let a681 = 52u64;
        nat_add(a680, a681)?
    })?, f79(v0.clone(), {
        let a682 = v1;
        let a683 = 56u64;
        nat_add(a682, a683)?
    })?, f79(v0, {
        let a684 = v1;
        let a685 = 60u64;
        nat_add(a684, a685)?
    })?))
}

pub fn f62(v0: u64) -> u32 {
    let (_, h686) = {
        let (_, h687) = f80(Fn10::F81, 32u64, (v0, (1u32, 0u32)));
        h687
    };
    h686
}

pub fn f63(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m689 = {
        let (h688, _) = v0.clone();
        h688
    };
    match m689.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f82(v1, {
                let (_, h690) = v0;
                h690
            }, {
                let m691 = v2;
                match m691.uncons() {
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

pub fn f64(v0: Fn6, p695: u64, p696: Adt5) -> R<Adt5> {
    let m694 = (p695, p696);
    {
        let mut m694 = m694;
        loop {
            let step = {
                let (v1, v2) = m694;
                let m692 = v1;
                if m692 == 0 {
                    Err::<(u64, Adt5), R<Adt5>>(Ok(v2))
                } else {
                    let v3 = m692 - 1;
                    Ok::<(u64, Adt5), R<Adt5>>((v3, {
                        let c693: Fn6 = v0.clone();
                        c693.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt5) -> R<Adt5> {
    f83(v0)
}

pub fn f66(v0: u32, v1: u32) -> u32 {
    let m699 = {
        let a697 = f84(v0);
        let a698 = f84(v1);
        fixed_u64::checked_add(a697, a698)
    };
    match m699 {
        None => {
            0u32
        }
        Some(v2) => {
            f53(v2)
        }
    }
}

pub fn f67(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a702 = {
        let a700 = v3;
        let a701 = 8u64;
        nat_add(a700, a701)?
    };
    let a703 = v2;
    if nat_eq(a702, a703) {
        Ok(f62({
            let a706 = {
                let a704 = v1;
                let a705 = 8u64;
                nat_mul(a704, a705)?
            };
            let a707 = 4294967296u64;
            let a708 = 0u64;
            nat_quot(a706, a707, a708)
        }))
    } else {
        let a711 = {
            let a709 = v3;
            let a710 = 4u64;
            nat_add(a709, a710)?
        };
        let a712 = v2;
        if nat_eq(a711, a712) {
            Ok(f62({
                let a715 = {
                    let a713 = v1;
                    let a714 = 8u64;
                    nat_mul(a713, a714)?
                };
                let a716 = 4294967296u64;
                let a717 = 0u64;
                nat_rem(a715, a716, a717)
            }))
        } else {
            Ok(f85(f86(v0.clone(), v1, v3), f86(v0.clone(), v1, {
                let a718 = v3;
                let a719 = 1u64;
                nat_add(a718, a719)?
            }), f86(v0.clone(), v1, {
                let a720 = v3;
                let a721 = 2u64;
                nat_add(a720, a721)?
            }), f86(v0, v1, {
                let a722 = v3;
                let a723 = 3u64;
                nat_add(a722, a723)?
            })))
        }
    }
}

pub fn f68(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: u64) -> R<Adt3> {
    f87(Adt3::C0({
        let a727 = match v4.clone() {
            Adt3::C0(h724, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h724
            }
        };
        let a728 = f88(v0.clone(), v1, v2, v3, {
            let a725 = v5;
            let a726 = 0u64;
            nat_add(a725, a726)?
        })?;
        fixed_u64::bit_xor(a727, a728)
    }, {
        let a732 = match v4.clone() {
            Adt3::C0(_, h729, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h729
            }
        };
        let a733 = f88(v0.clone(), v1, v2, v3, {
            let a730 = v5;
            let a731 = 8u64;
            nat_add(a730, a731)?
        })?;
        fixed_u64::bit_xor(a732, a733)
    }, {
        let a737 = match v4.clone() {
            Adt3::C0(_, _, h734, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h734
            }
        };
        let a738 = f88(v0.clone(), v1, v2, v3, {
            let a735 = v5;
            let a736 = 16u64;
            nat_add(a735, a736)?
        })?;
        fixed_u64::bit_xor(a737, a738)
    }, {
        let a742 = match v4.clone() {
            Adt3::C0(_, _, _, h739, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h739
            }
        };
        let a743 = f88(v0.clone(), v1, v2, v3, {
            let a740 = v5;
            let a741 = 24u64;
            nat_add(a740, a741)?
        })?;
        fixed_u64::bit_xor(a742, a743)
    }, {
        let a747 = match v4.clone() {
            Adt3::C0(_, _, _, _, h744, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h744
            }
        };
        let a748 = f88(v0.clone(), v1, v2, v3, {
            let a745 = v5;
            let a746 = 32u64;
            nat_add(a745, a746)?
        })?;
        fixed_u64::bit_xor(a747, a748)
    }, {
        let a752 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, h749, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h749
            }
        };
        let a753 = f88(v0.clone(), v1, v2, v3, {
            let a750 = v5;
            let a751 = 40u64;
            nat_add(a750, a751)?
        })?;
        fixed_u64::bit_xor(a752, a753)
    }, {
        let a757 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, h754, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h754
            }
        };
        let a758 = f88(v0.clone(), v1, v2, v3, {
            let a755 = v5;
            let a756 = 48u64;
            nat_add(a755, a756)?
        })?;
        fixed_u64::bit_xor(a757, a758)
    }, {
        let a762 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h759, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h759
            }
        };
        let a763 = f88(v0.clone(), v1, v2, v3, {
            let a760 = v5;
            let a761 = 56u64;
            nat_add(a760, a761)?
        })?;
        fixed_u64::bit_xor(a762, a763)
    }, {
        let a767 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h764, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h764
            }
        };
        let a768 = f88(v0.clone(), v1, v2, v3, {
            let a765 = v5;
            let a766 = 64u64;
            nat_add(a765, a766)?
        })?;
        fixed_u64::bit_xor(a767, a768)
    }, {
        let a772 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h769, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h769
            }
        };
        let a773 = f88(v0.clone(), v1, v2, v3, {
            let a770 = v5;
            let a771 = 72u64;
            nat_add(a770, a771)?
        })?;
        fixed_u64::bit_xor(a772, a773)
    }, {
        let a777 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h774, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h774
            }
        };
        let a778 = f88(v0.clone(), v1, v2, v3, {
            let a775 = v5;
            let a776 = 80u64;
            nat_add(a775, a776)?
        })?;
        fixed_u64::bit_xor(a777, a778)
    }, {
        let a782 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h779, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h779
            }
        };
        let a783 = f88(v0.clone(), v1, v2, v3, {
            let a780 = v5;
            let a781 = 88u64;
            nat_add(a780, a781)?
        })?;
        fixed_u64::bit_xor(a782, a783)
    }, {
        let a787 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h784, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h784
            }
        };
        let a788 = f88(v0.clone(), v1, v2, v3, {
            let a785 = v5;
            let a786 = 96u64;
            nat_add(a785, a786)?
        })?;
        fixed_u64::bit_xor(a787, a788)
    }, {
        let a792 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h789, _, _, _, _, _, _, _, _, _, _, _) => {
                h789
            }
        };
        let a793 = f88(v0.clone(), v1, v2, v3, {
            let a790 = v5;
            let a791 = 104u64;
            nat_add(a790, a791)?
        })?;
        fixed_u64::bit_xor(a792, a793)
    }, {
        let a797 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h794, _, _, _, _, _, _, _, _, _, _) => {
                h794
            }
        };
        let a798 = f88(v0.clone(), v1, v2, v3, {
            let a795 = v5;
            let a796 = 112u64;
            nat_add(a795, a796)?
        })?;
        fixed_u64::bit_xor(a797, a798)
    }, {
        let a802 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h799, _, _, _, _, _, _, _, _, _) => {
                h799
            }
        };
        let a803 = f88(v0.clone(), v1, v2, v3, {
            let a800 = v5;
            let a801 = 120u64;
            nat_add(a800, a801)?
        })?;
        fixed_u64::bit_xor(a802, a803)
    }, {
        let a807 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h804, _, _, _, _, _, _, _, _) => {
                h804
            }
        };
        let a808 = f88(v0, v1, v2, v3, {
            let a805 = v5;
            let a806 = 128u64;
            nat_add(a805, a806)?
        })?;
        fixed_u64::bit_xor(a807, a808)
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h809, _, _, _, _, _, _, _) => {
            h809
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h810, _, _, _, _, _, _) => {
            h810
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h811, _, _, _, _, _) => {
            h811
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h812, _, _, _, _) => {
            h812
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h813, _, _, _) => {
            h813
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h814, _, _) => {
            h814
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h815, _) => {
            h815
        }
    }, match v4 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h816) => {
            h816
        }
    }))
}

pub fn f69(v0: Fn7, p820: u64, p821: Adt7) -> R<Adt7> {
    let m819 = (p820, p821);
    {
        let mut m819 = m819;
        loop {
            let step = {
                let (v1, v2) = m819;
                let m817 = v1;
                if m817 == 0 {
                    Err::<(u64, Adt7), R<Adt7>>(Ok(v2))
                } else {
                    let v3 = m817 - 1;
                    Ok::<(u64, Adt7), R<Adt7>>((v3, {
                        let c818: Fn7 = v0.clone();
                        c818.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m819 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Adt7) -> R<Adt7> {
    f89(v0)
}

pub fn f71(v0: u64, v1: u64) -> u64 {
    let v2: u64 = 4294967295u64;
    let v3: u64 = f90({
        let a822 = v0;
        let a823 = v2;
        fixed_u64::bit_and(a822, a823)
    }, {
        let a824 = v1;
        let a825 = v2;
        fixed_u64::bit_and(a824, a825)
    });
    let v4: u64 = f90(f90(f54(v0, 32u32), f54(v1, 32u32)), f54(v3, 32u32));
    let a830 = f91({
        let a826 = v4;
        let a827 = v2;
        fixed_u64::bit_and(a826, a827)
    }, 32u32);
    let a831 = {
        let a828 = v3;
        let a829 = v2;
        fixed_u64::bit_and(a828, a829)
    };
    fixed_u64::bit_or(a830, a831)
}

pub fn f72(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a834 = {
        let a832 = v3;
        let a833 = 16u64;
        nat_add(a832, a833)?
    };
    let a835 = v2;
    if nat_eq(a834, a835) {
        Ok(0u64)
    } else {
        let a838 = {
            let a836 = v3;
            let a837 = 8u64;
            nat_add(a836, a837)?
        };
        let a839 = v2;
        if nat_eq(a838, a839) {
            Ok(f92({
                let a840 = v1;
                let a841 = 8u64;
                nat_mul(a840, a841)?
            }))
        } else {
            Ok(f93(f94(v0.clone(), v1, {
                let a842 = v3;
                let a843 = 0u64;
                nat_add(a842, a843)?
            }), f94(v0.clone(), v1, {
                let a844 = v3;
                let a845 = 1u64;
                nat_add(a844, a845)?
            }), f94(v0.clone(), v1, {
                let a846 = v3;
                let a847 = 2u64;
                nat_add(a846, a847)?
            }), f94(v0.clone(), v1, {
                let a848 = v3;
                let a849 = 3u64;
                nat_add(a848, a849)?
            }), f94(v0.clone(), v1, {
                let a850 = v3;
                let a851 = 4u64;
                nat_add(a850, a851)?
            }), f94(v0.clone(), v1, {
                let a852 = v3;
                let a853 = 5u64;
                nat_add(a852, a853)?
            }), f94(v0.clone(), v1, {
                let a854 = v3;
                let a855 = 6u64;
                nat_add(a854, a855)?
            }), f94(v0, v1, {
                let a856 = v3;
                let a857 = 7u64;
                nat_add(a856, a857)?
            })))
        }
    }
}

pub fn f73(v0: (u64, List<Adt1>), v1: Adt1) -> R<(u64, List<Adt1>)> {
    let v2: Adt10 = f95(Fn11::F96, 40u64, Adt10::C0(v1, {
        let a859 = {
            let (h858, _) = v0.clone();
            h858
        };
        let a860 = 1u64;
        nat_add(a859, a860)?
    }, {
        let (_, h861) = v0;
        h861
    }));
    Ok(({
        let a863 = {
            let (h862, _) = v0;
            h862
        };
        let a864 = 1u64;
        nat_add(a863, a864)?
    }, List::cons(match v2.clone() {
        Adt10::C0(h865, _, _) => {
            h865
        }
    }, match v2 {
        Adt10::C0(_, _, h866) => {
            h866
        }
    })))
}

pub fn f74(v0: Fn8, p870: u64, p871: (Adt9, Adt8)) -> (Adt9, Adt8) {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v1, v2) = m869;
                let m867 = v1;
                if m867 == 0 {
                    Err::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>(v2)
                } else {
                    let v3 = m867 - 1;
                    Ok::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>((v3, {
                        let c868: Fn8 = v0.clone();
                        c868.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    f97(v0)
}

pub fn f76(v0: Fn9, p875: u64, p876: (u64, Adt1)) -> R<(u64, Adt1)> {
    let m874 = (p875, p876);
    {
        let mut m874 = m874;
        loop {
            let step = {
                let (v1, v2) = m874;
                let m872 = v1;
                if m872 == 0 {
                    Err::<(u64, (u64, Adt1)), R<(u64, Adt1)>>(Ok(v2))
                } else {
                    let v3 = m872 - 1;
                    Ok::<(u64, (u64, Adt1)), R<(u64, Adt1)>>((v3, {
                        let c873: Fn9 = v0.clone();
                        c873.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m874 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt1)) -> R<(u64, Adt1)> {
    Ok(({
        let a878 = {
            let (h877, _) = v3.clone();
            h877
        };
        let a879 = 1u64;
        nat_add(a878, a879)?
    }, f59({
        let (_, h880) = v3.clone();
        h880
    }, f61(v1, {
        let a884 = v2;
        let a885 = {
            let a882 = 64u64;
            let a883 = {
                let (h881, _) = v3.clone();
                h881
            };
            nat_mul(a882, a883)?
        };
        nat_add(a884, a885)?
    })?, v0, 64u32, {
        let a887 = {
            let (h886, _) = v3;
            h886
        };
        let a888 = 0u64;
        if nat_eq(a887, a888) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f78() -> Adt1 {
    Adt1::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f79(v0: Bytes, v1: u64) -> R<u32> {
    let a899 = f98(v0.clone(), v1);
    let a900 = {
        let a897 = f99(f98(v0.clone(), {
            let a889 = v1;
            let a890 = 1u64;
            nat_add(a889, a890)?
        }), 8u32);
        let a898 = {
            let a895 = f99(f98(v0.clone(), {
                let a891 = v1;
                let a892 = 2u64;
                nat_add(a891, a892)?
            }), 16u32);
            let a896 = f99(f98(v0, {
                let a893 = v1;
                let a894 = 3u64;
                nat_add(a893, a894)?
            }), 24u32);
            fixed_u32::bit_or(a895, a896)
        };
        fixed_u32::bit_or(a897, a898)
    };
    Ok(fixed_u32::bit_or(a899, a900))
}

pub fn f80(v0: Fn10, p904: u64, p905: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m903 = (p904, p905);
    {
        let mut m903 = m903;
        loop {
            let step = {
                let (v1, v2) = m903;
                let m901 = v1;
                if m901 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m901 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c902: Fn10 = v0.clone();
                        c902.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h906, _) = v0;
        h906
    };
    let v2: u32 = {
        let (h907, _) = {
            let (_, h908) = v0;
            h908
        };
        h907
    };
    let v3: u32 = {
        let (_, h909) = {
            let (_, h910) = v0;
            h910
        };
        h909
    };
    ({
        let a911 = v1;
        let a912 = 2u64;
        let a913 = 0u64;
        nat_quot(a911, a912, a913)
    }, (f99(v2, 1u32), {
        let a917 = {
            let a914 = v1;
            let a915 = 2u64;
            let a916 = 0u64;
            nat_rem(a914, a915, a916)
        };
        let a918 = 1u64;
        if nat_eq(a917, a918) {
            let a919 = v3;
            let a920 = v2;
            fixed_u32::bit_or(a919, a920)
        } else {
            v3
        }
    }))
}

pub fn f82(v0: Adt1, v1: Adt1, v2: bool) -> Adt1 {
    f59(f78(), f100(v0, v1), 0u64, 64u32, {
        let a921 = 4u32;
        let a922 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a921, a922)
    })
}

pub fn f83(v0: Adt5) -> R<Adt5> {
    let v1: u32 = f101(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, _, h923) => {
            h923
        }
    });
    let v2: u32 = f66(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h924, _, _) => {
            h924
        }
    }, f66(f102(match v0.clone() {
        Adt5::C0(_, _, _, _, h925, _, _, _, _, _) => {
            h925
        }
    }), f66(f103(match v0.clone() {
        Adt5::C0(_, _, _, _, h926, _, _, _, _, _) => {
            h926
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h927, _, _, _, _) => {
            h927
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h928, _, _, _) => {
            h928
        }
    }), f66(v1, {
        let m931 = match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, _, _, h930, _) => {
                h930
            }
        };
        match m931 {
            Adt4::C0(h929, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h929
            }
        }
    }))));
    let v3: u32 = f66(f104(match v0.clone() {
        Adt5::C0(h932, _, _, _, _, _, _, _, _, _) => {
            h932
        }
    }), f105(match v0.clone() {
        Adt5::C0(h933, _, _, _, _, _, _, _, _, _) => {
            h933
        }
    }, match v0.clone() {
        Adt5::C0(_, h934, _, _, _, _, _, _, _, _) => {
            h934
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h935, _, _, _, _, _, _, _) => {
            h935
        }
    }));
    Ok(Adt5::C0(f66(v2, v3), match v0.clone() {
        Adt5::C0(h936, _, _, _, _, _, _, _, _, _) => {
            h936
        }
    }, match v0.clone() {
        Adt5::C0(_, h937, _, _, _, _, _, _, _, _) => {
            h937
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h938, _, _, _, _, _, _, _) => {
            h938
        }
    }, f66(match v0.clone() {
        Adt5::C0(_, _, _, h939, _, _, _, _, _, _) => {
            h939
        }
    }, v2), match v0.clone() {
        Adt5::C0(_, _, _, _, h940, _, _, _, _, _) => {
            h940
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h941, _, _, _, _) => {
            h941
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h942, _, _, _) => {
            h942
        }
    }, f106(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, h943, _) => {
            h943
        }
    }), {
        let a945 = match v0 {
            Adt5::C0(_, _, _, _, _, _, _, _, _, h944) => {
                h944
            }
        };
        let a946 = 1u64;
        nat_add(a945, a946)?
    }))
}

pub fn f84(v0: u32) -> u64 {
    let m948 = {
        let a947 = v0;
        fixed_u64::convert(i128::from(a947))
    };
    match m948 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f85(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a953 = f99(v0, 24u32);
    let a954 = {
        let a951 = f99(v1, 16u32);
        let a952 = {
            let a949 = f99(v2, 8u32);
            let a950 = v3;
            fixed_u32::bit_or(a949, a950)
        };
        fixed_u32::bit_or(a951, a952)
    };
    fixed_u32::bit_or(a953, a954)
}

pub fn f86(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a955 = v2;
    let a956 = v1;
    if nat_lt(a955, a956) {
        let m959 = {
            let a957 = v0;
            let a958 = v2;
            index_bytes(a957, a958)
        };
        match m959 {
            None => {
                0u32
            }
            Some(v3) => {
                let m961 = {
                    let a960 = v3;
                    fixed_u32::convert(i128::from(a960))
                };
                match m961 {
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
        let a962 = v2;
        let a963 = v1;
        if nat_eq(a962, a963) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f87(v0: Adt3) -> R<Adt3> {
    let (_, h964) = f48(Fn4::F107, 24u64, (0u64, v0))?;
    Ok(h964)
}

pub fn f88(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    Ok(f108(f109(v0.clone(), v1, v2, v3, {
        let a965 = v4;
        let a966 = 0u64;
        nat_add(a965, a966)?
    })?, f109(v0.clone(), v1, v2, v3, {
        let a967 = v4;
        let a968 = 1u64;
        nat_add(a967, a968)?
    })?, f109(v0.clone(), v1, v2, v3, {
        let a969 = v4;
        let a970 = 2u64;
        nat_add(a969, a970)?
    })?, f109(v0.clone(), v1, v2, v3, {
        let a971 = v4;
        let a972 = 3u64;
        nat_add(a971, a972)?
    })?, f109(v0.clone(), v1, v2, v3, {
        let a973 = v4;
        let a974 = 4u64;
        nat_add(a973, a974)?
    })?, f109(v0.clone(), v1, v2, v3, {
        let a975 = v4;
        let a976 = 5u64;
        nat_add(a975, a976)?
    })?, f109(v0.clone(), v1, v2, v3, {
        let a977 = v4;
        let a978 = 6u64;
        nat_add(a977, a978)?
    })?, f109(v0, v1, v2, v3, {
        let a979 = v4;
        let a980 = 7u64;
        nat_add(a979, a980)?
    })?))
}

pub fn f89(v0: Adt7) -> R<Adt7> {
    let v1: u64 = f110(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h981) => {
            h981
        }
    });
    let v2: u64 = f71(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h982, _, _) => {
            h982
        }
    }, f71(f111(match v0.clone() {
        Adt7::C0(_, _, _, _, h983, _, _, _, _, _) => {
            h983
        }
    }), f71(f112(match v0.clone() {
        Adt7::C0(_, _, _, _, h984, _, _, _, _, _) => {
            h984
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h985, _, _, _, _) => {
            h985
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h986, _, _, _) => {
            h986
        }
    }), f71(v1, {
        let m989 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h988, _) => {
                h988
            }
        };
        match m989 {
            Adt6::C0(h987, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h987
            }
        }
    }))));
    let v3: u64 = f71(f113(match v0.clone() {
        Adt7::C0(h990, _, _, _, _, _, _, _, _, _) => {
            h990
        }
    }), f114(match v0.clone() {
        Adt7::C0(h991, _, _, _, _, _, _, _, _, _) => {
            h991
        }
    }, match v0.clone() {
        Adt7::C0(_, h992, _, _, _, _, _, _, _, _) => {
            h992
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h993, _, _, _, _, _, _, _) => {
            h993
        }
    }));
    Ok(Adt7::C0(f71(v2, v3), match v0.clone() {
        Adt7::C0(h994, _, _, _, _, _, _, _, _, _) => {
            h994
        }
    }, match v0.clone() {
        Adt7::C0(_, h995, _, _, _, _, _, _, _, _) => {
            h995
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h996, _, _, _, _, _, _, _) => {
            h996
        }
    }, f71(match v0.clone() {
        Adt7::C0(_, _, _, h997, _, _, _, _, _, _) => {
            h997
        }
    }, v2), match v0.clone() {
        Adt7::C0(_, _, _, _, h998, _, _, _, _, _) => {
            h998
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h999, _, _, _, _) => {
            h999
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h1000, _, _, _) => {
            h1000
        }
    }, f115(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h1001, _) => {
            h1001
        }
    }), {
        let a1003 = match v0 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h1002) => {
                h1002
            }
        };
        let a1004 = 1u64;
        nat_add(a1003, a1004)?
    }))
}

pub fn f90(v0: u64, v1: u64) -> u64 {
    let m1007 = {
        let a1005 = v0;
        let a1006 = v1;
        fixed_u64::checked_add(a1005, a1006)
    };
    match m1007 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f91(v0: u64, v1: u32) -> u64 {
    let m1010 = {
        let a1008 = v0;
        let a1009 = v1;
        fixed_u64::shift_left(a1008, a1009)
    };
    match m1010 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: u64) -> u64 {
    let (_, h1011) = {
        let (_, h1012) = f116(Fn12::F117, 64u64, (v0, (1u64, 0u64)));
        h1012
    };
    h1011
}

pub fn f93(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1025 = f91(v0, 56u32);
    let a1026 = {
        let a1023 = f91(v1, 48u32);
        let a1024 = {
            let a1021 = f91(v2, 40u32);
            let a1022 = {
                let a1019 = f91(v3, 32u32);
                let a1020 = {
                    let a1017 = f91(v4, 24u32);
                    let a1018 = {
                        let a1015 = f91(v5, 16u32);
                        let a1016 = {
                            let a1013 = f91(v6, 8u32);
                            let a1014 = v7;
                            fixed_u64::bit_or(a1013, a1014)
                        };
                        fixed_u64::bit_or(a1015, a1016)
                    };
                    fixed_u64::bit_or(a1017, a1018)
                };
                fixed_u64::bit_or(a1019, a1020)
            };
            fixed_u64::bit_or(a1021, a1022)
        };
        fixed_u64::bit_or(a1023, a1024)
    };
    fixed_u64::bit_or(a1025, a1026)
}

pub fn f94(v0: Bytes, v1: u64, v2: u64) -> u64 {
    let a1027 = v2;
    let a1028 = v1;
    if nat_lt(a1027, a1028) {
        f118(v0, v2)
    } else {
        let a1029 = v2;
        let a1030 = v1;
        if nat_eq(a1029, a1030) {
            128u64
        } else {
            0u64
        }
    }
}

pub fn f95(v0: Fn11, p1034: u64, p1035: Adt10) -> Adt10 {
    let m1033 = (p1034, p1035);
    {
        let mut m1033 = m1033;
        loop {
            let step = {
                let (v1, v2) = m1033;
                let m1031 = v1;
                if m1031 == 0 {
                    Err::<(u64, Adt10), Adt10>(v2)
                } else {
                    let v3 = m1031 - 1;
                    Ok::<(u64, Adt10), Adt10>((v3, {
                        let c1032: Fn11 = v0.clone();
                        c1032.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1033 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: Adt10) -> Adt10 {
    f119(v0)
}

pub fn f97(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    (f120(f121({
        let (h1036, _) = v0.clone();
        h1036
    }, {
        let (_, h1037) = v0.clone();
        h1037
    }), {
        let (_, h1038) = v0.clone();
        h1038
    }), f122({
        let (_, h1039) = v0;
        h1039
    }))
}

pub fn f98(v0: Bytes, v1: u64) -> u32 {
    let m1042 = {
        let a1040 = v0;
        let a1041 = v1;
        index_bytes(a1040, a1041)
    };
    match m1042 {
        None => {
            0u32
        }
        Some(v2) => {
            let m1044 = {
                let a1043 = v2;
                fixed_u32::convert(i128::from(a1043))
            };
            match m1044 {
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

pub fn f99(v0: u32, v1: u32) -> u32 {
    let m1047 = {
        let a1045 = v0;
        let a1046 = v1;
        fixed_u32::shift_left(a1045, a1046)
    };
    match m1047 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f100(v0: Adt1, v1: Adt1) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt1::C0(h1048, _, _, _, _, _, _, _) => {
            h1048
        }
    }, match v0.clone() {
        Adt1::C0(_, h1049, _, _, _, _, _, _) => {
            h1049
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1050, _, _, _, _, _) => {
            h1050
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1051, _, _, _, _) => {
            h1051
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1052, _, _, _) => {
            h1052
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1053, _, _) => {
            h1053
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h1054, _) => {
            h1054
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h1055) => {
            h1055
        }
    }, match v1.clone() {
        Adt1::C0(h1056, _, _, _, _, _, _, _) => {
            h1056
        }
    }, match v1.clone() {
        Adt1::C0(_, h1057, _, _, _, _, _, _) => {
            h1057
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h1058, _, _, _, _, _) => {
            h1058
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h1059, _, _, _, _) => {
            h1059
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h1060, _, _, _) => {
            h1060
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1061, _, _) => {
            h1061
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1062, _) => {
            h1062
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h1063) => {
            h1063
        }
    })
}

pub fn f101(v0: u64) -> u32 {
    let a1064 = v0;
    let a1065 = 32u64;
    if nat_lt(a1064, a1065) {
        let a1066 = v0;
        let a1067 = 16u64;
        if nat_lt(a1066, a1067) {
            let a1068 = v0;
            let a1069 = 8u64;
            if nat_lt(a1068, a1069) {
                let a1070 = v0;
                let a1071 = 4u64;
                if nat_lt(a1070, a1071) {
                    let a1072 = v0;
                    let a1073 = 2u64;
                    if nat_lt(a1072, a1073) {
                        let a1074 = v0;
                        let a1075 = 1u64;
                        if nat_lt(a1074, a1075) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1076 = v0;
                        let a1077 = 3u64;
                        if nat_lt(a1076, a1077) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1078 = v0;
                    let a1079 = 6u64;
                    if nat_lt(a1078, a1079) {
                        let a1080 = v0;
                        let a1081 = 5u64;
                        if nat_lt(a1080, a1081) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1082 = v0;
                        let a1083 = 7u64;
                        if nat_lt(a1082, a1083) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1084 = v0;
                let a1085 = 12u64;
                if nat_lt(a1084, a1085) {
                    let a1086 = v0;
                    let a1087 = 10u64;
                    if nat_lt(a1086, a1087) {
                        let a1088 = v0;
                        let a1089 = 9u64;
                        if nat_lt(a1088, a1089) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1090 = v0;
                        let a1091 = 11u64;
                        if nat_lt(a1090, a1091) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1092 = v0;
                    let a1093 = 14u64;
                    if nat_lt(a1092, a1093) {
                        let a1094 = v0;
                        let a1095 = 13u64;
                        if nat_lt(a1094, a1095) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1096 = v0;
                        let a1097 = 15u64;
                        if nat_lt(a1096, a1097) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1098 = v0;
            let a1099 = 24u64;
            if nat_lt(a1098, a1099) {
                let a1100 = v0;
                let a1101 = 20u64;
                if nat_lt(a1100, a1101) {
                    let a1102 = v0;
                    let a1103 = 18u64;
                    if nat_lt(a1102, a1103) {
                        let a1104 = v0;
                        let a1105 = 17u64;
                        if nat_lt(a1104, a1105) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1106 = v0;
                        let a1107 = 19u64;
                        if nat_lt(a1106, a1107) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1108 = v0;
                    let a1109 = 22u64;
                    if nat_lt(a1108, a1109) {
                        let a1110 = v0;
                        let a1111 = 21u64;
                        if nat_lt(a1110, a1111) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1112 = v0;
                        let a1113 = 23u64;
                        if nat_lt(a1112, a1113) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1114 = v0;
                let a1115 = 28u64;
                if nat_lt(a1114, a1115) {
                    let a1116 = v0;
                    let a1117 = 26u64;
                    if nat_lt(a1116, a1117) {
                        let a1118 = v0;
                        let a1119 = 25u64;
                        if nat_lt(a1118, a1119) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1120 = v0;
                        let a1121 = 27u64;
                        if nat_lt(a1120, a1121) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1122 = v0;
                    let a1123 = 30u64;
                    if nat_lt(a1122, a1123) {
                        let a1124 = v0;
                        let a1125 = 29u64;
                        if nat_lt(a1124, a1125) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1126 = v0;
                        let a1127 = 31u64;
                        if nat_lt(a1126, a1127) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1128 = v0;
        let a1129 = 48u64;
        if nat_lt(a1128, a1129) {
            let a1130 = v0;
            let a1131 = 40u64;
            if nat_lt(a1130, a1131) {
                let a1132 = v0;
                let a1133 = 36u64;
                if nat_lt(a1132, a1133) {
                    let a1134 = v0;
                    let a1135 = 34u64;
                    if nat_lt(a1134, a1135) {
                        let a1136 = v0;
                        let a1137 = 33u64;
                        if nat_lt(a1136, a1137) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1138 = v0;
                        let a1139 = 35u64;
                        if nat_lt(a1138, a1139) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1140 = v0;
                    let a1141 = 38u64;
                    if nat_lt(a1140, a1141) {
                        let a1142 = v0;
                        let a1143 = 37u64;
                        if nat_lt(a1142, a1143) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1144 = v0;
                        let a1145 = 39u64;
                        if nat_lt(a1144, a1145) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1146 = v0;
                let a1147 = 44u64;
                if nat_lt(a1146, a1147) {
                    let a1148 = v0;
                    let a1149 = 42u64;
                    if nat_lt(a1148, a1149) {
                        let a1150 = v0;
                        let a1151 = 41u64;
                        if nat_lt(a1150, a1151) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1152 = v0;
                        let a1153 = 43u64;
                        if nat_lt(a1152, a1153) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1154 = v0;
                    let a1155 = 46u64;
                    if nat_lt(a1154, a1155) {
                        let a1156 = v0;
                        let a1157 = 45u64;
                        if nat_lt(a1156, a1157) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1158 = v0;
                        let a1159 = 47u64;
                        if nat_lt(a1158, a1159) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1160 = v0;
            let a1161 = 56u64;
            if nat_lt(a1160, a1161) {
                let a1162 = v0;
                let a1163 = 52u64;
                if nat_lt(a1162, a1163) {
                    let a1164 = v0;
                    let a1165 = 50u64;
                    if nat_lt(a1164, a1165) {
                        let a1166 = v0;
                        let a1167 = 49u64;
                        if nat_lt(a1166, a1167) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1168 = v0;
                        let a1169 = 51u64;
                        if nat_lt(a1168, a1169) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1170 = v0;
                    let a1171 = 54u64;
                    if nat_lt(a1170, a1171) {
                        let a1172 = v0;
                        let a1173 = 53u64;
                        if nat_lt(a1172, a1173) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1174 = v0;
                        let a1175 = 55u64;
                        if nat_lt(a1174, a1175) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1176 = v0;
                let a1177 = 60u64;
                if nat_lt(a1176, a1177) {
                    let a1178 = v0;
                    let a1179 = 58u64;
                    if nat_lt(a1178, a1179) {
                        let a1180 = v0;
                        let a1181 = 57u64;
                        if nat_lt(a1180, a1181) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1182 = v0;
                        let a1183 = 59u64;
                        if nat_lt(a1182, a1183) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1184 = v0;
                    let a1185 = 62u64;
                    if nat_lt(a1184, a1185) {
                        let a1186 = v0;
                        let a1187 = 61u64;
                        if nat_lt(a1186, a1187) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1188 = v0;
                        let a1189 = 63u64;
                        if nat_lt(a1188, a1189) {
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

pub fn f102(v0: u32) -> u32 {
    let a1192 = {
        let a1190 = f123(v0, 6u32, 26u32);
        let a1191 = f123(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1190, a1191)
    };
    let a1193 = f123(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1192, a1193)
}

pub fn f103(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1199 = {
        let a1194 = v0;
        let a1195 = v1;
        fixed_u32::bit_and(a1194, a1195)
    };
    let a1200 = {
        let a1197 = {
            let a1196 = v0;
            fixed_u32::bit_not(a1196)
        };
        let a1198 = v2;
        fixed_u32::bit_and(a1197, a1198)
    };
    fixed_u32::bit_xor(a1199, a1200)
}

pub fn f104(v0: u32) -> u32 {
    let a1203 = {
        let a1201 = f123(v0, 2u32, 30u32);
        let a1202 = f123(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1201, a1202)
    };
    let a1204 = f123(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1203, a1204)
}

pub fn f105(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1213 = {
        let a1209 = {
            let a1205 = v0;
            let a1206 = v1;
            fixed_u32::bit_and(a1205, a1206)
        };
        let a1210 = {
            let a1207 = v0;
            let a1208 = v2;
            fixed_u32::bit_and(a1207, a1208)
        };
        fixed_u32::bit_xor(a1209, a1210)
    };
    let a1214 = {
        let a1211 = v1;
        let a1212 = v2;
        fixed_u32::bit_and(a1211, a1212)
    };
    fixed_u32::bit_xor(a1213, a1214)
}

pub fn f106(v0: Adt4) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(_, h1215, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1215
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h1216, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1216
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h1217, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1217
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h1218, _, _, _, _, _, _, _, _, _, _, _) => {
            h1218
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h1219, _, _, _, _, _, _, _, _, _, _) => {
            h1219
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h1220, _, _, _, _, _, _, _, _, _) => {
            h1220
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h1221, _, _, _, _, _, _, _, _) => {
            h1221
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h1222, _, _, _, _, _, _, _) => {
            h1222
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1223, _, _, _, _, _, _) => {
            h1223
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h1224, _, _, _, _, _) => {
            h1224
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, h1225, _, _, _, _) => {
            h1225
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1226, _, _, _) => {
            h1226
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1227, _, _) => {
            h1227
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1228, _) => {
            h1228
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1229) => {
            h1229
        }
    }, f66(f124(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1230, _) => {
            h1230
        }
    }), f66(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1231, _, _, _, _, _, _) => {
            h1231
        }
    }, f66(f125(match v0.clone() {
        Adt4::C0(_, h1232, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1232
        }
    }), match v0 {
        Adt4::C0(h1233, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1233
        }
    }))))
}

pub fn f107(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    f126(v0)
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1246 = f91(v7, 56u32);
    let a1247 = {
        let a1244 = f91(v6, 48u32);
        let a1245 = {
            let a1242 = f91(v5, 40u32);
            let a1243 = {
                let a1240 = f91(v4, 32u32);
                let a1241 = {
                    let a1238 = f91(v3, 24u32);
                    let a1239 = {
                        let a1236 = f91(v2, 16u32);
                        let a1237 = {
                            let a1234 = f91(v1, 8u32);
                            let a1235 = v0;
                            fixed_u64::bit_or(a1234, a1235)
                        };
                        fixed_u64::bit_or(a1236, a1237)
                    };
                    fixed_u64::bit_or(a1238, a1239)
                };
                fixed_u64::bit_or(a1240, a1241)
            };
            fixed_u64::bit_or(a1242, a1243)
        };
        fixed_u64::bit_or(a1244, a1245)
    };
    fixed_u64::bit_or(a1246, a1247)
}

pub fn f109(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let v5: u64 = {
        let a1248 = v4;
        let a1249 = v1;
        if nat_lt(a1248, a1249) {
            f118(v0, v4)
        } else {
            let a1250 = v4;
            let a1251 = v1;
            if nat_eq(a1250, a1251) {
                v3
            } else {
                0u64
            }
        }
    };
    let a1254 = {
        let a1252 = v4;
        let a1253 = 1u64;
        nat_add(a1252, a1253)?
    };
    let a1255 = v2;
    if nat_eq(a1254, a1255) {
        let a1256 = v5;
        let a1257 = 128u64;
        Ok(fixed_u64::bit_or(a1256, a1257))
    } else {
        Ok(v5)
    }
}

pub fn f110(v0: u64) -> u64 {
    let a1258 = v0;
    let a1259 = 40u64;
    if nat_lt(a1258, a1259) {
        let a1260 = v0;
        let a1261 = 20u64;
        if nat_lt(a1260, a1261) {
            let a1262 = v0;
            let a1263 = 10u64;
            if nat_lt(a1262, a1263) {
                let a1264 = v0;
                let a1265 = 5u64;
                if nat_lt(a1264, a1265) {
                    let a1266 = v0;
                    let a1267 = 2u64;
                    if nat_lt(a1266, a1267) {
                        let a1268 = v0;
                        let a1269 = 1u64;
                        if nat_lt(a1268, a1269) {
                            4794697086780616226u64
                        } else {
                            8158064640168781261u64
                        }
                    } else {
                        let a1270 = v0;
                        let a1271 = 3u64;
                        if nat_lt(a1270, a1271) {
                            13096744586834688815u64
                        } else {
                            let a1272 = v0;
                            let a1273 = 4u64;
                            if nat_lt(a1272, a1273) {
                                16840607885511220156u64
                            } else {
                                4131703408338449720u64
                            }
                        }
                    }
                } else {
                    let a1274 = v0;
                    let a1275 = 7u64;
                    if nat_lt(a1274, a1275) {
                        let a1276 = v0;
                        let a1277 = 6u64;
                        if nat_lt(a1276, a1277) {
                            6480981068601479193u64
                        } else {
                            10538285296894168987u64
                        }
                    } else {
                        let a1278 = v0;
                        let a1279 = 8u64;
                        if nat_lt(a1278, a1279) {
                            12329834152419229976u64
                        } else {
                            let a1280 = v0;
                            let a1281 = 9u64;
                            if nat_lt(a1280, a1281) {
                                15566598209576043074u64
                            } else {
                                1334009975649890238u64
                            }
                        }
                    }
                }
            } else {
                let a1282 = v0;
                let a1283 = 15u64;
                if nat_lt(a1282, a1283) {
                    let a1284 = v0;
                    let a1285 = 12u64;
                    if nat_lt(a1284, a1285) {
                        let a1286 = v0;
                        let a1287 = 11u64;
                        if nat_lt(a1286, a1287) {
                            2608012711638119052u64
                        } else {
                            6128411473006802146u64
                        }
                    } else {
                        let a1288 = v0;
                        let a1289 = 13u64;
                        if nat_lt(a1288, a1289) {
                            8268148722764581231u64
                        } else {
                            let a1290 = v0;
                            let a1291 = 14u64;
                            if nat_lt(a1290, a1291) {
                                9286055187155687089u64
                            } else {
                                11230858885718282805u64
                            }
                        }
                    }
                } else {
                    let a1292 = v0;
                    let a1293 = 17u64;
                    if nat_lt(a1292, a1293) {
                        let a1294 = v0;
                        let a1295 = 16u64;
                        if nat_lt(a1294, a1295) {
                            13951009754708518548u64
                        } else {
                            16472876342353939154u64
                        }
                    } else {
                        let a1296 = v0;
                        let a1297 = 18u64;
                        if nat_lt(a1296, a1297) {
                            17275323862435702243u64
                        } else {
                            let a1298 = v0;
                            let a1299 = 19u64;
                            if nat_lt(a1298, a1299) {
                                1135362057144423861u64
                            } else {
                                2597628984639134821u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1300 = v0;
            let a1301 = 30u64;
            if nat_lt(a1300, a1301) {
                let a1302 = v0;
                let a1303 = 25u64;
                if nat_lt(a1302, a1303) {
                    let a1304 = v0;
                    let a1305 = 22u64;
                    if nat_lt(a1304, a1305) {
                        let a1306 = v0;
                        let a1307 = 21u64;
                        if nat_lt(a1306, a1307) {
                            3308224258029322869u64
                        } else {
                            5365058923640841347u64
                        }
                    } else {
                        let a1308 = v0;
                        let a1309 = 23u64;
                        if nat_lt(a1308, a1309) {
                            6679025012923562964u64
                        } else {
                            let a1310 = v0;
                            let a1311 = 24u64;
                            if nat_lt(a1310, a1311) {
                                8573033837759648693u64
                            } else {
                                10970295158949994411u64
                            }
                        }
                    }
                } else {
                    let a1312 = v0;
                    let a1313 = 27u64;
                    if nat_lt(a1312, a1313) {
                        let a1314 = v0;
                        let a1315 = 26u64;
                        if nat_lt(a1314, a1315) {
                            12119686244451234320u64
                        } else {
                            12683024718118986047u64
                        }
                    } else {
                        let a1316 = v0;
                        let a1317 = 28u64;
                        if nat_lt(a1316, a1317) {
                            13788192230050041572u64
                        } else {
                            let a1318 = v0;
                            let a1319 = 29u64;
                            if nat_lt(a1318, a1319) {
                                14330467153632333762u64
                            } else {
                                15395433587784984357u64
                            }
                        }
                    }
                }
            } else {
                let a1320 = v0;
                let a1321 = 35u64;
                if nat_lt(a1320, a1321) {
                    let a1322 = v0;
                    let a1323 = 32u64;
                    if nat_lt(a1322, a1323) {
                        let a1324 = v0;
                        let a1325 = 31u64;
                        if nat_lt(a1324, a1325) {
                            489312712824947311u64
                        } else {
                            1452737877330783856u64
                        }
                    } else {
                        let a1326 = v0;
                        let a1327 = 33u64;
                        if nat_lt(a1326, a1327) {
                            2861767655752347644u64
                        } else {
                            let a1328 = v0;
                            let a1329 = 34u64;
                            if nat_lt(a1328, a1329) {
                                3322285676063803686u64
                            } else {
                                5560940570517711597u64
                            }
                        }
                    }
                } else {
                    let a1330 = v0;
                    let a1331 = 37u64;
                    if nat_lt(a1330, a1331) {
                        let a1332 = v0;
                        let a1333 = 36u64;
                        if nat_lt(a1332, a1333) {
                            5996557281743188959u64
                        } else {
                            7280758554555802590u64
                        }
                    } else {
                        let a1334 = v0;
                        let a1335 = 38u64;
                        if nat_lt(a1334, a1335) {
                            8532644243296465576u64
                        } else {
                            let a1336 = v0;
                            let a1337 = 39u64;
                            if nat_lt(a1336, a1337) {
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
        let a1338 = v0;
        let a1339 = 60u64;
        if nat_lt(a1338, a1339) {
            let a1340 = v0;
            let a1341 = 50u64;
            if nat_lt(a1340, a1341) {
                let a1342 = v0;
                let a1343 = 45u64;
                if nat_lt(a1342, a1343) {
                    let a1344 = v0;
                    let a1345 = 42u64;
                    if nat_lt(a1344, a1345) {
                        let a1346 = v0;
                        let a1347 = 41u64;
                        if nat_lt(a1346, a1347) {
                            11727347734174303076u64
                        } else {
                            12113106623233404929u64
                        }
                    } else {
                        let a1348 = v0;
                        let a1349 = 43u64;
                        if nat_lt(a1348, a1349) {
                            14000437183269869457u64
                        } else {
                            let a1350 = v0;
                            let a1351 = 44u64;
                            if nat_lt(a1350, a1351) {
                                14369950271660146224u64
                            } else {
                                15101387698204529176u64
                            }
                        }
                    }
                } else {
                    let a1352 = v0;
                    let a1353 = 47u64;
                    if nat_lt(a1352, a1353) {
                        let a1354 = v0;
                        let a1355 = 46u64;
                        if nat_lt(a1354, a1355) {
                            15463397548674623760u64
                        } else {
                            17586052441742319658u64
                        }
                    } else {
                        let a1356 = v0;
                        let a1357 = 48u64;
                        if nat_lt(a1356, a1357) {
                            1182934255886127544u64
                        } else {
                            let a1358 = v0;
                            let a1359 = 49u64;
                            if nat_lt(a1358, a1359) {
                                1847814050463011016u64
                            } else {
                                2177327727835720531u64
                            }
                        }
                    }
                }
            } else {
                let a1360 = v0;
                let a1361 = 55u64;
                if nat_lt(a1360, a1361) {
                    let a1362 = v0;
                    let a1363 = 52u64;
                    if nat_lt(a1362, a1363) {
                        let a1364 = v0;
                        let a1365 = 51u64;
                        if nat_lt(a1364, a1365) {
                            2830643537854262169u64
                        } else {
                            3796741975233480872u64
                        }
                    } else {
                        let a1366 = v0;
                        let a1367 = 53u64;
                        if nat_lt(a1366, a1367) {
                            4115178125766777443u64
                        } else {
                            let a1368 = v0;
                            let a1369 = 54u64;
                            if nat_lt(a1368, a1369) {
                                5681478168544905931u64
                            } else {
                                6601373596472566643u64
                            }
                        }
                    }
                } else {
                    let a1370 = v0;
                    let a1371 = 57u64;
                    if nat_lt(a1370, a1371) {
                        let a1372 = v0;
                        let a1373 = 56u64;
                        if nat_lt(a1372, a1373) {
                            7507060721942968483u64
                        } else {
                            8399075790359081724u64
                        }
                    } else {
                        let a1374 = v0;
                        let a1375 = 58u64;
                        if nat_lt(a1374, a1375) {
                            8693463985226723168u64
                        } else {
                            let a1376 = v0;
                            let a1377 = 59u64;
                            if nat_lt(a1376, a1377) {
                                9568029438360202098u64
                            } else {
                                10144078919501101548u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1378 = v0;
            let a1379 = 70u64;
            if nat_lt(a1378, a1379) {
                let a1380 = v0;
                let a1381 = 65u64;
                if nat_lt(a1380, a1381) {
                    let a1382 = v0;
                    let a1383 = 62u64;
                    if nat_lt(a1382, a1383) {
                        let a1384 = v0;
                        let a1385 = 61u64;
                        if nat_lt(a1384, a1385) {
                            10430055236837252648u64
                        } else {
                            11840083180663258601u64
                        }
                    } else {
                        let a1386 = v0;
                        let a1387 = 63u64;
                        if nat_lt(a1386, a1387) {
                            13761210420658862357u64
                        } else {
                            let a1388 = v0;
                            let a1389 = 64u64;
                            if nat_lt(a1388, a1389) {
                                14299343276471374635u64
                            } else {
                                14566680578165727644u64
                            }
                        }
                    }
                } else {
                    let a1390 = v0;
                    let a1391 = 67u64;
                    if nat_lt(a1390, a1391) {
                        let a1392 = v0;
                        let a1393 = 66u64;
                        if nat_lt(a1392, a1393) {
                            15097957966210449927u64
                        } else {
                            16922976911328602910u64
                        }
                    } else {
                        let a1394 = v0;
                        let a1395 = 68u64;
                        if nat_lt(a1394, a1395) {
                            17689382322260857208u64
                        } else {
                            let a1396 = v0;
                            let a1397 = 69u64;
                            if nat_lt(a1396, a1397) {
                                500013540394364858u64
                            } else {
                                748580250866718886u64
                            }
                        }
                    }
                }
            } else {
                let a1398 = v0;
                let a1399 = 75u64;
                if nat_lt(a1398, a1399) {
                    let a1400 = v0;
                    let a1401 = 72u64;
                    if nat_lt(a1400, a1401) {
                        let a1402 = v0;
                        let a1403 = 71u64;
                        if nat_lt(a1402, a1403) {
                            1242879168328830382u64
                        } else {
                            1977374033974150939u64
                        }
                    } else {
                        let a1404 = v0;
                        let a1405 = 73u64;
                        if nat_lt(a1404, a1405) {
                            2944078676154940804u64
                        } else {
                            let a1406 = v0;
                            let a1407 = 74u64;
                            if nat_lt(a1406, a1407) {
                                3659926193048069267u64
                            } else {
                                4368137639120453308u64
                            }
                        }
                    }
                } else {
                    let a1408 = v0;
                    let a1409 = 77u64;
                    if nat_lt(a1408, a1409) {
                        let a1410 = v0;
                        let a1411 = 76u64;
                        if nat_lt(a1410, a1411) {
                            4836135668995329356u64
                        } else {
                            5532061633213252278u64
                        }
                    } else {
                        let a1412 = v0;
                        let a1413 = 78u64;
                        if nat_lt(a1412, a1413) {
                            6448918945643986474u64
                        } else {
                            let a1414 = v0;
                            let a1415 = 79u64;
                            if nat_lt(a1414, a1415) {
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

pub fn f111(v0: u64) -> u64 {
    let a1418 = {
        let a1416 = f127(v0, 14u32, 50u32);
        let a1417 = f127(v0, 18u32, 46u32);
        fixed_u64::bit_xor(a1416, a1417)
    };
    let a1419 = f127(v0, 41u32, 23u32);
    fixed_u64::bit_xor(a1418, a1419)
}

pub fn f112(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1425 = {
        let a1420 = v0;
        let a1421 = v1;
        fixed_u64::bit_and(a1420, a1421)
    };
    let a1426 = {
        let a1423 = {
            let a1422 = v0;
            fixed_u64::bit_not(a1422)
        };
        let a1424 = v2;
        fixed_u64::bit_and(a1423, a1424)
    };
    fixed_u64::bit_xor(a1425, a1426)
}

pub fn f113(v0: u64) -> u64 {
    let a1429 = {
        let a1427 = f127(v0, 28u32, 36u32);
        let a1428 = f127(v0, 34u32, 30u32);
        fixed_u64::bit_xor(a1427, a1428)
    };
    let a1430 = f127(v0, 39u32, 25u32);
    fixed_u64::bit_xor(a1429, a1430)
}

pub fn f114(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1439 = {
        let a1435 = {
            let a1431 = v0;
            let a1432 = v1;
            fixed_u64::bit_and(a1431, a1432)
        };
        let a1436 = {
            let a1433 = v0;
            let a1434 = v2;
            fixed_u64::bit_and(a1433, a1434)
        };
        fixed_u64::bit_xor(a1435, a1436)
    };
    let a1440 = {
        let a1437 = v1;
        let a1438 = v2;
        fixed_u64::bit_and(a1437, a1438)
    };
    fixed_u64::bit_xor(a1439, a1440)
}

pub fn f115(v0: Adt6) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(_, h1441, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1441
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h1442, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1442
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h1443, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1443
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h1444, _, _, _, _, _, _, _, _, _, _, _) => {
            h1444
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h1445, _, _, _, _, _, _, _, _, _, _) => {
            h1445
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h1446, _, _, _, _, _, _, _, _, _) => {
            h1446
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h1447, _, _, _, _, _, _, _, _) => {
            h1447
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h1448, _, _, _, _, _, _, _) => {
            h1448
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1449, _, _, _, _, _, _) => {
            h1449
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1450, _, _, _, _, _) => {
            h1450
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h1451, _, _, _, _) => {
            h1451
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1452, _, _, _) => {
            h1452
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1453, _, _) => {
            h1453
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1454, _) => {
            h1454
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1455) => {
            h1455
        }
    }, f71(f128(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1456, _) => {
            h1456
        }
    }), f71(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1457, _, _, _, _, _, _) => {
            h1457
        }
    }, f71(f129(match v0.clone() {
        Adt6::C0(_, h1458, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1458
        }
    }), match v0 {
        Adt6::C0(h1459, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1459
        }
    }))))
}

pub fn f116(v0: Fn12, p1463: u64, p1464: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let m1462 = (p1463, p1464);
    {
        let mut m1462 = m1462;
        loop {
            let step = {
                let (v1, v2) = m1462;
                let m1460 = v1;
                if m1460 == 0 {
                    Err::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>(v2)
                } else {
                    let v3 = m1460 - 1;
                    Ok::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>((v3, {
                        let c1461: Fn12 = v0.clone();
                        c1461.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let v1: u64 = {
        let (h1465, _) = v0;
        h1465
    };
    let v2: u64 = {
        let (h1466, _) = {
            let (_, h1467) = v0;
            h1467
        };
        h1466
    };
    let v3: u64 = {
        let (_, h1468) = {
            let (_, h1469) = v0;
            h1469
        };
        h1468
    };
    ({
        let a1470 = v1;
        let a1471 = 2u64;
        let a1472 = 0u64;
        nat_quot(a1470, a1471, a1472)
    }, (f91(v2, 1u32), {
        let a1476 = {
            let a1473 = v1;
            let a1474 = 2u64;
            let a1475 = 0u64;
            nat_rem(a1473, a1474, a1475)
        };
        let a1477 = 1u64;
        if nat_eq(a1476, a1477) {
            let a1478 = v3;
            let a1479 = v2;
            fixed_u64::bit_or(a1478, a1479)
        } else {
            v3
        }
    }))
}

pub fn f118(v0: Bytes, v1: u64) -> u64 {
    let m1482 = {
        let a1480 = v0;
        let a1481 = v1;
        index_bytes(a1480, a1481)
    };
    match m1482 {
        None => {
            0u64
        }
        Some(v2) => {
            let m1484 = {
                let a1483 = v2;
                fixed_u64::convert(i128::from(a1483))
            };
            match m1484 {
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

pub fn f119(v0: Adt10) -> Adt10 {
    let a1489 = {
        let a1486 = match v0.clone() {
            Adt10::C0(_, h1485, _) => {
                h1485
            }
        };
        let a1487 = 2u64;
        let a1488 = 0u64;
        nat_rem(a1486, a1487, a1488)
    };
    let a1490 = 0u64;
    if nat_eq(a1489, a1490) {
        let m1492 = match v0.clone() {
            Adt10::C0(_, _, h1491) => {
                h1491
            }
        };
        match m1492.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt10::C0(f82(v1, match v0.clone() {
                    Adt10::C0(h1493, _, _) => {
                        h1493
                    }
                }, false), {
                    let a1495 = match v0 {
                        Adt10::C0(_, h1494, _) => {
                            h1494
                        }
                    };
                    let a1496 = 2u64;
                    let a1497 = 0u64;
                    nat_quot(a1495, a1496, a1497)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f120(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f130(match v0.clone() {
        Adt9::C0(h1498, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1498
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1499, _, _, _, _, _, _, _, _, _, _) => {
            h1499
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1500, _, _, _, _, _) => {
            h1500
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1501) => {
            h1501
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1502, _, _, _, _, _, _, _) => {
            h1502
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1503, _, _, _, _, _, _) => {
            h1503
        }
    });
    let v3: Adt11 = f130(match v0.clone() {
        Adt9::C0(_, h1504, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1504
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1505, _, _, _, _, _, _, _, _, _) => {
            h1505
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1506, _, _, _, _) => {
            h1506
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1507, _, _, _) => {
            h1507
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1508, _, _, _, _, _) => {
            h1508
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1509, _, _, _, _) => {
            h1509
        }
    });
    let v4: Adt11 = f130(match v0.clone() {
        Adt9::C0(_, _, h1510, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1510
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1511, _, _, _, _, _, _, _, _) => {
            h1511
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1512, _, _, _, _, _, _, _) => {
            h1512
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1513, _, _) => {
            h1513
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1514, _, _, _) => {
            h1514
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1515, _, _) => {
            h1515
        }
    });
    let v5: Adt11 = f130(match v0.clone() {
        Adt9::C0(_, _, _, h1516, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1516
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1517, _, _, _, _, _, _, _, _, _, _, _) => {
            h1517
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1518, _, _, _, _, _, _) => {
            h1518
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1519, _) => {
            h1519
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1520, _) => {
            h1520
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1521) => {
            h1521
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1522, _, _, _) => {
            h1522
        }
    }, match v3.clone() {
        Adt11::C0(h1523, _, _, _) => {
            h1523
        }
    }, match v4.clone() {
        Adt11::C0(h1524, _, _, _) => {
            h1524
        }
    }, match v5.clone() {
        Adt11::C0(h1525, _, _, _) => {
            h1525
        }
    }, match v5.clone() {
        Adt11::C0(_, h1526, _, _) => {
            h1526
        }
    }, match v2.clone() {
        Adt11::C0(_, h1527, _, _) => {
            h1527
        }
    }, match v3.clone() {
        Adt11::C0(_, h1528, _, _) => {
            h1528
        }
    }, match v4.clone() {
        Adt11::C0(_, h1529, _, _) => {
            h1529
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1530, _) => {
            h1530
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1531, _) => {
            h1531
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1532, _) => {
            h1532
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1533, _) => {
            h1533
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1534) => {
            h1534
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1535) => {
            h1535
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1536) => {
            h1536
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1537) => {
            h1537
        }
    })
}

pub fn f121(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f130(match v0.clone() {
        Adt9::C0(h1538, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1538
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1539, _, _, _, _, _, _, _, _, _, _, _) => {
            h1539
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1540, _, _, _, _, _, _, _) => {
            h1540
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1541, _, _, _) => {
            h1541
        }
    }, match v1.clone() {
        Adt8::C0(h1542, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1542
        }
    }, match v1.clone() {
        Adt8::C0(_, h1543, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1543
        }
    });
    let v3: Adt11 = f130(match v0.clone() {
        Adt9::C0(_, h1544, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1544
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1545, _, _, _, _, _, _, _, _, _, _) => {
            h1545
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1546, _, _, _, _, _, _) => {
            h1546
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1547, _, _) => {
            h1547
        }
    }, match v1.clone() {
        Adt8::C0(_, _, h1548, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1548
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, h1549, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1549
        }
    });
    let v4: Adt11 = f130(match v0.clone() {
        Adt9::C0(_, _, h1550, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1550
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1551, _, _, _, _, _, _, _, _, _) => {
            h1551
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1552, _, _, _, _, _) => {
            h1552
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1553, _) => {
            h1553
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, h1554, _, _, _, _, _, _, _, _, _, _, _) => {
            h1554
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, h1555, _, _, _, _, _, _, _, _, _, _) => {
            h1555
        }
    });
    let v5: Adt11 = f130(match v0.clone() {
        Adt9::C0(_, _, _, h1556, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1556
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1557, _, _, _, _, _, _, _, _) => {
            h1557
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1558, _, _, _, _) => {
            h1558
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1559) => {
            h1559
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, h1560, _, _, _, _, _, _, _, _, _) => {
            h1560
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, h1561, _, _, _, _, _, _, _, _) => {
            h1561
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1562, _, _, _) => {
            h1562
        }
    }, match v3.clone() {
        Adt11::C0(h1563, _, _, _) => {
            h1563
        }
    }, match v4.clone() {
        Adt11::C0(h1564, _, _, _) => {
            h1564
        }
    }, match v5.clone() {
        Adt11::C0(h1565, _, _, _) => {
            h1565
        }
    }, match v2.clone() {
        Adt11::C0(_, h1566, _, _) => {
            h1566
        }
    }, match v3.clone() {
        Adt11::C0(_, h1567, _, _) => {
            h1567
        }
    }, match v4.clone() {
        Adt11::C0(_, h1568, _, _) => {
            h1568
        }
    }, match v5.clone() {
        Adt11::C0(_, h1569, _, _) => {
            h1569
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1570, _) => {
            h1570
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1571, _) => {
            h1571
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1572, _) => {
            h1572
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1573, _) => {
            h1573
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1574) => {
            h1574
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1575) => {
            h1575
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1576) => {
            h1576
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1577) => {
            h1577
        }
    })
}

pub fn f122(v0: Adt8) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt8::C0(_, _, h1578, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1578
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h1579, _, _, _, _, _, _, _, _, _) => {
            h1579
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, h1580, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1580
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1581, _, _, _, _, _) => {
            h1581
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h1582, _, _, _, _, _, _, _, _) => {
            h1582
        }
    }, match v0.clone() {
        Adt8::C0(h1583, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1583
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h1584, _, _, _, _, _, _, _, _, _, _, _) => {
            h1584
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1585, _, _) => {
            h1585
        }
    }, match v0.clone() {
        Adt8::C0(_, h1586, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1586
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1587, _, _, _, _) => {
            h1587
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1588, _, _, _) => {
            h1588
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h1589, _, _, _, _, _, _, _, _, _, _) => {
            h1589
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1590, _, _, _, _, _, _) => {
            h1590
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1591, _) => {
            h1591
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1592) => {
            h1592
        }
    }, match v0 {
        Adt8::C0(_, _, _, _, _, _, _, _, h1593, _, _, _, _, _, _, _) => {
            h1593
        }
    })
}

pub fn f123(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1594 = f45(v0, v1);
    let a1595 = f99(v0, v2);
    fixed_u32::bit_or(a1594, a1595)
}

pub fn f124(v0: u32) -> u32 {
    let a1598 = {
        let a1596 = f123(v0, 17u32, 15u32);
        let a1597 = f123(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1596, a1597)
    };
    let a1599 = f45(v0, 10u32);
    fixed_u32::bit_xor(a1598, a1599)
}

pub fn f125(v0: u32) -> u32 {
    let a1602 = {
        let a1600 = f123(v0, 7u32, 25u32);
        let a1601 = f123(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1600, a1601)
    };
    let a1603 = f45(v0, 3u32);
    fixed_u32::bit_xor(a1602, a1603)
}

pub fn f126(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a1605 = {
            let (h1604, _) = v0.clone();
            h1604
        };
        let a1606 = 1u64;
        nat_add(a1605, a1606)?
    }, f131(f132(f133(f134({
        let (_, h1607) = v0;
        h1607
    }))), {
        let (h1608, _) = v0;
        h1608
    })))
}

pub fn f127(v0: u64, v1: u32, v2: u32) -> u64 {
    let a1609 = f54(v0, v1);
    let a1610 = f91(v0, v2);
    fixed_u64::bit_or(a1609, a1610)
}

pub fn f128(v0: u64) -> u64 {
    let a1613 = {
        let a1611 = f127(v0, 19u32, 45u32);
        let a1612 = f127(v0, 61u32, 3u32);
        fixed_u64::bit_xor(a1611, a1612)
    };
    let a1614 = f54(v0, 6u32);
    fixed_u64::bit_xor(a1613, a1614)
}

pub fn f129(v0: u64) -> u64 {
    let a1617 = {
        let a1615 = f127(v0, 1u32, 63u32);
        let a1616 = f127(v0, 8u32, 56u32);
        fixed_u64::bit_xor(a1615, a1616)
    };
    let a1618 = f54(v0, 7u32);
    fixed_u64::bit_xor(a1617, a1618)
}

pub fn f130(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt11 {
    let v6: u32 = f66(f66(v0, v1), v4);
    let v7: u32 = f123({
        let a1619 = v3;
        let a1620 = v6;
        fixed_u32::bit_xor(a1619, a1620)
    }, 16u32, 16u32);
    let v8: u32 = f66(v2, v7);
    let v9: u32 = f123({
        let a1621 = v1;
        let a1622 = v8;
        fixed_u32::bit_xor(a1621, a1622)
    }, 12u32, 20u32);
    let v10: u32 = f66(f66(v6, v9), v5);
    let v11: u32 = f123({
        let a1623 = v7;
        let a1624 = v10;
        fixed_u32::bit_xor(a1623, a1624)
    }, 8u32, 24u32);
    let v12: u32 = f66(v8, v11);
    let v13: u32 = f123({
        let a1625 = v9;
        let a1626 = v12;
        fixed_u32::bit_xor(a1625, a1626)
    }, 7u32, 25u32);
    Adt11::C0(v10, v13, v12, v11)
}

pub fn f131(v0: Adt3, v1: u64) -> Adt3 {
    let v2: u64 = f135(v1);
    Adt3::C0({
        let a1628 = match v0.clone() {
            Adt3::C0(h1627, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1627
            }
        };
        let a1629 = v2;
        fixed_u64::bit_xor(a1628, a1629)
    }, match v0.clone() {
        Adt3::C0(_, h1630, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1630
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1631, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1631
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1632, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1632
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h1633, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1633
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1634, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1634
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1635, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1635
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1636, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1636
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1637, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1637
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1638, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1638
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1639, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1639
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1640, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1640
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1641, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1641
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1642, _, _, _, _, _, _, _, _, _, _, _) => {
            h1642
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1643, _, _, _, _, _, _, _, _, _, _) => {
            h1643
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1644, _, _, _, _, _, _, _, _, _) => {
            h1644
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1645, _, _, _, _, _, _, _, _) => {
            h1645
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1646, _, _, _, _, _, _, _) => {
            h1646
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1647, _, _, _, _, _, _) => {
            h1647
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1648, _, _, _, _, _) => {
            h1648
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1649, _, _, _, _) => {
            h1649
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1650, _, _, _) => {
            h1650
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1651, _, _) => {
            h1651
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1652, _) => {
            h1652
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1653) => {
            h1653
        }
    })
}

pub fn f132(v0: Adt3) -> Adt3 {
    Adt3::C0({
        let a1660 = match v0.clone() {
            Adt3::C0(h1654, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1654
            }
        };
        let a1661 = {
            let a1658 = {
                let a1656 = match v0.clone() {
                    Adt3::C0(_, h1655, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1655
                    }
                };
                fixed_u64::bit_not(a1656)
            };
            let a1659 = match v0.clone() {
                Adt3::C0(_, _, h1657, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1657
                }
            };
            fixed_u64::bit_and(a1658, a1659)
        };
        fixed_u64::bit_xor(a1660, a1661)
    }, {
        let a1668 = match v0.clone() {
            Adt3::C0(_, h1662, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1662
            }
        };
        let a1669 = {
            let a1666 = {
                let a1664 = match v0.clone() {
                    Adt3::C0(_, _, h1663, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1663
                    }
                };
                fixed_u64::bit_not(a1664)
            };
            let a1667 = match v0.clone() {
                Adt3::C0(_, _, _, h1665, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1665
                }
            };
            fixed_u64::bit_and(a1666, a1667)
        };
        fixed_u64::bit_xor(a1668, a1669)
    }, {
        let a1676 = match v0.clone() {
            Adt3::C0(_, _, h1670, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1670
            }
        };
        let a1677 = {
            let a1674 = {
                let a1672 = match v0.clone() {
                    Adt3::C0(_, _, _, h1671, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1671
                    }
                };
                fixed_u64::bit_not(a1672)
            };
            let a1675 = match v0.clone() {
                Adt3::C0(_, _, _, _, h1673, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1673
                }
            };
            fixed_u64::bit_and(a1674, a1675)
        };
        fixed_u64::bit_xor(a1676, a1677)
    }, {
        let a1684 = match v0.clone() {
            Adt3::C0(_, _, _, h1678, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1678
            }
        };
        let a1685 = {
            let a1682 = {
                let a1680 = match v0.clone() {
                    Adt3::C0(_, _, _, _, h1679, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1679
                    }
                };
                fixed_u64::bit_not(a1680)
            };
            let a1683 = match v0.clone() {
                Adt3::C0(h1681, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1681
                }
            };
            fixed_u64::bit_and(a1682, a1683)
        };
        fixed_u64::bit_xor(a1684, a1685)
    }, {
        let a1692 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1686, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1686
            }
        };
        let a1693 = {
            let a1690 = {
                let a1688 = match v0.clone() {
                    Adt3::C0(h1687, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1687
                    }
                };
                fixed_u64::bit_not(a1688)
            };
            let a1691 = match v0.clone() {
                Adt3::C0(_, h1689, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1689
                }
            };
            fixed_u64::bit_and(a1690, a1691)
        };
        fixed_u64::bit_xor(a1692, a1693)
    }, {
        let a1700 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1694, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1694
            }
        };
        let a1701 = {
            let a1698 = {
                let a1696 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1695, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1695
                    }
                };
                fixed_u64::bit_not(a1696)
            };
            let a1699 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h1697, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1697
                }
            };
            fixed_u64::bit_and(a1698, a1699)
        };
        fixed_u64::bit_xor(a1700, a1701)
    }, {
        let a1708 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1702, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1702
            }
        };
        let a1709 = {
            let a1706 = {
                let a1704 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, h1703, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1703
                    }
                };
                fixed_u64::bit_not(a1704)
            };
            let a1707 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, h1705, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1705
                }
            };
            fixed_u64::bit_and(a1706, a1707)
        };
        fixed_u64::bit_xor(a1708, a1709)
    }, {
        let a1716 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1710, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1710
            }
        };
        let a1717 = {
            let a1714 = {
                let a1712 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, h1711, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1711
                    }
                };
                fixed_u64::bit_not(a1712)
            };
            let a1715 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, h1713, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1713
                }
            };
            fixed_u64::bit_and(a1714, a1715)
        };
        fixed_u64::bit_xor(a1716, a1717)
    }, {
        let a1724 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1718, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1718
            }
        };
        let a1725 = {
            let a1722 = {
                let a1720 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, h1719, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1719
                    }
                };
                fixed_u64::bit_not(a1720)
            };
            let a1723 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, h1721, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1721
                }
            };
            fixed_u64::bit_and(a1722, a1723)
        };
        fixed_u64::bit_xor(a1724, a1725)
    }, {
        let a1732 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1726, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1726
            }
        };
        let a1733 = {
            let a1730 = {
                let a1728 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, h1727, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1727
                    }
                };
                fixed_u64::bit_not(a1728)
            };
            let a1731 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, h1729, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1729
                }
            };
            fixed_u64::bit_and(a1730, a1731)
        };
        fixed_u64::bit_xor(a1732, a1733)
    }, {
        let a1740 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1734, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1734
            }
        };
        let a1741 = {
            let a1738 = {
                let a1736 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1735, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1735
                    }
                };
                fixed_u64::bit_not(a1736)
            };
            let a1739 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1737, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1737
                }
            };
            fixed_u64::bit_and(a1738, a1739)
        };
        fixed_u64::bit_xor(a1740, a1741)
    }, {
        let a1748 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1742, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1742
            }
        };
        let a1749 = {
            let a1746 = {
                let a1744 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1743, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1743
                    }
                };
                fixed_u64::bit_not(a1744)
            };
            let a1747 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1745, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1745
                }
            };
            fixed_u64::bit_and(a1746, a1747)
        };
        fixed_u64::bit_xor(a1748, a1749)
    }, {
        let a1756 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1750, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1750
            }
        };
        let a1757 = {
            let a1754 = {
                let a1752 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1751, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1751
                    }
                };
                fixed_u64::bit_not(a1752)
            };
            let a1755 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1753, _, _, _, _, _, _, _, _, _, _) => {
                    h1753
                }
            };
            fixed_u64::bit_and(a1754, a1755)
        };
        fixed_u64::bit_xor(a1756, a1757)
    }, {
        let a1764 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1758, _, _, _, _, _, _, _, _, _, _, _) => {
                h1758
            }
        };
        let a1765 = {
            let a1762 = {
                let a1760 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1759, _, _, _, _, _, _, _, _, _, _) => {
                        h1759
                    }
                };
                fixed_u64::bit_not(a1760)
            };
            let a1763 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1761, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1761
                }
            };
            fixed_u64::bit_and(a1762, a1763)
        };
        fixed_u64::bit_xor(a1764, a1765)
    }, {
        let a1772 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1766, _, _, _, _, _, _, _, _, _, _) => {
                h1766
            }
        };
        let a1773 = {
            let a1770 = {
                let a1768 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1767, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1767
                    }
                };
                fixed_u64::bit_not(a1768)
            };
            let a1771 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1769, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1769
                }
            };
            fixed_u64::bit_and(a1770, a1771)
        };
        fixed_u64::bit_xor(a1772, a1773)
    }, {
        let a1780 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1774, _, _, _, _, _, _, _, _, _) => {
                h1774
            }
        };
        let a1781 = {
            let a1778 = {
                let a1776 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1775, _, _, _, _, _, _, _, _) => {
                        h1775
                    }
                };
                fixed_u64::bit_not(a1776)
            };
            let a1779 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1777, _, _, _, _, _, _, _) => {
                    h1777
                }
            };
            fixed_u64::bit_and(a1778, a1779)
        };
        fixed_u64::bit_xor(a1780, a1781)
    }, {
        let a1788 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1782, _, _, _, _, _, _, _, _) => {
                h1782
            }
        };
        let a1789 = {
            let a1786 = {
                let a1784 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1783, _, _, _, _, _, _, _) => {
                        h1783
                    }
                };
                fixed_u64::bit_not(a1784)
            };
            let a1787 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1785, _, _, _, _, _, _) => {
                    h1785
                }
            };
            fixed_u64::bit_and(a1786, a1787)
        };
        fixed_u64::bit_xor(a1788, a1789)
    }, {
        let a1796 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1790, _, _, _, _, _, _, _) => {
                h1790
            }
        };
        let a1797 = {
            let a1794 = {
                let a1792 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1791, _, _, _, _, _, _) => {
                        h1791
                    }
                };
                fixed_u64::bit_not(a1792)
            };
            let a1795 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1793, _, _, _, _, _) => {
                    h1793
                }
            };
            fixed_u64::bit_and(a1794, a1795)
        };
        fixed_u64::bit_xor(a1796, a1797)
    }, {
        let a1804 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1798, _, _, _, _, _, _) => {
                h1798
            }
        };
        let a1805 = {
            let a1802 = {
                let a1800 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1799, _, _, _, _, _) => {
                        h1799
                    }
                };
                fixed_u64::bit_not(a1800)
            };
            let a1803 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1801, _, _, _, _, _, _, _, _, _) => {
                    h1801
                }
            };
            fixed_u64::bit_and(a1802, a1803)
        };
        fixed_u64::bit_xor(a1804, a1805)
    }, {
        let a1812 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1806, _, _, _, _, _) => {
                h1806
            }
        };
        let a1813 = {
            let a1810 = {
                let a1808 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1807, _, _, _, _, _, _, _, _, _) => {
                        h1807
                    }
                };
                fixed_u64::bit_not(a1808)
            };
            let a1811 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1809, _, _, _, _, _, _, _, _) => {
                    h1809
                }
            };
            fixed_u64::bit_and(a1810, a1811)
        };
        fixed_u64::bit_xor(a1812, a1813)
    }, {
        let a1820 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1814, _, _, _, _) => {
                h1814
            }
        };
        let a1821 = {
            let a1818 = {
                let a1816 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1815, _, _, _) => {
                        h1815
                    }
                };
                fixed_u64::bit_not(a1816)
            };
            let a1819 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1817, _, _) => {
                    h1817
                }
            };
            fixed_u64::bit_and(a1818, a1819)
        };
        fixed_u64::bit_xor(a1820, a1821)
    }, {
        let a1828 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1822, _, _, _) => {
                h1822
            }
        };
        let a1829 = {
            let a1826 = {
                let a1824 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1823, _, _) => {
                        h1823
                    }
                };
                fixed_u64::bit_not(a1824)
            };
            let a1827 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1825, _) => {
                    h1825
                }
            };
            fixed_u64::bit_and(a1826, a1827)
        };
        fixed_u64::bit_xor(a1828, a1829)
    }, {
        let a1836 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1830, _, _) => {
                h1830
            }
        };
        let a1837 = {
            let a1834 = {
                let a1832 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1831, _) => {
                        h1831
                    }
                };
                fixed_u64::bit_not(a1832)
            };
            let a1835 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1833) => {
                    h1833
                }
            };
            fixed_u64::bit_and(a1834, a1835)
        };
        fixed_u64::bit_xor(a1836, a1837)
    }, {
        let a1844 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1838, _) => {
                h1838
            }
        };
        let a1845 = {
            let a1842 = {
                let a1840 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1839) => {
                        h1839
                    }
                };
                fixed_u64::bit_not(a1840)
            };
            let a1843 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1841, _, _, _, _) => {
                    h1841
                }
            };
            fixed_u64::bit_and(a1842, a1843)
        };
        fixed_u64::bit_xor(a1844, a1845)
    }, {
        let a1852 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1846) => {
                h1846
            }
        };
        let a1853 = {
            let a1850 = {
                let a1848 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1847, _, _, _, _) => {
                        h1847
                    }
                };
                fixed_u64::bit_not(a1848)
            };
            let a1851 = match v0 {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1849, _, _, _) => {
                    h1849
                }
            };
            fixed_u64::bit_and(a1850, a1851)
        };
        fixed_u64::bit_xor(a1852, a1853)
    })
}

pub fn f133(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h1854, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1854
        }
    }, f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1855, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1855
        }
    }, 44u32, 20u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1856, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1856
        }
    }, 43u32, 21u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1857, _, _, _, _, _, _) => {
            h1857
        }
    }, 21u32, 43u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1858) => {
            h1858
        }
    }, 14u32, 50u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, h1859, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1859
        }
    }, 28u32, 36u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1860, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1860
        }
    }, 20u32, 44u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1861, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1861
        }
    }, 3u32, 61u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1862, _, _, _, _, _, _, _, _) => {
            h1862
        }
    }, 45u32, 19u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1863, _, _) => {
            h1863
        }
    }, 61u32, 3u32), f136(match v0.clone() {
        Adt3::C0(_, h1864, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1864
        }
    }, 1u32, 63u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1865, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1865
        }
    }, 6u32, 58u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1866, _, _, _, _, _, _, _, _, _, _, _) => {
            h1866
        }
    }, 25u32, 39u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1867, _, _, _, _, _) => {
            h1867
        }
    }, 8u32, 56u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1868, _, _, _, _) => {
            h1868
        }
    }, 18u32, 46u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, h1869, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1869
        }
    }, 27u32, 37u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1870, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1870
        }
    }, 36u32, 28u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1871, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1871
        }
    }, 10u32, 54u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1872, _, _, _, _, _, _, _) => {
            h1872
        }
    }, 15u32, 49u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1873, _) => {
            h1873
        }
    }, 56u32, 8u32), f136(match v0.clone() {
        Adt3::C0(_, _, h1874, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1874
        }
    }, 62u32, 2u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1875, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1875
        }
    }, 55u32, 9u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1876, _, _, _, _, _, _, _, _, _, _) => {
            h1876
        }
    }, 39u32, 25u32), f136(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1877, _, _, _, _, _, _, _, _, _) => {
            h1877
        }
    }, 41u32, 23u32), f136(match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1878, _, _, _) => {
            h1878
        }
    }, 2u32, 62u32))
}

pub fn f134(v0: Adt3) -> Adt3 {
    let v1: u64 = {
        let a1890 = {
            let a1887 = {
                let a1884 = {
                    let a1881 = match v0.clone() {
                        Adt3::C0(h1879, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1879
                        }
                    };
                    let a1882 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1880, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1880
                        }
                    };
                    fixed_u64::bit_xor(a1881, a1882)
                };
                let a1885 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1883, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1883
                    }
                };
                fixed_u64::bit_xor(a1884, a1885)
            };
            let a1888 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1886, _, _, _, _, _, _, _, _, _) => {
                    h1886
                }
            };
            fixed_u64::bit_xor(a1887, a1888)
        };
        let a1891 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1889, _, _, _, _) => {
                h1889
            }
        };
        fixed_u64::bit_xor(a1890, a1891)
    };
    let v2: u64 = {
        let a1903 = {
            let a1900 = {
                let a1897 = {
                    let a1894 = match v0.clone() {
                        Adt3::C0(_, h1892, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1892
                        }
                    };
                    let a1895 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, h1893, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1893
                        }
                    };
                    fixed_u64::bit_xor(a1894, a1895)
                };
                let a1898 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1896, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1896
                    }
                };
                fixed_u64::bit_xor(a1897, a1898)
            };
            let a1901 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1899, _, _, _, _, _, _, _, _) => {
                    h1899
                }
            };
            fixed_u64::bit_xor(a1900, a1901)
        };
        let a1904 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1902, _, _, _) => {
                h1902
            }
        };
        fixed_u64::bit_xor(a1903, a1904)
    };
    let v3: u64 = {
        let a1916 = {
            let a1913 = {
                let a1910 = {
                    let a1907 = match v0.clone() {
                        Adt3::C0(_, _, h1905, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1905
                        }
                    };
                    let a1908 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, h1906, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1906
                        }
                    };
                    fixed_u64::bit_xor(a1907, a1908)
                };
                let a1911 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1909, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1909
                    }
                };
                fixed_u64::bit_xor(a1910, a1911)
            };
            let a1914 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1912, _, _, _, _, _, _, _) => {
                    h1912
                }
            };
            fixed_u64::bit_xor(a1913, a1914)
        };
        let a1917 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1915, _, _) => {
                h1915
            }
        };
        fixed_u64::bit_xor(a1916, a1917)
    };
    let v4: u64 = {
        let a1929 = {
            let a1926 = {
                let a1923 = {
                    let a1920 = match v0.clone() {
                        Adt3::C0(_, _, _, h1918, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1918
                        }
                    };
                    let a1921 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, h1919, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1919
                        }
                    };
                    fixed_u64::bit_xor(a1920, a1921)
                };
                let a1924 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1922, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1922
                    }
                };
                fixed_u64::bit_xor(a1923, a1924)
            };
            let a1927 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1925, _, _, _, _, _, _) => {
                    h1925
                }
            };
            fixed_u64::bit_xor(a1926, a1927)
        };
        let a1930 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1928, _) => {
                h1928
            }
        };
        fixed_u64::bit_xor(a1929, a1930)
    };
    let v5: u64 = {
        let a1942 = {
            let a1939 = {
                let a1936 = {
                    let a1933 = match v0.clone() {
                        Adt3::C0(_, _, _, _, h1931, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1931
                        }
                    };
                    let a1934 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, _, h1932, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1932
                        }
                    };
                    fixed_u64::bit_xor(a1933, a1934)
                };
                let a1937 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1935, _, _, _, _, _, _, _, _, _, _) => {
                        h1935
                    }
                };
                fixed_u64::bit_xor(a1936, a1937)
            };
            let a1940 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1938, _, _, _, _, _) => {
                    h1938
                }
            };
            fixed_u64::bit_xor(a1939, a1940)
        };
        let a1943 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1941) => {
                h1941
            }
        };
        fixed_u64::bit_xor(a1942, a1943)
    };
    let v6: u64 = {
        let a1944 = v5;
        let a1945 = f136(v2, 1u32, 63u32);
        fixed_u64::bit_xor(a1944, a1945)
    };
    let v7: u64 = {
        let a1946 = v1;
        let a1947 = f136(v3, 1u32, 63u32);
        fixed_u64::bit_xor(a1946, a1947)
    };
    let v8: u64 = {
        let a1948 = v2;
        let a1949 = f136(v4, 1u32, 63u32);
        fixed_u64::bit_xor(a1948, a1949)
    };
    let v9: u64 = {
        let a1950 = v3;
        let a1951 = f136(v5, 1u32, 63u32);
        fixed_u64::bit_xor(a1950, a1951)
    };
    let v10: u64 = {
        let a1952 = v4;
        let a1953 = f136(v1, 1u32, 63u32);
        fixed_u64::bit_xor(a1952, a1953)
    };
    Adt3::C0({
        let a1955 = match v0.clone() {
            Adt3::C0(h1954, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1954
            }
        };
        let a1956 = v6;
        fixed_u64::bit_xor(a1955, a1956)
    }, {
        let a1958 = match v0.clone() {
            Adt3::C0(_, h1957, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1957
            }
        };
        let a1959 = v7;
        fixed_u64::bit_xor(a1958, a1959)
    }, {
        let a1961 = match v0.clone() {
            Adt3::C0(_, _, h1960, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1960
            }
        };
        let a1962 = v8;
        fixed_u64::bit_xor(a1961, a1962)
    }, {
        let a1964 = match v0.clone() {
            Adt3::C0(_, _, _, h1963, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1963
            }
        };
        let a1965 = v9;
        fixed_u64::bit_xor(a1964, a1965)
    }, {
        let a1967 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1966, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1966
            }
        };
        let a1968 = v10;
        fixed_u64::bit_xor(a1967, a1968)
    }, {
        let a1970 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1969, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1969
            }
        };
        let a1971 = v6;
        fixed_u64::bit_xor(a1970, a1971)
    }, {
        let a1973 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1972, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1972
            }
        };
        let a1974 = v7;
        fixed_u64::bit_xor(a1973, a1974)
    }, {
        let a1976 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1975, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1975
            }
        };
        let a1977 = v8;
        fixed_u64::bit_xor(a1976, a1977)
    }, {
        let a1979 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1978, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1978
            }
        };
        let a1980 = v9;
        fixed_u64::bit_xor(a1979, a1980)
    }, {
        let a1982 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1981, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1981
            }
        };
        let a1983 = v10;
        fixed_u64::bit_xor(a1982, a1983)
    }, {
        let a1985 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1984, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1984
            }
        };
        let a1986 = v6;
        fixed_u64::bit_xor(a1985, a1986)
    }, {
        let a1988 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1987, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1987
            }
        };
        let a1989 = v7;
        fixed_u64::bit_xor(a1988, a1989)
    }, {
        let a1991 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1990, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1990
            }
        };
        let a1992 = v8;
        fixed_u64::bit_xor(a1991, a1992)
    }, {
        let a1994 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1993, _, _, _, _, _, _, _, _, _, _, _) => {
                h1993
            }
        };
        let a1995 = v9;
        fixed_u64::bit_xor(a1994, a1995)
    }, {
        let a1997 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1996, _, _, _, _, _, _, _, _, _, _) => {
                h1996
            }
        };
        let a1998 = v10;
        fixed_u64::bit_xor(a1997, a1998)
    }, {
        let a2000 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1999, _, _, _, _, _, _, _, _, _) => {
                h1999
            }
        };
        let a2001 = v6;
        fixed_u64::bit_xor(a2000, a2001)
    }, {
        let a2003 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2002, _, _, _, _, _, _, _, _) => {
                h2002
            }
        };
        let a2004 = v7;
        fixed_u64::bit_xor(a2003, a2004)
    }, {
        let a2006 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2005, _, _, _, _, _, _, _) => {
                h2005
            }
        };
        let a2007 = v8;
        fixed_u64::bit_xor(a2006, a2007)
    }, {
        let a2009 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2008, _, _, _, _, _, _) => {
                h2008
            }
        };
        let a2010 = v9;
        fixed_u64::bit_xor(a2009, a2010)
    }, {
        let a2012 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2011, _, _, _, _, _) => {
                h2011
            }
        };
        let a2013 = v10;
        fixed_u64::bit_xor(a2012, a2013)
    }, {
        let a2015 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2014, _, _, _, _) => {
                h2014
            }
        };
        let a2016 = v6;
        fixed_u64::bit_xor(a2015, a2016)
    }, {
        let a2018 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2017, _, _, _) => {
                h2017
            }
        };
        let a2019 = v7;
        fixed_u64::bit_xor(a2018, a2019)
    }, {
        let a2021 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2020, _, _) => {
                h2020
            }
        };
        let a2022 = v8;
        fixed_u64::bit_xor(a2021, a2022)
    }, {
        let a2024 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2023, _) => {
                h2023
            }
        };
        let a2025 = v9;
        fixed_u64::bit_xor(a2024, a2025)
    }, {
        let a2027 = match v0 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2026) => {
                h2026
            }
        };
        let a2028 = v10;
        fixed_u64::bit_xor(a2027, a2028)
    })
}

pub fn f135(v0: u64) -> u64 {
    let a2029 = v0;
    let a2030 = 12u64;
    if nat_lt(a2029, a2030) {
        let a2031 = v0;
        let a2032 = 6u64;
        if nat_lt(a2031, a2032) {
            let a2033 = v0;
            let a2034 = 3u64;
            if nat_lt(a2033, a2034) {
                let a2035 = v0;
                let a2036 = 1u64;
                if nat_lt(a2035, a2036) {
                    1u64
                } else {
                    let a2037 = v0;
                    let a2038 = 2u64;
                    if nat_lt(a2037, a2038) {
                        32898u64
                    } else {
                        9223372036854808714u64
                    }
                }
            } else {
                let a2039 = v0;
                let a2040 = 4u64;
                if nat_lt(a2039, a2040) {
                    9223372039002292224u64
                } else {
                    let a2041 = v0;
                    let a2042 = 5u64;
                    if nat_lt(a2041, a2042) {
                        32907u64
                    } else {
                        2147483649u64
                    }
                }
            }
        } else {
            let a2043 = v0;
            let a2044 = 9u64;
            if nat_lt(a2043, a2044) {
                let a2045 = v0;
                let a2046 = 7u64;
                if nat_lt(a2045, a2046) {
                    9223372039002292353u64
                } else {
                    let a2047 = v0;
                    let a2048 = 8u64;
                    if nat_lt(a2047, a2048) {
                        9223372036854808585u64
                    } else {
                        138u64
                    }
                }
            } else {
                let a2049 = v0;
                let a2050 = 10u64;
                if nat_lt(a2049, a2050) {
                    136u64
                } else {
                    let a2051 = v0;
                    let a2052 = 11u64;
                    if nat_lt(a2051, a2052) {
                        2147516425u64
                    } else {
                        2147483658u64
                    }
                }
            }
        }
    } else {
        let a2053 = v0;
        let a2054 = 18u64;
        if nat_lt(a2053, a2054) {
            let a2055 = v0;
            let a2056 = 15u64;
            if nat_lt(a2055, a2056) {
                let a2057 = v0;
                let a2058 = 13u64;
                if nat_lt(a2057, a2058) {
                    2147516555u64
                } else {
                    let a2059 = v0;
                    let a2060 = 14u64;
                    if nat_lt(a2059, a2060) {
                        9223372036854775947u64
                    } else {
                        9223372036854808713u64
                    }
                }
            } else {
                let a2061 = v0;
                let a2062 = 16u64;
                if nat_lt(a2061, a2062) {
                    9223372036854808579u64
                } else {
                    let a2063 = v0;
                    let a2064 = 17u64;
                    if nat_lt(a2063, a2064) {
                        9223372036854808578u64
                    } else {
                        9223372036854775936u64
                    }
                }
            }
        } else {
            let a2065 = v0;
            let a2066 = 21u64;
            if nat_lt(a2065, a2066) {
                let a2067 = v0;
                let a2068 = 19u64;
                if nat_lt(a2067, a2068) {
                    32778u64
                } else {
                    let a2069 = v0;
                    let a2070 = 20u64;
                    if nat_lt(a2069, a2070) {
                        9223372039002259466u64
                    } else {
                        9223372039002292353u64
                    }
                }
            } else {
                let a2071 = v0;
                let a2072 = 22u64;
                if nat_lt(a2071, a2072) {
                    9223372036854808704u64
                } else {
                    let a2073 = v0;
                    let a2074 = 23u64;
                    if nat_lt(a2073, a2074) {
                        2147483649u64
                    } else {
                        9223372039002292232u64
                    }
                }
            }
        }
    }
}

pub fn f136(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2075 = f91(v0, v1);
    let a2076 = f54(v0, v2);
    fixed_u64::bit_or(a2075, a2076)
}
