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
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt2, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F8(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F8(k0) => f8(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt1)) -> R<(u64, Adt1)> {
        match self {
            Fn1::F14(k0, k1, k2) => f14(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F22,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt3) -> R<Adt3> {
        match self {
            Fn2::F22 => f22(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F38,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn3::F38 => f38(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m1 = f1(v0, v1)?;
    match m1 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a6 = match v2.clone() {
                    Adt0::C0(h2, _) => {
                        h2
                    }
                };
                let a7 = {
                    let a4 = f2(Str::lit("@"));
                    let a5 = match v2 {
                        Adt0::C0(_, h3) => {
                            h3
                        }
                    };
                    append_bytes(a4, a5)
                };
                append_bytes(a6, a7)
            }))
        }
    }
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Option<Adt0>> {
    if f3(v0.clone())? {
        Ok(Some(Adt0::C0(v0, f4(v1)?)))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f2(v0: Str) -> Bytes {
    let a8 = v0;
    utf8_encode(a8)
}

pub fn f3(v0: Bytes) -> R<bool> {
    let a10 = 0u64;
    let a11 = {
        let a9 = v0.clone();
        length_bytes(a9)
    };
    if nat_lt(a10, a11) {
        let a13 = f5(v0.clone())?;
        let a14 = {
            let a12 = v0;
            length_bytes(a12)
        };
        Ok(nat_eq(a13, a14))
    } else {
        Ok(false)
    }
}

pub fn f4(v0: Bytes) -> R<Bytes> {
    let a15 = f2(Str::lit("sha256:"));
    let a16 = f6(v0)?;
    Ok(append_bytes(a15, a16))
}

pub fn f5(v0: Bytes) -> R<u64> {
    let (h17, _) = f7(Fn0::F8(v0.clone()), {
        let a18 = v0;
        length_bytes(a18)
    }, 0u64)?;
    Ok(h17)
}

pub fn f6(v0: Bytes) -> R<Bytes> {
    Ok(f9(f10(v0)?))
}

pub fn f7(v0: Fn0, p23: u64, p24: u64) -> R<(u64, bool)> {
    let m22 = (p23, p24);
    {
        let mut m22 = m22;
        loop {
            let step = {
                let (v1, v2) = m22;
                let m19 = v1;
                if m19 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m19 - 1;
                    let m21 = {
                        let c20: Fn0 = v0.clone();
                        c20.apply(v2)?
                    };
                    match m21 {
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
                Ok(next) => { m22 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f11(v0, v1)? {
        Ok(Some({
            let a25 = v1;
            let a26 = 1u64;
            nat_add(a25, a26)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f9(v0: Adt1) -> Bytes {
    let a47 = f12(match v0.clone() {
        Adt1::C0(h27, _, _, _, _, _, _, _) => {
            h27
        }
    });
    let a48 = {
        let a45 = f12(match v0.clone() {
            Adt1::C0(_, h28, _, _, _, _, _, _) => {
                h28
            }
        });
        let a46 = {
            let a43 = f12(match v0.clone() {
                Adt1::C0(_, _, h29, _, _, _, _, _) => {
                    h29
                }
            });
            let a44 = {
                let a41 = f12(match v0.clone() {
                    Adt1::C0(_, _, _, h30, _, _, _, _) => {
                        h30
                    }
                });
                let a42 = {
                    let a39 = f12(match v0.clone() {
                        Adt1::C0(_, _, _, _, h31, _, _, _) => {
                            h31
                        }
                    });
                    let a40 = {
                        let a37 = f12(match v0.clone() {
                            Adt1::C0(_, _, _, _, _, h32, _, _) => {
                                h32
                            }
                        });
                        let a38 = {
                            let a35 = f12(match v0.clone() {
                                Adt1::C0(_, _, _, _, _, _, h33, _) => {
                                    h33
                                }
                            });
                            let a36 = f12(match v0 {
                                Adt1::C0(_, _, _, _, _, _, _, h34) => {
                                    h34
                                }
                            });
                            append_bytes(a35, a36)
                        };
                        append_bytes(a37, a38)
                    };
                    append_bytes(a39, a40)
                };
                append_bytes(a41, a42)
            };
            append_bytes(a43, a44)
        };
        append_bytes(a45, a46)
    };
    append_bytes(a47, a48)
}

pub fn f10(v0: Bytes) -> R<Adt1> {
    let v1: u64 = {
        let a49 = v0.clone();
        length_bytes(a49)
    };
    let v2: u64 = {
        let a55 = {
            let a52 = {
                let a50 = v1;
                let a51 = 72u64;
                nat_add(a50, a51)?
            };
            let a53 = 64u64;
            let a54 = 0u64;
            nat_quot(a52, a53, a54)
        };
        let a56 = 64u64;
        nat_mul(a55, a56)?
    };
    let (_, h57) = f13(Fn1::F14(v0, v1, v2), {
        let a58 = v2;
        let a59 = 64u64;
        let a60 = 0u64;
        nat_quot(a58, a59, a60)
    }, (0u64, f15()))?;
    Ok(h57)
}

pub fn f11(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f16(v0, v1)?;
    let a61 = 96u64;
    let a62 = v2;
    if nat_lt(a61, a62) {
        let a63 = v2;
        let a64 = 123u64;
        Ok(nat_lt(a63, a64))
    } else {
        let a65 = 47u64;
        let a66 = v2;
        if nat_lt(a65, a66) {
            let a67 = v2;
            let a68 = 58u64;
            Ok(nat_lt(a67, a68))
        } else {
            let a69 = v2;
            let a70 = 45u64;
            Ok(nat_eq(a69, a70))
        }
    }
}

pub fn f12(v0: u32) -> Bytes {
    let a99 = f17({
        let a71 = f18(v0, 28u32);
        let a72 = 15u32;
        fixed_u32::bit_and(a71, a72)
    });
    let a100 = {
        let a97 = f17({
            let a73 = f18(v0, 24u32);
            let a74 = 15u32;
            fixed_u32::bit_and(a73, a74)
        });
        let a98 = {
            let a95 = f17({
                let a75 = f18(v0, 20u32);
                let a76 = 15u32;
                fixed_u32::bit_and(a75, a76)
            });
            let a96 = {
                let a93 = f17({
                    let a77 = f18(v0, 16u32);
                    let a78 = 15u32;
                    fixed_u32::bit_and(a77, a78)
                });
                let a94 = {
                    let a91 = f17({
                        let a79 = f18(v0, 12u32);
                        let a80 = 15u32;
                        fixed_u32::bit_and(a79, a80)
                    });
                    let a92 = {
                        let a89 = f17({
                            let a81 = f18(v0, 8u32);
                            let a82 = 15u32;
                            fixed_u32::bit_and(a81, a82)
                        });
                        let a90 = {
                            let a87 = f17({
                                let a83 = f18(v0, 4u32);
                                let a84 = 15u32;
                                fixed_u32::bit_and(a83, a84)
                            });
                            let a88 = f17({
                                let a85 = v0;
                                let a86 = 15u32;
                                fixed_u32::bit_and(a85, a86)
                            });
                            append_bytes(a87, a88)
                        };
                        append_bytes(a89, a90)
                    };
                    append_bytes(a91, a92)
                };
                append_bytes(a93, a94)
            };
            append_bytes(a95, a96)
        };
        append_bytes(a97, a98)
    };
    append_bytes(a99, a100)
}

pub fn f13(v0: Fn1, p104: u64, p105: (u64, Adt1)) -> R<(u64, Adt1)> {
    let m103 = (p104, p105);
    {
        let mut m103 = m103;
        loop {
            let step = {
                let (v1, v2) = m103;
                let m101 = v1;
                if m101 == 0 {
                    Err::<(u64, (u64, Adt1)), R<(u64, Adt1)>>(Ok(v2))
                } else {
                    let v3 = m101 - 1;
                    Ok::<(u64, (u64, Adt1)), R<(u64, Adt1)>>((v3, {
                        let c102: Fn1 = v0.clone();
                        c102.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt1)) -> R<(u64, Adt1)> {
    Ok(({
        let a107 = {
            let (h106, _) = v3.clone();
            h106
        };
        let a108 = 64u64;
        nat_add(a107, a108)?
    }, f19({
        let (_, h109) = v3;
        h109
    }, f20(v0, v1, v2, {
        let (h110, _) = v3;
        h110
    })?)?))
}

pub fn f15() -> Adt1 {
    Adt1::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f16(v0: Bytes, v1: u64) -> R<u64> {
    let m113 = {
        let a111 = v0;
        let a112 = v1;
        index_bytes(a111, a112)
    };
    match m113 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a158 = {
                let a134 = {
                    let a122 = {
                        let a116 = {
                            let a114 = v3;
                            let a115 = 128u8;
                            fixed_u8::bit_and(a114, a115)
                        };
                        let a117 = 128u8;
                        if equal(a116, a117) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a123 = {
                        let a120 = {
                            let a118 = v3;
                            let a119 = 64u8;
                            fixed_u8::bit_and(a118, a119)
                        };
                        let a121 = 64u8;
                        if equal(a120, a121) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a122, a123)?
                };
                let a135 = {
                    let a132 = {
                        let a126 = {
                            let a124 = v3;
                            let a125 = 32u8;
                            fixed_u8::bit_and(a124, a125)
                        };
                        let a127 = 32u8;
                        if equal(a126, a127) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a133 = {
                        let a130 = {
                            let a128 = v3;
                            let a129 = 16u8;
                            fixed_u8::bit_and(a128, a129)
                        };
                        let a131 = 16u8;
                        if equal(a130, a131) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a132, a133)?
                };
                nat_add(a134, a135)?
            };
            let a159 = {
                let a156 = {
                    let a144 = {
                        let a138 = {
                            let a136 = v3;
                            let a137 = 8u8;
                            fixed_u8::bit_and(a136, a137)
                        };
                        let a139 = 8u8;
                        if equal(a138, a139) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a145 = {
                        let a142 = {
                            let a140 = v3;
                            let a141 = 4u8;
                            fixed_u8::bit_and(a140, a141)
                        };
                        let a143 = 4u8;
                        if equal(a142, a143) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a144, a145)?
                };
                let a157 = {
                    let a154 = {
                        let a148 = {
                            let a146 = v3;
                            let a147 = 2u8;
                            fixed_u8::bit_and(a146, a147)
                        };
                        let a149 = 2u8;
                        if equal(a148, a149) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a155 = {
                        let a152 = {
                            let a150 = v3;
                            let a151 = 1u8;
                            fixed_u8::bit_and(a150, a151)
                        };
                        let a153 = 1u8;
                        if equal(a152, a153) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a154, a155)?
                };
                nat_add(a156, a157)?
            };
            nat_add(a158, a159)
        }
    }
}

pub fn f17(v0: u32) -> Bytes {
    let a162 = {
        let a160 = v0;
        let a161 = 8u32;
        fixed_u32::bit_and(a160, a161)
    };
    let a163 = 0u32;
    if equal(a162, a163) {
        let a166 = {
            let a164 = v0;
            let a165 = 4u32;
            fixed_u32::bit_and(a164, a165)
        };
        let a167 = 0u32;
        if equal(a166, a167) {
            let a170 = {
                let a168 = v0;
                let a169 = 2u32;
                fixed_u32::bit_and(a168, a169)
            };
            let a171 = 0u32;
            if equal(a170, a171) {
                let a174 = {
                    let a172 = v0;
                    let a173 = 1u32;
                    fixed_u32::bit_and(a172, a173)
                };
                let a175 = 0u32;
                if equal(a174, a175) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a178 = {
                    let a176 = v0;
                    let a177 = 1u32;
                    fixed_u32::bit_and(a176, a177)
                };
                let a179 = 0u32;
                if equal(a178, a179) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a182 = {
                let a180 = v0;
                let a181 = 2u32;
                fixed_u32::bit_and(a180, a181)
            };
            let a183 = 0u32;
            if equal(a182, a183) {
                let a186 = {
                    let a184 = v0;
                    let a185 = 1u32;
                    fixed_u32::bit_and(a184, a185)
                };
                let a187 = 0u32;
                if equal(a186, a187) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a190 = {
                    let a188 = v0;
                    let a189 = 1u32;
                    fixed_u32::bit_and(a188, a189)
                };
                let a191 = 0u32;
                if equal(a190, a191) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a194 = {
            let a192 = v0;
            let a193 = 4u32;
            fixed_u32::bit_and(a192, a193)
        };
        let a195 = 0u32;
        if equal(a194, a195) {
            let a198 = {
                let a196 = v0;
                let a197 = 2u32;
                fixed_u32::bit_and(a196, a197)
            };
            let a199 = 0u32;
            if equal(a198, a199) {
                let a202 = {
                    let a200 = v0;
                    let a201 = 1u32;
                    fixed_u32::bit_and(a200, a201)
                };
                let a203 = 0u32;
                if equal(a202, a203) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a206 = {
                    let a204 = v0;
                    let a205 = 1u32;
                    fixed_u32::bit_and(a204, a205)
                };
                let a207 = 0u32;
                if equal(a206, a207) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a210 = {
                let a208 = v0;
                let a209 = 2u32;
                fixed_u32::bit_and(a208, a209)
            };
            let a211 = 0u32;
            if equal(a210, a211) {
                let a214 = {
                    let a212 = v0;
                    let a213 = 1u32;
                    fixed_u32::bit_and(a212, a213)
                };
                let a215 = 0u32;
                if equal(a214, a215) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a218 = {
                    let a216 = v0;
                    let a217 = 1u32;
                    fixed_u32::bit_and(a216, a217)
                };
                let a219 = 0u32;
                if equal(a218, a219) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f18(v0: u32, v1: u32) -> u32 {
    let m222 = {
        let a220 = v0;
        let a221 = v1;
        fixed_u32::shift_right(a220, a221)
    };
    match m222 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f19(v0: Adt1, v1: Adt2) -> R<Adt1> {
    let v2: Adt3 = f21(Fn2::F22, 64u64, Adt3::C0(match v0.clone() {
        Adt1::C0(h223, _, _, _, _, _, _, _) => {
            h223
        }
    }, match v0.clone() {
        Adt1::C0(_, h224, _, _, _, _, _, _) => {
            h224
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h225, _, _, _, _, _) => {
            h225
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h226, _, _, _, _) => {
            h226
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h227, _, _, _) => {
            h227
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h228, _, _) => {
            h228
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h229, _) => {
            h229
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h230) => {
            h230
        }
    }, v1, 0u64))?;
    Ok(Adt1::C0(f23(match v0.clone() {
        Adt1::C0(h231, _, _, _, _, _, _, _) => {
            h231
        }
    }, match v2.clone() {
        Adt3::C0(h232, _, _, _, _, _, _, _, _, _) => {
            h232
        }
    }), f23(match v0.clone() {
        Adt1::C0(_, h233, _, _, _, _, _, _) => {
            h233
        }
    }, match v2.clone() {
        Adt3::C0(_, h234, _, _, _, _, _, _, _, _) => {
            h234
        }
    }), f23(match v0.clone() {
        Adt1::C0(_, _, h235, _, _, _, _, _) => {
            h235
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h236, _, _, _, _, _, _, _) => {
            h236
        }
    }), f23(match v0.clone() {
        Adt1::C0(_, _, _, h237, _, _, _, _) => {
            h237
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h238, _, _, _, _, _, _) => {
            h238
        }
    }), f23(match v0.clone() {
        Adt1::C0(_, _, _, _, h239, _, _, _) => {
            h239
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h240, _, _, _, _, _) => {
            h240
        }
    }), f23(match v0.clone() {
        Adt1::C0(_, _, _, _, _, h241, _, _) => {
            h241
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h242, _, _, _, _) => {
            h242
        }
    }), f23(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h243, _) => {
            h243
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h244, _, _, _) => {
            h244
        }
    }), f23(match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h245) => {
            h245
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h246, _, _) => {
            h246
        }
    })))
}

pub fn f20(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    Ok(Adt2::C0(f24(v0.clone(), v1, v2, v3)?, f24(v0.clone(), v1, v2, {
        let a247 = v3;
        let a248 = 4u64;
        nat_add(a247, a248)?
    })?, f24(v0.clone(), v1, v2, {
        let a249 = v3;
        let a250 = 8u64;
        nat_add(a249, a250)?
    })?, f24(v0.clone(), v1, v2, {
        let a251 = v3;
        let a252 = 12u64;
        nat_add(a251, a252)?
    })?, f24(v0.clone(), v1, v2, {
        let a253 = v3;
        let a254 = 16u64;
        nat_add(a253, a254)?
    })?, f24(v0.clone(), v1, v2, {
        let a255 = v3;
        let a256 = 20u64;
        nat_add(a255, a256)?
    })?, f24(v0.clone(), v1, v2, {
        let a257 = v3;
        let a258 = 24u64;
        nat_add(a257, a258)?
    })?, f24(v0.clone(), v1, v2, {
        let a259 = v3;
        let a260 = 28u64;
        nat_add(a259, a260)?
    })?, f24(v0.clone(), v1, v2, {
        let a261 = v3;
        let a262 = 32u64;
        nat_add(a261, a262)?
    })?, f24(v0.clone(), v1, v2, {
        let a263 = v3;
        let a264 = 36u64;
        nat_add(a263, a264)?
    })?, f24(v0.clone(), v1, v2, {
        let a265 = v3;
        let a266 = 40u64;
        nat_add(a265, a266)?
    })?, f24(v0.clone(), v1, v2, {
        let a267 = v3;
        let a268 = 44u64;
        nat_add(a267, a268)?
    })?, f24(v0.clone(), v1, v2, {
        let a269 = v3;
        let a270 = 48u64;
        nat_add(a269, a270)?
    })?, f24(v0.clone(), v1, v2, {
        let a271 = v3;
        let a272 = 52u64;
        nat_add(a271, a272)?
    })?, f24(v0.clone(), v1, v2, {
        let a273 = v3;
        let a274 = 56u64;
        nat_add(a273, a274)?
    })?, f24(v0, v1, v2, {
        let a275 = v3;
        let a276 = 60u64;
        nat_add(a275, a276)?
    })?))
}

pub fn f21(v0: Fn2, p280: u64, p281: Adt3) -> R<Adt3> {
    let m279 = (p280, p281);
    {
        let mut m279 = m279;
        loop {
            let step = {
                let (v1, v2) = m279;
                let m277 = v1;
                if m277 == 0 {
                    Err::<(u64, Adt3), R<Adt3>>(Ok(v2))
                } else {
                    let v3 = m277 - 1;
                    Ok::<(u64, Adt3), R<Adt3>>((v3, {
                        let c278: Fn2 = v0.clone();
                        c278.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m279 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Adt3) -> R<Adt3> {
    f25(v0)
}

pub fn f23(v0: u32, v1: u32) -> u32 {
    let m284 = {
        let a282 = f26(v0);
        let a283 = f26(v1);
        fixed_u64::checked_add(a282, a283)
    };
    match m284 {
        None => {
            0u32
        }
        Some(v2) => {
            f27(v2)
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a287 = {
        let a285 = v3;
        let a286 = 8u64;
        nat_add(a285, a286)?
    };
    let a288 = v2;
    if nat_eq(a287, a288) {
        Ok(f28({
            let a291 = {
                let a289 = v1;
                let a290 = 8u64;
                nat_mul(a289, a290)?
            };
            let a292 = 4294967296u64;
            let a293 = 0u64;
            nat_quot(a291, a292, a293)
        }))
    } else {
        let a296 = {
            let a294 = v3;
            let a295 = 4u64;
            nat_add(a294, a295)?
        };
        let a297 = v2;
        if nat_eq(a296, a297) {
            Ok(f28({
                let a300 = {
                    let a298 = v1;
                    let a299 = 8u64;
                    nat_mul(a298, a299)?
                };
                let a301 = 4294967296u64;
                let a302 = 0u64;
                nat_rem(a300, a301, a302)
            }))
        } else {
            Ok(f29(f30(v0.clone(), v1, v3), f30(v0.clone(), v1, {
                let a303 = v3;
                let a304 = 1u64;
                nat_add(a303, a304)?
            }), f30(v0.clone(), v1, {
                let a305 = v3;
                let a306 = 2u64;
                nat_add(a305, a306)?
            }), f30(v0, v1, {
                let a307 = v3;
                let a308 = 3u64;
                nat_add(a307, a308)?
            })))
        }
    }
}

pub fn f25(v0: Adt3) -> R<Adt3> {
    let v1: u32 = f31(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h309) => {
            h309
        }
    });
    let v2: u32 = f23(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h310, _, _) => {
            h310
        }
    }, f23(f32(match v0.clone() {
        Adt3::C0(_, _, _, _, h311, _, _, _, _, _) => {
            h311
        }
    }), f23(f33(match v0.clone() {
        Adt3::C0(_, _, _, _, h312, _, _, _, _, _) => {
            h312
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h313, _, _, _, _) => {
            h313
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h314, _, _, _) => {
            h314
        }
    }), f23(v1, {
        let m317 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h316, _) => {
                h316
            }
        };
        match m317 {
            Adt2::C0(h315, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h315
            }
        }
    }))));
    let v3: u32 = f23(f34(match v0.clone() {
        Adt3::C0(h318, _, _, _, _, _, _, _, _, _) => {
            h318
        }
    }), f35(match v0.clone() {
        Adt3::C0(h319, _, _, _, _, _, _, _, _, _) => {
            h319
        }
    }, match v0.clone() {
        Adt3::C0(_, h320, _, _, _, _, _, _, _, _) => {
            h320
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h321, _, _, _, _, _, _, _) => {
            h321
        }
    }));
    Ok(Adt3::C0(f23(v2, v3), match v0.clone() {
        Adt3::C0(h322, _, _, _, _, _, _, _, _, _) => {
            h322
        }
    }, match v0.clone() {
        Adt3::C0(_, h323, _, _, _, _, _, _, _, _) => {
            h323
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h324, _, _, _, _, _, _, _) => {
            h324
        }
    }, f23(match v0.clone() {
        Adt3::C0(_, _, _, h325, _, _, _, _, _, _) => {
            h325
        }
    }, v2), match v0.clone() {
        Adt3::C0(_, _, _, _, h326, _, _, _, _, _) => {
            h326
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h327, _, _, _, _) => {
            h327
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h328, _, _, _) => {
            h328
        }
    }, f36(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h329, _) => {
            h329
        }
    }), {
        let a331 = match v0 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h330) => {
                h330
            }
        };
        let a332 = 1u64;
        nat_add(a331, a332)?
    }))
}

pub fn f26(v0: u32) -> u64 {
    let m334 = {
        let a333 = v0;
        fixed_u64::convert(i128::from(a333))
    };
    match m334 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f27(v0: u64) -> u32 {
    let m338 = {
        let a337 = {
            let a335 = v0;
            let a336 = 4294967295u64;
            fixed_u64::bit_and(a335, a336)
        };
        fixed_u32::convert(i128::from(a337))
    };
    match m338 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f28(v0: u64) -> u32 {
    let (_, h339) = {
        let (_, h340) = f37(Fn3::F38, 32u64, (v0, (1u32, 0u32)));
        h340
    };
    h339
}

pub fn f29(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a345 = f39(v0, 24u32);
    let a346 = {
        let a343 = f39(v1, 16u32);
        let a344 = {
            let a341 = f39(v2, 8u32);
            let a342 = v3;
            fixed_u32::bit_or(a341, a342)
        };
        fixed_u32::bit_or(a343, a344)
    };
    fixed_u32::bit_or(a345, a346)
}

pub fn f30(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a347 = v2;
    let a348 = v1;
    if nat_lt(a347, a348) {
        let m351 = {
            let a349 = v0;
            let a350 = v2;
            index_bytes(a349, a350)
        };
        match m351 {
            None => {
                0u32
            }
            Some(v3) => {
                let m353 = {
                    let a352 = v3;
                    fixed_u32::convert(i128::from(a352))
                };
                match m353 {
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
        let a354 = v2;
        let a355 = v1;
        if nat_eq(a354, a355) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f31(v0: u64) -> u32 {
    let a356 = v0;
    let a357 = 32u64;
    if nat_lt(a356, a357) {
        let a358 = v0;
        let a359 = 16u64;
        if nat_lt(a358, a359) {
            let a360 = v0;
            let a361 = 8u64;
            if nat_lt(a360, a361) {
                let a362 = v0;
                let a363 = 4u64;
                if nat_lt(a362, a363) {
                    let a364 = v0;
                    let a365 = 2u64;
                    if nat_lt(a364, a365) {
                        let a366 = v0;
                        let a367 = 1u64;
                        if nat_lt(a366, a367) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a368 = v0;
                        let a369 = 3u64;
                        if nat_lt(a368, a369) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a370 = v0;
                    let a371 = 6u64;
                    if nat_lt(a370, a371) {
                        let a372 = v0;
                        let a373 = 5u64;
                        if nat_lt(a372, a373) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a374 = v0;
                        let a375 = 7u64;
                        if nat_lt(a374, a375) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a376 = v0;
                let a377 = 12u64;
                if nat_lt(a376, a377) {
                    let a378 = v0;
                    let a379 = 10u64;
                    if nat_lt(a378, a379) {
                        let a380 = v0;
                        let a381 = 9u64;
                        if nat_lt(a380, a381) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a382 = v0;
                        let a383 = 11u64;
                        if nat_lt(a382, a383) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a384 = v0;
                    let a385 = 14u64;
                    if nat_lt(a384, a385) {
                        let a386 = v0;
                        let a387 = 13u64;
                        if nat_lt(a386, a387) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a388 = v0;
                        let a389 = 15u64;
                        if nat_lt(a388, a389) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a390 = v0;
            let a391 = 24u64;
            if nat_lt(a390, a391) {
                let a392 = v0;
                let a393 = 20u64;
                if nat_lt(a392, a393) {
                    let a394 = v0;
                    let a395 = 18u64;
                    if nat_lt(a394, a395) {
                        let a396 = v0;
                        let a397 = 17u64;
                        if nat_lt(a396, a397) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a398 = v0;
                        let a399 = 19u64;
                        if nat_lt(a398, a399) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a400 = v0;
                    let a401 = 22u64;
                    if nat_lt(a400, a401) {
                        let a402 = v0;
                        let a403 = 21u64;
                        if nat_lt(a402, a403) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a404 = v0;
                        let a405 = 23u64;
                        if nat_lt(a404, a405) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a406 = v0;
                let a407 = 28u64;
                if nat_lt(a406, a407) {
                    let a408 = v0;
                    let a409 = 26u64;
                    if nat_lt(a408, a409) {
                        let a410 = v0;
                        let a411 = 25u64;
                        if nat_lt(a410, a411) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a412 = v0;
                        let a413 = 27u64;
                        if nat_lt(a412, a413) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a414 = v0;
                    let a415 = 30u64;
                    if nat_lt(a414, a415) {
                        let a416 = v0;
                        let a417 = 29u64;
                        if nat_lt(a416, a417) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a418 = v0;
                        let a419 = 31u64;
                        if nat_lt(a418, a419) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a420 = v0;
        let a421 = 48u64;
        if nat_lt(a420, a421) {
            let a422 = v0;
            let a423 = 40u64;
            if nat_lt(a422, a423) {
                let a424 = v0;
                let a425 = 36u64;
                if nat_lt(a424, a425) {
                    let a426 = v0;
                    let a427 = 34u64;
                    if nat_lt(a426, a427) {
                        let a428 = v0;
                        let a429 = 33u64;
                        if nat_lt(a428, a429) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a430 = v0;
                        let a431 = 35u64;
                        if nat_lt(a430, a431) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a432 = v0;
                    let a433 = 38u64;
                    if nat_lt(a432, a433) {
                        let a434 = v0;
                        let a435 = 37u64;
                        if nat_lt(a434, a435) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a436 = v0;
                        let a437 = 39u64;
                        if nat_lt(a436, a437) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a438 = v0;
                let a439 = 44u64;
                if nat_lt(a438, a439) {
                    let a440 = v0;
                    let a441 = 42u64;
                    if nat_lt(a440, a441) {
                        let a442 = v0;
                        let a443 = 41u64;
                        if nat_lt(a442, a443) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a444 = v0;
                        let a445 = 43u64;
                        if nat_lt(a444, a445) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a446 = v0;
                    let a447 = 46u64;
                    if nat_lt(a446, a447) {
                        let a448 = v0;
                        let a449 = 45u64;
                        if nat_lt(a448, a449) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a450 = v0;
                        let a451 = 47u64;
                        if nat_lt(a450, a451) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a452 = v0;
            let a453 = 56u64;
            if nat_lt(a452, a453) {
                let a454 = v0;
                let a455 = 52u64;
                if nat_lt(a454, a455) {
                    let a456 = v0;
                    let a457 = 50u64;
                    if nat_lt(a456, a457) {
                        let a458 = v0;
                        let a459 = 49u64;
                        if nat_lt(a458, a459) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a460 = v0;
                        let a461 = 51u64;
                        if nat_lt(a460, a461) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a462 = v0;
                    let a463 = 54u64;
                    if nat_lt(a462, a463) {
                        let a464 = v0;
                        let a465 = 53u64;
                        if nat_lt(a464, a465) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a466 = v0;
                        let a467 = 55u64;
                        if nat_lt(a466, a467) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a468 = v0;
                let a469 = 60u64;
                if nat_lt(a468, a469) {
                    let a470 = v0;
                    let a471 = 58u64;
                    if nat_lt(a470, a471) {
                        let a472 = v0;
                        let a473 = 57u64;
                        if nat_lt(a472, a473) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a474 = v0;
                        let a475 = 59u64;
                        if nat_lt(a474, a475) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a476 = v0;
                    let a477 = 62u64;
                    if nat_lt(a476, a477) {
                        let a478 = v0;
                        let a479 = 61u64;
                        if nat_lt(a478, a479) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a480 = v0;
                        let a481 = 63u64;
                        if nat_lt(a480, a481) {
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

pub fn f32(v0: u32) -> u32 {
    let a484 = {
        let a482 = f40(v0, 6u32, 26u32);
        let a483 = f40(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a482, a483)
    };
    let a485 = f40(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a484, a485)
}

pub fn f33(v0: u32, v1: u32, v2: u32) -> u32 {
    let a491 = {
        let a486 = v0;
        let a487 = v1;
        fixed_u32::bit_and(a486, a487)
    };
    let a492 = {
        let a489 = {
            let a488 = v0;
            fixed_u32::bit_not(a488)
        };
        let a490 = v2;
        fixed_u32::bit_and(a489, a490)
    };
    fixed_u32::bit_xor(a491, a492)
}

pub fn f34(v0: u32) -> u32 {
    let a495 = {
        let a493 = f40(v0, 2u32, 30u32);
        let a494 = f40(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a493, a494)
    };
    let a496 = f40(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a495, a496)
}

pub fn f35(v0: u32, v1: u32, v2: u32) -> u32 {
    let a505 = {
        let a501 = {
            let a497 = v0;
            let a498 = v1;
            fixed_u32::bit_and(a497, a498)
        };
        let a502 = {
            let a499 = v0;
            let a500 = v2;
            fixed_u32::bit_and(a499, a500)
        };
        fixed_u32::bit_xor(a501, a502)
    };
    let a506 = {
        let a503 = v1;
        let a504 = v2;
        fixed_u32::bit_and(a503, a504)
    };
    fixed_u32::bit_xor(a505, a506)
}

pub fn f36(v0: Adt2) -> Adt2 {
    Adt2::C0(match v0.clone() {
        Adt2::C0(_, h507, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h507
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h508, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h508
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h509, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h509
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h510, _, _, _, _, _, _, _, _, _, _, _) => {
            h510
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h511, _, _, _, _, _, _, _, _, _, _) => {
            h511
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h512, _, _, _, _, _, _, _, _, _) => {
            h512
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h513, _, _, _, _, _, _, _, _) => {
            h513
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h514, _, _, _, _, _, _, _) => {
            h514
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h515, _, _, _, _, _, _) => {
            h515
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h516, _, _, _, _, _) => {
            h516
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h517, _, _, _, _) => {
            h517
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h518, _, _, _) => {
            h518
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h519, _, _) => {
            h519
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h520, _) => {
            h520
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h521) => {
            h521
        }
    }, f23(f41(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h522, _) => {
            h522
        }
    }), f23(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h523, _, _, _, _, _, _) => {
            h523
        }
    }, f23(f42(match v0.clone() {
        Adt2::C0(_, h524, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h524
        }
    }), match v0 {
        Adt2::C0(h525, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h525
        }
    }))))
}

pub fn f37(v0: Fn3, p529: u64, p530: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m528 = (p529, p530);
    {
        let mut m528 = m528;
        loop {
            let step = {
                let (v1, v2) = m528;
                let m526 = v1;
                if m526 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m526 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c527: Fn3 = v0.clone();
                        c527.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h531, _) = v0;
        h531
    };
    let v2: u32 = {
        let (h532, _) = {
            let (_, h533) = v0;
            h533
        };
        h532
    };
    let v3: u32 = {
        let (_, h534) = {
            let (_, h535) = v0;
            h535
        };
        h534
    };
    ({
        let a536 = v1;
        let a537 = 2u64;
        let a538 = 0u64;
        nat_quot(a536, a537, a538)
    }, (f39(v2, 1u32), {
        let a542 = {
            let a539 = v1;
            let a540 = 2u64;
            let a541 = 0u64;
            nat_rem(a539, a540, a541)
        };
        let a543 = 1u64;
        if nat_eq(a542, a543) {
            let a544 = v3;
            let a545 = v2;
            fixed_u32::bit_or(a544, a545)
        } else {
            v3
        }
    }))
}

pub fn f39(v0: u32, v1: u32) -> u32 {
    let m548 = {
        let a546 = v0;
        let a547 = v1;
        fixed_u32::shift_left(a546, a547)
    };
    match m548 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40(v0: u32, v1: u32, v2: u32) -> u32 {
    let a549 = f18(v0, v1);
    let a550 = f39(v0, v2);
    fixed_u32::bit_or(a549, a550)
}

pub fn f41(v0: u32) -> u32 {
    let a553 = {
        let a551 = f40(v0, 17u32, 15u32);
        let a552 = f40(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a551, a552)
    };
    let a554 = f18(v0, 10u32);
    fixed_u32::bit_xor(a553, a554)
}

pub fn f42(v0: u32) -> u32 {
    let a557 = {
        let a555 = f40(v0, 7u32, 25u32);
        let a556 = f40(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a555, a556)
    };
    let a558 = f18(v0, 3u32);
    fixed_u32::bit_xor(a557, a558)
}
