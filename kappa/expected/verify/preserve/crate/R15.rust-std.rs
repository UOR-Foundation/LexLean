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
    F7(Bytes, u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn0::F7(k0, k1, k2) => f7(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn1::F14 => f14(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F30,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn2::F30 => f30(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Bytes> {
    let a1 = f1(Str::lit("sha256:"));
    let a2 = f2(v0)?;
    Ok(append_bytes(a1, a2))
}

pub fn f1(v0: Str) -> Bytes {
    let a3 = v0;
    utf8_encode(a3)
}

pub fn f2(v0: Bytes) -> R<Bytes> {
    Ok(f3(f4(v0)?))
}

pub fn f3(v0: Adt0) -> Bytes {
    let a24 = f5(match v0.clone() {
        Adt0::C0(h4, _, _, _, _, _, _, _) => {
            h4
        }
    });
    let a25 = {
        let a22 = f5(match v0.clone() {
            Adt0::C0(_, h5, _, _, _, _, _, _) => {
                h5
            }
        });
        let a23 = {
            let a20 = f5(match v0.clone() {
                Adt0::C0(_, _, h6, _, _, _, _, _) => {
                    h6
                }
            });
            let a21 = {
                let a18 = f5(match v0.clone() {
                    Adt0::C0(_, _, _, h7, _, _, _, _) => {
                        h7
                    }
                });
                let a19 = {
                    let a16 = f5(match v0.clone() {
                        Adt0::C0(_, _, _, _, h8, _, _, _) => {
                            h8
                        }
                    });
                    let a17 = {
                        let a14 = f5(match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h9, _, _) => {
                                h9
                            }
                        });
                        let a15 = {
                            let a12 = f5(match v0.clone() {
                                Adt0::C0(_, _, _, _, _, _, h10, _) => {
                                    h10
                                }
                            });
                            let a13 = f5(match v0 {
                                Adt0::C0(_, _, _, _, _, _, _, h11) => {
                                    h11
                                }
                            });
                            append_bytes(a12, a13)
                        };
                        append_bytes(a14, a15)
                    };
                    append_bytes(a16, a17)
                };
                append_bytes(a18, a19)
            };
            append_bytes(a20, a21)
        };
        append_bytes(a22, a23)
    };
    append_bytes(a24, a25)
}

pub fn f4(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a26 = v0.clone();
        length_bytes(a26)
    };
    let v2: u64 = {
        let a32 = {
            let a29 = {
                let a27 = v1;
                let a28 = 72u64;
                nat_add(a27, a28)?
            };
            let a30 = 64u64;
            let a31 = 0u64;
            nat_quot(a29, a30, a31)
        };
        let a33 = 64u64;
        nat_mul(a32, a33)?
    };
    let (_, h34) = f6(Fn0::F7(v0, v1, v2), {
        let a35 = v2;
        let a36 = 64u64;
        let a37 = 0u64;
        nat_quot(a35, a36, a37)
    }, (0u64, f8()))?;
    Ok(h34)
}

pub fn f5(v0: u32) -> Bytes {
    let a66 = f9({
        let a38 = f10(v0, 28u32);
        let a39 = 15u32;
        fixed_u32::bit_and(a38, a39)
    });
    let a67 = {
        let a64 = f9({
            let a40 = f10(v0, 24u32);
            let a41 = 15u32;
            fixed_u32::bit_and(a40, a41)
        });
        let a65 = {
            let a62 = f9({
                let a42 = f10(v0, 20u32);
                let a43 = 15u32;
                fixed_u32::bit_and(a42, a43)
            });
            let a63 = {
                let a60 = f9({
                    let a44 = f10(v0, 16u32);
                    let a45 = 15u32;
                    fixed_u32::bit_and(a44, a45)
                });
                let a61 = {
                    let a58 = f9({
                        let a46 = f10(v0, 12u32);
                        let a47 = 15u32;
                        fixed_u32::bit_and(a46, a47)
                    });
                    let a59 = {
                        let a56 = f9({
                            let a48 = f10(v0, 8u32);
                            let a49 = 15u32;
                            fixed_u32::bit_and(a48, a49)
                        });
                        let a57 = {
                            let a54 = f9({
                                let a50 = f10(v0, 4u32);
                                let a51 = 15u32;
                                fixed_u32::bit_and(a50, a51)
                            });
                            let a55 = f9({
                                let a52 = v0;
                                let a53 = 15u32;
                                fixed_u32::bit_and(a52, a53)
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

pub fn f6(v0: Fn0, p71: u64, p72: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m70 = (p71, p72);
    {
        let mut m70 = m70;
        loop {
            let step = {
                let (v1, v2) = m70;
                let m68 = v1;
                if m68 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m68 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c69: Fn0 = v0.clone();
                        c69.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m70 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a74 = {
            let (h73, _) = v3.clone();
            h73
        };
        let a75 = 64u64;
        nat_add(a74, a75)?
    }, f11({
        let (_, h76) = v3;
        h76
    }, f12(v0, v1, v2, {
        let (h77, _) = v3;
        h77
    })?)?))
}

pub fn f8() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f9(v0: u32) -> Bytes {
    let a80 = {
        let a78 = v0;
        let a79 = 8u32;
        fixed_u32::bit_and(a78, a79)
    };
    let a81 = 0u32;
    if equal(a80, a81) {
        let a84 = {
            let a82 = v0;
            let a83 = 4u32;
            fixed_u32::bit_and(a82, a83)
        };
        let a85 = 0u32;
        if equal(a84, a85) {
            let a88 = {
                let a86 = v0;
                let a87 = 2u32;
                fixed_u32::bit_and(a86, a87)
            };
            let a89 = 0u32;
            if equal(a88, a89) {
                let a92 = {
                    let a90 = v0;
                    let a91 = 1u32;
                    fixed_u32::bit_and(a90, a91)
                };
                let a93 = 0u32;
                if equal(a92, a93) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a96 = {
                    let a94 = v0;
                    let a95 = 1u32;
                    fixed_u32::bit_and(a94, a95)
                };
                let a97 = 0u32;
                if equal(a96, a97) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a100 = {
                let a98 = v0;
                let a99 = 2u32;
                fixed_u32::bit_and(a98, a99)
            };
            let a101 = 0u32;
            if equal(a100, a101) {
                let a104 = {
                    let a102 = v0;
                    let a103 = 1u32;
                    fixed_u32::bit_and(a102, a103)
                };
                let a105 = 0u32;
                if equal(a104, a105) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a108 = {
                    let a106 = v0;
                    let a107 = 1u32;
                    fixed_u32::bit_and(a106, a107)
                };
                let a109 = 0u32;
                if equal(a108, a109) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a112 = {
            let a110 = v0;
            let a111 = 4u32;
            fixed_u32::bit_and(a110, a111)
        };
        let a113 = 0u32;
        if equal(a112, a113) {
            let a116 = {
                let a114 = v0;
                let a115 = 2u32;
                fixed_u32::bit_and(a114, a115)
            };
            let a117 = 0u32;
            if equal(a116, a117) {
                let a120 = {
                    let a118 = v0;
                    let a119 = 1u32;
                    fixed_u32::bit_and(a118, a119)
                };
                let a121 = 0u32;
                if equal(a120, a121) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a124 = {
                    let a122 = v0;
                    let a123 = 1u32;
                    fixed_u32::bit_and(a122, a123)
                };
                let a125 = 0u32;
                if equal(a124, a125) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a128 = {
                let a126 = v0;
                let a127 = 2u32;
                fixed_u32::bit_and(a126, a127)
            };
            let a129 = 0u32;
            if equal(a128, a129) {
                let a132 = {
                    let a130 = v0;
                    let a131 = 1u32;
                    fixed_u32::bit_and(a130, a131)
                };
                let a133 = 0u32;
                if equal(a132, a133) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a136 = {
                    let a134 = v0;
                    let a135 = 1u32;
                    fixed_u32::bit_and(a134, a135)
                };
                let a137 = 0u32;
                if equal(a136, a137) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f10(v0: u32, v1: u32) -> u32 {
    let m140 = {
        let a138 = v0;
        let a139 = v1;
        fixed_u32::shift_right(a138, a139)
    };
    match m140 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f11(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f13(Fn1::F14, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h141, _, _, _, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, h142, _, _, _, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h143, _, _, _, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h144, _, _, _, _) => {
            h144
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h145, _, _, _) => {
            h145
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h146, _, _) => {
            h146
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h147, _) => {
            h147
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h148) => {
            h148
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f15(match v0.clone() {
        Adt0::C0(h149, _, _, _, _, _, _, _) => {
            h149
        }
    }, match v2.clone() {
        Adt2::C0(h150, _, _, _, _, _, _, _, _, _) => {
            h150
        }
    }), f15(match v0.clone() {
        Adt0::C0(_, h151, _, _, _, _, _, _) => {
            h151
        }
    }, match v2.clone() {
        Adt2::C0(_, h152, _, _, _, _, _, _, _, _) => {
            h152
        }
    }), f15(match v0.clone() {
        Adt0::C0(_, _, h153, _, _, _, _, _) => {
            h153
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h154, _, _, _, _, _, _, _) => {
            h154
        }
    }), f15(match v0.clone() {
        Adt0::C0(_, _, _, h155, _, _, _, _) => {
            h155
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h156, _, _, _, _, _, _) => {
            h156
        }
    }), f15(match v0.clone() {
        Adt0::C0(_, _, _, _, h157, _, _, _) => {
            h157
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h158, _, _, _, _, _) => {
            h158
        }
    }), f15(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h159, _, _) => {
            h159
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h160, _, _, _, _) => {
            h160
        }
    }), f15(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h161, _) => {
            h161
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h162, _, _, _) => {
            h162
        }
    }), f15(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h163) => {
            h163
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h164, _, _) => {
            h164
        }
    })))
}

pub fn f12(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f16(v0.clone(), v1, v2, v3)?, f16(v0.clone(), v1, v2, {
        let a165 = v3;
        let a166 = 4u64;
        nat_add(a165, a166)?
    })?, f16(v0.clone(), v1, v2, {
        let a167 = v3;
        let a168 = 8u64;
        nat_add(a167, a168)?
    })?, f16(v0.clone(), v1, v2, {
        let a169 = v3;
        let a170 = 12u64;
        nat_add(a169, a170)?
    })?, f16(v0.clone(), v1, v2, {
        let a171 = v3;
        let a172 = 16u64;
        nat_add(a171, a172)?
    })?, f16(v0.clone(), v1, v2, {
        let a173 = v3;
        let a174 = 20u64;
        nat_add(a173, a174)?
    })?, f16(v0.clone(), v1, v2, {
        let a175 = v3;
        let a176 = 24u64;
        nat_add(a175, a176)?
    })?, f16(v0.clone(), v1, v2, {
        let a177 = v3;
        let a178 = 28u64;
        nat_add(a177, a178)?
    })?, f16(v0.clone(), v1, v2, {
        let a179 = v3;
        let a180 = 32u64;
        nat_add(a179, a180)?
    })?, f16(v0.clone(), v1, v2, {
        let a181 = v3;
        let a182 = 36u64;
        nat_add(a181, a182)?
    })?, f16(v0.clone(), v1, v2, {
        let a183 = v3;
        let a184 = 40u64;
        nat_add(a183, a184)?
    })?, f16(v0.clone(), v1, v2, {
        let a185 = v3;
        let a186 = 44u64;
        nat_add(a185, a186)?
    })?, f16(v0.clone(), v1, v2, {
        let a187 = v3;
        let a188 = 48u64;
        nat_add(a187, a188)?
    })?, f16(v0.clone(), v1, v2, {
        let a189 = v3;
        let a190 = 52u64;
        nat_add(a189, a190)?
    })?, f16(v0.clone(), v1, v2, {
        let a191 = v3;
        let a192 = 56u64;
        nat_add(a191, a192)?
    })?, f16(v0, v1, v2, {
        let a193 = v3;
        let a194 = 60u64;
        nat_add(a193, a194)?
    })?))
}

pub fn f13(v0: Fn1, p198: u64, p199: Adt2) -> R<Adt2> {
    let m197 = (p198, p199);
    {
        let mut m197 = m197;
        loop {
            let step = {
                let (v1, v2) = m197;
                let m195 = v1;
                if m195 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m195 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c196: Fn1 = v0.clone();
                        c196.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Adt2) -> R<Adt2> {
    f17(v0)
}

pub fn f15(v0: u32, v1: u32) -> u32 {
    let m202 = {
        let a200 = f18(v0);
        let a201 = f18(v1);
        fixed_u64::checked_add(a200, a201)
    };
    match m202 {
        None => {
            0u32
        }
        Some(v2) => {
            f19(v2)
        }
    }
}

pub fn f16(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a205 = {
        let a203 = v3;
        let a204 = 8u64;
        nat_add(a203, a204)?
    };
    let a206 = v2;
    if nat_eq(a205, a206) {
        Ok(f20({
            let a209 = {
                let a207 = v1;
                let a208 = 8u64;
                nat_mul(a207, a208)?
            };
            let a210 = 4294967296u64;
            let a211 = 0u64;
            nat_quot(a209, a210, a211)
        }))
    } else {
        let a214 = {
            let a212 = v3;
            let a213 = 4u64;
            nat_add(a212, a213)?
        };
        let a215 = v2;
        if nat_eq(a214, a215) {
            Ok(f20({
                let a218 = {
                    let a216 = v1;
                    let a217 = 8u64;
                    nat_mul(a216, a217)?
                };
                let a219 = 4294967296u64;
                let a220 = 0u64;
                nat_rem(a218, a219, a220)
            }))
        } else {
            Ok(f21(f22(v0.clone(), v1, v3), f22(v0.clone(), v1, {
                let a221 = v3;
                let a222 = 1u64;
                nat_add(a221, a222)?
            }), f22(v0.clone(), v1, {
                let a223 = v3;
                let a224 = 2u64;
                nat_add(a223, a224)?
            }), f22(v0, v1, {
                let a225 = v3;
                let a226 = 3u64;
                nat_add(a225, a226)?
            })))
        }
    }
}

pub fn f17(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f23(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h227) => {
            h227
        }
    });
    let v2: u32 = f15(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h228, _, _) => {
            h228
        }
    }, f15(f24(match v0.clone() {
        Adt2::C0(_, _, _, _, h229, _, _, _, _, _) => {
            h229
        }
    }), f15(f25(match v0.clone() {
        Adt2::C0(_, _, _, _, h230, _, _, _, _, _) => {
            h230
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h231, _, _, _, _) => {
            h231
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h232, _, _, _) => {
            h232
        }
    }), f15(v1, {
        let m235 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h234, _) => {
                h234
            }
        };
        match m235 {
            Adt1::C0(h233, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h233
            }
        }
    }))));
    let v3: u32 = f15(f26(match v0.clone() {
        Adt2::C0(h236, _, _, _, _, _, _, _, _, _) => {
            h236
        }
    }), f27(match v0.clone() {
        Adt2::C0(h237, _, _, _, _, _, _, _, _, _) => {
            h237
        }
    }, match v0.clone() {
        Adt2::C0(_, h238, _, _, _, _, _, _, _, _) => {
            h238
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h239, _, _, _, _, _, _, _) => {
            h239
        }
    }));
    Ok(Adt2::C0(f15(v2, v3), match v0.clone() {
        Adt2::C0(h240, _, _, _, _, _, _, _, _, _) => {
            h240
        }
    }, match v0.clone() {
        Adt2::C0(_, h241, _, _, _, _, _, _, _, _) => {
            h241
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h242, _, _, _, _, _, _, _) => {
            h242
        }
    }, f15(match v0.clone() {
        Adt2::C0(_, _, _, h243, _, _, _, _, _, _) => {
            h243
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h244, _, _, _, _, _) => {
            h244
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h245, _, _, _, _) => {
            h245
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h246, _, _, _) => {
            h246
        }
    }, f28(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h247, _) => {
            h247
        }
    }), {
        let a249 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h248) => {
                h248
            }
        };
        let a250 = 1u64;
        nat_add(a249, a250)?
    }))
}

pub fn f18(v0: u32) -> u64 {
    let m252 = {
        let a251 = v0;
        fixed_u64::convert(i128::from(a251))
    };
    match m252 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f19(v0: u64) -> u32 {
    let m256 = {
        let a255 = {
            let a253 = v0;
            let a254 = 4294967295u64;
            fixed_u64::bit_and(a253, a254)
        };
        fixed_u32::convert(i128::from(a255))
    };
    match m256 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f20(v0: u64) -> u32 {
    let (_, h257) = {
        let (_, h258) = f29(Fn2::F30, 32u64, (v0, (1u32, 0u32)));
        h258
    };
    h257
}

pub fn f21(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a263 = f31(v0, 24u32);
    let a264 = {
        let a261 = f31(v1, 16u32);
        let a262 = {
            let a259 = f31(v2, 8u32);
            let a260 = v3;
            fixed_u32::bit_or(a259, a260)
        };
        fixed_u32::bit_or(a261, a262)
    };
    fixed_u32::bit_or(a263, a264)
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a265 = v2;
    let a266 = v1;
    if nat_lt(a265, a266) {
        let m269 = {
            let a267 = v0;
            let a268 = v2;
            index_bytes(a267, a268)
        };
        match m269 {
            None => {
                0u32
            }
            Some(v3) => {
                let m271 = {
                    let a270 = v3;
                    fixed_u32::convert(i128::from(a270))
                };
                match m271 {
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
        let a272 = v2;
        let a273 = v1;
        if nat_eq(a272, a273) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f23(v0: u64) -> u32 {
    let a274 = v0;
    let a275 = 32u64;
    if nat_lt(a274, a275) {
        let a276 = v0;
        let a277 = 16u64;
        if nat_lt(a276, a277) {
            let a278 = v0;
            let a279 = 8u64;
            if nat_lt(a278, a279) {
                let a280 = v0;
                let a281 = 4u64;
                if nat_lt(a280, a281) {
                    let a282 = v0;
                    let a283 = 2u64;
                    if nat_lt(a282, a283) {
                        let a284 = v0;
                        let a285 = 1u64;
                        if nat_lt(a284, a285) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a286 = v0;
                        let a287 = 3u64;
                        if nat_lt(a286, a287) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a288 = v0;
                    let a289 = 6u64;
                    if nat_lt(a288, a289) {
                        let a290 = v0;
                        let a291 = 5u64;
                        if nat_lt(a290, a291) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a292 = v0;
                        let a293 = 7u64;
                        if nat_lt(a292, a293) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a294 = v0;
                let a295 = 12u64;
                if nat_lt(a294, a295) {
                    let a296 = v0;
                    let a297 = 10u64;
                    if nat_lt(a296, a297) {
                        let a298 = v0;
                        let a299 = 9u64;
                        if nat_lt(a298, a299) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a300 = v0;
                        let a301 = 11u64;
                        if nat_lt(a300, a301) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a302 = v0;
                    let a303 = 14u64;
                    if nat_lt(a302, a303) {
                        let a304 = v0;
                        let a305 = 13u64;
                        if nat_lt(a304, a305) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a306 = v0;
                        let a307 = 15u64;
                        if nat_lt(a306, a307) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a308 = v0;
            let a309 = 24u64;
            if nat_lt(a308, a309) {
                let a310 = v0;
                let a311 = 20u64;
                if nat_lt(a310, a311) {
                    let a312 = v0;
                    let a313 = 18u64;
                    if nat_lt(a312, a313) {
                        let a314 = v0;
                        let a315 = 17u64;
                        if nat_lt(a314, a315) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a316 = v0;
                        let a317 = 19u64;
                        if nat_lt(a316, a317) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a318 = v0;
                    let a319 = 22u64;
                    if nat_lt(a318, a319) {
                        let a320 = v0;
                        let a321 = 21u64;
                        if nat_lt(a320, a321) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a322 = v0;
                        let a323 = 23u64;
                        if nat_lt(a322, a323) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a324 = v0;
                let a325 = 28u64;
                if nat_lt(a324, a325) {
                    let a326 = v0;
                    let a327 = 26u64;
                    if nat_lt(a326, a327) {
                        let a328 = v0;
                        let a329 = 25u64;
                        if nat_lt(a328, a329) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a330 = v0;
                        let a331 = 27u64;
                        if nat_lt(a330, a331) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a332 = v0;
                    let a333 = 30u64;
                    if nat_lt(a332, a333) {
                        let a334 = v0;
                        let a335 = 29u64;
                        if nat_lt(a334, a335) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a336 = v0;
                        let a337 = 31u64;
                        if nat_lt(a336, a337) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a338 = v0;
        let a339 = 48u64;
        if nat_lt(a338, a339) {
            let a340 = v0;
            let a341 = 40u64;
            if nat_lt(a340, a341) {
                let a342 = v0;
                let a343 = 36u64;
                if nat_lt(a342, a343) {
                    let a344 = v0;
                    let a345 = 34u64;
                    if nat_lt(a344, a345) {
                        let a346 = v0;
                        let a347 = 33u64;
                        if nat_lt(a346, a347) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a348 = v0;
                        let a349 = 35u64;
                        if nat_lt(a348, a349) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a350 = v0;
                    let a351 = 38u64;
                    if nat_lt(a350, a351) {
                        let a352 = v0;
                        let a353 = 37u64;
                        if nat_lt(a352, a353) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a354 = v0;
                        let a355 = 39u64;
                        if nat_lt(a354, a355) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a356 = v0;
                let a357 = 44u64;
                if nat_lt(a356, a357) {
                    let a358 = v0;
                    let a359 = 42u64;
                    if nat_lt(a358, a359) {
                        let a360 = v0;
                        let a361 = 41u64;
                        if nat_lt(a360, a361) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a362 = v0;
                        let a363 = 43u64;
                        if nat_lt(a362, a363) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a364 = v0;
                    let a365 = 46u64;
                    if nat_lt(a364, a365) {
                        let a366 = v0;
                        let a367 = 45u64;
                        if nat_lt(a366, a367) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a368 = v0;
                        let a369 = 47u64;
                        if nat_lt(a368, a369) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a370 = v0;
            let a371 = 56u64;
            if nat_lt(a370, a371) {
                let a372 = v0;
                let a373 = 52u64;
                if nat_lt(a372, a373) {
                    let a374 = v0;
                    let a375 = 50u64;
                    if nat_lt(a374, a375) {
                        let a376 = v0;
                        let a377 = 49u64;
                        if nat_lt(a376, a377) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a378 = v0;
                        let a379 = 51u64;
                        if nat_lt(a378, a379) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a380 = v0;
                    let a381 = 54u64;
                    if nat_lt(a380, a381) {
                        let a382 = v0;
                        let a383 = 53u64;
                        if nat_lt(a382, a383) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a384 = v0;
                        let a385 = 55u64;
                        if nat_lt(a384, a385) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a386 = v0;
                let a387 = 60u64;
                if nat_lt(a386, a387) {
                    let a388 = v0;
                    let a389 = 58u64;
                    if nat_lt(a388, a389) {
                        let a390 = v0;
                        let a391 = 57u64;
                        if nat_lt(a390, a391) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a392 = v0;
                        let a393 = 59u64;
                        if nat_lt(a392, a393) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a394 = v0;
                    let a395 = 62u64;
                    if nat_lt(a394, a395) {
                        let a396 = v0;
                        let a397 = 61u64;
                        if nat_lt(a396, a397) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a398 = v0;
                        let a399 = 63u64;
                        if nat_lt(a398, a399) {
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

pub fn f24(v0: u32) -> u32 {
    let a402 = {
        let a400 = f32(v0, 6u32, 26u32);
        let a401 = f32(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a400, a401)
    };
    let a403 = f32(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a402, a403)
}

pub fn f25(v0: u32, v1: u32, v2: u32) -> u32 {
    let a409 = {
        let a404 = v0;
        let a405 = v1;
        fixed_u32::bit_and(a404, a405)
    };
    let a410 = {
        let a407 = {
            let a406 = v0;
            fixed_u32::bit_not(a406)
        };
        let a408 = v2;
        fixed_u32::bit_and(a407, a408)
    };
    fixed_u32::bit_xor(a409, a410)
}

pub fn f26(v0: u32) -> u32 {
    let a413 = {
        let a411 = f32(v0, 2u32, 30u32);
        let a412 = f32(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a411, a412)
    };
    let a414 = f32(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a413, a414)
}

pub fn f27(v0: u32, v1: u32, v2: u32) -> u32 {
    let a423 = {
        let a419 = {
            let a415 = v0;
            let a416 = v1;
            fixed_u32::bit_and(a415, a416)
        };
        let a420 = {
            let a417 = v0;
            let a418 = v2;
            fixed_u32::bit_and(a417, a418)
        };
        fixed_u32::bit_xor(a419, a420)
    };
    let a424 = {
        let a421 = v1;
        let a422 = v2;
        fixed_u32::bit_and(a421, a422)
    };
    fixed_u32::bit_xor(a423, a424)
}

pub fn f28(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h425, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h425
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h426, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h426
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h427, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h427
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h428, _, _, _, _, _, _, _, _, _, _, _) => {
            h428
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h429, _, _, _, _, _, _, _, _, _, _) => {
            h429
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h430, _, _, _, _, _, _, _, _, _) => {
            h430
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h431, _, _, _, _, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h432, _, _, _, _, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h433, _, _, _, _, _, _) => {
            h433
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h434, _, _, _, _, _) => {
            h434
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h435, _, _, _, _) => {
            h435
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h436, _, _, _) => {
            h436
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h437, _, _) => {
            h437
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h438, _) => {
            h438
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h439) => {
            h439
        }
    }, f15(f33(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h440, _) => {
            h440
        }
    }), f15(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h441, _, _, _, _, _, _) => {
            h441
        }
    }, f15(f34(match v0.clone() {
        Adt1::C0(_, h442, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h442
        }
    }), match v0 {
        Adt1::C0(h443, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h443
        }
    }))))
}

pub fn f29(v0: Fn2, p447: u64, p448: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m446 = (p447, p448);
    {
        let mut m446 = m446;
        loop {
            let step = {
                let (v1, v2) = m446;
                let m444 = v1;
                if m444 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m444 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c445: Fn2 = v0.clone();
                        c445.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h449, _) = v0;
        h449
    };
    let v2: u32 = {
        let (h450, _) = {
            let (_, h451) = v0;
            h451
        };
        h450
    };
    let v3: u32 = {
        let (_, h452) = {
            let (_, h453) = v0;
            h453
        };
        h452
    };
    ({
        let a454 = v1;
        let a455 = 2u64;
        let a456 = 0u64;
        nat_quot(a454, a455, a456)
    }, (f31(v2, 1u32), {
        let a460 = {
            let a457 = v1;
            let a458 = 2u64;
            let a459 = 0u64;
            nat_rem(a457, a458, a459)
        };
        let a461 = 1u64;
        if nat_eq(a460, a461) {
            let a462 = v3;
            let a463 = v2;
            fixed_u32::bit_or(a462, a463)
        } else {
            v3
        }
    }))
}

pub fn f31(v0: u32, v1: u32) -> u32 {
    let m466 = {
        let a464 = v0;
        let a465 = v1;
        fixed_u32::shift_left(a464, a465)
    };
    match m466 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f32(v0: u32, v1: u32, v2: u32) -> u32 {
    let a467 = f10(v0, v1);
    let a468 = f31(v0, v2);
    fixed_u32::bit_or(a467, a468)
}

pub fn f33(v0: u32) -> u32 {
    let a471 = {
        let a469 = f32(v0, 17u32, 15u32);
        let a470 = f32(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a469, a470)
    };
    let a472 = f10(v0, 10u32);
    fixed_u32::bit_xor(a471, a472)
}

pub fn f34(v0: u32) -> u32 {
    let a475 = {
        let a473 = f32(v0, 7u32, 25u32);
        let a474 = f32(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a473, a474)
    };
    let a476 = f10(v0, 3u32);
    fixed_u32::bit_xor(a475, a476)
}
