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
    C0(u64, List<Adt1>, Option<Bytes>, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(bool, u64, List<(Bytes, Bytes)>, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, Bytes, Bytes, bool),
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
    C0(bool, bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Fn0 {
    F4(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn0::F4(k0) => f4(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F10(Bytes),
    F56(Bytes),
    F93(u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F10(k0) => f10(k0.clone(), p0),
            Fn1::F56(k0) => f56(k0.clone(), p0),
            Fn1::F93(k0, k1) => f93(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F19(Bytes, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt2)) -> R<(u64, Adt2)> {
        match self {
            Fn2::F19(k0, k1, k2) => f19(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F37(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn3::F37(k0) => f37(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F40(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F40(k0) => f40(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F47,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt5) -> R<Adt5> {
        match self {
            Fn5::F47 => f47(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F52(Bytes, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt6, p1: (Bytes, Bytes)) -> Adt6 {
        match self {
            Fn6::F52(k0, k1) => f52(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F58,
    F59,
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> R<Option<Bytes>> {
        match self {
            Fn7::F58 => f58(p0, p1),
            Fn7::F59 => Ok(f59(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F72,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F72 => f72(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F81,
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn9::F81 => f81(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v1) => {
            Ok(Some(f2(v1)?))
        }
        Err(_) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<Bytes, Bytes>> {
    let v1: Adt0 = {
        let (h2, _) = f3(Fn0::F4(v0.clone()), {
            let a4 = {
                let a3 = v0.clone();
                length_bytes(a3)
            };
            let a5 = 2u64;
            nat_add(a4, a5)?
        }, Adt0::C0(0u64, List::<Adt1>::nil(), None::<Bytes>, Bytes::lit(&[])))?;
        h2
    };
    let a8 = 0u64;
    let a9 = {
        let a7 = match v1.clone() {
            Adt0::C0(_, _, _, h6) => {
                h6
            }
        };
        length_bytes(a7)
    };
    if nat_lt(a8, a9) {
        Ok(Err::<Bytes, Bytes>(match v1 {
            Adt0::C0(_, _, _, h10) => {
                h10
            }
        }))
    } else {
        let m12 = match v1.clone() {
            Adt0::C0(_, _, h11, _) => {
                h11
            }
        };
        match m12 {
            None => {
                Ok(Err::<Bytes, Bytes>(f6(Str::lit("unexpected end of input"))))
            }
            Some(v2) => {
                let a15 = f5(v0.clone(), match v1 {
                    Adt0::C0(h13, _, _, _) => {
                        h13
                    }
                })?;
                let a16 = {
                    let a14 = v0;
                    length_bytes(a14)
                };
                if nat_eq(a15, a16) {
                    Ok(Ok::<Bytes, Bytes>(v2))
                } else {
                    Ok(Err::<Bytes, Bytes>(f6(Str::lit("trailing input"))))
                }
            }
        }
    }
}

pub fn f2(v0: Bytes) -> R<Bytes> {
    let a17 = f6(Str::lit("sha256:"));
    let a18 = f7(v0)?;
    Ok(append_bytes(a17, a18))
}

pub fn f3(v0: Fn0, p23: u64, p24: Adt0) -> R<(Adt0, bool)> {
    let m22 = (p23, p24);
    {
        let mut m22 = m22;
        loop {
            let step = {
                let (v1, v2) = m22;
                let m19 = v1;
                if m19 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m19 - 1;
                    let m21 = {
                        let c20: Fn0 = v0.clone();
                        c20.apply(v2.clone())?
                    };
                    match m21 {
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
                Ok(next) => { m22 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Bytes, v1: Adt0) -> R<Option<Adt0>> {
    f8(v0, v1)
}

pub fn f5(v0: Bytes, v1: u64) -> R<u64> {
    let (h25, _) = f9(Fn1::F10(v0.clone()), {
        let a26 = v0;
        length_bytes(a26)
    }, v1)?;
    Ok(h25)
}

pub fn f6(v0: Str) -> Bytes {
    let a27 = v0;
    utf8_encode(a27)
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    Ok(f11(f12(v0)?))
}

pub fn f8(v0: Bytes, v1: Adt0) -> R<Option<Adt0>> {
    let a30 = 0u64;
    let a31 = {
        let a29 = match v1.clone() {
            Adt0::C0(_, _, _, h28) => {
                h28
            }
        };
        length_bytes(a29)
    };
    if nat_lt(a30, a31) {
        Ok(None::<Adt0>)
    } else {
        let m33 = match v1.clone() {
            Adt0::C0(_, _, h32, _) => {
                h32
            }
        };
        match m33 {
            None => {
                let v3: u64 = f5(v0.clone(), match v1.clone() {
                    Adt0::C0(h34, _, _, _) => {
                        h34
                    }
                })?;
                let v4: u64 = f13(v0.clone(), v3)?;
                let m36 = match v1.clone() {
                    Adt0::C0(_, h35, _, _) => {
                        h35
                    }
                };
                match m36.uncons() {
                    None => {
                        Ok(Some(f14(v1, v0, v3, v4)?))
                    }
                    Some((v5, v6)) => {
                        let v7: u64 = match v5.clone() {
                            Adt1::C0(_, h37, _, _) => {
                                h37
                            }
                        };
                        let v8: bool = if match v5.clone() {
                            Adt1::C0(h38, _, _, _) => {
                                h38
                            }
                        } {
                            let a39 = v7;
                            let a40 = 2u64;
                            nat_eq(a39, a40)
                        } else {
                            let a41 = v7;
                            let a42 = 2u64;
                            if nat_eq(a41, a42) {
                                true
                            } else {
                                let a43 = v7;
                                let a44 = 0u64;
                                if nat_eq(a43, a44) {
                                    let a47 = {
                                        let a45 = v4;
                                        let a46 = 93u64;
                                        nat_eq(a45, a46)
                                    };
                                    bool_not(a47)
                                } else {
                                    false
                                }
                            }
                        };
                        if v8 {
                            Ok(Some(f14(v1, v0, v3, v4)?))
                        } else {
                            Ok(Some(f15(v1, v0, v5, v6, v3, v4)?))
                        }
                    }
                }
            }
            Some(_) => {
                Ok(None::<Adt0>)
            }
        }
    }
}

pub fn f9(v0: Fn1, p52: u64, p53: u64) -> R<(u64, bool)> {
    let m51 = (p52, p53);
    {
        let mut m51 = m51;
        loop {
            let step = {
                let (v1, v2) = m51;
                let m48 = v1;
                if m48 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m48 - 1;
                    let m50 = {
                        let c49: Fn1 = v0.clone();
                        c49.apply(v2)?
                    };
                    match m50 {
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
                Ok(next) => { m51 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f16(f13(v0, v1)?) {
        Ok(Some({
            let a54 = v1;
            let a55 = 1u64;
            nat_add(a54, a55)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f11(v0: Adt2) -> Bytes {
    let a76 = f17(match v0.clone() {
        Adt2::C0(h56, _, _, _, _, _, _, _) => {
            h56
        }
    });
    let a77 = {
        let a74 = f17(match v0.clone() {
            Adt2::C0(_, h57, _, _, _, _, _, _) => {
                h57
            }
        });
        let a75 = {
            let a72 = f17(match v0.clone() {
                Adt2::C0(_, _, h58, _, _, _, _, _) => {
                    h58
                }
            });
            let a73 = {
                let a70 = f17(match v0.clone() {
                    Adt2::C0(_, _, _, h59, _, _, _, _) => {
                        h59
                    }
                });
                let a71 = {
                    let a68 = f17(match v0.clone() {
                        Adt2::C0(_, _, _, _, h60, _, _, _) => {
                            h60
                        }
                    });
                    let a69 = {
                        let a66 = f17(match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h61, _, _) => {
                                h61
                            }
                        });
                        let a67 = {
                            let a64 = f17(match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h62, _) => {
                                    h62
                                }
                            });
                            let a65 = f17(match v0 {
                                Adt2::C0(_, _, _, _, _, _, _, h63) => {
                                    h63
                                }
                            });
                            append_bytes(a64, a65)
                        };
                        append_bytes(a66, a67)
                    };
                    append_bytes(a68, a69)
                };
                append_bytes(a70, a71)
            };
            append_bytes(a72, a73)
        };
        append_bytes(a74, a75)
    };
    append_bytes(a76, a77)
}

pub fn f12(v0: Bytes) -> R<Adt2> {
    let v1: u64 = {
        let a78 = v0.clone();
        length_bytes(a78)
    };
    let v2: u64 = {
        let a84 = {
            let a81 = {
                let a79 = v1;
                let a80 = 72u64;
                nat_add(a79, a80)?
            };
            let a82 = 64u64;
            let a83 = 0u64;
            nat_quot(a81, a82, a83)
        };
        let a85 = 64u64;
        nat_mul(a84, a85)?
    };
    let (_, h86) = f18(Fn2::F19(v0, v1, v2), {
        let a87 = v2;
        let a88 = 64u64;
        let a89 = 0u64;
        nat_quot(a87, a88, a89)
    }, (0u64, f20()))?;
    Ok(h86)
}

pub fn f13(v0: Bytes, v1: u64) -> R<u64> {
    let m92 = {
        let a90 = v0;
        let a91 = v1;
        index_bytes(a90, a91)
    };
    match m92 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a137 = {
                let a113 = {
                    let a101 = {
                        let a95 = {
                            let a93 = v3;
                            let a94 = 128u8;
                            fixed_u8::bit_and(a93, a94)
                        };
                        let a96 = 128u8;
                        if equal(a95, a96) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a102 = {
                        let a99 = {
                            let a97 = v3;
                            let a98 = 64u8;
                            fixed_u8::bit_and(a97, a98)
                        };
                        let a100 = 64u8;
                        if equal(a99, a100) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a101, a102)?
                };
                let a114 = {
                    let a111 = {
                        let a105 = {
                            let a103 = v3;
                            let a104 = 32u8;
                            fixed_u8::bit_and(a103, a104)
                        };
                        let a106 = 32u8;
                        if equal(a105, a106) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a112 = {
                        let a109 = {
                            let a107 = v3;
                            let a108 = 16u8;
                            fixed_u8::bit_and(a107, a108)
                        };
                        let a110 = 16u8;
                        if equal(a109, a110) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a111, a112)?
                };
                nat_add(a113, a114)?
            };
            let a138 = {
                let a135 = {
                    let a123 = {
                        let a117 = {
                            let a115 = v3;
                            let a116 = 8u8;
                            fixed_u8::bit_and(a115, a116)
                        };
                        let a118 = 8u8;
                        if equal(a117, a118) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a124 = {
                        let a121 = {
                            let a119 = v3;
                            let a120 = 4u8;
                            fixed_u8::bit_and(a119, a120)
                        };
                        let a122 = 4u8;
                        if equal(a121, a122) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a123, a124)?
                };
                let a136 = {
                    let a133 = {
                        let a127 = {
                            let a125 = v3;
                            let a126 = 2u8;
                            fixed_u8::bit_and(a125, a126)
                        };
                        let a128 = 2u8;
                        if equal(a127, a128) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a134 = {
                        let a131 = {
                            let a129 = v3;
                            let a130 = 1u8;
                            fixed_u8::bit_and(a129, a130)
                        };
                        let a132 = 1u8;
                        if equal(a131, a132) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a133, a134)?
                };
                nat_add(a135, a136)?
            };
            nat_add(a137, a138)
        }
    }
}

pub fn f14(v0: Adt0, v1: Bytes, v2: u64, v3: u64) -> R<Adt0> {
    let a139 = v3;
    let a140 = 123u64;
    if nat_eq(a139, a140) {
        Ok(f21(v0, true, {
            let a141 = v2;
            let a142 = 1u64;
            nat_add(a141, a142)?
        }))
    } else {
        let a143 = v3;
        let a144 = 91u64;
        if nat_eq(a143, a144) {
            Ok(f21(v0, false, {
                let a145 = v2;
                let a146 = 1u64;
                nat_add(a145, a146)?
            }))
        } else {
            let a147 = v3;
            let a148 = 34u64;
            if nat_eq(a147, a148) {
                let v4: Adt3 = f22(v1, v2)?;
                if match v4.clone() {
                    Adt3::C0(_, _, _, h149) => {
                        h149
                    }
                } {
                    Ok(f23(v0.clone(), match v0 {
                        Adt0::C0(_, h150, _, _) => {
                            h150
                        }
                    }, f24(match v4.clone() {
                        Adt3::C0(_, h151, _, _) => {
                            h151
                        }
                    })?, match v4 {
                        Adt3::C0(h152, _, _, _) => {
                            h152
                        }
                    }))
                } else {
                    Ok(f25(v0, match v4 {
                        Adt3::C0(_, _, h153, _) => {
                            h153
                        }
                    }))
                }
            } else {
                let a154 = v3;
                let a155 = 116u64;
                if nat_eq(a154, a155) {
                    f26(v0, v1, v2, Bytes::lit(&[0x74, 0x72, 0x75, 0x65]), Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
                } else {
                    let a156 = v3;
                    let a157 = 102u64;
                    if nat_eq(a156, a157) {
                        f26(v0, v1, v2, Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]), Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
                    } else {
                        let a158 = v3;
                        let a159 = 110u64;
                        if nat_eq(a158, a159) {
                            f26(v0, v1, v2, Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]), Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
                        } else {
                            let a160 = v3;
                            let a161 = 45u64;
                            if nat_eq(a160, a161) {
                                f27(v0, v1, v2)
                            } else {
                                let a162 = 47u64;
                                let a163 = v3;
                                if nat_lt(a162, a163) {
                                    let a164 = v3;
                                    let a165 = 58u64;
                                    if nat_lt(a164, a165) {
                                        f27(v0, v1, v2)
                                    } else {
                                        Ok(f25(v0, f6(Str::lit("unexpected input"))))
                                    }
                                } else {
                                    Ok(f25(v0, f6(Str::lit("unexpected input"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f15(v0: Adt0, v1: Bytes, v2: Adt1, v3: List<Adt1>, v4: u64, v5: u64) -> R<Adt0> {
    let v6: u64 = match v2.clone() {
        Adt1::C0(_, h166, _, _) => {
            h166
        }
    };
    if match v2.clone() {
        Adt1::C0(h167, _, _, _) => {
            h167
        }
    } {
        let a168 = v5;
        let a169 = 125u64;
        if nat_eq(a168, a169) {
            let a170 = v6;
            let a171 = 0u64;
            if nat_eq(a170, a171) {
                f28(v0, v2, v3, {
                    let a172 = v4;
                    let a173 = 1u64;
                    nat_add(a172, a173)?
                })
            } else {
                let a174 = v6;
                let a175 = 3u64;
                if nat_eq(a174, a175) {
                    f28(v0, v2, v3, {
                        let a176 = v4;
                        let a177 = 1u64;
                        nat_add(a176, a177)?
                    })
                } else {
                    Ok(f25(v0, f6(Str::lit("unexpected input"))))
                }
            }
        } else {
            let a178 = v5;
            let a179 = 34u64;
            if nat_eq(a178, a179) {
                let a180 = v6;
                let a181 = 0u64;
                if nat_eq(a180, a181) {
                    f29(v0, v1, v2, v3, v4)
                } else {
                    let a182 = v6;
                    let a183 = 4u64;
                    if nat_eq(a182, a183) {
                        f29(v0, v1, v2, v3, v4)
                    } else {
                        Ok(f25(v0, f6(Str::lit("unexpected input"))))
                    }
                }
            } else {
                let a184 = v5;
                let a185 = 58u64;
                if nat_eq(a184, a185) {
                    let a186 = v6;
                    let a187 = 1u64;
                    if nat_eq(a186, a187) {
                        Ok(f30(v0, f31(v2.clone(), 2u64, match v2 {
                            Adt1::C0(_, _, _, h188) => {
                                h188
                            }
                        }), v3, {
                            let a189 = v4;
                            let a190 = 1u64;
                            nat_add(a189, a190)?
                        }))
                    } else {
                        Ok(f25(v0, f6(Str::lit("unexpected input"))))
                    }
                } else {
                    let a191 = v5;
                    let a192 = 44u64;
                    if nat_eq(a191, a192) {
                        let a193 = v6;
                        let a194 = 3u64;
                        if nat_eq(a193, a194) {
                            Ok(f30(v0, f31(v2, 4u64, Bytes::lit(&[])), v3, {
                                let a195 = v4;
                                let a196 = 1u64;
                                nat_add(a195, a196)?
                            }))
                        } else {
                            Ok(f25(v0, f6(Str::lit("unexpected input"))))
                        }
                    } else {
                        Ok(f25(v0, f6(Str::lit("unexpected input"))))
                    }
                }
            }
        }
    } else {
        let a197 = v5;
        let a198 = 93u64;
        if nat_eq(a197, a198) {
            f28(v0, v2, v3, {
                let a199 = v4;
                let a200 = 1u64;
                nat_add(a199, a200)?
            })
        } else {
            let a201 = v5;
            let a202 = 44u64;
            if nat_eq(a201, a202) {
                let a203 = v6;
                let a204 = 1u64;
                if nat_eq(a203, a204) {
                    Ok(f30(v0, f31(v2, 2u64, Bytes::lit(&[])), v3, {
                        let a205 = v4;
                        let a206 = 1u64;
                        nat_add(a205, a206)?
                    }))
                } else {
                    Ok(f25(v0, f6(Str::lit("unexpected input"))))
                }
            } else {
                Ok(f25(v0, f6(Str::lit("unexpected input"))))
            }
        }
    }
}

pub fn f16(v0: u64) -> bool {
    let a207 = v0;
    let a208 = 32u64;
    if nat_eq(a207, a208) {
        true
    } else {
        let a209 = v0;
        let a210 = 10u64;
        if nat_eq(a209, a210) {
            true
        } else {
            let a211 = v0;
            let a212 = 13u64;
            if nat_eq(a211, a212) {
                true
            } else {
                let a213 = v0;
                let a214 = 9u64;
                nat_eq(a213, a214)
            }
        }
    }
}

pub fn f17(v0: u32) -> Bytes {
    let a243 = f32({
        let a215 = f33(v0, 28u32);
        let a216 = 15u32;
        fixed_u32::bit_and(a215, a216)
    });
    let a244 = {
        let a241 = f32({
            let a217 = f33(v0, 24u32);
            let a218 = 15u32;
            fixed_u32::bit_and(a217, a218)
        });
        let a242 = {
            let a239 = f32({
                let a219 = f33(v0, 20u32);
                let a220 = 15u32;
                fixed_u32::bit_and(a219, a220)
            });
            let a240 = {
                let a237 = f32({
                    let a221 = f33(v0, 16u32);
                    let a222 = 15u32;
                    fixed_u32::bit_and(a221, a222)
                });
                let a238 = {
                    let a235 = f32({
                        let a223 = f33(v0, 12u32);
                        let a224 = 15u32;
                        fixed_u32::bit_and(a223, a224)
                    });
                    let a236 = {
                        let a233 = f32({
                            let a225 = f33(v0, 8u32);
                            let a226 = 15u32;
                            fixed_u32::bit_and(a225, a226)
                        });
                        let a234 = {
                            let a231 = f32({
                                let a227 = f33(v0, 4u32);
                                let a228 = 15u32;
                                fixed_u32::bit_and(a227, a228)
                            });
                            let a232 = f32({
                                let a229 = v0;
                                let a230 = 15u32;
                                fixed_u32::bit_and(a229, a230)
                            });
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
}

pub fn f18(v0: Fn2, p248: u64, p249: (u64, Adt2)) -> R<(u64, Adt2)> {
    let m247 = (p248, p249);
    {
        let mut m247 = m247;
        loop {
            let step = {
                let (v1, v2) = m247;
                let m245 = v1;
                if m245 == 0 {
                    Err::<(u64, (u64, Adt2)), R<(u64, Adt2)>>(Ok(v2))
                } else {
                    let v3 = m245 - 1;
                    Ok::<(u64, (u64, Adt2)), R<(u64, Adt2)>>((v3, {
                        let c246: Fn2 = v0.clone();
                        c246.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt2)) -> R<(u64, Adt2)> {
    Ok(({
        let a251 = {
            let (h250, _) = v3.clone();
            h250
        };
        let a252 = 64u64;
        nat_add(a251, a252)?
    }, f34({
        let (_, h253) = v3;
        h253
    }, f35(v0, v1, v2, {
        let (h254, _) = v3;
        h254
    })?)?))
}

pub fn f20() -> Adt2 {
    Adt2::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f21(v0: Adt0, v1: bool, v2: u64) -> Adt0 {
    Adt0::C0(v2, List::cons(Adt1::C0(v1, 0u64, List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])), match v0 {
        Adt0::C0(_, h255, _, _) => {
            h255
        }
    }), None::<Bytes>, Bytes::lit(&[]))
}

pub fn f22(v0: Bytes, v1: u64) -> R<Adt3> {
    let (h256, _) = f36(Fn3::F37(v0.clone()), {
        let a258 = {
            let a257 = v0;
            length_bytes(a257)
        };
        let a259 = 2u64;
        nat_add(a258, a259)?
    }, Adt3::C0({
        let a260 = v1;
        let a261 = 1u64;
        nat_add(a260, a261)?
    }, Bytes::lit(&[]), Bytes::lit(&[]), false))?;
    Ok(h256)
}

pub fn f23(v0: Adt0, v1: List<Adt1>, v2: Bytes, v3: u64) -> Adt0 {
    let m262 = v1;
    match m262.uncons() {
        None => {
            Adt0::C0(v3, List::<Adt1>::nil(), Some(v2), Bytes::lit(&[]))
        }
        Some((v4, v5)) => {
            if match v4.clone() {
                Adt1::C0(h263, _, _, _) => {
                    h263
                }
            } {
                let m266 = f38(match v4.clone() {
                    Adt1::C0(_, _, _, h264) => {
                        h264
                    }
                }, v2, match v4 {
                    Adt1::C0(_, _, h265, _) => {
                        h265
                    }
                });
                match m266 {
                    None => {
                        f25(v0, f6(Str::lit("duplicate member name")))
                    }
                    Some(v6) => {
                        Adt0::C0(v3, List::cons(Adt1::C0(true, 3u64, v6, Bytes::lit(&[])), v5), None::<Bytes>, Bytes::lit(&[]))
                    }
                }
            } else {
                Adt0::C0(v3, List::cons(Adt1::C0(false, 1u64, List::cons((Bytes::lit(&[]), v2), match v4 {
                    Adt1::C0(_, _, h267, _) => {
                        h267
                    }
                }), Bytes::lit(&[])), v5), None::<Bytes>, Bytes::lit(&[]))
            }
        }
    }
}

pub fn f24(v0: Bytes) -> R<Bytes> {
    let a275 = Bytes::lit(&[0x22]);
    let a276 = {
        let a273 = {
            let (_, h268) = {
                let (h269, _) = f39(Fn4::F40(v0.clone()), {
                    let a271 = {
                        let a270 = v0;
                        length_bytes(a270)
                    };
                    let a272 = 1u64;
                    nat_add(a271, a272)?
                }, (0u64, Bytes::lit(&[])))?;
                h269
            };
            h268
        };
        let a274 = Bytes::lit(&[0x22]);
        append_bytes(a273, a274)
    };
    Ok(append_bytes(a275, a276))
}

pub fn f25(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h277, _, _, _) => {
            h277
        }
    }, match v0 {
        Adt0::C0(_, h278, _, _) => {
            h278
        }
    }, None::<Bytes>, v1)
}

pub fn f26(v0: Adt0, v1: Bytes, v2: u64, v3: Bytes, v4: Bytes) -> R<Adt0> {
    if f41(v1, v3.clone(), v2) {
        Ok(f23(v0.clone(), match v0 {
            Adt0::C0(_, h279, _, _) => {
                h279
            }
        }, v4, {
            let a281 = v2;
            let a282 = {
                let a280 = v3;
                length_bytes(a280)
            };
            nat_add(a281, a282)?
        }))
    } else {
        Ok(f25(v0, f6(Str::lit("bad literal"))))
    }
}

pub fn f27(v0: Adt0, v1: Bytes, v2: u64) -> R<Adt0> {
    let v3: bool = {
        let a283 = f13(v1.clone(), v2)?;
        let a284 = 45u64;
        nat_eq(a283, a284)
    };
    let v4: u64 = if v3 {
        let a285 = v2;
        let a286 = 1u64;
        nat_add(a285, a286)?
    } else {
        v2
    };
    let v5: u64 = f13(v1.clone(), v4)?;
    let a287 = v5;
    let a288 = 48u64;
    if nat_eq(a287, a288) {
        let v6: u64 = {
            let a289 = v4;
            let a290 = 1u64;
            nat_add(a289, a290)?
        };
        f42(v0, v1, v2, v6, v3)
    } else {
        let a291 = 48u64;
        let a292 = v5;
        if nat_lt(a291, a292) {
            let a293 = v5;
            let a294 = 58u64;
            if nat_lt(a293, a294) {
                f42(v0, v1.clone(), v2, f43(v1, {
                    let a295 = v4;
                    let a296 = 1u64;
                    nat_add(a295, a296)?
                })?, false)
            } else {
                Ok(f25(v0, f6(Str::lit("bad number"))))
            }
        } else {
            Ok(f25(v0, f6(Str::lit("bad number"))))
        }
    }
}

pub fn f28(v0: Adt0, v1: Adt1, v2: List<Adt1>, v3: u64) -> R<Adt0> {
    if match v1.clone() {
        Adt1::C0(h297, _, _, _) => {
            h297
        }
    } {
        Ok(f23(v0, v2, {
            let a301 = Bytes::lit(&[0x7b]);
            let a302 = {
                let a299 = f44(match v1 {
                    Adt1::C0(_, _, h298, _) => {
                        h298
                    }
                })?;
                let a300 = Bytes::lit(&[0x7d]);
                append_bytes(a299, a300)
            };
            append_bytes(a301, a302)
        }, v3))
    } else {
        Ok(f23(v0, v2, {
            let a306 = Bytes::lit(&[0x5b]);
            let a307 = {
                let a304 = f45(match v1 {
                    Adt1::C0(_, _, h303, _) => {
                        h303
                    }
                })?;
                let a305 = Bytes::lit(&[0x5d]);
                append_bytes(a304, a305)
            };
            append_bytes(a306, a307)
        }, v3))
    }
}

pub fn f29(v0: Adt0, v1: Bytes, v2: Adt1, v3: List<Adt1>, v4: u64) -> R<Adt0> {
    let v5: Adt3 = f22(v1, v4)?;
    if match v5.clone() {
        Adt3::C0(_, _, _, h308) => {
            h308
        }
    } {
        Ok(f30(v0, f31(v2, 1u64, match v5.clone() {
            Adt3::C0(_, h309, _, _) => {
                h309
            }
        }), v3, match v5 {
            Adt3::C0(h310, _, _, _) => {
                h310
            }
        }))
    } else {
        Ok(f25(v0, match v5 {
            Adt3::C0(_, _, h311, _) => {
                h311
            }
        }))
    }
}

pub fn f30(_: Adt0, v1: Adt1, v2: List<Adt1>, v3: u64) -> Adt0 {
    Adt0::C0(v3, List::cons(v1, v2), None::<Bytes>, Bytes::lit(&[]))
}

pub fn f31(v0: Adt1, v1: u64, v2: Bytes) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h312, _, _, _) => {
            h312
        }
    }, v1, match v0 {
        Adt1::C0(_, _, h313, _) => {
            h313
        }
    }, v2)
}

pub fn f32(v0: u32) -> Bytes {
    let a316 = {
        let a314 = v0;
        let a315 = 8u32;
        fixed_u32::bit_and(a314, a315)
    };
    let a317 = 0u32;
    if equal(a316, a317) {
        let a320 = {
            let a318 = v0;
            let a319 = 4u32;
            fixed_u32::bit_and(a318, a319)
        };
        let a321 = 0u32;
        if equal(a320, a321) {
            let a324 = {
                let a322 = v0;
                let a323 = 2u32;
                fixed_u32::bit_and(a322, a323)
            };
            let a325 = 0u32;
            if equal(a324, a325) {
                let a328 = {
                    let a326 = v0;
                    let a327 = 1u32;
                    fixed_u32::bit_and(a326, a327)
                };
                let a329 = 0u32;
                if equal(a328, a329) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a332 = {
                    let a330 = v0;
                    let a331 = 1u32;
                    fixed_u32::bit_and(a330, a331)
                };
                let a333 = 0u32;
                if equal(a332, a333) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a336 = {
                let a334 = v0;
                let a335 = 2u32;
                fixed_u32::bit_and(a334, a335)
            };
            let a337 = 0u32;
            if equal(a336, a337) {
                let a340 = {
                    let a338 = v0;
                    let a339 = 1u32;
                    fixed_u32::bit_and(a338, a339)
                };
                let a341 = 0u32;
                if equal(a340, a341) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a344 = {
                    let a342 = v0;
                    let a343 = 1u32;
                    fixed_u32::bit_and(a342, a343)
                };
                let a345 = 0u32;
                if equal(a344, a345) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a348 = {
            let a346 = v0;
            let a347 = 4u32;
            fixed_u32::bit_and(a346, a347)
        };
        let a349 = 0u32;
        if equal(a348, a349) {
            let a352 = {
                let a350 = v0;
                let a351 = 2u32;
                fixed_u32::bit_and(a350, a351)
            };
            let a353 = 0u32;
            if equal(a352, a353) {
                let a356 = {
                    let a354 = v0;
                    let a355 = 1u32;
                    fixed_u32::bit_and(a354, a355)
                };
                let a357 = 0u32;
                if equal(a356, a357) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a360 = {
                    let a358 = v0;
                    let a359 = 1u32;
                    fixed_u32::bit_and(a358, a359)
                };
                let a361 = 0u32;
                if equal(a360, a361) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a364 = {
                let a362 = v0;
                let a363 = 2u32;
                fixed_u32::bit_and(a362, a363)
            };
            let a365 = 0u32;
            if equal(a364, a365) {
                let a368 = {
                    let a366 = v0;
                    let a367 = 1u32;
                    fixed_u32::bit_and(a366, a367)
                };
                let a369 = 0u32;
                if equal(a368, a369) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a372 = {
                    let a370 = v0;
                    let a371 = 1u32;
                    fixed_u32::bit_and(a370, a371)
                };
                let a373 = 0u32;
                if equal(a372, a373) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f33(v0: u32, v1: u32) -> u32 {
    let m376 = {
        let a374 = v0;
        let a375 = v1;
        fixed_u32::shift_right(a374, a375)
    };
    match m376 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f34(v0: Adt2, v1: Adt4) -> R<Adt2> {
    let v2: Adt5 = f46(Fn5::F47, 64u64, Adt5::C0(match v0.clone() {
        Adt2::C0(h377, _, _, _, _, _, _, _) => {
            h377
        }
    }, match v0.clone() {
        Adt2::C0(_, h378, _, _, _, _, _, _) => {
            h378
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h379, _, _, _, _, _) => {
            h379
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h380, _, _, _, _) => {
            h380
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h381, _, _, _) => {
            h381
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h382, _, _) => {
            h382
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h383, _) => {
            h383
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h384) => {
            h384
        }
    }, v1, 0u64))?;
    Ok(Adt2::C0(f48(match v0.clone() {
        Adt2::C0(h385, _, _, _, _, _, _, _) => {
            h385
        }
    }, match v2.clone() {
        Adt5::C0(h386, _, _, _, _, _, _, _, _, _) => {
            h386
        }
    }), f48(match v0.clone() {
        Adt2::C0(_, h387, _, _, _, _, _, _) => {
            h387
        }
    }, match v2.clone() {
        Adt5::C0(_, h388, _, _, _, _, _, _, _, _) => {
            h388
        }
    }), f48(match v0.clone() {
        Adt2::C0(_, _, h389, _, _, _, _, _) => {
            h389
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h390, _, _, _, _, _, _, _) => {
            h390
        }
    }), f48(match v0.clone() {
        Adt2::C0(_, _, _, h391, _, _, _, _) => {
            h391
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, h392, _, _, _, _, _, _) => {
            h392
        }
    }), f48(match v0.clone() {
        Adt2::C0(_, _, _, _, h393, _, _, _) => {
            h393
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, h394, _, _, _, _, _) => {
            h394
        }
    }), f48(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h395, _, _) => {
            h395
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, h396, _, _, _, _) => {
            h396
        }
    }), f48(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h397, _) => {
            h397
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h398, _, _, _) => {
            h398
        }
    }), f48(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h399) => {
            h399
        }
    }, match v2 {
        Adt5::C0(_, _, _, _, _, _, _, h400, _, _) => {
            h400
        }
    })))
}

pub fn f35(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    Ok(Adt4::C0(f49(v0.clone(), v1, v2, v3)?, f49(v0.clone(), v1, v2, {
        let a401 = v3;
        let a402 = 4u64;
        nat_add(a401, a402)?
    })?, f49(v0.clone(), v1, v2, {
        let a403 = v3;
        let a404 = 8u64;
        nat_add(a403, a404)?
    })?, f49(v0.clone(), v1, v2, {
        let a405 = v3;
        let a406 = 12u64;
        nat_add(a405, a406)?
    })?, f49(v0.clone(), v1, v2, {
        let a407 = v3;
        let a408 = 16u64;
        nat_add(a407, a408)?
    })?, f49(v0.clone(), v1, v2, {
        let a409 = v3;
        let a410 = 20u64;
        nat_add(a409, a410)?
    })?, f49(v0.clone(), v1, v2, {
        let a411 = v3;
        let a412 = 24u64;
        nat_add(a411, a412)?
    })?, f49(v0.clone(), v1, v2, {
        let a413 = v3;
        let a414 = 28u64;
        nat_add(a413, a414)?
    })?, f49(v0.clone(), v1, v2, {
        let a415 = v3;
        let a416 = 32u64;
        nat_add(a415, a416)?
    })?, f49(v0.clone(), v1, v2, {
        let a417 = v3;
        let a418 = 36u64;
        nat_add(a417, a418)?
    })?, f49(v0.clone(), v1, v2, {
        let a419 = v3;
        let a420 = 40u64;
        nat_add(a419, a420)?
    })?, f49(v0.clone(), v1, v2, {
        let a421 = v3;
        let a422 = 44u64;
        nat_add(a421, a422)?
    })?, f49(v0.clone(), v1, v2, {
        let a423 = v3;
        let a424 = 48u64;
        nat_add(a423, a424)?
    })?, f49(v0.clone(), v1, v2, {
        let a425 = v3;
        let a426 = 52u64;
        nat_add(a425, a426)?
    })?, f49(v0.clone(), v1, v2, {
        let a427 = v3;
        let a428 = 56u64;
        nat_add(a427, a428)?
    })?, f49(v0, v1, v2, {
        let a429 = v3;
        let a430 = 60u64;
        nat_add(a429, a430)?
    })?))
}

pub fn f36(v0: Fn3, p435: u64, p436: Adt3) -> R<(Adt3, bool)> {
    let m434 = (p435, p436);
    {
        let mut m434 = m434;
        loop {
            let step = {
                let (v1, v2) = m434;
                let m431 = v1;
                if m431 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m431 - 1;
                    let m433 = {
                        let c432: Fn3 = v0.clone();
                        c432.apply(v2.clone())?
                    };
                    match m433 {
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
                Ok(next) => { m434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: Bytes, v1: Adt3) -> R<Option<Adt3>> {
    f50(v0, v1)
}

pub fn f38(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> Option<List<(Bytes, Bytes)>> {
    let v3: Adt6 = f51(Fn6::F52(v0.clone(), v1.clone()), Adt6::C0(false, false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt6::C0(_, h437, _) => {
            h437
        }
    } {
        None::<List<(Bytes, Bytes)>>
    } else {
        if match v3.clone() {
            Adt6::C0(h438, _, _) => {
                h438
            }
        } {
            Some(f53(match v3 {
                Adt6::C0(_, _, h439) => {
                    h439
                }
            }))
        } else {
            Some(f53(List::cons((v0, v1), match v3 {
                Adt6::C0(_, _, h440) => {
                    h440
                }
            })))
        }
    }
}

pub fn f39(v0: Fn4, p445: u64, p446: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m444 = (p445, p446);
    {
        let mut m444 = m444;
        loop {
            let step = {
                let (v1, v2) = m444;
                let m441 = v1;
                if m441 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m441 - 1;
                    let m443 = {
                        let c442: Fn4 = v0.clone();
                        c442.apply(v2.clone())?
                    };
                    match m443 {
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
                Ok(next) => { m444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f54(v0, v1)
}

pub fn f41(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m451 = {
        let a448 = v0;
        let a449 = v2;
        let a450 = {
            let a447 = v1.clone();
            length_bytes(a447)
        };
        slice_bytes(a448, a449, a450)
    };
    match m451 {
        None => {
            false
        }
        Some(v3) => {
            let a452 = v3;
            let a453 = v1;
            equal(a452, a453)
        }
    }
}

pub fn f42(v0: Adt0, v1: Bytes, v2: u64, v3: u64, v4: bool) -> R<Adt0> {
    let v5: u64 = f13(v1.clone(), v3)?;
    let a454 = v5;
    let a455 = 46u64;
    if nat_eq(a454, a455) {
        Ok(f25(v0, f6(Str::lit("number outside the integer subset"))))
    } else {
        let a456 = v5;
        let a457 = 101u64;
        if nat_eq(a456, a457) {
            Ok(f25(v0, f6(Str::lit("number outside the integer subset"))))
        } else {
            let a458 = v5;
            let a459 = 69u64;
            if nat_eq(a458, a459) {
                Ok(f25(v0, f6(Str::lit("number outside the integer subset"))))
            } else {
                if v4 {
                    Ok(f25(v0, f6(Str::lit("number outside the integer subset"))))
                } else {
                    Ok(f23(v0.clone(), match v0 {
                        Adt0::C0(_, h460, _, _) => {
                            h460
                        }
                    }, f55(v1, v2, {
                        let a461 = v3;
                        let a462 = v2;
                        nat_sub(a461, a462)
                    }), v3))
                }
            }
        }
    }
}

pub fn f43(v0: Bytes, v1: u64) -> R<u64> {
    let (h463, _) = f9(Fn1::F56(v0.clone()), {
        let a464 = v0;
        length_bytes(a464)
    }, v1)?;
    Ok(h463)
}

pub fn f44(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let m465 = f57(Fn7::F58, None::<Bytes>, v0)?;
    match m465 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f45(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let m466 = f57(Fn7::F59, None::<Bytes>, v0)?;
    match m466 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f46(v0: Fn5, p470: u64, p471: Adt5) -> R<Adt5> {
    let m469 = (p470, p471);
    {
        let mut m469 = m469;
        loop {
            let step = {
                let (v1, v2) = m469;
                let m467 = v1;
                if m467 == 0 {
                    Err::<(u64, Adt5), R<Adt5>>(Ok(v2))
                } else {
                    let v3 = m467 - 1;
                    Ok::<(u64, Adt5), R<Adt5>>((v3, {
                        let c468: Fn5 = v0.clone();
                        c468.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Adt5) -> R<Adt5> {
    f60(v0)
}

pub fn f48(v0: u32, v1: u32) -> u32 {
    let m474 = {
        let a472 = f61(v0);
        let a473 = f61(v1);
        fixed_u64::checked_add(a472, a473)
    };
    match m474 {
        None => {
            0u32
        }
        Some(v2) => {
            f62(v2)
        }
    }
}

pub fn f49(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a477 = {
        let a475 = v3;
        let a476 = 8u64;
        nat_add(a475, a476)?
    };
    let a478 = v2;
    if nat_eq(a477, a478) {
        Ok(f63({
            let a481 = {
                let a479 = v1;
                let a480 = 8u64;
                nat_mul(a479, a480)?
            };
            let a482 = 4294967296u64;
            let a483 = 0u64;
            nat_quot(a481, a482, a483)
        }))
    } else {
        let a486 = {
            let a484 = v3;
            let a485 = 4u64;
            nat_add(a484, a485)?
        };
        let a487 = v2;
        if nat_eq(a486, a487) {
            Ok(f63({
                let a490 = {
                    let a488 = v1;
                    let a489 = 8u64;
                    nat_mul(a488, a489)?
                };
                let a491 = 4294967296u64;
                let a492 = 0u64;
                nat_rem(a490, a491, a492)
            }))
        } else {
            Ok(f64(f65(v0.clone(), v1, v3), f65(v0.clone(), v1, {
                let a493 = v3;
                let a494 = 1u64;
                nat_add(a493, a494)?
            }), f65(v0.clone(), v1, {
                let a495 = v3;
                let a496 = 2u64;
                nat_add(a495, a496)?
            }), f65(v0, v1, {
                let a497 = v3;
                let a498 = 3u64;
                nat_add(a497, a498)?
            })))
        }
    }
}

pub fn f50(v0: Bytes, v1: Adt3) -> R<Option<Adt3>> {
    if match v1.clone() {
        Adt3::C0(_, _, _, h499) => {
            h499
        }
    } {
        Ok(None::<Adt3>)
    } else {
        let a502 = 0u64;
        let a503 = {
            let a501 = match v1.clone() {
                Adt3::C0(_, _, h500, _) => {
                    h500
                }
            };
            length_bytes(a501)
        };
        if nat_lt(a502, a503) {
            Ok(None::<Adt3>)
        } else {
            let v2: u64 = f13(v0.clone(), match v1.clone() {
                Adt3::C0(h504, _, _, _) => {
                    h504
                }
            })?;
            let a505 = v2;
            let a506 = 256u64;
            if nat_eq(a505, a506) {
                Ok(f66(v1, f6(Str::lit("unterminated string"))))
            } else {
                let a507 = v2;
                let a508 = 34u64;
                if nat_eq(a507, a508) {
                    Ok(Some(Adt3::C0({
                        let a510 = match v1.clone() {
                            Adt3::C0(h509, _, _, _) => {
                                h509
                            }
                        };
                        let a511 = 1u64;
                        nat_add(a510, a511)?
                    }, match v1 {
                        Adt3::C0(_, h512, _, _) => {
                            h512
                        }
                    }, Bytes::lit(&[]), true)))
                } else {
                    let a513 = v2;
                    let a514 = 92u64;
                    if nat_eq(a513, a514) {
                        f67(v0, v1)
                    } else {
                        let a515 = v2;
                        let a516 = 32u64;
                        if nat_lt(a515, a516) {
                            Ok(f66(v1, f6(Str::lit("control character in string"))))
                        } else {
                            let a517 = v2;
                            let a518 = 128u64;
                            if nat_lt(a517, a518) {
                                f68(v1, v2, 1u64)
                            } else {
                                f69(v0, v1, v2)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Fn6, p522: Adt6, p523: List<(Bytes, Bytes)>) -> Adt6 {
    let m521 = (p522, p523);
    {
        let mut m521 = m521;
        loop {
            let step = {
                let (v1, v2) = m521;
                let m519 = v2;
                match m519.uncons() {
                    None => {
                        Err::<(Adt6, List<(Bytes, Bytes)>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<(Bytes, Bytes)>), Adt6>(({
                            let c520: Fn6 = v0.clone();
                            c520.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Bytes, v1: Bytes, v2: Adt6, v3: (Bytes, Bytes)) -> Adt6 {
    if match v2.clone() {
        Adt6::C0(h524, _, _) => {
            h524
        }
    } {
        Adt6::C0(true, match v2.clone() {
            Adt6::C0(_, h525, _) => {
                h525
            }
        }, List::cons(v3, match v2 {
            Adt6::C0(_, _, h526) => {
                h526
            }
        }))
    } else {
        let a528 = v0.clone();
        let a529 = {
            let (h527, _) = v3.clone();
            h527
        };
        if equal(a528, a529) {
            Adt6::C0(true, true, List::cons(v3, match v2 {
                Adt6::C0(_, _, h530) => {
                    h530
                }
            }))
        } else {
            if f70(v0.clone(), {
                let (h531, _) = v3.clone();
                h531
            }) {
                Adt6::C0(true, false, List::cons(v3, List::cons((v0, v1), match v2 {
                    Adt6::C0(_, _, h532) => {
                        h532
                    }
                })))
            } else {
                Adt6::C0(false, false, List::cons(v3, match v2 {
                    Adt6::C0(_, _, h533) => {
                        h533
                    }
                }))
            }
        }
    }
}

pub fn f53(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f71(Fn8::F72, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f54(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h534, _) = v1;
        h534
    };
    let v3: u64 = f13(v0, v2)?;
    let a535 = v3;
    let a536 = 256u64;
    if nat_eq(a535, a536) {
        Ok(None::<(u64, Bytes)>)
    } else {
        Ok(Some(({
            let a537 = v2;
            let a538 = 1u64;
            nat_add(a537, a538)?
        }, {
            let a566 = {
                let (_, h539) = v1;
                h539
            };
            let a567 = {
                let a540 = v3;
                let a541 = 34u64;
                if nat_eq(a540, a541) {
                    Bytes::lit(&[0x5c, 0x22])
                } else {
                    let a542 = v3;
                    let a543 = 92u64;
                    if nat_eq(a542, a543) {
                        Bytes::lit(&[0x5c, 0x5c])
                    } else {
                        let a544 = v3;
                        let a545 = 8u64;
                        if nat_eq(a544, a545) {
                            Bytes::lit(&[0x5c, 0x62])
                        } else {
                            let a546 = v3;
                            let a547 = 9u64;
                            if nat_eq(a546, a547) {
                                Bytes::lit(&[0x5c, 0x74])
                            } else {
                                let a548 = v3;
                                let a549 = 10u64;
                                if nat_eq(a548, a549) {
                                    Bytes::lit(&[0x5c, 0x6e])
                                } else {
                                    let a550 = v3;
                                    let a551 = 12u64;
                                    if nat_eq(a550, a551) {
                                        Bytes::lit(&[0x5c, 0x66])
                                    } else {
                                        let a552 = v3;
                                        let a553 = 13u64;
                                        if nat_eq(a552, a553) {
                                            Bytes::lit(&[0x5c, 0x72])
                                        } else {
                                            let a554 = v3;
                                            let a555 = 32u64;
                                            if nat_lt(a554, a555) {
                                                let a564 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                                let a565 = {
                                                    let a562 = f55(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
                                                        let a556 = v3;
                                                        let a557 = 16u64;
                                                        let a558 = 0u64;
                                                        nat_quot(a556, a557, a558)
                                                    }, 1u64);
                                                    let a563 = f55(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
                                                        let a559 = v3;
                                                        let a560 = 16u64;
                                                        let a561 = 0u64;
                                                        nat_rem(a559, a560, a561)
                                                    }, 1u64);
                                                    append_bytes(a562, a563)
                                                };
                                                append_bytes(a564, a565)
                                            } else {
                                                f73(v3)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            append_bytes(a566, a567)
        })))
    }
}

pub fn f55(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m571 = {
        let a568 = v0;
        let a569 = v1;
        let a570 = v2;
        slice_bytes(a568, a569, a570)
    };
    match m571 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f56(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f13(v0, v1)?;
    let a572 = 47u64;
    let a573 = v2;
    if nat_lt(a572, a573) {
        let a574 = v2;
        let a575 = 58u64;
        if nat_lt(a574, a575) {
            Ok(Some({
                let a576 = v1;
                let a577 = 1u64;
                nat_add(a576, a577)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f57(v0: Fn7, p581: Option<Bytes>, p582: List<(Bytes, Bytes)>) -> R<Option<Bytes>> {
    let m580 = (p581, p582);
    {
        let mut m580 = m580;
        loop {
            let step = {
                let (v1, v2) = m580;
                let m578 = v2;
                match m578.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), R<Option<Bytes>>>(({
                            let c579: Fn7 = v0.clone();
                            c579.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Option<Bytes>, v1: (Bytes, Bytes)) -> R<Option<Bytes>> {
    let v2: Bytes = {
        let a587 = f24({
            let (h583, _) = v1;
            h583
        })?;
        let a588 = {
            let a585 = Bytes::lit(&[0x3a]);
            let a586 = {
                let (_, h584) = v1;
                h584
            };
            append_bytes(a585, a586)
        };
        append_bytes(a587, a588)
    };
    let m589 = v0;
    match m589 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            Ok(Some({
                let a592 = v3;
                let a593 = {
                    let a590 = Bytes::lit(&[0x2c]);
                    let a591 = v2;
                    append_bytes(a590, a591)
                };
                append_bytes(a592, a593)
            }))
        }
    }
}

pub fn f59(v0: Option<Bytes>, v1: (Bytes, Bytes)) -> Option<Bytes> {
    let m594 = v0;
    match m594 {
        None => {
            Some({
                let (_, h595) = v1;
                h595
            })
        }
        Some(v2) => {
            Some({
                let a599 = {
                    let (_, h596) = v1;
                    h596
                };
                let a600 = {
                    let a597 = Bytes::lit(&[0x2c]);
                    let a598 = v2;
                    append_bytes(a597, a598)
                };
                append_bytes(a599, a600)
            })
        }
    }
}

pub fn f60(v0: Adt5) -> R<Adt5> {
    let v1: u32 = f74(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, _, h601) => {
            h601
        }
    });
    let v2: u32 = f48(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h602, _, _) => {
            h602
        }
    }, f48(f75(match v0.clone() {
        Adt5::C0(_, _, _, _, h603, _, _, _, _, _) => {
            h603
        }
    }), f48(f76(match v0.clone() {
        Adt5::C0(_, _, _, _, h604, _, _, _, _, _) => {
            h604
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h605, _, _, _, _) => {
            h605
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h606, _, _, _) => {
            h606
        }
    }), f48(v1, {
        let m609 = match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, _, _, h608, _) => {
                h608
            }
        };
        match m609 {
            Adt4::C0(h607, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h607
            }
        }
    }))));
    let v3: u32 = f48(f77(match v0.clone() {
        Adt5::C0(h610, _, _, _, _, _, _, _, _, _) => {
            h610
        }
    }), f78(match v0.clone() {
        Adt5::C0(h611, _, _, _, _, _, _, _, _, _) => {
            h611
        }
    }, match v0.clone() {
        Adt5::C0(_, h612, _, _, _, _, _, _, _, _) => {
            h612
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h613, _, _, _, _, _, _, _) => {
            h613
        }
    }));
    Ok(Adt5::C0(f48(v2, v3), match v0.clone() {
        Adt5::C0(h614, _, _, _, _, _, _, _, _, _) => {
            h614
        }
    }, match v0.clone() {
        Adt5::C0(_, h615, _, _, _, _, _, _, _, _) => {
            h615
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h616, _, _, _, _, _, _, _) => {
            h616
        }
    }, f48(match v0.clone() {
        Adt5::C0(_, _, _, h617, _, _, _, _, _, _) => {
            h617
        }
    }, v2), match v0.clone() {
        Adt5::C0(_, _, _, _, h618, _, _, _, _, _) => {
            h618
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h619, _, _, _, _) => {
            h619
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h620, _, _, _) => {
            h620
        }
    }, f79(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, h621, _) => {
            h621
        }
    }), {
        let a623 = match v0 {
            Adt5::C0(_, _, _, _, _, _, _, _, _, h622) => {
                h622
            }
        };
        let a624 = 1u64;
        nat_add(a623, a624)?
    }))
}

pub fn f61(v0: u32) -> u64 {
    let m626 = {
        let a625 = v0;
        fixed_u64::convert(i128::from(a625))
    };
    match m626 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f62(v0: u64) -> u32 {
    let m630 = {
        let a629 = {
            let a627 = v0;
            let a628 = 4294967295u64;
            fixed_u64::bit_and(a627, a628)
        };
        fixed_u32::convert(i128::from(a629))
    };
    match m630 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f63(v0: u64) -> u32 {
    let (_, h631) = {
        let (_, h632) = f80(Fn9::F81, 32u64, (v0, (1u32, 0u32)));
        h632
    };
    h631
}

pub fn f64(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a637 = f82(v0, 24u32);
    let a638 = {
        let a635 = f82(v1, 16u32);
        let a636 = {
            let a633 = f82(v2, 8u32);
            let a634 = v3;
            fixed_u32::bit_or(a633, a634)
        };
        fixed_u32::bit_or(a635, a636)
    };
    fixed_u32::bit_or(a637, a638)
}

pub fn f65(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a639 = v2;
    let a640 = v1;
    if nat_lt(a639, a640) {
        let m643 = {
            let a641 = v0;
            let a642 = v2;
            index_bytes(a641, a642)
        };
        match m643 {
            None => {
                0u32
            }
            Some(v3) => {
                let m645 = {
                    let a644 = v3;
                    fixed_u32::convert(i128::from(a644))
                };
                match m645 {
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
        let a646 = v2;
        let a647 = v1;
        if nat_eq(a646, a647) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f66(v0: Adt3, v1: Bytes) -> Option<Adt3> {
    Some(Adt3::C0(match v0.clone() {
        Adt3::C0(h648, _, _, _) => {
            h648
        }
    }, match v0 {
        Adt3::C0(_, h649, _, _) => {
            h649
        }
    }, v1, false))
}

pub fn f67(v0: Bytes, v1: Adt3) -> R<Option<Adt3>> {
    let v2: u64 = match v1.clone() {
        Adt3::C0(h650, _, _, _) => {
            h650
        }
    };
    let v3: u64 = f13(v0.clone(), {
        let a651 = v2;
        let a652 = 1u64;
        nat_add(a651, a652)?
    })?;
    let v4: u64 = f83(v3);
    let a653 = v4;
    let a654 = 256u64;
    if nat_lt(a653, a654) {
        f68(v1, v4, 2u64)
    } else {
        let a655 = v3;
        let a656 = 117u64;
        if nat_eq(a655, a656) {
            let v5: u64 = f84(v0, {
                let a657 = v2;
                let a658 = 2u64;
                nat_add(a657, a658)?
            })?;
            let a659 = v5;
            let a660 = 65536u64;
            if nat_lt(a659, a660) {
                f68(v1, v5, 6u64)
            } else {
                Ok(f66(v1, f6(Str::lit("bad unicode escape"))))
            }
        } else {
            Ok(f66(v1, f6(Str::lit("bad escape"))))
        }
    }
}

pub fn f68(v0: Adt3, v1: u64, v2: u64) -> R<Option<Adt3>> {
    let m662 = f85(match v0.clone() {
        Adt3::C0(_, h661, _, _) => {
            h661
        }
    }, v1)?;
    match m662 {
        None => {
            Ok(f66(v0, f6(Str::lit("string outside the NFC subset"))))
        }
        Some(v3) => {
            Ok(Some(Adt3::C0({
                let a664 = match v0 {
                    Adt3::C0(h663, _, _, _) => {
                        h663
                    }
                };
                let a665 = v2;
                nat_add(a664, a665)?
            }, v3, Bytes::lit(&[]), false)))
        }
    }
}

pub fn f69(v0: Bytes, v1: Adt3, v2: u64) -> R<Option<Adt3>> {
    let v3: u64 = f13(v0, {
        let a667 = match v1.clone() {
            Adt3::C0(h666, _, _, _) => {
                h666
            }
        };
        let a668 = 1u64;
        nat_add(a667, a668)?
    })?;
    let a669 = 127u64;
    let a670 = v3;
    if nat_lt(a669, a670) {
        let a671 = v3;
        let a672 = 192u64;
        if nat_lt(a671, a672) {
            let a673 = v2;
            let a674 = 204u64;
            if nat_eq(a673, a674) {
                f68(v1, {
                    let a677 = 768u64;
                    let a678 = {
                        let a675 = v3;
                        let a676 = 128u64;
                        nat_sub(a675, a676)
                    };
                    nat_add(a677, a678)?
                }, 2u64)
            } else {
                let a679 = v2;
                let a680 = 194u64;
                if nat_eq(a679, a680) {
                    f68(v1, {
                        let a685 = {
                            let a681 = 2u64;
                            let a682 = 64u64;
                            nat_mul(a681, a682)?
                        };
                        let a686 = {
                            let a683 = v3;
                            let a684 = 128u64;
                            nat_sub(a683, a684)
                        };
                        nat_add(a685, a686)?
                    }, 2u64)
                } else {
                    let a687 = v2;
                    let a688 = 195u64;
                    if nat_eq(a687, a688) {
                        f68(v1, {
                            let a693 = {
                                let a689 = 3u64;
                                let a690 = 64u64;
                                nat_mul(a689, a690)?
                            };
                            let a694 = {
                                let a691 = v3;
                                let a692 = 128u64;
                                nat_sub(a691, a692)
                            };
                            nat_add(a693, a694)?
                        }, 2u64)
                    } else {
                        Ok(f66(v1, f6(Str::lit("string outside the NFC subset"))))
                    }
                }
            }
        } else {
            Ok(f66(v1, f6(Str::lit("invalid UTF-8"))))
        }
    } else {
        Ok(f66(v1, f6(Str::lit("invalid UTF-8"))))
    }
}

pub fn f70(v0: Bytes, v1: Bytes) -> bool {
    let a699 = {
        let a695 = v0;
        let a696 = v1;
        compare_bytes(a695, a696)
    };
    let a700 = {
        let a697 = Bytes::lit(&[]);
        let a698 = Bytes::lit(&[0x01]);
        compare_bytes(a697, a698)
    };
    equal(a699, a700)
}

pub fn f71(v0: Fn8, p704: List<(Bytes, Bytes)>, p705: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m703 = (p704, p705);
    {
        let mut m703 = m703;
        loop {
            let step = {
                let (v1, v2) = m703;
                let m701 = v2;
                match m701.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c702: Fn8 = v0.clone();
                            c702.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f73(v0: u64) -> Bytes {
    f55(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f74(v0: u64) -> u32 {
    let a706 = v0;
    let a707 = 32u64;
    if nat_lt(a706, a707) {
        let a708 = v0;
        let a709 = 16u64;
        if nat_lt(a708, a709) {
            let a710 = v0;
            let a711 = 8u64;
            if nat_lt(a710, a711) {
                let a712 = v0;
                let a713 = 4u64;
                if nat_lt(a712, a713) {
                    let a714 = v0;
                    let a715 = 2u64;
                    if nat_lt(a714, a715) {
                        let a716 = v0;
                        let a717 = 1u64;
                        if nat_lt(a716, a717) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a718 = v0;
                        let a719 = 3u64;
                        if nat_lt(a718, a719) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a720 = v0;
                    let a721 = 6u64;
                    if nat_lt(a720, a721) {
                        let a722 = v0;
                        let a723 = 5u64;
                        if nat_lt(a722, a723) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a724 = v0;
                        let a725 = 7u64;
                        if nat_lt(a724, a725) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a726 = v0;
                let a727 = 12u64;
                if nat_lt(a726, a727) {
                    let a728 = v0;
                    let a729 = 10u64;
                    if nat_lt(a728, a729) {
                        let a730 = v0;
                        let a731 = 9u64;
                        if nat_lt(a730, a731) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a732 = v0;
                        let a733 = 11u64;
                        if nat_lt(a732, a733) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a734 = v0;
                    let a735 = 14u64;
                    if nat_lt(a734, a735) {
                        let a736 = v0;
                        let a737 = 13u64;
                        if nat_lt(a736, a737) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a738 = v0;
                        let a739 = 15u64;
                        if nat_lt(a738, a739) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a740 = v0;
            let a741 = 24u64;
            if nat_lt(a740, a741) {
                let a742 = v0;
                let a743 = 20u64;
                if nat_lt(a742, a743) {
                    let a744 = v0;
                    let a745 = 18u64;
                    if nat_lt(a744, a745) {
                        let a746 = v0;
                        let a747 = 17u64;
                        if nat_lt(a746, a747) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a748 = v0;
                        let a749 = 19u64;
                        if nat_lt(a748, a749) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a750 = v0;
                    let a751 = 22u64;
                    if nat_lt(a750, a751) {
                        let a752 = v0;
                        let a753 = 21u64;
                        if nat_lt(a752, a753) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a754 = v0;
                        let a755 = 23u64;
                        if nat_lt(a754, a755) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a756 = v0;
                let a757 = 28u64;
                if nat_lt(a756, a757) {
                    let a758 = v0;
                    let a759 = 26u64;
                    if nat_lt(a758, a759) {
                        let a760 = v0;
                        let a761 = 25u64;
                        if nat_lt(a760, a761) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a762 = v0;
                        let a763 = 27u64;
                        if nat_lt(a762, a763) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a764 = v0;
                    let a765 = 30u64;
                    if nat_lt(a764, a765) {
                        let a766 = v0;
                        let a767 = 29u64;
                        if nat_lt(a766, a767) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a768 = v0;
                        let a769 = 31u64;
                        if nat_lt(a768, a769) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a770 = v0;
        let a771 = 48u64;
        if nat_lt(a770, a771) {
            let a772 = v0;
            let a773 = 40u64;
            if nat_lt(a772, a773) {
                let a774 = v0;
                let a775 = 36u64;
                if nat_lt(a774, a775) {
                    let a776 = v0;
                    let a777 = 34u64;
                    if nat_lt(a776, a777) {
                        let a778 = v0;
                        let a779 = 33u64;
                        if nat_lt(a778, a779) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a780 = v0;
                        let a781 = 35u64;
                        if nat_lt(a780, a781) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a782 = v0;
                    let a783 = 38u64;
                    if nat_lt(a782, a783) {
                        let a784 = v0;
                        let a785 = 37u64;
                        if nat_lt(a784, a785) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a786 = v0;
                        let a787 = 39u64;
                        if nat_lt(a786, a787) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a788 = v0;
                let a789 = 44u64;
                if nat_lt(a788, a789) {
                    let a790 = v0;
                    let a791 = 42u64;
                    if nat_lt(a790, a791) {
                        let a792 = v0;
                        let a793 = 41u64;
                        if nat_lt(a792, a793) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a794 = v0;
                        let a795 = 43u64;
                        if nat_lt(a794, a795) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a796 = v0;
                    let a797 = 46u64;
                    if nat_lt(a796, a797) {
                        let a798 = v0;
                        let a799 = 45u64;
                        if nat_lt(a798, a799) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a800 = v0;
                        let a801 = 47u64;
                        if nat_lt(a800, a801) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a802 = v0;
            let a803 = 56u64;
            if nat_lt(a802, a803) {
                let a804 = v0;
                let a805 = 52u64;
                if nat_lt(a804, a805) {
                    let a806 = v0;
                    let a807 = 50u64;
                    if nat_lt(a806, a807) {
                        let a808 = v0;
                        let a809 = 49u64;
                        if nat_lt(a808, a809) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a810 = v0;
                        let a811 = 51u64;
                        if nat_lt(a810, a811) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a812 = v0;
                    let a813 = 54u64;
                    if nat_lt(a812, a813) {
                        let a814 = v0;
                        let a815 = 53u64;
                        if nat_lt(a814, a815) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a816 = v0;
                        let a817 = 55u64;
                        if nat_lt(a816, a817) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a818 = v0;
                let a819 = 60u64;
                if nat_lt(a818, a819) {
                    let a820 = v0;
                    let a821 = 58u64;
                    if nat_lt(a820, a821) {
                        let a822 = v0;
                        let a823 = 57u64;
                        if nat_lt(a822, a823) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a824 = v0;
                        let a825 = 59u64;
                        if nat_lt(a824, a825) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a826 = v0;
                    let a827 = 62u64;
                    if nat_lt(a826, a827) {
                        let a828 = v0;
                        let a829 = 61u64;
                        if nat_lt(a828, a829) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a830 = v0;
                        let a831 = 63u64;
                        if nat_lt(a830, a831) {
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

pub fn f75(v0: u32) -> u32 {
    let a834 = {
        let a832 = f86(v0, 6u32, 26u32);
        let a833 = f86(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a832, a833)
    };
    let a835 = f86(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a834, a835)
}

pub fn f76(v0: u32, v1: u32, v2: u32) -> u32 {
    let a841 = {
        let a836 = v0;
        let a837 = v1;
        fixed_u32::bit_and(a836, a837)
    };
    let a842 = {
        let a839 = {
            let a838 = v0;
            fixed_u32::bit_not(a838)
        };
        let a840 = v2;
        fixed_u32::bit_and(a839, a840)
    };
    fixed_u32::bit_xor(a841, a842)
}

pub fn f77(v0: u32) -> u32 {
    let a845 = {
        let a843 = f86(v0, 2u32, 30u32);
        let a844 = f86(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a843, a844)
    };
    let a846 = f86(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a845, a846)
}

pub fn f78(v0: u32, v1: u32, v2: u32) -> u32 {
    let a855 = {
        let a851 = {
            let a847 = v0;
            let a848 = v1;
            fixed_u32::bit_and(a847, a848)
        };
        let a852 = {
            let a849 = v0;
            let a850 = v2;
            fixed_u32::bit_and(a849, a850)
        };
        fixed_u32::bit_xor(a851, a852)
    };
    let a856 = {
        let a853 = v1;
        let a854 = v2;
        fixed_u32::bit_and(a853, a854)
    };
    fixed_u32::bit_xor(a855, a856)
}

pub fn f79(v0: Adt4) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(_, h857, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h857
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h858, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h858
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h859, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h859
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h860, _, _, _, _, _, _, _, _, _, _, _) => {
            h860
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h861, _, _, _, _, _, _, _, _, _, _) => {
            h861
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h862, _, _, _, _, _, _, _, _, _) => {
            h862
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h863, _, _, _, _, _, _, _, _) => {
            h863
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h864, _, _, _, _, _, _, _) => {
            h864
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h865, _, _, _, _, _, _) => {
            h865
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h866, _, _, _, _, _) => {
            h866
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, h867, _, _, _, _) => {
            h867
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, h868, _, _, _) => {
            h868
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h869, _, _) => {
            h869
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h870, _) => {
            h870
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h871) => {
            h871
        }
    }, f48(f87(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h872, _) => {
            h872
        }
    }), f48(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h873, _, _, _, _, _, _) => {
            h873
        }
    }, f48(f88(match v0.clone() {
        Adt4::C0(_, h874, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h874
        }
    }), match v0 {
        Adt4::C0(h875, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h875
        }
    }))))
}

pub fn f80(v0: Fn9, p879: u64, p880: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m878 = (p879, p880);
    {
        let mut m878 = m878;
        loop {
            let step = {
                let (v1, v2) = m878;
                let m876 = v1;
                if m876 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m876 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c877: Fn9 = v0.clone();
                        c877.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m878 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h881, _) = v0;
        h881
    };
    let v2: u32 = {
        let (h882, _) = {
            let (_, h883) = v0;
            h883
        };
        h882
    };
    let v3: u32 = {
        let (_, h884) = {
            let (_, h885) = v0;
            h885
        };
        h884
    };
    ({
        let a886 = v1;
        let a887 = 2u64;
        let a888 = 0u64;
        nat_quot(a886, a887, a888)
    }, (f82(v2, 1u32), {
        let a892 = {
            let a889 = v1;
            let a890 = 2u64;
            let a891 = 0u64;
            nat_rem(a889, a890, a891)
        };
        let a893 = 1u64;
        if nat_eq(a892, a893) {
            let a894 = v3;
            let a895 = v2;
            fixed_u32::bit_or(a894, a895)
        } else {
            v3
        }
    }))
}

pub fn f82(v0: u32, v1: u32) -> u32 {
    let m898 = {
        let a896 = v0;
        let a897 = v1;
        fixed_u32::shift_left(a896, a897)
    };
    match m898 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f83(v0: u64) -> u64 {
    let a899 = v0;
    let a900 = 34u64;
    if nat_eq(a899, a900) {
        34u64
    } else {
        let a901 = v0;
        let a902 = 92u64;
        if nat_eq(a901, a902) {
            92u64
        } else {
            let a903 = v0;
            let a904 = 47u64;
            if nat_eq(a903, a904) {
                47u64
            } else {
                let a905 = v0;
                let a906 = 98u64;
                if nat_eq(a905, a906) {
                    8u64
                } else {
                    let a907 = v0;
                    let a908 = 102u64;
                    if nat_eq(a907, a908) {
                        12u64
                    } else {
                        let a909 = v0;
                        let a910 = 110u64;
                        if nat_eq(a909, a910) {
                            10u64
                        } else {
                            let a911 = v0;
                            let a912 = 114u64;
                            if nat_eq(a911, a912) {
                                13u64
                            } else {
                                let a913 = v0;
                                let a914 = 116u64;
                                if nat_eq(a913, a914) {
                                    9u64
                                } else {
                                    256u64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f89(f13(v0.clone(), v1)?);
    let v3: u64 = f89(f13(v0.clone(), {
        let a915 = v1;
        let a916 = 1u64;
        nat_add(a915, a916)?
    })?);
    let v4: u64 = f89(f13(v0.clone(), {
        let a917 = v1;
        let a918 = 2u64;
        nat_add(a917, a918)?
    })?);
    let v5: u64 = f89(f13(v0, {
        let a919 = v1;
        let a920 = 3u64;
        nat_add(a919, a920)?
    })?);
    let a921 = v2;
    let a922 = 16u64;
    if nat_lt(a921, a922) {
        let a923 = v3;
        let a924 = 16u64;
        if nat_lt(a923, a924) {
            let a925 = v4;
            let a926 = 16u64;
            if nat_lt(a925, a926) {
                let a927 = v5;
                let a928 = 16u64;
                if nat_lt(a927, a928) {
                    let a939 = {
                        let a937 = {
                            let a933 = {
                                let a929 = v2;
                                let a930 = 4096u64;
                                nat_mul(a929, a930)?
                            };
                            let a934 = {
                                let a931 = v3;
                                let a932 = 256u64;
                                nat_mul(a931, a932)?
                            };
                            nat_add(a933, a934)?
                        };
                        let a938 = {
                            let a935 = v4;
                            let a936 = 16u64;
                            nat_mul(a935, a936)?
                        };
                        nat_add(a937, a938)?
                    };
                    let a940 = v5;
                    nat_add(a939, a940)
                } else {
                    Ok(65536u64)
                }
            } else {
                Ok(65536u64)
            }
        } else {
            Ok(65536u64)
        }
    } else {
        Ok(65536u64)
    }
}

pub fn f85(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let a941 = v1;
    let a942 = 128u64;
    if nat_lt(a941, a942) {
        Ok(Some({
            let a943 = v0;
            let a944 = f73(v1);
            append_bytes(a943, a944)
        }))
    } else {
        let a945 = v1;
        let a946 = 256u64;
        if nat_lt(a945, a946) {
            Ok(Some({
                let a947 = v0;
                let a948 = f90(v1)?;
                append_bytes(a947, a948)
            }))
        } else {
            let a949 = 767u64;
            let a950 = v1;
            if nat_lt(a949, a950) {
                let a951 = v1;
                let a952 = 808u64;
                if nat_lt(a951, a952) {
                    let v2: u64 = {
                        let a953 = v0.clone();
                        length_bytes(a953)
                    };
                    let v3: u64 = {
                        let a954 = 0u64;
                        let a955 = v2;
                        if nat_lt(a954, a955) {
                            f13(v0.clone(), {
                                let a956 = v2;
                                let a957 = 1u64;
                                nat_sub(a956, a957)
                            })?
                        } else {
                            256u64
                        }
                    };
                    let v4: u64 = {
                        let a958 = v3;
                        let a959 = 128u64;
                        if nat_lt(a958, a959) {
                            f91({
                                let a960 = v1;
                                let a961 = 768u64;
                                nat_sub(a960, a961)
                            }, v3)?
                        } else {
                            0u64
                        }
                    };
                    let a962 = 0u64;
                    let a963 = v4;
                    if nat_lt(a962, a963) {
                        Ok(Some({
                            let a966 = f55(v0, 0u64, {
                                let a964 = v2;
                                let a965 = 1u64;
                                nat_sub(a964, a965)
                            });
                            let a967 = f90(v4)?;
                            append_bytes(a966, a967)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f86(v0: u32, v1: u32, v2: u32) -> u32 {
    let a968 = f33(v0, v1);
    let a969 = f82(v0, v2);
    fixed_u32::bit_or(a968, a969)
}

pub fn f87(v0: u32) -> u32 {
    let a972 = {
        let a970 = f86(v0, 17u32, 15u32);
        let a971 = f86(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a970, a971)
    };
    let a973 = f33(v0, 10u32);
    fixed_u32::bit_xor(a972, a973)
}

pub fn f88(v0: u32) -> u32 {
    let a976 = {
        let a974 = f86(v0, 7u32, 25u32);
        let a975 = f86(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a974, a975)
    };
    let a977 = f33(v0, 3u32);
    fixed_u32::bit_xor(a976, a977)
}

pub fn f89(v0: u64) -> u64 {
    let a978 = 47u64;
    let a979 = v0;
    if nat_lt(a978, a979) {
        let a980 = v0;
        let a981 = 58u64;
        if nat_lt(a980, a981) {
            let a982 = v0;
            let a983 = 48u64;
            nat_sub(a982, a983)
        } else {
            let a984 = 64u64;
            let a985 = v0;
            if nat_lt(a984, a985) {
                let a986 = v0;
                let a987 = 71u64;
                if nat_lt(a986, a987) {
                    let a988 = v0;
                    let a989 = 55u64;
                    nat_sub(a988, a989)
                } else {
                    let a990 = 96u64;
                    let a991 = v0;
                    if nat_lt(a990, a991) {
                        let a992 = v0;
                        let a993 = 103u64;
                        if nat_lt(a992, a993) {
                            let a994 = v0;
                            let a995 = 87u64;
                            nat_sub(a994, a995)
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

pub fn f90(v0: u64) -> R<Bytes> {
    let a1006 = f73({
        let a999 = 192u64;
        let a1000 = {
            let a996 = v0;
            let a997 = 64u64;
            let a998 = 0u64;
            nat_quot(a996, a997, a998)
        };
        nat_add(a999, a1000)?
    });
    let a1007 = f73({
        let a1004 = 128u64;
        let a1005 = {
            let a1001 = v0;
            let a1002 = 64u64;
            let a1003 = 0u64;
            nat_rem(a1001, a1002, a1003)
        };
        nat_add(a1004, a1005)?
    });
    Ok(append_bytes(a1006, a1007))
}

pub fn f91(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f92(v0, v1)?;
    let a1008 = v2;
    let a1009 = 53u64;
    if nat_lt(a1008, a1009) {
        f13(Bytes::lit(&[0x00, 0x41, 0xc0, 0x00, 0x45, 0xc8, 0x00, 0x49, 0xcc, 0x00, 0x4f, 0xd2, 0x00, 0x55, 0xd9, 0x00, 0x61, 0xe0, 0x00, 0x65, 0xe8, 0x00, 0x69, 0xec, 0x00, 0x6f, 0xf2, 0x00, 0x75, 0xf9, 0x01, 0x41, 0xc1, 0x01, 0x45, 0xc9, 0x01, 0x49, 0xcd, 0x01, 0x4f, 0xd3, 0x01, 0x55, 0xda, 0x01, 0x59, 0xdd, 0x01, 0x61, 0xe1, 0x01, 0x65, 0xe9, 0x01, 0x69, 0xed, 0x01, 0x6f, 0xf3, 0x01, 0x75, 0xfa, 0x01, 0x79, 0xfd, 0x02, 0x41, 0xc2, 0x02, 0x45, 0xca, 0x02, 0x49, 0xce, 0x02, 0x4f, 0xd4, 0x02, 0x55, 0xdb, 0x02, 0x61, 0xe2, 0x02, 0x65, 0xea, 0x02, 0x69, 0xee, 0x02, 0x6f, 0xf4, 0x02, 0x75, 0xfb, 0x03, 0x41, 0xc3, 0x03, 0x4e, 0xd1, 0x03, 0x4f, 0xd5, 0x03, 0x61, 0xe3, 0x03, 0x6e, 0xf1, 0x03, 0x6f, 0xf5, 0x08, 0x41, 0xc4, 0x08, 0x45, 0xcb, 0x08, 0x49, 0xcf, 0x08, 0x4f, 0xd6, 0x08, 0x55, 0xdc, 0x08, 0x61, 0xe4, 0x08, 0x65, 0xeb, 0x08, 0x69, 0xef, 0x08, 0x6f, 0xf6, 0x08, 0x75, 0xfc, 0x08, 0x79, 0xff, 0x0a, 0x41, 0xc5, 0x0a, 0x61, 0xe5, 0x27, 0x43, 0xc7, 0x27, 0x63, 0xe7]), {
            let a1012 = {
                let a1010 = v2;
                let a1011 = 3u64;
                nat_mul(a1010, a1011)?
            };
            let a1013 = 2u64;
            nat_add(a1012, a1013)?
        })
    } else {
        Ok(0u64)
    }
}

pub fn f92(v0: u64, v1: u64) -> R<u64> {
    let (h1014, _) = f9(Fn1::F93(v1, v0), 54u64, 0u64)?;
    Ok(h1014)
}

pub fn f93(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: Bytes = Bytes::lit(&[0x00, 0x41, 0xc0, 0x00, 0x45, 0xc8, 0x00, 0x49, 0xcc, 0x00, 0x4f, 0xd2, 0x00, 0x55, 0xd9, 0x00, 0x61, 0xe0, 0x00, 0x65, 0xe8, 0x00, 0x69, 0xec, 0x00, 0x6f, 0xf2, 0x00, 0x75, 0xf9, 0x01, 0x41, 0xc1, 0x01, 0x45, 0xc9, 0x01, 0x49, 0xcd, 0x01, 0x4f, 0xd3, 0x01, 0x55, 0xda, 0x01, 0x59, 0xdd, 0x01, 0x61, 0xe1, 0x01, 0x65, 0xe9, 0x01, 0x69, 0xed, 0x01, 0x6f, 0xf3, 0x01, 0x75, 0xfa, 0x01, 0x79, 0xfd, 0x02, 0x41, 0xc2, 0x02, 0x45, 0xca, 0x02, 0x49, 0xce, 0x02, 0x4f, 0xd4, 0x02, 0x55, 0xdb, 0x02, 0x61, 0xe2, 0x02, 0x65, 0xea, 0x02, 0x69, 0xee, 0x02, 0x6f, 0xf4, 0x02, 0x75, 0xfb, 0x03, 0x41, 0xc3, 0x03, 0x4e, 0xd1, 0x03, 0x4f, 0xd5, 0x03, 0x61, 0xe3, 0x03, 0x6e, 0xf1, 0x03, 0x6f, 0xf5, 0x08, 0x41, 0xc4, 0x08, 0x45, 0xcb, 0x08, 0x49, 0xcf, 0x08, 0x4f, 0xd6, 0x08, 0x55, 0xdc, 0x08, 0x61, 0xe4, 0x08, 0x65, 0xeb, 0x08, 0x69, 0xef, 0x08, 0x6f, 0xf6, 0x08, 0x75, 0xfc, 0x08, 0x79, 0xff, 0x0a, 0x41, 0xc5, 0x0a, 0x61, 0xe5, 0x27, 0x43, 0xc7, 0x27, 0x63, 0xe7]);
    let a1015 = v2;
    let a1016 = 53u64;
    if nat_lt(a1015, a1016) {
        let a1019 = f13(v3.clone(), {
            let a1017 = v2;
            let a1018 = 3u64;
            nat_mul(a1017, a1018)?
        })?;
        let a1020 = v1;
        if nat_eq(a1019, a1020) {
            let a1025 = f13(v3, {
                let a1023 = {
                    let a1021 = v2;
                    let a1022 = 3u64;
                    nat_mul(a1021, a1022)?
                };
                let a1024 = 1u64;
                nat_add(a1023, a1024)?
            })?;
            let a1026 = v0;
            if nat_eq(a1025, a1026) {
                Ok(None::<u64>)
            } else {
                Ok(Some({
                    let a1027 = v2;
                    let a1028 = 1u64;
                    nat_add(a1027, a1028)?
                }))
            }
        } else {
            Ok(Some({
                let a1029 = v2;
                let a1030 = 1u64;
                nat_add(a1029, a1030)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}
