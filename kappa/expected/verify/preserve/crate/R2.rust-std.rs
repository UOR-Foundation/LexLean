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
    F24,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn2::F24 => f24(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F40,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn3::F40 => f40(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    if f1(v0.clone())? {
        Ok(Some(f2({
            let a1 = f3(v0.clone())?;
            let a2 = v0;
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
        if f4(v0.clone(), f5(Str::lit("sha256:"))) {
            let a6 = f6(v0, 7u64)?;
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
    let a8 = f5(Str::lit("sha256:"));
    let a9 = f7(v0)?;
    Ok(append_bytes(a8, a9))
}

pub fn f3(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f8(f9(v0.clone(), 7u64)?);
    let v2: u64 = f8(f9(v0, 8u64)?);
    let a17 = {
        let a14 = {
            let a12 = {
                let a10 = v1;
                let a11 = 16u64;
                nat_mul(a10, a11)?
            };
            let a13 = v2;
            nat_add(a12, a13)?
        };
        let a15 = 9u64;
        let a16 = 0u64;
        nat_rem(a14, a15, a16)
    };
    let a18 = 8u64;
    if nat_eq(a17, a18) {
        Ok(Bytes::lit(&[0x06]))
    } else {
        Ok(Bytes::lit(&[0x05]))
    }
}

pub fn f4(v0: Bytes, v1: Bytes) -> bool {
    let m23 = {
        let a20 = v0;
        let a21 = 0u64;
        let a22 = {
            let a19 = v1.clone();
            length_bytes(a19)
        };
        slice_bytes(a20, a21, a22)
    };
    match m23 {
        None => {
            false
        }
        Some(v2) => {
            let a24 = v2;
            let a25 = v1;
            equal(a24, a25)
        }
    }
}

pub fn f5(v0: Str) -> Bytes {
    let a26 = v0;
    utf8_encode(a26)
}

pub fn f6(v0: Bytes, v1: u64) -> R<u64> {
    let (h27, _) = f10(Fn0::F11(v0.clone()), {
        let a28 = v0;
        length_bytes(a28)
    }, v1)?;
    Ok(h27)
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    Ok(f12(f13(v0)?))
}

pub fn f8(v0: u64) -> u64 {
    let a29 = 47u64;
    let a30 = v0;
    if nat_lt(a29, a30) {
        let a31 = v0;
        let a32 = 58u64;
        if nat_lt(a31, a32) {
            let a33 = v0;
            let a34 = 48u64;
            nat_sub(a33, a34)
        } else {
            let a35 = 64u64;
            let a36 = v0;
            if nat_lt(a35, a36) {
                let a37 = v0;
                let a38 = 71u64;
                if nat_lt(a37, a38) {
                    let a39 = v0;
                    let a40 = 55u64;
                    nat_sub(a39, a40)
                } else {
                    let a41 = 96u64;
                    let a42 = v0;
                    if nat_lt(a41, a42) {
                        let a43 = v0;
                        let a44 = 103u64;
                        if nat_lt(a43, a44) {
                            let a45 = v0;
                            let a46 = 87u64;
                            nat_sub(a45, a46)
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

pub fn f9(v0: Bytes, v1: u64) -> R<u64> {
    let m49 = {
        let a47 = v0;
        let a48 = v1;
        index_bytes(a47, a48)
    };
    match m49 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a94 = {
                let a70 = {
                    let a58 = {
                        let a52 = {
                            let a50 = v3;
                            let a51 = 128u8;
                            fixed_u8::bit_and(a50, a51)
                        };
                        let a53 = 128u8;
                        if equal(a52, a53) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a59 = {
                        let a56 = {
                            let a54 = v3;
                            let a55 = 64u8;
                            fixed_u8::bit_and(a54, a55)
                        };
                        let a57 = 64u8;
                        if equal(a56, a57) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a58, a59)?
                };
                let a71 = {
                    let a68 = {
                        let a62 = {
                            let a60 = v3;
                            let a61 = 32u8;
                            fixed_u8::bit_and(a60, a61)
                        };
                        let a63 = 32u8;
                        if equal(a62, a63) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a69 = {
                        let a66 = {
                            let a64 = v3;
                            let a65 = 16u8;
                            fixed_u8::bit_and(a64, a65)
                        };
                        let a67 = 16u8;
                        if equal(a66, a67) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a68, a69)?
                };
                nat_add(a70, a71)?
            };
            let a95 = {
                let a92 = {
                    let a80 = {
                        let a74 = {
                            let a72 = v3;
                            let a73 = 8u8;
                            fixed_u8::bit_and(a72, a73)
                        };
                        let a75 = 8u8;
                        if equal(a74, a75) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a81 = {
                        let a78 = {
                            let a76 = v3;
                            let a77 = 4u8;
                            fixed_u8::bit_and(a76, a77)
                        };
                        let a79 = 4u8;
                        if equal(a78, a79) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a80, a81)?
                };
                let a93 = {
                    let a90 = {
                        let a84 = {
                            let a82 = v3;
                            let a83 = 2u8;
                            fixed_u8::bit_and(a82, a83)
                        };
                        let a85 = 2u8;
                        if equal(a84, a85) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a91 = {
                        let a88 = {
                            let a86 = v3;
                            let a87 = 1u8;
                            fixed_u8::bit_and(a86, a87)
                        };
                        let a89 = 1u8;
                        if equal(a88, a89) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a90, a91)?
                };
                nat_add(a92, a93)?
            };
            nat_add(a94, a95)
        }
    }
}

pub fn f10(v0: Fn0, p100: u64, p101: u64) -> R<(u64, bool)> {
    let m99 = (p100, p101);
    {
        let mut m99 = m99;
        loop {
            let step = {
                let (v1, v2) = m99;
                let m96 = v1;
                if m96 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m96 - 1;
                    let m98 = {
                        let c97: Fn0 = v0.clone();
                        c97.apply(v2)?
                    };
                    match m98 {
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
                Ok(next) => { m99 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f14(v0, v1)? {
        Ok(Some({
            let a102 = v1;
            let a103 = 1u64;
            nat_add(a102, a103)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f12(v0: Adt0) -> Bytes {
    let a124 = f15(match v0.clone() {
        Adt0::C0(h104, _, _, _, _, _, _, _) => {
            h104
        }
    });
    let a125 = {
        let a122 = f15(match v0.clone() {
            Adt0::C0(_, h105, _, _, _, _, _, _) => {
                h105
            }
        });
        let a123 = {
            let a120 = f15(match v0.clone() {
                Adt0::C0(_, _, h106, _, _, _, _, _) => {
                    h106
                }
            });
            let a121 = {
                let a118 = f15(match v0.clone() {
                    Adt0::C0(_, _, _, h107, _, _, _, _) => {
                        h107
                    }
                });
                let a119 = {
                    let a116 = f15(match v0.clone() {
                        Adt0::C0(_, _, _, _, h108, _, _, _) => {
                            h108
                        }
                    });
                    let a117 = {
                        let a114 = f15(match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h109, _, _) => {
                                h109
                            }
                        });
                        let a115 = {
                            let a112 = f15(match v0.clone() {
                                Adt0::C0(_, _, _, _, _, _, h110, _) => {
                                    h110
                                }
                            });
                            let a113 = f15(match v0 {
                                Adt0::C0(_, _, _, _, _, _, _, h111) => {
                                    h111
                                }
                            });
                            append_bytes(a112, a113)
                        };
                        append_bytes(a114, a115)
                    };
                    append_bytes(a116, a117)
                };
                append_bytes(a118, a119)
            };
            append_bytes(a120, a121)
        };
        append_bytes(a122, a123)
    };
    append_bytes(a124, a125)
}

pub fn f13(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a126 = v0.clone();
        length_bytes(a126)
    };
    let v2: u64 = {
        let a132 = {
            let a129 = {
                let a127 = v1;
                let a128 = 72u64;
                nat_add(a127, a128)?
            };
            let a130 = 64u64;
            let a131 = 0u64;
            nat_quot(a129, a130, a131)
        };
        let a133 = 64u64;
        nat_mul(a132, a133)?
    };
    let (_, h134) = f16(Fn1::F17(v0, v1, v2), {
        let a135 = v2;
        let a136 = 64u64;
        let a137 = 0u64;
        nat_quot(a135, a136, a137)
    }, (0u64, f18()))?;
    Ok(h134)
}

pub fn f14(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f9(v0, v1)?;
    let a138 = 47u64;
    let a139 = v2;
    if nat_lt(a138, a139) {
        let a140 = v2;
        let a141 = 58u64;
        if nat_lt(a140, a141) {
            Ok(true)
        } else {
            let a142 = 96u64;
            let a143 = v2;
            if nat_lt(a142, a143) {
                let a144 = v2;
                let a145 = 103u64;
                Ok(nat_lt(a144, a145))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f15(v0: u32) -> Bytes {
    let a174 = f19({
        let a146 = f20(v0, 28u32);
        let a147 = 15u32;
        fixed_u32::bit_and(a146, a147)
    });
    let a175 = {
        let a172 = f19({
            let a148 = f20(v0, 24u32);
            let a149 = 15u32;
            fixed_u32::bit_and(a148, a149)
        });
        let a173 = {
            let a170 = f19({
                let a150 = f20(v0, 20u32);
                let a151 = 15u32;
                fixed_u32::bit_and(a150, a151)
            });
            let a171 = {
                let a168 = f19({
                    let a152 = f20(v0, 16u32);
                    let a153 = 15u32;
                    fixed_u32::bit_and(a152, a153)
                });
                let a169 = {
                    let a166 = f19({
                        let a154 = f20(v0, 12u32);
                        let a155 = 15u32;
                        fixed_u32::bit_and(a154, a155)
                    });
                    let a167 = {
                        let a164 = f19({
                            let a156 = f20(v0, 8u32);
                            let a157 = 15u32;
                            fixed_u32::bit_and(a156, a157)
                        });
                        let a165 = {
                            let a162 = f19({
                                let a158 = f20(v0, 4u32);
                                let a159 = 15u32;
                                fixed_u32::bit_and(a158, a159)
                            });
                            let a163 = f19({
                                let a160 = v0;
                                let a161 = 15u32;
                                fixed_u32::bit_and(a160, a161)
                            });
                            append_bytes(a162, a163)
                        };
                        append_bytes(a164, a165)
                    };
                    append_bytes(a166, a167)
                };
                append_bytes(a168, a169)
            };
            append_bytes(a170, a171)
        };
        append_bytes(a172, a173)
    };
    append_bytes(a174, a175)
}

pub fn f16(v0: Fn1, p179: u64, p180: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m178 = (p179, p180);
    {
        let mut m178 = m178;
        loop {
            let step = {
                let (v1, v2) = m178;
                let m176 = v1;
                if m176 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m176 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c177: Fn1 = v0.clone();
                        c177.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a182 = {
            let (h181, _) = v3.clone();
            h181
        };
        let a183 = 64u64;
        nat_add(a182, a183)?
    }, f21({
        let (_, h184) = v3;
        h184
    }, f22(v0, v1, v2, {
        let (h185, _) = v3;
        h185
    })?)?))
}

pub fn f18() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f19(v0: u32) -> Bytes {
    let a188 = {
        let a186 = v0;
        let a187 = 8u32;
        fixed_u32::bit_and(a186, a187)
    };
    let a189 = 0u32;
    if equal(a188, a189) {
        let a192 = {
            let a190 = v0;
            let a191 = 4u32;
            fixed_u32::bit_and(a190, a191)
        };
        let a193 = 0u32;
        if equal(a192, a193) {
            let a196 = {
                let a194 = v0;
                let a195 = 2u32;
                fixed_u32::bit_and(a194, a195)
            };
            let a197 = 0u32;
            if equal(a196, a197) {
                let a200 = {
                    let a198 = v0;
                    let a199 = 1u32;
                    fixed_u32::bit_and(a198, a199)
                };
                let a201 = 0u32;
                if equal(a200, a201) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a204 = {
                    let a202 = v0;
                    let a203 = 1u32;
                    fixed_u32::bit_and(a202, a203)
                };
                let a205 = 0u32;
                if equal(a204, a205) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a208 = {
                let a206 = v0;
                let a207 = 2u32;
                fixed_u32::bit_and(a206, a207)
            };
            let a209 = 0u32;
            if equal(a208, a209) {
                let a212 = {
                    let a210 = v0;
                    let a211 = 1u32;
                    fixed_u32::bit_and(a210, a211)
                };
                let a213 = 0u32;
                if equal(a212, a213) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a216 = {
                    let a214 = v0;
                    let a215 = 1u32;
                    fixed_u32::bit_and(a214, a215)
                };
                let a217 = 0u32;
                if equal(a216, a217) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a220 = {
            let a218 = v0;
            let a219 = 4u32;
            fixed_u32::bit_and(a218, a219)
        };
        let a221 = 0u32;
        if equal(a220, a221) {
            let a224 = {
                let a222 = v0;
                let a223 = 2u32;
                fixed_u32::bit_and(a222, a223)
            };
            let a225 = 0u32;
            if equal(a224, a225) {
                let a228 = {
                    let a226 = v0;
                    let a227 = 1u32;
                    fixed_u32::bit_and(a226, a227)
                };
                let a229 = 0u32;
                if equal(a228, a229) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a232 = {
                    let a230 = v0;
                    let a231 = 1u32;
                    fixed_u32::bit_and(a230, a231)
                };
                let a233 = 0u32;
                if equal(a232, a233) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a236 = {
                let a234 = v0;
                let a235 = 2u32;
                fixed_u32::bit_and(a234, a235)
            };
            let a237 = 0u32;
            if equal(a236, a237) {
                let a240 = {
                    let a238 = v0;
                    let a239 = 1u32;
                    fixed_u32::bit_and(a238, a239)
                };
                let a241 = 0u32;
                if equal(a240, a241) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a244 = {
                    let a242 = v0;
                    let a243 = 1u32;
                    fixed_u32::bit_and(a242, a243)
                };
                let a245 = 0u32;
                if equal(a244, a245) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f20(v0: u32, v1: u32) -> u32 {
    let m248 = {
        let a246 = v0;
        let a247 = v1;
        fixed_u32::shift_right(a246, a247)
    };
    match m248 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f21(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f23(Fn2::F24, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h249, _, _, _, _, _, _, _) => {
            h249
        }
    }, match v0.clone() {
        Adt0::C0(_, h250, _, _, _, _, _, _) => {
            h250
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h251, _, _, _, _, _) => {
            h251
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h252, _, _, _, _) => {
            h252
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h253, _, _, _) => {
            h253
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h254, _, _) => {
            h254
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h255, _) => {
            h255
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h256) => {
            h256
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f25(match v0.clone() {
        Adt0::C0(h257, _, _, _, _, _, _, _) => {
            h257
        }
    }, match v2.clone() {
        Adt2::C0(h258, _, _, _, _, _, _, _, _, _) => {
            h258
        }
    }), f25(match v0.clone() {
        Adt0::C0(_, h259, _, _, _, _, _, _) => {
            h259
        }
    }, match v2.clone() {
        Adt2::C0(_, h260, _, _, _, _, _, _, _, _) => {
            h260
        }
    }), f25(match v0.clone() {
        Adt0::C0(_, _, h261, _, _, _, _, _) => {
            h261
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h262, _, _, _, _, _, _, _) => {
            h262
        }
    }), f25(match v0.clone() {
        Adt0::C0(_, _, _, h263, _, _, _, _) => {
            h263
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h264, _, _, _, _, _, _) => {
            h264
        }
    }), f25(match v0.clone() {
        Adt0::C0(_, _, _, _, h265, _, _, _) => {
            h265
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h266, _, _, _, _, _) => {
            h266
        }
    }), f25(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h267, _, _) => {
            h267
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h268, _, _, _, _) => {
            h268
        }
    }), f25(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h269, _) => {
            h269
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h270, _, _, _) => {
            h270
        }
    }), f25(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h271) => {
            h271
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h272, _, _) => {
            h272
        }
    })))
}

pub fn f22(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f26(v0.clone(), v1, v2, v3)?, f26(v0.clone(), v1, v2, {
        let a273 = v3;
        let a274 = 4u64;
        nat_add(a273, a274)?
    })?, f26(v0.clone(), v1, v2, {
        let a275 = v3;
        let a276 = 8u64;
        nat_add(a275, a276)?
    })?, f26(v0.clone(), v1, v2, {
        let a277 = v3;
        let a278 = 12u64;
        nat_add(a277, a278)?
    })?, f26(v0.clone(), v1, v2, {
        let a279 = v3;
        let a280 = 16u64;
        nat_add(a279, a280)?
    })?, f26(v0.clone(), v1, v2, {
        let a281 = v3;
        let a282 = 20u64;
        nat_add(a281, a282)?
    })?, f26(v0.clone(), v1, v2, {
        let a283 = v3;
        let a284 = 24u64;
        nat_add(a283, a284)?
    })?, f26(v0.clone(), v1, v2, {
        let a285 = v3;
        let a286 = 28u64;
        nat_add(a285, a286)?
    })?, f26(v0.clone(), v1, v2, {
        let a287 = v3;
        let a288 = 32u64;
        nat_add(a287, a288)?
    })?, f26(v0.clone(), v1, v2, {
        let a289 = v3;
        let a290 = 36u64;
        nat_add(a289, a290)?
    })?, f26(v0.clone(), v1, v2, {
        let a291 = v3;
        let a292 = 40u64;
        nat_add(a291, a292)?
    })?, f26(v0.clone(), v1, v2, {
        let a293 = v3;
        let a294 = 44u64;
        nat_add(a293, a294)?
    })?, f26(v0.clone(), v1, v2, {
        let a295 = v3;
        let a296 = 48u64;
        nat_add(a295, a296)?
    })?, f26(v0.clone(), v1, v2, {
        let a297 = v3;
        let a298 = 52u64;
        nat_add(a297, a298)?
    })?, f26(v0.clone(), v1, v2, {
        let a299 = v3;
        let a300 = 56u64;
        nat_add(a299, a300)?
    })?, f26(v0, v1, v2, {
        let a301 = v3;
        let a302 = 60u64;
        nat_add(a301, a302)?
    })?))
}

pub fn f23(v0: Fn2, p306: u64, p307: Adt2) -> R<Adt2> {
    let m305 = (p306, p307);
    {
        let mut m305 = m305;
        loop {
            let step = {
                let (v1, v2) = m305;
                let m303 = v1;
                if m303 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m303 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c304: Fn2 = v0.clone();
                        c304.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Adt2) -> R<Adt2> {
    f27(v0)
}

pub fn f25(v0: u32, v1: u32) -> u32 {
    let m310 = {
        let a308 = f28(v0);
        let a309 = f28(v1);
        fixed_u64::checked_add(a308, a309)
    };
    match m310 {
        None => {
            0u32
        }
        Some(v2) => {
            f29(v2)
        }
    }
}

pub fn f26(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a313 = {
        let a311 = v3;
        let a312 = 8u64;
        nat_add(a311, a312)?
    };
    let a314 = v2;
    if nat_eq(a313, a314) {
        Ok(f30({
            let a317 = {
                let a315 = v1;
                let a316 = 8u64;
                nat_mul(a315, a316)?
            };
            let a318 = 4294967296u64;
            let a319 = 0u64;
            nat_quot(a317, a318, a319)
        }))
    } else {
        let a322 = {
            let a320 = v3;
            let a321 = 4u64;
            nat_add(a320, a321)?
        };
        let a323 = v2;
        if nat_eq(a322, a323) {
            Ok(f30({
                let a326 = {
                    let a324 = v1;
                    let a325 = 8u64;
                    nat_mul(a324, a325)?
                };
                let a327 = 4294967296u64;
                let a328 = 0u64;
                nat_rem(a326, a327, a328)
            }))
        } else {
            Ok(f31(f32(v0.clone(), v1, v3), f32(v0.clone(), v1, {
                let a329 = v3;
                let a330 = 1u64;
                nat_add(a329, a330)?
            }), f32(v0.clone(), v1, {
                let a331 = v3;
                let a332 = 2u64;
                nat_add(a331, a332)?
            }), f32(v0, v1, {
                let a333 = v3;
                let a334 = 3u64;
                nat_add(a333, a334)?
            })))
        }
    }
}

pub fn f27(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f33(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h335) => {
            h335
        }
    });
    let v2: u32 = f25(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h336, _, _) => {
            h336
        }
    }, f25(f34(match v0.clone() {
        Adt2::C0(_, _, _, _, h337, _, _, _, _, _) => {
            h337
        }
    }), f25(f35(match v0.clone() {
        Adt2::C0(_, _, _, _, h338, _, _, _, _, _) => {
            h338
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h339, _, _, _, _) => {
            h339
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h340, _, _, _) => {
            h340
        }
    }), f25(v1, {
        let m343 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h342, _) => {
                h342
            }
        };
        match m343 {
            Adt1::C0(h341, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h341
            }
        }
    }))));
    let v3: u32 = f25(f36(match v0.clone() {
        Adt2::C0(h344, _, _, _, _, _, _, _, _, _) => {
            h344
        }
    }), f37(match v0.clone() {
        Adt2::C0(h345, _, _, _, _, _, _, _, _, _) => {
            h345
        }
    }, match v0.clone() {
        Adt2::C0(_, h346, _, _, _, _, _, _, _, _) => {
            h346
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h347, _, _, _, _, _, _, _) => {
            h347
        }
    }));
    Ok(Adt2::C0(f25(v2, v3), match v0.clone() {
        Adt2::C0(h348, _, _, _, _, _, _, _, _, _) => {
            h348
        }
    }, match v0.clone() {
        Adt2::C0(_, h349, _, _, _, _, _, _, _, _) => {
            h349
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h350, _, _, _, _, _, _, _) => {
            h350
        }
    }, f25(match v0.clone() {
        Adt2::C0(_, _, _, h351, _, _, _, _, _, _) => {
            h351
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h352, _, _, _, _, _) => {
            h352
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h353, _, _, _, _) => {
            h353
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h354, _, _, _) => {
            h354
        }
    }, f38(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h355, _) => {
            h355
        }
    }), {
        let a357 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h356) => {
                h356
            }
        };
        let a358 = 1u64;
        nat_add(a357, a358)?
    }))
}

pub fn f28(v0: u32) -> u64 {
    let m360 = {
        let a359 = v0;
        fixed_u64::convert(i128::from(a359))
    };
    match m360 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f29(v0: u64) -> u32 {
    let m364 = {
        let a363 = {
            let a361 = v0;
            let a362 = 4294967295u64;
            fixed_u64::bit_and(a361, a362)
        };
        fixed_u32::convert(i128::from(a363))
    };
    match m364 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f30(v0: u64) -> u32 {
    let (_, h365) = {
        let (_, h366) = f39(Fn3::F40, 32u64, (v0, (1u32, 0u32)));
        h366
    };
    h365
}

pub fn f31(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a371 = f41(v0, 24u32);
    let a372 = {
        let a369 = f41(v1, 16u32);
        let a370 = {
            let a367 = f41(v2, 8u32);
            let a368 = v3;
            fixed_u32::bit_or(a367, a368)
        };
        fixed_u32::bit_or(a369, a370)
    };
    fixed_u32::bit_or(a371, a372)
}

pub fn f32(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a373 = v2;
    let a374 = v1;
    if nat_lt(a373, a374) {
        let m377 = {
            let a375 = v0;
            let a376 = v2;
            index_bytes(a375, a376)
        };
        match m377 {
            None => {
                0u32
            }
            Some(v3) => {
                let m379 = {
                    let a378 = v3;
                    fixed_u32::convert(i128::from(a378))
                };
                match m379 {
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
        let a380 = v2;
        let a381 = v1;
        if nat_eq(a380, a381) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f33(v0: u64) -> u32 {
    let a382 = v0;
    let a383 = 32u64;
    if nat_lt(a382, a383) {
        let a384 = v0;
        let a385 = 16u64;
        if nat_lt(a384, a385) {
            let a386 = v0;
            let a387 = 8u64;
            if nat_lt(a386, a387) {
                let a388 = v0;
                let a389 = 4u64;
                if nat_lt(a388, a389) {
                    let a390 = v0;
                    let a391 = 2u64;
                    if nat_lt(a390, a391) {
                        let a392 = v0;
                        let a393 = 1u64;
                        if nat_lt(a392, a393) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a394 = v0;
                        let a395 = 3u64;
                        if nat_lt(a394, a395) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a396 = v0;
                    let a397 = 6u64;
                    if nat_lt(a396, a397) {
                        let a398 = v0;
                        let a399 = 5u64;
                        if nat_lt(a398, a399) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a400 = v0;
                        let a401 = 7u64;
                        if nat_lt(a400, a401) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a402 = v0;
                let a403 = 12u64;
                if nat_lt(a402, a403) {
                    let a404 = v0;
                    let a405 = 10u64;
                    if nat_lt(a404, a405) {
                        let a406 = v0;
                        let a407 = 9u64;
                        if nat_lt(a406, a407) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a408 = v0;
                        let a409 = 11u64;
                        if nat_lt(a408, a409) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a410 = v0;
                    let a411 = 14u64;
                    if nat_lt(a410, a411) {
                        let a412 = v0;
                        let a413 = 13u64;
                        if nat_lt(a412, a413) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a414 = v0;
                        let a415 = 15u64;
                        if nat_lt(a414, a415) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a416 = v0;
            let a417 = 24u64;
            if nat_lt(a416, a417) {
                let a418 = v0;
                let a419 = 20u64;
                if nat_lt(a418, a419) {
                    let a420 = v0;
                    let a421 = 18u64;
                    if nat_lt(a420, a421) {
                        let a422 = v0;
                        let a423 = 17u64;
                        if nat_lt(a422, a423) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a424 = v0;
                        let a425 = 19u64;
                        if nat_lt(a424, a425) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a426 = v0;
                    let a427 = 22u64;
                    if nat_lt(a426, a427) {
                        let a428 = v0;
                        let a429 = 21u64;
                        if nat_lt(a428, a429) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a430 = v0;
                        let a431 = 23u64;
                        if nat_lt(a430, a431) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a432 = v0;
                let a433 = 28u64;
                if nat_lt(a432, a433) {
                    let a434 = v0;
                    let a435 = 26u64;
                    if nat_lt(a434, a435) {
                        let a436 = v0;
                        let a437 = 25u64;
                        if nat_lt(a436, a437) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a438 = v0;
                        let a439 = 27u64;
                        if nat_lt(a438, a439) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a440 = v0;
                    let a441 = 30u64;
                    if nat_lt(a440, a441) {
                        let a442 = v0;
                        let a443 = 29u64;
                        if nat_lt(a442, a443) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a444 = v0;
                        let a445 = 31u64;
                        if nat_lt(a444, a445) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a446 = v0;
        let a447 = 48u64;
        if nat_lt(a446, a447) {
            let a448 = v0;
            let a449 = 40u64;
            if nat_lt(a448, a449) {
                let a450 = v0;
                let a451 = 36u64;
                if nat_lt(a450, a451) {
                    let a452 = v0;
                    let a453 = 34u64;
                    if nat_lt(a452, a453) {
                        let a454 = v0;
                        let a455 = 33u64;
                        if nat_lt(a454, a455) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a456 = v0;
                        let a457 = 35u64;
                        if nat_lt(a456, a457) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a458 = v0;
                    let a459 = 38u64;
                    if nat_lt(a458, a459) {
                        let a460 = v0;
                        let a461 = 37u64;
                        if nat_lt(a460, a461) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a462 = v0;
                        let a463 = 39u64;
                        if nat_lt(a462, a463) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a464 = v0;
                let a465 = 44u64;
                if nat_lt(a464, a465) {
                    let a466 = v0;
                    let a467 = 42u64;
                    if nat_lt(a466, a467) {
                        let a468 = v0;
                        let a469 = 41u64;
                        if nat_lt(a468, a469) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a470 = v0;
                        let a471 = 43u64;
                        if nat_lt(a470, a471) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a472 = v0;
                    let a473 = 46u64;
                    if nat_lt(a472, a473) {
                        let a474 = v0;
                        let a475 = 45u64;
                        if nat_lt(a474, a475) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a476 = v0;
                        let a477 = 47u64;
                        if nat_lt(a476, a477) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a478 = v0;
            let a479 = 56u64;
            if nat_lt(a478, a479) {
                let a480 = v0;
                let a481 = 52u64;
                if nat_lt(a480, a481) {
                    let a482 = v0;
                    let a483 = 50u64;
                    if nat_lt(a482, a483) {
                        let a484 = v0;
                        let a485 = 49u64;
                        if nat_lt(a484, a485) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a486 = v0;
                        let a487 = 51u64;
                        if nat_lt(a486, a487) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a488 = v0;
                    let a489 = 54u64;
                    if nat_lt(a488, a489) {
                        let a490 = v0;
                        let a491 = 53u64;
                        if nat_lt(a490, a491) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a492 = v0;
                        let a493 = 55u64;
                        if nat_lt(a492, a493) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a494 = v0;
                let a495 = 60u64;
                if nat_lt(a494, a495) {
                    let a496 = v0;
                    let a497 = 58u64;
                    if nat_lt(a496, a497) {
                        let a498 = v0;
                        let a499 = 57u64;
                        if nat_lt(a498, a499) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a500 = v0;
                        let a501 = 59u64;
                        if nat_lt(a500, a501) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a502 = v0;
                    let a503 = 62u64;
                    if nat_lt(a502, a503) {
                        let a504 = v0;
                        let a505 = 61u64;
                        if nat_lt(a504, a505) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a506 = v0;
                        let a507 = 63u64;
                        if nat_lt(a506, a507) {
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

pub fn f34(v0: u32) -> u32 {
    let a510 = {
        let a508 = f42(v0, 6u32, 26u32);
        let a509 = f42(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a508, a509)
    };
    let a511 = f42(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a510, a511)
}

pub fn f35(v0: u32, v1: u32, v2: u32) -> u32 {
    let a517 = {
        let a512 = v0;
        let a513 = v1;
        fixed_u32::bit_and(a512, a513)
    };
    let a518 = {
        let a515 = {
            let a514 = v0;
            fixed_u32::bit_not(a514)
        };
        let a516 = v2;
        fixed_u32::bit_and(a515, a516)
    };
    fixed_u32::bit_xor(a517, a518)
}

pub fn f36(v0: u32) -> u32 {
    let a521 = {
        let a519 = f42(v0, 2u32, 30u32);
        let a520 = f42(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a519, a520)
    };
    let a522 = f42(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a521, a522)
}

pub fn f37(v0: u32, v1: u32, v2: u32) -> u32 {
    let a531 = {
        let a527 = {
            let a523 = v0;
            let a524 = v1;
            fixed_u32::bit_and(a523, a524)
        };
        let a528 = {
            let a525 = v0;
            let a526 = v2;
            fixed_u32::bit_and(a525, a526)
        };
        fixed_u32::bit_xor(a527, a528)
    };
    let a532 = {
        let a529 = v1;
        let a530 = v2;
        fixed_u32::bit_and(a529, a530)
    };
    fixed_u32::bit_xor(a531, a532)
}

pub fn f38(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h533, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h533
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h534, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h534
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h535, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h535
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h536, _, _, _, _, _, _, _, _, _, _, _) => {
            h536
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h537, _, _, _, _, _, _, _, _, _, _) => {
            h537
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h538, _, _, _, _, _, _, _, _, _) => {
            h538
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h539, _, _, _, _, _, _, _, _) => {
            h539
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h540, _, _, _, _, _, _, _) => {
            h540
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h541, _, _, _, _, _, _) => {
            h541
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h542, _, _, _, _, _) => {
            h542
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h543, _, _, _, _) => {
            h543
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h544, _, _, _) => {
            h544
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h545, _, _) => {
            h545
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h546, _) => {
            h546
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h547) => {
            h547
        }
    }, f25(f43(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h548, _) => {
            h548
        }
    }), f25(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h549, _, _, _, _, _, _) => {
            h549
        }
    }, f25(f44(match v0.clone() {
        Adt1::C0(_, h550, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h550
        }
    }), match v0 {
        Adt1::C0(h551, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h551
        }
    }))))
}

pub fn f39(v0: Fn3, p555: u64, p556: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m554 = (p555, p556);
    {
        let mut m554 = m554;
        loop {
            let step = {
                let (v1, v2) = m554;
                let m552 = v1;
                if m552 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m552 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c553: Fn3 = v0.clone();
                        c553.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h557, _) = v0;
        h557
    };
    let v2: u32 = {
        let (h558, _) = {
            let (_, h559) = v0;
            h559
        };
        h558
    };
    let v3: u32 = {
        let (_, h560) = {
            let (_, h561) = v0;
            h561
        };
        h560
    };
    ({
        let a562 = v1;
        let a563 = 2u64;
        let a564 = 0u64;
        nat_quot(a562, a563, a564)
    }, (f41(v2, 1u32), {
        let a568 = {
            let a565 = v1;
            let a566 = 2u64;
            let a567 = 0u64;
            nat_rem(a565, a566, a567)
        };
        let a569 = 1u64;
        if nat_eq(a568, a569) {
            let a570 = v3;
            let a571 = v2;
            fixed_u32::bit_or(a570, a571)
        } else {
            v3
        }
    }))
}

pub fn f41(v0: u32, v1: u32) -> u32 {
    let m574 = {
        let a572 = v0;
        let a573 = v1;
        fixed_u32::shift_left(a572, a573)
    };
    match m574 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f42(v0: u32, v1: u32, v2: u32) -> u32 {
    let a575 = f20(v0, v1);
    let a576 = f41(v0, v2);
    fixed_u32::bit_or(a575, a576)
}

pub fn f43(v0: u32) -> u32 {
    let a579 = {
        let a577 = f42(v0, 17u32, 15u32);
        let a578 = f42(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a577, a578)
    };
    let a580 = f20(v0, 10u32);
    fixed_u32::bit_xor(a579, a580)
}

pub fn f44(v0: u32) -> u32 {
    let a583 = {
        let a581 = f42(v0, 7u32, 25u32);
        let a582 = f42(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a581, a582)
    };
    let a584 = f20(v0, 3u32);
    fixed_u32::bit_xor(a583, a584)
}
